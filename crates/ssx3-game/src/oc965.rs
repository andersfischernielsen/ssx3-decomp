//! Staging module oc965 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc965`.

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
        0x001e_0da8,
        cfe_state_create_step2_dtor,
        "cFEStateCreateStep2_dtor",
    ),
    (
        0x001e_0dd8,
        cfe_state_create_duplicate_ctor_2,
        "cFEStateCreateDuplicate_ctor_2",
    ),
    (
        0x001e_0e18,
        cfe_state_create_duplicate_dtor,
        "cFEStateCreateDuplicate_dtor",
    ),
    (0x001e_0e48, cfe_state_login_ctor_2, "cFEStateLogin_ctor_2"),
    (0x001e_0e98, cfe_state_stats_dtor, "cFEStateStats_dtor"),
    (0x001e_0ec8, cfe_state_net_news_dtor, "cFEStateNetNews_dtor"),
    (
        0x001e_0ef8,
        cfe_state_relogin_ctor_2,
        "cFEStateRelogin_ctor_2",
    ),
    (0x001e_0f30, cfe_state_relogin_dtor, "cFEStateRelogin_dtor"),
];

/// The shape shared by the five one-call destructors of the FE popup states:
/// a 0x10 frame holding `ra`, the vtable `0x470000 - vtable_off` stored at
/// `+0x8` of `this` (`a0`), then the base destructor `base_dtor` returning to
/// `ret_to`. `a0` and `a1` reach the base destructor as the caller left them.
fn oc965_vtable_dtor(
    c: &mut Cpu,
    vtable_off: u32,
    base_dtor: u32,
    ret_to: u32,
) -> crate::call::Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx(0x47_0000 - vtable_off);
    let vtable = c.g[V0][0];
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, vtable);
    call_ptr(c, base_dtor, ret_to)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEStateCreateStep2::~cFEStateCreateStep2(this)` (0x1e0da8): vtable
/// `0x4685b0` (`0x470000 - 0x7a50`) at `+0x8`, then the base destructor
/// `NetFEState_dtor` (0x1a85d0).
pub fn cfe_state_create_step2_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc965_vtable_dtor(c, 0x7a50, 0x001a_85d0, 0x001e_0dc4)
    })
}

/// `cFEStateCreateDuplicate::cFEStateCreateDuplicate(this)`, the second
/// constructor (0x1e0dd8): build the base with `NetFEState_ctor(this, 0, 0)`
/// (0x1a8500), then the vtable `0x4684a0` (`0x470000 - 0x7b60`) at `+0x8`.
pub fn cfe_state_create_duplicate_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001a_8500, 0x001e_0df4)?;
        // The base constructor returns with `this` still in s0.
        c.g[V1][0] = sx(0x47_0000 - 0x7b60);
        c.g[V0][0] = c.g[S0][0];
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateDuplicate::~cFEStateCreateDuplicate(this)` (0x1e0e18):
/// vtable `0x4684a0` (`0x470000 - 0x7b60`) at `+0x8`, then `NetFEState_dtor`.
pub fn cfe_state_create_duplicate_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc965_vtable_dtor(c, 0x7b60, 0x001a_85d0, 0x001e_0e34)
    })
}

/// `cFEStateLogin::cFEStateLogin(this)`, the second constructor (0x1e0e48):
/// build the base with `NetFEState_ctor(this, 0, 0)` (0x1a8500), then clear
/// `+0x6d0` and set the vtable `0x468388` (`0x470000 - 0x7c78`) at `+0x8`.
pub fn cfe_state_login_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001a_8500, 0x001e_0e64)?;
        c.g[V1][0] = sx(0x47_0000 - 0x7c78);
        c.g[V0][0] = c.g[S0][0];
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x6d0)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateStats::~cFEStateStats(this)` (0x1e0e98): vtable `0x468278`
/// (`0x470000 - 0x7d88`) at `+0x8`, then `NetFEState_dtor`.
pub fn cfe_state_stats_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc965_vtable_dtor(c, 0x7d88, 0x001a_85d0, 0x001e_0eb4)
    })
}

/// `cFEStateNetNews::~cFEStateNetNews(this)` (0x1e0ec8): vtable `0x468168`
/// (`0x470000 - 0x7e98`) at `+0x8`, then `NetFEState_dtor`.
pub fn cfe_state_net_news_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc965_vtable_dtor(c, 0x7e98, 0x001a_85d0, 0x001e_0ee4)
    })
}

/// `cFEStateRelogin::cFEStateRelogin(this)`, the second constructor (0x1e0ef8):
/// build the login base with `cFEStateLogin_ctor(this, 0)` (0x1bee30), then
/// the vtable `0x468050` (`0x470000 - 0x7fb0`) at `+0x8`.
pub fn cfe_state_relogin_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001b_ee30, 0x001e_0f10)?;
        c.g[V1][0] = sx(0x47_0000 - 0x7fb0);
        c.g[V0][0] = c.g[S0][0];
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRelogin::~cFEStateRelogin(this)` (0x1e0f30): vtable `0x468050`
/// (`0x470000 - 0x7fb0`) at `+0x8`, then the login destructor
/// `cFEStateLogin_dtor` (0x1bee78), which takes `a1` as the caller left it.
pub fn cfe_state_relogin_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc965_vtable_dtor(c, 0x7fb0, 0x001b_ee78, 0x001e_0f4c)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA, GAME_GP};

    /// The FE popup state object every case builds (`this`).
    pub(crate) const OC965_THIS: u32 = DATA + 0x1_0000;

    /// The globals the NetFEState code reads through `gp`: the BELibrary dummy
    /// and the world with its vtable-backed fields.
    pub(crate) fn oc965_globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_c000);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x6_8000);
        c.bus.write32(DATA + 0x6_8000 + 0x8c, DATA + 0x6_9000);
        c.bus.write32(DATA + 0x6_9000 + 0x4, DATA + 0x6_9100);
        c.bus.write16(DATA + 0x6_9100 + 0x20, 0x10);
        c.bus.write32(DATA + 0x6_9100 + 0x24, fakes::FAKE + 0x10);
    }

    /// An empty message list at the net FE state `this`: both heads point at
    /// the sentinel node, which points at itself.
    pub(crate) fn oc965_empty_net_state(c: &mut Cpu, this: u32) {
        let head = DATA + 0x2_0000;
        c.bus.write32(this + 0x6cc, head);
        c.bus.write32(this + 0x6c8 + 0x4, head);
        c.bus.write32(head, head);
        c.bus.write32(head + 0x4, head);
    }

    /// One case: the machine, the globals, an empty net state at
    /// [`OC965_THIS`], `a0 = this`, and `a1` alternating with the seed.
    pub(crate) fn oc965_case(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let _r = fakes::machine(c, seed, salt, 3);
        oc965_globals(c);
        oc965_empty_net_state(c, OC965_THIS);
        c.pc = entry;
        c.g[A0][0] = sx(OC965_THIS);
        c.g[A1][0] = (seed % 2) as u64;
    }

    #[test]
    fn cfe_state_create_step2_dtor_matches() {
        check(
            ssx3_ee::s04::f_001e0da8__generated,
            cfe_state_create_step2_dtor,
            40,
            &|c, seed| oc965_case(c, seed, 0x0da8, 0x001e_0da8),
        );
    }

    #[test]
    fn cfe_state_create_duplicate_ctor_2_matches() {
        check(
            ssx3_ee::s04::f_001e0dd8__generated,
            cfe_state_create_duplicate_ctor_2,
            40,
            &|c, seed| oc965_case(c, seed, 0x0dd8, 0x001e_0dd8),
        );
    }

    #[test]
    fn cfe_state_create_duplicate_dtor_matches() {
        check(
            ssx3_ee::s04::f_001e0e18__generated,
            cfe_state_create_duplicate_dtor,
            40,
            &|c, seed| oc965_case(c, seed, 0x0e18, 0x001e_0e18),
        );
    }

    #[test]
    fn cfe_state_login_ctor_2_matches() {
        check(
            ssx3_ee::s04::f_001e0e48__generated,
            cfe_state_login_ctor_2,
            40,
            &|c, seed| {
                oc965_case(c, seed, 0x0e48, 0x001e_0e48);
                // The login state's `+0x6d0` is written by the body; start it
                // non-zero so the store is visible.
                c.bus.write32(OC965_THIS + 0x6d0, DATA + 0x3_0000);
            },
        );
    }

    #[test]
    fn cfe_state_stats_dtor_matches() {
        check(
            ssx3_ee::s04::f_001e0e98__generated,
            cfe_state_stats_dtor,
            40,
            &|c, seed| oc965_case(c, seed, 0x0e98, 0x001e_0e98),
        );
    }

    #[test]
    fn cfe_state_net_news_dtor_matches() {
        check(
            ssx3_ee::s04::f_001e0ec8__generated,
            cfe_state_net_news_dtor,
            40,
            &|c, seed| oc965_case(c, seed, 0x0ec8, 0x001e_0ec8),
        );
    }

    #[test]
    fn cfe_state_relogin_ctor_2_matches() {
        check(
            ssx3_ee::s04::f_001e0ef8__generated,
            cfe_state_relogin_ctor_2,
            40,
            &|c, seed| oc965_case(c, seed, 0x0ef8, 0x001e_0ef8),
        );
    }

    #[test]
    fn cfe_state_relogin_dtor_matches() {
        check(
            ssx3_ee::s04::f_001e0f30__generated,
            cfe_state_relogin_dtor,
            40,
            &|c, seed| {
                oc965_case(c, seed, 0x0f30, 0x001e_0f30);
                // The login destructor frees a non-zero `+0x6d0`; keep it zero.
                c.bus.write32(OC965_THIS + 0x6d0, 0);
            },
        );
    }
}
