//! `src/ui/uilistbox.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::funcs::call_indirect;
use crate::call::call_ptr;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Unwound;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_a670, ui_list_box_clear, "UIListBox_Clear"),
    (0x0039_a708, uilistbox_get_selected_data, "UIListBox_GetSelectedData"),
    (0x0039_a738, uilistbox_get_selected_value, "UIListBox_GetSelectedValue"),
    (0x0039_a768, ui_list_box_find_item_by_value, "UIListBox_FindItemByValue"),
    (0x0039_a7a8, uilistbox_set_selection, "UIListBox_SetSelection"),
    (0x0039_a7f0, ui_list_box_set_item_text, "UIListBox_SetItemText"),
    (0x0039_a8d8, uilistbox_set_mode, "UIListBox_SetMode"),
    (0x0039_aac0, uimenu_load, "UIMenu_Load"),
    (0x0039_ab00, uimenu_set_nav_buttons, "UIMenu_SetNavButtons"),
    (0x0039_ac48, uimenu_find_prev_selectable, "UIMenu_FindPrevSelectable"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_LIST_ADD_CHOICE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::sound::bankmonitor::callee::SPEECH_SYSTEM_STOP_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BANK_MONITOR_TIMER_KILL;
    pub(crate) use crate::game::sound::bankmonitor::callee::SNDSTRM_SET_PREBUFFER;
    pub(crate) use crate::game::sound::bankmonitor::callee::AUDIO_RAND;
    pub(crate) use crate::game::sound::bankmonitor::callee::SONG_INSTANCE_GET_REQUEST_STATUS;
    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_MAN_CALL_TASK_FUNC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_REMOVE;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;
    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2 as MEMSET;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::TEX_MAN_FREE_TEXTURE;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::sound::bankmonitor::callee::SECTION_ALLOCATOR_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
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
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x00416810, 0x0039a828)?;
    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.g[V0][0] = 0x14;
    c.mult(0, 2, 20, 2);
    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
    c.pc = t; return Ok(());
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
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::elf_image;
    use crate::difftest::compare;
    use crate::difftest::seed_count;
    use crate::difftest::put_vec;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::put_vtable;
    pub(crate) use crate::game::wscript::wscriptman::tests::w;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_8 as WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::ai::ai::tests::strict;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::globals_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::pool_4 as pool_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_3;


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
        check_lohi(ssx3_ee::s13::f_0039a708, uilistbox_get_selected_data, 200, &listbox_selected_setup);
    }


    #[test]
    fn uilistbox_get_selected_value_matches() {
        check_lohi(ssx3_ee::s13::f_0039a738, uilistbox_get_selected_value, 200, &listbox_selected_setup);
    }


    #[test]
    fn uilistbox_set_selection_matches() {
        check(ssx3_ee::s13::f_0039a7a8, uilistbox_set_selection, 200, &|c, seed| {
            let mut r = Rng(0x39a7_a800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | (seed % 10) as u64;
            c.bus.write8(DATA + 0x318, (r.next_u32() % 10) as u8);
        });
    }


    #[test]
    fn uimenu_set_nav_buttons_matches() {
        check(ssx3_ee::s13::f_0039ab00, uimenu_set_nav_buttons, 200, &|c, seed| {
            let mut r = Rng(0x39ab_0000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | if seed % 2 == 0 { 0 } else { (seed % 7) as u64 + 1 };
            for k in 0x91..=0x94 {
                c.bus.write8(DATA + k, r.next_u32() as u8);
            }
        });
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
        check(ee::s13::f_0039aac0__generated, uimenu_load, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x39aac0, 0);
            let self_at = DATA + 0x200;
            let res = DATA + 0x400;
            c.g[A0][0] = self_at as u64;
            c.g[A1][0] = res as u64;
            c.bus.write32(res + 4, (seed % 3) as u32);
            c.bus.write32(res + 8, (seed % 5) as u32);
            c.bus.write32(self_at + 0x14, (seed % 3) as u32);
            c.bus.write32(self_at + 0x74, 0);
        });
    }


    // --------------------------------------------------------------- UI list

    #[test]
    fn uilistbox_set_mode_matches() {
        check(ee::s13::f_0039a8d8__generated, uilistbox_set_mode, 80, &|c, seed| {
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
        });
    }


    // ------------------------------------------------------------------ UI

    #[test]
    fn ui_list_box_clear_matches() {
        check(ssx3_ee::s13::f_0039a670__generated, ui_list_box_clear, 24, &|c, seed| {
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
        });
    }


    #[test]
    fn ui_list_box_set_item_text_matches() {
        diff(ee::s13::f_0039a7f0__generated, ui_list_box_set_item_text, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x39_a7f0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = sx(DATA + 0x2000);
            c.g[A3][0] = (seed % 3) as u64;
            w(c, DATA + 0x2000, r.next_u32());
            w(c, DATA + 0x2004, r.next_u32());
            w(c, DATA + 0x2008, 0);
        });
    }


    #[test]
    fn uimenu_find_prev_selectable_matches() {
        strict(ee::s13::f_0039ac48__generated, uimenu_find_prev_selectable, SEEDS, &|c, seed| {
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
        });
    }
}
