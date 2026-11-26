//! Staging module oc701 (wave wave239): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc701`.

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
        0x0020_cb58,
        game_ui_is_replay_menu_open,
        "GameUI_IsReplayMenuOpen",
    ),
    (
        0x0026_a138,
        path_get_first_event_type1_start,
        "Path_GetFirstEventType1Start",
    ),
    (0x0026_ade8, path_data_get_field_4c, "PathData_GetField4C"),
    (
        0x002d_1a78,
        wscriptman_run_obj_script0_hook,
        "WScriptMan_RunObjScript0Hook",
    ),
    (
        0x002f_7ba8,
        rider_base_reset_attack_fx_thunk,
        "RiderBase_ResetAttackFXThunk",
    ),
    (
        0x002f_7ed0,
        rider_base_reset_power_up_trail_thunk,
        "RiderBase_ResetPowerUpTrailThunk",
    ),
];

/// `bool GameUI_IsReplayMenuOpen(void)` (0x20cb58): whether the UI state stack
/// holds the "replay" menu state. Looks up the state by the hash of the
/// string at 0x471a40 in the stack at `world->module->+0x48 + 0x18`.
pub fn game_ui_is_replay_menu_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        c.g[A0][0] = sx(0x0047_1a40);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        // GetHashValue32(str_64replay).
        call_ptr(c, 0x0031_7670, 0x0020_cb80)?;
        // cUIStateStack_findState(stack, hash) != 0.
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_f9d8, 0x0020_cb8c)?;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `PathData` field `+0x4c` (the file-loaded flag), 0x26ade8.
pub fn path_data_get_field_4c(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4c)).lw(c);
    ret(c);
}

/// `float Path_GetFirstEventType1Start(tPathTable* t)` (0x26a138): the
/// `ParamStart` of the first event whose type is 1, or -1.0f. The table is
/// `{ int NumEvents; tPathEventNode* Events; }`, nodes 0x10 bytes apart with
/// `Type` at +0 and `ParamStart` (float) at +8.
pub fn path_get_first_event_type1_start(c: &mut Cpu) {
    c.cycles += 3;
    let table = c.g[A0][0] as u32;
    c.g[A1][0] = Addr(table).lw(c);
    c.g[V1][0] = 0;
    if (c.g[A1][0] as i64) > 0 {
        c.cycles += 3;
        c.g[A0][0] = Addr(table).off(0x4).lw(c);
        c.g[A2][0] = 1;
        loop {
            c.cycles += 3;
            let entry = c.g[A0][0] as u32;
            c.g[V0][0] = Addr(entry).lw(c);
            let not_type1 = c.g[V0][0] != c.g[A2][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if not_type1 {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_a150);
                    }
                    continue;
                }
                break;
            } else {
                c.cycles += 2;
                let v = Addr(entry.wrapping_add(0x8)).read_fpu(c);
                crate::abi::set_fret(c, v);
                ret(c);
                return;
            }
        }
    }
    c.cycles += 4;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.f[0] = 0xbf80_0000;
    ret(c);
}

/// `void PathData_LoadFile(cPathSys* p, const char* name)` (0x26ada8): load the
/// pack `name` into `p->Data` (+0x48), mark it loaded (+0x4c) and resolve
/// its paths.
///
/// Not installed: its callee FILE_loadpack (0x3e22f0) is unported and runs
/// its real disc loop on test data (runaway at pc 0x3de768), and it has no
/// override slot to fake. The port is kept for when the loader is ported.
#[cfg(test)]
#[allow(dead_code)]
pub fn path_data_load_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = 0;
        // FILE_loadpack(name, 0).
        call_ptr(c, 0x003e_22f0, 0x0026_adc4)?;
        c.g[V1][0] = 1;
        Addr((c.g[S0][0] as u32).wrapping_add(0x48)).sw(c, c.g[V0][0]);
        Addr((c.g[S0][0] as u32).wrapping_add(0x4c)).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.g[S0][0];
        // cPathSys_resolvePaths(p).
        call_ptr(c, 0x0026_adf0, 0x0026_add8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `bool PathData_ReloadPaths(cPathSys* p, const char* name)` (0x26ae70): drop
/// the loaded pack (+0x48, zeroed), load `name` and read its path tables.
/// Returns the file-loaded flag.
///
/// Not installed: it calls PathData_LoadFile and FILE_loadpack (0x3e22f0),
/// the unported disc loader that runs away on test data. Kept until the
/// loader is ported.
#[cfg(test)]
#[allow(dead_code)]
pub fn path_data_reload_paths(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x48)).lw(c);
        c.g[S0][0] = c.g[A1][0];
        if c.g[A0][0] != 0 {
            // A previous pack is loaded: release it, then clear the field.
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x0026_ae98)?;
            Addr((c.g[S1][0] as u32).wrapping_add(0x48)).sw(c, 0);
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0026_ada8, 0x0026_aeac)?;
            Addr((c.g[S1][0] as u32).wrapping_add(0x48)).sw(c, 0);
        }
        // Read the path tables.
        c.cycles += 20;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x003e_22f0, 0x0026_aebc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[SP][0];
        Addr((c.g[S1][0] as u32).wrapping_add(0x48)).sw(c, c.g[S0][0]);
        c.g[A1][0] = c.g[S0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        call_ptr(c, 0x0026_ba68, 0x0026_aed4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0026_b410, 0x0026_aee0)?;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0026_b310, 0x0026_aeec)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void WScriptMan_RunObjScript0Hook(void* inst)` (0x2d1a78): forwards the
/// instance to `cWScriptMan_doOnInitScript` with the global script manager.
pub fn wscriptman_run_obj_script0_hook(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0xce8)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0030_a610, 0x002d_1a90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void RiderBase_ResetAttackFXThunk(void)` (0x2f7ba8): forwards to
/// `RiderBase_ResetAttackFX`.
pub fn rider_base_reset_attack_fx_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002f_1148, 0x002f_7bb8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void RiderBase_ResetPowerUpTrailThunk(void)` (0x2f7ed0): forwards to
/// `RiderBase_ResetPowerUpTrail`.
pub fn rider_base_reset_power_up_trail_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002e_adc0, 0x002f_7ee0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// `gp-0x848` -> world; `world+0x84` -> module; `module+0x48` = `state - 0x18`,
    /// so the replay code's `S0` is `state`.
    fn setup_world(c: &mut Cpu, state: u32) {
        let world = DATA + 0x1000;
        let module = DATA + 0x2000;
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, module);
        c.bus.write32(module + 0x48, state.wrapping_sub(0x18));
    }

    #[test]
    fn game_ui_is_replay_menu_open_matches() {
        check(
            ee::s05::f_0020cb58__generated,
            game_ui_is_replay_menu_open,
            200,
            &|c, seed| {
                let mut r = Rng(0x0020_cb58 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0020_cb58;
                setup_world(c, DATA + 0x3000);
                // The stack is left empty: findState returns 0 on both sides.
                let _ = seed;
            },
        );
    }

    #[test]
    fn path_data_get_field_4c_matches() {
        check(
            ee::s07::f_0026ade8__generated,
            path_data_get_field_4c,
            100,
            &|c, seed| {
                let mut r = Rng(0x0026_ade8 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0026_ade8;
                let at = DATA + 0x4000 + (r.next_u32() & 0xf0);
                c.g[A0][0] = sx(at);
                c.bus.write32(at + 0x4c, r.next_u32());
            },
        );
    }

    #[test]
    fn path_get_first_event_type1_start_matches() {
        check(
            ee::s07::f_0026a138__generated,
            path_get_first_event_type1_start,
            300,
            &|c, seed| {
                let mut r = Rng(0x0026_a138 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0026_a138;
                let table = DATA + 0x5000;
                let events = DATA + 0x5100;
                // Zero events, or 1-6 events with types 0..2 (1 for some).
                let n = seed % 7;
                c.bus.write32(table, n);
                c.bus.write32(table + 4, events);
                for i in 0..n {
                    let node = events + i * 0x10;
                    let ty = if seed % 3 == 0 { 1 } else { r.next_u32() % 3 };
                    c.bus.write32(node, ty);
                    c.bus.write32(node + 8, r.edge_f32());
                }
                c.g[A0][0] = sx(table);
            },
        );
    }

    #[test]
    fn wscriptman_run_obj_script0_hook_matches() {
        check(
            ee::s09::f_002d1a78__generated,
            wscriptman_run_obj_script0_hook,
            100,
            &|c, seed| {
                let mut r = Rng(0x002d_1a78 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x002d_1a78;
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x8000);
                c.g[A0][0] = sx(DATA + 0x9000 + (r.next_u32() & 0xf0));
                let _ = seed;
            },
        );
    }

    #[test]
    fn rider_base_reset_attack_fx_thunk_matches() {
        check(
            ee::s09::f_002f7ba8__generated,
            rider_base_reset_attack_fx_thunk,
            60,
            &|c, seed| {
                let mut r = Rng(0x002f_7ba8 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x002f_7ba8;
                c.g[GP][0] = GAME_GP as u64;
                let _ = seed;
            },
        );
    }

    #[test]
    fn rider_base_reset_power_up_trail_thunk_matches() {
        check(
            ee::s09::f_002f7ed0__generated,
            rider_base_reset_power_up_trail_thunk,
            60,
            &|c, seed| {
                let mut r = Rng(0x002f_7ed0 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x002f_7ed0;
                c.g[GP][0] = GAME_GP as u64;
                let _ = seed;
            },
        );
    }
}
