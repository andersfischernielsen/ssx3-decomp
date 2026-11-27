//! Staging module oc964 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc964`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::game::fe::festateoptions::fe_vptr;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001e_0a08,
        cfe_state_challenge_received_popup_ctor_2,
        "cFEStateChallengeReceivedPopup_ctor_2",
    ),
    (
        0x001e_0ac0,
        cfe_state_online_character_select_ctor_2,
        "cFEStateOnlineCharacterSelect_ctor_2",
    ),
    (
        0x001e_0b00,
        cfe_state_online_character_select_dtor,
        "cFEStateOnlineCharacterSelect_dtor",
    ),
    (
        0x001e_0b28,
        cfe_state_net_config_ctor_2,
        "cFEStateNetConfig_ctor_2",
    ),
    (
        0x001e_0bb0,
        net_config_auth_popup_ctor_2,
        "NetConfigAuthPopup_ctor_2",
    ),
    (
        0x001e_0c30,
        cfe_state_auth_dnas_error_ctor_2,
        "cFEStateAuthDNASError_ctor_2",
    ),
    (
        0x001e_0ca0,
        cfe_state_create_step1_ctor_2,
        "cFEStateCreateStep1_ctor_2",
    ),
    (
        0x001e_0d68,
        cfe_state_create_step2_ctor_2,
        "cFEStateCreateStep2_ctor_2",
    ),
];

/// `NetFEState_ctor(this, 0, 0)` (0x1a8500), the base constructor of the net
/// front-end states and popups.
pub(crate) const OC964_NET_BASE_CTOR: u32 = 0x001a_8500;

/// `NetFEState_dtor(this)` (0x1a85d0), the base destructor.
pub(crate) const OC964_NET_BASE_DTOR: u32 = 0x001a_85d0;

/// Shared prologue of the net constructors: `sp -= 0x20`, `s0` and `ra`
/// saved, `a1 = a2 = 0`, `s0 = this`, then the base constructor on `this`
/// returning to `ret`. The caller charges its own cycles first.
fn oc964_net_base_ctor(c: &mut Cpu, ret: u32) -> Result<Frame, Unwound> {
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = 0;
    frame.save(c, S0, 0x10);
    c.g[A2][0] = 0;
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, OC964_NET_BASE_CTOR, ret)?;
    Ok(frame)
}

/// `cFEStateChallengeReceivedPopup::cFEStateChallengeReceivedPopup` (0x1e0a08,
/// the `_ctor_2` alias): the net base constructor on `this`, then the popup
/// vtable at +8 and the `gp+0xda0` word at +0x6d0.
pub fn cfe_state_challenge_received_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = oc964_net_base_ctor(c, 0x001e_0a24)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        c.g[V1][0] = fe_vptr(0x72e0);
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sw(this.wrapping_add(0x6d0), c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOnlineCharacterSelect::cFEStateOnlineCharacterSelect` (0x1e0ac0,
/// the `_ctor_2` alias): the net base constructor on `this`, then the vtable
/// at +8 and a zero at +0x6dc.
pub fn cfe_state_online_character_select_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = oc964_net_base_ctor(c, 0x001e_0adc)?;
        c.g[V1][0] = fe_vptr(0x73f0);
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x6dc), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOnlineCharacterSelect::~cFEStateOnlineCharacterSelect` (0x1e0b00):
/// stores the vtable at +8, then runs the base destructor on the unchanged
/// `this` (a0).
pub fn cfe_state_online_character_select_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = fe_vptr(0x73f0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, OC964_NET_BASE_DTOR, 0x001e_0b1c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Fields the net config constructor clears, after its vtable store.
const OC964_NET_CONFIG_ZERO_FIELDS: [u32; 14] = [
    0x6d4, 0x6d8, 0x6dc, 0x6e4, 0x6e8, 0x6f0, 0x6f4, 0x6f8, 0x6fc, 0x700, 0x704, 0x710, 0x714,
    0x718,
];

/// `cFEStateNetConfig::cFEStateNetConfig` (0x1e0b28, the `_ctor_2` alias): the
/// net base constructor on `this`, then the vtable at +8, a zero at +0x6d0 and
/// the fourteen config words in [`OC964_NET_CONFIG_ZERO_FIELDS`].
pub fn cfe_state_net_config_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = oc964_net_base_ctor(c, 0x001e_0b44)?;
        c.g[V1][0] = fe_vptr(0x7500);
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x6d0), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        for off in OC964_NET_CONFIG_ZERO_FIELDS {
            c.sw(this.wrapping_add(off), 0);
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `NetConfigAuthPopup::NetConfigAuthPopup` (0x1e0bb0, the `_ctor_2` alias):
/// the net base constructor on `this`, then the vtable at +8, the words 2 and
/// 0x1e at +0x6f4 and +0x6f8, and zeros at +0x6d0, +0x6d4, +0x6e4 and +0x6fc.
/// The popup's `a0` and `a1` end as 0x1e and 2.
pub fn net_config_auth_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = oc964_net_base_ctor(c, 0x001e_0bcc)?;
        c.g[V1][0] = fe_vptr(0x7610);
        c.g[A1][0] = 2;
        c.g[A0][0] = 0x1e;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(this.wrapping_add(0x6f4), c.g[A1][0]);
        c.sw(this.wrapping_add(0x6f8), c.g[A0][0]);
        c.sw(this.wrapping_add(0x6d0), 0);
        c.sw(this.wrapping_add(0x6d4), 0);
        c.sw(this.wrapping_add(0x6e4), 0);
        c.sw(this.wrapping_add(0x6fc), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateAuthDNASError::cFEStateAuthDNASError` (0x1e0c30, the `_ctor_2`
/// alias): the net base constructor on `this`, then the vtable at +8 and zeros
/// at +0x6d0, +0x6d4 and +0x6d8.
pub fn cfe_state_auth_dnas_error_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = oc964_net_base_ctor(c, 0x001e_0c4c)?;
        c.g[V1][0] = fe_vptr(0x7720);
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x6d0), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sw(this.wrapping_add(0x6d4), 0);
        c.sw(this.wrapping_add(0x6d8), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateStep1::cFEStateCreateStep1` (0x1e0ca0, the `_ctor_2` alias):
/// the net base constructor on `this`, then the vtable at +8, the `gp+0xda0`
/// word at +0x6dc, +0x6d4 and +0x6d8, a 1 at +0x6ec and a zero at +0x6f0. The
/// popup's `a0` ends as 1 and `v1` as the `gp+0xda0` word.
pub fn cfe_state_create_step1_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = oc964_net_base_ctor(c, 0x001e_0cbc)?;
        c.g[V0][0] = fe_vptr(0x7940);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        c.g[A0][0] = 1;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x8), c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(this.wrapping_add(0x6dc), c.g[V1][0]);
        c.sw(this.wrapping_add(0x6ec), c.g[A0][0]);
        c.sw(this.wrapping_add(0x6d4), c.g[V1][0]);
        c.sw(this.wrapping_add(0x6d8), c.g[V1][0]);
        c.sw(this.wrapping_add(0x6f0), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateStep2::cFEStateCreateStep2` (0x1e0d68, the `_ctor_2` alias):
/// the net base constructor on `this`, then the vtable at +8. The return
/// address is restored before the vtable store, as the generated body does.
pub fn cfe_state_create_step2_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = oc964_net_base_ctor(c, 0x001e_0d84)?;
        c.g[V1][0] = fe_vptr(0x7a50);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered};
    use crate::difftest::DATA;
    use crate::game::fe::festateoptions::{fe_setup, FE_SEEDS};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate words and magics of this module's fakes of `NetFEState_ctor` and
    /// `NetFEState_dtor`.
    const OC964_NET_CTOR_GATE: u32 = DATA + 0xf_5000;
    const OC964_NET_CTOR_MAGIC: u32 = 0x0964_0c3d;
    const OC964_NET_DTOR_GATE: u32 = DATA + 0xf_5010;
    const OC964_NET_DTOR_MAGIC: u32 = 0x0964_0d7b;

    /// The slot bodies before this module's fakes (the generated bodies unless
    /// another fake chained on first).
    static OC964_PREV_NET_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC964_PREV_NET_DTOR: AtomicUsize = AtomicUsize::new(0);

    /// Logs the base constructor call and returns to `ra`; with the gate off,
    /// runs the body the slot held before.
    fn oc964_fake_net_ctor(c: &mut Cpu) {
        if c.bus.read32(OC964_NET_CTOR_GATE) != OC964_NET_CTOR_MAGIC {
            return fakes::fall_back(c, &OC964_PREV_NET_CTOR, ee::s03::f_001a8500__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, OC964_NET_BASE_CTOR, ra);
    }

    /// Logs the base destructor call and returns to `ra`; with the gate off,
    /// runs the body the slot held before.
    fn oc964_fake_net_dtor(c: &mut Cpu) {
        if c.bus.read32(OC964_NET_DTOR_GATE) != OC964_NET_DTOR_MAGIC {
            return fakes::fall_back(c, &OC964_PREV_NET_DTOR, ee::s03::f_001a85d0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, OC964_NET_BASE_DTOR, ra);
    }

    /// The popup fixture: the base constructor fake hooked and gated on, the
    /// front-end machine for the object at `a0`.
    fn oc964_ctor_setup(c: &mut Cpu, seed: u32, entry: u32, salt: u64) {
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A8500),
            oc964_fake_net_ctor,
            &OC964_PREV_NET_CTOR,
        );
        let _ = fe_setup(c, seed, entry, salt);
        c.bus.write32(OC964_NET_CTOR_GATE, OC964_NET_CTOR_MAGIC);
    }

    /// The destructor fixture: the base destructor fake hooked and gated on.
    fn oc964_dtor_setup(c: &mut Cpu, seed: u32, entry: u32, salt: u64) {
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A85D0),
            oc964_fake_net_dtor,
            &OC964_PREV_NET_DTOR,
        );
        let _ = fe_setup(c, seed, entry, salt);
        c.bus.write32(OC964_NET_DTOR_GATE, OC964_NET_DTOR_MAGIC);
    }

    #[test]
    fn cfe_state_challenge_received_popup_ctor_2_matches() {
        covered(
            ee::s04::f_001e0a08__generated,
            cfe_state_challenge_received_popup_ctor_2,
            FE_SEEDS,
            &|c, seed| oc964_ctor_setup(c, seed, 0x001e_0a08, 1),
        );
    }

    #[test]
    fn cfe_state_online_character_select_ctor_2_matches() {
        covered(
            ee::s04::f_001e0ac0__generated,
            cfe_state_online_character_select_ctor_2,
            FE_SEEDS,
            &|c, seed| oc964_ctor_setup(c, seed, 0x001e_0ac0, 2),
        );
    }

    #[test]
    fn cfe_state_online_character_select_dtor_matches() {
        covered(
            ee::s04::f_001e0b00__generated,
            cfe_state_online_character_select_dtor,
            FE_SEEDS,
            &|c, seed| oc964_dtor_setup(c, seed, 0x001e_0b00, 3),
        );
    }

    #[test]
    fn cfe_state_net_config_ctor_2_matches() {
        covered(
            ee::s04::f_001e0b28__generated,
            cfe_state_net_config_ctor_2,
            FE_SEEDS,
            &|c, seed| oc964_ctor_setup(c, seed, 0x001e_0b28, 4),
        );
    }

    #[test]
    fn net_config_auth_popup_ctor_2_matches() {
        covered(
            ee::s04::f_001e0bb0__generated,
            net_config_auth_popup_ctor_2,
            FE_SEEDS,
            &|c, seed| oc964_ctor_setup(c, seed, 0x001e_0bb0, 5),
        );
    }

    #[test]
    fn cfe_state_auth_dnas_error_ctor_2_matches() {
        covered(
            ee::s04::f_001e0c30__generated,
            cfe_state_auth_dnas_error_ctor_2,
            FE_SEEDS,
            &|c, seed| oc964_ctor_setup(c, seed, 0x001e_0c30, 6),
        );
    }

    #[test]
    fn cfe_state_create_step1_ctor_2_matches() {
        covered(
            ee::s04::f_001e0ca0__generated,
            cfe_state_create_step1_ctor_2,
            FE_SEEDS,
            &|c, seed| oc964_ctor_setup(c, seed, 0x001e_0ca0, 7),
        );
    }

    #[test]
    fn cfe_state_create_step2_ctor_2_matches() {
        covered(
            ee::s04::f_001e0d68__generated,
            cfe_state_create_step2_ctor_2,
            FE_SEEDS,
            &|c, seed| oc964_ctor_setup(c, seed, 0x001e_0d68, 8),
        );
    }
}
