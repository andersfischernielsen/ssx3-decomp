//! Native ground-cruise input (0x131620), retained uninstalled.
//!
//! This preserves the native input/transition ordering for rail investigation.
//! No differential test, build, replay or runtime installation is performed:
//! the user has disabled automated testing.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0013_1620;
const END: u32 = 0x0013_1c30;
const RESTORE_S0: u32 = 0x0013_1c08;
const RETURN: u32 = 0x0013_1c0c;
const VELOCITY: u32 = 0x1e0;
const GROUND_FORWARD: u32 = 0x3a0;
const ANIMATION: u32 = 0x784;
const SURFACE: u32 = 0x438;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }
#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 { r(c, reg).wrapping_add(offset) }
#[inline]
fn gp(c: &Cpu, negative: u32) -> u32 { r(c, GP).wrapping_sub(negative) }

/// `void RiderCruiseSub_HandleInput(CruiseSub* state, const Input* input)`.
/// S1 retains the substate (whose first word is Rider*); S0 first retains
/// input and then the selected ground animation. f21/f22/f20 hold the three
/// decoded slew targets, f23 zero. Native transition priority is finish,
/// recovery, airborne, handplant-or-rail, prewind, then board press.
///
/// The rail attempt has no course/section/material guard: input bit0x2000
/// selects handplant instead. Even an unsuccessful handplant skips the
/// rail attempt in that update. Ice surface4 affects a later animation
/// selection only, after the native rail attempt has already failed.
pub fn rider_cruise_sub_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x60 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 41;
                    Frame::open(c, 0x60);
                    c.lwc1(0, gp(c, 0x75f4));
                    frame.save(c, S0, 0x30);
                    frame.save(c, S1, 0x20);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save_fpr(c, 23, 0x58);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save_fpr(c, 22, 0x50);
                    frame.save_fpr(c, 21, 0x48);
                    frame.save_fpr(c, 20, 0x40);
                    frame.save(c, S2, 0x10);
                    frame.save_ra(c, 0);
                    c.mtc1(0, 23);
                    c.g[A0][0] = c.lw(r(c, S0));
                    c.g[V0][0] = c.lw(at(c, S0, 4));
                    c.g[V1][0] = sx(r(c, A0) << 6);
                    c.g[A2][0] = c.lw(r(c, S1));
                    c.g[V0][0] = sx(r(c, V0) << 26);
                    c.g[V1][0] = sx(((r(c, V1) as i32) >> 26) as u32);
                    c.g[A0][0] = sx(((r(c, A0) as i32) >> 26) as u32);
                    c.g[V0][0] = sx(((r(c, V0) as i32) >> 26) as u32);
                    c.lqc2(3, at(c, A2, VELOCITY));
                    c.lqc2(5, at(c, A2, GROUND_FORWARD));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(A1, 4);
                    c.mtc1(A1, 2);
                    c.mtc1(V1, 3);
                    c.cvt_s_w(3, 3);
                    c.mtc1(A0, 4);
                    c.cvt_s_w(4, 4);
                    c.mtc1(V0, 1);
                    c.cvt_s_w(1, 1);
                    c.c_lt_s(2, 23);
                    c.mul_s(21, 3, 0);
                    c.mul_s(20, 1, 0);
                    let backward = c.fcc();
                    c.mul_s(22, 4, 0);
                    if backward {
                        c.cycles += 1;
                        c.mov_s(20, 23);
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[A2][0];
                    call_ptr(c, 0x0011_6378, 0x0013_16d0)?;
                    pc = 0x0013_16d0;
                }
                0x0013_16d0 | 0x0013_16f0 | 0x0013_1700 | 0x0013_173c | 0x0013_1778
                | 0x0013_179c | 0x0013_17e0 => {
                    if c.g[V0][0] != 0 {
                        // Early transitions load S0 in the original delay slot.
                        frame.restore(c, S0, 0x30);
                        pc = RETURN;
                        continue;
                    }
                    pc = match pc {
                        0x0013_16d0 => 0x0013_16d4,
                        0x0013_16f0 => 0x0013_16f4,
                        0x0013_1700 => 0x0013_1704,
                        0x0013_173c => 0x0013_1740,
                        0x0013_1778 => 0x0013_1780,
                        0x0013_179c => 0x0013_17a0,
                        _ => 0x0013_17e4,
                    };
                }
                0x0013_16d4 => {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw(r(c, S0));
                    c.g[A2][0] = 0;
                    c.g[A0][0] = c.lw(r(c, S1));
                    c.g[A1][0] = sx(((r(c, A1) as i32) >> 12) as u32);
                    c.g[A1][0] &= 1;
                    call_ptr(c, 0x0011_6120, 0x0013_16f0)?;
                    pc = 0x0013_16f0;
                }
                0x0013_16f4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0013_1cc0, 0x0013_1700)?;
                    pc = 0x0013_1700;
                }
                0x0013_1704 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(r(c, S0));
                    c.g[V0][0] &= 0x2000;
                    if c.g[V0][0] == 0 {
                        pc = 0x0013_1770;
                    } else {
                        c.cycles += 6;
                        c.g[AT][0] = 0x3f00_0000;
                        c.mtc1(AT, 20);
                        c.c_lt_s(20, 22);
                        let handplant_side = c.fcc();
                        c.g[A1][0] = 1;
                        if !handplant_side {
                            c.cycles += 1;
                            c.g[A1][0] = 0;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.lw(r(c, S1));
                        call_ptr(c, 0x0010_7578, 0x0013_173c)?;
                        pc = 0x0013_173c;
                    }
                }
                0x0013_1740 => {
                    c.cycles += 6;
                    c.g[AT][0] = 0xffff_ffff_bf00_0000;
                    c.mtc1(AT, 0);
                    c.c_le_s(0, 21);
                    if c.fcc() { c.min_s(0, 21, 20); }
                    c.cycles += 5;
                    c.mov_s(21, 0);
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 22);
                    c.mov_s(20, 23);
                    pc = 0x0013_1780;
                }
                0x0013_1770 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(r(c, S1));
                    call_ptr(c, 0x0010_6848, 0x0013_1778)?;
                    pc = 0x0013_1778;
                }
                0x0013_1780 => {
                    c.cycles += 9;
                    c.g[A1][0] = c.lw(r(c, S0));
                    c.g[A0][0] = c.lw(r(c, S1));
                    c.g[A2][0] = sx(((r(c, A1) as i32) >> 14) as u32);
                    c.g[A1][0] = sx(((r(c, A1) as i32) >> 15) as u32);
                    c.g[A2][0] &= 1;
                    c.g[A1][0] &= 1;
                    call_ptr(c, 0x0011_62c8, 0x0013_179c)?;
                    pc = 0x0013_179c;
                }
                0x0013_17a0 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(r(c, S0));
                    c.g[V0][0] &= 0x8000;
                    let jump_held = c.g[V0][0] != 0;
                    c.g[V0][0] = c.lw(at(c, S0, 4));
                    if jump_held {
                        c.cycles += 3;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 22);
                        c.mtc1(0, 20);
                    }
                    c.cycles += 10;
                    c.lwc1(0, gp(c, 0x75f0));
                    c.g[V0][0] = sx(r(c, V0) << 20);
                    c.g[A0][0] = c.lw(r(c, S1));
                    c.g[V0][0] = sx(((r(c, V0) as i32) >> 26) as u32);
                    c.mtc1(V0, 12);
                    c.cvt_s_w(12, 12);
                    c.mul_s(12, 12, 0);
                    call_ptr(c, 0x0011_61d0, 0x0013_17e0)?;
                    pc = 0x0013_17e0;
                }
                0x0013_17e4 => {
                    c.cycles += 9;
                    c.g[A1][0] = c.lw(r(c, S0));
                    c.g[A0][0] = c.lw(r(c, S1));
                    c.g[A2][0] = sx(((r(c, A1) as i32) >> 19) as u32);
                    c.g[A1][0] = sx(((r(c, A1) as i32) >> 18) as u32);
                    c.g[A2][0] &= 1;
                    c.g[A1][0] &= 1;
                    call_ptr(c, 0x0011_63b0, 0x0013_1804)?;
                    pc = 0x0013_1804;
                }
                0x0013_1804 => {
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = c.lw(r(c, S0));
                    } else {
                        c.cycles += 8;
                        c.g[AT][0] = 0xffff_ffff_bf00_0000;
                        c.mtc1(AT, 0);
                        c.g[AT][0] = 0x3f00_0000;
                        c.mtc1(AT, 1);
                        c.c_le_s(0, 21);
                        if c.fcc() { c.min_s(0, 21, 1); }
                        c.cycles += 2;
                        c.mov_s(21, 0);
                        c.g[A1][0] = c.lw(r(c, S0));
                    }
                    c.cycles += 25;
                    c.g[A0][0] = c.lw(r(c, S1));
                    c.g[A2][0] = sx(((r(c, A1) as i32) >> 16) as u32);
                    c.g[A1][0] = sx(((r(c, A1) as i32) >> 17) as u32);
                    c.g[A2][0] &= 1;
                    c.g[A1][0] &= 1;
                    call_ptr(c, 0x0011_4130, 0x0013_184c)?;
                    pc = 0x0013_184c;
                }
                0x0013_184c => {
                    c.g[A0][0] = c.lw(r(c, S1));
                    c.mov_s(12, 21);
                    call_ptr(c, 0x0011_3e80, 0x0013_1858)?;
                    pc = 0x0013_1858;
                }
                0x0013_1858 => {
                    c.g[A0][0] = c.lw(r(c, S1));
                    c.mov_s(13, 20);
                    c.mov_s(12, 22);
                    call_ptr(c, 0x0011_3f88, 0x0013_1868)?;
                    pc = 0x0013_1868;
                }
                0x0013_1868 => {
                    c.g[A0][0] = c.lw(r(c, S1));
                    call_ptr(c, 0x0011_5b58, 0x0013_1870)?;
                    pc = 0x0013_1870;
                }
                0x0013_1870 => {
                    c.g[A0][0] = c.lw(r(c, S1));
                    call_ptr(c, 0x0011_5d48, 0x0013_1878)?;
                    pc = 0x0013_1878;
                }
                0x0013_1878 => {
                    c.g[V0][0] = c.lw(r(c, S1));
                    c.g[A1][0] = 2;
                    c.g[A0][0] = c.lw(at(c, V0, ANIMATION));
                    call_ptr(c, 0x0031_2aa0, 0x0013_1888)?;
                    pc = 0x0013_1888;
                }
                0x0013_1888 => {
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = 0x16;
                    let stopped_animation = c.g[S0][0] == c.g[V0][0];
                    c.g[S2][0] = c.g[S0][0];
                    if stopped_animation {
                        c.cycles += 11;
                        c.g[V1][0] = c.lw(r(c, S1));
                        c.lwc1(0, gp(c, 0x75ec));
                        c.sw(at(c, V1, 0x204), 0);
                        c.swc1(0, at(c, V1, 0x200));
                        c.g[V0][0] = c.lw(r(c, S1));
                        c.swc1(0, at(c, V0, 0x218));
                        c.sw(at(c, V0, 0x21c), 0);
                        c.g[V1][0] = c.lw(r(c, S1));
                        c.swc1(0, at(c, V1, 0x224));
                        c.sw(at(c, V1, 0x228), 0);
                        pc = 0x0013_1be8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x15;
                        if c.g[S0][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw(r(c, S1));
                            pc = 0x0013_18fc;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = c.lw(r(c, S1));
                            c.mtc1(0, 1);
                            c.lwc1(0, at(c, A0, 0x2dc));
                            c.c_eq_s(0, 1);
                            if !c.fcc() {
                                c.g[V0][0] = c.lw(r(c, S1));
                                pc = 0x0013_191c;
                            } else {
                                c.cycles += 4;
                                call_ptr(c, 0x0011_4cc0, 0x0013_18f4)?;
                                pc = 0x0013_18f4;
                            }
                        }
                    }
                }
                0x0013_18f4 => {
                    let turned = c.g[V0][0] != 0;
                    c.g[V0][0] = c.lw(r(c, S1));
                    pc = if turned { 0x0013_18fc } else { 0x0013_191c };
                }
                0x0013_18fc => {
                    c.cycles += 8;
                    c.g[S0][0] = 0x15;
                    c.lwc1(0, gp(c, 0x75e8));
                    c.sw(at(c, V0, 0x204), 0);
                    c.swc1(0, at(c, V0, 0x200));
                    c.g[V1][0] = c.lw(r(c, S1));
                    c.swc1(0, at(c, V1, 0x218));
                    c.sw(at(c, V1, 0x21c), 0);
                    pc = 0x0013_1be8;
                }
                0x0013_191c => {
                    c.cycles += 6;
                    c.g[A1][0] = 2;
                    c.g[A0][0] = c.lw(at(c, V0, ANIMATION));
                    call_ptr(c, 0x0031_1ae8, 0x0013_1928)?;
                    pc = 0x0013_1928;
                }
                0x0013_1928 => {
                    c.g[V1][0] = 0xa;
                    let animation10 = c.g[V0][0] == c.g[V1][0];
                    c.g[V0][0] = c.lw(r(c, S1));
                    if animation10 {
                        c.cycles += 4;
                        c.lwc1(0, gp(c, 0x75e4));
                        c.sw(at(c, V0, 0x204), 0);
                        c.swc1(0, at(c, V0, 0x200));
                        pc = 0x0013_1be8;
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = 2;
                        c.g[A0][0] = c.lw(at(c, V0, ANIMATION));
                        call_ptr(c, 0x0031_1ae8, 0x0013_1950)?;
                        pc = 0x0013_1950;
                    }
                }
                0x0013_1950 => {
                    c.g[V1][0] = 5;
                    if c.g[V0][0] == c.g[V1][0] {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw(r(c, S1));
                        c.lwc1(1, gp(c, 0x75e0));
                        c.lwc1(0, at(c, V0, 0x1f0));
                        c.swc1(1, at(c, V0, 0x200));
                        c.swc1(0, at(c, V0, 0x204));
                        c.g[V1][0] = c.lw(r(c, S1));
                        c.swc1(1, at(c, V1, 0x218));
                        c.sw(at(c, V1, 0x21c), 0);
                        pc = 0x0013_1be8;
                    } else {
                        c.g[V1][0] = c.lw(r(c, S1));
                        c.cycles += 16;
                        c.mtc1(0, 3);
                        c.lqc2(3, at(c, V1, VELOCITY));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.vu0().sqrt(4, 0);
                        c.vu0().waitq();
                        c.cfc2(V0, 22);
                        c.mtc1(V0, 20);
                        c.lwc1(0, at(c, V1, 0x214));
                        c.c_eq_s(0, 3);
                        let straight = c.fcc();
                        c.g[V0][0] = sx(r(c, S0).wrapping_sub(0xc));
                        pc = if straight { 0x0013_19d0 } else { 0x0013_1b6c };
                    }
                }
                0x0013_19d0 => {
                    c.cycles += 7;
                    c.lwc1(0, gp(c, 0x75dc));
                    c.lwc1(2, at(c, V1, 0x1f0));
                    c.lwc1(1, gp(c, 0x75d8));
                    c.c_lt_s(20, 0);
                    c.swc1(2, at(c, V1, 0x204));
                    let slow = c.fcc();
                    c.swc1(1, at(c, V1, 0x200));
                    if slow {
                        c.cycles += 4;
                        c.c_lt_s(3, 22);
                        if c.fcc() {
                            c.g[S0][0] = 0x16;
                            pc = 0x0013_1be8;
                            continue;
                        }
                        c.cycles += 6;
                        c.lwc1(1, gp(c, 0x75d4));
                        c.abs_s(0, 21);
                        c.c_lt_s(1, 0);
                        let turn = c.fcc();
                        c.g[S0][0] = 0x16;
                        if turn {
                            pc = 0x0013_1be8;
                            continue;
                        }
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(r(c, S1));
                    c.g[V1][0] = 4;
                    c.g[V0][0] = c.lw(at(c, A0, SURFACE));
                    if c.g[V0][0] != c.g[V1][0] {
                        c.lwc1(0, at(c, A0, 0x1fc));
                        pc = 0x0013_1a7c;
                    } else {
                        // Ice modifies only ground-animation selection here.
                        c.cycles += 13;
                        c.lwc1(1, at(c, A0, 0x1f0));
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 0);
                        c.abs_s(3, 1);
                        c.g[AT][0] = 0x3f00_0000;
                        c.mtc1(AT, 2);
                        c.sub_s(0, 0, 3);
                        c.lwc1(1, at(c, A0, 0x220));
                        c.mul_s(0, 0, 2);
                        c.c_le_s(1, 0);
                        let carving = c.fcc();
                        c.g[S0][0] = 0xe;
                        if !carving {
                            c.cycles += 6;
                            c.lwc1(1, at(c, A0, 0x2fc));
                            c.mtc1(0, 0);
                            c.c_lt_s(0, 1);
                            let boosting = c.fcc();
                            c.g[S0][0] = 0xf;
                            if boosting {
                                c.cycles += 4;
                                c.lwc1(0, gp(c, 0x75d0));
                                c.c_lt_s(3, 0);
                                if c.fcc() {
                                    c.cycles += 2;
                                    c.g[S0][0] = 8;
                                }
                            }
                        }
                        pc = 0x0013_1be8;
                    }
                }
                0x0013_1a7c => {
                    c.cycles += 6;
                    c.lwc1(1, gp(c, 0x75cc));
                    c.abs_s(0, 0);
                    c.c_lt_s(1, 0);
                    if !c.fcc() {
                        c.g[V0][0] = c.lw(r(c, S1));
                        pc = 0x0013_1af0;
                    } else {
                        c.cycles += 7;
                        c.lwc1(0, at(c, A0, 0x3b8));
                        c.lwc1(1, gp(c, 0x75c8));
                        c.abs_s(0, 0);
                        c.c_lt_s(1, 0);
                        if !c.fcc() {
                            c.g[V0][0] = c.lw(r(c, S1));
                            pc = 0x0013_1af0;
                        } else {
                            c.cycles += 5;
                            call_ptr(c, 0x0031_7810, 0x0013_1ab8)?;
                            pc = 0x0013_1ab8;
                        }
                    }
                }
                0x0013_1ab8 => {
                    c.g[V0][0] &= 1;
                    let skip_random_turn = c.g[V0][0] != 0;
                    c.g[V0][0] = c.lw(r(c, S1));
                    if skip_random_turn {
                        pc = 0x0013_1af0;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw(at(c, V0, 0x320));
                        let switch = c.g[V1][0] != 0;
                        c.lwc1(1, at(c, V0, 0x1fc));
                        if switch {
                            c.cycles += 1;
                            c.neg_s(1, 1);
                        }
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_lt_s(1, 0);
                        let negative = c.fcc();
                        c.g[S0][0] = 0x11;
                        if negative {
                            c.cycles += 2;
                            c.g[S0][0] = 0x10;
                        }
                        pc = 0x0013_1be8;
                    }
                }
                0x0013_1af0 => {
                    c.cycles += 13;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 1);
                    c.lwc1(0, at(c, V0, 0x1f0));
                    c.abs_s(0, 0);
                    c.g[AT][0] = 0x3f00_0000;
                    c.mtc1(AT, 3);
                    c.sub_s(1, 1, 0);
                    c.lwc1(2, at(c, V0, 0x220));
                    c.mul_s(1, 1, 3);
                    c.c_le_s(2, 1);
                    let carving = c.fcc();
                    c.g[S0][0] = 5;
                    if !carving {
                        c.cycles += 6;
                        c.lwc1(1, at(c, V0, 0x2fc));
                        c.mtc1(0, 0);
                        c.c_lt_s(0, 1);
                        let boosting = c.fcc();
                        c.g[S0][0] = 8;
                        if !boosting {
                            c.cycles += 5;
                            c.lwc1(0, gp(c, 0x75c4));
                            c.c_lt_s(20, 0);
                            let slow = c.fcc();
                            c.g[S0][0] = 7;
                            if !slow {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw(at(c, V0, SURFACE));
                                c.g[V0][0] = sx(r(c, V0).wrapping_sub(2));
                                c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                                if c.g[V0][0] == 0 {
                                    c.g[S0][0] = 6;
                                } else { c.cycles += 2; }
                            }
                        }
                    }
                    pc = 0x0013_1be8;
                }
                0x0013_1b6c => {
                    c.cycles += 7;
                    c.lwc1(0, gp(c, 0x75c0));
                    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                    c.swc1(3, at(c, V1, 0x204));
                    c.swc1(0, at(c, V1, 0x200));
                    c.lwc1(0, gp(c, 0x75bc));
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.lwc1(0, gp(c, 0x75b8));
                    }
                    c.cycles += 4;
                    c.c_lt_s(0, 20);
                    if c.fcc() {
                        c.g[S0][0] = 0xb;
                        pc = 0x0013_1be8;
                        continue;
                    }
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(r(c, S1));
                    c.lwc1(1, gp(c, 0x75b4));
                    c.lwc1(2, at(c, V0, 0x214));
                    c.abs_s(0, 2);
                    c.c_lt_s(0, 1);
                    let small_turn = c.fcc();
                    c.g[S0][0] = 0xb;
                    if !small_turn {
                        c.cycles += 4;
                        c.c_lt_s(3, 2);
                        let positive = c.fcc();
                        c.g[S0][0] = 0xd;
                        if positive {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw(at(c, V0, 0x320));
                            c.g[V0][0] = 0xc;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw(at(c, V0, 0x320));
                            c.g[S0][0] = 0xc;
                            c.g[V0][0] = 0xd;
                        }
                        c.cycles += 1;
                        if c.g[V1][0] != 0 { c.g[S0][0] = c.g[V0][0]; }
                    }
                    pc = 0x0013_1be8;
                }
                0x0013_1be8 => {
                    c.cycles += 2;
                    let same_animation = c.g[S0][0] == c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    if !same_animation {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw(r(c, S1));
                        c.g[AT][0] = 0xffff_ffff_bf80_0000;
                        c.mtc1(AT, 12);
                        c.g[A2][0] = 0;
                        c.g[A0][0] = c.lw(at(c, V0, ANIMATION));
                        call_ptr(c, 0x0031_28e8, RESTORE_S0)?;
                    }
                    pc = RESTORE_S0;
                }
                RESTORE_S0 => {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x30);
                    pc = RETURN;
                }
                RETURN => {
                    c.cycles += 9;
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.restore_fpr(c, 23, 0x58);
                    frame.restore_fpr(c, 22, 0x50);
                    frame.restore_fpr(c, 21, 0x48);
                    frame.restore_fpr(c, 20, 0x40);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    });
}
