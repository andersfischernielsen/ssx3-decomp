//! Staging module oc1088 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1088`.

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
        0x003a_6038,
        ui_scroll_set_up_thread_name,
        "cUIScroll_SetUpThreadName",
    ),
    (
        0x003a_6088,
        ui_scroll_set_down_thread_name,
        "cUIScroll_SetDownThreadName",
    ),
    (0x003a_60e0, ui_progress_ctor_2, "cUIProgress_ctor_2"),
    (0x003a_6120, ui_progress_ctor, "cUIProgress_ctor"),
    (0x003a_6190, ui_slider_ctor_2, "cUISlider_ctor_2"),
    (0x003a_61d0, ui_slider_ctor, "cUISlider_ctor"),
    (0x003a_6250, ui_text_scroll_ctor_2, "cUITextScroll_ctor_2"),
    (
        0x003a_63e0,
        ui_text_scroll_list_first,
        "cUITextScroll_ListFirst",
    ),
];

/// `cUIThing_cUIThing` (0x39fb30): the base constructor the widget
/// constructors below run first.
const UI_THING_CTOR_OC1088: Callee = Callee {
    addr: 0x0039_fb30,
    f: ee::s13::f_0039fb30,
};

/// `UIList_ctor` (0x3977e8): the scroll's name list.
const UI_LIST_CTOR_OC1088: Callee = Callee {
    addr: 0x0039_77e8,
    f: ee::s13::f_003977e8,
};

/// `cUITextScrollFlags_ctor` (0x3a6600): one flag byte at its argument.
const TEXT_FLAGS_CTOR_OC1088: Callee = Callee {
    addr: 0x003a_6600,
    f: ee::s13::f_003a6600,
};

/// `UIList_First` (0x397840): the first node of a list whose head is at +4.
const UI_LIST_FIRST_OC1088: Callee = Callee {
    addr: 0x0039_7840,
    f: ee::s13::f_00397840,
};

/// Shared body of the two thread-name setters (`cUIScrollBar_setUpAnimationName`
/// and `setDownAnimationName` in the reference): when `strlen(name) < 25` the
/// name is copied with `strcpy` into `self+name_off`, otherwise the field's
/// first byte is cleared. The setters differ only in the field offset and in
/// their call return addresses.
fn set_scroll_thread_name(c: &mut Cpu, name_off: u32, strlen_ret: u32, strcpy_ret: u32) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0041_6810, strlen_ret)?;
    c.g[V0][0] = (c.g[V0][0] < 0x19) as u64;
    let short_name = c.g[V0][0] != 0;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(name_off));
    c.cycles += 2;
    if short_name {
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0041_66f4, strcpy_ret)?;
    } else {
        let field = (c.g[S0][0] as u32).wrapping_add(name_off);
        Addr(field).sb(c, 0);
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cUIScroll_SetUpThreadName(cUIScroll* self, const char* name)`
/// (0x3a6038): copy the name into `self+0x8c`, or clear it when it is 25
/// bytes or longer.
pub fn ui_scroll_set_up_thread_name(c: &mut Cpu) {
    guest(c, |c| {
        set_scroll_thread_name(c, 0x8c, 0x003a_6058, 0x003a_6074)
    });
}

/// `void cUIScroll_SetDownThreadName(cUIScroll* self, const char* name)`
/// (0x3a6088): the same as the up-name setter, for the field at `self+0xa5`.
pub fn ui_scroll_set_down_thread_name(c: &mut Cpu) {
    guest(c, |c| {
        set_scroll_thread_name(c, 0xa5, 0x003a_60a8, 0x003a_60c4)
    });
}

/// `cUIProgress* cUIProgress_ctor_2(cUIProgress* self)` (0x3a60e0): the UI
/// thing constructor with zero flags, then the progress vtable at `+8`.
pub fn ui_progress_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UI_THING_CTOR_OC1088.call(c, 0x003a_60fc)?;
        c.g[V1][0] = sx(0x0049_4408);
        c.g[V0][0] = c.g[S0][0];
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIProgress* cUIProgress_ctor(cUIProgress* self)` (0x3a6120): the UI
/// thing constructor, the progress vtable at `+8`, and the four knob and
/// background fields (`+0x74..+0x84`) cleared.
pub fn ui_progress_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UI_THING_CTOR_OC1088.call(c, 0x003a_6134)?;
        c.g[V1][0] = sx(0x0049_4408);
        c.g[V0][0] = c.g[S0][0];
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x74)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        Addr((c.g[S0][0] as u32).wrapping_add(0x78)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x7c)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x80)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x84)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUISlider* cUISlider_ctor_2(cUISlider* self)` (0x3a6190): the UI thing
/// constructor with zero flags, then the slider vtable at `+8`.
pub fn ui_slider_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UI_THING_CTOR_OC1088.call(c, 0x003a_61ac)?;
        c.g[V1][0] = sx(0x0049_4348);
        c.g[V0][0] = c.g[S0][0];
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUISlider* cUISlider_ctor(cUISlider* self)` (0x3a61d0): the UI thing
/// constructor, the slider vtable at `+8`, and the slider's value and range
/// fields (`+0x74..+0x8c`) cleared.
pub fn ui_slider_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UI_THING_CTOR_OC1088.call(c, 0x003a_61e4)?;
        c.g[V1][0] = sx(0x0049_4348);
        c.g[V0][0] = c.g[S0][0];
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x74)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        Addr((c.g[S0][0] as u32).wrapping_add(0x78)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x7c)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x80)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x88)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x8c)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUITextScroll* cUITextScroll_ctor_2(cUITextScroll* self)` (0x3a6250):
/// the UI thing constructor, the text-scroll vtable at `+8`, the flag byte
/// at `+0x74` (`cUITextScrollFlags_ctor`), and the name list at `+0x8c`.
pub fn ui_text_scroll_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UI_THING_CTOR_OC1088.call(c, 0x003a_626c)?;
        c.g[V1][0] = sx(0x0049_4278);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        TEXT_FLAGS_CTOR_OC1088.call(c, 0x003a_6280)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
        UI_LIST_CTOR_OC1088.call(c, 0x003a_6288)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cUITextScroll_ListFirst(cUITextScroll* self)` (0x3a63e0): the
/// first node of the name list at `self+0x8c`.
pub fn ui_text_scroll_list_first(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8c));
        UI_LIST_FIRST_OC1088.call(c, 0x003a_63f0)?;
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
    use crate::difftest::DATA;
    use crate::game::fe::feasyncfile::tests::base_13 as base;

    /// A scroll at `DATA+0x1000` with junk around its name field (`+0x8c` and
    /// `+0xa5`), and a NUL-terminated thread name at `DATA+0x2000` of
    /// `seed % 41` bytes: names of 25 bytes or more take the clearing path.
    pub(crate) fn scroll_thread_name_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let scroll = DATA + 0x1000;
        let name = DATA + 0x2000;
        fill(c, &mut r, scroll + 0x80, 0x20);
        let len = seed % 41;
        for k in 0..len {
            c.bus.write8(name + k, b'a' + (k % 26) as u8);
        }
        c.bus.write8(name + len, 0);
        c.g[A0][0] = sx(scroll);
        c.g[A1][0] = sx(name);
    }

    /// A widget at `DATA+0x1000` filled with junk, for the constructors that
    /// only write their own fields and call the base constructor.
    pub(crate) fn widget_ctor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let obj = DATA + 0x1000;
        fill(c, &mut r, obj, 0x40);
        c.g[A0][0] = sx(obj);
    }

    #[test]
    fn ui_scroll_set_up_thread_name_matches() {
        check(
            ee::s13::f_003a6038__generated,
            ui_scroll_set_up_thread_name,
            200,
            &|c, seed| scroll_thread_name_setup(c, seed, 0x3a_6038),
        );
    }

    #[test]
    fn ui_scroll_set_down_thread_name_matches() {
        check(
            ee::s13::f_003a6088__generated,
            ui_scroll_set_down_thread_name,
            200,
            &|c, seed| scroll_thread_name_setup(c, seed, 0x3a_6088),
        );
    }

    #[test]
    fn ui_progress_ctor_2_matches() {
        check(
            ee::s13::f_003a60e0__generated,
            ui_progress_ctor_2,
            200,
            &|c, seed| widget_ctor_setup(c, seed, 0x3a_60e0),
        );
    }

    #[test]
    fn ui_progress_ctor_matches() {
        check(
            ee::s13::f_003a6120__generated,
            ui_progress_ctor,
            200,
            &|c, seed| widget_ctor_setup(c, seed, 0x3a_6120),
        );
    }

    #[test]
    fn ui_slider_ctor_2_matches() {
        check(
            ee::s13::f_003a6190__generated,
            ui_slider_ctor_2,
            200,
            &|c, seed| widget_ctor_setup(c, seed, 0x3a_6190),
        );
    }

    #[test]
    fn ui_slider_ctor_matches() {
        check(
            ee::s13::f_003a61d0__generated,
            ui_slider_ctor,
            200,
            &|c, seed| widget_ctor_setup(c, seed, 0x3a_61d0),
        );
    }

    #[test]
    fn ui_text_scroll_ctor_2_matches() {
        check(
            ee::s13::f_003a6250__generated,
            ui_text_scroll_ctor_2,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 0x3a_6250, 4);
                let obj = DATA + 0x8000;
                fill(c, &mut r, obj, 0x40);
                c.g[A0][0] = sx(obj);
            },
        );
    }

    #[test]
    fn ui_text_scroll_list_first_matches() {
        check(
            ee::s13::f_003a63e0__generated,
            ui_text_scroll_list_first,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_63e0, 0);
                let obj = DATA + 0x1000;
                let node = DATA + 0x1200;
                fill(c, &mut r, obj, 0x40);
                // The list head at `obj+0x8c` points at `node`; `node+4` is
                // the node itself (empty) on even seeds, junk otherwise.
                c.bus.write32(obj + 0x8c + 4, node);
                let next = if seed % 2 == 0 { node } else { r.next_u32() };
                c.bus.write32(node + 4, next);
                c.g[A0][0] = sx(obj);
            },
        );
    }
}
