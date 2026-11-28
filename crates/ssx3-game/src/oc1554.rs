//! Staging module oc1554 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1554`.

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
    (0x002c_a448, menu_sys_item_get_up_axis, "MenuSysItem_GetUpAxis"),
    (0x002c_a468, menu_sys_item_get_down_axis, "MenuSysItem_GetDownAxis"),
    (0x002c_a488, menu_sys_item_is_slow_mod_held, "MenuSysItem_IsSlowModHeld"),
    (0x002c_a4a8, menu_sys_item_is_fine_mod_held, "MenuSysItem_IsFineModHeld"),
    (0x002c_a988, menu_sys_item_draw_row, "MenuSysItem_DrawRow"),
    (0x002c_bd50, menu_sys_page_remove_item, "MenuSysPage_RemoveItem"),
    (0x002c_be20, menu_sys_page_sort_items, "MenuSysPage_SortItems"),
    (0x002c_c260, menu_sys_get_left_axis, "MenuSys_GetLeftAxis"),
];

/// The item wrappers of the menu axis queries: `item->menu` is the word at
/// `*item + 0x124`, and the query `callee(menu)` answers for the item. Shared
/// body of `MenuSysItem_GetUpAxis` and its three siblings (0x2ca448..0x2ca4a8):
/// each differs only in the callee and its return address.
fn menu_sys_item_forward(c: &mut Cpu, target: Callee, ret: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x124).lw(c);
    target.call(c, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `float MenuSysItem_GetUpAxis(MenuSysItem* item)` (0x2ca448): the item's
/// menu's up axis, from `MenuSys_GetUpAxis`.
pub fn menu_sys_item_get_up_axis(c: &mut Cpu) {
    guest(c, |c| {
        let target = Callee { addr: 0x002c_c2a0, f: ee::s09::MenuSys_GetUpAxis };
        menu_sys_item_forward(c, target, 0x002c_a45c)
    })
}

/// `float MenuSysItem_GetDownAxis(MenuSysItem* item)` (0x2ca468): the item's
/// menu's down axis, from `MenuSys_GetDownAxis`.
pub fn menu_sys_item_get_down_axis(c: &mut Cpu) {
    guest(c, |c| {
        let target = Callee { addr: 0x002c_c2c0, f: ee::s09::MenuSys_GetDownAxis };
        menu_sys_item_forward(c, target, 0x002c_a47c)
    })
}

/// `bool MenuSysItem_IsSlowModHeld(MenuSysItem* item)` (0x2ca488): whether the
/// item's menu has the slow modifier held, from `MenuSys_IsSlowModHeld`.
pub fn menu_sys_item_is_slow_mod_held(c: &mut Cpu) {
    guest(c, |c| {
        let target = Callee { addr: 0x002c_c2e0, f: ee::s09::MenuSys_IsSlowModHeld };
        menu_sys_item_forward(c, target, 0x002c_a49c)
    })
}

/// `bool MenuSysItem_IsFineModHeld(MenuSysItem* item)` (0x2ca4a8): whether the
/// item's menu has the fine modifier held, from `MenuSys_IsFineModHeld`.
pub fn menu_sys_item_is_fine_mod_held(c: &mut Cpu) {
    guest(c, |c| {
        let target = Callee { addr: 0x002c_c318, f: ee::s09::MenuSys_IsFineModHeld };
        menu_sys_item_forward(c, target, 0x002c_a4bc)
    })
}

/// `float MenuSys_GetLeftAxis(MenuSys* menu)` (0x2cc260): the input binding
/// at `menu+0x5c` evaluated for the left axis (code 0x3e) through the function
/// table (`InputBinding_EvalFloat`, 0x320bf0).
pub fn menu_sys_get_left_axis(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x3e;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x5c).lw(c);
        call_ptr(c, 0x0032_0bf0, 0x002c_c274)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_SortItems(MenuSysPage* page, int key)` (0x2cbe20): stores
/// the sort key in the global at `gp+0x23cc`, qsorts the page's items (count
/// at `+0`, 4-byte entries from `+0xc`) with the comparator at 0x2d4008, clears
/// the global again and re-validates the selection.
pub fn menu_sys_page_sort_items(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        let key_global = (c.g[GP][0] as u32).wrapping_add(0x23cc);
        c.sw(key_global, c.g[A1][0]);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A3][0] = 0x002d_4008;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A2][0] = 0x4;
        call_ptr(c, 0x0041_8ef8, 0x002c_be4c)?;
        c.g[A0][0] = c.g[S0][0];
        c.sw(key_global, 0u64);
        let validate = Callee { addr: 0x002c_aab0, f: ee::s09::MenuSysPage_ValidateSelection };
        validate.call(c, 0x002c_be58)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_RemoveItem(MenuSysPage* page, int index)` (0x2cbd50): shifts
/// the items after `index` down one slot (count at `+0`, selection at `+4`,
/// entries from `+0xc`), drops the count, clamps the selection to the new count
/// and re-validates it when it is now `index`.
fn remove_item_body(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    'b_remove_shift: {
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[A1][0];
        if t {
            break 'b_remove_shift;
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        loop {
            c.cycles += 8;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            if !more {
                break 'b_remove_shift;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_bd80);
            }
        }
    }
    'b_remove_count: {
        c.cycles += 6;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        let t = c.g[V1][0] == 0u64;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        if t {
            break 'b_remove_count;
        }
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    }
    'b_remove_clamp: {
        c.cycles += 5;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0u64 {
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            break 'b_remove_clamp;
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
    }
    'b_remove_validate: {
        c.cycles += 2;
        if c.g[A1][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            break 'b_remove_validate;
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        let validate = Callee { addr: 0x002c_aab0, f: ee::s09::MenuSysPage_ValidateSelection };
        validate.call(c, 0x002c_bdf0)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
    }
    {
        c.cycles += 2;
        if (c.g[V0][0] as i64) < 0 {
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        }
    }
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// See [`remove_item_body`].
pub fn menu_sys_page_remove_item(c: &mut Cpu) {
    guest(c, |c| remove_item_body(c))
}

/// `void MenuSysItem_DrawRow(MenuSysItem* item, ...)` (0x2ca988): picks the
/// row's text colour words from the page's selection (the item's own row is
/// the selected one: `0x4d5380`/`0x4d53a0`, others `0x4d5390`/`0x4d53b0`), loads
/// three float words from the table entry and draws the row text through
/// `MenuSysItem_DrawRowText`.
fn draw_row_body(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S0, 0x60);
    frame.save(c, S1, 0x50);
    frame.save(c, S2, 0x40);
    frame.save(c, S3, 0x30);
    frame.save(c, S4, 0x20);
    frame.save_ra(c, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A2][0];
    c.g[S2][0] = c.g[A1][0];
    c.g[S3][0] = c.g[A3][0];
    c.g[S4][0] = c.g[T0][0];
    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
    let selected = Callee { addr: 0x002c_bf30, f: ee::s09::MenuSysPage_GetSelectedItem };
    selected.call(c, 0x002c_a9c0)?;
    c.g[V0][0] = c.g[S0][0] ^ c.g[V0][0];
    c.g[V1][0] = (c.g[V0][0] < 0x1u64) as u64;
    let t = c.g[V1][0] == 0u64;
    c.g[V0][0] = 0x004d_0000;
    'b_row_selected: {
        'b_row_first: {
            if t {
                break 'b_row_first;
            }
            c.cycles += 2;
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5380));
            break 'b_row_selected;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x004d_0000;
        c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5390));
    }
    'b_row_colour: {
        'b_row_colour_inner: {
            c.cycles += 2;
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = 0x004d_0000;
            if t {
                break 'b_row_colour_inner;
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
            break 'b_row_colour;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x004d_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53b0));
    }
    c.cycles += 24;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S1][0];
    c.swc1(0, (c.g[SP][0] as u32));
    c.g[A3][0] = c.g[S3][0];
    c.g[T0][0] = c.g[S4][0];
    c.g[T2][0] = c.g[SP][0];
    c.lwc1(1, (c.g[V0][0] as u32));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
    let text = Callee { addr: 0x002c_a768, f: ee::s09::MenuSysItem_DrawRowText };
    text.call(c, 0x002c_aa38)?;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// `void MenuSysItem_DrawRow(MenuSysItem* item, ...)` (0x2ca988): see
/// [`draw_row_body`].
pub fn menu_sys_item_draw_row(c: &mut Cpu) {
    guest(c, |c| draw_row_body(c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{self, check, fakes, scramble, Rng, DATA};
    use ssx3_ee as ee;
    use std::ptr::addr_of_mut;

    const SEEDS: u32 = 200;

    /// Gate words (unique in each test machine) and the magic that opens them
    /// for the gated fakes below.
    const GATE_VALIDATE: u32 = DATA + 0x0020_0000;
    const GATE_SELECTED: u32 = DATA + 0x0020_0004;
    const GATE_TEXT: u32 = DATA + 0x0020_0008;
    const MAGIC: u32 = 0x1554_c0de;

    /// The input-binding VM table and its stop opcode (as the camera tests use).
    const VM_TABLE: u32 = 0x0048_e330;
    const VM_STOP_OP: u32 = 0x20;
    const VM_STOP: u32 = 0x0032_5ef8;

    fn oc1554_fake_validate(c: &mut Cpu) {
        if c.bus.read32(GATE_VALIDATE) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1554_fake_validate),
                ee::s09::f_002caab0__generated,
            );
        }
        fakes::fake_call(c, 0x002c_aab0);
    }

    fn oc1554_fake_selected(c: &mut Cpu) {
        if c.bus.read32(GATE_SELECTED) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1554_fake_selected),
                ee::s09::f_002cbf30__generated,
            );
        }
        fakes::fake_call(c, 0x002c_bf30);
    }

    fn oc1554_fake_text(c: &mut Cpu) {
        if c.bus.read32(GATE_TEXT) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1554_fake_text),
                ee::s09::f_002ca768__generated,
            );
        }
        fakes::fake_call(c, 0x002c_a768);
    }

    /// Hook the gated fakes into their callees' slots (once per process).
    fn oc1554_hooks() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CAAB0),
            oc1554_fake_validate,
            fakes::prev_cell(oc1554_fake_validate),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CBF30),
            oc1554_fake_selected,
            fakes::prev_cell(oc1554_fake_selected),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA768),
            oc1554_fake_text,
            fakes::prev_cell(oc1554_fake_text),
        );
    }

    /// A menu's binding and the input map behind it, so that
    /// `InputBinding_EvalFloat` (0x320bf0) runs for real and returns a value.
    fn oc1554_input_chain(c: &mut Cpu, menu: u32, seed: u32) {
        let binding = DATA + 0xa00;
        let map = DATA + 0xc00;
        c.bus.write32(menu + 0x5c, binding);
        c.bus.write32(binding, 0); // the map's binding index
        c.bus.write32(binding + 4, map);
        c.bus.write32(map + 0x4, map + 0x80);
        c.bus.write32(map + 0x14, map + 0x100);
        for i in 0..4u32 {
            c.bus.write32(map + 0x80 + 4 * i, i);
            c.bus.write32(map + 0x100 + 4 * i, VM_STOP_OP);
        }
        c.bus.write32(VM_TABLE + 4 * VM_STOP_OP, VM_STOP);
        // The stopped VM leaves `*(parser + 8)` in f0; the parser is the frame
        // of `InputMap_EvalFloat`, four frames below the test's stack. Both
        // compare outcomes of the menu queries' bools.
        let value = if seed % 2 == 0 { 0.0f32 } else { 1.0f32 };
        c.bus.write32(difftest::SP - 0x1f8, value.to_bits());
    }

    /// An item whose first word points at a holder with the menu at `+0x124`.
    fn oc1554_item_chain(c: &mut Cpu, seed: u32) {
        let item = DATA;
        let holder = DATA + 0x100;
        let menu = DATA + 0x800;
        c.bus.write32(item, holder);
        c.bus.write32(holder + 0x124, menu);
        oc1554_input_chain(c, menu, seed);
    }

    #[test]
    fn menu_sys_item_get_up_axis_matches() {
        check(
            ee::s09::f_002ca448__generated,
            menu_sys_item_get_up_axis,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x002c_a448 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                oc1554_item_chain(c, seed);
            },
        );
    }

    #[test]
    fn menu_sys_item_get_down_axis_matches() {
        check(
            ee::s09::f_002ca468__generated,
            menu_sys_item_get_down_axis,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x002c_a468 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                oc1554_item_chain(c, seed);
            },
        );
    }

    #[test]
    fn menu_sys_item_is_slow_mod_held_matches() {
        check(
            ee::s09::f_002ca488__generated,
            menu_sys_item_is_slow_mod_held,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x002c_a488 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                oc1554_item_chain(c, seed);
            },
        );
    }

    #[test]
    fn menu_sys_item_is_fine_mod_held_matches() {
        check(
            ee::s09::f_002ca4a8__generated,
            menu_sys_item_is_fine_mod_held,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x002c_a4a8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                oc1554_item_chain(c, seed);
            },
        );
    }

    #[test]
    fn menu_sys_get_left_axis_matches() {
        check(
            ee::s09::f_002cc260__generated,
            menu_sys_get_left_axis,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x002c_c260 ^ seed as u64);
                scramble(c, &mut r);
                let menu = DATA + 0x800;
                c.g[A0][0] = menu as u64;
                oc1554_input_chain(c, menu, seed);
            },
        );
    }

    #[test]
    fn menu_sys_page_remove_item_matches() {
        oc1554_hooks();
        check(
            ee::s09::f_002cbd50__generated,
            menu_sys_page_remove_item,
            SEEDS,
            &|c, seed| {
                fakes::machine(c, seed, 0x002c_bd50, 0);
                let page = DATA + 0x1000;
                let count = seed % 5;
                c.g[A0][0] = page as u64;
                c.g[A1][0] = ((seed / 5) % 5) as u64;
                c.bus.write32(page, count);
                let selected = if seed % 11 == 0 {
                    u32::MAX // negative: the clamp branch
                } else {
                    (seed / 25) % 5
                };
                c.bus.write32(page + 0x4, selected);
                for i in 0..5u32 {
                    c.bus.write32(page + 0xc + 4 * i, DATA + 0x2000 + 0x10 * i);
                }
                c.bus.write32(GATE_VALIDATE, MAGIC);
            },
        );
    }

    #[test]
    fn menu_sys_page_sort_items_matches() {
        oc1554_hooks();
        check(
            ee::s09::f_002cbe20__generated,
            menu_sys_page_sort_items,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x002c_be20, 0);
                let page = DATA + 0x1000;
                c.g[A0][0] = page as u64;
                c.g[A1][0] = r.next_u32() as u64;
                // Zero or one item: the qsort never calls the comparator.
                c.bus.write32(page, seed % 2);
                c.bus.write32(GATE_VALIDATE, MAGIC);
            },
        );
    }

    #[test]
    fn menu_sys_item_draw_row_matches() {
        oc1554_hooks();
        check(
            ee::s09::f_002ca988__generated,
            menu_sys_item_draw_row,
            SEEDS,
            &|c, seed| {
                fakes::machine(c, seed, 0x002c_a988, 0);
                let page = DATA + 0x1000;
                c.bus.write32(page, DATA + 0x2000);
                c.g[A0][0] = page as u64;
                // The selected item: the page itself (the row is selected) or
                // another pointer (a forced 0 gives a random one).
                let selected = if seed % 2 == 0 { page } else { 0 };
                c.bus.write32(fakes::LOG_RESULTS, selected);
                c.bus.write32(GATE_SELECTED, MAGIC);
                c.bus.write32(GATE_TEXT, MAGIC);
            },
        );
    }
}
