//! Staging module oc949 (wave wave256): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc949`.

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
        0x001d_f5c8,
        fe_online_main_menu_ctor_2,
        "cFEStateOnlineMainMenu_ctor_2",
    ),
    (
        0x001d_f608,
        fe_online_main_menu_dtor,
        "cFEStateOnlineMainMenu_dtor",
    ),
    (
        0x001d_f638,
        fe_quick_play_popup_ctor_2,
        "cFEStateNetQuickPlayPopup_ctor_2",
    ),
    (
        0x001d_f678,
        fe_quick_play_popup_dtor,
        "cFEStateNetQuickPlayPopup_dtor",
    ),
    (
        0x001d_f6a8,
        fe_ssx_world_ctor_2,
        "cFEStateNetSSXWorld_ctor_2",
    ),
    (0x001d_f6e8, fe_ssx_world_dtor, "cFEStateNetSSXWorld_dtor"),
    (
        0x001d_f710,
        fe_quick_messages_ctor_2,
        "cFEStateNetQuickMessages_ctor_2",
    ),
    (
        0x001d_f750,
        fe_quick_messages_dtor,
        "cFEStateNetQuickMessages_dtor",
    ),
];

/// `lui 0x470000`: the vtable pages, reached as `0x470000 - off`.
const HI_VTABLE: u32 = 0x0047_0000;
/// `cFEStateNet_cFEStateNet`: the base constructor the net states call.
const NET_STATE_CTOR: u32 = 0x001a_8500;
/// The base destructor the net states call first.
const NET_STATE_DTOR: u32 = 0x001a_85d0;

/// Constructor shell of the net front-end states: `ctor(self)` calls the base
/// constructor as `(self, 0, 0)`, stores the state's vtable (`0x470000 -
/// vtable_off`) at +8 of self, optionally zeroes the word at `zero_at`, and
/// returns self in v0. `pre` is the cycles charged before the frame opens;
/// `base_ret` the return address of the base call.
fn fe_state_ctor(
    c: &mut Cpu,
    pre: u64,
    base_ret: u32,
    vtable_off: u32,
    zero_at: Option<u32>,
) -> Flow {
    c.cycles += pre;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = 0;
    frame.save(c, S0, 0x10);
    c.g[A2][0] = 0;
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, NET_STATE_CTOR, base_ret)?;
    let this = c.g[S0][0] as u32;
    c.g[V1][0] = HI_VTABLE as u64;
    c.g[V0][0] = c.g[S0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(vtable_off));
    if let Some(off) = zero_at {
        Addr(this.wrapping_add(off)).sw(c, 0);
    }
    let vtable = c.g[V1][0];
    Addr(this.wrapping_add(0x8)).sw(c, vtable);
    frame.restore_ra(c, 0);
    frame.restore(c, S0, 0x10);
    frame.close(c);
    Ok(())
}

/// Destructor shell of the net front-end states: stores the state's vtable
/// (`0x470000 - vtable_off`) at +8 of self, then runs the base destructor
/// with self still in a0. `ret_addr` is the return address of that call.
fn fe_state_dtor(c: &mut Cpu, vtable_off: u32, ret_addr: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = HI_VTABLE as u64;
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(vtable_off));
    let vtable = c.g[V0][0];
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, vtable);
    call_ptr(c, NET_STATE_DTOR, ret_addr)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEStateOnlineMainMenu_ctor_2` (0x1df5c8): the net state constructor
/// with the online main menu's vtable.
pub fn fe_online_main_menu_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_state_ctor(c, 15, 0x001d_f5e4, 0x52b8, None))
}

/// `cFEStateOnlineMainMenu_dtor` (0x1df608).
pub fn fe_online_main_menu_dtor(c: &mut Cpu) {
    guest(c, |c| fe_state_dtor(c, 0x52b8, 0x001d_f624))
}

/// `cFEStateNetQuickPlayPopup_ctor_2` (0x1df638): also zeroes the word at
/// +0x6ec before the vtable store.
pub fn fe_quick_play_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_ctor(c, 16, 0x001d_f654, 0x54d8, Some(0x6ec))
    })
}

/// `cFEStateNetQuickPlayPopup_dtor` (0x1df678).
pub fn fe_quick_play_popup_dtor(c: &mut Cpu) {
    guest(c, |c| fe_state_dtor(c, 0x54d8, 0x001d_f694))
}

/// `cFEStateNetSSXWorld_ctor_2` (0x1df6a8).
pub fn fe_ssx_world_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_state_ctor(c, 15, 0x001d_f6c4, 0x55e8, None))
}

/// `cFEStateNetSSXWorld_dtor` (0x1df6e8).
pub fn fe_ssx_world_dtor(c: &mut Cpu) {
    guest(c, |c| fe_state_dtor(c, 0x55e8, 0x001d_f704))
}

/// `cFEStateNetQuickMessages_ctor_2` (0x1df710).
pub fn fe_quick_messages_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_state_ctor(c, 15, 0x001d_f72c, 0x56f8, None))
}

/// `cFEStateNetQuickMessages_dtor` (0x1df750).
pub fn fe_quick_messages_dtor(c: &mut Cpu) {
    guest(c, |c| fe_state_dtor(c, 0x56f8, 0x001d_f76c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Control words in the machine's RAM: the gate (magic), the unwind flag
    /// (1 = the base call unwinds), and the base-call count.
    pub(crate) const OC949_CTRL: u32 = DATA + 0x20_4900;
    pub(crate) const OC949_MAGIC: u32 = 0x4f43_3934;
    /// The state objects the constructors and destructors work on.
    pub(crate) const OC949_SELF: u32 = DATA + 0x21_0000;

    static OC949_PREV_NET_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC949_PREV_NET_DTOR: AtomicUsize = AtomicUsize::new(0);

    /// The gated base call: counts the call, unwinds when the fixture asks,
    /// else returns to `ra`. Both sides of a diff run this same fake.
    fn oc949_base_fake(c: &mut Cpu) {
        let calls = c.bus.read32(OC949_CTRL + 8);
        c.bus.write32(OC949_CTRL + 8, calls.wrapping_add(1));
        if c.bus.read32(OC949_CTRL + 4) == 1 {
            c.pc = 0xdead_0000;
            return;
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Fake of `cFEStateNet_cFEStateNet` (0x1a8500), gated on [`OC949_MAGIC`].
    pub(crate) fn oc949_fk_net_ctor(c: &mut Cpu) {
        if c.bus.read32(OC949_CTRL) != OC949_MAGIC {
            return fakes::fall_back(c, &OC949_PREV_NET_CTOR, ee::s03::f_001a8500__generated);
        }
        oc949_base_fake(c);
    }

    /// Fake of the net state base destructor (0x1a85d0), gated on [`OC949_MAGIC`].
    pub(crate) fn oc949_fk_net_dtor(c: &mut Cpu) {
        if c.bus.read32(OC949_CTRL) != OC949_MAGIC {
            return fakes::fall_back(c, &OC949_PREV_NET_DTOR, ee::s03::f_001a85d0__generated);
        }
        oc949_base_fake(c);
    }

    pub(crate) fn oc949_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A8500),
            oc949_fk_net_ctor,
            &OC949_PREV_NET_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A85D0),
            oc949_fk_net_dtor,
            &OC949_PREV_NET_DTOR,
        );
    }

    /// The fixture both sides of a diff start from: scrambled registers and
    /// hooks, the gate on, the base call unwinding on every third seed, self
    /// at one of four objects with scrambled words.
    pub(crate) fn oc949_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(OC949_CTRL, OC949_MAGIC);
        c.bus.write32(OC949_CTRL + 4, (seed % 3 == 0) as u32);
        c.bus.write32(OC949_CTRL + 8, 0);
        let this = OC949_SELF + 0x800 * (seed % 4);
        for k in 0..0x80 {
            c.bus.write32(this + 4 * k, r.next_u32());
        }
        c.g[A0][0] = sx(this);
    }

    #[test]
    fn fe_online_main_menu_ctor_2_matches() {
        oc949_install_fakes();
        check(
            ee::s04::f_001df5c8__generated,
            fe_online_main_menu_ctor_2,
            200,
            &|c, seed| oc949_fixture(c, seed, 0x5c8),
        );
    }

    #[test]
    fn fe_online_main_menu_dtor_matches() {
        oc949_install_fakes();
        check(
            ee::s04::f_001df608__generated,
            fe_online_main_menu_dtor,
            200,
            &|c, seed| oc949_fixture(c, seed, 0x608),
        );
    }

    #[test]
    fn fe_quick_play_popup_ctor_2_matches() {
        oc949_install_fakes();
        check(
            ee::s04::f_001df638__generated,
            fe_quick_play_popup_ctor_2,
            200,
            &|c, seed| oc949_fixture(c, seed, 0x638),
        );
    }

    #[test]
    fn fe_quick_play_popup_dtor_matches() {
        oc949_install_fakes();
        check(
            ee::s04::f_001df678__generated,
            fe_quick_play_popup_dtor,
            200,
            &|c, seed| oc949_fixture(c, seed, 0x678),
        );
    }

    #[test]
    fn fe_ssx_world_ctor_2_matches() {
        oc949_install_fakes();
        check(
            ee::s04::f_001df6a8__generated,
            fe_ssx_world_ctor_2,
            200,
            &|c, seed| oc949_fixture(c, seed, 0x6a8),
        );
    }

    #[test]
    fn fe_ssx_world_dtor_matches() {
        oc949_install_fakes();
        check(
            ee::s04::f_001df6e8__generated,
            fe_ssx_world_dtor,
            200,
            &|c, seed| oc949_fixture(c, seed, 0x6e8),
        );
    }

    #[test]
    fn fe_quick_messages_ctor_2_matches() {
        oc949_install_fakes();
        check(
            ee::s04::f_001df710__generated,
            fe_quick_messages_ctor_2,
            200,
            &|c, seed| oc949_fixture(c, seed, 0x710),
        );
    }

    #[test]
    fn fe_quick_messages_dtor_matches() {
        oc949_install_fakes();
        check(
            ee::s04::f_001df750__generated,
            fe_quick_messages_dtor,
            200,
            &|c, seed| oc949_fixture(c, seed, 0x750),
        );
    }
}
