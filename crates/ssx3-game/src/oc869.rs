//! Staging module oc869 (wave wave251): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc869`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_8f98,
        oc869_net_fe_state_on_net_timeout,
        "NetFEState_OnNetTimeout",
    ),
    (
        0x001a_92d8,
        oc869_net_fe_state_on_challenge_popup_event,
        "NetFEState_OnChallengePopupEvent",
    ),
    (
        0x001a_94d8,
        oc869_c_fe_state_online_character_select_empty_child_handler,
        "cFEStateOnlineCharacterSelect_EmptyChildHandler",
    ),
    (
        0x001a_98b0,
        oc869_net_fe_state_get_selected_user,
        "NetFEState_GetSelectedUser",
    ),
    (
        0x001a_9c50,
        oc869_net_fe_state_get_popup,
        "NetFEState_GetPopup",
    ),
];

/// `NetFEState` field offsets used by this module.
const NETFE_OFF_POPUP: u32 = 0x6a0;
const NETFE_OFF_PARENT: u32 = 0x20;
const NETFE_OFF_RESULT: u32 = 0x6bc;
const NETFE_OFF_SELECTED_USER: u32 = 0x6c0;
/// `cScreenPopup` fields: the `cPopupInfo` at +0xbc and the child-owned word at +0x178.
const POPUP_OFF_INFO: u32 = 0xbc;
const POPUP_OFF_KEEP: u32 = 0x178;

/// `void NetFEState_OnChildEventDefault`-style exit shared by the challenge
/// handler's paths (`'b_1a93c0`): restore s0, then the epilogue (s1, ra, sp).
fn oc869_challenge_exit(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, 0x20);
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// The challenge handler's default route (`'b_1a93b4`): forward the event to
/// `NetFEState_OnChildEventDefault` with the kind unchanged.
fn oc869_challenge_default(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x001a_93d0, 0x001a_93bc)?;
    oc869_challenge_exit(c, frame);
    Ok(())
}

/// The challenge handler's kind 22 route (`'b_1a93a0`): `vt[32]` on self
/// (the vtable is in V0 on entry), then, if the result is still 0, the
/// parent's `vt[17]` with `(self, 0xf)`, then the default forward with 22.
fn oc869_challenge_case22(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    let vt = c.g[V0][0];
    let slot = VSlot::read(c, vt, 0x100);
    c.g[A0][0] = slot.delta;
    c.g[V1][0] = slot.func;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(slot.delta as u32));
    call_ptr(c, slot.func as u32, 0x001a_9370)?;
    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(NETFE_OFF_RESULT)).lw(c);
    let handled = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S0][0];
    if !handled {
        c.cycles += 9;
        c.g[A3][0] = Addr((c.g[S0][0] as u32).wrapping_add(NETFE_OFF_PARENT)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0xf;
        let parent = c.g[A3][0];
        c.g[V0][0] = Addr((parent as u32).wrapping_add(8)).lw(c);
        let vt2 = c.g[V0][0];
        let slot2 = VSlot::read(c, vt2, 0x88);
        c.g[A0][0] = slot2.delta;
        c.g[V1][0] = slot2.func;
        c.g[A0][0] = sx((parent as u32).wrapping_add(slot2.delta as u32));
        call_ptr(c, slot2.func as u32, 0x001a_939c)?;
        c.g[A0][0] = c.g[S0][0];
    }
    c.cycles += 5;
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = 0x16;
    call_ptr(c, 0x001a_93d0, 0x001a_93ac)?;
    oc869_challenge_exit(c, frame);
    Ok(())
}

/// Body of `NetFEState_OnChallengePopupEvent` (0x1a92d8): the popup result
/// is set by the kind (21 -> 2, 15 -> 0, 22 -> the parent's vt[17] when still
/// 0), then the event goes to `NetFEState_OnChildEventDefault`.
fn oc869_challenge_body(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[V0][0] = 0x15;
    c.g[S0][0] = c.g[A0][0];
    let kind = c.g[A2][0];
    c.g[S1][0] = c.g[A1][0];
    if kind == 0x15 {
        // case 21: result 2, then the default child handler with 21.
        c.cycles += 8;
        c.g[V0][0] = 2;
        c.g[A0][0] = c.g[S0][0];
        let result = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(NETFE_OFF_RESULT)).sw(c, result);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x15;
        call_ptr(c, 0x001a_93d0, 0x001a_9340)?;
        oc869_challenge_exit(c, frame);
        return Ok(());
    }
    if kind < 0x16 {
        c.cycles += 3;
        c.g[V0][0] = 0xf;
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        if kind == 0xf {
            // case 15: result 0, then the default child handler with 15.
            c.cycles += 6;
            Addr((c.g[S0][0] as u32).wrapping_add(NETFE_OFF_RESULT)).sw(c, 0);
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_93d0, 0x001a_9358)?;
            oc869_challenge_exit(c, frame);
            return Ok(());
        }
        c.cycles += 2;
        return oc869_challenge_default(c, frame);
    }
    // kind >= 0x16
    c.cycles += 3;
    c.g[V0][0] = 0xf;
    c.cycles += 3;
    c.g[V0][0] = 0x16;
    if kind == 0x16 {
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
        return oc869_challenge_case22(c, frame);
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S0][0];
    oc869_challenge_default(c, frame)
}

/// `void NetFEState_OnChallengePopupEvent(NetFEState* self, cUIState* popup,
/// unsigned kind)` (0x1a92d8): the challenge popup's result is set from the
/// button kind, then the event is forwarded (see [`oc869_challenge_body`]).
pub fn oc869_net_fe_state_on_challenge_popup_event(c: &mut Cpu) {
    guest(c, oc869_challenge_body)
}

/// `void NetFEState_OnNetTimeout(NetFEState* self)` (0x1a8f98): builds the
/// "net timeout" popup (a `kT_BTNOK` button, the `POPUPCMNNetInterrupted`
/// text, message type 3, layer 6), shows it, then tears down the session.
pub fn oc869_net_fe_state_on_net_timeout(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x001a_9c50, 0x001a_8fb4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0046_0000;
        c.g[V0][0] = 1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
        let one = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(POPUP_OFF_INFO)).sw(c, one);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(POPUP_OFF_INFO));
        call_ptr(c, 0x0031_7670, 0x001a_8fd0)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001c_57c0, 0x001a_8fe0)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c40));
        call_ptr(c, 0x0031_7670, 0x001a_8fec)?;
        c.g[A1][0] = 0x83e;
        c.g[V1][0] = 3;
        c.g[A2][0] = 6;
        let s0 = c.g[S0][0] as u32;
        let msg_id = c.g[A1][0];
        Addr(s0.wrapping_add(0x150)).sw(c, msg_id);
        let text = c.g[V0][0];
        Addr(s0.wrapping_add(0xc4)).sw(c, text);
        c.g[A0][0] = c.g[S2][0];
        let msg_type = c.g[V1][0];
        Addr(s0.wrapping_add(0xc0)).sw(c, msg_type);
        c.g[A1][0] = c.g[S0][0];
        let layer = c.g[A2][0];
        Addr(s0.wrapping_add(0x14c)).sw(c, layer);
        call_ptr(c, 0x001a_9bc0, 0x001a_9014)?;
        // GetSession: create the session when the global is empty.
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x238)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            call_ptr(c, 0x0026_0f80, 0x001a_9028)?;
        }
        c.cycles += 4;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x238)).lw(c);
        call_ptr(c, 0x0026_1460, 0x001a_9030)?;
        let kept = c.g[V0][0] != 0;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x238)).lw(c);
        if !kept {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x238)).lw(c);
            call_ptr(c, 0x0026_13c8, 0x001a_9040)?;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x238)).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
            call_ptr(c, 0x0025_b800, 0x001a_9054)?;
        }
        c.cycles += 15;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x238)).lw(c);
        call_ptr(c, 0x0026_1408, 0x001a_905c)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
        call_ptr(c, 0x0025_b800, 0x001a_9064)?;
        call_ptr(c, 0x0025_91b8, 0x001a_906c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0026_36f0, 0x001a_9078)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOnlineCharacterSelect_EmptyChildHandler` (0x1a94d8): a no-op
/// child handler, only the return.
pub fn oc869_c_fe_state_online_character_select_empty_child_handler(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void* NetFEState_GetSelectedUser(NetFEState* self)` (0x1a98b0): the
/// selected user, the word at +0x6c0.
pub fn oc869_net_fe_state_get_selected_user(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(NETFE_OFF_SELECTED_USER)).lw(c);
    ret(c);
}

/// `cScreenPopup* NetFEState_GetPopup(NetFEState* self)` (0x1a9c50): the
/// state's popup. Created on first use (`cMemMan_alloc` of 0x360 bytes, then
/// its constructor) or reset in place; returns it, with its +0x178 word
/// cleared.
pub fn oc869_net_fe_state_get_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let s0 = c.g[S0][0] as u32;
        c.g[A0][0] = Addr(s0.wrapping_add(NETFE_OFF_POPUP)).lw(c);
        c.g[A1][0] = 0x0046_0000;
        if c.g[A0][0] == 0 {
            c.cycles += 11;
            c.g[A0][0] = 0x360;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d58));
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001a_9c80)?;
            c.g[A1][0] = Addr(s0.wrapping_add(0x10)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x001c_58e8, 0x001a_9c90)?;
            let popup = c.g[V0][0];
            Addr(s0.wrapping_add(NETFE_OFF_POPUP)).sw(c, popup);
        } else {
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(POPUP_OFF_INFO));
            call_ptr(c, 0x001c_5630, 0x001a_9ca0)?;
            c.g[V0][0] = Addr(s0.wrapping_add(NETFE_OFF_POPUP)).lw(c);
        }
        c.cycles += 6;
        let popup = c.g[V0][0] as u32;
        Addr(popup.wrapping_add(POPUP_OFF_KEEP)).sw(c, 0);
        c.g[V0][0] = Addr(s0.wrapping_add(NETFE_OFF_POPUP)).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, Rng, DATA, GAME_GP};

    /// The fakes' gate word and its magic (each machine of a diff has its own).
    pub(crate) const OC869_CTRL: u32 = DATA + 0x2f_0000;
    pub(crate) const OC869_MAGIC: u32 = 0x4f43_3836;
    pub(crate) const OC869_SELF: u32 = DATA + 0x20_0000;
    pub(crate) const OC869_POPUP: u32 = DATA + 0x21_0000;
    pub(crate) const OC869_PARENT: u32 = DATA + 0x22_0000;
    pub(crate) const OC869_SESSION: u32 = DATA + 0x23_0000;
    pub(crate) const OC869_LOBBY: u32 = DATA + 0x24_0000;

    /// Gated fakes for the callees of the ports below: with the gate open a
    /// call logs and returns (or unwinds per the machine's index); with it
    /// shut the slot's previous body runs.
    macro_rules! oc869_fakes {
        ($install:ident; $(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                pub(crate) static $prev: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC869_CTRL) != OC869_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            pub(crate) fn $install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc869_fakes!(
        oc869_install_generic;
        (oc869_fake_317670, OC869_PREV_317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670),
        (oc869_fake_317d70, OC869_PREV_317D70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated, 0x0031_7d70),
        (oc869_fake_1c57c0, OC869_PREV_1C57C0, ee::s03::OVERRIDE_001C57C0, ee::s03::f_001c57c0__generated, 0x001c_57c0),
        (oc869_fake_1a9bc0, OC869_PREV_1A9BC0, ee::s03::OVERRIDE_001A9BC0, ee::s03::f_001a9bc0__generated, 0x001a_9bc0),
        (oc869_fake_260f80, OC869_PREV_260F80, ee::s06::OVERRIDE_00260F80, ee::s06::f_00260f80__generated, 0x0026_0f80),
        (oc869_fake_261460, OC869_PREV_261460, ee::s06::OVERRIDE_00261460, ee::s06::f_00261460__generated, 0x0026_1460),
        (oc869_fake_2613c8, OC869_PREV_2613C8, ee::s06::OVERRIDE_002613C8, ee::s06::f_002613c8__generated, 0x0026_13c8),
        (oc869_fake_261408, OC869_PREV_261408, ee::s06::OVERRIDE_00261408, ee::s06::f_00261408__generated, 0x0026_1408),
        (oc869_fake_25b800, OC869_PREV_25B800, ee::s06::OVERRIDE_0025B800, ee::s06::f_0025b800__generated, 0x0025_b800),
        (oc869_fake_2636f0, OC869_PREV_2636F0, ee::s07::OVERRIDE_002636F0, ee::s07::f_002636f0__generated, 0x0026_36f0),
    );

    /// Hooks the gated fakes (once per process) and returns a scrambled machine
    /// with the fakes' gate open and the game's `gp` globals filled in.
    fn oc869_fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        oc869_install_generic();
        let r = fakes::machine(c, seed, salt, 4);
        c.bus.write32(OC869_CTRL, OC869_MAGIC);
        r
    }

    #[test]
    fn oc869_net_fe_state_on_net_timeout_matches() {
        check(
            ee::s03::f_001a8f98__generated,
            oc869_net_fe_state_on_net_timeout,
            300,
            &|c, seed| {
                let _ = oc869_fixture(c, seed, 0x8f98);
                c.g[A0][0] = sx(OC869_SELF);
                let popup = if seed % 3 == 0 { 0 } else { OC869_POPUP };
                c.bus.write32(OC869_SELF + NETFE_OFF_POPUP, popup);
                let session = if seed % 2 == 0 { 0 } else { OC869_SESSION };
                c.bus.write32(GAME_GP.wrapping_add(0x238), session);
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), OC869_LOBBY);
            },
        );
    }

    #[test]
    fn oc869_on_challenge_popup_event_matches() {
        check(
            ee::s03::f_001a92d8__generated,
            oc869_net_fe_state_on_challenge_popup_event,
            300,
            &|c, seed| {
                let mut r = oc869_fixture(c, seed, 0x92d8);
                let kinds = [0x15u64, 0xf, 0x16, 0x3, 0x17, 0x14, 0x10, 0x22];
                c.g[A0][0] = sx(OC869_SELF);
                c.g[A1][0] = sx(OC869_POPUP);
                c.g[A2][0] = kinds[(seed % 8) as usize];
                let vt = fakes::vtable(c, &mut r, &[0x100]);
                c.bus.write32(OC869_SELF + 8, vt);
                let parent_vt = fakes::vtable(c, &mut r, &[0x88]);
                c.bus.write32(OC869_PARENT + 8, parent_vt);
                c.bus.write32(OC869_SELF + NETFE_OFF_PARENT, OC869_PARENT);
                let result = if seed % 2 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(OC869_SELF + NETFE_OFF_RESULT, result);
            },
        );
    }

    #[test]
    fn oc869_empty_child_handler_matches() {
        check(
            ee::s03::f_001a94d8__generated,
            oc869_c_fe_state_online_character_select_empty_child_handler,
            16,
            &|c, seed| {
                let _ = oc869_fixture(c, seed, 0x94d8);
            },
        );
    }

    #[test]
    fn oc869_get_selected_user_matches() {
        check(
            ee::s03::f_001a98b0__generated,
            oc869_net_fe_state_get_selected_user,
            300,
            &|c, seed| {
                let mut r = oc869_fixture(c, seed, 0x98b0);
                c.g[A0][0] = sx(OC869_SELF);
                c.bus
                    .write32(OC869_SELF + NETFE_OFF_SELECTED_USER, r.next_u32());
            },
        );
    }

    #[test]
    fn oc869_get_popup_matches() {
        check(
            ee::s03::f_001a9c50__generated,
            oc869_net_fe_state_get_popup,
            300,
            &|c, seed| {
                let _ = oc869_fixture(c, seed, 0x9c50);
                c.g[A0][0] = sx(OC869_SELF);
                let popup = if seed % 2 == 0 { 0 } else { OC869_POPUP };
                c.bus.write32(OC869_SELF + NETFE_OFF_POPUP, popup);
            },
        );
    }
}
