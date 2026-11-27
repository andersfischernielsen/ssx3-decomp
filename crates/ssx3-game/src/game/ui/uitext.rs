//! `src/ui/uitext.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::script::scriptcontroller::vslot as vslot_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_7;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::shl32;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::main::gamemode::lh_r;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::p_2 as p_3;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_4;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
pub(crate) use crate::game::util::menu::p as p_9;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
pub(crate) use crate::game::wscript::wscriptman::p as p_6;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_0b38, ui_text_free_strings, "UIText_FreeStrings"),
    (
        0x003a_0c00,
        ui_text_get_owned_string,
        "UIText_GetOwnedString",
    ),
    (0x003a_0c28, ui_text_get_id_string, "UIText_GetIdString"),
    (
        0x003a_0c70,
        ui_text_set_ascii_string,
        "UIText_SetAsciiString",
    ),
    (0x003a_0e90, ui_text_set_wide_string, "UIText_SetWideString"),
    (0x003a_1030, ui_text_get_text_size, "UIText_GetTextSize"),
    (0x003a_1148, c_ui_text_scroll_ctor, "cUITextScroll_ctor"),
    (
        0x003a_12d0,
        c_ui_text_scroll_get_font,
        "cUITextScroll_GetFont",
    ),
    (
        0x003a_1310,
        c_ui_text_scroll_set_nav_buttons,
        "cUITextScroll_SetNavButtons",
    ),
    (
        0x003a_13e0,
        cuit_ext_scroll_handle_input,
        "cUITextScroll_HandleInput",
    ),
    (
        0x003a_18c0,
        ui_text_scroll_clear_lines,
        "cUITextScroll_ClearLines",
    ),
    (
        0x003a_1958,
        c_ui_text_scroll_grow_lines,
        "cUITextScroll_GrowLines",
    ),
    (
        0x003a_1bf8,
        c_ui_text_scroll_add_line,
        "cUITextScroll_AddLine",
    ),
    (
        0x003a_1cf0,
        ui_text_scroll_replace_wide_string,
        "cUITextScroll_ReplaceWideString",
    ),
    (
        0x003a_1f18,
        cui_text_scroll_set_text_id,
        "cUITextScroll_SetTextId",
    ),
    (
        0x003a_2068,
        c_ui_text_scroll_get_line,
        "cUITextScroll_GetLine",
    ),
    (0x003a_3280, ui_vector_ctor, "UIVector_ctor"),
    (0x003a_32f0, ui_vector_dtor, "UIVector_dtor"),
    (0x003a_3398, ui_vector_set_color, "UIVector_SetColor"),
    (0x003a_05e8, ui_text_render, "UIText_Render"),
    (
        0x003a_0eb0,
        ui_text_set_wide_string_internal,
        "UIText_SetWideStringInternal",
    ),
    (0x003a_1000, ui_text_set_text_id, "UIText_SetTextId"),
    (0x003a_0b90, ui_text_count_lines, "UIText_CountLines"),
    (
        0x003a_0d00,
        cui_text_set_unicode_string_wrap,
        "cUIText_setUnicodeStringWrap",
    ),
    (0x003a_0c50, ui_text_set_string, "UIText_SetString"),
    (0x003a_10f0, ui_text_get_cached_size, "UIText_GetCachedSize"),
    (0x003a_11b8, c_ui_text_scroll_load, "cUITextScroll_Load"),
    (
        0x003a_1360,
        ui_text_scroll_refresh_text,
        "cUITextScroll_RefreshText",
    ),
    (0x003a_1588, cuitextscroll_render, "cUITextScroll_Render"),
    (
        0x003a_1c80,
        ui_text_scroll_set_ascii_string,
        "cUITextScroll_ResetString",
    ),
    (
        0x003a_1f90,
        ui_text_scroll_remove_line,
        "cUITextScroll_RemoveLine",
    ),
    (
        0x003a_23b8,
        render2d_draw_tiled_sprite,
        "Render2D_DrawTiledSprite",
    ),
    (
        0x003a_2bc8,
        render2d_draw_tiled_sprite_xform,
        "Render2D_DrawTiledSpriteXform",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::UITEXT_SET_WIDE_STRING_INTERNAL;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;

    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE_2;

    pub(crate) use crate::game::fe::festaterewards::callee::CUI_TEXT_SCROLL_SET_NAV_BUTTONS;

    pub(crate) use crate::game::fe::festaterewards::callee::UI_LIST_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_OBJECT_CTOR;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;

    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
}

/// Cross-shard call targets (the generator emits `call_indirect` for them).
pub(crate) mod table {

    pub(crate) use crate::game::ai::computer::table::EA_MEM_FREE;
}

/// `void *cUITextScroll_GetFont(cUITextScroll *text)` (0x3a12d0): the font
/// record selected by bits 1..2 of the word at +0x74.
pub fn c_ui_text_scroll_get_font(c: &mut Cpu) {
    c.cycles += 5;
    let text = abi::arg(c, 0);
    c.g[V0][0] = Addr(text).off(0x5c).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xd0).lw(c);
    c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
    if c.g[A1][0] != 0 {
        c.g[V1][0] = Addr(text).off(0x74).lw(c);
        c.cycles += 8;
        c.g[V0][0] = 0xc;
        c.g[A0][0] = Addr(c.g[A1][0] as u32).off(0x8).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
        c.g[V1][0] = c.g[V1][0] & 0x3;
        c.mult(0, 3, 3, 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void cUITextScroll_SetNavButtons(cUITextScroll *text, int enabled)`
/// (0x3a1310): store the four navigation-button indices at +0x88.
pub fn c_ui_text_scroll_set_nav_buttons(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    let enabled = c.g[A1][0] != 0;
    c.g[A2][0] = c.g[A0][0];
    let text = c.g[A2][0] as u32;
    if enabled {
        c.cycles += 9;
        c.g[V1][0] = 0x4;
        c.g[A0][0] = 0x5;
        c.g[A1][0] = 0x6;
        c.g[V0][0] = 0x7;
        Addr(text).off(0x8b).sb(c, c.g[V0][0]);
        Addr(text).off(0x88).sb(c, c.g[V1][0]);
        Addr(text).off(0x89).sb(c, c.g[A0][0]);
        Addr(text).off(0x8a).sb(c, c.g[A1][0]);
        abi::ret(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = 0x1;
    c.g[A0][0] = 0x2;
    c.g[V1][0] = 0x3;
    Addr(text).off(0x89).sb(c, c.g[V0][0]);
    Addr(text).off(0x8b).sb(c, c.g[V1][0]);
    Addr(text).off(0x8a).sb(c, c.g[A0][0]);
    Addr(text).off(0x88).sb(c, 0);
    abi::ret(c);
}

/// `void *cUITextScroll_GetLine(cUITextScroll *text, int index)`
/// (0x3a2068): the line pointer at `text + index*4`, or null when out of range.
pub fn c_ui_text_scroll_get_line(c: &mut Cpu) {
    c.cycles += 3;
    let text = abi::arg(c, 0);
    c.g[A2][0] = Addr(text).off(0xbc).lw(c);
    let lines_present = c.g[A2][0] != 0;
    if lines_present {
        c.cycles += 2;
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(text).off(0xc8).lhu(c);
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            if !out_of_range {
                c.cycles += 3;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                abi::ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UIText_SetWideString / UIPairFlags_ctor / MPEG2_picture_temporal_scalable_extension.

/// `void UIText_SetWideString(UIText* t, char* str)` (0x3a0e90): clear the
/// cached byte length (`+0xb0`) and install a new wide string.
pub fn ui_text_set_wide_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(r32(c, V0).wrapping_add(0xb0)).sw(c, 0);
        callee::UITEXT_SET_WIDE_STRING_INTERNAL.call(c, 0x003a_0ea4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI text.

/// `UStr UIText_GetOwnedString(UIText *this)` (0x3a0c00): the owned string at
/// `+0xb4`, or 0 when bit 3 of the flags at `+0x74` is set.
pub fn ui_text_get_owned_string(c: &mut Cpu) {
    c.cycles += 5;
    let this = c.g[A0][0] as u32;
    let flags = Addr(this.wrapping_add(0x74)).lw(c);
    c.g[V0][0] = flags;
    c.g[V0][0] = sx(((flags as i32) >> 3) as u32);
    c.g[V0][0] &= 1;
    let owned = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if owned {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr(this.wrapping_add(0xb4)).lw(c);
        abi::ret(c);
    }
}

/// `UStr UIText_GetIdString(UIText *this)` (0x3a0c28): the id string at
/// `+0xb4`, or 0 when bit 3 of the flags at `+0x74` is clear.
pub fn ui_text_get_id_string(c: &mut Cpu) {
    c.cycles += 5;
    let this = c.g[A0][0] as u32;
    let flags = Addr(this.wrapping_add(0x74)).lw(c);
    c.g[V0][0] = flags;
    c.g[V0][0] = sx(((flags as i32) >> 3) as u32);
    c.g[V0][0] &= 1;
    let owned = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if owned {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr(this.wrapping_add(0xb4)).lw(c);
        abi::ret(c);
    }
}

/// `void cUITextScroll_ReplaceWideString(cUITextScroll* self, const
/// unsigned short* str)` (0x3A1CF0): drop the current lines and set the new
/// wide string.
pub fn ui_text_scroll_replace_wide_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::UI_TEXT_SCROLL_CLEAR_LINES.call(c, 0x003a_1d0c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::UI_TEXT_SCROLL_SET_WIDE_STRING.call(c, 0x003a_1d18)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIText_FreeStrings(UIText* t)` (0x3a0b38): clear the string count and
/// free the two string buffers at `+0xac` and `+0xb4`.
pub fn ui_text_free_strings(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.sw(p(c, S0, 0xa8), 0);
    c.g[A0][0] = Addr(p(c, S0, 0xac)).lw(c);
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        if call_ptr(c, table::EA_MEM_FREE, 0x003a_0b60).is_err() {
            return;
        }
        c.sw(p(c, S0, 0xac), 0);
    }
    c.g[A0][0] = Addr(p(c, S0, 0xb4)).lw(c);
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        if call_ptr(c, table::EA_MEM_FREE, 0x003a_0b78).is_err() {
            return;
        }
        c.sw(p(c, S0, 0xb4), 0);
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void UIVector_SetColor(UIVector* v, const Color* c)` (0x003a3398):
/// copy the colour to +0x1c and fill every quad of the array at +0x7c with
/// it, scaled to bytes (count = `+0x74 >> 17`).
pub fn ui_vector_set_color(c: &mut Cpu) {
    c.cycles += 21;
    let v = abi::arg(c, 0);
    let col = abi::arg(c, 1);
    c.g[T0][0] = sx(v);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    // *c -> stack, and *c -> v+0x1c
    c.ldl(2, (col).wrapping_add(0x7));
    c.ldr(2, col);
    c.ldl(3, (col).wrapping_add(0xf));
    c.ldr(3, (col).wrapping_add(0x8));
    c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
    c.sdr(2, c.g[SP][0] as u32);
    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.ldl(9, (col).wrapping_add(0x7));
    c.ldr(9, col);
    c.ldl(2, (col).wrapping_add(0xf));
    c.ldr(2, (col).wrapping_add(0x8));
    c.sdl(9, (c.g[T0][0] as u32).wrapping_add(0x23));
    c.sdr(9, (c.g[T0][0] as u32).wrapping_add(0x1c));
    c.sdl(2, (c.g[T0][0] as u32).wrapping_add(0x2b));
    c.sdr(2, (c.g[T0][0] as u32).wrapping_add(0x24));
    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        abi::ret(c);
        return;
    }
    c.cycles += 34;
    c.g[AT][0] = 0x437f_0000; // 255.0f
    c.mtc1(1, 0);
    c.g[A3][0] = 0;
    c.lwc1(3, (col).wrapping_add(0x4));
    c.lwc1(1, col);
    c.lwc1(2, (col).wrapping_add(0x8));
    c.mul_s(3, 3, 0);
    c.lwc1(4, (col).wrapping_add(0xc));
    c.mul_s(1, 1, 0);
    c.mul_s(2, 2, 0);
    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x74));
    c.mul_s(4, 4, 0);
    c.cvt_w_s(0, 3);
    c.mfc1(3, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 17);
    c.cvt_w_s(0, 1);
    c.mfc1(6, 0);
    c.cvt_w_s(0, 2);
    c.mfc1(4, 0);
    c.cvt_w_s(0, 4);
    c.mfc1(5, 0);
    c.mtc1(6, 0);
    c.cvt_s_w(0, 0);
    c.mtc1(3, 1);
    c.cvt_s_w(1, 1);
    c.mtc1(4, 2);
    c.cvt_s_w(2, 2);
    c.mtc1(5, 3);
    c.cvt_s_w(3, 3);
    c.swc1(0, c.g[SP][0] as u32);
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
    loop {
        c.cycles += 16;
        c.g[V1][0] = sx((c.g[A3][0] as u32) << 4);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[SP][0] as u32);
        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(2, c.g[V1][0] as u32);
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x74));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 17);
        c.g[V0][0] = (c.g[A3][0] < c.g[V0][0]) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
        if event_due(c) {
            deliver_event_here(c, 0x003a_3478);
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

/// `cUITextScroll* cUITextScroll_ctor(cUITextScroll* s, int a1, int a2)`
/// (0x3a1148).
pub fn c_ui_text_scroll_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_CTOR.call(c, 0x003a_115c)?;
        c.g[V1][0] = Addr(p_2(r32(c, S0), 0x74)).lbu(c);
        c.g[V0][0] = 0x0049_4278;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
        c.g[V1][0] = c.g[V1][0] & 0xc6;
        Addr(p_2(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        Addr(p_2(r32(c, S0), 0x74)).sb(c, c.g[V1][0]);
        callee::UI_LIST_CTOR.call(c, 0x003a_117c)?;
        Addr(p_2(r32(c, S0), 0xb8)).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Addr(p_2(r32(c, S0), 0xbc)).sw(c, 0);
        c.g[A1][0] = 0;
        Addr(p_2(r32(c, S0), 0xc0)).sw(c, 0);
        Addr(p_2(r32(c, S0), 0xc4)).sh(c, 0);
        Addr(p_2(r32(c, S0), 0xc6)).sh(c, 0);
        Addr(p_2(r32(c, S0), 0xc8)).sh(c, 0);
        Addr(p_2(r32(c, S0), 0xcc)).sw(c, 0);
        callee::CUI_TEXT_SCROLL_SET_NAV_BUTTONS.call(c, 0x003a_11a4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        lqsp(c, 16, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `UIVector* UIVector_ctor(UIVector* v, int a1, int a2)` (0x3a3280).
pub fn ui_vector_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_CTOR.call(c, 0x003a_3294)?;
        c.g[V1][0] = Addr(p_2(r32(c, S0), 0x74)).lw(c);
        c.g[A1][0] = 0xffff_ffff_ffff_fffe;
        c.g[A2][0] = 0xffff_ffff_ffff_fffd;
        c.g[V0][0] = 0x1_0000;
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
        c.g[A0][0] = 0x0049_41b8;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        Addr(p_2(r32(c, S0), 8)).sw(c, c.g[A0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p_2(r32(c, S0), 0x74)).sw(c, c.g[V1][0]);
        for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88, 0x8c] {
            Addr(p_2(r32(c, S0), off)).sw(c, 0);
        }
        frame.restore_ra(c, 0);
        lqsp(c, 16, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `void cUITextScroll_SetTextId(self, id)` (0x3a1f18): store the text id, look
/// the widget up through `+0x5c/+0xd0/+0x10` and, when present, ask it for its
/// wide string and set it, then re-wrap.
pub fn cui_text_scroll_set_text_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x20));
        c.sq(16, sp_2(c).wrapping_add(0x10));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        sw_r(c, S0, 0xc0, c.g[A1][0]);
        c.g[V0][0] = lw_r(c, S0, 0x5c);
        c.g[V1][0] = lw_r(c, V0, 0xd0);
        c.g[A0][0] = lw_r(c, V1, 0x10);
        c.g[A2][0] = lw_r(c, A0, 0x10);
        if c.g[A2][0] == 0 {
            c.lq(16, sp_2(c).wrapping_add(0x10));
        } else {
            c.cycles += 7;
            c.g[V1][0] = lw_r(c, A2, 0x4);
            c.g[A0][0] = lh_r(c, V1, 0x20);
            c.g[V0][0] = lw_r(c, V1, 0x24);
            let target = low(c, V0);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x003a_1f58)?;
            let empty = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[V0][0];
            if !empty {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                callee::CUI_TEXT_SCROLL_SET_WIDE_STRING.call(c, 0x003a_1f68)?;
            }
            c.cycles += 7;
            c.g[V0][0] = lw_r(c, S0, 0x8);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = lh_r(c, V0, 0x28);
            c.g[V1][0] = lw_r(c, V0, 0x2c);
            let target = low(c, V1);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x003a_1f80)?;
            c.lq(16, sp_2(c).wrapping_add(0x10));
        }
        c.cycles += 3;
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void cUITextScroll_RefreshText(self)` (0x3a1360): re-resolve the text id
/// (`self+0xc0`) through the UI object chain and replace the wide string,
/// then relayout the widget.
pub fn cuitextscroll_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        let id = lw_at(c, p_3(self_, 0xc0));
        if id == 0 {
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw_at(c, p_3(self_, 0x5c));
            c.g[V1][0] = lw_at(c, p_3(c.g[V0][0] as u32, 0xd0));
            c.g[A0][0] = lw_at(c, p_3(c.g[V1][0] as u32, 0x10));
            c.g[A2][0] = lw_at(c, p_3(c.g[A0][0] as u32, 0x10));
            if c.g[A2][0] == 0 {
                frame.restore(c, 16, 0x10);
            } else {
                c.cycles += 7;
                c.g[V1][0] = lw_at(c, p_3(c.g[A2][0] as u32, 4));
                let func = vslot(c, c.g[V1][0], 0x20, V0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x003a_13a8)?;
                let text = c.g[V0][0];
                c.g[A1][0] = text;
                if text != 0 {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = text;
                    call_ptr(c, 0x003a_1cf0, 0x003a_13b8)?;
                    c.g[A1][0] = text;
                }
                c.cycles += 7;
                c.g[V0][0] = lw_at(c, p_3(r32(c, S0), 8));
                c.g[A1][0] = 1;
                let func = vslot(c, c.g[V0][0], 0x28, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x003a_13d0)?;
                frame.restore(c, 16, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUITextScroll_AddLine(cUITextScroll* self, char16* line)`
/// (0x3a1bf8): grow the line array, duplicate the string and append it.
pub fn c_ui_text_scroll_add_line(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        open(c, 0x50);
        let sp0 = spv(c);
        c.sq(19, at(sp0, 0x10));
        c.sq(18, at(sp0, 0x20));
        c.g[S3][0] = c.g[A0][0];
        c.sq(16, at(sp0, 0x40));
        c.sq(17, at(sp0, 0x30));
        c.sd(sp0, c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        callee::GROW_LINES.call(c, 0x003a_1c1c)?;
        let this = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, callee::USTR_LEN, 0x003a_1c28)?;
        c.g[V0][0] = sx(at(c.g[V0][0] as u32, 1));
        c.g[A1][0] = 0x490000;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[A1][0] = sx(at(c.g[A1][0] as u32, 0x40b8));
        call_ptr(c, callee::OPERATOR_NEW_NAMED_2, 0x003a_1c44)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::USTR_CPY, 0x003a_1c54)?;
        c.g[V0][0] = lw_at_2(c, this, 0xbc);
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.lq(18, at(sp0, 0x20));
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V0][0] as u32));
        c.lq(19, at(sp0, 0x10));
        c.sw(c.g[S0][0] as u32, c.g[S1][0]);
        c.lq(16, at(sp0, 0x40));
        c.lq(17, at(sp0, 0x30));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x50));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI text.

/// `void cUIText_setAsciiString(cUIText* self, const char* str)` (0x3a0c70):
/// clear the text's animations, then convert up to 511 ASCII characters into
/// the stack buffer and set them.
pub fn ui_text_set_ascii_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x430);
        frame.save(c, S0, 0x420);
        frame.save(c, S1, 0x410);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x400);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 1;
        c.g[V0][0] = Addr(p_4(r32(c, S0), 8)).lw(c);
        c.g[A0][0] = Addr(p_4(r32(c, V0), 0x28)).lh(c);
        c.g[V1][0] = Addr(p_4(r32(c, V0), 0x2c)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p_4(r32(c, S0), r32(c, A0)));
        call_ptr(c, target, 0x003a_0ca0)?;
        c.g[V0][0] = Addr(p_4(r32(c, S0), 8)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(p_4(r32(c, V0), 0xc0)).lh(c);
        c.g[V1][0] = Addr(p_4(r32(c, V0), 0xc4)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p_4(r32(c, S0), r32(c, A0)));
        call_ptr(c, target, 0x003a_0cb8)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_6810, 0x003a_0cc0)?;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 512i64) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[SP][0];
            callee::UI_ASCII_TO_WIDE.call(c, 0x003a_0cdc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            callee::UI_TEXT_SET_WIDE_STRING_INTERNAL.call(c, 0x003a_0ce8)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x420);
        frame.restore(c, S1, 0x410);
        frame.restore_ra(c, 0x400);
        frame.close(c);
        Ok(())
    })
}

/// `cUITextScroll_ClearLines` (0x3a18c0): free every stored line string,
/// null the line array and reset the counters.
pub fn ui_text_scroll_clear_lines(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p_5(this, 0xbc)).lw(c);
        if c.g[V0][0] == 0 {
            Addr(p_5(this, 0xc8)).sh(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_5(this, 0xc8)).lhu(c);
            let none = c.g[V0][0] == 0;
            c.g[S0][0] = 0;
            if !none {
                c.cycles += 1;
                c.g[V1][0] = Addr(p_5(this, 0xbc)).lw(c);
                loop {
                    c.cycles += 5;
                    c.g[V0][0] = shl32(c.g[S0][0], 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A0][0] = Addr(r(c, V0)).lw(c);
                    let zero = c.g[A0][0] == 0;
                    c.g[V1][0] = sx(r(c, S0).wrapping_add(1));
                    if !zero {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x003a_190c);
                        callee::EA_MEM_FREE_2.call(c, 0x003a_190c)?;
                        c.g[V1][0] = sx(r(c, S0).wrapping_add(1));
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p_5(this, 0xc8)).lhu(c);
                    c.g[S0][0] = c.g[V1][0] & 0xffff;
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V1][0] = Addr(p_5(this, 0xbc)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_18f0);
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = Addr(p_5(this, 0xbc)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003a_1938);
                callee::EA_MEM_FREE_2.call(c, 0x003a_1938)?;
            }
            Addr(p_5(this, 0xbc)).sw(c, 0);
            c.cycles += 1;
            Addr(p_5(this, 0xc8)).sh(c, 0);
        }
        c.cycles += 6;
        Addr(p_5(this, 0xc6)).sh(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Text scroll.

/// `int cUITextScroll_GrowLines(cUITextScroll* self)` (0x3a1958): grow the
/// line-pointer array (`+0xbc`, count `+0xc8`) by one, keep the old entries,
/// and return the old count.
pub fn c_ui_text_scroll_grow_lines(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40a8));
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0x100;
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S3][0] = Addr(p_6(c.g[S1][0] as u32, 0xc8)).lhu(c);
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
        c.sh(p_6(c.g[S1][0] as u32, 0xc8), c.g[V0][0]);
        c.g[A0][0] = c.g[V0][0] & 0xffff;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        call_ptr(c, 0x0031_7e30, 0x003a_199c)?; // operator new
        c.g[A1][0] = lw(c, p_6(c.g[S1][0] as u32, 0xbc));
        let none = c.g[A1][0] == 0;
        c.g[S2][0] = c.g[V0][0];
        if !none {
            c.cycles += 8;
            c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0041_605c, 0x003a_19b8)?; // memcpy
            c.g[A0][0] = lw(c, p_6(c.g[S1][0] as u32, 0xbc));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            let gone = c.g[A0][0] == 0;
            sw(c, c.g[S0][0] as u32, 0);
            if !gone {
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, 0x003a_19d0)?; // EA_MemFree
            }
        }
        c.cycles += 9;
        sw(c, p_6(c.g[S1][0] as u32, 0xbc), c.g[S2][0]);
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void UIVector_dtor(self, flags)` (0x3a32f0): free the vector's five
/// buffers and destroy the UI object.
pub fn ui_vector_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0049_41b8);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p_7(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p_7(r32(c, S0), 0x7c)).lw(c);
        let null = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x003a3324)?;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p_7(r32(c, S0), 0x80)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a3338)?;
            c.g[A0][0] = Addr(p_7(r32(c, S0), 0x84)).lw(c);
        } else {
            c.g[A0][0] = Addr(p_7(r32(c, S0), 0x84)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a334c)?;
            c.g[A0][0] = Addr(p_7(r32(c, S0), 0x88)).lw(c);
        } else {
            c.g[A0][0] = Addr(p_7(r32(c, S0), 0x88)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a3360)?;
            c.g[A0][0] = Addr(p_7(r32(c, S0), 0x8c)).lw(c);
        } else {
            c.g[A0][0] = Addr(p_7(r32(c, S0), 0x8c)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a3374)?;
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_fc48, 0x003a3380)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIText_GetTextSize` (0x003a1030).
//
// `void UIText::GetTextSize(self, V2* out)`: return the measured text size,
// from the layout object when it is available and from the font otherwise.

pub fn ui_text_get_text_size(c: &mut Cpu) {
    guest(c, |c| {
        'b_3a1078: {
            'b_3a1064: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb4));
                let t = c.g[V0][0] == 0u64;
                c.g[S1][0] = c.g[A1][0];
                if t {
                    break 'b_3a1064;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_3a1078;
                    } else {
                        break 'b_3a1064;
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = vslot_2(c, c.g[V0][0], 0x80, V1);
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x003a1078);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x003a1078)?;
            break 'b_3a1078;
        }
        'b_3a10dc: {
            'b_3a10c0: {
                c.cycles += 5;
                c.g[RA][0] = sx(0x003a1080);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003a04f0, 0x003a1080)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb4));
                if c.g[A1][0] == 0u64 {
                    c.g[V0][0] = 0x500000;
                    break 'b_3a10c0;
                } else {
                    'b_3a10bc: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[V0][0];
                        if t {
                            break 'b_3a10bc;
                        } else {
                            c.cycles += 10;
                            c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x54));
                            c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x50));
                            c.g[A2][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x003a10a8);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x003921f0, 0x003a10a8)?;
                            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                            c.swc1(1, c.g[S1][0] as u32);
                            let t = true;
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                break 'b_3a10dc;
                            } else {
                                break 'b_3a10bc;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x500000;
                    break 'b_3a10c0;
                }
            }
            c.cycles += 7;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
            c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
            c.ldr(3, c.g[A2][0] as u32);
            c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x7));
            c.sdr(3, c.g[S1][0] as u32);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            break 'b_3a10dc;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003a13e0 cUITextScroll_HandleInput
// ---------------------------------------------------------------------------

/// `int cUITextScroll_HandleInput(cUITextScroll* self, void* input)` (0x3a13e0).
///
/// Keyboard handling of a scrolling text box. The row count comes from the
/// box height divided by the scaled font height plus three; then, depending on
/// the "cursor mode" flag, either the cursor moves within the list (and the
/// visible top follows it) or the visible top scrolls and the cursor stays
/// put. Returns 1 when a key was consumed.
pub fn cuit_ext_scroll_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003a_1400);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x003a_12d0, 0x003a_1400)?; // cUITextScroll_GetFont
        let self_ = c.g[S0][0] as u32;
        let input = c.g[S1][0] as u32;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
            c.cycles += 6;
            frame.restore(c, 16, 0x30);
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        let font = c.g[V0][0] as u32;

        'ret: {
            'ret0: {
                'ret1: {
                    'scroll: {
                        c.cycles += 18;
                        c.lwc1(2, p_9(font, 0x14));
                        c.cvt_s_w(2, 2);
                        c.lwc1(3, p_9(font, 0x34));
                        c.lwc1(0, p_9(self_, 0x54));
                        c.g[AT][0] = 0x4040_0000; // 3.0
                        c.mtc1(1, 4);
                        c.mul_s(2, 2, 3);
                        c.lwc1(1, p_9(self_, 0x64));
                        c.g[V1][0] = c.lw(p_9(self_, 0x74));
                        c.mul_s(0, 0, 2);
                        c.g[V1][0] = c.g[V1][0] & 1;
                        c.add_s(0, 0, 4);
                        c.div_s(1, 1, 0);
                        c.cvt_w_s(0, 1);
                        c.mfc1(2, 0);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        let cursor_mode = c.g[V1][0] == 0;
                        c.g[S2][0] = sx((((c.g[V0][0] as u32) as i32) >> 24) as u32);
                        let rows = c.g[S2][0] as i32;

                        if cursor_mode {
                            break 'scroll;
                        }
                        // --- cursor mode: up ---------------------------------
                        c.cycles += 8;
                        c.g[V1][0] = c.lw(p_9(input, 8));
                        c.g[A1][0] = c.lb(p_9(self_, 0x88));
                        c.g[A0][0] = c.lh(p_9(c.g[V1][0] as u32, 0x10));
                        c.g[V0][0] = c.lw(p_9(c.g[V1][0] as u32, 0x14));
                        let f = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(input.wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, f, 0x003a_1468)?;
                        if c.g[V0][0] != 0 {
                            c.cycles += 3;
                            c.g[V0][0] = c.lhu(p_9(self_, 0xc4));
                            let empty = c.g[V0][0] == 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            if empty {
                                break 'ret1;
                            }
                            c.cycles += 2;
                            c.sh(p_9(self_, 0xc4), c.g[V0][0]);
                            break 'ret1;
                        }
                        // --- cursor mode: down -------------------------------
                        c.g[V1][0] = c.lw(p_9(input, 8));
                        c.cycles += 7;
                        c.g[A1][0] = c.lb(p_9(self_, 0x89));
                        c.g[A0][0] = c.lh(p_9(c.g[V1][0] as u32, 0x10));
                        c.g[V0][0] = c.lw(p_9(c.g[V1][0] as u32, 0x14));
                        let f = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(input.wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, f, 0x003a_1498)?;
                        if c.g[V0][0] != 0 {
                            c.cycles += 6;
                            c.g[V0][0] = c.lhu(p_9(self_, 0xc8));
                            c.g[V1][0] = c.lhu(p_9(self_, 0xc4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let last = c.g[V0][0] == 0;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                            if last {
                                break 'ret1;
                            }
                            c.cycles += 2;
                            c.sh(p_9(self_, 0xc4), c.g[V0][0]);
                            break 'ret1;
                        }
                        // Neither key: keep the cursor visible.
                        c.g[V1][0] = c.lhu(p_9(self_, 0xc4));
                        c.cycles += 6;
                        c.g[V0][0] = c.lhu(p_9(self_, 0xc6));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                        c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 16) as u32);
                        let above = (c.g[V0][0] as i64) >= 0;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                        if !above {
                            c.cycles += 2;
                            c.sh(p_9(self_, 0xc6), c.g[V1][0]);
                            break 'ret0;
                        }
                        c.cycles += 2;
                        let visible = c.g[V0][0] != 0;
                        c.g[V0][0] = 0;
                        if visible {
                            break 'ret;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                        c.sh(p_9(self_, 0xc6), c.g[V0][0]);
                        let _ = rows;
                        break 'ret0;
                    }
                    // --- scroll mode: up ------------------------------------
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(p_9(input, 8));
                    c.g[A1][0] = c.lb(p_9(self_, 0x88));
                    c.g[A0][0] = c.lh(p_9(c.g[V1][0] as u32, 0x10));
                    c.g[V0][0] = c.lw(p_9(c.g[V1][0] as u32, 0x14));
                    let f = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(input.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, f, 0x003a_1510)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 3;
                        c.g[V0][0] = c.lhu(p_9(self_, 0xc6));
                        let empty = c.g[V0][0] == 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                        if empty {
                            break 'ret1;
                        }
                        c.cycles += 2;
                        c.sh(p_9(self_, 0xc6), c.g[V0][0]);
                        break 'ret1;
                    }
                    // --- scroll mode: down ----------------------------------
                    c.g[V1][0] = c.lw(p_9(input, 8));
                    c.cycles += 7;
                    c.g[A1][0] = c.lb(p_9(self_, 0x89));
                    c.g[A0][0] = c.lh(p_9(c.g[V1][0] as u32, 0x10));
                    c.g[V0][0] = c.lw(p_9(c.g[V1][0] as u32, 0x14));
                    let f = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(input.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, f, 0x003a_1540)?;
                    let down = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if down {
                        break 'ret;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lhu(p_9(self_, 0xc8));
                    c.g[V1][0] = c.lhu(p_9(self_, 0xc6));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let full = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    if full {
                        break 'ret1;
                    }
                    c.cycles += 1;
                    c.sh(p_9(self_, 0xc6), c.g[V0][0]);
                    break 'ret1;
                }
                // Return 1.
                c.cycles += 2;
                c.g[V0][0] = 1;
                break 'ret;
            }
            // Return 0.
            c.cycles += 1;
            c.g[V0][0] = 0;
            break 'ret;
        }

        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------------------ UIText_Render

/// `cUIText` fields read by `UIText_Render`.
pub(crate) const UI_TEXT_RENDER_VTABLE: u32 = 0x08;

/// Flags word: bit 2 "string built", bit 3 "ignore the off colour", bit 5
/// "highlight allowed"; byte +0x15 bits 0..4: the draw layer.
pub(crate) const UI_TEXT_RENDER_FLAGS14: u32 = 0x14;

pub(crate) const UI_TEXT_RENDER_COLOR: u32 = 0x1c;

pub(crate) const UI_TEXT_RENDER_X: u32 = 0x44;

pub(crate) const UI_TEXT_RENDER_Y: u32 = 0x48;

pub(crate) const UI_TEXT_RENDER_SCALE_X: u32 = 0x50;

pub(crate) const UI_TEXT_RENDER_SCALE_Y: u32 = 0x54;

pub(crate) const UI_TEXT_RENDER_OWNER: u32 = 0x5c;

pub(crate) const UI_TEXT_RENDER_WIDTH: u32 = 0x60;

/// Flags: bit 2 shadow, bit 4 highlighted, bit 5 off, bit 6 formatted lines.
pub(crate) const UI_TEXT_RENDER_FLAGS: u32 = 0x74;

pub(crate) const UI_TEXT_RENDER_SHADOW_COLOR: u32 = 0x78;

pub(crate) const UI_TEXT_RENDER_HI_COLOR: u32 = 0x88;

pub(crate) const UI_TEXT_RENDER_OFF_COLOR: u32 = 0x98;

pub(crate) const UI_TEXT_RENDER_NUM_LINES: u32 = 0xa8;

pub(crate) const UI_TEXT_RENDER_LINES: u32 = 0xac;

pub(crate) const UI_TEXT_RENDER_STR: u32 = 0xb4;

/// `sFont` fields.
pub(crate) const UI_TEXT_RENDER_FONT_HEIGHT: u32 = 0x14;

pub(crate) const UI_TEXT_RENDER_FONT_SHADOW_OFS: u32 = 0x28;

pub(crate) const UI_TEXT_RENDER_FONT_SCALE: u32 = 0x30;

pub(crate) const UI_TEXT_RENDER_FONT_CUR: u32 = 0x38;

pub(crate) const UI_TEXT_RENDER_FONT_COLOR: u32 = 0x40;

pub(crate) const UI_TEXT_RENDER_FONT_SHADOW_COLOR: u32 = 0x50;

/// `D_00508750`: the 0x400-char wide string of 'W's measuring a box.
pub(crate) const UI_TEXT_RENDER_W_BUF: u32 = 0x0050_8750;

/// `gp - 0x854`: the graphics manager (`GrMan`).
pub(crate) const UI_TEXT_RENDER_GRMAN_GP: u32 = 0x854;

/// `gp + 0x2970` (`D_004A5A60`): the default shadow offset.
pub(crate) const UI_TEXT_RENDER_SHADOW_OFS_GP: u32 = 0x2970;

pub(crate) const UI_TEXT_RENDER_GET_FONT: Callee = Callee {
    addr: 0x003a_04f0,
    f: ee::s13::f_003a04f0,
};

pub(crate) const UI_TEXT_RENDER_GET_CHAR_WIDTH: Callee = Callee {
    addr: 0x0039_2ba8,
    f: ee::s13::f_00392ba8,
};

pub(crate) const UI_TEXT_RENDER_APPLY_ALIGNMENT: Callee = Callee {
    addr: 0x0039_feb8,
    f: ee::s13::f_0039feb8,
};

pub(crate) const UI_TEXT_RENDER_USTR_LEN: u32 = 0x003d_bca0;

pub(crate) const UI_TEXT_RENDER_OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;

pub(crate) const UI_TEXT_RENDER_USTR_SPRINTF_ASCII_FMT: u32 = 0x002c_2718;

pub(crate) const UI_TEXT_RENDER_FONT_MEASURE: u32 = 0x0039_21f0;

pub(crate) const UI_TEXT_RENDER_FONT_PRINT: u32 = 0x0039_1e30;

pub(crate) const UI_TEXT_RENDER_EA_MEM_FREE: u32 = 0x0031_7e98;

/// Bit `n` of a word, as the `sra; andi 1` pair leaves it.
#[inline]
pub(crate) fn ui_text_render_bit(w: u64, n: u32) -> u64 {
    ((w as u32 >> n) & 1) as u64
}

/// Copy 16 bytes (two unaligned doublewords through registers `r0`, `r1`:
/// `ldl/ldr` both, then `sdl/sdr` both).
pub(crate) fn ui_text_render_copy16(c: &mut Cpu, r0: usize, r1: usize, src: u32, dst: u32) {
    c.g[r0][0] = Addr(src).read_u64_unaligned(c);
    c.g[r1][0] = Addr(src.wrapping_add(8)).read_u64_unaligned(c);
    let v = c.g[r0][0];
    Addr(dst).write_u64_unaligned(c, v);
    let v = c.g[r1][0];
    Addr(dst.wrapping_add(8)).write_u64_unaligned(c, v);
}

/// `void cUIText_render2D(cUIText* self, float x, float y)` (0x3a05e8):
/// draws a UI text element at its position plus (x, y). Rebuilds the string
/// first (vtable slot 16) unless it is already built; if the owner's
/// override flag (bit 27) is set, fills the 'W' buffer to measure the box
/// width. Then sets the font scale, the colour (highlight, off or normal),
/// the shadow and the draw layer, and draws each line (measure, align,
/// print), advancing y by the font height plus 3. A formatted text with
/// several lines is formatted into a temporary buffer and drawn once.
pub fn ui_text_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xf0);
        frame.save(c, S1, 0xc0);
        frame.save_fpr(c, 21, 0xe8);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0xe0);
        c.f[21] = c.f[13];
        frame.save(c, S0, 0xd0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S4, 0x90);
        frame.save(c, S5, 0x80);
        frame.save(c, S6, 0x70);
        frame.save_ra(c, 0x60);
        c.f[20] = c.f[12];
        ui_text_render_body(c, frame)?;
        c.cycles += 11;
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 21, 0xe8);
        frame.restore_fpr(c, 20, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// Everything between the prologue and the epilogue (every exit has
/// restored `s0` already).
pub(crate) fn ui_text_render_body(c: &mut Cpu, frame: Frame) -> Flow {
    let this = |c: &Cpu| Addr(c.g[S1][0] as u32);
    let font = |c: &Cpu| Addr(c.g[S0][0] as u32);

    // Rebuild the string unless it is there and built.
    c.g[V0][0] = this(c).off(UI_TEXT_RENDER_STR).lw(c);
    let mut built = false;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        let flags = this(c).off(UI_TEXT_RENDER_FLAGS14).lw(c);
        c.g[V0][0] = ui_text_render_bit(flags, 2);
        built = c.g[V0][0] != 0;
    }
    if !built {
        c.cycles += 6;
        let vt = this(c).off(UI_TEXT_RENDER_VTABLE).lw(c);
        c.g[V0][0] = vt;
        let slot = VSlot::read(c, vt, 0x80);
        c.g[V1][0] = slot.func;
        c.g[A0][0] = slot.this(c.g[S1][0]);
        call_ptr(c, slot.func as u32, 0x003a_0650)?;
    }
    c.g[V0][0] = this(c).off(UI_TEXT_RENDER_OWNER).lw(c);

    // The owner's override: fill the 'W' buffer to the box width.
    c.cycles += 5;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xd0).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
    c.g[S4][0] = Addr(c.g[A0][0] as u32).off(0x10).lw(c);
    'measure: {
        if c.g[S4][0] == 0 {
            break 'measure;
        }
        c.cycles += 5;
        let ov = Addr(c.g[S4][0] as u32).lw(c);
        c.g[V0][0] = ui_text_render_bit(ov, 27);
        if c.g[V0][0] == 0 {
            break 'measure;
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        UI_TEXT_RENDER_GET_FONT.call(c, 0x003a_0684)?;
        c.g[A0][0] = c.g[V0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0xd0);
            return Ok(());
        }
        c.cycles += 12;
        c.g[A1][0] = 0x57;
        UI_TEXT_RENDER_GET_CHAR_WIDTH.call(c, 0x003a_0694)?;
        let scale_x = this(c).off(UI_TEXT_RENDER_SCALE_X).read_fpu(c);
        c.g[A0][0] = 0;
        let width = this(c).off(UI_TEXT_RENDER_WIDTH).read_fpu(c);
        let char_w = FpuF32(c.f[0]) * scale_x;
        let q = width.div(char_w);
        let n = q.to_i32();
        c.f[2] = scale_x.0;
        c.f[1] = q.0;
        c.f[0] = n as u32;
        c.g[A1][0] = sx(n as u32);
        c.g[V0][0] = (n < 1024) as u64;
        c.g[V1][0] = 0x51_0000;
        if n < 1024 {
            c.cycles += 2;
            c.g[V0][0] = sx(UI_TEXT_RENDER_W_BUF);
            if n > 0 {
                c.cycles += 3;
                c.g[A2][0] = 0x57;
                c.g[A0][0] = c.g[A1][0];
                loop {
                    c.cycles += 7;
                    let w = c.g[A2][0];
                    Addr(c.g[V0][0] as u32).sh(c, w);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(2));
                    if c.g[A0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_06d0);
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[A1][0];
            }
        }
        c.cycles += 5;
        c.g[V1][0] = sx(UI_TEXT_RENDER_W_BUF);
        c.g[V0][0] = sx(((c.g[A0][0] as u32) << 1).wrapping_add(UI_TEXT_RENDER_W_BUF));
        Addr(c.g[V0][0] as u32).sh(c, 0);
    }
    c.g[V0][0] = this(c).off(UI_TEXT_RENDER_STR).lw(c);

    c.cycles += 2;
    frame.restore(c, S0, 0xd0);
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S1][0];
    UI_TEXT_RENDER_GET_FONT.call(c, 0x003a_0714)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] == 0 {
        frame.restore(c, S0, 0xd0);
        return Ok(());
    }

    // Font scale: s = (sx, sy) kept at sp+0.
    c.cycles += 16;
    let spa = sp(c);
    let scale_x = this(c).off(UI_TEXT_RENDER_SCALE_X).read_fpu(c);
    let scale_y = this(c).off(UI_TEXT_RENDER_SCALE_Y).read_fpu(c);
    Addr(spa).write_fpu(c, scale_x);
    Addr(spa + 4).write_fpu(c, scale_y);
    let font_sx = font(c).off(UI_TEXT_RENDER_FONT_SCALE).read_fpu(c);
    let font_sy = font(c).off(UI_TEXT_RENDER_FONT_SCALE + 4).read_fpu(c);
    let cur_x = font_sx * scale_x;
    font(c).off(UI_TEXT_RENDER_FONT_CUR).write_fpu(c, cur_x);
    let s_y = Addr(spa + 4).read_fpu(c);
    let cur_y = font_sy * s_y;
    font(c).off(UI_TEXT_RENDER_FONT_CUR + 4).write_fpu(c, cur_y);
    c.f[3] = scale_x.0;
    c.f[1] = s_y.0;
    c.f[0] = cur_x.0;
    c.f[2] = cur_y.0;

    // Colour: highlight, off or normal.
    let flags14 = this(c).off(UI_TEXT_RENDER_FLAGS14).lw(c);
    c.g[V0][0] = ui_text_render_bit(flags14, 5);
    'color: {
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            let flags = this(c).off(UI_TEXT_RENDER_FLAGS).lw(c);
            c.g[V0][0] = ui_text_render_bit(flags, 4);
            if c.g[V0][0] != 0 {
                c.cycles += 10;
                let (src, dst) = (
                    this(c).off(UI_TEXT_RENDER_HI_COLOR).0,
                    font(c).off(UI_TEXT_RENDER_FONT_COLOR).0,
                );
                ui_text_render_copy16(c, V0, V1, src, dst);
                break 'color;
            }
        }
        c.cycles += 4;
        let flags14 = this(c).off(UI_TEXT_RENDER_FLAGS14).lw(c);
        c.g[V0][0] = ui_text_render_bit(flags14, 3);
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            let flags = this(c).off(UI_TEXT_RENDER_FLAGS).lw(c);
            c.g[V0][0] = ui_text_render_bit(flags, 5);
            if c.g[V0][0] != 0 {
                c.cycles += 10;
                let (src, dst) = (
                    this(c).off(UI_TEXT_RENDER_OFF_COLOR).0,
                    font(c).off(UI_TEXT_RENDER_FONT_COLOR).0,
                );
                ui_text_render_copy16(c, A3, T0, src, dst);
                break 'color;
            }
        }
        c.cycles += 9;
        let (src, dst) = (
            this(c).off(UI_TEXT_RENDER_COLOR).0,
            font(c).off(UI_TEXT_RENDER_FONT_COLOR).0,
        );
        ui_text_render_copy16(c, A0, A1, src, dst);
    }

    // Shadow: the shadow colour with the text's red, and the default offset.
    let flags = this(c).off(UI_TEXT_RENDER_FLAGS).lw(c);
    c.cycles += 4;
    c.g[V0][0] = ui_text_render_bit(flags, 2);
    let gp = c.g[GP][0] as u32;
    if c.g[V0][0] == 0 {
        Addr(spa + 0x10).sw(c, 0);
        c.cycles += 6;
        Addr(spa + 0x14).sw(c, 0);
        c.g[V0][0] = Addr(spa + 0x10).read_u64_unaligned(c);
        let zero = c.g[V0][0];
        let dst = font(c).off(UI_TEXT_RENDER_FONT_SHADOW_OFS);
        dst.write_u64_unaligned(c, zero);
    } else {
        c.cycles += 24;
        let red = this(c).off(UI_TEXT_RENDER_COLOR).read_fpu(c);
        c.f[0] = red.0;
        let src = this(c).off(UI_TEXT_RENDER_SHADOW_COLOR).0;
        ui_text_render_copy16(c, V0, V1, src, spa + 0x10);
        Addr(spa + 0x10).write_fpu(c, red);
        let dst = font(c).off(UI_TEXT_RENDER_FONT_SHADOW_COLOR).0;
        ui_text_render_copy16(c, A3, T0, spa + 0x10, dst);
        c.g[A0][0] = Addr(gp.wrapping_add(UI_TEXT_RENDER_SHADOW_OFS_GP)).read_u64_unaligned(c);
        let ofs = c.g[A0][0];
        let dst = font(c).off(UI_TEXT_RENDER_FONT_SHADOW_OFS);
        dst.write_u64_unaligned(c, ofs);
    }
    c.g[V1][0] = Addr(gp.wrapping_sub(UI_TEXT_RENDER_GRMAN_GP)).lw(c);

    // Draw layer (bits 5..9 of the draw state) and the start position.
    c.cycles += 21;
    c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
    let layer = this(c).off(UI_TEXT_RENDER_FLAGS14 + 1).lbu(c);
    c.g[S2][0] = 0;
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0xe84).lw(c);
    let state = Addr(c.g[A0][0] as u32).off(8).lw(c);
    c.g[V0][0] = (((layer & 0x1f) + 9) << 5) & 0x3e0;
    c.g[V1][0] = (state & c.g[A1][0]) | c.g[V0][0];
    let v = c.g[V1][0];
    Addr(c.g[A0][0] as u32).off(8).sw(c, v);
    Addr(spa + 0x18).sw(c, 0);
    let pos_x = this(c).off(UI_TEXT_RENDER_X).read_fpu(c) + FpuF32(c.f[20]);
    let y = this(c).off(UI_TEXT_RENDER_Y).read_fpu(c);
    c.g[V0][0] = this(c).off(UI_TEXT_RENDER_NUM_LINES).lw(c);
    let pos_y = y + FpuF32(c.f[21]);
    c.f[0] = pos_x.0;
    c.f[1] = pos_y.0;
    Addr(spa + 0x10).write_fpu(c, pos_x);
    Addr(spa + 0x14).write_fpu(c, pos_y);
    if (c.g[V0][0] as i64) <= 0 {
        c.cycles += 1;
        frame.restore(c, S0, 0xd0);
        return Ok(());
    }
    c.cycles += 4;
    c.g[AT][0] = 0x4040_0000;
    c.f[20] = 0x4040_0000; // 3.0: the line gap
    c.g[S5][0] = 0x49_0000;
    c.g[S6][0] = 0x4a_0000;

    loop {
        // The line: the string for line 0, else lines[i].
        c.cycles += 2;
        let i = c.g[S2][0] as u32;
        c.g[V0][0] = sx(i << 2);
        if i == 0 {
            c.cycles += 1;
            c.g[S3][0] = this(c).off(UI_TEXT_RENDER_STR).lw(c);
        } else {
            c.cycles += 4;
            c.g[V1][0] = this(c).off(UI_TEXT_RENDER_LINES).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S3][0] = Addr(c.g[V0][0] as u32).lw(c);
        }
        c.cycles += 2;
        if c.g[S4][0] != 0 {
            c.cycles += 5;
            let ov = Addr(c.g[S4][0] as u32).lw(c);
            let over = ui_text_render_bit(ov, 27) != 0;
            c.g[V0][0] = 0x51_0000;
            if over {
                c.cycles += 1;
                c.g[S3][0] = sx(UI_TEXT_RENDER_W_BUF);
            }
            c.cycles += 1;
        }
        let flags = this(c).off(UI_TEXT_RENDER_FLAGS).lw(c);
        c.cycles += 4;
        c.g[V0][0] = ui_text_render_bit(flags, 6);
        c.f[12] = Addr(spa).read_u32(c);
        let formatted = if c.g[V0][0] == 0 {
            c.cycles += 1;
            false
        } else {
            c.cycles += 4;
            let lines = this(c).off(UI_TEXT_RENDER_NUM_LINES).lw(c);
            c.g[V0][0] = ((lines as i64) < 2) as u64;
            c.g[V0][0] == 0
        };

        if formatted {
            // Format the line into a temporary buffer, draw it, done.
            c.cycles += 57;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, UI_TEXT_RENDER_USTR_LEN, 0x003a_0960)?;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(5));
            c.g[A3][0] = 0;
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4078));
            c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[A2][0] = 0x100;
            call_ptr(c, UI_TEXT_RENDER_OPERATOR_NEW_NAMED, 0x003a_0978)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x47b0));
            c.g[A2][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, UI_TEXT_RENDER_USTR_SPRINTF_ASCII_FMT, 0x003a_098c)?;
            let spa = sp(c);
            c.f[12] = Addr(spa).read_u32(c);
            c.g[A2][0] = sx(spa + 0x20);
            c.f[13] = Addr(spa + 4).read_u32(c);
            c.g[A3][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, UI_TEXT_RENDER_FONT_MEASURE, 0x003a_09a8)?;
            ui_text_render_align(c, 0x40, 0x003a_09f8)?;
            ui_text_render_print(c, 0x40, S2, 0x003a_0a34)?;
            frame.restore(c, S0, 0xd0);
            if c.g[S2][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, UI_TEXT_RENDER_EA_MEM_FREE, 0x003a_0a44)?;
                frame.restore(c, S0, 0xd0);
            }
            return Ok(());
        }

        c.g[A2][0] = sx(spa + 0x20);
        c.cycles += 45;
        c.f[13] = Addr(spa + 4).read_u32(c);
        c.g[A3][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, UI_TEXT_RENDER_FONT_MEASURE, 0x003a_0a64)?;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
        ui_text_render_align(c, 0x50, 0x003a_0ab8)?;
        ui_text_render_print(c, 0x50, S3, 0x003a_0af4)?;
        c.g[V0][0] = this(c).off(UI_TEXT_RENDER_NUM_LINES).lw(c);
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0xd0);
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_08f8);
        }
    }
}

/// After measuring the line into the rect at sp+0x20: the box size
/// (rect.x1, sy * height * font scale y) at sp+0x30, a copy of the pen
/// position (sp+0x10) at `p` (sp-relative), and `UIObject_ApplyAlignment
/// (self, &p, &size)`.
pub(crate) fn ui_text_render_align(c: &mut Cpu, p: u32, ret: u32) -> Flow {
    let spa = sp(c);
    let font = Addr(c.g[S0][0] as u32);
    let height = FpuF32::from_i32(font.off(UI_TEXT_RENDER_FONT_HEIGHT).read_u32(c) as i32);
    let font_sy = font.off(UI_TEXT_RENDER_FONT_SCALE + 4).read_fpu(c);
    let sy = Addr(c.g[S1][0] as u32)
        .off(UI_TEXT_RENDER_SCALE_Y)
        .read_fpu(c);
    c.g[A1][0] = sx(spa + p);
    let x1 = Addr(spa + 0x28).read_fpu(c);
    c.g[A0][0] = c.g[S1][0];
    let line_h = height * font_sy;
    Addr(spa + 0x38).sw(c, 0);
    Addr(spa + 0x30).write_fpu(c, x1);
    let size_y = sy * line_h;
    Addr(spa + 0x34).write_fpu(c, size_y);
    c.f[1] = line_h.0;
    c.f[3] = font_sy.0;
    c.f[2] = x1.0;
    c.f[0] = size_y.0;
    c.g[A3][0] = Addr(spa + 0x10).read_u64_unaligned(c);
    c.g[T0][0] = Addr(spa + 0x18).lw(c);
    let pos = c.g[A3][0];
    Addr(spa + p).write_u64_unaligned(c, pos);
    let z = c.g[T0][0];
    Addr(spa + p + 8).sw(c, z);
    c.g[A2][0] = sx(spa + 0x30);
    UI_TEXT_RENDER_APPLY_ALIGNMENT.call(c, ret)
}

/// Advance the pen by `height * font scale y * s.y + 3` and print the
/// string in register `text` at the aligned position at `p` (sp-relative).
pub(crate) fn ui_text_render_print(c: &mut Cpu, p: u32, text: usize, ret: u32) -> Flow {
    let spa = sp(c);
    let font = Addr(c.g[S0][0] as u32);
    let height = FpuF32::from_i32(font.off(UI_TEXT_RENDER_FONT_HEIGHT).read_u32(c) as i32);
    let font_sy = font.off(UI_TEXT_RENDER_FONT_SCALE + 4).read_fpu(c);
    let s_y = Addr(spa + 4).read_fpu(c);
    c.g[A0][0] = c.g[S0][0];
    let pen_y = Addr(spa + 0x14).read_fpu(c);
    c.g[A1][0] = c.g[text][0];
    let step = height * font_sy;
    c.f[12] = Addr(spa + p).read_u32(c);
    c.f[13] = Addr(spa + p + 4).read_u32(c);
    let step = step * s_y + FpuF32(c.f[20]);
    let pen_y = pen_y + step;
    c.f[0] = step.0;
    c.f[1] = pen_y.0;
    c.f[2] = font_sy.0;
    c.f[3] = s_y.0;
    Addr(spa + 0x14).write_fpu(c, pen_y);
    call_ptr(c, UI_TEXT_RENDER_FONT_PRINT, ret)
}

pub(crate) mod oc445_callee {
    use super::*;

    /// `UIText_FreeStrings(UIText *)`.
    pub(crate) const UI_TEXT_FREE_STRINGS: Callee = Callee {
        addr: 0x003a_0b38,
        f: ee::s13::f_003a0b38,
    };

    /// `UIText_CountLines(UIText *)`.
    pub(crate) const UI_TEXT_COUNT_LINES: Callee = Callee {
        addr: 0x003a_0b90,
        f: ee::s13::f_003a0b90,
    };

    /// `UIScreen_CreateObject(screen, rec)`.
    pub(crate) const UI_SCREEN_CREATE_OBJECT: Callee = Callee {
        addr: 0x0039_d078,
        f: ee::s13::f_0039d078,
    };

    /// `UIList_PushBack(list, node)`.
    pub(crate) const UI_LIST_PUSH_BACK: Callee = Callee {
        addr: 0x0039_7910,
        f: ee::s13::f_00397910,
    };

    /// `UIList_GetAt(list, index)`.
    pub(crate) const UI_LIST_GET_AT: Callee = Callee {
        addr: 0x0039_7870,
        f: ee::s13::f_00397870,
    };

    /// `UIMenu_SetCurrentItem(menu, item, flag)`.
    pub(crate) const UI_MENU_SET_CURRENT_ITEM: Callee = Callee {
        addr: 0x0039_ad38,
        f: ee::s13::f_0039ad38,
    };

    /// `EATimer_SetVideoMode(mode)`.
    pub(crate) const EA_TIMER_SET_VIDEO_MODE: Callee = Callee {
        addr: 0x003e_6690,
        f: ee::s14::f_003e6690,
    };

    /// `EA_AddShutdownHandler(fn)`.
    pub(crate) const EA_ADD_SHUTDOWN_HANDLER: Callee = Callee {
        addr: 0x003e_4a30,
        f: ee::s14::f_003e4a30,
    };
}

// ---------------------------------------------------------------------------
// Local helpers (register-relative loads, vtable calls).

/// `lw off(reg)`.
#[inline]
pub(crate) fn oc445_lw(c: &mut Cpu, base: usize, off: u32) -> u64 {
    Addr((c.g[base][0] as u32).wrapping_add(off)).lw(c)
}

/// `lh off(reg)`.
#[inline]
pub(crate) fn oc445_lh(c: &mut Cpu, base: usize, off: u32) -> u64 {
    Addr((c.g[base][0] as u32).wrapping_add(off)).lh(c)
}

/// `lhu off(reg)`.
#[inline]
pub(crate) fn oc445_lhu(c: &mut Cpu, base: usize, off: u32) -> u64 {
    Addr((c.g[base][0] as u32).wrapping_add(off)).lhu(c)
}

/// `addiu`-style 32-bit add of a constant to a register, sign-extended.
#[inline]
pub(crate) fn oc445_add(c: &Cpu, reg: usize, k: u32) -> u64 {
    sx((c.g[reg][0] as u32).wrapping_add(k))
}

/// `addu a, b` of two registers, sign-extended.
#[inline]
pub(crate) fn oc445_addu(c: &Cpu, a: usize, b: usize) -> u64 {
    sx((c.g[a][0] as u32).wrapping_add(c.g[b][0] as u32))
}

/// A gcc 2.9x virtual call: `A0 = lh off(vt)`, `func = lw off+4(vt)`,
/// `A0 = obj + A0`, `jalr func` returning to `ret`. `vt`, `obj` and `func`
/// name registers.
pub(crate) fn oc445_vcall(
    c: &mut Cpu,
    vt: usize,
    obj: usize,
    off: u32,
    func: usize,
    ret: u32,
) -> Flow {
    c.g[A0][0] = oc445_lh(c, vt, off);
    c.g[func][0] = oc445_lw(c, vt, off + 4);
    let target = c.g[func][0] as u32;
    c.g[A0][0] = oc445_addu(c, obj, A0);
    call_ptr(c, target, ret)
}

// ---------------------------------------------------------------------------
// UIText_SetWideStringInternal / UIText_SetTextId.

/// `void UIText_SetWideStringInternal(UIText *text, const u16 *str)`
/// (0x3a0eb0): replace the text. Calls vtable slots 0x28 and 0xc0 with 1,
/// frees the old strings, and when `str` has at most 800 units copies it
/// into a new buffer (`+0xb4`, size in units at `+0xb8`). When it holds
/// several lines (`+0xa8` count, from `UIText_CountLines`) a line-pointer
/// array (`+0xac`, size `+0xba`) is built: a pair of backslashes or a CR
/// ends a line with a zero unit and the next line starts after it.
pub fn ui_text_set_wide_string_internal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 0x1;
        c.g[V0][0] = oc445_lw(c, S0, 0x8);
        oc445_vcall(c, V0, S0, 0x28, V1, 0x003a_0ee0)?;
        c.g[V0][0] = oc445_lw(c, S0, 0x8);
        c.g[A1][0] = 0x1;
        oc445_vcall(c, V0, S0, 0xc0, V1, 0x003a_0ef8)?;
        c.g[A0][0] = c.g[S0][0];
        oc445_callee::UI_TEXT_FREE_STRINGS.call(c, 0x003a_0f00)?;
        c.g[A0][0] = c.g[S1][0];
        // USTR_length(str)
        call_ptr(c, 0x003d_bca0, 0x003a_0f08)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 801) as u64;
        let too_long = c.g[V0][0] == 0;
        c.g[V0][0] = oc445_add(c, V1, 0x1);
        'copy: {
            if too_long {
                break 'copy;
            }
            c.cycles += 17;
            c.g[A1][0] = 0x490000;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[A1][0] = oc445_add(c, A1, 0x4088);
            Addr(oc445_add(c, S0, 0xb8) as u32).sh(c, c.g[V0][0]);
            // operator new((len + 1) * 2, "UITextUStr", 0x100, 0)
            call_ptr(c, 0x0031_7e30, 0x003a_0f34)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            Addr(oc445_add(c, S0, 0xb4) as u32).sw(c, c.g[V0][0]);
            // USTR_copy(dst, src)
            call_ptr(c, 0x003d_be20, 0x003a_0f44)?;
            c.g[A0][0] = c.g[S0][0];
            oc445_callee::UI_TEXT_COUNT_LINES.call(c, 0x003a_0f4c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 2) as u64;
            let single_line = c.g[V0][0] != 0;
            Addr(oc445_add(c, S0, 0xa8) as u32).sw(c, c.g[A0][0]);
            if single_line {
                break 'copy;
            }
            c.cycles += 16;
            c.g[A1][0] = 0x490000;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A1][0] = oc445_add(c, A1, 0x4098);
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            // operator new(lines * 4, "UITextLine", 0x100, 0)
            call_ptr(c, 0x0031_7e30, 0x003a_0f74)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V1][0] = oc445_lw(c, S0, 0xb4);
            c.g[V0][0] = oc445_lhu(c, S0, 0xa8);
            Addr(oc445_add(c, S0, 0xac) as u32).sw(c, c.g[A1][0]);
            c.g[A0][0] = c.g[V1][0];
            Addr(oc445_add(c, S0, 0xba) as u32).sh(c, c.g[V0][0]);
            Addr(c.g[A1][0] as u32).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lhu(c);
            let empty = c.g[V0][0] == 0;
            frame.restore(c, 16, 0x20);
            if empty {
                // Straight to the epilogue: the `lq s0` was the delay slot,
                // the "+1 cycle, lq s0" of the common exit is skipped.
                c.cycles += 4;
                frame.restore(c, 17, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 3;
            c.g[A2][0] = oc445_add(c, A1, 0x4);
            c.g[A3][0] = 0xd;
            c.g[V1][0] = Addr(c.g[A0][0] as u32).lhu(c);
            loop {
                c.cycles += 3;
                c.g[V0][0] = oc445_lhu(c, GP, 0x16b8);
                let not_backslash = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = oc445_lhu(c, A0, 0x2);
                // A line ends here and the next one is recorded (`record`),
                // or the unit just advances (`skip`).
                let mut record = false;
                let mut skip = false;
                if !not_backslash {
                    c.cycles += 2;
                    let lone = c.g[A1][0] != c.g[V1][0];
                    if !lone {
                        // Two backslashes: end the line, skip both.
                        c.cycles += 3;
                        Addr(c.g[A0][0] as u32).sh(c, 0);
                        c.g[A0][0] = oc445_add(c, A0, 0x4);
                        record = true;
                    }
                }
                if !record {
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[A3][0] {
                        c.g[A0][0] = oc445_add(c, A0, 0x2);
                        skip = true;
                    } else {
                        // CR: end the line.
                        c.cycles += 2;
                        Addr(c.g[A0][0] as u32).sh(c, 0);
                        c.g[A0][0] = oc445_add(c, A0, 0x2);
                    }
                }
                if !skip {
                    c.cycles += 2;
                    Addr(c.g[A2][0] as u32).sw(c, c.g[A0][0]);
                    c.g[A2][0] = oc445_add(c, A2, 0x4);
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lhu(c);
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = Addr(c.g[A0][0] as u32).lhu(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_0fa8);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 1;
        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIText_SetTextId(UIText *text, int id)` (0x3a1000): remember the
/// text id (`+0xb0`) and call the text's vtable slot 0x28 with `a1 = 0`.
pub fn ui_text_set_text_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Addr(oc445_add(c, A0, 0xb0) as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = oc445_lw(c, A0, 0x8);
        c.g[A1][0] = 0;
        c.g[V1][0] = oc445_lh(c, V0, 0x28);
        c.g[A2][0] = oc445_lw(c, V0, 0x2c);
        let target = c.g[A2][0] as u32;
        c.g[A0][0] = oc445_addu(c, A0, V1);
        call_ptr(c, target, 0x003a_1024)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The address `gp + off` (a 32-bit guest pointer).
pub(crate) fn spch_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off)
}

/// The address `base + off`, for a register value `base`.
pub(crate) fn spch_ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `void cUIText_setUnicodeStringWrap(cUIText* self, const u16* str)`
/// (0x3a0d00): sets the wide string and wraps it. It asks the text's font
/// (`UIText_GetFont`, through the object's vtable slot +0x2c), and when there
/// is one, copies `str` (`USTR_copy`) into a fresh buffer of `len + 1`
/// halfwords (`operator_new_named`), breaking the copy into lines: each
/// glyph's advance (`Font_GetCharWidthW`, scaled by the object's +0x50 and
/// +0x60 floats) is summed, and a line break is taken at a space (0x20) or
/// at the width limit. The lines are stored with `UIText_SetWideStringInternal`
/// and the buffer is freed afterwards.
pub fn cui_text_set_unicode_string_wrap(c: &mut Cpu) {
    guest(c, cui_text_set_unicode_string_wrap_body);
}

pub(crate) fn cui_text_set_unicode_string_wrap_body(c: &mut Cpu) -> Flow {
    c.cycles += 24;
    let frame = Frame::open(c, 0xa0);
    frame.save(c, S0, 0x80);
    frame.save(c, S3, 0x50);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S6, 0x20);
    c.g[S3][0] = c.g[A0][0];
    frame.save(c, S1, 0x70);
    c.g[A1][0] = 1;
    frame.save(c, S2, 0x60);
    frame.save(c, S4, 0x40);
    frame.save(c, S5, 0x30);
    frame.save(c, S7, 0x10);
    frame.save_ra(c, 0);
    frame.save_fpr(c, 20, 0x90);
    c.g[V0][0] = c.lw(spch_ea(c.g[S3][0], 0x8));
    c.g[A0][0] = c.lh(spch_ea(c.g[V0][0], 0x28));
    c.g[V1][0] = c.lw(spch_ea(c.g[V0][0], 0x2c));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(spch_ea(c.g[S3][0], c.g[A0][0] as u32));
    call_ptr(c, t, 0x003a_0d4c)?;
    c.g[A0][0] = c.g[S3][0];
    Callee {
        addr: 0x003a_04f0,
        f: ee::s13::f_003a04f0,
    }
    .call(c, 0x003a_0d54)?;
    c.g[S6][0] = c.g[V0][0];
    if c.g[S6][0] == 0 {
        frame.restore(c, S0, 0x80);
    } else {
        'b_3a0e44: {
            c.cycles += 19;
            c.g[A0][0] = c.g[S0][0];
            c.g[S4][0] = u64::MAX;
            call_ptr(c, 0x003d_bca0, 0x003a_0d6c)?;
            c.mtc1(0, 20);
            c.g[S5][0] = c.g[V0][0];
            c.g[A1][0] = 0x49_0000;
            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4078));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            c.g[S1][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x003a_0d94)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x003d_be20, 0x003a_0da4)?;
            let t = (c.g[S5][0] as i64) <= 0;
            c.g[S7][0] = 0xd;
            if t {
                break 'b_3a0e44;
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 1);
            loop {
                'b_3a0e38: {
                    'b_3a0de8: {
                        'b_3a0de4: {
                            c.cycles += 6;
                            c.g[V1][0] = c.lhu(spch_gp(c, 0x16b8));
                            c.g[S0][0] = c.g[V0][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                            let t = c.g[A0][0] != c.g[V1][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            if t {
                                break 'b_3a0de4;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lhu(spch_ea(c.g[V0][0], 0x2));
                            let t = c.g[V0][0] != c.g[A0][0];
                            c.g[A0][0] = c.g[S6][0];
                            if t {
                                break 'b_3a0de8;
                            }
                            c.cycles += 4;
                            c.g[S4][0] = u64::MAX;
                            c.mtc1(0, 20);
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                            break 'b_3a0e38;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S6][0];
                    }
                    'b_3a0e34: {
                        c.cycles += 12;
                        c.g[RA][0] = sx(0x003a_0df0);
                        c.g[A1][0] = c.lhu(c.g[S0][0] as u32);
                        Callee {
                            addr: 0x0039_2c60,
                            f: ee::s13::f_00392c60,
                        }
                        .call(c, 0x003a_0df0)?;
                        c.lwc1(1, spch_ea(c.g[S3][0], 0x50));
                        c.lwc1(2, spch_ea(c.g[S3][0], 0x60));
                        c.mul_s(0, 0, 1);
                        c.g[V0][0] = c.lhu(c.g[S0][0] as u32);
                        c.g[V0][0] ^= 0x20;
                        c.add_s(20, 20, 0);
                        c.c_lt_s(2, 20);
                        let t = !c.fcc();
                        if c.g[V0][0] == 0 {
                            c.g[S4][0] = c.g[S1][0];
                        }
                        if t {
                            break 'b_3a0e34;
                        }
                        c.cycles += 2;
                        let t = (c.g[S4][0] as i64) < 0;
                        c.g[V0][0] = sx((c.g[S4][0] as u32) << 1);
                        if t {
                            break 'b_3a0e34;
                        }
                        c.cycles += 5;
                        c.g[S1][0] = c.g[S4][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.mtc1(0, 20);
                        c.sh(c.g[V0][0] as u32, c.g[S7][0]);
                        c.g[S4][0] = u64::MAX;
                    }
                    c.cycles += 1;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S5][0] as i64)) as u64;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 1);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_0db0);
                    }
                    continue;
                }
                break 'b_3a0e44;
            }
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x003a_0eb0,
            f: ee::s13::f_003a0eb0,
        }
        .call(c, 0x003a_0e50)?;
        let t = c.g[S2][0] == 0;
        frame.restore(c, S0, 0x80);
        if !t {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003a_0e60);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0031_7e98, 0x003a_0e60)?;
            frame.restore(c, S0, 0x80);
        }
    }
    c.cycles += 11;
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore(c, S4, 0x40);
    frame.restore(c, S5, 0x30);
    frame.restore(c, S6, 0x20);
    frame.restore(c, S7, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x90);
    frame.close(c);
    Ok(())
}

/// `cUIText_getNumTextLines` (0x3a0b90): the number of lines in the text of a
/// UI text object (`self+0xb4`, a zero-terminated array of 16-bit characters).
/// A pair of equal line-break characters (`gp+0x16b8`) counts as one line, and
/// a carriage return (13) ends a line. Returns the count as a sign-extended
/// byte.
pub fn ui_text_count_lines(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xb4)).lw(c);
    let has_text = c.g[V0][0] != 0;
    c.g[V1][0] = c.g[V0][0];
    if !has_text {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
    c.g[A1][0] = 1;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x16b8)).lhu(c);
        c.g[A3][0] = 0xd;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
            let t = c.g[V0][0] != c.g[A2][0];
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(2)).lhu(c);
            let mut pair = false;
            if !t {
                c.cycles += 2;
                pair = c.g[A0][0] == c.g[V0][0];
            }
            if pair {
                // Two equal line-break characters: skip both, one line.
                c.cycles += 3;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                let carriage = c.g[V0][0] == c.g[A3][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(2));
                if carriage {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                }
            }
            // Loop test at the back edge.
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_0bb8);
            }
        }
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
    ret(c);
}

/// `void UIText_SetString(UIText* text, const char* s)` (0x3a0c50): clears the
/// text's cached-string word at +0xb0, then `UIText_SetAsciiString(text, s)`.
pub fn ui_text_set_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(r32(c, V0).wrapping_add(UI_TEXT_SET_STRING_CACHE)).sw(c, 0);
        Callee {
            addr: 0x003a_0c70,
            f: ee::s13::f_003a0c70,
        }
        .call(c, 0x003a_0c64)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The text's cached-string word cleared by `UIText_SetString`.
pub(crate) const UI_TEXT_SET_STRING_CACHE: u32 = 0xb0;

pub(crate) mod oc1077_callee {
    use super::ee;
    use super::Callee;

    /// `Render2D_DrawQuadUV(gfx, verts, rect, uv)`.
    pub(crate) const DRAW_QUAD_UV: Callee = Callee {
        addr: 0x0038_e5e8,
        f: ee::s12::Render2D_DrawQuadUV,
    };

    /// `Render2D_DrawQuadUVRot90(gfx, verts, rect, uv)`: same, UVs turned 90 degrees.
    pub(crate) const DRAW_QUAD_UV_ROT90: Callee = Callee {
        addr: 0x0038_e738,
        f: ee::s12::Render2D_DrawQuadUVRot90,
    };
}

/// `void Render2D_DrawTiledSprite(Sprite* o /*a0*/, Rect* dest /*a1*/, Colour* col /*a2*/,
/// u32 flags /*a3*/, float tx /*f12*/, float ty /*f13*/)` (0x3a23b8,
/// `cTextureElement::renderTiled`): draws the sprite's texture tiled `tx` times
/// across and `ty` times down `dest`. Sets the draw state's texture to `o->tex`,
/// builds the four colour vertices (a = 128 * alpha, r/g/b = 255 * channel) in a
/// stack template, then draws `ny = (int)ty` rows of `nx = (int)tx` whole tiles
/// (`w = dest.w / tx` wide, `h = dest.h / ty` high), a clipped column per row when
/// `tx` has a fraction, and a last clipped row when `ty` has one. `flags` bit 0
/// swaps U, bit 1 swaps V, bit 2 draws through the rot90 quad path (the UV
/// fractions then clip V instead of U). Does nothing when `tx <= 0` or `ty <= 0`.
///
/// Transcribed block for block from the generated body: every scratch register,
/// the callee-saved FPU/GPR saves and the stack scratch (rect at +0xe0, uv at
/// +0xf0, colour verts at +0x20) are reproduced.
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code)]
pub fn render2d_draw_tiled_sprite(c: &mut Cpu) {
    guest(c, |c| {
        'b_3a29a8: {
            'b_3a29a4: {
                c.cycles += 58;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x200));
                c.mtc1(0, 0);
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.g[S4][0] = c.g[SP][0];
                c.swc1(31, (c.g[SP][0] as u32).wrapping_add(0x1f8));
                c.g[S6][0] = c.g[A3][0];
                c.swc1(30, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x1e8));
                c.mov_s(30, 12);
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.c_le_s(30, 0);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x120), c.g[RA][0]);
                c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x1d8));
                c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x1c8));
                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1b8));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[A1][0] as u32);
                c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[SP][0] as u32);
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[A0][0];
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(3, c.g[A2][0] as u32);
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x1f));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.g[A0][0] = c.lw(c.g[A1][0] as u32);
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
                c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                c.lwc1(1, c.g[SP][0] as u32);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x104));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x108));
                c.lwc1(22, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.lwc1(31, (c.g[A1][0] as u32).wrapping_add(0x18));
                c.lwc1(21, (c.g[A1][0] as u32).wrapping_add(0x10));
                let t = c.fcc();
                c.lwc1(29, (c.g[A1][0] as u32).wrapping_add(0x14));
                if t {
                    break 'b_3a29a4;
                } else {
                    c.cycles += 5;
                    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.c_le_s(3, 0);
                    let t = c.fcc();
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
                    if t {
                        break 'b_3a29a8;
                    } else {
                        'b_3a24d8: {
                            c.cycles += 7;
                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.g[S6][0] & 0x1;
                            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
                            c.div_s(27, 0, 30);
                            c.div_s(28, 1, 3);
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.g[S6][0] & 0x2;
                            if t {
                                break 'b_3a24d8;
                            } else {
                                c.cycles += 2;
                                c.mov_s(21, 29);
                                c.lwc1(29, (c.g[A1][0] as u32).wrapping_add(0x10));
                                break 'b_3a24d8;
                            }
                        }
                        'b_3a24e8: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_3a24e8;
                            } else {
                                c.cycles += 2;
                                c.mov_s(22, 31);
                                c.lwc1(31, (c.g[A1][0] as u32).wrapping_add(0xc));
                                break 'b_3a24e8;
                            }
                        }
                        c.cycles += 10;
                        c.cvt_w_s(0, 30);
                        c.mfc1(17, 0);
                        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x108));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = 0xffffffffffffffff;
                        c.cvt_w_s(0, 0);
                        c.mfc1(21, 0);
                        c.g[V0][0] = 0x3;
                        'l_3a2510: loop {
                            c.cycles += 7;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc0));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003a_2510);
                                }
                                continue 'l_3a2510;
                            } else {
                                c.cycles += 11;
                                c.mtc1(21, 1);
                                c.cvt_s_w(1, 1);
                                c.g[AT][0] = 0x43000000;
                                c.mtc1(1, 4);
                                c.g[AT][0] = 0x437f0000;
                                c.mtc1(1, 2);
                                c.g[V1][0] = c.g[S2][0];
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 3);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10c));
                                c.lwc1(0, c.g[A2][0] as u32);
                                'l_3a2558: loop {
                                    c.cycles += 20;
                                    c.mul_s(0, 0, 4);
                                    c.cvt_w_s(1, 0);
                                    c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x1c));
                                    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.mul_s(1, 1, 2);
                                    c.cvt_w_s(0, 1);
                                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.mul_s(0, 0, 2);
                                    c.cvt_w_s(1, 0);
                                    c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
                                    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
                                    c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.mul_s(1, 1, 2);
                                    c.cvt_w_s(0, 1);
                                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.lwc1(0, c.g[A2][0] as u32);
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003a_2558);
                                        }
                                        continue 'l_3a2558;
                                    } else {
                                        'b_3a2798: {
                                            c.cycles += 2;
                                            if c.g[S5][0] == 0u64 {
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x10c));
                                                break 'b_3a2798;
                                            } else {
                                                'b_3a2794: {
                                                    c.cycles += 3;
                                                    c.lwc1(
                                                        25,
                                                        (c.g[SP][0] as u32).wrapping_add(0x108),
                                                    );
                                                    let t = (c.g[S5][0] as i64) <= 0;
                                                    c.g[V1][0] = 0u64;
                                                    if t {
                                                        break 'b_3a2794;
                                                    } else {
                                                        c.cycles += 1;
                                                        'l_3a25c0: loop {
                                                            'b_3a2788: {
                                                                'b_3a26ac: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c.g[S6][0] & 0x4;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.lwc1(
                                                                        20,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x104),
                                                                    );
                                                                    if t {
                                                                        break 'b_3a26ac;
                                                                    } else {
                                                                        'b_3a2638: {
                                                                            c.cycles += 6;
                                                                            c.add_s(1, 25, 28);
                                                                            c.g[S3][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1,
                                                                                    ));
                                                                            c.mtc1(17, 23);
                                                                            c.cvt_s_w(23, 23);
                                                                            let t = (c.g[S1][0]
                                                                                as i64)
                                                                                <= 0;
                                                                            c.swc1(
                                                                                1,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x110,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_3a2638;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.sub_s(26, 29, 21);
                                                                                c.g[S0][0] =
                                                                                    c.g[S1][0];
                                                                                c.sub_s(24, 31, 22);
                                                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                                                                'l_3a25f8: loop {
                                                                                    c.cycles += 16;
                                                                                    c.g[A1][0] =
                                                                                        c.g[S2][0];
                                                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
                                                                                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                                                                                    c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                                                                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                                                                    c.add_s(
                                                                                        20, 20, 27,
                                                                                    );
                                                                                    c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xec));
                                                                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                                                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf4));
                                                                                    c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xf8));
                                                                                    c.g[RA][0] = sx(
                                                                                        0x003a_2630,
                                                                                    );
                                                                                    c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xfc));
                                                                                    oc1077_callee::DRAW_QUAD_UV_ROT90.call(c, 0x003a_2630)?;
                                                                                    if c.g[S0][0]
                                                                                        != 0u64
                                                                                    {
                                                                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x003a_25f8);
                                                                                        }
                                                                                        continue 'l_3a25f8;
                                                                                    } else {
                                                                                        break 'b_3a2638;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 4;
                                                                        c.c_eq_s(23, 30);
                                                                        let t = c.fcc();
                                                                        c.g[V1][0] = c.g[S3][0];
                                                                        if t {
                                                                            break 'b_3a2788;
                                                                        } else {
                                                                            c.cycles += 25;
                                                                            c.sub_s(3, 30, 23);
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[S4][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                            );
                                                                            c.sub_s(0, 31, 22);
                                                                            c.lwc1(
                                                                                2,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x104,
                                                                                    ),
                                                                            );
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x844,
                                                                                ));
                                                                            c.g[A1][0] = c.g[S2][0];
                                                                            c.add_s(1, 2, 1);
                                                                            c.swc1(
                                                                                20,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xe0,
                                                                                    ),
                                                                            );
                                                                            c.mul_s(0, 0, 3);
                                                                            c.swc1(
                                                                                25,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xe4,
                                                                                    ),
                                                                            );
                                                                            c.sub_s(2, 29, 21);
                                                                            c.swc1(
                                                                                28,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xec,
                                                                                    ),
                                                                            );
                                                                            c.sub_s(1, 1, 20);
                                                                            c.swc1(
                                                                                21,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf0,
                                                                                    ),
                                                                            );
                                                                            c.add_s(0, 22, 0);
                                                                            c.swc1(
                                                                                22,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf4,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                2,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf8,
                                                                                    ),
                                                                            );
                                                                            c.g[A2][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0xe0,
                                                                                    ));
                                                                            c.swc1(
                                                                                1,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xe8,
                                                                                    ),
                                                                            );
                                                                            c.g[A3][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0xf0,
                                                                                    ));
                                                                            c.sub_s(0, 0, 22);
                                                                            c.g[RA][0] =
                                                                                sx(0x003a_26a4);
                                                                            c.swc1(
                                                                                0,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xfc,
                                                                                    ),
                                                                            );
                                                                            oc1077_callee::DRAW_QUAD_UV_ROT90.call(c, 0x003a_26a4)?;
                                                                            let t = true;
                                                                            c.g[V1][0] = c.g[S3][0];
                                                                            if t {
                                                                                break 'b_3a2788;
                                                                            } else {
                                                                                break 'b_3a26ac;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_3a2718: {
                                                                    c.cycles += 6;
                                                                    c.add_s(3, 25, 28);
                                                                    c.g[S3][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    c.mtc1(17, 23);
                                                                    c.cvt_s_w(23, 23);
                                                                    let t =
                                                                        (c.g[S1][0] as i64) <= 0;
                                                                    c.swc1(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x110),
                                                                    );
                                                                    if t {
                                                                        break 'b_3a2718;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.sub_s(26, 29, 21);
                                                                        c.g[S0][0] = c.g[S1][0];
                                                                        c.sub_s(24, 31, 22);
                                                                        c.swc1(
                                                                            20,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xe0),
                                                                        );
                                                                        'l_3a26d8: loop {
                                                                            c.cycles += 16;
                                                                            c.g[A1][0] = c.g[S2][0];
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x844,
                                                                                ));
                                                                            c.g[A2][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0xe0,
                                                                                    ));
                                                                            c.swc1(
                                                                                25,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xe4,
                                                                                    ),
                                                                            );
                                                                            c.g[A3][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0xf0,
                                                                                    ));
                                                                            c.swc1(
                                                                                27,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xe8,
                                                                                    ),
                                                                            );
                                                                            c.add_s(20, 20, 27);
                                                                            c.swc1(
                                                                                28,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xec,
                                                                                    ),
                                                                            );
                                                                            c.g[S0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x1,
                                                                                    ));
                                                                            c.swc1(
                                                                                21,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf0,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                22,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf4,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                26,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf8,
                                                                                    ),
                                                                            );
                                                                            c.g[RA][0] =
                                                                                sx(0x003a_2710);
                                                                            c.swc1(
                                                                                24,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xfc,
                                                                                    ),
                                                                            );
                                                                            oc1077_callee::DRAW_QUAD_UV
                                                                                .call(
                                                                                    c,
                                                                                    0x003a_2710,
                                                                                )?;
                                                                            if c.g[S0][0] != 0u64 {
                                                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x003a_26d8);
                                                                                }
                                                                                continue 'l_3a26d8;
                                                                            } else {
                                                                                break 'b_3a2718;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.c_eq_s(23, 30);
                                                                let t = c.fcc();
                                                                c.g[V1][0] = c.g[S3][0];
                                                                if t {
                                                                    break 'b_3a2788;
                                                                } else {
                                                                    c.cycles += 24;
                                                                    c.sub_s(3, 30, 23);
                                                                    c.lwc1(
                                                                        1,
                                                                        (c.g[S4][0] as u32)
                                                                            .wrapping_add(0x8),
                                                                    );
                                                                    c.sub_s(0, 29, 21);
                                                                    c.lwc1(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x104),
                                                                    );
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x844));
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    c.add_s(1, 2, 1);
                                                                    c.swc1(
                                                                        20,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xe0),
                                                                    );
                                                                    c.mul_s(0, 0, 3);
                                                                    c.swc1(
                                                                        25,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xe4),
                                                                    );
                                                                    c.sub_s(2, 31, 22);
                                                                    c.swc1(
                                                                        28,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xec),
                                                                    );
                                                                    c.sub_s(1, 1, 20);
                                                                    c.swc1(
                                                                        21,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0),
                                                                    );
                                                                    c.add_s(0, 21, 0);
                                                                    c.swc1(
                                                                        22,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf4),
                                                                    );
                                                                    c.swc1(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xfc),
                                                                    );
                                                                    c.g[A2][0] = sx((c.g[SP][0]
                                                                        as u32)
                                                                        .wrapping_add(0xe0));
                                                                    c.swc1(
                                                                        1,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xe8),
                                                                    );
                                                                    c.g[A3][0] = sx((c.g[SP][0]
                                                                        as u32)
                                                                        .wrapping_add(0xf0));
                                                                    c.sub_s(0, 0, 21);
                                                                    c.g[RA][0] = sx(0x003a_2784);
                                                                    c.swc1(
                                                                        0,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf8),
                                                                    );
                                                                    oc1077_callee::DRAW_QUAD_UV
                                                                        .call(c, 0x003a_2784)?;
                                                                    c.g[V1][0] = c.g[S3][0];
                                                                    break 'b_3a2788;
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V0][0] = ((c.g[V1][0] as i64)
                                                                < (c.g[S5][0] as i64))
                                                                as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.lwc1(
                                                                25,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x110),
                                                            );
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c,
                                                                        0x003a_25c0,
                                                                    );
                                                                }
                                                                continue 'l_3a25c0;
                                                            } else {
                                                                break 'b_3a2794;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x10c));
                                                break 'b_3a2798;
                                            }
                                        }
                                        c.cycles += 5;
                                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.c_eq_s(3, 0);
                                        if c.fcc() {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
                                            break 'b_3a29a8;
                                        } else {
                                            'b_3a28b4: {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.g[S6][0] & 0x4;
                                                let t = c.g[V0][0] == 0u64;
                                                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x104));
                                                if t {
                                                    break 'b_3a28b4;
                                                } else {
                                                    'b_3a2840: {
                                                        c.cycles += 14;
                                                        c.sub_s(3, 0, 3);
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 1);
                                                        c.sub_s(2, 29, 21);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[S4][0] as u32).wrapping_add(0xc),
                                                        );
                                                        c.mtc1(17, 23);
                                                        c.cvt_s_w(23, 23);
                                                        c.sub_s(1, 1, 3);
                                                        c.lwc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x108),
                                                        );
                                                        c.mul_s(2, 2, 1);
                                                        c.add_s(0, 3, 0);
                                                        c.add_s(21, 21, 2);
                                                        let t = (c.g[S1][0] as i64) <= 0;
                                                        c.sub_s(24, 0, 25);
                                                        if t {
                                                            break 'b_3a2840;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.sub_s(28, 29, 21);
                                                            c.g[S0][0] = c.g[S1][0];
                                                            c.sub_s(26, 31, 22);
                                                            c.swc1(
                                                                20,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xe0),
                                                            );
                                                            'l_3a2800: loop {
                                                                c.cycles += 16;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x844));
                                                                c.g[A2][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0xe0));
                                                                c.swc1(
                                                                    25,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xe4),
                                                                );
                                                                c.g[A3][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0xf0));
                                                                c.swc1(
                                                                    27,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xe8),
                                                                );
                                                                c.add_s(20, 20, 27);
                                                                c.swc1(
                                                                    24,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xec),
                                                                );
                                                                c.g[S0][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x1));
                                                                c.swc1(
                                                                    21,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0),
                                                                );
                                                                c.swc1(
                                                                    22,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf4),
                                                                );
                                                                c.swc1(
                                                                    28,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf8),
                                                                );
                                                                c.g[RA][0] = sx(0x003a_2838);
                                                                c.swc1(
                                                                    26,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xfc),
                                                                );
                                                                oc1077_callee::DRAW_QUAD_UV_ROT90
                                                                    .call(c, 0x003a_2838)?;
                                                                if c.g[S0][0] != 0u64 {
                                                                    c.swc1(
                                                                        20,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xe0),
                                                                    );
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c,
                                                                            0x003a_2800,
                                                                        );
                                                                    }
                                                                    continue 'l_3a2800;
                                                                } else {
                                                                    break 'b_3a2840;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.c_eq_s(23, 30);
                                                    let t = c.fcc();
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x190),
                                                    );
                                                    if t {
                                                        break 'b_3a29a8;
                                                    } else {
                                                        c.cycles += 25;
                                                        c.sub_s(2, 30, 23);
                                                        c.lwc1(
                                                            1,
                                                            (c.g[S4][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sub_s(0, 31, 22);
                                                        c.lwc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x104),
                                                        );
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x844));
                                                        c.g[A1][0] = c.g[S2][0];
                                                        c.add_s(1, 3, 1);
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0xe0),
                                                        );
                                                        c.mul_s(0, 0, 2);
                                                        c.swc1(
                                                            25,
                                                            (c.g[SP][0] as u32).wrapping_add(0xe4),
                                                        );
                                                        c.sub_s(3, 29, 21);
                                                        c.swc1(
                                                            24,
                                                            (c.g[SP][0] as u32).wrapping_add(0xec),
                                                        );
                                                        c.sub_s(1, 1, 20);
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf0),
                                                        );
                                                        c.add_s(0, 22, 0);
                                                        c.swc1(
                                                            22,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf4),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf8),
                                                        );
                                                        c.g[A2][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xe0));
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0xe8),
                                                        );
                                                        c.g[A3][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                        c.sub_s(0, 0, 22);
                                                        c.g[RA][0] = sx(0x003a_28ac);
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0xfc),
                                                        );
                                                        oc1077_callee::DRAW_QUAD_UV_ROT90
                                                            .call(c, 0x003a_28ac)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x190),
                                                        );
                                                        if t {
                                                            break 'b_3a29a8;
                                                        } else {
                                                            break 'b_3a28b4;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_3a2938: {
                                                c.cycles += 13;
                                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x100));
                                                c.mtc1(17, 23);
                                                c.cvt_s_w(23, 23);
                                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x108));
                                                c.sub_s(0, 1, 2);
                                                c.sub_s(2, 31, 22);
                                                c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0xc));
                                                c.add_s(1, 3, 1);
                                                c.mul_s(2, 2, 0);
                                                c.sub_s(24, 1, 25);
                                                let t = (c.g[S1][0] as i64) <= 0;
                                                c.add_s(31, 22, 2);
                                                if t {
                                                    break 'b_3a2938;
                                                } else {
                                                    c.cycles += 4;
                                                    c.sub_s(28, 29, 21);
                                                    c.g[S0][0] = c.g[S1][0];
                                                    c.sub_s(26, 31, 22);
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0xe0),
                                                    );
                                                    'l_3a28f8: loop {
                                                        c.cycles += 16;
                                                        c.g[A1][0] = c.g[S2][0];
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x844));
                                                        c.g[A2][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xe0));
                                                        c.swc1(
                                                            25,
                                                            (c.g[SP][0] as u32).wrapping_add(0xe4),
                                                        );
                                                        c.g[A3][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                        c.swc1(
                                                            27,
                                                            (c.g[SP][0] as u32).wrapping_add(0xe8),
                                                        );
                                                        c.add_s(20, 20, 27);
                                                        c.swc1(
                                                            24,
                                                            (c.g[SP][0] as u32).wrapping_add(0xec),
                                                        );
                                                        c.g[S0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf0),
                                                        );
                                                        c.swc1(
                                                            22,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf4),
                                                        );
                                                        c.swc1(
                                                            28,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf8),
                                                        );
                                                        c.g[RA][0] = sx(0x003a_2930);
                                                        c.swc1(
                                                            26,
                                                            (c.g[SP][0] as u32).wrapping_add(0xfc),
                                                        );
                                                        oc1077_callee::DRAW_QUAD_UV
                                                            .call(c, 0x003a_2930)?;
                                                        if c.g[S0][0] != 0u64 {
                                                            c.swc1(
                                                                20,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xe0),
                                                            );
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x003a_28f8);
                                                            }
                                                            continue 'l_3a28f8;
                                                        } else {
                                                            break 'b_3a2938;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.c_eq_s(23, 30);
                                            let t = c.fcc();
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
                                            if t {
                                                break 'b_3a29a8;
                                            } else {
                                                c.cycles += 23;
                                                c.sub_s(2, 30, 23);
                                                c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x8));
                                                c.sub_s(0, 29, 21);
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
                                                c.g[A1][0] = c.g[S2][0];
                                                c.add_s(1, 3, 1);
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                                c.mul_s(0, 0, 2);
                                                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xe4));
                                                c.sub_s(3, 31, 22);
                                                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xec));
                                                c.sub_s(1, 1, 20);
                                                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.add_s(0, 21, 0);
                                                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf4));
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xfc));
                                                c.g[A2][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                                                c.g[A3][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.sub_s(0, 0, 21);
                                                c.g[RA][0] = sx(0x003a_29a4);
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf8));
                                                oc1077_callee::DRAW_QUAD_UV.call(c, 0x003a_29a4)?;
                                                break 'b_3a29a4;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
            break 'b_3a29a8;
        }
        c.cycles += 21;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x120));
        c.lwc1(31, (c.g[SP][0] as u32).wrapping_add(0x1f8));
        c.lwc1(30, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x1e8));
        c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x1d8));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x1c8));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1b8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x1a8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
        c.pc = t;
        Ok(())
    })
}

/// `void Render2D_DrawTiledSpriteXform(Element *el, Rect *dst, Colour *col,
/// Mat *xform, float tx, float ty, int flags)` (0x3a2bc8; the reference calls it
/// `cTextureElement_renderRotTiled`): draws the texture element `el` tiled
/// `tx` by `ty` times into the destination rectangle `dst` (x, y, w, h), through
/// the 2D renderer. `a0` element (texture id at +0, uv at +0xc..+0x18), `a1`
/// destination rect, `a2` colour (copied to the stack), `a3` transform, `f12`
/// tx, `f13` ty, `t0` flags (1 flip u, 2 flip v, 4 rotated quads via
/// `Render2D_DrawQuadUVRot90Xform`, else `Render2D_DrawQuadUVXform`). Whole
/// tiles are `floor(tx)` columns by `floor(ty)` rows; the fractional rest of
/// the row and of the column is drawn as partial tiles with matching uv.
/// Kept as the generated block structure (labels are the generated block
/// addresses) so every register, FPU register and cycle matches.
#[allow(unreachable_code, unused_assignments)]
pub fn render2d_draw_tiled_sprite_xform(c: &mut Cpu) {
    guest(c, |c| {
        'b_3a3228: {
            'b_3a3224: {
                c.cycles += 78;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x240));
                c.mtc1(0, 0);
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[S5][0] = c.g[SP][0];
                c.swc1(31, (c.g[SP][0] as u32).wrapping_add(0x238));
                c.g[S7][0] = c.g[T0][0];
                c.swc1(30, (c.g[SP][0] as u32).wrapping_add(0x230));
                c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x228));
                c.mov_s(30, 12);
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.c_le_s(30, 0);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x1e8));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x150), c.g[RA][0]);
                c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x220));
                c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x218));
                c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x208));
                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1f8));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[A1][0] as u32);
                c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[SP][0] as u32);
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[A0][0];
                c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[A2][0] as u32);
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0x17));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x10));
                c.ldl(9, (c.g[A2][0] as u32).wrapping_add(0x1f));
                c.ldr(9, (c.g[A2][0] as u32).wrapping_add(0x18));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x1f));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x27));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x2f));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                c.ldl(10, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.ldr(10, c.g[A3][0] as u32);
                c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0x47));
                c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x4f));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A0][0] = c.lw(c.g[A1][0] as u32);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
                c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                c.lwc1(1, c.g[SP][0] as u32);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x134));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x138));
                c.lwc1(22, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.lwc1(31, (c.g[A1][0] as u32).wrapping_add(0x18));
                c.lwc1(21, (c.g[A1][0] as u32).wrapping_add(0x10));
                let t = c.fcc();
                c.lwc1(29, (c.g[A1][0] as u32).wrapping_add(0x14));
                if t {
                    break 'b_3a3224;
                } else {
                    c.cycles += 5;
                    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.c_le_s(3, 0);
                    let t = c.fcc();
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                    if t {
                        break 'b_3a3228;
                    } else {
                        'b_3a2d38: {
                            c.cycles += 7;
                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.g[S7][0] & 0x1;
                            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
                            c.div_s(27, 0, 30);
                            c.div_s(28, 1, 3);
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.g[S7][0] & 0x2;
                            if t {
                                break 'b_3a2d38;
                            } else {
                                c.cycles += 2;
                                c.mov_s(21, 29);
                                c.lwc1(29, (c.g[A1][0] as u32).wrapping_add(0x10));
                                break 'b_3a2d38;
                            }
                        }
                        'b_3a2d48: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_3a2d48;
                            } else {
                                c.cycles += 2;
                                c.mov_s(22, 31);
                                c.lwc1(31, (c.g[A1][0] as u32).wrapping_add(0xc));
                                break 'b_3a2d48;
                            }
                        }
                        c.cycles += 10;
                        c.cvt_w_s(0, 30);
                        c.mfc1(17, 0);
                        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x138));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x130));
                        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[V1][0] = 0xffffffffffffffff;
                        c.cvt_w_s(0, 0);
                        c.mfc1(22, 0);
                        c.g[V0][0] = 0x3;
                        'l_3a2d70: loop {
                            c.cycles += 7;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc0));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003a2d70);
                                }
                                continue 'l_3a2d70;
                            } else {
                                c.cycles += 11;
                                c.mtc1(22, 1);
                                c.cvt_s_w(1, 1);
                                c.g[AT][0] = 0x43000000;
                                c.mtc1(1, 4);
                                c.g[AT][0] = 0x437f0000;
                                c.mtc1(1, 2);
                                c.g[V1][0] = c.g[S2][0];
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 3);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.lwc1(0, c.g[A2][0] as u32);
                                'l_3a2db8: loop {
                                    c.cycles += 20;
                                    c.mul_s(0, 0, 4);
                                    c.cvt_w_s(1, 0);
                                    c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x1c));
                                    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.mul_s(1, 1, 2);
                                    c.cvt_w_s(0, 1);
                                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.mul_s(0, 0, 2);
                                    c.cvt_w_s(1, 0);
                                    c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
                                    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
                                    c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.mul_s(1, 1, 2);
                                    c.cvt_w_s(0, 1);
                                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.lwc1(0, c.g[A2][0] as u32);
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003a2db8);
                                        }
                                        continue 'l_3a2db8;
                                    } else {
                                        'b_3a3008: {
                                            c.cycles += 2;
                                            if c.g[S6][0] == 0u64 {
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                break 'b_3a3008;
                                            } else {
                                                'b_3a3004: {
                                                    c.cycles += 3;
                                                    c.lwc1(
                                                        25,
                                                        (c.g[SP][0] as u32).wrapping_add(0x138),
                                                    );
                                                    let t = (c.g[S6][0] as i64) <= 0;
                                                    c.g[V1][0] = 0u64;
                                                    if t {
                                                        break 'b_3a3004;
                                                    } else {
                                                        c.cycles += 1;
                                                        'l_3a2e20: loop {
                                                            'b_3a2ff8: {
                                                                'b_3a2f14: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c.g[S7][0] & 0x4;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.lwc1(
                                                                        20,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x134),
                                                                    );
                                                                    if t {
                                                                        break 'b_3a2f14;
                                                                    } else {
                                                                        'b_3a2e9c: {
                                                                            c.cycles += 6;
                                                                            c.add_s(1, 25, 28);
                                                                            c.g[S4][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1,
                                                                                    ));
                                                                            c.mtc1(17, 23);
                                                                            c.cvt_s_w(23, 23);
                                                                            let t = (c.g[S1][0]
                                                                                as i64)
                                                                                <= 0;
                                                                            c.swc1(
                                                                                1,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x13c,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_3a2e9c;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.sub_s(26, 29, 21);
                                                                                c.g[S0][0] =
                                                                                    c.g[S1][0];
                                                                                c.sub_s(24, 31, 22);
                                                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x110));
                                                                                'l_3a2e58: loop {
                                                                                    c.cycles += 17;
                                                                                    c.g[A1][0] =
                                                                                        c.g[S2][0];
                                                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
                                                                                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                                                                                    c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x114));
                                                                                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                                                                                    c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x118));
                                                                                    c.g[T0][0] =
                                                                                        c.g[S3][0];
                                                                                    c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x11c));
                                                                                    c.add_s(
                                                                                        20, 20, 27,
                                                                                    );
                                                                                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x120));
                                                                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                                                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x124));
                                                                                    c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x128));
                                                                                    c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x12c));
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x0038ea60,
                                                                                        0x003a2e94,
                                                                                    )?;
                                                                                    if c.g[S0][0]
                                                                                        != 0u64
                                                                                    {
                                                                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x110));
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x003a2e58);
                                                                                        }
                                                                                        continue 'l_3a2e58;
                                                                                    } else {
                                                                                        break 'b_3a2e9c;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 4;
                                                                        c.c_eq_s(23, 30);
                                                                        let t = c.fcc();
                                                                        c.g[V1][0] = c.g[S4][0];
                                                                        if t {
                                                                            break 'b_3a2ff8;
                                                                        } else {
                                                                            c.cycles += 26;
                                                                            c.sub_s(3, 30, 23);
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[S5][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                            );
                                                                            c.sub_s(0, 31, 22);
                                                                            c.lwc1(
                                                                                2,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x134,
                                                                                    ),
                                                                            );
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x844,
                                                                                ));
                                                                            c.g[A1][0] = c.g[S2][0];
                                                                            c.add_s(1, 2, 1);
                                                                            c.swc1(
                                                                                20,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x110,
                                                                                    ),
                                                                            );
                                                                            c.mul_s(0, 0, 3);
                                                                            c.swc1(
                                                                                25,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x114,
                                                                                    ),
                                                                            );
                                                                            c.sub_s(2, 29, 21);
                                                                            c.swc1(
                                                                                28,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x11c,
                                                                                    ),
                                                                            );
                                                                            c.sub_s(1, 1, 20);
                                                                            c.swc1(
                                                                                21,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x120,
                                                                                    ),
                                                                            );
                                                                            c.add_s(0, 22, 0);
                                                                            c.swc1(
                                                                                22,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x124,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                2,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x128,
                                                                                    ),
                                                                            );
                                                                            c.g[A2][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x110,
                                                                                    ));
                                                                            c.swc1(
                                                                                1,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x118,
                                                                                    ),
                                                                            );
                                                                            c.g[A3][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x120,
                                                                                    ));
                                                                            c.sub_s(0, 0, 22);
                                                                            c.g[T0][0] = c.g[S3][0];
                                                                            c.swc1(
                                                                                0,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x12c,
                                                                                    ),
                                                                            );
                                                                            call_ptr(
                                                                                c, 0x0038ea60,
                                                                                0x003a2f0c,
                                                                            )?;
                                                                            let t = true;
                                                                            c.g[V1][0] = c.g[S4][0];
                                                                            if t {
                                                                                break 'b_3a2ff8;
                                                                            } else {
                                                                                break 'b_3a2f14;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_3a2f84: {
                                                                    c.cycles += 6;
                                                                    c.add_s(3, 25, 28);
                                                                    c.g[S4][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    c.mtc1(17, 23);
                                                                    c.cvt_s_w(23, 23);
                                                                    let t =
                                                                        (c.g[S1][0] as i64) <= 0;
                                                                    c.swc1(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x13c),
                                                                    );
                                                                    if t {
                                                                        break 'b_3a2f84;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.sub_s(26, 29, 21);
                                                                        c.g[S0][0] = c.g[S1][0];
                                                                        c.sub_s(24, 31, 22);
                                                                        c.swc1(
                                                                            20,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x110,
                                                                                ),
                                                                        );
                                                                        'l_3a2f40: loop {
                                                                            c.cycles += 17;
                                                                            c.g[A1][0] = c.g[S2][0];
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x844,
                                                                                ));
                                                                            c.g[A2][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x110,
                                                                                    ));
                                                                            c.swc1(
                                                                                25,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x114,
                                                                                    ),
                                                                            );
                                                                            c.g[A3][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x120,
                                                                                    ));
                                                                            c.swc1(
                                                                                27,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x118,
                                                                                    ),
                                                                            );
                                                                            c.g[T0][0] = c.g[S3][0];
                                                                            c.swc1(
                                                                                28,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x11c,
                                                                                    ),
                                                                            );
                                                                            c.add_s(20, 20, 27);
                                                                            c.swc1(
                                                                                21,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x120,
                                                                                    ),
                                                                            );
                                                                            c.g[S0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x1,
                                                                                    ));
                                                                            c.swc1(
                                                                                22,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x124,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                26,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x128,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                24,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x12c,
                                                                                    ),
                                                                            );
                                                                            call_ptr(
                                                                                c, 0x0038e880,
                                                                                0x003a2f7c,
                                                                            )?;
                                                                            if c.g[S0][0] != 0u64 {
                                                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x110));
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x003a2f40);
                                                                                }
                                                                                continue 'l_3a2f40;
                                                                            } else {
                                                                                break 'b_3a2f84;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.c_eq_s(23, 30);
                                                                let t = c.fcc();
                                                                c.g[V1][0] = c.g[S4][0];
                                                                if t {
                                                                    break 'b_3a2ff8;
                                                                } else {
                                                                    c.cycles += 25;
                                                                    c.sub_s(3, 30, 23);
                                                                    c.lwc1(
                                                                        1,
                                                                        (c.g[S5][0] as u32)
                                                                            .wrapping_add(0x8),
                                                                    );
                                                                    c.sub_s(0, 29, 21);
                                                                    c.lwc1(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x134),
                                                                    );
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x844));
                                                                    c.g[A1][0] = c.g[S2][0];
                                                                    c.add_s(1, 2, 1);
                                                                    c.swc1(
                                                                        20,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x110),
                                                                    );
                                                                    c.mul_s(0, 0, 3);
                                                                    c.swc1(
                                                                        25,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x114),
                                                                    );
                                                                    c.sub_s(2, 31, 22);
                                                                    c.swc1(
                                                                        28,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x11c),
                                                                    );
                                                                    c.sub_s(1, 1, 20);
                                                                    c.swc1(
                                                                        21,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x120),
                                                                    );
                                                                    c.add_s(0, 21, 0);
                                                                    c.swc1(
                                                                        22,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x124),
                                                                    );
                                                                    c.swc1(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x12c),
                                                                    );
                                                                    c.g[A2][0] = sx((c.g[SP][0]
                                                                        as u32)
                                                                        .wrapping_add(0x110));
                                                                    c.swc1(
                                                                        1,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x118),
                                                                    );
                                                                    c.g[A3][0] = sx((c.g[SP][0]
                                                                        as u32)
                                                                        .wrapping_add(0x120));
                                                                    c.sub_s(0, 0, 21);
                                                                    c.g[T0][0] = c.g[S3][0];
                                                                    c.swc1(
                                                                        0,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x128),
                                                                    );
                                                                    call_ptr(
                                                                        c, 0x0038e880, 0x003a2ff4,
                                                                    )?;
                                                                    c.g[V1][0] = c.g[S4][0];
                                                                    break 'b_3a2ff8;
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V0][0] = ((c.g[V1][0] as i64)
                                                                < (c.g[S6][0] as i64))
                                                                as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.lwc1(
                                                                25,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x13c),
                                                            );
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x003a2e20,
                                                                    );
                                                                }
                                                                continue 'l_3a2e20;
                                                            } else {
                                                                break 'b_3a3004;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                break 'b_3a3008;
                                            }
                                        }
                                        c.cycles += 5;
                                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        c.c_eq_s(3, 0);
                                        if c.fcc() {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                                            break 'b_3a3228;
                                        } else {
                                            'b_3a312c: {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.g[S7][0] & 0x4;
                                                let t = c.g[V0][0] == 0u64;
                                                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x134));
                                                if t {
                                                    break 'b_3a312c;
                                                } else {
                                                    'b_3a30b4: {
                                                        c.cycles += 14;
                                                        c.sub_s(3, 0, 3);
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 1);
                                                        c.sub_s(2, 29, 21);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[S5][0] as u32).wrapping_add(0xc),
                                                        );
                                                        c.mtc1(17, 23);
                                                        c.cvt_s_w(23, 23);
                                                        c.sub_s(1, 1, 3);
                                                        c.lwc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x138),
                                                        );
                                                        c.mul_s(2, 2, 1);
                                                        c.add_s(0, 3, 0);
                                                        c.add_s(21, 21, 2);
                                                        let t = (c.g[S1][0] as i64) <= 0;
                                                        c.sub_s(24, 0, 25);
                                                        if t {
                                                            break 'b_3a30b4;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.sub_s(28, 29, 21);
                                                            c.g[S0][0] = c.g[S1][0];
                                                            c.sub_s(26, 31, 22);
                                                            c.swc1(
                                                                20,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x110),
                                                            );
                                                            'l_3a3070: loop {
                                                                c.cycles += 17;
                                                                c.g[A1][0] = c.g[S2][0];
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x844));
                                                                c.g[A2][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x110));
                                                                c.swc1(
                                                                    25,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x114),
                                                                );
                                                                c.g[A3][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x120));
                                                                c.swc1(
                                                                    27,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x118),
                                                                );
                                                                c.g[T0][0] = c.g[S3][0];
                                                                c.swc1(
                                                                    24,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x11c),
                                                                );
                                                                c.add_s(20, 20, 27);
                                                                c.swc1(
                                                                    21,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x120),
                                                                );
                                                                c.g[S0][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x1));
                                                                c.swc1(
                                                                    22,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x124),
                                                                );
                                                                c.swc1(
                                                                    28,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x128),
                                                                );
                                                                c.swc1(
                                                                    26,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x12c),
                                                                );
                                                                call_ptr(
                                                                    c, 0x0038ea60, 0x003a30ac,
                                                                )?;
                                                                if c.g[S0][0] != 0u64 {
                                                                    c.swc1(
                                                                        20,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x110),
                                                                    );
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x003a3070,
                                                                        );
                                                                    }
                                                                    continue 'l_3a3070;
                                                                } else {
                                                                    break 'b_3a30b4;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.c_eq_s(23, 30);
                                                    let t = c.fcc();
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x1d0),
                                                    );
                                                    if t {
                                                        break 'b_3a3228;
                                                    } else {
                                                        c.cycles += 26;
                                                        c.sub_s(2, 30, 23);
                                                        c.lwc1(
                                                            1,
                                                            (c.g[S5][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sub_s(0, 31, 22);
                                                        c.lwc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x134),
                                                        );
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x844));
                                                        c.g[A1][0] = c.g[S2][0];
                                                        c.add_s(1, 3, 1);
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0x110),
                                                        );
                                                        c.mul_s(0, 0, 2);
                                                        c.swc1(
                                                            25,
                                                            (c.g[SP][0] as u32).wrapping_add(0x114),
                                                        );
                                                        c.sub_s(3, 29, 21);
                                                        c.swc1(
                                                            24,
                                                            (c.g[SP][0] as u32).wrapping_add(0x11c),
                                                        );
                                                        c.sub_s(1, 1, 20);
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.add_s(0, 22, 0);
                                                        c.swc1(
                                                            22,
                                                            (c.g[SP][0] as u32).wrapping_add(0x124),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x128),
                                                        );
                                                        c.g[T0][0] = c.g[S3][0];
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x118),
                                                        );
                                                        c.g[A2][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x110));
                                                        c.sub_s(0, 0, 22);
                                                        c.g[A3][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x120));
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x12c),
                                                        );
                                                        call_ptr(c, 0x0038ea60, 0x003a3124)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1d0),
                                                        );
                                                        if t {
                                                            break 'b_3a3228;
                                                        } else {
                                                            break 'b_3a312c;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_3a31b4: {
                                                c.cycles += 13;
                                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x130));
                                                c.mtc1(17, 23);
                                                c.cvt_s_w(23, 23);
                                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x138));
                                                c.sub_s(0, 1, 2);
                                                c.sub_s(2, 31, 22);
                                                c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0xc));
                                                c.add_s(1, 3, 1);
                                                c.mul_s(2, 2, 0);
                                                c.sub_s(24, 1, 25);
                                                let t = (c.g[S1][0] as i64) <= 0;
                                                c.add_s(31, 22, 2);
                                                if t {
                                                    break 'b_3a31b4;
                                                } else {
                                                    c.cycles += 4;
                                                    c.sub_s(28, 29, 21);
                                                    c.g[S0][0] = c.g[S1][0];
                                                    c.sub_s(26, 31, 22);
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0x110),
                                                    );
                                                    'l_3a3170: loop {
                                                        c.cycles += 17;
                                                        c.g[A1][0] = c.g[S2][0];
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x844));
                                                        c.g[A2][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x110));
                                                        c.swc1(
                                                            25,
                                                            (c.g[SP][0] as u32).wrapping_add(0x114),
                                                        );
                                                        c.g[A3][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x120));
                                                        c.swc1(
                                                            27,
                                                            (c.g[SP][0] as u32).wrapping_add(0x118),
                                                        );
                                                        c.g[T0][0] = c.g[S3][0];
                                                        c.swc1(
                                                            24,
                                                            (c.g[SP][0] as u32).wrapping_add(0x11c),
                                                        );
                                                        c.add_s(20, 20, 27);
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.g[S0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        c.swc1(
                                                            22,
                                                            (c.g[SP][0] as u32).wrapping_add(0x124),
                                                        );
                                                        c.swc1(
                                                            28,
                                                            (c.g[SP][0] as u32).wrapping_add(0x128),
                                                        );
                                                        c.swc1(
                                                            26,
                                                            (c.g[SP][0] as u32).wrapping_add(0x12c),
                                                        );
                                                        call_ptr(c, 0x0038e880, 0x003a31ac)?;
                                                        if c.g[S0][0] != 0u64 {
                                                            c.swc1(
                                                                20,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x110),
                                                            );
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x003a3170);
                                                            }
                                                            continue 'l_3a3170;
                                                        } else {
                                                            break 'b_3a31b4;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.c_eq_s(23, 30);
                                            let t = c.fcc();
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                                            if t {
                                                break 'b_3a3228;
                                            } else {
                                                c.cycles += 24;
                                                c.sub_s(2, 30, 23);
                                                c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0x8));
                                                c.sub_s(0, 29, 21);
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x134));
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
                                                c.g[A1][0] = c.g[S2][0];
                                                c.add_s(1, 3, 1);
                                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x110));
                                                c.mul_s(0, 0, 2);
                                                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x114));
                                                c.sub_s(3, 31, 22);
                                                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x11c));
                                                c.sub_s(1, 1, 20);
                                                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x120));
                                                c.add_s(0, 21, 0);
                                                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x124));
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x12c));
                                                c.g[T0][0] = c.g[S3][0];
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x118));
                                                c.g[A2][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x110));
                                                c.sub_s(0, 0, 21);
                                                c.g[A3][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x120));
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x128));
                                                call_ptr(c, 0x0038e880, 0x003a3224)?;
                                                break 'b_3a3224;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1d0));
            break 'b_3a3228;
        }
        c.cycles += 22;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x150));
        c.lwc1(31, (c.g[SP][0] as u32).wrapping_add(0x238));
        c.lwc1(30, (c.g[SP][0] as u32).wrapping_add(0x230));
        c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x228));
        c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x220));
        c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x218));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x210));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x208));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x200));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1f8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x1e8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x240));
        c.pc = t;
        Ok(())
    })
}

// cUITextScroll field offsets.
#[allow(dead_code)]
pub(crate) const CUITEXTSCROLL_SCROLL: u32 = 0xb8;

#[allow(dead_code)]
pub(crate) const CUITEXTSCROLL_TOP: u32 = 0xc6;

#[allow(dead_code)]
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
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
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
                        c.swc1(3, c.g[SP][0] as u32);
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
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                c.g[A0][0] = c.g[V0][0] & 0xffff;
                                c.g[V1][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
                                if c.g[V1][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb8));
                                    break 'b_3a1884;
                                } else {
                                    'b_3a17c8: {
                                        'b_3a17a4: {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            let t = c.g[V0][0] == 0u64;

                                            if t {
                                                break 'b_3a17a4;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lhu((c.g[S1][0] as u32).wrapping_add(0xc4));
                                                let t = c.g[A0][0] != c.g[V0][0];

                                                if t {
                                                    break 'b_3a17a4;
                                                } else {
                                                    c.cycles += 10;
                                                    c.ldl(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0xaf),
                                                    );
                                                    c.ldr(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0xa8),
                                                    );
                                                    c.ldl(
                                                        3,
                                                        (c.g[S1][0] as u32).wrapping_add(0xb7),
                                                    );
                                                    c.ldr(
                                                        3,
                                                        (c.g[S1][0] as u32).wrapping_add(0xb0),
                                                    );
                                                    c.sdl(
                                                        2,
                                                        (c.g[S2][0] as u32).wrapping_add(0x47),
                                                    );
                                                    c.sdr(
                                                        2,
                                                        (c.g[S2][0] as u32).wrapping_add(0x40),
                                                    );
                                                    c.sdl(
                                                        3,
                                                        (c.g[S2][0] as u32).wrapping_add(0x4f),
                                                    );
                                                    c.sdr(
                                                        3,
                                                        (c.g[S2][0] as u32).wrapping_add(0x48),
                                                    );
                                                    let t = true;
                                                    c.g[V0][0] = c.lhu(
                                                        (c.g[S1][0] as u32).wrapping_add(0xc6),
                                                    );
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
                                    c.g[V0][0] =
                                        sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0x54));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x50));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[S0][0] = c.lw(c.g[V0][0] as u32);
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
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003a1748);
                                        }
                                        continue 'l_3a1748;
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

pub(crate) const OC1083_UIOBJECT_LOAD: Callee = Callee {
    addr: 0x0039_fe00,
    f: ee::s13::f_0039fe00,
};

pub(crate) const OC1083_UITS_CLEAR_LINES: Callee = Callee {
    addr: 0x003a_18c0,
    f: ee::s13::f_003a18c0,
};

pub(crate) const OC1083_UITS_SET_TEXT_ID: Callee = Callee {
    addr: 0x003a_1f18,
    f: ee::s13::f_003a1f18,
};

/// `int UIText_GetCachedSize(UIText* self)` (0x3a10f0): the cached size at
/// `+0xa8`, after a refresh through the text's vtable slot 16 (at `+0x80`)
/// unless the cached flag (bit 2 of `+0x14`) is set and the text is laid out
/// (`+0xb4` non-zero).
pub fn ui_text_get_cached_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(this).off(0xb4).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(this).off(8).lw(c);
            ui_text_refresh(c, this)?;
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(this).off(0x14).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32) & 1;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = Addr(this).off(8).lw(c);
                ui_text_refresh(c, this)?;
            }
        }
        c.g[V0][0] = Addr(this).off(0xa8).lw(c);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The refresh call of [`ui_text_get_cached_size`]: vtable slot 16 at `v0`.
pub(crate) fn ui_text_refresh(c: &mut Cpu, this: u32) -> Flow {
    let _ = this;
    c.cycles += 5;
    let vt = c.g[V0][0];
    let slot = VSlot::read(c, vt, 0x80);
    c.g[A0][0] = slot.delta;
    c.g[V1][0] = slot.func;
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = slot.this(c.g[S0][0]);
    call_ptr(c, t, 0x003a_1134)?;
    Ok(())
}

/// `void cUITextScroll_Load(cUITextScroll* self, tUITextScroll* data)`
/// (0x3a11b8): load the base object, set the font and shadow bits of the
/// state word at `+0x74`, let the vtable hook (slot at `+0xb8`) see the
/// shadow flag, rebuild the text when the data has a string id, and copy the
/// shadow colour (scaled by 1/255) when bit 3 of the state word is set.
pub fn c_ui_text_scroll_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        let this = c.g[S1][0] as u32;
        let data = c.g[S0][0] as u32;
        OC1083_UIOBJECT_LOAD.call(c, 0x003a_11d4)?;

        c.g[V1][0] = Addr(data).off(0x20).lbu(c);
        c.g[A0][0] = 0xffff_ffff_ffff_fff9;
        c.g[V0][0] = Addr(this).off(0x74).lw(c);
        c.g[V1][0] = c.g[V1][0] & 3;
        c.g[A2][0] = Addr(this).off(8).lw(c);
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        let word = c.g[V0][0];
        Addr(this).off(0x74).sw(c, word);

        let vt = c.g[A2][0];
        let hook = VSlot::read(c, vt, 0xb8);
        c.g[A0][0] = hook.delta;
        c.g[A1][0] = Addr(data).off(0x21).lbu(c);
        c.g[V1][0] = hook.func;
        c.g[A0][0] = hook.this(c.g[S1][0]);
        c.g[A1][0] &= 1;
        call_ptr(c, hook.func as u32, 0x003a_1210)?;

        c.g[V0][0] = Addr(data).off(0x24).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(this).off(0x74).lw(c);
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            OC1083_UITS_CLEAR_LINES.call(c, 0x003a_1224)?;
            c.g[A1][0] = Addr(data).off(0x24).lw(c);
            c.g[A0][0] = c.g[S1][0];
            OC1083_UITS_SET_TEXT_ID.call(c, 0x003a_1230)?;
            c.g[V0][0] = Addr(this).off(0x74).lw(c);
        }

        // Shadow colour block.
        c.cycles += 4;
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32) & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 30;
            c.g[V0][0] = Addr(data).off(0x2b).lbu(c);
            c.g[V1][0] = Addr(data).off(0x28).lbu(c);
            c.g[A0][0] = Addr(data).off(0x29).lbu(c);
            c.mtc1(2, 4);
            c.cvt_s_w(4, 4);
            c.g[A1][0] = Addr(data).off(0x2a).lbu(c);
            c.mtc1(3, 2);
            c.cvt_s_w(2, 2);
            c.mtc1(4, 3);
            c.cvt_s_w(3, 3);
            let gp = c.g[GP][0] as u32;
            c.lwc1(0, gp.wrapping_sub(0x24f4));
            c.mtc1(5, 1);
            c.cvt_s_w(1, 1);
            c.mul_s(4, 4, 0);
            c.mul_s(2, 2, 0);
            c.mul_s(3, 3, 0);
            c.mul_s(1, 1, 0);
            let s = sp(c);
            c.swc1(4, s.wrapping_add(0xc));
            c.swc1(2, s);
            c.swc1(3, s.wrapping_add(4));
            c.swc1(1, s.wrapping_add(8));
            c.ldl(2, s.wrapping_add(7));
            c.ldr(2, s);
            c.ldl(3, s.wrapping_add(0xf));
            c.ldr(3, s.wrapping_add(8));
            let s1 = c.g[S1][0] as u32;
            c.sdl(2, s1.wrapping_add(0x7f));
            c.sdr(2, s1.wrapping_add(0x78));
            c.sdl(3, s1.wrapping_add(0x87));
            c.sdr(3, s1.wrapping_add(0x80));
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// A field of a guest object: `base + off` as an address (the `addiu` the
/// generated code folds into each load and store).
#[inline(always)]
pub(crate) fn ui_text_field(base: u64, off: u32) -> Addr {
    Addr((base as u32).wrapping_add(off))
}

/// `void cUITextScroll_RefreshText(cUITextScroll* self)` (0x3a1360): reads
/// the text id at `self+0xc0`; when set, resolves the owner chain
/// (`self+0x5c` font list, `+0xd0` table, `+0x10` owner, `+0x10` text object)
/// and calls the text object's vtable slot at `+0x20` (`this + delta`) for
/// the wide string, replaces the string with it (`cUITextScroll_ReplaceWideString`
/// when the call returns non-null), then calls the vtable slot at `self+8`
/// `+0x28` to relayout.
pub fn ui_text_scroll_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = ui_text_field(c.g[S0][0], 0xc0).lw(c);
        if c.g[A1][0] == 0 {
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 6;
            c.g[V0][0] = ui_text_field(c.g[S0][0], 0x5c).lw(c);
            c.g[V1][0] = ui_text_field(c.g[V0][0], 0xd0).lw(c);
            c.g[A0][0] = ui_text_field(c.g[V1][0], 0x10).lw(c);
            c.g[A2][0] = ui_text_field(c.g[A0][0], 0x10).lw(c);
            if c.g[A2][0] == 0 {
                frame.restore(c, 16, 0x10);
            } else {
                c.cycles += 7;
                c.g[V1][0] = ui_text_field(c.g[A2][0], 4).lw(c);
                c.g[A0][0] = ui_text_field(c.g[V1][0], 0x20).lh(c);
                c.g[V0][0] = ui_text_field(c.g[V1][0], 0x24).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x003a_13a8)?;
                let text_is_null = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if !text_is_null {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    let replace = Callee {
                        addr: 0x003a_1cf0,
                        f: ee::s13::f_003a1cf0,
                    };
                    replace.call(c, 0x003a_13b8)?;
                }
                c.cycles += 7;
                c.g[V0][0] = ui_text_field(c.g[S0][0], 8).lw(c);
                c.g[A1][0] = 1;
                c.g[A0][0] = ui_text_field(c.g[V0][0], 0x28).lh(c);
                c.g[V1][0] = ui_text_field(c.g[V0][0], 0x2c).lw(c);
                let relayout = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, relayout, 0x003a_13d0)?;
                frame.restore(c, 16, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUITextScroll_ResetString(cUITextScroll* self, char* str)`
/// (0x3a1c80): clears the lines, resets the line count, calls the vtable
/// slots at `self+8` (`+0x28`, then `+0xc0`), and sets the string.
pub fn ui_text_scroll_set_ascii_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let clear = Callee {
            addr: 0x003a_18c0,
            f: ee::s13::f_003a18c0,
        };
        clear.call(c, 0x003a_1c9c)?;
        c.g[V0][0] = ui_text_field(c.g[S0][0], 8).lw(c);
        c.g[A1][0] = 1;
        ui_text_field(c.g[S0][0], 0xc0).sw(c, 0);
        c.g[A0][0] = ui_text_field(c.g[V0][0], 0x28).lh(c);
        c.g[V1][0] = ui_text_field(c.g[V0][0], 0x2c).lw(c);
        let first = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, first, 0x003a_1cb8)?;
        c.g[V0][0] = ui_text_field(c.g[S0][0], 8).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = ui_text_field(c.g[V0][0], 0xc0).lh(c);
        c.g[V1][0] = ui_text_field(c.g[V0][0], 0xc4).lw(c);
        let second = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, second, 0x003a_1cd0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        let set_string = Callee {
            addr: 0x003a_19f8,
            f: ee::s13::f_003a19f8,
        };
        set_string.call(c, 0x003a_1cdc)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUITextScroll_RemoveLine(cUITextScroll* self, int idx)` (0x3a1f90):
/// when the line array is set and `idx` is below the line count, frees the
/// array entry (`EA_MemFree`), moves the tail down one slot and shrinks the
/// count; the array is reallocated to the new size first when it is not
/// empty.
pub fn ui_text_scroll_remove_line(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        'exit: {
            c.g[V0][0] = ui_text_field(c.g[S0][0], 0xbc).lw(c);
            c.g[S1][0] = c.g[A1][0];
            if c.g[V0][0] == 0 {
                break 'exit;
            }
            c.cycles += 4;
            c.g[A0][0] = ui_text_field(c.g[S0][0], 0xc8).lhu(c);
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let idx_below_count = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            if !idx_below_count {
                break 'exit;
            }
            'grow: {
                c.cycles += 8;
                c.g[A1][0] = 0x490000;
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40a8));
                c.g[A2][0] = 0x100;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x003a_1fdc)?;
                let idx_non_positive = (c.g[S1][0] as i64) <= 0;
                c.g[S2][0] = c.g[V0][0];
                if idx_non_positive {
                    break 'grow;
                }
                c.cycles += 4;
                c.g[A1][0] = ui_text_field(c.g[S0][0], 0xbc).lw(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = sx((c.g[S1][0] as u32) << 2);
                call_ptr(c, 0x0041_605c, 0x003a_1ff4)?;
            }
            'tail: {
                c.cycles += 5;
                c.g[A2][0] = ui_text_field(c.g[S0][0], 0xc8).lhu(c);
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] == 0;
                c.g[A0][0] = sx((c.g[S1][0] as u32) << 2);
                if t {
                    break 'tail;
                }
                c.cycles += 8;
                c.g[A1][0] = ui_text_field(c.g[S0][0], 0xbc).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                call_ptr(c, 0x0041_605c, 0x003a_2028)?;
            }
            'free: {
                c.cycles += 3;
                c.g[A0][0] = ui_text_field(c.g[S0][0], 0xbc).lw(c);
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = ui_text_field(c.g[S0][0], 0xc8).lhu(c);
                    break 'free;
                }
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x003a_203c)?;
                c.g[V0][0] = ui_text_field(c.g[S0][0], 0xc8).lhu(c);
            }
            c.cycles += 3;
            let new_array = c.g[S2][0];
            ui_text_field(c.g[S0][0], 0xbc).sw(c, new_array);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let new_count = c.g[V0][0];
            ui_text_field(c.g[S0][0], 0xc8).sh(c, new_count);
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::game::ealib::seg_2dcae8::tests::oc445_install_fakes;
    pub(crate) use crate::game::sound::soundsys::tests::oc756_install_fakes;

    use crate::difftest::fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::be::beintplayer::tests::w8;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::bx::bxstringctor::tests::be_library;
    pub(crate) use crate::game::bx::bxstringctor::tests::SCRATCH;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::pool_7 as pool;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::DST;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_4;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_4;

    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::input::inputmap::tests::alloc as alloc_2;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::slot;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;

    pub(crate) use crate::game::ui::uianimation::tests::pool_allocator;
    pub(crate) use crate::game::util::menu::tests::wf_2 as wf;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;

    use ssx3_ee as ee;

    /// Shared setup for the two font getters (`UIText` / `cUITextScroll`).
    pub(crate) fn get_font_setup(c: &mut Cpu, seed: u32, at_74_is_word: bool) {
        let mut r = Rng(0x3a04_f000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        let fontlist = DATA + 0x100;
        let fonts = DATA + 0x200;
        let base = DATA + 0x400;
        c.bus.write32(DATA + 0x5c, fontlist);
        c.bus.write32(fontlist + 0xd0, fonts);
        if seed % 3 == 0 {
            c.bus.write32(fonts + 0x10, 0);
        } else {
            c.bus.write32(fonts + 0x10, fonts);
            c.bus.write32(fonts + 0x8, base);
        }
        if at_74_is_word {
            c.bus
                .write32(DATA + 0x74, (r.next_u32() << 3) | (r.next_u32() & 0x7));
        } else {
            c.bus.write8(DATA + 0x74, (r.next_u32() & 0xff) as u8);
        }
        for k in 0..4 {
            c.bus.write32(base + k * 12 + 0xc, r.next_u32());
        }
    }

    #[test]
    fn c_ui_text_scroll_get_font_matches() {
        check_lohi(
            ssx3_ee::s13::f_003a12d0,
            c_ui_text_scroll_get_font,
            200,
            &|c, seed| get_font_setup(c, seed, true),
        );
    }

    #[test]
    fn c_ui_text_scroll_set_nav_buttons_matches() {
        check(
            ssx3_ee::s13::f_003a1310,
            c_ui_text_scroll_set_nav_buttons,
            200,
            &|c, seed| {
                let mut r = Rng(0x3a13_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32)
                    | if seed % 2 == 0 {
                        0
                    } else {
                        (seed % 7) as u64 + 1
                    };
                for k in 0x88..=0x8b {
                    c.bus.write8(DATA + k, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn c_ui_text_scroll_get_line_matches() {
        check(
            ssx3_ee::s13::f_003a2068,
            c_ui_text_scroll_get_line,
            300,
            &|c, seed| {
                let mut r = Rng(0x3a20_6800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let lines = DATA + 0x400;
                c.g[A1][0] = match seed % 4 {
                    0 => (r.next_u32() as u64) | 0x8000_0000_0000_0000, // negative
                    1 => 0,
                    2 => 20, // out of range below
                    _ => (seed % 10) as u64,
                };
                if seed % 5 == 0 {
                    c.bus.write32(DATA + 0xbc, 0);
                } else {
                    c.bus.write32(DATA + 0xbc, lines);
                    for k in 0..12 {
                        c.bus.write32(lines + k * 4, r.next_u32());
                    }
                }
                c.bus.write16(DATA + 0xc8, 10);
            },
        );
    }

    #[test]
    fn ui_text_set_wide_string_matches() {
        crate::difftest::check(
            ee::s13::f_003a0e90__generated,
            ui_text_set_wide_string,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a0e_90, 2);
                be_library(c);
                c.g[A0][0] = sx(DATA);
                // The wide string: a zeroed run so the length/copy loops stop at
                // the first unit.
                c.g[A1][0] = sx(DATA + 0x1000);
                for k in 0..8u32 {
                    c.bus.write32(DATA + 0x1000 + 4 * k, 0);
                }
                c.bus
                    .write32(DATA + 8, if seed % 3 == 0 { 0 } else { DATA + 0x200 });
            },
        );
    }

    #[test]
    fn ui_text_get_owned_string_matches() {
        check(
            ee::s13::f_003a0c00__generated,
            ui_text_get_owned_string,
            120,
            &|c, seed| {
                let mut r = Rng(0x3a0c_0000 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.g[A0][0] = SCRATCH as u64;
                c.bus
                    .write32(SCRATCH + 0x74, if seed % 2 == 0 { 0 } else { 1 << 3 });
                c.bus.write32(SCRATCH + 0xb4, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_text_get_id_string_matches() {
        check(
            ee::s13::f_003a0c28__generated,
            ui_text_get_id_string,
            120,
            &|c, seed| {
                let mut r = Rng(0x3a0c_0028 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.g[A0][0] = SCRATCH as u64;
                c.bus
                    .write32(SCRATCH + 0x74, if seed % 2 == 0 { 0 } else { 1 << 3 });
                c.bus.write32(SCRATCH + 0xb4, r.next_u32());
            },
        );
    }

    /// A 16-bit word of test data.
    pub(crate) fn h(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }

    #[test]
    fn ui_text_scroll_replace_wide_string_matches() {
        check(
            ssx3_ee::s13::f_003a1cf0__generated,
            ui_text_scroll_replace_wide_string,
            60,
            &|c, seed| {
                machine(c, seed, 0x3a1c_f0, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                match seed % 3 {
                    0 => {}
                    1 => h(c, DATA + 0x400, 0x41),
                    _ => {
                        h(c, DATA + 0x400, 0x41);
                        h(c, DATA + 0x402, 0x42);
                        h(c, DATA + 0x404, 0);
                    }
                }
            },
        );
    }

    #[test]
    fn ui_text_free_strings_matches() {
        check(
            ssx3_ee::s13::f_003a0b38__generated,
            ui_text_free_strings,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0xa0b38, 0);
                heap_init(c);
                let this = DATA;
                if seed % 2 == 0 {
                    let a = alloc(c, 0x20);
                    c.bus.write32(this + 0xac, a);
                }
                if (seed >> 1) % 2 == 0 {
                    let b = alloc(c, 0x20);
                    c.bus.write32(this + 0xb4, b);
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn ui_vector_set_color_matches() {
        check(
            ssx3_ee::s13::f_003a3398__generated,
            ui_vector_set_color,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a3398, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                for k in 0..4 {
                    word(c, DATA + 0x100 + 4 * k, r.edge_f32());
                }
                let count = match seed % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    _ => 1 + (r.next_u32() % 4),
                };
                word(c, DATA + 0x74, count << 17);
                word(c, DATA + 0x7c, if seed % 7 == 0 { 0 } else { DATA + 0x200 });
                for k in 0..16 {
                    word(c, DATA + 0x200 + 4 * k, r.next_u32());
                }
                word(c, DATA + 0x1c, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_vector_set_color_delivers_events() {
        use crate::difftest::events::schedule;
        check(
            ssx3_ee::s13::f_003a3398__generated,
            ui_vector_set_color,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a3398, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                for k in 0..4 {
                    word(c, DATA + 0x100 + 4 * k, r.edge_f32());
                }
                word(c, DATA + 0x74, 6 << 17);
                word(c, DATA + 0x7c, DATA + 0x200);
                schedule(c, &mut r, seed, 0x003a_3398);
            },
        );
    }

    #[test]
    fn c_ui_text_scroll_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a1148__generated,
            c_ui_text_scroll_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a1148, 0);
                let s = DATA + 0x1000;
                c.g[A0][0] = s as u64;
                w32(c, s + 0x14, r.next_u32());
                w8(c, s + 0x74, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn ui_vector_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a3280__generated,
            ui_vector_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a3280, 0);
                let v = DATA + 0x1000;
                c.g[A0][0] = v as u64;
                w32(c, v + 0x14, r.next_u32());
                w32(c, v + 0x74, r.next_u32());
            },
        );
    }

    #[test]
    fn cui_text_scroll_set_text_id_matches() {
        check(
            ee::s13::f_003a1f18__generated,
            cui_text_scroll_set_text_id,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a1f18, 3);
                be_library_2(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                w(c, obj + 0x5c, DATA + 0x1000);
                w(c, DATA + 0x1000 + 0xd0, DATA + 0x1100);
                w(c, DATA + 0x1100 + 0x10, DATA + 0x1200);
                let present = seed % 2 == 0;
                w(
                    c,
                    DATA + 0x1200 + 0x10,
                    if present { DATA + 0x1300 } else { 0 },
                );
                let vt = DATA + 0x3000;
                w(c, DATA + 0x1300 + 4, vt);
                slot(c, vt, 0x20, 0x30, FAKE);
                w(c, obj + 8, vt);
                slot(c, vt, 0x28, 0x40, FAKE + 0x40);
            },
        );
    }

    #[test]
    fn c_ui_text_scroll_add_line_matches() {
        check(
            ee::s13::f_003a1bf8__generated,
            c_ui_text_scroll_add_line,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a1bf8, 3);
                pool_allocator(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                // A short UTF-16 line.
                let n = 1 + seed % 4;
                for i in 0..n {
                    c.bus.write16(DATA + 0x400 + 2 * i, b'a' as u16 + i as u16);
                }
                c.bus.write16(DATA + 0x400 + 2 * n, 0);
                // An empty line table: GrowLines allocates it.
                c.bus.write32(DATA + 0xbc, 0);
                c.bus.write16(DATA + 0xc8, 0);
            },
        );
    }

    #[test]
    fn ui_text_set_ascii_string_matches() {
        check(
            ee::s13::f_003a0c70__generated,
            ui_text_set_ascii_string,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x3a_0c70);
                globals(c);
                let vt = vtable(c, &mut r, &[0x28, 0xc0]);
                w(c, DATA + 8, vt);
                // A short ASCII string.
                let n = (seed % 20) as u32;
                for i in 0..n {
                    c.bus.write8(DATA + 0x200 + i, b'a' + (i % 26) as u8);
                }
                c.bus.write8(DATA + 0x200 + n, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn ui_text_scroll_clear_lines_matches() {
        check(
            ee::s13::f_003a18c0__generated,
            ui_text_scroll_clear_lines,
            SEEDS_4,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let count = seed % 4;
                if seed & 1 == 0 {
                    w(c, this + 0xbc, 0);
                } else {
                    let array = alloc_2(c, 0x40);
                    w(c, this + 0xbc, array);
                    wh(c, this + 0xc8, count as u16);
                    for i in 0..count {
                        let entry = if seed & 2 == 0 { 0 } else { alloc_2(c, 0x40) };
                        w(c, array + 4 * i, entry);
                    }
                }
                c.g[A0][0] = this as u64;
            }),
        );
    }

    // ------------------------------------------------------------- text scroll

    #[test]
    fn c_ui_text_scroll_grow_lines_matches() {
        check(
            ssx3_ee::s13::f_003a1958__generated,
            c_ui_text_scroll_grow_lines,
            12,
            &|c, seed| {
                let _ = base_3(c, seed, 0x003a_1958);
                heap(c);
                pool(c);
                let obj = DATA + 0x100;
                clear(c, obj, 0x100);
                let old = if seed % 3 == 0 { 0 } else { DATA + 0x40_0400 };
                w(c, obj + 0xbc, old);
                c.bus.write16(obj + 0xc8, (seed % 4) as u16);
                c.g[A0][0] = sx(obj);
            },
        );
    }

    #[test]
    fn ui_vector_dtor_matches() {
        check(
            ee::s13::f_003a32f0__generated,
            ui_vector_dtor,
            10,
            &|c, seed| {
                let _ = base_4(c, seed, 0x3a32f0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, OBJ + 0x7c, if seed % 2 == 0 { 0 } else { DST });
                w(c, OBJ + 0x80, if seed % 3 == 0 { 0 } else { DST });
                w(c, OBJ + 0x84, 0);
                w(c, OBJ + 0x88, DST);
                w(c, OBJ + 0x8c, 0);
            },
        );
    }

    #[test]
    fn ui_text_get_text_size_matches() {
        strict(
            ee::s13::f_003a1030__generated,
            ui_text_get_text_size,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a10, 6);
                let this = DATA;
                c.bus
                    .write32(this + 0xb4, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0x14, r.next_u32());
                c.bus.write32(this + 8, DATA + 0x1000);
                c.bus.write16(DATA + 0x1000 + 0x80, 0);
                c.bus.write32(DATA + 0x1000 + 0x84, FAKE);
                c.bus.write32(this + 0x50, r.edge_f32());
                c.bus.write32(this + 0x54, r.edge_f32());
                c.bus.write32(DATA + 0x2000 + 8, r.next_u32());
                c.bus.write32(DATA + 0x2000, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    // -- 0x003a13e0 cUITextScroll_HandleInput ---------------------------

    #[test]
    fn cuit_ext_scroll_handle_input_matches() {
        check(
            ssx3_ee::s13::f_003a13e0__generated,
            cuit_ext_scroll_handle_input,
            120,
            &|c, seed| {
                let r = fakes::machine(c, seed, 0x3a13e0, 3);
                let this = DATA + 0x1000;
                let input = DATA + 0x2000;
                let fman = DATA + 0x3000;
                let ftab = DATA + 0x3100;
                let fobj = DATA + 0x3200;
                let font = DATA + 0x3300;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(input);
                // font lookup
                w(c, this + 0x5c, fman);
                w(c, fman + 0xd0, ftab);
                w(c, ftab + 0x10, fobj);
                w(c, fobj + 8, font);
                w(c, this + 0x74, (seed % 2) as u32);
                w(c, font + 0xc, DATA + 0x3400);
                w(c, font + 0x18, DATA + 0x3400);
                // the box's own fields
                wf(c, this + 0x54, 2.0f32.to_bits());
                wf(c, this + 0x64, 100.0f32.to_bits());
                wf(c, font + 0x14, 16.0f32.to_bits());
                wf(c, font + 0x34, 1.0f32.to_bits());
                c.bus.write16(this + 0xc4, (seed % 7) as u16);
                c.bus.write16(this + 0xc6, (seed % 5) as u16);
                c.bus.write16(this + 0xc8, 9);
                c.bus.write8(this + 0x88, 1);
                c.bus.write8(this + 0x89, 2);
                // input object: vtable-ish words
                w(c, input + 8, DATA + 0x3500);
                w(c, DATA + 0x3500 + 0x10, 0x10);
                w(c, DATA + 0x3500 + 0x14, FAKE);
                w(c, DATA + 0x3500 + 0x20, 0);
                // one of the two key checks reports "pressed"
                w(c, LOG_RESULTS + 4 * ((seed % 2) as u32), 1);
                let _ = r;
            },
        );
    }

    use crate::difftest::GAME_GP;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // ---------------------------------------------------- UIText_Render

    pub(crate) const UTR_THIS: u32 = DATA;

    pub(crate) const UTR_OWNER: u32 = DATA + 0x200;

    pub(crate) const UTR_P1: u32 = DATA + 0x300;

    pub(crate) const UTR_P2: u32 = DATA + 0x380;

    pub(crate) const UTR_OV: u32 = DATA + 0x400;

    pub(crate) const UTR_FONTS: u32 = DATA + 0x480;

    pub(crate) const UTR_FONT: u32 = DATA + 0x500;

    pub(crate) const UTR_GRMAN: u32 = DATA + 0x600;

    pub(crate) const UTR_STATE: u32 = DATA + 0x700;

    pub(crate) const UTR_DRAWER: u32 = DATA + 0x780;

    pub(crate) const UTR_STR: u32 = DATA + 0x800;

    pub(crate) const UTR_LINE_TABLE: u32 = DATA + 0x880;

    pub(crate) const UTR_LINES: u32 = DATA + 0x900;

    pub(crate) const UTR_VT: u32 = DATA + 0x1000;

    pub(crate) const UTR_GLYPHS: u32 = DATA + 0x2000;

    pub(crate) const UTR_BUF: u32 = DATA + 0x3000;

    /// Gate of the allocator/format/free fakes (+4: make the allocation fail).
    pub(crate) const UTR_CTRL: u32 = DATA + 0xf_f000;

    pub(crate) const UTR_MAGIC: u32 = 0x5554_5852;

    pub(crate) static UTR_PREV_NEW: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static UTR_PREV_FMT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static UTR_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn utr_fake(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if addr == 0x0031_7e30 && c.pc == ra && c.bus.read32(UTR_CTRL + 4) != 0 {
            c.g[V0][0] = 0;
        }
    }

    pub(crate) fn utr_fake_new(c: &mut Cpu) {
        if c.bus.read32(UTR_CTRL) != UTR_MAGIC {
            return fakes::fall_back(c, &UTR_PREV_NEW, ee::s10::f_00317e30__generated);
        }
        utr_fake(c, 0x0031_7e30)
    }

    pub(crate) fn utr_fake_fmt(c: &mut Cpu) {
        if c.bus.read32(UTR_CTRL) != UTR_MAGIC {
            return fakes::fall_back(c, &UTR_PREV_FMT, ee::s08::f_002c2718__generated);
        }
        utr_fake(c, 0x002c_2718)
    }

    pub(crate) fn utr_fake_free(c: &mut Cpu) {
        if c.bus.read32(UTR_CTRL) != UTR_MAGIC {
            return fakes::fall_back(c, &UTR_PREV_FREE, ee::s10::f_00317e98__generated);
        }
        utr_fake(c, 0x0031_7e98)
    }

    pub(crate) fn utr_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            utr_fake_new,
            &UTR_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2718),
            utr_fake_fmt,
            &UTR_PREV_FMT,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            utr_fake_free,
            &UTR_PREV_FREE,
        );
    }

    /// A short wide string of printable characters.
    pub(crate) fn utr_wide(c: &mut Cpu, r: &mut crate::difftest::Rng, at: u32, max: u32) {
        let n = r.next_u32() % (max + 1);
        for i in 0..n {
            c.bus
                .write16(at + 2 * i, (0x20 + r.next_u32() % 0x5f) as u16);
        }
        c.bus.write16(at + 2 * n, 0);
    }

    /// `UIText_Render` against the generated body: built/unbuilt string
    /// (vtable rebuild), the owner's 'W' override (box widths below 0,
    /// small, and past the 0x400 buffer), missing fonts, the three colour
    /// choices, shadow on/off, 0..3 lines and the formatted multi-line path
    /// (with a failing allocation). Font measuring, alignment and the 'W'
    /// width run real; the print's vtable call, the allocator, the format
    /// and the free are fakes.
    #[test]
    fn ui_text_render_matches() {
        utr_install_fakes();
        check(
            ee::s13::f_003a05e8__generated,
            ui_text_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_05e8, 6);
                c.pc = 0x003a_05e8;
                c.bus.write32(UTR_CTRL, UTR_MAGIC);
                c.bus.write32(UTR_CTRL + 4, (seed % 5 == 0) as u32);
                for i in 0..8 {
                    c.bus.write32(LOG_RESULTS + 4 * i, UTR_BUF);
                }
                // The formatted buffer holds a short string.
                utr_wide(c, &mut r, UTR_BUF, 4);
                let this = UTR_THIS;
                c.g[A0][0] = sx(this);
                c.f[12] = r.f32(300.0);
                c.f[13] = r.f32(300.0);
                for k in 0..0x30 {
                    c.bus.write32(this + 4 * k, r.next_u32());
                }
                // Rebuild vtable slot 16 (+0x80).
                c.bus.write32(this + 8, UTR_VT);
                c.bus.write16(UTR_VT + 0x80, (r.next_u32() as u16) & 0xfc);
                c.bus.write32(UTR_VT + 0x84, FAKE);
                c.bus.write32(
                    this + 0x50,
                    (0.5 + (r.next_u32() % 4) as f32 * 0.5).to_bits(),
                );
                c.bus.write32(this + 0x54, r.f32(2.0));
                c.bus.write32(this + 0x44, r.f32(200.0));
                c.bus.write32(this + 0x48, r.f32(200.0));
                // Box width: around zero, far past the 0x400 'W's, or small.
                let width = match seed % 7 {
                    0 => r.f32(50.0),
                    1 => 1.0e7f32.to_bits(),
                    _ => ((r.next_u32() % 400) as f32).to_bits(),
                };
                c.bus.write32(this + 0x60, width);
                c.bus.write32(this + 0x5c, UTR_OWNER);
                let lines = match seed % 9 {
                    0 => 0,
                    1 => -1i32 as u32,
                    n => 1 + n % 3,
                };
                c.bus.write32(this + 0xa8, lines);
                c.bus.write32(this + 0xac, UTR_LINE_TABLE);
                let has_str = seed % 11 != 0;
                c.bus
                    .write32(this + 0xb4, if has_str { UTR_STR } else { 0 });
                utr_wide(c, &mut r, UTR_STR, 6);
                for i in 0..4 {
                    let at = UTR_LINES + 0x40 * i;
                    c.bus.write32(UTR_LINE_TABLE + 4 * i, at);
                    utr_wide(c, &mut r, at, 6);
                }
                // Owner -> +0xd0 -> +0x10: the font list and the override flag.
                c.bus.write32(UTR_OWNER + 0xd0, UTR_P1);
                c.bus.write32(UTR_P1 + 0x10, UTR_P2);
                c.bus.write32(UTR_P2 + 8, UTR_FONTS);
                let ov = if seed % 3 == 0 { 0 } else { UTR_OV };
                c.bus.write32(UTR_P2 + 0x10, ov);
                c.bus.write32(UTR_OV, r.next_u32());
                let no_font = seed % 13 == 5;
                for i in 0..4 {
                    c.bus
                        .write32(UTR_FONTS + 12 * i + 0xc, if no_font { 0 } else { UTR_FONT });
                }
                // The font: glyphs ' '..'~', 12 bytes each.
                let font = UTR_FONT;
                for k in 0..0x18 {
                    c.bus.write32(font + 4 * k, r.next_u32());
                }
                c.bus.write32(font + 4, UTR_GLYPHS);
                c.bus.write32(font + 0xc, 0x20);
                c.bus.write32(font + 0x10, 0x7e);
                c.bus.write32(font + 0x14, 6 + r.next_u32() % 20);
                for k in [0x18, 0x1c, 0x20, 0x24, 0x30, 0x34] {
                    c.bus.write32(font + k, r.f32(3.0));
                }
                for g in 0..0x5f {
                    let at = UTR_GLYPHS + 12 * g;
                    for b in 0..12 {
                        c.bus.write8(at + b, (r.next_u32() % 24) as u8);
                    }
                }
                if seed % 17 == 3 {
                    // A zero-width 'W'.
                    c.bus.write8(UTR_GLYPHS + 12 * (0x57 - 0x20) + 8, 0);
                }
                // The graphics manager: draw state and the printer's vtable.
                c.bus.write32(GAME_GP - UI_TEXT_RENDER_GRMAN_GP, UTR_GRMAN);
                c.bus.write32(GAME_GP + 0x2a90, UTR_GRMAN);
                c.bus.write32(UTR_GRMAN + 0xe84, UTR_STATE);
                c.bus.write32(UTR_GRMAN + 0x10d8, UTR_DRAWER);
                c.bus.write32(UTR_STATE + 8, r.next_u32());
                c.bus
                    .write16(UTR_DRAWER + 0x288, (r.next_u32() as u16) & 0xfc);
                c.bus.write32(UTR_DRAWER + 0x28c, FAKE + 0x10);
                c.bus
                    .write32(GAME_GP + UI_TEXT_RENDER_SHADOW_OFS_GP, r.next_u32());
                c.bus
                    .write32(GAME_GP + UI_TEXT_RENDER_SHADOW_OFS_GP + 4, r.next_u32());
            },
        );
    }

    /// Gate word for this module's fakes (its own machine's RAM).
    pub(crate) const OC445_CTRL: u32 = DATA + 0xf_0000;

    pub(crate) const OC445_MAGIC: u32 = 0x0445_c0de;

    /// Count of `new` calls so far.
    pub(crate) const OC445_NEW_COUNT: u32 = OC445_CTRL + 4;

    /// Index of the next `UIScreen_CreateObject` result.
    pub(crate) const OC445_RESULT_NEXT: u32 = OC445_CTRL + 8;

    /// Open this test's fakes (both machines of a comparison get the gate).
    pub(crate) fn oc445_gate(c: &mut Cpu) {
        oc445_install_fakes();
        c.bus.write32(OC445_CTRL, OC445_MAGIC);
        c.bus.write32(OC445_NEW_COUNT, 0);
        c.bus.write32(OC445_RESULT_NEXT, 0);
    }

    /// A vtable with fake methods in the given slots, random deltas.
    pub(crate) fn oc445_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, slots: &[u32]) {
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
    }

    pub(crate) const OC445_OBJ: u32 = DATA + 0x1000;

    pub(crate) const OC445_VT: u32 = DATA + 0x4000;

    pub(crate) const OC445_STR: u32 = DATA + 0x7000;

    /// `UIText_SetTextId`: stores the id, calls vtable slot 0x28.
    #[test]
    fn ui_text_set_text_id_matches() {
        fakes::covered(
            ee::s13::f_003a1000__generated,
            ui_text_set_text_id,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a1000, 1);
                oc445_vtable(c, &mut r, OC445_VT, &[0x28]);
                c.bus.write32(OC445_OBJ + 8, OC445_VT);
                c.g[A0][0] = sx(OC445_OBJ);
                c.pc = 0x003a_1000;
            },
        );
    }

    /// `UIText_SetWideStringInternal`: empty, single-line, multi-line
    /// (double backslash and CR separators, lone backslashes), exactly 800
    /// units and too long (801) strings.
    #[test]
    fn ui_text_set_wide_string_internal_matches() {
        fakes::covered(
            ee::s13::f_003a0eb0__generated,
            ui_text_set_wide_string_internal,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a0eb0, 5);
                oc445_gate(c);
                oc445_vtable(c, &mut r, OC445_VT, &[0x28, 0xc0]);
                c.bus.write32(OC445_OBJ + 8, OC445_VT);
                c.bus.write32(OC445_OBJ + 0xb4, r.next_u32());
                // The line separator the game keeps at gp+0x16b8.
                c.bus.write16(GAME_GP + 0x16b8, 0x5c);
                let alphabet: [u16; 5] = [0x61, 0x5c, 0x5c, 13, 0x62];
                let units: Vec<u16> = match seed % 9 {
                    0 => vec![],
                    1 => vec![0x61, 0x62, 0x63],
                    2 => vec![0x61, 0x5c, 0x5c, 0x62],
                    3 => vec![0x61, 13, 0x62, 13, 13, 0x63],
                    4 => vec![0x61, 0x5c, 0x78, 0x62],
                    5 => vec![0x61; 801],
                    6 => {
                        let mut v = vec![0x61; 796];
                        v.extend([0x5c, 0x5c, 0x62, 13]);
                        v
                    }
                    _ => (0..1 + r.next_u32() % 30)
                        .map(|_| alphabet[(r.next_u32() % 5) as usize])
                        .collect(),
                };
                for (i, &u) in units.iter().enumerate() {
                    c.bus.write16(OC445_STR + 2 * i as u32, u);
                }
                c.bus.write16(OC445_STR + 2 * units.len() as u32, 0);
                c.g[A0][0] = sx(OC445_OBJ);
                c.g[A1][0] = sx(OC445_STR);
                c.pc = 0x003a_0eb0;
            },
        );
    }

    /// `cUIText_setUnicodeStringWrap`: a short wide string (possibly with
    /// spaces) against a zeroed text object whose vtable's font method is a
    /// fake. The font lookup (`UIText_GetFont`) and the glyph advance are the
    /// real functions; the allocator is the test heap.
    #[test]
    fn cui_text_set_unicode_string_wrap_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        const SELF: u32 = DATA + 0xa000;
        const VT: u32 = DATA + 0xa200;
        const STR: u32 = DATA + 0xa800;
        check_lohi(
            ee::s13::f_003a0d00__generated,
            cui_text_set_unicode_string_wrap,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003a_0d00, 0);
                crate::game::ai::computer::tests::rbc_test_allocator(c);
                c.g[RA][0] = 0x0012_3450;
                c.pc = 0x003a_0d00;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(STR);
                c.bus.write32(SELF + 0x8, VT);
                c.bus.write16(VT + 0x28, 0);
                c.bus.write32(VT + 0x2c, FAKE);
                c.bus.write32(SELF + 0x50, r.f32(2.0));
                c.bus.write32(SELF + 0x60, r.f32(2.0));
                c.bus.write16(GAME_GP + 0x16b8, 0x0a);
                let len = r.next_u32() % 20;
                for i in 0..len {
                    let ch = if r.next_u32() % 4 == 0 {
                        0x20
                    } else {
                        0x41 + (r.next_u32() % 26) as u16
                    };
                    c.bus.write16(STR + 2 * i, ch);
                }
                c.bus.write16(STR + 2 * len, 0);
            },
        );
    }

    #[test]
    fn ui_text_count_lines_matches() {
        check(
            ee::s13::f_003a0b90__generated,
            ui_text_count_lines,
            300,
            &|c, seed| {
                let mut r = Rng(0x003a_0b90 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003a_0b90;
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                let text = DATA + 0x1000;
                c.bus
                    .write32(DATA + 0xb4, if seed % 9 == 0 { 0 } else { text });
                let a2: u16 = [0x0a, 0x0d, 0x61][(seed % 3) as usize];
                c.bus.write16(GAME_GP + 0x16b8, a2);
                let n = if seed % 7 == 0 { 0 } else { r.next_u32() % 16 };
                for i in 0..n {
                    let ch: u16 = match r.next_u32() % 4 {
                        0 | 1 => a2,
                        2 => 0x0d,
                        _ => 0x61,
                    };
                    c.bus.write16(text + 2 * i, ch);
                }
                c.bus.write16(text + 2 * n, 0);
            },
        );
    }

    use crate::difftest::fakes::covered;
    pub(crate) use crate::game::sound::soundsys::tests::OC756_BASE;
    pub(crate) use crate::game::sound::soundsys::tests::OC756_CTRL;
    pub(crate) use crate::game::sound::soundsys::tests::OC756_MAGIC;

    /// `UIText_SetString`: clears the +0xb0 word of the text and calls the
    /// (faked, sometimes unwinding) `UIText_SetAsciiString`.
    #[test]
    fn ui_text_set_string_matches() {
        oc756_install_fakes();
        covered(
            ee::s13::f_003a0c50__generated,
            ui_text_set_string,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a0c50, 1);
                c.pc = 0x003a_0c50;
                c.bus.write32(OC756_CTRL, OC756_MAGIC);
                let text = OC756_BASE + 0x1000;
                c.bus.write32(text + UI_TEXT_SET_STRING_CACHE, r.next_u32());
                c.g[A0][0] = text as u64;
            },
        );
    }

    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fill;

    pub(crate) use crate::game::ui::uiscreen::tests::OC1081_FONT_RESULT;

    pub(crate) use crate::game::ui::uiscreen::tests::oc1081_gate;
    pub(crate) use crate::game::ui::uiscreen::tests::oc1081_vtable;
    pub(crate) use crate::game::ui::uiscreen::tests::oc1083_put_slot;
    pub(crate) use crate::game::ui::uiscreen::tests::oc1083_slot_delta;
    pub(crate) use crate::game::ui::uiscreen::tests::OC1081_SELF;
    pub(crate) use crate::game::ui::uiscreen::tests::OC1081_VT;
    pub(crate) use crate::game::ui::uiscreen::tests::SEEDS_9 as SEEDS_5;

    /// Gate word of this test's quad fakes (0 = fall back to the real callee).
    pub(crate) const OC1077_CTRL: u32 = DATA + 0x34_0000;

    pub(crate) const OC1077_MAGIC: u32 = 0x1077_0c01;

    /// Call counter; the call whose index equals the word at `+8` unwinds.
    pub(crate) const OC1077_COUNT: u32 = OC1077_CTRL + 4;

    pub(crate) const OC1077_UNWIND_AT: u32 = OC1077_CTRL + 8;

    pub(crate) static OC1077_PREV_UV: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1077_PREV_UV_ROT90: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1077_fake_body(c: &mut Cpu) {
        let n = c.bus.read32(OC1077_COUNT);
        c.bus.write32(OC1077_COUNT, n + 1);
        c.pc = if n == c.bus.read32(OC1077_UNWIND_AT) {
            0x0012_3450
        } else {
            c.g[RA][0] as u32
        };
    }

    pub(crate) fn oc1077_fake_uv(c: &mut Cpu) {
        if c.bus.read32(OC1077_CTRL) != OC1077_MAGIC {
            return fakes::fall_back(c, &OC1077_PREV_UV, ee::s12::f_0038e5e8__generated);
        }
        oc1077_fake_body(c);
    }

    pub(crate) fn oc1077_fake_uv_rot90(c: &mut Cpu) {
        if c.bus.read32(OC1077_CTRL) != OC1077_MAGIC {
            return fakes::fall_back(c, &OC1077_PREV_UV_ROT90, ee::s12::f_0038e738__generated);
        }
        oc1077_fake_body(c);
    }

    pub(crate) fn oc1077_hook() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038E5E8),
            oc1077_fake_uv,
            &OC1077_PREV_UV,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038E738),
            oc1077_fake_uv_rot90,
            &OC1077_PREV_UV_ROT90,
        );
    }

    pub(crate) const OC1077_O: u32 = DATA + 0x1234;

    pub(crate) const OC1077_R: u32 = DATA + 0x12_0000;

    pub(crate) const OC1077_C: u32 = DATA + 0x13_0000;

    pub(crate) const OC1077_GFX: u32 = DATA + 0x20_0000;

    pub(crate) const OC1077_TOP: u32 = DATA + 0x21_0000;

    /// Tile counts by seed: non-positive (early exits), under one, whole and
    /// fractional, so rows, clipped columns and the clipped last row all occur.
    pub(crate) const OC1077_TILES: [f32; 8] = [-1.0, 0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];

    pub(crate) fn oc1077_setup(c: &mut Cpu, seed: u32, unwind_at: u32) {
        let mut r = machine(c, seed, 0x3a_23b8, 0);
        scramble(c, &mut r);
        c.bus.write32(OC1077_CTRL, OC1077_MAGIC);
        c.bus.write32(OC1077_COUNT, 0);
        c.bus.write32(OC1077_UNWIND_AT, unwind_at);
        c.pc = 0x003a_23b8;
        c.bus.write32(OC1077_O, (seed % 8) as u32);
        for off in [0xcu32, 0x10, 0x14, 0x18] {
            c.bus.write32(OC1077_O + off, r.f32(1.0));
        }
        for i in 0..4u32 {
            c.bus.write32(OC1077_R + 4 * i, r.f32(100.0));
            c.bus.write32(OC1077_C + 4 * i, r.f32(1.0));
        }
        c.g[A0][0] = sx(OC1077_O);
        c.g[A1][0] = sx(OC1077_R);
        c.g[A2][0] = sx(OC1077_C);
        c.g[A3][0] = (seed / 3 % 8) as u64;
        c.f[12] = OC1077_TILES[(seed / 5 % 8) as usize].to_bits();
        c.f[13] = OC1077_TILES[(seed / 40 % 8) as usize].to_bits();
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP - 0x854, OC1077_GFX);
        c.bus.write32(OC1077_GFX + 0xe84, OC1077_TOP);
        c.bus.write32(OC1077_TOP + 0x10, 0);
        c.bus.write32(GAME_GP - 0x844, OC1077_GFX + 0x100);
        fill(c, &mut r, OC1077_GFX + 0x200, 0x20);
        fill(c, &mut r, OC1077_TOP + 0x20, 0x20);
    }

    /// All flag combinations (seed / 3 % 8) against every tx and ty class
    /// (seed / 5 % 8, seed / 40 % 8), plain and rot90 quad calls.
    #[test]
    fn render2d_draw_tiled_sprite_matches() {
        oc1077_hook();
        check(
            ee::s13::f_003a23b8__generated,
            render2d_draw_tiled_sprite,
            512,
            &|c, seed| oc1077_setup(c, seed, u32::MAX),
        );
    }

    /// Every quad call in turn unwinds (the caller returns at once).
    #[test]
    fn render2d_draw_tiled_sprite_unwinds() {
        oc1077_hook();
        check(
            ee::s13::f_003a23b8__generated,
            render2d_draw_tiled_sprite,
            256,
            &|c, seed| oc1077_setup(c, seed * 7 + 1, seed % 6),
        );
    }

    #[test]
    fn render2d_draw_tiled_sprite_events() {
        oc1077_hook();
        driven!(gen_tiled, ee::s13::f_003a23b8__generated);
        driven!(hand_tiled, render2d_draw_tiled_sprite);
        check_events(gen_tiled, hand_tiled, 120, &|c, seed| {
            oc1077_setup(c, 270 + seed / 3 % 15, u32::MAX);
            schedule(
                c,
                &mut Rng(0x3a_23b8 ^ (seed as u64) << 8),
                seed,
                0x003a_23b8,
            )
        });
    }

    /// Gate word of the oc1078 draw fakes (own RAM word, both sides get their own).
    pub(crate) const OC1078_CTRL: u32 = DATA + 0xe_0000;

    pub(crate) const OC1078_MAGIC: u32 = 0x1078_c0de;

    /// Number of logged draw calls, then the records.
    pub(crate) const OC1078_LOG_N: u32 = OC1078_CTRL + 4;

    pub(crate) const OC1078_LOG: u32 = OC1078_CTRL + 0x100;

    pub(crate) const OC1078_ELEM: u32 = DATA + 0x1000;

    pub(crate) const OC1078_DST: u32 = DATA + 0x1100;

    pub(crate) const OC1078_COL: u32 = DATA + 0x1200;

    pub(crate) const OC1078_XF: u32 = DATA + 0x1300;

    pub(crate) const OC1078_GRMAN: u32 = DATA + 0x2000;

    pub(crate) const OC1078_DRAWSTATE: u32 = DATA + 0x3000;

    pub(crate) static OC1078_PREV_QUAD: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1078_PREV_ROT: AtomicUsize = AtomicUsize::new(0);

    /// A gated fake draw call: logs the rect, uv and transform the caller built
    /// on its stack (the real callees read them), then the usual fake.
    pub(crate) fn oc1078_fake_draw(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(OC1078_LOG_N);
        c.bus.write32(OC1078_LOG_N, n + 1);
        let p = OC1078_LOG + n * 0x80;
        c.bus.write32(p, target);
        let (a1, a2, a3, t0) = (
            c.g[A1][0] as u32,
            c.g[A2][0] as u32,
            c.g[A3][0] as u32,
            c.g[T0][0] as u32,
        );
        for i in 0..4 {
            let w = c.bus.read32(a2 + 4 * i);
            c.bus.write32(p + 0x10 + 4 * i, w);
            let w = c.bus.read32(a3 + 4 * i);
            c.bus.write32(p + 0x20 + 4 * i, w);
        }
        for i in 0..9 {
            let w = c.bus.read32(t0 + 4 * i);
            c.bus.write32(p + 0x30 + 4 * i, w);
        }
        for i in 0..8 {
            let w = c.bus.read32(a1 + 4 * i);
            c.bus.write32(p + 0x58 + 4 * i, w);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
    }

    pub(crate) fn oc1078_fake_quad(c: &mut Cpu) {
        if c.bus.read32(OC1078_CTRL) != OC1078_MAGIC {
            return fakes::fall_back(c, &OC1078_PREV_QUAD, ee::s12::f_0038e880__generated);
        }
        oc1078_fake_draw(c, 0x0038_e880)
    }

    pub(crate) fn oc1078_fake_rot(c: &mut Cpu) {
        if c.bus.read32(OC1078_CTRL) != OC1078_MAGIC {
            return fakes::fall_back(c, &OC1078_PREV_ROT, ee::s12::f_0038ea60__generated);
        }
        oc1078_fake_draw(c, 0x0038_ea60)
    }

    pub(crate) fn oc1078_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038E880),
            oc1078_fake_quad,
            &OC1078_PREV_QUAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038EA60),
            oc1078_fake_rot,
            &OC1078_PREV_ROT,
        );
    }

    /// `Render2D_DrawTiledSpriteXform`: tile counts from none (tx or ty <= 0, the
    /// two early exits) to 3.x by 3.x, whole and fractional, each flag
    /// combination (u flip, v flip, rotated quads), random colour, rect, uv.
    #[test]
    fn render2d_draw_tiled_sprite_xform_matches() {
        let counts: [f32; 9] = [-1.0, 0.0, 0.5, 1.0, 1.5, 2.0, 2.75, 3.0, 3.25];
        fakes::covered(
            ee::s13::f_003a2bc8__generated,
            render2d_draw_tiled_sprite_xform,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a2bc8, 12);
                oc1078_install_fakes();
                c.bus.write32(OC1078_CTRL, OC1078_MAGIC);
                c.bus.write32(OC1078_LOG_N, 0);
                let flt = |r: &mut Rng| {
                    if seed % 5 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(100.0)
                    }
                };
                c.bus.write32(GAME_GP.wrapping_sub(0x854), OC1078_GRMAN);
                c.bus.write32(OC1078_GRMAN + 0xe84, OC1078_DRAWSTATE);
                c.bus.write32(GAME_GP.wrapping_sub(0x844), r.next_u32());
                c.bus.write32(OC1078_ELEM, r.next_u32() & 0xffff);
                for off in [0xc, 0x10, 0x14, 0x18] {
                    let v = flt(&mut r);
                    c.bus.write32(OC1078_ELEM + off, v);
                }
                for i in 0..4 {
                    let v = flt(&mut r);
                    c.bus.write32(OC1078_DST + 4 * i, v);
                }
                for i in 0..9 {
                    let v = if i < 4 { r.f32(1.5) } else { r.next_u32() };
                    c.bus.write32(OC1078_COL + 4 * i, v);
                }
                for i in 0..4 {
                    let v = r.next_u32();
                    c.bus.write32(OC1078_XF + 4 * i, v);
                }
                c.g[A0][0] = ssx3_ee_rt::cpu::sx(OC1078_ELEM);
                c.g[A1][0] = ssx3_ee_rt::cpu::sx(OC1078_DST);
                c.g[A2][0] = ssx3_ee_rt::cpu::sx(OC1078_COL);
                c.g[A3][0] = ssx3_ee_rt::cpu::sx(OC1078_XF);
                c.g[T0][0] = (seed / 9 % 8) as u64 | (r.next_u32() as u64 & 0x1000_0000_0000_0000);
                c.f[12] = counts[(seed % 9) as usize].to_bits();
                c.f[13] = counts[((seed / 3 + seed / 27) % 9) as usize].to_bits();
                c.pc = 0x003a_2bc8;
            },
        );
    }

    pub(crate) const OC1081_FONT: u32 = DATA + 0x6000;

    pub(crate) const OC1081_GRMAN: u32 = DATA + 0x7000;

    pub(crate) const OC1081_STATE: u32 = DATA + 0x9000;

    pub(crate) const OC1081_SCROLLW: u32 = DATA + 0x9400;

    pub(crate) const OC1081_STRS: u32 = DATA + 0x9800;

    /// `cUITextScroll_Render`: with and without the pre-render call, no font,
    /// shadow on or off, selection colour, rows from none to a negative
    /// (wrapped) byte, lines cut by the count, with and without the scroll
    /// bar.
    #[test]
    fn cuitextscroll_render_matches() {
        fakes::covered(
            ee::s13::f_003a1588__generated,
            cuitextscroll_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a1588, 12);
                oc1081_gate(c);
                oc1081_vtable(c, &mut r);
                let me = OC1081_SELF;
                for k in 0..0x40 {
                    c.bus.write32(me + 4 * k, r.next_u32());
                }
                c.bus.write32(me + 8, OC1081_VT);
                // Flags: bit 2 of +0x14 (skip the pre-render call), the layer byte.
                let f14 = (r.next_u32() & !(1 << 2)) | if seed % 3 == 0 { 1 << 2 } else { 0 };
                c.bus.write32(me + 0x14, f14);
                // +0x74: bit 0 selection colouring, bit 3 shadow.
                let f74 = (r.next_u32() & !9)
                    | if seed % 2 == 0 { 1 } else { 0 }
                    | if seed % 5 < 3 { 8 } else { 0 };
                c.bus.write32(me + 0x74, f74);
                c.bus
                    .write32(me + 0x50, (0.5 + (r.next_u32() % 4) as f32 * 0.5).to_bits());
                c.bus
                    .write32(me + 0x54, (0.5 + (r.next_u32() % 4) as f32 * 0.5).to_bits());
                c.bus.write32(me + 0x44, r.f32(200.0));
                c.bus.write32(me + 0x48, r.f32(200.0));
                c.bus.write32(me + 0xcc, r.f32(5.0));
                let height = match seed % 8 {
                    0 => 0.0f32,
                    1 => 3000.0,
                    2 => -50.0,
                    _ => 30.0 + (r.next_u32() % 250) as f32,
                };
                c.bus.write32(me + 0x64, height.to_bits());
                // Lines: top, count and current selection.
                let count = if seed % 10 == 4 {
                    0
                } else {
                    1 + r.next_u32() % 20
                };
                let top = r.next_u32() % 22;
                let cur = if seed % 4 == 1 {
                    top + r.next_u32() % 3
                } else {
                    r.next_u32() % 22
                };
                c.bus.write16(me + CUITEXTSCROLL_TOP, top as u16);
                c.bus.write16(me + CUITEXTSCROLL_COUNT, count as u16);
                c.bus.write16(me + 0xc4, cur as u16);
                c.bus.write32(me + 0xbc, OC1081_STRS);
                for k in 0..0x80 {
                    c.bus.write32(
                        OC1081_STRS + 4 * k,
                        DATA + 0x400 + 0x10 * (r.next_u32() % 40),
                    );
                }
                let scroll = if seed % 3 == 1 { 0 } else { OC1081_SCROLLW };
                c.bus.write32(me + CUITEXTSCROLL_SCROLL, scroll);
                // The font: none, or sizes in a plausible range.
                let font = if seed % 17 == 6 { 0 } else { OC1081_FONT };
                c.bus.write32(OC1081_FONT_RESULT, font);
                c.bus.write32(OC1081_FONT + 0x14, 6 + r.next_u32() % 20);
                c.bus.write32(OC1081_FONT + 0x30, r.f32(3.0));
                c.bus.write32(
                    OC1081_FONT + 0x34,
                    (0.5 + (r.next_u32() % 4) as f32 * 0.5).to_bits(),
                );
                for k in 0..0x30 {
                    c.bus.write32(OC1081_FONT + 0x38 + 4 * k, r.next_u32());
                }
                // The graphics manager and its draw-state word.
                c.bus
                    .write32(crate::difftest::GAME_GP - 0x854, OC1081_GRMAN);
                c.bus.write32(OC1081_GRMAN + 0xe84, OC1081_STATE);
                c.bus.write32(OC1081_STATE + 8, r.next_u32());
                // Floats on the stack the measure fake leaves untouched.
                let frame = c.g[SP][0] as u32 - 0xd0;
                for k in 0..0x30 {
                    c.bus.write32(frame + 4 * k, r.f32(100.0));
                }
                c.g[A0][0] = sx(me);
                c.f[12] = r.f32(100.0);
                c.f[13] = r.f32(100.0);
                c.pc = 0x003a_1588;
            },
        );
    }

    /// Gate words of the callee fakes: each answers only while its word holds
    /// `OC1083_MAGIC`. Placed above the pointer area.
    pub(crate) const OC1083_LOAD_CTRL: u32 = DATA + 0x0021_0830;

    pub(crate) const OC1083_CLEAR_CTRL: u32 = DATA + 0x0021_0840;

    pub(crate) const OC1083_SETID_CTRL: u32 = DATA + 0x0021_0850;

    pub(crate) const OC1083_START_CTRL: u32 = DATA + 0x0021_0860;

    pub(crate) const OC1083_MAGIC: u32 = 0x1083_c0de;

    pub(crate) static OC1083_PREV_LOAD: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1083_PREV_CLEAR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1083_PREV_SETID: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1083_PREV_START: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1083_fake_object_load(c: &mut Cpu) {
        if c.bus.read32(OC1083_LOAD_CTRL) != OC1083_MAGIC {
            return fakes::fall_back(c, &OC1083_PREV_LOAD, ssx3_ee::s13::f_0039fe00__generated);
        }
        fakes::fake_call(c, 0x0039_fe00);
    }

    pub(crate) fn oc1083_fake_clear_lines(c: &mut Cpu) {
        if c.bus.read32(OC1083_CLEAR_CTRL) != OC1083_MAGIC {
            return fakes::fall_back(c, &OC1083_PREV_CLEAR, ssx3_ee::s13::f_003a18c0__generated);
        }
        fakes::fake_call(c, 0x003a_18c0);
    }

    pub(crate) fn oc1083_fake_set_text_id(c: &mut Cpu) {
        if c.bus.read32(OC1083_SETID_CTRL) != OC1083_MAGIC {
            return fakes::fall_back(c, &OC1083_PREV_SETID, ssx3_ee::s13::f_003a1f18__generated);
        }
        fakes::fake_call(c, 0x003a_1f18);
    }

    pub(crate) fn oc1083_fake_start_thread(c: &mut Cpu) {
        if c.bus.read32(OC1083_START_CTRL) != OC1083_MAGIC {
            return fakes::fall_back(c, &OC1083_PREV_START, ssx3_ee::s13::f_0039c870__generated);
        }
        fakes::fake_call(c, 0x0039_c870);
    }

    pub(crate) fn oc1083_install_fakes() {
        fakes::hook(
            addr_of_mut!(ssx3_ee::s13::OVERRIDE_0039FE00),
            oc1083_fake_object_load,
            &OC1083_PREV_LOAD,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s13::OVERRIDE_003A18C0),
            oc1083_fake_clear_lines,
            &OC1083_PREV_CLEAR,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s13::OVERRIDE_003A1F18),
            oc1083_fake_set_text_id,
            &OC1083_PREV_SETID,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s13::OVERRIDE_0039C870),
            oc1083_fake_start_thread,
            &OC1083_PREV_START,
        );
    }

    #[test]
    fn ui_text_get_cached_size_matches() {
        check(
            ssx3_ee::s13::f_003a10f0__generated,
            ui_text_get_cached_size,
            SEEDS_5,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3a10_f000, 2);
                let this = DATA + 0x1000;
                let vt = DATA + 0x3000;
                c.bus
                    .write32(this + 0xb4, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(this + 0x14, r.next_u32());
                c.bus.write32(this + 8, vt);
                c.bus.write32(this + 0xa8, r.next_u32());
                let d = oc1083_slot_delta(&mut r);
                oc1083_put_slot(c, vt, 0x80, d, fakes::FAKE);
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn c_ui_text_scroll_load_matches() {
        oc1083_install_fakes();
        check(
            ssx3_ee::s13::f_003a11b8__generated,
            c_ui_text_scroll_load,
            SEEDS_5,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3a11_b800, 4);
                oc1083_install_fakes();
                c.bus.write32(OC1083_LOAD_CTRL, OC1083_MAGIC);
                c.bus.write32(OC1083_CLEAR_CTRL, OC1083_MAGIC);
                c.bus.write32(OC1083_SETID_CTRL, OC1083_MAGIC);
                let this = DATA + 0x1000;
                let data = DATA + 0x2000;
                let vt = DATA + 0x3000;
                c.bus.write32(this + 8, vt);
                c.bus.write32(this + 0x74, r.next_u32());
                let d = oc1083_slot_delta(&mut r);
                oc1083_put_slot(c, vt, 0xb8, d, fakes::FAKE + 0x40);
                c.bus.write8(data + 0x20, r.next_u32() as u8);
                c.bus.write8(data + 0x21, r.next_u32() as u8);
                c.bus
                    .write32(data + 0x24, if seed % 2 == 0 { 0 } else { 7 });
                for off in 0x28..0x2c {
                    c.bus.write8(data + off, r.next_u32() as u8);
                }
                c.bus.write32(GAME_GP.wrapping_sub(0x24f4), 0x3b80_8081);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = data as u64;
            },
        );
    }

    /// Gate magic: a gated fake answers with the logged call only when its
    /// own gate word holds this value in the test machine's RAM.
    pub(crate) const MAGIC: u32 = 0x1084_c0de;

    /// One gate word per fake, in the test data area (above the call log).
    pub(crate) const fn gate(k: u32) -> u32 {
        DATA + 0x20_0000 + 4 * k
    }

    pub(crate) const GATE_REPLACE: u32 = gate(0);

    pub(crate) const GATE_CLEAR: u32 = gate(1);

    pub(crate) const GATE_SET_STRING: u32 = gate(2);

    pub(crate) const GATE_FLAGS_CTOR: u32 = gate(3);

    pub(crate) const GATE_ALLOC: u32 = gate(4);

    pub(crate) const GATE_FREE: u32 = gate(5);

    pub(crate) const GATE_MOVE: u32 = gate(6);

    pub(crate) fn gates_on(c: &mut Cpu) {
        for g in [
            GATE_REPLACE,
            GATE_CLEAR,
            GATE_SET_STRING,
            GATE_FLAGS_CTOR,
            GATE_ALLOC,
            GATE_FREE,
            GATE_MOVE,
        ] {
            c.bus.write32(g, MAGIC);
        }
    }

    /// A gated fake: with its gate on, it logs the call and returns (the
    /// generated side and the port see the same call); with it off, it runs
    /// what the slot held before.
    macro_rules! gated {
        ($fake:ident, $prev:ident, $gate:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32($gate) != MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }
    #[allow(unused_imports)]
    pub(crate) use gated;

    gated!(
        oc1084_fake_replace,
        OC1084_PREV_REPLACE,
        GATE_REPLACE,
        0x003a_1cf0,
        ee::s13::f_003a1cf0__generated
    );

    gated!(
        oc1084_fake_clear,
        OC1084_PREV_CLEAR,
        GATE_CLEAR,
        0x003a_18c0,
        ee::s13::f_003a18c0__generated
    );

    gated!(
        oc1084_fake_set_string,
        OC1084_PREV_SET_STRING,
        GATE_SET_STRING,
        0x003a_19f8,
        ee::s13::f_003a19f8__generated
    );

    gated!(
        oc1084_fake_flags_ctor,
        OC1084_PREV_FLAGS_CTOR,
        GATE_FLAGS_CTOR,
        0x003a_6548,
        ee::s13::f_003a6548__generated
    );

    gated!(
        oc1084_fake_alloc,
        OC1084_PREV_ALLOC,
        GATE_ALLOC,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );

    gated!(
        oc1084_fake_free,
        OC1084_PREV_FREE,
        GATE_FREE,
        0x0031_7e98,
        ee::s10::f_00317e98__generated
    );

    gated!(
        oc1084_fake_move,
        OC1084_PREV_MOVE,
        GATE_MOVE,
        0x0041_605c,
        ee::s15::f_0041605c__generated
    );

    /// Puts the gated fakes in their override slots (once per process).
    pub(crate) fn install() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A1CF0),
            oc1084_fake_replace,
            &OC1084_PREV_REPLACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A18C0),
            oc1084_fake_clear,
            &OC1084_PREV_CLEAR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A19F8),
            oc1084_fake_set_string,
            &OC1084_PREV_SET_STRING,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A6548),
            oc1084_fake_flags_ctor,
            &OC1084_PREV_FLAGS_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1084_fake_alloc,
            &OC1084_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1084_fake_free,
            &OC1084_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0041605C),
            oc1084_fake_move,
            &OC1084_PREV_MOVE,
        );
    }

    /// A vtable slot `{delta, func}` at `vt + off`.
    pub(crate) fn vt_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, func: u32) {
        c.bus.write16(vt + off, (r.next_u32() & 0xfffc) as u16);
        c.bus.write32(vt + off + 4, func);
    }

    /// The fixture of `cUITextScroll_RefreshText`: the text id, the owner
    /// chain down to the text object, and the two vtables the calls read.
    /// Every fifth seed has no text object (the early exit), every fourth no
    /// id, every third a slot that returns null (the replace is skipped).
    pub(crate) fn refresh_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x3a13_60, 3);
        gates_on(c);
        let obj = DATA;
        let font = DATA + 0x100;
        let table = DATA + 0x200;
        let owner = DATA + 0x300;
        let text_obj = DATA + 0x500;
        let vt = DATA + 0x600;
        let vt2 = DATA + 0x700;
        c.g[A0][0] = obj as u64;
        let id = if seed % 4 == 0 { 0 } else { r.next_u32() | 1 };
        c.bus.write32(obj + 0xc0, id);
        c.bus.write32(obj + 0x5c, font);
        c.bus.write32(obj + 8, vt2);
        c.bus.write32(font + 0xd0, table);
        c.bus.write32(table + 0x10, owner);
        c.bus
            .write32(owner + 0x10, if seed % 5 == 0 { 0 } else { text_obj });
        c.bus.write32(text_obj + 4, vt);
        let null_func = if seed % 3 == 0 {
            0x0014_06c0
        } else {
            fakes::FAKE
        };
        vt_slot(c, &mut r, vt, 0x20, null_func);
        vt_slot(c, &mut r, vt2, 0x28, fakes::FAKE + 0x10);
    }

    #[test]
    fn ui_text_scroll_refresh_text_matches() {
        install();
        check(
            ee::s13::f_003a1360__generated,
            ui_text_scroll_refresh_text,
            300,
            &refresh_setup,
        );
    }

    /// The fixture of `cUITextScroll_ResetString`: the vtable at `self+8`
    /// with its two slots (`+0x28` and `+0xc0`), and a string argument.
    pub(crate) fn reset_string_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x3a1c_80, 3);
        gates_on(c);
        let obj = DATA;
        let vt = DATA + 0x600;
        c.g[A0][0] = obj as u64;
        c.g[A1][0] = if seed % 3 == 0 {
            0
        } else {
            (DATA + 0x800 + (r.next_u32() & 0x3f0)) as u64
        };
        c.bus.write32(obj + 8, vt);
        vt_slot(c, &mut r, vt, 0x28, fakes::FAKE);
        vt_slot(c, &mut r, vt, 0xc0, fakes::FAKE + 0x10);
    }

    #[test]
    fn ui_text_scroll_set_ascii_string_matches() {
        install();
        check(
            ee::s13::f_003a1c80__generated,
            ui_text_scroll_set_ascii_string,
            200,
            &reset_string_setup,
        );
    }

    /// The fixture of `cUITextScroll_RemoveLine`: a line array of `count`
    /// entries (empty on every fourth seed) and an index that is negative,
    /// zero, at the count or inside the array.
    pub(crate) fn remove_line_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x3a1f_90, 4);
        gates_on(c);
        let obj = DATA;
        let lines = DATA + 0x400;
        let count = (seed % 6) as u16;
        c.g[A0][0] = obj as u64;
        c.g[A1][0] = match seed % 5 {
            0 => u64::MAX,
            1 => count as u64,
            2 => 0,
            _ => (r.next_u32() % 8) as u64,
        };
        c.bus
            .write32(obj + 0xbc, if seed % 4 == 0 { 0 } else { lines });
        c.bus.write16(obj + 0xc8, count);
        for k in 0..8 {
            c.bus.write32(lines + 4 * k, DATA + 0x800 + 0x10 * k);
        }
    }

    #[test]
    fn ui_text_scroll_remove_line_matches() {
        install();
        check(
            ee::s13::f_003a1f90__generated,
            ui_text_scroll_remove_line,
            300,
            &remove_line_setup,
        );
    }
}
