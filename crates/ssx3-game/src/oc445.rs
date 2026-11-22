//! Staging module oc445 (wave wave223): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc445`.

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
///
/// `UIEngine_LoadTextures` (0x398138) is ported but has no entry: it calls
/// `FILE_loadpack`, which reaches `EAFile_CallWithPriority` and cannot be
/// bounded in a fixture (see its comment).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_b960, ui_menu_set_selection, "UIMenu_SetSelection"),
    (0x0039_ced0, ui_screen_create_objects, "UIScreen_CreateObjects"),
    (
        0x003a_0eb0,
        ui_text_set_wide_string_internal,
        "UIText_SetWideStringInternal",
    ),
    (0x003a_1000, ui_text_set_text_id, "UIText_SetTextId"),
    (0x003e_4af0, eatimer_init, "EATimer_Init"),
    (0x003e_57f8, eatimer_add_callback, "EATimer_AddCallback"),
];

// ---------------------------------------------------------------------------
// Local helpers (register-relative loads, vtable calls).

/// `lw off(reg)`.
#[inline]
fn oc445_lw(c: &mut Cpu, base: usize, off: u32) -> u64 {
    Addr((c.g[base][0] as u32).wrapping_add(off)).lw(c)
}

/// `lh off(reg)`.
#[inline]
fn oc445_lh(c: &mut Cpu, base: usize, off: u32) -> u64 {
    Addr((c.g[base][0] as u32).wrapping_add(off)).lh(c)
}

/// `lhu off(reg)`.
#[inline]
fn oc445_lhu(c: &mut Cpu, base: usize, off: u32) -> u64 {
    Addr((c.g[base][0] as u32).wrapping_add(off)).lhu(c)
}

/// `lbu off(reg)`.
#[inline]
fn oc445_lbu(c: &mut Cpu, base: usize, off: u32) -> u64 {
    Addr((c.g[base][0] as u32).wrapping_add(off)).lbu(c)
}

/// `addiu`-style 32-bit add of a constant to a register, sign-extended.
#[inline]
fn oc445_add(c: &Cpu, reg: usize, k: u32) -> u64 {
    sx((c.g[reg][0] as u32).wrapping_add(k))
}

/// `addu a, b` of two registers, sign-extended.
#[inline]
fn oc445_addu(c: &Cpu, a: usize, b: usize) -> u64 {
    sx((c.g[a][0] as u32).wrapping_add(c.g[b][0] as u32))
}

/// A gcc 2.9x virtual call: `A0 = lh off(vt)`, `func = lw off+4(vt)`,
/// `A0 = obj + A0`, `jalr func` returning to `ret`. `vt`, `obj` and `func`
/// name registers.
fn oc445_vcall(c: &mut Cpu, vt: usize, obj: usize, off: u32, func: usize, ret: u32) -> Flow {
    c.g[A0][0] = oc445_lh(c, vt, off);
    c.g[func][0] = oc445_lw(c, vt, off + 4);
    let target = c.g[func][0] as u32;
    c.g[A0][0] = oc445_addu(c, obj, A0);
    call_ptr(c, target, ret)
}

/// Direct callees.
mod oc445_callee {
    use super::*;

    /// `UIText_FreeStrings(UIText *)`.
    pub(super) const UI_TEXT_FREE_STRINGS: Callee = Callee {
        addr: 0x003a_0b38,
        f: ee::s13::f_003a0b38,
    };
    /// `UIText_CountLines(UIText *)`.
    pub(super) const UI_TEXT_COUNT_LINES: Callee = Callee {
        addr: 0x003a_0b90,
        f: ee::s13::f_003a0b90,
    };
    /// `UIScreen_CreateObject(screen, rec)`.
    pub(super) const UI_SCREEN_CREATE_OBJECT: Callee = Callee {
        addr: 0x0039_d078,
        f: ee::s13::f_0039d078,
    };
    /// `UIList_PushBack(list, node)`.
    pub(super) const UI_LIST_PUSH_BACK: Callee = Callee {
        addr: 0x0039_7910,
        f: ee::s13::f_00397910,
    };
    /// `UIList_GetAt(list, index)`.
    pub(super) const UI_LIST_GET_AT: Callee = Callee {
        addr: 0x0039_7870,
        f: ee::s13::f_00397870,
    };
    /// `UIMenu_SetCurrentItem(menu, item, flag)`.
    pub(super) const UI_MENU_SET_CURRENT_ITEM: Callee = Callee {
        addr: 0x0039_ad38,
        f: ee::s13::f_0039ad38,
    };
    /// `EATimer_SetVideoMode(mode)`.
    pub(super) const EA_TIMER_SET_VIDEO_MODE: Callee = Callee {
        addr: 0x003e_6690,
        f: ee::s14::f_003e6690,
    };
    /// `EA_AddShutdownHandler(fn)`.
    pub(super) const EA_ADD_SHUTDOWN_HANDLER: Callee = Callee {
        addr: 0x003e_4a30,
        f: ee::s14::f_003e4a30,
    };
}

// ---------------------------------------------------------------------------
// UIEngine_LoadTextures.

/// `void UIEngine_LoadTextures(UITextureBank *bank, int id, UITexData *data)`
/// (0x398138): load the screen's `<id>.ssh` pack (`FILE_loadpack`), upload
/// every shape through the UI engine (vtable slot 0x170) into a fresh id
/// array (`bank+4`, `count` at `bank+8`), then build the bank's 32-byte
/// element records (`bank+0xc`, `count` at `bank+0x10`) from `data` (a
/// count and 24-byte source records that name a texture index).
///
/// No `OVERRIDES` entry: `FILE_loadpack` reaches `EAFile_CallWithPriority`,
/// which a fixture cannot bound, so only the prologue up to the first call
/// is differentially tested. Transcribed from the generated body.
pub fn ui_engine_load_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0xe0);
        c.g[V0][0] = 0x490000;
        frame.save(c, 20, 0x90);
        frame.save(c, 17, 0xc0);
        c.g[S4][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, 16, 0xd0);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, 19, 0xa0);
        c.g[A1][0] = oc445_add(c, V0, 0x3d90);
        frame.save(c, 18, 0xb0);
        frame.save(c, 21, 0x80);
        frame.save_ra(c, 0x70);
        c.g[S0][0] = 0;
        // sprintf(name, "%s.ssh"-style format, id)
        call_ptr(c, 0x0041_7828, 0x0039_8178)?;
        c.g[A1][0] = 0x3000000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] |= 0x100;
        // FILE_loadpack(name, 0x3000100)
        call_ptr(c, 0x003e_22f0, 0x0039_8188)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A1][0] = 0x490000;
        c.g[V0][0] = oc445_lw(c, S3, 0x8);
        c.g[A2][0] = 0;
        c.g[A1][0] = oc445_add(c, A1, 0x3da0);
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
        Addr(oc445_add(c, S1, 0x8) as u32).sw(c, c.g[V0][0]);
        // operator new(count * 4, "UITextureIDs", 0, 0)
        call_ptr(c, 0x0031_7e30, 0x0039_81ac)?;
        c.g[A2][0] = oc445_lw(c, S1, 0x8);
        c.g[A0][0] = c.g[V0][0];
        Addr(oc445_add(c, S1, 0x4) as u32).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0xff;
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        // memset(ids, 0xff, count * 4)
        call_ptr(c, 0x0041_6210, 0x0039_81c4)?;
        c.g[V1][0] = oc445_lw(c, S1, 0x8);
        let no_shapes = (c.g[V1][0] as i64) <= 0;
        c.g[S2][0] = oc445_add(c, S3, 0x14);
        if !no_shapes {
            c.cycles += 2;
            c.g[S5][0] = 0x4a0000;
            loop {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                c.g[A1][0] = oc445_addu(c, S3, V0);
                let skip = c.g[A1][0] == 0;
                c.g[T2][0] = oc445_lw(c, GP, 0u32.wrapping_sub(0x854));
                if !skip {
                    c.cycles += 13;
                    c.g[A2][0] = oc445_add(c, S5, 0x4778);
                    c.g[A3][0] = 0;
                    c.g[T0][0] = 0x1;
                    c.g[V1][0] = oc445_lw(c, T2, 0x10d8);
                    c.g[T1][0] = u64::MAX;
                    // UIEngine vtable slot 0x170: load one shape into a page.
                    oc445_vcall(c, V1, T2, 0x170, V0, 0x0039_820c)?;
                    c.g[A0][0] = oc445_lw(c, S1, 0x4);
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V1][0] = oc445_addu(c, V1, A0);
                    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = oc445_lw(c, S1, 0x8);
                c.g[S0][0] = oc445_add(c, S0, 0x1);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = oc445_add(c, S2, 0x8);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_81d8);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 16;
        c.g[A0][0] = c.g[S3][0];
        c.g[S2][0] = oc445_add(c, S4, 0x4);
        // MEM_free(fsh)
        call_ptr(c, 0x0025_23a8, 0x0039_823c)?;
        c.g[S3][0] = oc445_add(c, S1, 0xc);
        Addr(c.g[S1][0] as u32).sw(c, c.g[S4][0]);
        c.g[A1][0] = 0x490000;
        c.g[A1][0] = oc445_add(c, A1, 0x3db0);
        c.g[A2][0] = 0;
        c.g[S0][0] = Addr(c.g[S4][0] as u32).lw(c);
        c.g[A3][0] = 0;
        Addr(oc445_add(c, S1, 0x10) as u32).sw(c, c.g[S0][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32) << 5);
        // operator new(count * 32, "UITextureELs", 0, 0)
        call_ptr(c, 0x0031_7e30, 0x0039_8264)?;
        c.g[A0][0] = oc445_add(c, S0, 0xffff_ffff);
        let no_elems = c.g[S0][0] == 0;
        c.g[V1][0] = c.g[V0][0];
        if !no_elems {
            c.cycles += 4;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.g[A1][0] = u64::MAX;
            // Initialise every record: id -1, 1.0 scales, zero offsets.
            loop {
                c.cycles += 10;
                Addr(c.g[V1][0] as u32).sw(c, c.g[A1][0]);
                c.g[A0][0] = oc445_add(c, A0, 0xffff_ffff);
                c.swc1(0, oc445_add(c, V1, 0x4) as u32);
                c.swc1(0, oc445_add(c, V1, 0x8) as u32);
                Addr(oc445_add(c, V1, 0xc) as u32).sw(c, 0);
                Addr(oc445_add(c, V1, 0x10) as u32).sw(c, 0);
                c.swc1(0, oc445_add(c, V1, 0x14) as u32);
                c.swc1(0, oc445_add(c, V1, 0x18) as u32);
                let more = c.g[A0][0] != c.g[A1][0];
                c.g[V1][0] = oc445_add(c, V1, 0x20);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_8280);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 9;
        Addr(c.g[S3][0] as u32).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0xff;
        c.g[A2][0] = oc445_lw(c, S1, 0x10);
        c.g[A0][0] = oc445_lw(c, S1, 0xc);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        // memset(elems, 0xff, count * 4)
        call_ptr(c, 0x0041_6210, 0x0039_82c0)?;
        c.g[V1][0] = oc445_lw(c, S1, 0x10);
        let no_copy = (c.g[V1][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !no_copy {
            c.cycles += 1;
            loop {
                c.cycles += 34;
                c.g[V1][0] = oc445_lw(c, S1, 0xc);
                c.g[A1][0] = sx((c.g[A3][0] as u32) << 5);
                c.lwc1(0, oc445_add(c, S2, 0xc) as u32);
                c.g[A3][0] = oc445_add(c, A3, 0x1);
                c.g[V1][0] = oc445_addu(c, A1, V1);
                c.swc1(0, oc445_add(c, V1, 0xc) as u32);
                c.g[V0][0] = oc445_lw(c, S1, 0xc);
                c.lwc1(0, oc445_add(c, S2, 0x8) as u32);
                c.g[V0][0] = oc445_addu(c, A1, V0);
                c.swc1(0, oc445_add(c, V0, 0x10) as u32);
                c.g[V1][0] = oc445_lw(c, S1, 0xc);
                c.lwc1(0, oc445_add(c, S2, 0x14) as u32);
                c.g[V1][0] = oc445_addu(c, A1, V1);
                c.swc1(0, oc445_add(c, V1, 0x18) as u32);
                c.g[V0][0] = oc445_lw(c, S1, 0xc);
                c.lwc1(0, oc445_add(c, S2, 0x10) as u32);
                c.g[V0][0] = oc445_addu(c, A1, V0);
                c.swc1(0, oc445_add(c, V0, 0x14) as u32);
                c.g[V1][0] = oc445_lw(c, S1, 0xc);
                c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
                c.g[V1][0] = oc445_addu(c, A1, V1);
                Addr(oc445_add(c, V1, 0x1c) as u32).sw(c, c.g[A0][0]);
                c.g[V0][0] = oc445_lbu(c, S2, 0x5);
                c.g[A2][0] = oc445_lw(c, S1, 0x4);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = oc445_lw(c, S1, 0xc);
                c.g[V0][0] = oc445_addu(c, V0, A2);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A1][0] = oc445_addu(c, A1, A0);
                Addr(c.g[A1][0] as u32).sw(c, c.g[V1][0]);
                c.g[V0][0] = oc445_lw(c, S1, 0x10);
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = oc445_add(c, S2, 0x18);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_82d0);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 9;
        frame.restore(c, 16, 0xd0);
        frame.restore(c, 17, 0xc0);
        frame.restore(c, 18, 0xb0);
        frame.restore(c, 19, 0xa0);
        frame.restore(c, 20, 0x90);
        frame.restore(c, 21, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UIMenu_SetSelection.

/// `void UIMenu_SetSelection(UIMenu *menu, uint8 index)` (0x39b960): select
/// item `index`. When the menu does not scroll (attribute bit 4 clear, or
/// the screen manager's flag word bit 5 clear) only the stored index
/// (`+0xa4`) is set. Otherwise the current index (`+0x95`) is set, the
/// scroll window (`+0x98` offset, `+0x97` visible count) is moved to keep it
/// in view (notifying the manager's vtable slot 0xa0 with events 3/4 when
/// scrolling down, 2/4 when scrolling up), and the list item at
/// `index - offset` becomes the current item.
pub fn ui_menu_set_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = oc445_lw(c, S0, 0x14);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
        c.g[V0][0] &= 1;
        let attr_clear = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0] & 0xff;
        let mut scrolls = false;
        if !attr_clear {
            c.cycles += 7;
            c.g[A3][0] = oc445_lw(c, S0, 0x5c);
            c.g[V1][0] = oc445_lw(c, A3, 0xd0);
            c.g[V0][0] = oc445_lw(c, V1, 0x1c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
            c.g[V0][0] &= 1;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = oc445_lbu(c, S0, 0x98);
                scrolls = true;
            }
        }
        if !scrolls {
            c.cycles += 2;
            Addr(oc445_add(c, S0, 0xa4) as u32).sb(c, c.g[S1][0]);
        } else {
            'tail: {
                'above: {
                    c.cycles += 5;
                    c.g[V1][0] = oc445_lbu(c, S0, 0x97);
                    c.g[V0][0] = oc445_addu(c, A0, V1);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let inside_bottom = c.g[V0][0] != 0;
                    Addr(oc445_add(c, S0, 0x95) as u32).sb(c, c.g[S1][0]);
                    if inside_bottom {
                        break 'above;
                    }
                    // Below the window: scroll down so the item is the last
                    // visible one.
                    c.cycles += 24;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[V0][0] = oc445_add(c, V0, 0x1);
                    c.g[A2][0] = 0x3;
                    Addr(oc445_add(c, S0, 0x98) as u32).sb(c, c.g[V0][0]);
                    c.g[T0][0] = oc445_lw(c, A3, 0xd0);
                    c.g[A3][0] = c.g[V0][0] & 0xff;
                    c.g[V1][0] = oc445_lw(c, T0, 0x8);
                    oc445_vcall(c, V1, T0, 0xa0, V0, 0x0039_b9f0)?;
                    c.g[A0][0] = oc445_lw(c, S0, 0x5c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A3][0] = oc445_lbu(c, S0, 0x98);
                    c.g[A2][0] = 0x4;
                    c.g[T0][0] = oc445_lw(c, A0, 0xd0);
                    c.g[V1][0] = oc445_lw(c, T0, 0x8);
                    oc445_vcall(c, V1, T0, 0xa0, V0, 0x0039_ba18)?;
                    c.g[A1][0] = oc445_lbu(c, S0, 0x98);
                    break 'tail;
                }
                // The item is not below the window: scroll up when it is
                // above the offset.
                c.cycles += 3;
                c.g[V0][0] = (c.g[S1][0] < c.g[A0][0]) as u64;
                let not_above = c.g[V0][0] == 0;
                c.g[A2][0] = 0x2;
                if !not_above {
                    c.cycles += 19;
                    Addr(oc445_add(c, S0, 0x98) as u32).sb(c, c.g[S1][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[T0][0] = oc445_lw(c, A3, 0xd0);
                    c.g[A3][0] = c.g[S1][0] & 0xff;
                    c.g[V1][0] = oc445_lw(c, T0, 0x8);
                    oc445_vcall(c, V1, T0, 0xa0, V0, 0x0039_ba50)?;
                    c.g[A0][0] = oc445_lw(c, S0, 0x5c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A3][0] = oc445_lbu(c, S0, 0x98);
                    c.g[A2][0] = 0x4;
                    c.g[T0][0] = oc445_lw(c, A0, 0xd0);
                    c.g[V1][0] = oc445_lw(c, T0, 0x8);
                    oc445_vcall(c, V1, T0, 0xa0, V0, 0x0039_ba78)?;
                }
                c.cycles += 1;
                c.g[A1][0] = oc445_lbu(c, S0, 0x98);
            }
            c.cycles += 7;
            c.g[A0][0] = oc445_add(c, S0, 0x74);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
            oc445_callee::UI_LIST_GET_AT.call(c, 0x0039_ba88)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = 0x1;
            oc445_callee::UI_MENU_SET_CURRENT_ITEM.call(c, 0x0039_ba98)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UIScreen_CreateObjects.

/// `void UIScreen_CreateObjects(UIScreen *screen)` (0x39ced0): build every
/// object of the screen's object block (`+0x38`: a count word, then
/// variable-size records, size in the halfword at +2). The pointer array
/// (`+0x3c`, `count * 4` bytes) is allocated and cleared; each record goes
/// through `UIScreen_CreateObject`, the new object gets the record's id
/// (`obj+0x38 = rec+4`) and is appended to the root list (`+0xb4`). A second
/// pass calls the screen manager's vtable slot 0x90 (`+0xd0`) for every
/// created object.
pub fn ui_screen_create_objects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = oc445_lw(c, S1, 0x38);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[S0][0] = oc445_add(c, V0, 0x4);
        if !empty {
            c.cycles += 13;
            c.g[A1][0] = 0x490000;
            c.g[A2][0] = 0x20000000;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A1][0] = oc445_add(c, A1, 0x3ef0);
            c.g[A3][0] = 0;
            // operator new(count * 4, "UIScreenObjs", 0x20000000, 0)
            call_ptr(c, 0x0031_7e30, 0x0039_cf14)?;
            c.g[V1][0] = oc445_lw(c, S1, 0x38);
            c.g[A0][0] = c.g[V0][0];
            Addr(oc445_add(c, S1, 0x3c) as u32).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0;
            c.g[A2][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
            // MEM_fill(array, 0, count * 4)
            call_ptr(c, 0x003e_6448, 0x0039_cf30)?;
        }
        c.cycles += 6;
        c.g[V1][0] = oc445_lw(c, S1, 0x38);
        c.g[S3][0] = 0;
        c.g[S2][0] = oc445_lw(c, S1, 0x3c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        let none = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[V1][0];
        if !none {
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                oc445_callee::UI_SCREEN_CREATE_OBJECT.call(c, 0x0039_cf54)?;
                c.g[V1][0] = c.g[V0][0];
                let failed = c.g[V1][0] == 0;
                c.g[A1][0] = c.g[V1][0];
                if !failed {
                    c.cycles += 6;
                    Addr(c.g[S2][0] as u32).sw(c, c.g[V1][0]);
                    c.g[S2][0] = oc445_add(c, S2, 0x4);
                    c.g[A0][0] = oc445_add(c, S1, 0xb4);
                    c.g[V0][0] = oc445_lw(c, S0, 0x4);
                    Addr(oc445_add(c, V1, 0x38) as u32).sw(c, c.g[V0][0]);
                    oc445_callee::UI_LIST_PUSH_BACK.call(c, 0x0039_cf78)?;
                }
                c.cycles += 8;
                c.g[V0][0] = oc445_lw(c, S1, 0x38);
                c.g[S3][0] = oc445_add(c, S3, 0x1);
                c.g[A0][0] = oc445_lhu(c, S0, 0x2);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A1][0] = c.g[V0][0];
                c.g[V1][0] = (c.g[S3][0] < c.g[V1][0]) as u64;
                let more = c.g[V1][0] != 0;
                c.g[S0][0] = oc445_addu(c, S0, A0);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_cf48);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let no_objects = c.g[V0][0] == 0;
        c.g[S0][0] = 0;
        if !no_objects {
            c.cycles += 1;
            c.g[V1][0] = oc445_lw(c, S1, 0x3c);
            loop {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[V0][0] = oc445_addu(c, V0, V1);
                c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                if c.g[A1][0] == 0 {
                    c.g[V1][0] = oc445_lw(c, S1, 0x38);
                } else {
                    c.cycles += 7;
                    c.g[A2][0] = oc445_lw(c, S1, 0xd0);
                    c.g[V0][0] = oc445_lw(c, A2, 0x8);
                    // Screen manager vtable slot 0x90: object created.
                    oc445_vcall(c, V0, A2, 0x90, V1, 0x0039_cfd4)?;
                    c.g[V1][0] = oc445_lw(c, S1, 0x38);
                }
                c.cycles += 5;
                c.g[S0][0] = oc445_add(c, S0, 0x1);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = oc445_lw(c, S1, 0x3c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_cfa8);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
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

// ---------------------------------------------------------------------------
// EATimer_Init / EATimer_AddCallback.

/// `0x450ddc`: the timer 1 interrupt handler id (non-zero once started).
#[cfg(test)]
pub(crate) const EA_TIMER_READY: u32 = 0x0045_0ddc;
/// `0x450dc8`: ticks per interrupt (EATimer_Init's result).
#[cfg(test)]
pub(crate) const EA_TIMER_TICKS_PER_IRQ: u32 = 0x0045_0dc8;
/// `0x450da0`: the known video mode constant (0: not measured yet).
#[cfg(test)]
pub(crate) const EA_TIMER_VIDEO_MODE: u32 = 0x0045_0da0;
/// `0x450dcc`: the interrupt count (advanced by the timer handler).
#[cfg(test)]
pub(crate) const EA_TIMER_IRQ_COUNT: u32 = 0x0045_0dcc;

/// `int EATimer_Init(int hz)` (0x3e4af0): start the EE timer 1 interrupt
/// (compare, mode, `AddIntcHandler(10, Rcnt1Handler)`, `EnableIntc(10)`) at
/// `hz` (0 means 100) and return the ticks per interrupt (`0x450dc8`). A
/// second call just returns that value. The prescaler (`mode` 0/1/2 for
/// hz >= 3150 / >= 196 / lower) divides the bus clock. Unless the video
/// mode is already known (`0x450da0`) it then measures a frame with
/// timer 0 (spinning on the interrupt count `0x450dcc`) to choose the
/// video mode via `EATimer_SetVideoMode`. Registers `EATimer_Shutdown` as
/// shutdown handler last.
pub fn eatimer_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 0x450000;
        frame.save(c, 18, 0x10);
        c.g[A3][0] = c.g[A0][0];
        c.g[V0][0] = oc445_lw(c, V1, 0xddc);
        c.g[S2][0] = c.g[V1][0];
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        let uninitialised = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        'done: {
            if !uninitialised {
                c.cycles += 3;
                c.g[V1][0] = 0x450000;
                c.g[V0][0] = oc445_lw(c, V1, 0xdc8);
                break 'done;
            }
            // hz -> compare value (+0xde0) and ticks per interrupt (+0xdc8).
            c.cycles += 5;
            c.g[V0][0] = 0x64;
            if c.g[A3][0] == 0 {
                c.g[A3][0] = c.g[V0][0];
            }
            c.g[V1][0] = ((c.g[A3][0] as i64) < 3150) as u64;
            let fast = c.g[V1][0] == 0;
            c.g[V0][0] = ((c.g[A3][0] as i64) < 196) as u64;
            if fast {
                // hz >= 3150: prescaler 0.
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[A3][0] as u32) >> 31);
                c.g[A1][0] = 0x8ca0000;
                c.g[V1][0] = oc445_addu(c, A3, V1);
                c.cycles += 14;
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                c.g[A0][0] = 0x450000;
                c.g[V1][0] = oc445_addu(c, V1, A1);
                c.g[A2][0] = 0x450000;
                c.div(0, V1, A3);
                c.g[S1][0] = c.g[A2][0];
                c.g[T0][0] = c.g[A0][0];
                c.g[A3][0] = 0;
                c.g[V1][0] = c.lo[0];
                Addr(oc445_add(c, A0, 0xde0) as u32).sh(c, c.g[V1][0]);
                c.g[V0][0] = c.g[V1][0] & 0xffff;
                c.div(0, A1, V0);
                c.g[A1][0] = c.lo[0];
                Addr(oc445_add(c, A2, 0xdc8) as u32).sw(c, c.g[A1][0]);
            } else {
                c.cycles += 2;
                let slow = c.g[V0][0] != 0;
                c.g[V1][0] = sx((c.g[A3][0] as u32) >> 31);
                if !slow {
                    // 196 <= hz < 3150: prescaler 1.
                    c.cycles += 6;
                    c.g[A0][0] = 0x8c0000;
                    c.g[V1][0] = oc445_addu(c, A3, V1);
                    c.g[A0][0] |= 0xa000;
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                    c.cycles += 7;
                    c.g[V1][0] = oc445_addu(c, V1, A0);
                    c.g[A1][0] = 0x450000;
                    c.div(0, V1, A3);
                    c.g[A2][0] = 0x450000;
                    c.g[S1][0] = c.g[A2][0];
                    c.g[T0][0] = c.g[A1][0];
                    c.g[A3][0] = 0x1;
                } else {
                    // hz < 196: prescaler 2.
                    c.cycles += 6;
                    c.g[A0][0] = 0x80000;
                    c.g[V1][0] = oc445_addu(c, A3, V1);
                    c.g[A0][0] |= 0xca00;
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                    c.cycles += 8;
                    c.g[V1][0] = oc445_addu(c, V1, A0);
                    c.g[A1][0] = 0x450000;
                    c.div(0, V1, A3);
                    c.g[A2][0] = 0x450000;
                    c.g[S1][0] = c.g[A2][0];
                    c.g[T0][0] = c.g[A1][0];
                    c.g[A3][0] = 0x2;
                }
                c.cycles += 7;
                c.g[V1][0] = c.lo[0];
                Addr(oc445_add(c, A1, 0xde0) as u32).sh(c, c.g[V1][0]);
                c.g[V0][0] = c.g[V1][0] & 0xffff;
                c.div(0, A0, V0);
                c.g[A0][0] = c.lo[0];
                Addr(oc445_add(c, A2, 0xdc8) as u32).sw(c, c.g[A0][0]);
            }
            // Program timer 1: count 0, compare, mode | 0x180.
            c.cycles += 24;
            c.g[V0][0] = 0x10000000;
            c.g[A2][0] = 0x10000000;
            c.g[V0][0] |= 0x800;
            c.g[A2][0] |= 0x820;
            Addr(c.g[V0][0] as u32).sw(c, 0);
            c.g[V1][0] = 0x10000000;
            c.g[V1][0] |= 0x810;
            c.g[A3][0] |= 0x180;
            c.g[V0][0] = oc445_lhu(c, T0, 0xde0);
            c.g[A1][0] = 0x3e0000;
            c.g[A1][0] = oc445_add(c, A1, 0x4db8);
            c.g[A0][0] = 0xa;
            Addr(c.g[A2][0] as u32).sw(c, c.g[V0][0]);
            c.g[S0][0] = 0x3e0000;
            Addr(c.g[V1][0] as u32).sw(c, c.g[A3][0]);
            c.g[A2][0] = 0;
            // AddIntcHandler(10, EATimer_Rcnt1Handler, 0)
            call_ptr(c, 0x0042_3a80, 0x003e_4c58)?;
            Addr(oc445_add(c, S2, 0xddc) as u32).sw(c, c.g[V0][0]);
            c.g[A0][0] = 0xa;
            // EnableIntc(10)
            call_ptr(c, 0x0042_48e8, 0x003e_4c64)?;
            c.g[V1][0] = 0x450000;
            c.g[V0][0] = oc445_lw(c, V1, 0xda0);
            let known = c.g[V0][0] != 0;
            c.g[V1][0] = 0x10000000;
            if !known {
                // Measure the frame length with timer 0 over one `ticks
                // per interrupt` wait on the interrupt count.
                c.cycles += 14;
                c.g[V0][0] = 0x10000000;
                c.g[A2][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] |= 0x10;
                c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V1][0] = 0x83;
                Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
                c.g[A1][0] = 0x450000;
                c.g[A0][0] = oc445_lw(c, A1, 0xdcc);
                c.g[V1][0] = oc445_lw(c, S1, 0xdc8);
                c.g[V0][0] = oc445_lw(c, A1, 0xdcc);
                c.g[A0][0] = oc445_addu(c, A0, V1);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let wait = c.g[V0][0] != 0;
                c.g[V1][0] = c.g[A1][0];
                if wait {
                    c.cycles += 1;
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = oc445_lw(c, V1, 0xdcc);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        let again = c.g[V0][0] != 0;
                        if again {
                            if event_due(c) {
                                deliver_event_here(c, 0x003e_4cb0);
                            }
                            continue;
                        }
                        break;
                    }
                }
                c.cycles += 7;
                c.g[V0][0] = 0x10000000;
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                Addr(c.g[V0][0] as u32).sw(c, c.g[A2][0]);
                Addr(c.g[V0][0] as u32).sw(c, c.g[A3][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                let nonneg = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = oc445_add(c, V1, 0u32.wrapping_sub(0x3d55));
                if !nonneg {
                    c.cycles += 3;
                    c.g[V0][0] = 0x10000;
                    c.g[V1][0] = oc445_addu(c, V1, V0);
                    c.g[V0][0] = oc445_add(c, V1, 0u32.wrapping_sub(0x3d55));
                }
                // d - 15701 < 99: mode 1; d - 15576 < 99: mode 2;
                // d - 28801 < 3199: mode 3.
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x63) as u64;
                let not1 = c.g[V0][0] == 0;
                c.g[V0][0] = oc445_add(c, V1, 0u32.wrapping_sub(0x3cd8));
                if !not1 {
                    c.cycles += 4;
                    c.g[A0][0] = 0x1;
                    oc445_callee::EA_TIMER_SET_VIDEO_MODE.call(c, 0x003e_4d08)?;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x63) as u64;
                    let not2 = c.g[V0][0] == 0;
                    c.g[V0][0] = oc445_add(c, V1, 0u32.wrapping_sub(0x7081));
                    if !not2 {
                        c.cycles += 4;
                        c.g[A0][0] = 0x2;
                        oc445_callee::EA_TIMER_SET_VIDEO_MODE.call(c, 0x003e_4d24)?;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[V0][0] < 0xc7f) as u64;
                        let not3 = c.g[V0][0] == 0;
                        if !not3 {
                            c.cycles += 2;
                            c.g[A0][0] = 0x3;
                            oc445_callee::EA_TIMER_SET_VIDEO_MODE.call(c, 0x003e_4d40)?;
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = oc445_add(c, S0, 0x4d68);
            // EA_AddShutdownHandler(EATimer_Shutdown)
            oc445_callee::EA_ADD_SHUTDOWN_HANDLER.call(c, 0x003e_4d48)?;
            c.g[V0][0] = oc445_lw(c, S1, 0xdc8);
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

/// Callback table: 16 records `{ fn, mode, due, f_c }` at `0x51ed98`.
#[cfg(test)]
pub(crate) const EA_TIMER_CALLBACKS: u32 = 0x0052_0000 - 0x1268;
/// Number of adds in progress (`+0xe00`).
#[cfg(test)]
pub(crate) const EA_TIMER_ADDING: u32 = 0x0045_0e00;
/// Current tick (`+0xdd0`).
#[cfg(test)]
pub(crate) const EA_TIMER_TICK: u32 = 0x0045_0dd0;

/// `void EATimer_AddCallback(void (*fn)(), int mode, int delay)` (0x3e57f8,
/// `SYNCTASK_add`): put `fn` into the 16-entry callback table, due `delay`
/// ticks from now. `mode` -1 is stored as 0, 0 as 1. An entry already
/// holding `fn` is replaced; otherwise the first free entry, after skipping
/// as many free entries as there are adds in progress (`+0xe00`, so a
/// nested add from a callback does not take the entry an outer add is
/// about to use). No room: nothing is added.
pub fn eatimer_add_callback(c: &mut Cpu) {
    c.cycles += 5;
    c.g[T1][0] = c.g[A0][0];
    c.g[T0][0] = c.g[A1][0];
    c.g[A0][0] = u64::MAX;
    let mode_given = c.g[T0][0] != c.g[A0][0];
    c.g[T4][0] = c.g[A2][0];
    c.cycles += 2;
    if mode_given {
        c.g[V0][0] = 0x1;
        if c.g[T0][0] == 0 {
            c.g[T0][0] = c.g[V0][0];
        }
    } else {
        c.g[T0][0] = 0;
    }
    c.cycles += 9;
    c.g[V0][0] = 0x450000;
    c.g[T3][0] = 0x520000;
    c.g[A1][0] = oc445_lw(c, V0, 0xe00);
    c.g[T5][0] = c.g[V0][0];
    c.g[A2][0] = 0;
    c.g[T2][0] = u64::MAX;
    c.g[V1][0] = oc445_add(c, A1, 0x1);
    c.g[A3][0] = oc445_add(c, T3, 0u32.wrapping_sub(0x1268));
    Addr(oc445_add(c, V0, 0xe00) as u32).sw(c, c.g[V1][0]);
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
        if c.g[V0][0] == c.g[T1][0] {
            // The entry already holds this function.
            c.g[A0][0] = c.g[A2][0];
            c.cycles += 1;
            c.g[A2][0] = oc445_add(c, A2, 0x1);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.g[A2][0] = oc445_add(c, A2, 0x1);
            } else {
                c.cycles += 2;
                if c.g[A0][0] != c.g[T2][0] {
                    c.g[A2][0] = oc445_add(c, A2, 0x1);
                } else {
                    c.cycles += 2;
                    if c.g[A1][0] == 0 {
                        c.g[A0][0] = c.g[A2][0];
                    } else {
                        c.cycles += 1;
                        c.g[A1][0] = oc445_add(c, A1, 0xffff_ffff);
                    }
                    c.cycles += 1;
                    c.g[A2][0] = oc445_add(c, A2, 0x1);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A2][0] as i64) < 16) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A3][0] = oc445_add(c, A3, 0x10);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003e_5840);
            }
            continue;
        }
        break;
    }
    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    let no_room = c.g[A0][0] == c.g[V0][0];
    c.g[V1][0] = oc445_add(c, T3, 0u32.wrapping_sub(0x1268));
    if !no_room {
        c.cycles += 12;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
        c.g[A1][0] = 0x450000;
        c.g[A2][0] = oc445_addu(c, A0, V1);
        c.g[V0][0] = oc445_lw(c, A1, 0xdd0);
        c.g[A3][0] = oc445_addu(c, V1, A0);
        Addr(c.g[A2][0] as u32).sw(c, c.g[T1][0]);
        c.g[A1][0] = c.g[A3][0];
        Addr(oc445_add(c, A3, 0x4) as u32).sw(c, c.g[T0][0]);
        c.g[V0][0] = oc445_addu(c, V0, T4);
        Addr(oc445_add(c, A1, 0x8) as u32).sw(c, c.g[V0][0]);
        c.g[V1][0] = c.g[A1][0];
        Addr(oc445_add(c, V1, 0xc) as u32).sw(c, 0);
    }
    c.cycles += 4;
    c.g[V0][0] = oc445_lw(c, T5, 0xe00);
    c.g[V0][0] = oc445_add(c, V0, 0xffff_ffff);
    let ra = c.g[RA][0] as u32;
    Addr(oc445_add(c, T5, 0xe00) as u32).sw(c, c.g[V0][0]);
    c.pc = ra;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, check_lohi, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word for this module's fakes (its own machine's RAM).
    pub(crate) const OC445_CTRL: u32 = DATA + 0xf_0000;
    pub(crate) const OC445_MAGIC: u32 = 0x0445_c0de;
    /// Count of `new` calls so far.
    pub(crate) const OC445_NEW_COUNT: u32 = OC445_CTRL + 4;
    /// Index of the next `UIScreen_CreateObject` result.
    pub(crate) const OC445_RESULT_NEXT: u32 = OC445_CTRL + 8;
    /// What `UIList_GetAt` returns.
    pub(crate) const OC445_GET_AT: u32 = OC445_CTRL + 0xc;
    /// `UIScreen_CreateObject` results (16 words).
    pub(crate) const OC445_RESULTS: u32 = OC445_CTRL + 0x10;
    /// Where the fake `operator new` puts its blocks (0x1000 apart).
    pub(crate) const OC445_NEW_AREA: u32 = DATA + 0x8_0000;

    pub(crate) const OC445_K_PLAIN: u32 = 0;
    pub(crate) const OC445_K_NEW: u32 = 1;
    pub(crate) const OC445_K_RESULT: u32 = 2;
    pub(crate) const OC445_K_GET_AT: u32 = 3;

    /// The logging fake, then a result by kind.
    pub(crate) fn oc445_fake(c: &mut Cpu, addr: u32, kind: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match kind {
            OC445_K_NEW => {
                let n = c.bus.read32(OC445_NEW_COUNT);
                c.bus.write32(OC445_NEW_COUNT, n + 1);
                c.g[V0][0] = sx(OC445_NEW_AREA + 0x1000 * (n & 7));
            }
            OC445_K_RESULT => {
                let i = c.bus.read32(OC445_RESULT_NEXT);
                c.bus.write32(OC445_RESULT_NEXT, i + 1);
                let r = c.bus.read32(OC445_RESULTS + 4 * (i & 0xf));
                c.g[V0][0] = sx(r);
            }
            OC445_K_GET_AT => {
                let r = c.bus.read32(OC445_GET_AT);
                c.g[V0][0] = sx(r);
            }
            _ => {}
        }
    }

    macro_rules! oc445_fakes {
        ($($name:ident $prev:ident $addr:literal $shard:ident $slot:ident $gen:ident $kind:expr;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC445_CTRL) != OC445_MAGIC {
                        return fakes::fall_back(c, &$prev, ee::$shard::$gen);
                    }
                    oc445_fake(c, $addr, $kind)
                }
            )*
            fn oc445_install_fakes() {
                $(
                    fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, &$prev);
                )*
            }
        };
    }

    oc445_fakes! {
        oc445_f_sprintf OC445_P_SPRINTF 0x0041_7828 s15 OVERRIDE_00417828 f_00417828__generated OC445_K_PLAIN;
        oc445_f_new OC445_P_NEW 0x0031_7e30 s10 OVERRIDE_00317E30 f_00317e30__generated OC445_K_NEW;
        oc445_f_memset OC445_P_MEMSET 0x0041_6210 s15 OVERRIDE_00416210 f_00416210__generated OC445_K_PLAIN;
        oc445_f_free OC445_P_FREE 0x0025_23a8 s06 OVERRIDE_002523A8 f_002523a8__generated OC445_K_PLAIN;
        oc445_f_fill OC445_P_FILL 0x003e_6448 s14 OVERRIDE_003E6448 f_003e6448__generated OC445_K_PLAIN;
        oc445_f_free_strings OC445_P_FREE_STRINGS 0x003a_0b38 s13 OVERRIDE_003A0B38 f_003a0b38__generated OC445_K_PLAIN;
        oc445_f_create_object OC445_P_CREATE_OBJECT 0x0039_d078 s13 OVERRIDE_0039D078 f_0039d078__generated OC445_K_RESULT;
        oc445_f_push_back OC445_P_PUSH_BACK 0x0039_7910 s13 OVERRIDE_00397910 f_00397910__generated OC445_K_PLAIN;
        oc445_f_get_at OC445_P_GET_AT 0x0039_7870 s13 OVERRIDE_00397870 f_00397870__generated OC445_K_GET_AT;
    }

    /// Open this test's fakes (both machines of a comparison get the gate).
    fn oc445_gate(c: &mut Cpu) {
        oc445_install_fakes();
        c.bus.write32(OC445_CTRL, OC445_MAGIC);
        c.bus.write32(OC445_NEW_COUNT, 0);
        c.bus.write32(OC445_RESULT_NEXT, 0);
    }

    /// A vtable with fake methods in the given slots, random deltas.
    fn oc445_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, slots: &[u32]) {
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
    }

    pub(crate) const OC445_OBJ: u32 = DATA + 0x1000;
    pub(crate) const OC445_OBJ2: u32 = DATA + 0x2000;
    pub(crate) const OC445_MGR: u32 = DATA + 0x3000;
    pub(crate) const OC445_VT: u32 = DATA + 0x4000;
    pub(crate) const OC445_ITEM: u32 = DATA + 0x5000;
    pub(crate) const OC445_VT_ITEM: u32 = DATA + 0x5800;
    pub(crate) const OC445_CUR: u32 = DATA + 0x6000;
    pub(crate) const OC445_STR: u32 = DATA + 0x7000;
    pub(crate) const OC445_NEW_OBJECTS: u32 = DATA + 0xa_0000;

    /// `UIEngine_LoadTextures` up to its first call: the frame, the saved
    /// registers and the `sprintf` arguments (the fake unwinds there).
    #[test]
    fn ui_engine_load_textures_prologue_matches() {
        check(
            ee::s13::f_00398138,
            ui_engine_load_textures,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x398138, 1);
                oc445_gate(c);
                c.bus.write32(fakes::LOG_UNWIND_AT, 0);
                c.g[A0][0] = sx(OC445_OBJ);
                c.g[A2][0] = sx(OC445_OBJ2);
                c.pc = 0x0039_8138;
            },
        );
    }

    /// `UIText_SetTextId`: stores the id, calls vtable slot 0x28.
    #[test]
    fn ui_text_set_text_id_matches() {
        fakes::covered(ee::s13::f_003a1000__generated, ui_text_set_text_id, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x3a1000, 1);
            oc445_vtable(c, &mut r, OC445_VT, &[0x28]);
            c.bus.write32(OC445_OBJ + 8, OC445_VT);
            c.g[A0][0] = sx(OC445_OBJ);
            c.pc = 0x003a_1000;
        });
    }

    /// `UIMenu_SetSelection`: non-scrolling menus (attribute or manager
    /// flag clear), selections inside the window, below it and above it.
    #[test]
    fn ui_menu_set_selection_matches() {
        fakes::covered(
            ee::s13::f_0039b960__generated,
            ui_menu_set_selection,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39b960, 8);
                oc445_gate(c);
                oc445_vtable(c, &mut r, OC445_VT, &[0x98, 0xa0]);
                oc445_vtable(c, &mut r, OC445_VT_ITEM, &[0x38]);
                c.bus.write32(OC445_OBJ2 + 0xd0, OC445_MGR);
                c.bus.write32(OC445_MGR + 8, OC445_VT);
                // Manager flag word bit 5 (the menu scrolls).
                let mgr_flag = if seed % 7 == 0 { 0 } else { 1 << 5 };
                c.bus.write32(OC445_MGR + 0x1c, mgr_flag | (r.next_u32() & !(1 << 5)));
                // Menu attribute bit 4.
                let attr = if seed % 5 == 0 { 0 } else { 1 << 4 };
                c.bus.write32(OC445_OBJ + 0x14, attr | (r.next_u32() & !(1 << 4)));
                c.bus.write32(OC445_OBJ + 0x5c, OC445_OBJ2);
                c.bus.write8(OC445_OBJ + 0x95, r.next_u32() as u8);
                c.bus.write8(OC445_OBJ + 0x97, 1 + (r.next_u32() % 5) as u8);
                c.bus.write8(OC445_OBJ + 0x98, (r.next_u32() % 6) as u8);
                // The current item (0 or a real object).
                let cur = if seed % 3 == 0 { OC445_CUR } else { 0 };
                c.bus.write32(OC445_OBJ + 0xa0, cur);
                c.bus.write32(OC445_CUR + 8, OC445_VT_ITEM);
                c.bus.write32(OC445_CUR + 0x14, r.next_u32());
                c.bus.write32(OC445_ITEM + 8, OC445_VT_ITEM);
                c.bus.write32(OC445_GET_AT, OC445_ITEM);
                c.g[A0][0] = sx(OC445_OBJ);
                let idx = r.next_u32() % 14;
                c.g[A1][0] = (r.next_u32() as u64) << 32 | (r.next_u32() as u64 & !0xff) | idx as u64;
                c.pc = 0x0039_b960;
            },
        );
    }

    /// `UIScreen_CreateObjects`: 0..4 records (some creations fail), the
    /// pointer array is built and every created object is announced.
    #[test]
    fn ui_screen_create_objects_matches() {
        fakes::covered(
            ee::s13::f_0039ced0__generated,
            ui_screen_create_objects,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39ced0, 12);
                oc445_gate(c);
                oc445_vtable(c, &mut r, OC445_VT, &[0x90]);
                c.bus.write32(OC445_MGR + 8, OC445_VT);
                c.bus.write32(OC445_OBJ + 0xd0, OC445_MGR);
                let n = seed % 5;
                let rec_size: u32 = if seed % 2 == 0 { 0x10 } else { 0x18 };
                c.bus.write32(OC445_OBJ + 0x38, OC445_OBJ2);
                c.bus.write32(OC445_OBJ + 0x3c, r.next_u32());
                c.bus.write32(OC445_OBJ2, n);
                for i in 0..n {
                    let rec = OC445_OBJ2 + 4 + rec_size * i;
                    c.bus.write16(rec + 2, rec_size as u16);
                    c.bus.write32(rec + 4, r.next_u32());
                }
                for i in 0..16u32 {
                    let obj = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        OC445_NEW_OBJECTS + 0x100 * i
                    };
                    c.bus.write32(OC445_RESULTS + 4 * i, obj);
                }
                c.g[A0][0] = sx(OC445_OBJ);
                c.pc = 0x0039_ced0;
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

    /// `EATimer_Init`: already started, every prescaler, hz 0, and the
    /// measured-video-mode path (only with hz values whose wait count is
    /// below the compare, so the interrupt-count spin loop is not entered).
    #[test]
    fn eatimer_init_matches() {
        check_lohi(ee::s14::f_003e4af0__generated, eatimer_init, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3e4af0, 0);
            // hz values; the last two give a zero compare value (the wait
            // loop is skipped for them), the others need a known video mode.
            let hzs: [i32; 14] = [
                0, 1, 50, 100, 195, 196, 197, 3000, 3149, 3150, 5000, 1_000_000, i32::MIN,
                0x7fff_ffff,
            ];
            let hz = hzs[(seed % 14) as usize];
            let zero_compare = hz == i32::MIN || hz == 0x7fff_ffff;
            c.bus.write32(EA_TIMER_READY, if seed % 5 == 0 { 0x1234 } else { 0 });
            c.bus.write32(EA_TIMER_TICKS_PER_IRQ, r.next_u32());
            c.bus.write32(
                EA_TIMER_VIDEO_MODE,
                if zero_compare && seed % 2 == 0 {
                    0
                } else {
                    0x3d09
                },
            );
            c.bus.write32(EA_TIMER_IRQ_COUNT, r.next_u32() >> 1);
            c.g[A0][0] = sx(hz as u32);
            c.pc = 0x003e_4af0;
        });
    }

    /// `EATimer_AddCallback`: matching entries, free entries behind the
    /// skip count, a full table and the -1/0 mode conversions.
    #[test]
    fn eatimer_add_callback_matches() {
        check(
            ee::s14::f_003e57f8__generated,
            eatimer_add_callback,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e57f8, 0);
                let fns: [u32; 4] = [0, 0x0012_3450, 0x0012_3460, 0x0012_3470];
                for i in 0..16u32 {
                    let f = match r.next_u32() % 4 {
                        0 | 1 => 0,
                        k => fns[k as usize],
                    };
                    c.bus.write32(EA_TIMER_CALLBACKS + 0x10 * i, f);
                    c.bus.write32(EA_TIMER_CALLBACKS + 0x10 * i + 4, r.next_u32());
                    c.bus.write32(EA_TIMER_CALLBACKS + 0x10 * i + 8, r.next_u32());
                    c.bus.write32(EA_TIMER_CALLBACKS + 0x10 * i + 0xc, r.next_u32());
                }
                if seed % 11 == 0 {
                    // A full table.
                    for i in 0..16u32 {
                        c.bus.write32(EA_TIMER_CALLBACKS + 0x10 * i, 0x0012_3480 + 4 * i);
                    }
                }
                c.bus.write32(EA_TIMER_ADDING, [0, 0, 1, 2, 5, 20][(r.next_u32() % 6) as usize]);
                c.bus.write32(EA_TIMER_TICK, r.next_u32());
                c.g[A0][0] = sx(fns[(r.next_u32() % 4) as usize]);
                c.g[A1][0] = match r.next_u32() % 4 {
                    0 => u64::MAX,
                    1 => 0,
                    2 => 1,
                    _ => sx(r.next_u32() % 7),
                };
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }
}
