//! Staging module oc1085 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1085`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::object::railmodifier::reset_vtable_and_delete;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_4da8, ui_sound_base_dtor, "UISound_BaseDtor"),
    (0x003a_4df0, uiicon_ctor_2, "UIIcon_ctor_2"),
    (0x003a_4e38, uiicon_ctor, "UIIcon_ctor"),
    (0x003a_4fb8, c_ui_group_set_offset, "cUIGroup_SetOffset"),
    (0x003a_4fe0, c_ui_group_ctor_2, "cUIGroup_ctor_2"),
    (0x003a_5028, c_ui_group_ctor, "cUIGroup_ctor"),
    (0x003a_5100, c_ui_group_first_child, "cUIGroup_FirstChild"),
    (0x003a_5120, c_ui_group_last_child, "cUIGroup_LastChild"),
];

/// `UIObject_ctor` (0x39fb30): the base constructor of every UI object.
pub(crate) const UIOBJ_BASE_CTOR: Callee = Callee {
    addr: 0x0039_fb30,
    f: ee::s13::f_0039fb30,
};

/// `cList_cList` (0x3977e8): the group's child list, at object +0x74.
pub(crate) const UILIST_BASE_CTOR: Callee = Callee {
    addr: 0x0039_77e8,
    f: ee::s13::f_003977e8,
};

/// `UIList_First` (0x397840) on the group's child list.
pub(crate) const UILIST_FIRST_CALL: Callee = Callee {
    addr: 0x0039_7840,
    f: ee::s13::f_00397840,
};

/// `UIList_Last` (0x397858) on the group's child list.
pub(crate) const UILIST_LAST_CALL: Callee = Callee {
    addr: 0x0039_7858,
    f: ee::s13::f_00397858,
};

/// `UISound_BaseDtor(self, flags)` (0x3a4da8): publish the sound base
/// vtable (`0x46dd60`) in word 0 and free the object when bit 0 of `flags`
/// is set.
pub fn ui_sound_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x003a_4dcc, 0x0046_dd60, 0)
    });
}

/// `UIIcon* UIIcon_ctor_2(UIIcon* self)` (0x3a4df0): `UIObject_ctor(self, 0,
/// 0)`, then the icon vtable (`0x494aa8`) at +8 and the `0x21ccc8` setup of
/// the sub-object at +0x74. Returns `self`.
pub fn uiicon_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UIOBJ_BASE_CTOR.call(c, 0x003a_4e0c)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(this.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4aa8));
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vtable);
        call_ptr(c, 0x0021_ccc8, 0x003a_4e20)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `UIIcon* UIIcon_ctor(UIIcon* self)` (0x3a4e38): as [`uiicon_ctor_2`]
/// (the caller's a1/a2 reach `UIObject_ctor` unchanged), then the
/// `-1`/`0` pair at +0x78/+0x7c. Returns `self`.
pub fn uiicon_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UIOBJ_BASE_CTOR.call(c, 0x003a_4e4c)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(this.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4aa8));
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vtable);
        call_ptr(c, 0x0021_ccc8, 0x003a_4e60)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = c.g[S0][0];
        let minus_one = c.g[V1][0];
        Addr(this.wrapping_add(0x78)).sw(c, minus_one);
        Addr(this.wrapping_add(0x7c)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIGroup_SetOffset(cUIGroup* self, float x, float y)` (0x3a4fb8):
/// the two float arguments (f12, f13) go through the stack and are copied
/// as one unaligned 8-byte word to the group's offset at +0x80.
pub fn c_ui_group_set_offset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_fpr(c, 12, 0);
        frame.save_fpr(c, 13, 4);
        let slot = sp(c);
        c.ldl(2, slot.wrapping_add(7));
        c.ldr(2, slot);
        let group = c.g[A0][0] as u32;
        c.sdl(2, group.wrapping_add(0x87));
        c.sdr(2, group.wrapping_add(0x80));
        frame.close(c);
        Ok(())
    })
}

/// `cUIGroup* cUIGroup_ctor_2(cUIGroup* self)` (0x3a4fe0): `UIObject_ctor`
/// with a1 = a2 = 0, then the group vtable (`0x4949e8`) at +8 and
/// `cList_cList` on the child list at +0x74. Returns `self`.
pub fn c_ui_group_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UIOBJ_BASE_CTOR.call(c, 0x003a_4ffc)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(this.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x49e8));
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vtable);
        UILIST_BASE_CTOR.call(c, 0x003a_5010)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cUIGroup* cUIGroup_ctor(cUIGroup* self)` (0x3a5028): as
/// [`c_ui_group_ctor_2`] with the caller's a1/a2 passed on to
/// `UIObject_ctor`. Returns `self`.
pub fn c_ui_group_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UIOBJ_BASE_CTOR.call(c, 0x003a_503c)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(this.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x49e8));
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vtable);
        UILIST_BASE_CTOR.call(c, 0x003a_5050)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `UIObject* cUIGroup_FirstChild(cUIGroup* self)` (0x3a5100):
/// `UIList_First(&self->children)`, the child list at +0x74.
pub fn c_ui_group_first_child(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        UILIST_FIRST_CALL.call(c, 0x003a_5110)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIObject* cUIGroup_LastChild(cUIGroup* self)` (0x3a5120):
/// `UIList_Last(&self->children)`, the child list at +0x74.
pub fn c_ui_group_last_child(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        UILIST_LAST_CALL.call(c, 0x003a_5130)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::game::object::railmodifier::tests::destructor_setup;

    /// Object for the constructor tests: random words at `DATA + 0x400`
    /// (the group or icon), random a1/a2 for the pass-through callers.
    pub(crate) fn oc1085_ctor_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        fill(c, &mut r, DATA + 0x400, 0x40);
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = sx(r.next_u32());
        c.g[A2][0] = sx(r.next_u32());
    }

    /// Group whose child list (object +0x74) has a head and a tail node at
    /// `DATA + 0x800`; the node's link word is the zero/non-zero switch for
    /// `UIList_First` and `UIList_Last`.
    pub(crate) fn oc1085_list_fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        fill(c, &mut r, DATA + 0x400, 0x40);
        c.g[A0][0] = sx(DATA + 0x400);
        r
    }

    #[test]
    fn ui_sound_base_dtor_matches() {
        check(
            ee::s13::f_003a4da8__generated,
            ui_sound_base_dtor,
            200,
            &|c, seed| destructor_setup(c, seed, 0x003a_4da8),
        );
    }

    #[test]
    fn uiicon_ctor_2_matches() {
        check(
            ee::s13::f_003a4df0__generated,
            uiicon_ctor_2,
            200,
            &|c, seed| oc1085_ctor_fixture(c, seed, 0x003a_4df0),
        );
    }

    #[test]
    fn uiicon_ctor_matches() {
        check(
            ee::s13::f_003a4e38__generated,
            uiicon_ctor,
            200,
            &|c, seed| oc1085_ctor_fixture(c, seed, 0x003a_4e38),
        );
    }

    #[test]
    fn c_ui_group_set_offset_matches() {
        check(
            ee::s13::f_003a4fb8__generated,
            c_ui_group_set_offset,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003a_4fb8, 0);
                fill(c, &mut r, DATA + 0x400, 0x40);
                c.g[A0][0] = sx(DATA + 0x400);
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
            },
        );
    }

    #[test]
    fn c_ui_group_ctor_2_matches() {
        check(
            ee::s13::f_003a4fe0__generated,
            c_ui_group_ctor_2,
            200,
            &|c, seed| oc1085_ctor_fixture(c, seed, 0x003a_4fe0),
        );
    }

    #[test]
    fn c_ui_group_ctor_matches() {
        check(
            ee::s13::f_003a5028__generated,
            c_ui_group_ctor,
            200,
            &|c, seed| oc1085_ctor_fixture(c, seed, 0x003a_5028),
        );
    }

    #[test]
    fn c_ui_group_first_child_matches() {
        check(
            ee::s13::f_003a5100__generated,
            c_ui_group_first_child,
            200,
            &|c, seed| {
                let mut r = oc1085_list_fixture(c, seed, 0x003a_5100);
                let node = DATA + 0x800;
                let head_link = if seed % 2 == 0 { node } else { r.next_u32() };
                c.bus.write32(node + 0x4, head_link);
                c.bus.write32(DATA + 0x400 + 0x74 + 0x4, node);
            },
        );
    }

    #[test]
    fn c_ui_group_last_child_matches() {
        check(
            ee::s13::f_003a5120__generated,
            c_ui_group_last_child,
            200,
            &|c, seed| {
                let mut r = oc1085_list_fixture(c, seed, 0x003a_5120);
                let node = DATA + 0x800;
                let tail_link = if seed % 2 == 0 { node } else { r.next_u32() };
                c.bus.write32(node, tail_link);
                c.bus.write32(DATA + 0x400 + 0x74 + 0xc, node);
            },
        );
    }
}
