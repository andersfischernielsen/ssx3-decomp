//! Native GMHandler0_OnRiderFinished at 0x239230, retained uninstalled.
//!
//! Generated EE is authoritative. No test, build, replay or game execution
//! is added or run: the user has disabled automated validation.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0023_9230;
const END: u32 = 0x0023_98e8;
const EPILOGUE: u32 = 0x0023_98b0;
const FRAME_BYTES: u32 = 0x160;
const APP_GP: i32 = -0x848;
const MODE_MANAGER_GP: i32 = -0x480;
const TOTALS: u32 = 0x0053_6640;
const QUICK_EVENT: u32 = 0x0053_5c11;
const GET_BE: u32 = 0x0014_dc80;
const GET_INTERFACE: u32 = 0x0014_dd58;
const GET_PLAYER_INDEX: u32 = 0x0014_6e98;
const FINISH_GATE: u32 = 0x0012_a250;
const SORT_BY_SCORE: u32 = 0x0023_8b70;
const ESTIMATE_SCORE: u32 = 0x0012_2e50;
const RECORD_WIN: u32 = 0x0014_7d20;
const RECORD_LOSS: u32 = 0x0014_7e18;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}
#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}
#[inline]
fn gp(c: &Cpu, offset: i32) -> u32 {
    r(c, GP).wrapping_add(offset as u32)
}

/// Copy one 0x20-byte group while preserving the original aligned/unaligned
/// scratch registers. The final 0x0c bytes are copied separately at 0x239374.
fn copy_stats_group(c: &mut Cpu, unaligned: bool) {
    if unaligned {
        c.cycles += 19;
        for (register, offset) in [(A0, 0), (A2, 8), (T0, 0x10), (T1, 0x18)] {
            c.ldl(register, at(c, V1, offset + 7));
            c.ldr(register, at(c, V1, offset));
        }
        for (register, offset) in [(A0, 0), (A2, 8), (T0, 0x10), (T1, 0x18)] {
            c.sdl(register, at(c, A1, offset + 7));
            c.sdr(register, at(c, A1, offset));
        }
    } else {
        c.cycles += 11;
        c.g[A0][0] = c.ld(r(c, V1));
        c.g[A2][0] = c.ld(at(c, V1, 8));
        c.g[T0][0] = c.ld(at(c, V1, 0x10));
        c.g[T1][0] = c.ld(at(c, V1, 0x18));
        c.sd(r(c, A1), c.g[A0][0]);
        c.sd(at(c, A1, 8), c.g[A2][0]);
        c.sd(at(c, A1, 0x10), c.g[T0][0]);
        c.sd(at(c, A1, 0x18), c.g[T1][0]);
    }
    c.g[V1][0] = sx(at(c, V1, 0x20));
    c.g[A1][0] = sx(at(c, A1, 0x20));
}

/// Sum the first two heat rows into the native sorting buffer. Both original
/// copies of this loop have the same instructions and 12-cycle block charge.
fn sum_heat_group(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = c.lw(r(c, A1));
    c.g[A2][0] = sx(at(c, A2, 1));
    c.g[V1][0] = c.lw(r(c, A0));
    c.g[A1][0] = sx(at(c, A1, 4));
    c.g[A0][0] = sx(at(c, A0, 4));
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
    c.sw(r(c, A3), c.g[V0][0]);
    c.g[V0][0] = c.lw(at(c, T0, 0x10));
    c.g[V0][0] = sx(at(c, V0, 1));
    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[A3][0] = sx(at(c, A3, 4));
}

/// Copy a selected heat's row back to the native sorting buffer. Loop state
/// remains in the original registers for native event/call continuations.
fn copy_score_group(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = c.lw(r(c, V1));
    c.g[A0][0] = sx(at(c, A0, 1));
    c.g[V1][0] = sx(at(c, V1, 4));
    c.sw(r(c, A1), c.g[V0][0]);
    c.g[V0][0] = c.lw(at(c, A2, 0x10));
    c.g[V0][0] = sx(at(c, V0, 1));
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[A1][0] = sx(at(c, A1, 4));
}

/// `void GMHandler0_OnRiderFinished(Handler0* self, int rider_index)`.
/// Store the rider's metrix score in the current heat row (zero for DNF).
/// Only after the AI finish gate at 0x12a250 succeeds, update/sort the heat
/// standings, qualification state and local-player placement feedback.
/// Heat 3 alone marks GameModeMan+9c ready and estimates unfinished riders'
/// values. This function neither reads nor writes the separate +98 gate.
/// All native call returns and loop heads accept direct re-entry.
pub fn gm_handler0_on_rider_finished(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 40;
                    Frame::open(c, FRAME_BYTES);
                    frame.save(c, S0, 0x140);
                    frame.save(c, S4, 0x100);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, S1, 0x130);
                    c.g[S4][0] = c.g[A0][0];
                    frame.save(c, S5, 0xf0);
                    frame.save(c, S6, 0xe0);
                    frame.save(c, S2, 0x120);
                    frame.save(c, S3, 0x110);
                    frame.save(c, S7, 0xd0);
                    frame.save(c, FP, 0xc0);
                    frame.save_ra(c, 0xb0);
                    frame.save_fpr(c, 20, 0x150);
                    call_ptr(c, GET_BE, 0x0023_926c)?;
                    pc = 0x0023_926c;
                }
                0x0023_926c => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, GET_INTERFACE, 0x0023_9278)?;
                    pc = 0x0023_9278;
                }
                0x0023_9278 => {
                    c.g[S0][0] = sx(r(c, S0) << 2);
                    call_ptr(c, GET_BE, 0x0023_9280)?;
                    pc = 0x0023_9280;
                }
                0x0023_9280 => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 1;
                    call_ptr(c, GET_INTERFACE, 0x0023_928c)?;
                    pc = 0x0023_928c;
                }
                0x0023_928c => {
                    c.g[S6][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    c.g[V0][0] = 0x0053_0000;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[S5][0] = c.lb(QUICK_EVENT);
                    call_ptr(c, GET_PLAYER_INDEX, 0x0023_92a4)?;
                    pc = 0x0023_92a4;
                }
                0x0023_92a4 => {
                    c.g[A0][0] = c.lw(gp(c, APP_GP));
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw(at(c, A0, 0x84));
                    c.g[V0][0] = c.lw(at(c, V1, 0xc));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S0)));
                    c.g[V1][0] = c.lw(at(c, V0, 0x28));
                    c.g[A0][0] = c.lw(at(c, V1, 0x790));
                    c.g[V1][0] = sx(at(c, A0, 0xfc));
                    c.g[V0][0] = c.g[V1][0] & 7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[SP][0];
                    c.cycles += 4;
                    c.g[S2][0] = c.g[S0][0];
                    c.g[V0][0] = sx(at(c, A0, 0x19c));
                    c.g[S0][0] = sx(at(c, S4, 4));
                    c.g[A3][0] = 0x0053_0000;
                    pc = if aligned { 0x0023_9344 } else { 0x0023_92e0 };
                }
                0x0023_92e0 | 0x0023_9344 => {
                    let unaligned = pc == 0x0023_92e0;
                    copy_stats_group(c, unaligned);
                    if c.g[V1][0] != c.g[V0][0] {
                        if back_edge_unwinds(c, pc) {
                            return Err(crate::call::Unwound);
                        }
                    } else {
                        c.cycles += if unaligned { 2 } else { 1 };
                        c.g[A0][0] = c.lw(gp(c, APP_GP));
                        pc = 0x0023_9374;
                    }
                }
                0x0023_9374 => {
                    c.cycles += 13;
                    c.ldl(V0, at(c, V1, 7));
                    c.ldr(V0, r(c, V1));
                    c.g[A2][0] = c.lw(at(c, V1, 8));
                    c.sdl(V0, at(c, A1, 7));
                    c.sdr(V0, r(c, A1));
                    c.sw(at(c, A1, 8), c.g[A2][0]);
                    c.g[V1][0] = c.lw(at(c, A0, 0x84));
                    c.g[V0][0] = c.lw(at(c, V1, 0xc));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S2)));
                    c.g[V1][0] = c.lw(at(c, V0, 0x28));
                    c.g[A0][0] = c.lw(at(c, V1, 0x480));
                    let dnf = c.g[A0][0] != 0;
                    c.g[A1][0] = 0x28;
                    c.cycles += if dnf { 12 } else { 11 };
                    c.g[A0][0] = c.lw(gp(c, MODE_MANAGER_GP));
                    if dnf {
                        c.g[V1][0] = sx(at(c, A3, 0x66a8));
                        c.g[A2][0] = 1;
                    } else {
                        c.g[A2][0] = c.lw(at(c, SP, 0x9c));
                        c.g[V1][0] = sx(at(c, A3, 0x66a8));
                    }
                    c.g[V0][0] = c.lw(r(c, A0));
                    c.g[V1][0] = sx(r(c, S2).wrapping_add(r(c, V1)));
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
                    c.mult(0, A0, V0, A1);
                    c.g[V0][0] = sx(r(c, A0).wrapping_add(r(c, S2)));
                    c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
                    c.sw(r(c, V0), if dnf { 0 } else { c.g[A2][0] });
                    c.sw(r(c, V1), if dnf { c.g[A2][0] } else { 0 });
                    pc = 0x0023_9404;
                }
                0x0023_9404 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(gp(c, APP_GP));
                    c.g[V1][0] = c.lw(at(c, V0, 0x84));
                    c.g[A0][0] = c.lw(at(c, V1, 0xc));
                    call_ptr(c, FINISH_GATE, 0x0023_9414)?;
                    pc = 0x0023_9414;
                }
                0x0023_9414 => {
                    let finish_ready = c.g[V0][0] != 0;
                    c.g[A1][0] = c.lw(gp(c, MODE_MANAGER_GP));
                    if !finish_ready {
                        pc = EPILOGUE;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = 2;
                    c.g[V1][0] = c.lw(r(c, A1));
                    let second_heat = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
                    if second_heat {
                        pc = 0x0023_95ac;
                        continue;
                    }
                    c.cycles += 2;
                    let third_or_other = c.g[V0][0] == 0;
                    c.g[V0][0] = 1;
                    if third_or_other {
                        c.cycles += 3;
                        c.g[V0][0] = 3;
                        let third_heat = c.g[V1][0] == c.g[V0][0];
                        c.g[S7][0] = sx(r(c, S1) << 2);
                        if third_heat {
                            pc = 0x0023_96d8;
                            continue;
                        }
                    } else {
                        c.cycles += 2;
                        let first_heat = c.g[V1][0] == c.g[V0][0];
                        c.g[S7][0] = sx(r(c, S1) << 2);
                        if first_heat {
                            pc = 0x0023_9458;
                            continue;
                        }
                    }
                    c.cycles += 2;
                    c.g[FP][0] = 0x0053_0000;
                    pc = 0x0023_9854;
                }
                0x0023_9458 => {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(at(c, A1, 0x70));
                    c.g[V0][0] = c.lw(at(c, A1, 0x10));
                    c.g[A2][0] = 0;
                    c.sw(at(c, A1, 0x74), c.g[V1][0]);
                    c.g[V0][0] = sx(at(c, V0, 1));
                    let have_scores = (c.g[V0][0] as i64) > 0;
                    c.g[FP][0] = 0x0053_0000;
                    if have_scores {
                        setup_sum(c);
                        pc = 0x0023_9488;
                    } else {
                        pc = 0x0023_94b8;
                    }
                }
                0x0023_9488 | 0x0023_95d8 => {
                    sum_heat_group(c);
                    if c.g[V0][0] != 0 {
                        if back_edge_unwinds(c, pc) {
                            return Err(crate::call::Unwound);
                        }
                    } else {
                        pc = if pc == 0x0023_9488 {
                            0x0023_94b8
                        } else {
                            0x0023_9608
                        };
                    }
                }
                0x0023_94b8 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw(gp(c, MODE_MANAGER_GP));
                    c.g[A1][0] = c.lw(at(c, A0, 0x10));
                    c.g[A1][0] = sx(at(c, A1, 1));
                    call_ptr(c, SORT_BY_SCORE, 0x0023_94c8)?;
                    pc = 0x0023_94c8;
                }
                0x0023_94c8 => {
                    c.g[V0][0] = sx(at(c, FP, 0x6730));
                    c.g[V0][0] = sx(r(c, S2).wrapping_add(r(c, V0)));
                    c.g[V1][0] = c.lw(r(c, V0));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 3) as u64;
                    let qualified = c.g[V1][0] != 0;
                    c.g[V1][0] = c.lw(gp(c, MODE_MANAGER_GP));
                    c.cycles += 2;
                    if qualified {
                        c.g[V0][0] = 3;
                    } else {
                        c.g[V0][0] = c.lw(r(c, V1));
                        c.g[V0][0] = sx(at(c, V0, 1));
                    }
                    pc = 0x0023_94f0;
                }
                0x0023_94f0 => {
                    c.cycles += 6;
                    c.sw(at(c, V1, 0x70), c.g[V0][0]);
                    c.g[A2][0] = c.lw(gp(c, MODE_MANAGER_GP));
                    c.g[V0][0] = c.lw(at(c, A2, 0x10));
                    c.g[V0][0] = sx(at(c, V0, 1));
                    let have_scores = (c.g[V0][0] as i64) > 0;
                    c.g[A0][0] = 0;
                    if have_scores {
                        c.cycles += 4;
                        c.g[V0][0] = 0x0053_0000;
                        c.g[V1][0] = c.g[S0][0];
                        c.g[A1][0] = sx(TOTALS);
                        pc = 0x0023_9518;
                    } else {
                        pc = 0x0023_953c;
                    }
                }
                0x0023_9518 | 0x0023_9638 | 0x0023_9780 => {
                    copy_score_group(c);
                    if c.g[V0][0] != 0 {
                        if back_edge_unwinds(c, pc) {
                            return Err(crate::call::Unwound);
                        }
                    } else {
                        pc = match pc {
                            0x0023_9518 => 0x0023_953c,
                            0x0023_9638 => 0x0023_965c,
                            _ => 0x0023_97a4,
                        };
                    }
                }
                0x0023_953c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx(at(c, FP, 0x6730));
                    c.g[S1][0] = sx(r(c, S2).wrapping_add(r(c, V1)));
                    c.g[V0][0] = c.lw(r(c, S1));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
                    let qualified = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lw(gp(c, APP_GP));
                    if qualified {
                        pc = 0x0023_9590;
                        continue;
                    }
                    c.cycles += 8;
                    c.g[A0][0] = c.lw(gp(c, MODE_MANAGER_GP));
                    c.g[A1][0] = c.lw(at(c, A0, 0x10));
                    c.g[A1][0] = sx(at(c, A1, 1));
                    call_ptr(c, SORT_BY_SCORE, 0x0023_9564)?;
                    pc = 0x0023_9564;
                }
                0x0023_9564 => {
                    c.g[V0][0] = c.lw(r(c, S1));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
                    let qualified = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lw(gp(c, APP_GP));
                    if qualified {
                        pc = 0x0023_9590;
                        continue;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(gp(c, APP_GP));
                    c.g[A0][0] = c.lw(at(c, V1, 0x84));
                    c.g[V0][0] = c.lw(at(c, A0, 0xc));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S2)));
                    c.g[V1][0] = c.lw(at(c, V0, 0x28));
                    c.sw(at(c, V1, 0x100), 0);
                    pc = 0x0023_9854;
                }
                0x0023_9590 => {
                    c.cycles += 7;
                    c.g[A1][0] = 1;
                    c.g[V1][0] = c.lw(at(c, A0, 0x84));
                    c.g[V0][0] = c.lw(at(c, V1, 0xc));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S2)));
                    c.g[V1][0] = c.lw(at(c, V0, 0x28));
                    c.sw(at(c, V1, 0x100), c.g[A1][0]);
                    pc = 0x0023_9854;
                }
                0x0023_95ac => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(at(c, A1, 0x10));
                    c.g[S7][0] = sx(r(c, S1) << 2);
                    c.g[A2][0] = 0;
                    c.g[V0][0] = sx(at(c, V0, 1));
                    let have_scores = (c.g[V0][0] as i64) > 0;
                    c.g[FP][0] = 0x0053_0000;
                    if have_scores {
                        c.cycles += 5;
                        c.g[V0][0] = 0x0053_0000;
                        c.g[T0][0] = c.g[A1][0];
                        c.g[A3][0] = sx(TOTALS);
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = sx(at(c, S4, 0x2c));
                        pc = 0x0023_95d8;
                    } else {
                        pc = 0x0023_9608;
                    }
                }
                0x0023_9608 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw(gp(c, MODE_MANAGER_GP));
                    c.g[A1][0] = c.lw(at(c, A0, 0x10));
                    c.g[A1][0] = sx(at(c, A1, 1));
                    call_ptr(c, SORT_BY_SCORE, 0x0023_9618)?;
                    pc = 0x0023_9618;
                }
                0x0023_9618 => {
                    c.g[A2][0] = c.lw(gp(c, MODE_MANAGER_GP));
                    c.g[V0][0] = c.lw(at(c, A2, 0x10));
                    c.g[V0][0] = sx(at(c, V0, 1));
                    let have_scores = (c.g[V0][0] as i64) > 0;
                    c.g[A0][0] = 0;
                    if have_scores {
                        c.cycles += 3;
                        c.g[V0][0] = 0x0053_0000;
                        c.g[V1][0] = sx(at(c, S4, 0x2c));
                        c.g[A1][0] = sx(TOTALS);
                        pc = 0x0023_9638;
                    } else {
                        pc = 0x0023_965c;
                    }
                }
                0x0023_965c => {
                    c.cycles += 9;
                    c.g[A1][0] = c.lw(gp(c, MODE_MANAGER_GP));
                    c.g[V0][0] = sx(at(c, FP, 0x6730));
                    c.g[A2][0] = sx(r(c, S2).wrapping_add(r(c, V0)));
                    c.g[V1][0] = c.lw(at(c, A1, 0x70));
                    c.sw(at(c, A1, 0x74), c.g[V1][0]);
                    c.g[V0][0] = c.lw(r(c, A2));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
                    let qualified = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lw(gp(c, APP_GP));
                    if !qualified {
                        c.cycles += 5;
                        c.g[V0][0] = 1;
                        c.sw(at(c, A1, 0x70), c.g[V0][0]);
                        c.g[V1][0] = c.lw(at(c, A0, 0x84));
                        c.g[V0][0] = c.lw(at(c, V1, 0xc));
                        pc = 0x0023_9804;
                        continue;
                    }
                    c.cycles += 9;
                    c.g[V0][0] = 3;
                    c.sw(at(c, A1, 0x70), c.g[V0][0]);
                    c.g[A1][0] = 1;
                    c.g[V1][0] = c.lw(at(c, A0, 0x84));
                    c.g[V0][0] = c.lw(at(c, V1, 0xc));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S2)));
                    c.g[V1][0] = c.lw(at(c, V0, 0x28));
                    c.sw(at(c, V1, 0x100), c.g[A1][0]);
                    if c.g[S5][0] != 0 {
                        pc = 0x0023_9854;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, A2));
                    let not_first = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(at(c, FP, 0x6730));
                    if not_first {
                        pc = 0x0023_9858;
                        continue;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, RECORD_WIN, 0x0023_96bc)?;
                    pc = 0x0023_96bc;
                }
                0x0023_96bc => {
                    c.g[V0][0] = sx(at(c, FP, 0x6730));
                    pc = 0x0023_9858;
                }
                0x0023_96d8 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw(gp(c, APP_GP));
                    c.g[V0][0] = 1;
                    c.sw(at(c, A1, 0x9c), c.g[V0][0]);
                    c.g[S0][0] = 0;
                    c.g[V0][0] = c.lw(at(c, A0, 0x84));
                    c.g[V1][0] = c.lw(at(c, V0, 0xc));
                    c.g[A0][0] = c.lw(at(c, V1, 0x80));
                    c.g[A0][0] = sx(at(c, A0, 1));
                    let have_riders = (c.g[A0][0] as i64) > 0;
                    c.g[FP][0] = 0x0053_0000;
                    if have_riders {
                        c.cycles += 4;
                        c.mtc1(ZERO, 20);
                        c.g[S3][0] = c.g[A0][0];
                        c.g[S1][0] = sx(at(c, S4, 0x54));
                        pc = 0x0023_9710;
                    } else {
                        pc = 0x0023_9760;
                    }
                }
                0x0023_9710 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.lw(gp(c, APP_GP));
                    c.g[A1][0] = sx(r(c, S0) << 2);
                    c.g[V1][0] = c.lw(at(c, A0, 0x84));
                    c.g[V0][0] = c.lw(at(c, V1, 0xc));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A1)));
                    c.g[A0][0] = c.lw(at(c, V0, 0x28));
                    c.lwc1(0, at(c, A0, 0x470));
                    c.c_le_s(20, 0);
                    let finished = c.fcc();
                    c.g[A2][0] = 1;
                    if !finished {
                        c.cycles += 1;
                        c.g[A2][0] = 0;
                    }
                    c.cycles += 2;
                    let skip_estimate = c.g[A2][0] != 0;
                    c.g[S0][0] = sx(at(c, S0, 1));
                    if skip_estimate {
                        pc = 0x0023_9754;
                        continue;
                    }
                    c.cycles += 3;
                    call_ptr(c, ESTIMATE_SCORE, 0x0023_9750)?;
                    pc = 0x0023_9750;
                }
                0x0023_9750 => {
                    c.sw(r(c, S1), c.g[V0][0]);
                    pc = 0x0023_9754;
                }
                0x0023_9754 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx(at(c, S1, 4));
                    if more {
                        if back_edge_unwinds(c, 0x0023_9710) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x0023_9710;
                    } else {
                        pc = 0x0023_9760;
                    }
                }
                0x0023_9760 => {
                    c.cycles += 5;
                    c.g[A2][0] = c.lw(gp(c, MODE_MANAGER_GP));
                    c.g[V0][0] = c.lw(at(c, A2, 0x10));
                    c.g[V0][0] = sx(at(c, V0, 1));
                    let have_scores = (c.g[V0][0] as i64) > 0;
                    c.g[A0][0] = 0;
                    if have_scores {
                        c.cycles += 3;
                        c.g[V0][0] = 0x0053_0000;
                        c.g[V1][0] = sx(at(c, S4, 0x54));
                        c.g[A1][0] = sx(TOTALS);
                        pc = 0x0023_9780;
                    } else {
                        pc = 0x0023_97a4;
                    }
                }
                0x0023_97a4 => {
                    c.cycles += 13;
                    c.g[A0][0] = c.lw(gp(c, MODE_MANAGER_GP));
                    c.g[A1][0] = c.lw(at(c, A0, 0x10));
                    c.g[A1][0] = sx(at(c, A1, 1));
                    call_ptr(c, SORT_BY_SCORE, 0x0023_97b4)?;
                    pc = 0x0023_97b4;
                }
                0x0023_97b4 => {
                    c.g[A1][0] = c.lw(gp(c, MODE_MANAGER_GP));
                    c.g[V1][0] = sx(at(c, FP, 0x6730));
                    c.g[V1][0] = sx(r(c, S2).wrapping_add(r(c, V1)));
                    c.g[A0][0] = c.lw(at(c, A1, 0x70));
                    c.sw(at(c, A1, 0x74), c.g[A0][0]);
                    c.g[V0][0] = c.lw(r(c, V1));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
                    let quick = c.g[S5][0] != 0;
                    c.g[V1][0] = c.g[V0][0] ^ 1;
                    if quick {
                        c.cycles += 1;
                        c.g[V0][0] = 3;
                        pc = 0x0023_97f0;
                    } else {
                        c.cycles += 2;
                        let lost = c.g[V1][0] != 0;
                        c.g[V0][0] = 3;
                        if lost {
                            pc = 0x0023_97f0;
                        } else {
                            c.cycles += 3;
                            c.sw(at(c, A1, 0x74), 0);
                            c.sw(at(c, A1, 0x70), 0);
                            pc = 0x0023_97f4;
                        }
                    }
                }
                0x0023_97f0 => {
                    c.cycles += 1;
                    c.sw(at(c, A1, 0x70), c.g[V0][0]);
                    pc = 0x0023_97f4;
                }
                0x0023_97f4 => {
                    c.cycles += 2;
                    let won = c.g[V1][0] == 0;
                    c.g[V1][0] = c.lw(gp(c, APP_GP));
                    if won {
                        pc = 0x0023_9828;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = c.lw(at(c, V1, 0x84));
                        c.g[V0][0] = c.lw(at(c, A0, 0xc));
                        pc = 0x0023_9804;
                    }
                }
                0x0023_9804 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S2)));
                    c.g[V1][0] = c.lw(at(c, V0, 0x28));
                    let quick = c.g[S5][0] != 0;
                    c.sw(at(c, V1, 0x100), 0);
                    if quick {
                        pc = 0x0023_9854;
                        continue;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, RECORD_LOSS, 0x0023_9820)?;
                    pc = 0x0023_9820;
                }
                0x0023_9820 => {
                    c.g[V0][0] = sx(at(c, FP, 0x6730));
                    pc = 0x0023_9858;
                }
                0x0023_9828 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw(gp(c, APP_GP));
                    c.g[A1][0] = 1;
                    c.g[V1][0] = c.lw(at(c, A0, 0x84));
                    c.g[V0][0] = c.lw(at(c, V1, 0xc));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S2)));
                    c.g[V1][0] = c.lw(at(c, V0, 0x28));
                    let quick = c.g[S5][0] != 0;
                    c.sw(at(c, V1, 0x100), c.g[A1][0]);
                    if quick {
                        pc = 0x0023_9854;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, RECORD_WIN, 0x0023_9854)?;
                    pc = 0x0023_9854;
                }
                0x0023_9854 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx(at(c, FP, 0x6730));
                    pc = 0x0023_9858;
                }
                0x0023_9858 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(r(c, S7).wrapping_add(r(c, V0)));
                    c.g[V1][0] = c.lw(r(c, V0));
                    let not_first = c.g[V1][0] != 0;
                    c.g[V0][0] = 1;
                    if !not_first {
                        c.cycles += 3;
                        c.lwc1(0, gp(c, -0x515c));
                        c.g[A0][0] = c.lw(gp(c, APP_GP));
                        pc = 0x0023_9884;
                        continue;
                    }
                    c.cycles += 2;
                    let not_second = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(gp(c, APP_GP));
                    if not_second {
                        pc = 0x0023_989c;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = c.lw(gp(c, APP_GP));
                        c.lwc1(0, gp(c, -0x5158));
                        pc = 0x0023_9884;
                    }
                }
                0x0023_9884 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(at(c, A0, 0x84));
                    c.g[V1][0] = c.lw(at(c, V0, 0xc));
                    c.g[A0][0] = c.lw(at(c, V1, 0x40));
                    c.g[V0][0] = c.lw(at(c, A0, 0x18));
                    c.swc1(0, at(c, V0, 0x2f8));
                    pc = EPILOGUE;
                }
                0x0023_989c => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(at(c, V0, 0x84));
                    c.g[A0][0] = c.lw(at(c, V1, 0xc));
                    c.g[V0][0] = c.lw(at(c, A0, 0x40));
                    c.g[V1][0] = c.lw(at(c, V0, 0x18));
                    c.sw(at(c, V1, 0x2f8), 0);
                    pc = EPILOGUE;
                }
                EPILOGUE => {
                    c.cycles += 13;
                    for (register, offset) in [
                        (S0, 0x140),
                        (S1, 0x130),
                        (S2, 0x120),
                        (S3, 0x110),
                        (S4, 0x100),
                        (S5, 0xf0),
                        (S6, 0xe0),
                        (S7, 0xd0),
                        (FP, 0xc0),
                    ] {
                        frame.restore(c, register, offset);
                    }
                    frame.restore_ra(c, 0xb0);
                    frame.restore_fpr(c, 20, 0x150);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}

/// The first heat's sum setup has a different A0/A1 assignment order from
/// the second heat. Preserve both explicit versions rather than swapping.
fn setup_sum(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0x0053_0000;
    c.g[T0][0] = c.g[A1][0];
    c.g[A3][0] = sx(TOTALS);
    c.g[A0][0] = sx(at(c, S4, 0x2c));
    c.g[A1][0] = c.g[S0][0];
}
