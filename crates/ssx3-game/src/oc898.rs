//! Staging module oc898 (wave wave253): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc898`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001c_1d10,
        c_fe_state_net_ssx_world_update,
        "cFEStateNetSSXWorld_Update",
    ),
    (
        0x001c_1d30,
        c_fe_state_net_ssx_world_v25,
        "cFEStateNetSSXWorld_v25",
    ),
    (
        0x001c_21a0,
        c_fe_state_net_quick_messages_update,
        "cFEStateNetQuickMessages_Update",
    ),
    (
        0x001c_21c0,
        c_fe_state_net_quick_messages_v25,
        "cFEStateNetQuickMessages_v25",
    ),
    (
        0x001c_2298,
        c_fe_state_net_quick_messages_on_widget_created,
        "cFEStateNetQuickMessages_OnWidgetCreated",
    ),
];

/// `void cFEStateNetSSXWorld_Update(self)` (0x1c1d10): forwards to the
/// network state's `NetFEState::Update` (0x1a8770) with a 0x10 frame.
pub fn c_fe_state_net_ssx_world_update(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x001a_8770), 0x001c_1d20);
}

/// `void cFEStateNetQuickMessages_Update(self)` (0x1c21a0): the same forward
/// to `NetFEState::Update` (0x1a8770) as the SSX World state.
pub fn c_fe_state_net_quick_messages_update(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x001a_8770), 0x001c_21b0);
}

/// `void cFEStateNetSSXWorld_v25(self, a1)` (0x1c1d30): the world help
/// popup's text and option setup. Four option rows through the widget calls
/// `0x1da510`/`0x1da530`, each row's label looked up by hash (`0x317670`).
pub fn c_fe_state_net_ssx_world_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x4;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001c_1d4c)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5a48));
        call_ptr(c, 0x0031_7670, 0x001c_1d58)?;
        let title = c.g[V0][0];
        Addr(r32(c, S0)).off(0x54).sw(c, title);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5a60));
        call_ptr(c, 0x0031_7670, 0x001c_1d68)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x001d_a530, 0x001c_1d78)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x1a0));
        call_ptr(c, 0x0031_7670, 0x001c_1d84)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A3][0] = 3;
        call_ptr(c, 0x001d_a510, 0x001c_1d98)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x2c50));
        call_ptr(c, 0x0031_7670, 0x001c_1da4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[A3][0] = 0;
        call_ptr(c, 0x001d_a510, 0x001c_1db8)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x1458));
        call_ptr(c, 0x0031_7670, 0x001c_1dc4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        c.g[A3][0] = 2;
        call_ptr(c, 0x001d_a510, 0x001c_1dd8)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x14e8));
        call_ptr(c, 0x0031_7670, 0x001c_1de4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        c.g[A3][0] = 1;
        call_ptr(c, 0x001d_a510, 0x001c_1df8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetQuickMessages_v25(self, a1)` (0x1c21c0): the message
/// popup's text and option setup, the same shape as the SSX World variant
/// with the message strings' offsets.
pub fn c_fe_state_net_quick_messages_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x4;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001c_21dc)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5b38));
        call_ptr(c, 0x0031_7670, 0x001c_21e8)?;
        let title = c.g[V0][0];
        Addr(r32(c, S0)).off(0x54).sw(c, title);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5b50));
        call_ptr(c, 0x0031_7670, 0x001c_21f8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x001d_a530, 0x001c_2208)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x1a0));
        call_ptr(c, 0x0031_7670, 0x001c_2214)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A3][0] = 3;
        call_ptr(c, 0x001d_a510, 0x001c_2228)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5b70));
        call_ptr(c, 0x0031_7670, 0x001c_2234)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[A3][0] = 0;
        call_ptr(c, 0x001d_a510, 0x001c_2248)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x14e8));
        call_ptr(c, 0x0031_7670, 0x001c_2254)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        c.g[A3][0] = 1;
        call_ptr(c, 0x001d_a510, 0x001c_2268)?;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5b88));
        call_ptr(c, 0x0031_7670, 0x001c_2274)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        c.g[A3][0] = 7;
        call_ptr(c, 0x001d_a510, 0x001c_2288)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIText_setAsciiString(text, "str_sp")` then `text->setVisible(1)`
/// through its vtable (`text` in s1): the visible-message step of
/// OnWidgetCreated. `set_ret` and `visible_ret` are the two return addresses.
fn oc898_show_text(c: &mut Cpu, set_ret: u32, visible_ret: u32) -> Flow {
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = sx(0x004a_0000u32.wrapping_add(0x1390));
    call_ptr(c, 0x003a_0c50, set_ret)?;
    c.g[V0][0] = Addr(r32(c, S1)).off(0x8).lw(c);
    c.g[A1][0] = 1;
    c.g[A0][0] = Addr(r32(c, V0)).off(0x48).lh(c);
    c.g[V1][0] = Addr(r32(c, V0)).off(0x4c).lw(c);
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, visible_ret)
}

/// `void cFEStateNetQuickMessages_OnWidgetCreated(self, text)` (0x1c2298):
/// tags a new text widget. The username text and the "Menu" text are shown
/// when their hashes match; otherwise the ten `messageNN` names are tried,
/// and a match shows the text, stores its slot index in `text->mMiscData`
/// and records the widget in `self->mpMessage[i]`.
pub fn c_fe_state_net_quick_messages_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x5ba0));
        c.g[S0][0] = Addr(r32(c, S1)).off(0x38).lw(c);
        call_ptr(c, 0x0031_7670, 0x001c_22d0)?;
        if c.g[S0][0] == c.g[V0][0] {
            c.cycles += 12;
            oc898_show_text(c, 0x001c_22e8, 0x001c_2300)?;
            frame.restore(c, S0, 0x80);
        } else {
            c.cycles += 5;
            c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x1398));
            c.g[S0][0] = Addr(r32(c, S1)).off(0x38).lw(c);
            call_ptr(c, 0x0031_7670, 0x001c_2314)?;
            let no_match = c.g[S0][0] != c.g[V0][0];
            c.g[S3][0] = sx(r32(c, S2).wrapping_add(0x6d4));
            if no_match {
                c.cycles += 3;
                c.g[S5][0] = 0x0046_0000;
                c.g[S2][0] = 0;
                c.g[S4][0] = 0x004a_0000;
                loop {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx(r32(c, S5).wrapping_add(0x5bb0));
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x0041_7828, 0x001c_2348)?;
                    c.g[S0][0] = Addr(r32(c, S1)).off(0x38).lw(c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0031_7670, 0x001c_2354)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                    } else {
                        c.cycles += 12;
                        oc898_show_text(c, 0x001c_2368, 0x001c_2380)?;
                        let index = c.g[S2][0];
                        Addr(r32(c, S1)).off(0x18).sw(c, index);
                        let text = c.g[S1][0];
                        Addr(r32(c, S3)).sw(c, text);
                        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                    }
                    c.cycles += 3;
                    let more = (c.g[S2][0] as i64) < 10;
                    c.g[V0][0] = more as u64;
                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x001c_2338);
                    }
                }
            } else {
                c.cycles += 4;
                let flags = Addr(r32(c, S1)).off(0x14).lw(c) | 1;
                c.g[V0][0] = flags;
                Addr(r32(c, S1)).off(0x14).sw(c, flags);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x80);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetQuickMessages_RefreshAll(self)` (0x1c2418): refresh the
/// ten message slots, `RefreshMessage(self, i)` for i = 0..9.
/// NOT INSTALLED (no OVERRIDES entry, no test): the real RefreshMessage (0x1c2468) allocates through EAFile and spins on test data.
#[allow(dead_code)]
pub fn c_fe_state_net_quick_messages_refresh_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001c_2468, 0x001c_2440)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            let more = (c.g[S0][0] as i64) < 10;
            c.g[V0][0] = more as u64;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_2438);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate word in the machine's RAM: the `NetFEState::Update` fake only
    /// runs where it is set; other tests' machines fall through to the body.
    const OC898_GATE: u32 = DATA + 0x20_0000;
    const OC898_MAGIC: u32 = 0x0898_0898;
    static OC898_PREV_UPDATE: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake for `NetFEState::Update` (0x1a8770): logs the call.
    fn oc898_fake_update(c: &mut Cpu) {
        if c.bus.read32(OC898_GATE) != OC898_MAGIC {
            return fakes::fall_back(c, &OC898_PREV_UPDATE, ee::s03::f_001a8770__generated);
        }
        fakes::fake_call(c, 0x001a_8770);
    }

    fn oc898_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A8770),
            oc898_fake_update,
            &OC898_PREV_UPDATE,
        );
    }

    /// A machine from the difftest harness at `entry` (scrambled registers,
    /// the game's GP, the miss and jump hooks).
    fn oc898_machine(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        machine(c, seed, salt, 0);
        c.pc = entry;
    }

    #[test]
    fn c_fe_state_net_ssx_world_update_matches() {
        oc898_install_fakes();
        check(
            ee::s03::f_001c1d10__generated,
            c_fe_state_net_ssx_world_update,
            20,
            &|c, seed| {
                oc898_machine(c, seed, 0x1c_1d10, 0x001c_1d10);
                c.bus.write32(OC898_GATE, OC898_MAGIC);
            },
        );
    }

    #[test]
    fn c_fe_state_net_quick_messages_update_matches() {
        oc898_install_fakes();
        check(
            ee::s03::f_001c21a0__generated,
            c_fe_state_net_quick_messages_update,
            20,
            &|c, seed| {
                oc898_machine(c, seed, 0x1c_21a0, 0x001c_21a0);
                c.bus.write32(OC898_GATE, OC898_MAGIC);
            },
        );
    }

    #[test]
    fn c_fe_state_net_ssx_world_v25_matches() {
        check(
            ee::s03::f_001c1d30__generated,
            c_fe_state_net_ssx_world_v25,
            20,
            &|c, seed| {
                oc898_machine(c, seed, 0x1c_1d30, 0x001c_1d30);
                c.g[A1][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_net_quick_messages_v25_matches() {
        check(
            ee::s03::f_001c21c0__generated,
            c_fe_state_net_quick_messages_v25,
            20,
            &|c, seed| {
                oc898_machine(c, seed, 0x1c_21c0, 0x001c_21c0);
                c.g[A1][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    /// Fixture: `self` at DATA+0x1000 with its message slots, `text` at
    /// DATA+0x2000 whose hash (+0x38) is random (so the username/Menu paths
    /// rarely match: no hash oracle here), and a vtable at DATA+0x2800 whose
    /// method is 0 (a call to 0 lands in the miss hook).
    #[test]
    fn c_fe_state_net_quick_messages_on_widget_created_matches() {
        check(
            ee::s03::f_001c2298__generated,
            c_fe_state_net_quick_messages_on_widget_created,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c_2298, 0);
                c.pc = 0x001c_2298;
                let this = DATA + 0x1000;
                let text = DATA + 0x2000;
                let vtable = DATA + 0x2800;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = text as u64;
                c.bus.write32(text + 0x38, r.next_u32());
                c.bus.write32(text + 0x8, vtable);
                c.bus.write32(text + 0x14, r.next_u32());
                c.bus.write32(vtable + 0x4c, 0);
            },
        );
    }
}
