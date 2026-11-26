//! Staging module oc878 (wave wave251): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc878`.

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
        0x001b_1048,
        c_fe_state_online_character_select_update,
        "cFEStateOnlineCharacterSelect_Update",
    ),
    (
        0x001b_1068,
        c_fe_state_online_character_select_on_gain_transition,
        "cFEStateOnlineCharacterSelect_onGainTransition",
    ),
    (
        0x001b_0538,
        c_fe_state_challenge_received_popup_on_widget_action,
        "cFEStateChallengeReceivedPopup_OnWidgetAction",
    ),
    (
        0x001b_0e10,
        c_fe_state_challenge_received_popup_accept,
        "cFEStateChallengeReceivedPopup_Accept",
    ),
    (
        0x001b_0c08,
        c_fe_state_challenge_received_popup_show_waiting,
        "cFEStateChallengeReceivedPopup_ShowWaiting",
    ),
];

/// `gp + off` as the generated code computes it (32-bit wrap).
#[inline(always)]
fn cfe_chal_gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `base + off` for a register value (32-bit wrap).
#[inline(always)]
fn cfe_chal_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `void cFEStateOnlineCharacterSelect_Update(void)` (0x1b1048): the state's
/// update is the net front end's (`NetFEState_Update`, 0x1a8770).
pub fn c_fe_state_online_character_select_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x001a_8770, 0x001b_1058)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int cFEStateOnlineCharacterSelect_onGainTransition(self, bFirstTime)`
/// (0x1b1068). On the first entry: show the challenger picked in the list,
/// then the opponent's character when the network option has one, then fill
/// the player info and clear the flag at +0x6dc. Returns 0.
pub fn c_fe_state_online_character_select_on_gain_transition(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        if c.g[A1][0] != 0 {
            c.cycles += 15;
            c.g[A0][0] = Addr(cfe_chal_at(c.g[S0][0], 0x6d0)).lw(c);
            call_ptr(c, 0x0039_a738, 0x001b_1088)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0;
            call_ptr(c, 0x001b_1b10, 0x001b_1098)?;
            c.g[V0][0] = Addr(cfe_chal_gp(c, -0x848)).lw(c);
            c.g[A1][0] = 7;
            c.g[A0][0] = Addr(cfe_chal_at(c.g[V0][0], 0x78)).lw(c);
            call_ptr(c, 0x0014_dd58, 0x001b_10a8)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[S1][0] = sx(cfe_chal_at(c.g[V1][0], 0x4b30));
            c.g[A1][0] = Addr(cfe_chal_at(c.g[S1][0], 0x32c)).lw(c);
            if (c.g[A1][0] as i64) >= 0 {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001b_1760, 0x001b_10c4)?;
                c.g[A1][0] = Addr(cfe_chal_at(c.g[S1][0], 0x32c)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 1;
                call_ptr(c, 0x001b_1b10, 0x001b_10d4)?;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001b_1808, 0x001b_10dc)?;
            Addr(cfe_chal_at(c.g[S0][0], 0x6dc)).sw(c, 0);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The message-5 (button press) branch of
/// [`c_fe_state_challenge_received_popup_on_widget_action`]: the button's misc
/// data 11 is accept (unless already accepted, then the lobby is told), 12 is
/// decline; any other value does nothing.
fn cfe_chal_recv_press(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[A1][0] = Addr(cfe_chal_at(c.g[A1][0], 0x18)).lw(c);
    c.g[V0][0] = 0xb;
    let is_accept = c.g[A1][0] == c.g[V0][0];
    c.g[V0][0] = 0xc;
    if !is_accept {
        c.cycles += 2;
        if c.g[A1][0] == c.g[V0][0] {
            // Decline: accept's popup handler.
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001b_0e10, 0x001b_05bc)?;
            frame.restore(c, S0, 0x10);
            return Ok(());
        }
        c.cycles += 2;
        frame.restore(c, S0, 0x10);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(cfe_chal_at(c.g[S0][0], 0x6d4)).lw(c);
    if c.g[V0][0] != 0 {
        frame.restore(c, S0, 0x10);
        return Ok(());
    }
    c.cycles += 8;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x001b_0c08, 0x001b_059c)?;
    c.g[A0][0] = Addr(cfe_chal_gp(c, -0xc8)).lw(c);
    c.g[A2][0] = Addr(cfe_chal_at(c.g[S0][0], 0x14)).lb(c);
    c.g[A1][0] = Addr(cfe_chal_at(c.g[A0][0], 0x98)).lw(c);
    call_ptr(c, 0x0025_fb58, 0x001b_05ac)?;
    frame.restore(c, S0, 0x10);
    Ok(())
}

/// `void cFEStateChallengeReceivedPopup_OnWidgetAction(self, obj, msg)`
/// (0x1b0538). A null object does nothing; message 5 (a button press) acts on
/// the button's misc data (see [`cfe_chal_recv_press`]); message 7 does
/// nothing; any other message goes to the net front end's widget handler.
pub fn c_fe_state_challenge_received_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        if c.g[A1][0] == 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let is_press = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = 7;
            if is_press {
                cfe_chal_recv_press(c, frame)?;
            } else {
                c.cycles += 2;
                if c.g[A2][0] == c.g[V0][0] {
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 2;
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x001a_8918, 0x001b_05cc)?;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateChallengeReceivedPopup_Accept(self)` (0x1b0e10): redraw the
/// waiting popup, then tell the lobby and the session about the accepted
/// challenge (or only the session about the opponent when it was not yet
/// accepted), and finish with the parent state's popup event 0x14.
pub fn c_fe_state_challenge_received_popup_accept(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001b_0c08, 0x001b_0e24)?;
        c.g[V0][0] = Addr(cfe_chal_at(c.g[S0][0], 0x6d4)).lw(c);
        c.g[A0][0] = Addr(cfe_chal_gp(c, -0xc8)).lw(c);
        if c.g[V0][0] != 0 {
            cfe_chal_accept_accepted(c)?;
        } else {
            cfe_chal_accept_refused(c)?;
        }
        c.cycles += 11;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0x14;
        c.g[V0][0] = Addr(cfe_chal_at(c.g[A3][0], 0x8)).lw(c);
        c.g[A0][0] = Addr(cfe_chal_at(c.g[V0][0], 0x88)).lh(c);
        c.g[V1][0] = Addr(cfe_chal_at(c.g[V0][0], 0x8c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001b_0f14)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The accepted branch of [`c_fe_state_challenge_received_popup_accept`].
fn cfe_chal_accept_accepted(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = 0;
    call_ptr(c, 0x0025_f740, 0x001b_0e38)?;
    c.g[V0][0] = Addr(cfe_chal_gp(c, 0x238)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        call_ptr(c, 0x0026_0f80, 0x001b_0e4c)?;
    }
    c.cycles += 4;
    c.g[A0][0] = Addr(cfe_chal_gp(c, 0x238)).lw(c);
    call_ptr(c, 0x0026_1460, 0x001b_0e54)?;
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = Addr(cfe_chal_gp(c, 0x238)).lw(c);
    if !t {
        c.cycles += 3;
        c.g[A0][0] = Addr(cfe_chal_gp(c, 0x238)).lw(c);
        call_ptr(c, 0x0026_13c8, 0x001b_0e64)?;
        c.g[V0][0] = Addr(cfe_chal_gp(c, 0x238)).lw(c);
    }
    c.cycles += 2;
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = Addr(cfe_chal_gp(c, -0xc8)).lw(c);
    if !t {
        c.cycles += 3;
        c.g[A0][0] = Addr(cfe_chal_gp(c, -0xc8)).lw(c);
        call_ptr(c, 0x0025_b800, 0x001b_0e78)?;
        c.g[V0][0] = Addr(cfe_chal_gp(c, -0xc8)).lw(c);
    }
    c.cycles += 10;
    c.g[A0][0] = Addr(cfe_chal_gp(c, 0x238)).lw(c);
    c.g[A1][0] = Addr(cfe_chal_at(c.g[V0][0], 0x98)).lw(c);
    call_ptr(c, 0x0026_3018, 0x001b_0e88)?;
    call_ptr(c, 0x0025_91d0, 0x001b_0e90)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 2;
    call_ptr(c, 0x0026_3ba8, 0x001b_0e9c)?;
    c.g[A3][0] = Addr(cfe_chal_at(c.g[S0][0], 0x20)).lw(c);
    Ok(())
}

/// The refused branch of [`c_fe_state_challenge_received_popup_accept`]: only
/// the session hears about the opponent.
fn cfe_chal_accept_refused(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(cfe_chal_gp(c, 0x238)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        call_ptr(c, 0x0026_0f80, 0x001b_0eb8)?;
    }
    c.cycles += 4;
    c.g[A0][0] = Addr(cfe_chal_gp(c, 0x238)).lw(c);
    call_ptr(c, 0x0026_1460, 0x001b_0ec0)?;
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = Addr(cfe_chal_gp(c, 0x238)).lw(c);
    if !t {
        c.cycles += 3;
        c.g[A0][0] = Addr(cfe_chal_gp(c, 0x238)).lw(c);
        call_ptr(c, 0x0026_13c8, 0x001b_0ed0)?;
        c.g[V0][0] = Addr(cfe_chal_gp(c, 0x238)).lw(c);
    }
    c.cycles += 2;
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = Addr(cfe_chal_gp(c, -0xc8)).lw(c);
    if !t {
        c.cycles += 3;
        c.g[A0][0] = Addr(cfe_chal_gp(c, -0xc8)).lw(c);
        call_ptr(c, 0x0025_b800, 0x001b_0ee4)?;
        c.g[V0][0] = Addr(cfe_chal_gp(c, -0xc8)).lw(c);
    }
    c.cycles += 4;
    c.g[A0][0] = Addr(cfe_chal_gp(c, 0x238)).lw(c);
    c.g[A1][0] = Addr(cfe_chal_at(c.g[V0][0], 0x98)).lw(c);
    call_ptr(c, 0x0026_3018, 0x001b_0ef4)?;
    c.g[A3][0] = Addr(cfe_chal_at(c.g[S0][0], 0x20)).lw(c);
    Ok(())
}

/// `void cFEStateChallengeReceivedPopup_ShowWaiting(self)` (0x1b0c08): marks
/// the challenge as shown (+0x6d4 = 1). When the popup has its screen, the
/// decline and accept buttons get focus and static text, the menu selects
/// entry 1, and the waiting text is shown.
pub fn c_fe_state_challenge_received_popup_show_waiting(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(cfe_chal_at(c.g[S1][0], 0x6d4)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(cfe_chal_at(c.g[S1][0], 0x40)).lw(c);
        let no_screen = c.g[V1][0] == 0;
        frame.restore(c, S0, 0x20);
        if !no_screen {
            cfe_chal_show_waiting_widgets(c)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The screen part of [`c_fe_state_challenge_received_popup_show_waiting`].
fn cfe_chal_show_waiting_widgets(c: &mut Cpu) -> Flow {
    // Decline: focus.
    c.cycles += 9;
    c.g[A0][0] = sx(0x0046_3618);
    call_ptr(c, 0x0031_7670, 0x001b_0c3c)?;
    c.g[A0][0] = Addr(cfe_chal_at(c.g[S1][0], 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001b_0c48)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    if c.g[S0][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = Addr(cfe_chal_at(c.g[S0][0], 0x8)).lw(c);
        c.g[A0][0] = Addr(cfe_chal_at(c.g[V0][0], 0x38)).lh(c);
        c.g[V1][0] = Addr(cfe_chal_at(c.g[V0][0], 0x3c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001b_0c68)?;
    }
    // Menu challenge: select entry 1.
    c.cycles += 8;
    c.g[A0][0] = sx(0x0046_30e0);
    call_ptr(c, 0x0031_7670, 0x001b_0c74)?;
    c.g[A0][0] = Addr(cfe_chal_at(c.g[S1][0], 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001b_0c80)?;
    if c.g[V0][0] == 0 {
        c.g[A0][0] = 0x0046_0000;
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0039_b960, 0x001b_0c94)?;
        c.g[A0][0] = 0x0046_0000;
    }
    // Staticed waiting text: show it.
    c.cycles += 8;
    c.g[A0][0] = sx(0x0046_3468);
    call_ptr(c, 0x0031_7670, 0x001b_0ca0)?;
    c.g[A0][0] = Addr(cfe_chal_at(c.g[S1][0], 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001b_0cac)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    if c.g[S0][0] != 0 {
        c.cycles += 11;
        c.g[V0][0] = Addr(cfe_chal_at(c.g[S0][0], 0x8)).lw(c);
        c.g[A0][0] = Addr(cfe_chal_at(c.g[V0][0], 0x48)).lh(c);
        c.g[V1][0] = Addr(cfe_chal_at(c.g[V0][0], 0x4c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001b_0ccc)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(0x0046_2750);
        call_ptr(c, 0x0031_7670, 0x001b_0cd8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x001b_0ce4)?;
    }
    // Accept: static.
    c.cycles += 9;
    c.g[A0][0] = sx(0x0046_35f8);
    call_ptr(c, 0x0031_7670, 0x001b_0cf0)?;
    c.g[A0][0] = Addr(cfe_chal_at(c.g[S1][0], 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001b_0cfc)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    if c.g[S0][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = Addr(cfe_chal_at(c.g[S0][0], 0x8)).lw(c);
        c.g[A0][0] = Addr(cfe_chal_at(c.g[V0][0], 0x40)).lh(c);
        c.g[V1][0] = Addr(cfe_chal_at(c.g[V0][0], 0x44)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001b_0d1c)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    #[test]
    fn c_fe_state_online_character_select_update_matches() {
        check(
            ssx3_ee::s03::f_001b1048__generated,
            c_fe_state_online_character_select_update,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x1b1048, 0);
                c.pc = 0x001b_1048;
            },
        );
    }

    #[test]
    fn c_fe_state_online_character_select_on_gain_transition_matches() {
        check(
            ssx3_ee::s03::f_001b1068__generated,
            c_fe_state_online_character_select_on_gain_transition,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1b1068, 0);
                // The BE library dummy and the pool allocator: without them
                // the callees reach BELibrary_GetInstance and load from disc.
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
                crate::game::ai::computer::tests::rbc_test_allocator(c);
                let this_ = DATA + 0x100;
                c.g[A0][0] = sx(this_);
                // bFirstTime 0 returns at once; otherwise the opponent's
                // character is shown only when the network option has one.
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { 1 };
                c.pc = 0x001b_1068;
                c.bus.write32(this_ + 0x6d0, DATA + 0x4000);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x78, DATA + 0x3000);
                let opponent = if seed % 3 == 0 { 0xffff_ffff } else { seed % 5 };
                c.bus.write32(0x0053_4b30 + 0x32c, opponent);
            },
        );
    }

    /// The popup, its parent state (vtable slot 0x88 is a fake), the session
    /// and the lobby with its opponent.
    fn challenge_fixture(c: &mut Cpu, seed: u32, this_: u32) {
        let parent = DATA + 0x2000;
        let parent_vt = DATA + 0x2400;
        let lobby = DATA + 0x6000;
        c.bus.write32(this_ + 0x20, parent);
        c.bus.write32(parent + 8, parent_vt);
        c.bus.write16(parent_vt + 0x88, 0);
        c.bus.write32(parent_vt + 0x8c, FAKE);
        c.bus.write32(
            GAME_GP.wrapping_add(0x238),
            if seed % 3 == 0 { 0 } else { DATA + 0x3000 },
        );
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), lobby);
        c.bus.write32(lobby + 0x98, DATA + 0x6800);
    }

    #[test]
    fn c_fe_state_challenge_received_popup_on_widget_action_matches() {
        check(
            ssx3_ee::s03::f_001b0538__generated,
            c_fe_state_challenge_received_popup_on_widget_action,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x1b0538, 0);
                let this_ = DATA + 0x100;
                let obj = DATA + 0x1400;
                // 0: no object; 1: msg 7; 2: press accept, not yet accepted;
                // 3: press accept, accepted; 4: press decline; 5: other misc;
                // 6: another message (net front end).
                let kind = seed % 7;
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = sx(if kind == 0 { 0 } else { obj });
                c.g[A2][0] = match kind {
                    1 => 7,
                    6 => 9,
                    _ => 5,
                };
                c.bus.write32(
                    obj + 0x18,
                    match kind {
                        2 | 3 => 11,
                        4 => 12,
                        _ => 3,
                    },
                );
                c.bus.write32(this_ + 0x6d4, if kind == 3 { 1 } else { 0 });
                c.bus.write32(this_ + 0x40, 0);
                c.bus.write8(this_ + 0x14, seed as u8);
                challenge_fixture(c, seed, this_);
                c.pc = 0x001b_0538;
            },
        );
    }

    #[test]
    fn c_fe_state_challenge_received_popup_accept_matches() {
        check(
            ssx3_ee::s03::f_001b0e10__generated,
            c_fe_state_challenge_received_popup_accept,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1b0e10, 0);
                let this_ = DATA + 0x100;
                c.g[A0][0] = sx(this_);
                c.pc = 0x001b_0e10;
                c.bus.write32(this_ + 0x40, 0);
                c.bus
                    .write32(this_ + 0x6d4, if seed % 2 == 0 { 0 } else { 1 });
                challenge_fixture(c, seed, this_);
            },
        );
    }

    #[test]
    fn c_fe_state_challenge_received_popup_show_waiting_matches() {
        check(
            ssx3_ee::s03::f_001b0c08__generated,
            c_fe_state_challenge_received_popup_show_waiting,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x1b0c08, 0);
                let this_ = DATA + 0x100;
                c.g[A0][0] = sx(this_);
                c.pc = 0x001b_0c08;
                c.bus.write32(this_ + 0x6d4, 0);
                // No screen, or a screen whose lookups find nothing.
                let screen = if seed % 3 == 0 { 0 } else { DATA + 0x1000 };
                c.bus.write32(this_ + 0x40, screen);
            },
        );
    }
}
