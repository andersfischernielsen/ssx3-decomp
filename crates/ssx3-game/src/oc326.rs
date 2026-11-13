//! Staging module oc326 (wave wave213): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc326`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0023_6af8,
        game_state_event_switch_load_save_state,
        "GameStateEventSwitchLoad_SaveState",
    ),
    (
        0x0023_61c8,
        game_state_warp_to_event_save_state,
        "GameStateWarpToEvent_SaveState",
    ),
    (
        0x0023_4eb8,
        game_state_race_save_state,
        "GameStateRace_SaveState",
    ),
    (
        0x0023_4de0,
        game_state_countdown_save_state,
        "GameStateCountdown_SaveState",
    ),
    (
        0x0023_3bd0,
        game_state_idle_save_state,
        "GameStateIdle_SaveState",
    ),
    (
        0x0023_3b90,
        game_state_quit_save_state,
        "GameStateQuit_SaveState",
    ),
    (0x0012_8660, player_save_state, "Player_SaveState"),
    (
        0x002f_4118,
        c_splash_manager_add_splash_all_views,
        "cSplashManager_AddSplashAllViews",
    ),
    (0x003d_d4e8, ea_file_queue_open, "EAFile_QueueOpen"),
    (
        0x0013_6de0,
        rider_wipeout_state_set_velocity,
        "RiderWipeoutState_SetVelocity",
    ),
];

// ---------------------------------------------------------------------------
// GameState*_SaveState (0x236af8, 0x2361c8, 0x234eb8, 0x234de0, 0x233bd0,
// 0x233b90)
//
// Six identical gcc forwarding wrappers: a 0x10 frame holding `ra`, one call
// to the base `GameState_SaveState(self, serialiser)` and a plain return.

/// `GameState_SaveState` (0x233af0), reached through its override slot.
pub(crate) const OC326_GAME_STATE_SAVE_STATE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};

/// `void GameStateEventSwitchLoad_SaveState(GameState* self, Serialiser* s)`
/// (0x236af8): save the base game state, forwarded unchanged.
pub fn game_state_event_switch_load_save_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC326_GAME_STATE_SAVE_STATE),
        0x0023_6b08,
    );
}

/// `void GameStateWarpToEvent_SaveState(GameState* self, Serialiser* s)`
/// (0x2361c8): save the base game state, forwarded unchanged.
pub fn game_state_warp_to_event_save_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC326_GAME_STATE_SAVE_STATE),
        0x0023_61d8,
    );
}

/// `void GameStateRace_SaveState(GameState* self, Serialiser* s)` (0x234eb8):
/// save the base game state, forwarded unchanged.
pub fn game_state_race_save_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC326_GAME_STATE_SAVE_STATE),
        0x0023_4ec8,
    );
}

/// `void GameStateCountdown_SaveState(GameState* self, Serialiser* s)`
/// (0x234de0): save the base game state, forwarded unchanged.
pub fn game_state_countdown_save_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC326_GAME_STATE_SAVE_STATE),
        0x0023_4df0,
    );
}

/// `void GameStateIdle_SaveState(GameState* self, Serialiser* s)` (0x233bd0):
/// save the base game state, forwarded unchanged.
pub fn game_state_idle_save_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC326_GAME_STATE_SAVE_STATE),
        0x0023_3be0,
    );
}

/// `void GameStateQuit_SaveState(GameState* self, Serialiser* s)` (0x233b90):
/// save the base game state, forwarded unchanged.
pub fn game_state_quit_save_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC326_GAME_STATE_SAVE_STATE),
        0x0023_3ba0,
    );
}

// ---------------------------------------------------------------------------
// Player_SaveState (0x128660)

/// `RiderBase_SaveState` (0x111ac0), through its override slot.
pub(crate) const OC326_RIDER_BASE_SAVE_STATE: Callee = Callee {
    addr: 0x0011_1ac0,
    f: ee::s00::RiderBase_SaveState,
};

/// `void Player_SaveState(Player* self, Stream* s)` (0x128660): save the
/// rider part of the player (the base state's save), forwarded unchanged.
pub fn player_save_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC326_RIDER_BASE_SAVE_STATE),
        0x0012_8670,
    );
}

// ---------------------------------------------------------------------------
// cSplashManager_AddSplashAllViews (0x2f4118)

/// The world at `gp - 0x848`; its game module is at `+0x84`.
pub(crate) const OC326_WORLD: u32 = 0x848;
/// `GameModule_GetSplashManager` (0x2306a8), through the function table.
pub(crate) const OC326_GET_SPLASH_MANAGER: u32 = 0x0023_06a8;
/// `cSplashManager_AddSplashFromPoint` (0x2f4260), through its slot.
pub(crate) const OC326_ADD_SPLASH_FROM_POINT: Callee = Callee {
    addr: 0x002f_4260,
    f: ee::s09::cSplashManager_AddSplashFromPoint,
};

/// `void cSplashManager_AddSplashAllViews(void* arg0, float speed)`
/// (0x2f4118): throw a splash at `arg0` in every view's splash manager, with
/// `speed` (`f12`, kept in `f20` across the calls). The view count comes from
/// the game module's `+0x84` view-list object (`+0x10`), the managers from
/// `GameModule_GetSplashManager(module, i)` (word at `module + 4*i + 0x68`).
pub fn c_splash_manager_add_splash_all_views(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = Addr(r32(c, GP).wrapping_sub(OC326_WORLD)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0; // i
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A0][0]; // the point
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A1)).off(0x84).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x84).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x10).lw(c); // count
        let empty = c.g[A0][0] == 0;
        c.mov_s(20, 12); // f20 = speed
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC326_WORLD)).lw(c);
            loop {
                c.cycles += 15;
                c.g[A1][0] = c.g[S0][0]; // index
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[A0][0] = Addr(r32(c, V0)).off(0x84).lw(c); // game module
                call_ptr(c, OC326_GET_SPLASH_MANAGER, 0x002f_4160)?;
                c.g[A0][0] = c.g[V0][0]; // the view's splash manager
                c.mov_s(12, 20); // speed
                c.g[A1][0] = c.g[S1][0]; // the point
                OC326_ADD_SPLASH_FROM_POINT.call(c, 0x002f_4170)?;
                // Reload the world and count for the loop test.
                c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC326_WORLD)).lw(c);
                c.g[V1][0] = Addr(r32(c, V0)).off(0x84).lw(c);
                c.g[A0][0] = Addr(r32(c, V1)).off(0x84).lw(c);
                c.g[V0][0] = Addr(r32(c, A0)).off(0x10).lw(c);
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC326_WORLD)).lw(c);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_4150);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAFile_QueueOpen (0x3dd4e8)

/// `EAFileRequest` words QueueOpen fills: arg2 at `+0x10`, arg3 at `+0x14`,
/// arg1 at `+0x18`; its status at `+0xc` and its file handle at `+0x24`.
pub(crate) const OC326_QO_ARG2: u32 = 0x10;
pub(crate) const OC326_QO_ARG3: u32 = 0x14;
pub(crate) const OC326_QO_ARG1: u32 = 0x18;
pub(crate) const OC326_QO_STATUS: u32 = 0xc;
pub(crate) const OC326_QO_HANDLE: u32 = 0x24;
/// Where the file name goes inside the file handle.
pub(crate) const OC326_QO_HANDLE_NAME: u32 = 0x10;
/// The name buffer's length.
pub(crate) const OC326_QO_NAME_LEN: u64 = 0x100;
/// Command bits in the request's word 0: the mask that clears the state
/// field (bits 20-23) and the "open" state (2 << 20).
pub(crate) const OC326_QO_CMD_MASK: u32 = 0xff0f_ffff;
pub(crate) const OC326_QO_CMD_OPEN: u64 = 0x0020_0000;
/// Request status 2: failed / signalled.
pub(crate) const OC326_QO_STATUS_FAILED: u64 = 2;

pub(crate) const OC326_EA_ALLOC_REQUEST: Callee = Callee {
    addr: 0x003d_e670,
    f: ee::s14::EAFile_AllocRequest,
};
pub(crate) const OC326_EA_ALLOC_FILE_HANDLE: Callee = Callee {
    addr: 0x003d_e800,
    f: ee::s14::EAFile_AllocFileHandle,
};
pub(crate) const OC326_EA_QUEUE_SIGNAL_STUB: Callee = Callee {
    addr: 0x003d_e668,
    f: ee::s14::EAFile_QueueSignalStub,
};
pub(crate) const OC326_EA_SERVICE_QUEUE: Callee = Callee {
    addr: 0x003d_dfa8,
    f: ee::s14::EAFile_ServiceQueue,
};
/// `strncpy` (0x416ce0), through the function table.
pub(crate) const OC326_STRNCPY: u32 = 0x0041_6ce0;

/// `int EAFile_QueueOpen(const char* name, int a1, int a2, int a3)`
/// (0x3dd4e8): take a request (`EAFile_AllocRequest`), make it an open (the
/// command bits of word 0), store the three arguments, take a file handle for
/// it (`EAFile_AllocFileHandle`, word `+0x24`); a null handle marks the
/// request failed (status 2, signal stub). Copy the name (0x100 bytes) into
/// the handle at `+0x10`, hand the request to `EAFile_ServiceQueue`, and
/// return the request's word 0 (its handle).
pub fn ea_file_queue_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A0][0]; // name
        OC326_EA_ALLOC_REQUEST.call(c, 0x003d_d518)?;
        c.g[S3][0] = c.g[V0][0]; // request
        let req = r32(c, S3);
        c.g[A0][0] = OC326_QO_CMD_OPEN;
        c.g[V1][0] = Addr(req).lw(c);
        c.g[V0][0] = sx(OC326_QO_CMD_MASK); // 0xffff_ffff_ff0f_ffff
        c.g[V0][0] |= 0xffff;
        c.sw(req.wrapping_add(OC326_QO_ARG3), c.g[S0][0]);
        c.g[V1][0] &= c.g[V0][0];
        c.sw(req.wrapping_add(OC326_QO_ARG1), c.g[S1][0]);
        c.g[V1][0] |= c.g[A0][0];
        c.sw(req.wrapping_add(OC326_QO_ARG2), c.g[S2][0]);
        c.sw(req, c.g[V1][0]);
        OC326_EA_ALLOC_FILE_HANDLE.call(c, 0x003d_d548)?;
        let has_handle = c.g[V0][0] != 0;
        c.sw(req.wrapping_add(OC326_QO_HANDLE), c.g[V0][0]);
        if !has_handle {
            c.cycles += 4;
            c.g[V0][0] = OC326_QO_STATUS_FAILED;
            c.g[A0][0] = c.g[S3][0];
            c.sw(req.wrapping_add(OC326_QO_STATUS), c.g[V0][0]);
            OC326_EA_QUEUE_SIGNAL_STUB.call(c, 0x003d_d560)?;
        }
        c.cycles += 16;
        // strncpy(handle->name, name, 0x100)
        c.g[A0][0] = Addr(r32(c, S3)).off(OC326_QO_HANDLE).lw(c);
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = OC326_QO_NAME_LEN;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(OC326_QO_HANDLE_NAME));
        call_ptr(c, OC326_STRNCPY, 0x003d_d574)?;
        c.g[A0][0] = c.g[S3][0];
        OC326_EA_SERVICE_QUEUE.call(c, 0x003d_d57c)?;
        c.g[V0][0] = Addr(r32(c, S3)).lw(c);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_SetVelocity (0x136de0)

/// The wipeout state's rider pointer (`+0x40`); the rider's angular velocity
/// (`+0x30`), up vector (`+0x370`), position (`+0x110`), velocity (`+0x1e0`)
/// and air predictor (`+0x788`); the "launched" flag (`+0x4`).
pub(crate) const OC326_RWSV_RIDER: u32 = 0x40;
pub(crate) const OC326_RWSV_ANG_VEL: u32 = 0x30;
pub(crate) const OC326_RWSV_UP: u32 = 0x370;
pub(crate) const OC326_RWSV_POS: u32 = 0x110;
pub(crate) const OC326_RWSV_VEL: u32 = 0x1e0;
pub(crate) const OC326_RWSV_AIR: u32 = 0x788;
pub(crate) const OC326_RWSV_LAUNCHED: u32 = 0x4;
/// The offset (`gp-0x72bc`) of the launch speed (`3333.3335f`) in the
/// `.lit4` pool.
pub(crate) const OC326_RWSV_LAUNCH_SPEED_LIT: u32 = 0x72bc;
/// `AirPredictor_Prime` (0x1135b8), through the function table.
pub(crate) const OC326_AIR_PREDICTOR_PRIME: u32 = 0x0011_35b8;

/// `void RiderWipeoutState_SetVelocity(motion, Vec4* vel, Vec4* angVel)`
/// (0x136de0): store the new angular velocity and velocity into the rider;
/// when not launched, slide the velocity back onto the surface along the
/// up vector (`vel -= up * dot(vel, up)`); when launched, kick off the air
/// predictor with the launch speed. Clears the launched flag either way.
pub fn rider_wipeout_state_set_velocity(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        // lq v1, *angVel ; sq v1, self+0x30
        c.lq(3, r32(c, A2));
        c.g[A0][0] = Addr(r32(c, S0)).off(OC326_RWSV_RIDER).lw(c);
        c.sq(3, r32(c, S0).wrapping_add(OC326_RWSV_ANG_VEL));
        // lq v0, *vel ; sq v0, rider->vel
        c.lq(2, r32(c, A1));
        c.sq(2, r32(c, A0).wrapping_add(OC326_RWSV_VEL));
        c.g[V1][0] = Addr(r32(c, S0)).lw(c); // self->launched
        if c.g[V1][0] != 0 {
            c.g[A1][0] = Addr(r32(c, S0)).off(OC326_RWSV_RIDER).lw(c);
            c.cycles += 6;
            c.lwc1(12, r32(c, GP).wrapping_sub(OC326_RWSV_LAUNCH_SPEED_LIT));
            c.g[A0][0] = Addr(r32(c, A1)).off(OC326_RWSV_AIR).lw(c);
            c.g[A2][0] = sx(r32(c, A1).wrapping_add(OC326_RWSV_VEL));
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(OC326_RWSV_POS));
            call_ptr(c, OC326_AIR_PREDICTOR_PRIME, 0x0013_6e84)?;
            c.sw(r32(c, S0).wrapping_add(OC326_RWSV_LAUNCHED), 0);
        } else {
            c.cycles += 24;
            c.g[V0][0] = Addr(r32(c, S0)).off(OC326_RWSV_RIDER).lw(c);
            c.lqc2(3, r32(c, V0).wrapping_add(OC326_RWSV_UP));
            c.lqc2(5, r32(c, A1));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.mfc1(4, 0);
            c.lqc2(4, r32(c, V0).wrapping_add(OC326_RWSV_UP));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp(c).wrapping_add(0x10));
            c.lq(3, sp(c).wrapping_add(0x10));
            c.g[V0][0] = Addr(r32(c, S0)).off(OC326_RWSV_RIDER).lw(c);
            c.sq(3, sp(c));
            c.lqc2(3, r32(c, V0).wrapping_add(OC326_RWSV_VEL));
            c.lqc2(4, sp(c));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, r32(c, V0).wrapping_add(OC326_RWSV_VEL));
            c.sw(r32(c, S0).wrapping_add(OC326_RWSV_LAUNCHED), 0);
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, vtable};
    use crate::difftest::{check, elf_image, put_vec, put_words, Rng, DATA, GAME_GP};

    // ------------------------------------------------- game state wrappers

    /// A game state at `DATA` and a serialiser at `DATA + 0x100` whose write
    /// method (vtable slot `+0x8`/`+0xc`) is a fake that may unwind; `a0` is
    /// the state (sometimes with upper-half garbage), `a1` the serialiser.
    fn oc326_state_fixture(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        let mut r = machine(c, seed, salt, 1);
        c.pc = entry;
        const THIS: u32 = DATA;
        const STREAM: u32 = DATA + 0x100;
        let vt = vtable(c, &mut r, &[0x8]);
        c.bus.write32(STREAM, vt);
        c.bus.write32(THIS, r.next_u32());
        c.g[A0][0] = if seed % 4 == 0 {
            (r.next_u32() as u64) << 32 | THIS as u64
        } else {
            sx(THIS)
        };
        c.g[A1][0] = sx(STREAM);
        r
    }

    /// GameStateEventSwitchLoad_SaveState: the base state's save, through a
    /// serialiser whose write method is a fake (some of them unwind).
    #[test]
    fn game_state_event_switch_load_save_state_matches() {
        covered(
            ee::s06::f_00236af8__generated,
            game_state_event_switch_load_save_state,
            200,
            &|c, seed| {
                oc326_state_fixture(c, seed, 0x23_6af8, 0x0023_6af8);
            },
        );
    }

    /// GameStateWarpToEvent_SaveState: as above.
    #[test]
    fn game_state_warp_to_event_save_state_matches() {
        covered(
            ee::s06::f_002361c8__generated,
            game_state_warp_to_event_save_state,
            200,
            &|c, seed| {
                oc326_state_fixture(c, seed, 0x23_61c8, 0x0023_61c8);
            },
        );
    }

    /// GameStateRace_SaveState: as above.
    #[test]
    fn game_state_race_save_state_matches() {
        covered(
            ee::s06::f_00234eb8__generated,
            game_state_race_save_state,
            200,
            &|c, seed| {
                oc326_state_fixture(c, seed, 0x23_4eb8, 0x0023_4eb8);
            },
        );
    }

    /// GameStateCountdown_SaveState: as above.
    #[test]
    fn game_state_countdown_save_state_matches() {
        covered(
            ee::s06::f_00234de0__generated,
            game_state_countdown_save_state,
            200,
            &|c, seed| {
                oc326_state_fixture(c, seed, 0x23_4de0, 0x0023_4de0);
            },
        );
    }

    /// GameStateIdle_SaveState: as above.
    #[test]
    fn game_state_idle_save_state_matches() {
        covered(
            ee::s06::f_00233bd0__generated,
            game_state_idle_save_state,
            200,
            &|c, seed| {
                oc326_state_fixture(c, seed, 0x23_3bd0, 0x0023_3bd0);
            },
        );
    }

    /// GameStateQuit_SaveState: as above.
    #[test]
    fn game_state_quit_save_state_matches() {
        covered(
            ee::s06::f_00233b90__generated,
            game_state_quit_save_state,
            200,
            &|c, seed| {
                oc326_state_fixture(c, seed, 0x23_3b90, 0x0023_3b90);
            },
        );
    }

    // ----------------------------------------------------- Player_SaveState

    /// Player_SaveState: the real `RiderBase_SaveState` on its own fixture
    /// (a fake stream, zeroed sub-state objects, targets present or not,
    /// path indices found or not), forwarded unchanged.
    #[test]
    fn player_save_state_matches() {
        use crate::game::ai::computer::tests::oc258_w32;
        use crate::game::ai::computer::{
            RB_PATH_DATA, RB_RIDER, RB_RIDER_AIR_PREDICTOR, RB_RIDER_AI_PATH, RB_RIDER_ANIM,
            RB_RIDER_BLOCK0, RB_RIDER_BLOCK1, RB_RIDER_METRIX, RB_RIDER_MODEL, RB_RIDER_TARGET0,
            RB_RIDER_TARGET1, RB_RIDER_TRACK_PATH, RB_TARGET_ID,
        };
        check(
            ee::s00::f_00128660__generated,
            player_save_state,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0012_8660, 40);
                c.pc = 0x0012_8660;
                let stream = DATA + 0x10;
                let vt = vtable(c, &mut r, &[0x8]);
                oc258_w32(c, stream, vt);
                let this = DATA + 0x20_0000;
                crate::difftest::fill(c, &mut r, this, 0xe00 / 4);
                let rider = DATA + 0x21_0000;
                oc258_w32(c, this + RB_RIDER, rider);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
                // Zeroed part objects (anim, model, air predictor, metrix).
                for (k, f) in [
                    RB_RIDER_ANIM,
                    RB_RIDER_MODEL,
                    RB_RIDER_AIR_PREDICTOR,
                    RB_RIDER_METRIX,
                ]
                .into_iter()
                .enumerate()
                {
                    oc258_w32(c, rider + f, DATA + 0x22_0000 + 0x1000 * k as u32);
                }
                // The anim's six sequence lists (8 bytes each, `count <= 0`).
                oc258_w32(c, DATA + 0x22_0000 + 0x50, DATA + 0x22_8000);
                // Targets: `*field` null or an object with an id.
                for (k, f) in [RB_RIDER_TARGET0, RB_RIDER_TARGET1].into_iter().enumerate() {
                    let cell = DATA + 0x23_0000 + 0x400 * k as u32;
                    let obj = cell + 0x100;
                    oc258_w32(c, rider + f, cell);
                    let present = (seed >> k) & 1 != 0;
                    oc258_w32(c, cell, if present { obj } else { 0 });
                    oc258_w32(c, obj + RB_TARGET_ID, r.next_u32());
                }
                oc258_w32(c, rider + RB_RIDER_BLOCK0, r.next_u32());
                oc258_w32(c, rider + RB_RIDER_BLOCK1, r.next_u32());
                // Path data: n track paths of 0x3c bytes (count +0x10, array
                // +0x14) and n AI paths of 0x40 bytes (count +0x8, array
                // +0xc); the rider's paths are one of them, a stranger, or
                // null.
                let n = seed % 5;
                let tracks = DATA + 0x24_0000;
                let ais = DATA + 0x25_0000;
                oc258_w32(c, RB_PATH_DATA + 0x10, n);
                oc258_w32(c, RB_PATH_DATA + 0x14, tracks);
                oc258_w32(c, RB_PATH_DATA + 0x8, n);
                oc258_w32(c, RB_PATH_DATA + 0xc, ais);
                for (f, base, stride) in [
                    (RB_RIDER_TRACK_PATH, tracks, 0x3c),
                    (RB_RIDER_AI_PATH, ais, 0x40),
                ] {
                    let v = match r.next_u32() % 3 {
                        0 => 0,
                        1 => base + stride * (r.next_u32() % 5),
                        _ => r.next_u32(),
                    };
                    oc258_w32(c, rider + f, v);
                }
            },
        );
    }

    // ----------------------------------------- cSplashManager_AddSplashAllViews

    /// cSplashManager_AddSplashAllViews: 0..3 views, each with its own splash
    /// manager (0..3 for the count, so the loop runs zero or more times), a
    /// point at the camera (the near path) or far away, snowing or not, and
    /// speeds below/inside/above the ramp. Real `GameModule_GetSplashManager`
    /// and `cSplashManager_AddSplashFromPoint` on the game image's literals.
    #[test]
    fn c_splash_manager_add_splash_all_views_matches() {
        const WORLD: u32 = DATA + 0x1000;
        const MODULE: u32 = DATA + 0x2000;
        const VIEWS: u32 = DATA + 0x3000;
        const POINT: u32 = DATA + 0x4000;
        const SPLASH0: u32 = DATA + 0x8000;
        const SPLASH_STRIDE: u32 = 0x2000;
        check(
            ee::s09::f_002f4118__generated,
            c_splash_manager_add_splash_all_views,
            60,
            &|c, seed| {
                let mut r = Rng(0x002f_4118 ^ (seed as u64) << 8);
                crate::difftest::scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x002f_4118;
                put_words(c, GAME_GP.wrapping_sub(OC326_WORLD), &[WORLD]);
                put_words(c, WORLD + 0x84, &[MODULE]);
                put_words(c, MODULE + 0x84, &[VIEWS]);
                let views = seed % 4;
                put_words(c, VIEWS + 0x10, &[views]);
                // The point: at the camera for the near path, far otherwise.
                let near = seed % 3 == 0;
                let cam = [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0];
                put_vec(c, POINT, cam);
                let thr = f32::from_bits(c.bus.read32(GAME_GP + 0x1188));
                for i in 0..3u32 {
                    let splash = SPLASH0 + SPLASH_STRIDE * i;
                    put_words(c, MODULE + 0x68 + 4 * i, &[splash]);
                    let camera = if near {
                        cam
                    } else {
                        [r.f32(50_000.0), r.f32(50_000.0), r.f32(50_000.0), 0]
                    };
                    put_vec(c, splash + 0x1010, camera);
                    put_words(c, splash + 0x1024, &[r.f32(3.0)]);
                    let snow = if seed % 5 == 1 { thr - 1.0 } else { thr + 2.0 };
                    put_words(c, splash + 0x1028, &[snow.to_bits()]);
                }
                c.g[A0][0] = sx(POINT);
                c.f[12] = ((seed % 9) as f32 * 600.0).to_bits();
            },
        );
    }

    // ------------------------------------------------------ EAFile_QueueOpen

    /// EAFile_QueueOpen: the real EAFile_AllocRequest (a pool with a free
    /// request slot; mutex with fake syscalls, some unwinding) and
    /// EAFile_AllocFileHandle (a free handle slot), a name buffer, and the
    /// real EAFile_ServiceQueue on its short path (a request already active
    /// at pool+0x24, 0..2 queued). The null-handle branch is not reached: the
    /// real AllocFileHandle spins when every handle is busy instead of
    /// returning 0.
    #[test]
    fn ea_file_queue_open_matches() {
        use crate::game::lib::eafile::{EAFILE_POOL_SLOTS, EAFILE_REQ_SIZE};
        const POOL: u32 = 0x0051_9ab0;
        const MUTEX: u32 = POOL + 0x14;
        const SLOTS: u32 = DATA + 0x1000;
        const HANDLES: u32 = DATA + 0x4000;
        const QUEUED: u32 = DATA + 0x8000;
        const NAME: u32 = DATA + 0xc000;
        const HANDLE_COUNT: u32 = POOL + 0x4;
        const HANDLE_SLOTS: u32 = POOL + 0x2c;
        check(
            ee::s14::f_003dd4e8__generated,
            ea_file_queue_open,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_d4e8, 8);
                c.pc = 0x003d_d4e8;
                // The request pool: 1..6 slots, the first free one anywhere.
                let count = 1 + r.next_u32() % 6;
                c.bus.write32(POOL, count);
                c.bus.write32(POOL + EAFILE_POOL_SLOTS, SLOTS);
                // The handle pool: 1..6 slots, all free.
                let handles = 1 + r.next_u32() % 6;
                c.bus.write32(HANDLE_COUNT, handles);
                c.bus.write32(HANDLE_SLOTS, HANDLES);
                for i in 0..6u32 {
                    c.bus.write32(HANDLES + 0x110 * i, 0);
                }
                // Mutex: owner never a fake thread id, depth, semaphore.
                c.bus
                    .write32(MUTEX + 0x4, 0x7700_0000 | r.next_u32() & 0xffff);
                c.bus.write32(MUTEX + 0x8, r.next_u32() & 3);
                c.bus.write32(MUTEX + 0xc, r.next_u32() & 0xff);
                c.bus.write32(0x0045_0de4, r.next_u32() & 1);
                let free = r.next_u32() % count;
                for i in 0..8u32 {
                    let mut w = r.next_u32();
                    if i == free {
                        w &= !0x00f0_0000;
                    } else if i < free && w & 0x00f0_0000 == 0 {
                        w |= 0x0010_0000;
                    }
                    c.bus.write32(SLOTS + EAFILE_REQ_SIZE * i, w);
                }
                c.bus.write32(0x0045_0c0c, r.next_u32() & 0xf_ffff);
                // The service queue: a request active; 0..2 queued.
                c.bus.write32(POOL + 0x24, QUEUED + 0x100);
                let queued = seed % 3;
                c.bus
                    .write32(POOL + 0x30, if queued == 0 { 0 } else { QUEUED });
                for k in 0..queued {
                    let q = QUEUED + 0x40 * k;
                    c.bus.write32(q + 0x10, r.next_u32() % 8);
                    c.bus
                        .write32(q + 0x2c, if k + 1 < queued { q + 0x40 } else { 0 });
                }
                c.bus.write32(POOL + 0xc, r.next_u32() & 0xff);
                // The name: a short, NUL-terminated string.
                let len = r.next_u32() % 0x40;
                for i in 0..len {
                    c.bus.write8(NAME + i, b'a' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(NAME + len, 0);
                c.g[A0][0] = sx(NAME);
                c.g[A1][0] = sx(r.next_u32() & 0x7_ffff);
                c.g[A2][0] = sx(r.next_u32() & 0x7_ffff);
                c.g[A3][0] = sx(r.next_u32() & 0x7_ffff);
            },
        );
    }

    // --------------------------------------- RiderWipeoutState_SetVelocity

    /// RiderWipeoutState_SetVelocity: launched (the real AirPredictor_Prime,
    /// its constants written by hand) or not (the VU0 projection onto the up
    /// vector), with random angular velocity, velocity, position and up
    /// vector.
    #[test]
    fn rider_wipeout_state_set_velocity_matches() {
        const SELF: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const VEL: u32 = DATA + 0x2000;
        const ANG_VEL: u32 = DATA + 0x2100;
        const AIR: u32 = DATA + 0x2200;
        check(
            ee::s01::f_00136de0__generated,
            rider_wipeout_state_set_velocity,
            200,
            &|c, seed| {
                let mut r = Rng(0x0013_6de0 ^ (seed as u64) << 8);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0013_6de0;
                c.g[GP][0] = GAME_GP as u64;
                // The air predictor's constants (reset and launch state).
                for i in 0..4u32 {
                    c.bus.write32(0x4ff160 + 4 * i, r.next_u32());
                    c.bus.write32(0x4ff120 + 4 * i, r.next_u32());
                }
                for i in 0..2u32 {
                    c.bus.write32(0x4a5960 + 4 * i, r.next_u32());
                }
                put_words(
                    c,
                    GAME_GP.wrapping_sub(OC326_RWSV_LAUNCH_SPEED_LIT),
                    &[0x4550_5556],
                );
                // The rider's vectors and its air predictor.
                put_words(
                    c,
                    RIDER + OC326_RWSV_POS,
                    &[r.f32(100.0), r.f32(100.0), r.f32(100.0), 0],
                );
                put_words(
                    c,
                    RIDER + OC326_RWSV_VEL,
                    &[r.f32(100.0), r.f32(100.0), r.f32(100.0), 0],
                );
                put_words(
                    c,
                    RIDER + OC326_RWSV_UP,
                    &[r.f32(1.0), r.f32(1.0), r.f32(1.0), 0],
                );
                put_words(c, RIDER + OC326_RWSV_AIR, &[AIR]);
                put_words(c, SELF + OC326_RWSV_RIDER, &[RIDER]);
                put_words(c, SELF, &[if seed % 2 == 0 { 0 } else { r.next_u32() | 1 }]);
                put_words(c, VEL, &[r.f32(100.0), r.f32(100.0), r.f32(100.0), 0]);
                put_words(c, ANG_VEL, &[r.f32(5.0), r.f32(5.0), r.f32(5.0), 0]);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(VEL);
                c.g[A2][0] = sx(ANG_VEL);
            },
        );
    }
}
