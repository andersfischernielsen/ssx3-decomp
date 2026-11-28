//! Staging module oc1572 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1572`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_3bb0, menu_sys_label_item_dtor, "MenuSysLabelItem_dtor"),
    (0x002d_3be0, menu_sys_spacer_item_dtor, "MenuSysSpacerItem_dtor"),
    (0x002d_3c00, menu_sys_int8_item_dtor, "MenuSysInt8Item_dtor"),
    (0x002d_3c30, menu_sys_int_item_dtor, "MenuSysIntItem_dtor"),
    (0x002d_3c60, menu_sys_float_item_dtor, "MenuSysFloatItem_dtor"),
    (0x002d_3ce0, menu_sys_float_range_item_dtor, "MenuSysFloatRangeItem_dtor"),
    (0x002d_3d20, menu_sys_text_edit_item_dtor, "MenuSysTextEditItem_dtor"),
    (0x002d_3d48, menu_sys_text_edit_item_set_text, "MenuSysTextEditItem_SetText"),
];

/// `MenuSysItem_dtor` (0x2ca280): the base item destructor that the eight
/// item destructors below forward to.
pub(crate) const OC1572_MENU_SYS_ITEM_DTOR: Callee = Callee {
    addr: 0x002c_a280,
    f: ee::s09::f_002ca280,
};

/// `MenuSysLabelItem_dtor` (0x2d3bb0): a forwarding wrapper. `a0` and `a1`
/// pass through unchanged to `MenuSysItem_dtor` (7 cycles, a 0x10 frame
/// holding `ra`).
pub fn menu_sys_label_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(OC1572_MENU_SYS_ITEM_DTOR), 0x002d_3bc0)
}

/// `MenuSysSpacerItem_dtor` (0x2d3be0): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_spacer_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(OC1572_MENU_SYS_ITEM_DTOR), 0x002d_3bf0)
}

/// `MenuSysInt8Item_dtor` (0x2d3c00): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_int8_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(OC1572_MENU_SYS_ITEM_DTOR), 0x002d_3c10)
}

/// `MenuSysIntItem_dtor` (0x2d3c30): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_int_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(OC1572_MENU_SYS_ITEM_DTOR), 0x002d_3c40)
}

/// `MenuSysFloatItem_dtor` (0x2d3c60): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_float_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(OC1572_MENU_SYS_ITEM_DTOR), 0x002d_3c70)
}

/// `MenuSysFloatRangeItem_dtor` (0x2d3ce0): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_float_range_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(OC1572_MENU_SYS_ITEM_DTOR), 0x002d_3cf0)
}

/// `MenuSysTextEditItem_dtor` (0x2d3d20): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_text_edit_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(OC1572_MENU_SYS_ITEM_DTOR), 0x002d_3d30)
}

/// `MenuSysTextEditItem_SetText(MenuSysTextEditItem* item, const char* text)`
/// (0x2d3d48): clear the first byte of the item's text buffer at `+0x18`, then
/// `strncat(buffer, text, 15)`. Returns what `strncat` returned.
pub fn menu_sys_text_edit_item_set_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let buffer = (c.g[V0][0] as u32).wrapping_add(0x18);
        c.g[A0][0] = sx(buffer);
        c.g[A2][0] = 0xf;
        Addr(buffer).sb(c, 0);
        call_ptr(c, 0x0041_6958, 0x002d_3d64)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};
    use crate::game::visualfx::worldlightman::tests::dtor_case;

    #[test]
    fn menu_sys_label_item_dtor_matches() {
        dtor_case(ee::s09::f_002d3bb0__generated, menu_sys_label_item_dtor, 0x002d_3bb0);
    }

    #[test]
    fn menu_sys_spacer_item_dtor_matches() {
        dtor_case(ee::s09::f_002d3be0__generated, menu_sys_spacer_item_dtor, 0x002d_3be0);
    }

    #[test]
    fn menu_sys_int8_item_dtor_matches() {
        dtor_case(ee::s09::f_002d3c00__generated, menu_sys_int8_item_dtor, 0x002d_3c00);
    }

    #[test]
    fn menu_sys_int_item_dtor_matches() {
        dtor_case(ee::s09::f_002d3c30__generated, menu_sys_int_item_dtor, 0x002d_3c30);
    }

    #[test]
    fn menu_sys_float_item_dtor_matches() {
        dtor_case(ee::s09::f_002d3c60__generated, menu_sys_float_item_dtor, 0x002d_3c60);
    }

    #[test]
    fn menu_sys_float_range_item_dtor_matches() {
        dtor_case(
            ee::s09::f_002d3ce0__generated,
            menu_sys_float_range_item_dtor,
            0x002d_3ce0,
        );
    }

    #[test]
    fn menu_sys_text_edit_item_dtor_matches() {
        dtor_case(
            ee::s09::f_002d3d20__generated,
            menu_sys_text_edit_item_dtor,
            0x002d_3d20,
        );
    }

    /// `SetText` with a random buffer (the first byte is cleared by the
    /// function) and a source string holding a NUL at a seed-chosen place, so
    /// `strncat` both stops early and copies the full 15 bytes.
    #[test]
    fn menu_sys_text_edit_item_set_text_matches() {
        check(
            ee::s09::f_002d3d48__generated,
            menu_sys_text_edit_item_set_text,
            200,
            &|c, seed| {
                let mut r = Rng(0x002d_3d48 ^ seed as u64);
                scramble(c, &mut r);
                let item = DATA + 0x100;
                let text = DATA + 0x400;
                c.g[A0][0] = sx(item);
                c.g[A1][0] = sx(text);
                for i in 0..4u32 {
                    c.bus.write32(item + 0x18 + 4 * i, r.next_u32());
                }
                let nul = seed % 20;
                for i in 0..24u32 {
                    let b = if i == nul { 0 } else { (r.next_u32() as u8) | 1 };
                    c.bus.write8(text + i, b);
                }
            },
        );
    }
}
