//! Staging module oc1500 (wave wave275): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1500`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0017_fe34, feui_input_direction_tail_73, "FEUIInput_IsDirection_tail73"),
    (0x0017_fe3c, feui_input_direction_tail_74, "FEUIInput_IsDirection_tail74"),
    (0x0017_fe44, feui_input_direction_tail_75, "FEUIInput_IsDirection_tail75"),
    (0x0026_af98, path_data_find_near_ai_paths_thunk, "PathData_FindNearAIPaths_Thunk"),
    (0x002c_c018, menu_sys_ctor, "MenuSys_ctor"),
    (0x0011_3d88, race_state_race_enter, "RaceStateRace_Enter"),
    (0x0040_c8b0, func_0040c8b0, "func_0040C8B0"),
];

/// Shared tail of the three `FEUIInput_IsDirection` labels (0x17fe34,
/// 0x17fe3c, 0x17fe44): each sets `a1` to its direction code and jumps to
/// the common exit at 0x17fe68 (a tail call, so no return address is set).
fn feui_input_direction_tail(c: &mut Cpu, direction: u64) {
    c.cycles += 2;
    c.g[A1][0] = direction;
    c.pc = 0x0017_fe68;
    call_indirect(c, 0x0017_fe68);
}

/// `FEUIInput_IsDirection` exit with `a1 = 0x73` (label 0x17fe34).
pub fn feui_input_direction_tail_73(c: &mut Cpu) {
    feui_input_direction_tail(c, 0x73);
}

/// `FEUIInput_IsDirection` exit with `a1 = 0x74` (label 0x17fe3c).
pub fn feui_input_direction_tail_74(c: &mut Cpu) {
    feui_input_direction_tail(c, 0x74);
}

/// `FEUIInput_IsDirection` exit with `a1 = 0x75` (label 0x17fe44).
pub fn feui_input_direction_tail_75(c: &mut Cpu) {
    feui_input_direction_tail(c, 0x75);
}

/// `PathData_FindNearAIPaths_Thunk` (0x26af98): calls
/// `PathData_FindNearAIPaths` with `t0 = 1` (the "enabled only" flag), the
/// arguments passed through unchanged.
pub fn path_data_find_near_ai_paths_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[T0][0] = 0x1;
        Callee {
            addr: 0x0026_afb8,
            f: ee::s07::f_0026afb8,
        }
        .call(c, 0x0026_afa8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `MenuSys::MenuSys(MenuSys* this)` (0x2cc018): clear the first word of
/// the menu, then `MenuSys_Reset(this)`; returns `this`.
pub fn menu_sys_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(c.g[S0][0] as u32).sw(c, 0);
        Callee {
            addr: 0x002c_c070,
            f: ee::s09::f_002cc070,
        }
        .call(c, 0x002c_c030)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `RaceStateRace_Enter` (0x113d88, `cRaceHandler_OnGainState`): when the
/// game comm object (`gp-0x204`) exists, `cGameComm_OnRaceStart(comm)`.
pub fn race_state_race_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
        let no_comm = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !no_comm {
            c.cycles += 2;
            call_ptr(c, 0x0025_8988, 0x0011_3da0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `func_0040C8B0(void*)` (0x40c8b0): `iSignalSema(*(int*)0x453438)`, a tail
/// call (the return address is the caller's).
pub fn func_0040c8b0(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x0045_0000;
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x3438)).lw(c);
    c.pc = 0x0042_3dd0;
    ee::s15::f_00423dd0(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn feui_input_direction_tail_73_matches() {
        fakes::covered(
            ee::s02::f_0017fe34__generated,
            feui_input_direction_tail_73,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x0017_fe34, 1);
            },
        );
    }

    #[test]
    fn feui_input_direction_tail_74_matches() {
        fakes::covered(
            ee::s02::f_0017fe3c__generated,
            feui_input_direction_tail_74,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x0017_fe3c, 1);
            },
        );
    }

    #[test]
    fn feui_input_direction_tail_75_matches() {
        fakes::covered(
            ee::s02::f_0017fe44__generated,
            feui_input_direction_tail_75,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x0017_fe44, 1);
            },
        );
    }

    #[test]
    fn path_data_find_near_ai_paths_thunk_matches() {
        const PATHDATA: u32 = DATA;
        const POINT: u32 = DATA + 0x80;
        const PATHS: u32 = DATA + 0x100;
        const OUT: u32 = DATA + 0x1000;
        check(
            ee::s07::f_0026af98__generated,
            path_data_find_near_ai_paths_thunk,
            120,
            &|c, seed| {
                let mut r = Rng(0x0026_af98 ^ (seed as u64) << 4);
                crate::difftest::scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.bus.write32(GAME_GP - 0x4a4c, 1.0e30f32.to_bits());
                c.bus.write32(GAME_GP - 0x4a48, 2.0e30f32.to_bits());
                let n = seed % 4;
                c.bus.write32(PATHDATA + 0x8, n);
                c.bus.write32(PATHDATA + 0xc, PATHS);
                for k in 0..3 {
                    c.bus.write32(POINT + 4 * k, r.f32(4.0));
                }
                for i in 0..n {
                    let path = PATHS + 0x40 * i;
                    for k in 0..3u32 {
                        let p = f32::from_bits(c.bus.read32(POINT + 4 * k));
                        c.bus.write32(path + 0x1c + 4 * k, (p - 1.0).to_bits());
                        c.bus.write32(path + 0x28 + 4 * k, (p + 1.0).to_bits());
                    }
                    c.bus.write32(path + 0x3c, (r.next_u32() % 3 != 0) as u32);
                }
                c.g[A0][0] = sx(PATHDATA);
                c.g[A1][0] = sx(POINT);
                c.g[A2][0] = sx(OUT);
                c.g[A3][0] = sx(((seed / 4 % 5) as i32 - 1) as u32);
                c.g[T0][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn menu_sys_ctor_matches() {
        check(ee::s09::f_002cc018__generated, menu_sys_ctor, 60, &|c, seed| {
            let mut r = Rng(0x2cc0_18 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }

    /// Gate word for the `cGameComm_OnRaceStart` fake (a unique cell of the
    /// machine's RAM; the fake only answers when it is set).
    const RSR_GATE: u32 = DATA + 0x20_0000;
    const RSR_MAGIC: u32 = 0x5253_5201;
    static RSR_PREV: AtomicUsize = AtomicUsize::new(0);

    fn rsr_fake_game_start(c: &mut Cpu) {
        if c.bus.read32(RSR_GATE) != RSR_MAGIC {
            return fakes::fall_back(c, &RSR_PREV, ee::s06::f_00258988__generated);
        }
        fakes::fake_call(c, 0x0025_8988);
    }

    #[test]
    fn race_state_race_enter_matches() {
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00258988),
            rsr_fake_game_start,
            &RSR_PREV,
        );
        fakes::covered(
            ee::s00::f_00113d88__generated,
            race_state_race_enter,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x0011_3d88, 1);
                let comm = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                c.bus.write32(GAME_GP.wrapping_sub(0x204), comm);
                c.bus.write32(RSR_GATE, RSR_MAGIC);
            },
        );
    }

    #[test]
    fn func_0040c8b0_matches() {
        check(ee::s15::f_0040c8b0__generated, func_0040c8b0, 60, &|c, seed| {
            fakes::machine(c, seed, 0x0040_c8b0, 0);
        });
    }
}
