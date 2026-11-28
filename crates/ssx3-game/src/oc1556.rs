//! Staging module oc1556 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1556`.

#![allow(unused_imports)]

use crate::abi::{arg, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Vtable of `MenuSysEnumItem` (`D_00486E30`).
const ENUM_ITEM_VTABLE: u32 = 0x0048_6e30;
/// Vtable of `MenuSysActionItem` (`D_00486D40`).
const ACTION_ITEM_VTABLE: u32 = 0x0048_6d40;
/// Vtable of `MenuSysCallbackItem` (`D_00486CF0`).
const CALLBACK_ITEM_VTABLE: u32 = 0x0048_6cf0;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_c7f8, menu_sys_enum_item_ctor_2, "MenuSysEnumItem_ctor_2"),
    (0x002c_c838, menu_sys_enum_item_ctor_3, "MenuSysEnumItem_ctor_3"),
    (0x002c_cb18, menu_sys_enum_item_measure, "MenuSysEnumItem_Measure"),
    (0x002c_cdc8, menu_sys_submenu_item_draw, "MenuSysSubmenuItem_Draw"),
    (0x002c_cee0, menu_sys_back_item_draw, "MenuSysBackItem_Draw"),
    (0x002c_cf50, menu_sys_action_item_ctor_2, "MenuSysActionItem_ctor_2"),
    (0x002c_d128, menu_sys_callback_item_ctor, "MenuSysCallbackItem_ctor"),
    (0x002c_d168, menu_sys_callback_item_ctor_2, "MenuSysCallbackItem_ctor_2"),
];

/// `cEnumMenuItem_cEnumMenuItem1` (0x2cc7f8), `MenuSysEnumItem_ctor_2(self, text)`:
/// the base item constructor, then the enum vtable at +0x10 and zeroed
/// fields at +0x14, +0x18 and +0x20.
pub fn menu_sys_enum_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_c80c)?;
        let item = Addr(c.g[S0][0] as u32);
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(ENUM_ITEM_VTABLE);
        item.off(0x14).sw(c, 0);
        item.off(0x10).sw(c, c.g[V1][0]);
        item.off(0x20).sw(c, 0);
        item.off(0x18).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cEnumMenuItem_cEnumMenuItem2` (0x2cc838), `MenuSysEnumItem_ctor_3(self, text,
/// a2, a3, a4, a5)`: the base constructor, then the enum vtable and the five
/// caller-given fields (+0x14 = a3, +0x18 = a4, +0x1c = a5, +0x20 = a2).
pub fn menu_sys_enum_item_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[T1][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_c86c)?;
        let item = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = sx(ENUM_ITEM_VTABLE);
        item.off(0x14).sw(c, c.g[S1][0]);
        item.off(0x20).sw(c, c.g[S2][0]);
        item.off(0x18).sw(c, c.g[S3][0]);
        c.g[V0][0] = c.g[S0][0];
        item.off(0x1c).sw(c, c.g[S4][0]);
        item.off(0x10).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Copy the 16 bytes at sp+0x10 to sp+0 through the registers `a` and `b`,
/// with the unaligned `ldl`/`ldr`/`sdl`/`sdr` pairs the generated code uses.
fn copy_block_through(c: &mut Cpu, a: usize, b: usize) {
    let s = sp(c);
    c.ldl(a, s.wrapping_add(0x17));
    c.ldr(a, s.wrapping_add(0x10));
    c.ldl(b, s.wrapping_add(0x1f));
    c.ldr(b, s.wrapping_add(0x18));
    c.sdl(a, s.wrapping_add(0x7));
    c.sdr(a, s);
    c.sdl(b, s.wrapping_add(0xf));
    c.sdr(b, s.wrapping_add(0x8));
}

/// `cEnumMenuItem_preRender` (0x2ccb18), `MenuSysEnumItem_Measure(out, item)`:
/// measures the item's rows with `MenuSysItem_MeasureRow` (row count at +0x18,
/// row table at +0x1c, 8 bytes per row, row text argument at +0x14), keeps
/// the 16-byte block of the measure in a stack buffer and copies it to `out`.
/// Returns `out`. The back edge of the row loop checks for due events.
pub fn menu_sys_enum_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x70);
        // The style pointer the row measure takes as its fifth argument.
        c.g[T0][0] = sx(0x0048_66b8);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[A3][0] = 0;
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let buf = frame.at(c, 0x10);
        c.g[A0][0] = sx(buf);
        c.g[S3][0] = c.g[A0][0];
        c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x14).lw(c);
        c.g[S0][0] = 0;
        Callee {
            addr: 0x002c_a4c8,
            f: ee::s09::MenuSysItem_MeasureRow,
        }
        .call(c, 0x002c_cb58)?;
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x18).lw(c);
        copy_block_through(c, 3, 4);
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x1c).lw(c);
            loop {
                c.cycles += 14;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 3);
                c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x14).lw(c);
                c.g[A0][0] = c.g[S3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A3][0] = 0;
                Callee {
                    addr: 0x002c_a4c8,
                    f: ee::s09::MenuSysItem_MeasureRow,
                }
                .call(c, 0x002c_cba8)?;
                let f1 = frame.at(c, 0x18);
                c.lwc1(1, f1);
                let f0 = frame.at(c, 0x8);
                c.lwc1(0, f0);
                c.c_lt_s(0, 1);
                if !c.fcc() {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x18).lw(c);
                } else {
                    c.cycles += 9;
                    copy_block_through(c, 2, 3);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x18).lw(c);
                }
                c.cycles += 4;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x1c).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002c_cb88);
                }
            }
        }
        c.cycles += 16;
        let s = sp(c);
        c.ldl(6, s.wrapping_add(0x7));
        c.ldr(6, s);
        c.ldl(3, s.wrapping_add(0xf));
        c.ldr(3, s.wrapping_add(0x8));
        let out = c.g[S2][0] as u32;
        c.sdl(6, out.wrapping_add(0x7));
        c.sdr(6, out);
        c.sdl(3, out.wrapping_add(0xf));
        c.g[V0][0] = c.g[S2][0];
        c.sdr(3, out.wrapping_add(0x8));
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `MenuSysSubmenuItem_Draw(item, a1)`, 0x2ccdc8: draws the item's row with
/// `MenuSysItem_DrawRow(item, a1, 0, item.+0x14, 0)` and returns.
pub fn menu_sys_submenu_item_draw(c: &mut Cpu) {
    draw_row_and_return(c, 0x002c_cde0);
}

/// `MenuSysBackItem_Draw(item, a1)`, 0x2ccee0: the same row draw as the
/// submenu item (`MenuSysItem_DrawRow` with the item's +0x14 field).
pub fn menu_sys_back_item_draw(c: &mut Cpu) {
    draw_row_and_return(c, 0x002c_cef8);
}

/// The shared body of the two row draws: one call to `MenuSysItem_DrawRow`
/// returning to `ret_to`, with a2 = 0, t0 = 0 and a3 = the item's +0x14 word.
fn draw_row_and_return(c: &mut Cpu, ret_to: u32) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0;
        c.g[A3][0] = Addr(arg(c, 0)).off(0x14).lw(c);
        Callee {
            addr: 0x002c_a988,
            f: ee::s09::MenuSysItem_DrawRow,
        }
        .call(c, ret_to)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cActionMenuItem_cActionMenuItem2` (0x2ccf50), `MenuSysActionItem_ctor_2(self)`:
/// the base constructor, then the action vtable at +0x10 and zeroed fields.
pub fn menu_sys_action_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_cf64)?;
        let item = Addr(c.g[S0][0] as u32);
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(ACTION_ITEM_VTABLE);
        item.off(0x14).sw(c, 0);
        item.off(0x10).sw(c, c.g[V1][0]);
        item.off(0x1c).sw(c, 0);
        item.off(0x20).sw(c, 0);
        item.off(0x18).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `MenuSysCallbackItem_ctor(self)`, 0x2cd128: the base constructor with a
/// data argument of -1, then the callback vtable at +0x10 and zeroed fields.
pub fn menu_sys_callback_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_d140)?;
        let item = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = sx(CALLBACK_ITEM_VTABLE);
        c.g[V0][0] = c.g[S0][0];
        item.off(0x14).sw(c, 0);
        item.off(0x10).sw(c, c.g[V1][0]);
        item.off(0x18).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `MenuSysCallbackItem_ctor_2(self, a1, a2, a3)`, 0x2cd168: the base
/// constructor, then the callback vtable and the caller's a2 (+0x14) and a3
/// (+0x18) fields.
pub fn menu_sys_callback_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A3][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_d18c)?;
        let item = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = sx(CALLBACK_ITEM_VTABLE);
        item.off(0x14).sw(c, c.g[S1][0]);
        item.off(0x18).sw(c, c.g[S2][0]);
        item.off(0x10).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, scramble, Rng, DATA, SP as TEST_SP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (a unique cell of the machine's
    /// RAM): the fakes answer only when a fixture sets it.
    const OC1556_GATE: u32 = DATA + 0x20_1556;
    const OC1556_MAGIC: u32 = 0x1556_f001;
    /// Four words the row-measure fake copies to its output buffer.
    const OC1556_MEASURE: u32 = DATA + 0x20_1600;

    /// A 64-bit value with random halves (sign and upper bits must match too).
    fn wide(r: &mut Rng) -> u64 {
        ((r.next_u32() as u64) << 32) | r.next_u32() as u64
    }

    /// `MenuSysItem_DrawRow` with this module's gate on: logs the call.
    fn oc1556_draw_row_fake(c: &mut Cpu) {
        if c.bus.read32(OC1556_GATE) != OC1556_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1556_draw_row_fake),
                ee::s09::f_002ca988__generated,
            );
        }
        fakes::fake_call(c, 0x002c_a988);
    }

    /// `MenuSysItem_MeasureRow` with this module's gate on: copies the four
    /// mailbox words to the output buffer (a0) and logs the call.
    fn oc1556_measure_row_fake(c: &mut Cpu) {
        if c.bus.read32(OC1556_GATE) != OC1556_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1556_measure_row_fake),
                ee::s09::f_002ca4c8__generated,
            );
        }
        let out = c.g[A0][0] as u32;
        for k in 0..4u32 {
            let w = c.bus.read32(OC1556_MEASURE + 4 * k);
            c.bus.write32(out + 4 * k, w);
        }
        fakes::fake_call(c, 0x002c_a4c8);
    }

    /// Hook this module's fakes into the callee slots (once per process).
    fn oc1556_install() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA988),
            oc1556_draw_row_fake,
            fakes::prev_cell(oc1556_draw_row_fake),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA4C8),
            oc1556_measure_row_fake,
            fakes::prev_cell(oc1556_measure_row_fake),
        );
    }

    /// `fakes::machine` with this module's gate switched on.
    fn oc1556_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC1556_GATE, OC1556_MAGIC);
        r
    }

    /// Arguments of a row draw: an item with a random +0x14 word.
    fn oc1556_draw_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = oc1556_machine(c, seed, salt, 2);
        c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | DATA as u64;
        c.g[A1][0] = wide(&mut r);
        c.bus.write32(DATA + 0x14, r.next_u32());
    }

    #[test]
    fn menu_sys_enum_item_ctor_2_matches() {
        check(
            ee::s09::f_002cc7f8__generated,
            menu_sys_enum_item_ctor_2,
            120,
            &|c, seed| {
                let mut r = Rng(0x2cc7f8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | DATA as u64;
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_enum_item_ctor_3_matches() {
        check(
            ee::s09::f_002cc838__generated,
            menu_sys_enum_item_ctor_3,
            120,
            &|c, seed| {
                let mut r = Rng(0x2cc838 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | DATA as u64;
                c.g[A1][0] = wide(&mut r);
                c.g[A2][0] = wide(&mut r);
                c.g[A3][0] = wide(&mut r);
                c.g[T0][0] = wide(&mut r);
                c.g[T1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_enum_item_measure_matches() {
        oc1556_install();
        check(
            ee::s09::f_002ccb18__generated,
            menu_sys_enum_item_measure,
            200,
            &|c, seed| {
                let mut r = oc1556_machine(c, seed, 0x2ccb18, 3);
                let (out, item, rows) = (DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
                // The four words the row measure leaves in its buffer.
                for k in 0..4u32 {
                    c.bus.write32(OC1556_MEASURE + 4 * k, r.f32(10.0));
                }
                // The frame word at sp+8 of the function: not written by the
                // row measure, so the float compare reads what is here.
                c.bus.write32(TEST_SP - 0x68, r.f32(10.0));
                // Zero, one, two or three rows.
                c.bus.write32(item + 0x14, r.next_u32());
                c.bus.write32(item + 0x18, seed % 4);
                c.bus.write32(item + 0x1c, rows);
                for i in 0..4u32 {
                    c.bus.write32(rows + 8 * i, r.next_u32());
                }
                c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | out as u64;
                c.g[A1][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | item as u64;
            },
        );
    }

    #[test]
    fn menu_sys_submenu_item_draw_matches() {
        oc1556_install();
        check(
            ee::s09::f_002ccdc8__generated,
            menu_sys_submenu_item_draw,
            120,
            &|c, seed| oc1556_draw_setup(c, seed, 0x2ccdc8),
        );
    }

    #[test]
    fn menu_sys_back_item_draw_matches() {
        oc1556_install();
        check(
            ee::s09::f_002ccee0__generated,
            menu_sys_back_item_draw,
            120,
            &|c, seed| oc1556_draw_setup(c, seed, 0x2ccee0),
        );
    }

    #[test]
    fn menu_sys_action_item_ctor_2_matches() {
        check(
            ee::s09::f_002ccf50__generated,
            menu_sys_action_item_ctor_2,
            120,
            &|c, seed| {
                let mut r = Rng(0x2ccf50 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | DATA as u64;
            },
        );
    }

    #[test]
    fn menu_sys_callback_item_ctor_matches() {
        check(
            ee::s09::f_002cd128__generated,
            menu_sys_callback_item_ctor,
            120,
            &|c, seed| {
                let mut r = Rng(0x2cd128 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | DATA as u64;
            },
        );
    }

    #[test]
    fn menu_sys_callback_item_ctor_2_matches() {
        check(
            ee::s09::f_002cd168__generated,
            menu_sys_callback_item_ctor_2,
            120,
            &|c, seed| {
                let mut r = Rng(0x2cd168 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | DATA as u64;
                c.g[A2][0] = wide(&mut r);
                c.g[A3][0] = wide(&mut r);
            },
        );
    }
}
