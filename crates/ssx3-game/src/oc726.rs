//! Staging module oc726 (wave wave240): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc726`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002c_0a58,
        world_painter_query_create,
        "WorldPainterQuery_Create",
    ),
    (
        0x0039_7cc0,
        ui_engine_create_screen,
        "UIEngine_CreateScreen",
    ),
    (0x0019_dc20, bolt_file_release, "BoltFile_Release"),
    (
        0x0039_a568,
        ui_list_box_add_item_str_id,
        "UIListBox_AddItemStrId",
    ),
];

/// `EA_MemAlloc(size, name)`: the named pool allocator (0x317d70).
const WPQ_ALLOC: u32 = 0x0031_7d70;
/// Return address after the allocation in `WorldPainterQuery_Create`.
const WPQ_ALLOC_RET: u32 = 0x002c_0a80;
/// Return address after `WorldPainterQuery_ctor` in `WorldPainterQuery_Create`.
const WPQ_CTOR_RET: u32 = 0x002c_0a8c;
/// `WorldPainterQuery_ctor` (0x2c0408).
const WPQ_CTOR: u32 = 0x002c_0408;

/// `WorldPainterQuery* cWorldPainterMan_getQuery(void* self, cWorldPainterManArg* arg)`
/// (0x2c0a58): allocates a 0x18-byte `WorldPainterQuery` (named by a string in
/// the image), constructs it from the word `*arg` points at, and returns it in
/// `v0`. The frame keeps `s0` (the `arg` pointer) and `ra` across the calls.
pub fn world_painter_query_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A3][0] = 0;
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        // EA_MemAlloc(0x18, name): a0 = size, a1 = name string.
        c.g[A0][0] = 0x18;
        c.g[A1][0] = sx(0x0048_3de8);
        call_ptr(c, WPQ_ALLOC, WPQ_ALLOC_RET)?;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: WPQ_CTOR,
            f: ee::s08::f_002c0408,
        }
        .call(c, WPQ_CTOR_RET)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `EA_MemAlloc` for the 0xd4-byte screen and its blocks (0x317d70).
const UI_SCREEN_ALLOC: u32 = 0x0031_7d70;
/// `tAttrScreen_tAttrScreen` (0x1dd580) on the new screen's attribute block.
const UI_TATTR_CTOR: u32 = 0x001d_d580;
/// `cList_cList` (0x3977e8), the screen's lists.
const UI_LIST_CTOR: u32 = 0x0039_77e8;
/// `UIObject_ctor` (0x39fb30) on the screen's base object.
const UI_OBJECT_CTOR: u32 = 0x0039_fb30;
/// `cUIScreen_setData` (0x39c688).
const UI_SCREEN_SET_DATA: u32 = 0x0039_c688;
/// `cUIState_addScreen` (0x39e758).
const UI_STATE_ADD_SCREEN: u32 = 0x0039_e758;
/// `cUIScreen_createAllObjects` (0x39ced0).
const UI_SCREEN_CREATE_OBJECTS: u32 = 0x0039_ced0;

/// `void cUIEngine_addScreenByHashName(void* table, cUIState* state,
/// u32 hash, cUIScreen* screen)` (0x397cc0): looks the screen's hash up in
/// the table at `table + 4` (a word count, then 8-byte `{key, offset}`
/// records, with the offset relative to the table's base). If the caller
/// gave no screen, a new 0xd4-byte `cUIScreen` is allocated and built (its
/// attribute block, two lists and its base object). The screen's data is set
/// from the record, the screen is added to the state, and its objects are
/// created. Returns the screen in `v0`.
pub fn ui_engine_create_screen(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);

        // The table pointer and the table base (A1 = T0 + offset).
        c.g[T0][0] = Addr(c.g[A0][0] as u32).off(4).lw(c);
        c.g[A0][0] = 0;
        c.g[V1][0] = Addr(c.g[T0][0] as u32).off(0xc).lw(c);
        c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));

        // Search the records for the key in A2. A0 is the record found, or 0.
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(c.g[A1][0] as u32).off(4).lw(c);
            loop {
                c.cycles += 2;
                if c.g[V0][0] == c.g[A2][0] {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[V1][0];
                    break;
                }
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0039_7d08);
                }
                c.cycles += 4;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[A3][0] < c.g[V0][0]) as u64;
                let exhausted = c.g[V0][0] == 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(8));
                if exhausted {
                    break;
                }
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            }
        }

        // The record's offset gives the key's value: S2 = base + offset.
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(4).lw(c);
        let has_screen = c.g[S1][0] != 0;
        c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if !has_screen {
            create_ui_screen(c)?;
        }

        // 0x397db4: set the data, add the screen to the state, create its objects.
        c.cycles += 16;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: UI_SCREEN_SET_DATA,
            f: ee::s13::f_0039c688,
        }
        .call(c, 0x0039_7dc0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S3][0];
        Callee {
            addr: UI_STATE_ADD_SCREEN,
            f: ee::s13::f_0039e758,
        }
        .call(c, 0x0039_7dcc)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: UI_SCREEN_CREATE_OBJECTS,
            f: ee::s13::f_0039ced0,
        }
        .call(c, 0x0039_7dd4)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The new-screen branch of `ui_engine_create_screen` (0x397d34): allocates
/// the 0xd4-byte screen, builds it, and leaves it in `s1`.
fn create_ui_screen(c: &mut Cpu) -> Flow {
    c.cycles += 32;
    c.g[A1][0] = sx(0x0049_3d80);
    c.g[A3][0] = 0;
    c.g[A2][0] = 0x100;
    c.g[A0][0] = 0xd4;
    call_ptr(c, UI_SCREEN_ALLOC, 0x0039_7d4c)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = sx(0x0049_4670);
    let s0 = c.g[S0][0] as u32;
    c.g[A0][0] = sx(s0.wrapping_add(0x14));
    let vtable = c.g[V1][0];
    Addr(s0.wrapping_add(0x8)).sw(c, vtable);
    c.g[S1][0] = c.g[S0][0];
    let this = c.g[S0][0];
    Addr(s0.wrapping_add(0x4)).sw(c, this);
    Addr(s0).sw(c, this);
    Addr(s0.wrapping_add(0xc)).sw(c, 0);
    Addr(s0.wrapping_add(0x10)).sw(c, 0);
    call_ptr(c, UI_TATTR_CTOR, 0x0039_7d78)?;
    let s0 = c.g[S0][0] as u32;
    c.g[A0][0] = sx(s0.wrapping_add(0x18));
    Callee {
        addr: UI_LIST_CTOR,
        f: ee::s13::f_003977e8,
    }
    .call(c, 0x0039_7d80)?;
    let s0 = c.g[S0][0] as u32;
    c.g[A0][0] = sx(s0.wrapping_add(0x40));
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    Addr(s0.wrapping_add(0x34)).sw(c, 0);
    Addr(s0.wrapping_add(0x38)).sw(c, 0);
    Addr(s0.wrapping_add(0x3c)).sw(c, 0);
    Callee {
        addr: UI_OBJECT_CTOR,
        f: ee::s13::f_0039fb30,
    }
    .call(c, 0x0039_7d9c)?;
    let s0 = c.g[S0][0] as u32;
    c.g[V1][0] = sx(0x0049_49e8);
    c.g[A0][0] = sx(s0.wrapping_add(0xb4));
    let item_vtable = c.g[V1][0];
    Addr(s0.wrapping_add(0x48)).sw(c, item_vtable);
    Callee {
        addr: UI_LIST_CTOR,
        f: ee::s13::f_003977e8,
    }
    .call(c, 0x0039_7db0)?;
    let s0 = c.g[S0][0] as u32;
    let s3 = c.g[S3][0];
    Addr(s0.wrapping_add(0xd0)).sw(c, s3);
    Ok(())
}

/// `BoltFile_FreeSlot` (0x19d4b8).
const BOLT_FREE_SLOT: u32 = 0x0019_d4b8;
/// `BoltFile_UnpackSlot` (0x19d000).
const BOLT_UNPACK_SLOT: u32 = 0x0019_d000;

/// `void BoltFile_Release(BoltFile* f, int a1, int a2)` (0x19dc20): drops one
/// use of a 0x120-byte slot of the bolt file table. The slot is at
/// `f + a1 * 0x12000 + a2 * 0x120`; its use count is at +0x114. When the count
/// stays positive the release ends there. Otherwise a slot whose state word
/// (+0) is 2 is unpacked (or its state is cleared when the callee says so),
/// and any other slot is freed with `BoltFile_FreeSlot`.
pub fn bolt_file_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = 0x10000;
        frame.save(c, S2, 0x30);
        c.g[A3][0] = c.g[A3][0] | 0x2000;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.mult(0, 2, 18, 7);
        c.g[S3][0] = c.g[A2][0];
        c.g[V1][0] = 0x120;
        frame.save(c, S1, 0x40);
        c.mult(0, 3, 19, 3);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x10);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[S0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));

        let plus_one = bolt_file_release_slot(c)?;
        if plus_one {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        c.cycles += 6;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// The slot part of `BoltFile_Release`, with `s0` the slot. Returns true when
/// the generated code takes its extra `+1` cycle block on the way out; false
/// after an early exit (the state already decided by a callee).
fn bolt_file_release_slot(c: &mut Cpu) -> Result<bool, Unwound> {
    let slot = c.g[S0][0] as u32;
    c.g[V0][0] = Addr(slot.wrapping_add(0x114)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    let uses = c.g[V0][0];
    Addr(slot.wrapping_add(0x114)).sw(c, uses);
    if (uses as i64) > 0 {
        return Ok(true);
    }
    c.cycles += 4;
    c.g[V1][0] = Addr(slot).lw(c);
    c.g[V0][0] = 2;
    if c.g[V1][0] != c.g[V0][0] {
        c.g[V0][0] = Addr(slot.wrapping_add(0x104)).lw(c);
    } else {
        c.cycles += 5;
        c.g[A0][0] = Addr(slot.wrapping_add(0x108)).lw(c);
        call_ptr(c, 0x003d_f980, 0x0019_dc88)?;
        c.g[V1][0] = 1;
        if c.g[V0][0] != c.g[V1][0] {
            return Ok(false);
        }
        c.cycles += 16;
        c.g[A0][0] = Addr(slot.wrapping_add(0x108)).lw(c);
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x003d_f9d8, 0x0019_dca4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S0][0];
        Callee {
            addr: BOLT_UNPACK_SLOT,
            f: ee::s03::f_0019d000,
        }
        .call(c, 0x0019_dcb4)?;
        Addr(slot.wrapping_add(0x10c)).sw(c, 0);
        c.g[V1][0] = 0x000b_0000;
        Addr(slot).sw(c, 0);
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let table = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(table.wrapping_add(0x4004)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let count = c.g[V0][0];
        Addr(table.wrapping_add(0x4004)).sw(c, count);
        c.g[V0][0] = Addr(slot.wrapping_add(0x104)).lw(c);
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        Addr(slot).sw(c, 0);
        return Ok(true);
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    Callee {
        addr: BOLT_FREE_SLOT,
        f: ee::s03::f_0019d4b8,
    }
    .call(c, 0x0019_dcec)?;
    Ok(false)
}

/// `cUIListBox::USTR_length` (0x3dbca0).
const LIST_STRING_LENGTH: u32 = 0x003d_bca0;
/// `USTR_copy` (0x3dbe20).
const LIST_STRING_COPY: u32 = 0x003d_be20;
/// `EA_MemAlloc` for the item text (0x317e30).
const LIST_TEXT_ALLOC: u32 = 0x0031_7e30;

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
fn add_item_text(c: &mut Cpu, list_box: u32) -> Flow {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, LOG_RESULTS};
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const SEEDS: u32 = 200;

    /// Gate of the callee fakes: a magic word in the machine's RAM turns a
    /// fake on (both sides of a diff get their own machine).
    const CTRL_MAGIC: u32 = 0x0726_0726;
    const CTRL: u32 = DATA + 0x20_0000;

    /// A gated fake of `$target`: logs the call and returns to `ra`, or runs
    /// the body it replaced (the previous override, else `$generated`).
    macro_rules! gated_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(CTRL) != CTRL_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    gated_fake!(
        oc726_fake_alloc,
        OC726_PREV_ALLOC,
        WPQ_ALLOC,
        ee::s10::f_00317d70
    );
    gated_fake!(
        oc726_fake_tattr,
        OC726_PREV_TATTR,
        UI_TATTR_CTOR,
        ee::s04::f_001dd580
    );
    gated_fake!(
        oc726_fake_list,
        OC726_PREV_LIST,
        UI_LIST_CTOR,
        ee::s13::f_003977e8
    );
    gated_fake!(
        oc726_fake_set_data,
        OC726_PREV_SET_DATA,
        UI_SCREEN_SET_DATA,
        ee::s13::f_0039c688
    );
    gated_fake!(
        oc726_fake_add_screen,
        OC726_PREV_ADD_SCREEN,
        UI_STATE_ADD_SCREEN,
        ee::s13::f_0039e758
    );
    gated_fake!(
        oc726_fake_create_objects,
        OC726_PREV_CREATE_OBJECTS,
        UI_SCREEN_CREATE_OBJECTS,
        ee::s13::f_0039ced0
    );
    gated_fake!(
        oc726_fake_free_slot,
        OC726_PREV_FREE_SLOT,
        BOLT_FREE_SLOT,
        ee::s03::f_0019d4b8
    );
    gated_fake!(
        oc726_fake_release_a,
        OC726_PREV_RELEASE_A,
        0x003d_f980,
        ee::s14::f_003df980
    );
    gated_fake!(
        oc726_fake_release_b,
        OC726_PREV_RELEASE_B,
        0x003d_f9d8,
        ee::s14::f_003df9d8
    );
    gated_fake!(
        oc726_fake_str_length,
        OC726_PREV_STR_LENGTH,
        LIST_STRING_LENGTH,
        ee::s14::f_003dbca0
    );
    gated_fake!(
        oc726_fake_str_copy,
        OC726_PREV_STR_COPY,
        LIST_STRING_COPY,
        ee::s14::f_003dbe20
    );
    gated_fake!(
        oc726_fake_text_alloc,
        OC726_PREV_TEXT_ALLOC,
        LIST_TEXT_ALLOC,
        ee::s10::f_00317e30
    );

    fn oc726_hook() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc726_fake_alloc,
            &OC726_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001DD580),
            oc726_fake_tattr,
            &OC726_PREV_TATTR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003977E8),
            oc726_fake_list,
            &OC726_PREV_LIST,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C688),
            oc726_fake_set_data,
            &OC726_PREV_SET_DATA,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E758),
            oc726_fake_add_screen,
            &OC726_PREV_ADD_SCREEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039CED0),
            oc726_fake_create_objects,
            &OC726_PREV_CREATE_OBJECTS,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_0019D4B8),
            oc726_fake_free_slot,
            &OC726_PREV_FREE_SLOT,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF980),
            oc726_fake_release_a,
            &OC726_PREV_RELEASE_A,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF9D8),
            oc726_fake_release_b,
            &OC726_PREV_RELEASE_B,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DBCA0),
            oc726_fake_str_length,
            &OC726_PREV_STR_LENGTH,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DBE20),
            oc726_fake_str_copy,
            &OC726_PREV_STR_COPY,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc726_fake_text_alloc,
            &OC726_PREV_TEXT_ALLOC,
        );
    }

    #[test]
    fn world_painter_query_create_matches() {
        oc726_hook();
        check(
            ee::s08::f_002c0a58__generated,
            world_painter_query_create,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 726, 1);
                let arg = fakes::ptr(&mut r);
                // The constructor skips its type switch when byte 1 of the word
                // is at least 0xe (it then makes no calls of its own).
                let word = ((0x0e + (r.next_u32() & 0xfff)) << 8) | (r.next_u32() & 0xff);
                c.bus.write32(arg, word);
                c.g[A1][0] = arg as u64;
                c.pc = 0x002c_0a58;
                c.bus.write32(CTRL, CTRL_MAGIC);
            },
        );
    }

    #[test]
    fn ui_engine_create_screen_matches() {
        oc726_hook();
        check(
            ee::s13::f_00397cc0__generated,
            ui_engine_create_screen,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 727, 1);
                // The object at DATA+0x1000 holds the table pointer at +4, and
                // the table's base offset at table+0xc.
                let object = DATA + 0x1000;
                let table = DATA + 0x2000;
                let base = table + 0x100;
                c.bus.write32(object + 4, table);
                c.bus.write32(table + 0xc, 0x100);
                let count = seed % 4;
                c.bus.write32(base, count);
                let mut keys = [0u32; 3];
                for (i, key) in keys.iter_mut().enumerate() {
                    *key = r.next_u32() | 1;
                    let rec = base + 4 + 8 * i as u32;
                    c.bus.write32(rec, *key);
                    c.bus.write32(rec + 4, 0x40 + 0x10 * i as u32);
                }
                // The key is one of the records (found) or a random one.
                let hash = if count > 0 && seed % 3 != 0 {
                    keys[(seed % count) as usize]
                } else {
                    r.next_u32()
                };
                c.g[A0][0] = object as u64;
                c.g[A1][0] = fakes::ptr(&mut r) as u64;
                c.g[A2][0] = hash as u64;
                // A non-zero screen skips the allocation branch.
                c.g[A3][0] = if seed % 2 == 0 {
                    0
                } else {
                    (DATA + 0x3000) as u64
                };
                c.pc = 0x0039_7cc0;
                c.bus.write32(CTRL, CTRL_MAGIC);
            },
        );
    }

    #[test]
    fn bolt_file_release_matches() {
        oc726_hook();
        check(
            ee::s03::f_0019dc20__generated,
            bolt_file_release,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 728, 1);
                // The first callee of the state-2 branch returns 2 (not the
                // unpack branch's 1, which would run the real unpacker).
                c.bus.write32(LOG_RESULTS, 2);
                let slot = DATA + 0x1000;
                c.g[A0][0] = slot as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.bus.write32(slot + 0x114, seed % 4);
                c.bus
                    .write32(slot, if seed % 3 == 0 { 2 } else { r.next_u32() });
                c.bus
                    .write32(slot + 0x104, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(slot + 0x108, DATA + 0x2000);
                // The slot table's use count at the table base (slot + 0xb0000).
                c.bus.write32(slot + 0xb0000 + 0x4004, 5);
                c.pc = 0x0019_dc20;
                c.bus.write32(CTRL, CTRL_MAGIC);
            },
        );
    }

    #[test]
    fn ui_list_box_add_item_str_id_matches() {
        oc726_hook();
        check(
            ee::s13::f_0039a568__generated,
            ui_list_box_add_item_str_id,
            SEEDS,
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
}
