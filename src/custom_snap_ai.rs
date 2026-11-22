//! Native Rider_SnapToNearestAIPath0x112d58 with imported Tricky query policy.
//! Retail courses keep the generated body. Automated tests/replays are
//! disabled by the user's development instructions.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0011_2d58, rider_snap_to_nearest_ai_path, "Rider_SnapToNearestAIPath",
)];

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }
#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

const ENTRY: u32 = 0x0011_2d58;
const END: u32 = 0x0011_2fac;
const CANDIDATE: u32 = 0x0011_2dd0;
const SCORE: u32 = 0x0011_2e2c;
const NEXT: u32 = 0x0011_2ee0;
const HAVE_PATH: u32 = 0x0011_2eec;
const RESTORE: u32 = 0x0011_2f7c;
const AI_PATH: u32 = 0xab8;
const AI_CURSOR: u32 = 0xabc;
const RESET_POSITION: u32 = 0x490;
const RESET_AHEAD: u32 = 0x4a0;
const AI_DISTANCE: u32 = 0x4c4;

/// Leave the four-component squared VU length in the original result/FPR.
#[inline]
fn squared_distance(c: &mut Cpu, vector: u32, register: usize, fpr: usize) {
    c.lqc2(3, vector);
    c.lqc2(5, vector);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(register, 4);
    c.mtc1(register, fpr);
}

/// Select among at most six recoverable AI paths, scoring the route-seeded
/// projection and its point800cm farther along. Like Tricky118c08, skip
/// candidates within200cm of their path end for every rider. Resolve the
/// chosen projection through the native
/// path marker-boundary helper, retaining its distance/cursor and two reset
/// positions. Candidate eligibility is delegated to FindNearAIPaths; this
/// wrapper does not read the serialized respawnable word itself.
pub fn rider_snap_to_nearest_ai_path(c: &mut Cpu) {
    if crate::custom_tracks::active().is_none() {
        ssx3_ee::s00::f_00112d58__generated(c);
        return;
    }
    guest(c, |c| {
        let frame = Frame { size: 0x100 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 23;
                    Frame::open(c, 0x100);
                    c.g[A3][0] = 6;
                    frame.save(c, S1, 0xb0);
                    c.g[A2][0] = c.g[SP][0];
                    c.g[S1][0] = c.g[A0][0];
                    crate::custom_tricky_recovery::seed_query(c, r(c, S1));
                    frame.save(c, S2, 0xa0);
                    c.g[A0][0] = 0x004d_0000;
                    frame.save_fpr(c, 22, 0xe0);
                    frame.save(c, S0, 0xc0);
                    c.g[A0][0] = sx(at(c, A0, 0x33a0));
                    frame.save(c, S3, 0x90);
                    c.g[A1][0] = sx(crate::custom_tricky_recovery::query_position(r(c, S1)));
                    frame.save(c, S4, 0x80);
                    c.g[S2][0] = 0;
                    frame.save_ra(c, 0x70);
                    frame.save_fpr(c, 24, 0xf0);
                    frame.save_fpr(c, 23, 0xe8);
                    frame.save_fpr(c, 21, 0xd8);
                    c.g[RA][0] = sx(0x0011_2da8);
                    frame.save_fpr(c, 20, 0xd0);
                    call_ptr(c, 0x0026_af98, 0x0011_2da8)?;
                    pc = 0x0011_2da8;
                }
                0x0011_2da8 => {
                    c.lwc1(22, r(c, GP).wrapping_sub(0x7c78));
                    if (c.g[V0][0] as i64) <= 0 {
                        c.g[V0][0] = c.lw(at(c, S1, AI_PATH));
                        pc = HAVE_PATH;
                    } else {
                        c.cycles += 7;
                        c.g[AT][0] = 0x4348_0000;
                        c.mtc1(AT, 24);
                        c.g[S3][0] = c.g[V0][0];
                        c.g[AT][0] = crate::custom_tricky_recovery::ahead_bits() as u64;
                        c.mtc1(AT, 23);
                        c.g[S4][0] = sx(at(c, S1, 0x6c0));
                        c.g[S0][0] = c.g[SP][0];
                        pc = CANDIDATE;
                    }
                }
                CANDIDATE => {
                    c.cycles += 17;
                    c.g[A0][0] = c.lw(r(c, S0));
                    c.g[A1][0] = sx(crate::custom_tricky_recovery::query_position(r(c, S1)));
                    c.g[A2][0] = sx(at(c, SP, 0x20));
                    c.g[A3][0] = sx(at(c, SP, 0x60));
                    c.g[RA][0] = sx(0x0011_2de8);
                    c.g[T0][0] = 1;
                    call_ptr(c, 0x0026_a428, 0x0011_2de8)?;
                    pc = 0x0011_2de8;
                }
                0x0011_2de8 => {
                    c.g[A0][0] = c.lw(r(c, S0));
                    c.g[RA][0] = sx(0x0011_2df4);
                    c.mov_s(21, 0);
                    call_ptr(c, 0x0026_ac48, 0x0011_2df4)?;
                    pc = 0x0011_2df4;
                }
                0x0011_2df4 => {
                    c.g[V1][0] = c.lw(at(c, S1, 0x6c0));
                    c.mov_s(20, 0);
                    c.g[A0][0] = c.lh(at(c, V1, 0x40));
                    c.g[V0][0] = c.lw(at(c, V1, 0x44));
                    let target = r(c, V0);
                    c.g[RA][0] = sx(0x0011_2e0c);
                    c.g[A0][0] = sx(r(c, S4).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x0011_2e0c)?;
                    pc = 0x0011_2e0c;
                }
                0x0011_2e0c => {
                    if c.g[V0][0] != 0 && crate::custom_tracks::active().is_none() {
                        c.add_s(0, 21, 23);
                        pc = SCORE;
                    } else {
                        c.cycles += 5;
                        c.sub_s(0, 20, 24);
                        c.c_lt_s(0, 21);
                        if c.fcc() {
                            c.g[S3][0] = sx(r(c, S3).wrapping_sub(1));
                            pc = NEXT;
                        } else {
                            c.cycles += 1;
                            c.add_s(0, 21, 23);
                            pc = SCORE;
                        }
                    }
                }
                SCORE => {
                    c.cycles += 40;
                    c.g[A1][0] = c.lw(r(c, S0));
                    c.g[A0][0] = sx(at(c, SP, 0x40));
                    c.g[A2][0] = sx(at(c, SP, 0x64));
                    c.g[RA][0] = sx(0x0011_2e40);
                    c.swc1(0, at(c, SP, 0x64));
                    call_ptr(c, 0x0026_9f18, 0x0011_2e40)?;
                    pc = 0x0011_2e40;
                }
                0x0011_2e40 => {
                    c.lqc2(3, crate::custom_tricky_recovery::query_position(r(c, S1)));
                    c.lqc2(4, at(c, SP, 0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, SP, 0x50));
                    c.lq(V0, at(c, SP, 0x50));
                    c.sq(V0, at(c, SP, 0x30));
                    c.lqc2(3, crate::custom_tricky_recovery::query_position(r(c, S1)));
                    c.lqc2(4, at(c, SP, 0x20));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, SP, 0x50));
                    c.lq(V0, at(c, SP, 0x50));
                    c.sq(V0, at(c, SP, 0x40));
                    squared_distance(c, at(c, SP, 0x40), V1, 1);
                    squared_distance(c, at(c, SP, 0x30), V0, 0);
                    c.add_s(1, 1, 0);
                    c.c_lt_s(1, 22);
                    let not_better = !c.fcc();
                    c.g[S3][0] = sx(r(c, S3).wrapping_sub(1));
                    if !not_better {
                        c.cycles += 5;
                        c.g[S2][0] = c.lw(r(c, S0));
                        c.mov_s(22, 1);
                        c.g[V0][0] = c.lw(at(c, S1, AI_PATH));
                        c.g[V0][0] = c.g[S2][0] ^ c.g[V0][0];
                        if c.g[V0][0] == 0 { c.g[S2][0] = 0; }
                    }
                    pc = NEXT;
                }
                NEXT => {
                    c.cycles += 2;
                    let more = c.g[S3][0] != 0;
                    c.g[S0][0] = sx(at(c, S0, 4));
                    if more {
                        if event_due(c) { deliver_event_here(c, CANDIDATE); }
                        pc = CANDIDATE;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw(at(c, S1, AI_PATH));
                        pc = HAVE_PATH;
                    }
                }
                HAVE_PATH => {
                    c.cycles += 2;
                    let absent = c.g[V0][0] == 0;
                    frame.restore(c, S0, 0xc0);
                    if absent {
                        pc = RESTORE;
                    } else {
                        c.cycles += 2;
                        if c.g[S2][0] != 0 { c.sw(at(c, S1, AI_PATH), c.g[S2][0]); }
                        c.cycles += 32;
                        c.g[V1][0] = c.lw(at(c, S1, AI_CURSOR));
                        c.g[V0][0] = u64::MAX;
                        c.g[A3][0] = sx(at(c, S1, 0x4c8));
                        c.g[T1][0] = 1;
                        c.sw(at(c, V1, 0x14), c.g[V0][0]);
                        c.g[A1][0] = sx(crate::custom_tricky_recovery::query_position(r(c, S1)));
                        c.g[A2][0] = sx(at(c, S1, RESET_POSITION));
                        c.g[T0][0] = c.lw(at(c, S1, AI_CURSOR));
                        c.g[RA][0] = sx(0x0011_2f24);
                        c.g[A0][0] = c.lw(at(c, S1, AI_PATH));
                        call_ptr(c, 0x0026_a638, 0x0011_2f24)?;
                        pc = 0x0011_2f24;
                    }
                }
                0x0011_2f24 => {
                    c.g[S0][0] = sx(at(c, SP, 0x30));
                    c.g[A1][0] = c.lw(at(c, S1, AI_PATH));
                    c.swc1(0, at(c, S1, AI_DISTANCE));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0011_2f3c);
                    c.g[A2][0] = sx(at(c, S1, AI_DISTANCE));
                    call_ptr(c, 0x0026_9f18, 0x0011_2f3c)?;
                    pc = 0x0011_2f3c;
                }
                0x0011_2f3c => {
                    c.lwc1(0, at(c, S1, AI_DISTANCE));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[AT][0] = crate::custom_tricky_recovery::ahead_bits() as u64;
                    c.mtc1(AT, 1);
                    c.g[A2][0] = sx(at(c, SP, 0x68));
                    c.lq(V0, at(c, SP, 0x30));
                    c.add_s(0, 0, 1);
                    c.g[A1][0] = c.lw(at(c, S1, AI_PATH));
                    c.sq(V0, at(c, S1, RESET_POSITION));
                    c.g[RA][0] = sx(0x0011_2f68);
                    c.swc1(0, at(c, SP, 0x68));
                    call_ptr(c, 0x0026_9f18, 0x0011_2f68)?;
                    pc = 0x0011_2f68;
                }
                0x0011_2f68 => {
                    c.lq(V0, at(c, SP, 0x30));
                    c.lwc1(0, at(c, S1, AI_DISTANCE));
                    c.sq(V0, at(c, S1, RESET_AHEAD));
                    c.swc1(0, at(c, S1, 0x4c0));
                    frame.restore(c, S0, 0xc0);
                    pc = RESTORE;
                }
                RESTORE => {
                    c.cycles += 12;
                    for (register, offset) in [(S1, 0xb0), (S2, 0xa0), (S3, 0x90), (S4, 0x80)] {
                        frame.restore(c, register, offset);
                    }
                    frame.restore_ra(c, 0x70);
                    for (register, offset) in [(24, 0xf0), (23, 0xe8), (22, 0xe0), (21, 0xd8), (20, 0xd0)] {
                        frame.restore_fpr(c, register, offset);
                    }
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    })
}
