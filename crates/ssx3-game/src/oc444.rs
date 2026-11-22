//! Staging module oc444 (wave wave223): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc444`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
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
        0x0015_4ab8,
        be_score_int_submit_race_result,
        "BEScoreInt_SubmitRaceResult",
    ),
    (
        0x0015_4ee8,
        be_score_int_submit_placement,
        "BEScoreInt_SubmitPlacement",
    ),
    (
        0x0020_09d0,
        event_select_on_widget_created,
        "EventSelect_OnWidgetCreated",
    ),
    (0x0020_2828, event_select_update, "EventSelect_Update"),
    (
        0x0020_a8f8,
        game_ui_open_screen_for_state,
        "GameUI_OpenScreenForState",
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
        0x0023_6da0,
        game_state_results_enter,
        "GameStateResults_Enter",
    ),
    (
        0x0023_8358,
        game_mode_man_on_rider_finished,
        "GameModeMan_OnRiderFinished",
    ),
    (0x0026_a180, path_read, "Path_Read"),
    (
        0x0027_ac60,
        script_engine_queue_race_cutscene,
        "ScriptEngine_QueueRaceCutscene",
    ),
    (
        0x0036_6e90,
        tex_man_copy_to_temp_tex,
        "TexMan_CopyToTempTex",
    ),
];

/// `BEScoreInt_SubmitRaceResult` (0x00154ab8): Records a finished race for one human rider: copies the 0xa0-byte race stats into the score object, updates the best stats and the per-event high score table (insert sorted, shift lower entries down, bump the other human's rank), then the records and the character/event profile score. `(self, rider, stats, score)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn be_score_int_submit_race_result(c: &mut Cpu) {
    guest(c, |c| {
        'b_154eb8: {
            'b_154eb4: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S1][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S4][0] = c.g[A1][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S3][0] = c.g[A3][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S5][0] = c.g[A2][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x0014a0e0, 0x00154afc)?; // BE_GetRiderProfile
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = 0xb0;
                if t {
                    break 'b_154eb4;
                } else {
                    'b_154bec: {
                        'b_154b9c: {
                            c.cycles += 7;
                            c.mult(0, 2, 18, 3);
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = c.g[S5][0] | c.g[V1][0];
                            c.g[V0][0] = c.g[V0][0] & 0x7;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S5][0];
                            if t {
                                break 'b_154b9c;
                            } else {
                                c.cycles += 8;
                                c.g[A2][0] = sx((c.g[S2][0] as u32) << 2);
                                c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[A2][0]);
                                c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0xa0));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xcc));
                                c.g[S6][0] = 0x440000;
                                c.g[FP][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
                                'l_154b48: loop {
                                    c.cycles += 19;
                                    c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                                    c.ldr(8, (c.g[A0][0] as u32));
                                    c.ldl(9, (c.g[A0][0] as u32).wrapping_add(0xf));
                                    c.ldr(9, (c.g[A0][0] as u32).wrapping_add(0x8));
                                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
                                    c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
                                    c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                    c.ldr(7, (c.g[A0][0] as u32).wrapping_add(0x18));
                                    c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                                    c.sdr(8, (c.g[V1][0] as u32));
                                    c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
                                    c.sdr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                                    c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                    let t = c.g[A0][0] != c.g[V0][0];
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00154b48);
                                        }
                                        continue 'l_154b48;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0xb0;
                                        if t {
                                            break 'b_154bec;
                                        } else {
                                            break 'b_154b9c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[T0][0] = sx((c.g[S2][0] as u32) << 2);
                        c.g[T1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[T0][0]);
                        c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0xa0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xcc));
                        c.g[S6][0] = 0x440000;
                        c.g[FP][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
                        'l_154bbc: loop {
                            c.cycles += 11;
                            c.g[A2][0] = c.ld((c.g[A0][0] as u32));
                            c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                            c.g[T1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                            c.sd((c.g[V1][0] as u32), c.g[A2][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            let t = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00154bbc);
                                }
                                continue 'l_154bbc;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = 0xb0;
                                break 'b_154bec;
                            }
                        }
                    }
                    'b_154d14: {
                        'b_154ccc: {
                            c.cycles += 53;
                            c.g[S0][0] = 0xffffffffffffffff;
                            c.mult(0, 6, 18, 2);
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                            c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.ldr(7, (c.g[A0][0] as u32));
                            c.g[T0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.sdr(7, (c.g[V1][0] as u32));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S5][0];
                            c.sw((c.g[V0][0] as u32), c.g[S0][0]);
                            call_ptr(c, 0x00154a58, 0x00154c24)?; // BEScoreInt_UpdateBestStats
                            call_ptr(c, 0x0014dc80, 0x00154c2c)?; // BELibrary_GetInstance
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x0014dd58, 0x00154c38)?; // BELibrary_GetInterface
                            c.g[A0][0] = c.g[S4][0];
                            call_ptr(c, 0x0014a080, 0x00154c40)?; // BE_GetRiderCharacter
                            c.g[A2][0] = 0x530000;
                            c.g[S7][0] = c.g[V0][0];
                            c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5b0));
                            c.g[A3][0] = 0x530000;
                            c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = 0x64;
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5bc8));
                            c.g[T0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x26b0));
                            c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                            c.mult(0, 4, 4, 3);
                            c.g[V1][0] = 0xf88;
                            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                            c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                            c.g[A1][0] = 0x9b50;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.mult(0, 3, 23, 3);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                            c.mult(0, 5, 18, 5);
                            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                            c.g[T0][0] = 0x4a0000;
                            c.g[S6][0] = c.lw((c.g[A0][0] as u32));
                            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x6ca8));
                            c.sw((c.g[A2][0] as u32), 0u64);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A3][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.sw((c.g[A3][0] as u32), c.g[S0][0]);
                            c.g[V0][0] = c.g[S6][0] ^ 0x16;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                            c.g[V1][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x48));
                            let t = c.g[V1][0] == 0u64;
                            if c.g[V0][0] == 0 {
                                c.g[S6][0] = 0u64;
                            }
                            if t {
                                break 'b_154ccc;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0x5;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A0][0] = c.g[S1][0];
                                if t {
                                    break 'b_154d14;
                                } else {
                                    break 'b_154ccc;
                                }
                            }
                        }
                        'b_154d00: {
                            'b_154ce8: {
                                c.cycles += 3;
                                c.g[T1][0] = 0x1;
                                let t = (c.g[S3][0] as i64) < 0;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                                if t {
                                    break 'b_154ce8;
                                } else {
                                    c.cycles += 4;
                                    c.mtc1(19, 1);
                                    c.cvt_s_w(1, 1);
                                    let t = true;

                                    if t {
                                        break 'b_154d00;
                                    } else {
                                        break 'b_154ce8;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[V1][0] = sx((c.g[S3][0] as u32) >> 1);
                            c.g[V0][0] = c.g[S3][0] & 0x1;
                            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                            c.mtc1(2, 1);
                            c.cvt_s_w(1, 1);
                            c.add_s(1, 1, 1);
                            break 'b_154d00;
                        }
                        c.cycles += 5;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6dec));
                        c.mul_s(0, 1, 0);
                        c.cvt_w_s(1, 0);
                        c.mfc1(19, 1);
                        c.g[A0][0] = c.g[S1][0];
                        break 'b_154d14;
                    }
                    'b_154e48: {
                        c.cycles += 4;
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x001557e0, 0x00154d1c)?; // BEScoreInt_CanSubmitScore
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                        if t {
                            break 'b_154e48;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x0014ab20, 0x00154d2c)?; // BEScoreInt_GetHighScoreTable
                            c.g[A2][0] = c.g[V0][0];
                            let t = c.g[A2][0] == 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_154e48;
                            } else {
                                c.cycles += 8;
                                c.g[S0][0] = 0u64;
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
                                c.g[S1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[A1][0] = c.g[A2][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                c.g[S2][0] =
                                    sx((c.g[FP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                'l_154d58: loop {
                                    'b_154d70: {
                                        'b_154d6c: {
                                            c.cycles += 3;
                                            c.g[V1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                            if t {
                                                break 'b_154d6c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V0][0] = (c.g[V0][0] < c.g[S3][0]) as u64;
                                                if t {
                                                    break 'b_154d70;
                                                } else {
                                                    break 'b_154d6c;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                                        break 'b_154d70;
                                    }
                                    'b_154e38: {
                                        c.cycles += 2;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            break 'b_154e38;
                                        } else {
                                            'b_154dc0: {
                                                c.cycles += 3;
                                                c.g[V0][0] = ((c.g[S0][0] as i64) < 4i64) as u64;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = 0x4;
                                                if t {
                                                    break 'b_154dc0;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V1][0] =
                                                        sx((c.g[A2][0] as u32).wrapping_add(0x50));
                                                    'l_154d88: loop {
                                                        c.cycles += 14;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        c.ldl(
                                                            6,
                                                            (c.g[V1][0] as u32).wrapping_sub(0xd),
                                                        );
                                                        c.ldr(
                                                            6,
                                                            (c.g[V1][0] as u32).wrapping_sub(0x14),
                                                        );
                                                        c.ldl(
                                                            7,
                                                            (c.g[V1][0] as u32).wrapping_sub(0x5),
                                                        );
                                                        c.ldr(
                                                            7,
                                                            (c.g[V1][0] as u32).wrapping_sub(0xc),
                                                        );
                                                        c.g[T0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x4));
                                                        c.sdl(
                                                            6,
                                                            (c.g[V1][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.sdr(6, (c.g[V1][0] as u32));
                                                        c.sdl(
                                                            7,
                                                            (c.g[V1][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.sdr(
                                                            7,
                                                            (c.g[V1][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x10),
                                                            c.g[T0][0],
                                                        );
                                                        c.g[V0][0] = ((c.g[S0][0] as i64)
                                                            < (c.g[A0][0] as i64))
                                                            as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[V1][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x14));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x00154d88);
                                                            }
                                                            continue 'l_154d88;
                                                        } else {
                                                            break 'b_154dc0;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 19;
                                            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S7][0]);
                                            c.sw((c.g[A1][0] as u32), c.g[S3][0]);
                                            call_ptr(c, 0x0014dc80, 0x00154dcc)?; // BELibrary_GetInstance
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A1][0] = 0x1;
                                            call_ptr(c, 0x0014dd58, 0x00154dd8)?; // BELibrary_GetInterface
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            call_ptr(c, 0x00147170, 0x00154de4)?; // BENewPlayerInt_GetName
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x004166f4, 0x00154df0)?; // call_indirect
                                            c.sw((c.g[S2][0] as u32), c.g[S0][0]);
                                            c.g[V1][0] = 0x530000;
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                                            c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                            if t {
                                                break 'b_154e48;
                                            } else {
                                                c.cycles += 5;
                                                c.g[A1][0] = sx((c.g[FP][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                                c.g[V0][0] = ((c.g[V1][0] as i64)
                                                    < (c.g[S0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_154e48;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V1][0] =
                                                        ((c.g[V0][0] as i64) < 5i64) as u64;
                                                    let t = c.g[V1][0] != 0u64;
                                                    c.sw((c.g[A1][0] as u32), c.g[V0][0]);
                                                    if t {
                                                        break 'b_154e48;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A2][0] = 0xffffffffffffffff;
                                                        let t = true;
                                                        c.sw((c.g[A1][0] as u32), c.g[A2][0]);
                                                        if t {
                                                            break 'b_154e48;
                                                        } else {
                                                            break 'b_154e38;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < 5i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00154d58);
                                        }
                                        continue 'l_154d58;
                                    } else {
                                        break 'b_154e48;
                                    }
                                }
                            }
                        }
                    }
                    'b_154e80: {
                        'b_154e6c: {
                            c.cycles += 3;
                            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                            let t = c.g[A3][0] == 0u64;
                            c.g[A0][0] = c.g[S4][0];
                            if t {
                                break 'b_154e6c;
                            } else {
                                c.cycles += 6;
                                c.g[A1][0] = c.g[S5][0];
                                c.g[A2][0] = 0xffffffffffffffff;
                                c.g[A3][0] = c.g[S3][0];
                                call_ptr(c, 0x00155420, 0x00154e64)?; // BEScoreInt_UpdateRecords
                                let t = true;
                                c.g[T0][0] = 0x530000;
                                if t {
                                    break 'b_154e80;
                                } else {
                                    break 'b_154e6c;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S5][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[A3][0] = 0xffffffffffffffff;
                        call_ptr(c, 0x00155420, 0x00154e7c)?; // BEScoreInt_UpdateRecords
                        c.g[T0][0] = 0x530000;
                        break 'b_154e80;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5b0));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    let t = c.g[V1][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        break 'b_154eb8;
                    } else {
                        c.cycles += 9;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[A1][0] = c.g[S6][0];
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32));
                        c.g[A3][0] = c.g[S3][0];
                        call_ptr(c, 0x00152460, 0x00154ea4)?; // BE_RecordCharProfileEventScore
                        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                        break 'b_154eb4;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            break 'b_154eb8;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        Ok(())
    })
}

/// `BEScoreInt_SubmitPlacement` (0x00154ee8): Records a race placement for one rider: stores the result in the score object, converts it to a medal, notifies the event-result hooks and, unless the race is a replay or a practice run, reports the (medal, best time/score) to the reward interface. `(self, rider, result, score)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn be_score_int_submit_placement(c: &mut Cpu) {
    guest(c, |c| {
        'b_1550bc: {
            'b_1550b8: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S0][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S1][0] = c.g[A0][0];
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A2][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S6][0] = c.g[A3][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0014a0e0, 0x00154f2c)?; // BE_GetRiderProfile
                c.g[S5][0] = c.g[V0][0];
                c.g[V0][0] = ((c.g[S5][0] as i64) < 2i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0xb0;
                if t {
                    break 'b_1550b8;
                } else {
                    'b_154fc4: {
                        c.cycles += 31;
                        c.g[V1][0] = 0x530000;
                        c.mult(0, 4, 21, 2);
                        c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b0));
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0xcc), c.g[S2][0]);
                        call_ptr(c, 0x0014dc80, 0x00154f54)?; // BELibrary_GetInstance
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dd58, 0x00154f60)?; // BELibrary_GetInterface
                        c.g[A0][0] = c.g[S0][0];
                        c.g[S7][0] = c.g[V0][0];
                        call_ptr(c, 0x0014a080, 0x00154f6c)?; // BE_GetRiderCharacter
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                        c.g[A0][0] = 0x64;
                        c.g[FP][0] = c.g[V0][0];
                        c.g[A1][0] = 0x530000;
                        c.mult(0, 3, 3, 4);
                        c.g[V0][0] = 0x440000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x26b0));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[S1][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x5c12));
                        c.g[A1][0] = c.g[S2][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[S3][0] = c.lw((c.g[V1][0] as u32));
                        c.g[V0][0] = c.g[S3][0] ^ 0x16;
                        if c.g[V0][0] == 0 {
                            c.g[S3][0] = 0u64;
                        }
                        call_ptr(c, 0x00155328, 0x00154fa8)?; // BEScoreInt_PlaceToMedal
                        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
                        c.g[V1][0] = (c.g[V1][0] < 0x2u64) as u64;
                        let t = c.g[V1][0] == 0u64;
                        c.g[S0][0] = c.g[V0][0];
                        if t {
                            break 'b_154fc4;
                        } else {
                            c.cycles += 3;
                            c.g[S0][0] = 0xffffffffffffffff;
                            c.g[V0][0] = 0x1;
                            if c.g[S2][0] == 0 {
                                c.g[S0][0] = c.g[V0][0];
                            }
                            break 'b_154fc4;
                        }
                    }
                    'b_154fe4: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lb((c.g[S4][0] as u32).wrapping_add(0x49));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0xffffffffffffffff;
                        if t {
                            break 'b_154fe4;
                        } else {
                            c.cycles += 5;
                            c.g[A2][0] = c.g[S2][0];
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S3][0];
                            call_ptr(c, 0x001e25d8, 0x00154fe0)?; // call_indirect
                            c.g[V0][0] = 0xffffffffffffffff;
                            break 'b_154fe4;
                        }
                    }
                    c.cycles += 2;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        break 'b_1550bc;
                    } else {
                        'b_155024: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lb((c.g[S4][0] as u32).wrapping_add(0x49));
                            let t = c.g[V0][0] != 0u64;

                            if t {
                                break 'b_155024;
                            } else {
                                c.cycles += 5;
                                call_ptr(c, 0x001e1eb8, 0x00155000)?; // call_indirect
                                c.g[V0][0] = c.lb((c.g[S4][0] as u32).wrapping_add(0x49));
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x4;
                                if t {
                                    break 'b_155024;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x5;
                                    if t {
                                        break 'b_155024;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == c.g[V0][0];

                                        if t {
                                            break 'b_155024;
                                        } else {
                                            c.cycles += 2;
                                            call_ptr(c, 0x001e1550, 0x00155024)?; // call_indirect
                                            break 'b_155024;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 11;
                        call_ptr(c, 0x0014dc80, 0x0015502c)?; // BELibrary_GetInstance
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0xd;
                        call_ptr(c, 0x0014dd58, 0x00155038)?; // BELibrary_GetInterface
                        c.g[A0][0] = 0x530000;
                        c.g[S4][0] = c.g[V0][0];
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b0));
                        c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                            break 'b_1550bc;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[S2][0] = 0x4;
                            let t = c.g[V0][0] == c.g[S2][0];
                            c.g[A0][0] = c.g[S7][0];
                            if t {
                                break 'b_1550b8;
                            } else {
                                'b_1550a0: {
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[A2][0] = c.g[S1][0];
                                    call_ptr(c, 0x001456a0, 0x0015506c)?; // BENewRaceInt_GetEventRecordTarget
                                    c.g[A3][0] = c.g[V0][0];
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    let t = c.g[A3][0] == c.g[V0][0];
                                    c.g[A0][0] = c.g[S4][0];
                                    if t {
                                        break 'b_1550a0;
                                    } else {
                                        'b_155098: {
                                            'b_155090: {
                                                c.cycles += 2;
                                                if c.g[S1][0] == 0u64 {
                                                    c.g[V0][0] = 0x3c;
                                                    break 'b_155090;
                                                } else {
                                                    c.cycles += 2;
                                                    if c.g[S1][0] != c.g[S2][0] {
                                                        c.g[V0][0] = ((c.g[S6][0] as i64)
                                                            < (c.g[A3][0] as i64))
                                                            as u64;
                                                        break 'b_155098;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = 0x3c;
                                                        break 'b_155090;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.mult(0, 2, 7, 2);
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                                            break 'b_155098;
                                        }
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0 {
                                            c.g[S0][0] = 0u64;
                                        }
                                        c.g[A0][0] = c.g[S4][0];
                                        break 'b_1550a0;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A1][0] = c.g[S5][0];
                                c.g[A2][0] = c.g[FP][0];
                                c.g[A3][0] = c.g[S1][0];
                                c.g[T0][0] = c.g[S3][0];
                                c.g[T1][0] = c.g[S0][0];
                                call_ptr(c, 0x001591e8, 0x001550b8)?; // BERewardInt_OnEventResult
                                break 'b_1550b8;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            break 'b_1550bc;
        }
        c.cycles += 11;
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
        Ok(())
    })
}

/// `FEAsyncManager_LoadSlot` (0x001a3cd8): Starts loading the texture file of async slot `idx` (slot stride 0x11c, buffer 0x109a0 bytes from the FE async pool): synchronous slots are loaded at once (state 3), asynchronous ones get a load handle (state 2). `(self, idx)`.
/// NOT INSTALLED (no OVERRIDES entry, no test): EAFile/AsyncFile request layer (pc 0x3de768) has no override slot and spins on test data.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn fe_async_manager_load_slot(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a3da8: {
            'b_1a3d88: {
                c.cycles += 34;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.g[V0][0] = 0x11c;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = 0u64;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S4][0] = c.g[A1][0];
                c.mult(0, 17, 20, 2);
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S0][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10c));
                c.g[V0][0] = 0x10000;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x100));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[S3][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] | 0x9a0;
                c.g[A1][0] = 0x460000;
                c.g[A0][0] = 0x10000;
                c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1430));
                c.g[A0][0] = c.g[A0][0] | 0x9a0;
                c.g[A2][0] = 0x25000000;
                c.sw((c.g[S3][0] as u32), c.g[V0][0]);
                call_ptr(c, 0x00317e30, 0x001a3d3c)?; // call_indirect
                c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S4][0];
                c.g[A2][0] = 0x2;
                call_ptr(c, 0x001a39d8, 0x001a3d50)?; // FEAsyncManager_SetState
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x108));
                let t = c.g[V1][0] == 0u64;
                c.g[A1][0] = c.g[S4][0];
                if t {
                    break 'b_1a3d88;
                } else {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = 0x2;
                    call_ptr(c, 0x001a39d8, 0x001a3d6c)?; // FEAsyncManager_SetState
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32));
                    call_ptr(c, 0x003df748, 0x001a3d7c)?; // call_indirect
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    let t = true;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    if t {
                        break 'b_1a3da8;
                    } else {
                        break 'b_1a3d88;
                    }
                }
            }
            c.cycles += 8;
            c.g[A1][0] = c.lw((c.g[S2][0] as u32));
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
            c.g[A2][0] = c.lw((c.g[S3][0] as u32));
            call_ptr(c, 0x003e1bb8, 0x001a3d98)?; // call_indirect
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = 0x3;
            call_ptr(c, 0x001a39d8, 0x001a3da8)?; // FEAsyncManager_SetState
            break 'b_1a3da8;
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    })
}

/// `EventSelect_OnWidgetCreated` (0x002009d0): Widget-created hook of the event-select map: when the widget's label is the peak list, selects menu entry 2 (map style < 2) or `2 - location.peak`. `(self, menu)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn event_select_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_200a58: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            call_ptr(c, 0x0014dc80, 0x002009ec)?; // call_indirect
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x002009f8)?; // call_indirect
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
            call_ptr(c, 0x00317670, 0x00200a0c)?; // call_indirect
            let t = c.g[S0][0] != c.g[V0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_200a58;
            } else {
                'b_200a34: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        break 'b_200a34;
                    } else {
                        c.cycles += 4;
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x0039b960, 0x00200a2c)?; // call_indirect
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_200a58;
                        } else {
                            break 'b_200a34;
                        }
                    }
                }
                c.cycles += 9;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00144bc0, 0x00200a3c)?; // call_indirect
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x54));
                c.g[A1][0] = 0x2;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.g[A1][0] = c.g[A1][0] & 0xff;
                call_ptr(c, 0x0039b960, 0x00200a54)?; // call_indirect
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_200a58;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `EventSelect_Update` (0x00202828): Per-frame update of the event-select map widget: fades the map picture in, shows or hides the peak outlines / indicators and the "you are here" markers, and applies the async-loaded map texture once its slot is loaded. `(self)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn event_select_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_2029bc: {
            'b_20294c: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x00202850)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x0020285c)?; // call_indirect
                c.g[S5][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2dc));
                if c.g[V0][0] == 0u64 {
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2e0), 0u64);
                    break 'b_20294c;
                } else {
                    'b_202898: {
                        c.cycles += 7;
                        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x2e0));
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.c_lt_s(1, 0);
                        if !c.fcc() {
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e4));
                            break 'b_202898;
                        } else {
                            c.cycles += 4;
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5530));
                            c.add_s(0, 1, 0);
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2e0));
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e4));
                            break 'b_202898;
                        }
                    }
                    'b_202938: {
                        c.cycles += 26;
                        c.g[A1][0] = c.g[SP][0];
                        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x2e0));
                        c.ldl(2, (c.g[A0][0] as u32).wrapping_add(0x23));
                        c.ldr(2, (c.g[A0][0] as u32).wrapping_add(0x1c));
                        c.ldl(3, (c.g[A0][0] as u32).wrapping_add(0x2b));
                        c.ldr(3, (c.g[A0][0] as u32).wrapping_add(0x24));
                        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(2, (c.g[SP][0] as u32));
                        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(0, (c.g[SP][0] as u32));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                        let t = c.g[A2][0] as u32;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        call_ptr(c, t, 0x002028d8)?; // call_indirect
                        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e4));
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x002028f4)?; // call_indirect
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                        if c.g[V0][0] != 0u64 {
                            c.g[A1][0] = 0x470000;
                            break 'b_202938;
                        } else {
                            c.cycles += 14;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                            c.g[A1][0] = 0x470000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x588));
                            call_ptr(c, 0x0039e948, 0x00202910)?; // call_indirect
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                            c.g[A1][0] = 0x470000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x598));
                            call_ptr(c, 0x0039e948, 0x00202920)?; // call_indirect
                            c.g[A1][0] = 0x470000;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5a8));
                            call_ptr(c, 0x0039e948, 0x00202930)?; // call_indirect
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2f0));
                            if t {
                                break 'b_2029bc;
                            } else {
                                break 'b_202938;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a0));
                    call_ptr(c, 0x0039e948, 0x00202944)?; // call_indirect
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2f0));
                    if t {
                        break 'b_2029bc;
                    } else {
                        break 'b_20294c;
                    }
                }
            }
            'b_2029ac: {
                c.cycles += 10;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e4));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00202968)?; // call_indirect
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                if c.g[V0][0] != 0u64 {
                    c.g[A1][0] = 0x470000;
                    break 'b_2029ac;
                } else {
                    c.cycles += 14;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                    c.g[A1][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x588));
                    call_ptr(c, 0x0039e8b8, 0x00202984)?; // call_indirect
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                    c.g[A1][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x598));
                    call_ptr(c, 0x0039e8b8, 0x00202994)?; // call_indirect
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5a8));
                    call_ptr(c, 0x0039e8b8, 0x002029a4)?; // call_indirect
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2f0));
                    if t {
                        break 'b_2029bc;
                    } else {
                        break 'b_2029ac;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a0));
            call_ptr(c, 0x0039e8b8, 0x002029b8)?; // call_indirect
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2f0));
            break 'b_2029bc;
        }
        'b_202ab8: {
            c.cycles += 2;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                break 'b_202ab8;
            } else {
                c.cycles += 5;
                c.g[S1][0] = 0u64;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S4][0] = 0x470000;
                c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                'l_2029d8: loop {
                    'b_202a4c: {
                        'b_202a40: {
                            'b_202a3c: {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[S3][0] = c.g[A2][0];
                                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x7b0));
                                call_ptr(c, 0x00417828, 0x002029e8)?; // call_indirect
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2dc));
                                let t = c.g[V1][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                                if t {
                                    break 'b_202a3c;
                                } else {
                                    c.cycles += 2;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                                        break 'b_202a40;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S5][0];
                                        call_ptr(c, 0x00144bc0, 0x00202a04)?; // call_indirect
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x54));
                                        if c.g[S1][0] != c.g[V1][0] {
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                                            break 'b_202a40;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                                            if t {
                                                break 'b_202a40;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                                                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                                let t = c.g[V0][0] != 0u64;

                                                if t {
                                                    break 'b_202a40;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[A1][0] = c.g[S2][0];
                                                    call_ptr(c, 0x0039e948, 0x00202a34)?; // call_indirect
                                                    let t = true;
                                                    c.g[S1][0] = c.g[S3][0];
                                                    if t {
                                                        break 'b_202a4c;
                                                    } else {
                                                        break 'b_202a3c;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                            break 'b_202a40;
                        }
                        c.cycles += 3;
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x0039e8b8, 0x00202a48)?; // call_indirect
                        c.g[S1][0] = c.g[S3][0];
                        break 'b_202a4c;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002029d8);
                        }
                        continue 'l_2029d8;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x001a39c0, 0x00202a6c)?; // call_indirect
                        c.g[V1][0] = 0x3;
                        if c.g[V0][0] != c.g[V1][0] {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                            break 'b_202ab8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2dc));
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                break 'b_202ab8;
                            } else {
                                'b_202ab4: {
                                    c.cycles += 3;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e4));
                                    let t = c.g[A2][0] == 0u64;
                                    c.g[V1][0] = 0x11c;
                                    if t {
                                        break 'b_202ab4;
                                    } else {
                                        c.cycles += 9;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                                        c.g[A1][0] = 0x1;
                                        c.mult(0, 4, 2, 3);
                                        c.g[V0][0] =
                                            sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[S1][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x110));
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x7c), 0u64);
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x2dc), c.g[A1][0]);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x2e0), 0u64);
                                        break 'b_202ab4;
                                    }
                                }
                                c.cycles += 1;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                break 'b_202ab8;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    })
}

/// `GameUI_OpenScreenForState` (0x0020a8f8): Picks the overlay screen to open for a game-mode change (2: in-race pause/restart screen, 7: results) from the event setup, rewards and network state, then opens it (or sets the overlay status to 2). `(mode)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_ui_open_screen_for_state(c: &mut Cpu) {
    guest(c, |c| {
        'b_20a934: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw((c.g[A0][0] as u32));
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_20a934;
            } else {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                break 'b_20a934;
            }
        }
        'b_20ab30: {
            c.cycles += 2;
            let t = c.g[A1][0] != 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_20ab30;
            } else {
                'b_20ab2c: {
                    'b_20ab14: {
                        'b_20a9ec: {
                            'b_20a98c: {
                                c.cycles += 15;
                                c.g[S3][0] = 0u64;
                                call_ptr(c, 0x0014dc80, 0x0020a944)?; // call_indirect
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x0014dd58, 0x0020a950)?; // call_indirect
                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[V0][0] = 0x530000;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                                c.g[A0][0] = 0x2;
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc0));
                                c.g[S1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                                c.g[S4][0] = c.lw((c.g[V1][0] as u32));
                                c.g[S0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                                let t = c.g[S2][0] == c.g[A0][0];
                                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                                if t {
                                    break 'b_20a98c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x7;
                                    if c.g[S2][0] == c.g[V0][0] {
                                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x78));
                                        break 'b_20a9ec;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_20ab14;
                                        } else {
                                            break 'b_20a98c;
                                        }
                                    }
                                }
                            }
                            'b_20a9a0: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1ec8));
                                let t = c.g[V0][0] == 0u64;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9ec), 0u64);
                                if t {
                                    break 'b_20a9a0;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S3][0] = 0x11;
                                    if t {
                                        break 'b_20ab14;
                                    } else {
                                        break 'b_20a9a0;
                                    }
                                }
                            }
                            'b_20a9b4: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x4;
                                let t = c.g[S0][0] != c.g[V0][0];

                                if t {
                                    break 'b_20a9b4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[S2][0]);
                                    if t {
                                        break 'b_20ab14;
                                    } else {
                                        break 'b_20a9b4;
                                    }
                                }
                            }
                            'b_20a9c4: {
                                c.cycles += 2;
                                let t = c.g[S1][0] != 0u64;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), 0u64);
                                if t {
                                    break 'b_20a9c4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S3][0] = 0x8;
                                    if t {
                                        break 'b_20ab14;
                                    } else {
                                        break 'b_20a9c4;
                                    }
                                }
                            }
                            'b_20a9d4: {
                                c.cycles += 2;
                                let t = c.g[V1][0] != c.g[S2][0];
                                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
                                if t {
                                    break 'b_20a9d4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S3][0] = 0x15;
                                    if t {
                                        break 'b_20ab14;
                                    } else {
                                        break 'b_20a9d4;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[V1][0] = 0x7;
                            c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                            c.g[S3][0] = 0x9;
                            c.g[V0][0] = 0x14;
                            let t = true;
                            if c.g[V1][0] == 0 {
                                c.g[S3][0] = c.g[V0][0];
                            }
                            if t {
                                break 'b_20ab14;
                            } else {
                                break 'b_20a9ec;
                            }
                        }
                        'b_20aa08: {
                            c.cycles += 5;
                            c.g[A1][0] = 0x7;
                            call_ptr(c, 0x0014dd58, 0x0020a9f4)?; // call_indirect
                            c.g[V1][0] = 0x4;
                            let t = c.g[S0][0] != c.g[V1][0];

                            if t {
                                break 'b_20aa08;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[S0][0]);
                                if t {
                                    break 'b_20ab2c;
                                } else {
                                    break 'b_20aa08;
                                }
                            }
                        }
                        'b_20aa50: {
                            'b_20aa4c: {
                                c.cycles += 14;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), 0u64);
                                call_ptr(c, 0x0014dc80, 0x0020aa10)?; // call_indirect
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x8;
                                call_ptr(c, 0x0014dd58, 0x0020aa1c)?; // call_indirect
                                c.g[S0][0] = c.g[V0][0];
                                call_ptr(c, 0x0014dc80, 0x0020aa24)?; // call_indirect
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0xd;
                                call_ptr(c, 0x0014dd58, 0x0020aa30)?; // call_indirect
                                c.g[S2][0] = c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                let t = (c.g[V0][0] as i64) >= 0;
                                c.g[V1][0] = 0u64;
                                if t {
                                    break 'b_20aa4c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                    let t = (c.g[V0][0] as i64) < 0;

                                    if t {
                                        break 'b_20aa50;
                                    } else {
                                        break 'b_20aa4c;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V1][0] = 0x1;
                            break 'b_20aa50;
                        }
                        'b_20aab8: {
                            'b_20aa84: {
                                c.cycles += 2;
                                let t = c.g[V1][0] == 0u64;
                                c.g[V0][0] = 0x530000;
                                if t {
                                    break 'b_20aa84;
                                } else {
                                    'b_20aa78: {
                                        c.cycles += 3;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4b30));
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[S0][0] = c.g[V0][0];
                                        if t {
                                            break 'b_20aa78;
                                        } else {
                                            c.cycles += 5;
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A0][0] = c.g[S1][0];
                                            call_ptr(c, 0x0020a8b0, 0x0020aa70)?; // GameUI_GetResultsScreenId
                                            let t = true;
                                            c.g[S3][0] = c.g[V0][0];
                                            if t {
                                                break 'b_20aab8;
                                            } else {
                                                break 'b_20aa78;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f0), 0u64);
                                    let t = true;
                                    c.g[S3][0] = 0xf;
                                    if t {
                                        break 'b_20aab8;
                                    } else {
                                        break 'b_20aa84;
                                    }
                                }
                            }
                            'b_20aaa8: {
                                'b_20aaa0: {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S2][0];
                                    call_ptr(c, 0x00158f30, 0x0020aa8c)?; // call_indirect
                                    let t = (c.g[V0][0] as i64) > 0;
                                    c.g[S3][0] = 0x10;
                                    if t {
                                        break 'b_20aaa0;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A1][0] = c.g[S4][0];
                                        if t {
                                            break 'b_20aaa8;
                                        } else {
                                            break 'b_20aaa0;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[S0][0] = 0x530000;
                                if t {
                                    break 'b_20aab8;
                                } else {
                                    break 'b_20aaa8;
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x0020a8b0, 0x0020aab0)?; // GameUI_GetResultsScreenId
                            c.g[S0][0] = 0x530000;
                            c.g[S3][0] = c.g[V0][0];
                            break 'b_20aab8;
                        }
                        'b_20aaec: {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[S1][0] ^ 0x6;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = 0u64;
                            if t {
                                break 'b_20aaec;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[S1][0] ^ 0x7;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.g[S1][0] ^ 0x8;
                                if t {
                                    break 'b_20aaec;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.g[S1][0] ^ 0x9;
                                    if t {
                                        break 'b_20aaec;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.g[S1][0] ^ 0xa;
                                        if t {
                                            break 'b_20aaec;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = c.g[S1][0] ^ 0xb;
                                            if t {
                                                break 'b_20aaec;
                                            } else {
                                                c.cycles += 1;
                                                c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
                                                break 'b_20aaec;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        'b_20ab00: {
                            c.cycles += 2;
                            let t = c.g[A0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            if t {
                                break 'b_20ab00;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                                call_ptr(c, 0x002706b8, 0x0020ab00)?; // call_indirect
                                break 'b_20ab00;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4b30));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                        if t {
                            break 'b_20ab14;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = 0x8ca0;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                            break 'b_20ab14;
                        }
                    }
                    'b_20ab24: {
                        c.cycles += 2;
                        let t = c.g[S3][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_20ab24;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
                            if t {
                                break 'b_20ab2c;
                            } else {
                                break 'b_20ab24;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0020ab50, 0x0020ab2c)?; // GameUI_OpenScreen
                    break 'b_20ab2c;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_20ab30;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
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
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
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
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `cGameLoadScreen_LoadPictures` (0x00233048): Loads the load-screen pictures from the big file: the two character pictures in a two-player race, else the course picture. Stores the textures at self+0x10. `(self)`.
/// NOT INSTALLED (no OVERRIDES entry, no test): EAFile_LoadDecompressed/BigFile loads (pc 0x3de768) have no override slot and spin on test data.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn c_game_load_screen_load_pictures(c: &mut Cpu) {
    guest(c, |c| {
        'b_233238: {
            'b_233180: {
                c.cycles += 26;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[RA][0]);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x00233078)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x00233084)?; // call_indirect
                c.g[S3][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x0023308c)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00233098)?; // call_indirect
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = 0x2;
                c.g[V0][0] = 0x530000;
                c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                let t = c.g[A0][0] != c.g[V1][0];
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                if t {
                    break 'b_233180;
                } else {
                    c.cycles += 12;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A1][0] = 0x480000;
                    c.g[S7][0] = c.g[A0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4288));
                    c.g[A2][0] = 0x100;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x003168b0, 0x002330cc)?; // call_indirect
                    c.g[S1][0] = 0u64;
                    c.g[V0][0] = 0x440000;
                    c.g[S5][0] = 0x480000;
                    c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2928));
                    c.g[S4][0] = 0x480000;
                    'l_2330e0: loop {
                        'b_23315c: {
                            c.cycles += 29;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = (0u64 < c.g[S1][0]) as u64;
                            call_ptr(c, 0x00147488, 0x002330ec)?; // call_indirect
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x5b4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S6][0] as u32)));
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x4270));
                            call_ptr(c, 0x00417828, 0x00233108)?; // call_indirect
                            c.g[A1][0] = 0x3000000;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = c.g[A1][0] | 0x100;
                            call_ptr(c, 0x003e2190, 0x00233118)?; // call_indirect
                            c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x4260));
                            c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                            c.g[T0][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                            c.g[T1][0] = 0xffffffffffffffff;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x0023314c)?; // call_indirect
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                            if t {
                                break 'b_23315c;
                            } else {
                                c.cycles += 2;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00317e98, 0x0023315c)?; // call_indirect
                                break 'b_23315c;
                            }
                        }
                        c.cycles += 4;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002330e0);
                            }
                            continue 'l_2330e0;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S7][0];
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x003168e8, 0x00233178)?; // call_indirect
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                            if t {
                                break 'b_233238;
                            } else {
                                break 'b_233180;
                            }
                        }
                    }
                }
            }
            'b_233228: {
                c.cycles += 40;
                c.g[A1][0] = 0x480000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4250));
                c.g[A2][0] = 0x100;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x003168b0, 0x00233194)?; // call_indirect
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00144be0, 0x0023319c)?; // call_indirect
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[A2][0] = 0x440000;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2950));
                c.g[V1][0] = ((c.g[V0][0] as i64) < 17i64) as u64;
                c.g[A1][0] = 0x4a0000;
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = 0u64;
                }
                c.g[A0][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c30));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                call_ptr(c, 0x00417828, 0x002331d0)?; // call_indirect
                c.g[A1][0] = 0x3000000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[A1][0] | 0x100;
                call_ptr(c, 0x003e2190, 0x002331e0)?; // call_indirect
                c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A2][0] = 0x480000;
                c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4238));
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[A3][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                c.g[T0][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                c.g[T1][0] = 0xffffffffffffffff;
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00233218)?; // call_indirect
                let t = c.g[S0][0] == 0u64;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                if t {
                    break 'b_233228;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317e98, 0x00233228)?; // call_indirect
                    break 'b_233228;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x003168e8, 0x00233234)?; // call_indirect
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            break 'b_233238;
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        Ok(())
    })
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
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
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
                            c.g[A0][0] = c.lw((c.g[A0][0] as u32));
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
                                        sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
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
                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
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
                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
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
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                call_ptr(c, t, 0x00233e54)?; // call_indirect
                                                c.lqc2(3, (c.g[V0][0] as u32));
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
                                            c.sw((c.g[S3][0] as u32), c.g[V0][0]);
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
                                        sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
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
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
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
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
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
            c.g[S3][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
            c.g[S4][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x00144d98, 0x00234290)?; // call_indirect
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x002342a4)?; // call_indirect
            let t = (c.g[S3][0] as i64) <= 0;
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
            if t {
                break 'b_2342ec;
            } else {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                'l_2342b8: loop {
                    c.cycles += 13;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x28));
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x00147338, 0x002342c8)?; // call_indirect
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x001473d0, 0x002342e0)?; // call_indirect
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S4][0] as u32)));
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
                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[A0][0] as u32)));
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
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
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
            ee::s06::f_002343b0__generated(c);
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
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
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
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
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
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.pc = t;
            return Ok(());
        }
        Ok(())
    })
}

/// `GameStateReset_Enter` (0x002355c0): Enter handler of the Reset state: clears the user awards, picks the next state from the course and race mode, restarts the mission scripts and the audio, and plays the transport sequence lists. `(self)`.
/// NOT INSTALLED (no OVERRIDES entry, no test): a real unported callee frees fixture memory (DLHeap_Consolidate runaway); needs a faithful object graph.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_state_reset_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_2355fc: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), 0u64);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            call_ptr(c, 0x0027aa80, 0x002355f0)?; // call_indirect
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_2355fc;
            } else {
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                break 'b_2355fc;
            }
        }
        'b_235684: {
            'b_235674: {
                'b_235670: {
                    c.cycles += 17;
                    call_ptr(c, 0x0014dc80, 0x00235604)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x00235610)?; // call_indirect
                    c.g[S2][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x00235618)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0xd;
                    call_ptr(c, 0x0014dd58, 0x00235624)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00158e30, 0x0023562c)?; // call_indirect
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[S3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    let t = c.g[S3][0] != 0u64;
                    c.g[S4][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    if t {
                        break 'b_235670;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1bc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_235674;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = 0x4;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                            let t = true;
                            c.sw((c.g[S1][0] as u32), c.g[V0][0]);
                            if t {
                                break 'b_235684;
                            } else {
                                break 'b_235670;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'b_235674;
            }
            c.cycles += 4;
            c.g[V1][0] = (c.g[S3][0] < 0x1u64) as u64;
            c.sw((c.g[S1][0] as u32), c.g[V0][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
            c.g[V0][0] = 0x4;
            break 'b_235684;
        }
        'b_2356a0: {
            c.cycles += 2;
            if c.g[S4][0] != c.g[V0][0] {
                c.g[A0][0] = 0u64;
                break 'b_2356a0;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                c.g[A1][0] = 0x3;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x00113b10, 0x0023569c)?; // call_indirect
                c.g[A0][0] = 0u64;
                break 'b_2356a0;
            }
        }
        'b_23571c: {
            c.cycles += 13;
            c.g[S5][0] = 0x1;
            call_ptr(c, 0x0014de68, 0x002356a8)?; // call_indirect
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00309030, 0x002356b0)?; // call_indirect
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00309f18, 0x002356b8)?; // call_indirect
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x00144be0, 0x002356c0)?; // call_indirect
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x00144cc0, 0x002356cc)?; // call_indirect
            let t = c.g[V0][0] != c.g[S5][0];
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            if t {
                break 'b_23571c;
            } else {
                c.cycles += 18;
                call_ptr(c, 0x0014dc80, 0x002356dc)?; // call_indirect
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x002356e8)?; // call_indirect
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00144be0, 0x002356f4)?; // call_indirect
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00146e10, 0x00235704)?; // call_indirect
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00235718)?; // call_indirect
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                break 'b_23571c;
            }
        }
        'b_2357b0: {
            'b_235798: {
                c.cycles += 11;
                c.g[V1][0] = c.g[S4][0] ^ 0x4;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = 0u64;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
                call_ptr(c, 0x001f36d8, 0x00235740)?; // call_indirect
                if c.g[S3][0] == 0u64 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    break 'b_235798;
                } else {
                    c.cycles += 7;
                    c.sw((c.g[S1][0] as u32), c.g[S5][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                    c.g[V0][0] = c.g[V0][0] ^ 0x2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        break 'b_2357b0;
                    } else {
                        c.cycles += 8;
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0x1;
                        call_ptr(c, 0x00278f68, 0x00235770)?; // call_indirect
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                        c.g[V0][0] = c.g[V0][0] ^ 0x2;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_2357b0;
                        } else {
                            c.cycles += 5;
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0x1;
                            call_ptr(c, 0x00278f68, 0x00235790)?; // call_indirect
                            let t = true;

                            if t {
                                break 'b_2357b0;
                            } else {
                                break 'b_235798;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            if t {
                break 'b_2357b0;
            } else {
                c.cycles += 4;
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0x1;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00278f68, 0x002357b0)?; // call_indirect
                break 'b_2357b0;
            }
        }
        c.cycles += 17;
        call_ptr(c, 0x0028b180, 0x002357b8)?; // call_indirect
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
        call_ptr(c, 0x002b3a98, 0x002357c0)?; // call_indirect
        call_ptr(c, 0x0028b180, 0x002357c8)?; // call_indirect
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002a4b68, 0x002357d0)?; // call_indirect
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `GameStateRestartRace_Enter` (0x00235aa0): Enter handler of the NextHeat (restart race) state: resets the BE phase, fails the active local mission, restarts the heat, copies the AI characters into the player interface, queues the start sequence and shows the load indicator. `(self)`.
/// NOT INSTALLED (no OVERRIDES entry, no test): a real unported callee runs memcpy (0x4160e8) with a length from fixture data and runs away.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_state_restart_race_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_235c98: {
            'b_235b64: {
                'b_235b5c: {
                    'b_235b58: {
                        c.cycles += 40;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S7][0] = c.g[A0][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        c.g[A0][0] = 0u64;
                        call_ptr(c, 0x0014de68, 0x00235ad8)?; // call_indirect
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                        call_ptr(c, 0x0030b7f8, 0x00235ae0)?; // call_indirect
                        call_ptr(c, 0x0014dc80, 0x00235ae8)?; // call_indirect
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0014dd58, 0x00235af4)?; // call_indirect
                        c.g[S0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dc80, 0x00235afc)?; // call_indirect
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dd58, 0x00235b08)?; // call_indirect
                        c.g[S5][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[S6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                        c.g[A0][0] = c.g[S6][0];
                        call_ptr(c, 0x002382d8, 0x00235b1c)?; // GameModeMan_ResetMode
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        call_ptr(c, 0x00230180, 0x00235b28)?; // GameModule_ResetRace
                        c.g[V0][0] = 0x530000;
                        c.g[V1][0] = 0x5;
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                        c.g[A0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x48));
                        let t = c.g[A0][0] == c.g[V1][0];
                        c.g[FP][0] = c.g[V0][0];
                        if t {
                            break 'b_235b58;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x6;
                            let t = c.g[A0][0] == c.g[V0][0];
                            c.g[V0][0] = 0x3;
                            if t {
                                break 'b_235b5c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x49));
                                let t = c.g[V0][0] == 0u64;
                                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                if t {
                                    break 'b_235b64;
                                } else {
                                    break 'b_235b58;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x3;
                    break 'b_235b5c;
                }
                c.cycles += 2;
                let t = true;
                c.sw((c.g[S7][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                if t {
                    break 'b_235c98;
                } else {
                    break 'b_235b64;
                }
            }
            'b_235be4: {
                c.cycles += 16;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x14));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.g[S4][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x00144d98, 0x00235b88)?; // call_indirect
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00235b9c)?; // call_indirect
                let t = (c.g[S3][0] as i64) <= 0;
                c.g[V1][0] = sx((c.g[S4][0] as u32) << 2);
                if t {
                    break 'b_235be4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                    'l_235bb0: loop {
                        c.cycles += 13;
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x28));
                        c.g[A0][0] = c.g[S5][0];
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x00147338, 0x00235bc0)?; // call_indirect
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S5][0];
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                        call_ptr(c, 0x001473d0, 0x00235bd8)?; // call_indirect
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00235bb0);
                            }
                            continue 'l_235bb0;
                        } else {
                            break 'b_235be4;
                        }
                    }
                }
            }
            'b_235c28: {
                c.cycles += 12;
                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00235bf8)?; // call_indirect
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012ab20, 0x00235c08)?; // call_indirect
                c.g[V0][0] = ((c.g[S3][0] as i64) < 2i64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x18), 0u64);
                    break 'b_235c28;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012ac48, 0x00235c24)?; // call_indirect
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x18), 0u64);
                    break 'b_235c28;
                }
            }
            'b_235c50: {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5bc8));
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                let t = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                if t {
                    break 'b_235c50;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0027a860, 0x00235c4c)?; // call_indirect
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    break 'b_235c50;
                }
            }
            c.cycles += 18;
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x002790a0, 0x00235c58)?; // call_indirect
            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x98));
            c.g[V0][0] = 0x2;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            c.g[A1][0] = 0x3;
            if c.g[V1][0] == 0 {
                c.g[A1][0] = c.g[V0][0];
            }
            call_ptr(c, 0x0027aaf8, 0x00235c70)?; // call_indirect
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x00278f38, 0x00235c7c)?; // call_indirect
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 0x3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
            call_ptr(c, 0x001f36c0, 0x00235c90)?; // call_indirect
            c.sw((c.g[S7][0] as u32).wrapping_add(0x10), 0u64);
            c.sw((c.g[S7][0] as u32).wrapping_add(0x14), 0u64);
            break 'b_235c98;
        }
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
        Ok(())
    })
}

/// `GameStateResults_Enter` (0x00236da0): Enter handler of the PostEvent (results) state: outside a freeroam path, clears the first-time flag of every rider, stops the script list, clears the timer and opens the results overlay (mode 7). `(self)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_state_results_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_236e2c: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S3][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x00236dc0)?; // call_indirect
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00236dcc)?; // call_indirect
            c.g[V1][0] = 0x530000;
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
            c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            if t {
                break 'b_236e2c;
            } else {
                c.cycles += 9;
                c.g[S0][0] = 0u64;
                call_ptr(c, 0x0014dc80, 0x00236de8)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x00236df4)?; // call_indirect
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                if t {
                    break 'b_236e2c;
                } else {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S0][0];
                    'l_236e08: loop {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = 0u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        call_ptr(c, 0x00145cb0, 0x00236e18)?; // call_indirect
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = c.g[S0][0];
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00236e08);
                            }
                            continue 'l_236e08;
                        } else {
                            c.cycles += 1;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                            break 'b_236e2c;
                        }
                    }
                }
            }
        }
        c.cycles += 12;
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x002790a0, 0x00236e34)?; // call_indirect
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
        c.g[A0][0] = 0x7;
        call_ptr(c, 0x0020a8f8, 0x00236e40)?; // call_indirect
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `GameModeMan_OnRiderFinished` (0x00238358): `cGameModeMan::raceFinish`: tells the mode handler of the current mode that a rider finished, and for a normal (non-replay) race on a known course submits the race result, the mountain-conquered award and the placement to the score interface. `(self, rider, stats)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_mode_man_on_rider_finished(c: &mut Cpu) {
    guest(c, |c| {
        'b_2384e8: {
            'b_2384e4: {
                c.cycles += 35;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.g[V1][0] = 0x530000;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6668));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S1][0] = c.g[A2][0];
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S7][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x002383b8)?; // call_indirect
                call_ptr(c, 0x0014dc80, 0x002383c0)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x002383cc)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00144bc0, 0x002383d4)?; // call_indirect
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[V1][0] = ((c.g[V1][0] as i64) < 17i64) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_2384e4;
                } else {
                    'b_2383fc: {
                        c.cycles += 5;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[A0][0] = c.lw((c.g[A0][0] as u32));
                        let t = c.g[A0][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_2383fc;
                        } else {
                            c.cycles += 1;
                            c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                            break 'b_2383fc;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[A1][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    if t {
                        break 'b_2384e8;
                    } else {
                        'b_2384c0: {
                            c.cycles += 18;
                            c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                            call_ptr(c, 0x0014dc80, 0x0023840c)?; // call_indirect
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0x8;
                            call_ptr(c, 0x0014dd58, 0x00238418)?; // call_indirect
                            c.g[S6][0] = c.g[S0][0];
                            c.g[S5][0] = c.g[V0][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[V0][0] = 0x530000;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[S4][0] = c.lw((c.g[S0][0] as u32));
                            c.g[A3][0] = c.g[S4][0];
                            call_ptr(c, 0x00154ab8, 0x00238444)?; // call_indirect
                            if c.g[S2][0] != 0u64 {
                                c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x9c));
                                break 'b_2384c0;
                            } else {
                                'b_2384bc: {
                                    c.cycles += 17;
                                    c.g[A0][0] = 0u64;
                                    call_ptr(c, 0x0014a0e0, 0x00238454)?; // call_indirect
                                    c.g[A0][0] = 0u64;
                                    c.g[S1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014a080, 0x00238460)?; // call_indirect
                                    c.g[S3][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014dc80, 0x00238468)?; // call_indirect
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A1][0] = 0xd;
                                    call_ptr(c, 0x0014dd58, 0x00238474)?; // call_indirect
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A2][0] = c.g[S3][0];
                                    call_ptr(c, 0x001577e0, 0x00238488)?; // call_indirect
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        break 'b_2384bc;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[A2][0] = c.g[S1][0];
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x0015a2e0, 0x002384a0)?; // call_indirect
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S7][0] as u32).wrapping_add(0x9c));
                                            break 'b_2384c0;
                                        } else {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[A2][0] = c.g[S3][0];
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00159cd0, 0x002384bc)?; // call_indirect
                                            break 'b_2384bc;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x9c));
                                break 'b_2384c0;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x530000;
                        if t {
                            break 'b_2384e4;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A3][0] = c.g[S4][0];
                            c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                            call_ptr(c, 0x00154ee8, 0x002384e4)?; // call_indirect
                            break 'b_2384e4;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_2384e8;
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        Ok(())
    })
}

/// `Path_Read` (0x0026a180): Reads a path from a big-endian data stream: item blocks through the path's virtual `+0x18`/`+0x20` slots, two counts, three vectors, then the segment and event arrays. Returns the bytes consumed. `(path, data)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn path_read(c: &mut Cpu) {
    guest(c, |c| {
        'b_26a238: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = 0u64;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S4][0] = c.g[S0][0];
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0026a1c0)?; // call_indirect
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0026ba68, 0x0026a1cc)?; // PathData_ReadInt
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc));
            if t {
                break 'b_26a238;
            } else {
                c.cycles += 1;
                'l_26a1e0: loop {
                    c.cycles += 22;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0026ba68, 0x0026a1ec)?; // PathData_ReadInt
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0026ba68, 0x0026a1fc)?; // PathData_ReadInt
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0026a224)?; // call_indirect
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026a1e0);
                        }
                        continue 'l_26a1e0;
                    } else {
                        break 'b_26a238;
                    }
                }
            }
        }
        'b_26a2d4: {
            c.cycles += 28;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0026ba68, 0x0026a244)?; // PathData_ReadInt
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.g[S1][0] = 0u64;
            call_ptr(c, 0x0026ba68, 0x0026a264)?; // PathData_ReadInt
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            c.sw((c.g[S2][0] as u32), c.g[V0][0]);
            call_ptr(c, 0x0026ba48, 0x0026a27c)?; // PathData_ReadVec3
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1c));
            call_ptr(c, 0x0026ba48, 0x0026a288)?; // PathData_ReadVec3
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x0026ba48, 0x0026a298)?; // PathData_ReadVec3
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
            let t = (c.g[V0][0] as i64) <= 0;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[S0][0]);
            if t {
                break 'b_26a2d4;
            } else {
                'l_26a2a8: loop {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                    c.g[A2][0] = sx((c.g[S1][0] as u32) << 4);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x0026a338, 0x0026a2c4)?; // Path_ReadSegment
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026a2a8);
                        }
                        continue 'l_26a2a8;
                    } else {
                        break 'b_26a2d4;
                    }
                }
            }
        }
        'b_26a314: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32));
            c.g[S1][0] = 0u64;
            let t = (c.g[V0][0] as i64) <= 0;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[S0][0]);
            if t {
                break 'b_26a314;
            } else {
                c.cycles += 1;
                'l_26a2e8: loop {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = sx((c.g[S1][0] as u32) << 4);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x0026a378, 0x0026a304)?; // Path_ReadEvent
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026a2e8);
                        }
                        continue 'l_26a2e8;
                    } else {
                        break 'b_26a314;
                    }
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[S4][0] as u32)));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `ScriptEngine_QueueFinishScripts` (0x0027a860): `cSSXScriptEngine::PlayTransportSequence`: adds the transport sequence (intro optional, online or offline variants by world mode) to script list 1, readies and plays it and fires music event 20. `(self, online, intro)`.
/// NOT INSTALLED (no OVERRIDES entry, no test): its music event reaches MusicPlayer_SelectNextSong, which loops forever unless the player object holds a valid song mask; the player is deep inside the audio singleton.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn script_engine_queue_finish_scripts(c: &mut Cpu) {
    guest(c, |c| {
        'b_27a9ac: {
            'b_27a98c: {
                'b_27a974: {
                    'b_27a8f8: {
                        c.cycles += 5;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        let t = c.g[A1][0] == 0u64;
                        c.g[S0][0] = c.g[A0][0];
                        if t {
                            break 'b_27a8f8;
                        } else {
                            'b_27a898: {
                                c.cycles += 2;
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_27a898;
                                } else {
                                    c.cycles += 7;
                                    c.g[A2][0] = 0x12;
                                    c.g[A3][0] = 0x1;
                                    c.g[T0][0] = 0x1;
                                    c.g[T1][0] = 0xffffffffffffffff;
                                    c.g[T2][0] = 0xffffffffffffffff;
                                    c.g[T3][0] = 0xffffffffffffffff;
                                    call_ptr(c, 0x00278e50, 0x0027a898)?; // ScriptEngine_ListAddChoice
                                    break 'b_27a898;
                                }
                            }
                            'b_27a8e8: {
                                c.cycles += 16;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = 0x1;
                                c.g[A2][0] = 0x13;
                                c.g[A3][0] = 0u64;
                                c.g[T0][0] = 0u64;
                                c.g[T1][0] = 0xffffffffffffffff;
                                c.g[T2][0] = 0xffffffffffffffff;
                                c.g[T3][0] = 0xffffffffffffffff;
                                call_ptr(c, 0x00278e50, 0x0027a8bc)?; // ScriptEngine_ListAddChoice
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A1][0] = 0x1;
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                                let t = c.g[A0][0] != c.g[A1][0];
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_27a8e8;
                                } else {
                                    c.cycles += 4;
                                    c.g[A2][0] = 0x14;
                                    c.g[A3][0] = 0x8;
                                    let t = true;
                                    c.g[T0][0] = 0u64;
                                    if t {
                                        break 'b_27a974;
                                    } else {
                                        break 'b_27a8e8;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[A2][0] = 0x15;
                            c.g[A3][0] = 0x8;
                            let t = true;
                            c.g[T0][0] = 0u64;
                            if t {
                                break 'b_27a974;
                            } else {
                                break 'b_27a8f8;
                            }
                        }
                    }
                    'b_27a920: {
                        c.cycles += 2;
                        let t = c.g[A2][0] == 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_27a920;
                        } else {
                            c.cycles += 8;
                            c.g[A1][0] = 0x1;
                            c.g[A2][0] = 0xc;
                            c.g[A3][0] = 0x1;
                            c.g[T0][0] = 0x1;
                            c.g[T1][0] = 0xffffffffffffffff;
                            c.g[T2][0] = 0xffffffffffffffff;
                            c.g[T3][0] = 0xffffffffffffffff;
                            call_ptr(c, 0x00278e50, 0x0027a920)?; // ScriptEngine_ListAddChoice
                            break 'b_27a920;
                        }
                    }
                    c.cycles += 18;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = 0xd;
                    c.g[A3][0] = 0u64;
                    c.g[T0][0] = 0x1;
                    c.g[T1][0] = 0xffffffffffffffff;
                    c.g[T2][0] = 0xffffffffffffffff;
                    c.g[T3][0] = 0xffffffffffffffff;
                    call_ptr(c, 0x00278e50, 0x0027a944)?; // ScriptEngine_ListAddChoice
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xf44), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    let t = c.g[A0][0] != c.g[A1][0];
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_27a98c;
                    } else {
                        c.cycles += 3;
                        c.g[A2][0] = 0xe;
                        c.g[A3][0] = 0x8;
                        c.g[T0][0] = 0x1;
                        break 'b_27a974;
                    }
                }
                c.cycles += 6;
                c.g[T1][0] = 0xffffffffffffffff;
                c.g[T2][0] = 0xffffffffffffffff;
                c.g[T3][0] = 0xffffffffffffffff;
                call_ptr(c, 0x00278e50, 0x0027a984)?; // ScriptEngine_ListAddChoice
                let t = true;
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_27a9ac;
                } else {
                    break 'b_27a98c;
                }
            }
            c.cycles += 8;
            c.g[A2][0] = 0xf;
            c.g[A3][0] = 0x8;
            c.g[T0][0] = 0u64;
            c.g[T1][0] = 0xffffffffffffffff;
            c.g[T2][0] = 0xffffffffffffffff;
            c.g[T3][0] = 0xffffffffffffffff;
            call_ptr(c, 0x00278e50, 0x0027a9a8)?; // ScriptEngine_ListAddChoice
            c.g[A0][0] = c.g[S0][0];
            break 'b_27a9ac;
        }
        c.cycles += 17;
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x00278f38, 0x0027a9b4)?; // ScriptEngine_ListPrepare
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x1;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0x1;
        call_ptr(c, 0x00278f68, 0x0027a9c8)?; // ScriptEngine_ListStart
        call_ptr(c, 0x0028b180, 0x0027a9d0)?; // SSXAudioSystem_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x14;
        c.g[A2][0] = 0x1;
        call_ptr(c, 0x0028e8c0, 0x0027a9e0)?; // SSXAudioSystem_HandleMusicEvent
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `ScriptEngine_QueueRaceCutscene` (0x0027ac60): `cSSXScriptEngine::AppendEndEventSequence`: for a finished race adds the winner script (by position and character) to script list 0 and, for peak events, the first-win scripts and the peak flag. A dispatcher: its loop back edges are resume points.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn script_engine_queue_race_cutscene(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0027ac60 => {
                    c.cycles += 23;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
                    c.g[RA][0] = sx(0x0027ac94);
                    call_ptr(c, 0x0014dc80, 0x0027ac94)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0027aca0);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x0027aca0)?; // call_indirect
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x90), c.g[V0][0]);
                    c.g[V0][0] = 0x530000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[S6][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = 0x4;
                    let t = c.g[S6][0] == c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x0027b08c;
                        continue;
                    }
                    pc = 0x0027acbc;
                    continue;
                }
                0x0027acbc => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc0));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x98));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x0027b08c;
                        continue;
                    }
                    pc = 0x0027accc;
                    continue;
                }
                0x0027accc => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    if t {
                        pc = 0x0027b090;
                        continue;
                    }
                    pc = 0x0027ace0;
                    continue;
                }
                0x0027ace0 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x49));
                    c.g[S4][0] = 0u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    if t {
                        pc = 0x0027ad50;
                        continue;
                    }
                    pc = 0x0027acf0;
                    continue;
                }
                0x0027acf0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x7c));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x0027ad50;
                        continue;
                    }
                    pc = 0x0027acfc;
                    continue;
                }
                0x0027acfc => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x530000;
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                    pc = 0x0027ad08;
                    continue;
                }
                0x0027ad08 => {
                    c.cycles += 14;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0027ad28);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    call_ptr(c, t, 0x0027ad28)?; // call_indirect
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0027af30;
                        continue;
                    }
                    pc = 0x0027ad40;
                    continue;
                }
                0x0027ad40 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x7c));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0027ad08;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0027ad08;
                        continue;
                    }
                    pc = 0x0027ad50;
                    continue;
                }
                0x0027ad50 => {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;
                    c.g[V0][0] = 0x530000;
                    if t {
                        pc = 0x0027ae84;
                        continue;
                    }
                    pc = 0x0027ad58;
                    continue;
                }
                0x0027ad58 => {
                    c.cycles += 8;
                    c.g[S4][0] = 0x3;
                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6708));
                    c.g[S1][0] = 0u64;
                    c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[FP][0] = 0xffffffffffffffff;
                    c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
                    c.g[S0][0] = c.g[SP][0];
                    pc = 0x0027ad78;
                    continue;
                }
                0x0027ad78 => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                    c.sw((c.g[S0][0] as u32), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x78));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    if t {
                        pc = 0x0027ad98;
                        continue;
                    }
                    pc = 0x0027ad90;
                    continue;
                }
                0x0027ad90 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32), c.g[FP][0]);
                    if t {
                        pc = 0x0027adbc;
                        continue;
                    }
                    pc = 0x0027ad98;
                    continue;
                }
                0x0027ad98 => {
                    c.cycles += 9;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0027adb8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    call_ptr(c, t, 0x0027adb8)?; // call_indirect
                    if c.g[V0][0] != 0 {
                        c.g[S4][0] = c.g[S1][0];
                    }
                    pc = 0x0027adbc;
                    continue;
                }
                0x0027adbc => {
                    c.cycles += 5;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0027ad78;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0027ad78;
                        continue;
                    }
                    pc = 0x0027add0;
                    continue;
                }
                0x0027add0 => {
                    c.cycles += 14;
                    c.g[V1][0] = 0x480000;
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1e90));
                    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
                    c.ldr(4, (c.g[A3][0] as u32));
                    c.ldl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                    c.ldr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x17));
                    c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x1f));
                    c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = c.g[S4][0] != 0u64;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                    if t {
                        pc = 0x0027ae60;
                        continue;
                    }
                    pc = 0x0027ae08;
                    continue;
                }
                0x0027ae08 => {
                    c.cycles += 18;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                    c.g[S0][0] = 0x9;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0027ae34);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    call_ptr(c, t, 0x0027ae34)?; // call_indirect
                    c.g[RA][0] = sx(0x0027ae3c);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014a0b0, 0x0027ae3c)?; // call_indirect
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xa));
                    c.g[V0][0] = (c.g[V0][0] < 0xbu64) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[S0][0] = 0xa;
                        pc = 0x0027ae60;
                        continue;
                    }
                    pc = 0x0027ae50;
                    continue;
                }
                0x0027ae50 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1c;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    if t {
                        pc = 0x0027ae64;
                        continue;
                    }
                    pc = 0x0027ae5c;
                    continue;
                }
                0x0027ae5c => {
                    c.cycles += 1;
                    c.g[S0][0] = 0xa;
                    pc = 0x0027ae60;
                    continue;
                }
                0x0027ae60 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    pc = 0x0027ae64;
                    continue;
                }
                0x0027ae64 => {
                    c.cycles += 8;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32));
                    c.g[A1][0] = 0u64;
                    c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = 0x1;
                    c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[RA][0] = sx(0x0027ae84);
                    c.g[T0][0] = 0u64;
                    call_ptr(c, 0x00278e50, 0x0027ae84)?; // ScriptEngine_ListAddChoice
                    pc = 0x0027ae84;
                    continue;
                }
                0x0027ae84 => {
                    c.cycles += 5;
                    c.g[T0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5bc8));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    let t = c.g[V1][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    if t {
                        pc = 0x0027b090;
                        continue;
                    }
                    pc = 0x0027ae98;
                    continue;
                }
                0x0027ae98 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S6][0] < 0x4u64) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                        pc = 0x0027b094;
                        continue;
                    }
                    pc = 0x0027aea4;
                    continue;
                }
                0x0027aea4 => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x0027aeac);
                    call_ptr(c, 0x0014dc80, 0x0027aeac)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0027aeb8);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014dd58, 0x0027aeb8)?; // call_indirect
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[RA][0] = sx(0x0027aec4);
                    c.g[S3][0] = c.g[V0][0];
                    call_ptr(c, 0x00144bc0, 0x0027aec4)?; // call_indirect
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x0027af38;
                        continue;
                    }
                    pc = 0x0027aed4;
                    continue;
                }
                0x0027aed4 => {
                    c.cycles += 2;
                    c.g[S4][0] = (c.g[S6][0] < 0x1u64) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                    pc = 0x0027aedc;
                    continue;
                }
                0x0027aedc => {
                    c.cycles += 19;
                    c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(9, (c.g[V1][0] as u32));
                    c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.sdr(9, (c.g[A2][0] as u32));
                    c.sdl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                    c.sdr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sdl(5, (c.g[A2][0] as u32).wrapping_add(0x17));
                    c.sdr(5, (c.g[A2][0] as u32).wrapping_add(0x10));
                    c.sdl(7, (c.g[A2][0] as u32).wrapping_add(0x1f));
                    c.sdr(7, (c.g[A2][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0027aedc;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0027aedc;
                        continue;
                    }
                    pc = 0x0027af28;
                    continue;
                }
                0x0027af28 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x0027af6c;
                        continue;
                    }
                    pc = 0x0027af30;
                    continue;
                }
                0x0027af30 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S4][0] = 0x1;
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0027ad50;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0027ad50;
                        continue;
                    }
                    pc = 0x0027af38;
                    continue;
                }
                0x0027af38 => {
                    c.cycles += 2;
                    c.g[S4][0] = (c.g[S6][0] < 0x1u64) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                    pc = 0x0027af40;
                    continue;
                }
                0x0027af40 => {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32));
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd((c.g[A2][0] as u32), c.g[T0][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0027af40;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0027af40;
                        continue;
                    }
                    pc = 0x0027af6c;
                    continue;
                }
                0x0027af6c => {
                    c.cycles += 18;
                    c.g[A3][0] = c.lw((c.g[V1][0] as u32));
                    c.g[A0][0] = c.g[S3][0];
                    c.sw((c.g[A2][0] as u32), c.g[A3][0]);
                    c.g[RA][0] = sx(0x0027af80);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00146e98, 0x0027af80)?; // call_indirect
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x74));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0027af98);
                    c.g[A3][0] = c.g[S4][0];
                    call_ptr(c, 0x00146150, 0x0027af98)?; // call_indirect
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0027afa4);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0014a0b0, 0x0027afa4)?; // call_indirect
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xa));
                    c.g[V0][0] = (c.g[V0][0] < 0xbu64) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                        pc = 0x0027b090;
                        continue;
                    }
                    pc = 0x0027afb4;
                    continue;
                }
                0x0027afb4 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0027afc0);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00147398, 0x0027afc0)?; // call_indirect
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x74));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0027afd8);
                    c.g[T0][0] = c.g[S4][0];
                    call_ptr(c, 0x001464d0, 0x0027afd8)?; // call_indirect

                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                        pc = 0x0027b090;
                        continue;
                    }
                    pc = 0x0027afe0;
                    continue;
                }
                0x0027afe0 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    if t {
                        pc = 0x0027b090;
                        continue;
                    }
                    pc = 0x0027afe8;
                    continue;
                }
                0x0027afe8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x80));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x0027affc;
                        continue;
                    }
                    pc = 0x0027aff4;
                    continue;
                }
                0x0027aff4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x48));
                    if t {
                        pc = 0x0027b00c;
                        continue;
                    }
                    pc = 0x0027affc;
                    continue;
                }
                0x0027affc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));

                    if (c.g[V0][0] as i64) <= 0 {
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        pc = 0x0027b030;
                        continue;
                    }
                    pc = 0x0027b008;
                    continue;
                }
                0x0027b008 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                    pc = 0x0027b00c;
                    continue;
                }
                0x0027b00c => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0027b028);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    call_ptr(c, t, 0x0027b028)?; // call_indirect
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    pc = 0x0027b030;
                    continue;
                }
                0x0027b030 => {
                    c.cycles += 23;
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x18;
                    c.g[A3][0] = 0x1;
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[T2][0] = 0xffffffffffffffff;
                    c.g[RA][0] = sx(0x0027b050);
                    c.g[T3][0] = 0xffffffffffffffff;
                    call_ptr(c, 0x00278e50, 0x0027b050)?; // ScriptEngine_ListAddChoice
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x19;
                    c.g[A3][0] = 0x3;
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[T2][0] = 0xffffffffffffffff;
                    c.g[RA][0] = sx(0x0027b074);
                    c.g[T3][0] = 0xffffffffffffffff;
                    call_ptr(c, 0x00278e50, 0x0027b074)?; // ScriptEngine_ListAddChoice
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x74));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A3][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0027b08c);
                    c.g[T0][0] = 0x1;
                    call_ptr(c, 0x00146320, 0x0027b08c)?; // call_indirect
                    pc = 0x0027b08c;
                    continue;
                }
                0x0027b08c => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    pc = 0x0027b090;
                    continue;
                }
                0x0027b090 => {
                    c.cycles += 1;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                    pc = 0x0027b094;
                    continue;
                }
                0x0027b094 => {
                    c.cycles += 10;
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0027ac60, 0x0027b0bc);
                    return Ok(());
                }
            }
        }
        Ok(())
    })
}

/// `FXLoader_ctor` (0x002dbd10): `cFXLoader::cFXLoader`: loads the particle (and unless `particles_only`, the effects) texture files, binds the 81 effect shapes to texture ids and fills the ten effect texture slots. `(self, particles_only)`.
/// NOT INSTALLED (no OVERRIDES entry, no test): EAFile_LoadFileEx (pc 0x3de768) has no override slot and spins on test data.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn fx_loader_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2dbda4: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.g[V0][0] = 0x1;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S4][0] = 0x2;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            if c.g[A1][0] != 0 {
                c.g[S4][0] = c.g[V0][0];
            }
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = 0u64;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            let t = c.g[S4][0] == 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            if t {
                break 'b_2dbda4;
            } else {
                c.cycles += 3;
                c.g[S3][0] = 0x1;
                c.g[S2][0] = 0x480000;
                c.g[S1][0] = 0x480000;
                'l_2dbd60: loop {
                    'b_2dbd98: {
                        'b_2dbd94: {
                            'b_2dbd88: {
                                'b_2dbd78: {
                                    c.cycles += 2;
                                    if c.g[S0][0] == 0u64 {
                                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x7918));
                                        break 'b_2dbd78;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S0][0] == c.g[S3][0];
                                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7938));
                                        if t {
                                            break 'b_2dbd88;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            if t {
                                                break 'b_2dbd98;
                                            } else {
                                                break 'b_2dbd78;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[A1][0] = 0x100;
                                call_ptr(c, 0x003e18c8, 0x002dbd80)?; // call_indirect
                                let t = true;
                                c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                                if t {
                                    break 'b_2dbd94;
                                } else {
                                    break 'b_2dbd88;
                                }
                            }
                            c.cycles += 3;
                            c.g[A1][0] = 0x100;
                            call_ptr(c, 0x003e18c8, 0x002dbd90)?; // call_indirect
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            break 'b_2dbd94;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        break 'b_2dbd98;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002dbd60);
                        }
                        continue 'l_2dbd60;
                    } else {
                        break 'b_2dbda4;
                    }
                }
            }
        }
        c.cycles += 9;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
        c.g[V0][0] = 0x490000;
        c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6e50));
        c.g[S1][0] = 0u64;
        c.g[FP][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10b0));
        c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf50));
        c.g[S7][0] = 0xc;
        c.g[A1][0] = 0u64;
        'l_2dbdc8: loop {
            c.cycles += 4;
            c.g[S0][0] = 0u64;
            c.g[S2][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            'l_2dbdd8: loop {
                'b_2dbe04: {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.mult(0, 2, 17, 23);
                    if t {
                        break 'b_2dbe04;
                    } else {
                        c.cycles += 8;
                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                        call_ptr(c, 0x003b3900, 0x002dbdf8)?; // call_indirect
                        c.g[A1][0] = c.g[V0][0];
                        if c.g[A1][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            if event_due(c) {
                                deliver_event_here(c, 0x002dbdd8);
                            }
                            continue 'l_2dbdd8;
                        } else {
                            break 'b_2dbe04;
                        }
                    }
                }
                'b_2dbe44: {
                    c.cycles += 2;
                    let t = c.g[A1][0] == 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_2dbe44;
                    } else {
                        c.cycles += 14;
                        c.mult(0, 6, 17, 23);
                        c.g[T3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                        c.g[V0][0] = 0x490000;
                        c.g[T0][0] = 0x1;
                        c.g[V1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x10d8));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6e48));
                        c.g[T1][0] = 0xffffffffffffffff;
                        c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                        c.g[A3][0] = c.lw((c.g[T2][0] as u32));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x002dbe44)?; // call_indirect
                        break 'b_2dbe44;
                    }
                }
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                c.g[S1][0] = c.g[S3][0];
                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[V0][0] = ((c.g[S1][0] as i64) < 81i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = 0u64;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x002dbdc8);
                    }
                    continue 'l_2dbdc8;
                } else {
                    'b_2dbe88: {
                        c.cycles += 2;
                        let t = c.g[S4][0] == 0u64;
                        c.g[A0][0] = c.g[FP][0];
                        if t {
                            break 'b_2dbe88;
                        } else {
                            c.cycles += 3;
                            c.g[S1][0] = c.g[S4][0];
                            c.g[S0][0] = c.g[SP][0];
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                            'l_2dbe70: loop {
                                c.cycles += 5;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                call_ptr(c, 0x002523a8, 0x002dbe7c)?; // call_indirect
                                if c.g[S1][0] != 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002dbe70);
                                    }
                                    continue 'l_2dbe70;
                                } else {
                                    c.cycles += 1;
                                    c.g[A0][0] = c.g[FP][0];
                                    break 'b_2dbe88;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x11c));
                    c.g[A1][0] = 0x9;
                    'l_2dbe90: loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                        c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                        let t = (c.g[A1][0] as i64) >= 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002dbe90);
                            }
                            continue 'l_2dbe90;
                        } else {
                            c.cycles += 13;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                            let t = c.g[RA][0] as u32;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                            c.pc = t;
                            return Ok(());
                        }
                    }
                }
            }
        }
        Ok(())
    })
}

/// `TexMan_CopyToTempTex` (0x00366e90): Copies a texture into a temporary buffer one mip level at a time and swizzles each level (8-bit) into its place, then frees the buffer and sets the texture state to 10. `(self, tex)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn tex_man_copy_to_temp_tex(c: &mut Cpu) {
    guest(c, |c| {
        'b_366f98: {
            c.cycles += 38;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.g[A3][0] = 0u64;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S4][0] = c.g[A1][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A1][0] = 0x4a0000;
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4060));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S7][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S6][0] = 0u64;
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[A2][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x1f58));
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
            c.g[A2][0] = c.g[A2][0] ^ 0x100;
            call_ptr(c, 0x00317e30, 0x00366ee4)?; // call_indirect
            c.g[S5][0] = c.g[V0][0];
            c.g[A0][0] = 0x1;
            c.g[V0][0] = c.ld((c.g[S4][0] as u32).wrapping_add(0x38));
            c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x24));
            c.g[V1][0] = c.g[V0][0] >> 30;
            c.g[FP][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.g[V0][0] >> 26;
            c.g[V1][0] = c.g[V1][0] & 0xf;
            c.g[V0][0] = c.g[V0][0] & 0xf;
            c.g[V1][0] = c.g[V1][0] << 32;
            c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            c.g[S3][0] = sx((c.g[A0][0] as u32) << ((c.g[V1][0] as u32) & 31));
            c.g[S2][0] = sx((c.g[A0][0] as u32) << ((c.g[V0][0] as u32) & 31));
            let t = (c.g[A1][0] as i64) < 0;
            c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1c));
            if t {
                break 'b_366f98;
            } else {
                'l_366f28: loop {
                    c.cycles += 28;
                    c.mult(0, 3, 18, 19);
                    c.g[A2][0] = 0xffffffffffffffff;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                    c.mult(0, 3, 3, 30);
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7));
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[V1][0];
                    }
                    c.g[A2][0] = 0x100;
                    c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 3) as u32);
                    c.g[V1][0] = 0xff;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[A2][0];
                    }
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x003e6574, 0x00366f68)?; // call_indirect
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[T0][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    call_ptr(c, 0x00390198, 0x00366f84)?; // TexMan_SwizzleT8
                    c.g[S2][0] = sx(((c.g[S2][0] as u32) as i32 >> 1) as u32);
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S3][0] = sx(((c.g[S3][0] as u32) as i32 >> 1) as u32);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00366f28);
                        }
                        continue 'l_366f28;
                    } else {
                        break 'b_366f98;
                    }
                }
            }
        }
        'b_366fac: {
            c.cycles += 2;
            let t = c.g[S5][0] == 0u64;
            c.g[V0][0] = 0xa;
            if t {
                break 'b_366fac;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x00317e98, 0x00366fa8)?; // call_indirect
                c.g[V0][0] = 0xa;
                break 'b_366fac;
            }
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sw((c.g[S4][0] as u32).wrapping_add(0xc), c.g[V0][0]);
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
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{elf_image, Rng, DATA, GAME_GP};
    use ssx3_ee_rt::cpu::reg::*;

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

    /// A BE rider index (the profile tables map it to profile 0, 1 or none).
    fn oc444_rider(r: &mut Rng) -> u32 {
        r.next_u32() % 4
    }

    // ------------------------------------------------- BEScoreInt_Submit*.

    /// `BEScoreInt_SubmitRaceResult(self, rider, stats, score)`: copies the
    /// stats (aligned and unaligned source), updates the best-stats, the
    /// high-score table and the records for profile 0 and 1.
    #[test]
    fn be_score_int_submit_race_result_matches() {
        oc444_run(
            ee::s01::f_00154ab8__generated,
            be_score_int_submit_race_result,
            0x0015_4ab8,
            0x15_4ab8,
            120,
            &|c, r, seed| {
                c.g[A0][0] = OC444_OBJ as u64;
                c.g[A1][0] = oc444_rider(r) as u64;
                c.g[A2][0] = (OC444_OBJ2 + 4 * (seed % 2)) as u64;
                let scores = [0u32, 1, 59, 600, u32::MAX, r.next_u32() % 100_000];
                c.g[A3][0] = sx(scores[seed as usize % 6]);
            },
        );
    }

    /// `BEScoreInt_SubmitPlacement(self, rider, result, score)`.
    #[test]
    fn be_score_int_submit_placement_matches() {
        oc444_run(
            ee::s01::f_00154ee8__generated,
            be_score_int_submit_placement,
            0x0015_4ee8,
            0x15_4ee8,
            120,
            &|c, r, seed| {
                c.g[A0][0] = OC444_OBJ as u64;
                c.g[A1][0] = oc444_rider(r) as u64;
                let results = [0u32, 1, 2, 3, u32::MAX, 5];
                c.g[A2][0] = sx(results[seed as usize % 6]);
                c.g[A3][0] = [0u32, 100, 6000, 7200][r.next_u32() as usize % 4] as u64;
            },
        );
    }

    // ----------------------------------------- FEAsyncManager_LoadSlot.

    // --------------------------------------- EventSelect (event map UI).

    /// The hash the game computes for the `"SP_PeakList"` label.
    fn oc444_str_hash(addr: u32) -> u32 {
        use std::collections::HashMap;
        use std::sync::Mutex;
        static CACHE: Mutex<Option<HashMap<u32, u32>>> = Mutex::new(None);
        let mut cache = CACHE.lock().unwrap_or_else(|p| p.into_inner());
        *cache
            .get_or_insert_with(HashMap::new)
            .entry(addr)
            .or_insert_with(|| {
                let mut c = crate::difftest::fresh();
                elf_image(&mut c);
                c.g[A0][0] = addr as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.pc = 0x0031_7670;
                let f = ssx3_ee_rt::funcs::lookup(0x0031_7670).expect("StrHash");
                f(&mut c);
                c.g[V0][0] as u32
            })
    }

    /// `EventSelect_OnWidgetCreated(self, menu)`: only a menu labelled
    /// `SP_PeakList` selects an entry, by map style.
    #[test]
    fn event_select_on_widget_created_matches() {
        oc444_run(
            ee::s05::f_002009d0__generated,
            event_select_on_widget_created,
            0x0020_09d0,
            0x20_09d0,
            80,
            &|c, r, seed| {
                c.g[A0][0] = OC444_OBJ as u64;
                c.g[A1][0] = (OC444_OBJ2 + 0x100) as u64;
                let label = if seed % 4 == 0 {
                    r.next_u32() % 6
                } else {
                    oc444_str_hash(0x0047_0600)
                };
                c.bus.write32(OC444_OBJ2 + 0x100 + 0x38, label);
            },
        );
    }

    /// `EventSelect_Update(self)`: fade in or hide the map, the peak
    /// outlines (map state 0) or indicators, the "you are here" labels and the
    /// async-loaded texture.
    #[test]
    fn event_select_update_matches() {
        oc444_run(
            ee::s05::f_00202828__generated,
            event_select_update,
            0x0020_2828,
            0x20_2828,
            120,
            &|c, r, seed| {
                let slot = r.next_u32() % 3;
                c.g[A0][0] = OC444_OBJ as u64;
                c.bus.write32(OC444_OBJ, (seed % 5 == 0) as u32);
                c.bus.write32(OC444_OBJ + 0x2d8, slot);
                c.bus.write32(OC444_OBJ + 0x2dc, (seed % 2) as u32);
                let fade = [0u32, 0x3dcc_cccd, 0x3f66_6666, 0x3f80_0000][r.next_u32() as usize % 4];
                c.bus.write32(OC444_OBJ + 0x2e0, fade);
                c.bus.write32(OC444_OBJ + 0x2e4, OC444_L1 + 0x1000);
                c.bus.write32(OC444_OBJ + 0x2ec, OC444_L1 + 0x1400);
                c.bus.write32(OC444_OBJ + 0x2f0, (seed % 3 == 0) as u32);
                let state = if seed % 4 < 2 { 3 } else { r.next_u32() % 3 };
                c.bus.write32(OC444_ASYNC + 0x114 + 0x11c * slot, state);
            },
        );
    }

    // ------------------------------------------- GameUI_OpenScreenForState.

    /// `GameUI_OpenScreenForState(mode)`: modes 2 and 7 and an unknown one,
    /// with a replay running or not.
    #[test]
    fn game_ui_open_screen_for_state_matches() {
        oc444_run(
            ee::s05::f_0020a8f8__generated,
            game_ui_open_screen_for_state,
            0x0020_a8f8,
            0x20_a8f8,
            150,
            &|c, r, seed| {
                c.g[A0][0] = [2u32, 7, 7, 2, 3][seed as usize % 5] as u64;
                c.bus.write32(OC444_GM, r.next_u32() % 4);
            },
        );
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

    /// `GameStateResults_Enter(self)`.
    #[test]
    fn game_state_results_enter_matches() {
        oc444_run(
            ee::s06::f_00236da0__generated,
            game_state_results_enter,
            0x0023_6da0,
            0x23_6da0,
            120,
            &|c, r, _seed| {
                c.g[A0][0] = OC444_OBJ as u64;
                let n = r.next_u32() % 4;
                c.bus.write32(OC444_NEWRACE + 0x30, n);
            },
        );
    }

    /// `GameModeMan_OnRiderFinished(self, rider, stats)`.
    #[test]
    fn game_mode_man_on_rider_finished_matches() {
        oc444_run(
            ee::s06::f_00238358__generated,
            game_mode_man_on_rider_finished,
            0x0023_8358,
            0x23_8358,
            150,
            &|c, r, seed| {
                c.g[A0][0] = OC444_OBJ as u64;
                c.bus.write32(OC444_OBJ + 4, r.next_u32() % 4);
                c.bus.write32(OC444_OBJ + 0x9c, (seed % 3 != 0) as u32);
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = (OC444_OBJ2 + 4 * (seed % 2 * 2)) as u64;
                c.bus.write32(0x0053_5c08, r.next_u32() % 4);
            },
        );
    }

    // ------------------------------------------------------------ Path_Read.

    /// `Path_Read(path, data)`: reads a path from a stream of big-endian
    /// items: n item blocks, two counts, three vectors, then the segment and
    /// event lists.
    #[test]
    fn path_read_matches() {
        oc444_run(
            ee::s07::f_0026a180__generated,
            path_read,
            0x0026_a180,
            0x26_a180,
            100,
            &|c, r, seed| {
                let path = OC444_OBJ;
                let vt = oc444_vtable(c, r);
                c.bus.write32(path + 0x34, vt);
                c.g[A0][0] = path as u64;
                let data = OC444_OBJ2;
                c.g[A1][0] = data as u64;
                let n = seed % 3;
                let mut at = data;
                let mut put = |c: &mut Cpu, v: u32| {
                    c.bus.write32(at, v);
                    at += 4;
                };
                put(c, n);
                for _ in 0..n {
                    put(c, r.next_u32() % 5);
                    put(c, 8);
                    put(c, 1);
                    put(c, 2);
                }
                put(c, r.next_u32() % 3);
                put(c, r.next_u32() % 3);
            },
        );
    }

    /// A vtable with fake methods in slots 3 and 4 (delta 0).
    pub(crate) fn oc444_vtable(c: &mut Cpu, r: &mut Rng) -> u32 {
        let vt = OC444_OBJ2 + 0x800;
        for (i, slot) in [3u32, 4].into_iter().enumerate() {
            let delta = (r.next_u32() % 4) * 4;
            c.bus.write16(vt + 8 * slot, delta as u16);
            c.bus.write32(vt + 8 * slot + 4, FAKE + 0x10 * i as u32);
        }
        vt
    }

    // ---------------------------------------------------- ScriptEngine_*.

    /// `ScriptEngine_QueueRaceCutscene(self)`: a dispatcher with resume points
    /// at its loop heads.
    #[test]
    fn script_engine_queue_race_cutscene_matches() {
        oc444_run(
            ee::s07::f_0027ac60__generated,
            script_engine_queue_race_cutscene,
            0x0027_ac60,
            0x27_ac60,
            200,
            &|c, r, seed| {
                c.g[A0][0] = OC444_SCRIPT as u64;
                c.bus.write32(OC444_GM + 0x98, (seed % 4 != 0) as u32);
                c.bus.write32(OC444_GAME + 0x84, OC444_OBJ);
                c.bus.write32(OC444_OBJ + 0x10, (seed % 5 != 0) as u32);
                let _ = r;
            },
        );
    }

    // ------------------------------------------------------- FXLoader_ctor.

    // ------------------------------------------------ TexMan_CopyToTempTex.

    /// `TexMan_CopyToTempTex(self, tex)`: copies and swizzles every mip level
    /// of a small texture.
    #[test]
    fn tex_man_copy_to_temp_tex_matches() {
        oc444_run(
            ee::s12::f_00366e90__generated,
            tex_man_copy_to_temp_tex,
            0x0036_6e90,
            0x36_6e90,
            60,
            &|c, r, seed| {
                oc444_allocator(c);
                let this = DATA + 0xe800;
                c.g[A0][0] = this as u64;
                let tex = OC444_OBJ;
                c.g[A1][0] = tex as u64;
                c.bus.write32(this + 0x1f58, r.next_u32() % 4);
                c.bus.write32(tex + 0x10, 0x400);
                let (tw, th) = (2 + r.next_u32() % 3, 2 + r.next_u32() % 3);
                c.bus
                    .write64(tex + 0x38, (tw as u64) << 26 | (th as u64) << 30);
                c.bus.write32(tex + 0x18, [8u32, 32][seed as usize % 2]);
                c.bus.write32(tex + 0x1c, OC444_OBJ2);
                c.bus
                    .write32(tex + 0x24, (r.next_u32() % 4).wrapping_sub(1));
            },
        );
    }
}
