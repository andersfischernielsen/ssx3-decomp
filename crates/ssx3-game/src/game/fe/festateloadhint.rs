//! `src/fe/festateloadhint.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Flow;
use crate::call::Frame;
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
}
