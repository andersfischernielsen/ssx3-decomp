//! Staging module oc1557 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1557`.

#![allow(unused_imports)]

use crate::abi;
use crate::call::{guest, sp, Callee, Flow, Frame};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `void MenuSysCallbackItem_Draw(MenuSysItem* item, rect)` (0x2cd278): draws
/// the item row through `MenuSysItem_DrawRow` with the callback flags 0x4866c8
/// in t0, the item's +0x14 word as a2 and a3 = 0. Arguments a0/a1 pass through.
pub fn menu_sys_callback_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[T0][0] = 0x0048_66c8;
        c.g[A3][0] = 0;
        c.g[A2][0] = Addr(abi::arg(c, 0)).off(0x14).lw(c);
        Callee {
            addr: 0x002c_a988,
            f: ee::s09::MenuSysItem_DrawRow,
        }
        .call(c, 0x002c_d294)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysLabelItem_Draw(MenuSysItem* item, rect)` (0x2cd328): draws the
/// row with a2 = 0, t0 = 0 and a3 = the item's +0x14 word.
pub fn menu_sys_label_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0;
        c.g[A3][0] = Addr(abi::arg(c, 0)).off(0x14).lw(c);
        Callee {
            addr: 0x002c_a988,
            f: ee::s09::MenuSysItem_DrawRow,
        }
        .call(c, 0x002c_d340)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of `MenuSysLabelItem_ctor` and `MenuSysSpacerItem_ctor`:
/// `MenuSysItem_ctor(item, -1)`, then the item's data word a1 at +0x14, and
/// the class vtable at +0x10. Returns to `ret_to` from the base constructor.
fn named_item_ctor(c: &mut Cpu, vtable: u32, ret_to: u32) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
    Callee {
        addr: 0x002c_a258,
        f: ee::s09::MenuSysItem_ctor,
    }
    .call(c, ret_to)?;
    let item = Addr(c.g[S0][0] as u32);
    let data = c.g[S1][0];
    item.off(0x14).sw(c, data);
    c.g[V1][0] = sx(vtable);
    c.g[V0][0] = c.g[S0][0];
    let v1 = c.g[V1][0];
    item.off(0x10).sw(c, v1);
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `MenuSysLabelItem* MenuSysLabelItem_ctor(MenuSysLabelItem* item, int data)`
/// (0x2cd2a0): a named item with the label vtable 0x486ca0.
pub fn menu_sys_label_item_ctor(c: &mut Cpu) {
    guest(c, |c| named_item_ctor(c, 0x0048_6ca0, 0x002c_d2c0));
}

/// `MenuSysSpacerItem* MenuSysSpacerItem_ctor(MenuSysSpacerItem* item, int data)`
/// (0x2cd350): the same shape as the label item, with the spacer vtable 0x486c50.
pub fn menu_sys_spacer_item_ctor(c: &mut Cpu) {
    guest(c, |c| named_item_ctor(c, 0x0048_6c50, 0x002c_d370));
}

/// The Int8 item fields after the base constructor: vtable 0x486c00 at +0x10,
/// the int word at +0x14 and +0x18 cleared, the two bytes at +0x1c/+0x1d
/// cleared, and the words at +0x20, +0x24 and +0x28 cleared.
fn int8_item_fields(c: &mut Cpu) {
    let item = Addr(c.g[S0][0] as u32);
    c.g[V1][0] = sx(0x0048_6c00);
    c.g[V0][0] = c.g[S0][0];
    item.off(0x14).sw(c, 0);
    let v1 = c.g[V1][0];
    item.off(0x10).sw(c, v1);
    item.off(0x18).sw(c, 0);
    item.off(0x1c).sb(c, 0);
    item.off(0x1d).sb(c, 0);
    item.off(0x24).sw(c, 0);
    item.off(0x20).sw(c, 0);
    item.off(0x28).sw(c, 0);
}

/// `MenuSysInt8Item* MenuSysInt8Item_ctor(MenuSysInt8Item* item, ...)`
/// (0x2cd3f8): `MenuSysItem_ctor(item, -1)` and the Int8 item fields.
pub fn menu_sys_int8_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_d410)?;
        int8_item_fields(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysInt8Item_ctor_2` (0x2cd450): the same as `MenuSysInt8Item_ctor`,
/// but the base constructor keeps the caller's a1.
pub fn menu_sys_int8_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_d464)?;
        int8_item_fields(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysInt8Item_ctor_3(item, a1, a2, a3, t0, t1)` (0x2cd4a0): the base
/// constructor, then the vtable 0x486c00, the int word a2 at +0x18, the data
/// word t1 at +0x14, the bytes a3 and t0 (sign-extended from their low byte)
/// at +0x1c and +0x1d, and the words at +0x20, +0x24 and +0x28 cleared.
pub fn menu_sys_int8_item_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = sx((c.g[A3][0] as u32) << 24);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = sx((c.g[T0][0] as u32) << 24);
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[T1][0];
        c.g[S4][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
        c.g[S2][0] = sx(((c.g[S2][0] as u32) as i32 >> 24) as u32);
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_d4dc)?;
        let item = Addr(c.g[S0][0] as u32);
        let (s1, s2, s3, s4) = (c.g[S1][0], c.g[S2][0], c.g[S3][0], c.g[S4][0]);
        c.g[V1][0] = sx(0x0048_6c00);
        item.off(0x14).sw(c, s3);
        item.off(0x18).sw(c, s4);
        item.off(0x1c).sb(c, s1);
        c.g[V0][0] = c.g[S0][0];
        item.off(0x1d).sb(c, s2);
        let v1 = c.g[V1][0];
        item.off(0x10).sw(c, v1);
        item.off(0x24).sw(c, 0);
        item.off(0x20).sw(c, 0);
        item.off(0x28).sw(c, 0);
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

/// `MenuSysIntItem* MenuSysIntItem_ctor(MenuSysIntItem* item, ...)`
/// (0x2cd850): `MenuSysItem_ctor(item, -1)`, then the vtable 0x486bb0 and the
/// cleared int words at +0x14, +0x18, +0x1c, +0x20, +0x24 and +0x28, +0x2c.
pub fn menu_sys_int_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_d868)?;
        let item = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = sx(0x0048_6bb0);
        c.g[V0][0] = c.g[S0][0];
        item.off(0x14).sw(c, 0);
        let v1 = c.g[V1][0];
        item.off(0x10).sw(c, v1);
        item.off(0x18).sw(c, 0);
        item.off(0x1c).sw(c, 0);
        item.off(0x20).sw(c, 0);
        item.off(0x28).sw(c, 0);
        item.off(0x24).sw(c, 0);
        item.off(0x2c).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_d278, menu_sys_callback_item_draw, "MenuSysCallbackItem_Draw"),
    (0x002c_d2a0, menu_sys_label_item_ctor, "MenuSysLabelItem_ctor"),
    (0x002c_d328, menu_sys_label_item_draw, "MenuSysLabelItem_Draw"),
    (0x002c_d350, menu_sys_spacer_item_ctor, "MenuSysSpacerItem_ctor"),
    (0x002c_d3f8, menu_sys_int8_item_ctor, "MenuSysInt8Item_ctor"),
    (0x002c_d450, menu_sys_int8_item_ctor_2, "MenuSysInt8Item_ctor_2"),
    (0x002c_d4a0, menu_sys_int8_item_ctor_3, "MenuSysInt8Item_ctor_3"),
    (0x002c_d850, menu_sys_int_item_ctor, "MenuSysIntItem_ctor"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's `MenuSysItem_DrawRow` fake (a unique cell of
    /// the machine's RAM): the fake answers only when a fixture sets it.
    const OC1557_GATE: u32 = DATA + 0x35_0000;
    const OC1557_MAGIC: u32 = 0x1557_f001;

    /// Gated fake for the draw-row callee: logs the call with the gate on,
    /// otherwise runs what the slot held before (or the generated body).
    fn oc1557_fake_draw_row(c: &mut Cpu) {
        if c.bus.read32(OC1557_GATE) != OC1557_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1557_fake_draw_row),
                ee::s09::f_002ca988__generated,
            );
        }
        fakes::fake_call(c, 0x002c_a988);
    }

    /// Hook the draw-row fake into its callee's slot (once per process).
    fn oc1557_install() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA988),
            oc1557_fake_draw_row,
            fakes::prev_cell(oc1557_fake_draw_row),
        );
    }

    /// `fakes::machine` with this module's draw-row gate switched on.
    fn oc1557_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC1557_GATE, OC1557_MAGIC);
        r
    }

    /// A random 64-bit register value.
    fn wide(r: &mut Rng) -> u64 {
        ((r.next_u32() as u64) << 32) | r.next_u32() as u64
    }

    #[test]
    fn menu_sys_callback_item_draw_matches() {
        oc1557_install();
        fakes::covered(
            ee::s09::f_002cd278__generated,
            menu_sys_callback_item_draw,
            96,
            &|c, seed| {
                let mut r = oc1557_machine(c, seed, 0x002c_d278, 1);
                let item = DATA + 0x1000;
                fill(c, &mut r, item, 0x10);
                c.g[A0][0] = sx(item);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_label_item_draw_matches() {
        oc1557_install();
        fakes::covered(
            ee::s09::f_002cd328__generated,
            menu_sys_label_item_draw,
            96,
            &|c, seed| {
                let mut r = oc1557_machine(c, seed, 0x002c_d328, 1);
                let item = DATA + 0x1000;
                fill(c, &mut r, item, 0x10);
                c.g[A0][0] = sx(item);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_label_item_ctor_matches() {
        check(
            ee::s09::f_002cd2a0__generated,
            menu_sys_label_item_ctor,
            60,
            &|c, seed| {
                let mut r = Rng(0x2cd2_a000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x1000, 0x10);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_spacer_item_ctor_matches() {
        check(
            ee::s09::f_002cd350__generated,
            menu_sys_spacer_item_ctor,
            60,
            &|c, seed| {
                let mut r = Rng(0x2cd3_5000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x1000, 0x10);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_ctor_matches() {
        check(
            ee::s09::f_002cd3f8__generated,
            menu_sys_int8_item_ctor,
            60,
            &|c, seed| {
                let mut r = Rng(0x2cd3_f800 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x1000, 0x10);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_ctor_2_matches() {
        check(
            ee::s09::f_002cd450__generated,
            menu_sys_int8_item_ctor_2,
            60,
            &|c, seed| {
                let mut r = Rng(0x2cd4_5000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x1000, 0x10);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_ctor_3_matches() {
        check(
            ee::s09::f_002cd4a0__generated,
            menu_sys_int8_item_ctor_3,
            80,
            &|c, seed| {
                let mut r = Rng(0x2cd4_a000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x1000, 0x10);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = wide(&mut r);
                c.g[A2][0] = wide(&mut r);
                c.g[A3][0] = wide(&mut r);
                c.g[T0][0] = wide(&mut r);
                c.g[T1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_int_item_ctor_matches() {
        check(
            ee::s09::f_002cd850__generated,
            menu_sys_int_item_ctor,
            60,
            &|c, seed| {
                let mut r = Rng(0x2cd8_5000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x1000, 0x12);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }
}
