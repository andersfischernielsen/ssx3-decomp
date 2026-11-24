//! Staging module oc518 (wave wave230): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc518`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0023_9230,
    gm_handler0_on_rider_finished,
    "GMHandler0_OnRiderFinished",
)];

/// `void GMHandler0_OnRiderFinished(GMHandler0* self, int idx)` (0x239230,
/// `cGMBigAirHandler_raceFinish`): rider `idx` finished the race. Its heat
/// total (the score, or 0 for a did-not-finish, with the out flag
/// `D_005366A8[idx]`) goes to `self->t[heat - 1][idx]`. When the race is
/// over (`cAI_isRaceFinished`), the sorted standings pick the next heat
/// (`iNextHeat`, `iRestoreHeat`) and the rider's continue flag (`unk_100`),
/// and the quick-race flag (`0x535c11`) decides whether the player gets the
/// result screen (`func_00147D20` win, `func_00147E18` lose). Finally the
/// human player's boost bar is set from the place: 0.7 first, 0.35 second,
/// else 0.
///
/// The body is the generated control flow with its calls through
/// `call_ptr` and its back-edge events through `event_due`; the register
/// and cycle accounting (including the scratch registers the original leaves)
/// is the generated one.
#[allow(
    unreachable_code,
    unused_labels,
    unused_assignments,
    unused_variables,
    unused_mut
)]
pub fn gm_handler0_on_rider_finished(c: &mut Cpu) {
    guest(c, |c| {
        'b_239374: {
            'b_239334: {
                c.cycles += 40;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[S0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.g[S4][0] = c.g[A0][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
                c.g[RA][0] = sx(0x0023926c);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x150));
                call_ptr(c, 0x0014dc80, 0x0023926c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00239278);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00239278)?;
                c.g[RA][0] = sx(0x00239280);
                c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
                call_ptr(c, 0x0014dc80, 0x00239280)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0023928c);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x0023928c)?;
                c.g[S6][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[V0][0] = 0x530000;
                c.g[A0][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x002392a4);
                c.g[S5][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                call_ptr(c, 0x00146e98, 0x002392a4)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x790));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xfc));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.g[SP][0];
                if t {
                    break 'b_239334;
                } else {
                    c.cycles += 4;
                    c.g[S2][0] = c.g[S0][0];
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19c));
                    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = 0x530000;
                    'l_2392e0: loop {
                        c.cycles += 19;
                        c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(4, (c.g[V1][0] as u32));
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(4, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.sdr(4, (c.g[A1][0] as u32));
                        c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.sdr(6, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002392e0);
                            }
                            continue 'l_2392e0;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            if t {
                                break 'b_239374;
                            } else {
                                break 'b_239334;
                            }
                        }
                    }
                }
            }
            c.cycles += 4;
            c.g[S2][0] = c.g[S0][0];
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19c));
            c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
            c.g[A3][0] = 0x530000;
            'l_239344: loop {
                c.cycles += 11;
                c.g[A0][0] = c.ld((c.g[V1][0] as u32));
                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd((c.g[A1][0] as u32), c.g[A0][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00239344);
                    }
                    continue 'l_239344;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    break 'b_239374;
                }
            }
        }
        'b_239404: {
            'b_2393d8: {
                c.cycles += 13;
                c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(2, (c.g[V1][0] as u32));
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[A1][0] as u32));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x480));
                let t = c.g[A0][0] == 0u64;
                c.g[A1][0] = 0x28;
                if t {
                    break 'b_2393d8;
                } else {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x66a8));
                    c.g[A2][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.mult(0, 4, 2, 5);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    let t = true;
                    c.sw((c.g[V1][0] as u32), c.g[A2][0]);
                    if t {
                        break 'b_239404;
                    } else {
                        break 'b_2393d8;
                    }
                }
            }
            c.cycles += 11;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x9c));
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x66a8));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.mult(0, 4, 2, 5);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[A2][0]);
            c.sw((c.g[V1][0] as u32), 0u64);
            break 'b_239404;
        }
        'b_2398b0: {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[RA][0] = sx(0x00239414);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x0012a250, 0x00239414)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            if t {
                break 'b_2398b0;
            } else {
                'b_239858: {
                    'b_239854: {
                        'b_239828: {
                            'b_239804: {
                                'b_2396d8: {
                                    'b_2395ac: {
                                        c.cycles += 4;
                                        c.g[V0][0] = 0x2;
                                        c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                                        if t {
                                            break 'b_2395ac;
                                        } else {
                                            'b_239458: {
                                                'b_239444: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_239444;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[S7][0] = sx((c.g[S1][0] as u32) << 2);
                                                        if t {
                                                            break 'b_239458;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[FP][0] = 0x530000;
                                                            if t {
                                                                break 'b_239854;
                                                            } else {
                                                                break 'b_239444;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x3;
                                                let t = c.g[V1][0] == c.g[V0][0];
                                                c.g[S7][0] = sx((c.g[S1][0] as u32) << 2);
                                                if t {
                                                    break 'b_2396d8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[FP][0] = 0x530000;
                                                    if t {
                                                        break 'b_239854;
                                                    } else {
                                                        break 'b_239458;
                                                    }
                                                }
                                            }
                                            'b_2394b8: {
                                                c.cycles += 7;
                                                c.g[V1][0] =
                                                    c.lw((c.g[A1][0] as u32).wrapping_add(0x70));
                                                c.g[V0][0] =
                                                    c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                                                c.g[A2][0] = 0u64;
                                                c.sw(
                                                    (c.g[A1][0] as u32).wrapping_add(0x74),
                                                    c.g[V1][0],
                                                );
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                let t = (c.g[V0][0] as i64) <= 0;
                                                c.g[FP][0] = 0x530000;
                                                if t {
                                                    break 'b_2394b8;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[V0][0] = 0x530000;
                                                    c.g[T0][0] = c.g[A1][0];
                                                    c.g[A3][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x6640));
                                                    c.g[A0][0] =
                                                        sx((c.g[S4][0] as u32).wrapping_add(0x2c));
                                                    c.g[A1][0] = c.g[S0][0];
                                                    'l_239488: loop {
                                                        c.cycles += 12;
                                                        c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                                        c.g[A2][0] =
                                                            sx((c.g[A2][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                        c.g[A1][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[V1][0] as u32)));
                                                        c.sw((c.g[A3][0] as u32), c.g[V0][0]);
                                                        c.g[V0][0] = c
                                                            .lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[V0][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] = ((c.g[A2][0] as i64)
                                                            < (c.g[V0][0] as i64))
                                                            as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[A3][0] =
                                                            sx((c.g[A3][0] as u32)
                                                                .wrapping_add(0x4));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x00239488);
                                                            }
                                                            continue 'l_239488;
                                                        } else {
                                                            break 'b_2394b8;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_2394f0: {
                                                'b_2394e8: {
                                                    c.cycles += 10;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x480)
                                                        );
                                                    c.g[A1][0] = c
                                                        .lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                                    c.g[RA][0] = sx(0x002394c8);
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                    call_ptr(c, 0x00238b70, 0x002394c8)?;
                                                    c.g[V0][0] =
                                                        sx((c.g[FP][0] as u32)
                                                            .wrapping_add(0x6730));
                                                    c.g[V0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                    c.g[V1][0] =
                                                        ((c.g[V1][0] as i64) < 3i64) as u64;
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x480)
                                                        );
                                                    if t {
                                                        break 'b_2394e8;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[V0][0] = 0x3;
                                                        if t {
                                                            break 'b_2394f0;
                                                        } else {
                                                            break 'b_2394e8;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                break 'b_2394f0;
                                            }
                                            'b_23953c: {
                                                c.cycles += 6;
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x70),
                                                    c.g[V0][0],
                                                );
                                                c.g[A2][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                                c.g[V0][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                let t = (c.g[V0][0] as i64) <= 0;
                                                c.g[A0][0] = 0u64;
                                                if t {
                                                    break 'b_23953c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = 0x530000;
                                                    c.g[V1][0] = c.g[S0][0];
                                                    c.g[A1][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x6640));
                                                    'l_239518: loop {
                                                        c.cycles += 9;
                                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V1][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.sw((c.g[A1][0] as u32), c.g[V0][0]);
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[V0][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] = ((c.g[A0][0] as i64)
                                                            < (c.g[V0][0] as i64))
                                                            as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[A1][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x4));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x00239518);
                                                            }
                                                            continue 'l_239518;
                                                        } else {
                                                            break 'b_23953c;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_239590: {
                                                c.cycles += 6;
                                                c.g[V1][0] =
                                                    sx((c.g[FP][0] as u32).wrapping_add(0x6730));
                                                c.g[S1][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                                c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                if t {
                                                    break 'b_239590;
                                                } else {
                                                    c.cycles += 8;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x480)
                                                        );
                                                    c.g[A1][0] = c
                                                        .lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                                    c.g[RA][0] = sx(0x00239564);
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                    call_ptr(c, 0x00238b70, 0x00239564)?;
                                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                                    c.g[V0][0] =
                                                        ((c.g[V0][0] as i64) < 3i64) as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    if t {
                                                        break 'b_239590;
                                                    } else {
                                                        c.cycles += 7;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[A0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x84));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[S2][0] as u32)));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x28));
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x100),
                                                            0u64,
                                                        );
                                                        if t {
                                                            break 'b_239854;
                                                        } else {
                                                            break 'b_239590;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[A1][0] = 0x1;
                                            c.g[V1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[S2][0] as u32)));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                            let t = true;
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x100),
                                                c.g[A1][0],
                                            );
                                            if t {
                                                break 'b_239854;
                                            } else {
                                                break 'b_2395ac;
                                            }
                                        }
                                    }
                                    'b_239608: {
                                        c.cycles += 6;
                                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                                        c.g[S7][0] = sx((c.g[S1][0] as u32) << 2);
                                        c.g[A2][0] = 0u64;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[FP][0] = 0x530000;
                                        if t {
                                            break 'b_239608;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] = 0x530000;
                                            c.g[T0][0] = c.g[A1][0];
                                            c.g[A3][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x2c));
                                            'l_2395d8: loop {
                                                c.cycles += 12;
                                                c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.sw((c.g[A3][0] as u32), c.g[V0][0]);
                                                c.g[V0][0] =
                                                    c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                c.g[V0][0] = ((c.g[A2][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002395d8);
                                                    }
                                                    continue 'l_2395d8;
                                                } else {
                                                    break 'b_239608;
                                                }
                                            }
                                        }
                                    }
                                    'b_23965c: {
                                        c.cycles += 9;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = sx(0x00239618);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        call_ptr(c, 0x00238b70, 0x00239618)?;
                                        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[A0][0] = 0u64;
                                        if t {
                                            break 'b_23965c;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x530000;
                                            c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x2c));
                                            c.g[A1][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                                            'l_239638: loop {
                                                c.cycles += 9;
                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                                c.sw((c.g[A1][0] as u32), c.g[V0][0]);
                                                c.g[V0][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                c.g[V0][0] = ((c.g[A0][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x00239638);
                                                    }
                                                    continue 'l_239638;
                                                } else {
                                                    break 'b_23965c;
                                                }
                                            }
                                        }
                                    }
                                    'b_2396c4: {
                                        c.cycles += 9;
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6730));
                                        c.g[A2][0] =
                                            sx((c.g[S2][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x70));
                                        c.sw((c.g[A1][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                                        c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        if t {
                                            break 'b_2396c4;
                                        } else {
                                            c.cycles += 9;
                                            c.g[V0][0] = 0x3;
                                            c.sw(
                                                (c.g[A1][0] as u32).wrapping_add(0x70),
                                                c.g[V0][0],
                                            );
                                            c.g[A1][0] = 0x1;
                                            c.g[V1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[S2][0] as u32)));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                            let t = c.g[S5][0] != 0u64;
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x100),
                                                c.g[A1][0],
                                            );
                                            if t {
                                                break 'b_239854;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] =
                                                    sx((c.g[FP][0] as u32).wrapping_add(0x6730));
                                                if t {
                                                    break 'b_239858;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[A0][0] = c.g[S6][0];
                                                    c.g[RA][0] = sx(0x002396bc);
                                                    c.g[A1][0] = 0u64;
                                                    call_ptr(c, 0x00147d20, 0x002396bc)?;
                                                    let t = true;
                                                    c.g[V0][0] =
                                                        sx((c.g[FP][0] as u32)
                                                            .wrapping_add(0x6730));
                                                    if t {
                                                        break 'b_239858;
                                                    } else {
                                                        break 'b_2396c4;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[V0][0] = 0x1;
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                    if t {
                                        break 'b_239804;
                                    } else {
                                        break 'b_2396d8;
                                    }
                                }
                                'b_239760: {
                                    c.cycles += 10;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[V0][0] = 0x1;
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
                                    c.g[S0][0] = 0u64;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x80));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                    let t = (c.g[A0][0] as i64) <= 0;
                                    c.g[FP][0] = 0x530000;
                                    if t {
                                        break 'b_239760;
                                    } else {
                                        c.cycles += 4;
                                        c.mtc1(0, 20);
                                        c.g[S3][0] = c.g[A0][0];
                                        c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x54));
                                        'l_239710: loop {
                                            'b_239740: {
                                                c.cycles += 11;
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                                                c.g[V1][0] =
                                                    c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x470));
                                                c.c_le_s(20, 0);
                                                let t = c.fcc();
                                                c.g[A2][0] = 0x1;
                                                if t {
                                                    break 'b_239740;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[A2][0] = 0u64;
                                                    break 'b_239740;
                                                }
                                            }
                                            'b_239754: {
                                                c.cycles += 2;
                                                let t = c.g[A2][0] != 0u64;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_239754;
                                                } else {
                                                    c.cycles += 3;
                                                    call_ptr(c, 0x00122e50, 0x00239750)?;
                                                    c.sw((c.g[S1][0] as u32), c.g[V0][0]);
                                                    break 'b_239754;
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                ((c.g[S0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00239710);
                                                }
                                                continue 'l_239710;
                                            } else {
                                                break 'b_239760;
                                            }
                                        }
                                    }
                                }
                                'b_2397a4: {
                                    c.cycles += 5;
                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[A0][0] = 0u64;
                                    if t {
                                        break 'b_2397a4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x530000;
                                        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x54));
                                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                                        'l_239780: loop {
                                            c.cycles += 9;
                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                            c.sw((c.g[A1][0] as u32), c.g[V0][0]);
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            c.g[V0][0] =
                                                ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00239780);
                                                }
                                                continue 'l_239780;
                                            } else {
                                                break 'b_2397a4;
                                            }
                                        }
                                    }
                                }
                                'b_2397f4: {
                                    'b_2397f0: {
                                        'b_2397ec: {
                                            c.cycles += 13;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                            c.g[A1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                            c.g[RA][0] = sx(0x002397b4);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            call_ptr(c, 0x00238b70, 0x002397b4)?;
                                            c.g[A1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                            c.g[V1][0] =
                                                sx((c.g[FP][0] as u32).wrapping_add(0x6730));
                                            c.g[V1][0] = sx((c.g[S2][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            c.g[A0][0] =
                                                c.lw((c.g[A1][0] as u32).wrapping_add(0x70));
                                            c.sw(
                                                (c.g[A1][0] as u32).wrapping_add(0x74),
                                                c.g[A0][0],
                                            );
                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                            c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                                            let t = c.g[S5][0] != 0u64;
                                            c.g[V1][0] = c.g[V0][0] ^ 0x1;
                                            if t {
                                                break 'b_2397ec;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != 0u64;
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_2397f0;
                                                } else {
                                                    c.cycles += 3;
                                                    c.sw(
                                                        (c.g[A1][0] as u32).wrapping_add(0x74),
                                                        0u64,
                                                    );
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[A1][0] as u32).wrapping_add(0x70),
                                                        0u64,
                                                    );
                                                    if t {
                                                        break 'b_2397f4;
                                                    } else {
                                                        break 'b_2397ec;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = 0x3;
                                        break 'b_2397f0;
                                    }
                                    c.cycles += 1;
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                                    break 'b_2397f4;
                                }
                                c.cycles += 2;
                                let t = c.g[V1][0] == 0u64;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_239828;
                                } else {
                                    c.cycles += 2;
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                    break 'b_239804;
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                            let t = c.g[S5][0] != 0u64;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x100), 0u64);
                            if t {
                                break 'b_239854;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S6][0];
                                c.g[RA][0] = sx(0x00239820);
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x00147e18, 0x00239820)?;
                                let t = true;
                                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6730));
                                if t {
                                    break 'b_239858;
                                } else {
                                    break 'b_239828;
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = 0x1;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                        let t = c.g[S5][0] != 0u64;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x100), c.g[A1][0]);
                        if t {
                            break 'b_239854;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S6][0];
                            c.g[RA][0] = sx(0x00239854);
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x00147d20, 0x00239854)?;
                            break 'b_239854;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6730));
                    break 'b_239858;
                }
                'b_23989c: {
                    'b_239884: {
                        'b_239874: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                            let t = c.g[V1][0] != 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_239874;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x515c));
                                let t = true;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_239884;
                                } else {
                                    break 'b_239874;
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if t {
                            break 'b_23989c;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5158));
                            break 'b_239884;
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
                        break 'b_2398b0;
                    } else {
                        break 'b_23989c;
                    }
                }
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x2f8), 0u64);
                break 'b_2398b0;
            }
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x150));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fake_call, FAKE, LOG_RESULTS};
    use crate::difftest::{DATA, GAME_GP};
    use crate::game::scripter::ssxscriptengine::tests::{base_14, check3, WORLD_4};

    /// The handler object (`self->t` at +4, three rows of ten words).
    const SELF: u32 = DATA + 0x6_0000;
    /// The game-mode manager (`cGameModeHandler_pManager`, `gp-0x480`).
    const MGR: u32 = DATA + 0x6_1000;
    /// The BE interface object and the BE state with its vtable.
    const IFACE: u32 = DATA + 0x6_2000;
    const BE_STATE: u32 = DATA + 0x6_3000;
    const BE_VT: u32 = DATA + 0x6_3100;
    /// The game (`world +0x84`) and the AI (`game +0xc`) with its rider list
    /// at +0x28 and the last computer index at +0x80.
    const GAME: u32 = WORLD_4 + 0x1000;
    const AI: u32 = WORLD_4 + 0x2000;
    /// The riders (`ai +0x28 + 4 * i`), their metrics and the player chain.
    const RIDER_BASE: u32 = DATA + 0x3_0000;
    const METRIX_BASE: u32 = DATA + 0x4_0000;
    const CHAIN_BASE: u32 = DATA + 0x5_0000;
    /// Two riders: the human and one computer.
    const RIDERS: u32 = 2;

    fn rider(i: u32) -> u32 {
        RIDER_BASE + 0x1000 * i
    }

    /// The BE interfaces answer through a fake vtable slot (the state object
    /// the BE library keeps is not in the test image).
    fn be_interfaces(c: &mut Cpu) {
        c.hooks.miss = fake_call;
        c.hooks.jump_out = crate::difftest::fakes::fake_jump_out;
        c.bus
            .write32(crate::difftest::fakes::LOG_UNWIND_AT, u32::MAX);
        for n in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * n, IFACE);
        }
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(0x0053_08a8, BE_STATE);
        c.bus.write32(BE_STATE + 4, BE_VT);
        c.bus.write16(BE_VT + 8, 0);
        c.bus.write32(BE_VT + 0xc, FAKE);
    }

    /// A scrambled machine with the BE library, the world chain and the
    /// manager; the finishing rider is `seed % 2`, the heat `1..=3`.
    fn gm0_finish_setup(c: &mut Cpu, seed: u32) {
        let mut r = base_14(c, seed, 0x0023_9230);
        be_interfaces(c);
        c.pc = 0x0023_9230;
        c.bus
            .write8(0x0053_5c11, [2u32, 0, 1][seed as usize % 3] as u8);
        // The manager.
        c.bus.write32(GAME_GP - 0x480, MGR);
        c.bus.write32(MGR, 1 + seed % 3);
        c.bus.write32(MGR + 0x10, RIDERS - 1);
        c.bus.write32(MGR + 0x70, 0);
        c.bus.write32(MGR + 0x74, 0);
        c.bus.write32(MGR + 0x9c, 0);
        // The world chain: world +0x84 -> game, game +0xc -> AI.
        c.bus.write32(WORLD_4 + 0x84, GAME);
        c.bus.write32(GAME + 0xc, AI);
        c.bus.write32(AI + 0x8, r.next_u32() % 4000);
        c.bus.write32(AI + 0x40, CHAIN_BASE);
        c.bus.write32(CHAIN_BASE + 0x18, CHAIN_BASE + 0x100);
        c.bus.write32(AI + 0x80, RIDERS - 1);
        for i in 0..RIDERS {
            let rd = rider(i);
            let metrix = METRIX_BASE + 0x1000 * i;
            c.bus.write32(AI + 0x28 + 4 * i, rd);
            c.bus.write32(rd + 0x790, metrix);
            c.bus.write32(rd + 0x40, CHAIN_BASE);
            c.bus.write32(rd + 0x100, 0);
            c.bus.write32(rd + 0x470, r.next_u32());
            let dnf = seed % 4 == 1 && i == seed % 2;
            c.bus
                .write32(rd + 0x480, if dnf { r.next_u32() | 1 } else { 0 });
            for w in 0..0x30u32 {
                c.bus.write32(metrix + 0xfc + 4 * w, r.next_u32() % 3000);
            }
            c.bus.write32(0x0053_6730 + 4 * i, r.next_u32() % 4);
        }
        c.g[A0][0] = sx(SELF);
        c.g[A1][0] = sx(seed % 2);
    }

    #[test]
    fn gm_handler0_on_rider_finished_matches() {
        check3(
            ee::s06::f_00239230__generated,
            gm_handler0_on_rider_finished,
            60,
            &gm0_finish_setup,
        );
    }
}
