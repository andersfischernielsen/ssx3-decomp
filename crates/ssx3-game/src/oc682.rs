//! Staging module oc682 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc682`.

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
        0x00236250,
        game_state_event_select_enter,
        "GameStateEventSelect_Enter",
    ),
    (
        0x00236418,
        game_state_event_select_update,
        "GameStateEventSelect_Update",
    ),
    (0x002370a0, input_sync_dtor, "InputSync_dtor"),
    (0x00237a98, input_sync_send_sample, "InputSync_SendSample"),
    (
        0x00239938,
        gm_handler0_pick_opponents,
        "GMHandler0_PickOpponents",
    ),
    (
        0x0023a760,
        gm_handler1_on_rider_finished,
        "GMHandler1_OnRiderFinished",
    ),
    (
        0x00244b48,
        game_module_update_pause_and_ui,
        "GameModule_UpdatePauseAndUI",
    ),
    (0x00244e60, game_module_sim_step, "GameModule_SimStep"),
    (
        0x00245730,
        c_game_load_state_conquer_init,
        "cGameLoadStateConquer_Init",
    ),
];

/// `void GameStateEventSelect_Enter(GameStateEventSelect *self, int mode)` (0x236250, `cGFUserSelectState::gainFocus`): enter the event-select state. Fails the active local mission, sets the AI state to 3 and clears the first rider's boost bar, resets the state fields (`+0x18` next location = 0x17, `+0x14` mode, `+0x8`/`+0x10`/`+0x1c` flags; mode 1 marks the overlay done and takes the next location from the new-race interface), restarts the race when the next location equals the current one, then queues the NIS list for the mode (0: two categories 22/23; 1..3: category 11 unless mode 1, ready, `cRider_initAlways`, play).
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_state_event_select_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_2362ec: {
            c.cycles += 32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x00236274);
            c.g[S1][0] = c.g[A1][0];
            call_ptr(c, 0x0030b7f8, 0x00236274)?;
            c.g[RA][0] = sx(0x0023627c);
            call_ptr(c, 0x0014dc80, 0x0023627c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00236288);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00236288)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = 0x3;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[RA][0] = sx(0x0023629c);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x00113b10, 0x0023629c)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A1][0] = 0x17;
            c.g[A2][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x2f8), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
            let t = c.g[S1][0] != c.g[A2][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0u64);
            if t {
                break 'b_2362ec;
            } else {
                c.cycles += 7;
                c.g[RA][0] = sx(0x002362d8);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                call_ptr(c, 0x0014dc80, 0x002362d8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002362e4);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x002362e4)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f34));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                break 'b_2362ec;
            }
        }
        'b_236314: {
            c.cycles += 6;
            c.g[RA][0] = sx(0x002362f4);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x00144bc0, 0x002362f4)?;
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            if c.g[V0][0] != c.g[V1][0] {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                break 'b_236314;
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[RA][0] = sx(0x00236310);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                Callee {
                    addr: 0x00230180,
                    f: ee::s06::GameModule_ResetRace,
                }
                .call(c, 0x00236310)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                break 'b_236314;
            }
        }
        'b_236404: {
            'b_236388: {
                c.cycles += 2;
                let t = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                if t {
                    break 'b_236388;
                } else {
                    c.cycles += 27;
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = 0x16;
                    c.g[A3][0] = 0x3;
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = 0xffffffffffffffff;
                    c.g[T2][0] = 0xffffffffffffffff;
                    c.g[RA][0] = sx(0x0023633c);
                    c.g[T3][0] = 0xffffffffffffffff;
                    call_ptr(c, 0x00278e50, 0x0023633c)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[A2][0] = 0x17;
                    c.g[A3][0] = 0x1;
                    c.g[A1][0] = 0x1;
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = 0xffffffffffffffff;
                    c.g[T2][0] = 0xffffffffffffffff;
                    c.g[RA][0] = sx(0x00236360);
                    c.g[T3][0] = 0xffffffffffffffff;
                    call_ptr(c, 0x00278e50, 0x00236360)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[RA][0] = sx(0x0023636c);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x00278f38, 0x0023636c)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = 0x1;
                    c.g[RA][0] = sx(0x00236380);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00278f68, 0x00236380)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_236404;
                    } else {
                        break 'b_236388;
                    }
                }
            }
            'b_236400: {
                c.cycles += 2;
                let t = (c.g[V1][0] as i64) < 0;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
                if t {
                    break 'b_236400;
                } else {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[V1][0] ^ 0x1;
                    if t {
                        break 'b_236400;
                    } else {
                        'b_2363c0: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_2363c0;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                c.g[A2][0] = 0xb;
                                c.g[A3][0] = 0x3;
                                c.g[T0][0] = 0u64;
                                c.g[T1][0] = 0xffffffffffffffff;
                                c.g[T2][0] = 0xffffffffffffffff;
                                c.g[RA][0] = sx(0x002363c0);
                                c.g[T3][0] = 0xffffffffffffffff;
                                call_ptr(c, 0x00278e50, 0x002363c0)?;
                                break 'b_2363c0;
                            }
                        }
                        c.cycles += 16;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        c.g[RA][0] = sx(0x002363cc);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x00278f38, 0x002363cc)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x002363e4);
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0011d390, 0x002363e4)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        c.g[A3][0] = 0u64;
                        c.g[A2][0] = c.g[A2][0] ^ 0x2;
                        c.g[RA][0] = sx(0x00236400);
                        c.g[A2][0] = (c.g[A2][0] < 0x1u64) as u64;
                        call_ptr(c, 0x00278f68, 0x00236400)?;
                        break 'b_236400;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_236404;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameStateEventSelect_Update(GameStateEventSelect *self)` (0x236418, `cGFUserSelectState::update`): the event-select state machine over `self->+0x10`. State 0 waits for the NIS list, triggers the overlay (0x1f, or 33 with the map style) and pushes the pause state; state 1 waits for the overlay, then moves on to state 2 (mode 0) or 3 (location change / request the module state); state 2 handles the NIS transitions and the location change; state 3 goes to the next game state.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_state_event_select_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_236714: {
            'b_236710: {
                'b_23670c: {
                    'b_236658: {
                        'b_236540: {
                            c.cycles += 10;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[S0][0] = c.g[A0][0];
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[S2][0] = 0x1;
                            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            let t = c.g[V1][0] == c.g[S2][0];
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_236540;
                            } else {
                                'b_236468: {
                                    'b_236458: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] = 0x2;
                                            break 'b_236458;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                            if t {
                                                break 'b_236468;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_236714;
                                                } else {
                                                    break 'b_236458;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    if c.g[V1][0] == c.g[V0][0] {
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        break 'b_236658;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_236714;
                                        } else {
                                            break 'b_236468;
                                        }
                                    }
                                }
                                'b_236490: {
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x00236470);
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x00279220, 0x00236470)?;
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4));
                                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        break 'b_236490;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V1][0] != 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            break 'b_236714;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            break 'b_236490;
                                        }
                                    }
                                }
                                'b_2364fc: {
                                    'b_2364a0: {
                                        c.cycles += 2;
                                        if c.g[V1][0] != 0u64 {
                                            c.g[V0][0] = 0x2;
                                            break 'b_2364a0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] = 0x1f;
                                            if t {
                                                break 'b_2364fc;
                                            } else {
                                                break 'b_2364a0;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_23670c;
                                    } else {
                                        'b_2364e8: {
                                            c.cycles += 12;
                                            c.g[RA][0] = sx(0x002364b0);
                                            c.g[S1][0] = 0u64;
                                            call_ptr(c, 0x0014dc80, 0x002364b0)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x002364bc);
                                            c.g[A1][0] = 0u64;
                                            call_ptr(c, 0x0014dd58, 0x002364bc)?;
                                            c.g[RA][0] = sx(0x002364c4);
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x00144bc0, 0x002364c4)?;
                                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[V1][0] = ((c.g[V1][0] as i64) < 17i64) as u64;
                                            c.g[V1][0] = c.g[V1][0] ^ 0x1;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[V1][0] = 0x530000;
                                            if t {
                                                break 'b_2364e8;
                                            } else {
                                                c.cycles += 4;
                                                c.g[S1][0] = c.g[S2][0];
                                                c.g[V0][0] =
                                                    c.lb((c.g[V1][0] as u32).wrapping_add(0x5c10));
                                                c.g[V0][0] = c.g[V0][0] ^ 0x4;
                                                if c.g[V0][0] != 0 {
                                                    c.g[S1][0] = 0u64;
                                                }
                                                break 'b_2364e8;
                                            }
                                        }
                                        'b_2364f4: {
                                            c.cycles += 2;
                                            let t = c.g[S1][0] == 0u64;
                                            c.g[V0][0] = 0x4;
                                            if t {
                                                break 'b_2364f4;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] = 0x3;
                                                break 'b_2364f4;
                                            }
                                        }
                                        c.cycles += 2;
                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0xb5c), c.g[V0][0]);
                                        c.g[A0][0] = 0x21;
                                        break 'b_2364fc;
                                    }
                                }
                                c.cycles += 17;
                                c.g[RA][0] = sx(0x00236504);
                                call_ptr(c, 0x0020ab50, 0x00236504)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                c.g[A1][0] = 0x440000;
                                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28f8));
                                c.g[V1][0] = 0x530000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
                                c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[A2][0]);
                                c.g[A1][0] = 0x2;
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A0][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                                let t = true;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_23670c;
                                } else {
                                    break 'b_236540;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        if c.g[V0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_236714;
                        } else {
                            'b_23659c: {
                                c.cycles += 7;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                c.g[RA][0] = sx(0x00236558);
                                c.g[A1][0] = 0x1e;
                                call_ptr(c, 0x00278de8, 0x00236558)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                if t {
                                    break 'b_23659c;
                                } else {
                                    c.cycles += 13;
                                    c.g[A0][0] = 0x530000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x66e8));
                                    c.g[A1][0] = 0x440000;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28f0));
                                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[V0][0]);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
                                    break 'b_23659c;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x2;
                            if t {
                                break 'b_23670c;
                            } else {
                                'b_236638: {
                                    'b_23662c: {
                                        'b_236624: {
                                            c.cycles += 14;
                                            c.g[RA][0] = sx(0x002365b0);
                                            call_ptr(c, 0x0014dc80, 0x002365b0)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x002365bc);
                                            c.g[A1][0] = 0u64;
                                            call_ptr(c, 0x0014dd58, 0x002365bc)?;
                                            c.g[RA][0] = sx(0x002365c4);
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x00144bc0, 0x002365c4)?;
                                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                            c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xe));
                                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                                            c.g[A2][0] = (c.g[A2][0] < 0x3u64) as u64;
                                            let t = c.g[V0][0] != c.g[V1][0];
                                            c.g[A0][0] = (c.g[A0][0] < 0x3u64) as u64;
                                            if t {
                                                break 'b_236624;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A0][0] != 0u64;
                                                c.g[V0][0] = 0x530000;
                                                if t {
                                                    break 'b_23662c;
                                                } else {
                                                    c.cycles += 15;
                                                    c.g[V0][0] = 0x3;
                                                    c.g[RA][0] = sx(0x002365f4);
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x10),
                                                        c.g[V0][0],
                                                    );
                                                    call_ptr(c, 0x0014dc80, 0x002365f4)?;
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x00236600);
                                                    c.g[A1][0] = 0xa;
                                                    call_ptr(c, 0x0014dd58, 0x00236600)?;
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x10),
                                                        c.g[S2][0],
                                                    );
                                                    c.g[A1][0] = 0xf;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[A2][0] = 0u64;
                                                    c.g[A3][0] = 0u64;
                                                    c.g[RA][0] = sx(0x0023661c);
                                                    c.g[A0][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                                    Callee {
                                                        addr: 0x00231250,
                                                        f: ee::s06::GameModule_RequestState,
                                                    }
                                                    .call(c, 0x0023661c)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_236714;
                                                    } else {
                                                        break 'b_236624;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[A0][0] == 0u64;
                                        c.g[V0][0] = 0x530000;
                                        if t {
                                            break 'b_236638;
                                        } else {
                                            break 'b_23662c;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                                    if c.g[V1][0] == 0u64 {
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                                        break 'b_236638;
                                    } else {
                                        break 'b_236638;
                                    }
                                }
                                'b_236644: {
                                    c.cycles += 2;
                                    let t = c.g[A0][0] != 0u64;
                                    c.g[A1][0] = 0u64;
                                    if t {
                                        break 'b_236644;
                                    } else {
                                        c.cycles += 1;
                                        c.g[A1][0] = (c.g[A2][0] < 0x1u64) as u64;
                                        break 'b_236644;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                c.g[RA][0] = sx(0x00236650);
                                c.g[A2][0] = 0x1;
                                call_ptr(c, 0x0027a860, 0x00236650)?;
                                let t = true;
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_23670c;
                                } else {
                                    break 'b_236658;
                                }
                            }
                        }
                    }
                    'b_236700: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        if t {
                            break 'b_236700;
                        } else {
                            'b_2366d4: {
                                'b_236684: {
                                    c.cycles += 7;
                                    c.g[A1][0] = 0x1;
                                    c.g[RA][0] = sx(0x0023666c);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x00279148, 0x0023666c)?;
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[V0][0] = 0xe;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x14;
                                    if t {
                                        break 'b_236684;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] = 0xd;
                                        if t {
                                            break 'b_2366d4;
                                        } else {
                                            break 'b_236684;
                                        }
                                    }
                                }
                                'b_2366a8: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    if c.g[V0][0] != 0u64 {
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0u64);
                                        break 'b_2366a8;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                        if c.g[V0][0] != 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            break 'b_236714;
                                        } else {
                                            c.cycles += 3;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                                            let t = true;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x1c),
                                                c.g[S2][0],
                                            );
                                            if t {
                                                break 'b_236710;
                                            } else {
                                                break 'b_2366a8;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 11;
                                c.g[A2][0] = 0x7;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c8), c.g[S2][0]);
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[RA][0] = sx(0x002366cc);
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                call_ptr(c, 0x0022cea8, 0x002366cc)?;
                                let t = true;
                                c.g[V0][0] = 0x3;
                                if t {
                                    break 'b_23670c;
                                } else {
                                    break 'b_2366d4;
                                }
                            }
                            'b_2366e4: {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 0x13;
                                if t {
                                    break 'b_2366e4;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_236714;
                                    } else {
                                        break 'b_2366e4;
                                    }
                                }
                            }
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                            c.g[RA][0] = sx(0x002366f8);
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
                            call_ptr(c, 0x001f36c0, 0x002366f8)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_236714;
                            } else {
                                break 'b_236700;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00236708);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00233aa0,
                        f: ee::s06::GameState_GotoNext,
                    }
                    .call(c, 0x00236708)?;
                    c.g[V0][0] = 0x3;
                    break 'b_23670c;
                }
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                break 'b_236710;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_236714;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameStateEventSwitchLoad_Update(self)` (0x236960, `cGFSectionLoadState::update`): while the section manager waits for the transport, state 0 sets the stream focus at the NIS end position once the section is loaded, state 1 waits for the focus, the transition manager and the NIS transition, ends the transport sequence and activates the section.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]

/// `void InputSync_dtor(InputSync *self, int flags)` (0x2370a0, `cInputSync::~cInputSync`): for every synced pad free its samples (if any) and delete its destination pad and input; delete `self` when `flags & 1`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn input_sync_dtor(c: &mut Cpu) {
    guest(c, |c| {
        'b_237110: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = 0u64;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S3][0] = c.g[A1][0];
            if t {
                break 'b_237110;
            } else {
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                'l_2370d8: loop {
                    'b_2370ec: {
                        c.cycles += 2;
                        if c.g[A0][0] == 0u64 {
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            break 'b_2370ec;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x002370e8);
                            call_ptr(c, 0x00317e98, 0x002370e8)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            break 'b_2370ec;
                        }
                    }
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x002370f4);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00317e50, 0x002370f4)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[RA][0] = sx(0x00237100);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x00317e50, 0x00237100)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x002370d8);
                        }
                        continue 'l_2370d8;
                    } else {
                        break 'b_237110;
                    }
                }
            }
        }
        'b_237128: {
            c.cycles += 3;
            c.g[V0][0] = c.g[S3][0] & 0x1;
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            if t {
                break 'b_237128;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00237124);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00317e50, 0x00237124)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_237128;
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

/// `void InputSync_SendSample(InputSync *self, Sample *src)` (0x237a98, `cInputSync::sendClientInput`): pack the sample into a 0x20-byte stack buffer, unpack it back into `self` (the local copy), and send the packet through `cGameComm` when it is synced, error-free and connected.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn input_sync_send_sample(c: &mut Cpu) {
    guest(c, |c| {
        'b_237af4: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A2][0] = c.g[SP][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[RA][0] = sx(0x00237abc);
            c.g[S1][0] = c.g[A1][0];
            Callee {
                addr: 0x00237b28,
                f: ee::s06::InputSync_PackSample,
            }
            .call(c, 0x00237abc)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00237ad0);
            c.g[A2][0] = c.g[SP][0];
            Callee {
                addr: 0x00237be8,
                f: ee::s06::InputSync_UnpackSample,
            }
            .call(c, 0x00237ad0)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_237af4;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                let t = c.g[V0][0] != 0u64;

                if t {
                    break 'b_237af4;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
                    break 'b_237af4;
                }
            }
        }
        'b_237b0c: {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            if t {
                break 'b_237b0c;
            } else {
                c.cycles += 4;
                c.g[A3][0] = c.g[S2][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x00237b0c);
                c.g[A2][0] = c.g[SP][0];
                Callee {
                    addr: 0x00257e98,
                    f: ee::s06::cGameComm_SendPacket,
                }
                .call(c, 0x00237b0c)?;
                break 'b_237b0c;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `void GMHandler0_PickOpponents(void)` (0x239938): build the list of characters 0..9 without the human rider's and the rival's, shuffle it, and fill the game mode manager's character and cheat-character tables (human, rival, then the shuffled rest); on a game path the rival slot takes the last shuffled one.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn gm_handler0_pick_opponents(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.g[RA][0] = sx(0x0023995c);
        c.g[S3][0] = 0u64;
        call_ptr(c, 0x0014dc80, 0x0023995c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00239968);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014dd58, 0x00239968)?;
        c.g[RA][0] = sx(0x00239970);
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0014dc80, 0x00239970)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023997c);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0014dd58, 0x0023997c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0023998c);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x00146e98, 0x0023998c)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0023999c);
        c.g[A1][0] = c.g[S4][0];
        call_ptr(c, 0x00147398, 0x0023999c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x002399a8);
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, 0x00145750, 0x002399a8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 0u64;
        c.g[A0][0] = c.g[SP][0];
        'l_2399b8: loop {
            'b_2399d8: {
                c.cycles += 2;
                if c.g[V1][0] == c.g[S1][0] {
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    break 'b_2399d8;
                } else {
                    c.cycles += 2;
                    if c.g[V1][0] == c.g[S0][0] {
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        break 'b_2399d8;
                    } else {
                        c.cycles += 4;
                        c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        break 'b_2399d8;
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10i64) as u64;
            let t = c.g[V0][0] != 0u64;

            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002399b8);
                }
                continue 'l_2399b8;
            } else {
                'b_239a54: {
                    c.cycles += 14;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x002399f0);
                    c.g[A1][0] = c.g[S3][0];
                    Callee {
                        addr: 0x0023c770,
                        f: ee::s06::GameRand_Shuffle,
                    }
                    .call(c, 0x002399f0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00239a04);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[S1][0]);
                    call_ptr(c, 0x00147410, 0x00239a04)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[S0][0]);
                    let t = (c.g[S3][0] as i64) <= 0;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x44), 0u64);
                    if t {
                        break 'b_239a54;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x48));
                        c.g[A2][0] = c.g[SP][0];
                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                        'l_239a28: loop {
                            c.cycles += 7;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            c.sw(c.g[A0][0] as u32, 0u64);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                            c.sw((c.g[A0][0] as u32).wrapping_sub(0x28), c.g[V1][0]);
                            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            if t {
                                break 'b_239a54;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = ((c.g[V0][0] as i64) < 10i64) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00239a28);
                                    }
                                    continue 'l_239a28;
                                } else {
                                    break 'b_239a54;
                                }
                            }
                        }
                    }
                }
                'b_239a7c: {
                    c.cycles += 5;
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    if t {
                        break 'b_239a7c;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                        break 'b_239a7c;
                    }
                }
                c.cycles += 8;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void GMHandler1_OnRiderFinished(self, idx)` (0x23a760, `cGMRaceHandler::raceFinish`): record the finish time (or 6000s for a rider that did not finish) in the per-heat table, and once the race and the rider are finished estimate the others' times, sort them, decide the next heat from the ranking (the heat unlock calls, the boost bar of the player's rider by place) and lock the results after heat 3.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn gm_handler1_on_rider_finished(c: &mut Cpu) {
    guest(c, |c| {
        'b_23a8b4: {
            'b_23a82c: {
                c.cycles += 37;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[S7][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x0023a79c);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
                call_ptr(c, 0x0014dc80, 0x0023a79c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0023a7a8);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x0023a7a8)?;
                c.g[RA][0] = sx(0x0023a7b0);
                c.g[FP][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x0023a7b0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0023a7bc);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x0023a7bc)?;
                c.g[A0][0] = c.g[FP][0];
                c.g[RA][0] = sx(0x0023a7c8);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00146e98, 0x0023a7c8)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                c.g[S0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[RA][0] = sx(0x0023a7ec);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0012a250, 0x0023a7ec)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_23a82c;
                } else {
                    c.cycles += 14;
                    c.g[A0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[S6][0] = c.g[A0][0];
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0023a824);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x0023a824)?;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_23a8b4;
                    } else {
                        break 'b_23a82c;
                    }
                }
            }
            'b_23a890: {
                c.cycles += 12;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[S6][0] = c.g[A0][0];
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x480));
                let t = c.g[A0][0] == 0u64;
                c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x478));
                if t {
                    break 'b_23a890;
                } else {
                    c.cycles += 13;
                    c.g[V1][0] = 0x28;
                    c.g[A0][0] = 0x50000;
                    c.mult(0, 2, 17, 3);
                    c.g[A0][0] = c.g[A0][0] | 0x7e40;
                    c.g[A1][0] = 0x1;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    c.g[V0][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66a8));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                    if t {
                        break 'b_23a8b4;
                    } else {
                        break 'b_23a890;
                    }
                }
            }
            c.cycles += 9;
            c.g[V0][0] = 0x28;
            c.g[V1][0] = 0x530000;
            c.mult(0, 4, 17, 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66a8));
            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S6][0] as u32));
            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
            c.sw(c.g[V1][0] as u32, 0u64);
            break 'b_23a8b4;
        }
        'b_23abdc: {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0023a8bc);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0012a250, 0x0023a8bc)?;
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_23abdc;
            } else {
                c.cycles += 12;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0023a8ec);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, t, 0x0023a8ec)?;
                let t = c.g[V0][0] == 0u64;
                c.g[S3][0] = sx((c.g[S3][0] as u32) << 2);
                if t {
                    break 'b_23abdc;
                } else {
                    'b_23a994: {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.sw(c.g[SP][0] as u32, c.g[S3][0]);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[S4][0] = 0u64;
                        if t {
                            break 'b_23a994;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = 0x28;
                            c.g[V1][0] = 0x530000;
                            c.mult(0, 17, 17, 2);
                            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x58));
                            c.mtc1(0, 20);
                            c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x28));
                            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            'l_23a930: loop {
                                'b_23a94c: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                                    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x470));
                                    c.c_le_s(20, 0);
                                    let t = c.fcc();
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_23a94c;
                                    } else {
                                        c.cycles += 1;
                                        c.g[V0][0] = 0u64;
                                        break 'b_23a94c;
                                    }
                                }
                                'b_23a960: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x58));
                                    if t {
                                        break 'b_23a960;
                                    } else {
                                        c.cycles += 3;
                                        c.g[RA][0] = sx(0x0023a95c);
                                        call_ptr(c, 0x00122d78, 0x0023a95c)?;
                                        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                                        break 'b_23a960;
                                    }
                                }
                                c.cycles += 13;
                                c.g[V0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                                c.sw(c.g[S5][0] as u32, c.g[V1][0]);
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0023a930);
                                    }
                                    continue 'l_23a930;
                                } else {
                                    break 'b_23a994;
                                }
                            }
                        }
                    }
                    'b_23ab7c: {
                        'b_23ab64: {
                            'b_23ab60: {
                                'b_23aad0: {
                                    c.cycles += 14;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                    c.g[RA][0] = sx(0x0023a9a0);
                                    c.g[A1][0] = 0xa;
                                    Callee {
                                        addr: 0x00238bf8,
                                        f: ee::s06::GameModeMan_SortByTime,
                                    }
                                    .call(c, 0x0023a9a0)?;
                                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                    c.g[T0][0] = 0x530000;
                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x6730));
                                    c.g[A3][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x70));
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = 0x1;
                                    if t {
                                        break 'b_23aad0;
                                    } else {
                                        'b_23aa08: {
                                            c.cycles += 13;
                                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[T0][0] = 0x3;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            c.sw(
                                                (c.g[A2][0] as u32).wrapping_add(0x70),
                                                c.g[V0][0],
                                            );
                                            c.g[V1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S6][0] as u32));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x100),
                                                c.g[A1][0],
                                            );
                                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                            let t = c.g[V0][0] != c.g[T0][0];
                                            c.g[A0][0] = 0x530000;
                                            if t {
                                                break 'b_23aa08;
                                            } else {
                                                c.cycles += 2;
                                                c.sw(
                                                    (c.g[A2][0] as u32).wrapping_add(0x84),
                                                    c.g[A1][0],
                                                );
                                                c.sw(
                                                    (c.g[A2][0] as u32).wrapping_add(0x70),
                                                    c.g[T0][0],
                                                );
                                                break 'b_23aa08;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                                        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                                        let t = c.g[V1][0] != 0u64;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        if t {
                                            break 'b_23ab64;
                                        } else {
                                            'b_23aab8: {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V1][0] = 0x3;
                                                if t {
                                                    break 'b_23aab8;
                                                } else {
                                                    'b_23aab4: {
                                                        'b_23aaa8: {
                                                            'b_23aa54: {
                                                                c.cycles += 5;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x480));
                                                                c.g[A2][0] =
                                                                    c.lw(c.g[V0][0] as u32);
                                                                c.g[V1][0] = ((c.g[A2][0] as i64)
                                                                    < 2i64)
                                                                    as u64;
                                                                let t = c.g[V1][0] == 0u64;
                                                                c.g[V1][0] = 0x530000;
                                                                if t {
                                                                    break 'b_23aa54;
                                                                } else {
                                                                    c.cycles += 7;
                                                                    c.g[A0][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x6640));
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x6640));
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_sub(
                                                                            (c.g[A1][0] as u32),
                                                                        ));
                                                                    c.g[V0][0] = (c.g[V0][0]
                                                                        < 0x259u64)
                                                                        as u64;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = c.g[FP][0];
                                                                    if t {
                                                                        break 'b_23aaa8;
                                                                    } else {
                                                                        break 'b_23aa54;
                                                                    }
                                                                }
                                                            }
                                                            'b_23aa7c: {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = 0x2;
                                                                let t = c.g[A2][0] != c.g[V0][0];
                                                                c.g[V1][0] = 0x530000;
                                                                if t {
                                                                    break 'b_23aa7c;
                                                                } else {
                                                                    c.cycles += 7;
                                                                    c.g[A0][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x6640));
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x6640));
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_sub(
                                                                            (c.g[A1][0] as u32),
                                                                        ));
                                                                    c.g[V0][0] = (c.g[V0][0]
                                                                        < 0x12du64)
                                                                        as u64;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = c.g[FP][0];
                                                                    if t {
                                                                        break 'b_23aaa8;
                                                                    } else {
                                                                        break 'b_23aa7c;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A2][0] != c.g[T0][0];
                                                            c.g[V1][0] = 0x530000;
                                                            if t {
                                                                break 'b_23ab7c;
                                                            } else {
                                                                c.cycles += 8;
                                                                c.g[V1][0] = 0x530000;
                                                                c.g[A0][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x6640));
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x6640));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_sub(
                                                                        (c.g[A1][0] as u32),
                                                                    ));
                                                                c.g[V0][0] =
                                                                    (c.g[V0][0] < 0xb5u64) as u64;
                                                                let t = c.g[V0][0] != 0u64;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x480));
                                                                if t {
                                                                    break 'b_23aab4;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[A0][0] = c.g[FP][0];
                                                                    break 'b_23aaa8;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.g[RA][0] = sx(0x0023aab0);
                                                        c.g[A1][0] = 0x1;
                                                        call_ptr(c, 0x00147d20, 0x0023aab0)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x480));
                                                        break 'b_23aab4;
                                                    }
                                                    c.cycles += 1;
                                                    c.g[V1][0] = 0x3;
                                                    break 'b_23aab8;
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                            let t = c.g[V0][0] != c.g[V1][0];
                                            c.g[V1][0] = 0x530000;
                                            if t {
                                                break 'b_23ab7c;
                                            } else {
                                                c.cycles += 3;
                                                c.sw((c.g[A0][0] as u32).wrapping_add(0x74), 0u64);
                                                let t = true;
                                                c.sw((c.g[A0][0] as u32).wrapping_add(0x70), 0u64);
                                                if t {
                                                    break 'b_23ab60;
                                                } else {
                                                    break 'b_23aad0;
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 16;
                                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                c.g[V0][0] = 0x1;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[T0][0] = 0x530000;
                                c.sw((c.g[A2][0] as u32).wrapping_add(0x70), c.g[V1][0]);
                                c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5bc8));
                                c.sw((c.g[S7][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                                c.sw((c.g[S7][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x100), 0u64);
                                c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x49));
                                if c.g[V0][0] != 0u64 {
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                    break 'b_23ab64;
                                } else {
                                    'b_23ab58: {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A0][0] = c.g[FP][0];
                                            break 'b_23ab58;
                                        } else {
                                            'b_23ab54: {
                                                'b_23ab3c: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x2;
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[V0][0] = 0x3;
                                                    if t {
                                                        break 'b_23ab3c;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                                        c.g[V0][0] =
                                                            ((c.g[V0][0] as i64) < 4i64) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x3;
                                                        if t {
                                                            break 'b_23ab54;
                                                        } else {
                                                            break 'b_23ab3c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != c.g[V0][0];
                                                c.g[V1][0] = 0x530000;
                                                if t {
                                                    break 'b_23ab7c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                                                    c.g[V0][0] = 0x5;
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x480)
                                                        );
                                                    if t {
                                                        break 'b_23ab64;
                                                    } else {
                                                        break 'b_23ab54;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A0][0] = c.g[FP][0];
                                            break 'b_23ab58;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[RA][0] = sx(0x0023ab60);
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x00147e18, 0x0023ab60)?;
                                    break 'b_23ab60;
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                            break 'b_23ab64;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = 0x3;
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[V1][0] = 0x530000;
                        if t {
                            break 'b_23ab7c;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = 0x1;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
                            break 'b_23ab7c;
                        }
                    }
                    'b_23abc8: {
                        'b_23abb0: {
                            'b_23aba0: {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6730));
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                let t = c.g[V1][0] != 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_23aba0;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5154));
                                    let t = true;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_23abb0;
                                    } else {
                                        break 'b_23aba0;
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            if t {
                                break 'b_23abc8;
                            } else {
                                c.cycles += 2;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5150));
                                break 'b_23abb0;
                            }
                        }
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                        let t = true;
                        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2f8));
                        if t {
                            break 'b_23abdc;
                        } else {
                            break 'b_23abc8;
                        }
                    }
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x2f8), 0u64);
                    break 'b_23abdc;
                }
            }
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameModule_UpdatePauseAndUI(GameModule *self)` (0x244b48, `cGame::updateUI`): `bool cGame_updateUI(cGame*)`: the per-frame UI update: the overlay state manager, then the mission message queue (command 8 opens the mission overlay 29, command 9 runs the mission command and opens 0x1e or pops the message), a pending overlay (34), the active menu, the pause menu request (overlay, pause audio) and the UI engine/playback update. Returns true when the game should simulate this frame.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_module_update_pause_and_ui(c: &mut Cpu) {
    guest(c, |c| {
        'b_244e4c: {
            'b_244c9c: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[RA][0] = sx(0x00244b60);
                c.g[S1][0] = c.g[A0][0];
                call_ptr(c, 0x0020cfd0, 0x00244b60)?;
                c.g[RA][0] = sx(0x00244b68);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                call_ptr(c, 0x002cc098, 0x00244b68)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1024));
                if t {
                    break 'b_244c9c;
                } else {
                    'b_244c98: {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c4));
                        let t = c.g[V1][0] == c.g[A0][0];
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c4));
                        if t {
                            break 'b_244c98;
                        } else {
                            'b_244bfc: {
                                c.cycles += 7;
                                c.g[RA][0] = sx(0x00244b8c);
                                c.g[A0][0] = c.g[A1][0];
                                call_ptr(c, 0x0030c870, 0x00244b8c)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = 0x8;
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.g[V0][0] = 0x9;
                                    break 'b_244bfc;
                                } else {
                                    c.cycles += 23;
                                    c.g[RA][0] = sx(0x00244ba8);
                                    c.g[A0][0] = 0x1d;
                                    call_ptr(c, 0x0020ab50, 0x00244ba8)?;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                    c.g[A1][0] = 0x440000;
                                    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28fc));
                                    c.g[V1][0] = 0x530000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
                                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                                    c.g[A1][0] = 0x3;
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[A2][0]);
                                    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                                    c.g[RA][0] = sx(0x00244bdc);
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A0][0]);
                                    call_ptr(c, 0x0028b180, 0x00244bdc)?;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 12);
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A1][0] = 0x4;
                                    c.g[RA][0] = sx(0x00244bf4);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x0029ced8, 0x00244bf4)?;
                                    let t = true;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_244e4c;
                                    } else {
                                        break 'b_244bfc;
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1024));
                            if t {
                                break 'b_244c9c;
                            } else {
                                'b_244c88: {
                                    c.cycles += 15;
                                    c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                                    c.g[A2][0] = 0x5;
                                    c.g[A1][0] = c.lw(c.g[A1][0] as u32);
                                    c.g[RA][0] = sx(0x00244c18);
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                                    call_ptr(c, 0x0030a868, 0x00244c18)?;
                                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                                    c.g[A0][0] = 0x480000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4868));
                                    c.g[RA][0] = sx(0x00244c2c);
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x00317670, 0x00244c2c)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x00244c38);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039f9d8, 0x00244c38)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                                        break 'b_244c88;
                                    } else {
                                        c.cycles += 18;
                                        c.g[RA][0] = sx(0x00244c48);
                                        c.g[A0][0] = 0x1e;
                                        call_ptr(c, 0x0020ab50, 0x00244c48)?;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                        c.g[V1][0] = 0x530000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
                                        c.g[A1][0] = 0x440000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28fc));
                                        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A0][0]);
                                        c.g[A3][0] = 0x3;
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                        c.sw(c.g[V0][0] as u32, c.g[A3][0]);
                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[A2][0]);
                                        c.g[RA][0] = sx(0x00244c80);
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                                        call_ptr(c, 0x00398038, 0x00244c80)?;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_244e4c;
                                        } else {
                                            break 'b_244c88;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00244c90);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c4));
                                call_ptr(c, 0x0030c820, 0x00244c90)?;
                                let t = true;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_244e4c;
                                } else {
                                    break 'b_244c98;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1024));
                    break 'b_244c9c;
                }
            }
            'b_244d00: {
                c.cycles += 3;
                c.g[V0][0] = !(0u64 | c.g[V0][0]);
                let t = c.g[V0][0] == 0u64;

                if t {
                    break 'b_244d00;
                } else {
                    c.cycles += 22;
                    c.g[RA][0] = sx(0x00244cb0);
                    c.g[A0][0] = 0x22;
                    call_ptr(c, 0x0020ab50, 0x00244cb0)?;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                    c.g[V1][0] = 0x440000;
                    c.g[A0][0] = 0x530000;
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2920));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x66e8));
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[A2][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = 0xffffffffffff0000;
                    c.g[A0][0] = 0xc;
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1024), c.g[V1][0]);
                    c.g[RA][0] = sx(0x00244cf0);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A1][0]);
                    call_ptr(c, 0x0028b180, 0x00244cf0)?;
                    c.g[RA][0] = sx(0x00244cf8);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0029ed18, 0x00244cf8)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_244e4c;
                    } else {
                        break 'b_244d00;
                    }
                }
            }
            'b_244d28: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00244d08);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                call_ptr(c, 0x002cc098, 0x00244d08)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S1][0];
                if t {
                    break 'b_244d28;
                } else {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x00244d18);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                    call_ptr(c, 0x00398038, 0x00244d18)?;
                    c.g[RA][0] = sx(0x00244d20);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                    call_ptr(c, 0x002cc0c0, 0x00244d20)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_244e4c;
                    } else {
                        break 'b_244d28;
                    }
                }
            }
            'b_244e00: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00244d30);
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x00231840,
                    f: ee::s06::GameModule_CheckPauseRequest,
                }
                .call(c, 0x00244d30)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                if t {
                    break 'b_244e00;
                } else {
                    'b_244d4c: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = 0u64;
                        if t {
                            break 'b_244d4c;
                        } else {
                            c.cycles += 1;
                            c.g[V1][0] = ((c.g[V0][0] as i64) < 10i64) as u64;
                            break 'b_244d4c;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                    if t {
                        break 'b_244e00;
                    } else {
                        c.cycles += 5;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x70));
                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                        if t {
                            break 'b_244e00;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00244d70);
                            call_ptr(c, 0x0020cbe8, 0x00244d70)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                            if t {
                                break 'b_244e00;
                            } else {
                                c.cycles += 34;
                                c.g[RA][0] = sx(0x00244d80);
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x850));
                                call_ptr(c, 0x00326ae0, 0x00244d80)?;
                                c.g[RA][0] = sx(0x00244d88);
                                call_ptr(c, 0x0020ca10, 0x00244d88)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                c.g[V1][0] = 0x530000;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
                                c.g[A1][0] = 0x440000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.g[A3][0] = 0x2;
                                c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28f8));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A0][0]);
                                c.sw(c.g[V0][0] as u32, c.g[A3][0]);
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[A2][0]);
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                                c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x228));
                                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x160));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x164));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00244dd8);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00244dd8)?;
                                c.g[RA][0] = sx(0x00244de0);
                                call_ptr(c, 0x0028b180, 0x00244de0)?;
                                c.g[RA][0] = sx(0x00244de8);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x00289b70, 0x00244de8)?;
                                c.g[RA][0] = sx(0x00244df0);
                                call_ptr(c, 0x0028b180, 0x00244df0)?;
                                c.g[RA][0] = sx(0x00244df8);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x00294f48, 0x00244df8)?;
                                let t = true;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_244e4c;
                                } else {
                                    break 'b_244e00;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
            c.g[RA][0] = sx(0x00244e0c);
            c.g[S0][0] = c.g[V0][0] & 0x1;
            call_ptr(c, 0x00398038, 0x00244e0c)?;
            c.g[RA][0] = sx(0x00244e14);
            call_ptr(c, 0x0020cba0, 0x00244e14)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_244e4c;
            } else {
                c.cycles += 6;
                c.g[RA][0] = sx(0x00244e24);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                call_ptr(c, 0x0026c5f8, 0x00244e24)?;
                c.g[RA][0] = sx(0x00244e2c);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                call_ptr(c, 0x002cc098, 0x00244e2c)?;
                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = 0u64;
                    break 'b_244e4c;
                } else {
                    'b_244e48: {
                        c.cycles += 2;
                        let t = c.g[S0][0] == 0u64;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                        if t {
                            break 'b_244e48;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = c.g[V1][0] & 0x1;
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_244e4c;
                            } else {
                                break 'b_244e48;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    break 'b_244e4c;
                }
            }
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

/// `void GameModule_SimStep(GameModule *self)` (0x244e60, `cGame::updateAI`): one simulation step of the game module: state update and bucket merge (state 3), replay and script engine update, then (unless paused) the bucket updates, the AI, the world script, crowd and particle managers; updates the views, bumps the update tick and triggers the world audio per view.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_module_sim_step(c: &mut Cpu) {
    guest(c, |c| {
        'b_244f28: {
            'b_244e9c: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                if c.g[A0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x214));
                    break 'b_244e9c;
                } else {
                    c.cycles += 5;
                    c.g[S2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2898));
                    c.g[RA][0] = sx(0x00244e90);
                    Callee {
                        addr: 0x00237950,
                        f: ee::s06::InputSync_IsReady,
                    }
                    .call(c, 0x00244e90)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                    if t {
                        break 'b_244f28;
                    } else {
                        c.cycles += 1;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x214));
                        break 'b_244e9c;
                    }
                }
            }
            'b_244edc: {
                'b_244ecc: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[V1][0] != c.g[V0][0];

                    if t {
                        break 'b_244ecc;
                    } else {
                        c.cycles += 9;
                        c.g[S0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2898));
                        c.g[RA][0] = sx(0x00244eb4);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00230f40,
                            f: ee::s06::GameModule_ProcessStateChange,
                        }
                        .call(c, 0x00244eb4)?;
                        c.g[S2][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x00244ec4);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x00354c98, 0x00244ec4)?;
                        let t = true;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                        if t {
                            break 'b_244edc;
                        } else {
                            break 'b_244ecc;
                        }
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x00244ed4);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x00230f40,
                    f: ee::s06::GameModule_ProcessStateChange,
                }
                .call(c, 0x00244ed4)?;
                c.g[S2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2898));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                break 'b_244edc;
            }
            'b_244ef0: {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let t = c.g[V0][0] != 0u64;

                if t {
                    break 'b_244ef0;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00244ef0);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                    call_ptr(c, 0x0026f4a8, 0x00244ef0)?;
                    break 'b_244ef0;
                }
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x00244ef8);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x00270280, 0x00244ef8)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
            if t {
                break 'b_244f28;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 0x8;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                if t {
                    break 'b_244f28;
                } else {
                    c.cycles += 7;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2a8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00244f24);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00244f24)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                    break 'b_244f28;
                }
            }
        }
        'b_244f48: {
            'b_244f44: {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let t = c.g[V0][0] != 0u64;
                c.g[S0][0] = 0u64;
                if t {
                    break 'b_244f44;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00244f3c);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                    call_ptr(c, 0x00270280, 0x00244f3c)?;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        break 'b_244f48;
                    } else {
                        break 'b_244f44;
                    }
                }
            }
            c.cycles += 1;
            c.g[S0][0] = 0x1;
            break 'b_244f48;
        }
        'b_245074: {
            'b_245050: {
                c.cycles += 2;
                let t = c.g[S0][0] != 0u64;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_245050;
                } else {
                    'b_244f7c: {
                        'b_244f6c: {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                            let t = c.g[A0][0] == 0u64;

                            if t {
                                break 'b_244f6c;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00244f64);
                                Callee {
                                    addr: 0x002379c8,
                                    f: ee::s06::InputSync_ApplyInput,
                                }
                                .call(c, 0x00244f64)?;
                                if c.g[V0][0] == 0u64 {
                                    c.g[S0][0] = 0x1;
                                    break 'b_244f7c;
                                } else {
                                    break 'b_244f6c;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x00244f74);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                        call_ptr(c, 0x0026ce50, 0x00244f74)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[S0][0] = 0x1;
                            break 'b_244f7c;
                        } else {
                            break 'b_244f7c;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        break 'b_245050;
                    } else {
                        'b_244fa0: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                            c.g[V0][0] = c.g[V0][0] & 0x2;
                            let t = c.g[V0][0] != 0u64;

                            if t {
                                break 'b_244fa0;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x00244f9c);
                                c.g[A1][0] = 0x1;
                                call_ptr(c, 0x00354f98, 0x00244f9c)?;
                                c.g[A0][0] = c.g[S2][0];
                                break 'b_244fa0;
                            }
                        }
                        'b_244fd4: {
                            c.cycles += 9;
                            c.g[RA][0] = sx(0x00244fa8);
                            c.g[A1][0] = 0x5;
                            call_ptr(c, 0x00354f98, 0x00244fa8)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x00244fb4);
                            c.g[A1][0] = 0x6;
                            call_ptr(c, 0x00354f98, 0x00244fb4)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                            c.g[V0][0] = c.g[V0][0] & 0x2;
                            if c.g[V0][0] != 0u64 {
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                break 'b_244fd4;
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x00244fd0);
                                c.g[A1][0] = 0x1;
                                call_ptr(c, 0x00355028, 0x00244fd0)?;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                break 'b_244fd4;
                            }
                        }
                        'b_245008: {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xcc));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00244fe8);
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00244fe8)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                            c.g[V0][0] = c.g[V0][0] & 0x10;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                            if t {
                                break 'b_245008;
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x00245004);
                                c.g[A1][0] = 0x2;
                                call_ptr(c, 0x00354f98, 0x00245004)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                                break 'b_245008;
                            }
                        }
                        'b_24501c: {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[V0][0] & 0x4;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = c.g[S2][0];
                            if t {
                                break 'b_24501c;
                            } else {
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x0024501c);
                                c.g[A1][0] = 0x3;
                                call_ptr(c, 0x00354f98, 0x0024501c)?;
                                break 'b_24501c;
                            }
                        }
                        c.cycles += 13;
                        c.g[RA][0] = sx(0x00245024);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                        call_ptr(c, 0x00309270, 0x00245024)?;
                        c.g[RA][0] = sx(0x0024502c);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        call_ptr(c, 0x002294c8, 0x0024502c)?;
                        c.g[RA][0] = sx(0x00245034);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                        call_ptr(c, 0x003440c8, 0x00245034)?;
                        c.g[RA][0] = sx(0x0024503c);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        call_ptr(c, 0x00357bf8, 0x0024503c)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x00245048);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x00354c98, 0x00245048)?;
                        let t = true;
                        c.g[A0][0] = c.g[S2][0];
                        if t {
                            break 'b_245074;
                        } else {
                            break 'b_245050;
                        }
                    }
                }
            }
            c.cycles += 6;
            c.g[RA][0] = sx(0x00245058);
            c.g[A1][0] = 0x5;
            call_ptr(c, 0x00354f98, 0x00245058)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
            c.g[V0][0] = c.g[V0][0] & 0x4;
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S2][0];
            if t {
                break 'b_245074;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00245070);
                c.g[A1][0] = 0x3;
                call_ptr(c, 0x00354f98, 0x00245070)?;
                c.g[A0][0] = c.g[S2][0];
                break 'b_245074;
            }
        }
        'b_245090: {
            c.cycles += 6;
            c.g[RA][0] = sx(0x0024507c);
            c.g[A1][0] = 0x4;
            call_ptr(c, 0x00354f98, 0x0024507c)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = 0u64;
            if t {
                break 'b_245090;
            } else {
                c.cycles += 1;
                c.g[V1][0] = ((c.g[V0][0] as i64) < 10i64) as u64;
                break 'b_245090;
            }
        }
        'b_2450ac: {
            'b_2450a0: {
                c.cycles += 2;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = c.g[V0][0] ^ 0x3;
                if t {
                    break 'b_2450a0;
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xac));
                        break 'b_2450ac;
                    } else {
                        break 'b_2450a0;
                    }
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x002450a8);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
            call_ptr(c, 0x0022e840, 0x002450a8)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xac));
            break 'b_2450ac;
        }
        'b_2450fc: {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw((c.g[S1][0] as u32).wrapping_add(0xac), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
            let t = c.g[V1][0] == 0u64;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_2450fc;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                'l_2450c8: loop {
                    c.cycles += 13;
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4c));
                    c.g[A2][0] = c.g[S0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x002450e8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    call_ptr(c, 0x002b7848, 0x002450e8)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                        if event_due(c) {
                            deliver_event_here(c, 0x002450c8);
                        }
                        continue 'l_2450c8;
                    } else {
                        break 'b_2450fc;
                    }
                }
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cGameLoadStateConquer_Init(self)` (0x245730): create the `110ctrl_load` UI screen, play frame 0 and show the PS2 default or pro controller widgets (and their texts) by the controller mapping of player interface 0.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn c_game_load_state_conquer_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_245788: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = 0x480000;
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4328));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x0024575c);
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x0024575c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00245770);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x00245770)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_245788;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x00245788);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x00245788)?;
                break 'b_245788;
            }
        }
        'b_2457ec: {
            c.cycles += 20;
            c.g[V0][0] = 0x480000;
            c.g[S0][0] = 0x480000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4318));
            c.g[RA][0] = sx(0x0024579c);
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4308));
            call_ptr(c, 0x0014dc80, 0x0024579c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002457a8);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x002457a8)?;
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x002457b4);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x00147618, 0x002457b4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002457c0);
            c.g[S2][0] = c.g[V0][0];
            call_ptr(c, 0x00317670, 0x002457c0)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x002457cc);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x002457cc)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = (c.g[S2][0] < 0x1u64) as u64;
            if t {
                break 'b_2457ec;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x002457ec);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002457ec)?;
                break 'b_2457ec;
            }
        }
        'b_245824: {
            c.cycles += 9;
            c.g[A0][0] = 0x480000;
            c.g[RA][0] = sx(0x002457f8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x42f8));
            call_ptr(c, 0x00317670, 0x002457f8)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x00245804);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00245804)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = (c.g[S2][0] < 0x1u64) as u64;
            if t {
                break 'b_245824;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00245824);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00245824)?;
                break 'b_245824;
            }
        }
        'b_245858: {
            c.cycles += 8;
            c.g[RA][0] = sx(0x0024582c);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x00317670, 0x0024582c)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x00245838);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00245838)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = (0u64 < c.g[S2][0]) as u64;
            if t {
                break 'b_245858;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00245858);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00245858)?;
                break 'b_245858;
            }
        }
        'b_245890: {
            c.cycles += 9;
            c.g[A0][0] = 0x480000;
            c.g[RA][0] = sx(0x00245864);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x42e8));
            call_ptr(c, 0x00317670, 0x00245864)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x00245870);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00245870)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = (0u64 < c.g[S2][0]) as u64;
            if t {
                break 'b_245890;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00245890);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00245890)?;
                break 'b_245890;
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake, machine};
    use crate::difftest::{Rng, DATA, GAME_GP, RA as DT_RA, SP as DT_SP};
    use ssx3_ee_rt::funcs::register_functions;

    /// Environment switch of the child process (see [`oc682_child`]).
    pub(crate) const OC682_CHILD_ENV: &str = "SSX3_OC682_CHILD";
    /// Seeds per test.
    pub(crate) const OC682_SEEDS: u32 = 160;
    /// Controls the fakes read: the "unknown indirect call returns 0" flag
    /// (+4) and the forced-result table (see [`oc682_force`]).
    pub(crate) const OC682_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC682_FORCE: u32 = OC682_CTRL + 0x100;
    pub(crate) const OC682_FORCE_SLOTS: u32 = 12;
    /// Objects of the fixtures (outside the pointer soup, filled like it).
    pub(crate) const OC682_OBJ: u32 = DATA + 0x5_0000;
    /// The pointer soup: 32-bit words that are pointers into the soup itself,
    /// zero, or small values the state machines compare against.
    pub(crate) const OC682_SOUP_WORDS: u32 = 0x1_0000;
    const OC682_SMALL: [u32; 16] = [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        8,
        9,
        10,
        12,
        13,
        14,
        17,
        0x16,
        0xffff_ffff,
    ];

    /// A direct callee (a slot of an installed function) replaced by a logging
    /// fake with the forced-result table (child process only).
    macro_rules! oc682_slot_fake {
        ($fake:ident, $addr:expr) => {
            fn $fake(c: &mut Cpu) {
                let ra = c.g[RA][0] as u32;
                oc682_fake_at(c, $addr, ra)
            }
        };
    }
    oc682_slot_fake!(oc682_fake_process_state_change, 0x0023_0f40);
    oc682_slot_fake!(oc682_fake_goto_next, 0x0023_3aa0);
    oc682_slot_fake!(oc682_fake_request_state, 0x0023_1250);
    oc682_slot_fake!(oc682_fake_get_transition_man, 0x0023_0698);
    oc682_slot_fake!(oc682_fake_rand_shuffle, 0x0023_c770);
    oc682_slot_fake!(oc682_fake_sort_by_time, 0x0023_8bf8);
    oc682_slot_fake!(oc682_fake_apply_input, 0x0023_79c8);
    oc682_slot_fake!(oc682_fake_is_ready, 0x0023_7950);
    oc682_slot_fake!(oc682_fake_check_pause_request, 0x0023_1840);
    oc682_slot_fake!(oc682_fake_unpack_sample, 0x0023_7be8);
    oc682_slot_fake!(oc682_fake_pack_sample, 0x0023_7b28);

    /// Child process only: an empty function table (every `call_indirect`
    /// reaches the fakes) and fakes in the slots of the installed functions
    /// the ports call directly. Ungated: the child runs this one test.
    pub(crate) fn oc682_install() {
        if std::env::var_os(OC682_CHILD_ENV).is_none() {
            return;
        }
        register_functions(&[]);
        // SAFETY: the child process runs this one test, single-threaded.
        unsafe {
            ee::s06::OVERRIDE_00230F40 = oc682_fake_process_state_change;
            ee::s06::OVERRIDE_00233AA0 = oc682_fake_goto_next;
            ee::s06::OVERRIDE_00231250 = oc682_fake_request_state;
            ee::s06::OVERRIDE_00230698 = oc682_fake_get_transition_man;
            ee::s06::OVERRIDE_0023C770 = oc682_fake_rand_shuffle;
            ee::s06::OVERRIDE_00238BF8 = oc682_fake_sort_by_time;
            ee::s06::OVERRIDE_002379C8 = oc682_fake_apply_input;
            ee::s06::OVERRIDE_00237950 = oc682_fake_is_ready;
            ee::s06::OVERRIDE_00231840 = oc682_fake_check_pause_request;
            ee::s06::OVERRIDE_00237BE8 = oc682_fake_unpack_sample;
            ee::s06::OVERRIDE_00237B28 = oc682_fake_pack_sample;
        }
    }

    /// Force the result of calls to `target`: call k returns `v1` if bit k of
    /// `mask` is set, else `v0`.
    pub(crate) fn oc682_force(c: &mut Cpu, slot: u32, target: u32, v0: u32, v1: u32, mask: u32) {
        let e = OC682_FORCE + slot * 20;
        c.bus.write32(e, target);
        c.bus.write32(e + 4, v0);
        c.bus.write32(e + 8, v1);
        c.bus.write32(e + 12, mask);
        c.bus.write32(e + 16, 0);
    }

    /// A logging fake call of `target` returning to `ra`, then the forced
    /// result (or, for unknown indirect targets, 0 half the time when the
    /// fixture asks for it).
    pub(crate) fn oc682_fake_at(c: &mut Cpu, target: u32, ra: u32) {
        fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        for slot in 0..OC682_FORCE_SLOTS {
            let e = OC682_FORCE + slot * 20;
            if c.bus.read32(e) == target {
                let k = c.bus.read32(e + 16);
                c.bus.write32(e + 16, k + 1);
                let mask = c.bus.read32(e + 12);
                let v = if (mask >> (k & 31)) & 1 != 0 {
                    c.bus.read32(e + 8)
                } else {
                    c.bus.read32(e + 4)
                };
                c.g[V0][0] = sx(v);
                return;
            }
        }
        let indirect = target < 0x0010_0000 || target >= 0x0080_0000;
        if indirect && c.bus.read32(OC682_CTRL + 4) != 0 && target & 4 != 0 {
            c.g[V0][0] = 0;
        }
    }

    /// The `miss` hook: calls to addresses outside the (empty) table.
    pub(crate) fn oc682_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        oc682_fake_at(c, target, ra)
    }

    /// Run `test` in a child process (a private function table) unless this
    /// is that child. Returns true in the child.
    pub(crate) fn oc682_child(module: &str, test: &str) -> bool {
        if std::env::var_os(OC682_CHILD_ENV).is_some() {
            return true;
        }
        let name = format!("{}::{test}", module.split_once("::").map_or("", |p| p.1));
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([name.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(OC682_CHILD_ENV, "1")
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
        false
    }

    /// Scrambled registers, the fakes, the pointer soup at `DATA`, the entry
    /// `pc` and the return address.
    pub(crate) fn oc682_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.hooks.miss = oc682_miss;
        oc682_install();
        c.pc = entry;
        c.g[RA][0] = DT_RA as u64;
        // The soup, and the fixture objects (also soup until a test sets a field).
        for i in 0..OC682_SOUP_WORDS + 0x1400 {
            let x = r.next_u32();
            let v = match x % 6 {
                0 | 1 => DATA + ((x >> 8) % OC682_SOUP_WORDS) * 4,
                2 => 0,
                _ => OC682_SMALL[((x >> 8) % 16) as usize],
            };
            let at = if i < OC682_SOUP_WORDS {
                DATA + i * 4
            } else {
                OC682_OBJ + (i - OC682_SOUP_WORDS) * 4
            };
            c.bus.write32(at, v);
        }
        r
    }

    /// One of `xs`.
    pub(crate) fn oc682_pick(r: &mut Rng, xs: &[u32]) -> u32 {
        xs[(r.next_u32() as usize) % xs.len()]
    }

    /// A word at `gp + off`.
    pub(crate) fn oc682_gp(c: &mut Cpu, off: i32, v: u32) {
        c.bus.write32(GAME_GP.wrapping_add(off as u32), v);
    }

    /// `GameStateEventSelect_Enter` against the generated body: every mode
    /// (0..3, out of range), the next location equal to the current one (the
    /// race restart) or not, and unwinds at random calls. `GameModule_ResetRace`
    /// is the real generated body (its calls are fakes).
    #[test]
    fn game_state_event_select_enter_matches() {
        if !oc682_child(module_path!(), "game_state_event_select_enter_matches") {
            return;
        }
        covered(
            ee::s06::f_00236250__generated,
            game_state_event_select_enter,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x23_6250, 14, 0x0023_6250);
                let mode = [0u32, 1, 2, 3, 4, 0xffff_ffff][(seed % 6) as usize];
                let location = oc682_pick(&mut r, &OC682_SMALL);
                oc682_gp(c, -0x1f34, location);
                // The next location is 0x17, or (mode 1) the new-race one.
                let expected = if mode == 1 { location } else { 0x17 };
                let at = OC682_OBJ + 0x1000;
                let equal = (seed / 6) % 2 == 0;
                c.bus
                    .write32(at, if equal { expected } else { expected ^ 4 });
                oc682_force(c, 0, 0x0014_4bc0, at, at, 0);
                c.g[A0][0] = OC682_OBJ as u64;
                c.g[A1][0] = sx(mode);
            },
        );
    }

    /// `GameStateEventSelect_Update` against the generated body: the states
    /// 0..2 and out-of-range ones, every mode, the NIS play state, overlay
    /// done flags, locations in and out of the back country range.
    #[test]
    fn game_state_event_select_update_matches() {
        if !oc682_child(module_path!(), "game_state_event_select_update_matches") {
            return;
        }
        covered(
            ee::s06::f_00236418__generated,
            game_state_event_select_update,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x23_6418, 14, 0x0023_6418);
                let this = OC682_OBJ;
                let bits = r.next_u32();
                let state = [0u32, 1, 2, 3, 0xffff_ffff, 1, 0, 2][(seed % 8) as usize];
                c.bus.write32(this + 0x10, state);
                c.bus
                    .write32(this + 0x14, [0u32, 1, 2, 3][((seed / 8) % 4) as usize]);
                c.bus.write32(this + 0x8, bits & 1);
                c.bus.write32(this + 0x1c, (bits >> 1) & 1);
                c.bus.write32(
                    this + 0x18,
                    [0xdu32, 0xe, 0xf, 0x10, 0x11, 0x17][((bits >> 2) % 6) as usize],
                );
                oc682_gp(c, -0x69c, (bits >> 8) % 6);
                oc682_gp(c, -0x6a0, r.next_u32());
                oc682_gp(c, -0xb5c, r.next_u32());
                c.bus
                    .write8(0x0053_5c10, [0u8, 4, 2][((bits >> 12) % 3) as usize]);
                c.bus.write8(0x0053_5c11, ((bits >> 14) & 1) as u8);
                // The NIS list play state, the location and the NIS state.
                let play = oc682_pick(&mut r, &[4, 5, 2, 9, 0, 1]);
                oc682_force(c, 0, 0x0027_9220, play, 5, 0);
                let at = OC682_OBJ + 0x1000;
                let location = oc682_pick(&mut r, &[0xd, 0xe, 0xf, 0x10, 0x11, 0x17, 5]);
                c.bus.write32(at, location);
                oc682_force(c, 1, 0x0014_4bc0, at, at, 0);
                let nis = oc682_pick(&mut r, &[0xe, 0x14, 0xd, 0x13, 7]);
                oc682_force(c, 2, 0x0027_9148, nis, nis, 0);
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// `GameStateEventSwitchLoad_Update` against the generated body: the
    /// states 0..2, a missing transport wait value, the stream focus distance
    /// on both sides of the limit, a busy or idle transition manager.

    /// `InputSync_dtor` against the generated body: 0..3 synced pads (and a
    /// negative count), with and without samples, `flags` bit 0 set or not.
    #[test]
    fn input_sync_dtor_matches() {
        if !oc682_child(module_path!(), "input_sync_dtor_matches") {
            return;
        }
        covered(
            ee::s06::f_002370a0__generated,
            input_sync_dtor,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x23_70a0, 10, 0x0023_70a0);
                let this = OC682_OBJ;
                let n = [0u32, 1, 2, 3, 0xffff_ffff, 2][(seed % 6) as usize];
                c.bus.write32(this + 0x10, n);
                for i in 0..3 {
                    let entry = this + 0x14 + i * 0x18;
                    let samples = if r.next_u32() & 1 != 0 {
                        OC682_OBJ + 0x800
                    } else {
                        0
                    };
                    c.bus.write32(entry, samples);
                    c.bus.write32(entry + 0x10, OC682_OBJ + 0x900);
                    c.bus.write32(entry + 0x14, OC682_OBJ + 0xa00);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = ((seed / 6) % 4) as u64;
            },
        );
    }

    /// `InputSync_SendSample` against the generated body: the game comm
    /// synced/error/connected flags (the packet is sent only when all agree;
    /// the send itself runs the real `cGameComm_SendPacket` with an empty
    /// send queue), pack/unpack are fakes.
    #[test]
    fn input_sync_send_sample_matches() {
        if !oc682_child(module_path!(), "input_sync_send_sample_matches") {
            return;
        }
        covered(
            ee::s06::f_00237a98__generated,
            input_sync_send_sample,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x23_7a98, 6, 0x0023_7a98);
                let comm = OC682_OBJ + 0x2000;
                oc682_gp(c, -0x204, comm);
                c.bus
                    .write32(comm + 0x40, [1u32, 0, 1, 1][(seed % 4) as usize]);
                c.bus
                    .write32(comm + 4, [0u32, 0, 1, 0][((seed / 4) % 4) as usize]);
                c.bus
                    .write32(comm, [5u32, 1, 0, 0xffff_ffff][((seed / 16) % 4) as usize]);
                c.bus.write32(comm + 0x364, 0);
                c.bus.write32(comm + 0x368, 0);
                c.g[A0][0] = (OC682_OBJ + 0x100) as u64;
                c.g[A1][0] = (OC682_OBJ + 0x200) as u64;
                let _ = r.next_u32();
            },
        );
    }

    /// `GMHandler0_PickOpponents` against the generated body: the human and
    /// rival characters inside and outside 0..9 (equal or not), the game path
    /// flag; the shuffle is a fake.
    #[test]
    fn gm_handler0_pick_opponents_matches() {
        if !oc682_child(module_path!(), "gm_handler0_pick_opponents_matches") {
            return;
        }
        covered(
            ee::s06::f_00239938__generated,
            gm_handler0_pick_opponents,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x23_9938, 10, 0x0023_9938);
                oc682_gp(c, -0x480, OC682_OBJ + 0x2000);
                let chars = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12];
                let a = oc682_pick(&mut r, &chars);
                let b = if seed % 5 == 0 {
                    a
                } else {
                    oc682_pick(&mut r, &chars)
                };
                oc682_force(c, 0, 0x0014_7398, a, a, 0);
                oc682_force(c, 1, 0x0014_5750, b, b, 0);
                oc682_force(c, 2, 0x0014_6e98, seed % 4, seed % 4, 0);
                c.bus.write8(0x0053_5c11, (seed & 1) as u8);
            },
        );
    }

    /// `GMHandler1_OnRiderFinished` against the generated body: heats 0..4,
    /// 0..9 AI riders, race and rider finished or not, every place and time
    /// gap of the heat logic, the did-not-finish case, the boost bar of the
    /// player's rider; the sort is a fake.
    #[test]
    fn gm_handler1_on_rider_finished_matches() {
        if !oc682_child(module_path!(), "gm_handler1_on_rider_finished_matches") {
            return;
        }
        covered(
            ee::s06::f_0023a760__generated,
            gm_handler1_on_rider_finished,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x23_a760, 14, 0x0023_a760);
                let bits = r.next_u32();
                // The game mode manager (gp-0x480): heat, AI count, next heats.
                let man = OC682_OBJ + 0x2000;
                oc682_gp(c, -0x480, man);
                for i in 0..0x40 {
                    let v = oc682_pick(&mut r, &OC682_SMALL);
                    c.bus.write32(man + 4 * i, v);
                }
                c.bus
                    .write32(man, [1u32, 2, 3, 0, 4][((bits >> 4) % 5) as usize]);
                c.bus.write32(
                    man + 0x10,
                    [0u32, 1, 2, 3, 9, 0xffff_ffff][(seed % 6) as usize],
                );
                // The riders: gp-0x848 -> +0x84 -> +0xc -> riders at +0x28.
                let (world, game, riders) =
                    (OC682_OBJ + 0x2100, OC682_OBJ + 0x2200, OC682_OBJ + 0x2300);
                oc682_gp(c, -0x848, world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0xc, riders);
                for i in 0..12 {
                    let rider = DATA + (r.next_u32() % OC682_SOUP_WORDS) * 4;
                    c.bus.write32(riders + 0x28 + 4 * i, rider);
                }
                // The race tables: times, did-not-finish flags and places.
                let base = r.next_u32() % 600;
                c.bus.write32(0x0053_6640, base);
                let gap = oc682_pick(&mut r, &[100, 179, 181, 299, 301, 599, 601, 700]);
                c.bus.write32(0x0053_6644, base + gap);
                for i in 2..10 {
                    c.bus.write32(0x0053_6640 + 4 * i, r.next_u32() % 900);
                }
                for i in 0..16 {
                    c.bus.write32(0x0053_66a8 + 4 * i, r.next_u32() & 1);
                    let place = r.next_u32() % 7;
                    c.bus.write32(0x0053_6730 + 4 * i, place);
                }
                c.bus.write8(0x0053_5c11, ((bits >> 8) & 1) as u8);
                oc682_gp(
                    c,
                    -0x5154,
                    [0x3f33_3333u32, 0x3eb3_3333, 0][((bits >> 10) % 3) as usize],
                );
                oc682_gp(
                    c,
                    -0x5150,
                    [0x3f33_3333u32, 0x3eb3_3333, 0][((bits >> 12) % 3) as usize],
                );
                // Race finished (first and later calls), the human rider.
                let a = (bits >> 14) & 1;
                let b = (bits >> 15) & 1;
                oc682_force(c, 0, 0x0012_a250, a, b, bits >> 16 & 0xff);
                oc682_force(c, 1, 0x0014_6e98, (bits >> 24) % 4, (bits >> 24) % 4, 0);
                c.bus.write32(OC682_CTRL + 4, 1);
                c.g[A0][0] = (OC682_OBJ + 0x4000) as u64;
                c.g[A1][0] = ((bits >> 26) % 4) as u64;
            },
        );
    }

    /// `GameModule_UpdatePauseAndUI` against the generated body: the mission
    /// message queue (commands 8, 9 and others), a pending overlay (-1 flag),
    /// the menu active or not, the pause request, replay/playback racing and
    /// the pause flags.
    #[test]
    fn game_module_update_pause_and_ui_matches() {
        if !oc682_child(module_path!(), "game_module_update_pause_and_ui_matches") {
            return;
        }
        covered(
            ee::s06::f_00244b48__generated,
            game_module_update_pause_and_ui,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x24_4b48, 22, 0x0024_4b48);
                let bits = r.next_u32();
                let this = OC682_OBJ;
                let (script, replay, playback, entry) = (
                    OC682_OBJ + 0x2000,
                    OC682_OBJ + 0x2100,
                    OC682_OBJ + 0x2200,
                    OC682_OBJ + 0x2300,
                );
                c.bus.write32(this + 0x30, script);
                c.bus.write32(script + 0x1c4, 7);
                c.bus
                    .write32(script + 0x1c8, if bits & 1 != 0 { 7 } else { 8 });
                c.bus.write32(this + 0x28, replay);
                c.bus
                    .write32(replay, [0u32, 5, 12][((bits >> 1) % 3) as usize]);
                c.bus.write32(this + 0x34, playback);
                c.bus
                    .write32(playback + 0x70, [0u32, 1, 2][((bits >> 3) % 3) as usize]);
                let cmd = [8u32, 9, 3, 8, 9][((bits >> 5) % 5) as usize];
                c.bus.write32(entry + 4, cmd);
                oc682_force(c, 0, 0x0030_c870, entry, entry, 0);
                oc682_gp(
                    c,
                    -0x1024,
                    [0xffff_ffffu32, 0, 7][((bits >> 8) % 3) as usize],
                );
                oc682_gp(c, -0x6a0, (bits >> 10) & 0x1f);
                oc682_gp(c, -0x69c, (bits >> 15) % 6);
                let mask = r.next_u32();
                oc682_force(c, 1, 0x002c_c098, 0, 1, mask);
                oc682_force(c, 2, 0x0023_1840, (bits >> 16) & 1, 1, 0);
                oc682_force(c, 3, 0x0039_f9d8, (bits >> 17) & 1, 1, 0);
                oc682_force(c, 4, 0x0020_cbe8, (bits >> 18) & 1, 1, 0);
                oc682_force(c, 5, 0x0020_cba0, (bits >> 19) & 1, 1, 0);
                c.bus.write32(OC682_CTRL + 4, 1);
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// `GameModule_SimStep` against the generated body: with and without input
    /// sync (ready or not), the state 3 bucket merge, the pause flags, the
    /// replay/playback AI pauses, replay modes and 0..3 views.
    #[test]
    fn game_module_sim_step_matches() {
        if !oc682_child(module_path!(), "game_module_sim_step_matches") {
            return;
        }
        covered(
            ee::s06::f_00244e60__generated,
            game_module_sim_step,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x24_4e60, 30, 0x0024_4e60);
                let bits = r.next_u32();
                let this = OC682_OBJ;
                let (replay, views, script) =
                    (OC682_OBJ + 0x2000, OC682_OBJ + 0x2100, OC682_OBJ + 0x2200);
                c.bus.write32(
                    this + 0x7c,
                    if bits & 1 != 0 { OC682_OBJ + 0x2300 } else { 0 },
                );
                c.bus
                    .write32(this + 0x214, [3u32, 0, 1, 3][((bits >> 1) % 4) as usize]);
                c.bus.write32(this + 0x28, replay);
                c.bus
                    .write32(replay, [0u32, 3, 5, 9, 10, 12][((bits >> 3) % 6) as usize]);
                c.bus.write32(this + 0x84, views);
                c.bus
                    .write32(views + 0x10, [0u32, 1, 2, 3][((bits >> 6) % 4) as usize]);
                oc682_gp(c, -0x84c, script);
                c.bus
                    .write32(script + 0x2a8, DATA + (r.next_u32() % OC682_SOUP_WORDS) * 4);
                oc682_gp(c, -0x6a0, (bits >> 8) & 0x1f);
                let mask = r.next_u32();
                oc682_force(c, 0, 0x0023_7950, 0, 1, mask);
                oc682_force(c, 1, 0x0023_79c8, 0, 1, mask >> 3);
                oc682_force(c, 2, 0x0027_0280, 0, 1, mask >> 5);
                oc682_force(c, 3, 0x0026_ce50, (bits >> 13) & 1, 1, 0);
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// `cGameLoadStateConquer_Init` against the generated body: the screen
    /// created or not, the widgets found or not, the controller mapping 0 or
    /// not.
    #[test]
    fn c_game_load_state_conquer_init_matches() {
        if !oc682_child(module_path!(), "c_game_load_state_conquer_init_matches") {
            return;
        }
        covered(
            ee::s06::f_00245730__generated,
            c_game_load_state_conquer_init,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x24_5730, 12, 0x0024_5730);
                let bits = r.next_u32();
                let widget = DATA + 0x1000;
                oc682_force(
                    c,
                    0,
                    0x0039_7cc0,
                    if bits & 1 != 0 { widget } else { 0 },
                    widget,
                    0,
                );
                oc682_force(c, 1, 0x0039_d860, 0, widget, bits >> 1 & 0xf);
                oc682_force(
                    c,
                    2,
                    0x0014_7618,
                    [0u32, 1, 3][((bits >> 5) % 3) as usize],
                    0,
                    0,
                );
                c.g[A0][0] = OC682_OBJ as u64;
            },
        );
    }
}
