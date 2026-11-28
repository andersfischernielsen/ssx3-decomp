//! Staging module oc1573 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1573`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_3d70, menu_sys_color_channel_item_dtor, "MenuSysColorChannelItem_dtor"),
    (0x002d_3d90, menu_sys_group_item_page_dtor, "MenuSysGroupItem_PageDtor"),
    (0x002d_3db0, menu_sys_group_item_dtor, "MenuSysGroupItem_dtor"),
    (0x002d_3df8, menu_sys_color_swatch_item_dtor, "MenuSysColorSwatchItem_dtor"),
    (0x002d_3e20, menu_sys_color_item_dtor, "MenuSysColorItem_dtor"),
    (0x002d_3ea8, menu_sys_vector_label_item_dtor, "MenuSysVectorLabelItem_dtor"),
    (0x002d_3ed0, menu_sys_vector_item_dtor, "MenuSysVectorItem_dtor"),
    (0x002d_3f50, menu_sys_point2d_item_dtor, "MenuSysPoint2DItem_dtor"),
];

/// `MenuSysItem_dtor` (0x2ca280): the base item destructor, called on each
/// sub-object and on `this`.
const MENU_ITEM_DTOR: Callee = Callee {
    addr: 0x002c_a280,
    f: ee::s09::f_002ca280,
};

/// `MenuSysPage_dtor` (0x2caa80): the page member of the group items.
const MENU_PAGE_DTOR: Callee = Callee {
    addr: 0x002c_aa80,
    f: ee::s09::f_002caa80,
};

/// `MenuSysColorChannelItem_dtor` (0x2d3d70): forwards to `MenuSysItem_dtor`
/// with the same `this` and flags, returning to 0x2d3d80.
pub fn menu_sys_color_channel_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(MENU_ITEM_DTOR), 0x002d_3d80);
}

/// `MenuSysGroupItem_PageDtor` (0x2d3d90): forwards to `MenuSysPage_dtor`
/// with the same `this` and flags, returning to 0x2d3da0.
pub fn menu_sys_group_item_page_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(MENU_PAGE_DTOR), 0x002d_3da0);
}

/// `MenuSysColorSwatchItem_dtor` (0x2d3df8): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_color_swatch_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(MENU_ITEM_DTOR), 0x002d_3e08);
}

/// `MenuSysVectorLabelItem_dtor` (0x2d3ea8): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_vector_label_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(MENU_ITEM_DTOR), 0x002d_3eb8);
}

/// `MenuSysPoint2DItem_dtor` (0x2d3f50): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_point2d_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(MENU_ITEM_DTOR), 0x002d_3f60);
}

/// The composite item destructors (`MenuSysGroupItem`, `MenuSysColorItem`,
/// `MenuSysVectorItem`): a 0x30 frame holding `this` (`s0`) and the flags
/// (`s1`), then the member destructors, each called with flags 2 (no delete)
/// on `this + off`, and finally `base_dtor(this, flags)` on the object itself.
/// `subs` lists `(offset, destructor, return address)` in call order; the
/// last call returns to `last_ret`.
fn menu_sys_composite_dtor(
    c: &mut Cpu,
    cycles: u64,
    subs: &[(u32, Callee, u32)],
    last_ret: u32,
) -> Flow {
    c.cycles += cycles;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    for &(off, dtor, ret) in subs {
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(off));
        c.g[A1][0] = 0x2;
        dtor.call(c, ret)?;
    }
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    MENU_ITEM_DTOR.call(c, last_ret)?;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `MenuSysGroupItem_dtor` (0x2d3db0): the page member at `+0x18`, then the
/// item itself.
pub fn menu_sys_group_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        menu_sys_composite_dtor(
            c,
            17,
            &[(0x18, MENU_PAGE_DTOR, 0x002d_3dd4)],
            0x002d_3de0,
        )
    });
}

/// `MenuSysColorItem_dtor` (0x2d3e20): five item members at `+0x5b0..+0x640`,
/// the page member at `+0x18`, then the item itself.
pub fn menu_sys_color_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        menu_sys_composite_dtor(
            c,
            32,
            &[
                (0x640, MENU_ITEM_DTOR, 0x002d_3e44),
                (0x61c, MENU_ITEM_DTOR, 0x002d_3e50),
                (0x5f8, MENU_ITEM_DTOR, 0x002d_3e5c),
                (0x5d4, MENU_ITEM_DTOR, 0x002d_3e68),
                (0x5b0, MENU_ITEM_DTOR, 0x002d_3e74),
                (0x18, MENU_PAGE_DTOR, 0x002d_3e80),
            ],
            0x002d_3e8c,
        )
    });
}

/// `MenuSysVectorItem_dtor` (0x2d3ed0): four item members at `+0x5b8..+0x654`,
/// the page member at `+0x18`, then the item itself.
pub fn menu_sys_vector_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        menu_sys_composite_dtor(
            c,
            29,
            &[
                (0x654, MENU_ITEM_DTOR, 0x002d_3ef4),
                (0x620, MENU_ITEM_DTOR, 0x002d_3f00),
                (0x5ec, MENU_ITEM_DTOR, 0x002d_3f0c),
                (0x5b8, MENU_ITEM_DTOR, 0x002d_3f18),
                (0x18, MENU_PAGE_DTOR, 0x002d_3f24),
            ],
            0x002d_3f30,
        )
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{machine, ptr, LOG_RESULTS};

    /// The allocator's mutex (`MEM_MUTEX`), set up so that the `operator
    /// delete` the last member may call takes its short path.
    const OC1573_MEM_MUTEX: u32 = 0x0053_8b00;

    /// The deleting-destructor fixture: a random object pointer, flags 0..3
    /// (bit 0 makes the last member free its object) and the allocator words.
    fn oc1573_dtor_case(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 200, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            c.bus.write32(OC1573_MEM_MUTEX + 4, 0x42);
            c.bus.write32(LOG_RESULTS, 0x42);
            c.bus.write32(OC1573_MEM_MUTEX + 8, r.next_u32() % 4);
            c.g[A0][0] = ptr(&mut r) as u64;
            c.g[A1][0] = (seed % 4) as u64;
        });
    }

    #[test]
    fn menu_sys_color_channel_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3d70__generated,
            menu_sys_color_channel_item_dtor,
            0x2d_3d70,
        );
    }

    #[test]
    fn menu_sys_group_item_page_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3d90__generated,
            menu_sys_group_item_page_dtor,
            0x2d_3d90,
        );
    }

    #[test]
    fn menu_sys_group_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3db0__generated,
            menu_sys_group_item_dtor,
            0x2d_3db0,
        );
    }

    #[test]
    fn menu_sys_color_swatch_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3df8__generated,
            menu_sys_color_swatch_item_dtor,
            0x2d_3df8,
        );
    }

    #[test]
    fn menu_sys_color_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3e20__generated,
            menu_sys_color_item_dtor,
            0x2d_3e20,
        );
    }

    #[test]
    fn menu_sys_vector_label_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3ea8__generated,
            menu_sys_vector_label_item_dtor,
            0x2d_3ea8,
        );
    }

    #[test]
    fn menu_sys_vector_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3ed0__generated,
            menu_sys_vector_item_dtor,
            0x2d_3ed0,
        );
    }

    #[test]
    fn menu_sys_point2d_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3f50__generated,
            menu_sys_point2d_item_dtor,
            0x2d_3f50,
        );
    }
}
