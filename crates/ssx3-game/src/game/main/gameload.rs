//! `src/main/gameload.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::dirtysock::tagsunk::arg;
pub(crate) use crate::game::dirtysock::tagsunk::gp_3 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_m2;
pub(crate) use crate::game::fe::feasyncfile::gp_12 as gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp_4;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p as p_m2;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_3;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::render::lightman::at;
pub(crate) use crate::game::render::lightman::gp_2 as gp;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_2;
use crate::game::visualfx::boardwakefx as bw;
#[cfg(test)]
pub(crate) use crate::game::visualfx::boardwakefx::CAM_PROJECTOR_VT;
#[cfg(test)]
pub(crate) use crate::game::visualfx::boardwakefx::LENS_CAMERA_GP;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0023_00f0,
        game_module_reset_scripts_and_world,
        "GameModule_ResetScriptsAndWorld",
    ),
    (
        0x0023_02a8,
        game_module_quick_restart,
        "GameModule_QuickRestart",
    ),
    (
        0x0023_0338,
        game_module_invalidate_focus,
        "GameModule_InvalidateFocus",
    ),
    (
        0x0023_0360,
        game_module_on_section_unload,
        "GameModule_OnSectionUnload",
    ),
    (
        0x0023_05c8,
        game_module_end_replay_playback,
        "GameModule_EndReplayPlayback",
    ),
    (
        0x0023_0698,
        game_module_get_transition_man,
        "GameModule_GetTransitionMan",
    ),
    (
        0x0023_06a8,
        game_module_get_splash_manager,
        "GameModule_GetSplashManager",
    ),
    (0x0023_0e98, game_module_try_exit, "GameModule_TryExit"),
    (
        0x0023_1320,
        game_module_advance_state,
        "GameModule_AdvanceState",
    ),
    (
        0x0023_1840,
        game_module_check_pause_request,
        "GameModule_CheckPauseRequest",
    ),
    (
        0x0023_1c70,
        app_module_try_exit_when_loaded,
        "AppModule_TryExitWhenLoaded",
    ),
    (0x0023_1cd0, app_module_ctor, "AppModule_ctor"),
    (0x0023_2328, pre_start_screen_ctor, "PreStartScreen_ctor"),
    (
        0x0023_82d8,
        game_mode_man_reset_mode,
        "GameModeMan_ResetMode",
    ),
    (0x0023_1d60, app_module_render_fade, "AppModule_RenderFade"),
    (0x0023_1348, game_module_save_state, "GameModule_SaveState"),
    (
        0x0023_0f40,
        game_module_process_state_change,
        "GameModule_ProcessStateChange",
    ),
    (0x0023_1cf0, app_module_try_exit, "AppModule_TryExit"),
    (0x0023_06b8, game_module_update, "GameModule_Update"),
    (0x0023_1cb0, app_module_update, "AppModule_Update"),
    (0x0023_1fc0, load_indicator_render, "LoadIndicator_Render"),
    (0x0022_ebc8, game_module_load, "GameModule_Load"),
    (0x0022_f6b0, game_module_load_world, "GameModule_LoadWorld"),
    (
        0x0023_0430,
        game_module_create_section_script_table,
        "GameModule_CreateSectionScriptTable",
    ),
    (0x0023_0050, game_module_enter_2, "GameModule_Enter"),
    (0x0022_fa98, game_module_unload, "GameModule_Unload"),
    (
        0x0023_1280,
        game_module_cancel_overlay_state,
        "GameModule_CancelOverlayState",
    ),
    (
        0x0023_1b50,
        c_load_screen_draw_splash,
        "cLoadScreen_drawSplash",
    ),
    (
        0x0023_0180,
        oc681_game_module_reset_race,
        "GameModule_ResetRace",
    ),
    (
        0x0023_0488,
        oc681_game_module_quit_to_load_screen,
        "GameModule_QuitToLoadScreen",
    ),
    (
        0x0023_0558,
        oc681_game_module_begin_replay_playback,
        "GameModule_BeginReplayPlayback",
    ),
    (0x0023_14d0, game_module_load_state, "GameModule_LoadState"),
    (
        0x0023_1ab8,
        oc681_game_module_can_restart_from_pause,
        "GameModule_CanRestartFromPause",
    ),
    (0x001d_c930, app_module_unload_001dc930, "AppModule_Unload"),
    (0x001d_c938, app_module_enter_001dc938, "AppModule_Enter"),
];

// ---------------------------------------------------------------------------
// GameModule_CheckPauseRequest (0x231840).

/// `gp` globals of the pause check.
pub(crate) mod pause {

    /// `D_004A2EEC`: the active "loading" screen (its `+8` freezes input).
    pub const LOADING: i32 = -0x204;

    /// `D_004A2E9C`: disables the select-confirm input evaluation.
    pub const NO_CONFIRM: i32 = -0x254;

    /// `D_00534B30`: a pending restart flag.
    #[cfg(test)]
    pub const RESTART: u32 = 0x0053_4b30;
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::GAME_STATE_GOTO_NEXT;

    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_TRY_EXIT_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;

    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;

    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;

    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::APP_MODULE_TRY_EXIT;
}

// ---------------------------------------------------------------------------
// Game module.

/// `void *GameModule_GetTransitionMan(GameModule *self, int index)` (0x230698):
/// `self->transitions[index]` (word at `self + index*4 + 0x5c`).
pub fn game_module_get_transition_man(c: &mut Cpu) {
    const TRANSITIONS: u32 = 0x5c;
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    let slot = (c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32);
    c.g[A0][0] = sx(slot);
    c.g[V0][0] = Addr(slot).off(TRANSITIONS).lw(c);
    abi::ret(c);
}

/// `void *GameModule_GetSplashManager(GameModule *self, int index)` (0x2306a8):
/// `self->splash[index]` (word at `self + index*4 + 0x68`).
pub fn game_module_get_splash_manager(c: &mut Cpu) {
    const SPLASH: u32 = 0x68;
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    let slot = (c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32);
    c.g[A0][0] = sx(slot);
    c.g[V0][0] = Addr(slot).off(SPLASH).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// App module.

/// `AppModule *AppModule_ctor(AppModule *self)` (0x231cd0): install vtable
/// 0x47d980, zero the flag at `+8`, copy the float at `gp-0x51b8` into `+4`,
/// and return `self`.
pub fn app_module_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2680;
    const FADE: u32 = 0x4;
    const FLAG: u32 = 0x8;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 8;
    let fade = Addr((c.g[GP][0] as u32).wrapping_sub(0x51b8)).read_fpu(c);
    c.f[0] = fade.0;
    let vtable = sx(VTABLE);
    c.g[V0][0] = vtable;
    this.off(FLAG).sw(c, 0);
    this.sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.off(FADE).write_fpu(c, fade);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Simple screen/state constructors: install a vtable and zero some fields.

/// `PreStartScreen *PreStartScreen_ctor(PreStartScreen *self)` (0x232328):
/// vtable 0x47d938, zero `+4` and `+8`, return `self`.
pub fn pre_start_screen_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x26c8;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 7;
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    this.sw(c, vtable);
    this.off(0x8).sw(c, 0);
    abi::ret(c);
}

/// The body after the early out: pick the player the pause menu should
/// select and store the ready mask (+0x224) and selection (+0x228).
pub(crate) fn pause_request_body(c: &mut Cpu) -> Result<(), Unwound> {
    // `cBE_getBE()` / `cBE_getInterface(be, 1)`.
    c.cycles += 10;
    c.g[S4][0] = 0;
    call_ptr(c, 0x0014_dc80, 0x0023_1884)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    call_ptr(c, 0x0014_dd58, 0x0023_1890)?;
    c.g[V1][0] = c.lw(at(c.g[S2][0], 0x28));
    c.g[S6][0] = c.g[V0][0];
    c.g[V0][0] = c.lw(at(c.g[V1][0], 0x608));
    let single = c.g[V0][0] != 0;
    c.g[S3][0] = 0xffff_ffff_ffff_ffff;
    if single {
        // One active player (`game+0x28`): mask him if not ready, else let
        // the confirm input select him.
        c.cycles += 8;
        c.g[S0][0] = c.lw(at(c.g[V1][0], 0x28));
        c.g[V0][0] = c.lw(gp(c, -0x848));
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.lw(at(c.g[V0][0], 0xb0));
        c.g[V0][0] = c.lw(at(c.g[A0][0], 0));
        c.g[A1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(at(c.g[V0][0], 0));
            c.g[A1][0] = (0 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        c.g[V0][0] = c.lw(gp(c, pause::NO_CONFIRM));
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[A1][0] = 0x3c;
                call_ptr(c, 0x0032_0c48, 0x0023_18f0)?;
                if c.g[V0][0] != 0 {
                    c.g[S3][0] = c.g[S0][0];
                }
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            c.g[S4][0] = sx((c.g[V0][0] as u32) << ((c.g[S0][0] as u32) & 31));
        }
    } else {
        // Several riders: mask every active one that is not ready.
        c.cycles += 4;
        c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
        c.g[V0][0] = c.lw(at(c.g[V1][0], 0x7c));
        let no_riders = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !no_riders {
            c.cycles += 2;
            c.g[S5][0] = 1;
            c.g[V0][0] = c.lw(at(c.g[S2][0], 0xc));
            loop {
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = c.lw(at(c.g[V0][0], 0x40));
                c.g[V1][0] = c.lw(at(c.g[A0][0], 0x18));
                c.g[V0][0] = c.lw(at(c.g[V1][0], 0x87c));
                let inactive = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S6][0];
                if !inactive {
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0014_7290, 0x0023_1934)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw(gp(c, -0x848));
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.lw(at(c.g[V0][0], 0xb0));
                    c.g[V0][0] = c.lw(at(c.g[A0][0], 0));
                    let not_ready = c.g[V0][0] == 0;
                    c.g[A1][0] = 0;
                    if !not_ready {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c.g[V0][0], 0));
                        c.g[A1][0] = c.g[S5][0];
                        if c.g[V0][0] == 0 {
                            c.g[A1][0] = 0;
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S5][0] as u32) << ((c.g[S0][0] as u32) & 31));
                    if c.g[A1][0] != 0 {
                        // Ready: select it unless one is selected already.
                        c.cycles += 2;
                        if (c.g[S3][0] as i64) >= 0 {
                            c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(gp(c, pause::NO_CONFIRM));
                            if c.g[V0][0] != 0 {
                                c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                            } else {
                                c.cycles += 3;
                                c.g[A1][0] = 0x3c;
                                call_ptr(c, 0x0032_0c48, 0x0023_198c)?;
                                if c.g[V0][0] != 0 {
                                    c.g[S3][0] = c.g[S0][0];
                                }
                                c.cycles += 1;
                                c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                            }
                        }
                    } else {
                        c.cycles += 2;
                        let bit = c.g[V0][0];
                        c.g[S4][0] |= bit;
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                    }
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                }
                c.cycles += 5;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.lw(at(c.g[V1][0], 0x7c));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.lw(at(c.g[S2][0], 0xc));
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_1910);
                    }
                    continue;
                }
                break;
            }
        }
    }

    // The ready mask and the selection.
    c.cycles += 2;
    let no_mask = c.g[S4][0] == 0;
    let s2 = c.g[S2][0];
    let mask = c.g[S4][0];
    c.sw(at(s2, 0x224), mask);
    if !no_mask {
        c.cycles += 2;
        if (c.g[S3][0] as i64) >= 0 {
            c.sw(at(s2, 0x228), c.g[S3][0]);
        } else {
            c.cycles += 5;
            c.g[V1][0] = c.lw(at(s2, 0xc));
            c.g[S0][0] = 0;
            c.g[V0][0] = c.lw(at(c.g[V1][0], 0x7c));
            let no_riders = (c.g[V0][0] as i64) <= 0;
            c.g[S3][0] = 0;
            if no_riders {
                c.cycles += 1;
                c.sw(at(s2, 0x228), c.g[S3][0]);
            } else {
                // Search for the first ready rider to select.
                c.cycles += 3;
                c.g[S1][0] = 1;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                loop {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw(at(c.g[V0][0], 0x40));
                    c.g[A0][0] = c.lw(at(c.g[V1][0], 0x18));
                    c.g[V0][0] = c.lw(at(c.g[A0][0], 0x87c));
                    let inactive = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S6][0];
                    if !inactive {
                        c.cycles += 10;
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x0014_7290, 0x0023_19f8)?;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw(gp(c, -0x848));
                        c.g[A0][0] = sx((c.g[A2][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = c.lw(at(c.g[V0][0], 0xb0));
                        c.g[V0][0] = c.lw(at(c.g[V1][0], 0));
                        let not_ready = c.g[V0][0] == 0;
                        c.g[A1][0] = 0;
                        if !not_ready {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(at(c.g[V0][0], 0));
                            c.g[A1][0] = c.g[S1][0];
                            if c.g[V0][0] == 0 {
                                c.g[A1][0] = 0;
                            }
                        }
                        c.cycles += 2;
                        if c.g[A1][0] != 0 {
                            c.g[S3][0] = c.g[A2][0];
                            c.cycles += 1;
                            c.sw(at(s2, 0x228), c.g[S3][0]);
                            break;
                        }
                    }
                    c.cycles += 6;
                    c.g[V1][0] = c.lw(at(s2, 0xc));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = c.lw(at(c.g[V1][0], 0x7c));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_19d8);
                        }
                        continue;
                    }
                    c.cycles += 2;
                    c.sw(at(s2, 0x228), c.g[S3][0]);
                    break;
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 1;
        return Ok(());
    }

    // No one is ready: only a pending restart lets the pause be left.
    c.cycles += 2;
    if (c.g[S3][0] as i64) < 0 {
        c.g[V0][0] = 0;
        return Ok(());
    }
    c.cycles += 9;
    call_ptr(c, 0x0014_dc80, 0x0023_1a5c)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 7;
    call_ptr(c, 0x0014_dd58, 0x0023_1a68)?;
    c.g[V1][0] = 0x0053_0000;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
    if c.g[V0][0] != 0 {
        c.sw(at(s2, 0x228), c.g[S3][0]);
        c.cycles += 1;
        c.g[V0][0] = 1;
        return Ok(());
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    Callee {
        addr: 0x0023_1ab8,
        f: ssx3_ee::s06::GameModule_CanRestartFromPause,
    }
    .call(c, 0x0023_1a80)?;
    let restart = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !restart {
        return Ok(());
    }
    c.sw(at(s2, 0x228), c.g[S3][0]);
    c.cycles += 1;
    c.g[V0][0] = 1;
    Ok(())
}

/// `int GameModule_CheckPauseRequest(GameModule *g)` (0x231840): decides
/// which player the pause menu selects. Returns 1 when the pause can be
/// confirmed (a ready player is selected, or a restart is permitted), 0
/// otherwise; the ready mask goes to `g+0x224` and the selection to `g+0x228`.
///
/// Frame 0x80: s2 at +0x50, s0 at +0x70, s1 at +0x60, s3 at +0x40, s4 at
/// +0x30, s5 at +0x20, s6 at +0x10, ra at 0.
pub fn game_module_check_pause_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = c.lw(gp(c, pause::LOADING));
        frame.save(c, S2, 0x50);
        frame.save(c, S0, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        let loading = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        let mut stopped = false;
        if loading {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c.g[V0][0], 8));
            stopped = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = 0;
        }
        if !stopped {
            pause_request_body(c)?;
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// GameModule_InvalidateFocus / GameStateResults_Exit / GameRand_Seed.

/// `void GameModule_InvalidateFocus(GameModule* m)` (0x230338): drop the
/// focused widget by setting its focus timer (`+0xd0`) to -1.
pub fn game_module_invalidate_focus(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xa4));
        let none = c.g[V1][0] == 0;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if !none {
            c.cycles += 1;
            Addr(r32(c, V1).wrapping_add(0xd0)).sw(c, c.g[V0][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Game states.

/// `void GameModule_AdvanceState(GameModule* self)` (0x231320): if the module
/// has a pending state at `+0x200`, ask it for the next state.
pub fn game_module_advance_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(arg(c, 0).wrapping_add(0x200)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 3;
            callee::GAME_STATE_GOTO_NEXT.call(c, 0x0023_133c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Game loading and results.

/// `int AppModule_TryExitWhenLoaded(AppModule* self)` (0x231C70): once the
/// loading progress global reaches 100%, ask the module to exit.
pub fn app_module_try_exit_when_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        let loader = c.lw_r_2(V0, gp_2(c, 0x2a74));
        frame.save_ra(c, 0);
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(1, 1);
        c.ld_f(0, (loader as u32).wrapping_add(0x38));
        c.c_eq_s(0, 1);
        let loaded = c.fcc();
        c.g[V0][0] = 0;
        if loaded {
            c.cycles += 2;
            callee::APP_MODULE_TRY_EXIT.call(c, 0x0023_1ca0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cGameModeMan_restartHeat(GameModeMan* gm)` (0x2382d8): restart the
/// current mode object (virtual slot 2) and clear the six mode-history words.
pub fn game_mode_man_reset_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0053_6668;
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, A0), 4)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
        c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x10)).lh(c);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x14)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
        call::call_ptr(c, target, 0x0023_830c)?;
        c.g[V0][0] = 0x0053_66d0;
        c.g[V1][0] = 5;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
        loop {
            c.cycles += 7;
            Addr(c.g[V0][0] as u32).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_8320);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `func_002305C8` (0x2305c8): `GameModule_EndReplayPlayback`. Stops the
/// replay (`0x26f980`), hides the replay UI (`0x2cc0b8(ui, 1)`), closes it
/// (`0x2cc460(ui)`), then pops one entry off the two replay stacks at
/// `D_005367`/`D_005368` (a counter at `gp-0x69c` and the saved slot at
/// `gp-0x6a0`). Signature: `void end_replay(GameModule* this)`.
pub fn game_module_end_replay_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x0023_05e0);
        c.g[A0][0] = lw_r(c, S0, 0x28);
        call_ptr(c, 0x0026_f980, 0x0023_05e0)?;
        c.g[A0][0] = lw_r(c, S0, 0xa4);
        c.g[RA][0] = sx(0x0023_05ec);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x002c_c0b8, 0x0023_05ec)?;
        c.g[RA][0] = sx(0x0023_05f4);
        c.g[A0][0] = lw_r(c, S0, 0xa4);
        call_ptr(c, 0x002c_c460, 0x0023_05f4)?;
        c.g[V0][0] = lw_r(c, GP, 0u32.wrapping_sub(0x69c));
        c.g[A0][0] = 0x53_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x66e8));
        c.g[A1][0] = 0x44_0000;
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(0x1));
        c.g[A1][0] = sx(low(c, A1).wrapping_add(0x28f0));
        c.g[V1][0] = sx(low(c, V0) << 2);
        sw_r(c, GP, 0u32.wrapping_sub(0x69c), c.g[V0][0]);
        c.g[V1][0] = sx(low(c, V1).wrapping_add(low(c, A0)));
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = Addr(low(c, V1)).lw(c);
        c.g[RA][0] = c.ld(low(c, SP));
        c.g[V0][0] = sx(low(c, V0) << 2);
        c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, A1)));
        c.g[V1][0] = Addr(low(c, V0)).lw(c);
        sw_r(c, GP, 0u32.wrapping_sub(0x6a0), c.g[V1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Game module reset.

/// `void GameModule_ResetScriptsAndWorld(GameModule* self)` (0x2300f0): tear
/// down the script/animation banks and reset the world map.
pub fn game_module_reset_scripts_and_world(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(gp(c, 0x2898));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p_2(r32(c, S1), 0x34)).lw(c);
        call_ptr(c, 0x0026_ca90, 0x0023_0110)?;
        c.g[A0][0] = Addr(p_2(r32(c, S1), 0x30)).lw(c);
        call_ptr(c, 0x0030_9030, 0x0023_0118)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_4c98, 0x0023_0124)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_5118, 0x0023_0130)?;
        c.g[A1][0] = 8;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0035_5118, 0x0023_013c)?;
        c.g[V0][0] = Addr(p_2(r32(c, S1), 0xc)).lw(c);
        c.g[A0][0] = Addr(p_2(r32(c, V0), 0xa4)).lw(c);
        call_ptr(c, 0x0010_3358, 0x0023_0148)?;
        call_ptr(c, 0x002d_9b40, 0x0023_0150)?;
        c.g[A0][0] = Addr(p_2(r32(c, S1), 0x10)).lw(c);
        call_ptr(c, 0x003a_6800, 0x0023_0158)?;
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        call_ptr(c, 0x0030_9f18, 0x0023_0160)?;
        c.g[A0][0] = Addr(p_2(r32(c, S1), 0x30)).lw(c);
        call_ptr(c, 0x0030_8c60, 0x0023_0168)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Game module and FE state entry points.

/// `void cGame_restart(GameModule* self)` (0x2302a8): reset the race and the
/// game mode, clear every rider's state word, then request the restart state
/// and re-enter the menu flow.
pub fn game_module_quick_restart(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        call_ptr(c, callee::GAME_MODULE_RESET_RACE, 0x0023_02bc)?;
        c.g[V0][0] = c.lw(gp_3(c, -0x848));
        c.g[A0][0] = c.lw(at_2(c, V0, 0xc0));
        call_ptr(c, callee::GAME_MODE_MAN_RESET_MODE, 0x0023_02c8)?;
        c.g[V1][0] = c.lw(at_2(c, S0, 0xc));
        c.g[V0][0] = c.lw(at_2(c, V1, 0x78));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[V1][0] = c.lw(at_2(c, S0, 0xc));
            c.g[A0][0] = sx(r32(c, A1) << 2);
            loop {
                c.cycles += 9;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.g[V0][0] = c.lw(at_2(c, V1, 0x28));
                c.sw(at_2(c, V0, 0x2f8), 0);
                c.g[V1][0] = c.lw(at_2(c, S0, 0xc));
                c.g[V0][0] = c.lw(at_2(c, V1, 0x78));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = sx(r32(c, A1) << 2);
                if event_due(c) {
                    deliver_event_here(c, 0x0023_02e0);
                }
            }
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 2;
        c.g[A3][0] = 1;
        call_ptr(c, callee::GAME_MODULE_REQUEST_STATE_2, 0x0023_0318)?;
        call_ptr(c, callee::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0023_0320)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::GAME_MODULE_START_NEXT_MODE, 0x0023_0328)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameModule_Enter(self)` (0x230050): tear down the front-end, reset
/// the managers and set the game-entered flags.
pub fn game_module_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[A0][0] = lw(c, V0, 0x8c);
        call_ptr(c, 0x0019_5d50, 0x0023_006c)?;
        c.g[A0][0] = lw(c, S0, 0x94);
        Addr(gp(c, -0x9e4)).sw(c, 0);
        call_ptr(c, 0x001f_30d8, 0x0023_0078)?;
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        call_ptr(c, 0x0027_8710, 0x0023_0080)?;
        c.g[A1][0] = Addr(gp(c, 0x1158)).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(4).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x88).lh(c);
        c.g[V1][0] = lw(c, V0, 0x8c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_0098)?;
        c.g[A1][0] = Addr(gp(c, 0x1158)).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(4).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
        c.g[V1][0] = lw(c, V0, 0x24);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_00b0)?;
        c.g[A0][0] = Addr(gp(c, -0x854)).lw(c);
        call_ptr(c, 0x0036_9ff8, 0x0023_00b8)?;
        c.g[A0][0] = lw(c, S0, 0x10);
        call_ptr(c, 0x003a_67e0, 0x0023_00c0)?;
        c.g[A0][0] = Addr(gp(c, 0x21e4)).lw(c);
        call_ptr(c, 0x002b_68d0, 0x0023_00c8)?;
        c.g[A0][0] = Addr(gp(c, 0x410)).lw(c);
        call_ptr(c, 0x0028_67e8, 0x0023_00d0)?;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(gp(c, 0x12d8)).sw(c, c.g[V0][0]);
        Addr(gp(c, 0x1234)).sw(c, c.g[V0][0]);
        Addr(gp(c, 0x14e8)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `int GameModule_TryExit(self)` (0x230e98): if the module's virtual "can
/// exit" lets it, shut the app module down and report success.
pub fn game_module_try_exit(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'end: {
            c.cycles += 12;
            frame.save(c, S0, 0x20);
            frame.save_ra(c, 0x10);
            c.g[S0][0] = c.g[A0][0];
            c.g[A1][0] = lw_2(c, at(c.g[S0][0], 0x98));
            c.g[V1][0] = lw_2(c, c.g[A1][0] as u32);
            c.g[A0][0] = lh(c, at(c.g[V1][0], 0x18));
            c.g[V0][0] = lw_2(c, at(c.g[V1][0], 0x1c));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0023_0ec0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_0ec0)?;
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if !t {
                'attempt: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0023_0ed0);
                    c.g[A0][0] = c.g[S0][0];
                    callee::APP_MODULE_TRY_EXIT_2.call(c, 0x0023_0ed0)?;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = lw_2(c, gp_4(c, -0x854));
                    if t {
                        break 'attempt;
                    }
                    c.cycles += 21;
                    c.g[V1][0] = lw_2(c, at(c.g[V0][0], 0x10d8));
                    c.g[A0][0] = lh(c, at(c.g[V1][0], 0x78));
                    Addr(frame.at(c, 0)).sw(c, 0);
                    Addr(frame.at(c, 4)).sw(c, 0);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Addr(frame.at(c, 8)).sw(c, 0);
                    c.g[V0][0] = lw_2(c, at(c.g[V1][0], 0x7c));
                    let target = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0023_0efc);
                    c.g[A1][0] = sx(frame.at(c, 0));
                    call_ptr(c, target, 0x0023_0efc)?;
                    c.g[V1][0] = lw_2(c, gp_4(c, -0x854));
                    c.g[V0][0] = 1;
                    c.g[A0][0] = lw_2(c, gp_4(c, -0x84c));
                    Addr(at(c.g[V1][0], 0x270)).sw(c, c.g[V0][0]);
                    Addr(gp_4(c, 0x1234)).sw(c, 0);
                    Addr(gp_4(c, 0x14e8)).sw(c, 0);
                    c.g[RA][0] = sx(0x0023_0f1c);
                    Addr(gp_4(c, 0x12d8)).sw(c, 0);
                    call_ptr(c, 0x0027_8718, 0x0023_0f1c)?;
                    c.g[RA][0] = sx(0x0023_0f24);
                    c.g[A0][0] = lw_2(c, at(c.g[S0][0], 0x94));
                    call_ptr(c, 0x001f_3170, 0x0023_0f24)?;
                    c.g[V0][0] = 1;
                    break 'end;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            break 'end;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void GameModule_OnSectionUnload(GameModule *self, int section)`
/// (0x230360): notify every section manager and release the module's crowd
/// and rail resources.
pub fn game_module_on_section_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0035_8700, 0x0023_0384)?;
        c.g[S0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2898));
        c.g[A0][0] = Addr(gp(c, -0x6f0)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0022_9408, 0x0023_0394)?;
        c.g[A0][0] = Addr(gp(c, 0xf00)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_3c08, 0x0023_03a0)?;
        c.g[A0][0] = Addr(gp(c, 0xf38)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0035_7b90, 0x0023_03ac)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_4c98, 0x0023_03b8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0035_51a8, 0x0023_03c8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 8;
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0035_51a8, 0x0023_03d8)?;
        c.g[A0][0] = Addr(p_3(c.g[S2][0], 0xc)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(p_3(c.g[A0][0], 0xa4)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 7;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0010_3308, 0x0023_03f8)?;
                c.g[V0][0] = Addr(p_3(c.g[S2][0], 0xc)).lw(c);
                c.g[A1][0] = 0x80;
                c.g[A0][0] = Addr(p_3(c.g[V0][0], 0xa4)).lw(c);
                call_ptr(c, 0x0010_3308, 0x0023_0408)?;
            }
        }
        c.g[A0][0] = Addr(p_3(c.g[S2][0], 0x30)).lw(c);
        c.cycles += 8;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0030_8fe0, 0x0023_0414)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModifierBlock_Save (0x353448)
// ---------------------------------------------------------------------------

/// `this = obj + vt[off].delta; v[func_reg] = vt[off].func; call` (the
/// game's virtual-call sequence: `lh a0`, `lw func`, `addu a0`, `jalr`).
pub(crate) fn vcall(
    c: &mut Cpu,
    obj: u64,
    vt: u64,
    off: u32,
    func_reg: usize,
    ret: u32,
) -> crate::call::Flow {
    let slot = VSlot::read(c, vt, off);
    c.g[func_reg][0] = slot.func;
    c.g[A0][0] = slot.this(obj);
    call_ptr(c, slot.func as u32, ret)
}

// ---------------------------------------------------------------------------
// AppModule_RenderFade (0x231d60)
// ---------------------------------------------------------------------------

/// The 2D render context (`*(gp - 0x854)`): its render-state stack pointer
/// (0x14-byte entries) and its vtable (+0x10d8).
pub(crate) const R2D_CTX_GP: u32 = 0x854;

pub(crate) const R2D_STATE_TOP: u32 = 0xe84;

pub(crate) const R2D_VT: u32 = 0x10d8;

/// Context slots: `SetViewport(0, -1, -1, 1, 1, 0, 1, 0)` and
/// `SetTexture(tex)`.
pub(crate) const VT_SET_VIEWPORT: u32 = 0xd0;

pub(crate) const VT_SET_TEXTURE: u32 = 0x110;

/// The default render state and the "no texture" object.
pub(crate) const DEFAULT_STATE: u32 = 0x0050_1420;

pub(crate) const NO_TEXTURE: u32 = 0x004f_f1a0;

/// The fade timer (seconds left).
pub(crate) const FADE_TIME: u32 = 0x4;

/// `bool AppModule_RenderFade(AppModule* m)` (0x231d60): while the module's
/// fade timer runs, push a copy of the 2D render state, reset it to the
/// default with a full-screen viewport, no texture and alpha blending, draw
/// a full-screen black rectangle with alpha `min(5 t, 1)` and pop the
/// state. Returns whether the rectangle was fully opaque.
pub fn app_module_render_fade(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x40);
        frame.save_fpr(c, 21, 0x58);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 23, 0x68);
        frame.save_fpr(c, 22, 0x60);
        frame.save_fpr(c, 20, 0x50);
        let module = r32(c, S0);
        let gp = c.g[GP][0] as u32;
        c.mtc1(0, 21);
        c.lwc1(0, module + FADE_TIME);
        c.c_lt_s(21, 0);
        let fading = c.fcc();
        c.g[A0][0] = Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c);
        if !fading {
            c.cycles += 1;
            c.g[V0][0] = 0;
            return epilogue(c, frame);
        }

        // Push the render state (copy the top 0x14-byte entry up).
        c.cycles += 88;
        c.g[T0][0] = 0x50_0000;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 20);
        c.g[A1][0] = 0;
        let ctx = r32(c, A0);
        c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
        let top = r32(c, V0);
        c.mov_s(16, 21);
        c.g[AT][0] = sx(0xbf80_0000);
        c.mtc1(AT, 22);
        c.mov_s(14, 20);
        c.ldl(V1, top + 0x7);
        c.ldr(V1, top);
        c.ldl(A2, top + 0xf);
        c.ldr(A2, top + 0x8);
        c.g[A3][0] = Addr(top).off(0x10).lw(c);
        c.sdl(V1, top + 0x1b);
        c.sdr(V1, top + 0x14);
        c.sdl(A2, top + 0x23);
        c.sdr(A2, top + 0x1c);
        let w = c.g[A3][0];
        Addr(top).off(0x24).sw(c, w);
        c.mov_s(15, 20);
        c.g[A2][0] = 0;
        c.mov_s(13, 22);
        c.g[V1][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
        c.mov_s(17, 20);
        c.g[A3][0] = Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c);
        c.mov_s(12, 22);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x14));
        c.mov_s(23, 21);
        let new_top = c.g[V1][0];
        Addr(ctx).off(R2D_STATE_TOP).sw(c, new_top);

        // *top = the default state.
        let ctx = r32(c, A3);
        c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
        let top = r32(c, V0);
        c.g[T1][0] = sx(DEFAULT_STATE);
        c.ldl(V1, DEFAULT_STATE + 0x7);
        c.ldr(V1, DEFAULT_STATE);
        c.ldl(A0, DEFAULT_STATE + 0xf);
        c.ldr(A0, DEFAULT_STATE + 0x8);
        c.g[A3][0] = Addr(DEFAULT_STATE).off(0x10).lw(c);
        c.sdl(V1, top + 0x7);
        c.sdr(V1, top);
        c.sdl(A0, top + 0xf);
        c.sdr(A0, top + 0x8);
        let w = c.g[A3][0];
        Addr(top).off(0x10).sw(c, w);

        // ctx->SetViewport(0, -1, -1, 1, 1, 0, 1, 0); ctx->SetTexture(none).
        c.g[A3][0] = Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c);
        let obj = c.g[A3][0];
        c.g[V0][0] = Addr(obj as u32).off(R2D_VT).lw(c);
        let vt = c.g[V0][0];
        vcall(c, obj, vt, VT_SET_VIEWPORT, V1, 0x0023_1e54)?;
        let gp = c.g[GP][0] as u32;
        c.g[A2][0] = Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c);
        c.g[A1][0] = sx(NO_TEXTURE);
        let obj = c.g[A2][0];
        c.g[V0][0] = Addr(obj as u32).off(R2D_VT).lw(c);
        let vt = c.g[V0][0];
        vcall(c, obj, vt, VT_SET_TEXTURE, V1, 0x0023_1e74)?;

        // Blend bits of the new state: alpha blend on, test/depth bits off.
        let gp = c.g[GP][0] as u32;
        c.g[A3][0] = Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c);
        let ctx = r32(c, A3);
        c.g[V1][0] = 0xffff_ffff_fe7f_ffff;
        c.g[T1][0] = 0x100_0000;
        c.g[T0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
        c.g[A1][0] = 0xffff_ffff_ffcf_ffff;
        c.g[A2][0] = 0xffff_ffff_fff0_0fff;
        let top = r32(c, T0);
        c.g[V0][0] = Addr(top).off(0x8).lw(c);
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(AT, 1);
        c.g[V0][0] |= 0x3e0;
        let v = c.g[V0][0];
        Addr(top).off(0x8).sw(c, v);
        for (reg, mask, or) in [(A0, V1, true), (V1, A1, false), (A0, A2, false)] {
            c.g[reg][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
            let top = r32(c, reg);
            c.g[V0][0] = Addr(top).off(0x4).lw(c);
            c.g[V0][0] &= c.g[mask][0];
            if or {
                c.g[V0][0] |= c.g[T1][0];
            }
            let v = c.g[V0][0];
            Addr(top).off(0x4).sw(c, v);
        }

        // alpha = min(5 t, 1) (0 when negative).
        c.lwc1(0, module + FADE_TIME);
        c.mul_s(0, 0, 1);
        c.c_le_s(21, 0);
        if c.fcc() {
            c.min_s(23, 0, 20);
        }

        // Render2D_DrawFilledRect(r2d, {-1,-1}, {1,1}, {alpha,0,0,0}, 0).
        c.cycles += 21;
        let gp = c.g[GP][0] as u32;
        let sp0 = sp(c);
        c.g[A0][0] = Addr(gp.wrapping_sub(0x844)).lw(c);
        c.mov_s(12, 21);
        c.swc1(22, sp0 + 0x4);
        c.g[A1][0] = sx(sp0);
        c.swc1(22, sp0);
        c.g[A2][0] = sx(sp0 + 0x10);
        c.swc1(20, sp0 + 0x10);
        c.g[A3][0] = sx(sp0 + 0x20);
        c.swc1(20, sp0 + 0x14);
        c.swc1(23, sp0 + 0x20);
        c.swc1(21, sp0 + 0x24);
        c.swc1(21, sp0 + 0x28);
        c.swc1(21, sp0 + 0x2c);
        call_ptr(c, 0x0038_ec40, 0x0023_1f30)?;

        // Pop the state.
        let gp = c.g[GP][0] as u32;
        c.g[V1][0] = Addr(gp.wrapping_sub(R2D_CTX_GP)).lw(c);
        let ctx = r32(c, V1);
        c.c_le_s(20, 23);
        c.g[A0][0] = 1;
        c.g[V0][0] = Addr(ctx).off(R2D_STATE_TOP).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
        let opaque = c.fcc();
        let top = c.g[V0][0];
        Addr(ctx).off(R2D_STATE_TOP).sw(c, top);
        if !opaque {
            c.cycles += 1;
            c.g[A0][0] = 0;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        epilogue(c, frame)
    });

    fn epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
        c.cycles += 8;
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 23, 0x68);
        frame.restore_fpr(c, 22, 0x60);
        frame.restore_fpr(c, 21, 0x58);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 0x00231348 GameModule_SaveState

/// The game module's state machine members saved by
/// [`game_module_save_state`]: (offset in the module, the state's
/// `SaveState`, return address of the call).
pub(crate) const GAME_MODULE_STATES: [(u32, Callee, u32); 15] = [
    (
        0xb0,
        Callee {
            addr: 0x0023_4990,
            f: ee::s06::GameStatePreRaceLoad_SaveState,
        },
        0x0023_1410,
    ),
    (
        0xd0,
        Callee {
            addr: 0x0023_6d60,
            f: ee::s06::GameStatePreRaceMenu_SaveState,
        },
        0x0023_141c,
    ),
    (
        0xe0,
        Callee {
            addr: 0x0023_4de0,
            f: ee::s06::GameStateCountdown_SaveState,
        },
        0x0023_1428,
    ),
    (
        0xf0,
        Callee {
            addr: 0x0023_4eb8,
            f: ee::s06::GameStateRace_SaveState,
        },
        0x0023_1434,
    ),
    (
        0x100,
        Callee {
            addr: 0x0023_40b8,
            f: ee::s06::GameStateFinish_SaveState,
        },
        0x0023_1440,
    ),
    (
        0x118,
        Callee {
            addr: 0x0023_3b90,
            f: ee::s06::GameStateQuit_SaveState,
        },
        0x0023_144c,
    ),
    (
        0x128,
        Callee {
            addr: 0x0023_6ec8,
            f: ee::s06::GameStateResults_SaveState,
        },
        0x0023_1458,
    ),
    (
        0x138,
        Callee {
            addr: 0x0023_3bd0,
            f: ee::s06::GameStateIdle_SaveState,
        },
        0x0023_1464,
    ),
    (
        0x148,
        Callee {
            addr: 0x0023_6b70,
            f: ee::s06::GameStateResetAI_SaveState,
        },
        0x0023_1470,
    ),
    (
        0x158,
        Callee {
            addr: 0x0023_5990,
            f: ee::s06::GameStateReset_SaveState,
        },
        0x0023_147c,
    ),
    (
        0x178,
        Callee {
            addr: 0x0023_6af8,
            f: ee::s06::GameStateEventSwitchLoad_SaveState,
        },
        0x0023_1488,
    ),
    (
        0x18c,
        Callee {
            addr: 0x0023_6fa8,
            f: ee::s06::GameStateFinishCinematic_SaveState,
        },
        0x0023_1494,
    ),
    (
        0x19c,
        Callee {
            addr: 0x0023_5f48,
            f: ee::s06::GameStateRestartRace_SaveState,
        },
        0x0023_14a0,
    ),
    (
        0x1b8,
        Callee {
            addr: 0x0023_6728,
            f: ee::s06::GameStateEventSelect_SaveState,
        },
        0x0023_14ac,
    ),
    (
        0x1dc,
        Callee {
            addr: 0x0023_61c8,
            f: ee::s06::GameStateWarpToEvent_SaveState,
        },
        0x0023_14b8,
    ),
];

/// `void GameModule_SaveState(GameModule* self, cStream* stream)`
/// (0x231348): serialize the module into a save stream: the two serializable
/// members at `+0x70`/`+0x74` (virtual `serialize`, vtable slot 0x60), the
/// four words at `+0x214`, `+0x218`, `+0x210`, `+0x21c` (`stream->write(p,
/// 4)`, slot 8), then every game state's `SaveState`.
pub fn game_module_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 97;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        // The serializable members: obj->serialize(stream) (a1 is the stream).
        for (member, ret) in [(0x70u32, 0x0023_1378u32), (0x74, 0x0023_1394)] {
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(member));
            if member == 0x74 {
                c.g[A1][0] = c.g[S0][0];
            }
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x60));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
            let method = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, ret)?;
        }
        // stream->write(&self->field, 4) for the four words.
        for (i, (field, ret)) in [
            (0x214u32, 0x0023_13b0u32),
            (0x218, 0x0023_13cc),
            (0x210, 0x0023_13e8),
            (0x21c, 0x0023_1404),
        ]
        .into_iter()
        .enumerate()
        {
            let vt = c.lw(c.g[S0][0] as u32);
            if i == 0 {
                c.g[A3][0] = vt;
            } else {
                c.g[V1][0] = vt;
            }
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(field));
            c.g[A2][0] = 0x4;
            c.g[A0][0] = c.lh((vt as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((vt as u32).wrapping_add(0xc));
            let method = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, ret)?;
        }
        // Each game state: state->SaveState(stream).
        for (member, save, ret) in GAME_MODULE_STATES {
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(member));
            c.g[A1][0] = c.g[S0][0];
            save.call(c, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod game_module {

    /// The loader (`GameViewMan_UpdateSplitScreen` argument).
    pub const LOADER: u32 = 0x84;

    /// The previous state object.
    pub const PREV: u32 = 0x1fc;

    /// The current state object (null for none).
    pub const CUR: u32 = 0x200;

    /// The pending (entered, still updating) state object.
    pub const PENDING: u32 = 0x204;

    /// Id (1..16) of the state to enter as pending; 0 for none.
    pub const PENDING_ID: u32 = 0x208;

    /// Id of the last entered pending state.
    pub const ENTERED_ID: u32 = 0x20c;

    /// Id (1..16) of the state to switch to; 0 for none.
    pub const NEXT_ID: u32 = 0x210;

    pub const CUR_ID: u32 = 0x214;

    pub const PREV_ID: u32 = 0x218;

    /// Argument for the next state's start.
    pub const NEXT_ARG: u32 = 0x21c;

    /// Non-zero holds the module while the global run flag is clear.
    pub const HOLD: u32 = 0x224;

    /// The state objects for ids 1..16 (each with its vtable at `+0xc`).
    pub const STATES: [u32; 16] = [
        0xb0, 0xd0, 0xe0, 0xf0, 0x100, 0x118, 0x128, 0x138, 0x148, 0x158, 0x178, 0x18c, 0x19c,
        0x1b8, 0x1dc, 0x1ec,
    ];

    /// State vtable slots.
    pub const VT_ENTER: u32 = 0x10;

    pub const VT_UPDATE: u32 = 0x18;

    pub const VT_START: u32 = 0x28;

    pub const VT_TICK: u32 = 0x30;

    pub const VT_EXIT: u32 = 0x38;

    /// The two `switch` jump tables (pending id, next id).
    pub const PENDING_TABLE: u32 = 0x0047_b7b0;

    pub const NEXT_TABLE: u32 = 0x0047_b7f0;

    /// The case labels of the two tables.
    pub const PENDING_CASE0: u32 = 0x0023_0ffc;

    pub const NEXT_CASE0: u32 = 0x0023_1124;

    pub const START: u32 = 0x0023_0f40;

    pub const END: u32 = 0x0023_124c;
}

/// `bool AppModule_TryExit(AppModule *self)` (0x231cf0): mark the flag at `+8`
/// and report whether the fade value at `+4` has reached the threshold at
/// `gp_m2-0x51b4` (the FPU condition is left set either way).
pub fn app_module_try_exit(c: &mut Cpu) {
    const FADE: u32 = 0x4;
    const FLAG: u32 = 0x8;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 7;
    let threshold = Addr((c.g[GP][0] as u32).wrapping_sub(0x51b4)).read_fpu(c); // f1
    c.f[1] = threshold.0;
    c.g[V0][0] = 1;
    let current = this.off(FADE).read_fpu(c); // f0
    c.f[0] = current.0;
    let done = threshold.le(current);
    abi::set_fcc(c, done);
    this.off(FLAG).sw(c, c.g[V0][0]);
    if !done {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `GameModule_CanRestartFromPause` (0x231ab8): true when the game module is
/// not loading and its state is not one of the pause-menu transition states.
pub fn game_module_can_restart_from_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p_m2(this, 0x210)).lw(c);
        c.g[V0][0] = 5;
        let zero = c.g[V1][0] == 0;
        let mut skip_check = false;
        if zero {
            c.cycles += 1;
            c.g[A0][0] = Addr(gp_m2(c, -0x84c)).lw(c);
            skip_check = true;
        } else {
            c.cycles += 2;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
                frame.restore(c, S1, 0x10);
                frame.restore(c, S0, 0x20);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 0xe;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
                frame.restore(c, S1, 0x10);
                frame.restore(c, S0, 0x20);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
                frame.restore(c, S1, 0x10);
                frame.restore(c, S0, 0x20);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 0xa;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
                frame.restore(c, S1, 0x10);
                frame.restore(c, S0, 0x20);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            c.g[A0][0] = Addr(gp_m2(c, -0x84c)).lw(c);
        }
        let _ = skip_check;
        c.cycles += 4;
        c.g[RA][0] = sx(0x0023_1b0c);
        c.g[S0][0] = 0;
        call_ptr(c, callee::GAME_STATE_IS_LOADING, 0x0023_1b0c)?;
        let loaded = c.g[V0][0] == 0;
        c.g[V0][0] = 5;
        if loaded {
            c.g[S0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(p_m2(this, 0x214)).lw(c);
            let five = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xe;
            if !five {
                c.cycles += 2;
                let fourteen = c.g[A0][0] == c.g[V0][0];
                c.g[V1][0] = 1;
                if !fourteen {
                    c.cycles += 2;
                    let one = c.g[A0][0] == c.g[V1][0];
                    c.g[V0][0] = c.g[A0][0] ^ 0xa;
                    if !one {
                        c.cycles += 2;
                        c.g[S0][0] = c.g[V1][0];
                        if c.g[V0][0] == 0 {
                            c.g[S0][0] = 0;
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `state->vt[slot](state)` (state vtable at `+0xc`), returning to `ret`.
pub(crate) fn game_state_call(c: &mut Cpu, obj: usize, slot: u32, ret: u32) -> crate::call::Flow {
    let state = r32(c, obj);
    c.g[V0][0] = c.lw(bw::p(state, 0xc));
    c.g[A0][0] = c.lh(bw::p(r32(c, V0), slot));
    c.g[V1][0] = c.lw(bw::p(r32(c, V0), slot + 4));
    let method = r32(c, V1);
    c.g[A0][0] = sx(state.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

/// `void GameModule_ProcessStateChange(GameModule* m)` (0x230f40): the
/// game's state machine step. Unless held (the global run flag at
/// 0x534b30 clear with `hold` set, or flag 0x20 at gp_m2-0x6a0): tick the
/// current state (not while state 3 waits for no switch); enter a pending
/// state (`enter`) and keep updating it; then, on a requested switch, exit
/// the current state, remember it as previous, select the next (ids 1..16),
/// drop the pending state if it is the one now current, refresh the split
/// screen and start the new state with the request's argument.
pub fn game_module_process_state_change(c: &mut Cpu) {
    use game_module::*;
    if c.pc != START {
        let pc = c.pc;
        c.jump_out(pc, START, END);
        return;
    }
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw(bw::gp(c, -0x848));
        frame.save(c, S0, 0x30);
        c.g[A1][0] = 7;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let m = r32(c, S0);
        c.g[A0][0] = c.lw(bw::p(r32(c, V0), 0x78));
        call_ptr(c, 0x0014_dd58, 0x0023_0f68)?; // BELibrary_GetInterface(be, 7)
        'out: {
            c.g[V1][0] = 0x0053_0000;
            c.g[V0][0] = c.lw(0x0053_4b30);
            let running = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(bw::gp(c, -0x6a0));
            if !running {
                c.cycles += 3;
                c.g[V0][0] = c.lw(bw::p(m, HOLD));
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'out;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(bw::gp(c, -0x6a0));
            }
            c.cycles += 3;
            c.g[V0][0] &= 0x20;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            // Tick the current state.
            c.cycles += 3;
            c.g[A1][0] = c.lw(bw::p(m, CUR));
            c.g[V0][0] = 3;
            'ticked: {
                if c.g[A1][0] != 0 {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(bw::p(m, CUR_ID));
                    if c.g[V1][0] == c.g[V0][0] {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(bw::p(m, NEXT_ID));
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = c.lw(bw::p(m, PENDING_ID));
                            break 'ticked;
                        }
                        c.cycles += 1;
                    }
                    c.g[V0][0] = c.lw(bw::p(r32(c, A1), 0xc));
                    c.cycles += 4;
                    c.g[A0][0] = c.lh(bw::p(r32(c, V0), VT_TICK));
                    c.g[V1][0] = c.lw(bw::p(r32(c, V0), VT_TICK + 4));
                    let method = r32(c, V1);
                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    call_ptr(c, method, 0x0023_0fcc)?;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(bw::p(m, PENDING_ID));
            }
            // Enter the pending state.
            c.cycles += 2;
            let none = c.g[V0][0] == 0;
            c.g[V1][0] = sx(r32(c, V0).wrapping_sub(1));
            if !none {
                c.cycles += 3;
                let in_range = c.g[V1][0] < 0x10;
                c.g[V0][0] = in_range as u64;
                c.g[V0][0] = 0x0048_0000;
                if in_range {
                    c.cycles += 6;
                    c.g[V1][0] = sx(r32(c, V1) << 2);
                    c.g[V0][0] = sx(PENDING_TABLE);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(PENDING_TABLE));
                    c.g[A0][0] = c.lw(r32(c, V1));
                    let target = r32(c, A0);
                    let k = target.wrapping_sub(PENDING_CASE0) / 8;
                    if target.wrapping_sub(PENDING_CASE0) % 8 != 0 || k >= 16 {
                        c.jump_out(target, START, END);
                        return Ok(());
                    }
                    c.cycles += if k == 15 { 1 } else { 2 };
                    c.g[V0][0] = sx(m.wrapping_add(STATES[k as usize]));
                    c.cycles += 1;
                    c.sw(bw::p(m, PENDING), c.g[V0][0]);
                }
                c.cycles += 9;
                c.g[V0][0] = c.lw(bw::p(m, PENDING_ID));
                c.sw(bw::p(m, PENDING_ID), 0);
                c.sw(bw::p(m, ENTERED_ID), c.g[V0][0]);
                c.g[A1][0] = c.lw(bw::p(m, PENDING));
                game_state_call(c, A1, VT_ENTER, 0x0023_10a0)?;
            }
            // Update the pending state.
            c.cycles += 3;
            c.g[A1][0] = c.lw(bw::p(m, PENDING));
            if c.g[A1][0] != 0 {
                c.cycles += 6;
                game_state_call(c, A1, VT_UPDATE, 0x0023_10c0)?;
            }
            c.g[S1][0] = c.lw(bw::p(m, NEXT_ID));
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            // Switch: exit the current state.
            c.cycles += 3;
            c.g[A1][0] = c.lw(bw::p(m, CUR));
            c.g[S2][0] = c.lw(bw::p(m, NEXT_ARG));
            if c.g[A1][0] != 0 {
                c.cycles += 5;
                game_state_call(c, A1, VT_EXIT, 0x0023_10ec)?;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lw(bw::p(m, CUR_ID));
            c.g[A1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[V1][0] = c.lw(bw::p(m, CUR));
            let in_range = c.g[A1][0] < 0x10;
            c.g[A0][0] = in_range as u64;
            c.sw(bw::p(m, PREV_ID), c.g[V0][0]);
            c.sw(bw::p(m, PREV), c.g[V1][0]);
            if in_range {
                c.cycles += 7;
                c.g[V0][0] = sx(NEXT_TABLE);
                c.g[V1][0] = sx(r32(c, A1) << 2);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(NEXT_TABLE));
                c.g[A0][0] = c.lw(r32(c, V1));
                let target = r32(c, A0);
                let k = target.wrapping_sub(NEXT_CASE0) / 0xc;
                if target.wrapping_sub(NEXT_CASE0) % 0xc != 0 || k >= 16 {
                    c.jump_out(target, START, END);
                    return Ok(());
                }
                c.cycles += if k == 15 { 2 } else { 3 };
                c.g[V0][0] = sx(m.wrapping_add(STATES[k as usize]));
                c.g[V1][0] = k as u64 + 1;
                c.cycles += 2;
                c.sw(bw::p(m, CUR), c.g[V0][0]);
                c.sw(bw::p(m, CUR_ID), c.g[V1][0]);
            }
            c.cycles += 5;
            c.g[V1][0] = c.lw(bw::p(m, ENTERED_ID));
            c.g[V0][0] = c.lw(bw::p(m, CUR_ID));
            c.sw(bw::p(m, NEXT_ID), 0);
            let differ = c.g[V1][0] != c.g[V0][0];
            c.sw(bw::p(m, NEXT_ARG), 0);
            if !differ {
                // The pending state is the new current one.
                c.cycles += 2;
                c.sw(bw::p(m, ENTERED_ID), 0);
                c.sw(bw::p(m, PENDING), 0);
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(bw::p(m, CUR));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            // Start the new state.
            c.cycles += 10;
            c.g[A0][0] = c.lw(bw::p(m, LOADER));
            call_ptr(c, 0x0022_e550, 0x0023_1218)?; // GameViewMan_UpdateSplitScreen
            c.g[A2][0] = c.lw(bw::p(m, CUR));
            c.g[A1][0] = c.g[S2][0];
            game_state_call(c, A2, VT_START, 0x0023_1234)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

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
pub(crate) const GMU_GP_FOG: i32 = 0x2a74;

// D_004A5B64
pub(crate) const GMU_GP_SKY_FOG_MIN: i32 = -0x51bc;

// 0.9f
pub(crate) const GMU_GP_GAME_COMM: i32 = -0x204;

// D_004A2EEC
pub(crate) const GMU_GP_SCRIPT_ENGINE: i32 = -0x84c;

// D_004A28A4
pub(crate) const GMU_GP_WORLD: i32 = -0x848;

// D_004A28A8
pub(crate) const GMU_GP_AUDIO_HOST: i32 = -0x854;

// D_004A289C
pub(crate) const GMU_GP_PAD_QUEUE: i32 = -0x850;

// D_004A28A0
pub(crate) const GMU_GP_NET_CONN: i32 = -0x238;

// D_004A2EB8
pub(crate) const GMU_GP_STATE_DEPTH: i32 = -0x69c;

// D_004A2A54
pub(crate) const GMU_GP_STATE_FLAGS: i32 = -0x6a0;

// D_004A2A50
pub(crate) const GMU_GP_PENDING_OVERLAY: i32 = -0x1024;

// D_004A20CC
pub(crate) const GMU_GP_PROCESS_LIST: i32 = 0x2898;

// D_004A5988
pub(crate) const GMU_GP_WSCRIPT: i32 = 0xce8;

// D_004A3DD8
/// The state stack (`D_005366E8`) and the per-state flag words
/// (`D_004428F0`).
pub(crate) const GMU_STATE_STACK: u64 = 0x0053_66e8;

pub(crate) const GMU_STATE_FLAGS_2: u32 = 0x0044_28f8;

pub(crate) const GMU_STATE_FLAGS_3: u32 = 0x0044_28fc;

pub(crate) const GMU_STATE_FLAGS_12: u32 = 0x0044_2920;

/// The UI state name hashed for the mission event (`D_0047B798`).
pub(crate) const GMU_MISSION_STATE_NAME: u64 = 0x0047_b798;

pub(crate) const GMU_GAME_COMM_UPDATE: Callee = Callee {
    addr: 0x0025_70d8,
    f: ee::s06::cGameComm_Update,
};

pub(crate) const GMU_UPDATE_FADE: Callee = Callee {
    addr: 0x0023_1d18,
    f: ee::s06::AppModule_UpdateFade,
};

pub(crate) const GMU_PROCESS_STATE_CHANGE: Callee = Callee {
    addr: 0x0023_0f40,
    f: ee::s06::GameModule_ProcessStateChange,
};

pub(crate) const GMU_INPUT_SYNC_UPDATE: Callee = Callee {
    addr: 0x0023_73b8,
    f: ee::s06::InputSync_Update,
};

pub(crate) const GMU_INPUT_SYNC_READY: Callee = Callee {
    addr: 0x0023_7950,
    f: ee::s06::InputSync_IsReady,
};

pub(crate) const GMU_INPUT_SYNC_APPLY: Callee = Callee {
    addr: 0x0023_79c8,
    f: ee::s06::InputSync_ApplyInput,
};

pub(crate) const GMU_INPUT_SYNC_HOOK: Callee = Callee {
    addr: 0x0023_7948,
    f: ee::s06::InputSync_EmptyHook,
};

pub(crate) const GMU_CHECK_PAUSE: Callee = Callee {
    addr: 0x0023_1840,
    f: ee::s06::GameModule_CheckPauseRequest,
};

#[inline]
pub(crate) fn gmu_gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// The address of a field of the module (`s1`).
#[inline]
pub(crate) fn gmu_field(c: &Cpu, off: u32) -> u32 {
    r32(c, S1).wrapping_add(off)
}

/// `a0 = &D_004A5988; a1 = list; ProcessList_UpdatePriority` (and the merge /
/// post-update forms): the register setup of the process-list calls.
#[inline]
pub(crate) fn gmu_process_list(c: &mut Cpu, target: u32, list: u64, ret: u32) -> Flow {
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
pub(crate) fn gmu_frame(c: &mut Cpu) -> Flow {
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
pub(crate) fn gmu_hold(c: &mut Cpu) -> Flow {
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
pub(crate) fn gmu_resume(c: &mut Cpu) -> Flow {
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
pub(crate) fn gmu_update_state(c: &mut Cpu) -> Flow {
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
pub(crate) fn gmu_event_overlay(c: &mut Cpu) -> Flow {
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
pub(crate) fn gmu_mission_event(c: &mut Cpu) -> Flow {
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
pub(crate) fn gmu_pending_overlay(c: &mut Cpu) -> Flow {
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
pub(crate) fn gmu_open_pause(c: &mut Cpu) -> Flow {
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
pub(crate) fn gmu_update_world(c: &mut Cpu) -> Flow {
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
pub(crate) fn gmu_simulate(c: &mut Cpu) -> Flow {
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

// ---------------------------------------------------------------------------
// LoadIndicator_Render (0x00231fc0)

/// `void LoadIndicator::Render(this)` (0x231fc0): draw the pulsing sprite of the loading indicator at the bottom-right of the 640x480 screen: save a render state, begin the 2D scene, set the sprite state and viewport, size the sprite with a sine of the time, draw it and restore.
pub fn load_indicator_render(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x90 };
        'b_232180: {
            'b_232164: {
                c.cycles += 101;
                Frame::open(c, 0x90);
                c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                frame.save(c, S0, 0x70);
                frame.save(c, S1, 0x60);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x50);
                frame.save_ra(c, 0x40);
                frame.save_fpr(c, 21, 0x88);
                frame.save_fpr(c, 20, 0x80);
                c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0xe84));
                c.ldl(3, r32(c, V0).wrapping_add(0x7));
                c.ldr(3, r32(c, V0));
                c.ldl(4, r32(c, V0).wrapping_add(0xf));
                c.ldr(4, r32(c, V0).wrapping_add(0x8));
                c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(0x10));
                c.sdl(3, r32(c, V0).wrapping_add(0x1b));
                c.sdr(3, r32(c, V0).wrapping_add(0x14));
                c.sdl(4, r32(c, V0).wrapping_add(0x23));
                c.sdr(4, r32(c, V0).wrapping_add(0x1c));
                c.sw(r32(c, V0).wrapping_add(0x24), c.g[A2][0]);
                c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0xe84));
                c.g[A2][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x14));
                c.sw(r32(c, A1).wrapping_add(0xe84), c.g[V1][0]);
                c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0xa8));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xac));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0023_2034)?;
                c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0xf8));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xfc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0023_204c)?;
                c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                c.g[A0][0] = 0x500000;
                c.g[A3][0] = 0xfffffffffe7f0000;
                c.g[T3][0] = 0x1000000;
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xe84));
                c.g[A3][0] = c.g[A3][0] | 0xffff;
                c.g[T0][0] = 0xffffffffffcf0000;
                c.g[T2][0] = 0xfffffffffff00000;
                c.g[A2][0] = sx(r32(c, A0).wrapping_add(0x1420));
                c.ldl(9, r32(c, A2).wrapping_add(0x7));
                c.ldr(9, r32(c, A2));
                c.ldl(2, r32(c, A2).wrapping_add(0xf));
                c.ldr(2, r32(c, A2).wrapping_add(0x8));
                c.g[A1][0] = c.lw(r32(c, A2).wrapping_add(0x10));
                c.sdl(9, r32(c, V1).wrapping_add(0x7));
                c.sdr(9, r32(c, V1));
                c.sdl(2, r32(c, V1).wrapping_add(0xf));
                c.sdr(2, r32(c, V1).wrapping_add(0x8));
                c.sw(r32(c, V1).wrapping_add(0x10), c.g[A1][0]);
                c.g[T0][0] = c.g[T0][0] | 0xffff;
                c.g[T2][0] = c.g[T2][0] | 0xfff;
                c.g[T1][0] = 0x10000;
                c.g[A2][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                c.g[T1][0] = c.g[T1][0] | 0x4000;
                c.g[T4][0] = 0xffffffffffffff83;
                c.g[A1][0] = 0x500000;
                c.g[A0][0] = c.lw(r32(c, A2).wrapping_add(0xe84));
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0xe60));
                c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
                c.sw(r32(c, A0).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0xe84));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.sw(r32(c, V1).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw(r32(c, A2).wrapping_add(0xe84));
                c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                c.sw(r32(c, A0).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0xe84));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                c.sw(r32(c, V1).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw(r32(c, A2).wrapping_add(0xe84));
                c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] | 0x3e0;
                c.sw(r32(c, A0).wrapping_add(0x8), c.g[V0][0]);
                c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x110));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x114));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0023_2128)?;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x10d8));
                c.g[V0][0] = c.lh(r32(c, V1).wrapping_add(0xb8));
                c.g[S2][0] = sx(r32(c, V1).wrapping_add(0xbc));
                c.g[A1][0] = c.lh(r32(c, V1).wrapping_add(0x50));
                c.g[S1][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x54));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                call_ptr(c, t, 0x0023_214c)?;
                let t = (c.g[V0][0] as i64) < 0;
                c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                if t {
                    break 'b_232164;
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 20);
                    c.cvt_s_w(20, 20);
                    let t = true;
                    c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x10d8));
                    if t {
                        break 'b_232180;
                    } else {
                        break 'b_232164;
                    }
                }
            }
            c.cycles += 7;
            c.g[V1][0] = sx(r32(c, V0) >> 1);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 20);
            c.cvt_s_w(20, 20);
            c.add_s(20, 20, 20);
            c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x10d8));
            break 'b_232180;
        }
        'b_2321bc: {
            'b_2321a8: {
                c.cycles += 6;
                c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x58));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x5c));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0023_2190)?;
                let t = (c.g[V0][0] as i64) < 0;
                c.g[V1][0] = sx(r32(c, V0) >> 1);
                if t {
                    break 'b_2321a8;
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 15);
                    c.cvt_s_w(15, 15);
                    let t = true;

                    if t {
                        break 'b_2321bc;
                    } else {
                        break 'b_2321a8;
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 15);
            c.cvt_s_w(15, 15);
            c.add_s(15, 15, 15);
            break 'b_2321bc;
        }
        c.cycles += 90;
        c.mtc1(0, 12);
        c.mov_s(14, 20);
        c.g[V0][0] = c.lw(r32(c, S2));
        c.g[A0][0] = c.g[S1][0];
        c.mov_s(20, 12);
        c.mov_s(13, 12);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 21);
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = 0x1;
        call_ptr(c, t, 0x0023_21e4)?;
        c.g[A3][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        c.mov_s(16, 20);
        c.g[AT][0] = 0x4420_0000;
        c.mtc1(1, 14);
        c.mov_s(17, 21);
        c.g[V1][0] = c.lw(r32(c, A3).wrapping_add(0x10d8));
        c.mov_s(13, 20);
        c.g[AT][0] = 0x43f0_0000;
        c.mtc1(1, 15);
        c.mov_s(12, 20);
        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0xd0));
        c.g[A1][0] = 0x1;
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xd4));
        c.g[A2][0] = 0u64;
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
        call_ptr(c, t, 0x0023_2224)?;
        c.lwc1(0, r32(c, S0));
        c.lwc1(12, r32(c, GP).wrapping_sub(0x51a4));
        c.mul_s(12, 0, 12);
        call_ptr(c, 0x0031_c040, 0x0023_2234)?;
        c.add_s(0, 0, 21);
        c.g[AT][0] = 0x4110_0000;
        c.mtc1(1, 1);
        c.g[AT][0] = 0x4190_0000;
        c.mtc1(1, 3);
        c.g[T1][0] = sx(r32(c, S0).wrapping_add(0xc));
        c.lwc1(2, r32(c, GP).wrapping_sub(0x51a0));
        c.g[A1][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.mul_s(0, 0, 1);
        frame.save_fpr(c, 20, 0x18);
        c.lwc1(1, r32(c, GP).wrapping_sub(0x519c));
        c.g[A3][0] = sx(r32(c, SP).wrapping_add(0x20));
        frame.save_fpr(c, 21, 0x1c);
        c.g[T0][0] = sx(r32(c, SP).wrapping_add(0x30));
        frame.save_fpr(c, 3, 0x4);
        c.g[A2][0] = c.g[SP][0];
        frame.save_fpr(c, 0, 0);
        frame.save_fpr(c, 2, 0x10);
        frame.save_fpr(c, 1, 0x14);
        c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        c.g[T2][0] = c.lw(r32(c, V1).wrapping_add(0xa0));
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0xe84));
        c.sh(r32(c, V0).wrapping_add(0x10), c.g[T2][0]);
        frame.save_fpr(c, 20, 0x24);
        c.g[T2][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        c.lwc1(0, r32(c, S0).wrapping_add(0x4));
        frame.save_fpr(c, 21, 0x34);
        c.swc1(0, r32(c, S0).wrapping_add(0xc));
        frame.save_fpr(c, 20, 0x20);
        frame.save_fpr(c, 21, 0x30);
        c.g[V0][0] = c.lw(r32(c, T2).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x270));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x274));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(r32(c, T2).wrapping_add(r32(c, A0)));
        call_ptr(c, t, 0x0023_22c4)?;
        c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x100));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x104));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, t, 0x0023_22dc)?;
        c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0xb0));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xb4));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, t, 0x0023_22f4)?;
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        frame.restore(c, S0, 0x70);
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0xe84));
        frame.restore(c, S1, 0x60);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        c.sw(r32(c, A0).wrapping_add(0xe84), c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void AppModule_Update(AppModule* self)` (0x231cb0): forwards to
/// `AppModule_UpdateFade`.
pub fn app_module_update(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x0023_1d18,
            f: ee::s06::f_00231d18,
        }),
        0x0023_1cc0,
    );
}

// ---- GameModule_Load (0x22ebc8) --------------------------------------------

/// Entry address of `cGame_load`.
pub(crate) const GAME_LOAD_ENTRY: u32 = 0x0022_ebc8;

// Callees (all reached through the function table unless noted).
pub(crate) const LOAD_PROGRESS_BEGIN: u32 = 0x0031_7530;

pub(crate) const LOAD_PROGRESS_SET: u32 = 0x0031_7550;

pub(crate) const OPERATOR_NEW: u32 = 0x0031_7d70;

pub(crate) const BE_GET_BE: u32 = 0x0014_dc80;

pub(crate) const BE_GET_INTERFACE: u32 = 0x0014_dd58;

// `cGame` fields (the object is `s1` throughout).
pub(crate) const GAME_WORLD: u32 = 0x10;

pub(crate) const GAME_AI: u32 = 0x0c;

pub(crate) const GAME_BX_NASH: u32 = 0x20;

pub(crate) const GAME_DEBUG_FONT: u32 = 0x24;

pub(crate) const GAME_REPLAY: u32 = 0x28;

pub(crate) const GAME_FX_LOADER: u32 = 0x2c;

pub(crate) const GAME_WSCRIPT_MAN: u32 = 0x30;

pub(crate) const GAME_PLAYBACK_MAN: u32 = 0x34;

pub(crate) const GAME_CROWD_MAN_2D: u32 = 0x38;

pub(crate) const GAME_CROWD_MAN_3D: u32 = 0x3c;

pub(crate) const GAME_MULTI_PARTICLE_MAN: u32 = 0x40;

pub(crate) const GAME_UI: u32 = 0x48;

pub(crate) const GAME_AI_VFX_MAN: u32 = 0x50;

pub(crate) const GAME_VFX_MAN: u32 = 0x54;

pub(crate) const GAME_STATS_MAN: u32 = 0x58;

pub(crate) const GAME_TRANSITION_MAN: u32 = 0x5c;

pub(crate) const GAME_CUTSCENE_TRANSITION_MAN: u32 = 0x64;

pub(crate) const GAME_SPLASH_MAN: u32 = 0x68;

pub(crate) const GAME_FLAG_MANAGER: u32 = 0x70;

pub(crate) const GAME_SNOW_FALL: u32 = 0x74;

pub(crate) const GAME_INPUT_SYNC: u32 = 0x7c;

pub(crate) const GAME_VIEW_MAN: u32 = 0x84;

pub(crate) const GAME_INSTANCE_MAN: u32 = 0x88;

pub(crate) const GAME_WORLD_LIGHT_MAN: u32 = 0x90;

pub(crate) const GAME_OCCLUSION_MAN: u32 = 0x8c;

pub(crate) const GAME_LOAD_INDICATOR: u32 = 0x94;

pub(crate) const GAME_STREAM_PAUSE: u32 = 0x98;

pub(crate) const GAME_IS_STREAM_PAUSED: u32 = 0x9c;

pub(crate) const GAME_FLUSH_STREAMER: u32 = 0xa0;

pub(crate) const GAME_MENU_SYS: u32 = 0xa4;

pub(crate) const GAME_TRACK_BIG_FILE: u32 = 0xa8;

pub(crate) const GAME_UPDATE_TICK: u32 = 0xac;

pub(crate) const GAME_BOX_TEXTURE_ID: u32 = 0x220;

/// `r` as the 32-bit address it holds.
#[inline(always)]
pub(crate) fn gml_r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// `gp + off`.
#[inline(always)]
pub(crate) fn gml_gp(c: &Cpu, off: u32) -> u32 {
    gml_r(c, GP).wrapping_add(off)
}

/// `gp - off`.
#[inline(always)]
pub(crate) fn gml_gp_sub(c: &Cpu, off: u32) -> u32 {
    gml_r(c, GP).wrapping_sub(off)
}

/// `reg = sign_extend(value)`.
#[inline(always)]
pub(crate) fn gml_set(c: &mut Cpu, reg: usize, value: u32) {
    c.g[reg][0] = sx(value);
}

/// `operator new(size, name, flags, 0)` (`cMemMan_alloc`): result in `v0`.
pub(crate) fn gml_new(c: &mut Cpu, size: u32, name: u32, flags: u32, ret: u32) -> Flow {
    c.g[A0][0] = size as u64;
    gml_set(c, A1, name);
    c.g[A2][0] = flags as u64;
    c.g[A3][0] = 0;
    call_ptr(c, OPERATOR_NEW, ret)
}

/// `void cGame_load(cGame* self)` (0x22ebc8, "GameModule_Load"): builds every
/// manager of a race (stream pause, occlusion, world, lights, script, crowds,
/// particles, instances, the track and replay, the AI and playback, the view
/// manager, menus, transitions, flags, snow, audio, the UI engine with its HUD
/// state), then plays the replay set-up or loads the first state (10) and
/// waits for it. The progress bar is advanced after each group (13.2, 43.5,
/// 55, 93, 99). Returns nothing; `s1` is the game object throughout.
///
/// Entered at any other `pc` (a resume after a thread unwind), it runs the
/// generated body, which dispatches on `pc` itself.
pub fn game_module_load(c: &mut Cpu) {
    if c.pc != GAME_LOAD_ENTRY {
        return game_module_load_resume(c);
    }
    guest(c, game_module_load_body);
}

/// The generated body: every re-entry label of `cGame_load` lives there.
pub(crate) fn game_module_load_resume(c: &mut Cpu) {
    ee::s05::f_0022ebc8__generated(c)
}

pub(crate) fn game_module_load_body(c: &mut Cpu) -> Flow {
    // 0x22ebc8: frame, saves, first progress steps, locale, script engine.
    c.cycles += 52;
    let frame = Frame::open(c, 0xb0);
    c.g[V0][0] = c.lw(gml_gp(c, 0x2a74));
    frame.save(c, S1, 0x90);
    c.g[S1][0] = c.g[A0][0];
    c.lwc1(12, gml_gp_sub(c, 0x51c0));
    frame.save(c, S0, 0xa0);
    gml_set(c, A0, gml_r(c, V0).wrapping_add(0x38));
    frame.save(c, S2, 0x80);
    gml_set(c, S0, gml_r(c, S1).wrapping_add(GAME_BX_NASH));
    frame.save(c, S3, 0x70);
    gml_set(c, S2, gml_gp(c, 0x2898));
    frame.save(c, S5, 0x50);
    c.g[S3][0] = 0x530000;
    frame.save(c, S6, 0x40);
    gml_set(c, S5, gml_r(c, S1).wrapping_add(GAME_TRANSITION_MAN));
    frame.save(c, S7, 0x30);
    gml_set(c, S6, gml_r(c, S1).wrapping_add(GAME_SPLASH_MAN));
    frame.save(c, FP, 0x20);
    c.g[S7][0] = 0x480000;
    frame.save(c, S4, 0x60);
    frame.save_ra(c, 0x10);
    c.g[FP][0] = 0x480000;
    call_ptr(c, LOAD_PROGRESS_BEGIN, 0x0022_ec24)?; // (appman + 0x38, 13.2)
    c.g[A0][0] = c.lw(gml_gp(c, 0x2a74));
    c.g[AT][0] = 0x422e_0000;
    c.mtc1(1, 12);
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x38));
    call_ptr(c, LOAD_PROGRESS_SET, 0x0022_ec38)?; // 43.5
    c.g[V0][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[A0][0] = c.lw(gml_r(c, V0).wrapping_add(0x8c));
    call_ptr(c, 0x0019_5de0, 0x0022_ec44)?; // locale
                                            // Graphics manager virtual (slot at +0x160/+0x164 of its vtable).
    c.g[A2][0] = c.lw(gml_gp_sub(c, 0x854));
    c.g[A1][0] = 0;
    c.g[V0][0] = c.lw(gml_r(c, A2).wrapping_add(0x10d8));
    c.g[A0][0] = c.lh(gml_r(c, V0).wrapping_add(0x160));
    c.g[V1][0] = c.lw(gml_r(c, V0).wrapping_add(0x164));
    let method = gml_r(c, V1);
    gml_set(c, A0, gml_r(c, A2).wrapping_add(gml_r(c, A0)));
    call_ptr(c, method, 0x0022_ec60)?;
    c.g[A0][0] = c.lw(gml_gp_sub(c, 0x84c));
    c.sw(gml_r(c, S1).wrapping_add(GAME_IS_STREAM_PAUSED), 0);
    c.sw(gml_r(c, S1).wrapping_add(GAME_FLUSH_STREAMER), 0);
    call_ptr(c, 0x0027_8358, 0x0022_ec70)?; // cSSXScriptEngine_Load
                                            // The 168-byte grid ("Nash"): `new` result in v0.
    gml_new(c, 0xa8, 0x4a_0000u32.wrapping_add(0x2a58), 0, 0x0022_ec88)?;
    c.g[T1][0] = u64::MAX;
    c.g[V1][0] = c.g[V0][0];
    c.g[A0][0] = 1;

    // The grid constructor (inlined): three nested one-pass loops that clear
    // 0x14-byte entries. The state lives in registers; every back edge checks
    // for events (an unwind resumes at the loop head).
    let mut head = 0x0022_ec98u32;
    loop {
        match head {
            0x0022_ec98 => {
                c.cycles += 4;
                gml_set(c, T0, gml_r(c, A0).wrapping_sub(1));
                gml_set(c, A3, gml_r(c, V1).wrapping_add(0x50));
                c.g[A0][0] = 1;
                head = 0x0022_eca8;
            }
            0x0022_eca8 => {
                c.cycles += 4;
                gml_set(c, A2, gml_r(c, A0).wrapping_sub(1));
                gml_set(c, A1, gml_r(c, V1).wrapping_add(0x28));
                c.g[A0][0] = 1;
                head = 0x0022_ecb8;
            }
            0x0022_ecb8 => {
                c.cycles += 7;
                c.sw(gml_r(c, V1).wrapping_add(0x10), 0);
                gml_set(c, A0, gml_r(c, A0).wrapping_sub(1));
                let again = c.g[A0][0] != c.g[T1][0];
                gml_set(c, V1, gml_r(c, V1).wrapping_add(0x14));
                if again {
                    if back_edge_unwinds(c, 0x0022_ecb8) {
                        return Ok(());
                    }
                    head = 0x0022_ecb8;
                } else {
                    head = 0x0022_ecd4;
                }
            }
            0x0022_ecd4 => {
                c.cycles += 3;
                c.g[A0][0] = c.g[A2][0];
                let again = c.g[A0][0] != c.g[T1][0];
                c.g[V1][0] = c.g[A1][0];
                if again {
                    if back_edge_unwinds(c, 0x0022_eca8) {
                        return Ok(());
                    }
                    head = 0x0022_eca8;
                } else {
                    head = 0x0022_ece0;
                }
            }
            0x0022_ece0 => {
                c.cycles += 3;
                c.g[A0][0] = c.g[T0][0];
                let again = c.g[A0][0] != c.g[T1][0];
                c.g[V1][0] = c.g[A3][0];
                if again {
                    if back_edge_unwinds(c, 0x0022_ec98) {
                        return Ok(());
                    }
                    head = 0x0022_ec98;
                } else {
                    break;
                }
            }
            _ => unreachable!(),
        }
    }

    // 0x22ecec: register the grid, debug font, game comm.
    c.cycles += 21;
    c.sw(gml_r(c, S0), c.g[V0][0]); // self->BXNash = grid
    c.sw(gml_r(c, V0).wrapping_add(0xa0), 0);
    c.sw(gml_r(c, V0).wrapping_add(0xa4), 0);
    call_ptr(c, BE_GET_BE, 0x0022_ecfc)?;
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, BE_GET_INTERFACE, 0x0022_ed08)?; // interface 0
    gml_new(c, 0x84, 0x48_0000u32.wrapping_sub(0x4ab8), 0, 0x0022_ed20)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A0][0] = c.g[V0][0];
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4aa8));
    c.g[A2][0] = 0;
    call_ptr(c, 0x0039_15e8, 0x0022_ed34)?; // cFont_cFont (debug font)
    c.g[A0][0] = c.lw(gml_gp_sub(c, 0x204)); // cGameComm_mGameComm
    let no_game_comm = c.g[A0][0] == 0;
    c.sw(gml_r(c, S1).wrapping_add(GAME_DEBUG_FONT), c.g[V0][0]);
    if !no_game_comm {
        c.cycles += 2; // 0x22ed40
        call_ptr(c, 0x0025_8448, 0x0022_ed48)?; // cGameComm_syncGame
    }

    // 0x22ed48: stream, occlusion, world, lights, scripts, crowds, particles.
    c.cycles += 143;
    call_ptr(c, BE_GET_BE, 0x0022_ed50)?;
    c.g[A1][0] = 9;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, BE_GET_INTERFACE, 0x0022_ed5c)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_bd98, 0x0022_ed64)?;
    c.g[A0][0] = c.g[S2][0]; // the bucket manager (gp + 0x2898)
    c.g[A1][0] = 9;
    call_ptr(c, 0x0035_49e0, 0x0022_ed70)?; // cBucketMan_init
    c.g[A1][0] = 1;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0035_5260, 0x0022_ed7c)?; // cBucketMan_setsortflag
    c.g[A0][0] = 0x4c_0000;
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x5830));
    call_ptr(c, 0x0016_c918, 0x0022_ed88)?; // cCameraTriggerMan_init
    call_ptr(c, 0x0024_2d10, 0x0022_ed90)?; // cStreamPause_construct
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a90));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x114;
    c.sw(gml_r(c, S1).wrapping_add(GAME_STREAM_PAUSE), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_edac)?; // OcclusionMan
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0024_2500, 0x0022_edb4)?;
    c.g[A1][0] = 0x4a_0000;
    c.g[A2][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_add(0x2a60));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x18;
    c.sw(gml_r(c, S1).wrapping_add(GAME_OCCLUSION_MAN), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_edd0)?; // World
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x003a_6688, 0x0022_edd8)?; // cWorld_cWorld
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a80));
    c.g[A0][0] = 0x32a0;
    c.sw(gml_r(c, S1).wrapping_add(GAME_WORLD), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_edf4)?; // WorldLightMan
    c.g[A1][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_BX_NASH));
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x002f_5180, 0x0022_ee00)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a70));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x498;
    c.sw(gml_r(c, S1).wrapping_add(GAME_WORLD_LIGHT_MAN), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_ee1c)?; // WScriptMan
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0030_87d0, 0x0022_ee24)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a60));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x2720;
    c.sw(gml_r(c, S1).wrapping_add(GAME_WSCRIPT_MAN), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_ee40)?; // CrowdMan2d
    c.g[A0][0] = c.g[V0][0];
    CROWD_MAN_2D_CTOR.call(c, 0x0022_ee48)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a50));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x9c4;
    c.sw(gml_r(c, S1).wrapping_add(GAME_CROWD_MAN_2D), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_ee64)?; // CrowdMan3d
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0034_3a68, 0x0022_ee6c)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a40));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x10;
    c.sw(gml_r(c, S1).wrapping_add(GAME_CROWD_MAN_3D), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_ee88)?; // MultiParticleMan
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0035_79c8, 0x0022_ee90)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a28));
    c.g[A0][0] = 0x10;
    c.g[A2][0] = 0x2000_0000;
    c.sw(
        gml_r(c, S1).wrapping_add(GAME_MULTI_PARTICLE_MAN),
        c.g[V0][0],
    );
    call_ptr(c, OPERATOR_NEW, 0x0022_eeac)?; // InstancesMan
    c.g[A1][0] = 0x80;
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = 0x40;
    call_ptr(c, 0x0035_1030, 0x0022_eebc)?; // cWScriptCompile_parseKeywords
    c.g[A0][0] = 0x50_0000;
    c.sw(gml_r(c, S1).wrapping_add(GAME_INSTANCE_MAN), c.g[V0][0]);
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x47f8));
    call_ptr(c, 0x0038_acd0, 0x0022_eecc)?; // cIrradianceDataBase_Load
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x6580));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x10;
    call_ptr(c, OPERATOR_NEW, 0x0022_eee4)?; // LoadIndicator
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x001f_30c0, 0x0022_eeec)?;
    c.g[A1][0] = 0x4a_0000;
    c.g[A2][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_add(0x2a68));
    c.g[A3][0] = 0;
    c.g[A0][0] = 8;
    c.sw(gml_r(c, S1).wrapping_add(GAME_LOAD_INDICATOR), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_ef08)?; // BigFile
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0031_6898, 0x0022_ef10)?;
    c.sw(gml_r(c, S1).wrapping_add(GAME_TRACK_BIG_FILE), c.g[V0][0]);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0022_f6b0, 0x0022_ef1c)?; // cGame_loadTrack
    c.g[A1][0] = 0x4a_0000;
    c.g[A2][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_add(0x2a70));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x688;
    call_ptr(c, OPERATOR_NEW, 0x0022_ef34)?; // Replay
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0026_efb8, 0x0022_ef3c)?;
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a18));
    c.g[A0][0] = 1;
    c.sw(gml_r(c, S1).wrapping_add(GAME_REPLAY), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_ef58)?; // FXLoader
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x002d_bd10, 0x0022_ef64)?;
    c.sw(gml_r(c, S1).wrapping_add(GAME_FX_LOADER), c.g[V0][0]);
    c.g[A1][0] = 7;
    c.g[V0][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[A0][0] = c.lw(gml_r(c, V0).wrapping_add(0x78));
    call_ptr(c, BE_GET_INTERFACE, 0x0022_ef78)?; // network interface
    c.g[V0][0] = c.lw(gml_r(c, S3).wrapping_add(0x4b30)); // bIsNetworkGame
    let is_network = c.g[V0][0] != 0;
    gml_set(c, A0, gml_r(c, S3).wrapping_add(0x4b30));
    if is_network {
        // 0x22ef84: the input-sync queue length from the latency.
        c.cycles += 25;
        c.g[V0][0] = c.lw(gml_gp_sub(c, 0x204));
        c.g[A1][0] = 0x48_0000;
        c.g[V1][0] = c.lw(gml_gp_sub(c, 0x848));
        gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4a08));
        c.lwc1(1, gml_r(c, V0).wrapping_add(0x120));
        c.cvt_s_w(1, 1);
        c.lwc1(0, gml_r(c, A0).wrapping_add(0x10));
        c.lwc1(2, gml_r(c, V1).wrapping_add(0x14));
        c.g[A0][0] = 0x44;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.mul_s(1, 1, 2);
        c.add_s(0, 0, 1);
        c.div_s(0, 0, 2);
        c.cvt_w_s(1, 0);
        c.mfc1(16, 1);
        gml_set(c, S0, gml_r(c, S0).wrapping_add(2));
        call_ptr(c, OPERATOR_NEW, 0x0022_efcc)?; // InputSync
        c.g[V1][0] = c.lw(gml_gp_sub(c, 0x204));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.lw(gml_r(c, V1).wrapping_add(0x120));
        call_ptr(c, 0x0023_7060, 0x0022_efe0)?; // cInputSync_cInputSync
        c.sw(gml_r(c, S1).wrapping_add(GAME_INPUT_SYNC), c.g[V0][0]);
    } else {
        c.cycles += 4; // 0x22efe8
        call_ptr(c, 0x0031_77f0, 0x0022_eff0)?; // BXrand
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0023_7cb0, 0x0022_eff8)?; // GMsrand
    }

    // 0x22eff8: the AI, playback manager, replay init.
    c.cycles += 38;
    c.g[A1][0] = 0x4a_0000;
    c.g[A2][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_add(0x2a78));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0xd0;
    call_ptr(c, OPERATOR_NEW, 0x0022_f010)?; // AI
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0012_86a0, 0x0022_f018)?; // cAI_cAI
    c.sw(gml_r(c, S1).wrapping_add(GAME_AI), c.g[V0][0]);
    c.g[V1][0] = c.lw(gml_r(c, V0).wrapping_add(0xcc));
    c.g[A0][0] = c.lh(gml_r(c, V1).wrapping_add(0x10));
    c.g[A1][0] = c.lw(gml_r(c, V1).wrapping_add(0x14));
    let ai_method = gml_r(c, A1);
    gml_set(c, A0, gml_r(c, V0).wrapping_add(gml_r(c, A0)));
    call_ptr(c, ai_method, 0x0022_f030)?; // AI vtable slot 2
    c.g[A0][0] = c.lw(gml_gp(c, 0x2a74));
    c.g[AT][0] = 0x425c_0000;
    c.mtc1(1, 12);
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x38));
    call_ptr(c, LOAD_PROGRESS_SET, 0x0022_f044)?; // 55
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x49f8));
    c.g[A0][0] = 0x7c;
    call_ptr(c, OPERATOR_NEW, 0x0022_f05c)?; // PlaybackMan
    c.g[A1][0] = 0x28;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0026_c458, 0x0022_f068)?;
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_REPLAY));
    c.sw(gml_r(c, S1).wrapping_add(GAME_PLAYBACK_MAN), c.g[V0][0]);
    call_ptr(c, 0x0026_f180, 0x0022_f074)?; // cReplay_init
    c.g[V0][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[A1][0] = 5;
    c.g[A0][0] = c.lw(gml_r(c, V0).wrapping_add(0x78));
    call_ptr(c, BE_GET_INTERFACE, 0x0022_f084)?; // save interface
    c.g[S4][0] = c.lw(gml_gp_sub(c, 0x1ec8)); // cBESaveInterface_bDoReplay
    let replay = c.g[S4][0] != 0;
    c.g[S2][0] = c.g[V0][0];
    if replay {
        game_module_load_replay_setup(c)?;
    }

    // 0x22f184: the view manager (the player count, or 1 for network games).
    c.cycles += 3;
    c.g[V0][0] = c.lw(gml_r(c, S3).wrapping_add(0x4b30));
    let is_network = c.g[V0][0] != 0;
    gml_set(c, A1, gml_r(c, S7).wrapping_sub(0x49e8));
    if is_network {
        c.cycles += 7; // 0x22f190
        c.g[A0][0] = 0x38;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, OPERATOR_NEW, 0x0022_f1a0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
    } else {
        c.cycles += 7; // 0x22f1ac
        c.g[A0][0] = 0x38;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, OPERATOR_NEW, 0x0022_f1bc)?;
        c.g[V1][0] = 0x53_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(gml_r(c, V1).wrapping_add(0x5bfc)); // player count
    }
    c.cycles += 6; // 0x22f1c8
    GAME_VIEW_MAN_CTOR.call(c, 0x0022_f1d0)?;
    c.sw(gml_r(c, S1).wrapping_add(GAME_VIEW_MAN), c.g[V0][0]);
    c.g[V0][0] = c.lw(gml_r(c, S3).wrapping_add(0x4b30));
    let is_network = c.g[V0][0] != 0;
    gml_set(c, V1, gml_r(c, S3).wrapping_add(0x4b30));
    let mut chase_camera = false;
    if !is_network {
        c.cycles += 1; // 0x22f20c
        c.g[A1][0] = 0x4a_0000;
    } else {
        c.cycles += 3; // 0x22f1e0
        c.g[V0][0] = c.lw(gml_r(c, V1).wrapping_add(8)); // bIsHost
        let is_host = c.g[V0][0] != 0;
        c.g[A1][0] = 0x4a_0000;
        if !is_host {
            chase_camera = true;
        }
    }
    if chase_camera {
        // Joined client: aim the chase camera at the second player's rider.
        c.cycles += 4; // 0x22f1ec
        c.g[V0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
        c.g[V0][0] = c.lw(gml_r(c, V0).wrapping_add(0x44));
        let no_second_player = c.g[V0][0] == 0;
        c.g[A1][0] = 0;
        if !no_second_player {
            c.cycles += 1; // 0x22f1fc
            c.g[A1][0] = c.lw(gml_r(c, V0).wrapping_add(0x18));
        }
        c.cycles += 3; // 0x22f200
        c.g[V0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_VIEW_MAN));
        c.g[A0][0] = c.lw(gml_r(c, V0).wrapping_add(4));
        call_ptr(c, 0x0015_edc8, 0x0022_f20c)?;
        c.cycles += 1; // 0x22f20c
        c.g[A1][0] = 0x4a_0000;
    }

    // 0x22f210: menu system, menu graphics, tables.
    c.cycles += 36;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_add(0x2a80));
    c.g[A0][0] = 0x68;
    call_ptr(c, OPERATOR_NEW, 0x0022_f224)?; // MenuSys
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x002c_c018, 0x0022_f22c)?; // cMenuSys_cMenuSys
    c.g[V1][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_DEBUG_FONT));
    c.g[A0][0] = 0x48_0000;
    c.sw(gml_r(c, S1).wrapping_add(GAME_MENU_SYS), c.g[V0][0]);
    gml_set(c, A0, gml_r(c, A0).wrapping_sub(0x49d8));
    c.sw(gml_r(c, V0).wrapping_add(0x58), c.g[V1][0]);
    c.g[A1][0] = 0x100;
    c.g[V1][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[A2][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_MENU_SYS));
    c.g[V0][0] = c.lw(gml_r(c, V1).wrapping_add(0xb0));
    c.sw(gml_r(c, A2).wrapping_add(0x5c), c.g[V0][0]);
    call_ptr(c, 0x003e_18c8, 0x0022_f258)?; // FILE_load("menu.ssh")
    c.g[T2][0] = c.lw(gml_gp_sub(c, 0x854));
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = c.lw(gml_r(c, S0).wrapping_add(0x14));
    c.g[A2][0] = 0x4a_0000;
    c.g[V1][0] = c.lw(gml_r(c, T2).wrapping_add(0x10d8));
    gml_set(c, A2, gml_r(c, A2).wrapping_add(0x2a88));
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, S0).wrapping_add(gml_r(c, A1)));
    c.g[A0][0] = c.lh(gml_r(c, V1).wrapping_add(0x170));
    c.g[T0][0] = 1;
    c.g[V0][0] = c.lw(gml_r(c, V1).wrapping_add(0x174));
    c.g[T1][0] = u64::MAX;
    let load_texture = gml_r(c, V0);
    gml_set(c, A0, gml_r(c, T2).wrapping_add(gml_r(c, A0)));
    call_ptr(c, load_texture, 0x0022_f290)?; // graphics vslot: texture from .ssh
    c.g[V1][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_MENU_SYS));
    c.sw(gml_r(c, S1).wrapping_add(GAME_BOX_TEXTURE_ID), c.g[V0][0]);
    let no_file = c.g[S0][0] == 0;
    c.sw(gml_r(c, V1).wrapping_add(0x60), c.g[V0][0]);
    if !no_file {
        c.cycles += 2; // 0x22f2a0
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7e98, 0x0022_f2a8)?; // cMemMan_free
    }
    c.cycles += 8; // 0x22f2a8
    c.g[A0][0] = c.lw(gml_gp(c, 0x2a74));
    c.g[S3][0] = c.g[S6][0];
    c.g[AT][0] = 0x42ba_0000;
    c.mtc1(1, 12);
    c.g[S2][0] = c.g[S5][0];
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x38));
    c.g[S0][0] = 0;
    call_ptr(c, LOAD_PROGRESS_SET, 0x0022_f2c8)?; // 93

    // 0x22f2c8: two transition managers and two splash managers.
    loop {
        c.cycles += 26;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x84;
        gml_set(c, A1, gml_r(c, FP).wrapping_sub(0x49c0));
        c.g[A2][0] = 0;
        call_ptr(c, OPERATOR_NEW, 0x0022_f2dc)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, 0x002e_4228, 0x0022_f2ec)?; // cTransitionMan(3, i)
        c.sw(gml_r(c, S2), c.g[V0][0]);
        c.g[A0][0] = 0x1030;
        c.g[V0][0] = 0x48_0000;
        c.g[A2][0] = 0;
        gml_set(c, A1, gml_r(c, V0).wrapping_sub(0x49b0));
        c.g[A3][0] = 0;
        call_ptr(c, OPERATOR_NEW, 0x0022_f308)?;
        gml_set(c, S2, gml_r(c, S2).wrapping_add(4));
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        gml_set(c, S0, gml_r(c, S0).wrapping_add(1));
        call_ptr(c, 0x002f_3550, 0x0022_f320)?; // cSplashManager(2, i)
        c.sw(gml_r(c, S3), c.g[V0][0]);
        c.g[V0][0] = (c.g[S0][0] < 2) as u64;
        let again = c.g[V0][0] != 0;
        gml_set(c, S3, gml_r(c, S3).wrapping_add(4));
        if !again {
            break;
        }
        if back_edge_unwinds(c, 0x0022_f2c8) {
            return Ok(());
        }
    }

    // 0x22f330: cut-scene transitions, flags, snow, audio, the UI engine.
    c.cycles += 126;
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x49a0));
    c.g[A0][0] = 0x48;
    call_ptr(c, OPERATOR_NEW, 0x0022_f348)?;
    c.g[A1][0] = 3;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x002e_a670, 0x0022_f354)?; // cCutSceneTransitionMan
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4988));
    c.g[A0][0] = 0x1718;
    c.sw(
        gml_r(c, S1).wrapping_add(GAME_CUTSCENE_TRANSITION_MAN),
        c.g[V0][0],
    );
    call_ptr(c, OPERATOR_NEW, 0x0022_f370)?;
    c.g[A1][0] = 2;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0034_c428, 0x0022_f37c)?; // cFlagManager
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4978));
    c.g[A0][0] = 0x60;
    c.sw(gml_r(c, S1).wrapping_add(GAME_FLAG_MANAGER), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_f398)?;
    c.g[A1][0] = 2;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x002e_5920, 0x0022_f3a4)?; // cSnowFall
    c.g[A0][0] = c.lw(gml_gp(c, 0x410)); // gSSXAudio
    c.sw(gml_r(c, S1).wrapping_add(GAME_UPDATE_TICK), 0);
    c.sw(gml_r(c, S1).wrapping_add(GAME_SNOW_FALL), c.g[V0][0]);
    call_ptr(c, 0x0028_62a8, 0x0022_f3b4)?; // cSSXAudio_InGameLoad
    c.g[A0][0] = c.lw(gml_gp(c, 0x2a74));
    c.g[AT][0] = 0x42c6_0000;
    c.mtc1(1, 12);
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x38));
    call_ptr(c, LOAD_PROGRESS_SET, 0x0022_f3c8)?; // 99
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4968));
    c.g[A0][0] = 0x70;
    call_ptr(c, OPERATOR_NEW, 0x0022_f3e0)?; // UIEngine
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0039_7b08, 0x0022_f3e8)?;
    c.g[A0][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[V1][0] = 0x48_0000;
    c.sw(gml_r(c, S1).wrapping_add(GAME_UI), c.g[V0][0]);
    gml_set(c, A1, gml_r(c, V1).wrapping_sub(0x4958));
    c.g[V1][0] = c.lw(gml_r(c, A0).wrapping_add(0x8c)); // FELocale
    c.g[A0][0] = 0x48_0000;
    c.sw(gml_r(c, V0).wrapping_add(0x10), c.g[V1][0]);
    gml_set(c, V1, gml_r(c, A0).wrapping_sub(0x4958));
    // The 11-byte name D_0047B6A8 is copied to the stack (unaligned 8 + 3).
    c.ldl(6, gml_r(c, V1).wrapping_add(7));
    c.ldr(6, gml_r(c, V1));
    c.g[A3][0] = c.lb(gml_r(c, V1).wrapping_add(8));
    c.g[V0][0] = c.lb(gml_r(c, V1).wrapping_add(9));
    c.sdl(6, sp(c).wrapping_add(7));
    c.sdr(6, sp(c));
    c.sb(sp(c).wrapping_add(8), c.g[A3][0]);
    c.sb(sp(c).wrapping_add(9), c.g[V0][0]);
    c.g[A2][0] = c.lb(gml_r(c, V1).wrapping_add(0xa));
    c.sb(sp(c).wrapping_add(0xa), c.g[A2][0]);
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_UI));
    call_ptr(c, 0x0039_7c40, 0x0022_f438)?; // cUIEngine_loadFile
    c.g[A1][0] = 0x48_0000;
    c.g[A0][0] = 0x1c;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4948));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, OPERATOR_NEW, 0x0022_f450)?; // OVUIInput (data)
    c.g[V1][0] = 0x47_0000;
    c.g[T0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_UI));
    gml_set(c, V1, gml_r(c, V1).wrapping_sub(0x2e30));
    c.sb(gml_r(c, V0).wrapping_add(4), 0);
    c.sw(gml_r(c, V0).wrapping_add(8), c.g[V1][0]); // cFEInput vtable
    c.g[A1][0] = 0x48_0000;
    c.sw(gml_r(c, V0).wrapping_add(0x18), 0);
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4938));
    c.sw(gml_r(c, V0).wrapping_add(0xc), 0);
    c.g[A2][0] = 0;
    c.sw(gml_r(c, V0).wrapping_add(0x10), 0);
    c.g[A3][0] = 0;
    c.sw(gml_r(c, V0).wrapping_add(0x14), 0);
    c.g[A0][0] = 8;
    c.sw(gml_r(c, T0).wrapping_add(0xc), c.g[V0][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_f490)?; // OVUISound
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x001a_2f38, 0x0022_f498)?;
    c.g[V1][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_UI));
    c.g[A1][0] = 0x48_0000;
    c.g[T0][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[A3][0] = 0;
    c.sw(gml_r(c, V1).wrapping_add(0x14), c.g[V0][0]);
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4928));
    c.g[A0][0] = 0xc98;
    c.g[A2][0] = 0;
    c.g[V1][0] = c.lw(gml_r(c, T0).wrapping_add(0x88)); // Fonts
    c.g[V0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_UI));
    c.sw(gml_r(c, V0).wrapping_add(8), c.g[V1][0]);
    call_ptr(c, OPERATOR_NEW, 0x0022_f4c8)?; // cOVStateHUD1P
    c.g[A1][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_UI));
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = 0;
    call_ptr(c, 0x001e_9a30, 0x0022_f4d8)?;
    c.g[V1][0] = c.lw(gml_r(c, V0).wrapping_add(0x1c));
    c.g[A0][0] = 0xffff_ffff_ffff_c0ff;
    c.g[A1][0] = c.g[V0][0];
    c.g[V1][0] &= c.g[A0][0];
    c.g[V1][0] |= 0x500; // mode 5
    c.sw(gml_r(c, V0).wrapping_add(0x1c), c.g[V1][0]);
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_UI));
    gml_set(c, A0, gml_r(c, A0).wrapping_add(0x18));
    call_ptr(c, 0x0039_f238, 0x0022_f4fc)?; // cUIStateStack_pushExplicit
                                            // The state machine fields start cleared; `cGameState_mpGame = self`.
    for off in [
        0x204, 0x1fc, 0x200, 0x20c, 0x208, 0x218, 0x214, 0x210, 0x21c,
    ] {
        c.sw(gml_r(c, S1).wrapping_add(off), 0);
    }
    let no_replay = c.g[S4][0] == 0;
    c.sw(gml_gp_sub(c, 0x488), c.g[S1][0]);
    // 1: `cGame_setState(self, a1, 0, 0)` (0x22f598); 2: with a1 = 10 (0x22f5ac).
    let set_state: u32;
    if no_replay {
        c.cycles += 4; // 0x22f540: preload state 10 and wait for it
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x0023_1278, 0x0022_f54c)?; // cGame_setPreloadState
        loop {
            c.cycles += 8; // 0x22f550
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0023_0f40, 0x0022_f558)?; // cGame_updateState
            c.g[A0][0] = 0;
            call_ptr(c, 0x003e_5398, 0x0022_f560)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0023_12d8, 0x0022_f568)?; // cGame_isPreloadedStateReady
            let not_ready = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw(gml_gp_sub(c, 0x204));
            if !not_ready {
                break;
            }
            if back_edge_unwinds(c, 0x0022_f550) {
                return Ok(());
            }
        }
        c.cycles += 2; // 0x22f570
        let no_game_comm = c.g[A0][0] == 0;
        c.g[V0][0] = 0x53_0000;
        if !no_game_comm {
            c.cycles += 3; // 0x22f578
            call_ptr(c, 0x0025_86b0, 0x0022_f580)?; // cGameComm_syncWait
            c.g[V0][0] = 0x53_0000;
        }
        c.cycles += 4; // 0x22f584
        gml_set(c, V0, gml_r(c, V0).wrapping_add(0x5bc8));
        c.g[V1][0] = c.lb(gml_r(c, V0).wrapping_add(0x49));
        let flag = c.g[V1][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        if !flag {
            c.cycles += 1; // 0x22f594
            c.g[A1][0] = 0xa;
            set_state = 1;
        } else {
            set_state = 2;
        }
    } else {
        c.cycles += 6; // 0x22f528: replay: hand the replay its seed, start state 0x10
        c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_REPLAY));
        c.g[A1][0] = c.lw(gml_gp(c, 0x394));
        call_ptr(c, 0x0026_f898, 0x0022_f534)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x10;
        set_state = 1;
    }
    if set_state == 1 {
        c.cycles += 5; // 0x22f598
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0023_1250, 0x0022_f5a4)?; // cGame_setState
        c.g[A1][0] = 0x48_0000;
    } else {
        c.cycles += 5; // 0x22f5ac
        c.g[A1][0] = 0xa;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0023_1250, 0x0022_f5bc)?;
        c.g[A1][0] = 0x48_0000;
    }

    // 0x22f5c0: the three object nodes with their managers.
    c.cycles += 59;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4918));
    c.g[A0][0] = 0x14;
    call_ptr(c, OPERATOR_NEW, 0x0022_f5d4)?; // AIVisualEffectsMan node
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 6;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0035_4648, 0x0022_f5e4)?; // cObjNode_cObjNode1
    c.g[V1][0] = 0x48_0000;
    gml_set(c, V1, gml_r(c, V1).wrapping_sub(0x34a0));
    c.sw(gml_r(c, S0).wrapping_add(0xc), c.g[V1][0]);
    call_ptr(c, 0x002f_0390, 0x0022_f5f4)?; // cAIVisualEffectsMan_construct
    c.sw(gml_r(c, S1).wrapping_add(GAME_AI_VFX_MAN), c.g[S0][0]);
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x4900));
    c.sw(gml_r(c, S0).wrapping_add(0x10), c.g[V0][0]);
    c.g[A0][0] = 0x14;
    call_ptr(c, OPERATOR_NEW, 0x0022_f614)?; // VisualEffectsMan node
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 3;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0035_4648, 0x0022_f624)?;
    c.g[V1][0] = 0x48_0000;
    gml_set(c, V1, gml_r(c, V1).wrapping_sub(0x2f90));
    c.sw(gml_r(c, S0).wrapping_add(0xc), c.g[V1][0]);
    call_ptr(c, 0x002f_0510, 0x0022_f634)?; // cVisualEffectsMan_construct
    c.sw(gml_r(c, S1).wrapping_add(GAME_VFX_MAN), c.g[S0][0]);
    c.g[A1][0] = 0x48_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    gml_set(c, A1, gml_r(c, A1).wrapping_sub(0x48e8));
    c.sw(gml_r(c, S0).wrapping_add(0x10), c.g[V0][0]);
    c.g[A0][0] = 0x14;
    call_ptr(c, OPERATOR_NEW, 0x0022_f654)?; // SSXStatsMan node
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0035_4648, 0x0022_f664)?;
    c.g[V1][0] = 0x48_0000;
    gml_set(c, V1, gml_r(c, V1).wrapping_sub(0x3518));
    c.sw(gml_r(c, S0).wrapping_add(0xc), c.g[V1][0]);
    call_ptr(c, 0x002d_2100, 0x0022_f674)?; // SSXStatsMan construct
    c.sw(gml_r(c, S1).wrapping_add(GAME_STATS_MAN), c.g[S0][0]);
    c.sw(gml_r(c, S0).wrapping_add(0x10), c.g[V0][0]);
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    frame.restore(c, FP, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// The `cCrowdMan2d` and `cGameViewMan` constructors: the two
/// direct (same-shard) calls of `cGame_load`.
pub(crate) const CROWD_MAN_2D_CTOR: Callee = Callee {
    addr: 0x0022_9180,
    f: ee::s05::CrowdMan2d_ctor,
};

pub(crate) const GAME_VIEW_MAN_CTOR: Callee = Callee {
    addr: 0x0022_e3b8,
    f: ee::s05::GameViewMan_ctor,
};

/// The replay branch of `cGame_load` (0x22f090..0x22f184): initialise the AI
/// riders and the game mode, hand the AI the podium actor list from the
/// replay buffer, start the AI actors unless the mode/sub-mode pair says
/// otherwise, wait until the AI is loaded, then load the replay from the
/// buffer and release it.
pub(crate) fn game_module_load_replay_setup(c: &mut Cpu) -> Flow {
    c.cycles += 17; // 0x22f090
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
    call_ptr(c, 0x0012_8958, 0x0022_f098)?; // cAI_initAIRiders
    c.g[V0][0] = c.lw(gml_gp_sub(c, 0x848));
    c.g[S0][0] = 0x53_0000;
    gml_set(c, S0, gml_r(c, S0).wrapping_add(0x5bc8)); // cBENewRaceInterface_newRace
    c.g[A0][0] = c.lw(gml_r(c, V0).wrapping_add(0xc0));
    c.g[A1][0] = c.lb(gml_r(c, S0).wrapping_add(0x4a)); // sub mode
    call_ptr(c, 0x0023_8160, 0x0022_f0b0)?; // cGameModeMan_initGameMode
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0015_26d0, 0x0022_f0b8)?; // getGameReplayBuffer
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
    gml_set(c, A1, gml_r(c, V0).wrapping_add(0x20));
    call_ptr(c, 0x0012_bac0, 0x0022_f0c4)?; // cAI_setPodiumActorList
    c.g[V1][0] = c.lb(gml_r(c, S0).wrapping_add(0x48)); // mode
    c.g[V0][0] = 4;
    let mode_is_4 = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = c.lb(gml_r(c, S0).wrapping_add(0x4a));
    'chain: {
        if mode_is_4 {
            break 'chain;
        }
        c.cycles += 3; // 0x22f0d4
        c.g[A1][0] = 5;
        if c.g[V1][0] != c.g[A1][0] {
            // (branch-likely: the delay slot only runs when taken)
            c.g[V0][0] = 6;
        } else {
            c.cycles += 2; // 0x22f0e0: mode 5 needs sub mode 4
            let skip = c.g[A0][0] != c.g[V0][0];
            c.g[V0][0] = 6;
            if skip {
                break 'chain;
            }
        }
        c.cycles += 2; // 0x22f0e8
        if c.g[V1][0] != c.g[V0][0] {
            c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
        } else {
            c.cycles += 2; // 0x22f0f0: mode 6 needs sub mode 5
            let skip = c.g[A0][0] != c.g[A1][0];
            if skip {
                break 'chain;
            }
            c.cycles += 1; // 0x22f0f8
            c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
        }
        c.cycles += 3; // 0x22f0fc
        c.g[V0][0] = c.lw(gml_r(c, A0).wrapping_add(0x80)); // AI already has actors
        if c.g[V0][0] != 0 {
            break 'chain;
        }
        c.cycles += 2; // 0x22f108
        if c.g[V1][0] != 0 {
            c.cycles += 3; // 0x22f120
            c.g[A1][0] = 2;
            call_ptr(c, 0x0012_8998, 0x0022_f128)?; // cAI_initAIActors(ai, 2)
        } else {
            c.cycles += 4; // 0x22f110
            c.g[A1][0] = 0;
            call_ptr(c, 0x0012_8998, 0x0022_f118)?; // cAI_initAIActors(ai, 0)
        }
        c.g[V0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
        c.cycles += 2; // 0x22f12c
        c.sw(gml_r(c, V0).wrapping_add(0x8c), 0);
    }
    // 0x22f148: wait until the AI is loaded and ready, yielding a tick each time.
    loop {
        c.cycles += 4;
        c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
        call_ptr(c, 0x0012_a180, 0x0022_f150)?; // cAI_isAILoadedAndReady
        if c.g[V0][0] != 0 {
            break;
        }
        if back_edge_unwinds(c, 0x0022_f138) {
            return Err(Unwound);
        }
        c.cycles += 4; // 0x22f138
        c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_AI));
        call_ptr(c, 0x0012_96f8, 0x0022_f140)?; // cAI_loadRiders
        c.g[A0][0] = 1;
        call_ptr(c, 0x003e_52b0, 0x0022_f148)?; // THREAD_yieldticks(1)
    }
    c.cycles += 11; // 0x22f158
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_REPLAY));
    call_ptr(c, 0x0027_0ef8, 0x0022_f160)?; // cReplay_initLoad
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0015_26d0, 0x0022_f168)?;
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_REPLAY));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0027_1758, 0x0022_f174)?; // cReplay_load
    c.g[A0][0] = c.lw(gml_r(c, S1).wrapping_add(GAME_REPLAY));
    call_ptr(c, 0x0027_0f18, 0x0022_f17c)?; // cReplay_purgeLoad
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0015_2728, 0x0022_f184)?; // releaseReplayBuffer
    Ok(())
}

/// `void cGame_loadTrack(Game* self)` (0x22f6b0): load the race track's
/// world. Builds the world path `data/worlds/<location>` (the location name
/// from the race interface: `+0x44` of its record, or `+0x34` when
/// gp[-0x1f38] is zero), opens `<path>.big` in the game's big file, loads the
/// world (`World_Load`), then creates the sky box (0x18 bytes, +0x14), the
/// section manager (0x1d4, +0x78), the surface-property cache
/// (`SurfPropCache_InitDefaults`), the world sound (0x250, +0x4c) and the
/// forward-diff cache, loads the world's current section (event) and spins
/// `SectionMan_Update` / `World_Update` / `EAThread_Yield(0)` until it is
/// loaded, loads the camera triggers (`<path>/cam`), creates the fog
/// particle manager (0x3010 bytes) with its lens effect (+0x1c) and the view
/// queries.
///
/// The generated body is a `pc` dispatcher whose loop heads (0x22f890,
/// 0x22f978) are resume points: entered anywhere else, this runs the
/// generated body.
#[allow(unreachable_code)]
pub fn game_module_load_world(c: &mut Cpu) {
    if c.pc != 0x0022_f6b0 {
        return game_module_load_world_resume(c);
    }
    guest(c, |c| {
        c.cycles += 21;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
        c.g[S4][0] = c.g[A0][0];
        call_ptr(c, 0x0014dc80, 0x0022f6e4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0014dd58, 0x0022f6f0)?;
        c.g[S5][0] = c.g[V0][0];
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f38));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        let t = c.g[V1][0] == 0u64;
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        if t {
            c.cycles += 25;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.g[S5][0];
            c.g[S3][0] = 0x480000;
            call_ptr(c, 0x00144bc0, 0x0022f780)?;
            c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4b80));
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x003182d8, 0x0022f790)?;
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[S6][0] = c.g[S0][0];
            call_ptr(c, 0x003186d0, 0x0022f7a4)?;
            c.g[FP][0] = c.g[S3][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x00318448, 0x0022f7b4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x00318260, 0x0022f7c0)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x00318260, 0x0022f7cc)?;
            c.g[A2][0] = 0x4a0000;
        } else {
            c.cycles += 26;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.g[S5][0];
            c.g[S3][0] = 0x480000;
            call_ptr(c, 0x00144bc0, 0x0022f718)?;
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4b80));
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003182d8, 0x0022f728)?;
            c.g[S6][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[FP][0] = c.g[S3][0];
            call_ptr(c, 0x003186d0, 0x0022f740)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x00318448, 0x0022f74c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x00318260, 0x0022f758)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x00318260, 0x0022f764)?;
            c.g[A2][0] = 0x4a0000;
        }
        c.cycles += 48;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2a90));
        c.g[A0][0] = c.g[S6][0];
        call_ptr(c, 0x00318630, 0x0022f7e0)?;
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x44));
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 0u64;
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xa8));
        c.g[S7][0] = sx((c.g[S4][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x00316978, 0x0022f7f8)?;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0022f804)?;
        c.g[A1][0] = c.lw(c.g[SP][0] as u32);
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
        call_ptr(c, 0x003a6ae0, 0x0022f814)?;
        c.g[A1][0] = 0x4a0000;
        c.g[A2][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2a98));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0x18;
        call_ptr(c, 0x00317d70, 0x0022f82c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x00353ac0, 0x0022f834)?;
        c.g[A1][0] = 0x480000;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48d8));
        c.g[A0][0] = 0x1d4;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        call_ptr(c, 0x00317d70, 0x0022f850)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0022cce8, 0x0022f85c)?;
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x78), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48c8));
        c.g[A0][0] = 0xd10;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x0022f878)?;
        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[A1][0] = 0xffffffffffffffff;
        c.g[V1][0] = 0x12;
        // 19 passes of an empty counting loop (the compiler kept the loop).
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.g[V1][0] = 0x3;
            c.g[V1][0] = c.g[A0][0];
            let t = c.g[V1][0] != c.g[A1][0];
            if t {
                if back_edge_unwinds(c, 0x0022_f890) {
                    return Ok(());
                }
                continue;
            }
            break;
        }
        c.cycles += 50;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[S0][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x4b80));
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x44));
        call_ptr(c, 0x002c2f18, 0x0022f8bc)?;
        c.g[A1][0] = 0x480000;
        c.g[A3][0] = 0u64;
        c.g[A2][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48b8));
        c.g[A0][0] = 0x250;
        call_ptr(c, 0x00317d70, 0x0022f8d4)?;
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002b6aa0, 0x0022f8e0)?;
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
        call_ptr(c, 0x00334800, 0x0022f8ec)?;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        c.g[A0][0] = c.g[S5][0];
        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x3d4), 0u64);
        call_ptr(c, 0x00144bc0, 0x0022f908)?;
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003182d8, 0x0022f914)?;
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x003186d0, 0x0022f924)?;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x00318448, 0x0022f930)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0022f93c)?;
        c.g[A1][0] = 0x2;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00318260, 0x0022f948)?;
        c.g[A0][0] = c.g[S5][0];
        call_ptr(c, 0x00144bc0, 0x0022f950)?;
        c.g[S0][0] = c.lw(c.g[V0][0] as u32);
        c.g[A2][0] = 0u64;
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x78));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0022d088, 0x0022f964)?;
        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x78));
        c.g[V0][0] = 0x1;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1a4), c.g[V0][0]);
        // Wait for the section to load (the first pass skips the update).
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x78));
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0022d278, 0x0022f99c)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A3][0] = 0x1;
            if t {
                if back_edge_unwinds(c, 0x0022_f978) {
                    return Ok(());
                }
                c.cycles += 6;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x78));
                call_ptr(c, 0x0022d8d8, 0x0022f980)?;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x003a6928, 0x0022f988)?;
                c.g[A0][0] = 0u64;
                call_ptr(c, 0x003e5398, 0x0022f990)?;
                continue;
            }
            break;
        }
        c.cycles += 61;
        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 0x4a0000;
        c.g[A2][0] = c.g[S6][0];
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2aa0));
        c.g[A0][0] = c.g[S3][0];
        c.g[S0][0] = 0x4c0000;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x3d4), c.g[A3][0]);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5830));
        call_ptr(c, 0x003186d0, 0x0022f9cc)?;
        c.g[A2][0] = 0x4a0000;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2aa8));
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x00318630, 0x0022f9e0)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016ca10, 0x0022f9ec)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0022f9f8)?;
        c.g[A1][0] = 0x2;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x00318260, 0x0022fa04)?;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = 0x3010;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48a8));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x0022fa1c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002dbf80, 0x0022fa28)?;
        c.g[V0][0] = 0x490000;
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7d20));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V0][0] = 0xff;
        c.sw(c.g[S7][0] as u32, c.g[S0][0]);
        call_ptr(c, 0x002ec3e0, 0x0022fa48)?;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        call_ptr(c, 0x002ed048, 0x0022fa50)?;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0022fa5c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0022fa68)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        Ok(())
    })
}

/// Native resume of `game_module_load_world` (converted from the `generated` body): any entry
/// other than 0x0022f6b0, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn game_module_load_world_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0022f6b0 => {
                    c.cycles += 21;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
                    c.g[RA][0] = sx(0x0022f6e4);
                    c.g[S4][0] = c.g[A0][0];
                    call_ptr(c, 0x0014dc80, 0x0022f6e4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0022f6f0);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x0022f6f0)?;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f38));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
                    let t = c.g[V1][0] == 0u64;
                    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                    if t {
                        pc = 0x0022f76c;
                        continue;
                    }
                    pc = 0x0022f704;
                    continue;
                }
                0x0022f704 => {
                    c.cycles += 26;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x0022f718);
                    c.g[S3][0] = 0x480000;
                    call_ptr(c, 0x00144bc0, 0x0022f718)?;
                    c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4b80));
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x0022f728);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x003182d8, 0x0022f728)?;
                    c.g[S6][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0022f740);
                    c.g[FP][0] = c.g[S3][0];
                    call_ptr(c, 0x003186d0, 0x0022f740)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0022f74c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00318448, 0x0022f74c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0022f758);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0022f758)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0022f764);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0022f764)?;
                    let t = true;
                    c.g[A2][0] = 0x4a0000;
                    if t {
                        pc = 0x0022f7d0;
                        continue;
                    }
                    pc = 0x0022f76c;
                    continue;
                }
                0x0022f76c => {
                    c.cycles += 25;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x0022f780);
                    c.g[S3][0] = 0x480000;
                    call_ptr(c, 0x00144bc0, 0x0022f780)?;
                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4b80));
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[RA][0] = sx(0x0022f790);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x003182d8, 0x0022f790)?;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0022f7a4);
                    c.g[S6][0] = c.g[S0][0];
                    call_ptr(c, 0x003186d0, 0x0022f7a4)?;
                    c.g[FP][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0022f7b4);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00318448, 0x0022f7b4)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0022f7c0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0022f7c0)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0022f7cc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0022f7cc)?;
                    c.g[A2][0] = 0x4a0000;
                    pc = 0x0022f7d0;
                    continue;
                }
                0x0022f7d0 => {
                    c.cycles += 48;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2a90));
                    c.g[RA][0] = sx(0x0022f7e0);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x00318630, 0x0022f7e0)?;
                    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x44));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xa8));
                    c.g[RA][0] = sx(0x0022f7f8);
                    c.g[S7][0] = sx((c.g[S4][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x00316978, 0x0022f7f8)?;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0022f804);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0022f804)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0022f814);
                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
                    call_ptr(c, 0x003a6ae0, 0x0022f814)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2a98));
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x0022f82c);
                    c.g[A0][0] = 0x18;
                    call_ptr(c, 0x00317d70, 0x0022f82c)?;
                    c.g[RA][0] = sx(0x0022f834);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00353ac0, 0x0022f834)?;
                    c.g[A1][0] = 0x480000;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48d8));
                    c.g[A0][0] = 0x1d4;
                    c.g[RA][0] = sx(0x0022f850);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    call_ptr(c, 0x00317d70, 0x0022f850)?;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0022f85c);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0022cce8, 0x0022f85c)?;
                    c.g[A1][0] = 0x480000;
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48c8));
                    c.g[A0][0] = 0xd10;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0022f878);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x0022f878)?;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[A1][0] = 0xffffffffffffffff;
                    c.g[V1][0] = 0x12;
                    pc = 0x0022f890;
                    continue;
                }
                0x0022f890 => {
                    c.cycles += 7;
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = 0x3;
                    c.g[V1][0] = c.g[A0][0];
                    let t = c.g[V1][0] != c.g[A1][0];

                    if t {
                        if back_edge_unwinds(c, 0x0022f890) {
                            return Ok(());
                        }
                        pc = 0x0022f890;
                        continue;
                    }
                    pc = 0x0022f8ac;
                    continue;
                }
                0x0022f8ac => {
                    c.cycles += 50;
                    c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                    c.g[S0][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x4b80));
                    c.g[RA][0] = sx(0x0022f8bc);
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x44));
                    call_ptr(c, 0x002c2f18, 0x0022f8bc)?;
                    c.g[A1][0] = 0x480000;
                    c.g[A3][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48b8));
                    c.g[RA][0] = sx(0x0022f8d4);
                    c.g[A0][0] = 0x250;
                    call_ptr(c, 0x00317d70, 0x0022f8d4)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x0022f8e0);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x002b6aa0, 0x0022f8e0)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x0022f8ec);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                    call_ptr(c, 0x00334800, 0x0022f8ec)?;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
                    c.g[A0][0] = c.g[S5][0];
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[RA][0] = sx(0x0022f908);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x3d4), 0u64);
                    call_ptr(c, 0x00144bc0, 0x0022f908)?;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[RA][0] = sx(0x0022f914);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x003182d8, 0x0022f914)?;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0022f924);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x003186d0, 0x0022f924)?;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0022f930);
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x00318448, 0x0022f930)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0022f93c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0022f93c)?;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0022f948);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00318260, 0x0022f948)?;
                    c.g[RA][0] = sx(0x0022f950);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00144bc0, 0x0022f950)?;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x78));
                    c.g[RA][0] = sx(0x0022f964);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0022d088, 0x0022f964)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x78));
                    c.g[V0][0] = 0x1;
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1a4), c.g[V0][0]);
                    if t {
                        pc = 0x0022f990;
                        continue;
                    }
                    pc = 0x0022f974;
                    continue;
                }
                0x0022f974 => {
                    c.cycles += 1;
                    pc = 0x0022f978;
                    continue;
                }
                0x0022f978 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x0022f980);
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0022d8d8, 0x0022f980)?;
                    c.g[RA][0] = sx(0x0022f988);
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x003a6928, 0x0022f988)?;
                    c.g[RA][0] = sx(0x0022f990);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x003e5398, 0x0022f990)?;
                    pc = 0x0022f990;
                    continue;
                }
                0x0022f990 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x78));
                    c.g[RA][0] = sx(0x0022f99c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0022d278, 0x0022f99c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A3][0] = 0x1;
                    if t {
                        if back_edge_unwinds(c, 0x0022f978) {
                            return Ok(());
                        }
                        pc = 0x0022f978;
                        continue;
                    }
                    pc = 0x0022f9a4;
                    continue;
                }
                0x0022f9a4 => {
                    c.cycles += 61;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = c.g[S6][0];
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2aa0));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[S0][0] = 0x4c0000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x3d4), c.g[A3][0]);
                    c.g[RA][0] = sx(0x0022f9cc);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5830));
                    call_ptr(c, 0x003186d0, 0x0022f9cc)?;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2aa8));
                    c.g[RA][0] = sx(0x0022f9e0);
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x00318630, 0x0022f9e0)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x0022f9ec);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0016ca10, 0x0022f9ec)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0022f9f8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0022f9f8)?;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0022fa04);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x00318260, 0x0022fa04)?;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0x3010;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48a8));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0022fa1c);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x0022fa1c)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0022fa28);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002dbf80, 0x0022fa28)?;
                    c.g[V0][0] = 0x490000;
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7d20));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V0][0] = 0xff;
                    c.g[RA][0] = sx(0x0022fa48);
                    c.sw((c.g[S7][0] as u32), c.g[S0][0]);
                    call_ptr(c, 0x002ec3e0, 0x0022fa48)?;
                    c.g[RA][0] = sx(0x0022fa50);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    call_ptr(c, 0x002ed048, 0x0022fa50)?;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0022fa5c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0022fa5c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0022fa68);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0022fa68)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0022f6b0, 0x0022fa98);
                    return Ok(());
                }
            }
        }
    });
}

/// `void GameModule_CreateSectionScriptTable(GameModule* self, int sectionId)` (the section id is `a1`; `a0` is overwritten): when the section's entry (12 bytes each, kind >= 2 and count > 0) says it has scripts, creates the section's local script table (`WScriptMan_CreateLocalTable`).
/// (`GameModule_CreateSectionScriptTable`, 0x00230430)
pub fn game_module_create_section_script_table(c: &mut Cpu) {
    guest(c, |c| {
        'b_230468: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = 0xc;
            c.mult(0, 4, 5, 4);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = c.lh(c.g[A0][0] as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = 0u64;
            if t {
                break 'b_230468;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = (0u64 < c.g[V0][0]) as u64;
                break 'b_230468;
            }
        }
        'b_23047c: {
            c.cycles += 2;
            let t = c.g[A2][0] == 0u64;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            if t {
                break 'b_23047c;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                call_ptr(c, 0x00308b58, 0x00230478)?;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                break 'b_23047c;
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `int GameModule_Enter(GameModule *self)` (0x230050): the per-event
/// enter hook: starts the world's bolt/anim managers (0x195d50), clears
/// `gp - 0x9e4`, runs 0x1f30d8 on `self->+0x94`, 0x278710, two virtual calls
/// on the object at `gp + 0x1158` (slots at `+0x88`/`+0x8c` and
/// `+0x20`/`+0x24` of its vtable), then 0x369ff8, 0x3a67e0 (`self->+0x10`),
/// 0x2b68d0 and 0x2867e8, and sets the three "entered" flags (`gp + 0x12d8`,
/// `0x1234`, `0x14e8`) to 1. Frame 0x20 (s0).
pub fn game_module_enter_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x8c)).lw(c);
        call_ptr(c, 0x0019_5d50, 0x0023_006c)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x94)).lw(c);
        Addr(r32(c, GP).wrapping_sub(0x9e4)).sw(c, 0);
        call_ptr(c, 0x001f_30d8, 0x0023_0078)?;
        c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x84c)).lw(c);
        call_ptr(c, 0x0027_8710, 0x0023_0080)?;
        // Two virtual calls on the object at gp + 0x1158 (vtable at +4: a
        // halfword `this` delta and the function).
        for (delta_off, slot_off, ret_addr) in
            [(0x88, 0x8c, 0x0023_0098), (0x20, 0x24, 0x0023_00b0)]
        {
            c.g[A1][0] = Addr(r32(c, GP).wrapping_add(0x1158)).lw(c);
            c.g[V0][0] = Addr(r32(c, A1).wrapping_add(4)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(delta_off)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(slot_off)).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, ret_addr)?;
        }
        c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x854)).lw(c);
        call_ptr(c, 0x0036_9ff8, 0x0023_00b8)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x10)).lw(c);
        call_ptr(c, 0x003a_67e0, 0x0023_00c0)?;
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0x21e4)).lw(c);
        call_ptr(c, 0x002b_68d0, 0x0023_00c8)?;
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0x410)).lw(c);
        call_ptr(c, 0x0028_67e8, 0x0023_00d0)?;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, GP).wrapping_add(0x12d8)).sw(c, c.g[V0][0]);
        Addr(r32(c, GP).wrapping_add(0x1234)).sw(c, c.g[V0][0]);
        Addr(r32(c, GP).wrapping_add(0x14e8)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `GameModule_QuitToLoadScreen` (0x00230488): Leaves the race: stops the auto replay, requests game state 6, quits the network race and then makes the pre-game (`to_pregame != 0`) or pre-front-end load screen the next module. `(game, to_pregame)`.
/// NOT INSTALLED (no OVERRIDES entry, no test): a real unported callee frees fixture memory (DLHeap_Consolidate runaway); needs a faithful object graph.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_module_quit_to_load_screen(c: &mut Cpu) {
    guest(c, |c| {
        'b_2304d4: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x002706f0, 0x002304a8)?; // call_indirect
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x6;
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00231250, 0x002304bc)?; // GameModule_RequestState
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                break 'b_2304d4;
            } else {
                c.cycles += 3;
                call_ptr(c, 0x00258ae0, 0x002304d0)?; // cGameComm_QuitRace
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                break 'b_2304d4;
            }
        }
        'b_2304e4: {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;

            if t {
                break 'b_2304e4;
            } else {
                c.cycles += 2;
                call_ptr(c, 0x00266df8, 0x002304e4)?; // call_indirect
                break 'b_2304e4;
            }
        }
        'b_230548: {
            'b_23051c: {
                c.cycles += 2;
                let t = c.g[S1][0] == 0u64;
                c.g[A2][0] = 0x100;
                if t {
                    break 'b_23051c;
                } else {
                    c.cycles += 12;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0x8;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4890));
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x00230500)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00232720, 0x00230508)?; // cPreGameLoadScreen_ctor
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00316ef8, 0x00230514)?; // call_indirect
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_230548;
                    } else {
                        break 'b_23051c;
                    }
                }
            }
            c.cycles += 11;
            c.g[A1][0] = 0x480000;
            c.g[A0][0] = 0x8;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4878));
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317d70, 0x00230530)?; // call_indirect
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x00232738, 0x00230538)?; // cPreFELoadScreen_ctor
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x00316ef8, 0x00230544)?; // call_indirect
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_230548;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// The `GP`-relative address `gp + off`.
#[inline(always)]
pub(crate) fn gp_at(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add_signed(off)
}

/// The `off(s0)` address of the game object, read from the callee-saved
/// register each time (a callee may not change it, but the port does not
/// keep a copy across calls).
#[inline(always)]
pub(crate) fn s0_at(c: &Cpu, off: u32) -> u32 {
    (c.g[S0][0] as u32).wrapping_add(off)
}

/// The virtual deleting call of `VDelete` on the object in `a2`: its vtable
/// pointer sits at `obj + vt_at`, the `this` adjustment at vtable+8 and the
/// deleting entry at vtable+0xc (`a1` is set by the caller).
pub(crate) fn vdelete(c: &mut Cpu, vt_at: u32, ret: u32) -> Flow {
    let obj = c.g[A2][0] as u32;
    c.g[V0][0] = c.lw(obj.wrapping_add(vt_at));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// `void GameModule_Unload(GameModule* self)` (0x22fa98; the reference names
/// it `cGame_purge`): tear the race down. Shuts the back end's network stats
/// down (copying the two riders' stats into the network record, then the
/// interface's virtual `+0xc` entry), purges the managers and renderers,
/// deletes the owned objects (`if (self->X) X->dtor(3)` for each field, with
/// the virtual deletes for the vtable-owned ones), clears `SSXApp->Game` and
/// the global purge flags.
pub fn game_module_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[A0][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0x4d_0000;
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(gp_at(c, 0x2898));
        c.g[S3][0] = 0x4c_0000;
        // cBE_setState(0)
        call_ptr(c, 0x0014_de68, 0x0022_facc)?;
        c.g[S4][0] = 0x50_0000;
        c.g[V0][0] = c.lw(gp_at(c, -0x848));
        c.g[A1][0] = 7;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        // cBE_getInterface(BE, 7): the interface's vtable at +0xc
        call_ptr(c, 0x0014_dd58, 0x0022_fae0)?;
        c.g[V1][0] = 0x53_0000;
        c.g[T1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(0x0053_4b30);
        let t = c.g[V0][0] == 0;
        c.g[A0][0] = sx(0x0053_4b30);
        'network: {
            if t {
                break 'network;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw(gp_at(c, -0x848));
            if t {
                break 'network;
            }
            // The copies of both riders' stats (networkOpt+0x330 is MyStats,
            // +0x3dc OpponentStats; 0xac bytes each, the first 0xa0 in a loop).
            'copies: {
                'my_stats: {
                    c.cycles += 16;
                    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x330));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A2][0] = (0u64 < c.g[V1][0]) as u64;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = (c.g[V1][0] < 1u64) as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x790));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xfc));
                    c.g[V0][0] = c.g[A1][0] | c.g[A3][0];
                    c.g[V0][0] &= 7;
                    let t = c.g[V0][0] == 0;
                    c.g[T0][0] = sx((c.g[A2][0] as u32) << 2);
                    if t {
                        break 'my_stats;
                    }
                    // Unaligned: 32 bytes per step with ldl/ldr and sdl/sdr.
                    c.cycles += 2;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x19c));
                    c.g[V0][0] = 0x53_0000;
                    'l_my_u: loop {
                        c.cycles += 19;
                        let a1 = c.g[A1][0] as u32;
                        let a3 = c.g[A3][0] as u32;
                        c.ldl(4, a1.wrapping_add(0x7));
                        c.ldr(4, a1);
                        c.ldl(6, a1.wrapping_add(0xf));
                        c.ldr(6, a1.wrapping_add(0x8));
                        c.ldl(10, a1.wrapping_add(0x17));
                        c.ldr(10, a1.wrapping_add(0x10));
                        c.ldl(11, a1.wrapping_add(0x1f));
                        c.ldr(11, a1.wrapping_add(0x18));
                        c.sdl(4, a3.wrapping_add(0x7));
                        c.sdr(4, a3);
                        c.sdl(6, a3.wrapping_add(0xf));
                        c.sdr(6, a3.wrapping_add(0x8));
                        c.sdl(10, a3.wrapping_add(0x17));
                        c.sdr(10, a3.wrapping_add(0x10));
                        c.sdl(11, a3.wrapping_add(0x1f));
                        c.sdr(11, a3.wrapping_add(0x18));
                        c.g[A1][0] = sx(a1.wrapping_add(0x20));
                        let t = c.g[A1][0] != c.g[V1][0];
                        c.g[A3][0] = sx(a3.wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0022_fb48);
                            }
                            continue 'l_my_u;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.lw(gp_at(c, -0x848));
                        break 'copies;
                    }
                }
                // Aligned: 32 bytes per step with ld/sd.
                c.cycles += 2;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x19c));
                c.g[V0][0] = 0x53_0000;
                'l_my_a: loop {
                    c.cycles += 11;
                    let a1 = c.g[A1][0] as u32;
                    let a3 = c.g[A3][0] as u32;
                    c.g[A0][0] = c.ld(a1);
                    c.g[A2][0] = c.ld(a1.wrapping_add(0x8));
                    c.g[T2][0] = c.ld(a1.wrapping_add(0x10));
                    c.g[T3][0] = c.ld(a1.wrapping_add(0x18));
                    c.sd(a3, c.g[A0][0]);
                    c.sd(a3.wrapping_add(0x8), c.g[A2][0]);
                    c.sd(a3.wrapping_add(0x10), c.g[T2][0]);
                    c.sd(a3.wrapping_add(0x18), c.g[T3][0]);
                    c.g[A1][0] = sx(a1.wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V1][0];
                    c.g[A3][0] = sx(a3.wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_fba4);
                        }
                        continue 'l_my_a;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(gp_at(c, -0x848));
                    break 'copies;
                }
            }
            // The opponent's stats: the 12-byte header of the first record,
            // then 0xa0 bytes in a loop, each from the other rider.
            'opp_stats: {
                c.cycles += 18;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4f0c));
                c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[A1][0] as u32);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                c.sdl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[A3][0] as u32);
                let v = c.g[V1][0];
                c.sw((c.g[A3][0] as u32).wrapping_add(0x8), v);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x790));
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xfc));
                c.g[V0][0] = c.g[A0][0] | c.g[A2][0];
                c.g[V0][0] &= 7;
                let t = c.g[V0][0] == 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x19c));
                if t {
                    // Aligned: the loop below, straight from here.
                    'l_opp_a: loop {
                        c.cycles += 11;
                        let a0 = c.g[A0][0] as u32;
                        let a2 = c.g[A2][0] as u32;
                        c.g[T2][0] = c.ld(a0);
                        c.g[T3][0] = c.ld(a0.wrapping_add(0x8));
                        c.g[V0][0] = c.ld(a0.wrapping_add(0x10));
                        c.g[A1][0] = c.ld(a0.wrapping_add(0x18));
                        c.sd(a2, c.g[T2][0]);
                        c.sd(a2.wrapping_add(0x8), c.g[T3][0]);
                        c.sd(a2.wrapping_add(0x10), c.g[V0][0]);
                        c.sd(a2.wrapping_add(0x18), c.g[A1][0]);
                        c.g[A0][0] = sx(a0.wrapping_add(0x20));
                        let t = c.g[A0][0] != c.g[V1][0];
                        c.g[A2][0] = sx(a2.wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0022_fc70);
                            }
                            continue 'l_opp_a;
                        }
                        break 'opp_stats;
                    }
                }
                // Unaligned: 32 bytes per step with ldl/ldr and sdl/sdr.
                'l_opp_u: loop {
                    c.cycles += 19;
                    let a0 = c.g[A0][0] as u32;
                    let a2 = c.g[A2][0] as u32;
                    c.ldl(2, a0.wrapping_add(0x7));
                    c.ldr(2, a0);
                    c.ldl(5, a0.wrapping_add(0xf));
                    c.ldr(5, a0.wrapping_add(0x8));
                    c.ldl(7, a0.wrapping_add(0x17));
                    c.ldr(7, a0.wrapping_add(0x10));
                    c.ldl(8, a0.wrapping_add(0x1f));
                    c.ldr(8, a0.wrapping_add(0x18));
                    c.sdl(2, a2.wrapping_add(0x7));
                    c.sdr(2, a2);
                    c.sdl(5, a2.wrapping_add(0xf));
                    c.sdr(5, a2.wrapping_add(0x8));
                    c.sdl(7, a2.wrapping_add(0x17));
                    c.sdr(7, a2.wrapping_add(0x10));
                    c.sdl(8, a2.wrapping_add(0x1f));
                    c.sdr(8, a2.wrapping_add(0x18));
                    c.g[A0][0] = sx(a0.wrapping_add(0x20));
                    let t = c.g[A0][0] != c.g[V1][0];
                    c.g[A2][0] = sx(a2.wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_fc1c);
                        }
                        continue 'l_opp_u;
                    }
                    c.cycles += 2;
                    break;
                }
            }
            // The last 12 bytes of the opponent's record, then the
            // interface's virtual entry (`vtEntry->fn(vthis(iface, *vtEntry))`).
            c.cycles += 11;
            let a0 = c.g[A0][0] as u32;
            let a2 = c.g[A2][0] as u32;
            c.ldl(7, a0.wrapping_add(0x7));
            c.ldr(7, a0);
            c.g[T0][0] = c.lw(a0.wrapping_add(0x8));
            c.sdl(7, a2.wrapping_add(0x7));
            c.sdr(7, a2);
            c.sw(a2.wrapping_add(0x8), c.g[T0][0]);
            c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0022_fcc8)?;
        }

        // cCameraTriggerMan_cleanupOnExit(&gCamTriggerMan)
        c.cycles += 16;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5830));
        call_ptr(c, 0x0016_c9b0, 0x0022_fcd0)?;
        // GrMan->FlushRenderer()
        c.g[A1][0] = c.lw(gp_at(c, -0x854));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_fce8)?;
        // cFEAsyncManager_FlushFEAsyncTextures(SSXApp->mFEAsyncManager)
        c.g[V0][0] = c.lw(gp_at(c, -0x848));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
        call_ptr(c, 0x001a_3c50, 0x0022_fcf4)?;
        // cSSXScriptEngine_ForceEndScriptsAll(ScriptEngine)
        c.g[A0][0] = c.lw(gp_at(c, -0x84c));
        call_ptr(c, 0x0027_8a98, 0x0022_fcfc)?;
        // if (self->UI) cUIEngine(self->UI, 3)
        c.g[A0][0] = c.lw(s0_at(c, 0x48));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0039_7b70, 0x0022_fd10)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x7c));
        // if (self->InputSync) cInputSync(self->InputSync, 3)
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            Callee {
                addr: 0x0023_70a0,
                f: ee::s06::InputSync_dtor,
            }
            .call(c, 0x0022_fd24)?;
        }
        // cCutSceneTransitionMan_ForceToRegularMode, SurfPropCache purge and
        // delete, then the SkyBox virtual delete.
        c.cycles += 9;
        c.g[A0][0] = c.lw(s0_at(c, 0x64));
        call_ptr(c, 0x002e_a820, 0x0022_fd2c)?;
        c.g[A0][0] = c.lw(s0_at(c, 0x44));
        call_ptr(c, 0x002c_3fa0, 0x0022_fd34)?;
        c.g[A0][0] = c.lw(s0_at(c, 0x44));
        call_ptr(c, 0x0031_7e50, 0x0022_fd3c)?;
        c.g[A2][0] = c.lw(s0_at(c, 0x14));
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !t {
            c.cycles += 5;
            vdelete(c, 0xc, 0x0022_fd5c)?;
        }
        // FogMan (vtable at +4).
        c.cycles += 3;
        c.g[A2][0] = c.lw(s0_at(c, 0x18));
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !t {
            c.cycles += 5;
            vdelete(c, 0x4, 0x0022_fd7c)?;
        }
        // LensFxMan (vtable at +0x649c).
        c.cycles += 3;
        c.g[A2][0] = c.lw(s0_at(c, 0x1c));
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !t {
            c.cycles += 5;
            vdelete(c, 0x649c, 0x0022_fd9c)?;
        }
        // cPathSys_purgePaths(gPaths), cBXNash purge, cMenuSys delete.
        c.cycles += 7;
        c.g[RA][0] = sx(0x0022_fda4);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x33a0));
        call_ptr(c, 0x0026_af00, 0x0022_fda4)?;
        c.g[A0][0] = c.lw(s0_at(c, 0x20));
        call_ptr(c, 0x0033_4850, 0x0022_fdac)?;
        c.g[A0][0] = c.lw(s0_at(c, 0xa4));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002c_c048, 0x0022_fdc0)?;
        }
        c.g[A0][0] = c.g[S1][0];
        // cBucketMan_mergeBucket(bm, 1), cBucketMan_purge(bm).
        c.cycles += 7;
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_4c98, 0x0022_fdcc)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0035_4b38, 0x0022_fdd4)?;
        // WScriptMan (vtable at +0x494).
        c.g[A2][0] = c.lw(s0_at(c, 0x30));
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !t {
            c.cycles += 5;
            vdelete(c, 0x494, 0x0022_fdf4)?;
        }
        // CrowdMan2d, CrowdMan3d, MultiParticleMan, PlaybackMan, InstanceMan.
        c.cycles += 3;
        c.g[A0][0] = c.lw(s0_at(c, 0x38));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0022_9278, 0x0022_fe08)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x3c));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0034_3b28, 0x0022_fe1c)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x40));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0035_7a00, 0x0022_fe30)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x34));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0026_c588, 0x0022_fe44)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x88));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0035_1120, 0x0022_fe58)?;
        }
        // cCameraTriggerMan_purge, cRenderStateMan_purge, ViewMan delete.
        c.cycles += 7;
        c.g[RA][0] = sx(0x0022_fe60);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5830));
        call_ptr(c, 0x0016_cbe0, 0x0022_fe60)?;
        call_ptr(c, 0x002e_cfa0, 0x0022_fe68)?;
        c.g[A0][0] = c.lw(s0_at(c, 0x84));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0022_e730, 0x0022_fe7c)?;
        }
        c.g[A2][0] = c.lw(s0_at(c, 0xc));
        // AI (vtable at +0xcc).
        c.cycles += 2;
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !t {
            c.cycles += 5;
            vdelete(c, 0xcc, 0x0022_fe9c)?;
        }
        // Replay.
        c.cycles += 3;
        c.g[A0][0] = c.lw(s0_at(c, 0x28));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0026_f088, 0x0022_feb0)?;
        }
        c.g[A2][0] = c.lw(s0_at(c, 0x98));
        // StreamPause (vtable at +0).
        c.cycles += 2;
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !t {
            c.cycles += 5;
            vdelete(c, 0x0, 0x0022_fed0)?;
        }
        // SectionMan, FXLoader, WorldLightMan, World, OcclusionMan, mpTrackBigFile.
        c.cycles += 3;
        c.g[A0][0] = c.lw(s0_at(c, 0x78));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0022_cd18, 0x0022_fee4)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x2c));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002d_bee0, 0x0022_fef8)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x90));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002f_5230, 0x0022_ff0c)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x10));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x003a_6740, 0x0022_ff20)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x8c));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0022_ff34);
            c.g[A1][0] = 3;
            Callee {
                addr: 0x0024_2518,
                f: ee::s06::OcclusionMan_dtor,
            }
            .call(c, 0x0022_ff34)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0xa8));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0031_68e8, 0x0022_ff48)?;
        }
        // LoadIndicator (plain delete), DebugFont, then the BXNash array.
        c.cycles += 5;
        c.g[A0][0] = c.lw(s0_at(c, 0x94));
        call_ptr(c, 0x0031_7e50, 0x0022_ff50)?;
        c.g[A0][0] = c.lw(s0_at(c, 0x24));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0039_16c0, 0x0022_ff64)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x20));
        // nash->~sObj20() over its 0x14-byte entries, then delete.
        c.cycles += 2;
        let t = c.g[A0][0] == 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa0));
        if !t {
            c.cycles += 2;
            let t = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
            if !t {
                loop {
                    c.cycles += 7;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                    if !t {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_ff78);
                    }
                }
            }
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0022_ff9c)?;
        }
        // GrMan->PurgeCharModels(), GrMan->RemoveTexture(self->mBoxTextureID),
        // cGraphicsMan_PurgeTextures, cIrradianceDataBase_Purge.
        c.cycles += 22;
        c.g[A1][0] = c.lw(gp_at(c, -0x854));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x388));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x38c));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_ffb4)?;
        c.g[A2][0] = c.lw(gp_at(c, -0x854));
        c.g[A1][0] = c.lw(s0_at(c, 0x220));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x190));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x194));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_ffd0)?;
        c.g[A0][0] = c.lw(gp_at(c, -0x854));
        call_ptr(c, 0x0036_a020, 0x0022_ffd8)?;
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x47f8));
        call_ptr(c, 0x0038_adb0, 0x0022_ffe0)?;
        // SSXApp->Game = 0; WorldSound delete.
        c.g[V0][0] = c.lw(gp_at(c, -0x848));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), 0);
        c.g[A0][0] = c.lw(s0_at(c, 0x4c));
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002b_6ae0, 0x0022_fffc)?;
        }
        // The world-level resets and the global purges.
        c.cycles += 21;
        c.g[A0][0] = c.lw(gp_at(c, 0x21e4));
        call_ptr(c, 0x002b_68d0, 0x0023_0004)?;
        c.g[A0][0] = c.lw(gp_at(c, 0x410));
        call_ptr(c, 0x0028_6a80, 0x0023_000c)?;
        c.g[A0][0] = c.lw(gp_at(c, 0x410));
        call_ptr(c, 0x0028_6c00, 0x0023_0014)?;
        c.g[A0][0] = c.lw(gp_at(c, -0x84c));
        call_ptr(c, 0x0027_8590, 0x0023_001c)?;
        call_ptr(c, 0x0036_a290, 0x0023_0024)?;
        c.g[V0][0] = c.lw(gp_at(c, 0x2a74));
        c.sw(gp_at(c, -0x69c), 0);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x34), 0);
        frame.close(c);
        Ok(())
    })
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

/// `void cLoadScreen::drawSplash(void)` (0x231b50), `GameModule_DrawFullscreenQuad`:
/// builds a 4-pass fullscreen-quad vertex block on the stack (zeroed, with a
/// 1.0 in f0), stores its colour words into the draw context at gp-0x854, and
/// draws it through the context's virtual method (slot at +0x23c, this at +0x238).
pub fn c_load_screen_draw_splash(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0xe0);
        c.g[T0][0] = c.g[A1][0];
        frame.save_ra(c, 0xd0);
        c.g[V0][0] = 0x3;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        // Four passes: v0 counts 3, 2, 1, 0.
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = 0x80;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_1b68);
            }
        }
        c.cycles += 5;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0); // f0 = 1.0
        c.g[V1][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
        loop {
            c.cycles += 8;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V0][0] == 0u64 {
                break;
            }
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
            if event_due(c) {
                deliver_event_here(c, 0x0023_1b98);
            }
        }
        c.cycles += 45;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A1][0] = 0x4;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), 0u64);
        c.g[A2][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
        c.g[A3][0] = 0u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), 0u64);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xcc));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), 0u64);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc4));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
        c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw(c.g[SP][0] as u32, 0u64);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x60), 0u64);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x64));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
        // Store the pass count into the draw context and draw.
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
        c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
        let draw = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, draw, 0x0023_1c60)?;
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameModule_ResetRace(cGame* self)` (0x230180): resets the race for a
/// restart (playback, buckets, AI, avalanche, views, world, replay, scripts,
/// camera, crowds, particles, HUD state event 7, audio) and sets game state 6.
pub fn oc681_game_module_reset_race(c: &mut Cpu) {
    guest(c, |c| {
        'b_230274: {
            c.cycles += 55;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2898));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.g[RA][0] = sx(0x002301a0);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
            call_ptr(c, 0x0026ca90, 0x002301a0)?;
            c.g[RA][0] = sx(0x002301a8);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
            call_ptr(c, 0x00309030, 0x002301a8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002301b4);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x00354c98, 0x002301b4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002301c0);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x00355118, 0x002301c0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002301cc);
            c.g[A1][0] = 0x8;
            call_ptr(c, 0x00355118, 0x002301cc)?;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = 0x3;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcc));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x002301e8);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x002301e8)?;
            c.g[RA][0] = sx(0x002301f0);
            call_ptr(c, 0x002d9b40, 0x002301f0)?;
            c.g[RA][0] = sx(0x002301f8);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
            call_ptr(c, 0x0022e7c8, 0x002301f8)?;
            c.g[RA][0] = sx(0x00230200);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x003a6800, 0x00230200)?;
            c.g[RA][0] = sx(0x00230208);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x0026f228, 0x00230208)?;
            c.g[RA][0] = sx(0x00230210);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            call_ptr(c, 0x002789e0, 0x00230210)?;
            c.g[RA][0] = sx(0x00230218);
            call_ptr(c, 0x0015db58, 0x00230218)?;
            c.g[RA][0] = sx(0x00230220);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            call_ptr(c, 0x00229498, 0x00230220)?;
            c.g[RA][0] = sx(0x00230228);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
            call_ptr(c, 0x00343bc0, 0x00230228)?;
            c.g[RA][0] = sx(0x00230230);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            call_ptr(c, 0x00357b38, 0x00230230)?;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ab0));
            c.g[RA][0] = sx(0x00230244);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x00317670, 0x00230244)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00230250);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039f9d8, 0x00230250)?;
            c.g[A3][0] = c.g[V0][0];
            let t = c.g[A3][0] == 0u64;
            c.g[A1][0] = 0x7;
            if t {
                break 'b_230274;
            } else {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00230274);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00230274)?;
                break 'b_230274;
            }
        }
        c.cycles += 13;
        c.g[RA][0] = sx(0x0023027c);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        call_ptr(c, 0x00309f18, 0x0023027c)?;
        c.g[RA][0] = sx(0x00230284);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
        call_ptr(c, 0x00308c60, 0x00230284)?;
        c.g[RA][0] = sx(0x0023028c);
        call_ptr(c, 0x0028b180, 0x0023028c)?;
        c.g[RA][0] = sx(0x00230294);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x00287108, 0x00230294)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameModule_QuitToLoadScreen(cGame* self, int toFrontEnd)` (0x230488):
/// requests state 6, quits the comm race and voip when present, then makes a
/// `cPreGameLoadScreen` (`toFrontEnd != 0`) or `cPreFELoadScreen` the next module.
pub fn oc681_game_module_quit_to_load_screen(c: &mut Cpu) {
    guest(c, |c| {
        'b_2304d4: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.g[RA][0] = sx(0x002304a8);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x002706f0, 0x002304a8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x6;
            c.g[A2][0] = 0u64;
            c.g[RA][0] = sx(0x002304bc);
            c.g[A3][0] = 0u64;
            Callee {
                addr: 0x00231250,
                f: ee::s06::GameModule_RequestState,
            }
            .call(c, 0x002304bc)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                break 'b_2304d4;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002304d0);
                Callee {
                    addr: 0x00258ae0,
                    f: ee::s06::cGameComm_QuitRace,
                }
                .call(c, 0x002304d0)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                break 'b_2304d4;
            }
        }
        'b_2304e4: {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;

            if t {
                break 'b_2304e4;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x002304e4);
                call_ptr(c, 0x00266df8, 0x002304e4)?;
                break 'b_2304e4;
            }
        }
        'b_230548: {
            'b_23051c: {
                c.cycles += 2;
                let t = c.g[S1][0] == 0u64;
                c.g[A2][0] = 0x100;
                if t {
                    break 'b_23051c;
                } else {
                    c.cycles += 12;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0x8;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4890));
                    c.g[RA][0] = sx(0x00230500);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x00230500)?;
                    c.g[RA][0] = sx(0x00230508);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x00232720,
                        f: ee::s06::cPreGameLoadScreen_ctor,
                    }
                    .call(c, 0x00230508)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[RA][0] = sx(0x00230514);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00316ef8, 0x00230514)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_230548;
                    } else {
                        break 'b_23051c;
                    }
                }
            }
            c.cycles += 11;
            c.g[A1][0] = 0x480000;
            c.g[A0][0] = 0x8;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4878));
            c.g[RA][0] = sx(0x00230530);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317d70, 0x00230530)?;
            c.g[RA][0] = sx(0x00230538);
            c.g[A0][0] = c.g[V0][0];
            Callee {
                addr: 0x00232738,
                f: ee::s06::cPreFELoadScreen_ctor,
            }
            .call(c, 0x00230538)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[RA][0] = sx(0x00230544);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x00316ef8, 0x00230544)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_230548;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameModule_BeginReplayPlayback(cGame* self)` (0x230558): pushes pause
/// state 8 on the pause stack, copies the pause flags, starts the replay
/// and activates the menu system.
pub fn oc681_game_module_begin_replay_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = 0x530000;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
        c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
        c.g[A1][0] = 0x440000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2910));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = 0x8;
        c.g[A1][0] = 0u64;
        c.sw(c.g[V0][0] as u32, c.g[A2][0]);
        c.g[A3][0] = 0u64;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[A0][0]);
        c.g[A2][0] = 0u64;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
        c.g[RA][0] = sx(0x002305ac);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[T0][0]);
        call_ptr(c, 0x0026f8a0, 0x002305ac)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
        c.g[RA][0] = sx(0x002305b8);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x002cc0b8, 0x002305b8)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `bool GameModule_CanRestartFromPause(cGame* self)` (0x231ab8): false while the
/// pending state is 5, 14, 1 or 10; otherwise the script engine's "pause menu
/// allowed" answer combined with the same test on the current state.
pub fn oc681_game_module_can_restart_from_pause(c: &mut Cpu) {
    guest(c, |c| {
        'b_231b3c: {
            'b_231b04: {
                'b_231b00: {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x210));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x5;
                    if t {
                        break 'b_231b00;
                    } else {
                        'b_231af8: {
                            c.cycles += 2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 0xe;
                            if t {
                                break 'b_231af8;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_231af8;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0xa;
                                    if t {
                                        break 'b_231af8;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                        if t {
                                            break 'b_231b04;
                                        } else {
                                            break 'b_231af8;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = true;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_231b3c;
                        } else {
                            break 'b_231b00;
                        }
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                break 'b_231b04;
            }
            'b_231b38: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00231b0c);
                c.g[S0][0] = 0u64;
                call_ptr(c, 0x00278da0, 0x00231b0c)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x5;
                if t {
                    break 'b_231b38;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x214));
                    let t = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = 0xe;
                    if t {
                        break 'b_231b38;
                    } else {
                        c.cycles += 2;
                        let t = c.g[A0][0] == c.g[V0][0];
                        c.g[V1][0] = 0x1;
                        if t {
                            break 'b_231b38;
                        } else {
                            c.cycles += 2;
                            let t = c.g[A0][0] == c.g[V1][0];
                            c.g[V0][0] = c.g[A0][0] ^ 0xa;
                            if t {
                                break 'b_231b38;
                            } else {
                                c.cycles += 2;
                                c.g[S0][0] = c.g[V1][0];
                                if c.g[V0][0] == 0 {
                                    c.g[S0][0] = 0u64;
                                }
                                break 'b_231b38;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
            break 'b_231b3c;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `GameModule_LoadState` (0x002314d0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn game_module_load_state(c: &mut Cpu) {
    if c.pc != 0x002314d0 {
        return game_module_load_state_resume(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002314d0;
        loop {
            match pc {
                0x002314d0 => {
                    c.cycles += 96;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A1][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S1][0] = c.g[A0][0];
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x68));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00231500);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00231500)?;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x68));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0023151c);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0023151c)?;
                    c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x214));
                    c.g[A2][0] = 0x4;
                    c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00231538);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00231538)?;
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x218));
                    c.g[A2][0] = 0x4;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00231554);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00231554)?;
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x210));
                    c.g[A2][0] = 0x4;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00231570);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00231570)?;
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A2][0] = 0x4;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x21c));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0023158c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0023158c)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb0));
                    c.g[RA][0] = sx(0x00231598);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00234a30,
                        f: ee::s06::GameStatePreRaceLoad_LoadState,
                    }
                    .call(c, 0x00231598)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
                    c.g[RA][0] = sx(0x002315a4);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00236d80,
                        f: ee::s06::GameStatePreRaceMenu_LoadState,
                    }
                    .call(c, 0x002315a4)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xe0));
                    c.g[RA][0] = sx(0x002315b0);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00234e00,
                        f: ee::s06::GameStateCountdown_LoadState,
                    }
                    .call(c, 0x002315b0)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf0));
                    c.g[RA][0] = sx(0x002315bc);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00234ed8,
                        f: ee::s06::GameStateRace_LoadState,
                    }
                    .call(c, 0x002315bc)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x100));
                    c.g[RA][0] = sx(0x002315c8);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00234120,
                        f: ee::s06::GameStateFinish_LoadState,
                    }
                    .call(c, 0x002315c8)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x118));
                    c.g[RA][0] = sx(0x002315d4);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00233bb0,
                        f: ee::s06::GameStateQuit_LoadState,
                    }
                    .call(c, 0x002315d4)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x128));
                    c.g[RA][0] = sx(0x002315e0);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00236ee8,
                        f: ee::s06::GameStateResults_LoadState,
                    }
                    .call(c, 0x002315e0)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x138));
                    c.g[RA][0] = sx(0x002315ec);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00233bf0,
                        f: ee::s06::GameStateIdle_LoadState,
                    }
                    .call(c, 0x002315ec)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x148));
                    c.g[RA][0] = sx(0x002315f8);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00236b90,
                        f: ee::s06::GameStateResetAI_LoadState,
                    }
                    .call(c, 0x002315f8)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x158));
                    c.g[RA][0] = sx(0x00231604);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00235a18,
                        f: ee::s06::GameStateReset_LoadState,
                    }
                    .call(c, 0x00231604)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x178));
                    c.g[RA][0] = sx(0x00231610);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00236b18,
                        f: ee::s06::GameStateEventSwitchLoad_LoadState,
                    }
                    .call(c, 0x00231610)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18c));
                    c.g[RA][0] = sx(0x0023161c);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00236fc8,
                        f: ee::s06::GameStateFinishCinematic_LoadState,
                    }
                    .call(c, 0x0023161c)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x19c));
                    c.g[RA][0] = sx(0x00231628);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00235fd0,
                        f: ee::s06::GameStateRestartRace_LoadState,
                    }
                    .call(c, 0x00231628)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1b8));
                    c.g[RA][0] = sx(0x00231634);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002367c8,
                        f: ee::s06::GameStateEventSelect_LoadState,
                    }
                    .call(c, 0x00231634)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00231640);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1dc));
                    Callee {
                        addr: 0x002361e8,
                        f: ee::s06::GameStateWarpToEvent_LoadState,
                    }
                    .call(c, 0x00231640)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x214));
                    c.g[V0][0] = (c.g[V1][0] < 0x11u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00231730;
                        continue;
                    }
                    pc = 0x00231650;
                    continue;
                }
                0x00231650 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x47d0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00231668 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), 0u64);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231670;
                    continue;
                }
                0x00231670 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x0023167c;
                    continue;
                }
                0x0023167c => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231688;
                    continue;
                }
                0x00231688 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xe0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231694;
                    continue;
                }
                0x00231694 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316a0;
                    continue;
                }
                0x002316a0 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x100));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316ac;
                    continue;
                }
                0x002316ac => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316b8;
                    continue;
                }
                0x002316b8 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x128));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316c4;
                    continue;
                }
                0x002316c4 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x138));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316d0;
                    continue;
                }
                0x002316d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x148));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316dc;
                    continue;
                }
                0x002316dc => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x158));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316e8;
                    continue;
                }
                0x002316e8 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x178));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316f4;
                    continue;
                }
                0x002316f4 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18c));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231700;
                    continue;
                }
                0x00231700 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x19c));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x0023170c;
                    continue;
                }
                0x0023170c => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1b8));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231718;
                    continue;
                }
                0x00231718 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1dc));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231724;
                    continue;
                }
                0x00231724 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1ec));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231730;
                    continue;
                }
                0x00231730 => {
                    c.cycles += 2;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x214), 0u64);
                    pc = 0x00231738;
                    continue;
                }
                0x00231738 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x218));
                    c.g[V0][0] = (c.g[V1][0] < 0x11u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00231820;
                        continue;
                    }
                    pc = 0x00231748;
                    continue;
                }
                0x00231748 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4780));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00231760 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x0023176c;
                    continue;
                }
                0x0023176c => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x00231778;
                    continue;
                }
                0x00231778 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xe0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x00231784;
                    continue;
                }
                0x00231784 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x00231790;
                    continue;
                }
                0x00231790 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x100));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x0023179c;
                    continue;
                }
                0x0023179c => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317a8;
                    continue;
                }
                0x002317a8 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x128));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317b4;
                    continue;
                }
                0x002317b4 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x138));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317c0;
                    continue;
                }
                0x002317c0 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x148));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317cc;
                    continue;
                }
                0x002317cc => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x158));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317d8;
                    continue;
                }
                0x002317d8 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x178));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317e4;
                    continue;
                }
                0x002317e4 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18c));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317f0;
                    continue;
                }
                0x002317f0 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x19c));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317fc;
                    continue;
                }
                0x002317fc => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1b8));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x00231808;
                    continue;
                }
                0x00231808 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1dc));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x00231814;
                    continue;
                }
                0x00231814 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1ec));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x00231820;
                    continue;
                }
                0x00231820 => {
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x218), 0u64);
                    pc = 0x00231824;
                    continue;
                }
                0x00231824 => {
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), 0u64);
                    pc = 0x00231828;
                    continue;
                }
                0x00231828 => {
                    c.cycles += 5;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002314d0, 0x0023183c);
                    return Ok(());
                }
            }
        }
    });
}

/// Native resume of `game_module_load_state` (converted from the `generated` body): any entry
/// other than 0x002314d0, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn game_module_load_state_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002314d0 => {
                    c.cycles += 96;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A1][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[S1][0] = c.g[A0][0];
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x68));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00231500);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00231500)?;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x68));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0023151c);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0023151c)?;
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32));
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x214));
                    c.g[A2][0] = 0x4;
                    c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00231538);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00231538)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x218));
                    c.g[A2][0] = 0x4;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00231554);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00231554)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x210));
                    c.g[A2][0] = 0x4;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00231570);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00231570)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                    c.g[A2][0] = 0x4;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x21c));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0023158c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0023158c)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb0));
                    c.g[RA][0] = sx(0x00231598);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00234a30,
                        f: ee::s06::GameStatePreRaceLoad_LoadState,
                    }
                    .call(c, 0x00231598)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
                    c.g[RA][0] = sx(0x002315a4);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00236d80,
                        f: ee::s06::GameStatePreRaceMenu_LoadState,
                    }
                    .call(c, 0x002315a4)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xe0));
                    c.g[RA][0] = sx(0x002315b0);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00234e00,
                        f: ee::s06::GameStateCountdown_LoadState,
                    }
                    .call(c, 0x002315b0)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf0));
                    c.g[RA][0] = sx(0x002315bc);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00234ed8,
                        f: ee::s06::GameStateRace_LoadState,
                    }
                    .call(c, 0x002315bc)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x100));
                    c.g[RA][0] = sx(0x002315c8);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00234120,
                        f: ee::s06::GameStateFinish_LoadState,
                    }
                    .call(c, 0x002315c8)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x118));
                    c.g[RA][0] = sx(0x002315d4);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00233bb0,
                        f: ee::s06::GameStateQuit_LoadState,
                    }
                    .call(c, 0x002315d4)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x128));
                    c.g[RA][0] = sx(0x002315e0);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00236ee8,
                        f: ee::s06::GameStateResults_LoadState,
                    }
                    .call(c, 0x002315e0)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x138));
                    c.g[RA][0] = sx(0x002315ec);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00233bf0,
                        f: ee::s06::GameStateIdle_LoadState,
                    }
                    .call(c, 0x002315ec)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x148));
                    c.g[RA][0] = sx(0x002315f8);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00236b90,
                        f: ee::s06::GameStateResetAI_LoadState,
                    }
                    .call(c, 0x002315f8)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x158));
                    c.g[RA][0] = sx(0x00231604);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00235a18,
                        f: ee::s06::GameStateReset_LoadState,
                    }
                    .call(c, 0x00231604)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x178));
                    c.g[RA][0] = sx(0x00231610);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00236b18,
                        f: ee::s06::GameStateEventSwitchLoad_LoadState,
                    }
                    .call(c, 0x00231610)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18c));
                    c.g[RA][0] = sx(0x0023161c);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00236fc8,
                        f: ee::s06::GameStateFinishCinematic_LoadState,
                    }
                    .call(c, 0x0023161c)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x19c));
                    c.g[RA][0] = sx(0x00231628);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00235fd0,
                        f: ee::s06::GameStateRestartRace_LoadState,
                    }
                    .call(c, 0x00231628)?;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1b8));
                    c.g[RA][0] = sx(0x00231634);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002367c8,
                        f: ee::s06::GameStateEventSelect_LoadState,
                    }
                    .call(c, 0x00231634)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00231640);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1dc));
                    Callee {
                        addr: 0x002361e8,
                        f: ee::s06::GameStateWarpToEvent_LoadState,
                    }
                    .call(c, 0x00231640)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x214));
                    c.g[V0][0] = (c.g[V1][0] < 0x11u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00231730;
                        continue;
                    }
                    pc = 0x00231650;
                    continue;
                }
                0x00231650 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x47d0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00231668 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), 0u64);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231670;
                    continue;
                }
                0x00231670 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x0023167c;
                    continue;
                }
                0x0023167c => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231688;
                    continue;
                }
                0x00231688 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xe0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231694;
                    continue;
                }
                0x00231694 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316a0;
                    continue;
                }
                0x002316a0 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x100));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316ac;
                    continue;
                }
                0x002316ac => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316b8;
                    continue;
                }
                0x002316b8 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x128));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316c4;
                    continue;
                }
                0x002316c4 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x138));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316d0;
                    continue;
                }
                0x002316d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x148));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316dc;
                    continue;
                }
                0x002316dc => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x158));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316e8;
                    continue;
                }
                0x002316e8 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x178));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x002316f4;
                    continue;
                }
                0x002316f4 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18c));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231700;
                    continue;
                }
                0x00231700 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x19c));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x0023170c;
                    continue;
                }
                0x0023170c => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1b8));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231718;
                    continue;
                }
                0x00231718 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1dc));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231724;
                    continue;
                }
                0x00231724 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1ec));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    if t {
                        pc = 0x00231738;
                        continue;
                    }
                    pc = 0x00231730;
                    continue;
                }
                0x00231730 => {
                    c.cycles += 2;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x200), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x214), 0u64);
                    pc = 0x00231738;
                    continue;
                }
                0x00231738 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x218));
                    c.g[V0][0] = (c.g[V1][0] < 0x11u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00231820;
                        continue;
                    }
                    pc = 0x00231748;
                    continue;
                }
                0x00231748 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4780));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00231760 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x0023176c;
                    continue;
                }
                0x0023176c => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x00231778;
                    continue;
                }
                0x00231778 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xe0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x00231784;
                    continue;
                }
                0x00231784 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf0));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x00231790;
                    continue;
                }
                0x00231790 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x100));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x0023179c;
                    continue;
                }
                0x0023179c => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317a8;
                    continue;
                }
                0x002317a8 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x128));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317b4;
                    continue;
                }
                0x002317b4 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x138));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317c0;
                    continue;
                }
                0x002317c0 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x148));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317cc;
                    continue;
                }
                0x002317cc => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x158));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317d8;
                    continue;
                }
                0x002317d8 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x178));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317e4;
                    continue;
                }
                0x002317e4 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18c));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317f0;
                    continue;
                }
                0x002317f0 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x19c));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x002317fc;
                    continue;
                }
                0x002317fc => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1b8));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x00231808;
                    continue;
                }
                0x00231808 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1dc));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x00231814;
                    continue;
                }
                0x00231814 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1ec));
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), c.g[V0][0]);
                    if t {
                        pc = 0x00231828;
                        continue;
                    }
                    pc = 0x00231820;
                    continue;
                }
                0x00231820 => {
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x218), 0u64);
                    pc = 0x00231824;
                    continue;
                }
                0x00231824 => {
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), 0u64);
                    pc = 0x00231828;
                    continue;
                }
                0x00231828 => {
                    c.cycles += 5;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002314d0, 0x0023183c);
                    return Ok(());
                }
            }
        }
    });
}

/// `AppModule_Unload` (0x001d_c930): straight-line leaf. Mechanical port of the generated body.
pub fn app_module_unload_001dc930(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `AppModule_Enter` (0x001d_c938): straight-line leaf. Mechanical port of the generated body.
pub fn app_module_enter_001dc938(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::slot;

    pub(crate) use crate::game::bx::bxstring::tests::heap_init_4 as heap_init_2;
    pub(crate) use crate::game::bx::bxstring::tests::pool;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::rng_2 as rng;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W;

    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS;
    pub(crate) use crate::game::input::inputparse::tests::EMPTY;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;

    use ssx3_ee as ee;

    #[test]
    fn game_module_get_transition_man_matches() {
        check(
            ssx3_ee::s06::f_00230698,
            game_module_get_transition_man,
            100,
            &|c, seed| {
                let mut r = Rng(0x0c11_0006 ^ seed as u64);
                scramble(c, &mut r);
                let idx = (seed % 16) as u32;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = idx as u64;
                c.bus.write32(DATA + idx * 4 + 0x5c, r.next_u32());
            },
        );
    }

    #[test]
    fn game_module_get_splash_manager_matches() {
        check(
            ssx3_ee::s06::f_002306a8,
            game_module_get_splash_manager,
            100,
            &|c, seed| {
                let mut r = Rng(0x0c11_0007 ^ seed as u64);
                scramble(c, &mut r);
                let idx = (seed % 16) as u32;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = idx as u64;
                c.bus.write32(DATA + idx * 4 + 0x68, r.next_u32());
            },
        );
    }

    #[test]
    fn app_module_ctor_matches() {
        check(ssx3_ee::s06::f_00231cd0, app_module_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_0008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x51b8), r.next_u32());
            for i in 0..3 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn pre_start_screen_ctor_matches() {
        check(
            ssx3_ee::s06::f_00232328,
            pre_start_screen_ctor,
            64,
            &|c, seed| {
                let mut r = Rng(0x0c11_000a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..3 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn check_pause_request_matches() {
        check(
            ssx3_ee::s06::f_00231840__generated,
            game_module_check_pause_request,
            300,
            &|c, seed| {
                let mut r = Rng(0x2318_4067 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let g = GAME_GP;
                // A loading screen, sometimes already frozen.
                let loading = DATA + 0x500;
                c.bus.write32(
                    g - 0x204,
                    match seed % 4 {
                        0 | 1 => 0,
                        _ => loading,
                    },
                );
                c.bus.write32(
                    loading + 8,
                    match seed % 3 {
                        0 => 0,
                        1 => 1,
                        _ => 0xffff_ffff,
                    },
                );
                // The confirm input evaluation runs the input VM; most seeds
                // skip it, some reach it with a stream that stops at once.
                c.bus.write32(g - 0x254, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(g - 0x84c, 0x002c_c098);
                let map = DATA + 0x6300;
                c.bus.write32(map + 4, DATA + 0x6400);
                c.bus.write32(map + 0x14, DATA + 0x6500);
                c.bus.write32(DATA + 0x6400 + 0xf0, 0);
                c.bus.write32(DATA + 0x6500, 0x20); // the VM's stop opcode
                                                    // The cBE singleton and the interface slot it indexes.
                c.bus.write32(g - 0x1f04, 0x0080_9000);
                c.bus.write32(g - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, 0);
                c.bus.write32(DATA + 0x1004, DATA + 0x1200);
                c.bus.write16(DATA + 0x1208, 0);
                c.bus.write32(DATA + 0x120c, 0x002c_c098);
                // The game module and its player/rider managers.
                let game = DATA + 0x200;
                c.g[A0][0] = game as u64;
                c.bus.write32(game + 0x28, DATA + 0x3000);
                c.bus.write32(game + 0xc, DATA + 0x4000);
                c.bus.write32(game + 0x210, 0); // CanRestartFromPause: 0
                c.bus.write32(game + 0x224, r.next_u32());
                c.bus.write32(game + 0x228, r.next_u32());
                // One active player, or a rider list.
                let one = seed % 3 != 0;
                c.bus.write32(DATA + 0x3000 + 0x608, one as u32);
                c.bus.write32(DATA + 0x3000 + 0x28, (seed % 4) as u32);
                c.bus.write32(DATA + 0x4000 + 0x7c, (seed % 4) as u32);
                for i in 0..4u32 {
                    let rider = DATA + 0x4100 + i * 0x20;
                    c.bus.write32(DATA + 0x4000 + 0x40 + i * 4, rider);
                    c.bus.write32(rider + 0x18, DATA + 0x4200 + i * 0x20);
                    c.bus
                        .write32(DATA + 0x4200 + i * 0x20 + 0x87c, ((seed + i) % 2) as u32);
                    // BENewPlayerInt slot table: slot i, player i.
                    c.bus.write32(0x0053_05b0 + i * 4, i);
                    c.bus.write32(0x0053_4fe0 + i * 0x1c + 0xc, i);
                }
                c.bus.write32(g - 0x848, DATA + 0x6000);
                for i in 0..4u32 {
                    c.bus
                        .write32(DATA + 0x6000 + i * 4 + 0xb0, DATA + 0x6100 + i * 0x20);
                    let ready = (seed + i * 7) % 2 == 0;
                    c.bus.write32(
                        DATA + 0x6100 + i * 0x20,
                        if ready { DATA + 0x6200 } else { 0 },
                    );
                    c.bus.write32(DATA + 0x6100 + i * 0x20 + 4, map);
                    c.bus.write32(DATA + 0x6200, 1);
                }
                c.bus.write32(
                    pause::RESTART,
                    match seed % 3 {
                        0 => 0,
                        1 => 1,
                        _ => r.next_u32(),
                    },
                );
            },
        );
    }

    #[test]
    fn game_module_invalidate_focus_matches() {
        crate::difftest::check(
            ee::s06::f_00230338__generated,
            game_module_invalidate_focus,
            200,
            &|c, seed| {
                let mut r = Rng(0x2303_3801 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                match seed % 3 {
                    0 => c.bus.write32(DATA + 0xc, 0),
                    1 => {
                        c.bus.write32(DATA + 0xc, DATA + 0x100);
                        c.bus.write32(DATA + 0x100 + 0xa4, 0);
                    }
                    _ => {
                        c.bus.write32(DATA + 0xc, DATA + 0x100);
                        c.bus.write32(DATA + 0x100 + 0xa4, DATA + 0x200);
                        c.bus.write32(DATA + 0x200 + 0xd0, r.next_u32());
                    }
                }
            },
        );
    }

    // ------------------------------------------------------------ game state

    #[test]
    fn game_module_advance_state_matches() {
        check(
            ssx3_ee::s06::f_00231320__generated,
            game_module_advance_state,
            150,
            &|c, seed| {
                let mut r = rng(seed, 0x1b);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x488, DATA + 0x3000);
                c.g[A0][0] = sx(DATA + 0x4000);
                let pending = DATA + 0x5000;
                c.bus.write32(
                    DATA + 0x4000 + 0x200,
                    if seed % 3 == 0 { 0 } else { pending },
                );
                c.bus
                    .write32(pending, if seed % 6 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(pending + 4, r.next_u32());
            },
        );
    }

    pub(crate) fn fb(x: f32) -> u32 {
        x.to_bits()
    }

    // ------------------------------------------------------- game and results

    #[test]
    fn app_module_try_exit_when_loaded_matches() {
        check(
            ssx3_ee::s06::f_00231c70__generated,
            app_module_try_exit_when_loaded,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x231c_70, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, GAME_GP + 0x2a74, DATA + 0x100);
                let progress = if seed % 2 == 0 { 100.0 } else { 1.0 };
                w(c, DATA + 0x100 + 0x38, fb(progress));
            },
        );
    }

    #[test]
    fn game_mode_man_reset_mode_matches() {
        check(
            ssx3_ee::s06::f_002382d8__generated,
            game_mode_man_reset_mode,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2382d8, 0);
                let gm = DATA + 0x1000;
                c.g[A0][0] = gm as u64;
                let mode = (seed % 4) as u32;
                w32(c, gm + 4, mode);
                let obj = DATA + 0x3000 + mode * 0x100;
                let vt = DATA + 0x5000;
                w32(c, 0x0053_6668 + mode * 4, obj);
                w32(c, obj, vt);
                slot(c, vt, 0x10, 0, EMPTY);
                for i in 0..6u32 {
                    w32(c, 0x0053_66d0 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn game_module_end_replay_playback_matches() {
        check(
            ee::s06::f_002305c8__generated,
            game_module_end_replay_playback,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x2305c8, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                w(c, this + 0x28, DATA + 0x400);
                w(c, this + 0xa4, DATA + 0x500);
                w(c, GAME_GP.wrapping_sub(0x69c), seed % 4);
            },
        );
    }

    #[test]
    fn game_module_reset_scripts_and_world_matches() {
        check(
            ee::s06::f_002300f0__generated,
            game_module_reset_scripts_and_world,
            40,
            &|c, seed| {
                let _ = base(c, seed, 0x23_00f0);
                globals(c);
                pool(c);
                heap_init_2(c);
                w(c, DATA + 0x30, DATA + 0x1000);
                w(c, DATA + 0x34, DATA + 0x2000);
                w(c, DATA + 0xc, DATA + 0x3000);
                w(c, DATA + 0x10, DATA + 0x4000);
                w(c, GAME_GP.wrapping_add(0xce8), 0);
                let _ = seed;
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn game_module_quick_restart_matches() {
        check(
            ee::s06::f_002302a8__generated,
            game_module_quick_restart,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x2302, 0);
                env(c);
                let this = DATA + 0x400;
                let race = DATA + 0x1000;
                let n = (seed % 4) as u32;
                w32(c, this + 0xc, race);
                w32(c, race + 0x78, n);
                for i in 0..n {
                    w32(c, race + 0x28 + 4 * i, DATA + 0x1800 + 0x40 * i);
                }
                w32(c, gpat(0x410), 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// A vtable slot_2 pointing at a fixed translated function.
    pub(crate) fn slot_to(c: &mut Cpu, vt: u32, off: u32, target: u32) {
        wh(c, vt + off, 0);
        w(c, vt + off + 4, target);
    }

    pub(crate) const RET_ZERO: u32 = 0x001a_8828;

    // ------------------------------------------------------ reward database

    // --------------------------------------------------------- game module

    #[test]
    fn game_module_try_exit_matches() {
        check(
            ee::s06::f_00230e98__generated,
            game_module_try_exit,
            SEEDS_5,
            &|c, seed| {
                let _ = base_2(c, seed, 0x230e_98, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                let module = obj + 0x1000;
                w(c, obj + 0x98, module);
                w(c, module, obj + 0x1100);
                if seed % 2 == 0 {
                    slot_to(c, obj + 0x1100, 0x1c, RET_ZERO);
                } else {
                    slot_2(c, obj + 0x1100, 0x1c, 0);
                }
                slot_to(c, obj + 0x1100, 0x18, RET_ZERO);
                // The world/race manager and the callback object.
                let game = W + 0x1000;
                w(c, game + 0x270, 0);
                w(c, IFACE + 0x10d8, obj + 0x2200);
                slot_2(c, obj + 0x2200, 0x78, 1);
                w(c, obj + 0x2200 + 0x7c, fakes::FAKE + 0x30);
                w(c, GAME_GP.wrapping_sub(0x84c), obj + 0x3000);
                w(c, obj + 0x94, obj + 0x3100);
            },
        );
    }

    #[test]
    fn game_module_on_section_unload_matches() {
        check(
            ee::s06::f_00230360__generated,
            game_module_on_section_unload,
            30,
            &|c, seed| {
                let _r = base_3(c, seed, 0x2303);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 16) as u64;
                c.bus.write32(gp_at(-0x6f0), DATA + 0x1000);
                c.bus.write32(gp_at(0xf00), DATA + 0x1100);
                c.bus.write32(gp_at(0xf38), DATA + 0x1200);
                c.bus.write32(DATA + 0xc, 0);
                c.bus.write32(DATA + 0x30, 0);
            },
        );
    }

    /// Fade timer negative, zero, partial and saturating (alpha 1), random
    /// render-state stack contents; the context's virtual calls and the
    /// camera call inside the real Render2D_DrawFilledRect are fakes.
    #[test]
    fn app_module_render_fade_matches() {
        covered(
            ee::s06::f_00231d60__generated,
            app_module_render_fade,
            300,
            &|c, seed| {
                c.pc = 0x0023_1d60;
                let mut r = machine(c, seed, 0x231d60, 4);
                let gp = c.g[GP][0] as u32;
                let module = DATA;
                let ctx = DATA + 0x1000;
                let stack = DATA + 0x2004;
                let r2d = DATA + 0x3000;
                let cam = DATA + 0x4000;
                let t = match seed % 6 {
                    0 => r.f32(1.0) | 0x8000_0000,
                    1 => 0,
                    2 => 0.1f32.to_bits(),
                    3 => 0.3f32.to_bits(),
                    4 => r.edge_f32(),
                    _ => r.f32(0.4),
                };
                c.bus.write32(module + FADE_TIME, t);
                c.bus.write32(gp.wrapping_sub(R2D_CTX_GP), ctx);
                let top = stack + 0x14 * (seed % 3);
                c.bus.write32(ctx + R2D_STATE_TOP, top);
                crate::difftest::fill(c, &mut r, stack, 0x14);
                crate::difftest::fill(c, &mut r, DEFAULT_STATE, 5);
                c.bus.write32(ctx + R2D_VT, DATA + 0x5000);
                slot(
                    c,
                    DATA + 0x5000,
                    VT_SET_VIEWPORT,
                    (r.next_u32() as i16) & !3,
                    FAKE,
                );
                slot(
                    c,
                    DATA + 0x5000,
                    VT_SET_TEXTURE,
                    (r.next_u32() as i16) & !3,
                    FAKE + 0x10,
                );
                c.bus.write32(gp.wrapping_sub(0x844), r2d);
                c.bus.write32(gp + LENS_CAMERA_GP, cam);
                c.bus.write32(cam + CAM_PROJECTOR_VT, DATA + 0x6000);
                slot(
                    c,
                    DATA + 0x6000,
                    0x238,
                    (r.next_u32() as i16) & !3,
                    FAKE + 0x20,
                );
                c.g[A0][0] = sx(module);
            },
        );
    }

    /// GameModule_SaveState: the two members' `serialize` and every stream
    /// write (all through the state SaveStates) are fakes (some unwind); the
    /// state objects are random words.
    #[test]
    fn game_module_save_state_matches() {
        use crate::difftest::fakes::{machine, vtable};
        use crate::difftest::{check, fill, DATA};
        const THIS: u32 = DATA + 0x2_0000;
        const STREAM: u32 = DATA + 0x2_1000;
        const MEMBERS: u32 = DATA + 0x2_2000;
        check(
            ee::s06::f_00231348__generated,
            game_module_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0023_1348, 40);
                c.pc = 0x0023_1348;
                fill(c, &mut r, THIS, 0x90);
                let vt = vtable(c, &mut r, &[0x60]);
                for k in 0..2 {
                    let m = MEMBERS + 0x100 * k;
                    fill(c, &mut r, m, 8);
                    c.bus.write32(m + 0xc, vt);
                    c.bus.write32(THIS + 0x70 + 4 * k, m);
                }
                let svt = vtable(c, &mut r, &[8]);
                c.bus.write32(STREAM, svt);
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(STREAM);
            },
        );
    }

    use crate::difftest::check_paths;
    use crate::difftest::fill;
    use crate::game::visualfx::boardwakefx::tests::gp_at as gp_at_m2;

    #[test]
    fn app_module_try_exit_matches() {
        check_paths(
            ssx3_ee::s06::f_00231cf0,
            app_module_try_exit,
            300,
            &[9, 10],
            &|c, seed| {
                let mut r = Rng(0x0c11_0009 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                let threshold = match seed % 4 {
                    0 => 0x3f80_0000,
                    1 => 0xc000_0000,
                    2 => 0,
                    _ => r.edge_f32(),
                };
                let current = if seed % 2 == 0 {
                    0x3f00_0000
                } else {
                    0x4000_0000
                };
                c.bus.write32(GAME_GP.wrapping_sub(0x51b4), threshold);
                c.bus.write32(DATA + 0x4, current);
            },
        );
    }

    /// `GameModule_ProcessStateChange` over the hold conditions, the tick
    /// (state 3 waiting on a switch), pending ids 0..17 and next ids 0..17
    /// (17: outside the `switch`), with every state method a fake (some
    /// seeds unwind at one) and `GameViewMan_UpdateSplitScreen` on its short
    /// path.
    #[test]
    fn game_module_process_state_change_matches() {
        use game_module::*;
        check(
            ee::s06::f_00230f40__generated,
            game_module_process_state_change,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x230f, 6);
                let (m, world, g, be, be_vt, svt, loader) = (
                    DATA,
                    DATA + 0x6000,
                    DATA + 0x6200,
                    DATA + 0x6400,
                    DATA + 0x6500,
                    DATA + 0x7000,
                    DATA + 0x6800,
                );
                fill(c, &mut r, m, 0x90);
                // BELibrary_GetInterface(world->be, 7): a fake interface method.
                c.bus.write32(gp_at_m2(-0x848), world);
                c.bus.write32(world + 0x78, r.next_u32());
                c.bus.write32(gp_at_m2(-0x1efc), 0);
                c.bus.write32(0x0053_08a8, be);
                c.bus.write32(be + 4, be_vt);
                c.bus.write16(be_vt + 8, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(be_vt + 0xc, FAKE + 0x40);
                // GameViewMan_UpdateSplitScreen: the view count is unchanged.
                c.bus.write32(world + 0x84, g);
                let views = r.next_u32();
                c.bus.write32(g + 0x214, views);
                c.bus.write32(m + LOADER, loader);
                c.bus.write32(loader + 0xc, views);
                // The state objects share a vtable of fakes.
                for off in STATES {
                    c.bus.write32(m + off + 0xc, svt);
                }
                for (i, slot) in [VT_ENTER, VT_UPDATE, VT_START, VT_TICK, VT_EXIT]
                    .into_iter()
                    .enumerate()
                {
                    c.bus.write16(svt + slot, (r.next_u32() & 0x3c) as u16);
                    c.bus.write32(svt + slot + 4, FAKE + 0x50 + 0x10 * i as u32);
                }
                for k in 0..16u32 {
                    c.bus.write32(PENDING_TABLE + 4 * k, PENDING_CASE0 + 8 * k);
                    c.bus.write32(NEXT_TABLE + 4 * k, NEXT_CASE0 + 0xc * k);
                }
                // Hold: the run flag, the module's hold, and gp_m2-0x6a0 bit 0x20.
                c.bus.write32(0x0053_4b30, (seed % 7 != 0) as u32);
                c.bus
                    .write32(m + HOLD, if seed % 2 == 0 { 0 } else { r.next_u32() });
                let flags = r.next_u32() & !0x20;
                c.bus.write32(
                    gp_at_m2(-0x6a0),
                    if seed % 11 == 3 { flags | 0x20 } else { flags },
                );
                let state = |r: &mut Rng| {
                    if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        m + STATES[(r.next_u32() % 16) as usize]
                    }
                };
                let cur = state(&mut r);
                c.bus.write32(m + CUR, cur);
                let pending = state(&mut r);
                c.bus.write32(m + PENDING, pending);
                let id = |r: &mut Rng| match r.next_u32() % 5 {
                    0 => 0,
                    1 => 17 + r.next_u32() % 3,
                    _ => 1 + r.next_u32() % 16,
                };
                c.bus
                    .write32(m + CUR_ID, if seed % 3 == 0 { 3 } else { id(&mut r) });
                c.bus.write32(m + NEXT_ID, id(&mut r));
                c.bus.write32(m + PENDING_ID, id(&mut r));
                c.bus.write32(
                    m + ENTERED_ID,
                    if seed % 2 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 16
                    },
                );
                c.bus.write32(m + NEXT_ARG, r.next_u32());
                c.g[A0][0] = sx(m);
                c.pc = START;
            },
        );
    }

    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's callee fakes (in the test machine's RAM) and the
    /// result code of each fake (`gmu_fake`).
    pub(crate) const GMU_CTRL: u32 = DATA + 0x2f_f000;

    pub(crate) const GMU_MAGIC: u32 = 0x6a3e_0b58;

    pub(crate) const GMU_CODES: u32 = GMU_CTRL + 0x10;

    /// Result codes: 0 / 1, a random bit per call, the fake's own pointer;
    /// anything else is returned as is (a pointer).
    pub(crate) const GMU_ZERO: u32 = 0;

    pub(crate) const GMU_ONE: u32 = 1;

    pub(crate) const GMU_BIT: u32 = 2;

    pub(crate) const GMU_HASH: u32 = 3;

    /// A logging fake (`fakes::fake`) whose `v0` the fixture picks per callee.
    pub(crate) fn gmu_fake(c: &mut Cpu, addr: u32, idx: u32) {
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
    pub(crate) const GMU_I_VIEWS_READY: u32 = 1;

    pub(crate) const GMU_I_MENU: u32 = 2;

    pub(crate) const GMU_I_PEEK: u32 = 3;

    pub(crate) const GMU_I_FIND_STATE: u32 = 4;

    pub(crate) const GMU_I_CHECK_PAUSE: u32 = 5;

    pub(crate) const GMU_I_CAN_PAUSE: u32 = 6;

    pub(crate) const GMU_I_PAUSE_OPEN: u32 = 7;

    pub(crate) const GMU_I_SYNC_READY: u32 = 8;

    pub(crate) const GMU_I_SYNC_APPLY: u32 = 9;

    pub(crate) const GMU_I_REPLAY_PAUSED: u32 = 10;

    pub(crate) const GMU_I_READ_INPUT: u32 = 11;

    pub(crate) const GMU_I_POP_FRONT: u32 = 12;

    pub(crate) const GMU_I_NEXT_SECTION: u32 = 13;

    pub(crate) const GMU_I_MISSION_2: u32 = 14;

    pub(crate) const GMU_FAKES: u32 = 57;

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
    pub(crate) const GMU_BASE: u32 = DATA + 0x20_0000;

    pub(crate) const GMU_MODULE: u32 = GMU_BASE;

    pub(crate) const GMU_VIEW_MAN: u32 = GMU_BASE + 0x1000;

    pub(crate) const GMU_SECTION_MAN: u32 = GMU_BASE + 0x2000;

    pub(crate) const GMU_WORLD_REF: u32 = GMU_BASE + 0x3000;

    pub(crate) const GMU_WORLD_OBJ: u32 = GMU_BASE + 0x3100;

    pub(crate) const GMU_RACE: u32 = GMU_BASE + 0x4000;

    pub(crate) const GMU_WSCRIPT: u32 = GMU_BASE + 0x5000;

    pub(crate) const GMU_PLAYBACK: u32 = GMU_BASE + 0x6000;

    pub(crate) const GMU_UI: u32 = GMU_BASE + 0x7000;

    pub(crate) const GMU_UI_STATE: u32 = GMU_BASE + 0x7100;

    pub(crate) const GMU_UI_VT: u32 = GMU_BASE + 0x7200;

    pub(crate) const GMU_RIDERS: u32 = GMU_BASE + 0x8000;

    pub(crate) const GMU_RIDER_LIST_VT: u32 = GMU_BASE + 0x8200;

    pub(crate) const GMU_RIDER: u32 = GMU_BASE + 0x8400;
    // + 0x800 per rider
    pub(crate) const GMU_RIDER_VT: u32 = GMU_BASE + 0x8300;

    pub(crate) const GMU_SCRIPT_ENGINE: u32 = GMU_BASE + 0xa000;

    pub(crate) const GMU_SCRIPT_ENGINE_VT: u32 = GMU_BASE + 0xa400;

    pub(crate) const GMU_AUDIO_HOST: u32 = GMU_BASE + 0xb000;

    pub(crate) const GMU_AUDIO_HOST_VT: u32 = GMU_BASE + 0xc100;

    pub(crate) const GMU_WORLD: u32 = GMU_BASE + 0xd000;

    pub(crate) const GMU_FOG: u32 = GMU_BASE + 0xd400;

    pub(crate) const GMU_PAD_QUEUE: u32 = GMU_BASE + 0xe000;

    pub(crate) const GMU_EVENT: u32 = GMU_BASE + 0x11000;

    pub(crate) const GMU_VIEWS: u32 = GMU_BASE + 0x11100;

    pub(crate) fn gmu_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    pub(crate) fn gmu_gp_w(c: &mut Cpu, off: i32, v: u32) {
        c.bus.write32(GAME_GP.wrapping_add(off as u32), v);
    }

    /// Set the result code of fake `i`.
    pub(crate) fn gmu_code(c: &mut Cpu, i: u32, v: u32) {
        c.bus.write32(GMU_CODES + 4 * i, v);
    }

    /// One of `[0, 1, random bit]`, weighted toward 0.
    pub(crate) fn gmu_bool(r: &mut Rng) -> u32 {
        match r.next_u32() % 5 {
            0 | 1 => GMU_ZERO,
            2 => GMU_ONE,
            _ => GMU_BIT,
        }
    }

    /// A vtable slot `{delta, method}` at `vt + off`.
    pub(crate) fn gmu_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, method: u32) {
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

    pub(crate) use crate::game::ai::rider::tests::w32_2;
    pub(crate) use crate::game::world::worldview::tests::oc410_fresult;
    pub(crate) use crate::game::world::worldview::tests::oc410_machine;
    pub(crate) use crate::game::world::worldview::tests::oc410_result;

    pub(crate) use crate::game::ai::ai::tests::oc414_env;

    /// `LoadIndicator_Render`: the 2D state setup, nine virtual calls on the
    /// graphics manager and the pulsing size.
    #[test]
    fn load_indicator_render_matches() {
        const GRMAN: u32 = DATA + 0x1000;
        const STATES: u32 = DATA + 0x2000;
        const WORLD: u32 = DATA + 0x3000;
        const GP: u32 = crate::difftest::GAME_GP;
        check(
            ee::s06::f_00231fc0__generated,
            load_indicator_render,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0023_1fc0, 9);
                c.pc = 0x0023_1fc0;
                let this = DATA + 0x500;
                c.g[A0][0] = this as u64;
                w32_2(c, this, r.edge_f32());
                w32_2(c, this + 4, r.edge_f32());
                w32_2(c, this + 0xc, r.edge_f32());
                w32_2(c, GP.wrapping_sub(0x854), GRMAN);
                w32_2(c, GP.wrapping_sub(0x848), WORLD);
                w32_2(c, WORLD + 0xa0, r.next_u32());
                w32_2(c, GP.wrapping_sub(0x51a4), r.edge_f32());
                w32_2(c, GP.wrapping_sub(0x51a0), r.edge_f32());
                w32_2(c, GP.wrapping_sub(0x519c), r.edge_f32());
                w32_2(c, GRMAN + 0xe84, STATES);
                for k in 0..10 {
                    w32_2(c, STATES + 4 * k, r.next_u32());
                }
                for k in 0..5 {
                    w32_2(c, 0x0050_1420 + 4 * k, r.next_u32());
                }
                let vt = fakes::vtable(
                    c,
                    &mut r,
                    &[
                        0xa8, 0xf8, 0x110, 0x50, 0x58, 0xb8, 0xd0, 0x270, 0x100, 0xb0,
                    ],
                );
                w32_2(c, GRMAN + 0x10d8, vt);
                // Width and height come from two fakes that can return negatives.
                w32_2(c, vt + 0x54, 0x0014_dc80);
                w32_2(c, vt + 0x5c, 0x0014_dd58);
                let size = |r: &mut Rng, n: u32| {
                    if n % 3 == 0 {
                        0xffff_fff0
                    } else {
                        r.next_u32() % 700
                    }
                };
                w32_2(c, oc410_result(6), size(&mut r, seed));
                w32_2(c, oc410_result(7), size(&mut r, seed / 3));
                w32_2(c, oc410_fresult(32), r.edge_f32());
            },
        );
    }

    /// `AppModule_Update` forwards to `AppModule_UpdateFade` (real callee on
    /// a zeroed module).
    #[test]
    fn app_module_update_matches() {
        check(
            ee::s06::f_00231cb0__generated,
            app_module_update,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x231cb0, 0);
                oc414_env(c);
                c.pc = 0x0023_1cb0;
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    use crate::difftest::fakes::fake;
    use ssx3_ee_rt::funcs::register_functions;
    use ssx3_ee_rt::funcs::FnEntry;

    /// `cGame_load` calls about 90 guest functions, half of them unported and
    /// all of them real code that would run on test data. The test therefore
    /// runs in a child process (the same test binary, this one test, `--exact`)
    /// whose function table holds only the stubs below: every other callee
    /// misses the table and lands in the logging fake (`fakes::fake_call`).
    /// A table swap in the shared test process would race with the other
    /// tests' `fresh()`.
    pub(crate) const GML_CHILD_ENV: &str = "SSX3_OC421_CHILD";

    /// Fixture layout (all in `DATA..DATA+0xfffff`, above the fakes' pointer
    /// range `0x800000..0x840000`).
    pub(crate) const GAME: u32 = DATA + 0x8_0000;

    pub(crate) const APP: u32 = DATA + 0x8_0800;

    pub(crate) const GRMAN: u32 = DATA + 0x8_4000;

    pub(crate) const GRMAN_VT: u32 = DATA + 0x8_6000;

    pub(crate) const AI_OBJ: u32 = DATA + 0x8_8000;

    pub(crate) const AI_VT: u32 = DATA + 0x8_9000;

    pub(crate) const SECOND_PLAYER: u32 = DATA + 0x8_9400;

    pub(crate) const GAME_COMM: u32 = DATA + 0x8_a000;

    /// Stub controls (`DATA + 0x200000` and up).
    pub(crate) const CTRL: u32 = DATA + 0x20_0000;

    /// `cBENetworkInterface_networkOpt` and `cBENewRaceInterface_newRace`.
    pub(crate) const NETWORK_OPT: u32 = 0x0053_4b30;

    pub(crate) const NEW_RACE: u32 = 0x0053_5bc8;

    /// A stub for a "ready?" query: returns 0 for the first `n` calls
    /// (`n` at `zeros_at`), then the fake's non-zero pointer.
    pub(crate) fn stub_ready(c: &mut Cpu, target: u32, zeros_at: u32, count_at: u32) {
        let ret = c.g[RA][0] as u32;
        let n = c.bus.read32(count_at);
        c.bus.write32(count_at, n + 1);
        fake(c, target, ret);
        if c.pc == ret && n < c.bus.read32(zeros_at) {
            c.g[V0][0] = 0;
        }
    }

    pub(crate) fn stub_ai_ready(c: &mut Cpu) {
        stub_ready(c, 0x0012_a180, CTRL, CTRL + 4)
    }

    pub(crate) fn stub_preload_ready(c: &mut Cpu) {
        stub_ready(c, 0x0023_12d8, CTRL + 8, CTRL + 0xc)
    }

    /// `cAI_cAI`: returns the fixture's AI object.
    pub(crate) fn stub_ai_new(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        fake(c, 0x0012_86a0, ret);
        if c.pc == ret {
            c.g[V0][0] = AI_OBJ as u64;
        }
    }

    /// `FILE_load`: a file pointer, or 0 (load failed).
    pub(crate) fn stub_file_load(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        fake(c, 0x003e_18c8, ret);
        if c.pc == ret {
            c.g[V0][0] = if c.bus.read32(CTRL + 0x10) != 0 {
                0
            } else {
                (DATA + 0x8_c000) as u64
            };
        }
    }

    pub(crate) static STUBS: [FnEntry; 4] = [
        FnEntry {
            start: 0x0012_86a0,
            end: 0x0012_86a4,
            f: stub_ai_new,
        },
        FnEntry {
            start: 0x0012_a180,
            end: 0x0012_a184,
            f: stub_ai_ready,
        },
        FnEntry {
            start: 0x0023_12d8,
            end: 0x0023_12dc,
            f: stub_preload_ready,
        },
        FnEntry {
            start: 0x003e_18c8,
            end: 0x003e_18cc,
            f: stub_file_load,
        },
    ];

    /// In the child process only: replace the function table by the stubs.
    pub(crate) fn gml_install_stub_table() {
        if std::env::var_os(GML_CHILD_ENV).is_some() {
            register_functions(&STUBS);
        }
    }

    /// The fixture for one seed: the game object in `a0`, the globals the
    /// function reads (network/replay flags, mode bytes, vtables), the stub
    /// controls; the branch picks come from the seed's random bits.
    pub(crate) fn gml_setup(c: &mut Cpu, seed: u32) {
        gml_install_stub_table();
        let mut r: Rng = machine(c, seed, 0x22_ebc8, 100);
        let bits = r.next_u32();
        let gp = crate::difftest::GAME_GP;
        c.pc = GAME_LOAD_ENTRY;
        c.g[A0][0] = GAME as u64;
        // The application object (gp - 0x848): clock step, back end, fonts, ...
        c.bus.write32(gp.wrapping_sub(0x848), APP);
        c.bus.write32(APP + 0x14, 0x3c88_8889); // 1/60
        for off in [0x78u32, 0x88, 0x8c, 0xb0, 0xc0] {
            c.bus.write32(APP + off, r.next_u32());
        }
        // Graphics manager (gp - 0x854) and its vtable slots (delta, method).
        c.bus.write32(gp.wrapping_sub(0x854), GRMAN);
        c.bus.write32(GRMAN + 0x10d8, GRMAN_VT);
        for off in [0x160u32, 0x170] {
            c.bus.write16(GRMAN_VT + off, (r.next_u32() & 0xfc) as u16);
            c.bus
                .write32(GRMAN_VT + off + 4, if bits & 1 == 0 { 0x4000 } else { 0 });
        }
        // Game comm (gp - 0x204), network options, replay flag.
        let comm = bits & 2 != 0;
        c.bus
            .write32(gp.wrapping_sub(0x204), if comm { GAME_COMM } else { 0 });
        c.bus.write32(GAME_COMM + 0x120, 2 + (bits >> 8) % 6);
        let network = bits & 4 != 0;
        c.bus.write32(NETWORK_OPT, network as u32);
        c.bus.write32(NETWORK_OPT + 8, (bits & 8 != 0) as u32); // bIsHost
        c.bus
            .write32(NETWORK_OPT + 0x10, 0x4220_0000 + (r.next_u32() & 0xffff)); // latency
        let replay = bits & 0x10 != 0;
        c.bus.write32(gp.wrapping_sub(0x1ec8), replay as u32);
        c.bus.write32(gp + 0x394, r.next_u32());
        // Race set-up bytes: mode, flag, sub mode.
        let modes: [(u8, u8); 8] = [
            (4, 0),
            (5, 4),
            (5, 1),
            (6, 5),
            (6, 2),
            (0, 0),
            (3, 1),
            (5, 5),
        ];
        let (mode, sub) = modes[((bits >> 5) & 7) as usize];
        c.bus.write8(NEW_RACE + 0x48, mode);
        c.bus.write8(NEW_RACE + 0x49, (bits >> 9 & 1) as u8);
        c.bus.write8(NEW_RACE + 0x4a, sub);
        c.bus.write32(0x0053_5bfc, 1 + (bits >> 10) % 4); // player count
                                                          // AI object: vtable slot 2, "already has actors", second player.
        c.bus.write32(AI_OBJ + 0xcc, AI_VT);
        c.bus.write16(AI_VT + 0x10, 0);
        c.bus.write32(AI_VT + 0x14, 0x4000);
        c.bus.write32(AI_OBJ + 0x80, (bits >> 12 & 1) as u32);
        c.bus.write32(
            AI_OBJ + 0x44,
            if bits >> 13 & 1 != 0 {
                SECOND_PLAYER
            } else {
                0
            },
        );
        c.bus.write32(SECOND_PLAYER + 0x18, r.next_u32());
        // Stub controls: loops that go round 0..2 times, file load failing.
        c.bus.write32(CTRL, (bits >> 14) % 3);
        c.bus.write32(CTRL + 8, (bits >> 16) % 3);
        c.bus.write32(
            CTRL + 0x10,
            (bits >> 18 & 1 != 0 && bits >> 19 & 1 != 0) as u32,
        );
    }

    pub(crate) fn gml_run_checked() {
        covered(
            ssx3_ee::s05::f_0022ebc8__generated,
            game_module_load,
            160,
            &gml_setup,
        );
    }

    /// `cGame_load` against the generated body: registers (strict: scratch
    /// registers and the arguments of all ~90 calls), cycles and memory, over
    /// network/local games, replay/non-replay (every mode and sub-mode pair,
    /// both AI-actor starts), the wait loops (0-2 rounds), a failing menu file
    /// load and unwinds at random call indices.
    #[test]
    fn game_module_load_matches() {
        if std::env::var_os(GML_CHILD_ENV).is_some() {
            return gml_run_checked();
        }
        let name = format!(
            "{}::game_module_load_matches",
            module_path!().split_once("::").map_or("", |p| p.1)
        );
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([name.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(GML_CHILD_ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child run of {name}:\n{text}"
        );
    }

    pub(crate) use crate::game::be::belibrary::tests::oc431_install;
    pub(crate) use crate::game::be::belibrary::tests::oc431_machine;

    pub(crate) use crate::game::be::belibrary::tests::OC431_BASE;

    /// `GameModule_CreateSectionScriptTable`: sections whose entry has a kind
    /// below 2 or no scripts are skipped; otherwise the local table is created
    /// (a present one is destroyed first). Allocation and the Luno table
    /// constructor/destructor are fakes.
    #[test]
    fn game_module_create_section_script_table_matches() {
        oc431_install();
        covered(
            ee::s06::f_00230430__generated,
            game_module_create_section_script_table,
            200,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x230430, 3, 0x0023_0430);
                let (root, mid, table, wman) = (
                    OC431_BASE + 0x100,
                    OC431_BASE + 0x200,
                    OC431_BASE + 0x300,
                    OC431_BASE + 0x1000,
                );
                let id = seed % 6;
                c.g[A1][0] = id as u64;
                c.bus.write32(GAME_GP + 0x16c8, root);
                c.bus.write32(root + 4, mid);
                c.bus.write32(mid + 4, table);
                let kind = [0u32, 1, 2, 5][(r.next_u32() % 4) as usize];
                c.bus.write16(table + 12 * id, kind as u16);
                let scripts = [0u32, 3, 1][(r.next_u32() % 3) as usize];
                c.bus.write32(table + 12 * id + 8, scripts);
                c.bus.write32(GAME_GP + 0xce8, wman);
                let present = if r.next_u32() & 1 == 0 {
                    0
                } else {
                    OC431_BASE + 0x2000
                };
                c.bus.write32(wman + 0x3cc + 4 * id, present);
            },
        );
    }

    /// A vtable at `vt` whose slots `offs` hold a zero `this` adjustment and
    /// a fake method (the miss hook logs the call).
    pub(crate) fn put_vtable(c: &mut Cpu, vt: u32, offs: &[u32]) {
        for (i, &off) in offs.iter().enumerate() {
            c.bus.write16(vt + off, 0);
            c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
    }

    /// Owned objects with virtual deleters: (self field, object, the offset
    /// of its vtable pointer, the vtable).
    pub(crate) const OWNED: [(u32, u32, u32, u32); 6] = [
        (0x0c, DATA + 0x2_0000, 0xcc, DATA + 0x3_4000),
        (0x14, DATA + 0x2_1000, 0xc, DATA + 0x3_5000),
        (0x18, DATA + 0x2_2000, 0x4, DATA + 0x3_6000),
        (0x1c, DATA + 0x2_3000, 0x649c, DATA + 0x3_7000),
        (0x30, DATA + 0x3_0000, 0x494, DATA + 0x3_8000),
        (0x98, DATA + 0x3_1000, 0x0, DATA + 0x3_9000),
    ];

    #[test]
    fn game_module_unload_matches() {
        check(
            ssx3_ee::s06::f_0022fa98__generated,
            game_module_unload,
            200,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x0022_fa98, 0);
                c.pc = 0x0022_fa98;
                c.g[RA][0] = crate::difftest::RA as u64;
                let this = DATA + 0x8000;
                c.g[A0][0] = this as u64;
                // SSXApp, its BE and the back end's interface table.
                let ssx = DATA + 0x3000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), ssx);
                c.bus.write32(ssx + 0x78, DATA + 0x1000);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x1000);
                c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
                let be_vt = DATA + 0x1200;
                put_vtable(c, be_vt, &[8]);
                c.bus.write32(0x0053_08a8, DATA + 0x1100);
                c.bus.write32(DATA + 0x1104, be_vt);
                // GrMan and its function block (flush, purge, remove-texture).
                let gr = DATA + 0x5000;
                let st = DATA + 0x6000;
                put_vtable(c, st, &[0x98, 0x190, 0x388]);
                c.bus.write32(gr + 0x10d8, st);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), gr);
                // The singletons the purges reach (empty).
                c.bus.write32(GAME_GP.wrapping_sub(0x84c), DATA + 0x7000);
                c.bus.write32(GAME_GP.wrapping_add(0x410), DATA + 0x7800);
                c.bus.write32(GAME_GP.wrapping_add(0x21e4), DATA + 0x7c00);
                c.bus.write32(GAME_GP.wrapping_add(0x2a74), DATA + 0x7a00);
                // The owned objects with virtual deleters; everything else empty.
                for (field, obj, vt_at, vt) in OWNED {
                    put_vtable(c, vt, &[8]);
                    c.bus.write32(obj + vt_at, vt);
                    c.bus.write32(this + field, obj);
                }
            },
        );
    }

    use crate::difftest::fakes::*;

    pub(crate) use crate::game::main::gamestate::tests::SEEDS_6;
    use ssx3_ee_rt::Cpu;

    #[test]
    fn game_module_cancel_overlay_state_matches() {
        covered(
            ssx3_ee::s06::f_00231280__generated,
            super::game_module_cancel_overlay_state,
            SEEDS_6,
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

    /// cSectionMan: the location (`idx`) and mode (7 = the table's mode)
    /// across the seeds, on the game's own location tables.

    /// cGame::partialRestart: the AI's virtual slot 5 is a fake (the mode-3
    /// call goes to it); every other callee sees the zeroed game object.

    /// cGame::startReplay: the replay counter in gp-0x69c steps up from
    /// seed-dependent values; the replay pointer slot is seeded.

    /// cLoadScreen::drawSplash: the draw context at gp-0x854 has its virtual
    /// draw method at a fake, so the call is logged and returns.
    #[test]
    fn c_load_screen_draw_splash_matches() {
        check(
            ssx3_ee::s06::f_00231b50__generated,
            c_load_screen_draw_splash,
            200,
            &|c, seed| {
                machine(c, seed, 0x0023_1b50, 0);
                c.g[A1][0] = (seed % 0x1000) as u64;
                c.bus.write32(GAME_GP - 0x854, DATA);
                c.bus.write32(DATA + 0xe84, DATA + 0x1000);
                c.bus.write32(DATA + 0x10d8, DATA + 0x2000);
                c.bus.write16(DATA + 0x2238, ((seed % 3) * 0x10) as u16);
                c.bus.write32(DATA + 0x223c, FAKE);
            },
        );
    }

    use std::ptr::addr_of_mut;

    pub(crate) use crate::game::luno::lunovm::oc681_luno_table_find_int;
    pub(crate) use crate::game::luno::lunovm::oc681_luno_table_read;
    pub(crate) use crate::game::luno::lunovm::oc681_luno_vm_register_add;
    pub(crate) use crate::game::main::game::oc681_section_man_request_event;
    pub(crate) use crate::game::main::gamestate::oc681_game_state_restart_race_enter;
    pub(crate) use crate::game::main::gamestate::oc681_game_state_warp_to_event_enter;

    /// Set in the child process that runs one of these tests: the function
    /// table is replaced by [`OC681_STUBS`] (every other call is a logged
    /// fake) and the direct callees' override slots hold fakes. Both are
    /// process-wide, so they never run in the shared test process.
    pub(crate) const OC681_CHILD_ENV: &str = "SSX3_OC681_CHILD";

    /// Fixture area, clear of the fakes' pointer results (`DATA..DATA+0xfffff`)
    /// and their log (`DATA+0x100000`).
    pub(crate) const OC681_BASE: u32 = DATA + 0x20_0000;

    /// Return values for the two stubbed callees.
    pub(crate) const OC681_RET_FIND: u32 = OC681_BASE + 0x8f00;

    pub(crate) const OC681_RET_QUERY: u32 = OC681_BASE + 0x8f04;

    /// `SSXApp`, its game object, the AI object and their vtables.
    pub(crate) const OC681_APP: u32 = OC681_BASE + 0x1000;

    pub(crate) const OC681_GAME: u32 = OC681_BASE + 0x2000;

    pub(crate) const OC681_AI: u32 = OC681_BASE + 0x3000;

    pub(crate) const OC681_AI_VT: u32 = OC681_BASE + 0x4000;

    pub(crate) const OC681_MODE_MAN: u32 = OC681_BASE + 0x5000;

    pub(crate) const OC681_HELD: u32 = OC681_BASE + 0x8000;

    pub(crate) const OC681_HELD_VT: u32 = OC681_BASE + 0x8100;

    /// The new-race record (`cBENewRaceInterface_newRace`).
    pub(crate) const OC681_NEW_RACE: u32 = 0x0053_5bc8;

    /// Runs `run` in a child process (this test, by name, with the env set).
    macro_rules! oc681_child {
        ($test:ident, $run:path) => {{
            if std::env::var_os(OC681_CHILD_ENV).is_some() {
                $run();
            } else {
                let name = format!(
                    "{}::{}",
                    module_path!().split_once("::").map_or("", |p| p.1),
                    stringify!($test)
                );
                oc681_spawn(&name);
            }
        }};
    }

    pub(crate) fn oc681_spawn(name: &str) {
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([name, "--exact", "--test-threads=1", "--nocapture"])
            .env(OC681_CHILD_ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child run of {name}:\n{text}"
        );
    }

    /// A stub that returns the word the fixture left at `at`, after logging.
    pub(crate) fn oc681_stub_value(c: &mut Cpu, target: u32, at: u32) {
        let ret = c.g[RA][0] as u32;
        fake(c, target, ret);
        if c.pc == ret {
            c.g[V0][0] = c.bus.read32(at) as u64;
        }
    }

    pub(crate) fn oc681_stub_find(c: &mut Cpu) {
        oc681_stub_value(c, 0x0039_f9d8, OC681_RET_FIND)
    }

    pub(crate) fn oc681_stub_query(c: &mut Cpu) {
        oc681_stub_value(c, 0x0027_8da0, OC681_RET_QUERY)
    }

    pub(crate) static OC681_STUBS: [FnEntry; 2] = [
        FnEntry {
            start: 0x0027_8da0,
            end: 0x0027_8da4,
            f: oc681_stub_query,
        },
        FnEntry {
            start: 0x0039_f9d8,
            end: 0x0039_f9dc,
            f: oc681_stub_find,
        },
    ];

    /// In the child only: every call but the two stubs is a logged fake.
    /// (`fresh()` registers the real table again for each machine.)
    pub(crate) fn oc681_use_stubs() {
        if std::env::var_os(OC681_CHILD_ENV).is_some() {
            register_functions(&OC681_STUBS);
        }
    }

    macro_rules! oc681_slot_fake {
        ($name:ident, $addr:expr) => {
            fn $name(c: &mut Cpu) {
                fakes::fake_call(c, $addr)
            }
        };
    }

    pub(crate) fn oc681_hook(slot: *mut fn(&mut Cpu), f: fn(&mut Cpu)) {
        fakes::hook(slot, f, fakes::prev_cell(f));
    }

    oc681_slot_fake!(oc681_fk_as_id, 0x0022_6620);

    oc681_slot_fake!(oc681_fk_as_float, 0x0022_6618);

    oc681_slot_fake!(oc681_fk_as_int, 0x0022_6610);

    oc681_slot_fake!(oc681_fk_dtor, 0x0022_5b90);

    oc681_slot_fake!(oc681_fk_read, 0x0022_5ca0);

    oc681_slot_fake!(oc681_fk_set, 0x0022_5338);

    oc681_slot_fake!(oc681_fk_unload, 0x0022_d598);

    oc681_slot_fake!(oc681_fk_load, 0x0022_d3d8);

    oc681_slot_fake!(oc681_fk_event_switch, 0x0022_d640);

    oc681_slot_fake!(oc681_fk_request_state, 0x0023_1250);

    oc681_slot_fake!(oc681_fk_pre_game, 0x0023_2720);

    oc681_slot_fake!(oc681_fk_pre_fe, 0x0023_2738);

    oc681_slot_fake!(oc681_fk_reset_mode, 0x0023_82d8);

    oc681_slot_fake!(oc681_fk_start_mode, 0x0023_8160);

    oc681_slot_fake!(oc681_fk_transition_man, 0x0023_0698);

    pub(crate) fn oc681_hook_luno() {
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00226620), oc681_fk_as_id);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00226618), oc681_fk_as_float);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00226610), oc681_fk_as_int);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00225B90), oc681_fk_dtor);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00225CA0), oc681_fk_read);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00225338), oc681_fk_set);
    }

    /// Writes a gcc vtable slot `{delta, fn}` at `vt + off`.
    pub(crate) fn oc681_vslot(c: &mut Cpu, vt: u32, off: u32, delta: u32, f: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, f);
    }

    /// The objects every game-module function reaches: `SSXApp` at
    /// `gp-0x848` with its game (+0x84), mode manager (+0xc0), and the game's
    /// AI object (+0xc) whose vtable slot 0x28 is a fake method.
    pub(crate) fn oc681_world(c: &mut Cpu, flags: u32) {
        let gp = GAME_GP;
        c.bus.write32(gp.wrapping_sub(0x848), OC681_APP);
        c.bus.write32(OC681_APP + 0x84, OC681_GAME);
        c.bus.write32(OC681_APP + 0xc0, OC681_MODE_MAN);
        c.bus.write32(OC681_GAME + 0xc, OC681_AI);
        c.bus.write32(OC681_AI + 0xcc, OC681_AI_VT);
        oc681_vslot(c, OC681_AI_VT, 0x28, 0, fakes::FAKE);
        c.bus.write32(OC681_HELD + 8, OC681_HELD_VT);
        oc681_vslot(c, OC681_HELD_VT, 0xc0, 0, fakes::FAKE + 0x10);
        c.bus
            .write32(OC681_RET_FIND, if flags & 1 == 0 { 0 } else { OC681_HELD });
        c.bus.write32(OC681_RET_QUERY, flags >> 1 & 1);
    }

    /// `cLunoTable_FindInt(self, key)`: finds the node whose index equals the
    /// integer `key` in the hash chain at `key % modulus` (nil when none).
    /// Fixture: random chains of 0-3 nodes of every index type; the key
    /// register on the stack is reloaded as integer or float. `AsId` and the
    /// destructor are fakes.
    pub(crate) fn oc681_luno_table_find_int_run() {
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00226620), oc681_fk_as_id);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00225B90), oc681_fk_dtor);
        covered(
            ee::s05::f_002250e0__generated,
            oc681_luno_table_find_int,
            240,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0022_50e0, 4);
                c.pc = 0x0022_50e0;
                let (this, table, nodes) =
                    (OC681_BASE + 0x100, OC681_BASE + 0x200, OC681_BASE + 0x1000);
                let key = r.next_u32() % 7;
                let modulus = 1 + r.next_u32() % 4;
                c.bus.write32(this, table);
                c.bus.write32(table + 0xc, modulus);
                c.bus.write32(table + 0x10, nodes);
                for i in 0..6u32 {
                    let n = nodes + 0x20 * i;
                    let idx_type = r.next_u32() % 5;
                    let idx_word = match r.next_u32() % 3 {
                        0 => key,
                        1 => (key as f32).to_bits(),
                        _ => r.next_u32(),
                    };
                    c.bus.write32(n, idx_word);
                    c.bus.write32(n + 8, idx_type);
                    c.bus.write32(n + 0xc, r.next_u32());
                    // The head nodes of the chains mostly hold values.
                    let val_type = if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 3
                    };
                    c.bus.write32(n + 0x14, val_type);
                    let next = if i < 5 && r.next_u32() % 2 == 0 {
                        nodes + 0x20 * (i + 1 + r.next_u32() % (5 - i))
                    } else {
                        0
                    };
                    c.bus
                        .write32(n + 0x18, if next > nodes + 0x20 * 5 { 0 } else { next });
                }
                let stack_key = match r.next_u32() % 3 {
                    0 => key,
                    1 => (key as f32).to_bits(),
                    _ => r.next_u32(),
                };
                c.bus.write32(crate::difftest::SP - 0x90, stack_key);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = key as u64;
            },
        );
    }

    #[test]
    fn oc681_luno_table_find_int_matches() {
        oc681_child!(
            oc681_luno_table_find_int_matches,
            oc681_luno_table_find_int_run
        );
    }

    /// `cLunoTable_Read(self, stream, base)`: destroys the old nodes, reads
    /// the 8-byte table header and the entry count through the stream's
    /// vtable, allocates a node array and sets every (key, value) read back.
    /// Fixture: 0-2 old nodes with virtual deleters (or none), moduli -1..2,
    /// entry counts -1..2; the stream, allocator, register reader and table
    /// set are fakes.
    pub(crate) fn oc681_luno_table_read_run() {
        oc681_hook_luno();
        covered(
            ee::s05::f_002259a8__generated,
            oc681_luno_table_read,
            240,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0022_59a8, 14);
                c.pc = 0x0022_59a8;
                let (this, table, old, node_vt) = (
                    OC681_BASE + 0x100,
                    OC681_BASE + 0x200,
                    OC681_BASE + 0x1000,
                    OC681_BASE + 0x3000,
                );
                let (stream, stream_vt) = (OC681_BASE + 0x400, OC681_BASE + 0x500);
                c.bus.write32(this, table);
                let have_old = r.next_u32() % 4 != 0;
                c.bus.write32(table + 0x10, if have_old { old } else { 0 });
                c.bus.write32(old - 0x10, r.next_u32() % 3);
                for i in 0..3u32 {
                    c.bus.write32(old + 0x20 * i + 0x1c, node_vt);
                }
                oc681_vslot(
                    c,
                    node_vt,
                    8,
                    (r.next_u32() & 0x7c) & !3,
                    fakes::FAKE + 0x20,
                );
                c.bus.write32(
                    table + 0xc,
                    [u32::MAX, 0, 1, 2][(r.next_u32() % 4) as usize],
                );
                c.bus.write32(stream, stream_vt);
                oc681_vslot(
                    c,
                    stream_vt,
                    0x10,
                    (r.next_u32() & 0x3c) & !3,
                    fakes::FAKE + 0x10,
                );
                c.bus.write32(
                    crate::difftest::SP - 0x90,
                    [u32::MAX, 0, 1, 2][(r.next_u32() % 4) as usize],
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    #[test]
    fn oc681_luno_table_read_matches() {
        oc681_child!(oc681_luno_table_read_matches, oc681_luno_table_read_run);
    }

    /// `LunoVMRegister_Add(out, lhs, rhs)`: adds two script values: same
    /// types add (int, float, index; others give nil), float + int and
    /// int + float mix, everything else is nil. Fixture: every pair of types
    /// 0..4 with integer and float words; the `As*` accessors are fakes.
    pub(crate) fn oc681_luno_vm_register_add_run() {
        oc681_hook_luno();
        covered(
            ee::s05::f_00226628__generated,
            oc681_luno_vm_register_add,
            250,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0022_6628, 2);
                c.pc = 0x0022_6628;
                let (out, lhs, rhs) = (OC681_BASE + 0x100, OC681_BASE + 0x200, OC681_BASE + 0x300);
                let lt = seed % 5;
                let rt = if seed % 2 == 0 { lt } else { (seed / 5) % 5 };
                let word = |r: &mut crate::difftest::Rng| match r.next_u32() % 3 {
                    0 => r.next_u32() % 100,
                    1 => r.edge_f32(),
                    _ => r.next_u32(),
                };
                let (lw, rw) = (word(&mut r), word(&mut r));
                c.bus.write32(lhs, lw);
                c.bus.write32(lhs + 8, lt);
                c.bus.write32(rhs, rw);
                c.bus.write32(rhs + 8, rt);
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(lhs);
                c.g[A2][0] = sx(rhs);
            },
        );
    }

    #[test]
    fn oc681_luno_vm_register_add_matches() {
        oc681_child!(
            oc681_luno_vm_register_add_matches,
            oc681_luno_vm_register_add_run
        );
    }

    /// `SectionMan_RequestEvent(self, idx, mode)`: unloads every BE section
    /// the location `idx` does not use (not the always-resident 43-48),
    /// records the location, loads its sections, the sky and the transport
    /// section, then starts the event switch for `mode` (mode 7 maps through
    /// the location-mode table). Fixture: random locations in the static
    /// location map (0-6 sections, sky/transport present or -1).
    pub(crate) fn oc681_section_man_request_event_run() {
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_0022D598), oc681_fk_unload);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_0022D3D8), oc681_fk_load);
        oc681_hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D640),
            oc681_fk_event_switch,
        );
        covered(
            ee::s05::f_0022cea8__generated,
            oc681_section_man_request_event,
            120,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0022_cea8, 30);
                c.pc = 0x0022_cea8;
                let idx = seed % 4;
                let entry = 0x0044_2488 + 0x28 * idx;
                c.bus.write32(entry + 4, r.next_u32() % 7);
                let sky = [u32::MAX, 5, 0x31, 44][(r.next_u32() % 4) as usize];
                c.bus.write32(entry + 8, sky);
                c.bus
                    .write32(entry + 0xc, [u32::MAX, 7, 0][(r.next_u32() % 3) as usize]);
                for j in 0..6u32 {
                    let id = match r.next_u32() % 4 {
                        0 => 43 + r.next_u32() % 6,
                        _ => r.next_u32() % 50,
                    };
                    c.bus.write32(entry + 0x10 + 4 * j, id);
                }
                // `mLocationModeMap[idx].mode`.
                c.bus.write32(0x0044_2820 + 8 * idx + 4, r.next_u32() % 16);
                c.g[A0][0] = sx(OC681_BASE + 0x100);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    7
                } else {
                    (r.next_u32() % 16) as u64
                };
            },
        );
    }

    #[test]
    fn oc681_section_man_request_event_matches() {
        oc681_child!(
            oc681_section_man_request_event_matches,
            oc681_section_man_request_event_run
        );
    }

    /// `GameModule_ResetRace(self)`: resets the race for a restart: playback,
    /// buckets, AI (virtual call), avalanche, views, world, replay, script
    /// engine, camera, crowds, particles, the HUD state (when found: virtual
    /// call), scripts, audio, then comm/voip and the load indicator.
    /// Every callee is a logged fake; the HUD lookup returns an object or 0.
    pub(crate) fn oc681_game_module_reset_race_run() {
        covered(
            ee::s06::f_00230180__generated,
            oc681_game_module_reset_race,
            120,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0023_0180, 30);
                oc681_world(c, seed);
                c.pc = 0x0023_0180;
                for off in [0x10u32, 0x28, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x48, 0x94] {
                    c.bus.write32(OC681_GAME + off, r.next_u32());
                }
                c.g[A0][0] = sx(OC681_GAME);
            },
        );
    }

    #[test]
    fn oc681_game_module_reset_race_matches() {
        oc681_child!(
            oc681_game_module_reset_race_matches,
            oc681_game_module_reset_race_run
        );
    }

    /// `GameModule_QuitToLoadScreen(self, toFrontEnd)`: requests state 6,
    /// leaves the comm session and voip, then makes a pre-game or a pre-FE
    /// load screen the next module. Not covered: a live game comm
    /// (`gp-0x204 != 0`) runs the unported, unbounded `cGameComm_QuitRace`
    /// on test data, so the fixture keeps it at 0.
    pub(crate) fn oc681_game_module_quit_to_load_screen_run() {
        oc681_hook(
            addr_of_mut!(ee::s06::OVERRIDE_00231250),
            oc681_fk_request_state,
        );
        oc681_hook(addr_of_mut!(ee::s06::OVERRIDE_00232720), oc681_fk_pre_game);
        oc681_hook(addr_of_mut!(ee::s06::OVERRIDE_00232738), oc681_fk_pre_fe);
        covered(
            ee::s06::f_00230488__generated,
            oc681_game_module_quit_to_load_screen,
            100,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0023_0488, 9);
                oc681_world(c, seed);
                c.pc = 0x0023_0488;
                c.bus.write32(GAME_GP.wrapping_sub(0x204), 0);
                c.bus.write32(
                    GAME_GP + 0x304,
                    if seed & 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(OC681_GAME + 0x28, r.next_u32());
                c.g[A0][0] = sx(OC681_GAME);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn oc681_game_module_quit_to_load_screen_matches() {
        oc681_child!(
            oc681_game_module_quit_to_load_screen_matches,
            oc681_game_module_quit_to_load_screen_run
        );
    }

    /// `GameModule_BeginReplayPlayback(self)`: pushes pause state 8 on the
    /// pause stack (index at `gp-0x69c`), copies the flags word, starts the
    /// replay and activates the menu system.
    pub(crate) fn oc681_game_module_begin_replay_playback_run() {
        covered(
            ee::s06::f_00230558__generated,
            oc681_game_module_begin_replay_playback,
            100,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0023_0558, 2);
                c.pc = 0x0023_0558;
                c.bus.write32(GAME_GP.wrapping_sub(0x69c), r.next_u32() % 4);
                c.bus.write32(0x0044_2910, r.next_u32());
                c.bus.write32(OC681_GAME + 0x28, r.next_u32());
                c.bus.write32(OC681_GAME + 0xa4, r.next_u32());
                c.g[A0][0] = sx(OC681_GAME);
            },
        );
    }

    #[test]
    fn oc681_game_module_begin_replay_playback_matches() {
        oc681_child!(
            oc681_game_module_begin_replay_playback_matches,
            oc681_game_module_begin_replay_playback_run
        );
    }

    /// `bool GameModule_CanRestartFromPause(self)`: false in the pending
    /// states 5, 14, 1, 10; otherwise whether the script engine allows the
    /// pause menu and the current state is not one of those. Fixture: both
    /// state words over the listed states, the script query returning 0/1.
    pub(crate) fn oc681_game_module_can_restart_from_pause_run() {
        covered(
            ee::s06::f_00231ab8__generated,
            oc681_game_module_can_restart_from_pause,
            160,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0023_1ab8, 1);
                oc681_world(c, seed >> 3);
                c.pc = 0x0023_1ab8;
                let states = [0u32, 5, 14, 1, 10, 2, 7];
                c.bus
                    .write32(OC681_GAME + 0x210, states[(seed % 7) as usize]);
                c.bus
                    .write32(OC681_GAME + 0x214, states[(r.next_u32() % 7) as usize]);
                c.g[A0][0] = sx(OC681_GAME);
            },
        );
    }

    #[test]
    fn oc681_game_module_can_restart_from_pause_matches() {
        oc681_child!(
            oc681_game_module_can_restart_from_pause_matches,
            oc681_game_module_can_restart_from_pause_run
        );
    }

    /// `GameStateRestartRace_Enter(self)`: resets the game mode and race,
    /// then (for a heat that is neither mode 5/6 nor flagged) re-adds the AI
    /// riders, restarts the replay/camera and sets the state to 0, or sets
    /// state 3 straight away. Every callee but `GameModeMan_ResetMode` and
    /// `GameModule_ResetRace` (both direct) is a logged fake.
    pub(crate) fn oc681_game_state_restart_race_enter_run() {
        oc681_hook(
            addr_of_mut!(ee::s06::OVERRIDE_002382D8),
            oc681_fk_reset_mode,
        );
        covered(
            ee::s06::f_00235aa0__generated,
            oc681_game_state_restart_race_enter,
            160,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0023_5aa0, 40);
                oc681_world(c, seed);
                c.pc = 0x0023_5aa0;
                let modes = [5u8, 6, 3, 0];
                c.bus
                    .write8(OC681_NEW_RACE + 0x48, modes[(seed % 4) as usize]);
                c.bus.write8(OC681_NEW_RACE + 0x49, ((seed >> 2) & 1) as u8);
                // The AI count (+0x10 - +0x14) and the per-AI stride.
                let n = r.next_u32() % 5;
                c.bus.write32(OC681_MODE_MAN + 0x10, n + 1);
                c.bus.write32(OC681_MODE_MAN + 0x14, 1);
                c.bus.write32(OC681_MODE_MAN + 0x98, r.next_u32() % 2);
                for i in 0..8u32 {
                    c.bus.write32(OC681_MODE_MAN + 0x18 + 4 * i, r.next_u32());
                    c.bus.write32(OC681_MODE_MAN + 0x40 + 4 * i, r.next_u32());
                }
                let (ai_man, ai_man2) = (OC681_BASE + 0x7000, OC681_BASE + 0x7100);
                c.bus.write32(GAME_GP.wrapping_sub(0x488), ai_man);
                c.bus.write32(ai_man + 0xc, ai_man2);
                c.bus.write32(ai_man2 + 0x7c, 1 + r.next_u32() % 2);
                c.g[A0][0] = sx(OC681_GAME);
            },
        );
    }

    #[test]
    fn oc681_game_state_restart_race_enter_matches() {
        oc681_child!(
            oc681_game_state_restart_race_enter_matches,
            oc681_game_state_restart_race_enter_run
        );
    }

    /// `GameStateWarpToEvent_Enter(self)`: fails the active mission, starts
    /// game mode 0xc, partially restarts the race, puts the rider at the
    /// start point and starts a white fade-in transition (a new
    /// `cFade_TObj` with colour 1.0 x4). Direct callees `StartMode`,
    /// `ResetRace`, `GetTransitionMan` and the rest are fakes.
    pub(crate) fn oc681_game_state_warp_to_event_enter_run() {
        oc681_hook(
            addr_of_mut!(ee::s06::OVERRIDE_00238160),
            oc681_fk_start_mode,
        );
        oc681_hook(
            addr_of_mut!(ee::s06::OVERRIDE_00230698),
            oc681_fk_transition_man,
        );
        covered(
            ee::s06::f_00236058__generated,
            oc681_game_state_warp_to_event_enter,
            80,
            &|c, seed| {
                oc681_use_stubs();
                let _r = machine(c, seed, 0x0023_6058, 24);
                oc681_world(c, seed);
                c.pc = 0x0023_6058;
                c.g[A0][0] = sx(OC681_GAME);
            },
        );
    }

    #[test]
    fn oc681_game_state_warp_to_event_enter_matches() {
        oc681_child!(
            oc681_game_state_warp_to_event_enter_matches,
            oc681_game_state_warp_to_event_enter_run
        );
    }

    #[test]
    fn game_module_load_state_matches() {
        check(
            ee::s06::f_002314d0__generated,
            game_module_load_state,
            200,
            &|c, seed| {
                machine(c, seed, 0x14d0, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x002314d0;
            },
        );
    }

    /// `AppModule_Unload` (0x001d_c930) against the generated body.
    #[test]
    fn app_module_unload_001dc930_matches() {
        check(
            ee::s04::f_001dc930__generated,
            app_module_unload_001dc930,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dc930, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AppModule_Enter` (0x001d_c938) against the generated body.
    #[test]
    fn app_module_enter_001dc938_matches() {
        check(
            ee::s04::f_001dc938__generated,
            app_module_enter_001dc938,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dc938, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
