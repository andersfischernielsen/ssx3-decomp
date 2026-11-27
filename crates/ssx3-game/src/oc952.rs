//! Staging module oc952 (wave wave256): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc952`.

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
        0x001e_01b0,
        cfe_state_peak_room_dtor,
        "cFEStatePeakRoom_dtor",
    ),
    (
        0x001e_01d8,
        cfe_state_trophy_room_ctor_2,
        "cFEStateTrophyRoom_ctor_2",
    ),
    (
        0x001e_0268,
        cfe_state_rewards_room_ctor_2,
        "cFEStateRewardsRoom_ctor_2",
    ),
    (
        0x001e_02a0,
        cfe_state_rewards_room_dtor,
        "cFEStateRewardsRoom_dtor",
    ),
    (
        0x001e_02d0,
        cfe_popup_select_multiplayer_mode_dtor,
        "cFEPopupSelectMultiplayerMode_dtor",
    ),
    (
        0x001e_0358,
        cfe_popup_select_mp_mode_on_widget_created,
        "cFEPopupSelectMultiplayerMode_OnWidgetCreated",
    ),
    (
        0x001e_0510,
        cfe_state_audio_options_ctor_2,
        "cFEStateAudioOptions_ctor_2",
    ),
    (
        0x001e_0548,
        cfe_state_audio_options_dtor,
        "cFEStateAudioOptions_dtor",
    ),
];

/// The base `UIState` constructor (`UIState_ctor`, 0x39e2a0).
pub(crate) const OC952_UISTATE_CTOR: u32 = 0x0039_e2a0;
/// The base `UIState` destructor (`UIState_dtor`, 0x39e390).
pub(crate) const OC952_UISTATE_DTOR: u32 = 0x0039_e390;
/// `GetHashValue32` / `StrHash` (0x317670): the hash of a NUL-terminated name.
pub(crate) const OC952_STR_HASH: u32 = 0x0031_7670;
/// `UIMenu_ResetScroll(UIMenu* m, int count)` (0x39b760).
pub(crate) const OC952_UIMENU_RESET_SCROLL: u32 = 0x0039_b760;

/// Shared body of the `UIState`-derived destructors: store the class vtable
/// (`0x470000 - vtable_off`) at `self+8`, then run the base
/// `UIState_dtor(self, flags)` with the flags passed through.
pub(crate) fn oc952_ui_state_dtor(c: &mut Cpu, vtable_off: u32, ret_addr: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x0047_0000;
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(vtable_off));
    let vt = c.g[V0][0];
    Addr((c.g[V1][0] as u32).wrapping_add(8)).sw(c, vt);
    call_ptr(c, OC952_UISTATE_DTOR, ret_addr)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// Shared body of the `UIState`-derived constructors: run the base
/// `UIState_ctor(self, 0)`, then store the class vtable (`0x470000 -
/// vtable_off`) at `self+8` and return `self` in `v0`.
pub(crate) fn oc952_ui_state_ctor(c: &mut Cpu, vtable_off: u32, ret_addr: u32) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = 0;
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, OC952_UISTATE_CTOR, ret_addr)?;
    c.g[V1][0] = 0x0047_0000;
    c.g[V0][0] = c.g[S0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(vtable_off));
    frame.restore_ra(c, 0);
    let vt = c.g[V1][0];
    Addr((c.g[S0][0] as u32).wrapping_add(8)).sw(c, vt);
    frame.restore(c, S0, 0x10);
    frame.close(c);
    Ok(())
}

/// Exit of `cFEPopupSelectMultiplayerMode_OnWidgetCreated`: the shared
/// epilogue (`cycles += 4`, restore `s1` and `ra`, return).
fn oc952_widget_created_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cFEStatePeakRoom_dtor(self, flags)` (0x1e01b0): the peak room's
/// vtable (`0x46d0d0`) at `+8`, then the base `UIState` destructor.
pub fn cfe_state_peak_room_dtor(c: &mut Cpu) {
    guest(c, |c| oc952_ui_state_dtor(c, 0x2f30, 0x001e_01cc));
}

/// `cFEStateTrophyRoom_ctor_2(self)` (0x1e01d8): the base `UIState` ctor, then
/// the trophy room's vtable (`0x4696b8`) at `+8`.
pub fn cfe_state_trophy_room_ctor_2(c: &mut Cpu) {
    guest(c, |c| oc952_ui_state_ctor(c, 0x6948, 0x001e_01f0));
}

/// `cFEStateRewardsRoom_ctor_2(self)` (0x1e0268): the base `UIState` ctor, then
/// the rewards room's vtable (`0x4695e8`) at `+8`.
pub fn cfe_state_rewards_room_ctor_2(c: &mut Cpu) {
    guest(c, |c| oc952_ui_state_ctor(c, 0x6a18, 0x001e_0280));
}

/// `cFEStateRewardsRoom_dtor(self, flags)` (0x1e02a0): the rewards room's
/// vtable (`0x46d0d0`) at `+8`, then the base `UIState` destructor.
pub fn cfe_state_rewards_room_dtor(c: &mut Cpu) {
    guest(c, |c| oc952_ui_state_dtor(c, 0x2f30, 0x001e_02bc));
}

/// `cFEPopupSelectMultiplayerMode_dtor(self, flags)` (0x1e02d0): the popup's
/// vtable (`0x469518`) at `+8`, then the base `UIState` destructor.
pub fn cfe_popup_select_multiplayer_mode_dtor(c: &mut Cpu) {
    guest(c, |c| oc952_ui_state_dtor(c, 0x6ae8, 0x001e_02ec));
}

/// `void cFEPopupSelectMultiplayerMode_OnWidgetCreated(self, item)` (0x1e0358):
/// the item's name hash (`item+0x38`, `StrHash` of the name) picks one of three
/// widget names. "menu0000" (0x460020): flag 1 and `UIMenu_ResetScroll(item,
/// 2)`, then flag 0x80. "0" (0x4a13a0): clear `item+0x18`. "1" (0x4a14f8): set
/// `item+0x18` to 1.
pub fn cfe_popup_select_mp_mode_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x0046_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x38).lw(c);
        call_ptr(c, OC952_STR_HASH, 0x001e_037c)?;
        let other = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x004a_0000;
        // `true` leaves through the shared `cycles += 1` exit; `false` goes
        // straight to the epilogue (the mismatch of the last name).
        let shared_exit = 'exit: {
            if !other {
                // "menu0000": flag 1, reset the list, then flag 0x80.
                c.cycles += 10;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x14).lw(c);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x2;
                c.g[V0][0] |= 0x1;
                let flags = c.g[V0][0];
                Addr(c.g[S1][0] as u32).off(0x14).sw(c, flags);
                call_ptr(c, OC952_UIMENU_RESET_SCROLL, 0x001e_039c)?;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x14).lw(c) | 0x80;
                let flags = c.g[V0][0];
                Addr(c.g[S1][0] as u32).off(0x14).sw(c, flags);
                break 'exit true;
            }
            // "0": clear `item+0x18` when the hash matches.
            c.cycles += 5;
            c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x38).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x13a0));
            call_ptr(c, OC952_STR_HASH, 0x001e_03b8)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 2;
                Addr(c.g[S1][0] as u32).off(0x18).sw(c, 0);
                break 'exit true;
            }
            // "1": set `item+0x18` when the hash matches.
            c.g[A0][0] = 0x004a_0000;
            c.cycles += 5;
            c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x38).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14f8));
            call_ptr(c, OC952_STR_HASH, 0x001e_03d4)?;
            let mismatch = c.g[S0][0] != c.g[V0][0];
            frame.restore(c, S0, 0x20);
            if mismatch {
                break 'exit false;
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            let one = c.g[V0][0];
            Addr(c.g[S1][0] as u32).off(0x18).sw(c, one);
            true
        };
        if shared_exit {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        oc952_widget_created_epilogue(c, frame)
    })
}

/// `cFEStateAudioOptions_ctor_2(self)` (0x1e0510): the base `UIState` ctor,
/// then the audio options' vtable (`0x469448`) at `+8`.
pub fn cfe_state_audio_options_ctor_2(c: &mut Cpu) {
    guest(c, |c| oc952_ui_state_ctor(c, 0x6bb8, 0x001e_0528));
}

/// `cFEStateAudioOptions_dtor(self, flags)` (0x1e0548): the audio options'
/// vtable (`0x46d0d0`) at `+8`, then the base `UIState` destructor.
pub fn cfe_state_audio_options_dtor(c: &mut Cpu) {
    guest(c, |c| oc952_ui_state_dtor(c, 0x2f30, 0x001e_0564));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, elf_image, Rng, DATA};
    use crate::game::hashvalue::str_hash;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate words of this module's two `UIState` fakes, in each machine's own
    /// RAM (a unique place above the fixture's usual data).
    const OC952_GATE_CTOR: u32 = DATA + 0x0029_5000;
    const OC952_GATE_DTOR: u32 = DATA + 0x0029_5004;
    const OC952_MAGIC: u32 = 0x0952_6a7e;
    static OC952_PREV_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC952_PREV_DTOR: AtomicUsize = AtomicUsize::new(0);

    /// The `UIState_ctor` slot's test fake: with its gate on it only logs the
    /// call; with the gate off it runs the body the slot held before.
    fn oc952_fake_ui_ctor(c: &mut Cpu) {
        if c.bus.read32(OC952_GATE_CTOR) != OC952_MAGIC {
            return fakes::fall_back(c, &OC952_PREV_CTOR, ee::s13::f_0039e2a0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, OC952_UISTATE_CTOR, ra);
    }

    /// The `UIState_dtor` slot's test fake, gated like [`oc952_fake_ui_ctor`].
    fn oc952_fake_ui_dtor(c: &mut Cpu) {
        if c.bus.read32(OC952_GATE_DTOR) != OC952_MAGIC {
            return fakes::fall_back(c, &OC952_PREV_DTOR, ee::s13::f_0039e390__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, OC952_UISTATE_DTOR, ra);
    }

    /// A machine with the two base-state fakes hooked (once per process).
    fn oc952_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc952_fake_ui_ctor,
            &OC952_PREV_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc952_fake_ui_dtor,
            &OC952_PREV_DTOR,
        );
        machine(c, seed, salt, 2)
    }

    /// A constructor case: the object at `DATA + 0x2000`, the base ctor faked
    /// (logged), and `a1` scrambled (the port must set it to 0 itself).
    fn oc952_ctor_case(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 60, &|c, seed| {
            let mut r = oc952_machine(c, seed, salt);
            c.bus.write32(OC952_GATE_CTOR, OC952_MAGIC);
            c.g[A0][0] = sx(DATA + 0x2000);
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    /// A destructor case: the object at `DATA + 0x2000`, the base dtor faked
    /// (logged), and random flags, which must pass through unchanged.
    fn oc952_dtor_case(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 60, &|c, seed| {
            let mut r = oc952_machine(c, seed, salt);
            c.bus.write32(OC952_GATE_DTOR, OC952_MAGIC);
            c.g[A0][0] = sx(DATA + 0x2000);
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    /// `StrHash` of the NUL-terminated name at `at` in the game image.
    fn oc952_name_hash(c: &mut Cpu, at: u32) -> u32 {
        let mut name = Vec::new();
        for k in 0..64 {
            let b = c.bus.read8(at + k);
            if b == 0 {
                break;
            }
            name.push(b);
        }
        str_hash(&name)
    }

    #[test]
    fn cfe_state_peak_room_dtor_matches() {
        oc952_dtor_case(
            ee::s04::f_001e01b0__generated,
            cfe_state_peak_room_dtor,
            0x1e_01b0,
        );
    }

    #[test]
    fn cfe_state_trophy_room_ctor_2_matches() {
        oc952_ctor_case(
            ee::s04::f_001e01d8__generated,
            cfe_state_trophy_room_ctor_2,
            0x1e_01d8,
        );
    }

    #[test]
    fn cfe_state_rewards_room_ctor_2_matches() {
        oc952_ctor_case(
            ee::s04::f_001e0268__generated,
            cfe_state_rewards_room_ctor_2,
            0x1e_0268,
        );
    }

    #[test]
    fn cfe_state_rewards_room_dtor_matches() {
        oc952_dtor_case(
            ee::s04::f_001e02a0__generated,
            cfe_state_rewards_room_dtor,
            0x1e_02a0,
        );
    }

    #[test]
    fn cfe_popup_select_multiplayer_mode_dtor_matches() {
        oc952_dtor_case(
            ee::s04::f_001e02d0__generated,
            cfe_popup_select_multiplayer_mode_dtor,
            0x1e_02d0,
        );
    }

    /// Three names reach the three branches: seed % 4 picks "menu0000",
    /// "0", "1" or a name that matches none of them.
    #[test]
    fn cfe_popup_select_mp_mode_on_widget_created_matches() {
        check(
            ee::s04::f_001e0358__generated,
            cfe_popup_select_mp_mode_on_widget_created,
            64,
            &|c, seed| {
                let mut r = oc952_machine(c, seed, 0x1e_0358);
                // The names live in the game image.
                elf_image(c);
                let item = DATA + 0x2000;
                c.bus.write32(item + 0x14, r.next_u32());
                c.bus.write32(item + 0x18, r.next_u32());
                let name = match seed % 4 {
                    0 => oc952_name_hash(c, 0x0046_0020),
                    1 => oc952_name_hash(c, 0x004a_13a0),
                    2 => oc952_name_hash(c, 0x004a_14f8),
                    _ => r.next_u32(),
                };
                c.bus.write32(item + 0x38, name);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(item);
            },
        );
    }

    #[test]
    fn cfe_state_audio_options_ctor_2_matches() {
        oc952_ctor_case(
            ee::s04::f_001e0510__generated,
            cfe_state_audio_options_ctor_2,
            0x1e_0510,
        );
    }

    #[test]
    fn cfe_state_audio_options_dtor_matches() {
        oc952_dtor_case(
            ee::s04::f_001e0548__generated,
            cfe_state_audio_options_dtor,
            0x1e_0548,
        );
    }
}
