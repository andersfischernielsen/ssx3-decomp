//! Staging module oc670: dispatcher-shaped bodies converted from the generated
//! code, each with a differential test. Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc670`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0022d3d8,
        section_man_request_load,
        "SectionMan_RequestLoad",
    ),
    (0x002314d0, game_module_load_state, "GameModule_LoadState"),
    (
        0x0023b0a8,
        gm_handler8_pick_opponents,
        "GMHandler8_PickOpponents",
    ),
    (
        0x002701a0,
        replay_recycle_oldest_snapshot,
        "Replay_RecycleOldestSnapshot",
    ),
    (
        0x0027bdb8,
        script_engine_eval_choice_params,
        "ScriptEngine_EvalChoiceParams",
    ),
    (0x002816a0, script_view_obj_tick, "ScriptViewObj_Tick"),
    (
        0x00282a80,
        script_engine_find_command_handler,
        "ScriptEngine_FindCommandHandler",
    ),
    (
        0x003266e8,
        input_preproc_directive,
        "InputPreproc_Directive",
    ),
    (0x003430d0, conveyor_ctor_2, "Conveyor_ctor_2"),
];

/// `SectionMan_RequestLoad` (0x0022d3d8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn section_man_request_load(c: &mut Cpu) {
    if c.pc != 0x0022d3d8 {
        return ee::s05::f_0022d3d8__generated(c);
    }
    guest(c, |c| {
        'b_22d470: {
            'b_22d46c: {
                c.cycles += 10;
                c.g[V0][0] = 0x440000;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[T0][0] = c.g[V0][0];
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2168));
                c.g[A2][0] = sx((c.g[A1][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                let t = (c.g[V1][0] as i64) < 0;
                c.g[T1][0] = c.g[A0][0];
                if t {
                    break 'b_22d46c;
                } else {
                    'b_22d45c: {
                        c.cycles += 5;
                        c.g[V1][0] = c.g[V0][0];
                        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A2][0] = 0u64;
                        if t {
                            break 'b_22d45c;
                        } else {
                            'b_22d454: {
                                'b_22d444: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x480000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4c60));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x0022d430 => {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                sx((c.g[T0][0] as u32).wrapping_add(0x2168));
                                            c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            let t = true;
                                            c.g[A0][0] = 0x1;
                                            if t {
                                                break 'b_22d454;
                                            } else {
                                                break 'b_22d444;
                                            }
                                        }
                                        0x0022d444 => {
                                            break 'b_22d444;
                                        }
                                        0x0022d45c => {
                                            break 'b_22d45c;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x0022d3d8, 0x0022d478);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x2168));
                                c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[A0][0] = 0x3;
                                break 'b_22d454;
                            }
                            c.cycles += 2;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                            c.g[A2][0] = 0x1;
                            break 'b_22d45c;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[A2][0] == 0u64;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    if t {
                        break 'b_22d470;
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x0022d46c);
                        c.g[A0][0] = c.g[T1][0];
                        Callee {
                            addr: 0x0022e298,
                            f: ee::s05::SectionMan_QueueSection,
                        }
                        .call(c, 0x0022d46c)?;
                        break 'b_22d46c;
                    }
                }
            }
            c.cycles += 1;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            break 'b_22d470;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
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
        return ee::s06::f_002314d0__generated(c);
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

/// `GMHandler8_PickOpponents` (0x0023b0a8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn gm_handler8_pick_opponents(c: &mut Cpu) {
    if c.pc != 0x0023b0a8 {
        return ee::s06::f_0023b0a8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0023b0a8;
        loop {
            match pc {
                0x0023b0a8 => {
                    c.cycles += 4;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[A2][0] = 0x9;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x24));
                    pc = 0x0023b0b8;
                    continue;
                }
                0x0023b0b8 => {
                    c.cycles += 7;
                    c.sw((c.g[V0][0] as u32), c.g[A2][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[A2][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0023b0b8) {
                            return Ok(());
                        }
                        pc = 0x0023b0b8;
                        continue;
                    }
                    pc = 0x0023b0d4;
                    continue;
                }
                0x0023b0d4 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0023b0e0);
                    c.g[A1][0] = 0xa;
                    Callee {
                        addr: 0x0023c770,
                        f: ee::s06::GameRand_Shuffle,
                    }
                    .call(c, 0x0023b0e0)?;
                    c.g[T1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                    c.g[T0][0] = 0x2;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.g[T1][0];
                    c.g[A3][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    if t {
                        pc = 0x0023b12c;
                        continue;
                    }
                    pc = 0x0023b104;
                    continue;
                }
                0x0023b104 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));

                    if c.g[A1][0] == c.g[V0][0] {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        pc = 0x0023b130;
                        continue;
                    }
                    pc = 0x0023b110;
                    continue;
                }
                0x0023b110 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                    pc = 0x0023b118;
                    continue;
                }
                0x0023b118 => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[A1][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x40), 0u64);
                    pc = 0x0023b12c;
                    continue;
                }
                0x0023b12c => {
                    c.cycles += 1;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    pc = 0x0023b130;
                    continue;
                }
                0x0023b130 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 10i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x0023b164;
                        continue;
                    }
                    pc = 0x0023b13c;
                    continue;
                }
                0x0023b13c => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[T1][0];
                    c.g[A1][0] = c.lw((c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    if t {
                        if back_edge_unwinds(c, 0x0023b12c) {
                            return Ok(());
                        }
                        pc = 0x0023b12c;
                        continue;
                    }
                    pc = 0x0023b150;
                    continue;
                }
                0x0023b150 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[T0][0] as i64) < 10i64) as u64;
                    if t {
                        if back_edge_unwinds(c, 0x0023b12c) {
                            return Ok(());
                        }
                        pc = 0x0023b12c;
                        continue;
                    }
                    pc = 0x0023b15c;
                    continue;
                }
                0x0023b15c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                    if t {
                        if back_edge_unwinds(c, 0x0023b118) {
                            return Ok(());
                        }
                        pc = 0x0023b118;
                        continue;
                    }
                    pc = 0x0023b164;
                    continue;
                }
                0x0023b164 => {
                    c.cycles += 3;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0023b0a8, 0x0023b170);
                    return Ok(());
                }
            }
        }
    });
}

/// `Replay_RecycleOldestSnapshot` (0x002701a0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn replay_recycle_oldest_snapshot(c: &mut Cpu) {
    if c.pc != 0x002701a0 {
        return ee::s07::f_002701a0__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002701a0;
        loop {
            match pc {
                0x002701a0 => {
                    c.cycles += 4;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                    c.g[A2][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[A3][0] = 0u64;
                    pc = 0x002701b0;
                    continue;
                }
                0x002701b0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3ac));
                    if t {
                        pc = 0x002701f4;
                        continue;
                    }
                    pc = 0x002701b8;
                    continue;
                }
                0x002701b8 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1f));
                    let t = c.g[V1][0] != 0u64;
                    c.g[A0][0] = c.g[V0][0];
                    if t {
                        pc = 0x002701ec;
                        continue;
                    }
                    pc = 0x002701c8;
                    continue;
                }
                0x002701c8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1e));

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                        pc = 0x002701f0;
                        continue;
                    }
                    pc = 0x002701d4;
                    continue;
                }
                0x002701d4 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = c.g[A2][0];
                    c.g[RA][0] = sx(0x002701e4);
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x3ac), c.g[V0][0]);
                    Callee {
                        addr: 0x00270238,
                        f: ee::s07::Replay_FreeSnapshot,
                    }
                    .call(c, 0x002701e4)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    if t {
                        pc = 0x0027022c;
                        continue;
                    }
                    pc = 0x002701ec;
                    continue;
                }
                0x002701ec => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                    pc = 0x002701f0;
                    continue;
                }
                0x002701f0 => {
                    c.cycles += 1;
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x3ac), c.g[V0][0]);
                    pc = 0x002701f4;
                    continue;
                }
                0x002701f4 => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3b0));
                        pc = 0x00270218;
                        continue;
                    }
                    pc = 0x002701fc;
                    continue;
                }
                0x002701fc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3b0));
                        pc = 0x00270218;
                        continue;
                    }
                    pc = 0x00270208;
                    continue;
                }
                0x00270208 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3ac));
                        if back_edge_unwinds(c, 0x002701b8) {
                            return Ok(());
                        }
                        pc = 0x002701b8;
                        continue;
                    }
                    pc = 0x00270214;
                    continue;
                }
                0x00270214 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3b0));
                    pc = 0x00270218;
                    continue;
                }
                0x00270218 => {
                    c.cycles += 4;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 2i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x3ac), c.g[V1][0]);
                    if t {
                        if back_edge_unwinds(c, 0x002701b0) {
                            return Ok(());
                        }
                        pc = 0x002701b0;
                        continue;
                    }
                    pc = 0x00270228;
                    continue;
                }
                0x00270228 => {
                    c.cycles += 1;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    pc = 0x0027022c;
                    continue;
                }
                0x0027022c => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002701a0, 0x00270234);
                    return Ok(());
                }
            }
        }
    });
}

/// `ScriptEngine_EvalChoiceParams` (0x0027bdb8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn script_engine_eval_choice_params(c: &mut Cpu) {
    if c.pc != 0x0027bdb8 {
        return ee::s07::f_0027bdb8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S2][0] = c.g[A1][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S3][0] = 0u64;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S4][0] = c.g[T0][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S5][0] = c.g[A3][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S6][0] = c.g[A2][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[FP][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
        c.g[S7][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
        'l_27be10: loop {
            'b_27bf10: {
                c.cycles += 5;
                c.g[V1][0] = (c.g[S3][0] < 0x15u64) as u64;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                let t = c.g[V1][0] == 0u64;
                c.sw((c.g[V0][0] as u32), 0u64);
                if t {
                    break 'b_27bf10;
                } else {
                    'b_27beec: {
                        'b_27bee8: {
                            'b_27bed0: {
                                'b_27beac: {
                                    'b_27be8c: {
                                        'b_27be6c: {
                                            'b_27be5c: {
                                                'b_27be4c: {
                                                    c.cycles += 6;
                                                    c.g[V0][0] = 0x480000;
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x1ec0));
                                                    c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                    let t = c.g[V1][0] as u32;
                                                    match t {
                                                        0x0027be3c => {
                                                            c.cycles += 2;
                                                            let t = (c.g[S6][0] as i64) < 0;
                                                            c.g[V0][0] =
                                                                sx((c.g[S6][0] as u32) << 2);
                                                            if t {
                                                                break 'b_27bf10;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[V0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                if t {
                                                                    break 'b_27bee8;
                                                                } else {
                                                                    break 'b_27be4c;
                                                                }
                                                            }
                                                        }
                                                        0x0027be4c => {
                                                            break 'b_27be4c;
                                                        }
                                                        0x0027be5c => {
                                                            break 'b_27be5c;
                                                        }
                                                        0x0027be6c => {
                                                            break 'b_27be6c;
                                                        }
                                                        0x0027be8c => {
                                                            break 'b_27be8c;
                                                        }
                                                        0x0027beac => {
                                                            break 'b_27beac;
                                                        }
                                                        0x0027bed0 => {
                                                            break 'b_27bed0;
                                                        }
                                                        _ => {
                                                            c.jump_out(t, 0x0027bdb8, 0x0027bf50);
                                                            return Ok(());
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = (c.g[S5][0] as i64) < 0;
                                                c.g[V0][0] = sx((c.g[S5][0] as u32) << 2);
                                                if t {
                                                    break 'b_27bf10;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    if t {
                                                        break 'b_27bee8;
                                                    } else {
                                                        break 'b_27be5c;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = (c.g[S4][0] as i64) < 0;
                                            c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                                            if t {
                                                break 'b_27bf10;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                if t {
                                                    break 'b_27bee8;
                                                } else {
                                                    break 'b_27be6c;
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x3));
                                        c.g[V0][0] =
                                            ((c.g[V1][0] as i64) < (c.g[FP][0] as i64)) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                        if t {
                                            break 'b_27bf10;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                                            let t = true;
                                            c.g[A0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                            if t {
                                                break 'b_27beec;
                                            } else {
                                                break 'b_27be8c;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5));
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S7][0] as i64)) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                    if t {
                                        break 'b_27bf10;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                                        let t = true;
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                        if t {
                                            break 'b_27beec;
                                        } else {
                                            break 'b_27beac;
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0xa));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                if t {
                                    break 'b_27bf10;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = true;
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_27beec;
                                    } else {
                                        break 'b_27bed0;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                            c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0xf));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                            if t {
                                break 'b_27bf10;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                break 'b_27bee8;
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                        break 'b_27beec;
                    }
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x0027bef4);
                    call_ptr(c, 0x00123128, 0x0027bef4)?;
                    c.g[A0][0] = 0x480000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1bb0));
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.sw((c.g[A1][0] as u32), c.g[V1][0]);
                    break 'b_27bf10;
                }
            }
            c.cycles += 4;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 21i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                if event_due(c) {
                    deliver_event_here(c, 0x0027be10);
                }
                continue 'l_27be10;
            } else {
                c.cycles += 12;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `ScriptViewObj_Tick` (0x002816a0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn script_view_obj_tick(c: &mut Cpu) {
    if c.pc != 0x002816a0 {
        return ee::s07::f_002816a0__generated(c);
    }
    guest(c, |c| 'l_2816a0: loop {
        'b_28175c: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A1][0];
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S1][0] = c.g[A0][0];
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
            c.mov_s(22, 13);
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.mov_s(21, 12);
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.g[S0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A1][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x002816e0);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            Callee {
                addr: 0x0027c8e0,
                f: ee::s07::ScriptView_ReadFields,
            }
            .call(c, 0x002816e0)?;
            c.g[RA][0] = sx(0x002816e8);
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0027c888,
                f: ee::s07::ScriptView_GetParams,
            }
            .call(c, 0x002816e8)?;
            c.g[V1][0] = c.lb((c.g[V0][0] as u32));
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
            if t {
                break 'b_28175c;
            } else {
                'b_281758: {
                    'b_281750: {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x002816fc);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0xf0), 0u64);
                        Callee {
                            addr: 0x0028b180,
                            f: ee::s07::SSXAudioSystem_GetInstance,
                        }
                        .call(c, 0x002816fc)?;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x530));
                        let t = c.g[V1][0] == 0u64;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                        if t {
                            break 'b_281750;
                        } else {
                            'b_281748: {
                                c.cycles += 8;
                                c.g[RA][0] = sx(0x00281710);
                                call_ptr(c, 0x002b4920, 0x00281710)?;
                                c.mov_s(20, 0);
                                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0xf4));
                                c.c_lt_s(0, 20);
                                let t = !c.fcc();
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_281748;
                                } else {
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xec));
                                    let t = c.g[V1][0] == 0u64;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0xf0), c.g[V0][0]);
                                    if t {
                                        break 'b_281748;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = c.g[SP][0];
                                        c.g[RA][0] = sx(0x00281744);
                                        c.mov_s(12, 21);
                                        Callee {
                                            addr: 0x00281c68,
                                            f: ee::s07::ScriptViewObj_CutToShot,
                                        }
                                        .call(c, 0x00281744)?;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0xec), 0u64);
                                        break 'b_281748;
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0xf4));
                            if t {
                                break 'b_281758;
                            } else {
                                break 'b_281750;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xf0), c.g[V0][0]);
                    break 'b_281758;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                break 'b_28175c;
            }
        }
        'b_2819dc: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_2819dc;
            } else {
                'b_2819d8: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe8));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    if t {
                        break 'b_2819d8;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.g[A0][0] = c.lb((c.g[V0][0] as u32));
                        c.g[V1][0] = (c.g[A0][0] < 0x9u64) as u64;
                        let t = c.g[V1][0] == 0u64;
                        c.g[V0][0] = 0x480000;
                        if t {
                            break 'b_2819d8;
                        } else {
                            'b_2819b8: {
                                'b_2818d4: {
                                    c.cycles += 6;
                                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1f20));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x002816a0 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002816a0);
                                            }
                                            continue 'l_2816a0;
                                        }
                                        0x0028179c => {
                                            'b_2817d4: {
                                                'b_2817c0: {
                                                    c.cycles += 5;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    let t = (c.g[V0][0] as i64) < 0;
                                                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                                                    if t {
                                                        break 'b_2817c0;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.mtc1(2, 0);
                                                        c.cvt_s_w(0, 0);
                                                        let t = true;

                                                        if t {
                                                            break 'b_2817d4;
                                                        } else {
                                                            break 'b_2817c0;
                                                        }
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                                c.mtc1(2, 0);
                                                c.cvt_s_w(0, 0);
                                                c.add_s(0, 0, 0);
                                                break 'b_2817d4;
                                            }
                                            'b_2817e4: {
                                                c.cycles += 4;
                                                c.c_lt_s(0, 21);
                                                if c.fcc() {
                                                    c.sw(
                                                        (c.g[S1][0] as u32).wrapping_add(0xe8),
                                                        0u64,
                                                    );
                                                    break 'b_2817e4;
                                                } else {
                                                    break 'b_2817e4;
                                                }
                                            }
                                            c.cycles += 12;
                                            c.g[A1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[RA][0] = sx(0x002817f0);
                                            c.g[A0][0] = c.g[S1][0];
                                            Callee {
                                                addr: 0x00281c40,
                                                f: ee::s07::ScriptViewObj_GetSlotCam,
                                            }
                                            .call(c, 0x002817f0)?;
                                            c.g[S3][0] = c.g[V0][0];
                                            c.g[A1][0] = 0x1;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[RA][0] = sx(0x00281808);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_sub((c.g[V0][0] as u32)));
                                            Callee {
                                                addr: 0x00281c40,
                                                f: ee::s07::ScriptViewObj_GetSlotCam,
                                            }
                                            .call(c, 0x00281808)?;
                                            c.g[S2][0] = c.g[V0][0];
                                            if c.g[S2][0] == 0u64 {
                                                c.sw((c.g[S1][0] as u32).wrapping_add(0xe8), 0u64);
                                                break 'b_2819d8;
                                            } else {
                                                'b_28185c: {
                                                    'b_281844: {
                                                        c.cycles += 8;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 20);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[V1][0] as u32)));
                                                        let t = (c.g[V0][0] as i64) < 0;
                                                        c.g[A1][0] = c.g[V1][0];
                                                        if t {
                                                            break 'b_281844;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.mtc1(2, 0);
                                                            c.cvt_s_w(0, 0);
                                                            let t = true;

                                                            if t {
                                                                break 'b_28185c;
                                                            } else {
                                                                break 'b_281844;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                                    c.mtc1(2, 0);
                                                    c.cvt_s_w(0, 0);
                                                    c.add_s(0, 0, 0);
                                                    break 'b_28185c;
                                                }
                                                'b_2818ac: {
                                                    'b_281870: {
                                                        c.cycles += 3;
                                                        c.c_lt_s(0, 21);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_281870;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0xe8),
                                                                0u64,
                                                            );
                                                            if t {
                                                                break 'b_2818ac;
                                                            } else {
                                                                break 'b_281870;
                                                            }
                                                        }
                                                    }
                                                    'b_2818a8: {
                                                        'b_281890: {
                                                            c.cycles += 4;
                                                            c.mtc1(5, 0);
                                                            c.cvt_s_w(0, 0);
                                                            let t = (c.g[A0][0] as i64) < 0;
                                                            c.sub_s(12, 21, 0);
                                                            if t {
                                                                break 'b_281890;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.mtc1(4, 0);
                                                                c.cvt_s_w(0, 0);
                                                                let t = true;

                                                                if t {
                                                                    break 'b_2818a8;
                                                                } else {
                                                                    break 'b_281890;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 6;
                                                        c.g[V1][0] = sx((c.g[A0][0] as u32) >> 1);
                                                        c.g[V0][0] = c.g[A0][0] & 0x1;
                                                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                                        c.mtc1(2, 0);
                                                        c.cvt_s_w(0, 0);
                                                        c.add_s(0, 0, 0);
                                                        break 'b_2818a8;
                                                    }
                                                    c.cycles += 1;
                                                    c.div_s(20, 12, 0);
                                                    break 'b_2818ac;
                                                }
                                                c.cycles += 10;
                                                c.mov_s(12, 20);
                                                c.g[RA][0] = sx(0x002818b8);
                                                c.g[A0][0] = c.g[S3][0];
                                                Callee {
                                                    addr: 0x0027d788,
                                                    f: ee::s07::ScriptObjCam_SetWeight,
                                                }
                                                .call(c, 0x002818b8)?;
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 12);
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[RA][0] = sx(0x002818cc);
                                                c.sub_s(12, 12, 20);
                                                Callee {
                                                    addr: 0x0027d788,
                                                    f: ee::s07::ScriptObjCam_SetWeight,
                                                }
                                                .call(c, 0x002818cc)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                if t {
                                                    break 'b_2819dc;
                                                } else {
                                                    break 'b_2818d4;
                                                }
                                            }
                                        }
                                        0x002818d4 => {
                                            break 'b_2818d4;
                                        }
                                        0x002819b8 => {
                                            break 'b_2819b8;
                                        }
                                        0x002819d8 => {
                                            break 'b_2819d8;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x002816a0, 0x00281a00);
                                            return Ok(());
                                        }
                                    }
                                }
                                'b_281918: {
                                    c.cycles += 17;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[RA][0] = sx(0x002818e0);
                                    c.g[A0][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x00281c40,
                                        f: ee::s07::ScriptViewObj_GetSlotCam,
                                    }
                                    .call(c, 0x002818e0)?;
                                    c.g[S3][0] = c.g[V0][0];
                                    c.g[A1][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x002818f8);
                                    c.g[A1][0] =
                                        sx((c.g[A1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                                    Callee {
                                        addr: 0x00281c40,
                                        f: ee::s07::ScriptViewObj_GetSlotCam,
                                    }
                                    .call(c, 0x002818f8)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd8));
                                    c.g[S2][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x00281908);
                                    c.mov_s(12, 22);
                                    Callee {
                                        addr: 0x00282d30,
                                        f: ee::s07::cTransitionMan_ScriptAdvanceTimer,
                                    }
                                    .call(c, 0x00281908)?;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd8));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                    if (c.g[V1][0] as i64) <= 0 {
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0xe8), 0u64);
                                        break 'b_281918;
                                    } else {
                                        break 'b_281918;
                                    }
                                }
                                'b_2819a8: {
                                    'b_2819a0: {
                                        'b_28198c: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xe8));
                                            let t = c.g[V0][0] == 0u64;

                                            if t {
                                                break 'b_28198c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S2][0] == 0u64;

                                                if t {
                                                    break 'b_2819a0;
                                                } else {
                                                    'b_281964: {
                                                        'b_281950: {
                                                            c.cycles += 5;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[V1][0] as u32)));
                                                            let t = (c.g[V0][0] as i64) < 0;
                                                            c.g[V1][0] =
                                                                sx((c.g[V0][0] as u32) >> 1);
                                                            if t {
                                                                break 'b_281950;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.mtc1(2, 0);
                                                                c.cvt_s_w(0, 0);
                                                                let t = true;

                                                                if t {
                                                                    break 'b_281964;
                                                                } else {
                                                                    break 'b_281950;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 5;
                                                        c.g[V0][0] = c.g[V0][0] & 0x1;
                                                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                                        c.mtc1(2, 0);
                                                        c.cvt_s_w(0, 0);
                                                        c.add_s(0, 0, 0);
                                                        break 'b_281964;
                                                    }
                                                    c.cycles += 3;
                                                    c.c_lt_s(21, 0);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_28198c;
                                                    } else {
                                                        c.cycles += 7;
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 12);
                                                        c.g[RA][0] = sx(0x00281980);
                                                        c.g[A0][0] = c.g[S2][0];
                                                        Callee {
                                                            addr: 0x0027d788,
                                                            f: ee::s07::ScriptObjCam_SetWeight,
                                                        }
                                                        .call(c, 0x00281980)?;
                                                        c.mtc1(0, 12);
                                                        let t = true;

                                                        if t {
                                                            break 'b_2819a8;
                                                        } else {
                                                            break 'b_28198c;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[S2][0] == 0u64;

                                        if t {
                                            break 'b_2819a0;
                                        } else {
                                            c.cycles += 3;
                                            c.mtc1(0, 12);
                                            c.g[RA][0] = sx(0x002819a0);
                                            c.g[A0][0] = c.g[S2][0];
                                            Callee {
                                                addr: 0x0027d788,
                                                f: ee::s07::ScriptObjCam_SetWeight,
                                            }
                                            .call(c, 0x002819a0)?;
                                            break 'b_2819a0;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 12);
                                    break 'b_2819a8;
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x002819b0);
                                c.g[A0][0] = c.g[S3][0];
                                Callee {
                                    addr: 0x0027d788,
                                    f: ee::s07::ScriptObjCam_SetWeight,
                                }
                                .call(c, 0x002819b0)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                if t {
                                    break 'b_2819dc;
                                } else {
                                    break 'b_2819b8;
                                }
                            }
                            c.cycles += 7;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd8));
                            c.g[RA][0] = sx(0x002819c4);
                            c.mov_s(12, 22);
                            Callee {
                                addr: 0x00282d30,
                                f: ee::s07::cTransitionMan_ScriptAdvanceTimer,
                            }
                            .call(c, 0x002819c4)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            let t = (c.g[V1][0] as i64) > 0;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                            if t {
                                break 'b_2819dc;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0xe8), 0u64);
                                break 'b_2819d8;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_2819dc;
            }
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `ScriptEngine_FindCommandHandler` (0x00282a80): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn script_engine_find_command_handler(c: &mut Cpu) {
    if c.pc != 0x00282a80 {
        return ee::s07::f_00282a80__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00282a80;
        loop {
            match pc {
                0x00282a80 => {
                    c.cycles += 21;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
                    c.g[A3][0] = 0x1e;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[S2][0] = 0u64;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[S3][0] = c.g[A1][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S4][0] = 0u64;
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[S6][0] = c.g[A2][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[RA][0]);
                    c.g[A2][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[RA][0] = sx(0x00282ac8);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                    Callee {
                        addr: 0x0027d578,
                        f: ee::s07::ScriptEngine_CollectFreeBindings,
                    }
                    .call(c, 0x00282ac8)?;
                    c.g[S5][0] = c.g[V0][0];
                    let t = true;
                    c.g[S1][0] = c.g[SP][0];
                    if t {
                        pc = 0x00282adc;
                        continue;
                    }
                    pc = 0x00282ad4;
                    continue;
                }
                0x00282ad4 => {
                    c.cycles += 1;
                    pc = 0x00282ad8;
                    continue;
                }
                0x00282ad8 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x00282adc;
                    continue;
                }
                0x00282adc => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S3][0];
                    if t {
                        pc = 0x00282b10;
                        continue;
                    }
                    pc = 0x00282ae8;
                    continue;
                }
                0x00282ae8 => {
                    c.cycles += 9;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[A2][0] = c.g[S6][0];
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00282b04);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00282b04)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x00282ad8) {
                            return Ok(());
                        }
                        pc = 0x00282ad8;
                        continue;
                    }
                    pc = 0x00282b0c;
                    continue;
                }
                0x00282b0c => {
                    c.cycles += 1;
                    c.g[S4][0] = c.g[S0][0];
                    pc = 0x00282b10;
                    continue;
                }
                0x00282b10 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.g[S4][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00282a80, 0x00282b3c);
                    return Ok(());
                }
            }
        }
    });
}

/// `InputPreproc_Directive` (0x003266e8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn input_preproc_directive(c: &mut Cpu) {
    if c.pc != 0x003266e8 {
        return ee::s10::f_003266e8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x003266e8;
        loop {
            match pc {
                0x003266e8 => {
                    c.cycles += 19;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1f0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                    c.g[S2][0] = c.g[A1][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                    c.g[S3][0] = 0xd;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x190));
                    c.g[S4][0] = c.g[A2][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x180));
                    c.g[S5][0] = c.g[A3][0];
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x170));
                    c.g[S6][0] = 0x9;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                    c.g[S7][0] = 0x20;
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x160));
                    let t = true;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x150), c.g[RA][0]);
                    if t {
                        pc = 0x00326740;
                        continue;
                    }
                    pc = 0x00326734;
                    continue;
                }
                0x00326734 => {
                    c.cycles += 1;
                    pc = 0x00326738;
                    continue;
                }
                0x00326738 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00326740);
                    c.g[A1][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00326478,
                        f: ee::s10::InputPreproc_NextChar,
                    }
                    .call(c, 0x00326740)?;
                    pc = 0x00326740;
                    continue;
                }
                0x00326740 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    let t = c.g[S0][0] == c.g[S7][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0032675c;
                        continue;
                    }
                    pc = 0x0032674c;
                    continue;
                }
                0x0032674c => {
                    c.cycles += 2;

                    if c.g[S0][0] == c.g[S6][0] {
                        c.g[V0][0] = 0x1;
                        pc = 0x00326760;
                        continue;
                    }
                    pc = 0x00326754;
                    continue;
                }
                0x00326754 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[S3][0];

                    if t {
                        pc = 0x00326760;
                        continue;
                    }
                    pc = 0x0032675c;
                    continue;
                }
                0x0032675c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00326760;
                    continue;
                }
                0x00326760 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        if back_edge_unwinds(c, 0x00326738) {
                            return Ok(());
                        }
                        pc = 0x00326738;
                        continue;
                    }
                    pc = 0x00326768;
                    continue;
                }
                0x00326768 => {
                    c.cycles += 2;
                    let t = (c.g[S0][0] as i64) > 0;
                    c.g[S0][0] = c.g[SP][0];
                    if t {
                        pc = 0x00326794;
                        continue;
                    }
                    pc = 0x00326770;
                    continue;
                }
                0x00326770 => {
                    c.cycles += 9;
                    c.g[T0][0] = 0x490000;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x0032678c);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1c08));
                    Callee {
                        addr: 0x00326a20,
                        f: ee::s10::InputPreproc_EmptyStub,
                    }
                    .call(c, 0x0032678c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                    if t {
                        pc = 0x003269f4;
                        continue;
                    }
                    pc = 0x00326794;
                    continue;
                }
                0x00326794 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    pc = 0x00326798;
                    continue;
                }
                0x00326798 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x61));
                    c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = 0u64;
                    if t {
                        pc = 0x003267b8;
                        continue;
                    }
                    pc = 0x003267a8;
                    continue;
                }
                0x003267a8 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x41));
                    c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x003267bc;
                        continue;
                    }
                    pc = 0x003267b8;
                    continue;
                }
                0x003267b8 => {
                    c.cycles += 1;
                    c.g[A0][0] = 0x1;
                    pc = 0x003267bc;
                    continue;
                }
                0x003267bc => {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        pc = 0x003267e8;
                        continue;
                    }
                    pc = 0x003267c4;
                    continue;
                }
                0x003267c4 => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003267d4);
                    c.sb((c.g[S0][0] as u32), c.g[V0][0]);
                    Callee {
                        addr: 0x00326478,
                        f: ee::s10::InputPreproc_NextChar,
                    }
                    .call(c, 0x003267d4)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[SP][0] as u32)));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        if back_edge_unwinds(c, 0x00326798) {
                            return Ok(());
                        }
                        pc = 0x00326798;
                        continue;
                    }
                    pc = 0x003267e8;
                    continue;
                }
                0x003267e8 => {
                    c.cycles += 7;
                    c.g[A1][0] = 0x4a0000;
                    c.sb((c.g[S0][0] as u32), 0u64);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3fa8));
                    c.g[RA][0] = sx(0x003267fc);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x004165a8, 0x003267fc)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[T0][0] = 0x490000;
                    if t {
                        pc = 0x003269cc;
                        continue;
                    }
                    pc = 0x00326804;
                    continue;
                }
                0x00326804 => {
                    c.cycles += 4;
                    c.g[S7][0] = 0x20;
                    c.g[S6][0] = 0x9;
                    let t = true;
                    c.g[S3][0] = 0xd;
                    if t {
                        pc = 0x00326820;
                        continue;
                    }
                    pc = 0x00326814;
                    continue;
                }
                0x00326814 => {
                    c.cycles += 1;
                    pc = 0x00326818;
                    continue;
                }
                0x00326818 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00326820);
                    c.g[A1][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00326478,
                        f: ee::s10::InputPreproc_NextChar,
                    }
                    .call(c, 0x00326820)?;
                    pc = 0x00326820;
                    continue;
                }
                0x00326820 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    let t = c.g[S0][0] == c.g[S7][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0032683c;
                        continue;
                    }
                    pc = 0x0032682c;
                    continue;
                }
                0x0032682c => {
                    c.cycles += 2;

                    if c.g[S0][0] == c.g[S6][0] {
                        c.g[V0][0] = 0x1;
                        pc = 0x00326840;
                        continue;
                    }
                    pc = 0x00326834;
                    continue;
                }
                0x00326834 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[S3][0];

                    if t {
                        pc = 0x00326840;
                        continue;
                    }
                    pc = 0x0032683c;
                    continue;
                }
                0x0032683c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00326840;
                    continue;
                }
                0x00326840 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        if back_edge_unwinds(c, 0x00326818) {
                            return Ok(());
                        }
                        pc = 0x00326818;
                        continue;
                    }
                    pc = 0x00326848;
                    continue;
                }
                0x00326848 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x22;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[T0][0] = 0x490000;
                    if t {
                        pc = 0x0032693c;
                        continue;
                    }
                    pc = 0x00326854;
                    continue;
                }
                0x00326854 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00326860);
                    c.g[A1][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00326478,
                        f: ee::s10::InputPreproc_NextChar,
                    }
                    .call(c, 0x00326860)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    let t = c.g[V0][0] == c.g[S0][0];
                    c.g[S3][0] = 0u64;
                    if t {
                        pc = 0x003268d0;
                        continue;
                    }
                    pc = 0x0032686c;
                    continue;
                }
                0x0032686c => {
                    c.cycles += 5;
                    c.g[S7][0] = 0xa;
                    c.g[FP][0] = 0x490000;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S6][0] = 0x22;
                    pc = 0x00326880;
                    continue;
                }
                0x00326880 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        pc = 0x00326898;
                        continue;
                    }
                    pc = 0x0032688c;
                    continue;
                }
                0x0032688c => {
                    c.cycles += 2;

                    if c.g[V0][0] != c.g[S7][0] {
                        c.g[V1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0xc));
                        pc = 0x003268ac;
                        continue;
                    }
                    pc = 0x00326894;
                    continue;
                }
                0x00326894 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S1][0];
                    pc = 0x00326898;
                    continue;
                }
                0x00326898 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S5][0];
                    let t = true;
                    c.g[T0][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x1be0));
                    if t {
                        pc = 0x0032694c;
                        continue;
                    }
                    pc = 0x003268ac;
                    continue;
                }
                0x003268ac => {
                    c.cycles += 9;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.sb((c.g[V0][0] as u32), c.g[V1][0]);
                    c.g[RA][0] = sx(0x003268c4);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    Callee {
                        addr: 0x00326478,
                        f: ee::s10::InputPreproc_NextChar,
                    }
                    .call(c, 0x003268c4)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    let t = c.g[V0][0] != c.g[S6][0];

                    if t {
                        if back_edge_unwinds(c, 0x00326880) {
                            return Ok(());
                        }
                        pc = 0x00326880;
                        continue;
                    }
                    pc = 0x003268d0;
                    continue;
                }
                0x003268d0 => {
                    c.cycles += 8;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003268e8);
                    c.sb((c.g[V0][0] as u32), 0u64);
                    Callee {
                        addr: 0x00326478,
                        f: ee::s10::InputPreproc_NextChar,
                    }
                    .call(c, 0x003268e8)?;
                    let t = true;
                    c.g[S3][0] = c.g[S0][0];
                    if t {
                        pc = 0x003268f8;
                        continue;
                    }
                    pc = 0x003268f0;
                    continue;
                }
                0x003268f0 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003268f8);
                    c.g[A1][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00326478,
                        f: ee::s10::InputPreproc_NextChar,
                    }
                    .call(c, 0x003268f8)?;
                    pc = 0x003268f8;
                    continue;
                }
                0x003268f8 => {
                    c.cycles += 4;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = 0x20;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[V1][0] = 0u64;
                    if t {
                        pc = 0x0032691c;
                        continue;
                    }
                    pc = 0x00326908;
                    continue;
                }
                0x00326908 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x9;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = 0xd;
                    if t {
                        pc = 0x0032691c;
                        continue;
                    }
                    pc = 0x00326914;
                    continue;
                }
                0x00326914 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[V0][0];

                    if t {
                        pc = 0x00326920;
                        continue;
                    }
                    pc = 0x0032691c;
                    continue;
                }
                0x0032691c => {
                    c.cycles += 1;
                    c.g[V1][0] = 0x1;
                    pc = 0x00326920;
                    continue;
                }
                0x00326920 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        if back_edge_unwinds(c, 0x003268f0) {
                            return Ok(());
                        }
                        pc = 0x003268f0;
                        continue;
                    }
                    pc = 0x00326928;
                    continue;
                }
                0x00326928 => {
                    c.cycles += 2;
                    let t = (c.g[S0][0] as i64) < 0;
                    c.g[V0][0] = 0xa;
                    if t {
                        pc = 0x00326968;
                        continue;
                    }
                    pc = 0x00326930;
                    continue;
                }
                0x00326930 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[T0][0] = 0x490000;
                    if t {
                        pc = 0x00326970;
                        continue;
                    }
                    pc = 0x00326938;
                    continue;
                }
                0x00326938 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S1][0];
                    pc = 0x0032693c;
                    continue;
                }
                0x0032693c => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1be0));
                    pc = 0x0032694c;
                    continue;
                }
                0x0032694c => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00326954);
                    Callee {
                        addr: 0x00326a20,
                        f: ee::s10::InputPreproc_EmptyStub,
                    }
                    .call(c, 0x00326954)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00326960);
                    c.g[A1][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00326678,
                        f: ee::s10::InputPreproc_SkipLine,
                    }
                    .call(c, 0x00326960)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                    if t {
                        pc = 0x003269f4;
                        continue;
                    }
                    pc = 0x00326968;
                    continue;
                }
                0x00326968 => {
                    c.cycles += 2;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A1][0] = c.g[S4][0];
                        pc = 0x00326980;
                        continue;
                    }
                    pc = 0x00326970;
                    continue;
                }
                0x00326970 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.g[A1][0] = c.g[S4][0];
                    pc = 0x00326980;
                    continue;
                }
                0x00326980 => {
                    c.cycles += 19;
                    c.g[A2][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0032698c);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00326078,
                        f: ee::s10::InputPreproc_ProcessFile,
                    }
                    .call(c, 0x0032698c)?;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3fa0));
                    c.g[A2][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x003269a8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x003269a8)?;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003269b8);
                    c.g[A2][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00326430,
                        f: ee::s10::InputPreproc_EmitLineMarker,
                    }
                    .call(c, 0x003269b8)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003269c4);
                    c.g[A1][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00326478,
                        f: ee::s10::InputPreproc_NextChar,
                    }
                    .call(c, 0x003269c4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                    if t {
                        pc = 0x003269f4;
                        continue;
                    }
                    pc = 0x003269cc;
                    continue;
                }
                0x003269cc => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x003269e4);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1bc0));
                    Callee {
                        addr: 0x00326a20,
                        f: ee::s10::InputPreproc_EmptyStub,
                    }
                    .call(c, 0x003269e4)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003269f0);
                    c.g[A1][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00326678,
                        f: ee::s10::InputPreproc_SkipLine,
                    }
                    .call(c, 0x003269f0)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                    pc = 0x003269f4;
                    continue;
                }
                0x003269f4 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x190));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x180));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x170));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x150));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003266e8, 0x00326a20);
                    return Ok(());
                }
            }
        }
    });
}

/// `Conveyor_ctor_2` (0x003430d0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn conveyor_ctor_2(c: &mut Cpu) {
    if c.pc != 0x003430d0 {
        return ee::s11::f_003430d0__generated(c);
    }
    guest(c, |c| {
        c.cycles += 23;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x003430ec);
        c.g[S1][0] = c.g[A2][0];
        Callee {
            addr: 0x0034fb90,
            f: ee::s11::WorldObj_ctor_2,
        }
        .call(c, 0x003430ec)?;
        c.g[V1][0] = 0x490000;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x570));
        c.g[A2][0] = 0x2c;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00343114);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00343114)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    #[test]
    fn section_man_request_load_matches() {
        check(
            ee::s05::f_0022d3d8__generated,
            section_man_request_load,
            200,
            &|c, seed| {
                machine(c, seed, 0xd3d8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0022d3d8;
            },
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

    #[test]
    fn gm_handler8_pick_opponents_matches() {
        check(
            ee::s06::f_0023b0a8__generated,
            gm_handler8_pick_opponents,
            200,
            &|c, seed| {
                machine(c, seed, 0xb0a8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0023b0a8;
            },
        );
    }

    #[test]
    fn replay_recycle_oldest_snapshot_matches() {
        check(
            ee::s07::f_002701a0__generated,
            replay_recycle_oldest_snapshot,
            200,
            &|c, seed| {
                machine(c, seed, 0x1a0, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x002701a0;
            },
        );
    }

    #[test]
    fn script_engine_eval_choice_params_matches() {
        check(
            ee::s07::f_0027bdb8__generated,
            script_engine_eval_choice_params,
            200,
            &|c, seed| {
                machine(c, seed, 0xbdb8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0027bdb8;
            },
        );
    }

    #[test]
    fn script_view_obj_tick_matches() {
        check(
            ee::s07::f_002816a0__generated,
            script_view_obj_tick,
            200,
            &|c, seed| {
                machine(c, seed, 0x16a0, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x002816a0;
            },
        );
    }

    #[test]
    fn script_engine_find_command_handler_matches() {
        check(
            ee::s07::f_00282a80__generated,
            script_engine_find_command_handler,
            200,
            &|c, seed| {
                machine(c, seed, 0x2a80, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00282a80;
            },
        );
    }

    #[test]
    fn input_preproc_directive_matches() {
        check(
            ee::s10::f_003266e8__generated,
            input_preproc_directive,
            200,
            &|c, seed| {
                machine(c, seed, 0x66e8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x003266e8;
            },
        );
    }

    #[test]
    fn conveyor_ctor_2_matches() {
        check(
            ee::s11::f_003430d0__generated,
            conveyor_ctor_2,
            200,
            &|c, seed| {
                machine(c, seed, 0x30d0, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x003430d0;
            },
        );
    }
}
