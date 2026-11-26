//! Staging module oc849 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc849`.

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
        0x0019_2918,
        c_fe_state_bragging_rights_find_row_index,
        "cFEStateBraggingRights_FindRowIndex",
    ),
    (
        0x0019_2968,
        c_fe_state_bragging_rights_set_row_mask,
        "cFEStateBraggingRights_SetRowMask",
    ),
    (
        0x0019_2970,
        c_fe_state_bragging_rights_hide_unused_rows,
        "cFEStateBraggingRights_HideUnusedRows",
    ),
    (
        0x0019_3318,
        c_fe_state_bragging_rights_render,
        "cFEStateBraggingRights_Render",
    ),
    (
        0x0019_3338,
        c_fe_state_bragging_rights_update,
        "cFEStateBraggingRights_Update",
    ),
    (
        0x0019_3650,
        c_fe_state_bragging_rights_on_widget_created,
        "cFEStateBraggingRights_OnWidgetCreated",
    ),
];

/// `StrHash` (0x317670), `UIScreen_FindObject` (0x39d860) and the other callees of the
/// bragging-rights state, by address.
const OC849_STR_HASH: u32 = 0x0031_7670;
const OC849_FIND_OBJECT: u32 = 0x0039_d860;

/// `address = reg + off` (32-bit wrap), as the generated code forms it.
#[inline(always)]
fn oc849_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `int cFEStateBraggingRights_FindRowIndex(self, int value)` (0x192918): the
/// index (0..8) of the first row slot (`self + 8 + 4*i`) holding `value`, or -1.
pub fn c_fe_state_bragging_rights_find_row_index(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx(oc849_at(c, A0, 0x8));
    c.g[V1][0] = 0;
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        if c.g[V0][0] == c.g[A1][0] {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0];
            ret(c);
            return;
        }
        c.g[V1][0] = sx(oc849_at(c, V1, 0x1));
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 8i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx(oc849_at(c, A0, 0x4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0019_2920);
            }
            continue;
        }
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        ret(c);
        return;
    }
}

/// `void cFEStateBraggingRights_SetRowMask(self, int mask)` (0x192968): store the
/// enabled-row bit mask at `self + 0x2c`.
pub fn c_fe_state_bragging_rights_set_row_mask(c: &mut Cpu) {
    c.cycles += 2;
    let at = oc849_at(c, A0, 0x2c);
    c.sw(at, c.g[A1][0]);
    ret(c);
}

/// `void cFEStateBraggingRights_Render(self)` (0x193318): tail wrapper around
/// `UIState_Render`.
pub fn c_fe_state_bragging_rights_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_e6b8, 0x0019_3328)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Look up the widget named by the string at `sp + name_off` in the screen (`s1`)
/// and, if it exists, call its vtable slot 0x48/0x4c (`setVisible(0)`).
/// `ret` is the return address of the string hash call; the lookup returns to
/// `ret + 0xc` and the method to `ret + 0x2c`. `charge` is the block cost not
/// already counted by the caller.
fn oc849_hide_named_widget(
    c: &mut Cpu,
    frame: Frame,
    name_off: u32,
    ret: u32,
    charge: u64,
) -> Flow {
    c.cycles += charge;
    c.g[A0][0] = sx(frame.at(c, name_off));
    call_ptr(c, OC849_STR_HASH, ret)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, OC849_FIND_OBJECT, ret + 0xc)?;
    c.g[A2][0] = c.g[V0][0];
    let missing = c.g[A2][0] == 0;
    c.g[A1][0] = 0;
    if !missing {
        c.cycles += 5;
        c.g[V0][0] = c.lw(oc849_at(c, A2, 0x8));
        c.g[A0][0] = c.lh(oc849_at(c, V0, 0x48));
        c.g[V1][0] = c.lw(oc849_at(c, V0, 0x4c));
        let method = c.g[V1][0] as u32;
        c.g[A0][0] = sx(oc849_at(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, ret + 0x2c)?;
    }
    Ok(())
}

/// `void cFEStateBraggingRights_HideUnusedRows(self, screen)` (0x192970): for each
/// of the 8 rows whose bit in the mask (`self + 0x2c`) is clear and that has an
/// item, make the item static and hidden, then hide the screen widgets of the
/// row by name (left/right arrows, arrows 1, row label, box, w1, w2). The names
/// are copied from the string table onto the stack, with the row digit patched in.
pub fn c_fe_state_bragging_rights_hide_unused_rows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 80;
        let frame = Frame::open(c, 0xd0);
        c.g[V0][0] = 0x46_0000;
        frame.save(c, S1, 0xb0);
        c.g[V1][0] = 0x46_0000;
        frame.save(c, S2, 0xa0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x90);
        c.g[S2][0] = 0;
        frame.save(c, S0, 0xc0);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x80);
        // Copy the row-name templates onto the stack (unaligned doubleword and
        // word copies, as the compiler emitted for the struct assignments).
        let a = |c: &Cpu, off: u32| frame.at(c, off);
        c.g[T1][0] = sx(oc849_at(c, V0, 0).wrapping_sub(0x378));
        c.ldl(6, oc849_at(c, T1, 0x7));
        c.ldr(6, oc849_at(c, T1, 0));
        c.g[A3][0] = c.lb(oc849_at(c, T1, 0x8));
        c.g[T0][0] = c.lb(oc849_at(c, T1, 0x9));
        let p = a(c, 0x7);
        c.sdl(6, p);
        let p = a(c, 0);
        c.sdr(6, p);
        let p = a(c, 0x8);
        c.sb(p, c.g[A3][0]);
        let p = a(c, 0x9);
        c.sb(p, c.g[T0][0]);
        c.g[A2][0] = c.lb(oc849_at(c, T1, 0xa));
        let p = a(c, 0xa);
        c.sb(p, c.g[A2][0]);
        c.g[T1][0] = sx(oc849_at(c, V1, 0).wrapping_sub(0x368));
        c.ldl(2, oc849_at(c, T1, 0x7));
        c.ldr(2, oc849_at(c, T1, 0));
        c.lwl(7, oc849_at(c, T1, 0xb));
        c.lwr(7, oc849_at(c, T1, 0x8));
        let p = a(c, 0x17);
        c.sdl(2, p);
        let p = a(c, 0x10);
        c.sdr(2, p);
        let p = a(c, 0x1b);
        c.swl(7, p);
        let p = a(c, 0x18);
        c.swr(7, p);
        c.g[A2][0] = 0x46_0000;
        c.g[T1][0] = sx(oc849_at(c, A2, 0).wrapping_sub(0x358));
        c.ldl(3, oc849_at(c, T1, 0x7));
        c.ldr(3, oc849_at(c, T1, 0));
        c.lwl(7, oc849_at(c, T1, 0xb));
        c.lwr(7, oc849_at(c, T1, 0x8));
        let p = a(c, 0x27);
        c.sdl(3, p);
        let p = a(c, 0x20);
        c.sdr(3, p);
        let p = a(c, 0x2b);
        c.swl(7, p);
        let p = a(c, 0x28);
        c.swr(7, p);
        c.g[V0][0] = 0x46_0000;
        c.g[T1][0] = sx(oc849_at(c, V0, 0).wrapping_sub(0x348));
        c.ldl(6, oc849_at(c, T1, 0x7));
        c.ldr(6, oc849_at(c, T1, 0));
        c.lwl(7, oc849_at(c, T1, 0xb));
        c.lwr(7, oc849_at(c, T1, 0x8));
        c.g[T0][0] = c.lb(oc849_at(c, T1, 0xc));
        let p = a(c, 0x37);
        c.sdl(6, p);
        let p = a(c, 0x30);
        c.sdr(6, p);
        let p = a(c, 0x3b);
        c.swl(7, p);
        let p = a(c, 0x38);
        c.swr(7, p);
        let p = a(c, 0x3c);
        c.sb(p, c.g[T0][0]);
        c.g[V1][0] = 0x4a_0000;
        c.g[T1][0] = sx(oc849_at(c, V1, 0x1740));
        c.lwl(2, oc849_at(c, T1, 0x3));
        c.lwr(2, oc849_at(c, T1, 0));
        let p = a(c, 0x43);
        c.swl(2, p);
        let p = a(c, 0x40);
        c.swr(2, p);
        c.g[A2][0] = 0x4a_0000;
        c.g[T1][0] = sx(oc849_at(c, A2, 0x1748));
        c.lwl(3, oc849_at(c, T1, 0x3));
        c.lwr(3, oc849_at(c, T1, 0));
        let p = a(c, 0x53);
        c.swl(3, p);
        let p = a(c, 0x50);
        c.swr(3, p);
        c.g[V0][0] = 0x4a_0000;
        c.g[V1][0] = 0x4a_0000;
        c.g[T1][0] = sx(oc849_at(c, V0, 0x14f8));
        c.g[A2][0] = c.lb(oc849_at(c, T1, 0));
        c.g[A3][0] = c.lb(oc849_at(c, T1, 0x1));
        let p = a(c, 0x60);
        c.sb(p, c.g[A2][0]);
        let p = a(c, 0x61);
        c.sb(p, c.g[A3][0]);
        c.g[A3][0] = sx(oc849_at(c, V1, 0x1750));
        c.lwl(2, oc849_at(c, A3, 0x3));
        c.lwr(2, oc849_at(c, A3, 0));
        c.g[A0][0] = c.lb(oc849_at(c, A3, 0x4));
        let p = a(c, 0x73);
        c.swl(2, p);
        let p = a(c, 0x70);
        c.swr(2, p);
        let p = a(c, 0x74);
        c.sb(p, c.g[A0][0]);
        // The row loop: s2 = row, s3 = self, s1 = screen.
        c.g[V0][0] = c.lw(oc849_at(c, S3, 0x2c));
        loop {
            c.cycles += 4;
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[S2][0] as u32) & 31)) as u32);
            c.g[V0][0] &= 0x1;
            if c.g[V0][0] != 0 {
                c.g[S2][0] = sx(oc849_at(c, S2, 0x1));
            } else {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[V0][0] = sx(oc849_at(c, S3, 0x8));
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                let no_item = c.g[A2][0] == 0;
                c.g[A1][0] = 0x1;
                if !no_item {
                    c.cycles += 30;
                    // item->setStatic(1)
                    c.g[V0][0] = c.lw(oc849_at(c, A2, 0x8));
                    c.g[A0][0] = c.lh(oc849_at(c, V0, 0x40));
                    c.g[V1][0] = c.lw(oc849_at(c, V0, 0x44));
                    let method = c.g[V1][0] as u32;
                    c.g[A0][0] = sx(oc849_at(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0019_2aec)?;
                    // item->setVisible(0)
                    c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A1][0] = 0;
                    c.g[V0][0] = c.lw(oc849_at(c, A2, 0x8));
                    c.g[A0][0] = c.lh(oc849_at(c, V0, 0x48));
                    c.g[V1][0] = c.lw(oc849_at(c, V0, 0x4c));
                    let method = c.g[V1][0] as u32;
                    c.g[A0][0] = sx(oc849_at(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0019_2b08)?;
                    // Patch the row digit into the eight name strings.
                    c.g[V1][0] = sx(oc849_at(c, S2, 0x31));
                    c.g[V0][0] = sx(oc849_at(c, S2, 0x30));
                    c.g[A0][0] = c.g[SP][0];
                    for off in [0x30u32, 0, 0x10, 0x20] {
                        let p = a(c, off);
                        c.sb(p, c.g[V0][0]);
                    }
                    for off in [0x52u32, 0x60, 0x73, 0x42] {
                        let p = a(c, off);
                        c.sb(p, c.g[V1][0]);
                    }
                    // Hide the widgets by name, in the original order.
                    oc849_hide_named_widget(c, frame, 0x00, 0x0019_2b38, 0)?;
                    oc849_hide_named_widget(c, frame, 0x10, 0x0019_2b6c, 8)?;
                    oc849_hide_named_widget(c, frame, 0x20, 0x0019_2ba0, 8)?;
                    oc849_hide_named_widget(c, frame, 0x30, 0x0019_2bd4, 8)?;
                    oc849_hide_named_widget(c, frame, 0x60, 0x0019_2c08, 8)?;
                    oc849_hide_named_widget(c, frame, 0x70, 0x0019_2c3c, 8)?;
                    oc849_hide_named_widget(c, frame, 0x40, 0x0019_2c70, 8)?;
                    oc849_hide_named_widget(c, frame, 0x50, 0x0019_2ca4, 8)?;
                }
                c.cycles += 1;
                c.g[S2][0] = sx(oc849_at(c, S2, 0x1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 8i64) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lw(oc849_at(c, S3, 0x2c));
                if event_due(c) {
                    deliver_event_here(c, 0x0019_2ab0);
                }
                continue;
            }
            break;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateBraggingRights_SetRowsVisible` (0x192740).
const OC849_SET_ROWS_VISIBLE: Callee = Callee {
    addr: 0x0019_2740,
    f: ee::s02::f_00192740,
};
/// `cFEStateBraggingRights_HideUnusedRows` (0x192970), through the generated
/// function (which dispatches through the override slot once installed).
const OC849_HIDE_UNUSED_ROWS: Callee = Callee {
    addr: 0x0019_2970,
    f: ee::s02::f_00192970,
};

/// `void cFEStateBraggingRights_Update(self)` (0x193338): for each local player
/// panel (one online, two offline) read the controller id, feed it to the UI input
/// object, and run the confirm / cancel logic on the player's menu control
/// (`self + 0x6d0 + 0x34 * panel`): confirm calls the menu's vtable slot 0x78 with
/// the input; cancel redraws the row visibility, rewinds the screen frame by label
/// and, online, calls `func_00265700` on the setup manager.
pub fn c_fe_state_bragging_rights_update(c: &mut Cpu) {
    guest(c, |c| {
        let is_net = |c: &mut Cpu| c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
        c.cycles += 25;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = is_net(c);
        frame.save(c, S2, 0x70);
        c.g[V1][0] = 0x1;
        frame.save(c, S5, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S6, 0x30);
        c.g[V0][0] ^= 0x1;
        frame.save(c, S7, 0x20);
        c.g[S5][0] = 0;
        frame.save(c, S0, 0x90);
        c.g[S7][0] = 0x2;
        frame.save(c, S1, 0x80);
        if c.g[V0][0] == 0 {
            c.g[S7][0] = c.g[V1][0];
        }
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.lw(oc849_at(c, S2, 0x10));
        c.g[S3][0] = c.lw(oc849_at(c, A0, 0xc));
        call_ptr(c, 0x0014_dc80, 0x0019_3388)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014_dd58, 0x0019_3394)?;
        c.g[S6][0] = c.g[V0][0];
        if c.g[S7][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = is_net(c);
            loop {
                // s5 = loop counter, s4 = panel index (0 online).
                c.cycles += 3;
                c.g[V0][0] ^= 0x1;
                let offline = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S5][0];
                if offline {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x0014_7318, 0x0019_33c8)?;
                    c.g[S4][0] = c.g[S5][0];
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x0014_7318, 0x0019_33b8)?;
                    c.g[S4][0] = 0;
                }
                'panel: {
                    'confirm_checked: {
                        // input->setController(id)
                        c.cycles += 15;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        c.g[V0][0] = c.lw(oc849_at(c, S3, 0x8));
                        c.g[A1][0] &= 0xff;
                        c.g[A0][0] = c.lh(oc849_at(c, V0, 0x160));
                        c.g[A2][0] = c.lw(oc849_at(c, V0, 0x164));
                        let method = c.g[A2][0] as u32;
                        c.g[A0][0] = sx(oc849_at(c, S3, 0).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0019_33ec)?;
                        // input->confirmPressed()
                        c.g[V1][0] = c.lw(oc849_at(c, S3, 0x8));
                        c.g[A0][0] = c.lh(oc849_at(c, V1, 0x140));
                        c.g[V0][0] = c.lw(oc849_at(c, V1, 0x144));
                        let method = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(oc849_at(c, S3, 0).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0019_3400)?;
                        if c.g[V0][0] == 0 {
                            c.g[V1][0] = c.lw(oc849_at(c, S3, 0x8));
                            break 'confirm_checked;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = 0x34;
                        c.mult(0, 3, 20, 4);
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[V1][0] = c.lw(oc849_at(c, V0, 0x6f8));
                        if c.g[V1][0] != 0 {
                            c.g[V1][0] = c.lw(oc849_at(c, S3, 0x8));
                            break 'confirm_checked;
                        }
                        // menu->vtable[0xf](input): the panel's menu takes the input.
                        c.cycles += 9;
                        c.g[A2][0] = c.lw(oc849_at(c, V0, 0x6d0));
                        c.g[A1][0] = c.g[S3][0];
                        c.g[V1][0] = c.lw(oc849_at(c, A2, 0x8));
                        c.g[A0][0] = c.lh(oc849_at(c, V1, 0x78));
                        c.g[V0][0] = c.lw(oc849_at(c, V1, 0x7c));
                        let method = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(oc849_at(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0019_343c)?;
                        c.g[S5][0] = sx(oc849_at(c, S5, 0x1));
                        break 'panel;
                    }
                    // input->cancelPressed()
                    c.cycles += 6;
                    c.g[A0][0] = c.lh(oc849_at(c, V1, 0x30));
                    c.g[V0][0] = c.lw(oc849_at(c, V1, 0x34));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(oc849_at(c, S3, 0).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0019_3454)?;
                    if c.g[V0][0] == 0 {
                        c.g[S5][0] = sx(oc849_at(c, S5, 0x1));
                        break 'panel;
                    }
                    'cancel_done: {
                        c.cycles += 6;
                        c.g[S0][0] = 0x34;
                        c.mult(0, 3, 20, 16);
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[V1][0] = c.lw(oc849_at(c, V0, 0x6f8));
                        let skip = c.g[V1][0] == 0;
                        c.g[V0][0] = 0x4a_0000;
                        if skip {
                            break 'cancel_done;
                        }
                        // Reset the row visibility and the label prefix on the stack.
                        c.cycles += 22;
                        c.g[A0][0] = sx(frame.at(c, 0x3));
                        c.g[A1][0] = 0;
                        c.g[V1][0] = sx(oc849_at(c, V0, 0x1788));
                        c.g[A3][0] = c.lb(oc849_at(c, V1, 0));
                        c.g[T0][0] = c.lb(oc849_at(c, V1, 0x1));
                        c.g[T1][0] = c.lb(oc849_at(c, V1, 0x2));
                        let p = frame.at(c, 0);
                        c.sb(p, c.g[A3][0]);
                        let p = frame.at(c, 0x1);
                        c.sb(p, c.g[T0][0]);
                        let p = frame.at(c, 0x2);
                        c.sb(p, c.g[T1][0]);
                        c.g[A2][0] = 0x2;
                        call_ptr(c, 0x0041_6210, 0x0019_34a0)?;
                        c.mult(0, 17, 20, 16);
                        c.g[A1][0] = 0;
                        c.g[S0][0] = sx(oc849_at(c, S1, 0x6d0));
                        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[A0][0] = c.g[S0][0];
                        OC849_SET_ROWS_VISIBLE.call(c, 0x0019_34b8)?;
                        c.g[A1][0] = c.lw(oc849_at(c, S2, 0x40));
                        c.g[A0][0] = c.g[S0][0];
                        OC849_HIDE_UNUSED_ROWS.call(c, 0x0019_34c4)?;
                        let first_panel = c.g[S4][0] == 0;
                        c.g[V0][0] = 0x32;
                        if !first_panel {
                            c.cycles += 1;
                            let p = frame.at(c, 0);
                            c.sb(p, c.g[V0][0]);
                        }
                        // screen->gotoFrame(label)
                        c.cycles += 13;
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[A1][0] = 0;
                        c.g[A0][0] = c.lw(oc849_at(c, V0, 0x6d0));
                        call_ptr(c, 0x0039_b960, 0x0019_34e0)?;
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x0031_7670, 0x0019_34e8)?;
                        c.g[A0][0] = c.lw(oc849_at(c, S2, 0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x0019_34f4)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let no_frame = c.g[A1][0] == c.g[V0][0];
                        c.g[A1][0] &= 0xffff;
                        if !no_frame {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(oc849_at(c, S2, 0x40));
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x0039_c870, 0x0019_3510)?;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = is_net(c);
                        c.g[V0][0] ^= 0x1;
                        if c.g[V0][0] != 0 {
                            c.g[S5][0] = sx(oc849_at(c, S5, 0x1));
                            break 'panel;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                        call_ptr(c, 0x0026_5700, 0x0019_3528)?;
                    }
                    c.cycles += 1;
                    c.g[S5][0] = sx(oc849_at(c, S5, 0x1));
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[S7][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = is_net(c);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_33a0);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `widget->vtable[slot](a1)` as the compiler emitted it for the `+0x38/0x3c`
/// slot (`setFocus(1)`): `a0 = widget + lh(vt + 0x38)`, target `lw(vt + 0x3c)`.
fn oc849_widget_focus(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A1][0] = 0x1;
    c.g[V0][0] = c.lw(oc849_at(c, S2, 0x8));
    c.g[A0][0] = c.lh(oc849_at(c, V0, 0x38));
    c.g[V1][0] = c.lw(oc849_at(c, V0, 0x3c));
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx(oc849_at(c, S2, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret)
}

/// The two back-end interface lookups (`cBE_getInterface(1)` and `(2)`) of the
/// player-name widgets; leaves `s1` = interface 1, `s0` = interface 2.
fn oc849_player_name_text(c: &mut Cpu, rets: [u32; 4], player: u64, charge: u64) -> Flow {
    c.cycles += charge;
    call_ptr(c, 0x0014_dc80, rets[0])?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x1;
    call_ptr(c, 0x0014_dd58, rets[1])?;
    c.g[S1][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dc80, rets[2])?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x0014_dd58, rets[3])?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = player;
    Ok(())
}

/// `void cFEStateBraggingRights_OnWidgetCreated(self, widget)` (0x193650): file the
/// widget the screen created by its name hash: "1Menu" / "2Menu" become the
/// player's menu control (`self + 0x6d0` / `+ 0x704`, focused, flagged), the
/// player name texts get the rider's short name, the two info groups go to
/// `self + 0x6d4` / `+ 0x708`, and the row widgets "11".."28" (player, row) fill
/// the item slots (`self + 0x6d8 + 0x34 * player + 4 * row`), flagged static.
pub fn c_fe_state_bragging_rights_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S6, 0x40);
        frame.save(c, S2, 0x80);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S1, 0x90);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x70);
        c.g[A0][0] = 0x4a_0000;
        frame.save(c, S4, 0x60);
        c.g[A0][0] = sx(oc849_at(c, A0, 0x17a0));
        frame.save(c, S5, 0x50);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        frame.save(c, S0, 0xa0);
        c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
        call_ptr(c, OC849_STR_HASH, 0x0019_3694)?;
        // true: left through the `+1` exit (the lq of s0 comes after it).
        let counted_exit: bool = 'main: {
            // "1Menu" / "2Menu": the player's menu control.
            let menu_found = 'menu: {
                if c.g[S0][0] == c.g[V0][0] {
                    c.cycles += 9;
                    c.sw(oc849_at(c, S6, 0x6d0), 0);
                    oc849_widget_focus(c, 0x0019_36b8)?;
                    c.sw(oc849_at(c, S6, 0x6d0), c.g[S2][0]);
                    break 'menu true;
                }
                c.g[A0][0] = 0x4a_0000;
                c.cycles += 5;
                c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
                c.g[A0][0] = sx(oc849_at(c, A0, 0x17a8));
                call_ptr(c, OC849_STR_HASH, 0x0019_36cc)?;
                if c.g[S0][0] == c.g[V0][0] {
                    c.cycles += 8;
                    c.sw(oc849_at(c, S6, 0x704), 0);
                    oc849_widget_focus(c, 0x0019_36f0)?;
                    c.sw(oc849_at(c, S6, 0x704), c.g[S2][0]);
                    break 'menu true;
                }
                c.g[A0][0] = 0x46_0000;
                false
            };
            if menu_found {
                c.cycles += 7;
                c.g[V0][0] = c.lw(oc849_at(c, S2, 0x14));
                c.g[V1][0] = c.lw(oc849_at(c, S2, 0x90));
                c.g[V0][0] |= 0x1;
                c.g[V1][0] |= 0x1;
                c.sw(oc849_at(c, S2, 0x14), c.g[V0][0]);
                c.sw(oc849_at(c, S2, 0x90), c.g[V1][0]);
                break 'main true;
            }
            // "Player1_title" / "Player1_name": a1 = 0; the Player2 pair: a1 = 1.
            c.cycles += 5;
            c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
            c.g[A0][0] = sx(oc849_at(c, A0, 0).wrapping_sub(0x2c8));
            call_ptr(c, OC849_STR_HASH, 0x0019_371c)?;
            let mut player1 = c.g[S0][0] == c.g[V0][0];
            c.g[A0][0] = 0x46_0000;
            if !player1 {
                c.cycles += 5;
                c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
                c.g[A0][0] = sx(oc849_at(c, A0, 0).wrapping_sub(0x2b8));
                call_ptr(c, OC849_STR_HASH, 0x0019_3730)?;
                player1 = c.g[S0][0] == c.g[V0][0];
                c.g[A0][0] = 0x46_0000;
            }
            let mut player_text = false;
            if player1 {
                oc849_player_name_text(
                    c,
                    [0x0019_3740, 0x0019_374c, 0x0019_3754, 0x0019_3760],
                    0,
                    14,
                )?;
                player_text = true;
            } else {
                c.cycles += 5;
                c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
                c.g[A0][0] = sx(oc849_at(c, A0, 0).wrapping_sub(0x2a8));
                call_ptr(c, OC849_STR_HASH, 0x0019_377c)?;
                let mut player2 = c.g[S0][0] == c.g[V0][0];
                c.g[A0][0] = 0x46_0000;
                if !player2 {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
                    c.g[A0][0] = sx(oc849_at(c, A0, 0).wrapping_sub(0x298));
                    call_ptr(c, OC849_STR_HASH, 0x0019_3790)?;
                    player2 = c.g[S0][0] == c.g[V0][0];
                    c.g[A0][0] = 0x46_0000;
                }
                if player2 {
                    oc849_player_name_text(
                        c,
                        [0x0019_37a0, 0x0019_37ac, 0x0019_37b4, 0x0019_37c0],
                        1,
                        13,
                    )?;
                    player_text = true;
                }
            }
            if player_text {
                // cUIText_setAsciiString(widget, cBECharacterInterface_getShortName(...))
                c.cycles += 10;
                call_ptr(c, 0x0014_6e98, 0x0019_37d4)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0014_ee58, 0x0019_37e0)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0c50, 0x0019_37ec)?;
                frame.restore(c, S0, 0xa0);
                break 'main false;
            }
            // "player1_info" / "player2_info": the info groups.
            c.cycles += 5;
            c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
            c.g[A0][0] = sx(oc849_at(c, A0, 0).wrapping_sub(0x288));
            call_ptr(c, OC849_STR_HASH, 0x0019_3800)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 2;
                c.sw(oc849_at(c, S6, 0x6d4), c.g[S2][0]);
                break 'main true;
            }
            c.g[A0][0] = 0x46_0000;
            c.cycles += 5;
            c.g[S0][0] = c.lw(oc849_at(c, S2, 0x38));
            c.g[A0][0] = sx(oc849_at(c, A0, 0).wrapping_sub(0x278));
            call_ptr(c, OC849_STR_HASH, 0x0019_381c)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 2;
                c.sw(oc849_at(c, S6, 0x708), c.g[S2][0]);
                break 'main true;
            }
            let p = frame.at(c, 0x2);
            c.sb(p, 0);
            // The row widgets: names "<player><row>" for 2 players x 8 rows.
            c.cycles += 5;
            c.g[S3][0] = 0;
            c.g[FP][0] = 0x34;
            c.g[S7][0] = 0xffff_ffff_ffff_fffe;
            c.mult(0, 3, 19, 30);
            loop {
                c.cycles += 6;
                c.g[S1][0] = 0;
                c.g[S4][0] = sx(oc849_at(c, S3, 0x31));
                c.g[S5][0] = sx(oc849_at(c, S3, 0x1));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S6][0] as u32));
                c.g[S0][0] = sx(oc849_at(c, V0, 0x6d8));
                loop {
                    c.cycles += 8;
                    c.g[V0][0] = sx(oc849_at(c, S1, 0x31));
                    let p = frame.at(c, 0);
                    c.sb(p, c.g[S4][0]);
                    let p = frame.at(c, 0x1);
                    c.sb(p, c.g[V0][0]);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, OC849_STR_HASH, 0x0019_386c)?;
                    c.g[V1][0] = c.lw(oc849_at(c, S2, 0x38));
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[S1][0] = sx(oc849_at(c, S1, 0x1));
                    } else {
                        c.cycles += 12;
                        c.sw(c.g[S0][0] as u32, c.g[S2][0]);
                        c.g[A1][0] = 0x1;
                        c.sw(oc849_at(c, S2, 0x18), c.g[S3][0]);
                        c.g[V0][0] = c.lw(oc849_at(c, S2, 0x14));
                        c.g[V1][0] = c.lw(oc849_at(c, S2, 0x8));
                        c.g[V0][0] &= c.g[S7][0];
                        c.sw(oc849_at(c, S2, 0x14), c.g[V0][0]);
                        c.g[A0][0] = c.lh(oc849_at(c, V1, 0xc0));
                        c.g[V0][0] = c.lw(oc849_at(c, V1, 0xc4));
                        let method = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(oc849_at(c, S2, 0).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0019_38a4)?;
                        c.g[S1][0] = sx(oc849_at(c, S1, 0x1));
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 8i64) as u64;
                    let more_rows = c.g[V0][0] != 0;
                    c.g[S0][0] = sx(oc849_at(c, S0, 0x4));
                    if more_rows {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_3858);
                        }
                        continue;
                    }
                    break;
                }
                c.cycles += 4;
                c.g[S3][0] = c.g[S5][0];
                c.g[V0][0] = ((c.g[S3][0] as i64) < 2i64) as u64;
                let more_players = c.g[V0][0] != 0;
                c.mult(0, 3, 19, 30);
                if more_players {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_3840);
                    }
                    continue;
                }
                break 'main true;
            }
        };
        if counted_exit {
            c.cycles += 1;
            frame.restore(c, S0, 0xa0);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::SP as OC849_STACK;
    use crate::game::main::loadscreens::tests::{
        oc736_fake_call, oc736_fixture, oc736_force, oc736_force_echo, oc736_force_zero, FX_CTRL,
        FX_MAGIC,
    };
    use std::sync::atomic::AtomicUsize;

    /// The state object, the screen value, the items and their vtable, the
    /// back-end object, and the menu controls' objects (all clear of the fake
    /// log and of the fixture of `oc736_fixture`).
    pub(crate) const OC849_THIS: u32 = DATA + 0x70_0000;
    pub(crate) const OC849_SCREEN: u32 = DATA + 0x70_1000;
    pub(crate) const OC849_ITEM: u32 = DATA + 0x71_0000;
    pub(crate) const OC849_VT: u32 = DATA + 0x72_0000;
    pub(crate) const OC849_WIDGET: u32 = DATA + 0x72_1000;
    pub(crate) const OC849_ENG: u32 = DATA + 0x73_0000;
    pub(crate) const OC849_BE: u32 = DATA + 0x73_1000;
    pub(crate) const OC849_BEVT: u32 = DATA + 0x73_2000;
    pub(crate) const OC849_MENU: u32 = DATA + 0x74_0000;
    pub(crate) const OC849_MENUVT: u32 = DATA + 0x75_0000;
    /// `RiderIface_HasVisited`: a real leaf that returns 0, as a method target.
    pub(crate) const OC849_ZERO: u32 = 0x0014_06c0;

    /// Gated fakes (gate: `FX_CTRL == FX_MAGIC`, written by `oc736_fixture`) for
    /// every callee with an override slot; with the gate off they run the body
    /// they replaced.
    macro_rules! oc849_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(FX_CTRL) != FX_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc736_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc849_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc849_fakes!(
        (
            oc849_fake_317670,
            OC849_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc849_fake_39d860,
            OC849_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc849_fake_39b960,
            OC849_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
        (
            oc849_fake_39c778,
            OC849_PREV_39C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778
        ),
        (
            oc849_fake_39c870,
            OC849_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc849_fake_3a0c50,
            OC849_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc849_fake_39e6b8,
            OC849_PREV_39E6B8,
            ee::s13::OVERRIDE_0039E6B8,
            ee::s13::f_0039e6b8__generated,
            0x0039_e6b8
        ),
        (
            oc849_fake_14dc80,
            OC849_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc849_fake_14dd58,
            OC849_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc849_fake_147318,
            OC849_PREV_147318,
            ee::s01::OVERRIDE_00147318,
            ee::s01::f_00147318__generated,
            0x0014_7318
        ),
        (
            oc849_fake_146e98,
            OC849_PREV_146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x0014_6e98
        ),
        (
            oc849_fake_14ee58,
            OC849_PREV_14EE58,
            ee::s01::OVERRIDE_0014EE58,
            ee::s01::f_0014ee58__generated,
            0x0014_ee58
        ),
        (
            oc849_fake_416210,
            OC849_PREV_416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
        (
            oc849_fake_265700,
            OC849_PREV_265700,
            ee::s07::OVERRIDE_00265700,
            ee::s07::f_00265700__generated,
            0x0026_5700
        ),
        (
            oc849_fake_192740,
            OC849_PREV_192740,
            ee::s02::OVERRIDE_00192740,
            ee::s02::f_00192740__generated,
            0x0019_2740
        ),
    );

    /// Zero `bytes` bytes at `base`.
    fn oc849_clear(c: &mut Cpu, base: u32, bytes: u32) {
        for i in 0..bytes / 4 {
            c.bus.write32(base + 4 * i, 0);
        }
    }

    /// A vtable method slot `{delta, target}`.
    fn oc849_slot(c: &mut Cpu, vt: u32, off: u32, delta: u16, target: u32) {
        c.bus.write16(vt + off, delta);
        c.bus.write32(vt + off + 4, target);
    }

    /// The common machine: the game image (string tables), the gated fakes and
    /// the shared fixture.
    fn oc849_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc849_install_fakes();
        elf_image(c);
        oc736_fixture(c, seed, salt, calls)
    }

    /// The item / widget vtable: `setStatic` at 0x40, `setVisible` at 0x48,
    /// `setFocus` at 0x38, flag setter at 0xc0 (all fake methods).
    fn oc849_item_vtable(c: &mut Cpu, seed: u32) {
        let delta = 0x10 * (seed % 3) as u16;
        for (i, off) in [0x38u32, 0x40, 0x48, 0xc0].into_iter().enumerate() {
            oc849_slot(c, OC849_VT, off, delta, fakes::FAKE + 0x10 * i as u32);
        }
    }

    #[test]
    fn find_row_index_matches() {
        check(
            ee::s02::f_00192918__generated,
            c_fe_state_bragging_rights_find_row_index,
            64,
            &|c, seed| {
                fakes::machine(c, seed, 0x19_2918, 0);
                let this = DATA + 0x100;
                for i in 0..8u32 {
                    c.bus.write32(this + 8 + 4 * i, (i * 3 + seed) % 12);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = ((seed / 2) % 13) as u64;
                c.pc = 0x0019_2918;
            },
        );
    }

    #[test]
    fn set_row_mask_matches() {
        check(
            ee::s02::f_00192968__generated,
            c_fe_state_bragging_rights_set_row_mask,
            16,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x19_2968, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = r.next_u32() as u64;
                c.pc = 0x0019_2968;
            },
        );
    }

    /// One call to `UIState_Render`, which also unwinds at chosen seeds.
    #[test]
    fn render_matches() {
        check(
            ee::s02::f_00193318__generated,
            c_fe_state_bragging_rights_render,
            60,
            &|c, seed| {
                oc849_base(c, seed, 0x19_3318, 1);
                c.g[A0][0] = sx(OC849_THIS);
                c.pc = 0x0019_3318;
            },
        );
    }

    /// Mask bits set / clear, rows with and without an item, and the name
    /// lookups finding the widget or not.
    #[test]
    fn hide_unused_rows_matches() {
        check(
            ee::s02::f_00192970__generated,
            c_fe_state_bragging_rights_hide_unused_rows,
            120,
            &|c, seed| {
                let mut r = oc849_base(c, seed, 0x19_2970, 40);
                oc849_clear(c, OC849_THIS, 0x100);
                c.bus.write32(OC849_THIS + 0x2c, r.next_u32() & 0xff);
                for i in 0..8u32 {
                    let item = if (seed + i) % 3 == 0 {
                        0
                    } else {
                        OC849_ITEM + i * 0x100
                    };
                    c.bus.write32(OC849_THIS + 8 + 4 * i, item);
                    if item != 0 {
                        c.bus.write32(item + 8, OC849_VT);
                    }
                }
                oc849_item_vtable(c, seed);
                c.bus.write32(OC849_WIDGET + 8, OC849_VT);
                oc736_force_echo(c, 0x0031_7670);
                if seed % 3 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC849_WIDGET);
                }
                c.g[A0][0] = sx(OC849_THIS);
                c.g[A1][0] = sx(OC849_SCREEN);
                c.pc = 0x0019_2970;
            },
        );
    }

    /// Online (one panel) and offline (two), confirm / cancel / neither
    /// presses, the panel flag set or clear, the screen frame found or not.
    #[test]
    fn update_matches() {
        check_lohi(
            ee::s02::f_00193338__generated,
            c_fe_state_bragging_rights_update,
            240,
            &|c, seed| {
                let mut r = oc849_base(c, seed, 0x19_3338, 14);
                oc849_clear(c, OC849_THIS, 0x800);
                for base in [
                    OC849_ENG,
                    OC849_BE,
                    OC849_BEVT,
                    OC849_MENU,
                    OC849_MENU + 0x100,
                ] {
                    oc849_clear(c, base, 0x200);
                }
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x1680), (seed % 2) as u32);
                c.bus.write32(GAME_GP + 0x2dc, r.next_u32());
                c.bus.write32(OC849_THIS + 0x10, OC849_ENG);
                c.bus.write32(OC849_THIS + 0x40, OC849_SCREEN);
                c.bus.write32(OC849_ENG + 0xc, OC849_BE);
                c.bus.write32(OC849_BE + 8, OC849_BEVT);
                let delta = 0x10 * (seed % 3) as u16;
                oc849_slot(c, OC849_BEVT, 0x160, delta, fakes::FAKE + 0x10);
                let confirm = if (seed / 2) % 3 == 0 {
                    OC849_ZERO
                } else {
                    fakes::FAKE + 0x20
                };
                oc849_slot(c, OC849_BEVT, 0x140, delta, confirm);
                let cancel = if (seed / 6) % 3 == 0 {
                    OC849_ZERO
                } else {
                    fakes::FAKE + 0x30
                };
                oc849_slot(c, OC849_BEVT, 0x30, delta, cancel);
                oc849_slot(c, OC849_MENUVT, 0x78, delta, fakes::FAKE + 0x40);
                oc849_item_vtable(c, seed);
                c.bus.write32(OC849_WIDGET + 8, OC849_VT);
                for i in 0..2u32 {
                    let ctl = OC849_THIS + 0x6d0 + 0x34 * i;
                    let menu = OC849_MENU + 0x100 * i;
                    c.bus.write32(ctl, menu);
                    c.bus.write32(menu + 8, OC849_MENUVT);
                    c.bus.write32(ctl + 0x28, (seed >> (3 + i)) & 1);
                    c.bus.write32(ctl + 0x2c, r.next_u32() & 0xff);
                    for j in 0..8u32 {
                        if (seed >> (j % 5)) & 1 == 1 {
                            let item = OC849_ITEM + 0x100 * (8 * i + j);
                            c.bus.write32(ctl + 8 + 4 * j, item);
                            c.bus.write32(item + 8, OC849_VT);
                        }
                    }
                }
                oc736_force_echo(c, 0x0031_7670);
                if seed % 5 == 0 {
                    oc736_force(c, 0x0039_c778, 0xffff);
                }
                if seed % 3 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC849_WIDGET);
                }
                c.g[A0][0] = sx(OC849_THIS);
                c.pc = 0x0019_3338;
            },
        );
    }

    /// Every widget kind the screen creates: the two menus, the four player
    /// name texts, the two info groups, the row widgets (name hash equals the
    /// stack string) and an unknown widget.
    #[test]
    fn on_widget_created_matches() {
        check_lohi(
            ee::s02::f_00193650__generated,
            c_fe_state_bragging_rights_on_widget_created,
            240,
            &|c, seed| {
                let mut r = oc849_base(c, seed, 0x19_3650, 14);
                oc849_clear(c, OC849_THIS, 0x800);
                oc849_clear(c, OC849_WIDGET, 0x100);
                oc849_item_vtable(c, seed);
                c.bus.write32(OC849_WIDGET + 8, OC849_VT);
                let names = [
                    0x004a_17a0u32,
                    0x004a_17a8,
                    0x0045_fd38,
                    0x0045_fd48,
                    0x0045_fd58,
                    0x0045_fd68,
                    0x0045_fd78,
                    0x0045_fd88,
                    OC849_STACK - 0xb0,
                    OC849_STACK - 0xb0,
                    0,
                    0x1234,
                ];
                c.bus
                    .write32(OC849_WIDGET + 0x38, names[(seed % 12) as usize]);
                c.bus.write32(OC849_WIDGET + 0x14, r.next_u32());
                c.bus.write32(OC849_WIDGET + 0x90, r.next_u32());
                oc736_force_echo(c, 0x0031_7670);
                c.g[A0][0] = sx(OC849_THIS);
                c.g[A1][0] = sx(OC849_WIDGET);
                c.pc = 0x0019_3650;
            },
        );
    }
}
