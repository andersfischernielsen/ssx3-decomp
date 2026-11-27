//! Staging module oc1086 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1086`.

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
    (0x003a_5140, ui_group_get_child_at, "cUIGroup_GetChildAt"),
    (
        0x003a_5160,
        ui_group_push_front_child,
        "cUIGroup_PushFrontChild",
    ),
    (
        0x003a_5180,
        ui_group_push_back_child,
        "cUIGroup_PushBackChild",
    ),
    (0x003a_5230, ui_menu_ctor, "UIMenu_ctor"),
    (0x003a_5420, ui_pair_ctor_2, "UIPair_ctor_2"),
    (0x003a_5468, ui_pair_ctor, "UIPair_ctor"),
    (0x003a_5530, ui_pair_set_first, "UIPair_SetFirst"),
    (0x003a_5588, ui_pair_set_second, "UIPair_SetSecond"),
];

/// `cUIGroup_getByIndex(group)` (0x3a5140): `UIList_GetAt(&group->children,
/// index)`. The group's child list sits at +0x74; the index is in a1.
pub fn ui_group_get_child_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        Callee {
            addr: 0x0039_7870,
            f: ee::s13::UIList_GetAt,
        }
        .call(c, 0x003a_5150)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIGroup_addToFront(group, item)` (0x3a5160): `UIList_PushFront` on the
/// group's child list (+0x74). The item is in a1.
pub fn ui_group_push_front_child(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        Callee {
            addr: 0x0039_78f0,
            f: ee::s13::UIList_PushFront,
        }
        .call(c, 0x003a_5170)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIGroup_addToEnd(group, item)` (0x3a5180): `UIList_PushBack` on the
/// group's child list (+0x74). The item is in a1.
pub fn ui_group_push_back_child(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        Callee {
            addr: 0x0039_7910,
            f: ee::s13::UIList_PushBack,
        }
        .call(c, 0x003a_5190)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIMenu::UIMenu(this)` (0x3a5230): the object base constructor, the menu's
/// child list at +0x74 (vtable word +0x8 set to the list's), then the menu
/// vtable at +0x8 again, `UIMenuFlags` constructor on the menu, and the
/// menu's own fields (+0x95..+0xa0) cleared.
pub fn ui_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0039_fb30,
            f: ee::s13::UIObject_ctor,
        }
        .call(c, 0x003a_5244)?;
        let menu = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(menu.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x49e8));
        let vt = c.g[V1][0];
        Addr(menu.wrapping_add(0x8)).sw(c, vt);
        Callee {
            addr: 0x0039_77e8,
            f: ee::s13::UIList_ctor,
        }
        .call(c, 0x003a_5258)?;
        let menu = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(menu.wrapping_add(0x90));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4928));
        let vt = c.g[V1][0];
        Addr(menu.wrapping_add(0x8)).sw(c, vt);
        Callee {
            addr: 0x003a_5338,
            f: ee::s13::UIMenuFlags_ctor,
        }
        .call(c, 0x003a_526c)?;
        let menu = c.g[S0][0] as u32;
        c.g[V1][0] = 0xff;
        c.g[V0][0] = c.g[S0][0];
        let flag = c.g[V1][0];
        Addr(menu.wrapping_add(0xa4)).sb(c, flag);
        Addr(menu.wrapping_add(0x95)).sb(c, 0);
        Addr(menu.wrapping_add(0x96)).sb(c, 0);
        Addr(menu.wrapping_add(0x97)).sb(c, 0);
        Addr(menu.wrapping_add(0x98)).sb(c, 0);
        Addr(menu.wrapping_add(0x9c)).sw(c, 0);
        Addr(menu.wrapping_add(0xa0)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIPair::UIPair(this)` without flags (0x3a5420, the first constructor
/// variant): the object base constructor, then the pair flags constructor
/// with a1 = a2 = 0 and the pair vtable at +0x8. Returns `this` in v0.
pub fn ui_pair_ctor_2(c: &mut Cpu) {
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
            f: ee::s13::UIObject_ctor,
        }
        .call(c, 0x003a_543c)?;
        let pair = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(pair.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4868));
        let vt = c.g[V1][0];
        Addr(pair.wrapping_add(0x8)).sw(c, vt);
        Callee {
            addr: 0x003a_5508,
            f: ee::s13::UIPairFlags_ctor,
        }
        .call(c, 0x003a_5450)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `UIPair::UIPair(this)` (0x3a5468): the object base constructor, the pair
/// vtable at +0x8 and the pair flags constructor, then the first and second
/// widget slots (+0x78, +0x7c) cleared and bit 1 set in the flags word +0x14.
pub fn ui_pair_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0039_fb30,
            f: ee::s13::UIObject_ctor,
        }
        .call(c, 0x003a_547c)?;
        let pair = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(pair.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4868));
        let vt = c.g[V1][0];
        Addr(pair.wrapping_add(0x8)).sw(c, vt);
        Callee {
            addr: 0x003a_5508,
            f: ee::s13::UIPairFlags_ctor,
        }
        .call(c, 0x003a_5490)?;
        let pair = c.g[S0][0] as u32;
        c.g[V1][0] = Addr(pair.wrapping_add(0x14)).lw(c);
        c.g[V0][0] = c.g[S0][0];
        Addr(pair.wrapping_add(0x78)).sw(c, 0);
        c.g[V1][0] = c.g[V1][0] | 0x2;
        Addr(pair.wrapping_add(0x7c)).sw(c, 0);
        let flags = c.g[V1][0];
        Addr(pair.wrapping_add(0x14)).sw(c, flags);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of `UIPair_SetFirst` (0x3a5530, `field` 0x78) and
/// `UIPair_SetSecond` (0x3a5588, `field` 0x7c): if the old widget in the
/// slot is set, call its vtable slot 3 with (old + delta, 3) returning to
/// `call_ret`; then store the new widget (a1) in the slot. The two differ only
/// in the slot offset and the return address of that call.
fn ui_pair_set_entry(c: &mut Cpu, field: u32, call_ret: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A2][0] = Addr(c.g[S0][0] as u32).off(field).lw(c);
    c.g[S1][0] = c.g[A1][0];
    if c.g[A2][0] != 0 {
        c.cycles += 6;
        let old = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(old).off(0x8).lw(c);
        c.g[A1][0] = 0x3;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(old.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, call_ret)?;
    }
    c.cycles += 6;
    let value = c.g[S1][0];
    Addr(c.g[S0][0] as u32).off(field).sw(c, value);
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `UIPair::setEntry`: store the first widget (+0x78) (0x3a5530).
pub fn ui_pair_set_first(c: &mut Cpu) {
    guest(c, |c| ui_pair_set_entry(c, 0x78, 0x003a_5568))
}

/// `UIPair::setWidget`: store the second widget (+0x7c) (0x3a5588).
pub fn ui_pair_set_second(c: &mut Cpu) {
    guest(c, |c| ui_pair_set_entry(c, 0x7c, 0x003a_55c0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{covered, machine, machine_rt, FAKE};
    use crate::difftest::{Rng, DATA};

    #[test]
    fn ui_group_get_child_at_matches() {
        check(
            ssx3_ee::s13::f_003a5140__generated,
            ui_group_get_child_at,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x003a_5140, 0);
                let group = DATA + 0x1000;
                let list = group + 0x74;
                c.g[A0][0] = sx(group);
                c.g[A1][0] = (seed % 5) as u64;
                let s = DATA + 0x2000;
                if seed % 4 == 0 {
                    c.bus.write32(list + 4, s);
                    c.bus.write32(s + 4, s);
                    c.bus.write32(s, s);
                } else {
                    // head -> a -> b -> sentinel
                    let a = DATA + 0x2100;
                    let b = DATA + 0x2200;
                    c.bus.write32(list + 4, a);
                    c.bus.write32(a + 4, b);
                    c.bus.write32(a, 0);
                    c.bus.write32(b + 4, s);
                    c.bus.write32(b, 0);
                    c.bus.write32(s + 4, s);
                    c.bus.write32(s, 0);
                }
            },
        );
    }

    #[test]
    fn ui_group_push_front_child_matches() {
        check(
            ssx3_ee::s13::f_003a5160__generated,
            ui_group_push_front_child,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x003a_5160, 0);
                let mut r = Rng(0x003a_5160 ^ seed as u64);
                let group = DATA + 0x1000;
                let list = group + 0x74;
                let item = DATA + 0x1100;
                let node = DATA + 0x1200;
                c.g[A0][0] = sx(group);
                c.g[A1][0] = sx(item);
                // An empty list (head = itself) or one node in front.
                c.bus
                    .write32(list + 4, if seed % 2 == 0 { node } else { list });
                c.bus.write32(node, r.next_u32());
                c.bus.write32(item, r.next_u32());
                c.bus.write32(item + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_group_push_back_child_matches() {
        check(
            ssx3_ee::s13::f_003a5180__generated,
            ui_group_push_back_child,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x003a_5180, 0);
                let mut r = Rng(0x003a_5180 ^ seed as u64);
                let group = DATA + 0x1000;
                let list = group + 0x74;
                let item = DATA + 0x1100;
                let node = DATA + 0x1200;
                c.g[A0][0] = sx(group);
                c.g[A1][0] = sx(item);
                // An empty list (tail = itself) or one node at the back.
                c.bus
                    .write32(list + 0xc, if seed % 2 == 0 { node } else { list });
                c.bus.write32(node + 4, r.next_u32());
                c.bus.write32(item, r.next_u32());
                c.bus.write32(item + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_menu_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a5230__generated,
            ui_menu_ctor,
            200,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x003a_5230, 0);
                let menu = DATA + 0x1000;
                c.g[A0][0] = sx(menu);
                c.pc = 0x003a_5230;
            },
        );
    }

    #[test]
    fn ui_pair_ctor_2_matches() {
        check(
            ssx3_ee::s13::f_003a5420__generated,
            ui_pair_ctor_2,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x003a_5420, 0);
                let pair = DATA + 0x1000;
                c.bus.write32(pair + 0x14, r.next_u32());
                c.g[A0][0] = sx(pair);
                c.pc = 0x003a_5420;
            },
        );
    }

    #[test]
    fn ui_pair_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a5468__generated,
            ui_pair_ctor,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x003a_5468, 0);
                let pair = DATA + 0x1000;
                // Flags word with and without bit 1 already set.
                c.bus.write32(pair + 0x14, r.next_u32());
                c.bus.write32(pair + 0x78, r.next_u32());
                c.bus.write32(pair + 0x7c, r.next_u32());
                c.g[A0][0] = sx(pair);
                c.pc = 0x003a_5468;
            },
        );
    }

    /// The pair and its old widget (whose vtable slot 3 is a fake), with the
    /// new widget in a1. A third of the seeds leave the slot empty, and
    /// `machine(.., 2)` makes some fake calls unwind.
    pub(crate) fn pair_set_setup(c: &mut Cpu, seed: u32, field: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 2);
        let pair = DATA + 0x1000;
        let old = DATA + 0x1400;
        let vt = DATA + 0x1800;
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt + 0x8, delta as u16);
        c.bus.write32(vt + 0xc, FAKE + 0x10 * (seed % 2));
        c.bus.write32(old + 0x8, vt);
        c.bus
            .write32(pair + field, if seed % 4 == 0 { 0 } else { old });
        c.g[A0][0] = sx(pair);
        c.g[A1][0] = sx(DATA + 0x1200);
    }

    #[test]
    fn ui_pair_set_first_matches() {
        covered(
            ssx3_ee::s13::f_003a5530__generated,
            ui_pair_set_first,
            200,
            &|c, seed| pair_set_setup(c, seed, 0x78, 0x003a_5530),
        );
    }

    #[test]
    fn ui_pair_set_second_matches() {
        covered(
            ssx3_ee::s13::f_003a5588__generated,
            ui_pair_set_second,
            200,
            &|c, seed| pair_set_setup(c, seed, 0x7c, 0x003a_5588),
        );
    }
}
