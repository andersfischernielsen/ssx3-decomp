//! Native MultiSplineModifier_UpdateNodes35ac20, retained uninstalled.
//! Its differential test keeps the real bounded path and trigonometry callees.

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0035_ac20;
const DISTANCES: u32 = 0x0035_aca0;
const WRAP_ENTRY: u32 = 0x0035_acb8;
const WRAP_DISTANCE: u32 = 0x0035_acd0;
const YAW_SETUP: u32 = 0x0035_ad10;
const AFTER_YAW_OFFSET: u32 = 0x0035_ad3c;
const ELEMENTS: u32 = 0x0035_ae04;
const EVALUATE: u32 = 0x0035_ae28;
const AFTER_EVALUATE: u32 = 0x0035_ae44;
const AFTER_YAW_ATAN: u32 = 0x0035_aeb8;
const PITCH: u32 = 0x0035_aee0;
const AFTER_PITCH_ATAN: u32 = 0x0035_af54;
const MATRIX: u32 = 0x0035_af80;
const AFTER_ROLL: u32 = 0x0035_afa4;
const AFTER_PITCH: u32 = 0x0035_afb8;
const AFTER_YAW: u32 = 0x0035_afc8;
const NEXT: u32 = 0x0035_b198;
const FINISH: u32 = 0x0035_b1ac;
const END: u32 = 0x0035_b200;

const COUNT: u32 = 4;
const MODE: u32 = 8;
const YAW_OFFSET: u32 = 0xc;
const DISTANCE: u32 = 0x10;
const DIRTY: u32 = 0x30;
const ELEMENT_TABLE: u32 = 0x3c;
const INSTANCES: u32 = 0x44;
const FOLLOW: u32 = 0x48;
const LENGTH: u32 = 0x54;
const FRAME_BYTES: u32 = 0x5d0;
const SIN_COS: u32 = 0x0031_be50;
const ATAN: u32 = 0x0031_c228;
const EVAL_PATH: Callee = Callee {
    addr: 0x0034_5248,
    f: ssx3_ee::s11::SplineModifier_EvalPathAtDistance,
};

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 {
    r(c, reg).wrapping_add(offset)
}

/// Construct the optional constant-axis yaw-offset matrix at stack+400.
/// The axis is the native vector at4ff160; scratch writes/registers retain
/// the generated Rodrigues construction and its FPU rounding order.
fn yaw_offset_matrix(c: &mut Cpu) {
    c.lwc1(11, at(c, SP, 0x4d4));
    c.g[V0][0] = sx(r(c, S0).wrapping_sub(0xea0));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 5);
    c.lwc1(3, at(c, V0, 8));
    c.sub_s(1, 5, 11);
    c.lwc1(0, r(c, S0).wrapping_sub(0xea0));
    c.lwc1(6, at(c, V0, 4));
    c.lwc1(8, at(c, SP, 0x4d0));
    c.mul_s(2, 1, 3);
    c.swc1(5, at(c, SP, 0x43c));
    c.mul_s(4, 1, 0);
    c.swc1(20, at(c, SP, 0x430));
    c.mul_s(1, 1, 6);
    c.swc1(20, at(c, SP, 0x434));
    c.mul_s(13, 8, 3);
    c.swc1(20, at(c, SP, 0x438));
    c.mul_s(12, 8, 0);
    c.swc1(20, at(c, SP, 0x40c));
    c.mul_s(9, 2, 3);
    c.swc1(20, at(c, SP, 0x41c));
    c.mul_s(10, 4, 3);
    c.swc1(20, at(c, SP, 0x42c));
    c.mul_s(5, 2, 0);
    c.mul_s(7, 4, 0);
    c.mul_s(3, 1, 3);
    c.mul_s(0, 1, 0);
    c.mul_s(8, 8, 6);
    c.mul_s(2, 2, 6);
    c.mul_s(4, 4, 6);
    c.mul_s(1, 1, 6);
    c.add_s(5, 5, 8);
    c.sub_s(0, 0, 13);
    c.sub_s(2, 2, 12);
    c.add_s(9, 9, 11);
    c.swc1(5, at(c, SP, 0x408));
    c.add_s(7, 7, 11);
    c.swc1(0, at(c, SP, 0x404));
    c.add_s(4, 4, 13);
    c.swc1(2, at(c, SP, 0x418));
    c.sub_s(10, 10, 8);
    c.swc1(9, at(c, SP, 0x428));
    c.add_s(1, 1, 11);
    c.swc1(7, at(c, SP, 0x400));
    c.add_s(3, 3, 12);
    c.swc1(4, at(c, SP, 0x410));
    c.swc1(10, at(c, SP, 0x420));
    c.swc1(1, at(c, SP, 0x414));
    c.swc1(3, at(c, SP, 0x424));
    c.g[V0][0] = c.lw(at(c, S2, COUNT));
}

/// Fill Instance+10's Euler rotation and translation from an evaluated path.
/// Sin/cos pairs are stack4d8..4ec; this deliberately retains every scalar
/// operation rather than replacing it with a host trigonometric/matrix API.
fn instance_matrix(c: &mut Cpu) {
    c.lwc1(0, at(c, SP, 0x4ec));
    c.lwc1(1, at(c, SP, 0x4e4));
    c.lwc1(2, at(c, SP, 0x4e0));
    c.mul_s(1, 1, 0);
    c.lwc1(3, at(c, SP, 0x4d8));
    c.lwc1(0, at(c, SP, 0x4e8));
    c.lwc1(4, at(c, SP, 0x4dc));
    c.mul_s(3, 3, 2);
    c.swc1(1, at(c, S0, 0x10));
    c.mul_s(4, 4, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 5);
    c.lwc1(0, at(c, SP, 0x4e4));
    c.lwc1(1, at(c, SP, 0x4e8));
    c.neg_s(0, 0);
    c.mul_s(0, 0, 1);
    c.swc1(0, at(c, S0, 0x14));
    c.lwc1(1, at(c, SP, 0x4e0));
    c.sw(at(c, S0, 0x1c), 0);
    c.swc1(1, at(c, S0, 0x18));
    c.lwc1(0, at(c, SP, 0x4ec));
    c.mul_s(0, 3, 0);
    c.add_s(0, 0, 4);
    c.swc1(0, at(c, S0, 0x20));
    c.lwc1(0, at(c, SP, 0x4ec));
    c.lwc1(2, at(c, SP, 0x4e8));
    c.lwc1(1, at(c, SP, 0x4dc));
    c.mul_s(3, 3, 2);
    c.mul_s(1, 1, 0);
    c.sub_s(1, 1, 3);
    c.swc1(1, at(c, S0, 0x24));
    c.lwc1(0, at(c, SP, 0x4d8));
    c.lwc1(1, at(c, SP, 0x4e4));
    c.neg_s(0, 0);
    c.sw(at(c, S0, 0x2c), 0);
    c.mul_s(0, 0, 1);
    c.swc1(0, at(c, S0, 0x28));
    c.lwc1(2, at(c, SP, 0x4e0));
    c.lwc1(1, at(c, SP, 0x4dc));
    c.lwc1(0, at(c, SP, 0x4d8));
    c.mul_s(1, 1, 2);
    c.lwc1(3, at(c, SP, 0x4e8));
    c.lwc1(2, at(c, SP, 0x4ec));
    c.mul_s(0, 0, 3);
    c.mul_s(1, 1, 2);
    c.sub_s(0, 0, 1);
    c.swc1(0, at(c, S0, 0x30));
    c.lwc1(2, at(c, SP, 0x4e0));
    c.lwc1(1, at(c, SP, 0x4ec));
    c.lwc1(0, at(c, SP, 0x4d8));
    c.mul_s(4, 4, 2);
    c.mul_s(0, 0, 1);
    c.add_s(4, 4, 0);
    c.swc1(4, at(c, S0, 0x34));
    c.lwc1(1, at(c, SP, 0x4e4));
    c.lwc1(0, at(c, SP, 0x4dc));
    c.mul_s(0, 0, 1);
    c.swc1(0, at(c, S0, 0x38));
    c.sw(at(c, S0, 0x3c), 0);
    c.lwc1(0, r(c, S1));
    c.swc1(0, at(c, S0, 0x40));
    c.lwc1(1, at(c, S1, 4));
    c.swc1(1, at(c, S0, 0x44));
    c.lwc1(2, at(c, S1, 8));
    c.swc1(5, at(c, S0, 0x4c));
    c.swc1(2, at(c, S0, 0x48));
}

/// Apply the original VU matrix product and its two temporary copies.
fn apply_yaw_offset(c: &mut Cpu) {
    c.lqc2(4, r(c, S3));
    c.lqc2(5, at(c, S3, 0x10));
    c.lqc2(6, at(c, S3, 0x20));
    c.lqc2(7, at(c, S3, 0x30));
    c.lqc2(8, r(c, FP));
    c.lqc2(9, at(c, FP, 0x10));
    c.lqc2(10, at(c, FP, 0x20));
    c.lqc2(11, at(c, FP, 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.vu0().mula(0b1111, 4, Src::Bc(9, X));
    c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
    c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
    c.vu0().mula(0b1111, 4, Src::Bc(10, X));
    c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
    c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
    c.vu0().mula(0b1111, 4, Src::Bc(11, X));
    c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
    c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
    c.sqc2(12, r(c, S7));
    c.sqc2(13, at(c, S7, 0x10));
    c.sqc2(14, at(c, S7, 0x20));
    c.sqc2(15, at(c, S7, 0x30));
    c.lqc2(1, r(c, S7));
    c.lqc2(2, at(c, S7, 0x10));
    c.lqc2(3, at(c, S7, 0x20));
    c.lqc2(4, at(c, S7, 0x30));
    c.sqc2(1, r(c, S6));
    c.sqc2(2, at(c, S6, 0x10));
    c.sqc2(3, at(c, S6, 0x20));
    c.sqc2(4, at(c, S6, 0x30));
    c.lqc2(1, r(c, S6));
    c.lqc2(2, at(c, S6, 0x10));
    c.lqc2(3, at(c, S6, 0x20));
    c.lqc2(4, at(c, S6, 0x30));
    c.sqc2(1, r(c, S3));
    c.sqc2(2, at(c, S3, 0x10));
    c.sqc2(3, at(c, S3, 0x20));
    c.sqc2(4, at(c, S3, 0x30));
}

/// `void MultiSplineModifier_UpdateNodes(MultiSplineModifier* self)`.
///
/// Evenly space count copies at currentDistance+i*length/count; wrap each
/// sample once into the path interval. Distances, total length and the path's
/// segment starts must use the same units. This routine does not normalize a
/// sample to [0, 1]: the segment's polynomial supplies that conversion.
/// Evaluate position/tangent/curvature into each 0x60-byte element.
/// Modes 0/1/2/3 mean no tangent orientation,
/// pitch only, yaw only, or both; roll stays zero. Fill each instance's
/// matrix, apply optional yaw offset, then clear dirty+30.
/// This routine does not allocate, create instances or scale model geometry.
pub fn multi_spline_modifier_update_nodes(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 29;
                    Frame::open(c, FRAME_BYTES);
                    c.g[V0][0] = 1;
                    for (reg, offset) in [(S2, 0x560), (S0, 0x580)] {
                        frame.save(c, reg, offset);
                    }
                    c.g[S2][0] = c.g[A0][0];
                    for (reg, offset) in [
                        (S1, 0x570),
                        (S3, 0x550),
                        (S4, 0x540),
                        (S5, 0x530),
                        (S6, 0x520),
                        (S7, 0x510),
                        (FP, 0x500),
                    ] {
                        frame.save(c, reg, offset);
                    }
                    frame.save_ra(c, 0x4f0);
                    for (reg, offset) in [
                        (26, 0x5c0),
                        (25, 0x5b8),
                        (24, 0x5b0),
                        (23, 0x5a8),
                        (22, 0x5a0),
                        (21, 0x598),
                        (20, 0x590),
                    ] {
                        frame.save_fpr(c, reg, offset);
                    }
                    c.g[V1][0] = c.lw(at(c, S2, COUNT));
                    c.lwc1(2, at(c, S2, LENGTH));
                    c.mtc1(V1, 1);
                    c.cvt_s_w(1, 1);
                    c.lwc1(0, at(c, S2, DISTANCE));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.div_s(2, 2, 1);
                    let single_or_empty = c.g[V0][0] == 0;
                    c.swc1(0, r(c, SP));
                    if single_or_empty {
                        pc = WRAP_ENTRY;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx(r(c, V1).wrapping_sub(1));
                    c.g[V1][0] = sx(at(c, SP, 4));
                    pc = DISTANCES;
                }
                DISTANCES => {
                    c.cycles += 6;
                    c.lwc1(0, r(c, V1).wrapping_sub(4));
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
                    c.add_s(0, 0, 2);
                    c.swc1(0, r(c, V1));
                    let more = c.g[V0][0] != 0;
                    c.g[V1][0] = sx(at(c, V1, 4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, DISTANCES);
                        }
                    } else {
                        pc = WRAP_ENTRY
                    }
                }
                WRAP_ENTRY => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, S2, COUNT));
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = sx(at(c, S2, COUNT));
                    if empty {
                        pc = YAW_SETUP;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.g[V0][0];
                    c.mtc1(ZERO, 2);
                    c.g[V0][0] = c.g[SP][0];
                    pc = WRAP_DISTANCE;
                }
                WRAP_DISTANCE => {
                    c.cycles += 5;
                    c.lwc1(1, r(c, V0));
                    c.c_lt_s(1, 2);
                    let negative = c.fcc();
                    c.lwc1(0, at(c, S2, LENGTH));
                    if negative {
                        c.cycles += 2;
                        c.add_s(0, 1, 0);
                        c.cycles += 2;
                        c.swc1(0, r(c, V0));
                        c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
                    } else {
                        c.cycles += 4;
                        c.c_le_s(0, 1);
                        if c.fcc() {
                            c.cycles += 1;
                            c.sub_s(0, 1, 0);
                            c.cycles += 2;
                            c.swc1(0, r(c, V0));
                            c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
                        } else {
                            c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
                        }
                    }
                    c.cycles += 2;
                    let more = c.g[V1][0] != 0;
                    c.g[V0][0] = sx(at(c, V0, 4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, WRAP_DISTANCE);
                        }
                    } else {
                        pc = YAW_SETUP
                    }
                }
                YAW_SETUP => {
                    c.cycles += 6;
                    c.lwc1(0, at(c, S2, YAW_OFFSET));
                    c.mtc1(ZERO, 20);
                    c.c_eq_s(0, 20);
                    if c.fcc() {
                        c.g[V0][0] = c.lw(at(c, S2, COUNT));
                        pc = ELEMENTS;
                        continue;
                    }
                    c.cycles += 55;
                    c.lwc1(12, at(c, A0, 8));
                    c.g[A1][0] = sx(at(c, SP, 0x4d4));
                    c.g[A0][0] = sx(at(c, SP, 0x4d0));
                    c.g[S0][0] = 0x50_0000;
                    call_ptr(c, SIN_COS, AFTER_YAW_OFFSET)?;
                    pc = AFTER_YAW_OFFSET;
                }
                AFTER_YAW_OFFSET => {
                    yaw_offset_matrix(c);
                    pc = ELEMENTS;
                }
                ELEMENTS => {
                    c.cycles += 3;
                    c.g[S4][0] = 0;
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[S1][0] = c.lw(at(c, S2, ELEMENT_TABLE));
                    if empty {
                        pc = FINISH;
                        continue;
                    }
                    c.cycles += 6;
                    c.lwc1(25, r(c, GP).wrapping_sub(0x2a24));
                    c.mov_s(26, 20);
                    c.g[S7][0] = sx(at(c, SP, 0x490));
                    c.g[FP][0] = sx(at(c, SP, 0x400));
                    c.g[S6][0] = sx(at(c, SP, 0x450));
                    c.g[S5][0] = c.g[SP][0];
                    pc = EVALUATE;
                }
                EVALUATE => {
                    c.cycles += 19;
                    c.lwc1(12, r(c, S5));
                    c.g[A0][0] = sx(at(c, S2, FOLLOW));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = sx(at(c, S1, 0x10));
                    c.mtc1(ZERO, 23);
                    c.g[A3][0] = sx(at(c, S1, 0x20));
                    EVAL_PATH.call(c, AFTER_EVALUATE)?;
                    pc = AFTER_EVALUATE;
                }
                AFTER_EVALUATE => {
                    c.g[S0][0] = sx(r(c, S4) << 2);
                    c.lwc1(21, at(c, S1, 0x10));
                    c.lwc1(20, at(c, S1, 0x14));
                    c.mul_s(0, 21, 21);
                    c.mul_s(1, 20, 20);
                    c.swc1(0, at(c, S1, 0x50));
                    c.swc1(1, at(c, S1, 0x54));
                    c.g[V0][0] = c.lw(at(c, S2, MODE));
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(2));
                    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                    let yaw = c.g[V0][0] != 0;
                    c.mov_s(24, 23);
                    if !yaw {
                        pc = PITCH;
                        continue;
                    }
                    c.cycles += 3;
                    c.c_eq_s(21, 23);
                    if !c.fcc() {
                        c.cycles += 7;
                        c.div_s(12, 20, 21);
                        call_ptr(c, ATAN, AFTER_YAW_ATAN)?;
                        pc = AFTER_YAW_ATAN;
                        continue;
                    }
                    c.cycles += 4;
                    c.c_eq_s(20, 23);
                    let zero_y = c.fcc();
                    c.mov_s(0, 20);
                    if zero_y {
                        c.cycles += 1;
                        c.sub_s(23, 24, 0);
                    } else {
                        c.cycles += 4;
                        c.c_le_s(23, 20);
                        c.lwc1(0, r(c, GP).wrapping_sub(0x2a20));
                        if !c.fcc() {
                            c.sub_s(23, 24, 0)
                        } else {
                            c.cycles += 3;
                            c.lwc1(0, r(c, GP).wrapping_sub(0x2a1c));
                            c.sub_s(23, 24, 0);
                        }
                    }
                    pc = PITCH;
                }
                AFTER_YAW_ATAN => {
                    c.c_lt_s(21, 23);
                    if !c.fcc() {
                        c.sub_s(23, 24, 0)
                    } else {
                        c.cycles += 4;
                        c.c_lt_s(23, 20);
                        if !c.fcc() {
                            c.sub_s(0, 0, 25)
                        } else {
                            c.cycles += 1;
                            c.add_s(0, 0, 25)
                        }
                        c.cycles += 1;
                        c.sub_s(23, 24, 0);
                    }
                    pc = PITCH;
                }
                PITCH => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(at(c, S2, MODE));
                    c.g[V0][0] = 1;
                    let pitch_only = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 3;
                    if !pitch_only {
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = c.lw(at(c, S2, INSTANCES));
                            pc = MATRIX;
                            continue;
                        }
                    }
                    c.cycles += 9;
                    c.lwc1(0, at(c, S1, 0x50));
                    c.lwc1(1, at(c, S1, 0x54));
                    c.mtc1(ZERO, 22);
                    c.add_s(0, 0, 1);
                    c.lwc1(21, at(c, S1, 0x18));
                    c.sqrt_s(20, 0);
                    c.c_eq_s(20, 22);
                    if !c.fcc() {
                        c.cycles += 7;
                        c.div_s(12, 21, 20);
                        call_ptr(c, ATAN, AFTER_PITCH_ATAN)?;
                        pc = AFTER_PITCH_ATAN;
                        continue;
                    }
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let zero_z = c.fcc();
                    c.mov_s(0, 21);
                    if zero_z {
                        c.cycles += 1;
                        c.add_s(24, 24, 0);
                    } else {
                        c.cycles += 4;
                        c.c_le_s(22, 21);
                        c.lwc1(0, r(c, GP).wrapping_sub(0x2a18));
                        if !c.fcc() {
                            c.add_s(24, 24, 0)
                        } else {
                            c.cycles += 3;
                            c.lwc1(0, r(c, GP).wrapping_sub(0x2a14));
                            c.add_s(24, 24, 0);
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, S2, INSTANCES));
                    pc = MATRIX;
                }
                AFTER_PITCH_ATAN => {
                    c.c_lt_s(20, 22);
                    if !c.fcc() {
                        c.add_s(24, 24, 0)
                    } else {
                        c.cycles += 4;
                        c.c_lt_s(22, 21);
                        if !c.fcc() {
                            c.sub_s(0, 0, 25)
                        } else {
                            c.cycles += 1;
                            c.add_s(0, 0, 25)
                        }
                        c.cycles += 1;
                        c.add_s(24, 24, 0);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, S2, INSTANCES));
                    pc = MATRIX;
                }
                MATRIX => {
                    c.cycles += 89;
                    c.g[A0][0] = sx(at(c, SP, 0x4d8));
                    c.swc1(24, at(c, SP, 0x444));
                    c.g[A1][0] = sx(at(c, SP, 0x4dc));
                    c.swc1(23, at(c, SP, 0x448));
                    c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
                    c.swc1(26, at(c, SP, 0x440));
                    c.mov_s(12, 26);
                    c.g[S0][0] = c.lw(r(c, V0));
                    call_ptr(c, SIN_COS, AFTER_ROLL)?;
                    pc = AFTER_ROLL;
                }
                AFTER_ROLL => {
                    c.lwc1(12, at(c, SP, 0x444));
                    c.g[A0][0] = sx(at(c, SP, 0x4e0));
                    c.g[A1][0] = sx(at(c, SP, 0x4e4));
                    c.g[S3][0] = sx(at(c, S0, 0x10));
                    call_ptr(c, SIN_COS, AFTER_PITCH)?;
                    pc = AFTER_PITCH;
                }
                AFTER_PITCH => {
                    c.lwc1(12, at(c, SP, 0x448));
                    c.g[A0][0] = sx(at(c, SP, 0x4e8));
                    c.g[A1][0] = sx(at(c, SP, 0x4ec));
                    call_ptr(c, SIN_COS, AFTER_YAW)?;
                    pc = AFTER_YAW;
                }
                AFTER_YAW => {
                    instance_matrix(c);
                    c.lwc1(0, at(c, S2, YAW_OFFSET));
                    c.c_eq_s(0, 26);
                    if c.fcc() {
                        c.g[V0][0] = c.lw(at(c, S2, COUNT));
                    } else {
                        c.cycles += 45;
                        apply_yaw_offset(c);
                        c.g[V0][0] = c.lw(at(c, S2, COUNT));
                    }
                    pc = NEXT;
                }
                NEXT => {
                    c.cycles += 5;
                    c.g[S4][0] = sx(r(c, S4).wrapping_add(1));
                    c.g[S5][0] = sx(at(c, S5, 4));
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx(at(c, S1, 0x60));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, EVALUATE);
                        }
                        pc = EVALUATE;
                    } else {
                        pc = FINISH
                    }
                }
                FINISH => {
                    c.cycles += 20;
                    c.sw(at(c, S2, DIRTY), 0);
                    for (reg, offset) in [
                        (S0, 0x580),
                        (S1, 0x570),
                        (S2, 0x560),
                        (S3, 0x550),
                        (S4, 0x540),
                        (S5, 0x530),
                        (S6, 0x520),
                        (S7, 0x510),
                        (FP, 0x500),
                    ] {
                        frame.restore(c, reg, offset);
                    }
                    frame.restore_ra(c, 0x4f0);
                    for (reg, offset) in [
                        (26, 0x5c0),
                        (25, 0x5b8),
                        (24, 0x5b0),
                        (23, 0x5a8),
                        (22, 0x5a0),
                        (21, 0x598),
                        (20, 0x590),
                    ] {
                        frame.restore_fpr(c, reg, offset);
                    }
                    frame.close(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, fill, put_vec, scramble, Rng, DATA, GAME_GP};

    /// Keep callees real: a single linear segment gives exactly the selected
    /// tangent, including the zero/axis-aligned cases in the Euler branches.
    /// The polynomial maps length-unit distance to a parameter; its basis
    /// provides position and tangent without touching world or heap state.
    #[test]
    fn multi_spline_modifier_update_nodes_matches() {
        check(
            ssx3_ee::s11::f_0035ac20,
            multi_spline_modifier_update_nodes,
            180,
            &|c, seed| {
                elf_image(c); // SinCos/Atan constants, tables, and yaw axis.
                let mut random = Rng(0x35ac_20d1_57a0_0000 ^ seed as u64);
                scramble(c, &mut random);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                c.pc = ENTRY;
                let stack = r(c, SP).wrapping_sub(FRAME_BYTES);
                fill(c, &mut random, stack, FRAME_BYTES / 4);

                let elements = DATA + 0x1000;
                let instances = DATA + 0x2000;
                let curve = DATA + 0x6000;
                let count = if seed % 19 == 0 {
                    0
                } else if seed % 23 == 0 {
                    u32::MAX
                } else if seed % 17 == 0 {
                    15
                } else if seed % 7 == 0 {
                    1
                } else {
                    3
                };
                c.bus.write32(DATA + COUNT, count);
                c.bus.write32(DATA + MODE, seed % 5);
                c.bus.write32(
                    DATA + YAW_OFFSET,
                    if (seed / 45) % 2 == 0 { 0 } else { 0x3fc9_0fdb },
                );
                // Cover both sides of the one-wrap correction, exact end
                // points, and distances beyond its own one-length domain.
                let distance = [-32.0f32, -16.0, -0.25, 0.0, 8.0, 16.0, 31.75, 40.0]
                    [(seed as usize / 3) % 8];
                c.bus.write32(DATA + DISTANCE, distance.to_bits());
                c.bus.write32(DATA + LENGTH, 16.0f32.to_bits());
                c.bus.write32(DATA + DIRTY, random.next_u32() | 1);
                c.bus.write32(DATA + ELEMENT_TABLE, elements);
                c.bus.write32(DATA + INSTANCES, instances);
                c.bus.write32(DATA + FOLLOW, 0x3731);
                c.bus.write32(DATA + FOLLOW + 4, 0);
                c.bus.write32(DATA + FOLLOW + 8, curve);
                for i in 0..15 {
                    fill(c, &mut random, elements + i * 0x60, 0x60 / 4);
                    let instance = DATA + 0x3000 + i * 0xb0;
                    fill(c, &mut random, instance, 0xb0 / 4);
                    c.bus.write32(instances + 4 * i, instance);
                }

                let tangents = [
                    [0.0f32, 0.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, -1.0, 0.0],
                    [1.0, 1.0, 1.0],
                    [1.0, -1.0, -1.0],
                    [-1.0, 1.0, 1.0],
                    [-1.0, -1.0, -1.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0, -1.0],
                ];
                let tangent = tangents[((seed / 5) % 9) as usize];
                put_vec(c, curve + 0x10, [0; 4]);
                put_vec(c, curve + 0x20, [0; 4]);
                put_vec(
                    c,
                    curve + 0x30,
                    [tangent[0].to_bits(), tangent[1].to_bits(), tangent[2].to_bits(), 0],
                );
                put_vec(
                    c,
                    curve + 0x40,
                    [37.0f32.to_bits(), (-11.0f32).to_bits(), 23.0f32.to_bits(), 1.0f32.to_bits()],
                );
                for (offset, value) in [(0x50, 0), (0x54, 0), (0x58, 0x3d80_0000), (0x5c, 0)] {
                    c.bus.write32(curve + offset, value);
                }
                c.bus.write32(curve + 0x60, 0);
                c.bus.write32(curve + 0x64, 0);
                c.bus.write32(curve + 0x84, 0);
            },
        );
    }
}
