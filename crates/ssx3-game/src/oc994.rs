//! Staging module oc994 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc994`.

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
        0x0021_1850,
        ov_state_profile_hide_action_buttons,
        "OVScreenProfileSavePDA_v37",
    ),
    (
        0x0021_1a08,
        ov_state_profile_hide_main_button,
        "OVScreenProfileSavePDA_v39",
    ),
];

/// A field of the overlay object whose pointer is held in `reg` (`off(reg)`).
/// Read from the callee-saved register each time, as the compiled code does.
fn profile_self_at(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr(c.g[reg][0] as u32).off(off)
}

/// `void cOVState_PROFILE_hideActionButtons(self, show)` (0x211850): hides or
/// shows the profile overlay's action buttons ("ps2x_b", "x_save", the delete
/// text, "ps2sq_b") when the hidden state changes, and caches it at +0x230.
/// `on` is 1 while `mIsOn` (+0x22c) is clear, otherwise `show`. Each widget
/// gets `setVisible(!on)` through its vtable slot 9; the two icons are
/// null-checked, the texts are not.
pub fn ov_state_profile_hide_action_buttons(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[V0][0] = profile_self_at(c, S0, 0x22c).lw(c);
        c.g[V1][0] = profile_self_at(c, S0, 0x230).lw(c);
        if c.g[V0][0] != 0 {
            c.g[S2][0] = c.g[A1][0];
        }
        let unchanged = c.g[V1][0] == c.g[S2][0];
        c.g[S1][0] = c.g[S2][0] ^ 0x1;
        if !unchanged {
            c.cycles += 9;
            c.g[A0][0] = 0x004a_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2090));
            call_ptr(c, 0x0031_7670, 0x0021_188c)?;
            c.g[A0][0] = profile_self_at(c, S0, 0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_1898)?;
            c.g[A2][0] = c.g[V0][0];
            if c.g[A2][0] == 0 {
                c.g[A0][0] = 0x004a_0000;
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0021_18bc)?;
                c.g[A0][0] = 0x004a_0000;
            }
            // "x_save" (never null-checked).
            c.cycles += 32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2568));
            call_ptr(c, 0x0031_7670, 0x0021_18c8)?;
            c.g[A0][0] = profile_self_at(c, S0, 0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_18d4)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(8).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
            c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0021_18ec)?;
            // The delete text (never null-checked).
            c.g[A0][0] = 0x0047_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1db0));
            call_ptr(c, 0x0031_7670, 0x0021_18f8)?;
            c.g[A0][0] = profile_self_at(c, S0, 0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_1904)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(8).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
            c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0021_191c)?;
            // "ps2sq_b" (null-checked).
            c.g[A0][0] = 0x004a_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20a0));
            call_ptr(c, 0x0031_7670, 0x0021_1928)?;
            c.g[A0][0] = profile_self_at(c, S0, 0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_1934)?;
            c.g[A2][0] = c.g[V0][0];
            let is_null = c.g[A2][0] == 0;
            c.g[A1][0] = c.g[S1][0];
            if !is_null {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0021_1954)?;
            }
            c.cycles += 1;
            let hidden = c.g[S2][0];
            profile_self_at(c, S0, 0x230).sw(c, hidden);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cOVState_PROFILE_hideMainButton(self, loadBlocked, on)` (0x211a08):
/// hides or shows the profile overlay's main ("ps2x_b") and save ("x_save")
/// buttons, stores `mMainButtonActive = !on` at +0x224 and sets the X icon
/// with `setVisible(!on)`. `loadBlocked` is not used.
pub fn ov_state_profile_hide_main_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x004a_0000;
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2090));
        c.g[S0][0] = c.g[S0][0] ^ 0x1;
        call_ptr(c, 0x0031_7670, 0x0021_1a34)?;
        c.g[A0][0] = profile_self_at(c, S1, 0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0021_1a40)?;
        c.g[A0][0] = 0x004a_0000;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2568));
        call_ptr(c, 0x0031_7670, 0x0021_1a50)?;
        c.g[A0][0] = profile_self_at(c, S1, 0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0021_1a5c)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(8).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
        c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
        let target = c.g[A2][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0021_1a74)?;
        let active = c.g[S0][0];
        profile_self_at(c, S1, 0x224).sw(c, active);
        c.g[A1][0] = c.g[S0][0];
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(8).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0021_1a90)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::DATA;
    use crate::game::fe::festaterewards::tests::base_4 as profile_base;

    /// The overlay object, its UI screen (hash table at +0x38/+0x3c), a hash
    /// node and the vtable of the widgets. Every string hashes to 0 on the
    /// test machine, so all lookups return the same node (0 = no widget).
    pub(crate) const HIDE_OBJ: u32 = DATA + 0x1000;
    pub(crate) const HIDE_SCREEN: u32 = DATA + 0x2000;
    pub(crate) const HIDE_NODE: u32 = DATA + 0x3000;
    pub(crate) const HIDE_VT: u32 = DATA + 0x4_0000;

    /// The screen with one hash node (`node`, 0 for none) and the widget
    /// vtable: slot 9 at +0x48 (delta 6) calls the fake. A null node reads
    /// its vtable through address 8, as the compiled code does.
    pub(crate) fn hide_buttons_setup(c: &mut Cpu, node: u32) {
        c.bus.write32(HIDE_OBJ + 0x40, HIDE_SCREEN);
        c.bus.write32(HIDE_SCREEN + 0x38, DATA + 0x30_0100);
        c.bus.write32(DATA + 0x30_0100, 1);
        c.bus.write32(HIDE_SCREEN + 0x3c, DATA + 0x30_0000);
        c.bus.write32(DATA + 0x30_0000, node);
        c.bus.write32(node + 0x38, 0);
        c.bus.write32(node + 0x8, HIDE_VT);
        c.bus.write16(HIDE_VT + 0x48, 6);
        c.bus.write32(HIDE_VT + 0x4c, FAKE);
    }

    #[test]
    fn ov_state_profile_hide_action_buttons_matches() {
        check(
            ssx3_ee::s05::f_00211850__generated,
            ov_state_profile_hide_action_buttons,
            60,
            &|c, seed| {
                let mut r = profile_base(c, seed, 0x0021_1850, 3);
                let is_on = if seed % 2 == 0 { 0 } else { r.next_u32() };
                let show = if seed % 3 == 0 { 0 } else { r.next_u32() };
                let on = if is_on == 0 { 1 } else { show };
                let hidden = match seed % 3 {
                    0 => on,
                    1 => on ^ 1,
                    _ => r.next_u32(),
                };
                c.g[A0][0] = sx(HIDE_OBJ);
                c.g[A1][0] = sx(show);
                c.bus.write32(HIDE_OBJ + 0x22c, is_on);
                c.bus.write32(HIDE_OBJ + 0x230, hidden);
                hide_buttons_setup(c, if seed % 5 == 0 { 0 } else { HIDE_NODE });
            },
        );
    }

    #[test]
    fn ov_state_profile_hide_main_button_matches() {
        check(
            ssx3_ee::s05::f_00211a08__generated,
            ov_state_profile_hide_main_button,
            60,
            &|c, seed| {
                let mut r = profile_base(c, seed, 0x0021_1a08, 2);
                c.g[A0][0] = sx(HIDE_OBJ);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(HIDE_OBJ + 0x220, r.next_u32());
                hide_buttons_setup(c, if seed % 5 == 0 { 0 } else { HIDE_NODE });
            },
        );
    }
}
