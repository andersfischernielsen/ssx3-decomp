//! Exact native nearby AI-path selection0x26afb8, retained uninstalled.
//! No tests, builds, replays or game runs: disabled by the user.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0026_afb8, find_near_ai_paths, "PathData_FindNearAIPaths",
)];

const ENTRY: u32 = 0x0026_afb8;
const DISTANCE_LOOP: u32 = 0x0026_b030;
const DISTANCE_RETURN: u32 = 0x0026_b040;
const SCORE_READY: u32 = 0x0026_b05c;
const NEXT_PATH: u32 = 0x0026_b090;
const FILL_START: u32 = 0x0026_b0a4;
const FILL_LOOP: u32 = 0x0026_b0b8;
const MINIMUM_LOOP: u32 = 0x0026_b0e0;
const STORE_MINIMUM: u32 = 0x0026_b10c;
const RETURN: u32 = 0x0026_b13c;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }
#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 { r(c, reg).wrapping_add(offset) }

/// `int PathData_FindNearAIPaths(data, point, out, max_out, recovery_only)`.
/// Score all 0x40-byte AIPath records by Path_BoundsDistance. When the fifth
/// argument is set, a zero word at path+0x3c replaces its score by the native
/// sentinel. First emit paths whose score is zero, then repeatedly emit the
/// smallest remaining score until min(data->ai_count, max_out) is reached.
/// The second pass does not exclude sentinel scores, and there is no ground,
/// obstacle or source event query here. The score scratch array is 0x320 bytes.
pub fn find_near_ai_paths(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x3e0 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 24;
                    Frame::open(c, 0x3e0);
                    frame.save(c, S0, 0x3c0);
                    frame.save(c, S2, 0x3a0);
                    c.g[S0][0] = 0;
                    frame.save(c, S4, 0x380);
                    c.g[S2][0] = 0;
                    frame.save(c, S6, 0x360);
                    c.g[S4][0] = c.g[A0][0];
                    frame.save(c, S7, 0x350);
                    frame.save(c, FP, 0x340);
                    c.g[S7][0] = c.g[A2][0];
                    frame.save(c, S1, 0x3b0);
                    c.g[FP][0] = c.g[T0][0];
                    frame.save(c, S3, 0x390);
                    frame.save(c, S5, 0x370);
                    frame.save_ra(c, 0x330);
                    c.swc1(21, at(c, SP, 0x3d8));
                    c.swc1(20, at(c, SP, 0x3d0));
                    c.sw(at(c, SP, 0x320), c.g[A1][0]);
                    c.g[V0][0] = c.lw(at(c, S4, 8));
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    c.g[S6][0] = c.g[V0][0];
                    let no_paths = (c.g[V0][0] as i64) <= 0;
                    if c.g[V1][0] == 0 { c.g[S6][0] = c.g[A3][0]; }
                    if no_paths { pc = FILL_START; }
                    else {
                        c.cycles += 6;
                        c.lwc1(20, r(c, GP).wrapping_sub(0x4a4c));
                        c.g[S1][0] = c.g[SP][0];
                        c.mtc1(0, 21);
                        c.g[S3][0] = 0;
                        c.g[S5][0] = c.g[S7][0];
                        pc = DISTANCE_LOOP;
                    }
                }
                DISTANCE_LOOP => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(at(c, S4, 0xc));
                    c.g[A1][0] = c.lw(at(c, SP, 0x320));
                    c.g[RA][0] = sx(DISTANCE_RETURN);
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S3)));
                    call_ptr(c, 0x0026_a9b0, DISTANCE_RETURN)?;
                    pc = DISTANCE_RETURN;
                }
                DISTANCE_RETURN => {
                    let no_filter = c.g[FP][0] == 0;
                    c.swc1(0, r(c, S1));
                    if !no_filter {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw(at(c, S4, 0xc));
                        c.g[V0][0] = sx(r(c, S3).wrapping_add(r(c, V0)));
                        c.g[V1][0] = c.lw(at(c, V0, 0x3c));
                        if c.g[V1][0] == 0 { c.swc1(20, r(c, S1)); }
                    }
                    pc = SCORE_READY;
                }
                SCORE_READY => {
                    c.cycles += 5;
                    c.lwc1(0, r(c, S1));
                    c.c_eq_s(0, 21);
                    if !c.fcc() {
                        c.g[V0][0] = c.lw(at(c, S4, 8));
                        pc = NEXT_PATH;
                    } else {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw(at(c, S4, 0xc));
                        c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                        c.swc1(20, r(c, S1));
                        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S3)));
                        c.sw(r(c, S5), c.g[V0][0]);
                        let full = c.g[S0][0] == c.g[S6][0];
                        c.g[S5][0] = sx(at(c, S5, 4));
                        if full { pc = FILL_START; }
                        else {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(at(c, S4, 8));
                            pc = NEXT_PATH;
                        }
                    }
                }
                NEXT_PATH => {
                    c.cycles += 5;
                    c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
                    c.g[S1][0] = sx(at(c, S1, 4));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S3][0] = sx(at(c, S3, 0x40));
                    if more {
                        if event_due(c) { deliver_event_here(c, DISTANCE_LOOP); }
                        pc = DISTANCE_LOOP;
                    } else { pc = FILL_START; }
                }
                FILL_START => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let full = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[S0][0];
                    if full { pc = RETURN; }
                    else {
                        c.cycles += 2;
                        c.lwc1(2, r(c, GP).wrapping_sub(0x4a48));
                        pc = FILL_LOOP;
                    }
                }
                FILL_LOOP => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(at(c, S4, 8));
                    c.g[S2][0] = 1;
                    c.g[A3][0] = sx(r(c, S0).wrapping_add(1));
                    c.lwc1(1, r(c, SP));
                    c.g[A0][0] = 0;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let only_one = c.g[V0][0] == 0;
                    c.g[A2][0] = sx(r(c, S0) << 2);
                    if only_one { pc = STORE_MINIMUM; }
                    else {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[V1][0];
                        c.g[V1][0] = sx(at(c, SP, 4));
                        pc = MINIMUM_LOOP;
                    }
                }
                MINIMUM_LOOP => {
                    c.cycles += 5;
                    c.lwc1(0, r(c, V1));
                    c.c_lt_s(0, 1);
                    if !c.fcc() { c.g[S2][0] = sx(r(c, S2).wrapping_add(1)); }
                    else {
                        c.cycles += 3;
                        c.mov_s(1, 0);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V1][0] = sx(at(c, V1, 4));
                    if more {
                        if event_due(c) { deliver_event_here(c, MINIMUM_LOOP); }
                        pc = MINIMUM_LOOP;
                    } else { pc = STORE_MINIMUM; }
                }
                STORE_MINIMUM => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw(at(c, S4, 0xc));
                    c.g[V1][0] = sx(r(c, A0) << 6);
                    c.g[A0][0] = sx(r(c, A0) << 2);
                    c.g[A2][0] = sx(r(c, A2).wrapping_add(r(c, S7)));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A1][0] = sx(r(c, SP).wrapping_add(r(c, A0)));
                    c.sw(r(c, A2), c.g[V0][0]);
                    c.g[S0][0] = c.g[A3][0];
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.swc1(2, r(c, A1));
                    if more {
                        if event_due(c) { deliver_event_here(c, FILL_LOOP); }
                        pc = FILL_LOOP;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S0][0];
                        pc = RETURN;
                    }
                }
                RETURN => {
                    c.cycles += 14;
                    for (reg, off) in [(S1,0x3b0),(S0,0x3c0),(S2,0x3a0),
                        (S3,0x390),(S4,0x380),(S5,0x370),(S6,0x360),
                        (S7,0x350),(FP,0x340)] { frame.restore(c, reg, off); }
                    frame.restore_ra(c, 0x330);
                    c.lwc1(21, at(c, SP, 0x3d8));
                    c.lwc1(20, at(c, SP, 0x3d0));
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, 0x0026_b174); return Ok(()); }
            }
        }
    });
}
