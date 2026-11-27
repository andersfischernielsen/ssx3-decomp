//! Staging module oc1095 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1095`.

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
    (0x003b_ffa0, sndenc_filter_stage9, "SNDENC_FilterStage9"),
    (0x003c_01a0, sndenc_filter_stage10, "SNDENC_FilterStage10"),
];

/// `void SNDENC_FilterStage9(const f32 *in /*a0*/, const f32 *coef /*a1*/, f32 *out /*a2*/)`
/// (0x003bffa0): a straight-line VU0 butterfly stage of the sound encoder
/// (fixed 128 cycles, leaf, no callees). The resume entries of the
/// generated twin exist only for the cycle-budget event, which a leaf never
/// reaches mid-body in the port.
pub fn sndenc_filter_stage9(c: &mut Cpu) {
    if c.pc != 0x003b_ffa0 {
        return ee::s13::f_003bffa0__entries(c);
    }
    c.cycles += 128;
    c.lqc2(15, (c.g[A1][0] as u32));
    c.lqc2(16, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.lqc2(1, (c.g[A0][0] as u32));
    c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x80));
    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x90));
    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0xa0));
    c.lqc2(8, (c.g[A0][0] as u32).wrapping_add(0xb0));
    c.vu0().add(0b1111, 9, 1, Src::Vec(5));
    c.vu0().add(0b1111, 10, 2, Src::Vec(6));
    c.vu0().add(0b1111, 11, 3, Src::Vec(7));
    c.vu0().add(0b1111, 12, 4, Src::Vec(8));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(15, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(15, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(15, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(15, W));
    c.sqc2(9, (c.g[A2][0] as u32));
    c.sqc2(10, (c.g[A2][0] as u32).wrapping_add(0x10));
    c.sqc2(11, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.sqc2(12, (c.g[A2][0] as u32).wrapping_add(0x30));
    c.vu0().sub(0b1111, 9, 8, Src::Vec(4));
    c.vu0().sub(0b1111, 10, 7, Src::Vec(3));
    c.vu0().sub(0b1111, 11, 6, Src::Vec(2));
    c.vu0().sub(0b1111, 12, 5, Src::Vec(1));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(16, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(16, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(16, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(16, W));
    c.sqc2(9, (c.g[A2][0] as u32).wrapping_add(0xc0));
    c.sqc2(10, (c.g[A2][0] as u32).wrapping_add(0xd0));
    c.sqc2(11, (c.g[A2][0] as u32).wrapping_add(0xe0));
    c.sqc2(12, (c.g[A2][0] as u32).wrapping_add(0xf0));
    c.lqc2(15, (c.g[A1][0] as u32).wrapping_add(0x10));
    c.lqc2(16, (c.g[A1][0] as u32).wrapping_add(0x20));
    c.lqc2(1, (c.g[A0][0] as u32).wrapping_add(0x40));
    c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x50));
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x60));
    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x70));
    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0xc0));
    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0xd0));
    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0xe0));
    c.lqc2(8, (c.g[A0][0] as u32).wrapping_add(0xf0));
    c.vu0().add(0b1111, 9, 1, Src::Vec(5));
    c.vu0().add(0b1111, 10, 2, Src::Vec(6));
    c.vu0().add(0b1111, 11, 3, Src::Vec(7));
    c.vu0().add(0b1111, 12, 4, Src::Vec(8));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(15, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(15, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(15, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(15, W));
    c.sqc2(9, (c.g[A2][0] as u32).wrapping_add(0x40));
    c.sqc2(10, (c.g[A2][0] as u32).wrapping_add(0x50));
    c.sqc2(11, (c.g[A2][0] as u32).wrapping_add(0x60));
    c.sqc2(12, (c.g[A2][0] as u32).wrapping_add(0x70));
    c.vu0().sub(0b1111, 9, 8, Src::Vec(4));
    c.vu0().sub(0b1111, 10, 7, Src::Vec(3));
    c.vu0().sub(0b1111, 11, 6, Src::Vec(2));
    c.vu0().sub(0b1111, 12, 5, Src::Vec(1));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(16, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(16, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(16, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(16, W));
    c.sqc2(9, (c.g[A2][0] as u32).wrapping_add(0x80));
    c.sqc2(10, (c.g[A2][0] as u32).wrapping_add(0x90));
    c.sqc2(11, (c.g[A2][0] as u32).wrapping_add(0xa0));
    c.sqc2(12, (c.g[A2][0] as u32).wrapping_add(0xb0));
    c.lqc2(1, (c.g[A1][0] as u32).wrapping_add(0x40));
    c.lqc2(2, (c.g[A1][0] as u32).wrapping_add(0x50));
    c.lqc2(9, (c.g[A0][0] as u32).wrapping_add(0x180));
    c.lqc2(10, (c.g[A0][0] as u32).wrapping_add(0x190));
    c.lqc2(11, (c.g[A0][0] as u32).wrapping_add(0x1a0));
    c.lqc2(12, (c.g[A0][0] as u32).wrapping_add(0x1b0));
    c.lqc2(13, (c.g[A0][0] as u32).wrapping_add(0x1c0));
    c.lqc2(14, (c.g[A0][0] as u32).wrapping_add(0x1d0));
    c.lqc2(15, (c.g[A0][0] as u32).wrapping_add(0x1e0));
    c.lqc2(16, (c.g[A0][0] as u32).wrapping_add(0x1f0));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(1, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(1, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(1, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(1, W));
    c.vu0().mul(0b1111, 13, 13, Src::Bc(2, X));
    c.vu0().mul(0b1111, 14, 14, Src::Bc(2, Y));
    c.vu0().mul(0b1111, 15, 15, Src::Bc(2, Z));
    c.vu0().mul(0b1111, 16, 16, Src::Bc(2, W));
    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x130));
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x120));
    c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x110));
    c.lqc2(1, (c.g[A0][0] as u32).wrapping_add(0x100));
    c.vu0().sub(0b1111, 5, 4, Src::Vec(12));
    c.vu0().sub(0b1111, 6, 3, Src::Vec(11));
    c.vu0().sub(0b1111, 7, 2, Src::Vec(10));
    c.vu0().sub(0b1111, 8, 1, Src::Vec(9));
    c.vu0().add(0b1111, 1, 1, Src::Vec(9));
    c.vu0().add(0b1111, 2, 2, Src::Vec(10));
    c.vu0().add(0b1111, 3, 3, Src::Vec(11));
    c.vu0().add(0b1111, 4, 4, Src::Vec(12));
    c.sqc2(1, (c.g[A2][0] as u32).wrapping_add(0x100));
    c.sqc2(2, (c.g[A2][0] as u32).wrapping_add(0x110));
    c.sqc2(3, (c.g[A2][0] as u32).wrapping_add(0x120));
    c.sqc2(4, (c.g[A2][0] as u32).wrapping_add(0x130));
    c.sqc2(5, (c.g[A2][0] as u32).wrapping_add(0x1c0));
    c.sqc2(6, (c.g[A2][0] as u32).wrapping_add(0x1d0));
    c.sqc2(7, (c.g[A2][0] as u32).wrapping_add(0x1e0));
    c.sqc2(8, (c.g[A2][0] as u32).wrapping_add(0x1f0));
    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x170));
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x160));
    c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x150));
    c.lqc2(1, (c.g[A0][0] as u32).wrapping_add(0x140));
    c.vu0().sub(0b1111, 5, 4, Src::Vec(16));
    c.vu0().sub(0b1111, 6, 3, Src::Vec(15));
    c.vu0().sub(0b1111, 7, 2, Src::Vec(14));
    c.vu0().sub(0b1111, 8, 1, Src::Vec(13));
    c.vu0().add(0b1111, 1, 1, Src::Vec(13));
    c.vu0().add(0b1111, 2, 2, Src::Vec(14));
    c.vu0().add(0b1111, 3, 3, Src::Vec(15));
    c.vu0().add(0b1111, 4, 4, Src::Vec(16));
    c.sqc2(1, (c.g[A2][0] as u32).wrapping_add(0x140));
    c.sqc2(2, (c.g[A2][0] as u32).wrapping_add(0x150));
    c.sqc2(3, (c.g[A2][0] as u32).wrapping_add(0x160));
    c.sqc2(4, (c.g[A2][0] as u32).wrapping_add(0x170));
    c.sqc2(5, (c.g[A2][0] as u32).wrapping_add(0x180));
    c.sqc2(6, (c.g[A2][0] as u32).wrapping_add(0x190));
    c.sqc2(7, (c.g[A2][0] as u32).wrapping_add(0x1a0));
    c.sqc2(8, (c.g[A2][0] as u32).wrapping_add(0x1b0));
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

/// `void SNDENC_FilterStage10(const f32 *in /*a0*/, const f32 *coef /*a1*/, f32 *out /*a2*/, u32 stride /*a3*/)`
/// (0x003c01a0): the same kind of butterfly stage, writing its output
/// blocks `stride` bytes apart (a2 advances by a3 after each block).
pub fn sndenc_filter_stage10(c: &mut Cpu) {
    if c.pc != 0x003c_01a0 {
        return ee::s13::f_003c01a0__entries(c);
    }
    c.cycles += 143;
    c.lqc2(16, (c.g[A1][0] as u32));
    c.lqc2(1, (c.g[A0][0] as u32));
    c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x100));
    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x110));
    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x120));
    c.lqc2(8, (c.g[A0][0] as u32).wrapping_add(0x130));
    c.vu0().add(0b1111, 9, 1, Src::Vec(5));
    c.vu0().add(0b1111, 10, 2, Src::Vec(6));
    c.vu0().add(0b1111, 11, 3, Src::Vec(7));
    c.vu0().add(0b1111, 12, 4, Src::Vec(8));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(16, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(16, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(16, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(16, W));
    c.sqc2(9, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(10, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(11, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(12, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.lqc2(16, (c.g[A1][0] as u32).wrapping_add(0x10));
    c.lqc2(1, (c.g[A0][0] as u32).wrapping_add(0x40));
    c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x50));
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x60));
    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x70));
    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x140));
    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x150));
    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x160));
    c.lqc2(8, (c.g[A0][0] as u32).wrapping_add(0x170));
    c.vu0().add(0b1111, 9, 1, Src::Vec(5));
    c.vu0().add(0b1111, 10, 2, Src::Vec(6));
    c.vu0().add(0b1111, 11, 3, Src::Vec(7));
    c.vu0().add(0b1111, 12, 4, Src::Vec(8));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(16, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(16, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(16, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(16, W));
    c.sqc2(9, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(10, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(11, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(12, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.lqc2(16, (c.g[A1][0] as u32).wrapping_add(0x20));
    c.lqc2(1, (c.g[A0][0] as u32).wrapping_add(0x80));
    c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x90));
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0xa0));
    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0xb0));
    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x180));
    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x190));
    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x1a0));
    c.lqc2(8, (c.g[A0][0] as u32).wrapping_add(0x1b0));
    c.vu0().add(0b1111, 9, 1, Src::Vec(5));
    c.vu0().add(0b1111, 10, 2, Src::Vec(6));
    c.vu0().add(0b1111, 11, 3, Src::Vec(7));
    c.vu0().add(0b1111, 12, 4, Src::Vec(8));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(16, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(16, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(16, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(16, W));
    c.sqc2(9, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(10, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(11, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(12, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.lqc2(16, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.lqc2(1, (c.g[A0][0] as u32).wrapping_add(0xc0));
    c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0xd0));
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0xe0));
    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0xf0));
    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x1c0));
    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x1d0));
    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x1e0));
    c.lqc2(8, (c.g[A0][0] as u32).wrapping_add(0x1f0));
    c.vu0().add(0b1111, 9, 1, Src::Vec(5));
    c.vu0().add(0b1111, 10, 2, Src::Vec(6));
    c.vu0().add(0b1111, 11, 3, Src::Vec(7));
    c.vu0().add(0b1111, 12, 4, Src::Vec(8));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(16, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(16, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(16, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(16, W));
    c.sqc2(9, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(10, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(11, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(12, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.lqc2(16, (c.g[A1][0] as u32).wrapping_add(0x40));
    c.vu0().sub(0b1111, 9, 8, Src::Vec(4));
    c.vu0().sub(0b1111, 10, 7, Src::Vec(3));
    c.vu0().sub(0b1111, 11, 6, Src::Vec(2));
    c.vu0().sub(0b1111, 12, 5, Src::Vec(1));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(16, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(16, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(16, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(16, W));
    c.sqc2(9, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(10, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(11, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(12, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.lqc2(16, (c.g[A1][0] as u32).wrapping_add(0x50));
    c.lqc2(8, (c.g[A0][0] as u32).wrapping_add(0x1b0));
    c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x1a0));
    c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x190));
    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x180));
    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0xb0));
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0xa0));
    c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x90));
    c.lqc2(1, (c.g[A0][0] as u32).wrapping_add(0x80));
    c.vu0().sub(0b1111, 9, 8, Src::Vec(4));
    c.vu0().sub(0b1111, 10, 7, Src::Vec(3));
    c.vu0().sub(0b1111, 11, 6, Src::Vec(2));
    c.vu0().sub(0b1111, 12, 5, Src::Vec(1));
    c.vu0().mul(0b1111, 9, 9, Src::Bc(16, X));
    c.vu0().mul(0b1111, 10, 10, Src::Bc(16, Y));
    c.vu0().mul(0b1111, 11, 11, Src::Bc(16, Z));
    c.vu0().mul(0b1111, 12, 12, Src::Bc(16, W));
    c.sqc2(9, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(10, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(11, (c.g[A2][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sqc2(12, (c.g[A2][0] as u32));
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA};

    /// Random input, coefficient and output areas; the output is pre-filled
    /// so unwritten bytes are compared too.
    fn oc1095_fixture(c: &mut Cpu, seed: u32, stride: u32) {
        let mut r = Rng(0x1095_0001 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x2000);
        c.g[A2][0] = sx(DATA + 0x8000);
        c.g[A3][0] = sx(stride);
        fill(c, &mut r, DATA, 0x200);
        fill(c, &mut r, DATA + 0x2000, 0x40);
        fill(c, &mut r, DATA + 0x8000, 0x800);
    }

    #[test]
    fn sndenc_filter_stage9_matches() {
        check(
            ee::s13::f_003bffa0__generated,
            sndenc_filter_stage9,
            40,
            &|c, seed| {
                oc1095_fixture(c, seed, 0);
                c.pc = 0x003b_ffa0;
            },
        );
    }

    #[test]
    fn sndenc_filter_stage10_matches() {
        check(
            ee::s13::f_003c01a0__generated,
            sndenc_filter_stage10,
            40,
            &|c, seed| {
                oc1095_fixture(c, seed, 0x10 * (1 + seed % 8));
                c.pc = 0x003c_01a0;
            },
        );
    }
}
