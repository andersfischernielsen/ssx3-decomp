//! `src/fe/festateloadhint.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
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
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::dirtysock::tagsunk::deleting_dtor;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::festatenethelppopup::fe_state_screen_init;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0024_5ae0,
        c_fe_load_hint_state_transition_in,
        "cFELoadHintState_TransitionIn",
    ),
    (0x0024_5b58, c_fe_load_state_dtor, "cFELoadState_dtor"),
    (0x0024_5b80, c_fe_load_state_init, "cFELoadState_Init"),
    (
        0x0024_5c60,
        c_fe_load_state_transition_in,
        "cFELoadState_TransitionIn",
    ),
    (
        0x0024_5cd0,
        cfe_load_state_on_widget_created,
        "cFELoadState_OnWidgetCreated",
    ),
    (
        0x0024_5d28,
        c_fe_load_state_in_lodge_dtor,
        "cFELoadStateInLodge_dtor",
    ),
    (
        0x0024_5d50,
        c_fe_load_state_in_lodge_init,
        "cFELoadStateInLodge_Init",
    ),
    (
        0x0024_5dc0,
        c_fe_load_state_in_lodge_transition_in,
        "cFELoadStateInLodge_TransitionIn",
    ),
    (
        0x0024_5e30,
        game_mode_man_compare_by_score,
        "GameModeMan_CompareByScore",
    ),
    (
        0x0024_5e78,
        game_mode_man_compare_by_time,
        "GameModeMan_CompareByTime",
    ),
    (
        0x0024_5ec0,
        game_mode_man_compare_standings,
        "GameModeMan_CompareStandings",
    ),
    (
        0x0024_5f00,
        generic_stream_pause_dtor,
        "GenericStreamPause_dtor",
    ),
    (
        0x0024_5f50,
        cfe_state_progressive_verify_ps2_init,
        "cFEStateProgressiveVerifyPS2_Init",
    ),
    (
        0x0024_6268,
        cfe_state_progressive_verify_ps2_on_widget_notify,
        "cFEStateProgressiveVerifyPS2_OnWidgetNotify",
    ),
    (
        0x0024_64e8,
        c_game_load_state_setup_qp_widget,
        "cGameLoadState_SetupQPWidget",
    ),
    (
        0x0024_6cf8,
        game_load_state_update_player_ranking,
        "cGameLoadState_UpdatePlayerRanking",
    ),
    (
        0x0024_5950,
        c_fe_load_hint_state_init,
        "cFELoadHintState_Init",
    ),
    (
        0x0024_5f30,
        c_fe_state_progressive_verify_ps2_dtor,
        "cFEStateProgressiveVerifyPS2_dtor",
    ),
    (
        0x0024_6098,
        c_fe_state_progressive_verify_ps2_update,
        "cFEStateProgressiveVerifyPS2_Update",
    ),
    (
        0x0024_61e0,
        cfe_progressive_verify_on_widget_action,
        "cFEStateProgressiveVerifyPS2_OnWidgetAction",
    ),
    (
        0x0024_6278,
        c_game_load_state_setup_mp_widget,
        "cGameLoadState_SetupMPWidget",
    ),
    (
        0x0024_72c0,
        cfe_load_state_show_mp_player_stats,
        "cFELoadState_ShowMPPlayerStats",
    ),
    (
        0x0024_7ab0,
        fe_load_state_setup_result_widget,
        "cFELoadState_SetupResultWidget",
    ),
    (0x0024_7e20, ssxapp_global_ctors, "SSXApp_GlobalCtors"),
    (0x0024_7e40, ssxapp_global_dtors, "SSXApp_GlobalDtors"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_STATE_DTOR;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
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
    let _frame = Frame::open(c, 0xf0);
    sqsp(c, 16, 0xe0);
    c.g[S0][0] = c.g[A0][0];
    sdsp(c, 0xd0, c.g[RA][0]);
    c.g[A0][0] = 0x004a_2bc8;
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
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
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
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
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

// ---------------------------------------------------------------------------
// cGameLoadState_SetupQPWidget (0x2464e8)
// ---------------------------------------------------------------------------

/// The widget's id (the `StrHash` of its name).
pub(crate) const QPW_WIDGET_ID: u32 = 0x38;

/// The widget's vtable pointer (slot 9 at +0x48: `SetVisible(bool)`-like).
pub(crate) const QPW_WIDGET_VT: u32 = 0x08;

/// `cBENewRaceInterface_newRace.cGameMode` (a signed byte).
#[cfg(test)]
pub(crate) const QPW_GAME_MODE: u32 = 0x0053_5bc8 + 0x48;

/// The load state's hint record (+0x48) and its texture handle (+0x10).
pub(crate) const QPW_STATE_HINT: u32 = 0x48;

/// The game-mode jump tables (.rodata).
pub(crate) const QPW_MODE_TABLE_1: u32 = 0x0047_db70;

pub(crate) const QPW_MODE_TABLE_2: u32 = 0x0047_db90;

pub(crate) const QPW_HASH: u32 = 0x0031_7670;

pub(crate) const QPW_GET_BE: u32 = 0x0014_dc80;

pub(crate) const QPW_GET_INTERFACE: u32 = 0x0014_dd58;

pub(crate) const QPW_SET_WIDE: u32 = 0x003a_0e90;

pub(crate) const QPW_SPRINTF: u32 = 0x002c_26d0;

pub(crate) const QPW_STRCPY: u32 = 0x002c_2508;

pub(crate) const QPW_GET_TRACK: u32 = 0x0014_4be0;

pub(crate) const QPW_GET_HUMAN_RIDER: u32 = 0x0014_6e98;

pub(crate) const QPW_START: u32 = 0x0024_64e8;

pub(crate) const QPW_END: u32 = 0x0024_6cf4;

/// `sp + off` as a register value.
pub(crate) fn qpw_sp(c: &Cpu, off: u32) -> u64 {
    sx(sp(c).wrapping_add(off))
}

/// `s0 = widget id; a0 = key; s0 == StrHash(key)?` (the `IsId` test the
/// function repeats for every widget name), returning to `ret`.
pub(crate) fn qpw_is_id(c: &mut Cpu, key: u32, ret: u32) -> Result<bool, Unwound> {
    c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
    c.g[A0][0] = sx(key);
    call_ptr(c, QPW_HASH, ret)?;
    Ok(c.g[S0][0] == c.g[V0][0])
}

/// The locale string of the key in `a0`: `locale = world->locale` (from
/// `world`), `locale->vt[4](locale + delta, StrHash(key))` (0x20/0x24).
pub(crate) fn qpw_locale_get(c: &mut Cpu, world: u32, ret_hash: u32, ret_get: u32) -> Flow {
    c.g[S0][0] = c.lw(world.wrapping_add(0x8c));
    c.g[S1][0] = c.lw(r32(c, S0).wrapping_add(4));
    c.g[V0][0] = c.lh(r32(c, S1).wrapping_add(0x20));
    c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
    call_ptr(c, QPW_HASH, ret_hash)?;
    c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x24));
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    let f = r32(c, V1);
    call_ptr(c, f, ret_get)
}

/// The epilogue from 0x246cd4 (`lq s1`; `s0` is already restored).
pub(crate) fn qpw_exit_cd4(c: &mut Cpu) {
    c.cycles += 1;
    let frame = Frame { size: 0x380 };
    frame.restore(c, S1, 0x360);
    qpw_exit_cd8(c);
}

/// The epilogue from 0x246cd8 (`s0`, `s1` restored).
pub(crate) fn qpw_exit_cd8(c: &mut Cpu) {
    c.cycles += 7;
    let frame = Frame { size: 0x380 };
    frame.restore(c, S2, 0x350);
    frame.restore(c, S3, 0x340);
    frame.restore(c, S4, 0x330);
    frame.restore(c, S5, 0x320);
    frame.restore_ra(c, 0x310);
    frame.close(c);
}

/// `lq s0` and the epilogue (the common exit after the last call).
pub(crate) fn qpw_exit(c: &mut Cpu) {
    Frame { size: 0x380 }.restore(c, S0, 0x370);
    qpw_exit_cd4(c);
}

/// Show the widget (0x246a2c): `widget->vt[9](widget + delta, 1)`; `v0`
/// holds the vtable.
pub(crate) fn qpw_activate(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    let vt = r32(c, V0);
    c.g[A1][0] = 1;
    c.g[A0][0] = c.lh(vt.wrapping_add(0x48));
    c.g[V1][0] = c.lw(vt.wrapping_add(0x4c));
    let f = r32(c, V1);
    c.g[A0][0] = sx(r32(c, S5).wrapping_add(r32(c, A0)));
    call_ptr(c, f, 0x0024_6a40)?;
    qpw_exit(c);
    Ok(())
}

/// The QP (track peak) line (0x246694): `sprintf(buf, "%s %s", Loc(peak
/// name), track name)` for peaks 0..2, then `SetWideString(widget, buf)`.
pub(crate) fn qpw_peak_line(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    c.g[A0][0] = qpw_sp(c, 0xa0);
    c.g[A1][0] = 0;
    c.g[S4][0] = c.g[A0][0];
    c.g[A2][0] = 0xc8;
    call_ptr(c, 0x0041_6210, 0x0024_66a8)?;
    c.g[V1][0] = 0x004a_0000;
    c.g[A0][0] = c.g[S3][0];
    c.g[S2][0] = sx(0x004a_2bd8);
    call_ptr(c, 0x0014_4c98, 0x0024_66b8)?;
    let peak = c.g[V0][0];
    c.g[V1][0] = peak;
    c.g[V0][0] = ((peak as i64) < 2) as u64;
    // The format block's return addresses, or no line.
    let line: Option<[u32; 4]> = if peak == 1 {
        c.cycles += 3;
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
        c.g[A0][0] = sx(0x0047_ba68);
        c.cycles += 19;
        Some([0x0024_671c, 0x0024_672c, 0x0024_6738, 0x0024_674c])
    } else {
        c.cycles += 2;
        let below2 = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if below2 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
            if peak == 0 {
                c.cycles += 3 + 19;
                c.g[A0][0] = sx(0x0047_ba58);
                Some([0x0024_671c, 0x0024_672c, 0x0024_6738, 0x0024_674c])
            } else {
                c.cycles += 2;
                None
            }
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S5][0];
            if peak == 2 {
                c.cycles += 21;
                c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
                c.g[A0][0] = sx(0x0047_ba78);
                Some([0x0024_6774, 0x0024_6784, 0x0024_6790, 0x0024_67a4])
            } else {
                c.cycles += 2;
                None
            }
        }
    };
    if let Some(rets) = line {
        let world = r32(c, V0);
        qpw_locale_get(c, world, rets[0], rets[1])?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0014_4bf0, rets[2])?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x002c_27c0, rets[3])?;
    }
    c.g[A0][0] = c.g[S5][0];
    // 0x2467a8
    c.cycles += 4;
    c.g[A1][0] = c.g[S4][0];
    call_ptr(c, QPW_SET_WIDE, 0x0024_67b0)?;
    qpw_exit(c);
    Ok(())
}

/// One game-mode case of the first switch: (key, return address, last case).
pub(crate) fn qpw_mode_case_1(target: u32) -> Option<(u32, u32, bool)> {
    Some(match target {
        0x0024_67e8 => (0x0047_bb80, 0x0024_67f8, false),
        0x0024_6808 => (0x0047_bb98, 0x0024_6818, false),
        0x0024_6828 => (0x0047_bba8, 0x0024_6838, false),
        0x0024_6848 => (0x0047_bbb8, 0x0024_6858, false),
        0x0024_6868 => (0x0047_bb80, 0x0024_6878, false),
        0x0024_6888 => (0x0047_bbc8, 0x0024_6898, false),
        0x0024_68a8 => (0x0047_bbc8, 0x0024_68b8, true),
        _ => return None,
    })
}

/// One game-mode case of the second switch: `Some(None)` is the empty case.
pub(crate) fn qpw_mode_case_2(target: u32) -> Option<Option<(u32, u32, bool)>> {
    Some(Some(match target {
        0x0024_68ec => (0x004a_2be0, 0x0024_68fc, false),
        0x0024_690c => (0x004a_2be8, 0x0024_691c, false),
        0x0024_692c => (0x004a_2bf0, 0x0024_693c, false),
        0x0024_694c => (0x004a_2bf8, 0x0024_695c, false),
        0x0024_696c => (0x0047_bbd8, 0x0024_697c, false),
        0x0024_698c => (0x0047_bbe8, 0x0024_699c, true),
        0x0024_69a4 => return Some(None),
        _ => return None,
    }))
}

/// A `sprintf(buf, Loc("..."), table[track])` line (0x246a5c / 0x246ad4):
/// the format at `sp+fmt`, the value from the stack table at `sp+table`.
pub(crate) fn qpw_track_table_line(c: &mut Cpu, fmt: u32, from_s4: bool, rets: [u32; 4]) -> Flow {
    c.cycles += 25;
    c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
    c.g[S2][0] = qpw_sp(c, fmt);
    c.g[A0][0] = sx(0x0047_bc30);
    let world = r32(c, V0);
    qpw_locale_get(c, world, rets[0], rets[1])?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, QPW_STRCPY, rets[2])?;
    c.g[S0][0] = qpw_sp(c, 0xa0);
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, QPW_GET_TRACK, rets[3])?;
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[A1][0] = c.g[S2][0];
    let at = if from_s4 {
        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        r32(c, V0)
    } else {
        c.g[V1][0] = sx(sp(c).wrapping_add(r32(c, V0)));
        r32(c, V1)
    };
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.lw(at);
    Ok(())
}

/// `void cGameLoadState_SetupQPWidget(cGameLoadState* self, cUIText* widget)`
/// (0x2464e8): fills in one widget of the race load screen by its name hash:
/// the track-peak line, the game-mode and track-location icons (shown via
/// vtable slot 9), the two per-track statistics lines (from two 17-entry
/// tables copied to the stack), the rider's short name, the rider's overall
/// rating line, or the hint texture handle.
pub fn c_game_load_state_setup_qp_widget(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x380);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S5, 0x320);
        c.g[A2][0] = sx(0x0047_bae0);
        frame.save(c, S2, 0x350);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S0, 0x370);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x360);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S3, 0x340);
        c.g[V0][0] = 0; // 0x47bae0 & 7: the aligned copy
        frame.save(c, S4, 0x330);
        frame.save_ra(c, 0x310);

        // tA = table 0x47bae0 (17 words) to sp+0.
        c.cycles += 4;
        c.g[V1][0] = sx(0x0047_bb20);
        c.g[V0][0] = 0x0048_0000;
        c.g[S4][0] = qpw_sp(c, 0x50);
        c.g[S0][0] = 0x0048_0000;
        loop {
            c.cycles += 11;
            let src = r32(c, A2);
            let dst = r32(c, A1);
            c.g[T2][0] = c.ld(src);
            c.g[A0][0] = c.ld(src.wrapping_add(8));
            c.g[A3][0] = c.ld(src.wrapping_add(0x10));
            c.g[T0][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[T2][0]);
            c.sd(dst.wrapping_add(8), c.g[A0][0]);
            c.sd(dst.wrapping_add(0x10), c.g[A3][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T0][0]);
            c.g[A2][0] = sx(src.wrapping_add(0x20));
            c.g[A1][0] = sx(dst.wrapping_add(0x20));
            if c.g[A2][0] == c.g[V1][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0024_6598);
            }
        }
        c.cycles += 1;
        c.g[A0][0] = sx(0x0047_bb28);
        c.cycles += 5;
        c.g[T1][0] = c.lw(r32(c, A2));
        c.g[V1][0] = c.g[S4][0];
        c.g[V0][0] = 0; // 0x47bb28 & 7
        c.sw(r32(c, A1), c.g[T1][0]);

        // tB = table 0x47bb28 (17 words) to sp+0x50.
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x40));
        loop {
            c.cycles += 11;
            let src = r32(c, A0);
            let dst = r32(c, V1);
            c.g[A3][0] = c.ld(src);
            c.g[T0][0] = c.ld(src.wrapping_add(8));
            c.g[T1][0] = c.ld(src.wrapping_add(0x10));
            c.g[T2][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[A3][0]);
            c.sd(dst.wrapping_add(8), c.g[T0][0]);
            c.sd(dst.wrapping_add(0x10), c.g[T1][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T2][0]);
            c.g[A0][0] = sx(src.wrapping_add(0x20));
            c.g[V1][0] = sx(dst.wrapping_add(0x20));
            if c.g[A0][0] == c.g[V0][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0024_6638);
            }
        }
        c.cycles += 12;
        c.g[V0][0] = c.lw(r32(c, A0));
        c.sw(r32(c, V1), c.g[V0][0]);

        // iface0 = BELibrary_GetInstance()->GetInterface(0) (s3).
        call_ptr(c, QPW_GET_BE, 0x0024_6670)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, QPW_GET_INTERFACE, 0x0024_667c)?;
        c.g[A0][0] = sx(0x0047_bb70);
        c.g[S3][0] = c.g[V0][0];
        c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
        call_ptr(c, QPW_HASH, 0x0024_668c)?;
        let is_peak = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = 0x0053_0000;
        if is_peak {
            return qpw_peak_line(c);
        }

        // First game-mode switch (0x2467b8).
        c.cycles += 6;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x5bc8));
        c.g[A0][0] = c.lb(r32(c, V0).wrapping_add(0x48));
        let in_range = c.g[A0][0] < 7;
        c.g[V1][0] = in_range as u64;
        c.g[V0][0] = 0x0048_0000;
        if in_range {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, A0) << 2);
            c.g[V0][0] = sx(QPW_MODE_TABLE_1);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.lw(r32(c, V1));
            let target = r32(c, A0);
            let Some((key, ret_at, last)) = qpw_mode_case_1(target) else {
                c.jump_out(target, QPW_START, QPW_END);
                return Err(Unwound);
            };
            c.cycles += 6;
            let hit = qpw_is_id(c, key, ret_at)?;
            if last {
                if hit {
                    c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_VT));
                    return qpw_activate(c);
                }
                c.cycles += 1;
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x5bc8));
            } else {
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x5bc8));
                if hit {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_VT));
                    return qpw_activate(c);
                }
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x5bc8));
        }

        // Second game-mode switch (0x2468c4).
        c.cycles += 4;
        c.g[A0][0] = c.lb(r32(c, V0).wrapping_add(0x48));
        let in_range = c.g[A0][0] < 8;
        c.g[V1][0] = in_range as u64;
        c.g[V0][0] = 0x0048_0000;
        if in_range {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, A0) << 2);
            c.g[V0][0] = sx(QPW_MODE_TABLE_2);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.lw(r32(c, V1));
            let target = r32(c, A0);
            let Some(case) = qpw_mode_case_2(target) else {
                c.jump_out(target, QPW_START, QPW_END);
                return Err(Unwound);
            };
            if let Some((key, ret_at, last)) = case {
                c.cycles += 6;
                if qpw_is_id(c, key, ret_at)? {
                    if !last {
                        c.cycles += 2;
                    }
                    c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_VT));
                    return qpw_activate(c);
                }
            }
        }

        // Track-location icons (0x2469a4).
        c.cycles += 9;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, QPW_GET_TRACK, 0x0024_69ac)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0014_4d18, 0x0024_69b8)?;
        let kind = c.g[V0][0];
        c.g[V1][0] = kind;
        c.g[V0][0] = ((kind as i64) < 2) as u64;
        // The location key to test, or None to go on with the line widgets
        // (`true`: s0 already loaded, from 0x246a4c).
        let probe: Result<u32, bool> = if kind == 1 {
            c.cycles += 4;
            c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
            Ok(0x0047_bbf8)
        } else {
            c.cycles += 2;
            let below2 = c.g[V0][0] != 0;
            c.g[V0][0] = 2;
            c.cycles += 2;
            if below2 {
                if kind == 0 {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
                    Ok(0x004a_2c00)
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = 0x0048_0000;
                    Err(false)
                }
            } else {
                c.g[A0][0] = 0x0048_0000;
                if kind == 2 {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
                    Ok(0x0047_bc08)
                } else {
                    c.cycles += 2;
                    c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
                    Err(true)
                }
            }
        };
        let s0_loaded = match probe {
            Ok(key) => {
                c.g[A0][0] = sx(key);
                c.cycles += 4;
                call_ptr(c, QPW_HASH, 0x0024_6a20)?;
                let hit = c.g[S0][0] == c.g[V0][0];
                c.g[A0][0] = 0x0048_0000;
                if hit {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_VT));
                    return qpw_activate(c);
                }
                false
            }
            Err(loaded) => loaded,
        };
        if !s0_loaded {
            c.cycles += 1;
            c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
        }

        // The statistics line from tB (0x246a4c).
        c.cycles += 4;
        c.g[A0][0] = sx(0x0047_bc18);
        call_ptr(c, QPW_HASH, 0x0024_6a54)?;
        if c.g[S0][0] == c.g[V0][0] {
            qpw_track_table_line(
                c,
                0x170,
                true,
                [0x0024_6a80, 0x0024_6a90, 0x0024_6a9c, 0x0024_6aa8],
            )?;
            return qpw_sprintf_and_set(c);
        }
        c.g[A0][0] = 0x0048_0000;

        // The statistics line from tA (0x246ac0).
        c.cycles += 5;
        if qpw_is_id(c, 0x0047_bc48, 0x0024_6acc)? {
            qpw_track_table_line(
                c,
                0x240,
                false,
                [0x0024_6af8, 0x0024_6b08, 0x0024_6b14, 0x0024_6b20],
            )?;
            return qpw_sprintf_and_set(c);
        }
        c.g[A0][0] = 0x0048_0000;

        // The rider's short name (0x246b38).
        c.cycles += 5;
        let hit = qpw_is_id(c, 0x0047_bc60, 0x0024_6b44)?;
        c.g[A0][0] = 0x0048_0000;
        if hit {
            c.cycles += 22;
            call_ptr(c, QPW_GET_BE, 0x0024_6b54)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            call_ptr(c, QPW_GET_INTERFACE, 0x0024_6b60)?;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, QPW_GET_BE, 0x0024_6b68)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 2;
            call_ptr(c, QPW_GET_INTERFACE, 0x0024_6b74)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            call_ptr(c, QPW_GET_HUMAN_RIDER, 0x0024_6b84)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_ee58, 0x0024_6b90)?;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x0024_6b9c)?;
            qpw_exit(c);
            return Ok(());
        }

        // The rider's overall rating line (0x246ba4).
        c.cycles += 5;
        let hit = qpw_is_id(c, 0x0047_bc70, 0x0024_6bb0)?;
        c.g[A0][0] = 0x004a_0000;
        if hit {
            c.cycles += 50;
            call_ptr(c, QPW_GET_BE, 0x0024_6bc0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            call_ptr(c, QPW_GET_INTERFACE, 0x0024_6bcc)?;
            c.g[S4][0] = c.g[V0][0];
            call_ptr(c, QPW_GET_BE, 0x0024_6bd4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 3;
            call_ptr(c, QPW_GET_INTERFACE, 0x0024_6be0)?;
            c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
            c.g[S2][0] = c.g[V0][0];
            c.g[S3][0] = qpw_sp(c, 0x170);
            c.g[A0][0] = sx(0x0047_bc80);
            let world = r32(c, V1);
            qpw_locale_get(c, world, 0x0024_6c08, 0x0024_6c18)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, QPW_STRCPY, 0x0024_6c24)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0;
            call_ptr(c, QPW_GET_HUMAN_RIDER, 0x0024_6c30)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_8950, 0x0024_6c3c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0;
            call_ptr(c, QPW_GET_HUMAN_RIDER, 0x0024_6c4c)?;
            // rating = riders[idx].flags & 2 ? 1 : rating (0x5308b8, 12 bytes each).
            c.g[V1][0] = 0xc;
            c.g[A3][0] = 0x0053_0000;
            c.mult(0, V0, V0, V1);
            c.g[A3][0] = sx(0x0053_08b8);
            c.g[S0][0] = qpw_sp(c, 0xa0);
            c.g[A2][0] = 1;
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
            c.g[V1][0] = c.lw(r32(c, V0));
            c.g[V1][0] = sx(r32(c, V1) >> 1);
            c.g[V1][0] &= 1;
            if c.g[V1][0] == 0 {
                c.g[A2][0] = c.g[S1][0];
            }
            return qpw_sprintf_and_set(c);
        }

        // The hint texture (0x246c9c).
        c.cycles += 5;
        let hit = qpw_is_id(c, 0x004a_2c08, 0x0024_6ca8)?;
        frame.restore(c, S0, 0x370);
        if !hit {
            qpw_exit_cd4(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(QPW_STATE_HINT));
        frame.restore(c, S1, 0x360);
        if c.g[A0][0] == 0 {
            qpw_exit_cd8(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0x10));
        if c.g[A0][0] == 0 {
            qpw_exit_cd8(c);
            return Ok(());
        }
        c.cycles += 3;
        let widget = r32(c, S5);
        c.sw(widget.wrapping_add(0x78), c.g[A0][0]);
        c.sw(widget.wrapping_add(0x7c), 0);
        qpw_exit(c);
        Ok(())
    })
}

/// 0x246c80: `UStr_Sprintf(a0, a1, a2)`, then `SetWideString(widget, s0)`.
pub(crate) fn qpw_sprintf_and_set(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    call_ptr(c, QPW_SPRINTF, 0x0024_6c88)?;
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, QPW_SET_WIDE, 0x0024_6c94)?;
    qpw_exit(c);
    Ok(())
}

/// `cGameLoadState_UpdatePlayerRanking` (0x00246cf8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn game_load_state_update_player_ranking(c: &mut Cpu) {
    if c.pc != 0x00246cf8 {
        return ee::s06::f_00246cf8__generated(c);
    }
    guest(c, |c| {
        'b_247294: {
            c.cycles += 25;
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x210));
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x150), c.g[A0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x154), c.g[A1][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1f0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1e0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1d0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1c0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1b0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x190));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x180));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x170), c.g[RA][0]);
            c.g[RA][0] = sx(0x00246d3c);
            call_ptr(c, 0x0014dc80, 0x00246d3c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00246d48);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00246d48)?;
            c.g[V1][0] = 0x530000;
            c.g[A0][0] = 0x2;
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
            let t = c.g[V0][0] != c.g[A0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
            if t {
                break 'b_247294;
            } else {
                'b_2470d0: {
                    'b_246fe0: {
                        'b_246fdc: {
                            c.cycles += 69;
                            c.g[V0][0] = 0x4a0000;
                            c.g[V1][0] = 0x4a0000;
                            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2c10));
                            c.lwl(4, (c.g[A3][0] as u32).wrapping_add(0x3));
                            c.lwr(4, c.g[A3][0] as u32);
                            c.swl(4, (c.g[SP][0] as u32).wrapping_add(0x3));
                            c.swr(4, c.g[SP][0] as u32);
                            c.g[FP][0] = 0xc;
                            c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2c18));
                            c.g[T0][0] = c.lb(c.g[A2][0] as u32);
                            c.g[V0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x11), c.g[V0][0]);
                            c.g[A1][0] = 0x4a0000;
                            c.g[S2][0] = 0x530000;
                            c.g[S6][0] = 0x1;
                            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                            c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8b8));
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x30));
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x31));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x158), c.g[A0][0]);
                            c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c20));
                            c.lwl(8, (c.g[A3][0] as u32).wrapping_add(0x3));
                            c.lwr(8, c.g[A3][0] as u32);
                            c.g[V1][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.swl(8, (c.g[SP][0] as u32).wrapping_add(0x23));
                            c.swr(8, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                            c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                            c.g[S1][0] = (0u64 < c.g[T0][0]) as u64;
                            c.g[RA][0] = sx(0x00246de0);
                            c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                            call_ptr(c, 0x0014dc80, 0x00246de0)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00246dec);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x0014dd58, 0x00246dec)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00246dfc);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x00146e98, 0x00246dfc)?;
                            c.g[RA][0] = sx(0x00246e04);
                            c.g[S4][0] = c.g[V0][0];
                            call_ptr(c, 0x0014dc80, 0x00246e04)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00246e10);
                            c.g[A1][0] = 0x3;
                            call_ptr(c, 0x0014dd58, 0x00246e10)?;
                            c.g[S3][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00246e20);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00146e98, 0x00246e20)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00246e2c);
                            c.g[A0][0] = c.g[S3][0];
                            call_ptr(c, 0x00148950, 0x00246e2c)?;
                            c.g[S5][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00246e3c);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00146e98, 0x00246e3c)?;
                            c.mult(0, 2, 2, 30);
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0x7;
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
                            c.g[V1][0] = c.g[V1][0] & 0x1;
                            c.g[RA][0] = sx(0x00246e64);
                            if c.g[V1][0] != 0 {
                                c.g[S5][0] = c.g[S6][0];
                            }
                            call_ptr(c, 0x0014dd58, 0x00246e64)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                            if t {
                                break 'b_246fdc;
                            } else {
                                'b_246e88: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == c.g[S6][0];
                                    c.g[A1][0] = 0x530000;
                                    if t {
                                        break 'b_246e88;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4b30));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[V1][0] = 0xc;
                                        if t {
                                            break 'b_246fe0;
                                        } else {
                                            break 'b_246e88;
                                        }
                                    }
                                }
                                'b_246ee4: {
                                    c.cycles += 4;
                                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4b30));
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_246ee4;
                                    } else {
                                        c.cycles += 10;
                                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[V1][0]);
                                        c.g[S5][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[A0][0]);
                                        c.g[A1][0] = 0x4a0000;
                                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                                        c.g[A0][0] = 0x28;
                                        c.g[V1][0] = 0x6;
                                        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x48));
                                        'l_246ec0: loop {
                                            c.cycles += 7;
                                            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                            let t = (c.g[V1][0] as i64) >= 0;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00246ec0);
                                                }
                                                continue 'l_246ec0;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                                                if t {
                                                    break 'b_2470d0;
                                                } else {
                                                    break 'b_246ee4;
                                                }
                                            }
                                        }
                                    }
                                }
                                'b_246f44: {
                                    c.cycles += 7;
                                    c.mult(0, 2, 20, 30);
                                    c.g[V1][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
                                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_246f44;
                                    } else {
                                        c.cycles += 8;
                                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[V1][0]);
                                        c.g[A1][0] = 0x4a0000;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[A0][0]);
                                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                                        c.g[A0][0] = 0x5;
                                        c.g[V1][0] = 0x6;
                                        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x48));
                                        'l_246f20: loop {
                                            c.cycles += 7;
                                            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                            let t = (c.g[V1][0] as i64) >= 0;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00246f20);
                                                }
                                                continue 'l_246f20;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                                                if t {
                                                    break 'b_2470d0;
                                                } else {
                                                    break 'b_246f44;
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 11;
                                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4a8));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[V0][0]);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[V1][0]);
                                c.g[A0][0] = 0u64;
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[A1][0] = 0x4a0000;
                                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                                'l_246f70: loop {
                                    c.cycles += 7;
                                    c.g[V0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = ((c.g[A2][0] as i64) < 7i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00246f70);
                                        }
                                        continue 'l_246f70;
                                    } else {
                                        'b_246fa8: {
                                            c.cycles += 7;
                                            c.g[V0][0] = 0x23;
                                            c.g[V1][0] = 0x530000;
                                            c.div(0, 4, 2);
                                            c.g[T0][0] = c.g[A3][0];
                                            c.g[A2][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x4fd8));
                                            if c.g[V0][0] == 0u64 {
                                                break 'b_246fa8;
                                            } else {
                                                break 'b_246fa8;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[A3][0] = 0u64;
                                        c.g[A0][0] = c.lo[0];
                                        c.g[S5][0] = c.g[A0][0];
                                        'l_246fb8: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A3][0] as u32));
                                            c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                            c.g[V0][0] = ((c.g[A3][0] as i64) < 7i64) as u64;
                                            c.sw(c.g[T0][0] as u32, c.g[V1][0]);
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00246fb8);
                                                }
                                                continue 'l_246fb8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                                                if t {
                                                    break 'b_2470d0;
                                                } else {
                                                    break 'b_246fdc;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = 0xc;
                        break 'b_246fe0;
                    }
                    'b_247044: {
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8b8));
                        c.mult(0, 3, 20, 3);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_247044;
                        } else {
                            c.cycles += 8;
                            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[V0][0]);
                            c.g[A1][0] = 0x4a0000;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[V1][0]);
                            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                            c.g[A0][0] = 0x5;
                            c.g[V1][0] = 0x6;
                            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x48));
                            'l_247020: loop {
                                c.cycles += 7;
                                c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                let t = (c.g[V1][0] as i64) >= 0;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00247020);
                                    }
                                    continue 'l_247020;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                                    if t {
                                        break 'b_2470d0;
                                    } else {
                                        break 'b_247044;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 35;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00247050);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x001484e0, 0x00247050)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00247060);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x00148680, 0x00247060)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00247070);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x00148410, 0x00247070)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00247080);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x00148750, 0x00247080)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00247090);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x001488f0, 0x00247090)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002470a0);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x00148820, 0x002470a0)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x002470b0);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                    call_ptr(c, 0x001485b0, 0x002470b0)?;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                    c.g[A1][0] = 0x4a0000;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[A0][0]);
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[A2][0]);
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                    break 'b_2470d0;
                }
                'b_24710c: {
                    'b_2470f4: {
                        c.cycles += 2;
                        let t = c.g[A3][0] != 0u64;
                        c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c28));
                        if t {
                            break 'b_2470f4;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = 0x480000;
                            c.g[RA][0] = sx(0x002470e4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4368));
                            call_ptr(c, 0x00317670, 0x002470e4)?;
                            c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                            c.g[A1][0] = c.g[V0][0];
                            let t = true;
                            c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_24710c;
                            } else {
                                break 'b_2470f4;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = 0x480000;
                    c.g[RA][0] = sx(0x00247100);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4358));
                    call_ptr(c, 0x00317670, 0x00247100)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                    break 'b_24710c;
                }
                'b_247140: {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x00247114);
                    call_ptr(c, 0x0039d860, 0x00247114)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00247128);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00417828, 0x00247128)?;
                    let t = c.g[S0][0] == 0u64;
                    c.g[S6][0] = 0u64;
                    if t {
                        break 'b_247140;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x0024713c);
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x003a0c50, 0x0024713c)?;
                        c.g[S6][0] = 0u64;
                        break 'b_247140;
                    }
                }
                c.cycles += 4;
                c.g[FP][0] = 0x5;
                c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x30));
                'l_247150: loop {
                    'b_247234: {
                        c.cycles += 19;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x0024715c);
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[S0][0]);
                        call_ptr(c, 0x00317670, 0x0024715c)?;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x0024716c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x0024716c)?;
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                        c.g[S4][0] = c.g[V0][0];
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x21), c.g[S0][0]);
                        c.g[RA][0] = sx(0x00247184);
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A2][0]);
                        call_ptr(c, 0x00317670, 0x00247184)?;
                        c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00247194);
                        c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x40));
                        call_ptr(c, 0x0039d860, 0x00247194)?;
                        let t = c.g[S4][0] == 0u64;
                        c.g[S5][0] = c.g[V0][0];
                        if t {
                            break 'b_247234;
                        } else {
                            c.cycles += 2;
                            let t = c.g[S5][0] == 0u64;
                            c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
                            if t {
                                break 'b_247234;
                            } else {
                                'b_2471ac: {
                                    c.cycles += 2;
                                    if c.g[FP][0] == 0u64 {
                                        break 'b_2471ac;
                                    } else {
                                        break 'b_2471ac;
                                    }
                                }
                                c.cycles += 34;
                                c.g[V1][0] =
                                    sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x30));
                                c.g[T0][0] = 0x480000;
                                c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
                                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4348));
                                c.div(0, 18, 30);
                                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[S3][0] = c.lo[0];
                                c.g[RA][0] = sx(0x002471dc);
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                call_ptr(c, 0x00317670, 0x002471dc)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                c.g[A0][0] = c.g[S0][0];
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x002471ec);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, t, 0x002471ec)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S7][0];
                                c.g[RA][0] = sx(0x002471fc);
                                c.g[A2][0] = c.g[S3][0];
                                call_ptr(c, 0x002c26d0, 0x002471fc)?;
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00247214);
                                c.g[A0][0] =
                                    sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00247214)?;
                                c.g[A0][0] = c.g[S4][0];
                                c.g[RA][0] = sx(0x00247220);
                                c.g[A1][0] = c.g[S7][0];
                                call_ptr(c, 0x003a0e90, 0x00247220)?;
                                c.g[V0][0] = 0x37;
                                c.g[A0][0] = c.g[S5][0];
                                c.sw((c.g[S5][0] as u32).wrapping_add(0x80), c.g[V0][0]);
                                c.g[RA][0] = sx(0x00247234);
                                c.g[A1][0] = c.g[S2][0];
                                call_ptr(c, 0x0039c428, 0x00247234)?;
                                break 'b_247234;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S6][0] as i64) < 7i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x30));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00247150);
                        }
                        continue 'l_247150;
                    } else {
                        'b_247290: {
                            c.cycles += 10;
                            c.g[RA][0] = sx(0x0024724c);
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x15c));
                            call_ptr(c, 0x00317670, 0x0024724c)?;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0024725c);
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                            call_ptr(c, 0x0039d860, 0x0024725c)?;
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x48));
                            let t = c.g[A0][0] == 0u64;
                            c.g[V1][0] = c.g[V0][0];
                            if t {
                                break 'b_247290;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] == 0u64;
                                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                                if t {
                                    break 'b_247290;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                    let t = c.g[V0][0] == 0u64;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
                                    if t {
                                        break 'b_247294;
                                    } else {
                                        c.cycles += 2;
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), 0u64);
                                        break 'b_247290;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
                        break 'b_247294;
                    }
                }
            }
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x170));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x210));
        c.pc = t;
        return Ok(());
    });
}

/// `GameModeMan_GetInstance` (0x237cf8), a direct call in the original.
pub(crate) const OC1007_GAME_MODE_MAN: Callee = Callee {
    addr: 0x0023_7cf8,
    f: ee::s06::GameModeMan_GetInstance,
};

/// One statistic line of the multiplayer result panel (the six identical-looking
/// blocks of `cFELoadState_ShowMPPlayerStats`): hash the widget name at
/// `sp+name`, find the text object on the screen (`this->+0x40`); when it exists
/// format `"%d"`-style text with `stats[field]` (the stats record is saved at
/// `sp+0x224`) into the buffer at `sp+0xa0` and set it as the object's ASCII text.
/// The six blocks differ only in the name slot, the record field and the scratch
/// register that holds the record pointer (`stat_reg`: A3, T0, T1, V0, V1, A3), which
/// the callees see. `ret` is the return address of the hash call; the other three
/// calls return to `ret + 0xc`, `+ 0x44` and `+ 0x50`.
pub(crate) fn mp_stat_line(
    c: &mut Cpu,
    name: u32,
    field: u32,
    stat_reg: usize,
    rets: [u32; 4],
) -> Flow {
    c.cycles += 8;
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(name));
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, rets[1])?;
    c.g[S3][0] = c.g[V0][0];
    let missing = c.g[S3][0] == 0u64;
    c.g[V0][0] = 0x4a0000;
    if missing {
        return Ok(());
    }
    c.cycles += 14;
    // The "%d" format (2 bytes + NUL) at 0x4a2c28, copied through the stack.
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2c28));
    c.g[A1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2c28));
    c.g[A2][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x2));
    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
    c.sh((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[A1][0]);
    c.g[A0][0] = c.g[S0][0];
    c.sb((c.g[SP][0] as u32).wrapping_add(0xc2), c.g[A2][0]);
    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
    c.g[stat_reg][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x224));
    c.g[A2][0] = c.lw((c.g[stat_reg][0] as u32).wrapping_add(field));
    call_ptr(c, 0x0041_7828, rets[2])?;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, 0x003a_0c50, rets[3])?;
    Ok(())
}

/// `cFELoadState_ShowMPPlayerStats(this, player)` (0x2472c0): fills the
/// multiplayer result panel of one player on the load screen. Looks up the
/// widgets `<n>_Player_rank_numb`, `<n>_rank`, `<n>_time`, `<n>_knockdowns`,
/// `<n>_wipeouts`, `<n>_tricks`, `<n>_handplants`, `<n>_ubertricks`,
/// `<n>_ubergrinds` (`n` = 1, or 2 for the second player) by hash on the load
/// screen (`this+0x40`) and sets their text: ranking, stat level (or 8 in a
/// ranked network game), race time or score, and the last event's counters
/// from `BEScoreInterface::getPlayerLastCompletedEventStats`; finally the player's
/// picture widget gets its texture from the parent module (`this+0x48`).
/// Stack frame 0x2d0: widget names at `sp+0x00..0x9f`, text buffers at
/// `sp+0xa0` (ascii) and `sp+0x150` (unicode), the stats record pointer at `sp+0x224`.
pub fn cfe_load_state_show_mp_player_stats(c: &mut Cpu) {
    guest(c, |c| {
        'b_2474b8: {
            c.cycles += 122;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2d0));
            c.g[V0][0] = 0x480000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2a0));
            c.g[V1][0] = 0x4a0000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x280));
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x250));
            c.g[S4][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
            c.g[S7][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2b0));
            c.g[A1][0] = 0x7;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x290));
            c.g[S2][0] = 0x530000;
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x270));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x260));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x240));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x230), c.g[RA][0]);
            // Stack names, 0x10 apart: the widget-name templates copied from the data section
            // ("1_Player_rank_numb" at +0x00, "1_rank" +0x20, "1_time" +0x30, "1_knockdowns" +0x40,
            // "1_wipeouts" +0x50, "1_tricks" +0x60, "1_handplants" +0x70, "1_ubertricks" +0x80,
            // "1_ubergrinds" +0x90). Unaligned 8/4-byte copies and single bytes, as the compiler emitted them.
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x41a0));
            c.ldl(6, (c.g[T1][0] as u32).wrapping_add(0x7));
            c.ldr(6, c.g[T1][0] as u32);
            c.ldl(7, (c.g[T1][0] as u32).wrapping_add(0xf));
            c.ldr(7, (c.g[T1][0] as u32).wrapping_add(0x8));
            c.g[T0][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x10));
            c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(6, c.g[SP][0] as u32);
            c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0xf));
            c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x8));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[T0][0]);
            c.g[A2][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x11));
            c.g[A3][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x12));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x11), c.g[A2][0]);
            c.sb((c.g[SP][0] as u32).wrapping_add(0x12), c.g[A3][0]);
            c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2c40));
            c.lwl(2, (c.g[T1][0] as u32).wrapping_add(0x3));
            c.lwr(2, c.g[T1][0] as u32);
            c.g[A3][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x4));
            c.g[T0][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x5));
            c.swl(2, (c.g[SP][0] as u32).wrapping_add(0x23));
            c.swr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x24), c.g[A3][0]);
            c.sb((c.g[SP][0] as u32).wrapping_add(0x25), c.g[T0][0]);
            c.g[V0][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x6));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x26), c.g[V0][0]);
            c.g[A2][0] = 0x4a0000;
            c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2c48));
            c.lwl(3, (c.g[T1][0] as u32).wrapping_add(0x3));
            c.lwr(3, c.g[T1][0] as u32);
            c.g[A3][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x4));
            c.g[T0][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x5));
            c.swl(3, (c.g[SP][0] as u32).wrapping_add(0x33));
            c.swr(3, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x34), c.g[A3][0]);
            c.sb((c.g[SP][0] as u32).wrapping_add(0x35), c.g[T0][0]);
            c.g[V1][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x6));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x36), c.g[V1][0]);
            c.g[V0][0] = 0x480000;
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4188));
            c.ldl(6, (c.g[T1][0] as u32).wrapping_add(0x7));
            c.ldr(6, c.g[T1][0] as u32);
            c.lwl(7, (c.g[T1][0] as u32).wrapping_add(0xb));
            c.lwr(7, (c.g[T1][0] as u32).wrapping_add(0x8));
            c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x47));
            c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.swl(7, (c.g[SP][0] as u32).wrapping_add(0x4b));
            c.swr(7, (c.g[SP][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = 0x480000;
            c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4178));
            c.ldl(2, (c.g[T1][0] as u32).wrapping_add(0x7));
            c.ldr(2, c.g[T1][0] as u32);
            c.g[A3][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x8));
            c.g[T0][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x9));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x57));
            c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
            c.sb((c.g[SP][0] as u32).wrapping_add(0x59), c.g[T0][0]);
            c.g[A2][0] = 0x4a0000;
            c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2c50));
            c.ldl(3, (c.g[T1][0] as u32).wrapping_add(0x7));
            c.ldr(3, c.g[T1][0] as u32);
            c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x67));
            c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = 0x480000;
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4168));
            c.ldl(6, (c.g[T1][0] as u32).wrapping_add(0x7));
            c.ldr(6, c.g[T1][0] as u32);
            c.lwl(7, (c.g[T1][0] as u32).wrapping_add(0xb));
            c.lwr(7, (c.g[T1][0] as u32).wrapping_add(0x8));
            c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x77));
            c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.swl(7, (c.g[SP][0] as u32).wrapping_add(0x7b));
            c.swr(7, (c.g[SP][0] as u32).wrapping_add(0x78));
            c.g[V1][0] = 0x480000;
            c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4158));
            c.ldl(2, (c.g[T1][0] as u32).wrapping_add(0x7));
            c.ldr(2, c.g[T1][0] as u32);
            c.lwl(7, (c.g[T1][0] as u32).wrapping_add(0xb));
            c.lwr(7, (c.g[T1][0] as u32).wrapping_add(0x8));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x87));
            c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.swl(7, (c.g[SP][0] as u32).wrapping_add(0x8b));
            c.swr(7, (c.g[SP][0] as u32).wrapping_add(0x88));
            c.g[A2][0] = 0x480000;
            c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4148));
            c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
            c.ldr(3, c.g[T0][0] as u32);
            c.lwl(4, (c.g[T0][0] as u32).wrapping_add(0xb));
            c.lwr(4, (c.g[T0][0] as u32).wrapping_add(0x8));
            c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x97));
            c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.swl(4, (c.g[SP][0] as u32).wrapping_add(0x9b));
            c.swr(4, (c.g[SP][0] as u32).wrapping_add(0x98));
            // cBE_getInterface(world->be (+0x78), 7); then, unless this is the network host (or not
            // a network game), flip the player index (side: `player = player < 1`).
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x220), c.g[S7][0]);
            call_ptr(c, 0x0014dd58, 0x00247490)?;
            c.g[V1][0] = c.g[S2][0];
            c.g[T1][0] = c.g[V1][0];
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4b30));
            c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8));
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x220));
                break 'b_2474b8;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
                c.g[V0][0] = (c.g[S7][0] < 0x1u64) as u64;
                if c.g[V1][0] != 0 {
                    c.g[S7][0] = c.g[V0][0];
                }
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x220));
                break 'b_2474b8;
            }
        }
        'b_2474e8: {
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0x32;
            if t {
                break 'b_2474e8;
            } else {
                c.cycles += 9;
                c.sb((c.g[SP][0] as u32).wrapping_add(0x90), c.g[V0][0]);
                c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x60), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x80), c.g[V0][0]);
                break 'b_2474e8;
            }
        }
        'b_2475cc: {
            c.cycles += 39;
            c.g[S1][0] = (0u64 < c.g[S7][0]) as u64;
            call_ptr(c, 0x0014dc80, 0x002474f0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x002474fc)?;
            c.g[S6][0] = c.g[S1][0];
            c.g[S5][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x00247508)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00247514)?;
            call_ptr(c, 0x0014dc80, 0x0024751c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0014dd58, 0x00247528)?;
            c.g[FP][0] = c.g[V0][0];
            OC1007_GAME_MODE_MAN.call(c, 0x00247530)?;
            call_ptr(c, 0x0014dc80, 0x00247538)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x8;
            call_ptr(c, 0x0014dd58, 0x00247544)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x00146e98, 0x00247554)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00155270, 0x00247560)?;
            c.g[A0][0] = c.g[SP][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x224), c.g[V0][0]);
            call_ptr(c, 0x00317670, 0x0024756c)?;
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00247578)?;
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] == 0u64;
            c.g[V0][0] = 0x530000;
            if t {
                break 'b_2475cc;
            } else {
                c.cycles += 18;
                c.g[A1][0] = 0x4a0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                c.g[V1][0] = sx((c.g[S7][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c28));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x2));
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[A2][0] = c.lw(c.g[V1][0] as u32);
                c.g[A0][0] = c.g[S0][0];
                c.g[A3][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2c28));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sb((c.g[SP][0] as u32).wrapping_add(0xc2), c.g[V0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[A3][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x00417828, 0x002475c0)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0c50, 0x002475cc)?;
                break 'b_2475cc;
            }
        }
        'b_2476a4: {
            c.cycles += 9;
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            call_ptr(c, 0x00317670, 0x002475d4)?;
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x002475e0)?;
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] == 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x228), c.g[A0][0]);
            if t {
                break 'b_2476a4;
            } else {
                'b_247640: {
                    'b_247628: {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = 0x7;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                        call_ptr(c, 0x0014dd58, 0x00247600)?;
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4b30));
                        if c.g[V1][0] == 0u64 {
                            c.g[A0][0] = c.g[S5][0];
                            break 'b_247628;
                        } else {
                            c.cycles += 5;
                            c.g[A1][0] = 0x530000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4b30));
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                            if c.g[V0][0] == 0u64 {
                                c.g[A0][0] = c.g[S5][0];
                                break 'b_247628;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[FP][0] = 0x8;
                                if t {
                                    break 'b_247640;
                                } else {
                                    break 'b_247628;
                                }
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S6][0];
                    call_ptr(c, 0x00146e98, 0x00247630)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00148950, 0x0024763c)?;
                    c.g[FP][0] = c.g[V0][0];
                    break 'b_247640;
                }
                c.cycles += 25;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[A0][0] = 0x480000;
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4380));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x228), c.g[S2][0]);
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x00317670, 0x00247668)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x00247678)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x002c2508, 0x00247684)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[FP][0];
                call_ptr(c, 0x002c26d0, 0x00247698)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0e90, 0x002476a4)?;
                break 'b_2476a4;
            }
        }
        'b_24780c: {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            call_ptr(c, 0x00317670, 0x002476ac)?;
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x002476b8)?;
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] == 0u64;
            c.g[V0][0] = 0x530000;
            if t {
                break 'b_24780c;
            } else {
                'b_247804: {
                    'b_247790: {
                        'b_2476d8: {
                            c.cycles += 3;
                            c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c10));
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = 0x5;
                            if t {
                                break 'b_2476d8;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A1][0] = c.g[S6][0];
                                if t {
                                    break 'b_247790;
                                } else {
                                    break 'b_2476d8;
                                }
                            }
                        }
                        'b_247724: {
                            'b_247710: {
                                c.cycles += 10;
                                c.g[A0][0] = c.g[S5][0];
                                c.g[A1][0] = c.g[S6][0];
                                call_ptr(c, 0x00146e98, 0x002476e4)?;
                                c.g[V1][0] = 0x530000;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                                let t = (c.g[V0][0] as i64) < 0;
                                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                                if t {
                                    break 'b_247710;
                                } else {
                                    c.cycles += 4;
                                    c.mtc1(2, 0);
                                    c.cvt_s_w(0, 0);
                                    let t = true;

                                    if t {
                                        break 'b_247724;
                                    } else {
                                        break 'b_247710;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                            c.mtc1(2, 0);
                            c.cvt_s_w(0, 0);
                            c.add_s(0, 0, 0);
                            break 'b_247724;
                        }
                        'b_247774: {
                            c.cycles += 13;
                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5008));
                            c.mul_s(12, 0, 12);
                            call_ptr(c, 0x004139f8, 0x00247730)?;
                            c.g[A0][0] = 0x4a0000;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c58));
                            call_ptr(c, 0x00317670, 0x00247740)?;
                            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x0024774c)?;
                            c.g[A2][0] = c.g[V0][0];
                            let t = c.g[A2][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_247774;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00247770)?;
                                c.g[A0][0] = c.g[S0][0];
                                break 'b_247774;
                            }
                        }
                        c.cycles += 7;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                        call_ptr(c, 0x0020a1a8, 0x0024777c)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x004166f4, 0x00247788)?;
                        let t = true;
                        c.g[A1][0] = c.g[S0][0];
                        if t {
                            break 'b_247804;
                        } else {
                            break 'b_247790;
                        }
                    }
                    'b_2477ec: {
                        c.cycles += 16;
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x00146e98, 0x00247798)?;
                        c.g[V1][0] = 0x530000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                        c.g[A0][0] = 0x4a0000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c60));
                        c.g[S1][0] = c.lw(c.g[V0][0] as u32);
                        call_ptr(c, 0x00317670, 0x002477b8)?;
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x002477c4)?;
                        c.g[A2][0] = c.g[V0][0];
                        let t = c.g[A2][0] == 0u64;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                        if t {
                            break 'b_2477ec;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x002477e8)?;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                            break 'b_2477ec;
                        }
                    }
                    c.cycles += 6;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c28));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x00247800)?;
                    c.g[A1][0] = c.g[S0][0];
                    break 'b_247804;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x003a0c50, 0x0024780c)?;
                break 'b_24780c;
            }
        }
        mp_stat_line(
            c,
            0x40,
            0x2c,
            A3,
            [0x0024_7814, 0x0024_7820, 0x0024_7858, 0x0024_7864],
        )?;
        mp_stat_line(
            c,
            0x50,
            0x28,
            T0,
            [0x0024_786c, 0x0024_7878, 0x0024_78b0, 0x0024_78bc],
        )?;
        mp_stat_line(
            c,
            0x60,
            0x14,
            T1,
            [0x0024_78c4, 0x0024_78d0, 0x0024_7908, 0x0024_7914],
        )?;
        mp_stat_line(
            c,
            0x70,
            0xc,
            V0,
            [0x0024_791c, 0x0024_7928, 0x0024_7960, 0x0024_796c],
        )?;
        mp_stat_line(
            c,
            0x80,
            0x18,
            V1,
            [0x0024_7974, 0x0024_7980, 0x0024_79b8, 0x0024_79c4],
        )?;
        mp_stat_line(
            c,
            0x90,
            0x20,
            A3,
            [0x0024_79cc, 0x0024_79d8, 0x0024_7a10, 0x0024_7a1c],
        )?;
        'b_247a84: {
            'b_247a80: {
                c.cycles += 17;
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x220));
                c.g[V0][0] = 0x4a0000;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2c18));
                c.g[T1][0] = c.lb(c.g[A2][0] as u32);
                c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
                c.sb((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[T1][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0xe1), c.g[A0][0]);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x228));
                c.sb((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[V1][0]);
                call_ptr(c, 0x00317670, 0x00247a48)?;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00247a54)?;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x48));
                let t = c.g[A0][0] == 0u64;
                c.g[V1][0] = c.g[V0][0];
                if t {
                    break 'b_247a80;
                } else {
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                    if t {
                        break 'b_247a80;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                        let t = c.g[V0][0] == 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                        if t {
                            break 'b_247a84;
                        } else {
                            c.cycles += 2;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), 0u64);
                            break 'b_247a80;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
            break 'b_247a84;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2b0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2a0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x290));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x280));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x270));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x260));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x250));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x240));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x230));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2d0));
        c.pc = t;
        Ok(())
    })
}

/// `void cGameLoadState_SetupMPWidget(void* self, UIObject* obj)` (0x00246278):
/// multiplayer load-screen widget setup. If the object's name hash (`obj+0x38`)
/// is the "track peak" label, formats the track name into a 200-byte buffer
/// (format picked by `BENewRaceInt_GetTrackPeak`) and sets it as the widget's
/// wide string. Otherwise, depending on the game mode byte at `0x535c10`, if
/// the object is that mode's label, calls its activate method (`obj+8`
/// vtable `+0x4c`). Any other `pc` (a resume after an unwound callee) runs
/// the generated body.
pub fn c_game_load_state_setup_mp_widget(c: &mut Cpu) {
    if c.pc != 0x00246278 {
        return ee::s06::f_00246278__generated(c);
    }
    guest(c, |c| {
        'b_2464c8: {
            'b_2463e0: {
                c.cycles += 19;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
                c.g[RA][0] = sx(0x0024629c);
                c.g[S2][0] = c.g[A1][0];
                call_ptr(c, 0x0014dc80, 0x0024629c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002462a8);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x002462a8)?;
                c.g[A0][0] = 0x480000;
                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                c.g[S3][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002462bc);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x45b8));
                call_ptr(c, 0x00317670, 0x002462bc)?;
                let t = c.g[S0][0] != c.g[V0][0];
                c.g[V0][0] = 0x530000;
                if t {
                    break 'b_2463e0;
                } else {
                    'b_2463d0: {
                        'b_246380: {
                            'b_246334: {
                                'b_246328: {
                                    c.cycles += 12;
                                    c.g[A0][0] = c.g[SP][0];
                                    c.g[A1][0] = 0u64;
                                    c.g[RA][0] = sx(0x002462d4);
                                    c.g[A2][0] = 0xc8;
                                    call_ptr(c, 0x00416210, 0x002462d4)?;
                                    c.g[V1][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[RA][0] = sx(0x002462e4);
                                    c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2bd8));
                                    call_ptr(c, 0x00144c98, 0x002462e4)?;
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                    if t {
                                        break 'b_246328;
                                    } else {
                                        'b_24631c: {
                                            'b_24630c: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_24630c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    if t {
                                                        break 'b_24631c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        if t {
                                                            break 'b_2463d0;
                                                        } else {
                                                            break 'b_24630c;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            if t {
                                                break 'b_246380;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A0][0] = c.g[S2][0];
                                                if t {
                                                    break 'b_2463d0;
                                                } else {
                                                    break 'b_24631c;
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x480000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x45a8));
                                        if t {
                                            break 'b_246334;
                                        } else {
                                            break 'b_246328;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = 0x480000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4598));
                                break 'b_246334;
                            }
                            c.cycles += 19;
                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[RA][0] = sx(0x00246348);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            call_ptr(c, 0x00317670, 0x00246348)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[A1][0] = c.g[V0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00246358);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, t, 0x00246358)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00246364);
                            c.g[A0][0] = c.g[S3][0];
                            call_ptr(c, 0x00144bf0, 0x00246364)?;
                            c.g[A1][0] = c.g[S4][0];
                            c.g[A2][0] = c.g[S0][0];
                            c.g[A3][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00246378);
                            c.g[A0][0] = c.g[SP][0];
                            call_ptr(c, 0x002c27c0, 0x00246378)?;
                            let t = true;
                            c.g[A0][0] = c.g[S2][0];
                            if t {
                                break 'b_2463d0;
                            } else {
                                break 'b_246380;
                            }
                        }
                        c.cycles += 20;
                        c.g[A0][0] = 0x480000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4588));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x0024639c);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x00317670, 0x0024639c)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A1][0] = c.g[V0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x002463ac);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, t, 0x002463ac)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x002463b8);
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x00144bf0, 0x002463b8)?;
                        c.g[A1][0] = c.g[S4][0];
                        c.g[A2][0] = c.g[S0][0];
                        c.g[A3][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x002463cc);
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x002c27c0, 0x002463cc)?;
                        c.g[A0][0] = c.g[S2][0];
                        break 'b_2463d0;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002463d8);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x003a0e90, 0x002463d8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                    if t {
                        break 'b_2464c8;
                    } else {
                        break 'b_2463e0;
                    }
                }
            }
            'b_2464c4: {
                c.cycles += 4;
                c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c10));
                c.g[V1][0] = (c.g[A0][0] < 0x7u64) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x480000;
                if t {
                    break 'b_2464c4;
                } else {
                    'b_246494: {
                        'b_246464: {
                            'b_246458: {
                                'b_246448: {
                                    'b_246438: {
                                        'b_246428: {
                                            'b_246418: {
                                                c.cycles += 6;
                                                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x24b0));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x00246408 => {
                                                        c.cycles += 4;
                                                        c.g[A0][0] = 0x480000;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                        let t = true;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x4578));
                                                        if t {
                                                            break 'b_246464;
                                                        } else {
                                                            break 'b_246418;
                                                        }
                                                    }
                                                    0x00246418 => {
                                                        break 'b_246418;
                                                    }
                                                    0x00246428 => {
                                                        break 'b_246428;
                                                    }
                                                    0x00246438 => {
                                                        break 'b_246438;
                                                    }
                                                    0x00246448 => {
                                                        break 'b_246448;
                                                    }
                                                    0x00246458 => {
                                                        break 'b_246458;
                                                    }
                                                    0x00246494 => {
                                                        break 'b_246494;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x00246278, 0x002464e4);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[A0][0] = 0x480000;
                                            c.g[S0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x4560));
                                            if t {
                                                break 'b_246464;
                                            } else {
                                                break 'b_246428;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[A0][0] = 0x480000;
                                        c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4550));
                                        if t {
                                            break 'b_246464;
                                        } else {
                                            break 'b_246438;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = 0x480000;
                                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4540));
                                    if t {
                                        break 'b_246464;
                                    } else {
                                        break 'b_246448;
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = 0x480000;
                                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4578));
                                if t {
                                    break 'b_246464;
                                } else {
                                    break 'b_246458;
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = 0x480000;
                            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4530));
                            break 'b_246464;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0024646c);
                        call_ptr(c, 0x00317670, 0x0024646c)?;
                        let t = c.g[S0][0] != c.g[V0][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                        if t {
                            break 'b_2464c8;
                        } else {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x0024648c);
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0024648c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                            if t {
                                break 'b_2464c8;
                            } else {
                                break 'b_246494;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = 0x480000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x002464a4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4530));
                    call_ptr(c, 0x00317670, 0x002464a4)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                    if t {
                        break 'b_2464c8;
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x002464c4);
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002464c4)?;
                        break 'b_2464c4;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
            break 'b_2464c8;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.pc = t;
        return Ok(());
    });
}

/// The address `gp - neg` (the game's gp-relative globals).
#[inline]
pub(crate) fn oc1009_gp_minus(c: &Cpu, neg: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(neg)
}

/// `reg + off` as a 32-bit address.
#[inline]
pub(crate) fn oc1009_ea(reg: u64, off: u32) -> u32 {
    (reg as u32).wrapping_add(off)
}

/// `a0 = base - sub` as the `lui`/`addiu` pair leaves it (sign-extended).
#[inline]
pub(crate) fn oc1009_base_minus(base: u32, sub: u32) -> u64 {
    sx(base.wrapping_sub(sub))
}

// ---------------------------------------------------------------------------
// cFELoadState_SetupResultWidget (0x247ab0)
// ---------------------------------------------------------------------------

/// Offset of a widget's name hash.
pub(crate) const FE_LOAD_STATE_WIDGET_HASH: u32 = 0x38;

/// One link of the widget-name chain: charges `extra` cycles, loads the
/// widget's hash (`s4+0x38` into `s0`), computes `GetHashValue32` of the name
/// string at `0x480000 - sub` (0x317670, returning to `ret`). Returns whether
/// the widget is NOT this one (`s0 != v0`). With `set_a0` the delay slot's
/// `a0 = 0x480000` follows.
pub(crate) fn oc1009_srw_hash_miss(
    c: &mut Cpu,
    extra: u64,
    sub: u32,
    ret: u32,
    set_a0: bool,
) -> Result<bool, crate::call::Unwound> {
    c.cycles += extra;
    c.g[S0][0] = c.lw(oc1009_ea(c.g[S4][0], FE_LOAD_STATE_WIDGET_HASH));
    c.g[A0][0] = oc1009_base_minus(c.g[A0][0] as u32, sub);
    call_ptr(c, 0x0031_7670, ret)?;
    let miss = c.g[S0][0] != c.g[V0][0];
    if set_a0 {
        c.g[A0][0] = 0x0048_0000;
    }
    Ok(miss)
}

/// Epilogue of `SetupResultWidget` (`s0` is restored by the caller).
pub(crate) fn oc1009_srw_finish(c: &mut Cpu, frame: Frame) {
    c.cycles += 8;
    frame.restore(c, S1, 0x100);
    frame.restore(c, S2, 0xf0);
    frame.restore(c, S3, 0xe0);
    frame.restore(c, S4, 0xd0);
    frame.restore(c, S5, 0xc0);
    frame.restore_ra(c, 0xb0);
    frame.close(c);
}

/// `UIText::SetText(widget, buf)` (0x3a0e90) with the text built in the stack
/// buffer, then the epilogue.
pub(crate) fn oc1009_srw_set_text(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, 0x003a_0e90, 0x0024_7d08)?;
    frame.restore(c, S0, 0x110);
    oc1009_srw_finish(c, frame);
    Ok(())
}

/// Appends the number in `a2` to the format in `a1` (0x2c26d0, wide-string
/// printf) into the stack buffer, then sets the widget text.
pub(crate) fn oc1009_srw_format_and_set(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    call_ptr(c, 0x002c_26d0, 0x0024_7cfc)?;
    c.g[A0][0] = c.g[S4][0];
    oc1009_srw_set_text(c, frame)
}

/// `void cFELoadState_SetupResultWidget(cFELoadState* this, cUIText* widget)`
/// (0x247ab0): fills the text of one widget of the result screen, picked by the
/// widget's name hash (`widget+0x38`): the rider's name (0x4138), a rider
/// string formatted with a statistic (0x4128), one of five numeric statistics
/// of the saved result record (`%d` format at `0x4a2c28`, fields `+0x2c`, `+0x28`,
/// `+0x18`, `+0x20`, `+0xc`), the race time as `mm:ss` (0x40a0, seconds `+0x48`
/// as float) and the score (0x4078, field `+0x88`).
pub fn fe_load_state_setup_result_widget(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 42;
        let frame = Frame::open(c, 0x120);
        frame.save(c, S4, 0xd0);
        frame.save(c, S0, 0x110);
        frame.save(c, S1, 0x100);
        frame.save(c, S2, 0xf0);
        frame.save(c, S3, 0xe0);
        frame.save(c, S5, 0xc0);
        frame.save_ra(c, 0xb0);
        c.g[S4][0] = c.g[A1][0];
        // interfaces 1, 2, 3 and 8 of the BE
        call_ptr(c, 0x0014_dc80, 0x0024_7ad8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0024_7ae4)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0024_7aec)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0014_dd58, 0x0024_7af8)?;
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0024_7b00)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, 0x0014_dd58, 0x0024_7b0c)?;
        c.g[S5][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0024_7b14)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 8;
        call_ptr(c, 0x0014_dd58, 0x0024_7b20)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_6e98, 0x0024_7b30)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0015_5270, 0x0024_7b3c)?;
        // s1 = the result record of the local player
        c.g[A0][0] = 0x0048_0000;
        c.g[S1][0] = c.g[V0][0];

        // arm 1 (0x4138): the first string of the rider
        if !oc1009_srw_hash_miss(c, 0, 0x4138, 0x0024_7b50, true)? {
            c.cycles += 11;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_6e98, 0x0024_7b64)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_ee58, 0x0024_7b70)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x002c_2540, 0x0024_7b7c)?;
            c.g[A0][0] = c.g[S4][0];
            return oc1009_srw_set_text(c, frame);
        }

        // arm 2 (0x4128): a game-object string formatted with the rider's value
        if !oc1009_srw_hash_miss(c, 5, 0x4128, 0x0024_7b90, true)? {
            c.cycles += 26;
            c.g[V0][0] = c.lw(oc1009_gp_minus(c, 0x848));
            c.g[A0][0] = 0x0048_0000;
            c.g[S2][0] = sx(oc1009_ea(c.g[SP][0] as u64, 0x30));
            c.g[A0][0] = oc1009_base_minus(c.g[A0][0] as u32, 0x4380);
            c.g[S0][0] = c.lw(oc1009_ea(c.g[V0][0], 0x8c));
            c.g[S1][0] = c.lw(oc1009_ea(c.g[S0][0], 0x4));
            c.g[V0][0] = c.lh(oc1009_ea(c.g[S1][0], 0x20));
            c.g[S0][0] = sx(oc1009_ea(c.g[S0][0], c.g[V0][0] as u32));
            call_ptr(c, 0x0031_7670, 0x0024_7bbc)?;
            c.g[V1][0] = c.lw(oc1009_ea(c.g[S1][0], 0x24));
            c.g[A0][0] = c.g[S0][0];
            let method = c.g[V1][0] as u32;
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, method, 0x0024_7bcc)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x002c_2508, 0x0024_7bd8)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_6e98, 0x0024_7be4)?;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_8950, 0x0024_7bf0)?;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[V0][0];
            return oc1009_srw_format_and_set(c, frame);
        }

        // arms 3..7: `%d` (wide format at 0x4a2c28) of one field of the record:
        // (name offset, hash return, format return, extra cycles, field)
        const NUMERIC: [(u32, u32, u32, u64, u32); 5] = [
            (0x4118, 0x0024_7c0c, 0x0024_7c28, 7, 0x2c),
            (0x4100, 0x0024_7c3c, 0x0024_7c58, 7, 0x28),
            (0x40e8, 0x0024_7c6c, 0x0024_7c88, 7, 0x18),
            (0x40d0, 0x0024_7c9c, 0x0024_7cb8, 7, 0x20),
            (0x40b8, 0x0024_7ccc, 0x0024_7ce8, 6, 0x0c),
        ];
        for (sub, hash_ret, fmt_ret, extra, field) in NUMERIC {
            if !oc1009_srw_hash_miss(c, 5, sub, hash_ret, true)? {
                c.cycles += extra;
                c.g[S0][0] = sx(oc1009_ea(c.g[SP][0] as u64, 0x30));
                c.g[A1][0] = 0x004a_0000;
                c.g[A1][0] = sx(oc1009_ea(c.g[A1][0], 0x2c28));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_2540, fmt_ret)?;
                c.g[A2][0] = c.lw(oc1009_ea(c.g[S1][0], field));
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[SP][0];
                return oc1009_srw_format_and_set(c, frame);
            }
        }

        // arm 8 (0x40a0): the race time as "%02.0f:%02.0f" (minutes, seconds)
        if !oc1009_srw_hash_miss(c, 5, 0x40a0, 0x0024_7d1c, true)? {
            c.cycles += 4;
            c.lwc1(0, oc1009_ea(c.g[S1][0], 0x48));
            c.g[V1][0] = 0x3c;
            c.cycles += 33;
            c.g[A1][0] = 0x0048_0000;
            c.cvt_w_s(1, 0);
            c.mfc1(4, 1);
            c.g[A2][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4088);
            c.g[V0][0] = 0x3c;
            c.g[S3][0] = sx(oc1009_ea(c.g[SP][0] as u64, 0x90));
            c.g[S2][0] = sx(oc1009_ea(c.g[SP][0] as u64, 0x70));
            c.div(0, 4, 3);
            c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_sub(0x4088));
            c.g[A1][0] = c.lhu(oc1009_ea(c.g[A2][0], 0xc));
            c.g[V0][0] = c.lw(oc1009_ea(c.g[A2][0], 0x8));
            c.sd(oc1009_ea(c.g[SP][0] as u64, 0x70), c.g[A3][0]);
            c.sh(oc1009_ea(c.g[SP][0] as u64, 0x7c), c.g[A1][0]);
            c.g[A0][0] = c.lo[0];
            c.g[S1][0] = c.hi[0];
            c.mtc1(4, 12);
            c.cvt_s_w(12, 12);
            c.sw(oc1009_ea(c.g[SP][0] as u64, 0x78), c.g[V0][0]);
            call_ptr(c, 0x0041_3af8, 0x0024_7d80)?;
            c.mtc1(17, 12);
            c.cvt_s_w(12, 12);
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0041_3af8, 0x0024_7d90)?;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0041_7828, 0x0024_7da4)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x003a_0c50, 0x0024_7db0)?;
            frame.restore(c, S0, 0x110);
            oc1009_srw_finish(c, frame);
            return Ok(());
        }

        // arm 9 (0x4078): the score
        let miss = oc1009_srw_hash_miss(c, 5, 0x4078, 0x0024_7dc4, false)?;
        frame.restore(c, S0, 0x110);
        if !miss {
            c.cycles += 13;
            c.g[S0][0] = sx(oc1009_ea(c.g[SP][0] as u64, 0x30));
            c.g[A1][0] = 0x004a_0000;
            c.g[A1][0] = sx(oc1009_ea(c.g[A1][0], 0x2c28));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c_2540, 0x0024_7de0)?;
            c.g[A2][0] = c.lw(oc1009_ea(c.g[S1][0], 0x88));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x002c_26d0, 0x0024_7df0)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0e90, 0x0024_7dfc)?;
            frame.restore(c, S0, 0x110);
        }
        oc1009_srw_finish(c, frame);
        Ok(())
    });
}

/// `void cFELoadHintState_Init(cFELoadHintState* self)` (0x245950): add the
/// hint screen (`self+0x40`) to the engine (`self+0x10`), play it, and set
/// the title and body texts from the rotating hint counter (`gp-0x4b8`,
/// wrapping at 12). When the screen has a hint object (`self+0x48` with
/// its word at `+0x10`), copy that word into the object's `+0x78`.
pub fn c_fe_load_hint_state_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, 17, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 18, 0x90);
        frame.save_ra(c, 0x80);
        frame.save(c, 16, 0xb0);
        c.g[A0][0] = sx(0x0048_0000u32.wrapping_sub(0x4228));
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_7670, 0x0024_5978)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0024_598c)?;
        let no_screen = c.g[V0][0] == 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0024_59a4)?;
        }
        // The hint counter (a signed byte at gp-0x4b8) and its wrap at 12.
        c.cycles += 5;
        let counter = (c.g[GP][0] as u32).wrapping_sub(0x4b8);
        c.g[V0][0] = c.lb(counter);
        c.g[V1][0] = 0xc;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        let t = c.g[S0][0] != c.g[V1][0];
        c.g[A0][0] = c.lbu(counter);
        if !t {
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
            c.sb(counter, c.g[V1][0]);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        }
        // The two strings: sprintf(title, "%d", n) and sprintf(body, ..., n).
        c.cycles += 15;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x4218));
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x0041_7828, 0x0024_59e0)?;
        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x4200));
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0041_7828, 0x0024_59f8)?;
        c.g[V1][0] = c.lb(counter);
        c.g[A1][0] = 0xf;
        // The counter advances modulo 15.
        c.cycles += 13;
        c.g[A0][0] = 0x0048_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = sx(0x0048_0000u32.wrapping_sub(0x41f0));
        c.div(0, V1, A1);
        c.g[V0][0] = c.hi[0];
        c.sb(counter, c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x0024_5a24)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0024_5a30)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.g[A0][0] = 0x0048_0000;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x0024_5a44)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x0024_5a50)?;
            c.g[A0][0] = 0x0048_0000;
        }
        // The body text.
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x41e0));
        call_ptr(c, 0x0031_7670, 0x0024_5a5c)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0024_5a68)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.g[A0][0] = 0x0048_0000;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0031_7670, 0x0024_5a7c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x0024_5a88)?;
            c.g[A0][0] = 0x0048_0000;
        }
        // The hint object of the screen (`self+0x48`): the three exits of the
        // generated labelled blocks, each falling through the same restores.
        'b_245ad4: {
            'b_245ad0: {
                'b_245acc: {
                    c.cycles += 7;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x41d0));
                    call_ptr(c, 0x0031_7670, 0x0024_5a94)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x0024_5aa0)?;
                    let no_text = c.g[V0][0] == 0;
                    frame.restore(c, 16, 0xb0);
                    if no_text {
                        break 'b_245acc;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                        let no_hint = c.g[A0][0] == 0;
                        frame.restore(c, 17, 0xa0);
                        if no_hint {
                            break 'b_245ad0;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                            let no_word = c.g[A0][0] == 0;
                            frame.restore(c, 18, 0x90);
                            if no_word {
                                break 'b_245ad4;
                            } else {
                                c.cycles += 3;
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), 0);
                                frame.restore(c, 16, 0xb0);
                                break 'b_245acc;
                            }
                        }
                    }
                }
                c.cycles += 1;
                frame.restore(c, 17, 0xa0);
                break 'b_245ad0;
            }
            c.cycles += 1;
            frame.restore(c, 18, 0x90);
            break 'b_245ad4;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateProgressiveVerifyPS2_dtor(self)` (0x245f30): forwards to the
/// base destructor at 0x39e390.
pub fn c_fe_state_progressive_verify_ps2_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0039_e390), 0x0024_5f40)
}

/// `cFEStateProgressiveVerifyPS2_Update(self)` (0x246098): the per-frame
/// update of the progressive-verify popup. Counts `self+0x4c` down; when it
/// reaches zero, once (the flag at `self+0x48`), sets the "display mode
/// changed" title and body text, the continue label, and disables the
/// "no" button and the graphics manager. Then `cUIState_onUpdate(self)`.
pub fn c_fe_state_progressive_verify_ps2_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4c));
        let count_zero = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        if !count_zero {
            c.cycles += 2;
            let t = c.g[V0][0] != 0;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
            if !t {
                c.cycles += 3;
                c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x48));
                let flag_set = c.g[V0][0] != 0;
                c.g[V0][0] = 0x1;
                if !flag_set {
                    progressive_verify_first_time(c)?;
                }
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e510, 0x0024_61cc)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The branch of the update that runs once, when the countdown ends and the
/// flag is clear: the title and body texts, the continue label, the "no"
/// button and the graphics manager's call.
pub(crate) fn progressive_verify_first_time(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 10;
    c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x2ac0));
    let flag = (c.g[S1][0] as u32).wrapping_add(0x48);
    c.sb(flag, c.g[V0][0]);
    call_ptr(c, 0x0031_7670, 0x0024_60dc)?;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x0024_60e8)?;
    c.g[A0][0] = c.g[V0][0];
    if c.g[A0][0] == 0 {
        c.g[A0][0] = 0x004a_0000;
    } else {
        // The title text: its mode word decides which string the call uses.
        c.cycles += 3;
        let mode = (c.g[S1][0] as u32).wrapping_add(0x48);
        c.g[V0][0] = c.lb(mode);
        let mode_set = c.g[V0][0] != 0;
        c.g[A1][0] = 0x0048_0000;
        if !mode_set {
            c.cycles += 5;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4708));
            call_ptr(c, 0x003a_0c50, 0x0024_610c)?;
            c.g[A0][0] = 0x004a_0000;
        } else {
            c.cycles += 3;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x46d0));
            call_ptr(c, 0x003a_0c50, 0x0024_611c)?;
            c.g[A0][0] = 0x004a_0000;
        }
    }
    // The continue label.
    c.cycles += 8;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ac8));
    call_ptr(c, 0x0031_7670, 0x0024_6128)?;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x0024_6134)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] == 0 {
        c.g[A0][0] = 0x004a_0000;
    } else {
        c.cycles += 7;
        c.g[A0][0] = 0x0048_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4698));
        call_ptr(c, 0x0031_7670, 0x0024_614c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x0024_6158)?;
        c.g[A0][0] = 0x004a_0000;
    }
    // The "no" button: hide it through its vtable (slots 9 and 8).
    c.cycles += 8;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ad0));
    call_ptr(c, 0x0031_7670, 0x0024_6164)?;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x0024_6170)?;
    c.g[S0][0] = c.g[V0][0];
    let no_button = c.g[S0][0] == 0;
    c.g[A1][0] = 0;
    if !no_button {
        c.cycles += 11;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let f = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, f, 0x0024_6190)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        let f = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, f, 0x0024_61a8)?;
    }
    // The graphics manager's vtable slot 8, cleared.
    c.cycles += 7;
    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
    c.g[A1][0] = 0;
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
    let f = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, f, 0x0024_61c4)
}

/// `cFEStateProgressiveVerifyPS2::OnWidgetAction(self = a0, item = a1, msg = a2)`
/// (0x2461e0, 132 B). For the widget message 5 on a live item whose hash
/// (`item+0x38`) equals the hash of the "yes" string, it creates a memory-card
/// state (584 bytes) and pushes it on the engine's state stack.
///
/// Register and cycle behaviour follows the generated body: the S0 reload
/// happens after each compare, and the end block restores S0 again on the
/// allocation path, as the compiled code does.
pub fn cfe_progressive_verify_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: 0x30-byte frame, s0/s1/ra saved.
        c.cycles += 6;
        let f = Frame::open(c, 0x30);
        f.save(c, S1, 0x10);
        f.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let no_item = c.g[A1][0] == 0;
        f.save_ra(c, 0);

        // `reached_end`: the body ran to its end block (cycles and S0 reload
        // there), not an early exit.
        let reached_end = if no_item {
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let other_msg = c.g[A2][0] != c.g[V0][0];
            f.restore(c, S0, 0x20);
            if other_msg {
                false
            } else {
                c.cycles += 6;
                let item = c.g[A1][0] as u32;
                c.g[A0][0] = 0x004a_0000u64;
                c.g[S0][0] = Addr(item).off(0x38).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ac8));
                call_ptr(c, 0x0031_7670, 0x0024_6214)?;
                let hash_differs = c.g[S0][0] != c.g[V0][0];
                f.restore(c, S0, 0x20);
                if hash_differs {
                    false
                } else {
                    // cMemMan_alloc(584, "cFEStateMemCard", 0, 0)
                    c.cycles += 13;
                    c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x4688));
                    c.g[A0][0] = 0x248;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, 0x0024_6234)?;
                    // func_00186A08(state, self->mpEngine)
                    let this = c.g[S1][0] as u32;
                    c.g[A1][0] = Addr(this).off(0x10).lw(c);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0018_6a08, 0x0024_6240)?;
                    // cUIStateStack_push(&engine->mStack, state)
                    c.g[A0][0] = Addr(this).off(0x10).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, 0x0024_6250)?;
                    true
                }
            }
        };

        if reached_end {
            c.cycles += 1;
            f.restore(c, S0, 0x20);
        }

        // Epilogue.
        c.cycles += 4;
        f.restore(c, S1, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `SSXApp_GlobalCtors` (0x247e20, 32 B): `SSXApp_StaticInitDestruct(1, 0xffff)`,
/// the static constructors' pass of the static-init table.
pub fn ssxapp_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let f = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        f.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        call_ptr(c, 0x0024_2eb8, 0x0024_7e34)?;
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `SSXApp_GlobalDtors` (0x247e40, 32 B): `SSXApp_StaticInitDestruct(0, 0xffff)`,
/// the static destructors' pass of the same table.
pub fn ssxapp_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let f = Frame::open(c, 0x10);
        c.g[A0][0] = 0;
        f.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        call_ptr(c, 0x0024_2eb8, 0x0024_7e54)?;
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::main::game::tests::oc804_install_fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::ai::ai::tests::belibrary;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::dtor_setup;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::rng_2 as rng;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;

    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS;

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
        check(
            ssx3_ee::s06::f_00245b58__generated,
            c_fe_load_state_dtor,
            80,
            &|c, seed| load_state_dtor_setup(c, seed, 0x23),
        );
    }

    #[test]
    fn c_fe_load_state_in_lodge_dtor_matches() {
        check(
            ssx3_ee::s06::f_00245d28__generated,
            c_fe_load_state_in_lodge_dtor,
            80,
            &|c, seed| load_state_dtor_setup(c, seed, 0x24),
        );
    }

    #[test]
    fn generic_stream_pause_dtor_matches() {
        check(
            ee::s06::f_00245f00__generated,
            generic_stream_pause_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x245f00),
        );
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
        check(
            ssx3_ee::s06::f_00245e30__generated,
            game_mode_man_compare_by_score,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x245e30, 0);
                mode_tables(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 4);
                word(c, DATA, (r.next_u32() % 8) as u32);
                word(c, DATA + 4, (r.next_u32() % 8) as u32);
            },
        );
        check(
            ssx3_ee::s06::f_00245e78__generated,
            game_mode_man_compare_by_time,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x245e78, 0);
                mode_tables(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 4);
                word(c, DATA, (r.next_u32() % 8) as u32);
                word(c, DATA + 4, (r.next_u32() % 8) as u32);
            },
        );
        check(
            ssx3_ee::s06::f_00245ec0__generated,
            game_mode_man_compare_standings,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x245ec0, 0);
                mode_tables(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 4);
                word(c, DATA, (r.next_u32() % 8) as u32);
                word(c, DATA + 4, (r.next_u32() % 8) as u32);
            },
        );
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
        diff(
            ee::s06::f_00245b80__generated,
            c_fe_load_state_init,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x24_5b80);
                c.g[A0][0] = sx(DATA);
                w(c, DATA + 0x10, r.next_u32());
                w(c, DATA + 0x20, r.next_u32());
            },
        );
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

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::put_words;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The fakes' gate and their controlled results.
    pub(crate) const OC394_CTRL: u32 = DATA + 0xf_0000;

    pub(crate) const OC394_MAGIC: u32 = 0x0c39_4a11;

    /// `BENewRaceInt_GetTrackPeak`, `..GetTrackField60`, `..GetTrack`,
    /// `BENewPlayerInt_GetHumanRider` results.
    pub(crate) const OC394_PEAK: u32 = OC394_CTRL + 4;

    pub(crate) const OC394_KIND: u32 = OC394_CTRL + 8;

    pub(crate) const OC394_TRACK: u32 = OC394_CTRL + 0xc;

    pub(crate) const OC394_RIDER: u32 = OC394_CTRL + 0x10;

    /// The callees: logged by `fakes::fake`; `StrHash` returns its key's
    /// address (so a widget id equal to a key address matches that key) and
    /// the interface getters the results the fixture chose.
    pub(crate) fn oc394_fake_body(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let a0 = c.g[A0][0];
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        let forced = |c: &mut Cpu, at: u32| sx(c.bus.read32(at));
        match target {
            QPW_HASH => c.g[V0][0] = a0,
            0x0014_4c98 => c.g[V0][0] = forced(c, OC394_PEAK),
            0x0014_4d18 => c.g[V0][0] = forced(c, OC394_KIND),
            QPW_GET_TRACK => c.g[V0][0] = forced(c, OC394_TRACK),
            QPW_GET_HUMAN_RIDER => c.g[V0][0] = forced(c, OC394_RIDER),
            _ => {}
        }
    }

    macro_rules! oc394_gated {
        ($name:ident, $prev:ident, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC394_CTRL) != OC394_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let target = c.pc;
                oc394_fake_body(c, target);
            }
        };
    }

    oc394_gated!(
        oc394_fake_14dc80,
        OC394_PREV_14DC80,
        ee::s01::f_0014dc80__generated
    );

    oc394_gated!(
        oc394_fake_14dd58,
        OC394_PREV_14DD58,
        ee::s01::f_0014dd58__generated
    );

    oc394_gated!(
        oc394_fake_317670,
        OC394_PREV_317670,
        ee::s10::f_00317670__generated
    );

    oc394_gated!(
        oc394_fake_416210,
        OC394_PREV_416210,
        ee::s15::f_00416210__generated
    );

    oc394_gated!(
        oc394_fake_144c98,
        OC394_PREV_144C98,
        ee::s01::f_00144c98__generated
    );

    oc394_gated!(
        oc394_fake_144bf0,
        OC394_PREV_144BF0,
        ee::s01::f_00144bf0__generated
    );

    oc394_gated!(
        oc394_fake_2c27c0,
        OC394_PREV_2C27C0,
        ee::s08::f_002c27c0__generated
    );

    oc394_gated!(
        oc394_fake_3a0e90,
        OC394_PREV_3A0E90,
        ee::s13::f_003a0e90__generated
    );

    oc394_gated!(
        oc394_fake_144be0,
        OC394_PREV_144BE0,
        ee::s01::f_00144be0__generated
    );

    oc394_gated!(
        oc394_fake_144d18,
        OC394_PREV_144D18,
        ee::s01::f_00144d18__generated
    );

    oc394_gated!(
        oc394_fake_2c2508,
        OC394_PREV_2C2508,
        ee::s08::f_002c2508__generated
    );

    oc394_gated!(
        oc394_fake_146e98,
        OC394_PREV_146E98,
        ee::s01::f_00146e98__generated
    );

    oc394_gated!(
        oc394_fake_14ee58,
        OC394_PREV_14EE58,
        ee::s01::f_0014ee58__generated
    );

    oc394_gated!(
        oc394_fake_148950,
        OC394_PREV_148950,
        ee::s01::f_00148950__generated
    );

    oc394_gated!(
        oc394_fake_2c26d0,
        OC394_PREV_2C26D0,
        ee::s08::f_002c26d0__generated
    );

    oc394_gated!(
        oc394_fake_3a0c70,
        OC394_PREV_3A0C70,
        ee::s13::f_003a0c70__generated
    );

    pub(crate) fn oc394_install() {
        use fakes::hook;
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc394_fake_14dc80,
            &OC394_PREV_14DC80,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc394_fake_14dd58,
            &OC394_PREV_14DD58,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc394_fake_317670,
            &OC394_PREV_317670,
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416210),
            oc394_fake_416210,
            &OC394_PREV_416210,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144C98),
            oc394_fake_144c98,
            &OC394_PREV_144C98,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BF0),
            oc394_fake_144bf0,
            &OC394_PREV_144BF0,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C27C0),
            oc394_fake_2c27c0,
            &OC394_PREV_2C27C0,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0E90),
            oc394_fake_3a0e90,
            &OC394_PREV_3A0E90,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BE0),
            oc394_fake_144be0,
            &OC394_PREV_144BE0,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144D18),
            oc394_fake_144d18,
            &OC394_PREV_144D18,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2508),
            oc394_fake_2c2508,
            &OC394_PREV_2C2508,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146E98),
            oc394_fake_146e98,
            &OC394_PREV_146E98,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014EE58),
            oc394_fake_14ee58,
            &OC394_PREV_14EE58,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00148950),
            oc394_fake_148950,
            &OC394_PREV_148950,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C26D0),
            oc394_fake_2c26d0,
            &OC394_PREV_2C26D0,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C70),
            oc394_fake_3a0c70,
            &OC394_PREV_3A0C70,
        );
    }

    /// Widget names (key addresses; the fake `StrHash` returns the address)
    /// and the game mode / location kind that reaches each one's test.
    pub(crate) const OC394_KEYS: &[(u32, &[i32])] = &[
        (0x0047_bb70, &[]),     // peak line
        (0x0047_bb80, &[0, 4]), // mode icons, first switch
        (0x0047_bb98, &[1]),
        (0x0047_bba8, &[2]),
        (0x0047_bbb8, &[3]),
        (0x0047_bbc8, &[5, 6]),
        (0x004a_2be0, &[0]), // second switch
        (0x004a_2be8, &[1]),
        (0x004a_2bf0, &[2]),
        (0x004a_2bf8, &[3]),
        (0x0047_bbd8, &[5]),
        (0x0047_bbe8, &[6]),
        (0x004a_2c00, &[]), // location icons (kind 0, 1, 2)
        (0x0047_bbf8, &[]),
        (0x0047_bc08, &[]),
        (0x0047_bc18, &[]), // statistics lines
        (0x0047_bc48, &[]),
        (0x0047_bc60, &[]), // rider name
        (0x0047_bc70, &[]), // rating line
        (0x004a_2c08, &[]), // hint texture
        (0x0012_3400, &[]), // no widget of this function
    ];

    pub(crate) fn oc394_setup(c: &mut Cpu, seed: u32) {
        let mut r: Rng = machine(c, seed, 0x0024_64e8, 14);
        c.pc = 0x0024_64e8;
        c.bus.write32(OC394_CTRL, OC394_MAGIC);
        let pick = |r: &mut Rng, n: u32| r.next_u32() % n;

        // The jump tables and the two 17-word statistics tables.
        put_words(
            c,
            QPW_MODE_TABLE_1,
            &[
                0x2467e8, 0x246808, 0x246828, 0x246848, 0x246868, 0x246888, 0x2468a8,
            ],
        );
        put_words(
            c,
            QPW_MODE_TABLE_2,
            &[
                0x2468ec, 0x24690c, 0x24692c, 0x24694c, 0x2469a4, 0x24696c, 0x24698c, 0x2469a4,
            ],
        );
        for i in 0..18 {
            c.bus.write32(0x0047_bae0 + 4 * i, r.next_u32());
            c.bus.write32(0x0047_bb28 + 4 * i, r.next_u32());
        }
        // Rider records (12 bytes each; bit 1 of the first word).
        for i in 0..8 {
            c.bus.write32(0x0053_08b8 + 0xc * i, r.next_u32());
        }

        // The widget, its vtable (slot 9) and the load state.
        let widget = DATA + 0x1000;
        let (key, modes) = OC394_KEYS[(seed as usize) % OC394_KEYS.len()];
        c.bus.write32(widget + QPW_WIDGET_ID, key);
        let vt = DATA + 0x2000;
        c.bus.write32(widget + QPW_WIDGET_VT, vt);
        c.bus.write16(vt + 0x48, (r.next_u32() & 0xfc) as u16);
        c.bus.write32(vt + 0x4c, fakes::FAKE + 0x90);
        let state = DATA + 0x3000;
        let hint = DATA + 0x3800;
        let hint_ptr = if pick(&mut r, 4) == 0 { 0 } else { hint };
        c.bus.write32(state + QPW_STATE_HINT, hint_ptr);
        let tex = if pick(&mut r, 3) == 0 {
            0
        } else {
            r.next_u32()
        };
        c.bus.write32(hint + 0x10, tex);

        // The world's locale object and its lookup slot (vt+0x20/0x24).
        let world = DATA + 0x4000;
        c.bus.write32(GAME_GP_WORLD, world);
        let locale = DATA + 0x5000;
        c.bus.write32(world + 0x8c, locale);
        let lvt = DATA + 0x6000;
        c.bus.write32(locale + 4, lvt);
        c.bus.write16(lvt + 0x20, (r.next_u32() & 0x7c) as u16);
        c.bus.write32(lvt + 0x24, fakes::FAKE + 0xa0);

        // The game mode: the one the key wants most of the time.
        let mode = if !modes.is_empty() && pick(&mut r, 4) != 0 {
            modes[pick(&mut r, modes.len() as u32) as usize]
        } else {
            pick(&mut r, 10) as i32 - 1
        };
        c.bus.write8(QPW_GAME_MODE, mode as u8);
        // Peak / location kind: -1..3, mostly matching the key's case.
        c.bus
            .write32(OC394_PEAK, (pick(&mut r, 5) as i32 - 1) as u32);
        let kind = match key {
            0x004a_2c00 => 0,
            0x0047_bbf8 => 1,
            0x0047_bc08 => 2,
            _ => pick(&mut r, 5) as i32 - 1,
        };
        let kind = if pick(&mut r, 5) == 0 {
            pick(&mut r, 5) as i32 - 1
        } else {
            kind
        };
        c.bus.write32(OC394_KIND, kind as u32);
        c.bus.write32(OC394_TRACK, pick(&mut r, 17));
        c.bus.write32(OC394_RIDER, pick(&mut r, 8));

        c.g[A0][0] = sx(state);
        c.g[A1][0] = sx(widget);
    }

    /// `gp - 0x848`: the world pointer.
    pub(crate) const GAME_GP_WORLD: u32 = crate::difftest::GAME_GP - 0x848;

    /// `cGameLoadState_SetupQPWidget`: every widget name, each game mode
    /// (-1..8, both jump tables), peak and location kinds -1..3, hint
    /// pointers null / empty / set, callee unwinds.
    #[test]
    fn c_game_load_state_setup_qp_widget_matches() {
        oc394_install();
        covered(
            ee::s06::f_002464e8__generated,
            c_game_load_state_setup_qp_widget,
            420,
            &oc394_setup,
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fill;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::bx::bxstringctor::tests::oc804_fixture;

    pub(crate) use crate::game::bx::bxstringctor::tests::OC804_SELF;
    pub(crate) use crate::game::bx::bxstringctor::tests::OC804_WORLD;

    pub(crate) const OC804_Z: u32 = DATA + 0x28_0000;

    pub(crate) const OC804_TBL: u32 = DATA + 0x2b_0000;

    pub(crate) const OC804_P: u32 = DATA + 0x2c_0000;

    pub(crate) const OC804_Q: u32 = DATA + 0x2d_0000;

    /// Per-callee forced results: one word per fake, at `OC804_FORCE + (addr & 0xffffc)`.
    pub(crate) const OC804_FORCE: u32 = DATA + 0x30_0000;

    /// The mailbox word of the fake for `addr`.
    pub(crate) fn oc804_mail(addr: u32) -> u32 {
        OC804_FORCE + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0`.
    pub(crate) fn oc804_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc804_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc804_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc804_mail(addr), 1);
    }

    /// `cGameLoadState_UpdatePlayerRanking(this, player)`: fills the ranking
    /// tables of the loading screen from the back-end interfaces and the stat
    /// tables of the player (`0x5308b8`), through the real loops, ending with
    /// the load-state object's texture objects.
    #[test]
    fn game_load_state_update_player_ranking_matches() {
        oc804_install_fakes();
        fakes::covered(
            ee::s06::f_00246cf8__generated,
            game_load_state_update_player_ranking,
            260,
            &|c, seed| {
                elf_image(c);
                let mut r = oc804_fixture(c, seed, 0x0024_6cf8, 40);
                c.bus.write8(0x0053_5c11, if seed % 9 == 0 { 1 } else { 2 });
                c.bus.write32(GAME_GP.wrapping_sub(0x238), seed % 2);
                for i in 0..0x48u32 {
                    let w = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(0x0053_08b8 + 4 * i, w);
                }
                c.bus
                    .write32(0x0053_4b30 + 4, if seed % 3 == 0 { 0 } else { 1 });
                for i in 0..8u32 {
                    c.bus.write32(0x0053_4b30 + 4 * i + 4, r.next_u32() % 2);
                }
                fill(c, &mut r, OC804_SELF, 0x20);
                c.bus
                    .write32(OC804_SELF + 0x48, if seed % 7 == 0 { 0 } else { OC804_TBL });
                for i in 0..0x20u32 {
                    c.bus.write32(OC804_TBL + 4 * i, r.next_u32() % 3);
                }
                c.bus.write32(OC804_WORLD + 0x78, OC804_Z);
                c.bus.write32(OC804_WORLD + 0x8c, OC804_P);
                c.bus.write32(OC804_P + 4, OC804_Q);
                c.bus.write16(OC804_Q + 0x20, 0);
                c.bus.write32(OC804_Q + 0x24, fakes::FAKE + 0x30);
                for i in 0..7u32 {
                    c.bus.write8(OC804_Z + 0x4a8 + i, (r.next_u32() % 4) as u8);
                }
                oc804_force(c, 0x0014_6e98, seed % 6);
                if seed % 4 == 0 {
                    oc804_force_zero(c, 0x0039_d860);
                }
                if seed % 5 == 1 {
                    oc804_force_zero(c, 0x0014_8950);
                }
                c.g[A0][0] = sx(OC804_SELF);
                c.g[A1][0] = [0u64, 1, 2, 3, 0xff][(seed % 5) as usize];
                c.pc = 0x0024_6cf8;
            },
        );
    }

    use crate::difftest::fakes::check_reaching;
    pub(crate) use crate::game::fe::festateloadscreen::tests::oc1016_machine;
    pub(crate) use crate::game::fe::festateloadscreen::tests::OC1016_CTRL;
    pub(crate) use crate::game::fe::festateloadscreen::tests::OC1016_MAGIC;
    pub(crate) use crate::game::sound::ssxaudio::tests::oc1017_setup;
    pub(crate) use crate::game::sound::ssxaudio::tests::oc1017_w;
    pub(crate) use crate::game::sound::ssxaudio::tests::OC1017_CTRL;

    pub(crate) use crate::game::sound::ssxaudio::tests::OC1017_OBJ;
    pub(crate) use crate::game::sound::ssxaudio::tests::OC1017_SELF;

    // Fixture objects, all in DATA+0x400000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC1007_CTRL: u32 = DATA + 0x40_0000;

    pub(crate) const OC1007_MAGIC: u32 = 0x1007_5a11;

    /// Bit k of this word: the k-th widget lookup finds the object.
    pub(crate) const OC1007_FIND_MASK: u32 = OC1007_CTRL + 4;

    /// Number of widget lookups made so far.
    pub(crate) const OC1007_FIND_COUNT: u32 = OC1007_CTRL + 8;

    /// The human-rider index the rider getter returns.
    pub(crate) const OC1007_RIDER: u32 = OC1007_CTRL + 0xc;

    pub(crate) const OC1007_STATS: u32 = DATA + 0x41_0000;

    pub(crate) const OC1007_WORLD: u32 = DATA + 0x42_0000;

    pub(crate) const OC1007_SELF: u32 = DATA + 0x43_0000;

    pub(crate) const OC1007_SCREEN: u32 = DATA + 0x44_0000;

    pub(crate) const OC1007_OBJ: u32 = DATA + 0x45_0000;

    pub(crate) const OC1007_OBJ_VT: u32 = DATA + 0x46_0000;

    pub(crate) const OC1007_LOCALE: u32 = DATA + 0x47_0000;

    pub(crate) const OC1007_LOCALE_VT: u32 = DATA + 0x48_0000;

    pub(crate) const OC1007_PARENT: u32 = DATA + 0x49_0000;

    /// The callee: logs the call like `fakes::fake_call`, then applies the
    /// fixture's results: the widget finder answers per the lookup mask, the
    /// rider getter and the stats getter return the fixture's values.
    pub(crate) fn oc1007_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        match addr {
            0x0039_d860 => {
                let n = c.bus.read32(OC1007_FIND_COUNT);
                c.bus.write32(OC1007_FIND_COUNT, n + 1);
                let mask = c.bus.read32(OC1007_FIND_MASK);
                c.g[V0][0] = if (mask >> (n % 32)) & 1 != 0 {
                    OC1007_OBJ as u64
                } else {
                    0
                };
            }
            0x0014_6e98 => c.g[V0][0] = c.bus.read32(OC1007_RIDER) as u64,
            0x0015_5270 => c.g[V0][0] = OC1007_STATS as u64,
            _ => {}
        }
    }

    macro_rules! oc1007_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1007_CTRL) != OC1007_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1007_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc1007_install() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1007_fakes!(
        (
            oc1007_fake_0014dc80,
            OC1007_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc1007_fake_0014dd58,
            OC1007_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc1007_fake_00146e98,
            OC1007_PREV_00146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x00146e98
        ),
        (
            oc1007_fake_00155270,
            OC1007_PREV_00155270,
            ee::s01::OVERRIDE_00155270,
            ee::s01::f_00155270__generated,
            0x00155270
        ),
        (
            oc1007_fake_00148950,
            OC1007_PREV_00148950,
            ee::s01::OVERRIDE_00148950,
            ee::s01::f_00148950__generated,
            0x00148950
        ),
        (
            oc1007_fake_002c26d0,
            OC1007_PREV_002C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c26d0
        ),
        (
            oc1007_fake_002c2508,
            OC1007_PREV_002C2508,
            ee::s08::OVERRIDE_002C2508,
            ee::s08::f_002c2508__generated,
            0x002c2508
        ),
        (
            oc1007_fake_00317670,
            OC1007_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc1007_fake_0039d860,
            OC1007_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc1007_fake_003a0c50,
            OC1007_PREV_003A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a0c50
        ),
        (
            oc1007_fake_003a0e90,
            OC1007_PREV_003A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a0e90
        ),
        (
            oc1007_fake_00417828,
            OC1007_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
        (
            oc1007_fake_004139f8,
            OC1007_PREV_004139F8,
            ee::s15::OVERRIDE_004139F8,
            ee::s15::f_004139f8__generated,
            0x004139f8
        ),
        (
            oc1007_fake_004166f4,
            OC1007_PREV_004166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x004166f4
        ),
        (
            oc1007_fake_0020a1a8,
            OC1007_PREV_0020A1A8,
            ee::s05::OVERRIDE_0020A1A8,
            ee::s05::f_0020a1a8__generated,
            0x0020a1a8
        ),
    );

    /// A load-state object with its screen and parent module, the world with
    /// the locale lookup slot, the network and game-mode globals and the
    /// per-rider result tables, all chosen from `seed`.
    pub(crate) fn oc1007_setup(c: &mut Cpu, seed: u32) {
        elf_image(c);
        let mut r: Rng = machine(c, seed, 0x0024_72c0, 45);
        c.pc = 0x0024_72c0;
        c.bus.write32(OC1007_CTRL, OC1007_MAGIC);
        let pick = |r: &mut Rng, n: u32| r.next_u32() % n;

        // Which widgets exist: mostly most of them, sometimes all or none.
        let mask = match seed % 11 {
            0 => 0,
            1 | 2 => u32::MAX,
            _ => r.next_u32() | r.next_u32(),
        };
        c.bus.write32(OC1007_FIND_MASK, mask);
        c.bus.write32(OC1007_RIDER, pick(&mut r, 6));
        fill(c, &mut r, OC1007_STATS, 0x20);

        // The world (gp-0x848): +0x78 back-end root, +0x8c the locale object whose
        // vtable slot (+0x24, offset in +0x20) the level text comes from.
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC1007_WORLD);
        c.bus.write32(OC1007_WORLD + 0x78, DATA + 0x4a_0000);
        c.bus.write32(OC1007_WORLD + 0x8c, OC1007_LOCALE);
        c.bus.write32(OC1007_LOCALE + 4, OC1007_LOCALE_VT);
        c.bus
            .write16(OC1007_LOCALE_VT + 0x20, (r.next_u32() & 0x7c) as u16);
        c.bus.write32(OC1007_LOCALE_VT + 0x24, fakes::FAKE + 0x30);
        // GameModeMan_GetInstance finds its instance (gp-0x484) already built.
        c.bus.write32(GAME_GP.wrapping_sub(0x484), DATA + 0x5000);

        // Network options (0x534b30: network game, ranked game, host) and game mode.
        for i in 0..3u32 {
            c.bus.write32(0x0053_4b30 + 4 * i, pick(&mut r, 2));
        }
        let modes = [0u8, 5, 1, 2, 3, 4, 0xff];
        c.bus
            .write8(0x0053_5c10, modes[(seed as usize / 3) % modes.len()]);
        // Rankings and per-rider scores/times (half with the sign bit, for the
        // unsigned-to-float conversion).
        for i in 0..8u32 {
            c.bus.write32(0x0053_6730 + 4 * i, pick(&mut r, 8));
            let w = r.next_u32();
            c.bus
                .write32(0x0053_6640 + 4 * i, if i % 2 == 0 { w } else { w >> 1 });
        }

        // The found widget with its vtable (slot 0x48/0x4c: `setVisible`).
        c.bus.write32(OC1007_OBJ + 8, OC1007_OBJ_VT);
        c.bus
            .write16(OC1007_OBJ_VT + 0x48, (r.next_u32() & 0x3c) as u16);
        c.bus.write32(OC1007_OBJ_VT + 0x4c, fakes::FAKE + 0x40);

        // The load state: screen (+0x40) and parent module (+0x48, sometimes none)
        // with its per-player texture ids (+0x10..; half of them 0).
        c.bus.write32(OC1007_SELF + 0x40, OC1007_SCREEN);
        let parent = if seed % 7 == 3 { 0 } else { OC1007_PARENT };
        c.bus.write32(OC1007_SELF + 0x48, parent);
        for i in 0..0x20u32 {
            let w = if r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32()
            };
            c.bus.write32(OC1007_PARENT + 4 * i, w);
        }

        c.g[A0][0] = sx(OC1007_SELF);
        c.g[A1][0] = [0u64, 1, 2, 0xff, 0x7f, 0x100][(seed % 6) as usize];
    }

    /// `cFELoadState_ShowMPPlayerStats(this, player)`: sets the multiplayer
    /// result texts (rank, level, time or score, stat counters, picture texture)
    /// through the real control flow; the callees are fakes (some unwinding)
    /// whose lookups find some widgets and not others.
    #[test]
    fn cfe_load_state_show_mp_player_stats_matches() {
        oc1007_install();
        covered(
            ee::s06::f_002472c0__generated,
            cfe_load_state_show_mp_player_stats,
            300,
            &oc1007_setup,
        );
    }

    /// Every arm makes its calls for some seed: the rank number, the stat
    /// level (both rank sources), the unicode text, race time and score, the six
    /// stat counters and the picture lookup.
    #[test]
    fn cfe_load_state_show_mp_player_stats_reaches_every_arm() {
        oc1007_install();
        check_reaching(
            ee::s06::f_002472c0__generated,
            cfe_load_state_show_mp_player_stats,
            300,
            &[
                0x0024_7578,
                0x0024_75c0,
                0x0024_75cc,
                0x0024_7630,
                0x0024_763c,
                0x0024_7698,
                0x0024_76a4,
                0x0024_7730,
                0x0024_7770,
                0x0024_7788,
                0x0024_77e8,
                0x0024_7800,
                0x0024_780c,
                0x0024_7858,
                0x0024_78b0,
                0x0024_7908,
                0x0024_7960,
                0x0024_79b8,
                0x0024_7a10,
                0x0024_7a1c,
                0x0024_7a54,
            ],
            &oc1007_setup,
        );
    }

    /// Gate word of this module's callee fakes (in the machine's RAM).
    pub(crate) const OC1008_CTRL: u32 = DATA + 0xe_1000;

    pub(crate) const OC1008_MAGIC: u32 = 0x1008_c0de;

    pub(crate) const OC1008_OBJ: u32 = DATA + 0x5_0000;

    pub(crate) const OC1008_OBJ2: u32 = DATA + 0x5_1000;

    pub(crate) const OC1008_WORLD: u32 = DATA + 0x6_0000;

    pub(crate) const OC1008_IFACE: u32 = DATA + 0x6_2000;

    pub(crate) const OC1008_IFACE_VT: u32 = DATA + 0x6_3000;

    /// The game-mode byte read by the second half.
    pub(crate) const OC1008_MODE: u32 = 0x0053_5c10;

    /// Callee fake: logs the call; `StrHash` (0x317670) answers its argument
    /// (so `obj+0x38` equal to the string address matches), `GetTrackPeak`
    /// (0x144c98) answers the value in the mailbox at `CTRL+0x10`.
    pub(crate) fn oc1008_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        if addr == 0x0031_7670 {
            c.g[V0][0] = a0;
        } else if addr == 0x0014_4c98 {
            let peak = c.bus.read32(OC1008_CTRL + 0x10);
            c.g[V0][0] = sx(peak);
        }
    }

    macro_rules! oc1008_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1008_CTRL) != OC1008_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1008_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc1008_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1008_fakes!(
        (
            oc1008_fake_00144bf0,
            OC1008_PREV_00144BF0,
            ee::s01::OVERRIDE_00144BF0,
            ee::s01::f_00144bf0__generated,
            0x00144bf0
        ),
        (
            oc1008_fake_00144c98,
            OC1008_PREV_00144C98,
            ee::s01::OVERRIDE_00144C98,
            ee::s01::f_00144c98__generated,
            0x00144c98
        ),
        (
            oc1008_fake_0014dc80,
            OC1008_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc1008_fake_0014dd58,
            OC1008_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc1008_fake_002c27c0,
            OC1008_PREV_002C27C0,
            ee::s08::OVERRIDE_002C27C0,
            ee::s08::f_002c27c0__generated,
            0x002c27c0
        ),
        (
            oc1008_fake_00317670,
            OC1008_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc1008_fake_003a0e90,
            OC1008_PREV_003A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a0e90
        ),
        (
            oc1008_fake_00416210,
            OC1008_PREV_00416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x00416210
        ),
    );

    /// The load-state widget fixture: the object whose name hash is one of
    /// the label strings (or none), a track-peak answer, and a mode byte.
    pub(crate) fn setup_mp_widget_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x246278, 8);
        elf_image(c);
        oc1008_install_fakes();
        c.bus.write32(OC1008_CTRL, OC1008_MAGIC);
        // Track-peak result: 0, 1, 2, 3, -1, 5.
        let peak: [i32; 6] = [0, 1, 2, 3, -1, 5];
        c.bus
            .write32(OC1008_CTRL + 0x10, peak[(seed / 7 % 6) as usize] as u32);
        // Mode byte 0..6, 7 and 0x80 (outside the table).
        let mode = [0u8, 1, 2, 3, 4, 5, 6, 7, 0x80][(seed / 3 % 9) as usize];
        c.bus.write8(OC1008_MODE, mode);
        // The label this mode looks for, per the jump table.
        let label = [
            0x0047_ba88u32,
            0x0047_baa0,
            0x0047_bab0,
            0x0047_bac0,
            0x0047_ba88,
            0x0047_bad0,
            0x0047_bad0,
        ];
        let key = match seed % 5 {
            0 | 1 => 0x0047_ba48,
            2 | 3 => label.get(mode as usize).copied().unwrap_or(0x0047_ba88),
            _ => r.next_u32() | 1,
        };
        c.bus.write32(OC1008_OBJ + 0x38, key);
        c.bus.write32(OC1008_OBJ + 8, OC1008_OBJ2);
        c.bus
            .write16(OC1008_OBJ2 + 0x48, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(OC1008_OBJ2 + 0x4c, fakes::FAKE);
        // The world and the interface object whose vtable it calls through.
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC1008_WORLD);
        c.bus.write32(OC1008_WORLD + 0x8c, OC1008_IFACE);
        c.bus.write32(OC1008_IFACE + 4, OC1008_IFACE_VT);
        c.bus
            .write16(OC1008_IFACE_VT + 0x20, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(OC1008_IFACE_VT + 0x24, fakes::FAKE);
        c.g[A0][0] = DATA as u64 + 0x7_0000;
        c.g[A1][0] = OC1008_OBJ as u64;
        c.pc = 0x0024_6278;
    }

    /// cGameLoadState_SetupMPWidget against the generated body, with every
    /// callee faked: the peak-label arm (all `GetTrackPeak` answers) and the
    /// per-mode label arms, matching and not, some seeds unwinding in a call.
    #[test]
    fn game_load_state_setup_mp_widget_matches() {
        check(
            ee::s06::f_00246278__generated,
            c_game_load_state_setup_mp_widget,
            180,
            &setup_mp_widget_setup,
        );
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..180 {
            let mut c = crate::difftest::fresh();
            setup_mp_widget_setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s06::f_00246278__generated);
            seen.insert(c.cycles - before);
        }
        assert!(
            seen.len() >= 8,
            "fixture reaches too few paths: {}",
            seen.len()
        );
    }

    /// Gate word of this module's fakes (its own address and magic in the
    /// machine's RAM, so other tests' machines run the real bodies).
    pub(crate) const OC1009_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC1009_MAGIC: u32 = 0x1009_f00d;

    /// Per-fake call counters (`+ 4 * index`) and result queues (`+ 0x40 *
    /// index`, 16 words: 0 = the fake's own pointer, 1 = zero, else the value).
    pub(crate) const OC1009_CNT: u32 = DATA + 0x20_0800;

    pub(crate) const OC1009_Q: u32 = DATA + 0x20_1000;

    pub(crate) const OC1009_GOBJ: u32 = DATA + 0x21_3000;

    /// Every faked callee: a gated fake in its override slot that logs the
    /// call (all registers) and answers from its result queue.
    macro_rules! oc1009_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            const OC1009_TARGETS: &[u32] = &[$($addr),*];
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1009_CTRL) != OC1009_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    oc1009_fake(c, $addr);
                }
            )*
            /// Puts every fake in its override slot (once per process).
            fn oc1009_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc1009_fakes! {
        oc1009_fake_0014dc80, 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc1009_fake_0014dd58, 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc1009_fake_00317d70, 0x0031_7d70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated;
        oc1009_fake_00397b08, 0x0039_7b08, ee::s13::OVERRIDE_00397B08, ee::s13::f_00397b08__generated;
        oc1009_fake_00397c40, 0x0039_7c40, ee::s13::OVERRIDE_00397C40, ee::s13::f_00397c40__generated;
        oc1009_fake_0039e2a0, 0x0039_e2a0, ee::s13::OVERRIDE_0039E2A0, ee::s13::f_0039e2a0__generated;
        oc1009_fake_0039f400, 0x0039_f400, ee::s13::OVERRIDE_0039F400, ee::s13::f_0039f400__generated;
        oc1009_fake_003168b0, 0x0031_68b0, ee::s10::OVERRIDE_003168B0, ee::s10::f_003168b0__generated;
        oc1009_fake_003168e8, 0x0031_68e8, ee::s10::OVERRIDE_003168E8, ee::s10::f_003168e8__generated;
        oc1009_fake_00147398, 0x0014_7398, ee::s01::OVERRIDE_00147398, ee::s01::f_00147398__generated;
        oc1009_fake_00417828, 0x0041_7828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated;
        oc1009_fake_00146e98, 0x0014_6e98, ee::s01::OVERRIDE_00146E98, ee::s01::f_00146e98__generated;
        oc1009_fake_003e2190, 0x003e_2190, ee::s14::OVERRIDE_003E2190, ee::s14::f_003e2190__generated;
        oc1009_fake_00317e98, 0x0031_7e98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated;
        oc1009_fake_00147488, 0x0014_7488, ee::s01::OVERRIDE_00147488, ee::s01::f_00147488__generated;
        oc1009_fake_00155270, 0x0015_5270, ee::s01::OVERRIDE_00155270, ee::s01::f_00155270__generated;
        oc1009_fake_00317670, 0x0031_7670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated;
        oc1009_fake_002c2540, 0x002c_2540, ee::s08::OVERRIDE_002C2540, ee::s08::f_002c2540__generated;
        oc1009_fake_0014ee58, 0x0014_ee58, ee::s01::OVERRIDE_0014EE58, ee::s01::f_0014ee58__generated;
        oc1009_fake_002c2508, 0x002c_2508, ee::s08::OVERRIDE_002C2508, ee::s08::f_002c2508__generated;
        oc1009_fake_00148950, 0x0014_8950, ee::s01::OVERRIDE_00148950, ee::s01::f_00148950__generated;
        oc1009_fake_002c26d0, 0x002c_26d0, ee::s08::OVERRIDE_002C26D0, ee::s08::f_002c26d0__generated;
        oc1009_fake_003a0e90, 0x003a_0e90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated;
        oc1009_fake_00413af8, 0x0041_3af8, ee::s15::OVERRIDE_00413AF8, ee::s15::f_00413af8__generated;
        oc1009_fake_003a0c50, 0x003a_0c50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated;
        oc1009_fake_0028f730, 0x0028_f730, ee::s07::OVERRIDE_0028F730, ee::s07::f_0028f730__generated;
        oc1009_fake_003b5a60, 0x003b_5a60, ee::s13::OVERRIDE_003B5A60, ee::s13::f_003b5a60__generated;
        oc1009_fake_002b0da0, 0x002b_0da0, ee::s08::OVERRIDE_002B0DA0, ee::s08::f_002b0da0__generated;
        oc1009_fake_003dedc0, 0x003d_edc0, ee::s14::OVERRIDE_003DEDC0, ee::s14::f_003dedc0__generated;
        oc1009_fake_002ab478, 0x002a_b478, ee::s08::OVERRIDE_002AB478, ee::s08::f_002ab478__generated;
        oc1009_fake_002aa648, 0x002a_a648, ee::s08::OVERRIDE_002AA648, ee::s08::f_002aa648__generated;
        oc1009_fake_0028bb10, 0x0028_bb10, ee::s07::OVERRIDE_0028BB10, ee::s07::f_0028bb10__generated;
        oc1009_fake_002adee8, 0x002a_dee8, ee::s08::OVERRIDE_002ADEE8, ee::s08::f_002adee8__generated;
        oc1009_fake_00317e50, 0x0031_7e50, ee::s10::OVERRIDE_00317E50, ee::s10::f_00317e50__generated;
        oc1009_fake_00285d98, 0x0028_5d98, ee::s07::OVERRIDE_00285D98, ee::s07::f_00285d98__generated;
        oc1009_fake_00288ae0, 0x0028_8ae0, ee::s07::OVERRIDE_00288AE0, ee::s07::f_00288ae0__generated;
        oc1009_fake_002a73a8, 0x002a_73a8, ee::s08::OVERRIDE_002A73A8, ee::s08::f_002a73a8__generated;
        oc1009_fake_00296e20, 0x0029_6e20, ee::s08::OVERRIDE_00296E20, ee::s08::f_00296e20__generated;
        oc1009_fake_00297438, 0x0029_7438, ee::s08::OVERRIDE_00297438, ee::s08::f_00297438__generated;
        oc1009_fake_0029b3c0, 0x0029_b3c0, ee::s08::OVERRIDE_0029B3C0, ee::s08::f_0029b3c0__generated;
        oc1009_fake_0028d960, 0x0028_d960, ee::s07::OVERRIDE_0028D960, ee::s07::f_0028d960__generated;
        oc1009_fake_002a4078, 0x002a_4078, ee::s08::OVERRIDE_002A4078, ee::s08::f_002a4078__generated;
        oc1009_fake_0028b1d8, 0x0028_b1d8, ee::s07::OVERRIDE_0028B1D8, ee::s07::f_0028b1d8__generated;
        oc1009_fake_0012a250, 0x0012_a250, ee::s00::OVERRIDE_0012A250, ee::s00::f_0012a250__generated;
        oc1009_fake_002a3708, 0x002a_3708, ee::s08::OVERRIDE_002A3708, ee::s08::f_002a3708__generated;
        oc1009_fake_002a34d0, 0x002a_34d0, ee::s08::OVERRIDE_002A34D0, ee::s08::f_002a34d0__generated;
        oc1009_fake_002a45c0, 0x002a_45c0, ee::s08::OVERRIDE_002A45C0, ee::s08::f_002a45c0__generated;
        oc1009_fake_00233640, 0x0023_3640, ee::s06::OVERRIDE_00233640, ee::s06::f_00233640__generated;
    }

    /// The logged call, with its result taken from the target's queue.
    pub(crate) fn oc1009_fake(c: &mut Cpu, target: u32) {
        fakes::fake_call(c, target);
        let idx = OC1009_TARGETS.iter().position(|t| *t == target).unwrap() as u32;
        let k = c.bus.read32(OC1009_CNT + 4 * idx);
        c.bus.write32(OC1009_CNT + 4 * idx, k + 1);
        if k < 16 {
            let w = c.bus.read32(OC1009_Q + 0x40 * idx + 4 * k);
            if w == 1 {
                c.g[V0][0] = 0;
            } else if w != 0 {
                c.g[V0][0] = sx(w);
            }
        }
    }

    /// The results of the successive calls to `target` (see `OC1009_Q`).
    pub(crate) fn oc1009_queue(c: &mut Cpu, target: u32, vals: &[u32]) {
        let idx = OC1009_TARGETS.iter().position(|t| *t == target).unwrap() as u32;
        for (k, v) in vals.iter().enumerate() {
            c.bus.write32(OC1009_Q + 0x40 * idx + 4 * k as u32, *v);
        }
    }

    /// Hooked fakes, scrambled registers, the gate on.
    pub(crate) fn oc1009_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1009_hook();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1009_CTRL, OC1009_MAGIC);
        r
    }

    /// The saved result record the widgets read.
    pub(crate) const OC1009_RECORD: u32 = DATA + 0x22_0000;

    pub(crate) const OC1009_NAME_OBJ: u32 = DATA + 0x22_1000;

    pub(crate) const OC1009_NAME_VT: u32 = DATA + 0x22_2000;

    /// `cFELoadState_SetupResultWidget`: every arm of the name chain (arm `seed % 10`;
    /// 9 = no arm matches), with varied statistics and race times, and unwinds.
    #[test]
    fn fe_load_state_setup_result_widget_matches() {
        check(
            ee::s06::f_00247ab0__generated,
            fe_load_state_setup_result_widget,
            200,
            &|c, seed| {
                let mut r = oc1009_machine(c, seed, 0x0024_7ab0, 26);
                let widget = DATA + 0x22_3000;
                c.g[A0][0] = (DATA + 0x21_0000) as u64;
                c.g[A1][0] = widget as u64;
                let hash = 0x1234_5000 | (r.next_u32() & 0xfff);
                c.bus.write32(widget + 0x38, hash);
                // GetHashValue32 results: misses until the arm, a hit at the arm
                let arm = seed % 10;
                let mut q = [0u32; 16];
                for (k, slot) in q.iter_mut().enumerate() {
                    *slot = if k as u32 == arm { hash } else { hash ^ 0x55 };
                }
                oc1009_queue(c, 0x0031_7670, &q);
                oc1009_queue(c, 0x0015_5270, &[OC1009_RECORD]);
                for off in [0x0c, 0x18, 0x20, 0x28, 0x2c, 0x88] {
                    c.bus.write32(OC1009_RECORD + off, r.next_u32());
                }
                let secs =
                    [0.0f32, 61.5, 125.25, 3599.9, 1e10, -7.0, 59.0][r.next_u32() as usize % 7];
                c.bus.write32(OC1009_RECORD + 0x48, secs.to_bits());
                // arm 2's object, vtable and its method
                c.bus.write32(GAME_GP - 0x848, OC1009_GOBJ);
                c.bus.write32(OC1009_GOBJ + 0x8c, OC1009_NAME_OBJ);
                c.bus.write32(OC1009_NAME_OBJ + 4, OC1009_NAME_VT);
                c.bus
                    .write16(OC1009_NAME_VT + 0x20, (r.next_u32() % 0x40) as u16);
                c.bus.write32(OC1009_NAME_VT + 0x24, fakes::FAKE);
            },
        );
    }

    /// A fake callee: logs the call (`fakes::fake`), then applies the forced
    /// result and the zero mask of this test.
    pub(crate) fn oc1016_fake_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        let forced = c.bus.read32(OC1016_CTRL + 16);
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        if forced != 0 {
            c.g[V0][0] = forced as u64;
        }
        let mask = c.bus.read64(OC1016_CTRL + 8);
        if n < 64 && (mask >> n) & 1 == 1 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc1016_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1016_CTRL) != OC1016_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1016_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc1016_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1016_fakes!(
        (
            oc1016_fake_00317e30,
            OC1016_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc1016_fake_00224df0,
            OC1016_PREV_00224DF0,
            ee::s05::OVERRIDE_00224DF0,
            ee::s05::f_00224df0__generated,
            0x0022_4df0
        ),
        (
            oc1016_fake_00225068,
            OC1016_PREV_00225068,
            ee::s05::OVERRIDE_00225068,
            ee::s05::f_00225068__generated,
            0x0022_5068
        ),
        (
            oc1016_fake_002227d0,
            OC1016_PREV_002227D0,
            ee::s05::OVERRIDE_002227D0,
            ee::s05::f_002227d0__generated,
            0x0022_27d0
        ),
        (
            oc1016_fake_00231278,
            OC1016_PREV_00231278,
            ee::s06::OVERRIDE_00231278,
            ee::s06::f_00231278__generated,
            0x0023_1278
        ),
        (
            oc1016_fake_00317670,
            OC1016_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc1016_fake_00397cc0,
            OC1016_PREV_00397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc1016_fake_0039c870,
            OC1016_PREV_0039C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc1016_fake_0039d860,
            OC1016_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc1016_fake_0039e390,
            OC1016_PREV_0039E390,
            ee::s13::OVERRIDE_0039E390,
            ee::s13::f_0039e390__generated,
            0x0039_e390
        ),
        (
            oc1016_fake_0039e510,
            OC1016_PREV_0039E510,
            ee::s13::OVERRIDE_0039E510,
            ee::s13::f_0039e510__generated,
            0x0039_e510
        ),
        (
            oc1016_fake_003a0c50,
            OC1016_PREV_003A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc1016_fake_003a1000,
            OC1016_PREV_003A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc1016_fake_00417828,
            OC1016_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
    );

    #[test]
    fn c_fe_load_hint_state_init_matches() {
        check(
            ee::s06::f_00245950__generated,
            c_fe_load_hint_state_init,
            300,
            &|c, seed| {
                let obj = DATA + 0x7000;
                let mask = fakes::mix(seed as u64) & fakes::mix(seed as u64 ^ 0x55);
                oc1016_machine(c, seed, 0x245950, 40, obj, mask);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x10, DATA + 0x2000);
                c.bus
                    .write32(this + 0x48, if seed % 3 == 0 { 0 } else { DATA + 0x3000 });
                c.bus.write32(
                    DATA + 0x3000 + 0x10,
                    if seed % 2 == 0 { 0 } else { DATA + 0x6000 },
                );
                c.bus.write8(GAME_GP.wrapping_sub(0x4b8), (seed % 14) as u8);
            },
        );
    }

    #[test]
    fn c_fe_state_progressive_verify_ps2_dtor_matches() {
        check(
            ee::s06::f_00245f30__generated,
            c_fe_state_progressive_verify_ps2_dtor,
            200,
            &|c, seed| {
                oc1016_machine(c, seed, 0x245f30, 4, 0, (seed % 2) as u64);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn c_fe_state_progressive_verify_ps2_update_matches() {
        check(
            ee::s06::f_00246098__generated,
            c_fe_state_progressive_verify_ps2_update,
            300,
            &|c, seed| {
                let obj = DATA + 0x7000;
                let mask = fakes::mix(seed as u64) & fakes::mix(seed as u64 ^ 0x55);
                oc1016_machine(c, seed, 0x246098, 16, obj, mask);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.bus
                    .write32(this + 0x4c, [0u32, 1, 2][(seed % 3) as usize]);
                c.bus.write8(this + 0x48, ((seed / 3) % 2) as u8);
                // Every fake returns `obj`, whose vtable calls the fake target.
                let vt = DATA + 0x7800;
                c.bus.write32(obj + 8, vt);
                c.bus.write16(vt + 0x40, 0);
                c.bus.write32(vt + 0x44, fakes::FAKE);
                c.bus.write16(vt + 0x48, 0);
                c.bus.write32(vt + 0x4c, fakes::FAKE);
                // The graphics manager (gp+0x2a90) and its slot 8.
                c.bus.write32(GAME_GP.wrapping_add(0x2a90), DATA + 0x9000);
                c.bus.write32(DATA + 0x9000 + 0x10d8, DATA + 0x9800);
                c.bus.write16(DATA + 0x9800 + 0x40, 0);
                c.bus.write32(DATA + 0x9800 + 0x44, fakes::FAKE);
            },
        );
    }

    #[test]
    fn cfe_progressive_verify_on_widget_action_matches() {
        check(
            ee::s06::f_002461e0__generated,
            cfe_progressive_verify_on_widget_action,
            300,
            &|c, seed| {
                let _ = oc1017_setup(c, seed, 0x0024_61e0);
                // The hash the callee returns is 0 on even seeds: the item's
                // hash then matches and the allocation path runs.
                let zero_hash = seed % 2 == 0;
                oc1017_w(c, OC1017_CTRL + 0x100, zero_hash as u32);
                oc1017_w(c, OC1017_OBJ + 0x38, if zero_hash { 0 } else { 0x1234 });
                oc1017_w(c, OC1017_SELF + 0x10, OC1017_OBJ + 0x800);
                let item = if seed % 4 == 0 { 0 } else { OC1017_OBJ };
                c.g[A0][0] = sx(OC1017_SELF);
                c.g[A1][0] = item as u64;
                c.g[A2][0] = if seed % 7 == 0 { 4 } else { 5 };
            },
        );
    }

    #[test]
    fn ssxapp_global_ctors_matches() {
        check(
            ee::s06::f_00247e20__generated,
            ssxapp_global_ctors,
            100,
            &|c, seed| {
                let _ = oc1017_setup(c, seed, 0x0024_7e20);
            },
        );
    }

    #[test]
    fn ssxapp_global_dtors_matches() {
        check(
            ee::s06::f_00247e40__generated,
            ssxapp_global_dtors,
            100,
            &|c, seed| {
                let _ = oc1017_setup(c, seed, 0x0024_7e40);
            },
        );
    }
}
