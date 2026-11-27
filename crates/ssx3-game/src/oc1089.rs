//! Staging module oc1089 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1089`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003a_6400,
        cui_text_scroll_list_last,
        "cUITextScroll_ListLast",
    ),
    (
        0x003a_6420,
        cui_text_scroll_list_get_at,
        "cUITextScroll_ListGetAt",
    ),
    (
        0x003a_6440,
        cui_text_scroll_list_push_front,
        "cUITextScroll_ListPushFront",
    ),
    (
        0x003a_6460,
        cui_text_scroll_list_push_back,
        "cUITextScroll_ListPushBack",
    ),
    (0x003a_64f8, ui_vector_ctor_2, "UIVector_ctor_2"),
    (0x003a_6548, ui_object_flags_ctor, "UIObjectFlags_ctor"),
    (0x003a_65a0, ui_list_box_flags_ctor, "UIListBoxFlags_ctor"),
    (0x003a_65b8, ui_list_box_scroll_ctor, "UIListBoxScroll_ctor"),
];

/// The `cList` member of a `cUITextScroll`, at +0x8c.
const TEXT_SCROLL_LIST_OFFSET: u32 = 0x8c;

/// `cUITextScroll_ListLast/GetAt/PushFront/PushBack(textScroll, ...)`: the
/// generated forwarder. A 0x10 frame holds `ra`, `a0` becomes
/// `textScroll + 0x8c` (the list), the list callee runs with the caller's other
/// arguments, and its result is returned.
fn text_scroll_list_forward(c: &mut Cpu, list_fn: Callee, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(TEXT_SCROLL_LIST_OFFSET));
        list_fn.call(c, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUITextScroll_ListLast(textScroll)` (0x3a6400): `UIList_Last(&list)`.
pub fn cui_text_scroll_list_last(c: &mut Cpu) {
    text_scroll_list_forward(
        c,
        Callee {
            addr: 0x0039_7858,
            f: ssx3_ee::s13::f_00397858,
        },
        0x003a_6410,
    )
}

/// `cUITextScroll_ListGetAt(textScroll, index)` (0x3a6420): `UIList_GetAt(&list, index)`.
pub fn cui_text_scroll_list_get_at(c: &mut Cpu) {
    text_scroll_list_forward(
        c,
        Callee {
            addr: 0x0039_7870,
            f: ssx3_ee::s13::f_00397870,
        },
        0x003a_6430,
    )
}

/// `cUITextScroll_ListPushFront(textScroll, node)` (0x3a6440): `UIList_PushFront(&list, node)`.
pub fn cui_text_scroll_list_push_front(c: &mut Cpu) {
    text_scroll_list_forward(
        c,
        Callee {
            addr: 0x0039_78f0,
            f: ssx3_ee::s13::f_003978f0,
        },
        0x003a_6450,
    )
}

/// `cUITextScroll_ListPushBack(textScroll, node)` (0x3a6460): `UIList_PushBack(&list, node)`.
pub fn cui_text_scroll_list_push_back(c: &mut Cpu) {
    text_scroll_list_forward(
        c,
        Callee {
            addr: 0x0039_7910,
            f: ssx3_ee::s13::f_00397910,
        },
        0x003a_6470,
    )
}

/// `UIVector* UIVector_ctor_2(UIVector* self)` (0x3a64f8): `UIObject_ctor(self, 0, 0)`,
/// the vtable pointer at +8 (0x4941b8), `UIVectorFlags_ctor(&self->flags)` at +0x74,
/// returns `self`.
pub fn ui_vector_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0039_fb30,
            f: ssx3_ee::s13::f_0039fb30,
        }
        .call(c, 0x003a_6514)?;
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_add(0x41b8));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        Callee {
            addr: 0x003a_6618,
            f: ssx3_ee::s13::f_003a6618,
        }
        .call(c, 0x003a_6528)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `UIObjectFlags_ctor(UIObject* o)` (0x3a6548): sets the flags word at +0 of an
/// object: bits 0, 1, 3 and 6 on; 2, 4, 5, 7 and bits 8..12 off; bits 13 and 16
/// on, bits 13..18 cleared first; the rest kept. Leaves the same scratch registers
/// the generated code does.
pub fn ui_object_flags_ctor(c: &mut Cpu) {
    c.cycles += 22;
    let obj = Addr(c.g[A0][0] as u32);
    c.g[V1][0] = obj.lw(c);
    c.g[V0][0] = 0xffff_ffff_ffff_ff7f;
    c.g[A1][0] = 0xffff_ffff_ffff_e0ff;
    c.g[A3][0] = 0xffff_ffff_ffff_ffdf;
    c.g[V1][0] |= 0x1;
    c.g[T0][0] = 0xffff_ffff_ffff_ffef;
    c.g[V1][0] &= c.g[V0][0];
    c.g[T1][0] = 0xffff_ffff_ffff_fffb;
    c.g[V1][0] &= c.g[A1][0];
    c.g[A2][0] = 0xffff_ffff_fff8_0000;
    c.g[V1][0] &= c.g[A3][0];
    c.g[A2][0] |= 0x1fff;
    c.g[V1][0] &= c.g[T0][0];
    c.g[A1][0] = 0x10000;
    c.g[V1][0] &= c.g[T1][0];
    c.g[A1][0] |= 0x2000;
    c.g[V1][0] |= 0x4a;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] &= c.g[A2][0];
    c.g[V1][0] |= c.g[A1][0];
    let flags = c.g[V1][0];
    obj.sw(c, flags);
    ret(c);
}

/// `UIListBoxFlags_ctor(attr)` (0x3a65a0): the byte at +0 becomes
/// `(byte | 4) & 0xc4`; returns `attr`.
pub fn ui_list_box_flags_ctor(c: &mut Cpu) {
    c.cycles += 6;
    let obj = Addr(c.g[A0][0] as u32);
    c.g[V1][0] = obj.lbu(c);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = (c.g[V1][0] | 0x4) & 0xc4;
    let flags = c.g[V1][0];
    obj.sb(c, flags);
    ret(c);
}

/// `UIListBoxScroll_ctor(entry)` (0x3a65b8): zeroes the words at +0, +4, +8,
/// +0xc and the byte at +0x10; returns `entry`.
pub fn ui_list_box_scroll_ctor(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = c.g[A0][0];
    let obj = Addr(c.g[A0][0] as u32);
    obj.sw(c, 0);
    obj.off(0x4).sw(c, 0);
    obj.off(0x8).sw(c, 0);
    obj.off(0xc).sw(c, 0);
    obj.off(0x10).sb(c, 0);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA};

    /// The textScroll object of the list tests; its list is at +0x8c.
    const LIST_TEST_TEXT_SCROLL: u32 = DATA + 0x1000;
    /// The list sentinel: self-linked, so a list of only this node is empty.
    const LIST_TEST_SENTINEL: u32 = DATA + 0x2080;
    /// The first of up to three list nodes (0x20 bytes apart).
    const LIST_TEST_NODES: u32 = DATA + 0x2100;
    /// A node that is not in the list yet (the push tests add it).
    const LIST_TEST_NEW_NODE: u32 = DATA + 0x2200;
    /// The object the flag and scroll constructors write.
    const LIST_TEST_OBJECT: u32 = DATA + 0x4000;

    /// A text scroll whose list holds `seed % 4` nodes between the sentinel and
    /// the head/last pointers. Every few seeds the last node is self-prev (the
    /// `UIList_Last` zero branch).
    fn list_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1089_0000 ^ seed as u64);
        scramble(c, &mut r);
        let list = LIST_TEST_TEXT_SCROLL + TEXT_SCROLL_LIST_OFFSET;
        let k = seed % 4;
        c.bus.write32(LIST_TEST_SENTINEL, LIST_TEST_SENTINEL);
        c.bus.write32(LIST_TEST_SENTINEL + 4, LIST_TEST_SENTINEL);
        for i in 0..k {
            let node = LIST_TEST_NODES + 0x20 * i;
            let next = if i + 1 < k {
                LIST_TEST_NODES + 0x20 * (i + 1)
            } else {
                LIST_TEST_SENTINEL
            };
            c.bus.write32(node, LIST_TEST_SENTINEL);
            c.bus.write32(node + 4, next);
        }
        let (first, last) = if k > 0 {
            (LIST_TEST_NODES, LIST_TEST_NODES + 0x20 * (k - 1))
        } else {
            (LIST_TEST_SENTINEL, LIST_TEST_SENTINEL)
        };
        c.bus.write32(list + 4, first);
        c.bus.write32(list + 0xc, last);
        if k > 0 && seed % 6 == 5 {
            c.bus.write32(last, last);
        }
    }

    /// [`list_fixture`] plus the forwarder's arguments: `a0` = textScroll, `a1` = `a1`.
    fn list_setup(c: &mut Cpu, seed: u32, a1: u64) {
        list_fixture(c, seed);
        c.g[A0][0] = sx(LIST_TEST_TEXT_SCROLL);
        c.g[A1][0] = a1;
    }

    #[test]
    fn cui_text_scroll_list_last_matches() {
        check(
            ssx3_ee::s13::f_003a6400__generated,
            cui_text_scroll_list_last,
            300,
            &|c, seed| list_setup(c, seed, sx(seed)),
        );
    }

    #[test]
    fn cui_text_scroll_list_get_at_matches() {
        check(
            ssx3_ee::s13::f_003a6420__generated,
            cui_text_scroll_list_get_at,
            300,
            &|c, seed| {
                // 0 and a negative index take the first-node path; others walk.
                let index = match seed % 7 {
                    0 => 0,
                    1 => 0x8000_0000,
                    _ => seed % 5,
                };
                list_setup(c, seed, sx(index));
            },
        );
    }

    #[test]
    fn cui_text_scroll_list_push_front_matches() {
        check(
            ssx3_ee::s13::f_003a6440__generated,
            cui_text_scroll_list_push_front,
            300,
            &|c, seed| list_setup(c, seed, sx(LIST_TEST_NEW_NODE)),
        );
    }

    #[test]
    fn cui_text_scroll_list_push_back_matches() {
        check(
            ssx3_ee::s13::f_003a6460__generated,
            cui_text_scroll_list_push_back,
            300,
            &|c, seed| list_setup(c, seed, sx(LIST_TEST_NEW_NODE)),
        );
    }

    #[test]
    fn ui_vector_ctor_2_matches() {
        check(
            ssx3_ee::s13::f_003a64f8__generated,
            ui_vector_ctor_2,
            100,
            &|c, seed| {
                let mut r = Rng(0x1089_1000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, LIST_TEST_OBJECT, 32);
                c.g[A0][0] = sx(LIST_TEST_OBJECT);
            },
        );
    }

    #[test]
    fn ui_object_flags_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a6548__generated,
            ui_object_flags_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x1089_2000 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(LIST_TEST_OBJECT, r.next_u32());
                c.g[A0][0] = sx(LIST_TEST_OBJECT);
            },
        );
    }

    #[test]
    fn ui_list_box_flags_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a65a0__generated,
            ui_list_box_flags_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x1089_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(LIST_TEST_OBJECT, r.next_u32());
                c.g[A0][0] = sx(LIST_TEST_OBJECT);
            },
        );
    }

    #[test]
    fn ui_list_box_scroll_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a65b8__generated,
            ui_list_box_scroll_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x1089_4000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, LIST_TEST_OBJECT, 8);
                c.g[A0][0] = sx(LIST_TEST_OBJECT);
            },
        );
    }
}
