//! Staging module oc513 (wave wave229): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc513`.

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
    (0x001d_ea20, cfe_popup_option_reset_init, "cFEPopupOptionReset_Init"),
    (
        0x001d_eaa8,
        cfe_popup_option_reset_on_widget_action,
        "cFEPopupOptionReset_OnWidgetAction",
    ),
    (0x001d_eb58, cfe_popup_video_calibration_dtor, "cFEPopupVideoCalibration_dtor"),
    (0x0022_67f0, luno_vm_global_ctors, "LunoVM_GlobalCtors"),
    (0x0022_6810, luno_vm_global_dtors, "LunoVM_GlobalDtors"),
    (0x0022_6830, ssxapp_ctor, "SSXApp_ctor"),
    (0x0022_7fa0, mc_manager_exists_wrapper, "MCManager_Exists_Wrapper"),
    (0x0022_8bb8, ssxapp_v02, "SSXApp_v02"),
    (0x0022_dfd0, section_man_is_event_switch_overlay_done, "SectionMan_IsEventSwitchOverlayDone"),
    (0x0022_e118, section_man_reload_event_and_restart, "SectionMan_ReloadEventAndRestart"),
];

/// `LunoVM_StaticInitDestroy(int, uint)` (0x225de8), called by the Luno
/// static constructor and destructor.
pub(crate) const OC513_LUNO_STATIC_INIT_DESTROY: Callee = Callee {
    addr: 0x0022_5de8,
    f: ssx3_ee::s05::f_00225de8,
};

/// `void cFEPopupVideoCalibration::dtor(void* self)` (0x1deb58): stores the
/// class vtable word 0x46b300 at `self+8`, then calls the base destructor
/// (0x39e390) on `self`.
pub fn cfe_popup_video_calibration_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = 0x0046_b300;
        let vtable = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        call_ptr(c, 0x0039_e390, 0x001d_eb74)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupOptionReset::onCreateScreen(void* self)` (0x1dea20): looks up
/// the hash of the option-reset string (0x45dd20) and stores it at `self+0xc`,
/// then fills the widget at `self+0x40` with the result of 0x397cc0 and, when
/// that is non-zero, calls 0x39c870 on it.
pub fn cfe_popup_option_reset_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x0045_dd20;
        call_ptr(c, 0x0031_7670, 0x001d_ea3c)?;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A2][0] = c.g[V0][0];
        let hash = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sw(c, hash);
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001d_ea54)?;
        let widget = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x40)).sw(c, widget);
        if widget != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001d_ea6c)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The body of `cFEPopupOptionReset::onWidgetEvent` (0x1deaa8) between its
/// prologue and epilogue. Returns `Ok` on every normal exit (the caller runs
/// the epilogue); `S0` is restored on each path, as the generated code does.
fn cfe_popup_option_reset_on_widget_body(c: &mut Cpu, frame: Frame) -> Flow {
    if c.g[S1][0] == 0 {
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = 5;
    let other_event = c.g[A2][0] != c.g[V0][0];
    frame.restore(c, S0, 0x30);
    if other_event {
        return Ok(());
    }
    c.cycles += 6;
    c.g[A0][0] = 0x004a_0000;
    c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x38)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14e8));
    call_ptr(c, 0x0031_7670, 0x001d_eae4)?;
    if c.g[S0][0] == c.g[V0][0] {
        // The stored id matches the first string: pop the UI state stack.
        c.cycles += 8;
        c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[V0][0] = 1;
        let flag = c.g[V0][0];
        Addr((c.g[S2][0] as u32).wrapping_add(0x48)).sb(c, flag);
        c.g[A1][0] = 1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f190, 0x001d_eb04)?;
        frame.restore(c, S0, 0x30);
        return Ok(());
    }
    c.g[A0][0] = 0x004a_0000;
    c.cycles += 5;
    c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x38)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14f0));
    call_ptr(c, 0x0031_7670, 0x001d_eb18)?;
    let mismatch = c.g[S0][0] != c.g[V0][0];
    frame.restore(c, S0, 0x30);
    if mismatch {
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x10)).lw(c);
    c.g[A1][0] = 1;
    Addr((c.g[S2][0] as u32).wrapping_add(0x48)).sb(c, 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    call_ptr(c, 0x0039_f190, 0x001d_eb34)?;
    c.cycles += 1;
    frame.restore(c, S0, 0x30);
    Ok(())
}

/// `void cFEPopupOptionReset::onWidgetEvent(void* self, int event, int id)`
/// (0x1deaa8). Compares the word at `id+0x38` with the hashes of two option
/// strings (0x4a14e8, 0x4a14f0); a match pops the UI state stack at
/// `self+0x10` (0x39f190) and sets the byte at `self+0x48`.
pub fn cfe_popup_option_reset_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        cfe_popup_option_reset_on_widget_body(c, frame)?;
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void luno::LunoVM_GlobalCtors(void)` (0x2267f0): `LunoVM_StaticInitDestroy(1, 0xffff)`.
pub fn luno_vm_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        OC513_LUNO_STATIC_INIT_DESTROY.call(c, 0x0022_6804)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void luno::LunoVM_GlobalDtors(void)` (0x226810): `LunoVM_StaticInitDestroy(0, 0xffff)`.
pub fn luno_vm_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        OC513_LUNO_STATIC_INIT_DESTROY.call(c, 0x0022_6824)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXApp::SSXApp(void* self)` (0x226830): constructs the app manager
/// (0x316cd0), sets the app's fields, clears the 11-word table at
/// `self+0x114` down to `self+0xbc` (the loop runs with event checks), and
/// zeroes three 8-word blocks with `MEM_fill` (0x3e6448).
pub fn ssxapp_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0031_6cd0, 0x0022_6844)?;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xda0)).lw(c);
        c.g[V0][0] = 0x0047_d9c8;
        let base = c.g[S0][0] as u32;
        Addr(base.wrapping_add(0xd4)).sw(c, 0);
        let vtable = c.g[V0][0];
        Addr(base.wrapping_add(0x5c)).sw(c, vtable);
        c.g[A0][0] = 0xa;
        let v1 = c.g[V1][0];
        Addr(base.wrapping_add(0xcc)).sw(c, v1);
        c.g[V0][0] = sx(base.wrapping_add(0x114));
        Addr(base.wrapping_add(0xc4)).sw(c, v1);
        Addr(base.wrapping_add(0xc8)).sw(c, v1);
        Addr(base.wrapping_add(0xd8)).sw(c, 0);
        Addr(base.wrapping_add(0xdc)).sw(c, 0);
        Addr(base.wrapping_add(0xe0)).sw(c, 0);
        Addr(base.wrapping_add(0xe4)).sw(c, 0);
        Addr(base.wrapping_add(0xe8)).sw(c, 0);
        loop {
            c.cycles += 7;
            let slot = c.g[V0][0] as u32;
            Addr(slot).sw(c, 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            let more = (c.g[A0][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_6880);
            }
        }
        c.cycles += 25;
        c.g[V0][0] = 1;
        let base = c.g[S0][0] as u32;
        Addr(base.wrapping_add(0x118)).sw(c, 0);
        Addr(base.wrapping_add(0x74)).sw(c, 0);
        c.g[A0][0] = sx(base.wrapping_add(0xa8));
        Addr(base.wrapping_add(0x68)).sw(c, 0);
        c.g[A1][0] = 0;
        Addr(base.wrapping_add(0x6c)).sw(c, 0);
        c.g[A2][0] = 8;
        Addr(base.wrapping_add(0x70)).sw(c, 0);
        let one = c.g[V0][0];
        Addr(base.wrapping_add(0x64)).sw(c, one);
        Addr(base.wrapping_add(0x60)).sw(c, one);
        call_ptr(c, 0x003e_6448, 0x0022_68cc)?;
        let base = c.g[S0][0] as u32;
        c.g[A0][0] = sx(base.wrapping_add(0xb8));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, 0x003e_6448, 0x0022_68dc)?;
        let base = c.g[S0][0] as u32;
        c.g[A0][0] = sx(base.wrapping_add(0xb0));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, 0x003e_6448, 0x0022_68ec)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool cSSXApp::hasMCManager(void)` (0x227fa0): tail-calls
/// `cMCOverlayManager_hasManager` (0x23c850) through the function table.
pub fn mc_manager_exists_wrapper(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0023_c850), 0x0022_7fb0);
}

/// `void cSSXApp::v02(void* self)` (0x228bb8): tears down the world trigger
/// manager (0x2b4b48), the audio system (0x284c28) and the app base (0x317400),
/// then calls the app's virtual slot at `+0x3bc` through the table.
pub fn ssxapp_v02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002b_4b48, 0x0022_8bcc)?;
        call_ptr(c, 0x0028_4c28, 0x0022_8bd4)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7400, 0x0022_8bdc)?;
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a90)).lw(c);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10d8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x3b8)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x3bc)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0022_8bf4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool SectionMan::isPreLoadModeDone(void* self)` (0x22dfd0): the overlay
/// state of the section manager at `self+0x1b0` (through 0x2312d8).
pub fn section_man_is_event_switch_overlay_done(c: &mut Cpu) {
    tail_wrapper(c, Some(0x1b0), Target::Table(0x0023_12d8), 0x0022_dfe0);
}

/// `void SectionMan::gotoLocation(void* self, int location)` (0x22e118; the
/// list calls it ReloadEventAndRestart): calls `cBE_getBE` (0x14dc80) and
/// 0x14dd58, then unless the byte at 0x535c11 is set, loads the event
/// (`SectionMan_LoadEvent`, 0x22d088) and sets `self+0x1cc` to 1.
pub fn section_man_reload_event_and_restart(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0022_e134)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0022_e140)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c11)).lb(c);
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 1;
            Callee {
                addr: 0x0022_d088,
                f: ssx3_ee::s05::f_0022d088,
            }
            .call(c, 0x0022_e160)?;
            c.g[V0][0] = 1;
            let one = c.g[V0][0];
            Addr((c.g[S1][0] as u32).wrapping_add(0x1cc)).sw(c, one);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fresh, scramble, Rng, DATA, GAME_GP};

    /// Scrambled machine with the test object at `DATA` in `A0`.
    fn object_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.g[A0][0] = sx(DATA);
        r
    }

    /// The string hash the generated code computes for `addr` on empty RAM.
    fn string_hash(addr: u32) -> u64 {
        let mut m = fresh();
        m.g[A0][0] = addr as u64;
        m.g[RA][0] = 0x0012_3450;
        m.pc = 0x0031_7670;
        ee::s10::f_00317670(&mut m);
        m.g[V0][0]
    }

    #[test]
    fn cfe_popup_video_calibration_dtor_matches() {
        check(
            ee::s04::f_001deb58__generated,
            cfe_popup_video_calibration_dtor,
            40,
            &|c, seed| {
                let _ = object_machine(c, seed, 0xeb58);
            },
        );
    }

    #[test]
    fn cfe_popup_option_reset_init_matches() {
        check(
            ee::s04::f_001dea20__generated,
            cfe_popup_option_reset_init,
            60,
            &|c, seed| {
                let mut r = object_machine(c, seed, 0xea20);
                c.bus.write32(DATA + 0x10, DATA + 0x2000);
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0x2000 + 0x40, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn cfe_popup_option_reset_on_widget_action_matches() {
        let h1 = string_hash(0x004a_14e8) as u32;
        let h2 = string_hash(0x004a_14f0) as u32;
        check(
            ee::s04::f_001deaa8__generated,
            cfe_popup_option_reset_on_widget_action,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0xeaa8, 0);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(DATA) };
                c.g[A2][0] = if seed % 3 == 0 { 5 } else { r.next_u32() as u64 };
                c.g[A0][0] = sx(DATA + 0x100);
                let stored = match seed % 4 {
                    0 => h1,
                    1 => h2,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + 0x38, stored);
                c.bus.write32(DATA + 0x110, DATA + 0x2000);
            },
        );
    }

    #[test]
    fn luno_vm_global_ctors_dtors_match() {
        for (gen, hand, salt) in [
            (
                ee::s05::f_002267f0__generated as fn(&mut Cpu),
                luno_vm_global_ctors as fn(&mut Cpu),
                0x67f0,
            ),
            (ee::s05::f_00226810__generated, luno_vm_global_dtors, 0x6810),
        ] {
            check(gen, hand, 30, &|c, seed| {
                let _ = machine(c, seed, salt, 0);
            });
        }
    }

    #[test]
    fn ssxapp_ctor_matches() {
        check(ee::s05::f_00226830__generated, ssxapp_ctor, 40, &|c, seed| {
            let mut r = object_machine(c, seed, 0x6830);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP.wrapping_add(0xda0), r.next_u32());
            c.g[A0][0] = sx(DATA + 0x1000);
        });
    }

    #[test]
    fn mc_manager_exists_wrapper_matches() {
        check(
            ee::s05::f_00227fa0__generated,
            mc_manager_exists_wrapper,
            30,
            &|c, seed| {
                let _ = machine(c, seed, 0x7fa0, 0);
            },
        );
    }

    #[test]
    fn ssxapp_v02_matches() {
        check(ee::s05::f_00228bb8__generated, ssxapp_v02, 40, &|c, seed| {
            let _ = object_machine(c, seed, 0x8bb8);
            c.bus.write32(GAME_GP.wrapping_add(0x2a90), DATA + 0x1000);
            c.bus.write32(DATA + 0x1000 + 0x10d8, DATA + 0x2000);
            c.bus.write16(DATA + 0x2000 + 0x3b8, (seed % 2) as u16);
            c.bus.write32(DATA + 0x2000 + 0x3bc, 0x0070_0000);
        });
    }

    #[test]
    fn section_man_is_event_switch_overlay_done_matches() {
        check(
            ee::s05::f_0022dfd0__generated,
            section_man_is_event_switch_overlay_done,
            30,
            &|c, seed| {
                let mut r = object_machine(c, seed, 0xdfd0);
                c.bus.write32(DATA + 0x1b0, r.next_u32());
            },
        );
    }

    #[test]
    fn section_man_reload_event_and_restart_matches() {
        check(
            ee::s05::f_0022e118__generated,
            section_man_reload_event_and_restart,
            60,
            &|c, seed| {
                let mut r = object_machine(c, seed, 0xe118);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
                c.bus.write8(0x0053_0000 + 0x5c11, (seed % 2) as u8);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }
}
