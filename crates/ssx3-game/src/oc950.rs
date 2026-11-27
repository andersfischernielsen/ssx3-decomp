//! Staging module oc950 (wave wave256): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc950`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::fe::festateoptions::{
    fe_options_dtor, fe_options_ui_ctor, fe_vptr, FE_UI_STATE_CTOR, FE_UI_STATE_DTOR,
};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_f780,
        cfe_state_lobby_menu_ctor_2,
        "cFEStateLobbyMenu_ctor_2",
    ),
    (
        0x001d_f828,
        cfe_state_net_create_room_popup_ctor_2,
        "cFEStateNetCreateRoomPopup_ctor_2",
    ),
    (
        0x001d_f900,
        cfe_state_net_help_popup_ctor_2,
        "cFEStateNetHelpPopup_ctor_2",
    ),
    (
        0x001d_f960,
        cfe_state_net_help_popup_dtor,
        "cFEStateNetHelpPopup_dtor",
    ),
    (
        0x001d_f9d8,
        cfe_state_reward_gallery_ctor_2,
        "cFEStateRewardGallery_ctor_2",
    ),
    (
        0x001d_fac8,
        cfe_state_reward_gallery_v29,
        "cFEStateRewardGallery_v29",
    ),
    (
        0x001d_faf0,
        cfe_state_reward_gallery_posters_ctor_2,
        "cFEStateRewardGalleryPosters_ctor_2",
    ),
    (
        0x001d_fbc0,
        cfe_state_reward_gallery_trading_cards_ctor_2,
        "cFEStateRewardGalleryTradingCards_ctor_2",
    ),
];

/// `NetFEState_ctor(this, a1, a2)` (0x1a8500): the net FE state base
/// constructor the two net popups call first.
pub(crate) const NET_FE_STATE_CTOR_1A8500: u32 = 0x001a_8500;

/// `cFEStateLobbyMenu::cFEStateLobbyMenu` (0x1df780, the `_ctor_2` alias):
/// runs the net FE state constructor on `this` (a1 = a2 = 0), then stores the
/// lobby menu vtable at +8 and the two `gp`-relative pointers loaded from
/// `gp + 0xda0` at +0xb80 and +0x76c.
pub fn cfe_state_lobby_menu_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, NET_FE_STATE_CTOR_1A8500, 0x001d_f79c)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x5808));
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sw(this.wrapping_add(0xb80), c.g[A0][0]);
        c.sw(this.wrapping_add(0x76c), c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetCreateRoomPopup::cFEStateNetCreateRoomPopup` (0x1df828, the
/// `_ctor_2` alias): the net FE state constructor on `this`, then the create
/// room popup vtable at +8 and three `gp`-relative pointers at +0x6e4, +0x6dc
/// and +0x6e0.
pub fn cfe_state_net_create_room_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, NET_FE_STATE_CTOR_1A8500, 0x001d_f844)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x5918));
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sw(this.wrapping_add(0x6e4), c.g[A0][0]);
        c.sw(this.wrapping_add(0x6dc), c.g[A0][0]);
        c.sw(this.wrapping_add(0x6e0), c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetHelpPopup::cFEStateNetHelpPopup` (0x1df900, the `_ctor_2`
/// alias): the `UIState` constructor on `this` (a1 = 0), then the help popup
/// vtable at +8, then a countdown from 6 that runs until it reaches -1. Each
/// pass is a loop back edge with an event check.
pub fn cfe_state_net_help_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, FE_UI_STATE_CTOR, 0x001d_f918)?;
        c.g[V0][0] = fe_vptr(0x59e8);
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 6;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x8), c.g[V0][0]);
        loop {
            c.cycles += 7;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let again = c.g[V1][0] != c.g[A0][0];
            c.g[V0][0] = c.g[S0][0];
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_f930);
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetHelpPopup::~cFEStateNetHelpPopup` (0x1df960): stores the help
/// popup vtable at +8, then runs the `UIState` destructor on `this`.
pub fn cfe_state_net_help_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_f97c, 0x59e8)
    })
}

/// `cFEStateRewardGallery::cFEStateRewardGallery` (0x1df9d8, the `_ctor_2`
/// alias): the `UIState` constructor on `this`, then the reward gallery
/// vtable at +8.
pub fn cfe_state_reward_gallery_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_f9f0, 0x5c38))
}

/// `cFEStateRewardGallery_v29` (0x1dfac8): an empty virtual that only returns.
pub fn cfe_state_reward_gallery_v29(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateRewardGalleryPosters::cFEStateRewardGalleryPosters` (0x1dfaf0,
/// the `_ctor_2` alias): the `UIState` constructor on `this`, then the posters
/// vtable at +8.
pub fn cfe_state_reward_gallery_posters_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_fb08, 0x5db8))
}

/// `cFEStateRewardGalleryTradingCards::cFEStateRewardGalleryTradingCards`
/// (0x1dfbc0, the `_ctor_2` alias): the `UIState` constructor on `this`, then
/// the trading cards vtable at +8.
pub fn cfe_state_reward_gallery_trading_cards_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_fbd8, 0x5f38))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered};
    use crate::difftest::{check, DATA};
    use crate::game::fe::festateoptions::{fe_setup, FE_SEEDS};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word and magic of this module's fake of `NetFEState_ctor`.
    const OC950_NET_GATE: u32 = DATA + 0xf_2000;
    const OC950_NET_MAGIC: u32 = 0x0950_0c3d;

    /// The slot's body before this module's fake (the generated body unless
    /// another fake chained on first).
    static OC950_PREV_NET_CTOR: AtomicUsize = AtomicUsize::new(0);

    /// Logs the `NetFEState_ctor` call (registers, cycles, target) and returns
    /// to `ra`; with the gate off, runs the body the slot held before.
    fn oc950_fake_net_ctor(c: &mut Cpu) {
        if c.bus.read32(OC950_NET_GATE) != OC950_NET_MAGIC {
            return fakes::fall_back(c, &OC950_PREV_NET_CTOR, ee::s03::f_001a8500__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, NET_FE_STATE_CTOR_1A8500, ra);
    }

    /// The `fe_setup` machine (the `UIState` fakes are gated by it), with this
    /// module's net constructor fake hooked and its gate set.
    fn oc950_setup(c: &mut Cpu, seed: u32, entry: u32, salt: u64) {
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A8500),
            oc950_fake_net_ctor,
            &OC950_PREV_NET_CTOR,
        );
        let _ = fe_setup(c, seed, entry, salt);
        c.bus.write32(OC950_NET_GATE, OC950_NET_MAGIC);
    }

    #[test]
    fn cfe_state_lobby_menu_ctor_2_matches() {
        covered(
            ee::s04::f_001df780__generated,
            cfe_state_lobby_menu_ctor_2,
            FE_SEEDS,
            &|c, seed| oc950_setup(c, seed, 0x001d_f780, 1),
        );
    }

    #[test]
    fn cfe_state_net_create_room_popup_ctor_2_matches() {
        covered(
            ee::s04::f_001df828__generated,
            cfe_state_net_create_room_popup_ctor_2,
            FE_SEEDS,
            &|c, seed| oc950_setup(c, seed, 0x001d_f828, 2),
        );
    }

    #[test]
    fn cfe_state_net_help_popup_ctor_2_matches() {
        covered(
            ee::s04::f_001df900__generated,
            cfe_state_net_help_popup_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                let _ = fe_setup(c, seed, 0x001d_f900, 3);
            },
        );
    }

    #[test]
    fn cfe_state_net_help_popup_dtor_matches() {
        covered(
            ee::s04::f_001df960__generated,
            cfe_state_net_help_popup_dtor,
            FE_SEEDS,
            &|c, seed| {
                let _ = fe_setup(c, seed, 0x001d_f960, 4);
            },
        );
    }

    #[test]
    fn cfe_state_reward_gallery_ctor_2_matches() {
        covered(
            ee::s04::f_001df9d8__generated,
            cfe_state_reward_gallery_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                let _ = fe_setup(c, seed, 0x001d_f9d8, 5);
            },
        );
    }

    #[test]
    fn cfe_state_reward_gallery_v29_matches() {
        check(
            ee::s04::f_001dfac8__generated,
            cfe_state_reward_gallery_v29,
            FE_SEEDS,
            &|c, seed| {
                let _ = fakes::machine(c, seed, 6, 0);
            },
        );
    }

    #[test]
    fn cfe_state_reward_gallery_posters_ctor_2_matches() {
        covered(
            ee::s04::f_001dfaf0__generated,
            cfe_state_reward_gallery_posters_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                let _ = fe_setup(c, seed, 0x001d_faf0, 7);
            },
        );
    }

    #[test]
    fn cfe_state_reward_gallery_trading_cards_ctor_2_matches() {
        covered(
            ee::s04::f_001dfbc0__generated,
            cfe_state_reward_gallery_trading_cards_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                let _ = fe_setup(c, seed, 0x001d_fbc0, 8);
            },
        );
    }
}
