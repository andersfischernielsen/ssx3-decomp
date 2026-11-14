//! Staging module oc349 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc349`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0023_06b8, game_module_update, "GameModule_Update")];

// ---------------------------------------------------------------------------
// GameModule_Update (0x2306b8): the game module's per-frame update.

/// `GameModule` fields read by [`game_module_update`].
pub(crate) const GM_UPDATE_HALTED: u32 = 0x8;
pub(crate) const GM_UPDATE_RIDERS: u32 = 0xc;
pub(crate) const GM_UPDATE_WORLD_REF: u32 = 0x10;
pub(crate) const GM_UPDATE_RACE: u32 = 0x28;
pub(crate) const GM_UPDATE_WSCRIPT: u32 = 0x30;
pub(crate) const GM_UPDATE_PLAYBACK: u32 = 0x34;
pub(crate) const GM_UPDATE_CROWD_2D: u32 = 0x38;
pub(crate) const GM_UPDATE_CROWD_3D: u32 = 0x3c;
pub(crate) const GM_UPDATE_PARTICLES: u32 = 0x40;
pub(crate) const GM_UPDATE_UI: u32 = 0x48;
pub(crate) const GM_UPDATE_WORLD_SOUND: u32 = 0x4c;
pub(crate) const GM_UPDATE_SECTION_MAN: u32 = 0x78;
pub(crate) const GM_UPDATE_INPUT_SYNC: u32 = 0x7c;
pub(crate) const GM_UPDATE_VIEW_MAN: u32 = 0x84;
pub(crate) const GM_UPDATE_LOAD_INDICATOR: u32 = 0x94;
/// Set while the world was held (sections loading) last frame.
pub(crate) const GM_UPDATE_WAS_HELD: u32 = 0x9c;
/// Prefetch the views once the hold ends.
pub(crate) const GM_UPDATE_PREFETCH: u32 = 0xa0;
pub(crate) const GM_UPDATE_MENU: u32 = 0xa4;
pub(crate) const GM_UPDATE_FRAME: u32 = 0xac;
pub(crate) const GM_UPDATE_MODE: u32 = 0x214;
pub(crate) const GM_UPDATE_PAUSE_ARG: u32 = 0x228;

/// `gp`-relative globals.
const GMU_GP_FOG: i32 = 0x2a74; // D_004A5B64
const GMU_GP_SKY_FOG_MIN: i32 = -0x51bc; // 0.9f
const GMU_GP_GAME_COMM: i32 = -0x204; // D_004A2EEC
const GMU_GP_SCRIPT_ENGINE: i32 = -0x84c; // D_004A28A4
const GMU_GP_WORLD: i32 = -0x848; // D_004A28A8
const GMU_GP_AUDIO_HOST: i32 = -0x854; // D_004A289C
const GMU_GP_PAD_QUEUE: i32 = -0x850; // D_004A28A0
const GMU_GP_NET_CONN: i32 = -0x238; // D_004A2EB8
const GMU_GP_STATE_DEPTH: i32 = -0x69c; // D_004A2A54
const GMU_GP_STATE_FLAGS: i32 = -0x6a0; // D_004A2A50
const GMU_GP_PENDING_OVERLAY: i32 = -0x1024; // D_004A20CC
const GMU_GP_PROCESS_LIST: i32 = 0x2898; // D_004A5988
const GMU_GP_WSCRIPT: i32 = 0xce8; // D_004A3DD8
/// The state stack (`D_005366E8`) and the per-state flag words
/// (`D_004428F0`).
const GMU_STATE_STACK: u64 = 0x0053_66e8;
const GMU_STATE_FLAGS_2: u32 = 0x0044_28f8;
const GMU_STATE_FLAGS_3: u32 = 0x0044_28fc;
const GMU_STATE_FLAGS_12: u32 = 0x0044_2920;
/// The UI state name hashed for the mission event (`D_0047B798`).
const GMU_MISSION_STATE_NAME: u64 = 0x0047_b798;

const GMU_GAME_COMM_UPDATE: Callee = Callee {
    addr: 0x0025_70d8,
    f: ee::s06::cGameComm_Update,
};
const GMU_UPDATE_FADE: Callee = Callee {
    addr: 0x0023_1d18,
    f: ee::s06::AppModule_UpdateFade,
};
const GMU_PROCESS_STATE_CHANGE: Callee = Callee {
    addr: 0x0023_0f40,
    f: ee::s06::GameModule_ProcessStateChange,
};
const GMU_INPUT_SYNC_UPDATE: Callee = Callee {
    addr: 0x0023_73b8,
    f: ee::s06::InputSync_Update,
};
const GMU_INPUT_SYNC_READY: Callee = Callee {
    addr: 0x0023_7950,
    f: ee::s06::InputSync_IsReady,
};
const GMU_INPUT_SYNC_APPLY: Callee = Callee {
    addr: 0x0023_79c8,
    f: ee::s06::InputSync_ApplyInput,
};
const GMU_INPUT_SYNC_HOOK: Callee = Callee {
    addr: 0x0023_7948,
    f: ee::s06::InputSync_EmptyHook,
};
const GMU_CHECK_PAUSE: Callee = Callee {
    addr: 0x0023_1840,
    f: ee::s06::GameModule_CheckPauseRequest,
};

#[inline]
fn gmu_gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// The address of a field of the module (`s1`).
#[inline]
fn gmu_field(c: &Cpu, off: u32) -> u32 {
    r32(c, S1).wrapping_add(off)
}

/// `a0 = &D_004A5988; a1 = list; ProcessList_UpdatePriority` (and the merge /
/// post-update forms): the register setup of the process-list calls.
#[inline]
fn gmu_process_list(c: &mut Cpu, target: u32, list: u64, ret: u32) -> Flow {
    c.g[A0][0] = sx(gmu_gp(c, GMU_GP_PROCESS_LIST));
    c.g[A1][0] = list;
    call_ptr(c, target, ret)
}

/// `void GameModule_Update(GameModule* self)` (0x2306b8, `func_002306B8`):
/// the game module's frame. Sky fog, the network game and the script engine's
/// loads, the fade; then (unless the world is loading) the pause/hold logic,
/// the game UI, the UI state machine (events from the script queue, overlays,
/// the pause menu), the process lists, scripts, crowds, particles, cameras,
/// world sounds and riders; then the network idle, the FE async loader, the
/// message center and the load indicator.
pub fn game_module_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        // Sky fog: always in split screen, else once the sky blend passes 0.9.
        c.g[V1][0] = c.lw(gmu_field(c, GM_UPDATE_VIEW_MAN));
        c.g[V0][0] = (c.lw(r32(c, V1).wrapping_add(0x10)) < 2) as u64;
        let single_view = c.g[V0][0] != 0;
        c.g[V1][0] = c.lw(gmu_gp(c, GMU_GP_FOG));
        if !single_view {
            c.cycles += 3;
            c.g[V0][0] = 1;
            c.sw(r32(c, V1).wrapping_add(0x34), c.g[V0][0]);
        } else {
            c.cycles += 8;
            c.g[A0][0] = 6;
            call_ptr(c, 0x002e_e400, 0x0023_06f4)?;
            c.lwc1(1, gmu_gp(c, GMU_GP_SKY_FOG_MIN));
            c.g[V1][0] = 1;
            c.c_lt_s(1, 0);
            let above = c.fcc();
            c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_FOG));
            if !above {
                c.cycles += 1;
                c.g[V1][0] = 0;
            }
            c.cycles += 1;
            c.sw(r32(c, V0).wrapping_add(0x34), c.g[V1][0]);
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(gmu_gp(c, GMU_GP_GAME_COMM));
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            GMU_GAME_COMM_UPDATE.call(c, 0x0023_0728)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.lw(gmu_gp(c, GMU_GP_SCRIPT_ENGINE));
        call_ptr(c, 0x0027_8918, 0x0023_0730)?;
        c.g[A0][0] = c.g[S1][0];
        GMU_UPDATE_FADE.call(c, 0x0023_0738)?;
        // canUpdate: not while the world is loading.
        c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_WORLD));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(4));
        let loading = c.g[V1][0] != 0;
        c.g[V0][0] = 1;
        if !loading {
            c.cycles += 1;
            c.g[V1][0] = 1;
        } else {
            c.cycles += 3;
            c.g[V1][0] = 0;
            c.sw(gmu_field(c, GM_UPDATE_HALTED), c.g[V0][0]);
        }
        c.cycles += 2;
        let can_update = c.g[V1][0] != 0;
        c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_NET_CONN));
        if can_update {
            // Leaves v0 = the network flag again.
            gmu_frame(c)?;
        }
        c.cycles += 2;
        let net = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_WORLD));
        if net {
            c.cycles += 3;
            call_ptr(c, 0x003f_42a0, 0x0023_0e50)?;
            c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_WORLD));
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x11c));
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, 0x001a_39f0, 0x0023_0e68)?;
        }
        c.cycles += 9;
        call_ptr(c, 0x001e_1458, 0x0023_0e70)?;
        c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_LOAD_INDICATOR));
        call_ptr(c, 0x001f_3188, 0x0023_0e78)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The part of the frame that runs unless the world is loading: the pause /
/// hold logic, then (once the views are ready) the UI state machine and the
/// world update. Leaves `v0` = the network flag (`gp-0x238`).
fn gmu_frame(c: &mut Cpu) -> Flow {
    // updatePause: hold the world until every view is ready and the
    // section manager has its sections.
    c.cycles += 4;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_VIEW_MAN));
    call_ptr(c, 0x0022_e920, 0x0023_0768)?;
    let ready = if c.g[V0][0] == 0 {
        c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_WORLD_REF));
        false
    } else {
        c.cycles += 4;
        c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_SECTION_MAN));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1d0));
        let sections = c.g[V1][0] != 0;
        c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_WORLD_REF));
        sections
    };
    if ready {
        gmu_resume(c)?;
    } else {
        gmu_hold(c)?;
    }
    c.cycles += 2;
    let held = c.g[V0][0] == 0;
    c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_NET_CONN));
    if held {
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_INPUT_SYNC));
    if c.g[A0][0] != 0 {
        c.cycles += 2;
        GMU_INPUT_SYNC_UPDATE.call(c, 0x0023_0864)?;
    }
    gmu_update_state(c)?;
    c.cycles += 2;
    let run = c.g[V1][0] != 0;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_INPUT_SYNC));
    if run {
        gmu_update_world(c)?;
        c.cycles += 1;
        c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_INPUT_SYNC));
    }
    c.cycles += 2;
    let sync = c.g[A0][0] != 0;
    c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_NET_CONN));
    if sync {
        c.cycles += 3;
        GMU_INPUT_SYNC_HOOK.call(c, 0x0023_0e3c)?;
        c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_NET_CONN));
    }
    Ok(())
}

/// Not ready (`v0` = the world reference): the world update without
/// sections, the cameras, the state change and the section manager while it
/// has none; once the views are ready, ask for a prefetch (the audio host's
/// method, the views, the hull pages). Leaves `v0 = 0`.
fn gmu_hold(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    c.g[V1][0] = c.lw(r32(c, V0));
    c.sw(r32(c, V1).wrapping_add(0x3d4), 0);
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_WORLD_REF));
    call_ptr(c, 0x003a_6928, 0x0023_0790)?;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_VIEW_MAN));
    call_ptr(c, 0x0022_e840, 0x0023_0798)?;
    c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_SECTION_MAN));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1d0));
    if c.g[V1][0] == 0 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S1][0];
        GMU_PROCESS_STATE_CHANGE.call(c, 0x0023_07b0)?;
        c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_SECTION_MAN));
        call_ptr(c, 0x0022_d8d8, 0x0023_07b8)?;
    }
    c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_WAS_HELD));
    c.cycles += 2;
    let was_held = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if !was_held {
        c.cycles += 4;
        c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_VIEW_MAN));
        call_ptr(c, 0x0022_e920, 0x0023_07cc)?;
        let views_ready = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !views_ready {
            c.cycles += 2;
            c.sw(gmu_field(c, GM_UPDATE_PREFETCH), c.g[V0][0]);
            c.g[V0][0] = 1;
        }
    }
    c.cycles += 3;
    c.g[V1][0] = c.lw(gmu_field(c, GM_UPDATE_PREFETCH));
    let prefetch = c.g[V1][0] != 0;
    c.sw(gmu_field(c, GM_UPDATE_WAS_HELD), c.g[V0][0]);
    if prefetch {
        c.cycles += 12;
        c.g[A1][0] = c.lw(gmu_gp(c, GMU_GP_AUDIO_HOST));
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x98));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x9c));
        let method = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0023_0800)?;
        c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_VIEW_MAN));
        call_ptr(c, 0x0022_e8b8, 0x0023_0808)?;
        c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_WORLD_REF));
        c.g[A0][0] = c.lw(r32(c, V0));
        call_ptr(c, 0x003a_88a8, 0x0023_0814)?;
        c.sw(gmu_field(c, GM_UPDATE_PREFETCH), 0);
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    Ok(())
}

/// Ready (`v0` = the world reference): the world update with sections and
/// the section manager. Leaves `v0 = 1`.
fn gmu_resume(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    c.g[A0][0] = 1;
    c.g[V1][0] = c.lw(r32(c, V0));
    c.sw(r32(c, V1).wrapping_add(0x3d4), c.g[A0][0]);
    c.sw(gmu_field(c, GM_UPDATE_WAS_HELD), 0);
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_WORLD_REF));
    c.sw(gmu_field(c, GM_UPDATE_PREFETCH), 0);
    call_ptr(c, 0x003a_6928, 0x0023_083c)?;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_SECTION_MAN));
    call_ptr(c, 0x0022_d8d8, 0x0023_0844)?;
    c.g[V0][0] = 1;
    Ok(())
}

/// updateState: the game UI, then the UI state machine. Leaves `v1` = 1 when
/// the world should run this frame, 0 when a menu, overlay or pause took it.
fn gmu_update_state(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    call_ptr(c, 0x0020_cfd0, 0x0023_086c)?;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_MENU));
    call_ptr(c, 0x002c_c098, 0x0023_0874)?;
    let menu = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_PENDING_OVERLAY));
    if !menu {
        // Events from the script manager's queue.
        c.cycles += 5;
        c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_WSCRIPT));
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x1c8));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1c4));
        let empty = c.g[V1][0] == c.g[A0][0];
        c.g[A1][0] = sx(r32(c, V0).wrapping_add(0x1c4));
        if empty {
            c.cycles += 1;
            c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_PENDING_OVERLAY));
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.g[A1][0];
            call_ptr(c, 0x0030_c870, 0x0023_0898)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(4));
            c.g[V0][0] = 8;
            let not_8 = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 9;
            if !not_8 {
                return gmu_event_overlay(c);
            }
            c.cycles += 2;
            let not_9 = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_PENDING_OVERLAY));
            if !not_9 {
                return gmu_mission_event(c);
            }
        }
    }
    // A pending overlay (`D_004A20CC != -1`).
    c.cycles += 3;
    c.g[V0][0] = !c.g[V0][0];
    if c.g[V0][0] != 0 {
        return gmu_pending_overlay(c);
    }
    c.cycles += 4;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_MENU));
    call_ptr(c, 0x002c_c098, 0x0023_0a14)?;
    let menu = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S1][0];
    if menu {
        c.cycles += 6;
        c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_UI));
        call_ptr(c, 0x0039_8038, 0x0023_0a24)?;
        c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_MENU));
        call_ptr(c, 0x002c_c0c0, 0x0023_0a2c)?;
        c.g[V1][0] = 0;
        return Ok(());
    }
    // The pause request: not in a race, not playing back, allowed.
    c.cycles += 4;
    c.g[A1][0] = 0;
    GMU_CHECK_PAUSE.call(c, 0x0023_0a3c)?;
    let requested = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS));
    if requested {
        c.cycles += 4;
        c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_RACE));
        c.g[V0][0] = c.lw(r32(c, V0));
        let idle = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !idle {
            c.cycles += 1;
            c.g[V1][0] = ((c.g[V0][0] as i64) < 10) as u64;
        }
        c.cycles += 2;
        let in_race = c.g[V1][0] != 0;
        c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS));
        if !in_race {
            c.cycles += 5;
            c.g[V1][0] = c.lw(gmu_field(c, GM_UPDATE_PLAYBACK));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x70)) ^ 1;
            let not_playing = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS));
            if not_playing {
                c.cycles += 4;
                call_ptr(c, 0x0020_cbe8, 0x0023_0a7c)?;
                let can_open = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS));
                if can_open {
                    return gmu_open_pause(c);
                }
            }
        }
    }
    // The UI engine and the replay loader; run unless a menu opened or the
    // pause just ended.
    c.cycles += 7;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_UI));
    c.g[S0][0] = c.g[V0][0] & 1;
    call_ptr(c, 0x0039_8038, 0x0023_0b18)?;
    call_ptr(c, 0x0020_cba0, 0x0023_0b20)?;
    if c.g[V0][0] != 0 {
        c.g[V1][0] = 0;
        return Ok(());
    }
    c.cycles += 6;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_PLAYBACK));
    call_ptr(c, 0x0026_c5f8, 0x0023_0b30)?;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_MENU));
    call_ptr(c, 0x002c_c098, 0x0023_0b38)?;
    let menu = c.g[V0][0] != 0;
    c.g[V1][0] = 0;
    if menu {
        return Ok(());
    }
    c.cycles += 2;
    let was_paused = c.g[S0][0] != 0;
    c.g[V1][0] = 1;
    if !was_paused {
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS)) & 1;
    if c.g[V0][0] == 0 {
        c.g[V1][0] = 0;
    }
    Ok(())
}

/// Event 8 (`a1` = the event): overlay 0x1d, push state 3, the multiplier
/// sound. Leaves `v1 = 0`.
fn gmu_event_overlay(c: &mut Cpu) -> Flow {
    c.cycles += 23;
    c.g[A0][0] = 0x1d;
    call_ptr(c, 0x0020_ab50, 0x0023_08b4)?;
    c.g[A0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_DEPTH));
    c.g[A1][0] = 0x44_0000;
    c.g[A2][0] = c.lw(GMU_STATE_FLAGS_3);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
    c.g[V1][0] = GMU_STATE_STACK;
    c.g[V0][0] = sx(r32(c, A0) << 2);
    c.g[A1][0] = 3;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.sw(gmu_gp(c, GMU_GP_STATE_FLAGS), c.g[A2][0]);
    c.sw(r32(c, V0), c.g[A1][0]);
    c.sw(gmu_gp(c, GMU_GP_STATE_DEPTH), c.g[A0][0]);
    call_ptr(c, 0x0028_b180, 0x0023_08e8)?;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 12);
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 4;
    c.g[A2][0] = 0;
    call_ptr(c, 0x0029_ced8, 0x0023_0900)?;
    c.g[V1][0] = 0;
    Ok(())
}

/// Event 9 (`a1` = the event): the mission command; unless the mission UI
/// state exists, overlay 0x1e, push state 3 and update the UI, else pop the
/// event. Leaves `v1 = 0`.
fn gmu_mission_event(c: &mut Cpu) -> Flow {
    c.cycles += 15;
    c.g[A3][0] = c.lw(r32(c, A1).wrapping_add(8));
    c.g[A2][0] = 5;
    c.g[A1][0] = c.lw(r32(c, A1));
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_WSCRIPT));
    call_ptr(c, 0x0030_a868, 0x0023_0924)?;
    c.g[S0][0] = c.lw(gmu_field(c, GM_UPDATE_UI));
    c.g[A0][0] = GMU_MISSION_STATE_NAME;
    c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x18));
    call_ptr(c, 0x0031_7670, 0x0023_0938)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_f9d8, 0x0023_0944)?;
    let found = c.g[V0][0] != 0;
    c.g[A0][0] = c.lw(gmu_gp(c, GMU_GP_WSCRIPT));
    if found {
        c.cycles += 4;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1c4));
        call_ptr(c, 0x0030_c820, 0x0023_099c)?;
    } else {
        c.cycles += 18;
        c.g[A0][0] = 0x1e;
        call_ptr(c, 0x0020_ab50, 0x0023_0954)?;
        c.g[A0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_DEPTH));
        c.g[V1][0] = GMU_STATE_STACK;
        c.g[A1][0] = 0x44_0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        c.g[A2][0] = c.lw(GMU_STATE_FLAGS_3);
        c.g[V0][0] = sx(r32(c, A0) << 2);
        c.sw(gmu_gp(c, GMU_GP_STATE_DEPTH), c.g[A0][0]);
        c.g[A3][0] = 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.sw(r32(c, V0), c.g[A3][0]);
        c.sw(gmu_gp(c, GMU_GP_STATE_FLAGS), c.g[A2][0]);
        c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_UI));
        call_ptr(c, 0x0039_8038, 0x0023_098c)?;
    }
    c.g[V1][0] = 0;
    Ok(())
}

/// A pending overlay: overlay 0x22, push state 12, clear the request and
/// stop the riders' sound loops. Leaves `v1 = 0`.
fn gmu_pending_overlay(c: &mut Cpu) -> Flow {
    c.cycles += 22;
    c.g[A0][0] = 0x22;
    call_ptr(c, 0x0020_ab50, 0x0023_09bc)?;
    c.g[A1][0] = c.lw(gmu_gp(c, GMU_GP_STATE_DEPTH));
    c.g[V1][0] = 0x44_0000;
    c.g[A0][0] = 0x53_0000;
    c.g[A2][0] = c.lw(GMU_STATE_FLAGS_12);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
    c.g[A0][0] = GMU_STATE_STACK;
    c.g[V0][0] = sx(r32(c, A1) << 2);
    c.sw(gmu_gp(c, GMU_GP_STATE_FLAGS), c.g[A2][0]);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    c.g[V1][0] = u64::MAX;
    c.g[A0][0] = 0xc;
    c.sw(r32(c, V0), c.g[A0][0]);
    c.sw(gmu_gp(c, GMU_GP_PENDING_OVERLAY), c.g[V1][0]);
    c.sw(gmu_gp(c, GMU_GP_STATE_DEPTH), c.g[A1][0]);
    call_ptr(c, 0x0028_b180, 0x0023_09fc)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0029_ed18, 0x0023_0a04)?;
    c.g[V1][0] = 0;
    Ok(())
}

/// Open the pause menu: stop the pads' actuators, open the menu, push state
/// 2, tell the UI (method 0x160 of the UI's state object, with the pause
/// argument), pause the game audio and tick the listener. Leaves `v1 = 0`.
fn gmu_open_pause(c: &mut Cpu) -> Flow {
    c.cycles += 34;
    c.g[A0][0] = c.lw(gmu_gp(c, GMU_GP_PAD_QUEUE));
    call_ptr(c, 0x0032_6ae0, 0x0023_0a8c)?;
    call_ptr(c, 0x0020_ca10, 0x0023_0a94)?;
    c.g[A0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_DEPTH));
    c.g[V1][0] = GMU_STATE_STACK;
    c.g[A1][0] = 0x44_0000;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
    c.g[A3][0] = 2;
    c.g[V0][0] = sx(r32(c, A0) << 2);
    c.g[A2][0] = c.lw(GMU_STATE_FLAGS_2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.sw(gmu_gp(c, GMU_GP_STATE_DEPTH), c.g[A0][0]);
    c.sw(r32(c, V0), c.g[A3][0]);
    c.sw(gmu_gp(c, GMU_GP_STATE_FLAGS), c.g[A2][0]);
    c.g[V1][0] = c.lw(gmu_field(c, GM_UPDATE_UI));
    c.g[A1][0] = c.lbu(gmu_field(c, GM_UPDATE_PAUSE_ARG));
    c.g[A2][0] = c.lw(r32(c, V1).wrapping_add(0xc));
    c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(8));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x160));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x164));
    let method = r32(c, V1);
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    call_ptr(c, method, 0x0023_0ae4)?;
    call_ptr(c, 0x0028_b180, 0x0023_0aec)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0028_9b70, 0x0023_0af4)?;
    call_ptr(c, 0x0028_b180, 0x0023_0afc)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0029_4f48, 0x0023_0b04)?;
    c.g[V1][0] = 0;
    Ok(())
}

/// The world's frame (`a0` = the input sync): the state change, the replay,
/// the script engine's method, the process lists, scripts, crowds and
/// particles (unless paused or waiting for input), the cameras, the frame
/// counter, the world sounds per view and every rider's update.
fn gmu_update_world(c: &mut Cpu) -> Flow {
    'tick: {
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[V1][0] = c.lw(gmu_field(c, GM_UPDATE_MODE));
        } else {
            c.cycles += 4;
            GMU_INPUT_SYNC_READY.call(c, 0x0023_0b6c)?;
            let waiting = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS));
            if waiting {
                break 'tick;
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw(gmu_field(c, GM_UPDATE_MODE));
        }
        c.cycles += 3;
        c.g[V0][0] = 3;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            GMU_PROCESS_STATE_CHANGE.call(c, 0x0023_0b8c)?;
            gmu_process_list(c, 0x0035_4c98, 1, 0x0023_0b98)?;
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            GMU_PROCESS_STATE_CHANGE.call(c, 0x0023_0ba8)?;
        }
        c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS));
        c.cycles += 3;
        c.g[V0][0] &= 1;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_RACE));
            call_ptr(c, 0x0026_f4a8, 0x0023_0bc0)?;
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_RACE));
        call_ptr(c, 0x0027_0280, 0x0023_0bc8)?;
        let replay_paused = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS));
        if replay_paused {
            break 'tick;
        }
        c.cycles += 3;
        c.g[V0][0] &= 8;
        let no_script = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS));
        if no_script {
            break 'tick;
        }
        c.cycles += 7;
        c.g[A1][0] = c.lw(gmu_gp(c, GMU_GP_SCRIPT_ENGINE));
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x2a8));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x28));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x2c));
        let method = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0023_0bf4)?;
        c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS));
    }
    // Skip the simulation when paused, the replay is paused, or the input
    // for this frame is missing.
    c.cycles += 3;
    c.g[V0][0] &= 1;
    let paused = c.g[V0][0] != 0;
    c.g[S0][0] = 0;
    let skip = if paused {
        true
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_RACE));
        call_ptr(c, 0x0027_0280, 0x0023_0c0c)?;
        c.g[V0][0] != 0
    };
    if skip {
        c.cycles += 1;
        c.g[S0][0] = 1;
    }
    let full = 'decide: {
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            break 'decide false;
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_INPUT_SYNC));
        'input: {
            if c.g[A0][0] != 0 {
                c.cycles += 4;
                GMU_INPUT_SYNC_APPLY.call(c, 0x0023_0c34)?;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = 1;
                    break 'input;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_PLAYBACK));
            call_ptr(c, 0x0026_ce50, 0x0023_0c44)?;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = 1;
            }
        }
        c.cycles += 2;
        let stalled = c.g[S0][0] != 0;
        c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS));
        !stalled
    };
    if full {
        gmu_simulate(c)?;
    } else {
        c.cycles += 7;
        gmu_process_list(c, 0x0035_4f98, 5, 0x0023_0d30)?;
        c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS)) & 4;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            gmu_process_list(c, 0x0035_4f98, 3, 0x0023_0d4c)?;
        }
    }
    c.cycles += 7;
    gmu_process_list(c, 0x0035_4f98, 4, 0x0023_0d58)?;
    // The cameras, unless in a race other than state 3.
    c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_RACE));
    c.g[V0][0] = c.lw(r32(c, V0));
    let idle = c.g[V0][0] == 0;
    c.g[V1][0] = 0;
    if !idle {
        c.cycles += 1;
        c.g[V1][0] = ((c.g[V0][0] as i64) < 10) as u64;
    }
    c.cycles += 2;
    let in_race = c.g[V1][0] != 0;
    c.g[V0][0] ^= 3;
    let cameras = if !in_race {
        true
    } else {
        c.cycles += 2;
        c.g[V0][0] != 0
    };
    if cameras {
        c.cycles += 3;
        c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_VIEW_MAN));
        call_ptr(c, 0x0022_e840, 0x0023_0d84)?;
    }
    c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_FRAME));
    // The frame counter, then the world sounds of every view.
    c.cycles += 6;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_VIEW_MAN));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    c.sw(gmu_field(c, GM_UPDATE_FRAME), c.g[V0][0]);
    c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x10));
    let no_views = c.g[V1][0] == 0;
    c.g[S0][0] = 0;
    if !no_views {
        c.cycles += 2;
        c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_VIEW_MAN));
        loop {
            c.cycles += 13;
            c.g[V1][0] = sx(r32(c, S0) << 2);
            c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_WORLD_SOUND));
            c.g[A2][0] = c.g[S0][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(4));
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x20));
            call_ptr(c, 0x002b_7848, 0x0023_0dc8)?;
            c.g[V1][0] = c.lw(gmu_field(c, GM_UPDATE_VIEW_MAN));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x10));
            c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_VIEW_MAN));
            if event_due(c) {
                deliver_event_here(c, 0x0023_0da8);
            }
        }
    }
    // Every rider's update (method 0x98 of the rider's vtable at +0x6c0).
    c.cycles += 4;
    c.g[V1][0] = c.lw(gmu_field(c, GM_UPDATE_RIDERS));
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x78));
    let no_riders = (c.g[V0][0] as i64) <= 0;
    c.g[S0][0] = 0;
    if !no_riders {
        c.cycles += 1;
        c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_RIDERS));
        loop {
            c.cycles += 14;
            c.g[V1][0] = sx(r32(c, S0) << 2);
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0x28));
            c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x6c0));
            c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x98));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x9c));
            let method = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0023_0e14)?;
            c.g[V1][0] = c.lw(gmu_field(c, GM_UPDATE_RIDERS));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x78));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lw(gmu_field(c, GM_UPDATE_RIDERS));
            if event_due(c) {
                deliver_event_here(c, 0x0023_0df0);
            }
        }
    }
    Ok(())
}

/// The simulation step (`v0` = the state flags): the process lists (by the
/// flags: 0x2 skips the player lists, 0x10 list 2, 0x4 list 3), the rider
/// list's method 0x18, scripts, crowds and particles, then merge list 1.
fn gmu_simulate(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] &= 2;
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        gmu_process_list(c, 0x0035_4f98, 1, 0x0023_0c6c)?;
    }
    c.cycles += 10;
    gmu_process_list(c, 0x0035_4f98, 5, 0x0023_0c78)?;
    gmu_process_list(c, 0x0035_4f98, 6, 0x0023_0c84)?;
    c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS)) & 2;
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        gmu_process_list(c, 0x0035_5028, 1, 0x0023_0ca0)?;
    }
    c.g[A1][0] = c.lw(gmu_field(c, GM_UPDATE_RIDERS));
    c.cycles += 9;
    c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0xcc));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x18));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1c));
    let method = r32(c, V1);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, method, 0x0023_0cb8)?;
    c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS)) & 0x10;
    let no_list_2 = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS));
    if !no_list_2 {
        c.cycles += 4;
        gmu_process_list(c, 0x0035_4f98, 2, 0x0023_0cd4)?;
        c.g[V0][0] = c.lw(gmu_gp(c, GMU_GP_STATE_FLAGS));
    }
    c.cycles += 3;
    c.g[V0][0] &= 4;
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        gmu_process_list(c, 0x0035_4f98, 3, 0x0023_0cf0)?;
    }
    c.cycles += 13;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_WSCRIPT));
    call_ptr(c, 0x0030_9270, 0x0023_0cf8)?;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_CROWD_2D));
    call_ptr(c, 0x0022_94c8, 0x0023_0d00)?;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_CROWD_3D));
    call_ptr(c, 0x0034_40c8, 0x0023_0d08)?;
    c.g[A0][0] = c.lw(gmu_field(c, GM_UPDATE_PARTICLES));
    call_ptr(c, 0x0035_7bf8, 0x0023_0d10)?;
    gmu_process_list(c, 0x0035_4c98, 1, 0x0023_0d1c)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's callee fakes (in the test machine's RAM) and the
    /// result code of each fake (`gmu_fake`).
    const GMU_CTRL: u32 = DATA + 0x2f_f000;
    const GMU_MAGIC: u32 = 0x6a3e_0b58;
    const GMU_CODES: u32 = GMU_CTRL + 0x10;
    /// Result codes: 0 / 1, a random bit per call, the fake's own pointer;
    /// anything else is returned as is (a pointer).
    const GMU_ZERO: u32 = 0;
    const GMU_ONE: u32 = 1;
    const GMU_BIT: u32 = 2;
    const GMU_HASH: u32 = 3;

    /// A logging fake (`fakes::fake`) whose `v0` the fixture picks per callee.
    fn gmu_fake(c: &mut Cpu, addr: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match c.bus.read32(GMU_CODES + 4 * idx) {
            GMU_ZERO => c.g[V0][0] = 0,
            GMU_ONE => c.g[V0][0] = 1,
            GMU_BIT => c.g[V0][0] = (c.g[V1][0] >> 7) & 1,
            GMU_HASH => {}
            p => c.g[V0][0] = sx(p),
        }
    }

    /// Gated fakes for every slot callee the frame reaches (directly, or
    /// through the real callees the fixture lets run: the game UI, the
    /// section manager, the script manager, the screens, the pause menu).
    macro_rules! gmu_fakes {
        ($($name:ident $prev:ident $idx:literal $addr:literal $shard:ident $slot:ident $gen:ident;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(GMU_CTRL) != GMU_MAGIC {
                        return fakes::fall_back(c, &$prev, ee::$shard::$gen);
                    }
                    gmu_fake(c, $addr, $idx)
                }
            )*
            fn gmu_install_fakes() {
                $(
                    fakes::hook(std::ptr::addr_of_mut!(ee::$shard::$slot), $name, &$prev);
                )*
            }
        };
    }

    // Indices 1..=14 get chosen results; the rest return the fake's pointer.
    const GMU_I_VIEWS_READY: u32 = 1;
    const GMU_I_MENU: u32 = 2;
    const GMU_I_PEEK: u32 = 3;
    const GMU_I_FIND_STATE: u32 = 4;
    const GMU_I_CHECK_PAUSE: u32 = 5;
    const GMU_I_CAN_PAUSE: u32 = 6;
    const GMU_I_PAUSE_OPEN: u32 = 7;
    const GMU_I_SYNC_READY: u32 = 8;
    const GMU_I_SYNC_APPLY: u32 = 9;
    const GMU_I_REPLAY_PAUSED: u32 = 10;
    const GMU_I_READ_INPUT: u32 = 11;
    const GMU_I_POP_FRONT: u32 = 12;
    const GMU_I_NEXT_SECTION: u32 = 13;
    const GMU_I_MISSION_2: u32 = 14;
    const GMU_FAKES: u32 = 57;

    gmu_fakes! {
        gmu_f_2ee400 GMU_P_2EE400 0 0x002e_e400 s09 OVERRIDE_002EE400 f_002ee400__generated;
        gmu_f_22e920 GMU_P_22E920 1 0x0022_e920 s05 OVERRIDE_0022E920 f_0022e920__generated;
        gmu_f_2cc098 GMU_P_2CC098 2 0x002c_c098 s09 OVERRIDE_002CC098 f_002cc098__generated;
        gmu_f_30c870 GMU_P_30C870 3 0x0030_c870 s10 OVERRIDE_0030C870 f_0030c870__generated;
        gmu_f_39f9d8 GMU_P_39F9D8 4 0x0039_f9d8 s13 OVERRIDE_0039F9D8 f_0039f9d8__generated;
        gmu_f_231840 GMU_P_231840 5 0x0023_1840 s06 OVERRIDE_00231840 f_00231840__generated;
        gmu_f_20cbe8 GMU_P_20CBE8 6 0x0020_cbe8 s05 OVERRIDE_0020CBE8 f_0020cbe8__generated;
        gmu_f_20cba0 GMU_P_20CBA0 7 0x0020_cba0 s05 OVERRIDE_0020CBA0 f_0020cba0__generated;
        gmu_f_237950 GMU_P_237950 8 0x0023_7950 s06 OVERRIDE_00237950 f_00237950__generated;
        gmu_f_2379c8 GMU_P_2379C8 9 0x0023_79c8 s06 OVERRIDE_002379C8 f_002379c8__generated;
        gmu_f_270280 GMU_P_270280 10 0x0027_0280 s07 OVERRIDE_00270280 f_00270280__generated;
        gmu_f_26ce50 GMU_P_26CE50 11 0x0026_ce50 s07 OVERRIDE_0026CE50 f_0026ce50__generated;
        gmu_f_30b208 GMU_P_30B208 12 0x0030_b208 s10 OVERRIDE_0030B208 f_0030b208__generated;
        gmu_f_22e348 GMU_P_22E348 13 0x0022_e348 s05 OVERRIDE_0022E348 f_0022e348__generated;
        gmu_f_30b8c0 GMU_P_30B8C0 14 0x0030_b8c0 s10 OVERRIDE_0030B8C0 f_0030b8c0__generated;
        gmu_f_1a39f0 GMU_P_1A39F0 15 0x001a_39f0 s03 OVERRIDE_001A39F0 f_001a39f0__generated;
        gmu_f_1e1458 GMU_P_1E1458 16 0x001e_1458 s04 OVERRIDE_001E1458 f_001e1458__generated;
        gmu_f_1f3188 GMU_P_1F3188 17 0x001f_3188 s04 OVERRIDE_001F3188 f_001f3188__generated;
        gmu_f_2294c8 GMU_P_2294C8 18 0x0022_94c8 s05 OVERRIDE_002294C8 f_002294c8__generated;
        gmu_f_22e840 GMU_P_22E840 19 0x0022_e840 s05 OVERRIDE_0022E840 f_0022e840__generated;
        gmu_f_22e8b8 GMU_P_22E8B8 20 0x0022_e8b8 s05 OVERRIDE_0022E8B8 f_0022e8b8__generated;
        gmu_f_230f40 GMU_P_230F40 21 0x0023_0f40 s06 OVERRIDE_00230F40 f_00230f40__generated;
        gmu_f_231d18 GMU_P_231D18 22 0x0023_1d18 s06 OVERRIDE_00231D18 f_00231d18__generated;
        gmu_f_26c5f8 GMU_P_26C5F8 23 0x0026_c5f8 s07 OVERRIDE_0026C5F8 f_0026c5f8__generated;
        gmu_f_26f4a8 GMU_P_26F4A8 24 0x0026_f4a8 s07 OVERRIDE_0026F4A8 f_0026f4a8__generated;
        gmu_f_278918 GMU_P_278918 25 0x0027_8918 s07 OVERRIDE_00278918 f_00278918__generated;
        gmu_f_289b70 GMU_P_289B70 26 0x0028_9b70 s07 OVERRIDE_00289B70 f_00289b70__generated;
        gmu_f_28b180 GMU_P_28B180 27 0x0028_b180 s07 OVERRIDE_0028B180 f_0028b180__generated;
        gmu_f_294f48 GMU_P_294F48 28 0x0029_4f48 s08 OVERRIDE_00294F48 f_00294f48__generated;
        gmu_f_29ed18 GMU_P_29ED18 29 0x0029_ed18 s08 OVERRIDE_0029ED18 f_0029ed18__generated;
        gmu_f_2b7848 GMU_P_2B7848 30 0x002b_7848 s08 OVERRIDE_002B7848 f_002b7848__generated;
        gmu_f_2cc0c0 GMU_P_2CC0C0 31 0x002c_c0c0 s09 OVERRIDE_002CC0C0 f_002cc0c0__generated;
        gmu_f_30c820 GMU_P_30C820 32 0x0030_c820 s10 OVERRIDE_0030C820 f_0030c820__generated;
        gmu_f_317670 GMU_P_317670 33 0x0031_7670 s10 OVERRIDE_00317670 f_00317670__generated;
        gmu_f_3440c8 GMU_P_3440C8 34 0x0034_40c8 s11 OVERRIDE_003440C8 f_003440c8__generated;
        gmu_f_354c98 GMU_P_354C98 35 0x0035_4c98 s11 OVERRIDE_00354C98 f_00354c98__generated;
        gmu_f_354f98 GMU_P_354F98 36 0x0035_4f98 s11 OVERRIDE_00354F98 f_00354f98__generated;
        gmu_f_355028 GMU_P_355028 37 0x0035_5028 s11 OVERRIDE_00355028 f_00355028__generated;
        gmu_f_357bf8 GMU_P_357BF8 38 0x0035_7bf8 s11 OVERRIDE_00357BF8 f_00357bf8__generated;
        gmu_f_398038 GMU_P_398038 39 0x0039_8038 s13 OVERRIDE_00398038 f_00398038__generated;
        gmu_f_3a6928 GMU_P_3A6928 40 0x003a_6928 s13 OVERRIDE_003A6928 f_003a6928__generated;
        gmu_f_39fa88 GMU_P_39FA88 41 0x0039_fa88 s13 OVERRIDE_0039FA88 f_0039fa88__generated;
        gmu_f_309118 GMU_P_309118 42 0x0030_9118 s10 OVERRIDE_00309118 f_00309118__generated;
        gmu_f_30b260 GMU_P_30B260 43 0x0030_b260 s10 OVERRIDE_0030B260 f_0030b260__generated;
        gmu_f_309750 GMU_P_309750 44 0x0030_9750 s10 OVERRIDE_00309750 f_00309750__generated;
        gmu_f_14dc80 GMU_P_14DC80 45 0x0014_dc80 s01 OVERRIDE_0014DC80 f_0014dc80__generated;
        gmu_f_14dd58 GMU_P_14DD58 46 0x0014_dd58 s01 OVERRIDE_0014DD58 f_0014dd58__generated;
        gmu_f_317d70 GMU_P_317D70 47 0x0031_7d70 s10 OVERRIDE_00317D70 f_00317d70__generated;
        gmu_f_39e2a0 GMU_P_39E2A0 48 0x0039_e2a0 s13 OVERRIDE_0039E2A0 f_0039e2a0__generated;
        gmu_f_39f320 GMU_P_39F320 49 0x0039_f320 s13 OVERRIDE_0039F320 f_0039f320__generated;
        gmu_f_1e3c00 GMU_P_1E3C00 50 0x001e_3c00 s04 OVERRIDE_001E3C00 f_001e3c00__generated;
        gmu_f_1f7548 GMU_P_1F7548 51 0x001f_7548 s04 OVERRIDE_001F7548 f_001f7548__generated;
        gmu_f_1f7738 GMU_P_1F7738 52 0x001f_7738 s04 OVERRIDE_001F7738 f_001f7738__generated;
        gmu_f_1fdbf0 GMU_P_1FDBF0 53 0x001f_dbf0 s05 OVERRIDE_001FDBF0 f_001fdbf0__generated;
        gmu_f_1fe968 GMU_P_1FE968 54 0x001f_e968 s05 OVERRIDE_001FE968 f_001fe968__generated;
        gmu_f_2006b8 GMU_P_2006B8 55 0x0020_06b8 s05 OVERRIDE_002006B8 f_002006b8__generated;
        gmu_f_212138 GMU_P_212138 56 0x0021_2138 s05 OVERRIDE_00212138 f_00212138__generated;
    }

    /// Fixture objects, far from the fakes' result pointers
    /// (`DATA..DATA+0x40000`).
    const GMU_BASE: u32 = DATA + 0x20_0000;
    const GMU_MODULE: u32 = GMU_BASE;
    const GMU_VIEW_MAN: u32 = GMU_BASE + 0x1000;
    const GMU_SECTION_MAN: u32 = GMU_BASE + 0x2000;
    const GMU_WORLD_REF: u32 = GMU_BASE + 0x3000;
    const GMU_WORLD_OBJ: u32 = GMU_BASE + 0x3100;
    const GMU_RACE: u32 = GMU_BASE + 0x4000;
    const GMU_WSCRIPT: u32 = GMU_BASE + 0x5000;
    const GMU_PLAYBACK: u32 = GMU_BASE + 0x6000;
    const GMU_UI: u32 = GMU_BASE + 0x7000;
    const GMU_UI_STATE: u32 = GMU_BASE + 0x7100;
    const GMU_UI_VT: u32 = GMU_BASE + 0x7200;
    const GMU_RIDERS: u32 = GMU_BASE + 0x8000;
    const GMU_RIDER_LIST_VT: u32 = GMU_BASE + 0x8200;
    const GMU_RIDER: u32 = GMU_BASE + 0x8400; // + 0x800 per rider
    const GMU_RIDER_VT: u32 = GMU_BASE + 0x8300;
    const GMU_SCRIPT_ENGINE: u32 = GMU_BASE + 0xa000;
    const GMU_SCRIPT_ENGINE_VT: u32 = GMU_BASE + 0xa400;
    const GMU_AUDIO_HOST: u32 = GMU_BASE + 0xb000;
    const GMU_AUDIO_HOST_VT: u32 = GMU_BASE + 0xc100;
    const GMU_WORLD: u32 = GMU_BASE + 0xd000;
    const GMU_FOG: u32 = GMU_BASE + 0xd400;
    const GMU_PAD_QUEUE: u32 = GMU_BASE + 0xe000;
    const GMU_EVENT: u32 = GMU_BASE + 0x11000;
    const GMU_VIEWS: u32 = GMU_BASE + 0x11100;

    fn gmu_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    fn gmu_gp_w(c: &mut Cpu, off: i32, v: u32) {
        c.bus.write32(GAME_GP.wrapping_add(off as u32), v);
    }

    /// Set the result code of fake `i`.
    fn gmu_code(c: &mut Cpu, i: u32, v: u32) {
        c.bus.write32(GMU_CODES + 4 * i, v);
    }

    /// One of `[0, 1, random bit]`, weighted toward 0.
    fn gmu_bool(r: &mut Rng) -> u32 {
        match r.next_u32() % 5 {
            0 | 1 => GMU_ZERO,
            2 => GMU_ONE,
            _ => GMU_BIT,
        }
    }

    /// A vtable slot `{delta, method}` at `vt + off`.
    fn gmu_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, method: u32) {
        c.bus.write16(vt + off, (r.next_u32() & 0x3c) as u16);
        c.bus.write32(vt + off + 4, method);
    }

    /// `GameModule_Update` against the generated body. Every slot callee is
    /// a gated logging fake with a chosen result; the callees without a slot
    /// run for real on short paths (game UI with no pending menu result,
    /// section manager with nothing queued, script manager with no
    /// processes, empty pad queue and hull cache, idle net). Event 8 (its
    /// multiplier sound runs deep into the audio system) and the network game
    /// update (`gp-0x204`) stay off.
    #[test]
    fn game_module_update_matches() {
        gmu_install_fakes();
        fakes::covered(
            ee::s06::f_002306b8__generated,
            game_module_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2306b8, 40);
                c.bus.write32(GMU_CTRL, GMU_MAGIC);
                for i in 0..GMU_FAKES {
                    c.bus.write32(GMU_CODES + 4 * i, GMU_HASH);
                }
                gmu_code(c, GMU_I_VIEWS_READY, gmu_bool(&mut r));
                let menu = match seed % 4 {
                    0 | 1 => GMU_ZERO,
                    2 => GMU_BIT,
                    _ => GMU_ONE,
                };
                gmu_code(c, GMU_I_MENU, menu);
                gmu_code(c, GMU_I_PEEK, GMU_EVENT);
                gmu_code(c, GMU_I_FIND_STATE, gmu_bool(&mut r));
                gmu_code(c, GMU_I_CHECK_PAUSE, gmu_bool(&mut r));
                gmu_code(c, GMU_I_CAN_PAUSE, gmu_bool(&mut r));
                gmu_code(c, GMU_I_PAUSE_OPEN, gmu_bool(&mut r));
                gmu_code(c, GMU_I_SYNC_READY, gmu_bool(&mut r));
                gmu_code(c, GMU_I_SYNC_APPLY, gmu_bool(&mut r));
                gmu_code(c, GMU_I_REPLAY_PAUSED, gmu_bool(&mut r));
                gmu_code(c, GMU_I_READ_INPUT, gmu_bool(&mut r));
                gmu_code(c, GMU_I_POP_FRONT, GMU_ZERO);
                gmu_code(c, GMU_I_NEXT_SECTION, GMU_ZERO);
                gmu_code(c, GMU_I_MISSION_2, GMU_BIT);

                // The module.
                let m = GMU_MODULE;
                gmu_w(c, m + GM_UPDATE_HALTED, r.next_u32());
                gmu_w(c, m + GM_UPDATE_RIDERS, GMU_RIDERS);
                gmu_w(c, m + GM_UPDATE_WORLD_REF, GMU_WORLD_REF);
                gmu_w(c, m + GM_UPDATE_RACE, GMU_RACE);
                gmu_w(c, m + GM_UPDATE_WSCRIPT, GMU_WSCRIPT);
                gmu_w(c, m + GM_UPDATE_PLAYBACK, GMU_PLAYBACK);
                gmu_w(c, m + GM_UPDATE_CROWD_2D, r.next_u32());
                gmu_w(c, m + GM_UPDATE_CROWD_3D, r.next_u32());
                gmu_w(c, m + GM_UPDATE_PARTICLES, r.next_u32());
                gmu_w(c, m + GM_UPDATE_UI, GMU_UI);
                gmu_w(c, m + GM_UPDATE_WORLD_SOUND, r.next_u32());
                gmu_w(c, m + GM_UPDATE_SECTION_MAN, GMU_SECTION_MAN);
                let sync = if seed % 3 == 0 { 0 } else { GMU_BASE + 0x12000 };
                gmu_w(c, m + GM_UPDATE_INPUT_SYNC, sync);
                gmu_w(c, m + GM_UPDATE_VIEW_MAN, GMU_VIEW_MAN);
                gmu_w(c, m + GM_UPDATE_LOAD_INDICATOR, r.next_u32());
                gmu_w(c, m + GM_UPDATE_WAS_HELD, r.next_u32() & 1);
                gmu_w(c, m + GM_UPDATE_PREFETCH, r.next_u32() & 1);
                gmu_w(c, m + GM_UPDATE_MENU, r.next_u32());
                gmu_w(c, m + GM_UPDATE_FRAME, r.next_u32());
                let mode = if r.next_u32() % 3 == 0 {
                    3
                } else {
                    r.next_u32() % 8
                };
                gmu_w(c, m + GM_UPDATE_MODE, mode);
                c.bus.write8(m + GM_UPDATE_PAUSE_ARG, r.next_u32() as u8);

                // Views (0..=3) for the fog choice and the world sounds.
                let views = r.next_u32() % 4;
                gmu_w(c, GMU_VIEW_MAN + 0x10, views);
                for i in 0..3 {
                    gmu_w(c, GMU_VIEW_MAN + 4 + 4 * i, GMU_VIEWS + 0x100 * i);
                }
                // The section manager: nothing queued, sections loaded or not.
                gmu_w(c, GMU_SECTION_MAN + 0x1d0, r.next_u32() & 1);
                // The world reference and its world (empty hull cache).
                gmu_w(c, GMU_WORLD_REF, GMU_WORLD_OBJ);
                // Race state: idle, in a race (3 or other), past the race.
                let race = match r.next_u32() % 5 {
                    0 => 0,
                    1 => 3,
                    2 => 1 + r.next_u32() % 9,
                    3 => 10 + r.next_u32() % 4,
                    _ => r.next_u32() | 0x8000_0000,
                };
                gmu_w(c, GMU_RACE, race);
                // The script manager's event queue: empty, or event 9 / other.
                let head = r.next_u32() & !3;
                gmu_w(c, GMU_WSCRIPT + 0x1c4, head);
                let queued = seed % 3 != 0;
                gmu_w(
                    c,
                    GMU_WSCRIPT + 0x1c8,
                    if queued { head ^ 0x10 } else { head },
                );
                let kind = if r.next_u32() % 2 == 0 {
                    9
                } else {
                    r.next_u32() % 8
                };
                let kind = if kind == 8 { 7 } else { kind };
                gmu_w(c, GMU_EVENT, r.next_u32());
                gmu_w(c, GMU_EVENT + 4, kind);
                gmu_w(c, GMU_EVENT + 8, r.next_u32());
                // Playback state: playing (1) or not.
                let playing = if r.next_u32() % 2 == 0 {
                    1
                } else {
                    r.next_u32()
                };
                gmu_w(c, GMU_PLAYBACK + 0x70, playing);
                // The UI's state object: method 0x160 (the pause).
                gmu_w(c, GMU_UI + 0xc, GMU_UI_STATE);
                gmu_w(c, GMU_UI_STATE + 8, GMU_UI_VT);
                gmu_slot(c, &mut r, GMU_UI_VT, 0x160, FAKE + 0x50);
                // The riders: method 0x18 of the list, method 0x98 of each rider.
                let riders = match r.next_u32() % 5 {
                    0 => 0,
                    1 => r.next_u32() | 0x8000_0000,
                    _ => 1 + r.next_u32() % 3,
                };
                gmu_w(c, GMU_RIDERS + 0x78, riders);
                gmu_w(c, GMU_RIDERS + 0xcc, GMU_RIDER_LIST_VT);
                gmu_slot(c, &mut r, GMU_RIDER_LIST_VT, 0x18, FAKE + 0x30);
                gmu_slot(c, &mut r, GMU_RIDER_VT, 0x98, FAKE + 0x10);
                for i in 0..3 {
                    let rider = GMU_RIDER + 0x800 * i;
                    gmu_w(c, GMU_RIDERS + 0x28 + 4 * i, rider);
                    gmu_w(c, rider + 0x6c0, GMU_RIDER_VT);
                }
                // The script engine's method 0x28 and the audio host's 0x98.
                gmu_gp_w(c, GMU_GP_SCRIPT_ENGINE, GMU_SCRIPT_ENGINE);
                gmu_w(c, GMU_SCRIPT_ENGINE + 0x2a8, GMU_SCRIPT_ENGINE_VT);
                gmu_slot(c, &mut r, GMU_SCRIPT_ENGINE_VT, 0x28, FAKE + 0x20);
                gmu_gp_w(c, GMU_GP_AUDIO_HOST, GMU_AUDIO_HOST);
                gmu_w(c, GMU_AUDIO_HOST + 0x10d8, GMU_AUDIO_HOST_VT);
                gmu_slot(c, &mut r, GMU_AUDIO_HOST_VT, 0x98, FAKE + 0x40);
                // The world: loading or not; the FE async manager or none.
                gmu_gp_w(c, GMU_GP_WORLD, GMU_WORLD);
                gmu_w(
                    c,
                    GMU_WORLD + 4,
                    if seed % 6 == 5 { r.next_u32() | 1 } else { 0 },
                );
                gmu_w(c, GMU_WORLD + 0x84, GMU_MODULE);
                gmu_w(c, GMU_WORLD + 0x78, r.next_u32());
                let fe_async = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    r.next_u32() | 1
                };
                gmu_w(c, GMU_WORLD + 0x11c, fe_async);
                // Globals.
                gmu_gp_w(c, GMU_GP_FOG, GMU_FOG);
                gmu_gp_w(c, GMU_GP_SKY_FOG_MIN, 0x3f66_6666);
                gmu_gp_w(c, GMU_GP_GAME_COMM, 0);
                gmu_gp_w(c, GMU_GP_PAD_QUEUE, GMU_PAD_QUEUE);
                gmu_gp_w(c, GMU_GP_NET_CONN, r.next_u32() & 1);
                gmu_gp_w(c, 0x1ab0, 0); // net: no ticks pending
                gmu_gp_w(c, -0x9f4, 5 + r.next_u32() % 16); // no menu result
                gmu_gp_w(c, GMU_GP_STATE_DEPTH, r.next_u32() % 6);
                let flags = if seed % 4 == 0 {
                    0
                } else {
                    r.next_u32() & 0x1f
                };
                gmu_gp_w(c, GMU_GP_STATE_FLAGS, flags);
                let pending = if r.next_u32() % 5 == 0 {
                    r.next_u32() % 0x30
                } else {
                    u32::MAX
                };
                gmu_gp_w(c, GMU_GP_PENDING_OVERLAY, pending);
                gmu_gp_w(c, GMU_GP_WSCRIPT, r.next_u32());
                gmu_w(c, GMU_STATE_FLAGS_2, r.next_u32());
                gmu_w(c, GMU_STATE_FLAGS_3, r.next_u32());
                gmu_w(c, GMU_STATE_FLAGS_12, r.next_u32());
                // The pause menu opens without the network game; the mission
                // command has no mission running.
                gmu_w(c, 0x0053_4b30, 0);
                gmu_w(c, 0x0053_5c10, 0);

                c.g[A0][0] = sx(GMU_MODULE);
                c.pc = 0x0023_06b8;
            },
        );
    }
}
