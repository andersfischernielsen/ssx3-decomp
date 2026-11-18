//! Staging module oc408 (wave wave218): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc408`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0011_d660, rider_place, "Rider_Place")];

/// `void Rider_Place(Rider *self, float *pos, float *dir, int kind, float snap)`
/// (0x11d660): put the rider at `pos` facing `dir` (heading by atan of
/// `dir.z/dir.x`, quadrant-corrected). With `snap >= 0` it first ray-casts
/// from `pos` along the world up axis and snaps to the hit. Then it builds
/// the up/forward/right frame, clears the motion and FX state, resets the
/// animation (idle, `kind`) and runs one animation/skeleton/bounds/light
/// update, notifying the AI object and the race progress.
pub fn rider_place(c: &mut Cpu) {
    guest(c, |c| {
        macro_rules! sp { ($o:expr) => { (c.g[SP][0] as u32).wrapping_add($o) }; }
        macro_rules! s1 { ($o:expr) => { (c.g[S1][0] as u32).wrapping_add($o) }; }
        macro_rules! s2 { ($o:expr) => { (c.g[S2][0] as u32).wrapping_add($o) }; }
        macro_rules! s3m { ($o:expr) => { (c.g[S3][0] as u32).wrapping_sub($o) }; }
        macro_rules! gp { ($o:expr) => { (c.g[GP][0] as u32).wrapping_sub($o) }; }
        'b_11d768: {
        'b_11d728: {
        c.cycles += 39;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x230));
        c.g[V1][0] = 0x500000;
        c.sq(17, sp!(0x1f0));
        c.sq(18, sp!(0x1e0));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, sp!(0x1d0));
        c.g[S2][0] = c.g[A1][0];
        c.sq(20, sp!(0x1c0));
        c.g[S3][0] = c.g[V1][0];
        c.swc1(23, sp!(0x228));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
        c.swc1(22, sp!(0x220));
        c.mov_s(23, 12);
        c.sq(16, sp!(0x200));
        c.g[S4][0] = c.g[A3][0];
        c.sd(sp!(0x1b0), c.g[RA][0]);
        c.swc1(21, sp!(0x218));
        c.swc1(20, sp!(0x210));
        c.mtc1(0, 22);
        c.g[V0][0] = c.lw(s1!(0x864));
        c.lq(4, (c.g[V1][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[V0][0] as u32), 0u64);
        c.sh((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
        c.sh((c.g[V0][0] as u32).wrapping_add(0x6), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
        c.g[V0][0] = c.lw(s1!(0x868));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[V0][0] as u32), 0u64);
        c.sh((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
        c.sh((c.g[V0][0] as u32).wrapping_add(0x6), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
        c.sq(4, sp!(0x10));
        c.lwc1(20, (c.g[A2][0] as u32));
        c.lq(2, (c.g[S2][0] as u32));
        c.c_eq_s(20, 22);
        c.lwc1(21, (c.g[A2][0] as u32).wrapping_add(0x4));
        let t = !c.fcc();
        c.sq(2, (c.g[SP][0] as u32));
        if t {
        break 'b_11d728;
        } else {
        c.cycles += 4;
        c.c_eq_s(21, 22);
        let t = c.fcc();
        c.mov_s(20, 21);
        if t {
        break 'b_11d768;
        } else {
        c.cycles += 4;
        c.c_le_s(22, 21);
        c.lwc1(20, gp!(0x79b0));
        let t = !c.fcc();

        if t {
        break 'b_11d768;
        } else {
        c.cycles += 3;
        c.lwc1(20, gp!(0x79ac));
        break 'b_11d768;
        }
        }
        }
        }
        'b_11d764: {
        c.cycles += 7;
        c.div_s(12, 21, 20);
        call_ptr(c, 0x0031c228, 0x0011d734)?;
        c.c_lt_s(20, 22);
        let t = !c.fcc();
        c.mov_s(12, 0);
        if t {
        break 'b_11d764;
        } else {
        'b_11d75c: {
        c.cycles += 3;
        c.c_lt_s(22, 21);
        let t = !c.fcc();

        if t {
        break 'b_11d75c;
        } else {
        c.cycles += 3;
        c.lwc1(0, gp!(0x79a8));
        c.add_s(12, 12, 0);
        break 'b_11d764;
        }
        }
        c.cycles += 2;
        c.lwc1(0, gp!(0x79a4));
        c.sub_s(12, 12, 0);
        break 'b_11d764;
        }
        }
        c.cycles += 1;
        c.mov_s(20, 12);
        break 'b_11d768;
        }
        'b_11d878: {
        c.cycles += 7;
        c.mtc1(0, 21);
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.c_le_s(21, 23);
        let t = !c.fcc();
        c.sw(s1!(0x5b8), c.g[V0][0]);
        if t {
        break 'b_11d878;
        } else {
        'b_11d84c: {
        c.cycles += 47;
        c.lwc1(0, gp!(0x79a0));
        c.g[S0][0] = sx(sp!(0xe0));
        c.mfc1(2, 0);
        c.lqc2(4, s3m!(0xea0));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp!(0x40));
        c.g[AT][0] = 0x43480000;
        c.mtc1(1, 1);
        c.lq(2, sp!(0x40));
        c.g[A1][0] = sx(sp!(0x20));
        c.lwc1(12, gp!(0x799c));
        c.g[A2][0] = sx(sp!(0x30));
        c.sq(2, sp!(0x30));
        c.g[A0][0] = c.g[S0][0];
        c.lqc2(3, (c.g[S2][0] as u32));
        c.lqc2(4, sp!(0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp!(0x40));
        c.g[A3][0] = 0x2;
        c.lq(2, sp!(0x40));
        c.sq(2, sp!(0x20));
        c.mfc1(3, 1);
        c.lqc2(4, s3m!(0xea0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp!(0x50));
        c.lq(2, sp!(0x50));
        c.sq(2, sp!(0x40));
        c.lqc2(3, (c.g[S2][0] as u32));
        c.lqc2(4, sp!(0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp!(0x50));
        c.lq(2, sp!(0x50));
        c.sq(2, sp!(0x30));
        call_ptr(c, 0x0032e100, 0x0011d814)?;
        c.g[V1][0] = c.lw(gp!(0x848));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx(sp!(0x60));
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20));
        call_ptr(c, 0x00336850, 0x0011d830)?;
        c.c_le_s(21, 0);
        let t = !c.fcc();
        c.lq(2, sp!(0x60));
        if t {
        break 'b_11d84c;
        } else {
        c.cycles += 3;
        c.lq(3, sp!(0x70));
        c.sq(2, (c.g[SP][0] as u32));
        c.sq(3, sp!(0x10));
        break 'b_11d84c;
        }
        }
        c.cycles += 11;
        c.mfc1(3, 23);
        c.lqc2(4, s3m!(0xea0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp!(0x50));
        c.lq(2, sp!(0x50));
        c.sq(2, sp!(0x40));
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(4, sp!(0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32));
        break 'b_11d878;
        }
        }
        'b_11d9e8: {
        c.cycles += 48;
        c.lwc1(12, gp!(0x7998));
        c.g[A0][0] = sx(sp!(0x190));
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 21);
        c.g[A1][0] = sx(sp!(0x194));
        c.sub_s(12, 20, 12);
        c.lq(2, (c.g[SP][0] as u32));
        c.g[S2][0] = sx(s3m!(0xea0));
        c.sq(2, s1!(0x110));
        c.mul_s(12, 12, 21);
        call_ptr(c, 0x0031be50, 0x0011d8a4)?;
        c.lwc1(1, s2!(0x4));
        c.lwc1(0, sp!(0x190));
        c.lwc1(2, s3m!(0xea0));
        c.lwc1(4, s2!(0x8));
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.lwc1(3, sp!(0x194));
        c.mul_s(0, 0, 4);
        c.sw(s1!(0x150), 0u64);
        c.swc1(1, sp!(0x24));
        c.swc1(3, sp!(0x2c));
        c.swc1(2, sp!(0x20));
        c.swc1(0, sp!(0x28));
        c.lwc1(1, gp!(0x7994));
        c.lq(2, sp!(0x20));
        c.sq(2, s1!(0x120));
        c.lqc2(4, s3m!(0xea0));
        c.lqc2(5, sp!(0x10));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
        c.sqc2(6, sp!(0x30));
        c.lq(2, sp!(0x30));
        c.sq(2, sp!(0x20));
        c.lqc2(3, sp!(0x20));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 0);
        c.c_lt_s(1, 0);
        let t = !c.fcc();

        if t {
        break 'b_11d9e8;
        } else {
        c.cycles += 44;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 12);
        c.mfc1(2, 0);
        c.qmtc2(2, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, sp!(0x20));
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, sp!(0x20));
        c.min_s(12, 0, 12);
        call_ptr(c, 0x0031c128, 0x0011d964)?;
        c.mul_s(12, 0, 21);
        c.g[A0][0] = sx(sp!(0x198));
        c.g[A1][0] = sx(sp!(0x19c));
        call_ptr(c, 0x0031be50, 0x0011d974)?;
        c.lwc1(0, sp!(0x198));
        c.lwc1(1, sp!(0x20));
        c.lwc1(2, sp!(0x24));
        c.lwc1(4, sp!(0x28));
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.lwc1(3, sp!(0x19c));
        c.mul_s(0, 0, 4);
        c.swc1(3, sp!(0x4c));
        c.swc1(1, sp!(0x40));
        c.swc1(2, sp!(0x44));
        c.swc1(0, sp!(0x48));
        c.lq(2, sp!(0x40));
        c.sq(2, sp!(0x30));
        c.lqc2(4, sp!(0x30));
        c.lqc2(5, s1!(0x120));
        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
        c.sqc2(8, sp!(0x50));
        c.lq(2, sp!(0x50));
        c.sq(2, s1!(0x120));
        c.sq(2, sp!(0x40));
        break 'b_11d9e8;
        }
        }
        'b_11dc4c: {
        c.cycles += 151;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0011e098, 0x0011d9f8)?;
        c.lq(2, sp!(0x10));
        c.g[A0][0] = 0x500000;
        c.lq(3, s1!(0x1b0));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xee0));
        c.sq(2, s1!(0x370));
        c.g[A1][0] = 0xffffffffffffffff;
        c.sq(3, sp!(0x70));
        c.lq(2, (c.g[A0][0] as u32));
        c.sq(3, sp!(0x50));
        c.sq(2, s1!(0x3d0));
        c.lqc2(3, sp!(0x70));
        c.lqc2(5, s1!(0x370));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 0);
        c.mfc1(6, 0);
        c.lqc2(4, s1!(0x370));
        c.qmtc2(6, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp!(0x80));
        c.lq(2, sp!(0x80));
        c.sq(2, sp!(0x60));
        c.lqc2(3, sp!(0x50));
        c.lqc2(4, sp!(0x60));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp!(0x80));
        c.lq(2, sp!(0x80));
        c.sq(2, sp!(0x40));
        c.lqc2(3, sp!(0x40));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, sp!(0x80));
        c.lq(2, sp!(0x80));
        c.sq(2, s1!(0x3a0));
        c.sq(2, sp!(0x30));
        c.lqc2(4, s1!(0x370));
        c.lqc2(5, s1!(0x3a0));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
        c.sqc2(6, sp!(0x40));
        c.lq(2, sp!(0x40));
        c.sq(2, sp!(0x30));
        c.sq(2, s1!(0x3b0));
        c.lq(3, s1!(0x110));
        c.lq(2, (c.g[A0][0] as u32));
        c.sq(3, s1!(0x460));
        c.sq(2, s1!(0x1e0));
        c.sw(s1!(0x438), 0u64);
        c.sw(s1!(0x454), 0u64);
        c.lq(2, (c.g[A0][0] as u32));
        c.sw(s1!(0x3f0), 0u64);
        c.sq(2, s1!(0x3e0));
        c.sw(s1!(0x3f4), 0u64);
        c.sw(s1!(0x1f8), 0u64);
        c.sw(s1!(0x1f0), 0u64);
        c.sw(s1!(0x1f4), 0u64);
        c.sw(s1!(0x204), 0u64);
        c.sw(s1!(0x1fc), 0u64);
        c.sw(s1!(0x200), 0u64);
        c.sw(s1!(0x210), 0u64);
        c.sw(s1!(0x208), 0u64);
        c.sw(s1!(0x20c), 0u64);
        c.sw(s1!(0x21c), 0u64);
        c.sw(s1!(0x214), 0u64);
        c.sw(s1!(0x218), 0u64);
        c.sw(s1!(0x228), 0u64);
        c.sw(s1!(0x220), 0u64);
        c.sw(s1!(0x224), 0u64);
        c.sw(s1!(0x234), 0u64);
        c.sw(s1!(0x22c), 0u64);
        c.sw(s1!(0x230), 0u64);
        c.sw(s1!(0x240), 0u64);
        c.sw(s1!(0x238), 0u64);
        c.sw(s1!(0x23c), 0u64);
        c.sw(s1!(0x24c), 0u64);
        c.sw(s1!(0x244), 0u64);
        c.sw(s1!(0x248), 0u64);
        c.sw(s1!(0x258), 0u64);
        c.sw(s1!(0x250), 0u64);
        c.sw(s1!(0x254), 0u64);
        c.swc1(20, s1!(0x264));
        c.swc1(20, s1!(0x25c));
        c.sw(s1!(0x260), 0u64);
        c.sw(s1!(0x270), 0u64);
        c.sw(s1!(0x268), 0u64);
        c.sw(s1!(0x26c), 0u64);
        c.sw(s1!(0x27c), 0u64);
        c.sw(s1!(0x274), 0u64);
        c.sw(s1!(0x278), 0u64);
        c.sw(s1!(0x288), 0u64);
        c.sw(s1!(0x280), 0u64);
        c.sw(s1!(0x284), 0u64);
        c.sw(s1!(0x294), 0u64);
        c.sw(s1!(0x28c), 0u64);
        c.sw(s1!(0x290), 0u64);
        c.sw(s1!(0x2a0), 0u64);
        c.sw(s1!(0x298), 0u64);
        c.sw(s1!(0x29c), 0u64);
        c.sw(s1!(0x2ac), 0u64);
        c.sw(s1!(0x2a4), 0u64);
        c.sw(s1!(0x2a8), 0u64);
        c.sw(s1!(0x2b8), 0u64);
        c.sw(s1!(0x2b0), 0u64);
        c.sw(s1!(0x2b4), 0u64);
        c.sw(s1!(0x2c4), 0u64);
        c.sw(s1!(0x2bc), 0u64);
        c.sw(s1!(0x2c0), 0u64);
        c.sw(s1!(0x2d0), 0u64);
        c.sw(s1!(0x2c8), 0u64);
        c.sw(s1!(0x2cc), 0u64);
        c.sw(s1!(0x2dc), 0u64);
        c.sw(s1!(0x2e0), 0u64);
        c.sw(s1!(0x2fc), 0u64);
        c.swc1(20, s1!(0x300));
        c.swc1(20, s1!(0x318));
        c.sw(s1!(0x31c), 0u64);
        c.sw(s1!(0x328), 0u64);
        c.sw(s1!(0x32c), 0u64);
        c.sw(s1!(0x330), 0u64);
        c.g[V1][0] = c.lw(s1!(0x324));
        c.lq(2, (c.g[A0][0] as u32));
        c.g[A0][0] = c.lw(s1!(0x784));
        c.sw(s1!(0x350), 0u64);
        c.sw(s1!(0x358), 0u64);
        c.sw(s1!(0x35c), 0u64);
        c.sq(2, s1!(0x340));
        c.sw(s1!(0x320), c.g[V1][0]);
        c.sw(s1!(0x354), c.g[A1][0]);
        call_ptr(c, 0x00311a50, 0x0011dc28)?;
        c.g[A0][0] = c.lw(s1!(0x784));
        c.g[V1][0] = c.lw(s1!(0x320));
        c.mtc1(0, 12);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
        c.g[V0][0] = c.lw(s1!(0x320));
        if c.g[V0][0] == 0u64 {
        c.neg_s(12, 12);
        break 'b_11dc4c;
        } else {
        c.cycles += 2;
        c.lwc1(12, gp!(0x7990));
        c.neg_s(12, 12);
        break 'b_11dc4c;
        }
        }
        'b_11dd64: {
        c.cycles += 58;
        c.g[V0][0] = 0x500000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
        c.g[S0][0] = c.lw(s1!(0x784));
        c.lq(3, (c.g[V0][0] as u32));
        c.g[A0][0] = sx(sp!(0x1a0));
        c.mul_s(12, 12, 21);
        c.g[A1][0] = sx(sp!(0x1a4));
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0x30));
        call_ptr(c, 0x0031be50, 0x0011dc70)?;
        c.lwc1(0, sp!(0x1a0));
        c.g[A2][0] = 0u64;
        c.lwc1(1, s2!(0x8));
        c.g[A1][0] = c.g[S4][0];
        c.lwc1(2, s3m!(0xea0));
        c.lwc1(4, s2!(0x4));
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.lwc1(3, sp!(0x1a4));
        c.mul_s(0, 0, 4);
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 12);
        c.swc1(3, sp!(0x3c));
        c.swc1(1, sp!(0x38));
        c.swc1(2, sp!(0x30));
        c.swc1(0, sp!(0x34));
        c.lq(2, sp!(0x30));
        c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = c.lw(s1!(0x784));
        call_ptr(c, 0x003128e8, 0x0011dcc0)?;
        c.g[A0][0] = c.lw(s1!(0x784));
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x00312598, 0x0011dccc)?;
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0011eb60, 0x0011dcd8)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0011eb98, 0x0011dce0)?;
        c.g[A0][0] = c.lw(s1!(0x780));
        call_ptr(c, 0x003103f0, 0x0011dce8)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0011e150, 0x0011dcf4)?;
        c.g[A0][0] = c.lw(s1!(0x860));
        c.g[A1][0] = sx(s1!(0x400));
        call_ptr(c, 0x00332db8, 0x0011dd00)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00122088, 0x0011dd08)?;
        c.g[A0][0] = c.lw(s1!(0x86c));
        call_ptr(c, 0x002105b0, 0x0011dd10)?;
        c.g[V0][0] = c.lw(s1!(0x6c0));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0011dd24)?;
        c.g[A0][0] = c.lw(s1!(0x870));
        c.g[V0][0] = (c.g[A0][0] < 0x2u64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.lw(gp!(0x848));
        if t {
        break 'b_11dd64;
        } else {
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        let t = c.g[V1][0] == 0u64;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        if t {
        break 'b_11dd64;
        } else {
        c.cycles += 8;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0011dd64)?;
        break 'b_11dd64;
        }
        }
        }
        c.cycles += 12;
        c.lq(16, sp!(0x200));
        c.lq(17, sp!(0x1f0));
        c.lq(18, sp!(0x1e0));
        c.lq(19, sp!(0x1d0));
        c.lq(20, sp!(0x1c0));
        c.g[RA][0] = c.ld(sp!(0x1b0));
        c.lwc1(23, sp!(0x228));
        c.lwc1(22, sp!(0x220));
        c.lwc1(21, sp!(0x218));
        c.lwc1(20, sp!(0x210));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp!(0x230));
        c.pc = t;
        Ok(())
        })
        }


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::GAME_GP;

    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{fill, put_vec, put_words, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's callee fakes (other tests' machines fall back).
    const RP_CTRL: u32 = DATA + 0x20_6000;
    const RP_MAGIC: u32 = 0x7e70_91ac;

    macro_rules! rp_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(RP_CTRL) != RP_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    rp_gated!(rp_fake_0031c228, RP_PREV_0031c228, 0x0031c228, ee::s10::f_0031c228__generated);
    rp_gated!(rp_fake_0032e100, RP_PREV_0032e100, 0x0032e100, ee::s11::f_0032e100__generated);
    rp_gated!(rp_fake_00336850, RP_PREV_00336850, 0x00336850, ee::s11::f_00336850__generated);
    rp_gated!(rp_fake_0031be50, RP_PREV_0031be50, 0x0031be50, ee::s10::f_0031be50__generated);
    rp_gated!(rp_fake_0031c128, RP_PREV_0031c128, 0x0031c128, ee::s10::f_0031c128__generated);
    rp_gated!(rp_fake_0011e098, RP_PREV_0011e098, 0x0011e098, ee::s00::f_0011e098__generated);
    rp_gated!(rp_fake_00311a50, RP_PREV_00311a50, 0x00311a50, ee::s10::f_00311a50__generated);
    rp_gated!(rp_fake_003128e8, RP_PREV_003128e8, 0x003128e8, ee::s10::f_003128e8__generated);
    rp_gated!(rp_fake_00312598, RP_PREV_00312598, 0x00312598, ee::s10::f_00312598__generated);
    rp_gated!(rp_fake_0011eb60, RP_PREV_0011eb60, 0x0011eb60, ee::s00::f_0011eb60__generated);
    rp_gated!(rp_fake_0011eb98, RP_PREV_0011eb98, 0x0011eb98, ee::s00::f_0011eb98__generated);
    rp_gated!(rp_fake_003103f0, RP_PREV_003103f0, 0x003103f0, ee::s10::f_003103f0__generated);
    rp_gated!(rp_fake_0011e150, RP_PREV_0011e150, 0x0011e150, ee::s00::f_0011e150__generated);
    rp_gated!(rp_fake_00332db8, RP_PREV_00332db8, 0x00332db8, ee::s11::f_00332db8__generated);
    rp_gated!(rp_fake_00122088, RP_PREV_00122088, 0x00122088, ee::s00::f_00122088__generated);
    rp_gated!(rp_fake_002105b0, RP_PREV_002105b0, 0x002105b0, ee::s05::f_002105b0__generated);

    fn rp_install_fakes() {
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_0031C228), rp_fake_0031c228, &RP_PREV_0031c228);
        fakes::hook(addr_of_mut!(ee::s11::OVERRIDE_0032E100), rp_fake_0032e100, &RP_PREV_0032e100);
        fakes::hook(addr_of_mut!(ee::s11::OVERRIDE_00336850), rp_fake_00336850, &RP_PREV_00336850);
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_0031BE50), rp_fake_0031be50, &RP_PREV_0031be50);
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_0031C128), rp_fake_0031c128, &RP_PREV_0031c128);
        fakes::hook(addr_of_mut!(ee::s00::OVERRIDE_0011E098), rp_fake_0011e098, &RP_PREV_0011e098);
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_00311A50), rp_fake_00311a50, &RP_PREV_00311a50);
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_003128E8), rp_fake_003128e8, &RP_PREV_003128e8);
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_00312598), rp_fake_00312598, &RP_PREV_00312598);
        fakes::hook(addr_of_mut!(ee::s00::OVERRIDE_0011EB60), rp_fake_0011eb60, &RP_PREV_0011eb60);
        fakes::hook(addr_of_mut!(ee::s00::OVERRIDE_0011EB98), rp_fake_0011eb98, &RP_PREV_0011eb98);
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_003103F0), rp_fake_003103f0, &RP_PREV_003103f0);
        fakes::hook(addr_of_mut!(ee::s00::OVERRIDE_0011E150), rp_fake_0011e150, &RP_PREV_0011e150);
        fakes::hook(addr_of_mut!(ee::s11::OVERRIDE_00332DB8), rp_fake_00332db8, &RP_PREV_00332db8);
        fakes::hook(addr_of_mut!(ee::s00::OVERRIDE_00122088), rp_fake_00122088, &RP_PREV_00122088);
        fakes::hook(addr_of_mut!(ee::s05::OVERRIDE_002105B0), rp_fake_002105b0, &RP_PREV_002105b0);
    }

    const RP_RIDER: u32 = DATA + 0x1_0000;
    const RP_POS: u32 = DATA + 0x2000;
    const RP_DIR: u32 = DATA + 0x2100;
    const RP_WORLD: u32 = DATA + 0x4000;
    const RP_W84: u32 = DATA + 0x4100;
    const RP_TAB: u32 = DATA + 0x4200;
    const RP_ENT: u32 = DATA + 0x4300;
    const RP_OBJ: u32 = DATA + 0x4400;
    const RP_VT2: u32 = DATA + 0x4500;
    const RP_VT: u32 = DATA + 0x4600;
    const RP_ANIM: u32 = DATA + 0x3200;

    fn rp_f(x: f32) -> u32 {
        x.to_bits()
    }

    /// `Rider_Place` against the generated body: dir.x/dir.z zero, negative
    /// or positive (all heading quadrants), a snap radius that is negative
    /// or not (ray-cast block, hit or miss), a degenerate or regular up
    /// axis, a zero or non-zero `+0x324`, and every tail path (AI index
    /// below 2 with and without the table, or not). The callees are gated
    /// fakes; some unwind.
    #[test]
    fn rider_place_matches() {
        rp_install_fakes();
        covered(
            ee::s00::f_0011d660__generated,
            rider_place,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11d660, 18);
                c.pc = 0x0011_d660;
                c.bus.write32(RP_CTRL, RP_MAGIC);
                // Constants: gp-relative literals and the world axes.
                let g = GAME_GP;
                let lits: [(u32, f32); 9] = [
                    (0x79b0, -1.5708),
                    (0x79ac, 1.5708),
                    (0x79a8, 3.14159),
                    (0x79a4, 3.14159),
                    (0x79a0, 0.5),
                    (0x799c, 2.0),
                    (0x7998, 0.25),
                    (0x7994, 0.001),
                    (0x7990, 3.0),
                ];
                for (o, v) in lits {
                    c.bus.write32(g - o, rp_f(v));
                }
                for a in [0x4ff160u32, 0x4ff120, 0x4ff130] {
                    let v = [r.f32(2.0), r.f32(2.0), r.f32(2.0), 0];
                    put_vec(c, a, v);
                }
                // Junk where the ray-cast writes its hit (sp+0x60 of the callee).
                let sp = c.g[SP][0] as u32;
                fill(c, &mut r, sp - 0x230 + 0x40, 0x30);
                // Rider and its pointers.
                fill(c, &mut r, RP_RIDER, 0x240);
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                w(c, RP_RIDER + 0x864, DATA + 0x3000);
                w(c, RP_RIDER + 0x868, DATA + 0x3100);
                w(c, RP_RIDER + 0x784, RP_ANIM);
                w(c, RP_RIDER + 0x6c0, RP_VT);
                c.bus.write16(RP_VT + 0x80, 0);
                w(c, RP_VT + 0x84, FAKE);
                if seed % 2 == 0 {
                    w(c, RP_RIDER + 0x324, 0);
                }
                let ai = [0u32, 1, 2, 0xffff_ffff, 1][(seed / 5 % 5) as usize];
                w(c, RP_RIDER + 0x870, ai);
                // World and the per-AI object chain.
                w(c, GAME_GP - 0x848, RP_WORLD);
                w(c, RP_WORLD + 0x84, RP_W84);
                w(c, RP_W84 + 0x84, if seed % 7 == 0 { 0 } else { RP_TAB });
                w(c, RP_W84 + 0x20, DATA + 0x4700);
                w(c, RP_TAB + 4, RP_ENT);
                w(c, RP_TAB + 8, RP_ENT);
                w(c, RP_ENT + 0xa8, RP_OBJ);
                w(c, RP_OBJ + 0x14, RP_VT2);
                c.bus.write16(RP_VT2 + 0x20, 0);
                w(c, RP_VT2 + 0x24, FAKE + 0x10);
                // Arguments.
                let comp = [0.0f32, 1.0, -1.0];
                let x = comp[(seed % 3) as usize];
                let y = comp[(seed / 3 % 3) as usize];
                w(c, RP_DIR, rp_f(x));
                w(c, RP_DIR + 4, rp_f(y));
                for i in 2..4 {
                    w(c, RP_DIR + 4 * i, r.next_u32());
                }
                let pos = [r.f32(100.0), r.f32(100.0), r.f32(100.0), rp_f(1.0)];
                put_words(c, RP_POS, &pos);
                c.g[A0][0] = RP_RIDER as u64;
                c.g[A1][0] = RP_POS as u64;
                c.g[A2][0] = RP_DIR as u64;
                c.g[A3][0] = (seed % 6) as u64;
                let snaps = [-1.0f32, 0.0, 5.0, 100.0];
                c.f[12] = rp_f(snaps[(seed / 2 % 4) as usize]);
            },
        );
    }
}
