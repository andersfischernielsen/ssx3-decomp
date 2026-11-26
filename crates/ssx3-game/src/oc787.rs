//! Staging module oc787 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc787`.

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
        0x00182db8,
        cfe_state_char_setup_init_rider_preview,
        "cFEStateCharSetup_InitRiderPreview",
    ),
    (0x002ad9c8, bank_monitor_base_ctor, "BankMonitor_BaseCtor"),
    (0x0039e4a0, ui_state_shutdown, "UIState_Shutdown"),
    (0x003d6840, spch_init_state, "SPCH_InitState"),
    (0x001a1dc0, fe_module_enter, "FEModule_Enter"),
    (
        0x002871b0,
        ssx_audio_system_reset_race_sounds,
        "SSXAudioSystem_ResetRaceSounds",
    ),
];

/// `void cFEStateCharSetup_InitRiderPreview(cFEStateCharSetup* self)` (0x182db8): finds the
/// preview widget of the screen (`self+0x40`) by its hash name, points it at the short name
/// of the chosen character (`self+0x44`), sets up the preview camera (position/target/
/// fov in the world's camera at `world+0x7c`), activates the rider preview, marks
/// `self+0x48` and updates it.
pub fn cfe_state_char_setup_init_rider_preview(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 66;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = 0x460000;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[RA][0] = sx(0x00182ddc);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27a0));
        call_ptr(c, 0x00317670, 0x00182ddc)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x00182de8);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x00182de8)?;
        c.g[RA][0] = sx(0x00182df0);
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0014dc80, 0x00182df0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00182dfc);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014dd58, 0x00182dfc)?;
        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
        c.g[RA][0] = sx(0x00182e08);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x00147488, 0x00182e08)?;
        c.g[S0][0] = sx((c.g[V0][0] as u32) << 24);
        c.g[RA][0] = sx(0x00182e14);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0014dc80, 0x00182e14)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00182e20);
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0014dd58, 0x00182e20)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x00182e30);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014eec8, 0x00182e30)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00182e3c);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x003a0c50, 0x00182e3c)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 1);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[AT][0] = 0x43480000;
        c.mtc1(1, 0);
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.sw(c.g[SP][0] as u32, 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
        c.g[RA][0] = sx(0x00182e84);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
        call_ptr(c, 0x0015e050, 0x00182e84)?;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5a0c));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x1;
        c.g[RA][0] = sx(0x00182e98);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x10));
        Callee {
            addr: 0x001831d0,
            f: ee::s02::cFEStateCharSetup_SetRiderPreviewActive,
        }
        .call(c, 0x00182e98)?;
        c.g[V0][0] = 0x1;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x00182ea8);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
        Callee {
            addr: 0x00182ec0,
            f: ee::s02::cFEStateCharSetup_UpdateRiderPreview,
        }
        .call(c, 0x00182ea8)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `cMonitor* BankMonitor_BaseCtor(cMonitor* self, int inchrg, int heap)` (0x2ad9c8):
/// builds the sound-manager reference (when `inchrg`), the play params, copies the 0x20-byte
/// vtable-like block from 0x483bc0, clears the counters and initialises the 48 bank timers
/// (0x2c bytes each, from `+0x98`); registers `self` in the global at `gp+0x688`.
pub fn bank_monitor_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ada04: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = c.g[A2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A0][0];
            let t = c.g[S1][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            if t {
                break 'b_2ada04;
            } else {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8dc));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002ada04);
                c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                Callee {
                    addr: 0x002ade88,
                    f: ee::s08::SoundManagerRef_ctor,
                }
                .call(c, 0x002ada04)?;
                break 'b_2ada04;
            }
        }
        'b_2ada68: {
            c.cycles += 9;
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x002ada14);
            c.g[A1][0] = 0u64;
            Callee {
                addr: 0x002a9350,
                f: ee::s08::SndPlayParams_ctor,
            }
            .call(c, 0x002ada14)?;
            c.g[V1][0] = 0x480000;
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3bc0));
            let t = c.g[S1][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
            if t {
                break 'b_2ada68;
            } else {
                c.cycles += 16;
                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x3bc0));
                c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd(c.g[SP][0] as u32, c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8dc));
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                break 'b_2ada68;
            }
        }
        c.cycles += 10;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x88), 0u64);
        c.g[V0][0] = 0x480000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3ba8));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8c), 0u64);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8d8), c.g[V0][0]);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x98));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x90), 0u64);
        c.g[S1][0] = 0x2f;
        c.g[S3][0] = 0xffffffffffffffff;
        'l_2ada90: loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002ada9c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            Callee {
                addr: 0x002ad848,
                f: ee::s08::BankMonitor_TimerInit,
            }
            .call(c, 0x002ada9c)?;
            let t = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002ada90);
                }
                continue 'l_2ada90;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.g[S2][0];
                c.sw((c.g[GP][0] as u32).wrapping_add(0x688), c.g[S2][0]);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void UIState_Shutdown(cUIState* self)` (0x39e4a0): deletes all entries of the
/// state's UI list (`self+0x24`).
pub fn ui_state_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0039e4b0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24));
        Callee {
            addr: 0x00397948,
            f: ee::s13::UIList_DeleteAll,
        }
        .call(c, 0x0039e4b0)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `int SPCH_InitState(a0, a1, a2)` (0x3d6840): initialises the speech system state:
/// globals, random seed, sentence/time-offset/event/bank/queue resets, the eight
/// channel slots; returns 1.
pub fn spch_init_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[V0][0] = 0x450000;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sw((c.g[V0][0] as u32).wrapping_sub(0x70), c.g[A0][0]);
        c.g[T0][0] = 0x450000;
        c.sq(31, c.g[SP][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x70));
        c.sw((c.g[GP][0] as u32).wrapping_add(0x1748), c.g[A2][0]);
        c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x58));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
        c.g[V1][0] = 0x3d0000;
        c.sw((c.g[T0][0] as u32).wrapping_sub(0x58), 0u64);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x76f0));
        c.sw((c.g[A3][0] as u32).wrapping_add(0x8), 0u64);
        c.g[A0][0] = c.g[A1][0];
        c.sw((c.g[GP][0] as u32).wrapping_add(0x173c), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[A3][0] as u32).wrapping_add(0x4), 0u64);
        c.g[RA][0] = sx(0x003d688c);
        c.sw((c.g[GP][0] as u32).wrapping_add(0x1744), c.g[A1][0]);
        Callee {
            addr: 0x003db790,
            f: ee::s14::SPCH_SeedRandom,
        }
        .call(c, 0x003d688c)?;
        c.g[RA][0] = sx(0x003d6894);
        Callee {
            addr: 0x003d9a40,
            f: ee::s14::SPCH_ResetSentence,
        }
        .call(c, 0x003d6894)?;
        c.g[RA][0] = sx(0x003d689c);
        c.g[A0][0] = 0u64;
        Callee {
            addr: 0x003da3c0,
            f: ee::s14::SPCH_SetTimeOffset,
        }
        .call(c, 0x003d689c)?;
        c.g[V0][0] = 0x450000;
        c.g[V1][0] = 0x7;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6a0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
        'l_3d68b0: loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x003d68b0);
                }
                continue 'l_3d68b0;
            } else {
                c.cycles += 7;
                c.g[RA][0] = sx(0x003d68d4);
                Callee {
                    addr: 0x003d8278,
                    f: ee::s14::SPCH_ClearEventTables,
                }
                .call(c, 0x003d68d4)?;
                c.g[V0][0] = 0x450000;
                c.g[T0][0] = 0xffff;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x660));
                c.g[A1][0] = 0u64;
                c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                'l_3d68e8: loop {
                    c.cycles += 10;
                    c.g[V0][0] = c.g[A1][0] << 32;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[A1][0] = c.g[A1][0].wrapping_add(0x1);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[A0][0] = ((c.g[A1][0] as i64) < 8i64) as u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sh(c.g[V1][0] as u32, c.g[T0][0]);
                    let t = c.g[A0][0] != 0u64;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003d68e8);
                        }
                        continue 'l_3d68e8;
                    } else {
                        c.cycles += 11;
                        c.g[RA][0] = sx(0x003d6918);
                        Callee {
                            addr: 0x003d6d40,
                            f: ee::s14::SPCH_ResetBankTable,
                        }
                        .call(c, 0x003d6918)?;
                        c.g[RA][0] = sx(0x003d6920);
                        Callee {
                            addr: 0x003d71d8,
                            f: ee::s14::SPCH_ResetQueue,
                        }
                        .call(c, 0x003d6920)?;
                        c.g[V1][0] = 0x1780000;
                        c.lq(31, c.g[SP][0] as u32);
                        c.g[V1][0] = c.g[V1][0] | 0x9a34;
                        c.g[V0][0] = 0x1;
                        c.sw((c.g[GP][0] as u32).wrapping_add(0x1730), c.g[V1][0]);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `void FEModule_Enter(FEModule* self)` (0x1a1dc0): when the module was left
/// active (`gp-0x1738`) resets three BE interface slots and clears a 0x20-byte block, then
/// clears the module counters, calls the screen's activation methods and
/// `FEFlowState_Init`.
pub fn fe_module_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a1e2c: {
            c.cycles += 6;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1738));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = c.g[A0][0];
            if t {
                break 'b_1a1e2c;
            } else {
                c.cycles += 21;
                c.g[RA][0] = sx(0x001a1de0);
                call_ptr(c, 0x0014dc80, 0x001a1de0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a1dec);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014de28, 0x001a1dec)?;
                c.g[RA][0] = sx(0x001a1df4);
                call_ptr(c, 0x0014dc80, 0x001a1df4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a1e00);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014de28, 0x001a1e00)?;
                c.g[RA][0] = sx(0x001a1e08);
                call_ptr(c, 0x0014dc80, 0x001a1e08)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a1e14);
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0014de28, 0x001a1e14)?;
                c.g[A0][0] = 0x530000;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                c.g[RA][0] = sx(0x001a1e28);
                c.g[A2][0] = 0x20;
                call_ptr(c, 0x003e6448, 0x001a1e28)?;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1738), 0u64);
                break 'b_1a1e2c;
            }
        }
        c.cycles += 36;
        c.g[V0][0] = 0xb0000;
        c.g[A0][0] = 0u64;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x5adc), 0u64);
        c.g[RA][0] = sx(0x001a1e44);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x5ad8), 0u64);
        call_ptr(c, 0x0014de68, 0x001a1e44)?;
        c.g[RA][0] = sx(0x001a1e4c);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        call_ptr(c, 0x00369ff8, 0x001a1e4c)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[RA][0] = sx(0x001a1e58);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
        call_ptr(c, 0x00195e58, 0x001a1e58)?;
        c.sw((c.g[GP][0] as u32).wrapping_add(0x1234), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_add(0x14e8), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_add(0x12d8), 0u64);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x78));
        c.sw(c.g[SP][0] as u32, 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x001a1e8c);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, t, 0x001a1e8c)?;
        c.g[A0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1d90));
        c.g[RA][0] = sx(0x001a1e98);
        c.g[A1][0] = 0x56;
        Callee {
            addr: 0x001a0648,
            f: ee::s03::FEFlowState_Init,
        }
        .call(c, 0x001a1e98)?;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001a1eac);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001a1eac)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void SSXAudioSystem_ResetRaceSounds(SSXAudioSystem* self)` (0x2871b0): clears the rider
/// air flags, sets the fade mode, and per race-manager entry calls the two reset hooks;
/// then fades out the monitor with the stored fade time.
pub fn ssx_audio_system_reset_race_sounds(c: &mut Cpu) {
    guest(c, |c| {
        'b_28725c: {
            c.cycles += 26;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x002871d4);
            c.g[S2][0] = 0u64;
            Callee {
                addr: 0x002948a0,
                f: ee::s07::SSXAudioSystem_ClearRiderAirFlags,
            }
            .call(c, 0x002871d4)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x002871e0);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0029b3c0, 0x002871e0)?;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 13);
            c.g[A1][0] = 0x3;
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 12);
            c.g[RA][0] = sx(0x002871fc);
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x00287f00,
                f: ee::s07::SSXAudioSystem_SetFadeMode,
            }
            .call(c, 0x002871fc)?;
            c.g[RA][0] = sx(0x00287204);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x21e4));
            call_ptr(c, 0x002b6900, 0x00287204)?;
            c.g[RA][0] = sx(0x0028720c);
            Callee {
                addr: 0x0028b1d8,
                f: ee::s07::SSXAudioSystem_GetRaceMan,
            }
            .call(c, 0x0028720c)?;
            c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
            if (c.g[S3][0] as i64) <= 0 {
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x59e4));
                break 'b_28725c;
            } else {
                'l_287218: loop {
                    c.cycles += 16;
                    c.g[RA][0] = sx(0x00287220);
                    c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                    Callee {
                        addr: 0x0028b1d8,
                        f: ee::s07::SSXAudioSystem_GetRaceMan,
                    }
                    .call(c, 0x00287220)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[RA][0] = sx(0x00287234);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00296e20, 0x00287234)?;
                    c.g[RA][0] = sx(0x0028723c);
                    Callee {
                        addr: 0x0028b1d8,
                        f: ee::s07::SSXAudioSystem_GetRaceMan,
                    }
                    .call(c, 0x0028723c)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0028724c);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    call_ptr(c, 0x00297438, 0x0028724c)?;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00287218);
                        }
                        continue 'l_287218;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x59e4));
                        break 'b_28725c;
                    }
                }
            }
        }
        'b_287288: {
            c.cycles += 3;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x6074), 0u64);
            let t = (c.g[V0][0] as i64) <= 0;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x6078), 0u64);
            if t {
                break 'b_287288;
            } else {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
                c.g[A2][0] = 0x1;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x59e0));
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x6074));
                c.g[RA][0] = sx(0x00287284);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                call_ptr(c, 0x002ad5f0, 0x00287284)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x59e4), 0u64);
                break 'b_287288;
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{fill, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc787's callee fakes in a machine's RAM.
    pub(crate) const OC787_CTRL: u32 = DATA + 0x32_0000;
    pub(crate) const OC787_MAGIC: u32 = 0x0787_f4ce;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc787_mail(addr: u32) -> u32 {
        DATA + 0x32_1000 + (addr & 0xffc)
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc787_arm(c: &mut Cpu) {
        c.bus.write32(OC787_CTRL, OC787_MAGIC);
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc787_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc787_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc787_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc787_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 3) as u64;
        }
    }

    macro_rules! oc787_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC787_CTRL) != OC787_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc787_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc787_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc787_fakes!(
        (
            oc787_fk_14dc80,
            OC787_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc787_fk_14dd58,
            OC787_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc787_fk_14eec8,
            OC787_PREV_14EEC8,
            ee::s01::OVERRIDE_0014EEC8,
            ee::s01::f_0014eec8__generated,
            0x0014_eec8
        ),
        (
            oc787_fk_15e050,
            OC787_PREV_15E050,
            ee::s01::OVERRIDE_0015E050,
            ee::s01::f_0015e050__generated,
            0x0015_e050
        ),
        (
            oc787_fk_14de28,
            OC787_PREV_14DE28,
            ee::s01::OVERRIDE_0014DE28,
            ee::s01::f_0014de28__generated,
            0x0014_de28
        ),
        (
            oc787_fk_14de68,
            OC787_PREV_14DE68,
            ee::s01::OVERRIDE_0014DE68,
            ee::s01::f_0014de68__generated,
            0x0014_de68
        ),
        (
            oc787_fk_182ec0,
            OC787_PREV_182EC0,
            ee::s02::OVERRIDE_00182EC0,
            ee::s02::f_00182ec0__generated,
            0x0018_2ec0
        ),
        (
            oc787_fk_1831d0,
            OC787_PREV_1831D0,
            ee::s02::OVERRIDE_001831D0,
            ee::s02::f_001831d0__generated,
            0x0018_31d0
        ),
        (
            oc787_fk_195e58,
            OC787_PREV_195E58,
            ee::s02::OVERRIDE_00195E58,
            ee::s02::f_00195e58__generated,
            0x0019_5e58
        ),
        (
            oc787_fk_1a0648,
            OC787_PREV_1A0648,
            ee::s03::OVERRIDE_001A0648,
            ee::s03::f_001a0648__generated,
            0x001a_0648
        ),
        (
            oc787_fk_287f00,
            OC787_PREV_287F00,
            ee::s07::OVERRIDE_00287F00,
            ee::s07::f_00287f00__generated,
            0x0028_7f00
        ),
        (
            oc787_fk_28b1d8,
            OC787_PREV_28B1D8,
            ee::s07::OVERRIDE_0028B1D8,
            ee::s07::f_0028b1d8__generated,
            0x0028_b1d8
        ),
        (
            oc787_fk_2948a0,
            OC787_PREV_2948A0,
            ee::s07::OVERRIDE_002948A0,
            ee::s07::f_002948a0__generated,
            0x0029_48a0
        ),
        (
            oc787_fk_296e20,
            OC787_PREV_296E20,
            ee::s08::OVERRIDE_00296E20,
            ee::s08::f_00296e20__generated,
            0x0029_6e20
        ),
        (
            oc787_fk_297438,
            OC787_PREV_297438,
            ee::s08::OVERRIDE_00297438,
            ee::s08::f_00297438__generated,
            0x0029_7438
        ),
        (
            oc787_fk_29b3c0,
            OC787_PREV_29B3C0,
            ee::s08::OVERRIDE_0029B3C0,
            ee::s08::f_0029b3c0__generated,
            0x0029_b3c0
        ),
        (
            oc787_fk_2ad5f0,
            OC787_PREV_2AD5F0,
            ee::s08::OVERRIDE_002AD5F0,
            ee::s08::f_002ad5f0__generated,
            0x002a_d5f0
        ),
        (
            oc787_fk_2ad848,
            OC787_PREV_2AD848,
            ee::s08::OVERRIDE_002AD848,
            ee::s08::f_002ad848__generated,
            0x002a_d848
        ),
        (
            oc787_fk_2ade88,
            OC787_PREV_2ADE88,
            ee::s08::OVERRIDE_002ADE88,
            ee::s08::f_002ade88__generated,
            0x002a_de88
        ),
        (
            oc787_fk_2b6900,
            OC787_PREV_2B6900,
            ee::s08::OVERRIDE_002B6900,
            ee::s08::f_002b6900__generated,
            0x002b_6900
        ),
        (
            oc787_fk_317670,
            OC787_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc787_fk_369ff8,
            OC787_PREV_369FF8,
            ee::s12::OVERRIDE_00369FF8,
            ee::s12::f_00369ff8__generated,
            0x0036_9ff8
        ),
        (
            oc787_fk_39d860,
            OC787_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc787_fk_397948,
            OC787_PREV_397948,
            ee::s13::OVERRIDE_00397948,
            ee::s13::f_00397948__generated,
            0x0039_7948
        ),
        (
            oc787_fk_3a0c70,
            OC787_PREV_3A0C70,
            ee::s13::OVERRIDE_003A0C70,
            ee::s13::f_003a0c70__generated,
            0x003a_0c70
        ),
        (
            oc787_fk_3d9a40,
            OC787_PREV_3D9A40,
            ee::s14::OVERRIDE_003D9A40,
            ee::s14::f_003d9a40__generated,
            0x003d_9a40
        ),
        (
            oc787_fk_3e6448,
            OC787_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
    );

    /// Object area of the fixtures (inside `DATA..DATA+0xfffff`; the fakes'
    /// pointer results land in `DATA..DATA+0x3fff0`, which the fixtures only
    /// use as opaque pointers).
    pub(crate) const OC787_OBJ: u32 = DATA + 0x1000;
    /// Fixed fixture structures above the object area.
    pub(crate) const OC787_AUX: u32 = DATA + 0xc_0000;
    pub(crate) const OC787_VT: u32 = DATA + 0xe_0000;
    pub(crate) const OC787_VT2: u32 = DATA + 0xe_1000;

    /// A vtable-like record at `at` whose slot `off` is `{delta, fake method}`.
    pub(crate) fn oc787_vtable(c: &mut Cpu, at: u32, off: u32, delta: u16) {
        c.bus.write16(at + off, delta);
        c.bus.write32(at + off + 4, fakes::FAKE);
    }

    /// `cFEStateCharSetup_InitRiderPreview`: the user number byte (negative,
    /// zero, other) at `+0x44`, a world whose camera is a plain record, and
    /// the unwinds of its calls (the widget text setter 0x3a0c50 is the real
    /// wrapper, its worker 0x3a0c70 a fake).
    #[test]
    fn cfe_state_char_setup_init_rider_preview_matches() {
        oc787_install_fakes();
        fakes::covered(
            ee::s02::f_00182db8__generated,
            cfe_state_char_setup_init_rider_preview,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x787a, 12);
                oc787_arm(c);
                c.pc = 0x0018_2db8;
                c.g[A0][0] = sx(OC787_OBJ);
                c.bus.write32(OC787_OBJ + 0x40, OC787_AUX);
                c.bus.write8(
                    OC787_OBJ + 0x44,
                    [0u8, 1, 2, 0xff, 0x80][(seed % 5) as usize],
                );
                c.bus.write32(OC787_OBJ + 0x48, r.next_u32());
                c.bus.write32(GAME_GP - 0x848, OC787_AUX + 0x100);
                c.bus.write32(OC787_AUX + 0x100 + 0x7c, OC787_AUX + 0x200);
                c.bus.write32(GAME_GP - 0x5a0c, r.next_u32());
            },
        );
    }

    /// `BankMonitor_BaseCtor`: with and without the sound-manager reference
    /// (`a1`), random heap argument and vtable block at 0x483bc0, unwinds in
    /// the constructor calls and in the 48 timer initialisations.
    #[test]
    fn bank_monitor_base_ctor_matches() {
        oc787_install_fakes();
        fakes::covered(
            ee::s08::f_002ad9c8__generated,
            bank_monitor_base_ctor,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x787b, 30);
                oc787_arm(c);
                c.pc = 0x002a_d9c8;
                c.g[A0][0] = sx(OC787_OBJ);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    sx(r.next_u32() | 1)
                };
                c.g[A2][0] = sx(r.next_u32());
                fill(c, &mut r, 0x0048_3bc0, 8);
                fill(c, &mut r, OC787_OBJ, 0x30);
            },
        );
    }

    /// `UIState_Shutdown`: deletes the UI list of the state (the list delete is
    /// a fake), with and without an unwind.
    #[test]
    fn ui_state_shutdown_matches() {
        oc787_install_fakes();
        fakes::covered(
            ee::s13::f_0039e4a0__generated,
            ui_state_shutdown,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x787c, 1);
                oc787_arm(c);
                c.pc = 0x0039_e4a0;
                c.g[A0][0] = sx(OC787_OBJ + (r.next_u32() & 0xff0));
            },
        );
    }

    /// `SPCH_InitState`: random state arguments; the reset functions are the
    /// real ones (on zeroed tables), `SPCH_ResetSentence` is a fake that may
    /// unwind.
    #[test]
    fn spch_init_state_matches() {
        oc787_install_fakes();
        fakes::covered(
            ee::s14::f_003d6840__generated,
            spch_init_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x787d, 1);
                oc787_arm(c);
                c.pc = 0x003d_6840;
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    /// `FEModule_Enter`: the "left active" flag at `gp-0x1738` set or clear
    /// (the three interface resets and the 0x20-byte clear), two vtables whose
    /// methods are fake calls, and the unwinds of every call.
    #[test]
    fn fe_module_enter_matches() {
        oc787_install_fakes();
        fakes::covered(
            ee::s03::f_001a1dc0__generated,
            fe_module_enter,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x787e, 14);
                oc787_arm(c);
                c.pc = 0x001a_1dc0;
                c.g[A0][0] = sx(OC787_OBJ);
                c.bus.write32(
                    GAME_GP - 0x1738,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(GAME_GP - 0x854, OC787_AUX);
                c.bus.write32(GAME_GP - 0x848, OC787_AUX + 0x8000);
                c.bus.write32(OC787_AUX + 0x8000 + 0x8c, r.next_u32());
                c.bus.write32(OC787_AUX + 0x10d8, OC787_VT);
                oc787_vtable(c, OC787_VT, 0x78, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(OC787_OBJ, OC787_VT2);
                oc787_vtable(c, OC787_VT2, 0x40, (r.next_u32() & 0xfc) as u16);
            },
        );
    }

    /// `SSXAudioSystem_ResetRaceSounds`: the race manager has 0..3 entries
    /// (or a negative count), the monitor fade count at `+0x59e4` is positive
    /// or not, and any call may unwind.
    #[test]
    fn ssx_audio_system_reset_race_sounds_matches() {
        oc787_install_fakes();
        fakes::covered(
            ee::s07::f_002871b0__generated,
            ssx_audio_system_reset_race_sounds,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x787f, 12);
                oc787_arm(c);
                c.pc = 0x0028_71b0;
                c.g[A0][0] = sx(OC787_OBJ);
                let raceman = OC787_AUX;
                oc787_force(c, 0x0028_b1d8, raceman);
                c.bus.write32(
                    raceman + 0x78,
                    [0u32, 1, 2, 3, 0xffff_ffff][(seed % 5) as usize],
                );
                fill(c, &mut r, raceman + 0x28, 4);
                c.bus.write32(
                    OC787_OBJ + 0x59e4,
                    [0u32, 1, 5, 0xffff_ffff][((seed / 5) % 4) as usize],
                );
                c.bus.write32(OC787_OBJ + 0x59e0, r.next_u32());
                c.bus.write32(OC787_OBJ + 0x118, OC787_AUX + 0x400);
                c.bus.write32(OC787_AUX + 0x400, OC787_AUX + 0x800);
                c.bus.write32(GAME_GP + 0x21e4, r.next_u32());
            },
        );
    }
}
