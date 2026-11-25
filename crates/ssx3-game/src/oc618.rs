//! Staging module oc618 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc618`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x00348290,
    flex_rail_find_closest_point,
    "FlexRail_FindClosestPoint",
)];

/// `FlexRail_FindClosestPoint` (0x00348290): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn flex_rail_find_closest_point(c: &mut Cpu) {
    if c.pc != 0x00348290 {
        return ee::s11::f_00348290__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00348290;
        loop {
            match pc {
                0x00348290 => {
                    c.cycles += 23;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x190));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.g[S0][0] = c.g[T3][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.g[S1][0] = c.g[A3][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[S5][0] = c.g[T2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
                    c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x188));
                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x180));
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x178));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x170));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[S6][0] = c.g[T1][0];
                    if t {
                        pc = 0x00348b00;
                        continue;
                    }
                    pc = 0x003482ec;
                    continue;
                }
                0x003482ec => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S3][0] = 0u64;
                    if t {
                        pc = 0x00348b00;
                        continue;
                    }
                    pc = 0x003482fc;
                    continue;
                }
                0x003482fc => {
                    c.cycles += 13;
                    c.g[V0][0] = 0x500000;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[FP][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
                    c.g[AT][0] = 0x3e800000;
                    c.mtc1(1, 23);
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 21);
                    c.mtc1(0, 22);
                    c.g[S4][0] = 0xffffffffffffffff;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                    pc = 0x00348330;
                    continue;
                }
                0x00348330 => {
                    c.cycles += 42;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 6);
                    c.mtc1(0, 9);
                    c.g[A2][0] = 0u64;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 10);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = 0x1;
                    c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x4c));
                    c.g[S7][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x48));
                    c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x44));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(4, (c.g[A0][0] as u32));
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32));
                    c.lqc2(3, (c.g[S1][0] as u32));
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 3);
                    pc = 0x003483d8;
                    continue;
                }
                0x003483d8 => {
                    c.cycles += 59;
                    c.mtc1(5, 0);
                    c.cvt_s_w(0, 0);
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x4c));
                    c.mov_s(2, 22);
                    c.mul_s(0, 0, 23);
                    c.mul_s(1, 0, 0);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x48));
                    c.mul_s(0, 0, 1);
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x44));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(4, (c.g[A0][0] as u32));
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(3, (c.g[S1][0] as u32));
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(3, 4);
                    c.mtc1(3, 1);
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.div_s(1, 1, 0);
                    c.c_le_s(22, 1);

                    if c.fcc() {
                        c.min_s(2, 1, 21);
                        pc = 0x003484c4;
                        continue;
                    }
                    pc = 0x003484c4;
                    continue;
                }
                0x003484c4 => {
                    c.cycles += 34;
                    c.mfc1(3, 2);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(3, (c.g[S1][0] as u32));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(0, 3);
                    let t = !c.fcc();
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x00348554;
                        continue;
                    }
                    pc = 0x0034854c;
                    continue;
                }
                0x0034854c => {
                    c.cycles += 2;
                    c.mov_s(3, 0);
                    c.g[A2][0] = c.g[A1][0];
                    pc = 0x00348554;
                    continue;
                }
                0x00348554 => {
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = ((c.g[A1][0] as i64) < 5i64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.sq(2, (c.g[SP][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x003483d8) {
                            return Ok(());
                        }
                        pc = 0x003483d8;
                        continue;
                    }
                    pc = 0x00348564;
                    continue;
                }
                0x00348564 => {
                    c.cycles += 167;
                    c.g[A1][0] = 0x490000;
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2b10));
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1828));
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(7, (c.g[V1][0] as u32));
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x37));
                    c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x3f));
                    c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x38));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                    c.g[A1][0] = 0x490000;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1810));
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(6, (c.g[V1][0] as u32));
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x67));
                    c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x6f));
                    c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x68));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T0][0]);
                    c.mov_s(13, 3);
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc4));
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2b0c));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.lwc1(10, (c.g[V1][0] as u32));
                    c.lwc1(9, (c.g[V0][0] as u32));
                    c.mov_s(12, 1);
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.mul_s(5, 10, 10);
                    c.sub_s(0, 10, 9);
                    c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x88));
                    c.mul_s(2, 9, 9);
                    c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x2b08));
                    c.mul_s(6, 10, 5);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 11);
                    c.mul_s(3, 0, 3);
                    c.g[A1][0] = 0u64;
                    c.mul_s(4, 9, 2);
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.mul_s(0, 0, 1);
                    c.add_s(7, 9, 3);
                    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, (c.g[A0][0] as u32));
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.add_s(4, 9, 0);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.mul_s(1, 7, 7);
                    c.sq(2, (c.g[SP][0] as u32));
                    c.mul_s(3, 4, 4);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.mul_s(0, 7, 1);
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.lqc2(4, (c.g[S1][0] as u32));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mov_s(20, 4);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mul_s(2, 4, 3);
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x88));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, (c.g[A0][0] as u32));
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.lqc2(4, (c.g[S1][0] as u32));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 8);
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x88));
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, (c.g[A0][0] as u32));
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32));
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.lqc2(4, (c.g[S1][0] as u32));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 3);
                    c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mov_s(2, 3);
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x88));
                    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, (c.g[A0][0] as u32));
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32));
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.lqc2(4, (c.g[S1][0] as u32));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    let t = true;
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        pc = 0x00348974;
                        continue;
                    }
                    pc = 0x00348800;
                    continue;
                }
                0x00348800 => {
                    c.cycles += 4;
                    c.c_lt_s(3, 8);

                    if !c.fcc() {
                        c.mov_s(10, 4);
                        pc = 0x003488c4;
                        continue;
                    }
                    pc = 0x00348810;
                    continue;
                }
                0x00348810 => {
                    c.cycles += 45;
                    c.mov_s(9, 7);
                    c.mul_s(2, 10, 13);
                    c.mov_s(7, 4);
                    c.swc1(11, (c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.mul_s(1, 7, 12);
                    c.mov_s(8, 3);
                    c.add_s(4, 1, 2);
                    c.mul_s(0, 4, 4);
                    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x88));
                    c.mov_s(20, 4);
                    c.mul_s(1, 4, 0);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, (c.g[A0][0] as u32));
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32));
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.lqc2(4, (c.g[S1][0] as u32));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.mov_s(3, 0);
                    let t = true;
                    c.mov_s(2, 3);
                    if t {
                        pc = 0x00348970;
                        continue;
                    }
                    pc = 0x003488c4;
                    continue;
                }
                0x003488c4 => {
                    c.cycles += 43;
                    c.mul_s(2, 9, 13);
                    c.mov_s(4, 7);
                    c.swc1(11, (c.g[SP][0] as u32).wrapping_add(0x8c));
                    c.mul_s(1, 4, 12);
                    c.mov_s(3, 8);
                    c.add_s(7, 1, 2);
                    c.mul_s(0, 7, 7);
                    c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x88));
                    c.mov_s(20, 7);
                    c.mul_s(1, 7, 0);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, (c.g[A0][0] as u32));
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32));
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.lqc2(4, (c.g[S1][0] as u32));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.mov_s(8, 0);
                    c.mov_s(2, 8);
                    pc = 0x00348970;
                    continue;
                }
                0x00348970 => {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    pc = 0x00348974;
                    continue;
                }
                0x00348974 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 24i64) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                        pc = 0x00348998;
                        continue;
                    }
                    pc = 0x00348980;
                    continue;
                }
                0x00348980 => {
                    c.cycles += 5;
                    c.sub_s(0, 10, 9);
                    c.abs_s(0, 0);
                    c.c_le_s(0, 14);
                    let t = !c.fcc();

                    if t {
                        if back_edge_unwinds(c, 0x00348800) {
                            return Ok(());
                        }
                        pc = 0x00348800;
                        continue;
                    }
                    pc = 0x00348994;
                    continue;
                }
                0x00348994 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                    pc = 0x00348998;
                    continue;
                }
                0x00348998 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x003489b4;
                        continue;
                    }
                    pc = 0x003489a0;
                    continue;
                }
                0x003489a0 => {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[S5][0] as u32));
                    c.c_lt_s(2, 0);

                    if !c.fcc() {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                        pc = 0x00348aec;
                        continue;
                    }
                    pc = 0x003489b4;
                    continue;
                }
                0x003489b4 => {
                    c.cycles += 76;
                    c.g[V1][0] = 0x2;
                    c.sw((c.g[S6][0] as u32), c.g[V0][0]);
                    c.add_s(1, 20, 20);
                    c.swc1(2, (c.g[S5][0] as u32));
                    c.g[A1][0] = c.g[S3][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x44), c.g[V1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[S4][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[S4][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[S4][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[S4][0]);
                    c.g[AT][0] = 0x40400000;
                    c.mtc1(1, 0);
                    c.lq(2, (c.g[FP][0] as u32));
                    c.lq(3, (c.g[SP][0] as u32));
                    c.mul_s(0, 20, 0);
                    c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xa4));
                    c.mul_s(0, 20, 0);
                    c.lq(2, (c.g[FP][0] as u32));
                    c.sq(3, (c.g[S0][0] as u32));
                    c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x30));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xac), 0u64);
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xa8));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x68), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x70), 0u64);
                    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lqc2(4, (c.g[A0][0] as u32));
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().rsqrt(0, 3, 4, 0);
                    c.vu0().waitq();
                    c.vu0().mul(0b1111, 5, 3, Src::Q);
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V1][0]);
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x188));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00348aac);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00348aac)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
                    c.g[V1][0] = ((0u64 as i64) < (c.g[V0][0] as i64)) as u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[S3][0]);
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = 0u64;
                    }
                    c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x68));
                    c.g[V1][0] = 0xc;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
                    c.mult(0, 2, 2, 3);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x88));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x1a));

                    if c.g[V0][0] != c.g[S4][0] {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                        pc = 0x00348ae8;
                        continue;
                    }
                    pc = 0x00348ae4;
                    continue;
                }
                0x00348ae4 => {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                    pc = 0x00348ae8;
                    continue;
                }
                0x00348ae8 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                    pc = 0x00348aec;
                    continue;
                }
                0x00348aec => {
                    c.cycles += 5;
                    c.g[S3][0] = c.g[S7][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                        if back_edge_unwinds(c, 0x00348330) {
                            return Ok(());
                        }
                        pc = 0x00348330;
                        continue;
                    }
                    pc = 0x00348b00;
                    continue;
                }
                0x00348b00 => {
                    c.cycles += 16;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x188));
                    c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x180));
                    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x178));
                    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x170));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00348290, 0x00348b40);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    #[test]
    fn flex_rail_find_closest_point_matches() {
        check(
            ee::s11::f_00348290__generated,
            flex_rail_find_closest_point,
            200,
            &|c, seed| {
                machine(c, seed, 0x8290, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00348290;
            },
        );
    }
}
