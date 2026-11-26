//! Staging module oc796 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc796`.

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
        0x002a_fd10,
        speech_instance_cancel_pending,
        "SpeechInstance_CancelPending",
    ),
    (
        0x0018_2c08,
        cfe_state_char_setup_shutdown,
        "cFEStateCharSetup_Shutdown",
    ),
    (
        0x002b_68d0,
        world_trigger_manager_reset,
        "WorldTriggerManager_Reset",
    ),
    (
        0x002b_1428,
        speech_system_clear_event_contexts,
        "SpeechSystem_ClearEventContexts",
    ),
    (
        0x0029_7f70,
        ssx_audio_system_clear_object_loops,
        "SSXAudioSystem_ClearObjectLoops",
    ),
    (
        0x0028_f700,
        ssx_audio_system_load_loading_screen_bank,
        "SSXAudioSystem_LoadLoadingScreenBank",
    ),
];

/// `SpeechInstance::CancelPending(this, channel)` (0x2afd10): dequeues
/// `channel` from the speech channel queue (`SPCH_DequeueChannel`); when that
/// reports something dequeued, clears the pending flag at `this + 0x900`.
pub fn speech_instance_cancel_pending(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, 0x003d_7ec8, 0x002a_fd28)?;
        if c.g[V0][0] != 0u64 {
            c.sw((c.g[S0][0] as u32).wrapping_add(0x900), 0u64);
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `cFEStateCharSetup::Shutdown(this)` (0x182c08): turns the rider preview
/// off (`SetRiderPreviewActive(this, 0)`), then runs the base `UIState`
/// shutdown (`0x39e4a0`, which deletes the state's UI list).
pub fn cfe_state_char_setup_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[A1][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0018_31d0,
            f: ee::s02::f_001831d0,
        }
        .call(c, 0x0018_2c20)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e4a0, 0x0018_2c28)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `WorldTriggerManager::Reset(this)` (0x2b68d0): stops every playing trigger
/// (`StopAll`), then clears the played list (`ResetPlayedList`).
pub fn world_trigger_manager_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        Callee {
            addr: 0x002b_5e68,
            f: ee::s08::f_002b5e68,
        }
        .call(c, 0x002b_68e4)?;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x002b_6900,
            f: ee::s08::f_002b6900,
        }
        .call(c, 0x002b_68ec)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `SpeechSystem::ClearEventContexts(this)` (0x2b1428): clears the first word
/// of each of the ten event contexts (0x20 bytes each, from `this + 0x138`
/// down to `this + 0x18`).
pub fn speech_system_clear_event_contexts(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x138));
    c.g[V0][0] = 0x9;
    loop {
        c.cycles += 7;
        c.sw(c.g[A0][0] as u32, 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        let t = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x002b_1430);
            }
            continue;
        }
        c.cycles += 2;
        ret(c);
        return;
    }
}

/// `SSXAudioSystem::ClearObjectLoops(this)` (0x297f70): clears the first word
/// of each of the thirty object-loop records (0x30 bytes each, from
/// `this + 0x5f70` down to `this`).
pub fn ssx_audio_system_clear_object_loops(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5f70));
    c.g[V0][0] = 0x1d;
    loop {
        c.cycles += 7;
        c.sw(c.g[A0][0] as u32, 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        let t = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x30));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x0029_7f78);
            }
            continue;
        }
        c.cycles += 2;
        ret(c);
        return;
    }
}

/// `SSXAudioSystem::LoadLoadingScreenBank(this)` (0x28f700): loads sound bank
/// 13 (the loading-screen bank) through the bank manager `*(this + 0x118)`,
/// with the bank name at `gp + 0x534` (`SndBankMan_LoadBank(mgr, 13, name, 0)`).
pub fn ssx_audio_system_load_loading_screen_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x534));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A1][0] = 0xd;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x118));
        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
        Callee {
            addr: 0x0028_bc58,
            f: ee::s07::f_0028bc58,
        }
        .call(c, 0x0028_f720)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::DATA;
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc796's callee fakes in a machine's RAM.
    pub(crate) const OC796_CTRL: u32 = DATA + 0x30_0000;
    pub(crate) const OC796_MAGIC: u32 = 0x0796_f4ce;

    pub(crate) fn oc796_mail(addr: u32) -> u32 {
        DATA + 0x30_1000 + (addr & 0xffc)
    }

    pub(crate) fn oc796_arm(c: &mut Cpu) {
        c.bus.write32(OC796_CTRL, OC796_MAGIC);
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc796_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc796_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// the forced result of its mailbox unless it unwound.
    pub(crate) fn oc796_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc796_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail == 1 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc796_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC796_CTRL) != OC796_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc796_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc796_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc796_fakes!(
        (
            oc796_fk_3d7ec8,
            OC796_PREV_3D7EC8,
            ee::s14::OVERRIDE_003D7EC8,
            ee::s14::f_003d7ec8__generated,
            0x003d_7ec8
        ),
        (
            oc796_fk_1831d0,
            OC796_PREV_1831D0,
            ee::s02::OVERRIDE_001831D0,
            ee::s02::f_001831d0__generated,
            0x0018_31d0
        ),
        (
            oc796_fk_397948,
            OC796_PREV_397948,
            ee::s13::OVERRIDE_00397948,
            ee::s13::f_00397948__generated,
            0x0039_7948
        ),
        (
            oc796_fk_2b5e68,
            OC796_PREV_2B5E68,
            ee::s08::OVERRIDE_002B5E68,
            ee::s08::f_002b5e68__generated,
            0x002b_5e68
        ),
        (
            oc796_fk_2b6900,
            OC796_PREV_2B6900,
            ee::s08::OVERRIDE_002B6900,
            ee::s08::f_002b6900__generated,
            0x002b_6900
        ),
        (
            oc796_fk_28b830,
            OC796_PREV_28B830,
            ee::s07::OVERRIDE_0028B830,
            ee::s07::f_0028b830__generated,
            0x0028_b830
        ),
        (
            oc796_fk_28a728,
            OC796_PREV_28A728,
            ee::s07::OVERRIDE_0028A728,
            ee::s07::f_0028a728__generated,
            0x0028_a728
        ),
        (
            oc796_fk_28b320,
            OC796_PREV_28B320,
            ee::s07::OVERRIDE_0028B320,
            ee::s07::f_0028b320__generated,
            0x0028_b320
        ),
    );

    pub(crate) const OC796_OBJ: u32 = DATA + 0x1_0000;

    /// `SpeechInstance_CancelPending`: the dequeue result zero or not, and
    /// an unwind of the call.
    #[test]
    fn speech_instance_cancel_pending_matches() {
        oc796_install_fakes();
        fakes::covered(
            ee::s08::f_002afd10__generated,
            speech_instance_cancel_pending,
            90,
            &|c, seed| {
                let _ = machine(c, seed, 0x796a, 1);
                oc796_arm(c);
                c.pc = 0x002a_fd10;
                c.g[A0][0] = sx(OC796_OBJ);
                c.bus.write32(OC796_OBJ + 0x900, 0x1234_5678);
                if seed % 2 == 0 {
                    oc796_force_zero(c, 0x003d_7ec8);
                }
            },
        );
    }

    /// `cFEStateCharSetup_Shutdown`: the rider preview call, the real
    /// `UIState_Shutdown` (its `UIList_DeleteAll` faked) and the unwinds.
    #[test]
    fn cfe_state_char_setup_shutdown_matches() {
        oc796_install_fakes();
        fakes::covered(
            ee::s02::f_00182c08__generated,
            cfe_state_char_setup_shutdown,
            90,
            &|c, seed| {
                let _ = machine(c, seed, 0x796b, 3);
                oc796_arm(c);
                c.pc = 0x0018_2c08;
                c.g[A0][0] = sx(OC796_OBJ);
            },
        );
    }

    /// `WorldTriggerManager_Reset`: both calls and their unwinds.
    #[test]
    fn world_trigger_manager_reset_matches() {
        oc796_install_fakes();
        fakes::covered(
            ee::s08::f_002b68d0__generated,
            world_trigger_manager_reset,
            90,
            &|c, seed| {
                let _ = machine(c, seed, 0x796c, 2);
                oc796_arm(c);
                c.pc = 0x002b_68d0;
                c.g[A0][0] = sx(OC796_OBJ);
            },
        );
    }

    /// Both clear loops: words zeroed over the whole record array, with
    /// events due inside the loop on some seeds.
    fn oc796_clear_setup(c: &mut Cpu, seed: u32, span: u32) {
        let mut r = machine(c, seed, 0x796d, 0);
        c.g[A0][0] = sx(OC796_OBJ);
        for off in (0..span + 0x40).step_by(4) {
            c.bus.write32(OC796_OBJ + off, r.next_u32() | 1);
        }
        if seed % 4 == 3 {
            c.next_event = c.cycles + 20;
        }
    }

    #[test]
    fn speech_system_clear_event_contexts_matches() {
        crate::difftest::check(
            ee::s08::f_002b1428__generated,
            speech_system_clear_event_contexts,
            40,
            &|c, seed| {
                c.pc = 0x002b_1428;
                oc796_clear_setup(c, seed, 0x140);
            },
        );
    }

    #[test]
    fn ssx_audio_system_clear_object_loops_matches() {
        crate::difftest::check(
            ee::s08::f_00297f70__generated,
            ssx_audio_system_clear_object_loops,
            40,
            &|c, seed| {
                c.pc = 0x0029_7f70;
                oc796_clear_setup(c, seed, 0x5f80);
            },
        );
    }

    /// `SSXAudioSystem_LoadLoadingScreenBank`: the real `SndBankMan_LoadBank`
    /// over faked bank calls (already loaded or not, with unwinds).
    #[test]
    fn ssx_audio_system_load_loading_screen_bank_matches() {
        oc796_install_fakes();
        fakes::covered(
            ee::s07::f_0028f700__generated,
            ssx_audio_system_load_loading_screen_bank,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x796e, 4);
                oc796_arm(c);
                c.pc = 0x0028_f700;
                c.g[A0][0] = sx(OC796_OBJ);
                c.bus.write32(OC796_OBJ + 0x118, OC796_OBJ + 0x1000);
                c.bus.write32(OC796_OBJ + 0x1000, OC796_OBJ + 0x2000);
                c.bus
                    .write32(OC796_OBJ + 0x2000 + 0xacc, OC796_OBJ + 0x3000);
                c.bus.write32(GAME_GP_BANK_NAME, r.next_u32());
                if seed % 2 == 0 {
                    oc796_force_zero(c, 0x0028_b830);
                }
            },
        );
    }

    /// `gp + 0x534` (the loading-screen bank name pointer) with the test gp.
    const GAME_GP_BANK_NAME: u32 = crate::difftest::GAME_GP + 0x534;
}
