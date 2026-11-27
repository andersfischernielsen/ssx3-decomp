//! Staging module oc951 (wave wave256): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc951`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::game::fe::feasyncfile::fe_ctor_with_vtable;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_fc90,
        cfe_state_reward_gallery_concept_arts_ctor_2,
        "cFEStateRewardGalleryConceptArts_ctor_2",
    ),
    (
        0x001d_fd60,
        cfe_state_reward_gallery_videos_ctor_2,
        "cFEStateRewardGalleryVideos_ctor_2",
    ),
    (
        0x001d_fe30,
        cfe_state_reward_gallery_toys_ctor_2,
        "cFEStateRewardGalleryToys_ctor_2",
    ),
    (
        0x001d_ff00,
        cfe_state_reward_gallery_cheat_chars_ctor_2,
        "cFEStateRewardGalleryCheatChars_ctor_2",
    ),
    (
        0x001d_ffd0,
        cfe_state_preview_reward_ctor_2,
        "cFEStatePreviewReward_ctor_2",
    ),
    (
        0x001e_0098,
        cfe_state_mountain_room_ctor_2,
        "cFEStateMountainRoom_ctor_2",
    ),
    (
        0x001e_00e0,
        cfe_state_mountain_room_hide_widget48,
        "cFEStateMountainRoom_HideWidget48",
    ),
    (
        0x001e_0178,
        cfe_state_peak_room_ctor_2,
        "cFEStatePeakRoom_ctor_2",
    ),
];

/// `cFEStateRewardGalleryConceptArts` constructor (0x1dfc90): the `UIState`
/// base constructor (0x39e2a0, `a1 = 0`), then the screen's vtable
/// `0x469f48` at `this + 8`. Returns `this`.
pub fn cfe_state_reward_gallery_concept_arts_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(0x0039_e2a0),
            0x001d_fca8,
            0x60b8,
        )
    });
}

/// `cFEStateRewardGalleryVideos` constructor (0x1dfd60): as the concept-arts
/// one, with the vtable `0x469dc8`.
pub fn cfe_state_reward_gallery_videos_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(0x0039_e2a0),
            0x001d_fd78,
            0x6238,
        )
    });
}

/// `cFEStateRewardGalleryToys` constructor (0x1dfe30): as the concept-arts
/// one, with the vtable `0x469c48`.
pub fn cfe_state_reward_gallery_toys_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(0x0039_e2a0),
            0x001d_fe48,
            0x63b8,
        )
    });
}

/// `cFEStateRewardGalleryCheatChars` constructor (0x1dff00): as the
/// concept-arts one, with the vtable `0x469ac8`.
pub fn cfe_state_reward_gallery_cheat_chars_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(0x0039_e2a0),
            0x001d_ff18,
            0x6538,
        )
    });
}

/// `cFEStatePreviewReward` constructor (0x1dffd0): as the concept-arts one,
/// with the vtable `0x4699f8`.
pub fn cfe_state_preview_reward_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(0x0039_e2a0),
            0x001d_ffe8,
            0x6608,
        )
    });
}

/// `cFEStateMountainRoom` constructor (0x1e0098): as the concept-arts one,
/// with the vtable `0x469858`.
pub fn cfe_state_mountain_room_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(0x0039_e2a0),
            0x001e_00b0,
            0x67a8,
        )
    });
}

/// `cFEStateMountainRoom_hidePass` (0x1e00e0): `(*(cUIThing**)(this + 0x48))
/// ->setVisible(0)`. The widget is `a2`; its vtable is at `+8`; the slot
/// function at vtable `+0x4c` is called with `a0 = widget + s16(vtable+0x48)`
/// and `a1 = 0`.
pub fn cfe_state_mountain_room_hide_widget48(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x48)).lw(c);
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001e_0104)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStatePeakRoom` constructor (0x1e0178): as the concept-arts one, with
/// the vtable `0x469788`.
pub fn cfe_state_peak_room_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(0x0039_e2a0),
            0x001e_0190,
            0x6878,
        )
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    /// Compare a constructor port with its generated body: the fixture is a
    /// scrambled machine with `this` at `DATA + 0x2000`.
    pub(crate) fn oc951_ctor_case(generated: GuestFn, hand: GuestFn, entry: u32) {
        check(generated, hand, 200, &|c, seed| {
            machine(c, seed, entry as u64, 0);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.pc = entry;
        });
    }

    #[test]
    fn cfe_state_reward_gallery_concept_arts_ctor_2_matches() {
        oc951_ctor_case(
            ee::s04::f_001dfc90__generated,
            cfe_state_reward_gallery_concept_arts_ctor_2,
            0x001d_fc90,
        );
    }

    #[test]
    fn cfe_state_reward_gallery_videos_ctor_2_matches() {
        oc951_ctor_case(
            ee::s04::f_001dfd60__generated,
            cfe_state_reward_gallery_videos_ctor_2,
            0x001d_fd60,
        );
    }

    #[test]
    fn cfe_state_reward_gallery_toys_ctor_2_matches() {
        oc951_ctor_case(
            ee::s04::f_001dfe30__generated,
            cfe_state_reward_gallery_toys_ctor_2,
            0x001d_fe30,
        );
    }

    #[test]
    fn cfe_state_reward_gallery_cheat_chars_ctor_2_matches() {
        oc951_ctor_case(
            ee::s04::f_001dff00__generated,
            cfe_state_reward_gallery_cheat_chars_ctor_2,
            0x001d_ff00,
        );
    }

    #[test]
    fn cfe_state_preview_reward_ctor_2_matches() {
        oc951_ctor_case(
            ee::s04::f_001dffd0__generated,
            cfe_state_preview_reward_ctor_2,
            0x001d_ffd0,
        );
    }

    #[test]
    fn cfe_state_mountain_room_ctor_2_matches() {
        oc951_ctor_case(
            ee::s04::f_001e0098__generated,
            cfe_state_mountain_room_ctor_2,
            0x001e_0098,
        );
    }

    #[test]
    fn cfe_state_peak_room_ctor_2_matches() {
        oc951_ctor_case(
            ee::s04::f_001e0178__generated,
            cfe_state_peak_room_ctor_2,
            0x001e_0178,
        );
    }

    /// `hidePass` through a widget whose vtable slot is the real leaf at
    /// `0x1406c0` (returns 0). The vtable's `s16` at `+0x48` varies in sign
    /// and value, so the widget pointer moves with it.
    #[test]
    fn cfe_state_mountain_room_hide_widget48_matches() {
        check(
            ee::s04::f_001e00e0__generated,
            cfe_state_mountain_room_hide_widget48,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001e_00e0, 0);
                let widget = DATA + 0x3000;
                let vtable = DATA + 0x3100;
                let off = if seed % 2 == 0 {
                    r.next_u32() & 0x7fff
                } else {
                    0x8000 | (r.next_u32() & 0x7fff)
                };
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.bus.write32(DATA + 0x2048, widget);
                c.bus.write32(widget + 0x8, vtable);
                c.bus.write16(vtable + 0x48, off as u16);
                c.bus.write32(vtable + 0x4c, 0x0014_06c0);
                c.pc = 0x001e_00e0;
            },
        );
    }
}
