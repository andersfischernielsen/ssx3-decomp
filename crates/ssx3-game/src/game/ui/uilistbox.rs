//! `src/ui/uilistbox.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::ui::uiengine::LIST_STRING_COPY;
pub(crate) use crate::game::ui::uiengine::LIST_STRING_LENGTH;
pub(crate) use crate::game::ui::uiengine::LIST_TEXT_ALLOC;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_a670, ui_list_box_clear, "UIListBox_Clear"),
    (
        0x0039_a708,
        uilistbox_get_selected_data,
        "UIListBox_GetSelectedData",
    ),
    (
        0x0039_a738,
        uilistbox_get_selected_value,
        "UIListBox_GetSelectedValue",
    ),
    (
        0x0039_a768,
        ui_list_box_find_item_by_value,
        "UIListBox_FindItemByValue",
    ),
    (
        0x0039_a7a8,
        uilistbox_set_selection,
        "UIListBox_SetSelection",
    ),
    (
        0x0039_a7f0,
        ui_list_box_set_item_text,
        "UIListBox_SetItemText",
    ),
    (0x0039_a8d8, uilistbox_set_mode, "UIListBox_SetMode"),
    (0x0039_aac0, uimenu_load, "UIMenu_Load"),
    (0x0039_ab00, uimenu_set_nav_buttons, "UIMenu_SetNavButtons"),
    (
        0x0039_ac48,
        uimenu_find_prev_selectable,
        "UIMenu_FindPrevSelectable",
    ),
    (
        0x0039_a568,
        ui_list_box_add_item_str_id,
        "UIListBox_AddItemStrId",
    ),
    (0x0039_a928, ui_menu_set_focus, "UIMenu_SetFocus"),
    (
        0x0039_a3f8,
        ui_list_box_add_item_ascii,
        "UIListBox_AddItemAscii",
    ),
    (
        0x0039_a4b0,
        ui_list_box_add_item_wide,
        "UIListBox_AddItemWide",
    ),
    (
        0x0039_ab50,
        ui_menu_find_next_selectable,
        "UIMenu_FindNextSelectable",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_OBJECT_LOAD;
}

// ---------------------------------------------------------------------------
// UI list box

/// Shared body of the `UIListBox_GetSelected*` accessors: if the selected
/// index (+0x319) is below the count (+0x318) return the word at
/// `this + selected*0x14 + field`, else 0.
pub(crate) fn uilistbox_get_selected(c: &mut Cpu, field: u32) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x319).lbu(c);
    c.g[V1][0] = Addr(this).off(0x318).lbu(c);
    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
    if c.g[V0][0] != 0 {
        c.g[V1][0] = Addr(this).off(0x319).lbu(c);
        c.cycles += 5;
        c.g[V0][0] = 0x14;
        c.mult(0, 5, 3, 2);
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(this));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(field).lw(c);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void *UIListBox_GetSelectedData(UIListBox *box)` (0x39a708).
pub fn uilistbox_get_selected_data(c: &mut Cpu) {
    uilistbox_get_selected(c, 0xa0);
}

/// `int UIListBox_GetSelectedValue(UIListBox *box)` (0x39a738).
pub fn uilistbox_get_selected_value(c: &mut Cpu) {
    uilistbox_get_selected(c, 0xa4);
}

/// `void UIListBox_SetSelection(UIListBox *box, int index)` (0x39a7a8):
/// store the index if it is below the item count.
pub fn uilistbox_set_selection(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x318).lbu(c);
    c.g[A1][0] = c.g[A1][0] & 0xff;
    c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
    if c.g[V0][0] != 0 {
        Addr(this).off(0x319).sb(c, c.g[A1][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI menu

/// `void UIMenu_SetNavButtons(UIMenu *menu, int enabled)` (0x39ab00): store
/// the four navigation-button indices, choosing the enabled or disabled set.
pub fn uimenu_set_nav_buttons(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    let enabled = c.g[A1][0] != 0;
    c.g[A2][0] = c.g[A0][0];
    let menu = c.g[A2][0] as u32;
    if enabled {
        c.cycles += 9;
        c.g[V1][0] = 0x4;
        c.g[A0][0] = 0x5;
        c.g[A1][0] = 0x6;
        c.g[V0][0] = 0x7;
        Addr(menu).off(0x94).sb(c, c.g[V0][0]);
        Addr(menu).off(0x91).sb(c, c.g[V1][0]);
        Addr(menu).off(0x92).sb(c, c.g[A0][0]);
        Addr(menu).off(0x93).sb(c, c.g[A1][0]);
        abi::ret(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = 0x1;
    c.g[A0][0] = 0x2;
    c.g[V1][0] = 0x3;
    Addr(menu).off(0x92).sb(c, c.g[V0][0]);
    Addr(menu).off(0x94).sb(c, c.g[V1][0]);
    Addr(menu).off(0x93).sb(c, c.g[A0][0]);
    Addr(menu).off(0x91).sb(c, 0);
    abi::ret(c);
}

/// `bool UIListBox_FindItemByValue(UIListBox* self, int key /*a1*/, u8* out
/// /*a2*/)` (0x39a768): the index of the first item (`+0xa4`, stride 0x14)
/// whose value equals `key`; stores it as a byte at `out`.
pub fn ui_list_box_find_item_by_value(c: &mut Cpu) {
    c.cycles += 3;
    let self_at = abi::arg(c, 0);
    let key = abi::arg64(c, 1);
    c.g[V0][0] = Addr(self_at).off(0x318).lbu(c);
    c.g[V1][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[A3][0] = sx(p(self_at, 0xa4));
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A3)).lw(c);
        if c.g[V0][0] == key {
            c.cycles += 3;
            c.sb(abi::arg(c, 2), c.g[V1][0]);
            let ra = r32(c, RA);
            c.g[V0][0] = 1;
            c.pc = ra;
            return;
        }
        c.g[V0][0] = Addr(self_at).off(0x318).lbu(c);
        c.cycles += 4;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x14));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_a778);
        }
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.g[V0][0] = 0;
    c.pc = ra;
}

/// `void UIMenu_Load(UIMenu* self, void* resource /*a1*/)` (0x39aac0): load
/// the object, record the list count in `+0x96`/`+0x97`, and set the default
/// navigation buttons.
pub fn uimenu_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_LOAD.call(c, 0x0039_aad4)?;
        c.g[A0][0] = sx(p(r32(c, S0), 0x74));
        callee::UI_LIST_COUNT.call(c, 0x0039_aadc)?;
        c.g[A0][0] = c.g[S0][0];
        c.sb(p(r32(c, S0), 0x96), c.g[V0][0]);
        c.sb(p(r32(c, S0), 0x97), c.g[V0][0]);
        c.g[A1][0] = 0;
        callee::UI_MENU_SET_NAV_BUTTONS.call(c, 0x0039_aaf0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIListBox_SetMode(UIListBox* box, int mode)` (0x39a8d8): store the
/// column count, clear the "dirty layout" byte, flag the object dirty and
/// tell the owner (`box->+0x5c->+0xd0`, slot 0x98) to rebuild with 9.
/// 19 cycles plus the call.
pub fn uilistbox_set_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 9;
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        Addr(p(self_, 0x318)).sb(c, c.g[A1][0]);
        let mut flags = c.lw(p(self_, 0x74));
        c.g[V0][0] = flags;
        c.g[A1][0] = c.g[A0][0];
        Addr(p(self_, 0x319)).sb(c, 0);
        flags |= 0x10;
        c.sw(p(self_, 0x74), flags);
        let owner = c.lw(p(self_, 0x5c)) as u32;
        let obj = c.lw(p(owner, 0xd0));
        c.g[A3][0] = obj;
        let vt = c.lw(p(obj as u32, 8));
        c.g[V1][0] = vt;
        c.g[V0][0] = c.lw(p(vt as u32, 0x9c));
        let m = VSlot::read(c, vt, 0x98);
        c.g[A0][0] = m.this(obj);
        call_ptr(c, m.func as u32, 0x0039_a918)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI list box.

/// `void UIListBox_Clear(UIListBox* box)` (0x39a670): empty the list. When the
/// box owns its item storage (bit 0x10 of `+0x74`) it frees the single data
/// block at `+0x9c`; otherwise it frees the data of every item (0x14 bytes
/// each, count at `+0x318`). Then zero the count and the flag at `+0x319`.
pub fn ui_list_box_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);

        c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x74));
        c.g[V0][0] &= 0x10;
        let mut done = false;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x318)).lbu(c);
        } else {
            c.cycles += 3;
            c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0x9c));
            if c.g[A0][0] == 0 {
                Addr(p(c.g[S1][0] as u32, 0x318)).sb(c, 0);
                done = true;
            } else {
                c.cycles += 4;
                call_ptr(c, 0x0031_7e98, 0x0039_a6ac)?; // EA_MemFree
                Addr(p(c.g[S1][0] as u32, 0x318)).sb(c, 0);
                done = true;
            }
        }
        if !done {
            c.cycles += 2;
            let empty = c.g[V0][0] == 0;
            c.g[S2][0] = 0;
            if !empty {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x9c));
                loop {
                    c.cycles += 3;
                    c.g[A0][0] = lw(c, c.g[S0][0] as u32);
                    let count;
                    if c.g[A0][0] == 0 {
                        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x318)).lbu(c);
                        count = c.g[V0][0];
                    } else {
                        c.cycles += 3;
                        call_ptr(c, 0x0031_7e98, 0x0039_a6d4)?; // EA_MemFree
                        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x318)).lbu(c);
                        count = c.g[V0][0];
                    }
                    c.cycles += 4;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (count as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0039_a6c0);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 1;
            Addr(p(c.g[S1][0] as u32, 0x318)).sb(c, 0);
        }
        c.cycles += 7;
        Addr(p(c.g[S1][0] as u32, 0x319)).sb(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// UIListBox_SetItemText (0x000039a7f0).

/// `UIListBox_SetItemText`: set the text of a list box item and resize the box.
pub fn ui_list_box_set_item_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_39a888: {
            'b_39a86c: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S1][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S2][0] = c.g[A2][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S4][0] = c.g[A1][0] & 0xff;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S5][0] = c.g[A3][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00416810, 0x0039a828)?;
                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = 0x14;
                c.mult(0, 2, 20, 2);
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                let t = c.g[S0][0] == 0u64;
                c.g[A1][0] = 0x490000;
                if t {
                    break 'b_39a86c;
                } else {
                    'b_39a860: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0xa8));
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] == 0u64;

                        if t {
                            break 'b_39a860;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x00317e98, 0x0039a85c)?;
                            c.g[S0][0] = 0u64;
                            break 'b_39a860;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_39a888;
                    } else {
                        c.cycles += 1;
                        c.g[A1][0] = 0x490000;
                        break 'b_39a86c;
                    }
                }
            }
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[S3][0] as u32) << 1);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3df0));
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0039a880)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            break 'b_39a888;
        }
        c.cycles += 19;
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x00397ad8, 0x0039a890)?;
        c.g[V0][0] = 0x14;
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.mult(0, 2, 20, 2);
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x9c), c.g[S0][0]);
        c.g[A0][0] = c.g[V1][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0xa0), c.g[S2][0]);
        c.g[V0][0] = c.g[V1][0];
        c.sw((c.g[V0][0] as u32).wrapping_add(0xa4), c.g[S5][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}

/// `UIMenu_FindPrevSelectable(self)` (0x39ac48): walk the menu's item list
/// backwards from the current-ish index and return the previous enabled item
/// index (0 when none is found). The generated body is a dispatcher: it can
/// be re-entered at the loop head after an event unwind, so the replacement
/// keeps the same block addresses as states.
pub fn uimenu_find_prev_selectable(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0039_ac48 => {
                    c.cycles += 12;
                    let frame = Frame::open(c, 0x50);
                    frame.save(c, 16, 0x40);
                    frame.save(c, 17, 0x30);
                    frame.save(c, 19, 0x10);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, 18, 0x20);
                    c.g[S3][0] = 0xff;
                    frame.save_ra(c, 0);
                    c.g[V0][0] = c.lbu(p(c.g[S1][0] as u32, 0x95));
                    c.g[S2][0] = c.lbu(p(c.g[S1][0] as u32, 0x96));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[S0][0] = c.g[V0][0] & 0xff;
                    pc = 0x0039_ac78;
                }
                0x0039_ac78 => {
                    c.cycles += 2;
                    if c.g[S0][0] == c.g[S3][0] {
                        c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x14));
                        pc = 0x0039_acec;
                    } else {
                        pc = 0x0039_ac80;
                    }
                }
                0x0039_ac80 => {
                    c.cycles += 4;
                    c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x90));
                    c.g[V0][0] &= 4;
                    let t = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                    pc = if t { 0x0039_acc4 } else { 0x0039_ac90 };
                }
                0x0039_ac90 => {
                    c.cycles += 11;
                    c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x5c));
                    c.g[A2][0] = 1;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = lw(c, p(c.g[V0][0] as u32, 0xd0));
                    c.g[V1][0] = lw(c, p(c.g[T0][0] as u32, 8));
                    c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0xa0));
                    c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0xa4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0039_acb4);
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.pc = t;
                    ssx3_ee_rt::funcs::call_indirect(c, t);
                    if c.pc != 0x0039_acb4 {
                        return Err(Unwound);
                    }
                    let zero = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                    pc = if zero { 0x0039_ad08 } else { 0x0039_acbc };
                }
                0x0039_acbc => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    pc = 0x0039_ad1c;
                }
                0x0039_acc4 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[RA][0] = sx(0x0039_acd0);
                    c.g[A1][0] = c.g[S0][0];
                    c.pc = 0x0039_7870;
                    ee::s13::UIList_GetAt(c);
                    if c.pc != 0x0039_acd0 {
                        return Err(Unwound);
                    }
                    c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x14));
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                    c.g[V1][0] &= 1;
                    let t = c.g[V1][0] == 0;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                    if t {
                        if back_edge_unwinds(c, 0x0039_acbc) {
                            return Ok(());
                        }
                        pc = 0x0039_acbc;
                    } else {
                        pc = 0x0039_ace4;
                    }
                }
                0x0039_ace4 => {
                    c.cycles += 2;
                    c.g[S0][0] = c.g[V0][0] & 0xff;
                    pc = 0x0039_ad0c;
                }
                0x0039_acec => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                    c.g[V0][0] &= 1;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lbu(p(c.g[S1][0] as u32, 0x96));
                        pc = 0x0039_ad04;
                    } else {
                        pc = 0x0039_acfc;
                    }
                }
                0x0039_acfc => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lbu(p(c.g[S1][0] as u32, 0x95));
                    pc = 0x0039_ad1c;
                }
                0x0039_ad04 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    pc = 0x0039_ad08;
                }
                0x0039_ad08 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.g[V0][0] & 0xff;
                    pc = 0x0039_ad0c;
                }
                0x0039_ad0c => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    c.g[S2][0] = c.g[V0][0] & 0xff;
                    let t = c.g[S2][0] != c.g[S3][0];
                    c.g[V0][0] = 0;
                    if t {
                        if back_edge_unwinds(c, 0x0039_ac78) {
                            return Ok(());
                        }
                        pc = 0x0039_ac78;
                    } else {
                        pc = 0x0039_ad1c;
                    }
                }
                0x0039_ad1c => {
                    c.cycles += 7;
                    let frame = Frame { size: 0x50 };
                    frame.restore(c, 16, 0x40);
                    frame.restore(c, 17, 0x30);
                    frame.restore(c, 18, 0x20);
                    frame.restore(c, 19, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039_ac48, 0x0039_ad38);
                    return Ok(());
                }
            }
        }
    })
}

/// `UIMenu::setSelectedByIndex` (0x39b960).
pub(crate) const OC711_UI_MENU_SET_SELECTION: Callee = Callee {
    addr: 0x0039_b960,
    f: ee::s13::f_0039b960,
};

/// `UIMenu::findFirstSelectable` (0x39b7b0).
pub(crate) const OC711_UI_MENU_FIND_FIRST: Callee = Callee {
    addr: 0x0039_b7b0,
    f: ee::s13::f_0039b7b0,
};

/// `UIObject::getEventThread` (0x39ff78).
pub(crate) const OC711_UI_OBJECT_EVENT_THREAD: Callee = Callee {
    addr: 0x0039_ff78,
    f: ee::s13::f_0039ff78,
};

/// `UIScreen::startThread` (0x39c870).
pub(crate) const OC711_UI_SCREEN_START_THREAD: Callee = Callee {
    addr: 0x0039_c870,
    f: ee::s13::f_0039c870,
};

/// `lw off(base)`, sign-extended.
#[inline]
pub(crate) fn oc711_lw(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lw(c)
}

/// `lh off(base)`, sign-extended.
#[inline]
pub(crate) fn oc711_lh(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lh(c)
}

/// `lbu off(base)`.
#[inline]
pub(crate) fn oc711_lbu(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lbu(c)
}

/// `void cUIMenu::setFocus(bool on)` (0x39a928, `UIMenu_SetFocus`): stores
/// `on` as bit 4 of the flags at `+0x14`, then, when focused, re-selects the
/// stored index (or the first highlightable item) and starts the selected
/// item's screen and event thread; when unfocused it does the same for the
/// current selection's event. The blocks below follow the generated body's
/// exits: every path ends in the common `lq s0` and epilogue.
pub fn ui_menu_set_focus(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = (0u64 < c.g[A1][0]) as u64;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = 0xffff_ffff_ffff_ffef;
        let me = c.g[S0][0];
        c.g[V0][0] = oc711_lw(c, me, 0x14);
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        let on = c.g[A1][0] != 0;
        let flags = c.g[V0][0];
        Addr((me as u32).wrapping_add(0x14)).sw(c, flags);
        if on {
            ui_menu_focus_on(c)?;
        } else {
            ui_menu_focus_off(c)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The focused branch of `UIMenu_SetFocus` (generated blocks `'b_39a978`,
/// `'b_39a9ac`, `'b_39a9f0` and the tail at 0x39aa08).
pub(crate) fn ui_menu_focus_on(c: &mut Cpu) -> Flow {
    let s0 = c.g[S0][0];
    c.cycles += 4;
    c.g[V1][0] = oc711_lbu(c, s0, 0xa4);
    c.g[V0][0] = 0xff;
    let stored_none = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = s0;
    if !stored_none {
        // The stored index is valid: select it and leave.
        c.cycles += 4;
        let a0 = c.g[A0][0];
        c.g[A1][0] = oc711_lbu(c, a0, 0xa4);
        OC711_UI_MENU_SET_SELECTION.call(c, 0x0039_a970)?;
        return Ok(());
    }
    // No stored index: the current item, else the first highlightable one.
    c.cycles += 3;
    c.g[A0][0] = oc711_lw(c, s0, 0xa0);
    if c.g[A0][0] != 0 {
        let a0 = c.g[A0][0];
        c.g[V0][0] = oc711_lw(c, a0, 0x14);
    } else {
        c.cycles += 10;
        c.g[A1][0] = 0;
        c.g[A0][0] = s0;
        OC711_UI_MENU_FIND_FIRST.call(c, 0x0039_a990)?;
        c.g[V1][0] = c.g[V0][0] ^ 0xff;
        c.g[A1][0] = 0;
        c.g[A0][0] = s0;
        if c.g[V1][0] != 0 {
            c.g[A1][0] = c.g[V0][0];
        }
        OC711_UI_MENU_SET_SELECTION.call(c, 0x0039_a9a4)?;
        return Ok(());
    }
    c.cycles += 4;
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
    c.g[V0][0] = c.g[V0][0] & 0x1;
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    // The selected item has no keyframer event yet: start its screen.
    c.cycles += 6;
    c.g[A1][0] = 0;
    OC711_UI_OBJECT_EVENT_THREAD.call(c, 0x0039_a9c4)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = 0xffff;
    if c.g[A1][0] == c.g[V0][0] {
        c.g[A2][0] = oc711_lw(c, s0, 0xa0);
    } else {
        c.cycles += 4;
        c.g[V0][0] = oc711_lw(c, s0, 0xa0);
        let v0 = c.g[V0][0];
        c.g[A0][0] = oc711_lw(c, v0, 0x5c);
        if c.g[A0][0] == 0 {
            c.g[A2][0] = oc711_lw(c, s0, 0xa0);
        } else {
            c.cycles += 3;
            c.g[A2][0] = 1;
            OC711_UI_SCREEN_START_THREAD.call(c, 0x0039_a9ec)?;
            c.g[A2][0] = oc711_lw(c, s0, 0xa0);
        }
    }
    // Tail: the item's `vtable[0x3c]` method and the screen's `vtable[0x9c]`.
    c.cycles += 17;
    c.g[A1][0] = 1;
    let a2 = c.g[A2][0];
    c.g[V0][0] = oc711_lw(c, a2, 8);
    let v0 = c.g[V0][0];
    c.g[A0][0] = oc711_lh(c, v0, 0x38);
    c.g[V1][0] = oc711_lw(c, v0, 0x3c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((a2 as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0039_aa08)?;
    c.g[A0][0] = oc711_lw(c, s0, 0x5c);
    c.g[A2][0] = 1;
    c.g[A1][0] = oc711_lw(c, s0, 0xa0);
    let a0 = c.g[A0][0];
    c.g[A3][0] = oc711_lw(c, a0, 0xd0);
    let a3 = c.g[A3][0];
    c.g[V0][0] = oc711_lw(c, a3, 8);
    let v0 = c.g[V0][0];
    c.g[A0][0] = oc711_lh(c, v0, 0x98);
    c.g[V1][0] = oc711_lw(c, v0, 0x9c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((a3 as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0039_aa2c)?;
    Ok(())
}

/// The unfocused branch of `UIMenu_SetFocus` (generated block at 0x39aa48
/// onwards): when a selection exists, its event thread is started and both
/// vtable methods are called with the flag value 2.
pub(crate) fn ui_menu_focus_off(c: &mut Cpu) -> Flow {
    let s0 = c.g[S0][0];
    c.cycles += 3;
    c.g[A0][0] = oc711_lw(c, s0, 0xa0);
    if c.g[A0][0] == 0 {
        return Ok(());
    }
    c.cycles += 6;
    c.g[A1][0] = 1;
    OC711_UI_OBJECT_EVENT_THREAD.call(c, 0x0039_aa48)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = 0xffff;
    if c.g[A1][0] == c.g[V0][0] {
        c.g[A2][0] = oc711_lw(c, s0, 0xa0);
    } else {
        c.cycles += 4;
        c.g[V0][0] = oc711_lw(c, s0, 0xa0);
        let v0 = c.g[V0][0];
        c.g[A0][0] = oc711_lw(c, v0, 0x5c);
        if c.g[A0][0] == 0 {
            c.g[A2][0] = oc711_lw(c, s0, 0xa0);
        } else {
            c.cycles += 3;
            c.g[A2][0] = 1;
            OC711_UI_SCREEN_START_THREAD.call(c, 0x0039_aa70)?;
            c.g[A2][0] = oc711_lw(c, s0, 0xa0);
        }
    }
    c.cycles += 16;
    c.g[A1][0] = 0;
    let a2 = c.g[A2][0];
    c.g[V0][0] = oc711_lw(c, a2, 8);
    let v0 = c.g[V0][0];
    c.g[A0][0] = oc711_lh(c, v0, 0x38);
    c.g[V1][0] = oc711_lw(c, v0, 0x3c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((a2 as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0039_aa8c)?;
    c.g[A0][0] = oc711_lw(c, s0, 0x5c);
    c.g[A2][0] = 2;
    c.g[A1][0] = oc711_lw(c, s0, 0xa0);
    let a0 = c.g[A0][0];
    c.g[A3][0] = oc711_lw(c, a0, 0xd0);
    let a3 = c.g[A3][0];
    c.g[V0][0] = oc711_lw(c, a3, 8);
    let v0 = c.g[V0][0];
    c.g[A0][0] = oc711_lh(c, v0, 0x98);
    c.g[V1][0] = oc711_lw(c, v0, 0x9c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((a3 as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0039_aab0)?;
    Ok(())
}

/// `unsigned char cUIListBox_addEntryByStringID(void* box, int id, int idx)`
/// (0x39a568): adds an entry to the list box. The entry's string comes from
/// the object's string table (its virtual `get` at +0x24 for the id in
/// `a2`'s entry). A non-empty string is copied to a fresh buffer, and a
/// 0x14-byte item record is written at the box's current count (`box+0x318`),
/// which is then incremented (the return value is the old count, low byte).
pub fn ui_list_box_add_item_str_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0);

        let list_box = c.g[S1][0] as u32;
        c.g[V0][0] = Addr(list_box).off(0x5c).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xd0).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
        c.g[A2][0] = Addr(c.g[A0][0] as u32).off(0x10).lw(c);
        let has_entry = c.g[A2][0] != 0;
        c.g[S3][0] = c.g[A1][0];

        let mut count_up = true;
        if has_entry {
            c.cycles += 8;
            c.g[V1][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
            let get_string = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, get_string, 0x0039_a5b8)?;
            c.g[S2][0] = c.g[V0][0];
            if c.g[S2][0] == 0 {
                c.g[V0][0] = Addr(list_box.wrapping_add(0x318)).lbu(c);
                count_up = false;
            } else {
                add_item_text(c, list_box)?;
            }
        }

        if count_up {
            c.cycles += 1;
            c.g[V0][0] = Addr(list_box.wrapping_add(0x318)).lbu(c);
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x50);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        frame.restore(c, S2, 0x30);
        let count_next = c.g[V1][0];
        Addr(list_box.wrapping_add(0x318)).sb(c, count_next);
        c.g[V0][0] &= 0xff;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The text branch of `ui_list_box_add_item_str_id` (0x39a5cc..0x39a63c): with
/// the string in `s2`, copies it to a new buffer and fills the item record at
/// the box's current count (`s0` holds the buffer).
pub(crate) fn add_item_text(c: &mut Cpu, list_box: u32) -> Flow {
    c.cycles += 30;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, LIST_STRING_LENGTH, 0x0039_a5cc)?;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[A1][0] = sx(0x0049_3de0);
    c.g[A2][0] = 0x100;
    c.g[A3][0] = 0;
    c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
    call_ptr(c, LIST_TEXT_ALLOC, 0x0039_a5e8)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, LIST_STRING_COPY, 0x0039_a5f8)?;

    // Four stores into the item record at `index * 0x14`.
    c.g[V1][0] = Addr(list_box.wrapping_add(0x318)).lbu(c);
    c.g[A0][0] = 0x14;
    c.mult(0, 2, 3, 4);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(list_box));
    let slot_a = (c.g[V1][0] as u32).wrapping_add(0xa0);
    Addr(slot_a).sw(c, 0);

    c.g[V0][0] = Addr(list_box.wrapping_add(0x318)).lbu(c);
    c.mult(0, 3, 2, 4);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(list_box));
    let text_at = (c.g[V0][0] as u32).wrapping_add(0x9c);
    let text = c.g[S0][0];
    Addr(text_at).sw(c, text);

    c.g[V1][0] = Addr(list_box.wrapping_add(0x318)).lbu(c);
    c.mult(0, 2, 3, 4);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(list_box));
    let id_at = (c.g[V1][0] as u32).wrapping_add(0xa4);
    let id = c.g[S4][0];
    Addr(id_at).sw(c, id);

    c.g[V0][0] = Addr(list_box.wrapping_add(0x318)).lbu(c);
    c.mult(0, 3, 2, 4);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(list_box));
    let item_at = (c.g[V0][0] as u32).wrapping_add(0x98);
    let item = c.g[S3][0];
    Addr(item_at).sw(c, item);
    Ok(())
}

/// `int UIListBox_AddItemAscii(UIListBox* list, const char* text, int data)`
/// (0x39a3f8, frame 0x50): converts the ASCII `text` to a freshly allocated
/// wide string (`strlen`, `operator_new_named`, `UI_AsciiToWide`) and stores
/// the entry `{wide text +0x9c, text +0xa0, data +0xa4}` (stride 0x14) at the
/// list's count byte (+0x318), then bumps the count. Returns the new entry's
/// index (the old count).
pub fn ui_list_box_add_item_ascii(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S3][0] = c.g[A2][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0039a424);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00416810, 0x0039a424)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[A1][0] = 0x490000;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[RA][0] = sx(0x0039a440);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3dd0));
        call_ptr(c, 0x00317e30, 0x0039a440)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0039a450);
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x00397ad8,
            f: ee::s13::f_00397ad8,
        }
        .call(c, 0x0039a450)?;
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.g[A0][0] = 0x14;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.mult(0, 2, 3, 4);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xa0), c.g[S1][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.mult(0, 3, 2, 4);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), c.g[S2][0]);
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.mult(0, 2, 3, 4);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xa4), c.g[S3][0]);
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sb((c.g[S0][0] as u32).wrapping_add(0x318), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] & 0xff;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `UIListBox::addEntryByUnicodeString(this, str, data)` (0x39a4b0). Copies the
/// wide string `str` into a new heap buffer of `(length + 1) * 2` bytes and
/// appends a record at `this + 0x9c + 20 * count`: buffer pointer (+0), zero
/// (+4), `data` (+8). Bumps the count at `+0x318`; `v0` gets the old count
/// (the new entry's index, a byte).
pub fn ui_list_box_add_item_wide(c: &mut Cpu) {
    // no resume points: the generated body ignores `c.pc` and always starts at the
    // entry, so every pc runs the body below (the guard is not needed).
    guest(c, |c| {
        c.cycles += 46;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[A2][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0039a4dc);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x003dbca0, 0x0039a4dc)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[A1][0] = 0x490000;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[RA][0] = sx(0x0039a4f8);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3dc0));
        call_ptr(c, 0x00317e30, 0x0039a4f8)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0039a508);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003dbe20, 0x0039a508)?;
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.g[A0][0] = 0x14;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.mult(0, 2, 3, 4);
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xa0), 0u64);
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.mult(0, 3, 2, 4);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), c.g[S1][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.mult(0, 2, 3, 4);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xa4), c.g[S3][0]);
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sb((c.g[S0][0] as u32).wrapping_add(0x318), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] & 0xff;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `UIMenu::getNext` (0x39ab50): the next selectable entry after the current
/// index (`+0x95`) over `count` (`+0x96`) entries. An entry is selectable when
/// its flags (`+0x14`) have bit 5 clear; with the menu's `+0x90` bit 2 set, the
/// owner's virtual hook decides instead (nonzero accepts). Entries come from
/// the list at `+0x74`. `v0` gets the index found (0 when none qualifies).
pub fn ui_menu_find_next_selectable(c: &mut Cpu) {
    if c.pc != 0x0039ab50 {
        return ui_menu_find_next_selectable_resume(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0039ab50;
        loop {
            match pc {
                0x0039ab50 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S3][0] = 0xff;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x95));
                    c.g[S2][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[S0][0] = c.g[V0][0] & 0xff;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
                    pc = 0x0039ab88;
                    continue;
                }
                0x0039ab88 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        pc = 0x0039ac00;
                        continue;
                    }
                    pc = 0x0039ab94;
                    continue;
                }
                0x0039ab94 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = c.g[V0][0] & 0x4;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x0039abd8;
                        continue;
                    }
                    pc = 0x0039aba4;
                    continue;
                }
                0x0039aba4 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0039abc8);
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0039abc8)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0039abf8;
                        continue;
                    }
                    pc = 0x0039abd0;
                    continue;
                }
                0x0039abd0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.g[S0][0];
                    if t {
                        pc = 0x0039ac2c;
                        continue;
                    }
                    pc = 0x0039abd8;
                    continue;
                }
                0x0039abd8 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[RA][0] = sx(0x0039abe4);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00397870,
                        f: ee::s13::UIList_GetAt,
                    }
                    .call(c, 0x0039abe4)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                    c.g[V1][0] = c.g[V1][0] & 0x1;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0039abd0) {
                            return Ok(());
                        }
                        pc = 0x0039abd0;
                        continue;
                    }
                    pc = 0x0039abf8;
                    continue;
                }
                0x0039abf8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x0039ac18;
                        continue;
                    }
                    pc = 0x0039ac00;
                    continue;
                }
                0x0039ac00 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x0039ac18;
                        continue;
                    }
                    pc = 0x0039ac10;
                    continue;
                }
                0x0039ac10 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x95));
                    if t {
                        pc = 0x0039ac2c;
                        continue;
                    }
                    pc = 0x0039ac18;
                    continue;
                }
                0x0039ac18 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[S2][0] = c.g[V0][0] & 0xff;

                    if c.g[S2][0] != c.g[S3][0] {
                        c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
                        if back_edge_unwinds(c, 0x0039ab88) {
                            return Ok(());
                        }
                        pc = 0x0039ab88;
                        continue;
                    }
                    pc = 0x0039ac28;
                    continue;
                }
                0x0039ac28 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x0039ac2c;
                    continue;
                }
                0x0039ac2c => {
                    c.cycles += 7;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039ab50, 0x0039ac48);
                    return Ok(());
                }
            }
        }
    });
}

/// Native resume of `ui_menu_find_next_selectable` (converted from the `generated` body): any entry
/// other than 0x0039ab50, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn ui_menu_find_next_selectable_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0039ab50 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S3][0] = 0xff;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x95));
                    c.g[S2][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[S0][0] = c.g[V0][0] & 0xff;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
                    pc = 0x0039ab88;
                    continue;
                }
                0x0039ab88 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        pc = 0x0039ac00;
                        continue;
                    }
                    pc = 0x0039ab94;
                    continue;
                }
                0x0039ab94 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = c.g[V0][0] & 0x4;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x0039abd8;
                        continue;
                    }
                    pc = 0x0039aba4;
                    continue;
                }
                0x0039aba4 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0039abc8);
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0039abc8)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0039abf8;
                        continue;
                    }
                    pc = 0x0039abd0;
                    continue;
                }
                0x0039abd0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.g[S0][0];
                    if t {
                        pc = 0x0039ac2c;
                        continue;
                    }
                    pc = 0x0039abd8;
                    continue;
                }
                0x0039abd8 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[RA][0] = sx(0x0039abe4);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00397870,
                        f: ee::s13::UIList_GetAt,
                    }
                    .call(c, 0x0039abe4)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                    c.g[V1][0] = c.g[V1][0] & 0x1;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0039abd0) {
                            return Ok(());
                        }
                        pc = 0x0039abd0;
                        continue;
                    }
                    pc = 0x0039abf8;
                    continue;
                }
                0x0039abf8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x0039ac18;
                        continue;
                    }
                    pc = 0x0039ac00;
                    continue;
                }
                0x0039ac00 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x0039ac18;
                        continue;
                    }
                    pc = 0x0039ac10;
                    continue;
                }
                0x0039ac10 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x95));
                    if t {
                        pc = 0x0039ac2c;
                        continue;
                    }
                    pc = 0x0039ac18;
                    continue;
                }
                0x0039ac18 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[S2][0] = c.g[V0][0] & 0xff;

                    if c.g[S2][0] != c.g[S3][0] {
                        c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
                        if back_edge_unwinds(c, 0x0039ab88) {
                            return Ok(());
                        }
                        pc = 0x0039ab88;
                        continue;
                    }
                    pc = 0x0039ac28;
                    continue;
                }
                0x0039ac28 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x0039ac2c;
                    continue;
                }
                0x0039ac2c => {
                    c.cycles += 7;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039ab50, 0x0039ac48);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;

    use crate::difftest::fill;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::ai::ai::tests::strict;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_3;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::put_vtable;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;

    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;

    pub(crate) use crate::game::wscript::wscriptman::tests::w;

    /// Shared setup for the two `UIListBox_GetSelected*` accessors.
    pub(crate) fn listbox_selected_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x39a7_0000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        let (count, sel) = match seed % 4 {
            0 => (0u8, 5u8),
            1 => (8, 0),
            2 => (8, 8),
            _ => (8, 3),
        };
        c.bus.write8(DATA + 0x318, count);
        c.bus.write8(DATA + 0x319, sel);
        for k in 0..10 {
            c.bus.write32(DATA + k * 0x14 + 0xa0, r.next_u32());
        }
    }

    #[test]
    fn uilistbox_get_selected_data_matches() {
        check_lohi(
            ssx3_ee::s13::f_0039a708,
            uilistbox_get_selected_data,
            200,
            &listbox_selected_setup,
        );
    }

    #[test]
    fn uilistbox_get_selected_value_matches() {
        check_lohi(
            ssx3_ee::s13::f_0039a738,
            uilistbox_get_selected_value,
            200,
            &listbox_selected_setup,
        );
    }

    #[test]
    fn uilistbox_set_selection_matches() {
        check(
            ssx3_ee::s13::f_0039a7a8,
            uilistbox_set_selection,
            200,
            &|c, seed| {
                let mut r = Rng(0x39a7_a800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | (seed % 10) as u64;
                c.bus.write8(DATA + 0x318, (r.next_u32() % 10) as u8);
            },
        );
    }

    #[test]
    fn uimenu_set_nav_buttons_matches() {
        check(
            ssx3_ee::s13::f_0039ab00,
            uimenu_set_nav_buttons,
            200,
            &|c, seed| {
                let mut r = Rng(0x39ab_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32)
                    | if seed % 2 == 0 {
                        0
                    } else {
                        (seed % 7) as u64 + 1
                    };
                for k in 0x91..=0x94 {
                    c.bus.write8(DATA + k, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn ui_list_box_find_item_by_value_matches() {
        check(
            ee::s13::f_0039a768__generated,
            ui_list_box_find_item_by_value,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x39a768, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (seed % 7) as u64;
                c.g[A2][0] = (DATA + 0x700) as u64;
                c.bus.write8(self_at + 0x318, (seed % 4) as u8);
                for k in 0..3u32 {
                    c.bus.write32(self_at + 0xa4 + 0x14 * k, k * 3);
                }
            },
        );
    }

    #[test]
    fn uimenu_load_matches() {
        check(
            ee::s13::f_0039aac0__generated,
            uimenu_load,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x39aac0, 0);
                let self_at = DATA + 0x200;
                let res = DATA + 0x400;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = res as u64;
                c.bus.write32(res + 4, (seed % 3) as u32);
                c.bus.write32(res + 8, (seed % 5) as u32);
                c.bus.write32(self_at + 0x14, (seed % 3) as u32);
                c.bus.write32(self_at + 0x74, 0);
            },
        );
    }

    // --------------------------------------------------------------- UI list

    #[test]
    fn uilistbox_set_mode_matches() {
        check(
            ee::s13::f_0039a8d8__generated,
            uilistbox_set_mode,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x39a8);
                let obj = DATA;
                let owner = DATA + 0x400;
                let target = DATA + 0x600;
                let vt = put_vtable(c, &mut r, DATA + 0x800, &[0x98]);
                c.bus.write32(obj + 0x5c, owner);
                c.bus.write16(obj + 0x319, 0xffff);
                c.bus.write32(obj + 0x74, r.next_u32());
                c.bus.write32(owner + 0xd0, target);
                c.bus.write32(target + 8, vt);
                fill(c, &mut r, target, 16);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    // ------------------------------------------------------------------ UI

    #[test]
    fn ui_list_box_clear_matches() {
        check(
            ssx3_ee::s13::f_0039a670__generated,
            ui_list_box_clear,
            24,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x0039_a670);
                let list_box = DATA + 0x100;
                clear(c, list_box, 0x340);
                c.g[A0][0] = sx(list_box);
                if seed % 3 == 0 {
                    // Owned storage: bit 0x10 set, no data block to free.
                    w(c, list_box + 0x74, 0x10);
                    w(c, list_box + 0x9c, 0);
                    w(c, list_box + 0x318, (seed % 7) as u32);
                } else {
                    let count = seed % 5;
                    w(c, list_box + 0x74, r.next_u32() & !0x10);
                    w(c, list_box + 0x318, count);
                    for i in 0..count {
                        // Null item data: the clear must not call the allocator.
                        w(c, list_box + 0x9c + 0x14 * i, 0);
                    }
                }
                w(c, list_box + 0x319, r.next_u32() as u32);
            },
        );
    }

    #[test]
    fn ui_list_box_set_item_text_matches() {
        diff(
            ee::s13::f_0039a7f0__generated,
            ui_list_box_set_item_text,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x39_a7f0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = (seed % 3) as u64;
                w(c, DATA + 0x2000, r.next_u32());
                w(c, DATA + 0x2004, r.next_u32());
                w(c, DATA + 0x2008, 0);
            },
        );
    }

    #[test]
    fn uimenu_find_prev_selectable_matches() {
        strict(
            ee::s13::f_0039ac48__generated,
            uimenu_find_prev_selectable,
            SEEDS,
            &|c, seed| {
                let mut r = base_3(c, seed, 0xac48, 0);
                let menu = DATA + 0x100;
                let items = DATA + 0x2000;
                // the list object behind menu + 0x74 and its entries
                w(c, menu + 0x74, items);
                let n = 3 + seed % 5;
                for k in 0..8u32 {
                    let item = DATA + 0x3000 + 0x40 * k;
                    w(c, items + 4 + 4 * k, item);
                    w(c, item + 0x14, r.next_u32());
                    w(c, item + 0x90, r.next_u32());
                }
                w(c, items, n);
                c.bus.write8(menu + 0x95, n as u8);
                c.bus.write8(menu + 0x96, (seed % 3) as u8);
                w(c, menu + 0x14, r.next_u32());
                w(c, menu + 0x90, r.next_u32());
                w(c, menu + 0x5c, DATA + 0x2800);
                c.g[A0][0] = sx(menu);
                c.pc = 0x0039_ac48;
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::FAKE;

    pub(crate) use crate::game::ui::uiengine::tests::oc726_hook;
    pub(crate) use crate::game::ui::uiengine::tests::CTRL;
    pub(crate) use crate::game::ui::uiengine::tests::CTRL_MAGIC;
    pub(crate) use crate::game::ui::uiengine::tests::SEEDS_10 as SEEDS_2;

    /// Writes a pointer-sized word at `at`.
    pub(crate) fn put(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }

    /// The menu's focus change: the flag word, the stored index (0xff or a
    /// real one), a selection or none, the screen, and vtable methods that
    /// land on the fake callee (`FAKE`, which logs and returns).
    #[test]
    fn ui_menu_set_focus_matches() {
        check(
            ee::s13::f_0039a928__generated,
            ui_menu_set_focus,
            200,
            &|c, seed| {
                let mut r = Rng(0x0039_a928 ^ seed as u64);
                scramble(c, &mut r);
                let me = DATA + 0x100;
                let sel = DATA + 0x300;
                let screen = DATA + 0x600;
                let vt = DATA + 0x500;
                let obj = DATA + 0x700;
                let vt2 = DATA + 0xa00;
                let a3 = DATA + 0x900;
                put(c, me + 0x14, r.next_u32());
                c.bus.write8(
                    me + 0xa4,
                    if seed % 3 == 0 {
                        0xff
                    } else {
                        (r.next_u32() % 8) as u8
                    },
                );
                put(c, me + 0xa0, if seed % 4 == 0 { 0 } else { sel });
                put(c, me + 0x5c, obj);
                put(c, sel + 0x5c, if seed % 5 == 0 { 0 } else { screen });
                put(c, sel + 0x14, r.next_u32());
                put(c, sel + 8, vt);
                put(c, vt + 0x3c, FAKE);
                put(c, obj + 0xd0, a3);
                put(c, a3 + 8, vt2);
                put(c, vt2 + 0x9c, FAKE);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = if seed % 2 == 0 { 1 } else { 0 };
            },
        );
    }

    #[test]
    fn ui_list_box_add_item_str_id_matches() {
        oc726_hook();
        check(
            ee::s13::f_0039a568__generated,
            ui_list_box_add_item_str_id,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 729, 1);
                let list_box = DATA + 0x1000;
                c.bus.write32(list_box + 0x5c, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0xd0, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 0x10, DATA + 0x2200);
                let entry = if seed % 5 == 0 { 0 } else { DATA + 0x2300 };
                c.bus.write32(DATA + 0x2200 + 0x10, entry);
                c.bus.write32(DATA + 0x2300 + 4, DATA + 0x2400);
                c.bus.write16(DATA + 0x2400 + 0x20, (seed % 0x40) as u16);
                // A real leaf that returns 0 (no text), or the miss hook's
                // fake (non-zero text).
                let get_string = if seed % 2 == 0 {
                    0x0014_06c0
                } else {
                    fakes::FAKE
                };
                c.bus.write32(DATA + 0x2400 + 0x24, get_string);
                c.bus.write8(list_box + 0x318, (seed % 4) as u8);
                c.g[A0][0] = list_box as u64;
                c.g[A1][0] = fakes::ptr(&mut r) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.pc = 0x0039_a568;
                c.bus.write32(CTRL, CTRL_MAGIC);
            },
        );
    }

    pub(crate) use crate::game::sound::soundsys::tests::oc769_install_fakes;
    pub(crate) use crate::game::sound::soundsys::tests::oc769_machine;

    pub(crate) use crate::game::sound::soundsys::tests::OC769_THIS;

    /// `UIListBox_AddItemAscii` with texts of 0..5 characters (real `strlen`
    /// and `UI_AsciiToWide`, faked allocator) appended at list positions
    /// 0..5, and a fake unwinding in the allocation.
    #[test]
    fn ui_list_box_add_item_ascii_matches() {
        oc769_install_fakes();
        check_lohi(
            ee::s13::f_0039a3f8__generated,
            ui_list_box_add_item_ascii,
            80,
            &|c, seed| {
                let mut r = oc769_machine(c, seed, 1);
                c.bus.write8(OC769_THIS + 0x318, (seed % 6) as u8);
                let text = OC769_THIS + 0x2000;
                let len = seed % 6;
                for i in 0..len {
                    c.bus.write8(text + i, 0x20 + (r.next_u32() % 0x5f) as u8);
                }
                c.bus.write8(text + len, 0);
                c.g[A0][0] = sx(OC769_THIS);
                c.g[A1][0] = sx(text);
                c.g[A2][0] = r.next_u32() as u64;
                c.pc = 0x0039_a3f8;
            },
        );
    }

    use crate::difftest::fakes::machine_rt;

    /// The menu under test: list header `+0x74`, current index `+0x95`,
    /// count `+0x96`, scroll state `+0x97`/`+0x98`.
    pub(crate) const NAV_MENU: u32 = DATA + 0x400;

    pub(crate) const NAV_OWNER: u32 = DATA + 0x800;

    pub(crate) const NAV_HELPER: u32 = DATA + 0xa00;

    pub(crate) const NAV_VT: u32 = DATA + 0xc00;

    pub(crate) const NAV_NODES: u32 = DATA + 0x1000;

    /// The menu's owner chain (`+0x5c` owner, `owner+0xd0` helper,
    /// `helper+8` vtable), a virtual hook at vtable slot `+0xa0` that calls
    /// the fake at `FAKE`, and an intrusive list of `count` nodes behind the
    /// menu's list header (first node at `+0x78`, `node+4` links, `node+0x14`
    /// flags).
    pub(crate) fn nav_wire(c: &mut Cpu, r: &mut Rng, count: u32) {
        c.bus.write32(NAV_MENU + 0x5c, NAV_OWNER);
        c.bus.write32(NAV_OWNER + 0xd0, NAV_HELPER);
        c.bus.write32(NAV_HELPER + 8, NAV_VT);
        c.bus
            .write16(NAV_VT + 0xa0, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(NAV_VT + 0xa4, fakes::FAKE);
        c.bus
            .write32(NAV_MENU + 0x78, if count == 0 { 0 } else { NAV_NODES });
        for i in 0..count {
            let node = NAV_NODES + 0x100 * i;
            let next = if i + 1 < count { node + 0x100 } else { 0 };
            c.bus.write32(node + 4, next);
            c.bus.write32(node + 0x14, r.next_u32());
        }
    }

    /// A menu with `seed % 6` entries and a random current index, the
    /// virtual-hook flag `+0x90` bit 2 on every third seed pair, and random
    /// menu flags (`+0x14`, bit 7 wraps the search).
    pub(crate) fn nav_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine_rt(c, seed, salt, 0);
        let count = seed % 6;
        let cur = if count == 0 { 0 } else { r.next_u32() % count };
        c.bus.write8(NAV_MENU + 0x95, cur as u8);
        c.bus.write8(NAV_MENU + 0x96, count as u8);
        c.bus.write32(NAV_MENU + 0x14, r.next_u32());
        c.bus
            .write32(NAV_MENU + 0x90, if (seed / 3) % 2 == 0 { 4 } else { 0 });
        nav_wire(c, &mut r, count);
        c.g[A0][0] = sx(NAV_MENU);
        r
    }

    #[test]
    fn ui_list_box_add_item_wide_matches() {
        check(
            ee::s13::f_0039a4b0__generated,
            ui_list_box_add_item_wide,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x39a4_b0, 0);
                let this = DATA + 0x400;
                let text = DATA + 0x2000;
                let len = (seed / 6) % 6;
                c.bus.write8(this + 0x318, (seed % 6) as u8);
                for i in 0..len {
                    c.bus
                        .write16(text + 2 * i, (0x20 + r.next_u32() % 0x5f) as u16);
                }
                c.bus.write16(text + 2 * len, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(text);
                c.g[A2][0] = r.next_u32() as u64;
                c.pc = 0x0039_a4b0;
            },
        );
    }

    #[test]
    fn ui_menu_find_next_selectable_matches() {
        check(
            ee::s13::f_0039ab50__generated,
            ui_menu_find_next_selectable,
            300,
            &|c, seed| {
                nav_setup(c, seed, 0x39ab_50);
                c.pc = 0x0039_ab50;
            },
        );
    }
}
