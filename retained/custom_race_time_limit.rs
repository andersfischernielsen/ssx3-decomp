//! Native race deadline and network-checksum maintenance (0x12bb20).
//!
//! AI+8 is the AI tick counter; +0xc is the separate race clock. AI+0x18
//! holds a post-finisher DNF grace countdown, not the Slopestyle time budget.
//! Generated EE is authoritative; the C++ reference is a readability aid.
//! No tests were added or run, as requested by the user.

use crate::call::{call_ptr, guest, Callee, Flow, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0012_bb20,
    ai_update_race_time_limit,
    "AI_UpdateRaceTimeLimit",
)];

const AI_TICKS: u32 = 8;
const DNF_COUNTDOWN: u32 = 0x18;
const HUMAN_PLAYERS: u32 = 0x40;
const HUMAN_COUNT: u32 = 0x7c;
const PLAYER_RIDER: u32 = 0x18;
const FINISHED_TIMER: u32 = 0x470;
const DID_NOT_FINISH: u32 = 0x480;

const CHECKSUM: Callee = Callee {
    addr: 0x0012_b420,
    f: ssx3_ee::s00::AI_GetSyncChecksum,
};
const ALL_FINISHED: Callee = Callee {
    addr: 0x0012_a250,
    f: ssx3_ee::s00::AI_AllPlayersFinished,
};
const ON_FINISH: Callee = Callee {
    addr: 0x0012_5108,
    f: ssx3_ee::s00::Rider_OnFinishRace,
};

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}
#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 {
    r(c, reg).wrapping_add(offset)
}
#[inline]
fn global(c: &Cpu, negative: u32) -> u32 {
    r(c, GP).wrapping_sub(negative)
}

/// Mark each unfinished human DNF. Both expiry paths have the same loop,
/// with different original return/back-edge addresses. The caller retains
/// the distinct charge and world load after the loop.
fn finish_unfinished_humans(c: &mut Cpu, head: u32, ret: u32) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = c.lw(at(c, S1, HUMAN_COUNT));
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[S2][0] = 0;
    if empty {
        return Ok(false);
    }
    c.cycles += 3;
    c.mtc1(ZERO, 20);
    c.g[S3][0] = 1;
    c.g[S0][0] = sx(at(c, S1, HUMAN_PLAYERS));
    loop {
        c.cycles += 7;
        c.g[V0][0] = c.lw(r(c, S0));
        c.g[V0][0] = c.lw(at(c, V0, PLAYER_RIDER));
        c.lwc1(0, at(c, V0, FINISHED_TIMER));
        c.c_le_s(20, 0);
        let finished = c.fcc();
        c.g[V1][0] = 1;
        if !finished {
            c.cycles += 1;
            c.g[V1][0] = 0;
        }
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.cycles += 5;
            c.sw(at(c, V0, DID_NOT_FINISH), c.g[S3][0]);
            c.g[V0][0] = c.lw(r(c, S0));
            c.g[A0][0] = c.lw(at(c, V0, PLAYER_RIDER));
            ON_FINISH.call(c, ret)?;
        }
        c.g[V0][0] = c.lw(at(c, S1, HUMAN_COUNT));
        c.cycles += 4;
        c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
        if !more {
            return Ok(true);
        }
        if back_edge_unwinds(c, head) {
            return Err(Unwound);
        }
    }
}

/// Finalize the event, clear the grace countdown and leave v0 at the world.
fn finalize_deadline(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[V1][0] = c.lw(at(c, V0, 0x84));
    c.g[A0][0] = c.lw(at(c, V1, 0x28));
    call_ptr(c, 0x0026_fa50, 0x0012_bda4)?;
    c.g[V0][0] = u64::MAX;
    c.sw(at(c, S1, DNF_COUNTDOWN), c.g[V0][0]);
    c.cycles += 1;
    c.g[V0][0] = c.lw(global(c, 0x848));
    Ok(())
}

/// Race/Rival Race only: update an existing grace deadline, or consider
/// seeding it from a finisher and enforce the hard AI-counter deadline.
fn update_deadline(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = c.lw(at(c, S1, DNF_COUNTDOWN));
    if (c.g[V0][0] as i64) >= 0 {
        c.cycles += 2;
        let nonpositive = (c.g[V0][0] as i64) <= 0;
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
        if !nonpositive {
            c.cycles += 2;
            let pending = (c.g[V0][0] as i64) > 0;
            c.sw(at(c, S1, DNF_COUNTDOWN), c.g[V0][0]);
            if !pending {
                let had_humans = finish_unfinished_humans(c, 0x0012_bc18, 0x0012_bc50)?;
                // Distinct original tail charges after a nonempty/empty loop.
                c.cycles += if had_humans { 2 } else { 1 };
                c.g[V0][0] = c.lw(global(c, 0x848));
                return finalize_deadline(c);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw(global(c, 0x848));
        return Ok(());
    }

    c.g[V1][0] = c.lw(at(c, S1, AI_TICKS));
    c.cycles += 4;
    c.g[V0][0] = 0xd32b;
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        ALL_FINISHED.call(c, 0x0012_bc84)?;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, S1, HUMAN_COUNT));
            if (c.g[V0][0] as i64) > 0 {
                c.cycles += 6;
                c.mtc1(ZERO, 1);
                c.g[A2][0] = c.g[V0][0];
                c.g[A3][0] = c.lw(global(c, 0x204));
                c.g[T1][0] = 0xd3a4;
                c.g[T0][0] = 0x1c20;
                c.g[A1][0] = sx(at(c, S1, HUMAN_PLAYERS));
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(r(c, A1));
                    c.g[V1][0] = c.lw(at(c, V0, PLAYER_RIDER));
                    c.lwc1(0, at(c, V1, FINISHED_TIMER));
                    c.c_le_s(1, 0);
                    let finished = c.fcc();
                    c.g[A0][0] = 1;
                    if !finished {
                        c.cycles += 1;
                        c.g[A0][0] = 0;
                    }
                    c.cycles += 2;
                    if c.g[A0][0] == 0 {
                        c.g[A2][0] = sx(r(c, A2).wrapping_sub(1));
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, A3, 0x8c));
                        let blocked = c.g[V0][0] != 0;
                        c.g[A2][0] = sx(r(c, A2).wrapping_sub(1));
                        if !blocked {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(at(c, A3, 0x48));
                            if c.g[V0][0] == 0 {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw(at(c, S1, AI_TICKS));
                                c.g[V0][0] = sx(r(c, T1).wrapping_sub(r(c, V0)));
                                c.g[V1][0] = ((c.g[V0][0] as i64) < 7201) as u64;
                                if c.g[V1][0] == 0 {
                                    c.g[V0][0] = c.g[T0][0];
                                }
                                c.sw(at(c, S1, DNF_COUNTDOWN), c.g[V0][0]);
                            }
                        }
                    }
                    c.cycles += 2;
                    let more = c.g[A2][0] != 0;
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(4));
                    if !more {
                        break;
                    }
                    if back_edge_unwinds(c, 0x0012_bcb0) {
                        return Err(Unwound);
                    }
                }
                c.cycles += 1;
            }
        }
    }
    c.g[V1][0] = c.lw(at(c, S1, AI_TICKS));
    c.cycles += 4;
    c.g[V0][0] = 0xd3a3;
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let within = c.g[V0][0] == 0;
    c.g[V0][0] = c.lw(global(c, 0x848));
    if within {
        return Ok(());
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    ALL_FINISHED.call(c, 0x0012_bd28)?;
    let all_finished = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(global(c, 0x848));
    if all_finished {
        return Ok(());
    }
    finish_unfinished_humans(c, 0x0012_bd48, 0x0012_bd80)?;
    c.cycles += 1;
    c.g[V0][0] = c.lw(global(c, 0x848));
    finalize_deadline(c)
}

fn finish_frame(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x60);
    frame.close(c);
    Ok(())
}

/// `void AI_UpdateRaceTimeLimit(AI* ai)` (0x12bb20).
/// Sample and submit network sync once per 60 AI ticks. In Race/Rival Race,
/// unfinished humans become DNF when a post-finisher grace period expires
/// or the native hard deadline is reached. This routine never writes the
/// Slopestyle budget or the race clock at AI+0xc.
pub fn ai_update_race_time_limit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.lw(global(c, 0x848));
        frame.save(c, S1, 0x40);
        c.g[A1][0] = 7;
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x60);
        c.g[A0][0] = c.lw(at(c, V0, 0x78));
        call_ptr(c, 0x0014_dd58, 0x0012_bb54)?;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = c.lw(at(c, V1, 0x4b30));
        let disabled = c.g[V0][0] == 0;
        c.g[V0][0] = 60;
        if disabled {
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
            c.cycles += 1;
            frame.restore(c, S1, 0x40);
            return finish_frame(c, frame);
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(at(c, S1, AI_TICKS));
        c.cycles += 4;
        c.div(0, A0, V0);
        c.g[V1][0] = c.hi[0];
        let not_sync_tick = c.g[V1][0] != 0;
        c.g[S4][0] = 0;
        if !not_sync_tick {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            CHECKSUM.call(c, 0x0012_bb88)?;
            c.g[S4][0] = c.g[V0][0];
        }
        c.cycles += 4;
        c.g[A1][0] = c.lw(r(c, S1));
        c.g[V0][0] = 5;
        let active = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = 3;
        if !active {
            c.cycles += 2;
            let wrong_state = c.g[A1][0] != c.g[V0][0];
            frame.restore(c, S0, 0x50);
            if wrong_state {
                c.cycles += 1;
                frame.restore(c, S1, 0x40);
                return finish_frame(c, frame);
            }
            c.cycles += 5;
            c.g[V0][0] = c.lw(global(c, 0x848));
            c.g[V1][0] = c.lw(at(c, V0, 0x84));
            c.g[A0][0] = c.lw(at(c, V1, 0x214));
            if c.g[A0][0] != c.g[A1][0] {
                frame.restore(c, S1, 0x40);
                return finish_frame(c, frame);
            }
        }
        c.cycles += 9;
        call_ptr(c, 0x0014_dc80, 0x0012_bbc0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0012_bbcc)?;
        c.g[V1][0] = 0x530000;
        c.g[V1][0] = c.lb(at(c, V1, 0x5c12));
        let normal_race = c.g[V1][0] == 0;
        c.g[V0][0] = 4;
        if normal_race {
            update_deadline(c)?;
        } else {
            c.cycles += 2;
            let rival_race = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = c.lw(global(c, 0x848));
            if rival_race {
                update_deadline(c)?;
            }
        }
        c.cycles += 5;
        c.g[V1][0] = c.lw(at(c, V0, 0x84));
        c.g[A0][0] = c.lw(at(c, V1, 0x28));
        c.g[A0][0] = c.lw(r(c, A0));
        let empty = c.g[A0][0] == 0;
        c.g[A1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[A1][0] = ((c.g[A0][0] as i64) < 10) as u64;
        }
        c.cycles += 2;
        let busy = c.g[A1][0] != 0;
        frame.restore(c, S0, 0x50);
        if busy {
            c.cycles += 1;
            frame.restore(c, S1, 0x40);
            return finish_frame(c, frame);
        }
        c.cycles += 4;
        c.g[A1][0] = c.lw(at(c, S1, AI_TICKS));
        c.g[V0][0] = 60;
        c.cycles += 4;
        c.div(0, A1, V0);
        c.g[V1][0] = c.hi[0];
        let not_sync_tick = c.g[V1][0] != 0;
        frame.restore(c, S0, 0x50);
        if not_sync_tick {
            c.cycles += 1;
            frame.restore(c, S1, 0x40);
            return finish_frame(c, frame);
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(global(c, 0x204));
        c.g[A2][0] = c.g[S4][0];
        call_ptr(c, 0x0025_8f20, 0x0012_bdfc)?;
        c.cycles += 1;
        frame.restore(c, S0, 0x50);
        c.cycles += 1;
        frame.restore(c, S1, 0x40);
        finish_frame(c, frame)
    })
}
