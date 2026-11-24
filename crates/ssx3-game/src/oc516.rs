//! Staging module oc516 (wave wave230): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc516`.

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
        0x0023_4e00,
        game_state_countdown_load_state,
        "GameStateCountdown_LoadState",
    ),
    (
        0x0023_4ed8,
        game_state_race_load_state,
        "GameStateRace_LoadState",
    ),
    (
        0x0023_61e8,
        game_state_warp_to_event_load_state,
        "GameStateWarpToEvent_LoadState",
    ),
    (
        0x0023_6b18,
        game_state_event_switch_load_load_state,
        "GameStateEventSwitchLoad_LoadState",
    ),
    (
        0x0023_6b90,
        game_state_reset_ai_load_state,
        "GameStateResetAI_LoadState",
    ),
    (
        0x0023_6d80,
        game_state_pre_race_menu_load_state,
        "GameStatePreRaceMenu_LoadState",
    ),
    (
        0x0023_6ee8,
        game_state_results_load_state,
        "GameStateResults_LoadState",
    ),
    (
        0x0023_6fc8,
        game_state_finish_cinematic_load_state,
        "GameStateFinishCinematic_LoadState",
    ),
    (
        0x0023_1280,
        game_module_cancel_overlay_state,
        "GameModule_CancelOverlayState",
    ),
];

/// `GameState_LoadState` (0x233b28), the shared body of the `*_LoadState`
/// wrappers below. It loads `a1`'s vtable and calls its slot 0x14 with
/// `a0 = a1 + (lh slot 0x10)`; the wrappers only forward `a0`/`a1`.
const GAME_STATE_LOAD_STATE: Callee = Callee {
    addr: 0x0023_3b28,
    f: ee::s06::f_00233b28,
};

/// `GameStateCountdown_LoadState(state)` (0x234e00): `GameState_LoadState`
/// on the same arguments; returns to 0x234e10.
pub fn game_state_countdown_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_4e10)
}

/// `GameStateRace_LoadState(state)` (0x234ed8): `GameState_LoadState`; returns
/// to 0x234ee8.
pub fn game_state_race_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_4ee8)
}

/// `GameStateWarpToEvent_LoadState(state)` (0x2361e8): `GameState_LoadState`;
/// returns to 0x2361f8.
pub fn game_state_warp_to_event_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_61f8)
}

/// `GameStateEventSwitchLoad_LoadState(state)` (0x236b18): `GameState_LoadState`;
/// returns to 0x236b28.
pub fn game_state_event_switch_load_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_6b28)
}

/// `GameStateResetAI_LoadState(state)` (0x236b90): `GameState_LoadState`;
/// returns to 0x236ba0.
pub fn game_state_reset_ai_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_6ba0)
}

/// `GameStatePreRaceMenu_LoadState(state)` (0x236d80): `GameState_LoadState`;
/// returns to 0x236d90.
pub fn game_state_pre_race_menu_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_6d90)
}

/// `GameStateResults_LoadState(state)` (0x236ee8): `GameState_LoadState`;
/// returns to 0x236ef8.
pub fn game_state_results_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_6ef8)
}

/// `GameStateFinishCinematic_LoadState(state)` (0x236fc8): `GameState_LoadState`;
/// returns to 0x236fd8.
pub fn game_state_finish_cinematic_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_6fd8)
}

/// `GameModule_CancelOverlayState(game)` (0x231280): clears the pending flag
/// at `game+0x208`; then, if `game+0x20c` is set, calls the vtable slot
/// (`lh` adjustment at +0x20, function at +0x24) of the object at
/// `game+0x204` with `this = object + adjustment`, and clears `+0x204` and
/// `+0x20c`.
pub fn game_module_cancel_overlay_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let game = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(game.wrapping_add(0x208)).lw(c);
        if c.g[V0][0] != 0u64 {
            Addr(game.wrapping_add(0x208)).sw(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(game.wrapping_add(0x20c)).lw(c);
        if c.g[V0][0] != 0u64 {
            c.cycles += 9;
            c.g[A1][0] = Addr(game.wrapping_add(0x204)).lw(c);
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_12c0)?;
            // The callee keeps s0: the state is re-read from it.
            let game = c.g[S0][0] as u32;
            Addr(game.wrapping_add(0x204)).sw(c, 0);
            Addr(game.wrapping_add(0x20c)).sw(c, 0);
            frame.restore(c, S0, 0x10);
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::*;
    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::Cpu;

    const SEEDS: u32 = 300;

    #[test]
    fn game_module_cancel_overlay_state_matches() {
        covered(
            ssx3_ee::s06::f_00231280__generated,
            super::game_module_cancel_overlay_state,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x516_1, 1);
                let game = ptr(&mut r);
                c.g[A0][0] = game as u64;
                c.bus
                    .write32(game + 0x208, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus
                    .write32(game + 0x20c, if seed % 3 == 0 { 0 } else { r.next_u32() });
                let obj = ptr(&mut r);
                c.bus.write32(game + 0x204, obj);
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(obj + 0xc, vt);
            },
        );
    }

    /// The `*_LoadState` fixture: `a1` is the state, whose vtable's slot
    /// (`lh` at +0x10, function at +0x14) is a fake. `c.pc` is the entry.
    fn load_state_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let mut r = machine(c, seed, salt, 1);
        let state = ptr(&mut r);
        c.g[A1][0] = state as u64;
        let vt = vtable(c, &mut r, &[0x10]);
        c.bus.write32(state, vt);
        c.pc = entry;
    }

    #[test]
    fn game_state_load_state_wrappers_match() {
        let cases: [(fn(&mut Cpu), fn(&mut Cpu), u32); 8] = [
            (
                ssx3_ee::s06::f_00234e00__generated,
                super::game_state_countdown_load_state,
                0x0023_4e00,
            ),
            (
                ssx3_ee::s06::f_00234ed8__generated,
                super::game_state_race_load_state,
                0x0023_4ed8,
            ),
            (
                ssx3_ee::s06::f_002361e8__generated,
                super::game_state_warp_to_event_load_state,
                0x0023_61e8,
            ),
            (
                ssx3_ee::s06::f_00236b18__generated,
                super::game_state_event_switch_load_load_state,
                0x0023_6b18,
            ),
            (
                ssx3_ee::s06::f_00236b90__generated,
                super::game_state_reset_ai_load_state,
                0x0023_6b90,
            ),
            (
                ssx3_ee::s06::f_00236d80__generated,
                super::game_state_pre_race_menu_load_state,
                0x0023_6d80,
            ),
            (
                ssx3_ee::s06::f_00236ee8__generated,
                super::game_state_results_load_state,
                0x0023_6ee8,
            ),
            (
                ssx3_ee::s06::f_00236fc8__generated,
                super::game_state_finish_cinematic_load_state,
                0x0023_6fc8,
            ),
        ];
        for (salt, (gen, hand, entry)) in cases.into_iter().enumerate() {
            covered(gen, hand, SEEDS, &|c, seed| {
                load_state_setup(c, seed, 0x516 + salt as u64, entry)
            });
        }
    }
}
