//! Staging module oc1015 (wave wave261): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1015`.

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
        0x0022_21c8,
        ov_screen_rewards_list_popup_dtor,
        "OVScreenRewardsListPopup_dtor",
    ),
    (
        0x0022_21f0,
        ov_screen_reward_list_dtor,
        "OVScreenRewardList_dtor",
    ),
    (
        0x0022_2218,
        ov_screen_reward_list_ctor,
        "OVScreenRewardList_ctor",
    ),
    (
        0x0022_2280,
        ov_screen_autosave_dtor,
        "OVScreenAutosave_dtor",
    ),
    (
        0x0022_22b8,
        ov_screen_autosave_ctor_2,
        "OVScreenAutosave_ctor_2",
    ),
    (0x0022_22f8, ov_screen_trophy_ctor, "OVScreenTrophy_ctor"),
    (
        0x0022_2358,
        ov_screen_trophy_ctor_2,
        "OVScreenTrophy_ctor_2",
    ),
    (0x0022_23c0, ov_screen_trophy_dtor, "OVScreenTrophy_dtor"),
];

/// Data page `0x470000`: the overlay vtables sit at `+0x4e08` (`cOVState`),
/// `+0x2108` (`cOVState_REWARDS`), `+0x1ea8` (`cOVStateTrophy`) and `+0x1f80`
/// (`cOVState_AUTOSAVE`).
const OC1015_DATA: u32 = 0x0047_0000;
const OC1015_VT_BASE: u32 = OC1015_DATA + 0x4e08;
const OC1015_VT_REWARDS: u32 = OC1015_DATA + 0x2108;
const OC1015_VT_TROPHY: u32 = OC1015_DATA + 0x1ea8;
const OC1015_VT_AUTOSAVE: u32 = OC1015_DATA + 0x1f80;
/// The game-image data page (`0x4a0000`) holding `str_sp_b[0]` at +0x2088.
const OC1015_DATA_PAGE: u32 = 0x004a_0000;
/// `UIState_ctor` (0x39e2a0) and `UIState_dtor` (0x39e390): the base overlay
/// state constructor and destructor.
const OC1015_UI_CTOR: u32 = 0x0039_e2a0;
const OC1015_UI_DTOR: u32 = 0x0039_e390;
/// `cFEMemCard` constructor (0x1d53b0, the autosave state's base) and destructor
/// (0x1d5428).
const OC1015_FEMEM_CTOR: u32 = 0x001d_53b0;
const OC1015_FEMEM_DTOR: u32 = 0x001d_5428;

/// The destructor shape of the overlay states (frame 0x10, holding `ra`):
/// `this` (a0, copied to v1) gets the vtable `vptr` at +0x8, then `callee`
/// runs on `this` (a0 still `this`) and returns to `ret`.
fn oc1015_dtor(c: &mut Cpu, vptr: u32, callee: u32, ret: u32) -> Flow {
    c.cycles += 10;
    let f = Frame::open(c, 0x10);
    f.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = vptr as u64;
    let this = c.g[V1][0] as u32;
    let value = c.g[V0][0];
    Addr(this.wrapping_add(0x8)).sw(c, value);
    call_ptr(c, callee, ret)?;
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// The two-register constructor shape of the reward list and the trophy
/// screen (frame 0x30, `s0` = this, `s1` = the state word in `a2`): the base
/// `UIState` constructor, which returns to `call_ret`, then the state word at
/// +0x98, the base vtable, the flags at +0x48/+0x4c/+0x54 cleared, the final
/// vtable `vptr` at +0x8 and the chat halfword at +0x58.
fn oc1015_ctor2(c: &mut Cpu, call_ret: u32, vptr: u32) -> Flow {
    c.cycles += 26;
    let f = Frame::open(c, 0x30);
    f.save(c, S0, 0x20);
    c.g[S0][0] = c.g[A0][0];
    f.save(c, S1, 0x10);
    f.save_ra(c, 0);
    c.g[S1][0] = c.g[A2][0];
    call_ptr(c, OC1015_UI_CTOR, call_ret)?;
    let this = c.g[S0][0] as u32;
    c.g[V1][0] = OC1015_DATA as u64;
    let state = c.g[S1][0];
    Addr(this.wrapping_add(0x98)).sw(c, state);
    Addr(this.wrapping_add(0x48)).sw(c, 0);
    c.g[V1][0] = OC1015_VT_BASE as u64;
    let base_vt = c.g[V1][0];
    Addr(this.wrapping_add(0x8)).sw(c, base_vt);
    c.g[A1][0] = OC1015_DATA_PAGE as u64;
    Addr(this.wrapping_add(0x4c)).sw(c, 0);
    c.g[V1][0] = OC1015_DATA as u64;
    Addr(this.wrapping_add(0x54)).sw(c, 0);
    c.g[V1][0] = vptr as u64;
    c.g[V0][0] = c.g[S0][0];
    f.restore(c, S1, 0x10);
    c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
    let final_vt = c.g[V1][0];
    Addr(this.wrapping_add(0x8)).sw(c, final_vt);
    let chat = c.g[A0][0];
    Addr(this.wrapping_add(0x58)).sh(c, chat);
    f.restore(c, S0, 0x20);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `OVScreenRewardsListPopup_dtor` (0x2221c8): the rewards-list popup's
/// destructor. Sets the base `cOVState` vtable at +0x8, then `UIState_dtor`.
pub fn ov_screen_rewards_list_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc1015_dtor(c, OC1015_VT_BASE, OC1015_UI_DTOR, 0x0022_21e4)
    });
}

/// `OVScreenRewardList_dtor` (0x2221f0): the reward list's destructor, the
/// same as the popup's.
pub fn ov_screen_reward_list_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc1015_dtor(c, OC1015_VT_BASE, OC1015_UI_DTOR, 0x0022_220c)
    });
}

/// `OVScreenRewardList_ctor` (0x222218): constructs the reward list overlay
/// state in `a0` with the state word from `a2`, vtable `cOVState_REWARDS`.
pub fn ov_screen_reward_list_ctor(c: &mut Cpu) {
    guest(c, |c| oc1015_ctor2(c, 0x0022_2234, OC1015_VT_REWARDS));
}

/// `OVScreenAutosave_dtor` (0x222280): the autosave overlay's destructor. Sets
/// the autosave vtable at +0x8, then runs the memory-card state's destructor
/// (0x1d5428).
pub fn ov_screen_autosave_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc1015_dtor(c, OC1015_VT_AUTOSAVE, OC1015_FEMEM_DTOR, 0x0022_229c)
    });
}

/// `OVScreenAutosave_ctor_2` (0x2222b8): constructs the autosave overlay state
/// on top of the memory-card state constructor (0x1d53b0, with null `a1` and
/// `a2`), then sets the autosave vtable at +0x8.
pub fn ov_screen_autosave_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OC1015_FEMEM_CTOR, 0x0022_22d4)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = OC1015_DATA as u64;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = OC1015_VT_AUTOSAVE as u64;
        f.restore_ra(c, 0);
        let vt = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vt);
        f.restore(c, S0, 0x10);
        f.close(c);
        Ok(())
    });
}

/// `OVScreenTrophy_ctor` (0x2222f8): constructs the trophy overlay state in
/// `a0` with overlay id 0: the base `UIState` constructor, then the flags,
/// the base vtable, the trophy vtable and the chat halfword.
pub fn ov_screen_trophy_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, S0, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OC1015_UI_CTOR, 0x0022_2310)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = OC1015_DATA as u64;
        Addr(this.wrapping_add(0x98)).sw(c, 0);
        c.g[V1][0] = OC1015_VT_BASE as u64;
        Addr(this.wrapping_add(0x48)).sw(c, 0);
        let base_vt = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, base_vt);
        c.g[A1][0] = OC1015_DATA_PAGE as u64;
        Addr(this.wrapping_add(0x4c)).sw(c, 0);
        c.g[V1][0] = OC1015_DATA as u64;
        Addr(this.wrapping_add(0x54)).sw(c, 0);
        c.g[V1][0] = OC1015_VT_TROPHY as u64;
        c.g[V0][0] = c.g[S0][0];
        f.restore_ra(c, 0);
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
        let final_vt = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, final_vt);
        let chat = c.g[A0][0];
        Addr(this.wrapping_add(0x58)).sh(c, chat);
        f.restore(c, S0, 0x10);
        f.close(c);
        Ok(())
    });
}

/// `OVScreenTrophy_ctor_2` (0x222358): the trophy overlay constructor with the
/// state word from `a2`, the same shape as the reward list's.
pub fn ov_screen_trophy_ctor_2(c: &mut Cpu) {
    guest(c, |c| oc1015_ctor2(c, 0x0022_2374, OC1015_VT_TROPHY));
}

/// `OVScreenTrophy_dtor` (0x2223c0): the trophy overlay's destructor, the same
/// as the base overlay state's.
pub fn ov_screen_trophy_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc1015_dtor(c, OC1015_VT_BASE, OC1015_UI_DTOR, 0x0022_23dc)
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// Random junk in every register and in the object (+0..0x100) at
    /// `DATA + 0x1000`, the object in `a0`. The constructors and destructors
    /// have no branches, so one path is the whole space; the junk makes sure
    /// every stored field is visibly written.
    fn oc1015_object_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0022_21c8 ^ seed as u64);
        scramble(c, &mut r);
        let obj = DATA + 0x1000;
        for k in 0..0x40 {
            c.bus.write32(obj + 4 * k, r.next_u32());
        }
        c.g[A0][0] = sx(obj);
    }

    #[test]
    fn ov_screen_rewards_list_popup_dtor_matches() {
        check(
            ee::s05::f_002221c8__generated,
            ov_screen_rewards_list_popup_dtor,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_reward_list_dtor_matches() {
        check(
            ee::s05::f_002221f0__generated,
            ov_screen_reward_list_dtor,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_reward_list_ctor_matches() {
        check(
            ee::s05::f_00222218__generated,
            ov_screen_reward_list_ctor,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_autosave_dtor_matches() {
        check(
            ee::s05::f_00222280__generated,
            ov_screen_autosave_dtor,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }

    /// The memory-card constructor (0x1d53b0) allocates for real, so the
    /// fixture gets the engine's allocator and the BELibrary dummy first.
    #[test]
    fn ov_screen_autosave_ctor_2_matches() {
        check(
            ee::s05::f_002222b8__generated,
            ov_screen_autosave_ctor_2,
            200,
            &|c, seed| {
                crate::difftest::fakes::machine_rt(c, seed, 0x0022_22b8, 0);
                oc1015_object_setup(c, seed)
            },
        );
    }

    #[test]
    fn ov_screen_trophy_ctor_matches() {
        check(
            ee::s05::f_002222f8__generated,
            ov_screen_trophy_ctor,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_trophy_ctor_2_matches() {
        check(
            ee::s05::f_00222358__generated,
            ov_screen_trophy_ctor_2,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_trophy_dtor_matches() {
        check(
            ee::s05::f_002223c0__generated,
            ov_screen_trophy_dtor,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }
}
