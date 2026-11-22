//! Geometric selection of the native rider's current race path.

use crate::call::{call_ptr, guest, Flow, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0011_27f0, rider_select_race_path, "Rider_SelectRacePath")];

const POSITION: u32 = 0x110;
const VELOCITY: u32 = 0x1e0;
const DISTANCE_TO_FINISH: u32 = 0x4d0;
const CURRENT_PATH: u32 = 0xab4;
const PATH_CURSOR: u32 = 0xac0;
const PATH_REMAINING: u32 = 0x38;
const SCAN_HEAD: u32 = 0x0011_28f8;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void Rider_SelectRacePath(Rider* rider, bool skipCurrent)` (0x1127f0).
/// Query up to three paths near the rider's velocity look-ahead position and
/// minimize the sum of squared projection/look-ahead errors. A candidate's
/// projection is sampled another 796 units ahead for the second error.
/// The native selection is geometric: it has no lap counter or monotonic
/// remaining-distance constraint. Large projection discontinuities on the
/// current path invalidate its cursor's segment hint at +0x14.
pub fn rider_select_race_path(c: &mut Cpu) {
    guest(c, |c| {
        match c.pc {
            0x0011_27f0 => {
                c.cycles += 30;
                let frame = Frame::open(c, 0x110);
                c.mtc1(ZERO, 0);
                frame.save(c, S1, 0xd0);
                frame.save(c, S4, 0xa0);
                c.g[S1][0] = c.g[A0][0];
                frame.save_fpr(c, 21, 0xf8);
                c.g[S4][0] = c.g[A1][0];
                frame.save(c, S0, 0xe0);
                frame.save(c, S2, 0xc0);
                frame.save(c, S3, 0xb0);
                frame.save(c, S5, 0x90);
                frame.save(c, S6, 0x80);
                frame.save_ra(c, 0x70);
                frame.save_fpr(c, 22, 0x100);
                frame.save_fpr(c, 20, 0xf0);

                c.lqc2(3, at(c, S1, VELOCITY));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(V0, 22);
                c.mtc1(V0, 20);
                c.lwc1(21, r(c, GP).wrapping_sub(0x7c80));
                c.c_eq_s(20, 0);
                let moving = !c.fcc();
                c.g[S3][0] = c.lw(at(c, S1, CURRENT_PATH));
                if moving {
                    c.cycles += 18;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0011_2588, 0x0011_2880)?;
                    c.div_s(0, 0, 20);
                    c.mfc1(V1, 0);
                    c.lqc2(4, at(c, S1, VELOCITY));
                    c.qmtc2(V1, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at(c, SP, 0x50));
                    c.lq(V0, at(c, SP, 0x50));
                    c.sq(V0, at(c, SP, 0x40));
                    c.lqc2(3, at(c, S1, POSITION));
                    c.lqc2(4, at(c, SP, 0x40));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, SP, 0x50));
                    c.lq(V0, at(c, SP, 0x50));
                    c.sq(V0, at(c, SP, 0x20));
                    c.sq(V0, at(c, SP, 0x30));
                } else {
                    c.cycles += 3;
                    c.lq(V0, at(c, S1, POSITION));
                    c.sq(V0, at(c, SP, 0x20));
                }

                c.cycles += 9;
                c.g[A1][0] = sx(at(c, SP, 0x20));
                c.g[A0][0] = 0x004d_0000;
                c.g[S6][0] = c.g[A1][0];
                c.g[A0][0] = sx(0x004d_33a0);
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = 3;
                call_ptr(c, 0x0026_b178, 0x0011_28d8)?;
                let nonempty = (c.g[V0][0] as i64) > 0;
                c.g[S2][0] = c.g[V0][0];
                if nonempty {
                    c.cycles += 6;
                    c.g[AT][0] = 0x44c7_0000;
                    c.mtc1(AT, 22);
                    c.g[AT][0] = 0x4447_0000;
                    c.mtc1(AT, 20);
                    c.g[S5][0] = u64::MAX;
                    c.g[S0][0] = c.g[SP][0];
                    scan_candidates(c)?;
                }
            }
            SCAN_HEAD => scan_candidates(c)?,
            pc => {
                c.jump_out(pc, 0x0011_27f0, 0x0011_2a50);
                return Ok(());
            }
        }
        c.cycles += 14;
        c.sw(at(c, S1, CURRENT_PATH), c.g[S3][0]);
        let frame = Frame { size: 0x110 };
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 22, 0x100);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    });
}

fn scan_candidates(c: &mut Cpu) -> Flow {
    loop {
        c.cycles += 2;
        let skip = if c.g[S4][0] == 0 {
            c.g[A0][0] = c.lw(r(c, S0));
            false
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.lw(r(c, S0));
            c.g[V0][0] = c.lw(at(c, S1, CURRENT_PATH));
            if c.g[V1][0] == c.g[V0][0] {
                c.g[S2][0] = sx(r(c, S2).wrapping_sub(1));
                true
            } else {
                c.cycles += 1;
                c.g[A0][0] = c.lw(r(c, S0));
                false
            }
        };
        if !skip {
            consider_candidate(c)?;
        }
        c.cycles += 2;
        let more = c.g[S2][0] != 0;
        c.g[S0][0] = sx(at(c, S0, 4));
        if !more {
            return Ok(());
        }
        if back_edge_unwinds(c, SCAN_HEAD) {
            return Err(Unwound);
        }
    }
}

fn consider_candidate(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.g[A1][0] = c.g[S6][0];
    c.g[A2][0] = sx(at(c, SP, 0x10));
    c.g[A3][0] = sx(at(c, SP, 0x60));
    c.g[T0][0] = 1;
    call_ptr(c, 0x0026_a428, 0x0011_2928)?;
    c.g[V1][0] = c.lw(r(c, S0));
    c.g[V0][0] = c.lw(at(c, S1, CURRENT_PATH));
    let current = c.g[V1][0] == c.g[V0][0];
    c.mov_s(12, 0);
    let mut add_cycle = true;
    if current {
        c.cycles += 9;
        c.lwc1(0, at(c, V1, PATH_REMAINING));
        c.lwc1(1, at(c, S1, DISTANCE_TO_FINISH));
        c.sub_s(0, 0, 1);
        c.sub_s(0, 0, 12);
        c.abs_s(0, 0);
        c.c_lt_s(22, 0);
        if c.fcc() {
            c.cycles += 2;
            c.g[V0][0] = c.lw(at(c, S1, PATH_CURSOR));
            c.sw(at(c, V0, 0x14), c.g[S5][0]);
        } else {
            add_cycle = false;
        }
    }
    if add_cycle {
        c.cycles += 1;
    }
    c.add_s(12, 12, 20);

    c.cycles += 39;
    c.g[A1][0] = c.lw(r(c, S0));
    c.g[A0][0] = sx(at(c, SP, 0x40));
    c.g[A2][0] = 0;
    call_ptr(c, 0x0026_ab20, 0x0011_2978)?;
    c.lqc2(3, at(c, SP, 0x20));
    c.lqc2(4, at(c, SP, 0x40));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, 0x50));
    c.lq(V0, at(c, SP, 0x50));
    c.sq(V0, at(c, SP, 0x30));
    c.lqc2(3, at(c, S1, POSITION));
    c.lqc2(4, at(c, SP, 0x10));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, 0x50));
    c.lq(V0, at(c, SP, 0x50));
    c.sq(V0, at(c, SP, 0x40));
    squared_length(c, 0x40, V1);
    c.mtc1(V1, 1);
    squared_length(c, 0x30, V0);
    c.mtc1(V0, 0);
    c.add_s(1, 1, 0);
    c.c_lt_s(1, 21);
    let improves = c.fcc();
    c.g[S2][0] = sx(r(c, S2).wrapping_sub(1));
    if improves {
        c.cycles += 2;
        c.g[S3][0] = c.lw(r(c, S0));
        c.mov_s(21, 1);
    }
    Ok(())
}

/// Original VU0 dot-product macro, retaining its complete scratch state.
fn squared_length(c: &mut Cpu, offset: u32, result: usize) {
    c.lqc2(3, at(c, SP, offset));
    c.lqc2(5, at(c, SP, offset));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(result, 4);
}
