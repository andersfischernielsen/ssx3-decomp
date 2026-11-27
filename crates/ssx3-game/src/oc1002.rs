//! Staging module oc1002 (wave wave260): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1002`.

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
        0x0021_cfe0,
        ov_screen_multiplayer_standings_dtor,
        "OVScreenMultiplayerStandings_dtor",
    ),
    (
        0x0021_d008,
        ov_screen_multiplayer_standings_ctor,
        "OVScreenMultiplayerStandings_ctor",
    ),
    (
        0x0021_d070,
        ov_screen_hiscore_list_ctor,
        "OVScreenHiscoreList_ctor",
    ),
    (
        0x0021_d0d0,
        ov_screen_hiscore_list_ctor_2,
        "OVScreenHiscoreList_ctor_2",
    ),
    (
        0x0021_d138,
        ov_screen_hiscore_list_dtor,
        "OVScreenHiscoreList_dtor",
    ),
    (0x0022_0d70, fe_state_lodge_dtor, "cFEStateLodge_dtor"),
    (
        0x0022_0dd0,
        fe_state_lodge_rider_detail_dtor,
        "cFEStateLodgeRiderDetail_dtor",
    ),
    (
        0x0022_0e30,
        fe_state_buy_attrib_dtor,
        "cFEStateBuyAttrib_dtor",
    ),
];

/// The one-call destructors of the menu states: `this->vtable = vtable` (the word
/// at +8), then `UIState_dtor(this)` (0x39e390) returning to `ret_to`. Frame of
/// 0x10 holding `ra`, as gcc emits for a base-class destructor.
fn oc1002_vtable_dtor(c: &mut Cpu, vtable: u32, ret_to: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x0047_0000;
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx(vtable);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, 0x0039_e390, ret_to)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// The stores that finish an `OVScreen*` constructor after its `UIState_ctor`
/// returned (`this` in s0): the overlay id at +0x98, the flags at +0x48, +0x4c and
/// +0x54 cleared, the overlay vtable (0x474e08) at +8, then the screen's own
/// vtable (`0x470000 + screen_vt_off`) at +8 and the chat-buffer init word from
/// 0x4a2088 at +0x58. Leaves `v0 = this`, `v1` the screen vtable, `a1 = 0x4a0000`.
fn oc1002_ov_screen_init(c: &mut Cpu, ov_id: u64, screen_vt_off: u32) {
    let this = c.g[S0][0] as u32;
    c.sw(this.wrapping_add(0x98), ov_id);
    c.sw(this.wrapping_add(0x48), 0);
    c.sw(
        this.wrapping_add(0x8),
        sx(0x0047_0000u32.wrapping_add(0x4e08)),
    );
    c.g[A1][0] = 0x004a_0000;
    c.sw(this.wrapping_add(0x4c), 0);
    c.sw(this.wrapping_add(0x54), 0);
    c.g[V1][0] = sx(0x0047_0000u32.wrapping_add(screen_vt_off));
    c.g[V0][0] = c.g[S0][0];
    c.g[A0][0] = Addr(0x004a_0000u32.wrapping_add(0x2088)).lhu(c);
    c.sw(this.wrapping_add(0x8), c.g[V1][0]);
    c.sh(this.wrapping_add(0x58), c.g[A0][0]);
}

/// `OVScreenMultiplayerStandings::~OVScreenMultiplayerStandings(this)` (0x21cfe0):
/// installs the overlay vtable (0x474e08) and runs the `UIState` destructor.
pub fn ov_screen_multiplayer_standings_dtor(c: &mut Cpu) {
    guest(c, |c| oc1002_vtable_dtor(c, 0x0047_4e08, 0x0021_cffc))
}

/// `OVScreenMultiplayerStandings` constructor `(this, engine, overlayId)`
/// (0x21d008): the `UIState` constructor, then the overlay id (a2) and the
/// screen's vtable (0x473c50, the tournament standings).
pub fn ov_screen_multiplayer_standings_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0021_d024)?;
        let ov_id = c.g[S1][0];
        oc1002_ov_screen_init(c, ov_id, 0x3c50);
        frame.restore(c, 17, 0x10);
        frame.restore(c, 16, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenHiscoreList` constructor `(this)` (0x21d070): the `UIState`
/// constructor with overlay id 0 (a1), then the hi-score list vtable (0x473b78).
pub fn ov_screen_hiscore_list_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x0021_d088)?;
        oc1002_ov_screen_init(c, 0, 0x3b78);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenHiscoreList` constructor `(this, engine, overlayId)` (0x21d0d0):
/// as the multiplayer one, with the hi-score list vtable (0x473b78).
pub fn ov_screen_hiscore_list_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0021_d0ec)?;
        let ov_id = c.g[S1][0];
        oc1002_ov_screen_init(c, ov_id, 0x3b78);
        frame.restore(c, 17, 0x10);
        frame.restore(c, 16, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenHiscoreList::~OVScreenHiscoreList(this)` (0x21d138): installs the
/// overlay vtable (0x474e08) and runs the `UIState` destructor.
pub fn ov_screen_hiscore_list_dtor(c: &mut Cpu) {
    guest(c, |c| oc1002_vtable_dtor(c, 0x0047_4e08, 0x0021_d154))
}

/// `cFEStateLodge::~cFEStateLodge(this)` (0x220d70): installs the front-end
/// state vtable (0x46d0d0) and runs the `UIState` destructor.
pub fn fe_state_lodge_dtor(c: &mut Cpu) {
    guest(c, |c| oc1002_vtable_dtor(c, 0x0046_d0d0, 0x0022_0d8c))
}

/// `cFEStateLodgeRiderDetail::~cFEStateLodgeRiderDetail(this)` (0x220dd0): the
/// same shape as [`fe_state_lodge_dtor`] (vtable 0x46d0d0, return 0x220dec).
pub fn fe_state_lodge_rider_detail_dtor(c: &mut Cpu) {
    guest(c, |c| oc1002_vtable_dtor(c, 0x0046_d0d0, 0x0022_0dec))
}

/// `cFEStateBuyAttrib::~cFEStateBuyAttrib(this)` (0x220e30): the same shape as
/// [`fe_state_lodge_dtor`] (vtable 0x46d0d0, return 0x220e4c).
pub fn fe_state_buy_attrib_dtor(c: &mut Cpu) {
    guest(c, |c| oc1002_vtable_dtor(c, 0x0046_d0d0, 0x0022_0e4c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{DATA, RA as DT_RA};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (its own machine RAM only).
    const OC1002_CTRL: u32 = DATA + 0x20_0000;
    const OC1002_MAGIC: u32 = 0x1002_6a7e;
    /// The screen object the constructors and destructors write into.
    const OC1002_OBJ: u32 = DATA + 0x20_1000;

    static OC1002_PREV_UI_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC1002_PREV_UI_DTOR: AtomicUsize = AtomicUsize::new(0);

    /// `UIState_ctor` (0x39e2a0) while the gate is armed: logs and returns.
    fn oc1002_fake_ui_ctor(c: &mut Cpu) {
        if c.bus.read32(OC1002_CTRL) != OC1002_MAGIC {
            return fakes::fall_back(c, &OC1002_PREV_UI_CTOR, ee::s13::f_0039e2a0__generated);
        }
        fakes::fake_call(c, 0x0039_e2a0);
    }

    /// `UIState_dtor` (0x39e390) while the gate is armed: logs and returns.
    fn oc1002_fake_ui_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1002_CTRL) != OC1002_MAGIC {
            return fakes::fall_back(c, &OC1002_PREV_UI_DTOR, ee::s13::f_0039e390__generated);
        }
        fakes::fake_call(c, 0x0039_e390);
    }

    fn oc1002_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc1002_fake_ui_ctor,
            &OC1002_PREV_UI_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc1002_fake_ui_dtor,
            &OC1002_PREV_UI_DTOR,
        );
    }

    fn oc1002_arm(c: &mut Cpu) {
        c.bus.write32(OC1002_CTRL, OC1002_MAGIC);
    }

    /// `OVScreenMultiplayerStandings_dtor` (0x21cfe0): vtable store, `UIState_dtor` call.
    #[test]
    fn ov_screen_multiplayer_standings_dtor_matches() {
        oc1002_install_fakes();
        fakes::covered(
            ee::s05::f_0021cfe0__generated,
            ov_screen_multiplayer_standings_dtor,
            80,
            &|c, seed| {
                let _ = machine(c, seed, 0x1002_0001, 1);
                oc1002_arm(c);
                c.g[A0][0] = sx(OC1002_OBJ);
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    /// `OVScreenMultiplayerStandings_ctor` (0x21d008): `UIState_ctor`, then the stores.
    #[test]
    fn ov_screen_multiplayer_standings_ctor_matches() {
        oc1002_install_fakes();
        fakes::covered(
            ee::s05::f_0021d008__generated,
            ov_screen_multiplayer_standings_ctor,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1002_0008, 1);
                oc1002_arm(c);
                c.g[A0][0] = sx(OC1002_OBJ);
                c.g[A2][0] = sx(r.next_u32());
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    /// `OVScreenHiscoreList_ctor` (0x21d070): `UIState_ctor` with a1 = 0.
    #[test]
    fn ov_screen_hiscore_list_ctor_matches() {
        oc1002_install_fakes();
        fakes::covered(
            ee::s05::f_0021d070__generated,
            ov_screen_hiscore_list_ctor,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1002_0070, 1);
                oc1002_arm(c);
                c.g[A0][0] = sx(OC1002_OBJ);
                c.g[A1][0] = sx(r.next_u32());
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    /// `OVScreenHiscoreList_ctor_2` (0x21d0d0): `UIState_ctor` with the overlay id from a2.
    #[test]
    fn ov_screen_hiscore_list_ctor_2_matches() {
        oc1002_install_fakes();
        fakes::covered(
            ee::s05::f_0021d0d0__generated,
            ov_screen_hiscore_list_ctor_2,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1002_00d0, 1);
                oc1002_arm(c);
                c.g[A0][0] = sx(OC1002_OBJ);
                c.g[A2][0] = sx(r.next_u32());
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    /// `OVScreenHiscoreList_dtor` (0x21d138): vtable store, `UIState_dtor` call.
    #[test]
    fn ov_screen_hiscore_list_dtor_matches() {
        oc1002_install_fakes();
        fakes::covered(
            ee::s05::f_0021d138__generated,
            ov_screen_hiscore_list_dtor,
            80,
            &|c, seed| {
                let _ = machine(c, seed, 0x1002_0138, 1);
                oc1002_arm(c);
                c.g[A0][0] = sx(OC1002_OBJ);
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    /// `cFEStateLodge_dtor` (0x220d70): vtable store, `UIState_dtor` call.
    #[test]
    fn fe_state_lodge_dtor_matches() {
        oc1002_install_fakes();
        fakes::covered(
            ee::s05::f_00220d70__generated,
            fe_state_lodge_dtor,
            80,
            &|c, seed| {
                let _ = machine(c, seed, 0x1002_0d70, 1);
                oc1002_arm(c);
                c.g[A0][0] = sx(OC1002_OBJ);
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    /// `cFEStateLodgeRiderDetail_dtor` (0x220dd0): vtable store, `UIState_dtor` call.
    #[test]
    fn fe_state_lodge_rider_detail_dtor_matches() {
        oc1002_install_fakes();
        fakes::covered(
            ee::s05::f_00220dd0__generated,
            fe_state_lodge_rider_detail_dtor,
            80,
            &|c, seed| {
                let _ = machine(c, seed, 0x1002_0dd0, 1);
                oc1002_arm(c);
                c.g[A0][0] = sx(OC1002_OBJ);
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    /// `cFEStateBuyAttrib_dtor` (0x220e30): vtable store, `UIState_dtor` call.
    #[test]
    fn fe_state_buy_attrib_dtor_matches() {
        oc1002_install_fakes();
        fakes::covered(
            ee::s05::f_00220e30__generated,
            fe_state_buy_attrib_dtor,
            80,
            &|c, seed| {
                let _ = machine(c, seed, 0x1002_0e30, 1);
                oc1002_arm(c);
                c.g[A0][0] = sx(OC1002_OBJ);
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }
}
