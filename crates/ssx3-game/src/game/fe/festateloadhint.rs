//! `src/fe/festateloadhint.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::ps2f::FpuF32;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::sp;
use crate::call;
pub(crate) use crate::game::dirtysock::tags2::OPERATOR_DELETE;
pub(crate) use crate::game::dirtysock::tags2::deleting_dtor;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p as p_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::fe::fepopup2::fe_state_screen_init;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw_2;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0024_5ae0, c_fe_load_hint_state_transition_in, "cFELoadHintState_TransitionIn"),
    (0x0024_5b58, c_fe_load_state_dtor, "cFELoadState_dtor"),
    (0x0024_5b80, c_fe_load_state_init, "cFELoadState_Init"),
    (0x0024_5c60, c_fe_load_state_transition_in, "cFELoadState_TransitionIn"),
    (0x0024_5cd0, cfe_load_state_on_widget_created, "cFELoadState_OnWidgetCreated"),
    (0x0024_5d28, c_fe_load_state_in_lodge_dtor, "cFELoadStateInLodge_dtor"),
    (0x0024_5d50, c_fe_load_state_in_lodge_init, "cFELoadStateInLodge_Init"),
    (0x0024_5dc0, c_fe_load_state_in_lodge_transition_in, "cFELoadStateInLodge_TransitionIn"),
    (0x0024_5e30, game_mode_man_compare_by_score, "GameModeMan_CompareByScore"),
    (0x0024_5e78, game_mode_man_compare_by_time, "GameModeMan_CompareByTime"),
    (0x0024_5ec0, game_mode_man_compare_standings, "GameModeMan_CompareStandings"),
    (0x0024_5f00, generic_stream_pause_dtor, "GenericStreamPause_dtor"),
    (0x0024_5f50, cfe_state_progressive_verify_ps2_init, "cFEStateProgressiveVerifyPS2_Init"),
    (0x0024_6268, cfe_state_progressive_verify_ps2_on_widget_notify, "cFEStateProgressiveVerifyPS2_OnWidgetNotify"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_MANAGER_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EVENT_SELECT_ON_WIDGET_NOTIFY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_ALLOC_NAMED_ALIGNED;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_SEND_LOCK_CHAL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SHARED_LOGIN_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_VOIP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_SET_LISTENER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOAD_INDICATOR_CLEAR_BUSY;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_SPAWN_MISSIONS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_READ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_GET_WORLD_SECTION_INDEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2 as DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2 as RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2 as SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2;
}


// ---------------------------------------------------------------------------
// Game/load-state destructors: store the derived vtable and run UIState_dtor.

/// Store `vtable` at `self+0x8` and call `UIState_dtor(self)`.
pub(crate) fn ui_state_dtor_with_vtable(c: &mut Cpu, vtable: u32, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x480000;
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(vtable));
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
    call_ptr(c, callee::UI_STATE_DTOR, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}


/// `void cFELoadState_dtor(self)` (0x245b58).
pub fn c_fe_load_state_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_with_vtable(c, 0x39f8, 0x0024_5b74))
}


/// `void cFELoadStateInLodge_dtor(self)` (0x245d28).
pub fn c_fe_load_state_in_lodge_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_with_vtable(c, 0x3ac8, 0x0024_5d44))
}


/// `void GenericStreamPause_dtor(GenericStreamPause *self, int flags)`
/// (0x245f00): republishes `D_0047DB28` at `self+0`, deletes on bit 0.
pub fn generic_stream_pause_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_db28, 0, 0x0024_5f24))
}


/// `void cFELoadState_OnWidgetCreated(cFELoadState* s, UIWidget* w)`
/// (0x245cd0): set up the result widget unless the load is already running
/// in the "computer players" mode.
pub fn cfe_load_state_on_widget_created(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    if call_ptr(c, 0x0014_dc80, 0x0024_5cec).is_err() {
        return;
    }
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    if call_ptr(c, 0x0014_dd58, 0x0024_5cf8).is_err() {
        return;
    }
    c.g[V1][0] = 0x0053_0000;
    c.g[A0][0] = 2;
    c.g[V0][0] = Addr(p(c, V1, 0x5c11)).lb(c);
    let mode = c.g[V0][0] == c.g[A0][0];
    c.g[A0][0] = c.g[S0][0];
    if !mode {
        c.cycles += 2;
        c.g[A1][0] = c.g[S1][0];
        if callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET
            .call(c, 0x0024_5d14)
            .is_err()
        {
            return;
        }
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 5;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


// ---------------------------------------------------------------------------
// Front-end widget handlers.

/// The handlers that accept only the "start" widget (`key == 6`) and answer
/// 0x101 (handled) or 0: `v0 = (a2 == 6) ? 0x101 : 0`, with the original's
/// `a2 ^= 6` in place.
pub(crate) fn accept_key_6(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] ^= 0x6;
    c.g[V0][0] = if c.g[A2][0] == 0 { 0x101 } else { 0 };
    abi::ret(c);
}


/// `int cFEStateProgressiveVerifyPS2_OnWidgetNotify(State*, void*, int key)`
/// (0x00246268).
pub fn cfe_state_progressive_verify_ps2_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}


/// `int GameModeMan_CompareByScore(GameMode** a, GameMode** b)`
/// (0x00245e30): -1 when b's score is better, else 1 for unequal scores
/// and 0 for a tie.
pub fn game_mode_man_compare_by_score(c: &mut Cpu) {
    c.cycles += 13;
    let table = 0x0053_6640u32;
    let ia = Addr(abi::arg(c, 0)).lw(c) as u32;
    let ib = Addr(abi::arg(c, 1)).lw(c) as u32;
    c.g[A2][0] = sx((ia << 2).wrapping_add(table));
    c.g[V1][0] = sx((ib << 2).wrapping_add(table));
    c.g[A2][0] = Addr(c.g[A2][0] as u32).lw(c);
    c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
    let better = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[A2][0] ^ c.g[V1][0];
    c.cycles += 2;
    if better {
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    } else {
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    }
    abi::ret(c);
}


/// `int GameModeMan_CompareByTime(GameMode** a, GameMode** b)` (0x00245e78):
/// the same order with the comparison reversed (a shorter time is better).
pub fn game_mode_man_compare_by_time(c: &mut Cpu) {
    c.cycles += 13;
    let table = 0x0053_6640u32;
    let ia = Addr(abi::arg(c, 0)).lw(c) as u32;
    let ib = Addr(abi::arg(c, 1)).lw(c) as u32;
    c.g[A2][0] = sx((ia << 2).wrapping_add(table));
    c.g[V1][0] = sx((ib << 2).wrapping_add(table));
    c.g[A2][0] = Addr(c.g[A2][0] as u32).lw(c);
    c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = (c.g[A2][0] < c.g[V1][0]) as u64;
    let better = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[A2][0] ^ c.g[V1][0];
    c.cycles += 2;
    if better {
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    } else {
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    }
    abi::ret(c);
}


/// `int GameModeMan_CompareStandings(GameMode** a, GameMode** b)`
/// (0x00245ec0): 1 when b's standing is better, else -1 (never 0).
pub fn game_mode_man_compare_standings(c: &mut Cpu) {
    c.cycles += 15;
    let table = 0x0053_6708u32;
    let ia = Addr(abi::arg(c, 0)).lw(c) as u32;
    let ib = Addr(abi::arg(c, 1)).lw(c) as u32;
    c.g[A2][0] = sx(ia << 2);
    c.g[V1][0] = table as u64;
    c.g[V0][0] = sx(ib << 2);
    c.g[A1][0] = 1;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(table));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(table));
    c.g[A0][0] = c.lw(c.g[A2][0] as u32);
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[V0][0] = c.g[A1][0];
    }
    abi::ret(c);
}


/// `cFELoadStateInLodge_Init(cFELoadStateInLodge* self)` (0x245d50).
pub fn c_fe_load_state_in_lodge_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0047_bfa0, [0x0024_5d74, 0x0024_5d88, 0x0024_5da0])
    })
}


/// `a + o`, 32-bit.
#[inline(always)]
pub(crate) fn p_3(a: u32, o: u32) -> u32 {
    a.wrapping_add(o)
}


/// `sq reg, off(sp)`.
#[inline(always)]
pub(crate) fn sqsp(c: &mut Cpu, reg: usize, off: u32) {
    let a = (c.g[SP][0] as u32).wrapping_add(off);
    c.sq(reg, a);
}


/// `lq reg, off(sp)`.
#[inline(always)]
pub(crate) fn lqsp(c: &mut Cpu, reg: usize, off: u32) {
    let a = (c.g[SP][0] as u32).wrapping_add(off);
    c.lq(reg, a);
}


/// `sd v, off(sp)`.
#[inline(always)]
pub(crate) fn sdsp(c: &mut Cpu, off: u32, v: u64) {
    let a = (c.g[SP][0] as u32).wrapping_add(off);
    c.sd(a, v);
}


/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}


/// The shared body of the five load-state "transition in" functions: look the
/// screen object up by hash and, when found, format the hint text and set it.
pub(crate) fn load_state_transition_in(c: &mut Cpu, entry: u32) -> call::Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0xf0);
    sqsp(c, 16, 0xe0);
    c.g[S0][0] = c.g[A0][0];
    sdsp(c, 0xd0, c.g[RA][0]);    c.g[A0][0] = 0x004a_2bc8;
    call::call_ptr(c, 0x0031_7670, entry + 0x1c)?;
    c.g[A0][0] = Addr(p_3(r32(c, S0), 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call::call_ptr(c, 0x0039_d860, entry + 0x28)?;
    c.g[S0][0] = c.g[V0][0];
    let missing = c.g[S0][0] == 0;
    c.g[V0][0] = Addr(gp(c, 0x2a74)).lw(c);
    if !missing {
        c.cycles += 10;
        c.g[A1][0] = 0x004a_2bd0;
        c.lwc1(0, p_3(c.g[V0][0] as u32, 0x38));
        c.cvt_w_s(1, 0);
        c.mfc1(6, 1);
        c.g[A0][0] = c.g[SP][0];
        call::call_ptr(c, 0x002c_27c0, entry + 0x50)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        call::call_ptr(c, 0x003a_0e90, entry + 0x5c)?;
    }
    c.cycles += 5;
    lqsp(c, 16, 0xe0);
    c.g[V0][0] = 0;
    c.g[RA][0] = Addr(p_3(c.g[SP][0] as u32, 0xd0)).ld(c);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
    c.pc = ra;
    Ok(())
}


/// `int cFELoadHintState_TransitionIn(LoadState* s)` (0x245ae0).
pub fn c_fe_load_hint_state_transition_in(c: &mut Cpu) {
    guest(c, |c| load_state_transition_in(c, 0x0024_5ae0))
}


/// `int cFELoadState_TransitionIn(LoadState* s)` (0x245c60).
pub fn c_fe_load_state_transition_in(c: &mut Cpu) {
    guest(c, |c| load_state_transition_in(c, 0x0024_5c60))
}


/// `int cFELoadStateInLodge_TransitionIn(LoadState* s)` (0x245dc0).
pub fn c_fe_load_state_in_lodge_transition_in(c: &mut Cpu) {
    guest(c, |c| load_state_transition_in(c, 0x0024_5dc0))
}


/// `int cFEStateProgressiveVerifyPS2_OnWidgetAction(cFEState* self, void* item,
/// int msg, u8* payload)` (0x2461e0): on the "verify" widget action, allocate
/// a memory-card state and switch the UI to it.
pub fn c_fe_state_progressive_verify_ps2_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let item_null = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if item_null {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            let wrong_msg = c.g[A2][0] != c.g[V0][0];
            frame.restore(c, S0, 0x20);
            if !wrong_msg {
                c.cycles += 6;
                c.g[A0][0] = 0x4a_0000;
                c.g[S0][0] = Addr(p_4(r32(c, A1), 0x38)).lw(c);
                c.g[RA][0] = sx(0x0024_6214);
                c.g[A0][0] = sx(0x004a_2ac8);
                call_ptr(c, 0x0031_7670, 0x0024_6214)?;
                if c.g[S0][0] == c.g[V0][0] {
                    c.cycles += 13;
                    c.g[A1][0] = 0x48_0000;
                    c.g[A0][0] = 0x248;
                    c.g[A1][0] = sx(0x0047_b978);
                    c.g[A2][0] = 0;
                    c.g[RA][0] = sx(0x0024_6234);
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, 0x0024_6234)?;
                    c.g[A1][0] = Addr(p_4(r32(c, S1), 0x10)).lw(c);
                    c.g[RA][0] = sx(0x0024_6240);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0018_6a08, 0x0024_6240)?;
                    c.g[A0][0] = Addr(p_4(r32(c, S1), 0x10)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0024_6250);
                    c.g[A0][0] = sx(p_4(r32(c, A0), 0x18));
                    call_ptr(c, 0x0039_f400, 0x0024_6250)?;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// cFELoadState_Init (0x0000245b80).

/// `cFELoadState_Init`: initialise the front-end load state: kick off the load and set the progress bar.
pub fn c_fe_load_state_init(c: &mut Cpu) {
    guest(c, |c| {
    'b_245c50: {
    'b_245c4c: {
    'b_245c14: {
    c.cycles += 14;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S1][0] = c.g[A0][0];
    call_ptr(c, 0x0014dc80, 0x00245b98)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x0014dd58, 0x00245ba4)?;
    c.g[V1][0] = 0x530000;
    c.g[A0][0] = 0x2;
    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
    if c.g[V0][0] != c.g[A0][0] {
    c.g[A0][0] = 0x480000;
    break 'b_245c14;
    } else {
    c.cycles += 11;
    c.g[A0][0] = 0x480000;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x41c0));
    call_ptr(c, 0x00317670, 0x00245bc8)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00397cc0, 0x00245bdc)?;
    let t = c.g[V0][0] == 0u64;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
    if t {
    break 'b_245c4c;
    } else {
    c.cycles += 12;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0u64;
    call_ptr(c, 0x0039c870, 0x00245bf4)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x002472c0, 0x00245c00)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0x1;
    call_ptr(c, 0x002472c0, 0x00245c0c)?;
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    if t {
    break 'b_245c50;
    } else {
    break 'b_245c14;
    }
    }
    }
    }
    c.cycles += 10;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x41b0));
    call_ptr(c, 0x00317670, 0x00245c20)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00397cc0, 0x00245c34)?;
    let t = c.g[V0][0] == 0u64;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
    if t {
    break 'b_245c4c;
    } else {
    c.cycles += 4;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0u64;
    call_ptr(c, 0x0039c870, 0x00245c4c)?;
    break 'b_245c4c;
    }
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    break 'b_245c50;
    }
    c.cycles += 4;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t; return Ok(());
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// cFEStateProgressiveVerifyPS2_Init (0x245f50).

/// The confirm window of the progressive-scan check (`+0x4c`), counted down
/// once per update by `cFEStateProgressiveVerifyPS2_Update`: 300 frames at
/// 60 Hz (5 s).
const PROGRESSIVE_VERIFY_FRAMES: u32 = 300;

/// `void cFEStateProgressiveVerifyPS2_Init(cFEStateProgressiveVerifyPS2* self)`
/// (0x245f50): create the screen (`+0x40`) from the state's screen set
/// (`+0x10`), set the message text (mode `+0x48` picks one of two strings),
/// fill the second text object, reset the button list, and arm the
/// 5-second confirm countdown (`+0x4c`).
///
/// Tick rate: the countdown takes one per update, so it is stored in ticks
/// (`ticks_from_frames60(300)`; 300 at 60).
pub fn cfe_state_progressive_verify_ps2_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        let this = r32(c, S1);
        c.g[A0][0] = 0x004a_2ad8;
        c.g[S0][0] = Addr(this).off(0x10).lw(c);
        call_ptr(c, 0x0031_7670, 0x0024_5f74)?; // hash of the screen name
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0024_5f88)?; // create the screen
        Addr(this).off(0x40).sw(c, c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0024_5fa0)?;
        }

        // The message text: one of two strings by the mode byte +0x48.
        c.cycles += 9;
        c.g[A0][0] = 0x004a_2ac0;
        call_ptr(c, 0x0031_7670, 0x0024_5fac)?;
        c.g[A0][0] = Addr(this).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0024_5fb8)?; // find the object by hash
        c.g[A0][0] = c.g[V0][0];
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(0x48).lb(c);
            c.g[A1][0] = 0x0048_0000;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x46d0));
                call_ptr(c, 0x003a_0c50, 0x0024_5fec)?;
            } else {
                c.cycles += 5;
                c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x4708));
                call_ptr(c, 0x003a_0c50, 0x0024_5fdc)?;
            }
        }
        c.g[A0][0] = 0x004a_0000;

        // The second text object.
        c.cycles += 8;
        c.g[A0][0] = 0x004a_2ac8;
        call_ptr(c, 0x0031_7670, 0x0024_5ff8)?;
        c.g[A0][0] = Addr(this).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0024_6004)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 7;
            c.g[A0][0] = sx(0x0048_0000u32.wrapping_sub(0x4698));
            call_ptr(c, 0x0031_7670, 0x0024_601c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x0024_6028)?;
        }
        c.g[A0][0] = 0x004a_0000;

        // The button list: virtual slots +0x48 (reset) and +0x40 (select 1).
        c.cycles += 8;
        c.g[A0][0] = 0x004a_2ad0;
        call_ptr(c, 0x0031_7670, 0x0024_6034)?;
        c.g[A0][0] = Addr(this).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0024_6040)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        if c.g[S0][0] != 0 {
            c.cycles += 11;
            let list = r32(c, S0);
            c.g[V0][0] = Addr(list).off(8).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x48).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x4c).lw(c);
            c.g[A0][0] = sx(list.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, r32(c, V1), 0x0024_6060)?;
            c.g[V0][0] = Addr(list).off(8).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = Addr(r32(c, V0)).off(0x40).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x44).lw(c);
            c.g[A0][0] = sx(list.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, r32(c, V1), 0x0024_6078)?;
        }

        c.cycles += 7;
        c.g[V0][0] = crate::tick::ticks_from_frames60(c, PROGRESSIVE_VERIFY_FRAMES) as u64;
        frame.restore(c, S0, 0x20);
        Addr(this).off(0x4c).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::compare;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2 as rng;
    pub(crate) use crate::game::dirtysock::tags2::tests::dtor_setup;
    pub(crate) use crate::game::ai::ai::tests::belibrary;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_8 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;


    // ---------------------------------------------------------- load states

    /// A zeroed load-state object; `a1`'s low bit selects whether UIState_dtor
    /// frees it, so keep it clear (the object is not a heap block).
    pub(crate) fn load_state_dtor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = rng(seed, salt);
        scramble(c, &mut r);
        c.g[A0][0] = sx(DATA + 0x4000);
        c.g[A1][0] = ((r.next_u32() % 4) * 2) as u64;
        for off in [0x24u32, 0x2c, 0x30, 0x38, 0x3c] {
            c.bus.write32(DATA + 0x4000 + off, 0);
        }
    }


    #[test]
    fn c_fe_load_state_dtor_matches() {
        check(ssx3_ee::s06::f_00245b58__generated, c_fe_load_state_dtor, 80, &|c, seed| {
            load_state_dtor_setup(c, seed, 0x23)
        });
    }


    #[test]
    fn c_fe_load_state_in_lodge_dtor_matches() {
        check(ssx3_ee::s06::f_00245d28__generated, c_fe_load_state_in_lodge_dtor, 80, &|c, seed| {
            load_state_dtor_setup(c, seed, 0x24)
        });
    }


    #[test]
    fn generic_stream_pause_dtor_matches() {
        check(ee::s06::f_00245f00__generated, generic_stream_pause_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x245f00)
        });
    }


    #[test]
    fn cfe_load_state_on_widget_created_matches() {
        check(
            ssx3_ee::s06::f_00245cd0__generated,
            cfe_load_state_on_widget_created,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x245cd0, 2);
                belibrary(c, &mut r);
                // Mode 2 is the "computer players" load: the result widget setup
                // (which needs the whole FE screen) is skipped.
                c.bus.write8(0x0053_5c11, 2);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (DATA + 0x500) as u64;
            },
        );
    }


    // --------------------------------------------------------- game mode tables

    /// The sort keys behind `GameModeMan_Compare*` live in `.bss` tables.
    pub(crate) fn mode_tables(c: &mut Cpu, r: &mut Rng) {
        for i in 0..8 {
            word(c, 0x0053_6640 + 4 * i, r.next_u32());
            word(c, 0x0053_6708 + 4 * i, r.next_u32());
        }
    }


    #[test]
    fn game_mode_man_compares_match() {
        check(ssx3_ee::s06::f_00245e30__generated, game_mode_man_compare_by_score, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x245e30, 0);
            mode_tables(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 4);
            word(c, DATA, (r.next_u32() % 8) as u32);
            word(c, DATA + 4, (r.next_u32() % 8) as u32);
        });
        check(ssx3_ee::s06::f_00245e78__generated, game_mode_man_compare_by_time, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x245e78, 0);
            mode_tables(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 4);
            word(c, DATA, (r.next_u32() % 8) as u32);
            word(c, DATA + 4, (r.next_u32() % 8) as u32);
        });
        check(ssx3_ee::s06::f_00245ec0__generated, game_mode_man_compare_standings, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x245ec0, 0);
            mode_tables(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 4);
            word(c, DATA, (r.next_u32() % 8) as u32);
            word(c, DATA + 4, (r.next_u32() % 8) as u32);
        });
    }


    #[test]
    fn c_fe_load_state_in_lodge_init_matches() {
        check(
            ssx3_ee::s06::f_00245d50__generated,
            c_fe_load_state_in_lodge_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x245d50);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }


    pub(crate) fn w32(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }


    pub(crate) const SEEDS_2: u32 = 200;


    pub(crate) fn load_state_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = machine(c, seed, salt, 0);
        let s = DATA + 0x1000;
        let screen = DATA + 0x2000;
        c.g[A0][0] = s as u64;
        w32(c, s + 0x40, screen);
        // No objects in the screen: the transition returns without touching
        // the string formatting path.
        w32(c, screen + 0x3c, 0);
    }


    #[test]
    fn c_fe_load_hint_state_transition_in_matches() {
        check(
            ssx3_ee::s06::f_00245ae0__generated,
            c_fe_load_hint_state_transition_in,
            SEEDS_2,
            &|c, seed| load_state_fixture(c, seed, 0x245ae0),
        );
    }


    #[test]
    fn c_fe_load_state_transition_in_matches() {
        check(
            ssx3_ee::s06::f_00245c60__generated,
            c_fe_load_state_transition_in,
            SEEDS_2,
            &|c, seed| load_state_fixture(c, seed, 0x245c60),
        );
    }


    #[test]
    fn c_fe_load_state_in_lodge_transition_in_matches() {
        check(
            ssx3_ee::s06::f_00245dc0__generated,
            c_fe_load_state_in_lodge_transition_in,
            SEEDS_2,
            &|c, seed| load_state_fixture(c, seed, 0x245dc0),
        );
    }


    #[test]
    fn c_fe_load_state_init_matches() {
        diff(ee::s06::f_00245b80__generated, c_fe_load_state_init, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x24_5b80);
            c.g[A0][0] = sx(DATA);
            w(c, DATA + 0x10, r.next_u32());
            w(c, DATA + 0x20, r.next_u32());
        });
    }


    /// The progressive-scan check's screen setup: the mode byte `+0x48`
    /// picks the message (both strings), the screen set `+0x10` is random.
    pub(crate) fn progressive_verify_init_setup(c: &mut Cpu, seed: u32) {
        let mut r = fe_base(c, seed, 0x24_5f50);
        c.g[A0][0] = sx(DATA);
        w(c, DATA + 0x10, r.next_u32());
        c.bus.write8(DATA + 0x48, (seed % 2) as u8);
        w(c, DATA + 0x4c, r.next_u32());
    }


    #[test]
    fn cfe_state_progressive_verify_ps2_init_matches() {
        diff(
            ee::s06::f_00245f50__generated,
            cfe_state_progressive_verify_ps2_init,
            20,
            &progressive_verify_init_setup,
        );
    }


    /// The confirm countdown is 300 frames at 60 Hz: 300 ticks at 60, 600
    /// at 120 and 1200 at 240 (5 s at every rate).
    #[test]
    fn cfe_state_progressive_verify_ps2_init_follows_the_rate() {
        for hz in [60u32, 120, 240] {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            progressive_verify_init_setup(c, 1);
            crate::tick::set_for_test(c, hz);
            c.g[RA][0] = sx(crate::difftest::RA);
            cfe_state_progressive_verify_ps2_init(c);
            assert_eq!(c.pc, crate::difftest::RA);
            let ticks = c.bus.read32(DATA + 0x4c);
            assert_eq!(ticks, 300 * hz / 60, "{hz} Hz");
        }
    }
}
