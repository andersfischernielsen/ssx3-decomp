//! Staging module oc647 (wave 234): hand-written game-mode handlers, each with
//! its differential test against the generated body. Moved into the
//! source-tree units by `tools/reorg_modules.py apply oc647`.

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
    (0x0023_9aa0, gm_handler0_init_standings, "GMHandler0_InitStandings"),
    (0x0023_9ec8, gm_handler9_on_rider_finished, "GMHandler9_OnRiderFinished"),
];

/// `gp-0x480`: the game-mode manager pointer (`cGameModeHandler_pManager`).
const GM_MANAGER_GP: u32 = 0x480;
/// `gp-0x848`: the world pointer; its `+0x84` word is the race.
const WORLD_GP: u32 = 0x848;
/// The race-path byte of `cBENewRaceInterface_newRace` (0x535c11).
const NEW_RACE_PATH_BYTE: u32 = 0x0053_5c11;

/// `cGMBigAirHandler_fillScoreTable` (0x00239aa0, `GMHandler0_InitStandings`):
/// fills the standings table of the big-air event. The non-simulated riders
/// (`iNumberAI - iSimAI`, plus the human) take rows 0.. with rank `5 - i` and
/// zero scores in all three heats. Each simulated rider then gets its three
/// heat thresholds (heat 1 is repeated when the race path asks for it). The
/// score table is sorted with `GameModeMan_SortByScore` at the end.
pub fn gm_handler0_init_standings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);

        c.g[S0][0] = c.g[A0][0]; // self
        // race = BE interface 0, player = BE interface 1
        call_ptr(c, 0x0014_dc80, 0x0023_9ad4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_9ae0)?;
        let s = sp(c);
        let race = c.g[V0][0];
        c.sw(s, race);
        call_ptr(c, 0x0014_dc80, 0x0023_9ae8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0023_9af4)?;

        // nB = iSimAI simulated riders; numNonSimulated = iNumberAI - nB.
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(GM_MANAGER_GP));
        c.g[T0][0] = 0; // i of the ranking loop
        let s = sp(c);
        let player = c.g[V0][0];
        c.sw(s.wrapping_add(4), player);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14)); // iSimAI
        let s = sp(c);
        let sim = c.g[V0][0];
        c.sw(s.wrapping_add(8), sim);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10)); // iNumberAI
        let s = sp(c);
        c.g[V1][0] = c.lw(s.wrapping_add(8));
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(1)); // rows of the first loop
        c.g[A3][0] = 0; // k: the next free row

        // The human and non-simulated rows: rank 5 - i, zero scores.
        if (c.g[A0][0] as i64) > 0 {
            c.cycles += 8;
            c.g[V0][0] = 0x53_0000;
            c.g[V1][0] = 0x53_0000;
            c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
            c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
            c.g[T1][0] = c.g[A0][0];
            c.g[T2][0] = 5;
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(4)); // &self->t[0][0]
            'rows: loop {
                c.cycles += 10;
                c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                c.g[A2][0] = c.g[V0][0];
                c.g[S5][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S3][0] as u32));
                let rank = c.g[V1][0];
                let rank_slot = c.g[V0][0] as u32;
                c.sw(rank_slot, rank);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                c.g[V0][0] = 2;
                // Heats 2, 1, 0 of this row: zero.
                'heats0: loop {
                    c.cycles += 7;
                    let cell = c.g[A0][0] as u32;
                    c.sw(cell, 0);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    let more = (c.g[V0][0] as i64) >= 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x28));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_9b68);
                        }
                        continue 'heats0;
                    }
                    break;
                }
                c.cycles += 8;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[T0][0] = c.g[V1][0];
                c.g[A0][0] = c.lw(c.g[V0][0] as u32); // self->t[0][i]
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T3][0] as u32));
                c.g[A3][0] = c.g[S5][0];
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                let score_slot = c.g[V1][0] as u32;
                c.sw(score_slot, c.g[A0][0]);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_9b40);
                    }
                    continue 'rows;
                }
                break;
            }
        }

        // The simulated riders: three heat thresholds each.
        c.cycles += 5;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1)); // nA = numNonSimulated + 1
        c.g[A0][0] = c.lw(sp(c).wrapping_add(8)); // nB
        let s = sp(c);
        let n_a = c.g[A1][0];
        c.sw(s.wrapping_add(0xc), n_a);
        c.g[S4][0] = 0; // simRider
        if (c.g[A0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            'sim: loop {
                c.cycles += 8;
                c.g[S2][0] = sx((c.g[A3][0] as u32) << 2); // k * 4
                c.g[S1][0] = 0; // heat
                c.g[S7][0] = c.g[S2][0];
                c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[S5][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.g[V0][0] = 0x53_0000;
                'heat: loop {
                    // Heat 0 and heats with the path byte clear: a fresh
                    // threshold. Else: a repeat of heat 0's score.
                    c.cycles += 7;
                    c.g[V0][0] = 0x53_5bc8;
                    c.g[V1][0] = c.lb(NEW_RACE_PATH_BYTE);
                    c.g[V0][0] = 1;
                    if c.g[S1][0] == 0 {
                        c.g[V0][0] = 0;
                    }
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = 0;
                    }
                    let fresh = c.g[V0][0] == 0;
                    c.g[V1][0] = 0x28;
                    if fresh {
                        c.cycles += 14;
                        let s = sp(c);
                        c.g[A0][0] = c.lw(s.wrapping_add(4)); // player
                        c.g[A1][0] = 0;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        call_ptr(c, 0x0014_7cb8, 0x0023_9c28)?;
                        let s = sp(c);
                        c.g[A0][0] = c.lw(s); // race
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S4][0];
                        call_ptr(c, 0x0014_53d0, 0x0023_9c3c)?;
                        c.g[A0][0] = 0x28;
                        c.mult(0, 4, 17, 4);
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        let slot = c.g[V1][0] as u32;
                        c.sw(slot, c.g[V0][0]);
                    } else {
                        c.cycles += 7;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        c.mult(0, 3, 17, 3);
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S7][0] as u32));
                        c.g[V1][0] = c.lw(c.g[S6][0] as u32);
                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        let slot = c.g[V0][0] as u32;
                        c.sw(slot, c.g[V1][0]);
                    }

                    c.cycles += 4;
                    c.g[S1][0] = c.g[S0][0];
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 3) as u64;
                    let more_heats = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x53_0000;
                    if more_heats {
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_9be0);
                        }
                        continue 'heat;
                    }

                    // The rider's score slot: the heat-0 score goes to the
                    // table and the simRider ranking is stored.
                    c.cycles += 15;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[A0][0] = 0x53_0000;
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6730));
                    c.g[V0][0] = 0x53_0000;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                    let rank_slot = c.g[V1][0] as u32;
                    c.sw(rank_slot, c.g[S4][0]);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S4][0] = c.g[FP][0];
                    let score_slot = c.g[A0][0] as u32;
                    c.sw(score_slot, c.g[A1][0]);
                    c.g[V1][0] = c.lw(sp(c).wrapping_add(8));
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more_sims = c.g[V0][0] != 0;
                    c.g[A3][0] = c.g[S5][0];
                    if more_sims {
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_9bc0);
                        }
                        continue 'sim;
                    }
                    break 'sim;
                }
            }
        }

        // Sort the score table: (nA + nB) rows.
        c.cycles += 17;
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0xc));
        c.g[V1][0] = c.lw(sp(c).wrapping_add(8));
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(GM_MANAGER_GP));
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, 0x0023_8b70, 0x0023_9cb0)?;

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
    })
}

/// `cGMMPSlopeHandler_raceFinish` (0x00239ec8, `GMHandler9_OnRiderFinished`):
/// a rider (`idx`) has finished. Copies the rider's totals (0xa8 bytes at
/// `rider->metrix + 0xfc`) to the frame, records its score (0 when it did
/// not finish, which also sets its did-not-finish flag) and, once the race is
/// over, locks the event and sorts the table.
pub fn gm_handler9_on_rider_finished(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save_ra(c, 0xb0);

        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(WORLD_GP));
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2); // idx * 4
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84)); // race
        c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc)); // riders
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28)); // rider
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790)); // metrix
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xfc)); // totals source
        c.g[V0][0] = c.g[V1][0] & 0x7;
        let aligned = c.g[V0][0] == 0;
        c.g[A2][0] = c.g[SP][0]; // the copy's destination cursor

        // Both copy loops share this set-up.
        c.cycles += 4;
        c.g[S0][0] = c.g[A1][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19c)); // copy end
        c.g[A1][0] = 0x53_0000;
        c.g[A3][0] = 0x53_0000;
        if aligned {
            'copy8: loop {
                c.cycles += 11;
                c.g[A0][0] = c.ld(c.g[V1][0] as u32);
                c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A2][0] as u32, c.g[A0][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let more = c.g[V1][0] != c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_9f74);
                    }
                    continue 'copy8;
                }
                c.cycles += 1;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(WORLD_GP));
                break;
            }
        } else {
            'copy_unaligned: loop {
                c.cycles += 19;
                c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(4, c.g[V1][0] as u32);
                c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.ldl(10, (c.g[V1][0] as u32).wrapping_add(0x17));
                c.ldr(10, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.ldl(11, (c.g[V1][0] as u32).wrapping_add(0x1f));
                c.ldr(11, (c.g[V1][0] as u32).wrapping_add(0x18));
                c.sdl(4, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.sdr(4, c.g[A2][0] as u32);
                c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.sdr(9, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sdl(10, (c.g[A2][0] as u32).wrapping_add(0x17));
                c.sdr(10, (c.g[A2][0] as u32).wrapping_add(0x10));
                c.sdl(11, (c.g[A2][0] as u32).wrapping_add(0x1f));
                c.sdr(11, (c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let more = c.g[V1][0] != c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_9f10);
                    }
                    continue 'copy_unaligned;
                }
                c.cycles += 2;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(WORLD_GP));
                break;
            }
        }

        // The copy's tail: 8 more bytes at the end of the frame copy.
        c.cycles += 13;
        c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[V1][0] as u32);
        c.g[T1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
        c.sdr(2, c.g[A2][0] as u32);
        c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T1][0]);
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84)); // race
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28)); // rider
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x480)); // did not finish
        let finished = c.g[A0][0] == 0;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6640)); // D_00536640
        if finished {
            c.cycles += 6;
            let s = sp(c);
            c.g[A0][0] = c.lw(s.wrapping_add(0x9c)); // st.TotalPoints
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x66a8));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let score_slot = c.g[V0][0] as u32;
            c.sw(score_slot, c.g[A0][0]);
            let dnf_slot = c.g[V1][0] as u32;
            c.sw(dnf_slot, 0);
        } else {
            c.cycles += 7;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x66a8));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = 1;
            let score_slot = c.g[V0][0] as u32;
            c.sw(score_slot, 0);
            let dnf_slot = c.g[V1][0] as u32;
            c.sw(dnf_slot, c.g[A0][0]);
        }

        // The race is over when cAI_isRaceFinished says so: lock the event
        // and sort the table; the rider that ranks first keeps no flag.
        c.cycles += 4;
        c.g[A0][0] = c.g[T0][0];
        call_ptr(c, 0x0012_a250, 0x0023_a014)?;
        let over = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(GM_MANAGER_GP));
        if over {
            c.cycles += 12;
            c.g[V1][0] = 1;
            c.g[A1][0] = 2;
            let flag_slot = (c.g[V0][0] as u32).wrapping_add(0x9c);
            c.sw(flag_slot, c.g[V1][0]); // bIsEventDone
            c.g[A0][0] = c.g[V0][0];
            let locked_slot = (c.g[V0][0] as u32).wrapping_add(0x84);
            c.sw(locked_slot, c.g[V1][0]); // bAreResultsLocked
            call_ptr(c, 0x0023_8b70, 0x0023_a034)?;
            c.g[V0][0] = 0x53_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32); // iRanking[idx]
            let ranked = (c.g[V1][0] as i64) > 0;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(WORLD_GP));
            if ranked {
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                let unk_slot = (c.g[V1][0] as u32).wrapping_add(0x100);
                c.sw(unk_slot, 0);
            }
        }

        c.cycles += 4;
        frame.restore(c, S0, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// The BE library singleton (`gp-0x1f04`), its interface table at
    /// `0x5308a8`, the interface and its method (returns 0).
    const STANDINGS_BE_LIB: u32 = DATA + 0x3000;
    const STANDINGS_IFACE: u32 = DATA + 0x3100;
    const STANDINGS_IFACE_VT: u32 = DATA + 0x3200;
    const STANDINGS_BE_IFACES: u32 = 0x0053_08a8;
    const STANDINGS_RETURN_ZERO: u32 = 0x003a_5ab0;
    /// The game-mode manager.
    const STANDINGS_MANAGER: u32 = DATA + 0x1_0000;

    /// Game state for `gm_handler0_init_standings`: the BE singleton and
    /// interfaces (race and player read as 0), the manager's counts, and the
    /// race-path byte.
    fn standings_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0023_9aa0 ^ (seed as u64) << 20 | 1);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP - 0x1f04, STANDINGS_BE_LIB);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(STANDINGS_BE_IFACES, STANDINGS_IFACE);
        c.bus.write32(STANDINGS_IFACE + 4, STANDINGS_IFACE_VT);
        c.bus.write16(STANDINGS_IFACE_VT + 8, 0);
        c.bus.write32(STANDINGS_IFACE_VT + 0xc, STANDINGS_RETURN_ZERO);
        c.bus.write32(GAME_GP - GM_MANAGER_GP, STANDINGS_MANAGER);
        // iNumberAI 0..5, iSimAI 0..3: numNonSimulated can be negative.
        c.bus.write32(STANDINGS_MANAGER + 0x10, seed % 6);
        c.bus.write32(STANDINGS_MANAGER + 0x14, (seed / 6) % 4);
        // The race path: repeats of heat 1 or fresh thresholds.
        c.bus.write8(NEW_RACE_PATH_BYTE, ((seed / 3) % 3 != 0) as u8);
    }

    #[test]
    fn gm_handler0_init_standings_matches() {
        check(
            ee::s06::f_00239aa0__generated,
            gm_handler0_init_standings,
            240,
            &|c, seed| {
                standings_fixture(c, seed);
                c.pc = 0x0023_9aa0;
            },
        );
    }

    /// Rider-finished fixture: the world, race, riders table and one rider
    /// per slot (did-not-finish flag by seed), each with a metrix whose
    /// totals sit at an aligned or unaligned address by seed.
    const FINISH_WORLD: u32 = DATA + 0x4000;
    const FINISH_RACE: u32 = DATA + 0x4100;
    const FINISH_RIDERS: u32 = DATA + 0x4200;
    const FINISH_RIDER0: u32 = DATA + 0x4400;
    const FINISH_METRIX0: u32 = DATA + 0x2_0000;
    const FINISH_MANAGER: u32 = DATA + 0x1_0000;

    fn finish_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0023_9ec8 ^ (seed as u64) << 20 | 1);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        let idx = seed % 4;
        c.g[A1][0] = idx as u64;
        c.bus.write32(GAME_GP - WORLD_GP, FINISH_WORLD);
        c.bus.write32(FINISH_WORLD + 0x84, FINISH_RACE);
        c.bus.write32(FINISH_RACE + 0xc, FINISH_RIDERS);
        for i in 0..4u32 {
            let rider = FINISH_RIDER0 + i * 0x800;
            let metrix = FINISH_METRIX0 + i * 0x200 + (seed % 2) * 4;
            c.bus.write32(FINISH_RIDERS + 4 * i + 0x28, rider);
            c.bus.write32(rider + 0x790, metrix);
            c.bus.write32(rider + 0x480, (seed >> i) & 1);
            for k in 0..0x2c {
                c.bus.write32(metrix + 0xfc + 4 * k, r.next_u32());
            }
        }
        c.bus.write32(GAME_GP - GM_MANAGER_GP, FINISH_MANAGER);
        c.bus.write32(FINISH_MANAGER + 0x10, 3);
        c.bus.write32(FINISH_MANAGER + 0x14, 2);
        // Rank of this rider: first, second, or out of the points.
        // cGameModeMan_iRanking (0x536730)
        c.bus.write32(0x0053_6730 + 4 * idx, (seed % 3).wrapping_sub(1));
    }

    #[test]
    fn gm_handler9_on_rider_finished_matches() {
        check(
            ee::s06::f_00239ec8__generated,
            gm_handler9_on_rider_finished,
            240,
            &|c, seed| {
                finish_fixture(c, seed);
                c.pc = 0x0023_9ec8;
            },
        );
    }
}
