//! Staging module oc1087 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1087`.

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
    (0x003a_55d8, ui_list_box_ctor_2, "UIListBox_ctor_2"),
    (0x003a_57b8, ui_state_ctor_3, "UIState_ctor_3"),
    (0x003a_5b98, ui_screen_ctor, "UIScreen_ctor"),
    (0x003a_5c08, ui_screen_ctor_2, "UIScreen_ctor_2"),
    (
        0x003a_5cf0,
        ui_screen_add_root_child,
        "UIScreen_AddRootChild",
    ),
    (0x003a_5d60, ui_text_ctor_2, "UIText_ctor_2"),
    (0x003a_5f60, cui_scroll_ctor_2, "cUIScroll_ctor_2"),
    (0x003a_5fa0, cui_scroll_ctor, "cUIScroll_ctor"),
];

/// Callees of the UI constructors, as the generated bodies call them.
const CT_OBJECT: Callee = Callee {
    addr: 0x0039_fb30,
    f: ee::s13::UIObject_ctor,
};
const CT_LIST: Callee = Callee {
    addr: 0x0039_77e8,
    f: ee::s13::UIList_ctor,
};
const CT_LIST_PUSH_BACK: Callee = Callee {
    addr: 0x0039_7910,
    f: ee::s13::UIList_PushBack,
};
const CT_LISTBOX_FLAGS: Callee = Callee {
    addr: 0x003a_65a0,
    f: ee::s13::UIListBoxFlags_ctor,
};
const CT_LISTBOX_SCROLL: Callee = Callee {
    addr: 0x003a_65b8,
    f: ee::s13::UIListBoxScroll_ctor,
};
const CT_STATE_FLAGS: Callee = Callee {
    addr: 0x003a_65d8,
    f: ee::s13::UIStateFlags_ctor,
};
const CT_TEXT_FLAGS: Callee = Callee {
    addr: 0x003a_65e8,
    f: ee::s13::UITextFlags_ctor,
};
/// `UIScreenFlags_ctor`, called through the function table (`jalr`).
const CT_SCREEN_FLAGS: u32 = 0x001d_d580;

/// Vtables the constructors store at +8 (or +0x48 for the list links), as
/// `0x490000 + the immediate` in the generated bodies.
const CT_VT_LISTBOX: u32 = 0x0049_4798;
const CT_VT_STATE: u32 = 0x0049_46c8;
const CT_VT_SCREEN: u32 = 0x0049_4670;
const CT_VT_TEXT: u32 = 0x0049_4588;
const CT_VT_SCROLL: u32 = 0x0049_44c8;
/// The vtable a screen's child lists point at (`+0x48`, `+0xb4` list).
const CT_VT_SCREEN_LIST: u32 = 0x0049_49e8;

/// `cUIListBox_cUIListBox1(self)` (0x3a55d8, `UIListBox_ctor_2`): the list box
/// constructor. Calls `UIObject_ctor(self, 0, 0)`, sets the vtable at +8,
/// builds the list box flags at +0x74, then initialises the 32 scroll records
/// of 0x14 bytes from +0x98 (the loop counts `s1` from 0x1f down to -1, one
/// `UIListBoxScroll_ctor` per record). The loop's back edge carries an event
/// check. Returns with `s2` (the object) still in place.
pub fn ui_list_box_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0;
        frame.save(c, S2, 0x20);
        c.g[A2][0] = 0;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x98));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x1f;
        CT_OBJECT.call(c, 0x003a_5608)?;
        c.g[S3][0] = u64::MAX;
        c.g[V0][0] = 0x0049_0000;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x74));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4798));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        CT_LISTBOX_FLAGS.call(c, 0x003a_5620)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            CT_LISTBOX_SCROLL.call(c, 0x003a_562c)?;
            let more = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_5620);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cUIState_cUIState2(self)` (0x3a57b8, `UIState_ctor_3`): the UI state
/// constructor. Both list links (+0 and +4) point at the state itself, the
/// vtable (0x4946c8) is stored at +8, the state flags are built at +0x1c and
/// the child list at +0x24. Returns `self`.
pub fn ui_state_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x46c8));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.sw(c.g[S0][0] as u32, c.g[S0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        CT_STATE_FLAGS.call(c, 0x003a_57e4)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
        CT_LIST.call(c, 0x003a_57ec)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cUIScreen_cUIScreen1(self)` (0x3a5b98, `UIScreen_ctor`): the screen
/// constructor, the variant with the screen flags built through the function
/// table. Both list links point at the screen, the vtable (0x494670) is at
/// +8, the screen flags at +0x14, the root child list at +0x18, the root
/// object at +0x40 (`UIObject_ctor(root, 0, 0)`), and the child list at +0xb4
/// with the 0x4949e8 vtable at +0x48. Returns `self`.
pub fn ui_screen_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4670));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
        c.sw(c.g[S0][0] as u32, c.g[S0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, CT_SCREEN_FLAGS, 0x003a_5bc4)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        CT_LIST.call(c, 0x003a_5bcc)?;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
        CT_OBJECT.call(c, 0x003a_5bdc)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb4));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x49e8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V1][0]);
        CT_LIST.call(c, 0x003a_5bf0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cUIScreen_cUIScreen(self, state)` (0x3a5c08, `UIScreen_ctor_2`): the
/// screen constructor that takes the owning UI state (`a1`, stored at +0xd0).
/// Same links and vtable as `UIScreen_ctor`, but the child list (+0x18) and
/// the root object (+0x40) are followed by zeroed fields: +0xc and +0x10
/// (before the flags call), +0x34, +0x38 and +0x3c (after the child list).
/// Returns `self`.
pub fn ui_screen_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4670));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
        c.sw(c.g[S0][0] as u32, c.g[S0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, CT_SCREEN_FLAGS, 0x003a_5c44)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        CT_LIST.call(c, 0x003a_5c4c)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x34), 0);
        c.g[A1][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x38), 0);
        c.g[A2][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
        CT_OBJECT.call(c, 0x003a_5c68)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb4));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x49e8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V1][0]);
        CT_LIST.call(c, 0x003a_5c7c)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xd0), c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cUIScreen_rootGroupPushBack(screen, obj)` (0x3a5cf0): appends `obj` to the
/// screen's child list at +0xb4 (`UIList_PushBack(&list, obj)`, the object in
/// `a1`, which the caller sets). Returns what `UIList_PushBack` returns.
pub fn ui_screen_add_root_child(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb4));
        CT_LIST_PUSH_BACK.call(c, 0x003a_5d00)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cUIText_cUIText1(self)` (0x3a5d60, `UIText_ctor_2`): the text object
/// constructor. Calls `UIObject_ctor(self, 0, 0)`, sets the vtable (0x494588)
/// at +8 and builds the text flags at +0x74. Returns `self`.
pub fn ui_text_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        CT_OBJECT.call(c, 0x003a_5d7c)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4588));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        CT_TEXT_FLAGS.call(c, 0x003a_5d90)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cUIScrollBar_cUIScrollBar1(self)` (0x3a5f60, `cUIScroll_ctor_2`): the scroll
/// bar constructor that only sets the scroll vtable (0x4944c8) at +8 after
/// `UIObject_ctor(self, 0, 0)`. Returns `self`.
pub fn cui_scroll_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        CT_OBJECT.call(c, 0x003a_5f7c)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x44c8));
        frame.restore_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cUIScrollBar_cUIScrollBar(self)` (0x3a5fa0, `cUIScroll_ctor`): the scroll bar
/// constructor with its own fields. After `UIObject_ctor(self, a1, a2)` (the
/// caller's `a1` and `a2` pass through unchanged, as the generated body leaves
/// them), it sets the scroll vtable (0x4944c8) at +8 and clears the knob and
/// arrow pointers, the value and page words, the animation names and the hide
/// flags. Returns `self` in v0.
pub fn cui_scroll_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        CT_OBJECT.call(c, 0x003a_5fb4)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x44c8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x74), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x78), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x80), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x84), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x88), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc8), 0);
        c.sb((c.g[S0][0] as u32).wrapping_add(0x8c), 0);
        c.sb((c.g[S0][0] as u32).wrapping_add(0xa5), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    /// Common constructor fixture: scrambled registers, a random object at
    /// `DATA` (its first words), and `a0` pointing at it.
    fn ui_ctor_fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        fill(c, &mut r, DATA, 0x80);
        c.g[A0][0] = sx(DATA);
        r
    }

    driven!(gen_ui_list_box_ctor_2, ee::s13::f_003a55d8__generated);
    driven!(hand_ui_list_box_ctor_2, ui_list_box_ctor_2);

    #[test]
    fn ui_list_box_ctor_2_matches() {
        check(
            ee::s13::f_003a55d8__generated,
            ui_list_box_ctor_2,
            200,
            &|c, seed| {
                ui_ctor_fixture(c, seed, 0x003a_55d8);
            },
        );
        // The scroll loop's back edge carries an event check: make the
        // first one due so both bodies deliver it at the same place.
        check_events(
            gen_ui_list_box_ctor_2,
            hand_ui_list_box_ctor_2,
            96,
            &|c, seed| {
                let mut r = ui_ctor_fixture(c, seed, 0x003a_55d9);
                c.g[RA][0] = crate::difftest::RA as u64;
                let mode = schedule(c, &mut r, seed, 0x003a_55d8);
                c.next_event = c.cycles + 4;
                mode
            },
        );
    }

    #[test]
    fn ui_state_ctor_3_matches() {
        check(
            ee::s13::f_003a57b8__generated,
            ui_state_ctor_3,
            200,
            &|c, seed| {
                ui_ctor_fixture(c, seed, 0x003a_57b8);
            },
        );
    }

    #[test]
    fn ui_screen_ctor_matches() {
        check(
            ee::s13::f_003a5b98__generated,
            ui_screen_ctor,
            200,
            &|c, seed| {
                ui_ctor_fixture(c, seed, 0x003a_5b98);
            },
        );
    }

    #[test]
    fn ui_screen_ctor_2_matches() {
        check(
            ee::s13::f_003a5c08__generated,
            ui_screen_ctor_2,
            200,
            &|c, seed| {
                let mut r = ui_ctor_fixture(c, seed, 0x003a_5c08);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ui_screen_add_root_child_matches() {
        check(
            ee::s13::f_003a5cf0__generated,
            ui_screen_add_root_child,
            200,
            &|c, seed| {
                let mut r = ui_ctor_fixture(c, seed, 0x003a_5cf0);
                // The child list at +0xb4 must hold a valid tail node: the
                // generated push reads and writes through it.
                c.bus.write32(DATA + 0xb4 + 0xc, DATA + 0x100);
                c.bus.write32(DATA + 0x104, r.next_u32());
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn ui_text_ctor_2_matches() {
        check(
            ee::s13::f_003a5d60__generated,
            ui_text_ctor_2,
            200,
            &|c, seed| {
                ui_ctor_fixture(c, seed, 0x003a_5d60);
            },
        );
    }

    #[test]
    fn cui_scroll_ctor_2_matches() {
        check(
            ee::s13::f_003a5f60__generated,
            cui_scroll_ctor_2,
            200,
            &|c, seed| {
                ui_ctor_fixture(c, seed, 0x003a_5f60);
            },
        );
    }

    #[test]
    fn cui_scroll_ctor_matches() {
        check(
            ee::s13::f_003a5fa0__generated,
            cui_scroll_ctor,
            200,
            &|c, seed| {
                // The generated body passes a1/a2 through to UIObject_ctor
                // unchanged, so they stay scrambled here.
                ui_ctor_fixture(c, seed, 0x003a_5fa0);
            },
        );
    }
}
