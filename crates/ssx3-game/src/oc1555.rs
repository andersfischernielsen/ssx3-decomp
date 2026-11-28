//! Staging module oc1555 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1555`.

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
    (0x002c_c280, menu_sys_get_right_axis, "MenuSys_GetRightAxis"),
    (0x002c_c2a0, menu_sys_get_up_axis, "MenuSys_GetUpAxis"),
    (0x002c_c2c0, menu_sys_get_down_axis, "MenuSys_GetDownAxis"),
    (0x002c_c2e0, menu_sys_is_slow_mod_held, "MenuSys_IsSlowModHeld"),
    (0x002c_c318, menu_sys_is_fine_mod_held, "MenuSys_IsFineModHeld"),
    (0x002c_c500, menu_sys_bool_item_ctor, "MenuSysBoolItem_ctor"),
    (0x002c_c648, menu_sys_bool_item_measure, "MenuSysBoolItem_Measure"),
    (0x002c_c7b0, menu_sys_enum_item_ctor, "MenuSysEnumItem_ctor"),
];

/// `InputBinding_EvalFloat(binding, id)` (0x320bf0): the float for input `id`
/// comes back in f0.
const INPUT_BINDING_EVAL_FLOAT: u32 = 0x0032_0bf0;

/// `float MenuSys_Get*Axis(MenuSys* m)` helper: `InputBinding_EvalFloat(
/// m->binding (+0x5c), id)`, the callee's f0 returned unchanged. `ret_addr` is
/// the return address after the call.
fn menu_sys_eval_axis(c: &mut Cpu, id: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = id as u64;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x5c)).lw(c);
        call_ptr(c, INPUT_BINDING_EVAL_FLOAT, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float MenuSys_GetRightAxis(MenuSys* m)` (0x2cc280): the right-stick axis
/// (input id 0x3f).
pub fn menu_sys_get_right_axis(c: &mut Cpu) {
    menu_sys_eval_axis(c, 0x3f, 0x002c_c294);
}

/// `float MenuSys_GetUpAxis(MenuSys* m)` (0x2cc2a0): input id 0x40.
pub fn menu_sys_get_up_axis(c: &mut Cpu) {
    menu_sys_eval_axis(c, 0x40, 0x002c_c2b4);
}

/// `float MenuSys_GetDownAxis(MenuSys* m)` (0x2cc2c0): input id 0x41.
pub fn menu_sys_get_down_axis(c: &mut Cpu) {
    menu_sys_eval_axis(c, 0x41, 0x002c_c2d4);
}

/// `bool` helper of the modifier keys: `InputBinding_EvalFloat(m->binding,
/// id)` compared with 0.0f (`c.eq.s`), returned as 1 when not equal (a NaN
/// counts as held) and 0 when equal.
fn menu_sys_modifier_held(c: &mut Cpu, id: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = id as u64;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x5c)).lw(c);
        call_ptr(c, INPUT_BINDING_EVAL_FLOAT, ret_addr)?;
        c.mtc1(0, 1);
        c.c_eq_s(0, 1);
        let not_equal = !c.fcc();
        c.g[V0][0] = 0x1;
        if !not_equal {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool MenuSys_IsSlowModHeld(MenuSys* m)` (0x2cc2e0): the slow-modifier
/// input (id 0x42) is non-zero.
pub fn menu_sys_is_slow_mod_held(c: &mut Cpu) {
    menu_sys_modifier_held(c, 0x42, 0x002c_c2f4);
}

/// `bool MenuSys_IsFineModHeld(MenuSys* m)` (0x2cc318): the fine-modifier
/// input (id 0x43) is non-zero.
pub fn menu_sys_is_fine_mod_held(c: &mut Cpu) {
    menu_sys_modifier_held(c, 0x43, 0x002c_c32c);
}

/// `cBoolMenuItem_cBoolMenuItem(self, text, flag, caption, on, off)` (0x2cc500):
/// the base `MenuSysItem_ctor(self, text)`, then the item's words: a3 at +0x14,
/// t0 at +0x18, t1 at +0x1c, a2 at +0x20, and the vtable `0x486e80` at +0x10.
/// Returns `self` in v0.
pub fn menu_sys_bool_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[T1][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A2][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_c534)?;
        c.g[V1][0] = 0x48_0000;
        let item = c.g[S0][0] as u32;
        let flag = c.g[S1][0];
        Addr(item.wrapping_add(0x14)).sw(c, flag);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6e80));
        let caption = c.g[S2][0];
        Addr(item.wrapping_add(0x18)).sw(c, caption);
        let on_text = c.g[S3][0];
        Addr(item.wrapping_add(0x1c)).sw(c, on_text);
        c.g[V0][0] = c.g[S0][0];
        let off_text = c.g[S4][0];
        Addr(item.wrapping_add(0x20)).sw(c, off_text);
        let vtable = c.g[V1][0];
        Addr(item.wrapping_add(0x10)).sw(c, vtable);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The two-row branch of `MenuSysBoolItem_Measure`: lays out the row into the
/// scratch at `sp+0x20` (s0), copies its two eight-byte halves out, lays the
/// row out again with the item's `+0x1c` as t0, then keeps the row whose first
/// float (`sp+0x18`) is not below `sp+0x8`, and copies that row to the output
/// at s2.
fn menu_sys_measure_two_rows(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 32;
    c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x14)).lw(c);
    c.g[A0][0] = c.g[S0][0];
    c.g[A3][0] = 0u64;
    Callee {
        addr: 0x002c_a4c8,
        f: ee::s09::MenuSysItem_MeasureRow,
    }
    .call(c, 0x002c_c688)?;
    c.ldl(2, frame.at(c, 0x27));
    c.ldr(2, frame.at(c, 0x20));
    c.ldl(3, frame.at(c, 0x2f));
    c.ldr(3, frame.at(c, 0x28));
    c.sdl(2, frame.at(c, 0x7));
    c.sdr(2, frame.at(c, 0x0));
    c.sdl(3, frame.at(c, 0xf));
    c.sdr(3, frame.at(c, 0x8));
    c.g[A3][0] = 0u64;
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
    c.g[T0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x1c)).lw(c);
    Callee {
        addr: 0x002c_a4c8,
        f: ee::s09::MenuSysItem_MeasureRow,
    }
    .call(c, 0x002c_c6c0)?;
    c.ldl(6, frame.at(c, 0x27));
    c.ldr(6, frame.at(c, 0x20));
    c.ldl(2, frame.at(c, 0x2f));
    c.ldr(2, frame.at(c, 0x28));
    c.sdl(6, frame.at(c, 0x17));
    c.sdr(6, frame.at(c, 0x10));
    c.sdl(2, frame.at(c, 0x1f));
    c.sdr(2, frame.at(c, 0x18));
    c.lwc1(1, frame.at(c, 0x8));
    c.lwc1(0, frame.at(c, 0x18));
    c.c_lt_s(0, 1);
    let not_less = !c.fcc();
    c.g[V0][0] = sx(frame.at(c, 0x10));
    if !not_less {
        c.cycles += 1;
        c.g[V0][0] = c.g[SP][0];
    }
    c.cycles += 9;
    let src = c.g[V0][0] as u32;
    c.ldl(5, src.wrapping_add(0x7));
    c.ldr(5, src);
    c.ldl(6, src.wrapping_add(0xf));
    c.ldr(6, src.wrapping_add(0x8));
    let dst = c.g[S2][0] as u32;
    c.sdl(5, dst.wrapping_add(0x7));
    c.sdr(5, dst);
    c.sdl(6, dst.wrapping_add(0xf));
    c.sdr(6, dst.wrapping_add(0x8));
    Ok(())
}

/// `void cBoolMenuItem_preRender(...)` (0x2cc648): measures the bool item's
/// row into the output `a0` (kept in s2). With a non-zero `+0x18` and a
/// non-zero `+0x1c` it lays out two rows and keeps one (see
/// `menu_sys_measure_two_rows`); otherwise a single row. `s0` is set on the
/// two-row test but not saved (the generated frame does not save it), and the
/// output is returned in v0.
pub fn menu_sys_bool_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x30);
        c.g[T0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[S2][0] = c.g[A0][0];
        let two_rows = if c.g[T0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x1c)).lw(c);
            c.g[S0][0] = sx(frame.at(c, 0x20));
            c.g[V0][0] != 0
        } else {
            false
        };
        if two_rows {
            menu_sys_measure_two_rows(c, frame)?;
        } else {
            c.cycles += 6;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[A3][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
            c.g[A2][0] = 0u64;
            c.g[T0][0] = 0u64;
            Callee {
                addr: 0x002c_a4c8,
                f: ee::s09::MenuSysItem_MeasureRow,
            }
            .call(c, 0x002c_c738)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `cEnumMenuItem_cEnumMenuItem(self)` (0x2cc7b0): `MenuSysItem_ctor(self,
/// -1)`, then the value word at +0x14 and the words at +0x18 and +0x20 zeroed,
/// and the vtable `0x486e30` at +0x10. Returns `self` in v0.
pub fn menu_sys_enum_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xffff_ffff_ffff_ffffu64;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_c7c8)?;
        c.g[V1][0] = 0x48_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6e30));
        let item = c.g[S0][0] as u32;
        Addr(item.wrapping_add(0x14)).sw(c, 0);
        let vtable = c.g[V1][0];
        Addr(item.wrapping_add(0x10)).sw(c, vtable);
        Addr(item.wrapping_add(0x20)).sw(c, 0);
        Addr(item.wrapping_add(0x18)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fall_back, hook};
    use crate::difftest::{check, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word and magic of the `InputBinding_EvalFloat` fake, and its
    /// forced float table (16 words, indexed by the input id & 15).
    const OC1555_AX_CTRL: u32 = DATA + 0xd_5550;
    const OC1555_AX_MAGIC: u32 = 0x1555_a115;
    const OC1555_AX_VALUES: u32 = DATA + 0xd_5560;

    /// Gate word, call counter and row table of the `MenuSysItem_MeasureRow`
    /// fake (two rows of four floats each).
    const OC1555_MS_CTRL: u32 = DATA + 0xd_55c0;
    const OC1555_MS_MAGIC: u32 = 0x1555_b115;
    const OC1555_MS_CALLS: u32 = DATA + 0xd_55c4;
    const OC1555_MS_VALUES: u32 = DATA + 0xd_55d0;

    /// The item and output buffers the measure fixtures use.
    const OC1555_ITEM: u32 = DATA + 0x1000;
    const OC1555_OUT: u32 = DATA + 0x2000;
    /// The object the item fixtures pass as `self`.
    const OC1555_SELF: u32 = DATA + 0x3000;

    static OC1555_PREV_AXIS: AtomicUsize = AtomicUsize::new(0);
    static OC1555_PREV_MEASURE: AtomicUsize = AtomicUsize::new(0);

    /// `InputBinding_EvalFloat(binding, id)`: the forced float at
    /// `OC1555_AX_VALUES + 4 * (id & 15)` in f0.
    fn oc1555_fake_axis(c: &mut Cpu) {
        if c.bus.read32(OC1555_AX_CTRL) != OC1555_AX_MAGIC {
            return fall_back(c, &OC1555_PREV_AXIS, ee::s10::f_00320bf0__generated);
        }
        let id = c.g[A1][0] as u32;
        c.f[0] = c.bus.read32(OC1555_AX_VALUES + 4 * (id & 0xf));
        c.cycles += 9;
        ret(c);
    }

    /// `MenuSysItem_MeasureRow(out, item, a2, a3, t0)`: writes one row of
    /// four floats into `out` (a0), taken from the table by call parity.
    fn oc1555_fake_measure_row(c: &mut Cpu) {
        if c.bus.read32(OC1555_MS_CTRL) != OC1555_MS_MAGIC {
            return fall_back(c, &OC1555_PREV_MEASURE, ee::s09::f_002ca4c8__generated);
        }
        let n = c.bus.read32(OC1555_MS_CALLS);
        c.bus.write32(OC1555_MS_CALLS, n + 1);
        let out = c.g[A0][0] as u32;
        for i in 0..4 {
            let w = c.bus.read32(OC1555_MS_VALUES + 4 * ((n & 1) * 4 + i));
            c.bus.write32(out + 4 * i, w);
        }
        c.cycles += 20;
        ret(c);
    }

    fn oc1555_install() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320BF0),
            oc1555_fake_axis,
            &OC1555_PREV_AXIS,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA4C8),
            oc1555_fake_measure_row,
            &OC1555_PREV_MEASURE,
        );
    }

    /// Axis and modifier fixture: `self` at `DATA` with a binding word at +0x5c,
    /// and the forced inputs. Every fourth seed gives 0.0 (the modifier is not
    /// held), the others edge floats (zeros, NaN, infinities, normals).
    fn oc1555_axis_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1555_a0a0 ^ seed as u64);
        scramble(c, &mut r);
        oc1555_install();
        c.bus.write32(OC1555_AX_CTRL, OC1555_AX_MAGIC);
        for i in 0..16 {
            let v = if seed % 4 == 0 { 0 } else { r.edge_f32() };
            c.bus.write32(OC1555_AX_VALUES + 4 * i, v);
        }
        c.g[A0][0] = sx(DATA);
        c.bus.write32(DATA + 0x5c, r.next_u32());
    }

    /// Item constructor fixture: `self` at `OC1555_SELF`, random argument words.
    fn oc1555_ctor_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1555_c500 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(OC1555_SELF);
        c.g[A1][0] = sx(r.next_u32());
        c.g[A2][0] = sx(r.next_u32());
        c.g[A3][0] = sx(r.next_u32());
        c.g[T0][0] = sx(r.next_u32());
        c.g[T1][0] = sx(r.next_u32());
    }

    /// Measure fixture: the item's `+0x14` (a2), `+0x18` (zero takes the
    /// single-row path) and `+0x1c` (zero takes the single row too, a non-zero
    /// one lays out two rows). Seeds cycle through the paths, and the row table
    /// gives both compare outcomes.
    fn oc1555_measure_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1555_c648 ^ seed as u64);
        scramble(c, &mut r);
        oc1555_install();
        c.bus.write32(OC1555_MS_CTRL, OC1555_MS_MAGIC);
        c.bus.write32(OC1555_MS_CALLS, 0);
        for i in 0..8 {
            c.bus.write32(OC1555_MS_VALUES + 4 * i, r.edge_f32());
        }
        let text = if seed % 4 == 0 { 0 } else { r.next_u32() | 1 };
        let width = if seed % 4 == 1 { 0 } else { r.next_u32() | 1 };
        c.bus.write32(OC1555_ITEM + 0x14, r.next_u32());
        c.bus.write32(OC1555_ITEM + 0x18, text);
        c.bus.write32(OC1555_ITEM + 0x1c, width);
        c.g[A0][0] = sx(OC1555_OUT);
        c.g[A1][0] = sx(OC1555_ITEM);
    }

    #[test]
    fn menu_sys_get_right_axis_matches() {
        check(
            ee::s09::f_002cc280__generated,
            menu_sys_get_right_axis,
            300,
            &|c, seed| oc1555_axis_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_get_up_axis_matches() {
        check(
            ee::s09::f_002cc2a0__generated,
            menu_sys_get_up_axis,
            300,
            &|c, seed| oc1555_axis_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_get_down_axis_matches() {
        check(
            ee::s09::f_002cc2c0__generated,
            menu_sys_get_down_axis,
            300,
            &|c, seed| oc1555_axis_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_is_slow_mod_held_matches() {
        check(
            ee::s09::f_002cc2e0__generated,
            menu_sys_is_slow_mod_held,
            300,
            &|c, seed| oc1555_axis_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_is_fine_mod_held_matches() {
        check(
            ee::s09::f_002cc318__generated,
            menu_sys_is_fine_mod_held,
            300,
            &|c, seed| oc1555_axis_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_bool_item_ctor_matches() {
        check(
            ee::s09::f_002cc500__generated,
            menu_sys_bool_item_ctor,
            200,
            &|c, seed| oc1555_ctor_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_bool_item_measure_matches() {
        check(
            ee::s09::f_002cc648__generated,
            menu_sys_bool_item_measure,
            300,
            &|c, seed| oc1555_measure_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_enum_item_ctor_matches() {
        check(
            ee::s09::f_002cc7b0__generated,
            menu_sys_enum_item_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x1555_c7b0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OC1555_SELF);
            },
        );
    }
}
