//! `src/fe/festateloadscreen.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintplayer::lh;
pub(crate) use crate::game::be::beintscore::gp as gp_2;
pub(crate) use crate::game::dirtysock::tagsunk::gp_2 as gp;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_2;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::festateloadhint::oc1009_base_minus;
pub(crate) use crate::game::fe::festateloadhint::oc1009_ea;
pub(crate) use crate::game::fe::festateloadhint::oc1009_gp_minus;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_m2;
pub(crate) use crate::game::main::gamestate::p_2 as p_m2;
pub(crate) use crate::game::main::gamestate::OC326_GAME_STATE_SAVE_STATE;
pub(crate) use crate::game::main::ssxappdtor::p;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0012_8a48, ai_set_rider_order_mode, "AI_SetRiderOrderMode"),
    (0x0023_38c8, c_fe_load_screen_enter, "cFELoadScreen_Enter"),
    (0x0023_3930, cfe_load_screen_unload, "cFELoadScreen_Unload"),
    (0x0023_39f8, c_fe_load_screen_render, "cFELoadScreen_Render"),
    (0x0023_3a70, c_fe_load_screen_update, "cFELoadScreen_Update"),
    (0x0023_3aa0, game_state_goto_next, "GameState_GotoNext"),
    (0x0023_3b60, game_state_quit_enter, "GameStateQuit_Enter"),
    (0x0023_3c10, game_state_finish_ctor, "GameStateFinish_ctor"),
    (
        0x0023_3c50,
        game_state_finish_enter,
        "GameStateFinish_Enter",
    ),
    (0x0023_4008, game_state_finish_exit, "GameStateFinish_Exit"),
    (
        0x0023_4120,
        game_state_finish_load_state,
        "GameStateFinish_LoadState",
    ),
    (
        0x0023_4188,
        game_state_pre_race_load_ctor,
        "GameStatePreRaceLoad_ctor",
    ),
    (
        0x0023_4a30,
        game_state_pre_race_load_load_state,
        "GameStatePreRaceLoad_LoadState",
    ),
    (
        0x0028_fa98,
        ssx_audio_system_stop_loading_screen_loop,
        "SSXAudioSystem_StopLoadingScreenLoop",
    ),
    (0x0023_3af0, game_state_save_state, "GameState_SaveState"),
    (
        0x0023_4990,
        game_state_pre_race_load_save_state,
        "GameStatePreRaceLoad_SaveState",
    ),
    (
        0x0023_40b8,
        game_state_finish_save_state,
        "GameStateFinish_SaveState",
    ),
    (
        0x0023_4910,
        gamestatepreraceeload_check_intro_script,
        "GameStatePreRaceLoad_CheckIntroScript",
    ),
    (
        0x0023_3b90,
        game_state_quit_save_state,
        "GameStateQuit_SaveState",
    ),
    (
        0x0023_3bd0,
        game_state_idle_save_state,
        "GameStateIdle_SaveState",
    ),
    (
        0x0023_3cd8,
        game_state_finish_update,
        "GameStateFinish_Update",
    ),
    (
        0x0023_41d0,
        game_state_pre_race_load_enter,
        "GameStatePreRaceLoad_Enter",
    ),
    (
        0x0023_43b0,
        game_state_pre_race_load_update,
        "GameStatePreRaceLoad_Update",
    ),
    (
        0x0023_4750,
        cgf_event_start_state_lose_focus,
        "GameStatePreRaceLoad_Exit",
    ),
    (0x0023_3438, fe_load_screen_load, "cFELoadScreen_Load"),
    (
        0x0023_3640,
        fe_load_screen_load_pictures,
        "cFELoadScreen_LoadPictures",
    ),
    (
        0x0023_3ad0,
        game_module_request_overlay_state_global,
        "GameModule_RequestOverlayStateGlobal",
    ),
    (0x0023_3b28, game_state_load_state, "GameState_LoadState"),
    (
        0x0023_3bb0,
        game_state_quit_load_state,
        "GameStateQuit_LoadState",
    ),
    (
        0x0023_3bf0,
        game_state_idle_load_state,
        "GameStateIdle_LoadState",
    ),
];

// ---------------------------------------------------------------------------
// Callees the generator emitted as direct calls (the readable alias, which
// dispatches through the override slot) and as cross-shard `call_indirect`s.

pub(crate) mod callee {

    pub(crate) use crate::game::bx::bxstringctor::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_2;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::GAME_MODULE_REQUEST_STATE;

    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;

    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2 as GAME_STATE_LOAD_STATE;

    pub(crate) use crate::game::main::ssxappdtor::callee::APP_MODULE_UPDATE;

    pub(crate) use crate::game::main::ssxappdtor::callee::UI_ENGINE_UPDATE;

    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
}

// ---------------------------------------------------------------------------
// Game-state constructors. Each installs two vtables back to back (only the
// second survives at the same slot), clears the global game-state pointer at
// `gp-0x488`, and fills a small set of state words.

/// `GameStateFinish *GameStateFinish_ctor(GameStateFinish *self)` (0x233c10):
/// state words `+0=7`, `+4=0`, `+0xc=0x47d5e0`, `+0x10=2`, `+0x14=0`.
pub fn game_state_finish_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2a20;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 16;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[A1][0] = 7;
    c.g[A2][0] = 2;
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 7);
    this.off(0x10).sw(c, 2);
    this.off(0x4).sw(c, 0);
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `GameStatePreRaceLoad *GameStatePreRaceLoad_ctor(GameStatePreRaceLoad *self)`
/// (0x234188): state words `+0=2`, `+4=0`, `+8=0`, `+0xc=0x47d530`,
/// `+0x10=0`, `+0x14=0`, `+0x1c=4`.
pub fn game_state_pre_race_load_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2ad0;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 18;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[A1][0] = 2;
    c.g[A2][0] = 4;
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 2);
    this.off(0x1c).sw(c, 4);
    this.off(0x4).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    this.off(0x8).sw(c, 0);
    abi::ret(c);
}

/// `void GameStateQuit_Enter(void)` (0x233b60): request game state 10 (quit).
pub fn game_state_quit_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[A0][0] = Addr(gp(c, -0x488)).lw(c);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xa;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        callee::GAME_MODULE_REQUEST_STATE.call(c, 0x0023_3b7c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameState_GotoNext(state)` 0x233aa0: if the state names a target state
/// (`*(state) != 0`), ask the game module for it with the delay at +4.
pub fn game_state_goto_next(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let target = c.lw(this);
        c.g[A1][0] = target;
        let none = target == 0;
        c.g[A3][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[A2][0] = lw_at(c, this as u64, 4);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            callee::GAME_MODULE_REQUEST_STATE_2.call(c, 0x0023_3ac0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFELoadScreen_Update(self)` (0x233a70): the front-end load screen's
/// update (same shape as the game one).
pub fn c_fe_load_screen_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(p(r32(c, S0), 0xc));
        call_ptr(c, callee::UI_ENGINE_UPDATE, 0x0023_3a88)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::APP_MODULE_UPDATE, 0x0023_3a90)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StopLoadingScreenLoop(SSXAudioSystem* s)` (0x28fa98):
/// when a loading-screen loop is playing (`s+0x62a8` >= 0), fade it out and
/// clear the index.
pub fn ssx_audio_system_stop_loading_screen_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x62a8));
        let idle = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = 1;
        if !idle {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x118));
            c.g[A0][0] = c.lw(r32(c, V0));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d8));
            callee::BANK_MONITOR_DELETE_INSTANCE.call(c, 0x0028_fac4)?;
            c.g[V0][0] = u64::MAX;
            c.sw(r32(c, S0).wrapping_add(0x62a8), c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFELoadScreen_Enter()` (0x2338c8): the front-end twin of
/// [`c_game_load_screen_enter`].
pub fn c_fe_load_screen_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = lw(c, gp_2(c, -0x854));
        frame.save_ra(c, 0x10);
        call_ptr(c, 0x0036_9ff8, 0x0023_38dc)?;
        c.g[A0][0] = lw(c, gp_2(c, -0x848));
        call_ptr(c, 0x0022_8238, 0x0023_38e4)?;
        sw(c, gp_2(c, 0x1234), 0);
        let g = lw(c, gp_2(c, -0x854));
        c.g[V0][0] = g;
        c.g[V1][0] = lw(c, p(g as u32, 0x10d8));
        c.g[A0][0] = lh(c, p(c.g[V1][0] as u32, 0x78));
        sw(c, sp(c), 0);
        sw(c, p(sp(c), 4), 0);
        c.g[A0][0] = sx((g as u32).wrapping_add(c.g[A0][0] as u32));
        sw(c, p(sp(c), 8), 0);
        c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x7c));
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = sp(c) as u64;
        call_ptr(c, target, 0x0023_3910)?;
        call_ptr(c, 0x0028_b180, 0x0023_3918)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0028_f768, 0x0023_3920)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateFinish_LoadState(self, stream)` (0x234120): run the base
/// load, then read the two 4-byte fields through stream slot 2.
pub fn game_state_finish_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0023_413c);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0023_3b28, 0x0023_413c)?;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0023_4158);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_4158)?;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0023_4174);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_4174)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

/// `AI_SetRiderOrderMode` (0x128A48): store the ordering mode and number every
/// rider (or clear all their order fields when the mode is 0).
pub fn ai_set_rider_order_mode(c: &mut Cpu) {
    c.cycles += 2;
    let enabled = c.g[A1][0] != 0;
    sw_at(c, A0, 0x74, c.g[A1][0]);
    if enabled {
        c.cycles += 3;
        c.g[V0][0] = lw_at_2(c, A0, 0x78);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if empty {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[A1][0] = low(c, A0).wrapping_add(0x28);
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, A1)).lw(c);
            sw_at(c, V0, 0xec, c.g[V1][0]);
            c.g[V1][0] = low(c, V1).wrapping_add(1);
            c.g[V0][0] = lw_at_2(c, A0, 0x78);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = low(c, A1).wrapping_add(4);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0012_8a98);
                }
                continue;
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 3;
    c.g[V0][0] = lw_at_2(c, A0, 0x78);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[A2][0] = 0;
    if empty {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[A1][0] = low(c, A0).wrapping_add(0x28);
    loop {
        c.cycles += 7;
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A2][0] = low(c, A2).wrapping_add(1);
        sw_at(c, V1, 0xec, 0);
        c.g[V0][0] = lw_at_2(c, A0, 0x78);
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = low(c, A1).wrapping_add(4);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0012_8a60);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Screen rendering.

/// The shared body of the three `Render` methods (0x232da8, 0x233390,
/// 0x2339f8): look up the app module (`gp-0x854`), call its `canRender` slot
/// (`vtable[0x88]/[0x8c]`); unless it says no, draw the fade
/// (`AppModule_RenderFade`), optionally refresh the widget (`0x397df8`), then
/// call the module's `render` slot (`vtable[0xa0]/[0xa4]`) and return 1.
pub(crate) fn render_screen(c: &mut Cpu, rets: [u32; 4]) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = lw_r(c, GP, 0u32.wrapping_sub(0x854));
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[V1][0] = lw_r(c, A1, 0x10d8);
    c.g[A0][0] = lh_r(c, V1, 0x88);
    c.g[V0][0] = lw_r(c, V1, 0x8c);
    let target = low_2(c, V0);
    c.g[RA][0] = sx(rets[0]);
    c.g[A0][0] = sx(low_2(c, A1).wrapping_add(low_2(c, A0)));
    call_ptr(c, target, rets[0])?;
    let hidden = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if hidden {
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    }
    c.cycles += 4;
    c.g[RA][0] = sx(rets[1]);
    c.g[A0][0] = c.g[S0][0];
    callee::APP_MODULE_RENDER_FADE.call(c, rets[1])?;
    let faded = c.g[V0][0] != 0;
    c.g[A1][0] = lw_r(c, GP, 0u32.wrapping_sub(0x854));
    if !faded {
        c.cycles += 3;
        c.g[RA][0] = sx(rets[2]);
        c.g[A0][0] = lw_r(c, S0, 0xc);
        call_ptr(c, 0x0039_7df8, rets[2])?;
        c.g[A1][0] = lw_r(c, GP, 0u32.wrapping_sub(0x854));
    }
    c.cycles += 6;
    c.g[V0][0] = lw_r(c, A1, 0x10d8);
    c.g[A0][0] = lh_r(c, V0, 0xa0);
    c.g[V1][0] = lw_r(c, V0, 0xa4);
    let target = low_2(c, V1);
    c.g[RA][0] = sx(rets[3]);
    c.g[A0][0] = sx(low_2(c, A1).wrapping_add(low_2(c, A0)));
    call_ptr(c, target, rets[3])?;
    c.g[V0][0] = 0x1;
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFELoadScreen_Render` (0x2339f8).
pub fn c_fe_load_screen_render(c: &mut Cpu) {
    guest(c, |c| {
        render_screen(c, [0x0023_3a20, 0x0023_3a30, 0x0023_3a40, 0x0023_3a58])
    })
}

// ---------------------------------------------------------------------------
// Game state.

/// `void GameStateFinish_Enter(GameStateFinish* self)` (0x233c50): stop the
/// replay recorder, close the UI states, stop the script list, mark the
/// loading indicator busy and flag the finished race.
///
/// Tick rate: the finish screen's delay `+0x14` (180 frames at 60 Hz) is
/// taken down by 1 per tick, so it is stored as
/// [`crate::tick::ticks_from_frames60`]`(180)`.
pub fn game_state_finish_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, -0x488)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x0023_3c6c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x28)).lw(c);
        call_ptr(c, 0x0026_fa50, 0x0023_3c6c)?;
        c.g[V0][0] = Addr(gp(c, -0x488)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lw(c);
        c.g[RA][0] = sx(0x0023_3c7c);
        c.g[A0][0] = sx(p(r32(c, A0), 0x18));
        call_ptr(c, 0x0039_f840, 0x0023_3c7c)?;
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        c.g[RA][0] = sx(0x0023_3c88);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0027_90a0, 0x0023_3c88)?;
        // The delay before the finish screen: 180 frames (3 s), counted down
        // once per tick by GameStateFinish_Update, so stored in ticks.
        c.g[V0][0] = crate::tick::ticks_from_frames60(c, 180) as u64;
        Addr(p(r32(c, S0), 0x10)).sw(c, 0);
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x4;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
        c.g[RA][0] = sx(0x0023_3ca8);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x94)).lw(c);
        call_ptr(c, 0x001f_36c0, 0x0023_3ca8)?;
        c.g[S0][0] = Addr(gp(c, -0x204)).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 5;
            c.g[RA][0] = sx(0x0023_3cbc);
            c.g[A0][0] = c.g[S0][0];
            callee::GAME_STATE_FINISH_CLEAR_FLAG40.call(c, 0x0023_3cbc)?;
            c.g[V0][0] = 0x1;
            Addr(p(r32(c, S0), 0x90)).sw(c, c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStatePreRaceLoad_LoadState(self, stream)` (0x234a30): base load
/// plus four 4-byte stream fields (0x10, 0x14, 0x1c, 0x8).
pub fn game_state_pre_race_load_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        callee::GAME_STATE_LOAD_STATE.call(c, 0x0023_4a4c)?;
        for (off, ret) in [
            (0x10u32, 0x0023_4a68u32),
            (0x14, 0x0023_4a84),
            (0x1c, 0x0023_4aa0),
            (0x8, 0x0023_4abc),
        ] {
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(off));
            c.g[A2][0] = 4;
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
            c.g[V0][0] = lw_2(c, V1, 0x14);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `GameStateFinish_Exit` (0x234008): restore the default camera on every
// rider, then arm the script engine's skip delay/list and set the finish
// state.
pub fn game_state_finish_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[S1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
        let none = (c.g[S1][0] as i64) <= 0;
        c.g[S0][0] = 0u64;
        if !none {
            c.cycles += 2;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            loop {
                c.cycles += 11;
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
                call_ptr(c, 0x00162290, 0x0023_4060)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0023_4040);
                }
            }
        }
        c.cycles += 18;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
        c.g[V0][0] = 0x2;
        c.g[A2][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x00278f68, 0x0023_4088)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x00278de8, 0x0023_4094)?;
        c.g[V0][0] = 0x7;
        c.lq(16, frame.at(c, 0x30));
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        c.lq(17, frame.at(c, 0x20));
        c.lq(18, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Load screen, avalanche init, menu cursor, transition and standings helpers.

/// `void cFELoadScreen_Unload(cFELoadScreen *self)` (0x233930): destroy the
/// screen's UI objects, release the load-screen graphics and the meter.
pub fn cfe_load_screen_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = Addr(gp_3(c, -0x854)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p_2(c.g[A1][0], 0x10d8)).lw(c);
        c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x98)).lh(c);
        c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x9c)).lw(c);
        let first = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, first, 0x0023_395c)?;
        c.g[A1][0] = Addr(p_2(c.g[S0][0], 0x18)).lw(c);
        let none = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = Addr(gp_3(c, -0x854)).lw(c);
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(p_2(c.g[A2][0], 0x10d8)).lw(c);
            c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x190)).lh(c);
            c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x194)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_397c)?;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p_2(c.g[S0][0], 0xc)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0039_7b70, 0x0023_3990)?;
        }
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.cycles += 1;
        c.g[S1][0] = 1;
        loop {
            c.cycles += 3;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
            let none = c.g[A1][0] == 0;
            c.g[A2][0] = Addr(gp_3(c, -0x854)).lw(c);
            if !none {
                c.cycles += 5;
                c.g[V0][0] = Addr(p_2(c.g[A2][0], 0x10d8)).lw(c);
                c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x190)).lh(c);
                c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x194)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_39b8)?;
            }
            c.cycles += 3;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_3998);
            }
        }
        c.cycles += 13;
        c.g[A0][0] = Addr(gp_3(c, -0x854)).lw(c);
        call_ptr(c, 0x0036_a020, 0x0023_39cc)?;
        call_ptr(c, 0x0028_b180, 0x0023_39d4)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0028_fa98, 0x0023_39e4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// GameState_SaveState (0x233af0)
// ---------------------------------------------------------------------------

/// The serialiser's virtual "save state" slot (`vt[1]`: delta at +8,
/// method at +0xc), and the size argument it gets.
pub(crate) const GAME_STATE_SAVE_SLOT: u32 = 0x8;

pub(crate) const GAME_STATE_SAVE_SIZE: u64 = 4;

/// `void GameState_SaveState(GameState* self, Serialiser* s)` (0x233af0):
/// the default save of a game state: `s->vt[1](s, self, 4)` (writes the
/// state's first word through the serialiser).
pub fn game_state_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        let serialiser = c.g[A1][0];
        c.g[A3][0] = serialiser;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = GAME_STATE_SAVE_SIZE;
        let vt = Addr(serialiser as u32).lw(c);
        c.g[V1][0] = vt;
        let slot = crate::call::VSlot::read(c, vt, GAME_STATE_SAVE_SLOT);
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(serialiser);
        call_ptr(c, slot.func as u32, 0x0023_3b18)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// GameStatePreRaceLoad_SaveState (0x234990).

pub(crate) const PRE_RACE_BASE_SAVE_STATE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};

/// `void GameStatePreRaceLoad_SaveState(GameStatePreRaceLoad* self, Stream*
/// s)` (0x234990): the base state's save, then the words at `+0x10`,
/// `+0x14`, `+0x1c` and `+0x8`, each written with the stream's virtual
/// `Write(s, p, 4)` (vtable slot `+8`).
pub fn game_state_pre_race_load_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        PRE_RACE_BASE_SAVE_STATE.call(c, 0x0023_49ac)?;
        for (field, ret) in [
            (0x10, 0x0023_49c8),
            (0x14, 0x0023_49e4),
            (0x1c, 0x0023_4a00),
            (0x08, 0x0023_4a1c),
        ] {
            let stream = r32(c, S0);
            c.g[V1][0] = Addr(stream).lw(c);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(field));
            c.g[A2][0] = 4;
            let vt = r32(c, V1);
            c.g[A0][0] = Addr(vt).off(8).lh(c);
            c.g[V0][0] = Addr(vt).off(0xc).lw(c);
            c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
            let write = r32(c, V0);
            call_ptr(c, write, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ GameStateFinish_SaveState

pub(crate) const FINISH_BASE_SAVE_STATE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};

/// `void GameStateFinish_SaveState(GameStateFinish* self, Stream* s)`
/// (0x2340b8): the base state's save, then the words at `+0x10` and
/// `+0x14`, each written with the stream's virtual `Write(s, p, 4)`
/// (vtable slot `+8`).
pub fn game_state_finish_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        FINISH_BASE_SAVE_STATE.call(c, 0x0023_40d4)?;
        for (field, ret) in [(0x10, 0x0023_40f0), (0x14, 0x0023_410c)] {
            let stream = r32(c, S0);
            c.g[V1][0] = Addr(stream).lw(c);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(field));
            c.g[A2][0] = 4;
            let vt = r32(c, V1);
            c.g[A0][0] = Addr(vt).off(8).lh(c);
            c.g[V0][0] = Addr(vt).off(0xc).lw(c);
            c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
            let write = r32(c, V0);
            call_ptr(c, write, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStatePreRaceLoad_CheckIntroScript(self)` (0x234910): start the
/// intro script when the engine is in state 2 and mark `self+0x14`.
pub fn gamestatepreraceeload_check_intro_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let engine = lw_at_m2(c, gw(c, -0x84c));
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = engine;
        c.g[V0][0] = lw_at_m2(c, p_m2(engine as u32, 0x550));
        c.g[V0][0] ^= 2;
        let not_state = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if !not_state {
            c.cycles += 5;
            c.g[A0][0] = c.g[V1][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = 1;
            c.g[A3][0] = 1;
            call_ptr(c, callee::SCRIPT_ENGINE_LIST_START.addr, 0x0023_4944)?;
        }
        c.cycles += 5;
        c.g[A0][0] = lw_at_m2(c, gw(c, -0x84c));
        c.g[V0][0] = lw_at_m2(c, p_m2(c.g[A0][0] as u32, 0x550));
        c.g[V0][0] ^= 2;
        if c.g[V0][0] != 0 {
            c.sw(p_m2(r32(c, S0), 0x14), 0);
        } else {
            c.cycles += 5;
            c.g[A1][0] = 0;
            c.g[A2][0] = 1;
            c.g[A3][0] = 1;
            call_ptr(c, callee::SCRIPT_ENGINE_LIST_START.addr, 0x0023_4968)?;
            c.sw(p_m2(r32(c, S0), 0x14), 0);
        }
        c.cycles += 5;
        c.g[V1][0] = 1;
        c.g[V0][0] = lw_at_m2(c, gw(c, -0x84c));
        c.g[V0][0] = lw_at_m2(c, p_m2(c.g[V0][0] as u32, 0x550));
        if c.g[V0][0] == c.g[V1][0] {
            c.sw(p_m2(r32(c, S0), 0x14), c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
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

/// `GameStateFinish_Update` (0x00233cd8): Per-frame update of the Finish state: counts down the finish-overlay timer (and closes the overlay), then by state 0 (wait until every rider is clear, reset and queue the end-event script list, decay aggression), 1 (wait for the script list, hide the load indicator) or 2 (go to the next game state in mode 4). `(self)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_state_finish_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_233db0: {
            'b_233dac: {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S3][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x78));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                call_ptr(c, 0x0014dc80, 0x00233d08)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00233d14)?; // call_indirect
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                if t {
                    break 'b_233dac;
                } else {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    if t {
                        break 'b_233dac;
                    } else {
                        'b_233d44: {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                            c.g[A0][0] = c.lw(c.g[A0][0] as u32);
                            let t = c.g[A0][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_233d44;
                            } else {
                                c.cycles += 1;
                                c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                                break 'b_233d44;
                            }
                        }
                        c.cycles += 2;
                        if c.g[A1][0] != 0u64 {
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                            break 'b_233db0;
                        } else {
                            c.cycles += 6;
                            call_ptr(c, 0x0020cc60, 0x00233d54)?; // call_indirect
                            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = 0x2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            if t {
                                break 'b_233dac;
                            } else {
                                c.cycles += 12;
                                c.g[A0][0] = 0x480000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3ff0));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x00317670, 0x00233d7c)?; // call_indirect
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039f9d8, 0x00233d88)?; // call_indirect
                                c.g[A3][0] = c.g[V0][0];
                                let t = c.g[A3][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_233dac;
                                } else {
                                    c.cycles += 6;
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.g[A2][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00233dac)?; // call_indirect
                                    break 'b_233dac;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
            break 'b_233db0;
        }
        'b_233fe4: {
            'b_233fe0: {
                'b_233fdc: {
                    'b_233fb0: {
                        'b_233f34: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x1;
                            let t = c.g[S0][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_233f34;
                            } else {
                                'b_233de4: {
                                    'b_233dd4: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] = 0x2;
                                            break 'b_233dd4;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[S0][0] == 0u64;
                                            c.g[V1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            if t {
                                                break 'b_233de4;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                if t {
                                                    break 'b_233fe0;
                                                } else {
                                                    break 'b_233dd4;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[S0][0] == c.g[V0][0];
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                    if t {
                                        break 'b_233fb0;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        if t {
                                            break 'b_233fe4;
                                        } else {
                                            break 'b_233de4;
                                        }
                                    }
                                }
                                'b_233e98: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x530000;
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x66a8));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                    c.g[S5][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc0));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                    let t = c.g[A0][0] != 0u64;
                                    c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                                    if t {
                                        break 'b_233e98;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S2][0] = 0u64;
                                        if t {
                                            break 'b_233e98;
                                        } else {
                                            c.cycles += 5;
                                            c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x5174));
                                            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
                                            c.lwc1(20, (c.g[GP][0] as u32).wrapping_add(0x1e70));
                                            c.g[S4][0] = 0x1;
                                            'l_233e20: loop {
                                                'b_233e38: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[GP][0] as u32)
                                                            .wrapping_add(0x1e74));
                                                    if c.g[V0][0] != 0u64 {
                                                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                                        break 'b_233e38;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.swc1(
                                                            21,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_add(0x1e70),
                                                        );
                                                        c.sw(
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_add(0x1e74),
                                                            c.g[S4][0],
                                                        );
                                                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                                        break 'b_233e38;
                                                    }
                                                }
                                                c.cycles += 21;
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                                                c.g[V1][0] =
                                                    c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                                c.g[A1][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                                let t = c.g[V0][0] as u32;
                                                c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[A1][0] as u32));
                                                call_ptr(c, t, 0x00233e54)?; // call_indirect
                                                c.lqc2(3, c.g[V0][0] as u32);
                                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                c.vu0().sqrt(4, 0);
                                                c.vu0().waitq();
                                                c.cfc2(3, 22);
                                                c.mtc1(3, 0);
                                                c.c_lt_s(20, 0);
                                                let t = c.fcc();
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_233fdc;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = ((c.g[S2][0] as i64)
                                                        < (c.g[S1][0] as i64))
                                                        as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[S0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x00233e20);
                                                        }
                                                        continue 'l_233e20;
                                                    } else {
                                                        break 'b_233e98;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                'b_233ec8: {
                                    c.cycles += 7;
                                    c.g[V1][0] = 0x530000;
                                    c.g[S0][0] = c.g[V1][0];
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                                    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
                                    c.g[V0][0] = c.g[V0][0] ^ 0x4;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = 0u64;
                                    if t {
                                        break 'b_233ec8;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x98));
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_233ec8;
                                        } else {
                                            c.cycles += 2;
                                            c.g[V0][0] =
                                                c.lw((c.g[S5][0] as u32).wrapping_add(0x9c));
                                            c.g[A0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                            break 'b_233ec8;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                if c.g[A0][0] != 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                    break 'b_233fe0;
                                } else {
                                    'b_233f08: {
                                        c.cycles += 13;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                        c.g[A1][0] = 0u64;
                                        call_ptr(c, 0x002790a0, 0x00233edc)?; // call_indirect
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                        call_ptr(c, 0x0027ac60, 0x00233ee4)?; // call_indirect
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                        c.g[A1][0] = 0u64;
                                        call_ptr(c, 0x00278f38, 0x00233ef0)?; // call_indirect
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                        c.g[A1][0] = 0u64;
                                        call_ptr(c, 0x00279248, 0x00233efc)?; // call_indirect
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[V0][0] = 0xc;
                                        if t {
                                            break 'b_233f08;
                                        } else {
                                            c.cycles += 1;
                                            c.sw(c.g[S3][0] as u32, c.g[V0][0]);
                                            break 'b_233f08;
                                        }
                                    }
                                    'b_233f2c: {
                                        c.cycles += 4;
                                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5bc8));
                                        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                                        let t = c.g[V1][0] != 0u64;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_233f2c;
                                        } else {
                                            c.cycles += 5;
                                            call_ptr(c, 0x00155a50, 0x00233f20)?; // call_indirect
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x00155e58, 0x00233f28)?; // call_indirect
                                            c.g[V0][0] = 0x1;
                                            break 'b_233f2c;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                    if t {
                                        break 'b_233fdc;
                                    } else {
                                        break 'b_233f34;
                                    }
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x002791d8, 0x00233f40)?; // call_indirect
                        let t = c.g[V0][0] == c.g[S0][0];
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if t {
                            break 'b_233fdc;
                        } else {
                            'b_233f90: {
                                c.cycles += 12;
                                c.g[A0][0] = 0x480000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3ff0));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x00317670, 0x00233f60)?; // call_indirect
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039f9d8, 0x00233f6c)?; // call_indirect
                                c.g[A3][0] = c.g[V0][0];
                                let t = c.g[A3][0] == 0u64;
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_233f90;
                                } else {
                                    c.cycles += 6;
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.g[A2][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00233f90)?; // call_indirect
                                    break 'b_233f90;
                                }
                            }
                            c.cycles += 8;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0x4;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                            call_ptr(c, 0x001f36d8, 0x00233fa4)?; // call_indirect
                            c.g[V0][0] = 0x2;
                            let t = true;
                            c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                            if t {
                                break 'b_233fdc;
                            } else {
                                break 'b_233fb0;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    let t = c.g[V1][0] != 0u64;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_233fe4;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V0][0] = 0x4;
                        let t = c.g[V1][0] != c.g[V0][0];

                        if t {
                            break 'b_233fe4;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S3][0];
                            call_ptr(c, 0x00233aa0, 0x00233fdc)?; // GameState_GotoNext
                            break 'b_233fdc;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                break 'b_233fe0;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_233fe4;
        }
        c.cycles += 9;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x78));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        Ok(())
    })
}

/// `GameStatePreRaceLoad_Enter` (0x002341d0): Enter handler of the PreRaceLoad state: sets the AI race state, resets the replay and the BE phase, copies the AI characters and boards of the game-mode manager into the player interface, and queues the start sequence for modes 1 and 2. `(self, mode)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_state_pre_race_load_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_2342ec: {
            c.cycles += 55;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S6][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S7][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S2][0] = 0u64;
            call_ptr(c, 0x0014dc80, 0x00234208)?; // call_indirect
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00234214)?; // call_indirect
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x0023421c)?; // call_indirect
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x00234228)?; // call_indirect
            c.g[S5][0] = c.g[V0][0];
            c.g[A1][0] = 0x3;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x00113b10, 0x0023423c)?; // call_indirect
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
            call_ptr(c, 0x001f36d8, 0x00234250)?; // call_indirect
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x0014de68, 0x00234258)?; // call_indirect
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc0));
            call_ptr(c, 0x0026f228, 0x0023426c)?; // call_indirect
            c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
            c.g[S3][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[S4][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x00144d98, 0x00234290)?; // call_indirect
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x002342a4)?; // call_indirect
            let t = (c.g[S3][0] as i64) <= 0;
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
            if t {
                break 'b_2342ec;
            } else {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S4][0] as u32));
                'l_2342b8: loop {
                    c.cycles += 13;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x28));
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00147338, 0x002342c8)?; // call_indirect
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x001473d0, 0x002342e0)?; // call_indirect
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002342b8);
                        }
                        continue 'l_2342b8;
                    } else {
                        break 'b_2342ec;
                    }
                }
            }
        }
        'b_234384: {
            'b_23434c: {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00234300)?; // call_indirect
                c.g[V0][0] = 0x1;
                let t = c.g[S6][0] == c.g[V0][0];
                c.g[V0][0] = ((c.g[S6][0] as i64) < 2i64) as u64;
                if t {
                    break 'b_23434c;
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[S7][0] as u32).wrapping_add(0x1c), 0u64);
                        break 'b_234384;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x2;
                        if c.g[S6][0] != c.g[V0][0] {
                            c.sw((c.g[S7][0] as u32).wrapping_add(0x1c), 0u64);
                            break 'b_234384;
                        } else {
                            c.cycles += 11;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x002790a0, 0x0023432c)?; // call_indirect
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x0027aaf8, 0x00234338)?; // call_indirect
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x00278f38, 0x00234344)?; // call_indirect
                            let t = true;
                            c.sw((c.g[S7][0] as u32).wrapping_add(0x1c), 0u64);
                            if t {
                                break 'b_234384;
                            } else {
                                break 'b_23434c;
                            }
                        }
                    }
                }
            }
            'b_234380: {
                c.cycles += 5;
                c.g[V1][0] = 0x530000;
                c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c10));
                c.g[V0][0] = c.g[V0][0] ^ 0x4;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                if t {
                    break 'b_234380;
                } else {
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002790a0, 0x00234368)?; // call_indirect
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0027aaf8, 0x00234374)?; // call_indirect
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00278f38, 0x00234380)?; // call_indirect
                    break 'b_234380;
                }
            }
            c.cycles += 1;
            c.sw((c.g[S7][0] as u32).wrapping_add(0x1c), 0u64);
            break 'b_234384;
        }
        c.cycles += 11;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        Ok(())
    })
}

/// `GameStatePreRaceLoad_Update` (0x002343b0): Per-frame update of the PreRaceLoad (event start) state: a state switch (0..4, jump table in the image) that waits for the intro script list, sets up or purges the AI riders and actors, loads the riders and goes to the next state. Resumes through the generated `__entries`. `(self)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_state_pre_race_load_update(c: &mut Cpu) {
    guest(c, |c| {
        if c.pc != 0x002343b0 {
            game_state_pre_race_load_update_resume(c);
            return Ok(());
        }
        'l_2343b0: loop {
            'b_234454: {
                'b_234414: {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_234414;
                    } else {
                        c.cycles += 10;
                        c.g[A1][0] = 0x2;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                        call_ptr(c, 0x001f36c0, 0x002343e4)?; // call_indirect
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x00279220, 0x002343f0)?; // call_indirect
                        c.g[V1][0] = 0x4;
                        if c.g[V0][0] != c.g[V1][0] {
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            break 'b_234454;
                        } else {
                            c.cycles += 6;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x00279040, 0x0023440c)?; // call_indirect
                            let t = true;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            if t {
                                break 'b_234454;
                            } else {
                                break 'b_234414;
                            }
                        }
                    }
                }
                'b_234440: {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00279220, 0x00234420)?; // call_indirect
                    c.g[V1][0] = 0x3;
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_234440;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0u64;
                        call_ptr(c, 0x00279040, 0x0023443c)?; // call_indirect
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        break 'b_234440;
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = 0x2;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
                call_ptr(c, 0x001f36d8, 0x00234450)?; // call_indirect
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                break 'b_234454;
            }
            'b_234740: {
                'b_23473c: {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        break 'b_23473c;
                    } else {
                        'b_2345f4: {
                            'b_2345c4: {
                                'b_234494: {
                                    c.cycles += 6;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3fe0));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x002343b0 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002343b0);
                                            }
                                            continue 'l_2343b0;
                                        }
                                        0x00234478 => {
                                            c.cycles += 4;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                            call_ptr(c, 0x00279298, 0x00234480)?; // call_indirect
                                            let t = c.g[V0][0] != 0u64;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_234740;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x1;
                                                let t = true;
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0x1c),
                                                    c.g[V0][0],
                                                );
                                                if t {
                                                    break 'b_234740;
                                                } else {
                                                    break 'b_234494;
                                                }
                                            }
                                        }
                                        0x00234494 => {
                                            break 'b_234494;
                                        }
                                        0x002345c4 => {
                                            break 'b_2345c4;
                                        }
                                        0x002345f4 => {
                                            break 'b_2345f4;
                                        }
                                        0x0023473c => {
                                            break 'b_23473c;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x002343b0, 0x00234750);
                                            return Ok(());
                                        }
                                    }
                                }
                                'b_23456c: {
                                    'b_234568: {
                                        'b_234518: {
                                            'b_2344f8: {
                                                'b_2344f4: {
                                                    c.cycles += 14;
                                                    call_ptr(c, 0x0014dc80, 0x0023449c)?; // call_indirect
                                                    c.g[A1][0] = 0u64;
                                                    c.g[A0][0] = c.g[V0][0];
                                                    call_ptr(c, 0x0014dd58, 0x002344a8)?; // call_indirect
                                                    c.g[A0][0] = c.g[S1][0];
                                                    call_ptr(c, 0x00234910, 0x002344b0)?; // GameStatePreRaceLoad_CheckIntroScript
                                                    c.g[V0][0] = 0x530000;
                                                    c.g[A2][0] = 0x4;
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x5bc8));
                                                    c.g[S0][0] = c
                                                        .lb((c.g[V0][0] as u32).wrapping_add(0x48));
                                                    c.g[V1][0] = c
                                                        .lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                                                    let t = c.g[S0][0] == c.g[A2][0];
                                                    c.g[A1][0] = c
                                                        .lb((c.g[V0][0] as u32).wrapping_add(0x49));
                                                    if t {
                                                        break 'b_2344f4;
                                                    } else {
                                                        'b_2344e4: {
                                                            c.cycles += 3;
                                                            c.g[A0][0] = 0x5;
                                                            let t = c.g[S0][0] != c.g[A0][0];
                                                            c.g[V0][0] = 0x6;
                                                            if t {
                                                                break 'b_2344e4;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = c.g[V1][0] != c.g[A2][0];
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x488));
                                                                if t {
                                                                    break 'b_2344f8;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[V0][0] = 0x6;
                                                                    break 'b_2344e4;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = c.g[S0][0] != c.g[V0][0];

                                                        if t {
                                                            break 'b_234518;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V1][0] == c.g[A0][0];

                                                            if t {
                                                                break 'b_234518;
                                                            } else {
                                                                break 'b_2344f4;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                                break 'b_2344f8;
                                            }
                                            c.cycles += 8;
                                            c.g[A0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            call_ptr(c, 0x0012ab20, 0x00234500)?; // call_indirect
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                            c.g[A0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            call_ptr(c, 0x0012ac48, 0x0023450c)?; // call_indirect
                                            c.g[V0][0] = 0x1;
                                            let t = true;
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0x10),
                                                c.g[V0][0],
                                            );
                                            if t {
                                                break 'b_234568;
                                            } else {
                                                break 'b_234518;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[A1][0] != 0u64;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                        if t {
                                            break 'b_23456c;
                                        } else {
                                            'b_234564: {
                                                c.cycles += 7;
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                call_ptr(c, 0x00128958, 0x00234528)?; // call_indirect
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                c.g[V1][0] =
                                                    c.lw((c.g[A0][0] as u32).wrapping_add(0x80));
                                                if c.g[V1][0] != 0u64 {
                                                    c.g[V1][0] =
                                                        c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                    break 'b_234564;
                                                } else {
                                                    'b_234560: {
                                                        'b_234554: {
                                                            c.cycles += 2;
                                                            let t = c.g[S0][0] != 0u64;

                                                            if t {
                                                                break 'b_234554;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A1][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x00128998, 0x0023454c,
                                                                )?; // call_indirect
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x488));
                                                                if t {
                                                                    break 'b_234560;
                                                                } else {
                                                                    break 'b_234554;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.g[A1][0] = 0x2;
                                                        call_ptr(c, 0x00128998, 0x0023455c)?; // call_indirect
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x488));
                                                        break 'b_234560;
                                                    }
                                                    c.cycles += 1;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                    break 'b_234564;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8c), 0u64);
                                            break 'b_234568;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                    break 'b_23456c;
                                }
                                'b_2345bc: {
                                    'b_2345a4: {
                                        'b_234590: {
                                            c.cycles += 6;
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            call_ptr(c, 0x00128a48, 0x00234578)?; // call_indirect
                                            c.g[V0][0] = 0x5;
                                            let t = c.g[S0][0] == c.g[V0][0];
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                            if t {
                                                break 'b_234590;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] != 0u64;
                                                c.g[V0][0] = 0x4;
                                                if t {
                                                    break 'b_2345a4;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x488)
                                                        );
                                                    break 'b_234590;
                                                }
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[A1][0] = 0x1;
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        call_ptr(c, 0x00128a48, 0x0023459c)?; // call_indirect
                                        let t = true;
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_2345bc;
                                        } else {
                                            break 'b_2345a4;
                                        }
                                    }
                                    'b_2345b8: {
                                        c.cycles += 2;
                                        let t = c.g[S0][0] == c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                        if t {
                                            break 'b_2345b8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A1][0] = 0x2;
                                            c.g[A0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            call_ptr(c, 0x00128a48, 0x002345b8)?; // call_indirect
                                            break 'b_2345b8;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = 0x2;
                                    break 'b_2345bc;
                                }
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                                if t {
                                    break 'b_23473c;
                                } else {
                                    break 'b_2345c4;
                                }
                            }
                            'b_2345ec: {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                call_ptr(c, 0x0012a180, 0x002345d0)?; // call_indirect
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x3;
                                if t {
                                    break 'b_2345ec;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                    call_ptr(c, 0x001296f8, 0x002345e4)?; // call_indirect
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_234740;
                                    } else {
                                        break 'b_2345ec;
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                            if t {
                                break 'b_23473c;
                            } else {
                                break 'b_2345f4;
                            }
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x00279220, 0x00234600)?; // call_indirect
                        let t = c.g[V0][0] != 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_234740;
                        } else {
                            'b_234620: {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                let t = (c.g[V1][0] as i64) <= 0;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_234620;
                                } else {
                                    c.cycles += 1;
                                    c.sw((c.g[A0][0] as u32).wrapping_add(0x8c), c.g[V0][0]);
                                    break 'b_234620;
                                }
                            }
                            'b_234638: {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x80));
                                let t = (c.g[V1][0] as i64) <= 0;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_234638;
                                } else {
                                    c.cycles += 1;
                                    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                    break 'b_234638;
                                }
                            }
                            'b_23466c: {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x002791d8, 0x00234644)?; // call_indirect
                                c.g[V1][0] = 0x2;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                if t {
                                    break 'b_23466c;
                                } else {
                                    c.cycles += 7;
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    c.g[A1][0] = 0u64;
                                    c.g[A3][0] = 0x1;
                                    c.g[A2][0] = (c.g[A2][0] < 0x1u64) as u64;
                                    call_ptr(c, 0x00278f68, 0x00234664)?; // call_indirect
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_234740;
                                    } else {
                                        break 'b_23466c;
                                    }
                                }
                            }
                            'b_234698: {
                                c.cycles += 9;
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x00279220, 0x00234674)?; // call_indirect
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0u64;
                                call_ptr(c, 0x00279148, 0x00234688)?; // call_indirect
                                let t = c.g[S0][0] == 0u64;
                                c.g[V1][0] = 0u64;
                                if t {
                                    break 'b_234698;
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.g[V0][0] ^ 0x5;
                                    c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                                    break 'b_234698;
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] != 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_234740;
                            } else {
                                'b_2346e8: {
                                    c.cycles += 10;
                                    call_ptr(c, 0x0014dc80, 0x002346a8)?; // call_indirect
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A1][0] = 0u64;
                                    call_ptr(c, 0x0014dd58, 0x002346b4)?; // call_indirect
                                    c.g[V1][0] = 0x530000;
                                    c.g[A0][0] = 0x4;
                                    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c10));
                                    let t = c.g[V0][0] == c.g[A0][0];
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                    if t {
                                        break 'b_2346e8;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        call_ptr(c, 0x001289f0, 0x002346d0)?; // call_indirect
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x80));
                                        let t = (c.g[V1][0] as i64) <= 0;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_2346e8;
                                        } else {
                                            c.cycles += 1;
                                            c.sw(
                                                (c.g[A0][0] as u32).wrapping_add(0x14),
                                                c.g[V0][0],
                                            );
                                            break 'b_2346e8;
                                        }
                                    }
                                }
                                'b_23472c: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                    if t {
                                        break 'b_23472c;
                                    } else {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        call_ptr(c, 0x0012abd0, 0x002346fc)?; // call_indirect
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                        if t {
                                            break 'b_23473c;
                                        } else {
                                            c.cycles += 4;
                                            c.g[A0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            call_ptr(c, 0x0012acf8, 0x0023470c)?; // call_indirect
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                            if t {
                                                break 'b_23473c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                call_ptr(c, 0x0012b030, 0x0023471c)?; // call_indirect
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                call_ptr(c, 0x0012b000, 0x00234728)?; // call_indirect
                                                c.sw((c.g[S1][0] as u32).wrapping_add(0x10), 0u64);
                                                break 'b_23472c;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = 0x4;
                                c.g[A0][0] = c.g[S1][0];
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                                call_ptr(c, 0x00233aa0, 0x0023473c)?; // GameState_GotoNext
                                break 'b_23473c;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_234740;
            }
            c.cycles += 4;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.pc = t;
            return Ok(());
        }
        Ok(())
    })
}

/// Native resume of `game_state_pre_race_load_update` (converted from the `entries` body): any entry
/// other than 0x002343b0, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn game_state_pre_race_load_update_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002343b0 => {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x00234414;
                        continue;
                    }
                    pc = 0x002343d4;
                    continue;
                }
                0x002343d4 => {
                    c.cycles += 10;
                    c.g[A1][0] = 0x2;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[RA][0] = sx(0x002343e4);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                    call_ptr(c, 0x001f36c0, 0x002343e4)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[RA][0] = sx(0x002343f0);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00279220, 0x002343f0)?;
                    c.g[V1][0] = 0x4;

                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        pc = 0x00234454;
                        continue;
                    }
                    pc = 0x002343fc;
                    continue;
                }
                0x002343fc => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x0023440c);
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x00279040, 0x0023440c)?;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                    if t {
                        pc = 0x00234454;
                        continue;
                    }
                    pc = 0x00234414;
                    continue;
                }
                0x00234414 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[RA][0] = sx(0x00234420);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00279220, 0x00234420)?;
                    c.g[V1][0] = 0x3;
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x00234440;
                        continue;
                    }
                    pc = 0x0023442c;
                    continue;
                }
                0x0023442c => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x0023443c);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x00279040, 0x0023443c)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    pc = 0x00234440;
                    continue;
                }
                0x00234440 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x2;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[RA][0] = sx(0x00234450);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
                    call_ptr(c, 0x001f36d8, 0x00234450)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                    pc = 0x00234454;
                    continue;
                }
                0x00234454 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x0023473c;
                        continue;
                    }
                    pc = 0x00234460;
                    continue;
                }
                0x00234460 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3fe0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00234478 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00234480);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    call_ptr(c, 0x00279298, 0x00234480)?;
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x00234740;
                        continue;
                    }
                    pc = 0x00234488;
                    continue;
                }
                0x00234488 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    if t {
                        pc = 0x00234740;
                        continue;
                    }
                    pc = 0x00234494;
                    continue;
                }
                0x00234494 => {
                    c.cycles += 14;
                    c.g[RA][0] = sx(0x0023449c);
                    call_ptr(c, 0x0014dc80, 0x0023449c)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x002344a8);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dd58, 0x002344a8)?;
                    c.g[RA][0] = sx(0x002344b0);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00234910,
                        f: ee::s06::GameStatePreRaceLoad_CheckIntroScript,
                    }
                    .call(c, 0x002344b0)?;
                    c.g[V0][0] = 0x530000;
                    c.g[A2][0] = 0x4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[S0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    let t = c.g[S0][0] == c.g[A2][0];
                    c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    if t {
                        pc = 0x002344f4;
                        continue;
                    }
                    pc = 0x002344cc;
                    continue;
                }
                0x002344cc => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x5;
                    let t = c.g[S0][0] != c.g[A0][0];
                    c.g[V0][0] = 0x6;
                    if t {
                        pc = 0x002344e4;
                        continue;
                    }
                    pc = 0x002344d8;
                    continue;
                }
                0x002344d8 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[A2][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    if t {
                        pc = 0x002344f8;
                        continue;
                    }
                    pc = 0x002344e0;
                    continue;
                }
                0x002344e0 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x6;
                    pc = 0x002344e4;
                    continue;
                }
                0x002344e4 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[V0][0];

                    if t {
                        pc = 0x00234518;
                        continue;
                    }
                    pc = 0x002344ec;
                    continue;
                }
                0x002344ec => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[A0][0];

                    if t {
                        pc = 0x00234518;
                        continue;
                    }
                    pc = 0x002344f4;
                    continue;
                }
                0x002344f4 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    pc = 0x002344f8;
                    continue;
                }
                0x002344f8 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x00234500);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012ab20, 0x00234500)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    c.g[RA][0] = sx(0x0023450c);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012ac48, 0x0023450c)?;
                    c.g[V0][0] = 0x1;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    if t {
                        pc = 0x00234568;
                        continue;
                    }
                    pc = 0x00234518;
                    continue;
                }
                0x00234518 => {
                    c.cycles += 2;
                    let t = c.g[A1][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    if t {
                        pc = 0x0023456c;
                        continue;
                    }
                    pc = 0x00234520;
                    continue;
                }
                0x00234520 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00234528);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x00128958, 0x00234528)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x80));

                    if c.g[V1][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        pc = 0x00234564;
                        continue;
                    }
                    pc = 0x0023453c;
                    continue;
                }
                0x0023453c => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;

                    if t {
                        pc = 0x00234554;
                        continue;
                    }
                    pc = 0x00234544;
                    continue;
                }
                0x00234544 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0023454c);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00128998, 0x0023454c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    if t {
                        pc = 0x00234560;
                        continue;
                    }
                    pc = 0x00234554;
                    continue;
                }
                0x00234554 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0023455c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00128998, 0x0023455c)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    pc = 0x00234560;
                    continue;
                }
                0x00234560 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    pc = 0x00234564;
                    continue;
                }
                0x00234564 => {
                    c.cycles += 1;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8c), 0u64);
                    pc = 0x00234568;
                    continue;
                }
                0x00234568 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    pc = 0x0023456c;
                    continue;
                }
                0x0023456c => {
                    c.cycles += 6;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00234578);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x00128a48, 0x00234578)?;
                    c.g[V0][0] = 0x5;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    if t {
                        pc = 0x00234590;
                        continue;
                    }
                    pc = 0x00234584;
                    continue;
                }
                0x00234584 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[V0][0] = 0x4;
                    if t {
                        pc = 0x002345a4;
                        continue;
                    }
                    pc = 0x0023458c;
                    continue;
                }
                0x0023458c => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    pc = 0x00234590;
                    continue;
                }
                0x00234590 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x0023459c);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x00128a48, 0x0023459c)?;
                    let t = true;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x002345bc;
                        continue;
                    }
                    pc = 0x002345a4;
                    continue;
                }
                0x002345a4 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    if t {
                        pc = 0x002345b8;
                        continue;
                    }
                    pc = 0x002345ac;
                    continue;
                }
                0x002345ac => {
                    c.cycles += 3;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x002345b8);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x00128a48, 0x002345b8)?;
                    pc = 0x002345b8;
                    continue;
                }
                0x002345b8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x2;
                    pc = 0x002345bc;
                    continue;
                }
                0x002345bc => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    if t {
                        pc = 0x0023473c;
                        continue;
                    }
                    pc = 0x002345c4;
                    continue;
                }
                0x002345c4 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    c.g[RA][0] = sx(0x002345d0);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012a180, 0x002345d0)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x002345ec;
                        continue;
                    }
                    pc = 0x002345d8;
                    continue;
                }
                0x002345d8 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    c.g[RA][0] = sx(0x002345e4);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x001296f8, 0x002345e4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x00234740;
                        continue;
                    }
                    pc = 0x002345ec;
                    continue;
                }
                0x002345ec => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    if t {
                        pc = 0x0023473c;
                        continue;
                    }
                    pc = 0x002345f4;
                    continue;
                }
                0x002345f4 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[RA][0] = sx(0x00234600);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x00279220, 0x00234600)?;
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x00234740;
                        continue;
                    }
                    pc = 0x00234608;
                    continue;
                }
                0x00234608 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00234620;
                        continue;
                    }
                    pc = 0x0023461c;
                    continue;
                }
                0x0023461c => {
                    c.cycles += 1;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x8c), c.g[V0][0]);
                    pc = 0x00234620;
                    continue;
                }
                0x00234620 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x80));
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00234638;
                        continue;
                    }
                    pc = 0x00234634;
                    continue;
                }
                0x00234634 => {
                    c.cycles += 1;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    pc = 0x00234638;
                    continue;
                }
                0x00234638 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[RA][0] = sx(0x00234644);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002791d8, 0x00234644)?;
                    c.g[V1][0] = 0x2;
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    if t {
                        pc = 0x0023466c;
                        continue;
                    }
                    pc = 0x00234650;
                    continue;
                }
                0x00234650 => {
                    c.cycles += 7;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = 0u64;
                    c.g[A3][0] = 0x1;
                    c.g[RA][0] = sx(0x00234664);
                    c.g[A2][0] = (c.g[A2][0] < 0x1u64) as u64;
                    call_ptr(c, 0x00278f68, 0x00234664)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x00234740;
                        continue;
                    }
                    pc = 0x0023466c;
                    continue;
                }
                0x0023466c => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x00234674);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00279220, 0x00234674)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00234688);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x00279148, 0x00234688)?;
                    let t = c.g[S0][0] == 0u64;
                    c.g[V1][0] = 0u64;
                    if t {
                        pc = 0x00234698;
                        continue;
                    }
                    pc = 0x00234690;
                    continue;
                }
                0x00234690 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] ^ 0x5;
                    c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                    pc = 0x00234698;
                    continue;
                }
                0x00234698 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x00234740;
                        continue;
                    }
                    pc = 0x002346a0;
                    continue;
                }
                0x002346a0 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x002346a8);
                    call_ptr(c, 0x0014dc80, 0x002346a8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x002346b4);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x002346b4)?;
                    c.g[V1][0] = 0x530000;
                    c.g[A0][0] = 0x4;
                    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c10));
                    let t = c.g[V0][0] == c.g[A0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    if t {
                        pc = 0x002346e8;
                        continue;
                    }
                    pc = 0x002346c8;
                    continue;
                }
                0x002346c8 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x002346d0);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x001289f0, 0x002346d0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x80));
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x002346e8;
                        continue;
                    }
                    pc = 0x002346e4;
                    continue;
                }
                0x002346e4 => {
                    c.cycles += 1;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    pc = 0x002346e8;
                    continue;
                }
                0x002346e8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    if t {
                        pc = 0x0023472c;
                        continue;
                    }
                    pc = 0x002346f4;
                    continue;
                }
                0x002346f4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002346fc);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012abd0, 0x002346fc)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    if t {
                        pc = 0x0023473c;
                        continue;
                    }
                    pc = 0x00234704;
                    continue;
                }
                0x00234704 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0023470c);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012acf8, 0x0023470c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    if t {
                        pc = 0x0023473c;
                        continue;
                    }
                    pc = 0x00234714;
                    continue;
                }
                0x00234714 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x0023471c);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012b030, 0x0023471c)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    c.g[RA][0] = sx(0x00234728);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012b000, 0x00234728)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), 0u64);
                    pc = 0x0023472c;
                    continue;
                }
                0x0023472c => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x4;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0023473c);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    Callee {
                        addr: 0x00233aa0,
                        f: ee::s06::GameState_GotoNext,
                    }
                    .call(c, 0x0023473c)?;
                    pc = 0x0023473c;
                    continue;
                }
                0x0023473c => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    pc = 0x00234740;
                    continue;
                }
                0x00234740 => {
                    c.cycles += 4;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002343b0, 0x00234750);
                    return Ok(());
                }
            }
        }
    });
}

/// `void cGFEventStartState::loseFocus(cGFEventStartState* self)` (0x234750,
/// `GameStatePreRaceLoad_Exit`).
///
/// Leaving the pre-race load state: by the game mode (byte 0x535c10) it
/// tears down the AI/camera/race managers (modes 0 vs the others), resets
/// the race manager's view slots (`0x128a48` with 0, and 1 or 2 by mode),
/// runs the mission and script cleanup, the optional NIS cleanup (byte
/// 0x535c11), clears `self + 0x14`, and, for a single view, resets one
/// slot-table entry of the world.
pub fn cgf_event_start_state_lose_focus(c: &mut Cpu) {
    guest(c, |c| {
        'b_2347d8: {
            'b_2347c0: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x002790a0, 0x00234778)?;
                call_ptr(c, 0x0014dc80, 0x00234780)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x0023478c)?;
                c.g[V1][0] = 0x530000;
                c.g[S3][0] = c.g[V1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                c.g[S0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
                let t = c.g[S0][0] != 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                if t {
                    break 'b_2347c0;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x00128958, 0x002347ac)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x001289f0, 0x002347b8)?;
                    let t = true;
                    c.g[S1][0] = 0x4;
                    if t {
                        break 'b_2347d8;
                    } else {
                        break 'b_2347c0;
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x00128a10, 0x002347c8)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x0026f7b8, 0x002347d4)?;
            c.g[S1][0] = 0x4;
            break 'b_2347d8;
        }
        'b_23480c: {
            c.cycles += 2;
            let t = c.g[S0][0] == c.g[S1][0];
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            if t {
                break 'b_23480c;
            } else {
                c.cycles += 11;
                c.g[A1][0] = 0x3;
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002347fc)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                c.g[V1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_23480c;
            }
        }
        'b_234858: {
            'b_234844: {
                'b_234830: {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x00128a48, 0x0023481c)?;
                    c.g[V0][0] = 0x5;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    if t {
                        break 'b_234830;
                    } else {
                        c.cycles += 2;
                        let t = c.g[S0][0] != 0u64;

                        if t {
                            break 'b_234844;
                        } else {
                            break 'b_234830;
                        }
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x00128a48, 0x0023483c)?;
                let t = true;

                if t {
                    break 'b_234858;
                } else {
                    break 'b_234844;
                }
            }
            c.cycles += 2;
            let t = c.g[S0][0] == c.g[S1][0];
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            if t {
                break 'b_234858;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0x2;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x00128a48, 0x00234858)?;
                break 'b_234858;
            }
        }
        'b_2348b8: {
            c.cycles += 19;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00308f38, 0x00234860)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00308c60, 0x00234868)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A1][0] = 0x3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), 0u64);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x00113b10, 0x00234880)?;
            c.g[A0][0] = 0x1;
            call_ptr(c, 0x0014de68, 0x00234888)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x00129160, 0x00234894)?;
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
            c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
            if c.g[V1][0] == 0u64 {
                c.sw((c.g[S2][0] as u32).wrapping_add(0x14), 0u64);
                break 'b_2348b8;
            } else {
                c.cycles += 5;
                call_ptr(c, 0x00155a50, 0x002348ac)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00155e58, 0x002348b4)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x14), 0u64);
                break 'b_2348b8;
            }
        }
        'b_2348f8: {
            c.cycles += 7;
            c.g[V0][0] = 0x1;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
            let t = c.g[V1][0] != c.g[V0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            if t {
                break 'b_2348f8;
            } else {
                c.cycles += 9;
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = 0x50;
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xb0));
                c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                c.mult(0, 2, 2, 6);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x250), 0u64);
                break 'b_2348f8;
            }
        }
        c.cycles += 6;
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

// ---------------------------------------------------------------------------
// cFELoadScreen_Load (0x233438)
// ---------------------------------------------------------------------------

/// Offsets of the load-screen state object (`this`).
pub(crate) const FE_LOAD_SCREEN_UI_ENGINE: u32 = 0xc;

pub(crate) const FE_LOAD_SCREEN_STATE: u32 = 0x18;

/// `void cFELoadScreen_Load(cFELoadScreen* this)` (0x233438): builds the
/// loading screen. Calls the app object's vtable slot (`+0x10d8`), creates the
/// UI engine (`this+0xc`, 0x70 bytes, `cUIEngine_loadFile`), copies two config
/// words into it, gives it a 0x1c-byte helper object, then picks one of three
/// screens (0x27: plain, no saved picture state or no mode byte: picture type
/// 1, else picture type 2), loads its pictures (`LoadPictures`), adds the screen
/// to the engine's list (`engine+0x18`) and marks `this+0x18 = -1`.
pub fn fe_load_screen_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 59;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = c.lw(oc1009_gp_minus(c, 0x854));
        frame.save(c, S0, 0x30);
        c.g[A1][0] = 0x100;
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(oc1009_ea(c.g[A2][0], 0x10d8));
        c.g[A0][0] = c.lh(oc1009_ea(c.g[V0][0], 0x160));
        c.g[V1][0] = c.lw(oc1009_ea(c.g[V0][0], 0x164));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(oc1009_ea(c.g[A2][0], c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_346c)?;
        // engine = new cUIEngine (0x70 bytes)
        c.g[A1][0] = 0x0048_0000;
        c.g[A2][0] = 0x100;
        c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4968);
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x70;
        call_ptr(c, 0x0031_7d70, 0x0023_3484)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0039_7b08, 0x0023_348c)?;
        c.g[A1][0] = 0x0048_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4050);
        c.sw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE), c.g[V0][0]);
        call_ptr(c, 0x0039_7c40, 0x0023_34a0)?;
        // copy two config words of the game object into the engine
        c.g[T0][0] = c.lw(oc1009_gp_minus(c, 0x848));
        c.g[A1][0] = 0x0048_0000;
        c.g[T1][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
        c.g[A0][0] = 0x1c;
        c.g[V0][0] = c.lw(oc1009_ea(c.g[T0][0], 0x88));
        c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4040);
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.sw(oc1009_ea(c.g[T1][0], 0x8), c.g[V0][0]);
        c.g[V1][0] = c.lw(oc1009_ea(c.g[T0][0], 0x8c));
        c.g[V0][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
        c.sw(oc1009_ea(c.g[V0][0], 0x10), c.g[V1][0]);
        // helper object (0x1c bytes) at engine+0xc
        call_ptr(c, 0x0031_7d70, 0x0023_34d4)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[A0][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
        c.g[V1][0] = oc1009_base_minus(c.g[V1][0] as u32, 0x2e30);
        c.sb(oc1009_ea(c.g[V0][0], 0x4), 0);
        c.sw(oc1009_ea(c.g[V0][0], 0x8), c.g[V1][0]);
        c.sw(oc1009_ea(c.g[V0][0], 0x18), 0);
        c.sw(oc1009_ea(c.g[V0][0], 0xc), 0);
        c.sw(oc1009_ea(c.g[V0][0], 0x10), 0);
        c.sw(oc1009_ea(c.g[V0][0], 0x14), 0);
        c.sw(oc1009_ea(c.g[A0][0], 0xc), c.g[V0][0]);
        c.g[V0][0] = c.lw(oc1009_gp_minus(c, 0x848));
        c.g[S0][0] = c.lw(oc1009_ea(c.g[V0][0], 0xc0));
        call_ptr(c, 0x0014_dc80, 0x0023_3508)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_3514)?;
        c.g[V1][0] = c.lw(oc1009_gp_minus(c, 0x172c));
        c.g[V0][0] = 0x27;
        // Which screen: 0 = id 0x27 (plain), 1 = picture type 1, 2 = picture type 2.
        let kind;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = c.lw(oc1009_ea(c.g[S0][0], 0x9c));
            c.cycles += 2;
            let no_state = c.g[V0][0] == 0;
            c.g[V0][0] = 0x0053_0000;
            if no_state {
                kind = 1;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lb(oc1009_ea(c.g[V0][0], 0x5c11));
                let has_mode = c.g[V1][0] != 0;
                c.g[A1][0] = 0x0048_0000;
                kind = if has_mode { 2 } else { 1 };
            }
        } else {
            kind = 0;
        }
        match kind {
            0 => {
                c.cycles += 15;
                c.g[A1][0] = 0x0048_0000;
                c.g[A0][0] = 0x48;
                c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4030);
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0023_353c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_e2a0, 0x0023_3550)?;
                c.g[V1][0] = 0x0048_0000;
                c.g[V1][0] = oc1009_base_minus(c.g[V1][0] as u32, 0x3ac8);
                c.sw(oc1009_ea(c.g[S0][0], 0x8), c.g[V1][0]);
                c.cycles += 1;
            }
            1 => {
                c.cycles += 21;
                c.g[A1][0] = 0x0048_0000;
                c.g[A0][0] = 0x4c;
                c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4018);
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0023_358c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_e2a0, 0x0023_35a0)?;
                c.g[V1][0] = 0x0048_0000;
                c.sw(oc1009_ea(c.g[S0][0], 0x48), 0);
                c.g[V1][0] = oc1009_base_minus(c.g[V1][0] as u32, 0x3928);
                c.g[A0][0] = c.g[S2][0];
                c.sw(oc1009_ea(c.g[S0][0], 0x8), c.g[V1][0]);
                c.g[A1][0] = 1;
                c.sw(oc1009_ea(c.g[S1][0], 0x48), c.g[S2][0]);
                call_ptr(c, 0x0023_3640, 0x0023_35c0)?;
            }
            _ => {
                c.cycles += 18;
                c.g[A0][0] = 0x4c;
                c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4000);
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0023_35dc)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_e2a0, 0x0023_35f0)?;
                c.g[V1][0] = 0x0048_0000;
                c.sw(oc1009_ea(c.g[S0][0], 0x48), 0);
                c.g[V1][0] = oc1009_base_minus(c.g[V1][0] as u32, 0x39f8);
                c.g[A0][0] = c.g[S2][0];
                c.sw(oc1009_ea(c.g[S0][0], 0x8), c.g[V1][0]);
                c.g[A1][0] = 2;
                c.sw(oc1009_ea(c.g[S1][0], 0x48), c.g[S2][0]);
                call_ptr(c, 0x0023_3640, 0x0023_3610)?;
                c.cycles += 1;
            }
        }
        // (screen kind 1 has no extra cycle: the generated block exits one step earlier)
        c.g[A0][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
        // engine->screens.add(screen)
        c.cycles += 11;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = sx(oc1009_ea(c.g[A0][0], 0x18));
        call_ptr(c, 0x0039_f400, 0x0023_3620)?;
        c.g[V0][0] = u64::MAX;
        frame.restore(c, S0, 0x30);
        c.sw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_STATE), c.g[V0][0]);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cFELoadScreen_LoadPictures (0x233640)
// ---------------------------------------------------------------------------

/// `void cFELoadScreen_LoadPictures(cFELoadScreen* this, int type)` (0x233640):
/// opens the `0x47bd78` big file (stack `cBigFile`), and loads the loading
/// screen's rider picture(s). Type 1: one picture named from the rider's
/// character id (player 0 or 1 depending on the load-screen options at
/// `0x534b30`, else the local player's) into `this+0x10`; type 2 with two
/// players (`0x535c11 == 2`): one picture per player into `this+0x10` and
/// `this+0x14`. Closes the big file (`cBigFile::~cBigFile(2)`).
pub fn fe_load_screen_load_pictures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0x100);
        frame.save(c, S2, 0xe0);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0xd0);
        frame.save(c, S7, 0x90);
        frame.save(c, S1, 0xf0);
        frame.save(c, S4, 0xc0);
        frame.save(c, S5, 0xb0);
        frame.save(c, S6, 0xa0);
        frame.save_ra(c, 0x80);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_3674)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0023_3680)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_3688)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0023_3694)?;
        // stack cBigFile at sp+0x70
        c.g[A0][0] = sx(oc1009_ea(c.g[SP][0], 0x70));
        c.g[A1][0] = 0x0048_0000;
        c.g[S7][0] = c.g[A0][0];
        c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4288);
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x0031_68b0, 0x0023_36ac)?;
        c.g[V1][0] = 1;
        let not_type1 = c.g[S0][0] != c.g[V1][0];
        c.g[V0][0] = 2;
        if !not_type1 {
            // type 1: one picture
            c.cycles += 10;
            c.g[V1][0] = c.lw(oc1009_gp_minus(c, 0x848));
            c.g[V0][0] = 0x0048_0000;
            c.g[S1][0] = oc1009_base_minus(c.g[V0][0] as u32, 0x4270);
            c.g[A1][0] = 7;
            c.g[A0][0] = c.lw(oc1009_ea(c.g[V1][0], 0x78));
            call_ptr(c, 0x0014_dd58, 0x0023_36d0)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[V0][0] = c.lw(oc1009_ea(c.g[V1][0], 0x4b30));
            let opt_off = c.g[V0][0] == 0;
            c.g[A0][0] = sx(oc1009_ea(c.g[V1][0], 0x4b30));
            if !opt_off {
                // options on: rider index 0 or 1 by the alternate flag at +8
                c.cycles += 3;
                c.g[V0][0] = c.lw(oc1009_ea(c.g[A0][0], 0x8));
                let alt_off = c.g[V0][0] == 0;
                c.g[S0][0] = 0x0044_0000;
                c.g[A0][0] = c.g[S3][0];
                if alt_off {
                    c.cycles += 3;
                    c.g[S0][0] = sx(oc1009_ea(c.g[S0][0], 0x2928));
                    c.g[A1][0] = 1;
                } else {
                    c.cycles += 4;
                    c.g[S0][0] = sx(oc1009_ea(c.g[S0][0], 0x2928));
                    c.g[A1][0] = 0;
                }
                c.cycles += 11;
                call_ptr(c, 0x0014_7398, 0x0023_3710)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A3][0] = c.lw(oc1009_gp_minus(c, 0x5b8));
                c.g[V0][0] = sx(oc1009_ea(c.g[V0][0], c.g[S0][0] as u32));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0041_7828, 0x0023_372c)?;
                c.g[A1][0] = 0x0300_0000;
            } else {
                c.cycles += 16;
                c.g[S0][0] = 0x0044_0000;
                c.g[A0][0] = c.g[S3][0];
                c.g[S0][0] = sx(oc1009_ea(c.g[S0][0], 0x2928));
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_6e98, 0x0023_3748)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0014_7398, 0x0023_3754)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A3][0] = c.lw(oc1009_gp_minus(c, 0x5b8));
                c.g[V0][0] = sx(oc1009_ea(c.g[V0][0], c.g[S0][0] as u32));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0041_7828, 0x0023_3770)?;
                c.g[A1][0] = 0x0300_0000;
            }
            // data = FileLoad(name, 0x3000100); this->tex = CreateTexture(data)
            c.cycles += 19;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] |= 0x100;
            call_ptr(c, 0x003e_2190, 0x0023_3780)?;
            c.g[T2][0] = c.lw(oc1009_gp_minus(c, 0x854));
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.lw(oc1009_ea(c.g[S0][0], 0x14));
            c.g[A2][0] = 0x0048_0000;
            c.g[V1][0] = c.lw(oc1009_ea(c.g[T2][0], 0x10d8));
            c.g[A2][0] = oc1009_base_minus(c.g[A2][0] as u32, 0x4260);
            c.g[A1][0] = sx(oc1009_ea(c.g[S0][0], c.g[A1][0] as u32));
            c.g[A3][0] = 0;
            c.g[A0][0] = c.lh(oc1009_ea(c.g[V1][0], 0x170));
            c.g[T0][0] = 1;
            c.g[V0][0] = c.lw(oc1009_ea(c.g[V1][0], 0x174));
            c.g[T1][0] = u64::MAX;
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx(oc1009_ea(c.g[T2][0], c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_37b8)?;
            let no_data = c.g[S0][0] == 0;
            c.sw(oc1009_ea(c.g[S2][0], 0x10), c.g[V0][0]);
            if no_data {
                c.cycles += 1;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0031_7e98, 0x0023_37c8)?;
            }
            c.g[A0][0] = c.g[S7][0];
        } else {
            c.cycles += 2;
            let not_type2 = c.g[S0][0] != c.g[V0][0];
            c.g[A0][0] = c.g[S7][0];
            if !not_type2 {
                c.cycles += 4;
                c.g[V0][0] = 0x0053_0000;
                c.g[V1][0] = c.lb(oc1009_ea(c.g[V0][0], 0x5c11));
                let not_two_players = c.g[V1][0] != c.g[S0][0];
                c.g[V0][0] = 0x0044_0000;
                if !not_two_players {
                    c.cycles += 6;
                    c.g[S2][0] = sx(oc1009_ea(c.g[S2][0], 0x10));
                    c.g[S6][0] = sx(oc1009_ea(c.g[V0][0], 0x2928));
                    c.g[S1][0] = 0;
                    c.g[S5][0] = 0x0048_0000;
                    c.g[S4][0] = 0x0048_0000;
                    loop {
                        c.cycles += 29;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0014_7488, 0x0023_380c)?;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A3][0] = c.lw(oc1009_gp_minus(c, 0x5b4));
                        c.g[V0][0] = sx(oc1009_ea(c.g[V0][0], c.g[S6][0] as u32));
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                        c.g[A1][0] = oc1009_base_minus(c.g[S5][0] as u32, 0x4270);
                        call_ptr(c, 0x0041_7828, 0x0023_3828)?;
                        c.g[A1][0] = 0x0300_0000;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A1][0] |= 0x100;
                        call_ptr(c, 0x003e_2190, 0x0023_3838)?;
                        c.g[T2][0] = c.lw(oc1009_gp_minus(c, 0x854));
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw(oc1009_ea(c.g[S0][0], 0x14));
                        c.g[A2][0] = oc1009_base_minus(c.g[S4][0] as u32, 0x4260);
                        c.g[V1][0] = c.lw(oc1009_ea(c.g[T2][0], 0x10d8));
                        c.g[A3][0] = 0;
                        c.g[A1][0] = sx(oc1009_ea(c.g[S0][0], c.g[A1][0] as u32));
                        c.g[T0][0] = 1;
                        c.g[A0][0] = c.lh(oc1009_ea(c.g[V1][0], 0x170));
                        c.g[T1][0] = u64::MAX;
                        c.g[V0][0] = c.lw(oc1009_ea(c.g[V1][0], 0x174));
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(oc1009_ea(c.g[T2][0], c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0023_386c)?;
                        let no_data = c.g[S0][0] == 0;
                        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                        if !no_data {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0031_7e98, 0x0023_387c)?;
                        }
                        c.cycles += 4;
                        c.g[S1][0] = sx(oc1009_ea(c.g[S1][0], 1));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < 2) as u64;
                        let again = c.g[V0][0] != 0;
                        c.g[S2][0] = sx(oc1009_ea(c.g[S2][0], 4));
                        if !again {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_3800);
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S7][0];
                }
            }
        }
        // cBigFile::~cBigFile(a0 = stack file, 2)
        c.cycles += 13;
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_68e8, 0x0023_3898)?;
        frame.restore(c, S0, 0x100);
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S7, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    });
}

/// `GameModule_RequestOverlayStateGlobal()` (0x233ad0): request the overlay
/// state of the game module held in the global at `gp-0x488`.
pub fn game_module_request_overlay_state_global(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
        frame.save_ra(c, 0);
        call_ptr(c, 0x0023_1278, 0x0023_3ae4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameState_LoadState(self, obj)` (0x233b28): the load-state hook of a
/// game state. `obj` is the loader; its vtable (`*obj`) holds a half-word
/// offset at `+0x10` (added to `obj` to make the `this` of the call) and the
/// function at `+0x14`, called with `(this, self, 4)`.
pub fn game_state_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = 0x4;
        let obj = c.g[A3][0] as u32;
        c.g[V1][0] = c.lw(obj);
        let vt = c.g[V1][0] as u32;
        c.g[A0][0] = c.lh(vt.wrapping_add(0x10));
        c.g[V0][0] = c.lw(vt.wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_3b50)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameStateQuit_LoadState(self, obj)` (0x233bb0): forwards to
/// `GameState_LoadState` with the same arguments.
pub fn game_state_quit_load_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x0023_3b28,
            f: ee::s06::f_00233b28,
        }),
        0x0023_3bc0,
    )
}

/// `GameStateIdle_LoadState(self, obj)` (0x233bf0): forwards to
/// `GameState_LoadState` with the same arguments.
pub fn game_state_idle_load_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x0023_3b28,
            f: ee::s06::f_00233b28,
        }),
        0x0023_3c00,
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::fe::festateloadhint::tests::oc1016_install_fakes;
    pub(crate) use crate::game::fe::ovstatepause::tests::oc778_install_fakes;

    use crate::difftest::fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    use crate::difftest::RA as TEST_RA;

    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base_2;
    pub(crate) use crate::game::render::particle::tests::gp_at;

    pub(crate) use crate::game::bx::bxstringctor::tests::RIDERS;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::rng_2 as rng;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::movie::movieplayer::tests::globals;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn game_state_finish_ctor_matches() {
        check(
            ssx3_ee::s06::f_00233c10,
            game_state_finish_ctor,
            64,
            &|c, seed| {
                let mut r = Rng(0x0c11_000d ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
                for i in 0..8 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn game_state_pre_race_load_ctor_matches() {
        check(
            ssx3_ee::s06::f_00234188,
            game_state_pre_race_load_ctor,
            64,
            &|c, seed| {
                let mut r = Rng(0x0c11_000e ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
                for i in 0..8 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn game_state_quit_enter_matches() {
        check(
            ssx3_ee::s06::f_00233b60__generated,
            game_state_quit_enter,
            100,
            &|c, seed| {
                let mut r = rng(seed, 0x1c);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x488, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn game_state_goto_next_matches() {
        check(
            ssx3_ee::s06::f_00233aa0__generated,
            game_state_goto_next,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0xb2, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                let target = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() % 0x40
                };
                c.bus.write32(DATA, target);
                c.bus.write32(DATA + 4, r.next_u32() % 0x100);
            },
        );
    }

    #[test]
    fn c_fe_load_screen_update_matches() {
        check(
            ee::s06::f_00233a70__generated,
            c_fe_load_screen_update,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x233a70, 4);
                globals_2(c);
                clear(c, DATA, 0x1000);
                c.g[A0][0] = DATA as u64;
                // UIEngine -> (+0xc) view -> (+8) vtable -> +0x170 slot.
                let engine = DATA + 0x800;
                let view = DATA + 0x900;
                c.bus.write32(DATA + 0xc, engine);
                c.bus.write32(engine + 0xc, view);
                let vt = vtable(c, &mut r, &[0x170]);
                c.bus.write32(view + 8, vt);
                let _ = seed;
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_loading_screen_loop_matches() {
        check(
            ee::s07::f_0028fa98__generated,
            ssx_audio_system_stop_loading_screen_loop,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_fa98, 0);
                let obj = DATA + 0x1000;
                c.bus.write32(
                    obj + 0x62a8,
                    match seed % 3 {
                        0 => u32::MAX,
                        1 => 3,
                        _ => 0,
                    },
                );
                c.bus.write32(obj + 0x118, DATA + 0x2000);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn c_fe_load_screen_enter_matches() {
        check(
            ssx3_ee::s06::f_002338c8__generated,
            c_fe_load_screen_enter,
            30,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2338c8);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), DATA + 0x200);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x300);
                let rm = DATA + 0x400;
                c.bus.write32(DATA + 0x200 + 0x10d8, rm);
                c.bus.write32(rm + 0x78, 0);
                c.bus.write32(rm + 0x7c, 0);
                c.bus.write32(DATA + 0x300 + 0x7c, 0);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn game_state_finish_load_state_matches() {
        check(
            ee::s06::f_00234120__generated,
            game_state_finish_load_state,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23_4120, 3);
                // The stream's vtable slot 2 (`+0x10`) is the fake field reader.
                let stream = vtable(c, &mut r, &[0x10]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    // ------------------------------------------------------------- AI / riders

    #[test]
    fn ai_set_rider_order_mode_matches() {
        check(
            ee::s00::f_00128a48__generated,
            ai_set_rider_order_mode,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x12_8a48, 0);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let n = (seed % 4) as u32;
                w(c, DATA + 0x2000 + 0x78, n);
                for i in 0..4u32 {
                    let rider = DATA + 0x2400 + 0x40 * i;
                    w(c, DATA + 0x2000 + 0x28 + 4 * i, rider);
                    w(c, rider + 0xec, 0xffff_ffff);
                }
                c.pc = 0x0012_8a48;
            },
        );
    }

    #[test]
    fn c_fe_load_screen_render_matches() {
        check(
            ee::s06::f_002339f8__generated,
            c_fe_load_screen_render,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x2339f8, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let app = DATA + 0x2000;
                w(c, GAME_GP.wrapping_sub(0x854), app);
                c.g[A0][0] = DATA as u64;
                let vt = DATA + 0x3000;
                w(c, app + 0x10d8, vt);
                slot(c, vt, 0x88, 0, FAKE);
                slot(c, vt, 0xa0, 0, FAKE + 0x10);
                w(c, DATA + 0xc, DATA + 0x400);
            },
        );
    }

    // ------------------------------------------------------------------
    // Game state.

    fn game_state_finish_enter_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = machine(c, seed, 0x233c50, calls);
        be_library(c);
        world(c);
        c.g[A0][0] = (DATA + 0x100) as u64;
        let replay = DATA + 0x200;
        w(c, GAME_GP.wrapping_sub(0x488), replay);
        w(c, replay + 0x28, 0);
        w(c, replay + 0x48, DATA + 0x280);
        w(c, GAME_GP.wrapping_sub(0x84c), 0);
        w(
            c,
            GAME_GP.wrapping_sub(0x204),
            if seed % 3 == 0 { 0 } else { DATA + 0x300 },
        );
        let _ = r.next_u32();
    }

    /// The finish screen's 180-frame delay is stored in ticks (3 s at any
    /// rate).
    #[test]
    fn game_state_finish_enter_delay_follows_rate() {
        for (hz, ticks) in [(60, 180), (120, 360), (240, 720)] {
            let mut c = crate::difftest::fresh();
            game_state_finish_enter_fixture(&mut c, 1, 0);
            crate::tick::set_for_test(&mut c, hz);
            game_state_finish_enter(&mut c);
            assert_eq!(c.pc, TEST_RA);
            assert_eq!(c.bus.read32(DATA + 0x100 + 0x14), ticks, "{hz} Hz");
        }
    }

    #[test]
    fn game_state_finish_enter_matches() {
        check(
            ssx3_ee::s06::f_00233c50__generated,
            game_state_finish_enter,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x233c50, 6);
                be_library(c);
                world(c);
                let st = DATA + 0x100;
                c.g[A0][0] = st as u64;
                // gp-0x488: the replay manager; +0x28 and +0x48 its objects.
                let replay = DATA + 0x200;
                w(c, GAME_GP.wrapping_sub(0x488), replay);
                w(c, replay + 0x28, 0);
                w(c, replay + 0x48, DATA + 0x280);
                // gp-0x84c: the script engine for ScriptEngine_ListStop.
                w(c, GAME_GP.wrapping_sub(0x84c), 0);
                // gp-0x204: the GameStateFinish instance (or none).
                if seed % 3 == 0 {
                    w(c, GAME_GP.wrapping_sub(0x204), 0);
                } else {
                    w(c, GAME_GP.wrapping_sub(0x204), DATA + 0x300);
                }
                let _ = r.next_u32();
            },
        );
    }

    pub(crate) fn r32_seed(seed: u32) -> u32 {
        seed.wrapping_mul(0x9e37_79b9) ^ 0x1234
    }

    #[test]
    fn game_state_pre_race_load_load_state_matches() {
        check(
            ssx3_ee::s06::f_00234a30__generated,
            game_state_pre_race_load_load_state,
            10,
            &|c, seed| {
                let _r = base(c, seed, 0x234a30, 2);
                let this = DATA + 0x1000;
                let stream = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = stream as u64;
                w_2(c, stream, DATA + 0x3000);
                c.bus.write16(DATA + 0x3000 + 0x10, 0);
                w_2(c, DATA + 0x3000 + 0x14, 0);
                w_2(c, this + 0x10, r32_seed(seed));
            },
        );
    }

    #[test]
    fn game_state_finish_exit_matches() {
        check(
            ee::s06::f_00234008__generated,
            game_state_finish_exit,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x2340);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(RIDERS + 0x10, (seed % 2) as u32);
                c.bus.write32(RIDERS + 4, DATA + 0x600);
                c.bus.write32(DATA + 0x600 + 0xa8, DATA + 0x700);
                c.bus.write32(gp_at(-0x84c), DATA + 0x800);
                fill(c, &mut r, DATA + 0x700, 0x40);
                fill(c, &mut r, DATA + 0x800, 0x40);
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    #[test]
    fn cfe_load_screen_unload_matches() {
        check(
            ee::s06::f_00233930__generated,
            cfe_load_screen_unload,
            40,
            &|c, seed| {
                let _r = base_3(c, seed, 0x2339);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x18, (seed % 4) as u32);
                c.bus
                    .write32(DATA + 0xc, if seed % 3 == 0 { 0 } else { DATA + 0x1000 });
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x14, seed as u32);
            },
        );
    }

    /// GameState_SaveState: a serialiser whose vtable slot 1 is a fake
    /// method (random this-delta; some calls unwind).
    #[test]
    fn game_state_save_state_matches() {
        use crate::difftest::fakes::{covered, machine, ptr, vtable};
        use crate::difftest::DATA;
        covered(
            ssx3_ee::s06::f_00233af0__generated,
            game_state_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23_3af0, 1);
                c.pc = 0x0023_3af0;
                let serialiser = DATA + 0x20_0000 + (r.next_u32() & 0xfff0);
                let vt = vtable(c, &mut r, &[GAME_STATE_SAVE_SLOT]);
                c.bus.write32(serialiser, vt);
                c.g[A0][0] = sx(ptr(&mut r));
                c.g[A1][0] = sx(serialiser);
            },
        );
    }

    /// GameStatePreRaceLoad_SaveState: a stream whose `Write` slot is a
    /// fake (logs every argument; one of the five calls may unwind); the
    /// real base GameState_SaveState makes the first call.
    #[test]
    fn game_state_pre_race_load_save_state_matches() {
        fakes::covered(
            ee::s06::f_00234990__generated,
            game_state_pre_race_load_save_state,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x234990, 5);
                let stream = DATA + 0x200;
                let vt = fakes::vtable(c, &mut r, &[8]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(DATA + 0x400 + (r.next_u32() & 0xfc));
                c.g[A1][0] = sx(stream);
                c.pc = 0x0023_4990;
            },
        );
    }

    /// A stream whose `Write` slot is a fake (logs every argument; one of
    /// the three calls may unwind); the real base GameState_SaveState makes
    /// the first call.
    #[test]
    fn game_state_finish_save_state_matches() {
        use crate::difftest::fakes;
        fakes::covered(
            ee::s06::f_002340b8__generated,
            game_state_finish_save_state,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2340b8, 3);
                let stream = DATA + 0x200;
                let vt = fakes::vtable(c, &mut r, &[8]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(DATA + 0x400 + (r.next_u32() & 0xfc));
                c.g[A1][0] = sx(stream);
                c.pc = 0x0023_40b8;
            },
        );
    }

    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_m2;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_m2;

    #[test]
    fn gamestatepreraceeload_check_intro_script_matches() {
        check(
            ee::s06::f_00234910__generated,
            gamestatepreraceeload_check_intro_script,
            SEEDS_m2,
            &|c, seed| {
                base_m2(c, seed, 0x4910, 0);
                let engine = DATA + 0x800;
                c.bus.write32(GAME_GP - 0x84c, engine);
                c.bus.write32(engine + 0x550, (seed % 3) as u32);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    use crate::difftest::fakes::covered;

    pub(crate) use crate::game::main::gamestate::tests::oc326_state_fixture;

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

    use crate::difftest::elf_image;
    use crate::difftest::fakes::LOG_RESULTS;

    // ------------------------------------------------------------ fixture.
    //
    // The functions here are BE/game-state glue: they walk a graph of game
    // objects (world, game, replay, game-mode manager, BE interfaces) and call
    // already ported and generated helpers. The fixture is a small, acyclic
    // object graph: level 2 holds small integers only, level 1 holds small
    // integers or pointers to level 2, so a dereference chain always ends and
    // no list or counter runs away. The BE interfaces come from the fake
    // calls (`BELibrary_GetInterface` ends in a vtable call the fakes answer
    // with a level-1 pointer).

    pub(crate) const OC444_L1: u32 = DATA;

    pub(crate) const OC444_L2: u32 = DATA + 0x4000;

    pub(crate) const OC444_WORLD: u32 = DATA + 0x8000;

    /// The test object most functions take as `self` (0x1000 bytes).
    pub(crate) const OC444_OBJ: u32 = DATA + 0x9000;

    /// A second object (stream, stats, texture data; 0x1000 bytes).
    pub(crate) const OC444_OBJ2: u32 = DATA + 0xa000;

    pub(crate) const OC444_GAME: u32 = DATA + 0xb000;

    pub(crate) const OC444_REPLAY: u32 = DATA + 0xb400;

    pub(crate) const OC444_AI: u32 = DATA + 0xb800;

    pub(crate) const OC444_GM: u32 = DATA + 0xc000;

    pub(crate) const OC444_SCRIPT: u32 = DATA + 0xc800;

    pub(crate) const OC444_ASYNC: u32 = DATA + 0xd000;

    pub(crate) const OC444_HANDLER: u32 = DATA + 0xd800;

    pub(crate) const OC444_VTABLE: u32 = DATA + 0xdc00;

    /// `D_005305B0`: the BE profile and race table (rider slots at +0..+0xc).
    pub(crate) const OC444_RACEDB: u32 = 0x0053_05b0;

    /// `cBENewRaceInterface_newRace` (`D_00535BC8`).
    pub(crate) const OC444_NEWRACE: u32 = 0x0053_5bc8;

    /// A word of the fixture graph: a small integer or a pointer into `base`.
    pub(crate) fn oc444_word(r: &mut Rng, base: u32) -> u32 {
        if r.next_u32() % 3 == 0 {
            base + (r.next_u32() & 0x3ff0)
        } else {
            r.next_u32() % 6
        }
    }

    /// `words` small integers (0..6) from `base`.
    pub(crate) fn oc444_small(c: &mut Cpu, r: &mut Rng, base: u32, words: u32) {
        for i in 0..words {
            let v = r.next_u32() % 6;
            c.bus.write32(base + 4 * i, v);
        }
    }

    /// The machine every test starts from: game image, fake calls, the object
    /// graph described above, the BE singletons and the race tables.
    pub(crate) fn oc444_env(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        elf_image(c);
        for i in 0..0x1000u32 {
            let v = r.next_u32() % 6;
            c.bus.write32(OC444_L2 + 4 * i, v);
        }
        for i in 0..0x1000u32 {
            let w = oc444_word(&mut r, OC444_L2);
            c.bus.write32(OC444_L1 + 4 * i, w);
        }
        for n in 0..8u32 {
            let p = OC444_L1 + (r.next_u32() & 0x3ff0);
            c.bus.write32(LOG_RESULTS + 4 * n, p);
        }
        for base in [
            OC444_WORLD,
            OC444_OBJ,
            OC444_OBJ2,
            OC444_GAME,
            OC444_REPLAY,
            OC444_AI,
            OC444_GM,
            OC444_SCRIPT,
            OC444_ASYNC,
        ] {
            oc444_small(c, &mut r, base, 0x100);
        }
        oc444_small(c, &mut r, OC444_OBJ, 0x400);
        oc444_small(c, &mut r, OC444_OBJ2, 0x400);
        // World -> game -> (replay, AI, ...), game-mode manager, FE async.
        c.bus.write32(OC444_WORLD + 0x84, OC444_GAME);
        c.bus.write32(OC444_WORLD + 0xc0, OC444_GM);
        c.bus.write32(OC444_WORLD + 0x11c, OC444_ASYNC);
        c.bus.write32(OC444_GAME + 0xc, OC444_AI);
        c.bus.write32(OC444_GAME + 0x28, OC444_REPLAY);
        let replay_mode = [0, 3, 10, 0][r.next_u32() as usize % 4];
        c.bus.write32(OC444_REPLAY, replay_mode);
        let ai_count = r.next_u32() % 4;
        c.bus.write32(OC444_AI + 0x7c, ai_count);
        for i in 0..4u32 {
            c.bus.write32(OC444_AI + 0x40 + 4 * i, OC444_L2 + 0x40 * i);
        }
        let gm_total = 3 + r.next_u32() % 3;
        c.bus.write32(OC444_GM + 0x10, gm_total);
        let gm_humans = r.next_u32() % 3;
        c.bus.write32(OC444_GM + 0x14, gm_humans);
        let gm_flag = r.next_u32() % 2;
        c.bus.write32(OC444_GM + 0x98, gm_flag);
        c.bus.write32(OC444_GM + 0x9c, r.next_u32() % 2);
        // gp singletons.
        for off in [
            0x1e74u32,
            0x2a90,
            0x304,
            0xce8,
            0xffff_e138,
            0xffff_fdfc,
            0xffff_fa4c,
            0xffff_f7b4,
            0xffff_f7ac,
        ] {
            let w = oc444_word(&mut r, OC444_L1);
            c.bus.write32(GAME_GP.wrapping_add(off), w);
        }
        c.bus.write32(GAME_GP - 0x848, OC444_WORLD);
        c.bus.write32(GAME_GP - 0x84c, OC444_SCRIPT);
        c.bus.write32(GAME_GP - 0x488, OC444_GAME);
        c.bus.write32(GAME_GP - 0x1f04, OC444_L1 + 0x3000);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        let map_state = r.next_u32() % 2;
        c.bus.write32(GAME_GP - 0xb54, map_state);
        let map_style = r.next_u32() % 4;
        c.bus.write32(GAME_GP - 0xb5c, map_style);
        // Race tables: BE rider -> profile, profile slots, race setup.
        oc444_small(c, &mut r, OC444_RACEDB, 0x18);
        oc444_small(c, &mut r, OC444_NEWRACE, 0x14);
        for k in 0..4u32 {
            let p = r.next_u32() % 3;
            c.bus.write32(OC444_RACEDB + 4 * k, p);
        }
        for p in 0..3u32 {
            let slot = 0x0053_5b20 + 0x1c * p;
            let unused = if r.next_u32() % 6 == 0 { u32::MAX } else { 0 };
            c.bus.write32(slot + 0xc, unused);
            let bits = r.next_u32();
            c.bus.write32(slot + 0x10, bits & 1);
            c.bus.write8(slot + 0x11, (bits >> 8) as u8 % 4);
        }
        let racedb_mode = [0u8, 5, 4, 1, 2, 3][r.next_u32() as usize % 6];
        c.bus.write8(OC444_RACEDB + 0x48, racedb_mode);
        let racedb_flag = (r.next_u32() % 5 == 0) as u8;
        c.bus.write8(OC444_RACEDB + 0x49, racedb_flag);
        c.bus.write8(OC444_NEWRACE + 0x48, (r.next_u32() % 7) as u8);
        c.bus.write8(OC444_NEWRACE + 0x49, (r.next_u32() % 2) as u8);
        c.bus
            .write8(OC444_NEWRACE + 0x4a, (r.next_u32() % 12) as u8);
        let humans = r.next_u32() % 4;
        c.bus.write32(OC444_NEWRACE + 0x34, humans);
        oc444_small(c, &mut r, 0x0053_5c18, 0x280);
        oc444_small(c, &mut r, 0x0053_6640, 0x40);
        // Game-mode handlers: vtable slot at +0x44 is a fake method.
        c.bus.write32(OC444_VTABLE + 0x40, 0);
        c.bus.write32(OC444_VTABLE + 0x44, FAKE);
        c.bus.write32(OC444_HANDLER, OC444_VTABLE);
        for h in 0..4u32 {
            c.bus.write32(0x0053_6668 + 4 * h, OC444_HANDLER);
        }
        let aux = r.next_u32() % 2;
        c.bus.write32(0x0053_66a8, aux);
        r
    }

    /// The pool allocator and the arena (`rbc_test_allocator`), for callees
    /// that allocate.
    pub(crate) fn oc444_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            for n in 0..16u32 {
                let node = nodes + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, nodes);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
        c.g[A0][0] = 0x004f_f250;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
    }

    /// Run `hand` against `generated` from the fixture machine, entering at
    /// `entry`; `args` sets the arguments and the function's own state.
    pub(crate) fn oc444_run(
        generated: GuestFn,
        hand: GuestFn,
        entry: u32,
        salt: u64,
        seeds: u32,
        args: &dyn Fn(&mut Cpu, &mut Rng, u32),
    ) {
        covered(generated, hand, seeds, &|c, seed| {
            let mut r = oc444_env(c, seed, salt, 4);
            args(c, &mut r, seed);
            c.pc = entry;
        });
    }

    // ----------------------------------- GameModule_QuitToLoadScreen etc.

    /// `GameStateFinish_Update(self)`: the countdown, the three states and the
    /// advance to the next game state.
    #[test]
    fn game_state_finish_update_matches() {
        oc444_run(
            ee::s06::f_00233cd8__generated,
            game_state_finish_update,
            0x0023_3cd8,
            0x23_3cd8,
            150,
            &|c, r, seed| {
                c.g[A0][0] = OC444_OBJ as u64;
                c.bus
                    .write32(OC444_OBJ + 0x10, [0u32, 1, 2, 0, 1, 7][seed as usize % 6]);
                c.bus
                    .write32(OC444_OBJ + 0x14, [0u32, 1, 2][r.next_u32() as usize % 3]);
            },
        );
    }

    /// `GameStatePreRaceLoad_Enter(self, mode)`.
    #[test]
    fn game_state_pre_race_load_enter_matches() {
        oc444_run(
            ee::s06::f_002341d0__generated,
            game_state_pre_race_load_enter,
            0x0023_41d0,
            0x23_41d0,
            120,
            &|c, r, seed| {
                c.g[A0][0] = OC444_OBJ as u64;
                c.g[A1][0] = [0u32, 1, 2, 3][seed as usize % 4] as u64;
                let _ = r;
            },
        );
    }

    /// `GameStatePreRaceLoad_Update(self)`: every state of the switch (the
    /// jump table is in the game image).
    #[test]
    fn game_state_pre_race_load_update_matches() {
        oc444_run(
            ee::s06::f_002343b0__generated,
            game_state_pre_race_load_update,
            0x0023_43b0,
            0x23_43b0,
            200,
            &|c, r, seed| {
                oc444_allocator(c);
                c.g[A0][0] = OC444_OBJ as u64;
                c.bus
                    .write32(OC444_OBJ + 0x1c, [0u32, 1, 2, 3, 4, 5][seed as usize % 6]);
                c.bus.write32(OC444_OBJ + 0x14, r.next_u32() % 2);
                c.bus.write32(OC444_OBJ + 0x18, r.next_u32() % 2);
            },
        );
    }

    pub(crate) use crate::game::fe::ovstatepause::tests::oc778_arm;

    /// `GameStatePreRaceLoad_Exit`: the game mode byte (0, 4, 5, other), the
    /// NIS byte, the one-view slot test (`+0x10 == 1`) and the unwinds.
    #[test]
    fn cgf_event_start_state_lose_focus_matches() {
        oc778_install_fakes();
        fakes::covered(
            ee::s06::f_00234750__generated,
            cgf_event_start_state_lose_focus,
            320,
            &|c, seed| {
                machine(c, seed, 0x778d, 14);
                oc778_arm(c);
                c.g[A0][0] = sx(DATA + 0x7_0000);
                c.bus
                    .write8(0x0053_5c10, [0u8, 4, 5, 1][(seed % 4) as usize]);
                c.bus.write8(0x0053_5c11, ((seed / 4) % 2) as u8);
                let game = DATA + 0x1_0000;
                let q = DATA + 0x2_0000;
                let vo = DATA + 0x3_0000;
                c.bus.write32(GAME_GP - 0x488, game);
                c.bus.write32(game + 0xc, q);
                c.bus.write32(q + 0xcc, vo);
                c.bus.write16(vo + 0x28, 0x10);
                c.bus.write32(vo + 0x2c, fakes::FAKE + 0x30);
                let world = DATA + 0x4_0000;
                let a3 = DATA + 0x5_0000;
                let a0p = DATA + 0x6_0000;
                let a1p = DATA + 0x8_0000;
                let table = DATA + 0x9_0000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, a3);
                c.bus.write32(a3 + 0x84, a0p);
                c.bus
                    .write32(a0p + 0x10, if (seed / 8) % 2 == 0 { 1 } else { 2 });
                c.bus.write32(a0p + 4, a1p);
                c.bus
                    .write32(a1p + 0xb0, [0u32, 1, 5][((seed / 16) % 3) as usize]);
                c.bus.write32(a3 + 0x10, table);
                c.bus.write32(table, DATA + 0xa_0000);
            },
        );
    }

    pub(crate) use crate::game::fe::festateloadhint::tests::oc1009_machine;
    pub(crate) use crate::game::fe::festateloadhint::tests::oc1009_queue;

    pub(crate) use crate::game::fe::festateloadhint::tests::OC1009_GOBJ;

    /// Fixture objects.
    pub(crate) const OC1009_APP: u32 = DATA + 0x21_1000;

    pub(crate) const OC1009_APP_VT: u32 = DATA + 0x21_2000;

    pub(crate) const OC1009_GM: u32 = DATA + 0x21_4000;

    pub(crate) const OC1009_THIS: u32 = DATA + 0x21_0000;

    /// The app object (`gp-0x854`) with its UI vtable: slots `+0x160` and
    /// `+0x170` point at fake method addresses outside the function table.
    pub(crate) fn oc1009_app_object(c: &mut Cpu, delta: u16) {
        c.bus.write32(GAME_GP - 0x854, OC1009_APP);
        c.bus.write32(OC1009_APP + 0x10d8, OC1009_APP_VT);
        c.bus.write16(OC1009_APP_VT + 0x160, delta);
        c.bus.write32(OC1009_APP_VT + 0x164, fakes::FAKE);
        c.bus.write16(OC1009_APP_VT + 0x170, delta);
        c.bus.write32(OC1009_APP_VT + 0x174, fakes::FAKE + 0x10);
    }

    /// `cFELoadScreen_Load`: the three screens (id 0x27; no saved state; two
    /// players) and unwinds at every call.
    #[test]
    fn fe_load_screen_load_matches() {
        check(
            ee::s06::f_00233438__generated,
            fe_load_screen_load,
            120,
            &|c, seed| {
                let mut r = oc1009_machine(c, seed, 0x0023_3438, 14);
                oc1009_app_object(c, (r.next_u32() % 0x40) as u16);
                c.bus.write32(GAME_GP - 0x848, OC1009_GOBJ);
                c.bus.write32(OC1009_GOBJ + 0x88, r.next_u32());
                c.bus.write32(OC1009_GOBJ + 0x8c, r.next_u32());
                c.bus.write32(OC1009_GOBJ + 0xc0, OC1009_GM);
                let kind = seed % 4;
                c.bus
                    .write32(GAME_GP - 0x172c, if kind == 0 { 0x27 } else { 0x26 });
                c.bus
                    .write32(OC1009_GM + 0x9c, if kind == 1 { 0 } else { 1 });
                c.bus.write8(0x0053_5c11, if kind == 3 { 2 } else { 0 });
                c.g[A0][0] = OC1009_THIS as u64;
            },
        );
    }

    /// The rider-name tables the picture loader indexes with the (small) ids
    /// its callees return, and the options record at `0x534b30`.
    pub(crate) fn oc1009_lp_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = oc1009_machine(c, seed, salt, 22);
        oc1009_app_object(c, (r.next_u32() % 0x40) as u16);
        c.bus.write32(GAME_GP - 0x848, OC1009_GOBJ);
        c.bus.write32(OC1009_GOBJ + 0x78, r.next_u32());
        c.bus.write32(GAME_GP - 0x5b8, r.next_u32());
        c.bus.write32(GAME_GP - 0x5b4, r.next_u32());
        // the callee ids are small (they index a table)
        let ids: Vec<u32> = (0..4).map(|_| 2 + r.next_u32() % 6).collect();
        oc1009_queue(c, 0x0014_7398, &ids);
        oc1009_queue(c, 0x0014_7488, &ids);
        oc1009_queue(c, 0x0014_6e98, &ids);
        // the picture data: present or missing, per call
        let missing = |bit: u32| if (seed >> bit) & 1 == 1 { 1 } else { 0 };
        oc1009_queue(c, 0x003e_2190, &[missing(3), missing(4)]);
        let (on, alt) = ((seed >> 5) & 1, (seed >> 6) & 1);
        c.bus.write32(0x0053_4b30, on);
        c.bus.write32(0x0053_4b38, alt);
        r
    }

    /// `cFELoadScreen_LoadPictures`: type 1 (options on/off, alternate rider
    /// on/off), type 2 with one and two players, another type; present and
    /// missing picture data.
    #[test]
    fn fe_load_screen_load_pictures_matches() {
        check(
            ee::s06::f_00233640__generated,
            fe_load_screen_load_pictures,
            320,
            &|c, seed| {
                let _r = oc1009_lp_setup(c, seed, 0x0023_3640);
                let ty = [1u64, 1, 2, 3][(seed % 4) as usize];
                c.g[A0][0] = OC1009_THIS as u64;
                c.g[A1][0] = ty;
                c.bus
                    .write8(0x0053_5c11, if (seed >> 7) & 1 == 1 { 2 } else { 0 });
            },
        );
    }

    /// The gate of this module's call fakes, in each test's own RAM: the
    /// magic at `OC1016_CTRL`, the zero mask at `+8` (bit n: call n returns 0)
    /// and the result every fake returns when non-zero, at `+16`.
    pub(crate) const OC1016_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC1016_MAGIC: u32 = 0x1016_c0de;

    /// A test machine with the callee fakes armed: `forced` (0 for none) is
    /// the result of every fake, `mask` the calls that return 0.
    pub(crate) fn oc1016_machine(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        calls: u32,
        forced: u32,
        mask: u64,
    ) -> Rng {
        oc1016_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1016_CTRL, OC1016_MAGIC);
        c.bus.write64(OC1016_CTRL + 8, mask);
        c.bus.write32(OC1016_CTRL + 16, forced);
        r
    }

    /// The setup of the three load-state functions: `self` in a0, the loader
    /// object pointer in a1 whose vtable has a half-word offset and the fake
    /// call target (outside the table: the machine's miss hook).
    pub(crate) fn load_state_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 8);
        let this = DATA + 0x1000;
        let slot = DATA + 0x2000;
        let obj = DATA + 0x3000;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(slot);
        c.bus.write32(slot, obj);
        c.bus.write16(obj + 0x10, (r.next_u32() & 0xffff) as u16);
        c.bus.write32(obj + 0x14, fakes::FAKE);
    }

    #[test]
    fn game_module_request_overlay_state_global_matches() {
        check(
            ee::s06::f_00233ad0__generated,
            game_module_request_overlay_state_global,
            200,
            &|c, seed| {
                oc1016_machine(c, seed, 0x233ad0, 4, 0, (seed % 2) as u64);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x488),
                    DATA + 0x3000 + 0x100 * (seed % 3),
                );
            },
        );
    }

    #[test]
    fn game_state_load_state_matches() {
        check(
            ee::s06::f_00233b28__generated,
            game_state_load_state,
            300,
            &|c, seed| load_state_setup(c, seed, 0x233b28),
        );
    }

    #[test]
    fn game_state_quit_load_state_matches() {
        check(
            ee::s06::f_00233bb0__generated,
            game_state_quit_load_state,
            200,
            &|c, seed| load_state_setup(c, seed, 0x233bb0),
        );
    }

    #[test]
    fn game_state_idle_load_state_matches() {
        check(
            ee::s06::f_00233bf0__generated,
            game_state_idle_load_state,
            200,
            &|c, seed| load_state_setup(c, seed, 0x233bf0),
        );
    }
}
