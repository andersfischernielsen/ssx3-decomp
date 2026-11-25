//! Staging module oc637 (wave 234, list oc637): hand-written replacements, each
//! with its differential test against the generated body. Moved into the
//! source-tree units by `tools/reorg_modules.py apply oc637`.
//!
//! The bodies are transcriptions of the generated code: the same stores, loads,
//! cycle charges and register writes in the same order, with each call made
//! through `call_ptr` (the unwind check) and the generated labelled blocks kept.

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
    (0x0016_ad98, algorithmic_script_camera_algo_reset_for_rider, "AlgorithmicScriptCameraAlgo_ResetForRider"),
    (0x0017_1a20, camera_ellipse_bound_obj_read, "CameraEllipseBoundObj_Read"),
    (0x0017_1b10, camera_box_bound_obj_read, "CameraBoxBoundObj_Read"),
    (0x0017_1c00, camera_line_bound_obj_read, "CameraLineBoundObj_Read"),
    (0x0017_1d58, camera_trigger_read_spline, "CameraTrigger_ReadSpline"),
];

#[allow(unused_labels, unreachable_code, unused_assignments)]
/// `AlgorithmicScriptCameraAlgo_ResetForRider(algo)` (0x16ad98): the scripted
/// camera's per-rider reset. Reads the camera entity and the rider, places the
/// target at the rider plus the lead offsets, builds the look direction from the
/// optimal pitch and heading (the atan2 path when the header flag is 0), sets the
/// position `k` units behind the target and updates both jitterers.
pub fn algorithmic_script_camera_algo_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        'b_16ae50: {
        c.cycles += 41;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S1][0] = c.g[A0][0];
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0xe8));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lwc1(24, (c.g[GP][0] as u32).wrapping_sub(0x64f8));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[RA][0] = sx(0x0016addc);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0027c6a0, 0x0016addc)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0016adec);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c));
        call_ptr(c, 0x0027c6f8, 0x0016adec)?;
        c.lwc1(0, (c.g[S2][0] as u32));
        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x64));
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
        c.mul_s(2, 2, 24);
        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x60));
        c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.mul_s(1, 1, 24);
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
        c.swc1(2, (c.g[S1][0] as u32));
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x70));
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[RA][0] = sx(0x0016ae30);
        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x1d));
        call_ptr(c, 0x00279f18, 0x0016ae30)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0u64 {
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x780));
        break 'b_16ae50;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[S0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x780));
        break 'b_16ae50;
        }
        }
        'b_16b070: {
        'b_16b060: {
        c.cycles += 48;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c0));
        c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x140));
        c.sq(2, (c.g[SP][0] as u32));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c0));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0016ae70);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0016ae70)?;
        c.lq(3, (c.g[S0][0] as u32).wrapping_add(0x1b0));
        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4c));
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.mul_s(0, 0, 1);
        c.mfc1(4, 0);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lqc2(3, (c.g[V0][0] as u32));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x50));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lq(2, (c.g[S0][0] as u32).wrapping_add(0x1c0));
        c.mul_s(0, 0, 1);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.mfc1(3, 0);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x20));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x1e));
        if c.g[V1][0] != 0u64 {
        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x58));
        break 'b_16b060;
        } else {
        'b_16afd0: {
        'b_16afc8: {
        'b_16afc4: {
        'b_16af84: {
        c.cycles += 18;
        c.lq(2, (c.g[S0][0] as u32).wrapping_add(0x1b0));
        c.mtc1(0, 22);
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x58));
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.mul_s(23, 2, 24);
        c.mul_s(2, 0, 0);
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.mul_s(3, 1, 1);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
        c.add_s(2, 2, 3);
        c.sqrt_s(20, 2);
        c.c_eq_s(20, 22);
        let t = !c.fcc();
        
        if t {
        break 'b_16af84;
        } else {
        c.cycles += 4;
        c.c_eq_s(21, 22);
        let t = c.fcc();
        c.mov_s(0, 21);
        if t {
        break 'b_16afc4;
        } else {
        c.cycles += 4;
        c.c_le_s(22, 21);
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64f4));
        let t = !c.fcc();
        c.lwc1(21, (c.g[S0][0] as u32));
        if t {
        break 'b_16afc8;
        } else {
        c.cycles += 3;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64f0));
        let t = true;
        c.sub_s(23, 0, 23);
        if t {
        break 'b_16afd0;
        } else {
        break 'b_16af84;
        }
        }
        }
        }
        }
        'b_16afc0: {
        c.cycles += 7;
        c.div_s(12, 21, 20);
        c.g[RA][0] = sx(0x0016af90);
        call_ptr(c, 0x0031c228, 0x0016af90)?;
        c.c_lt_s(20, 22);
        let t = !c.fcc();
        c.mov_s(12, 0);
        if t {
        break 'b_16afc0;
        } else {
        'b_16afb8: {
        c.cycles += 3;
        c.c_lt_s(22, 21);
        let t = !c.fcc();
        
        if t {
        break 'b_16afb8;
        } else {
        c.cycles += 3;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64ec));
        let t = true;
        c.add_s(12, 12, 0);
        if t {
        break 'b_16afc0;
        } else {
        break 'b_16afb8;
        }
        }
        }
        c.cycles += 2;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64e8));
        c.sub_s(12, 12, 0);
        break 'b_16afc0;
        }
        }
        c.cycles += 1;
        c.mov_s(0, 12);
        break 'b_16afc4;
        }
        c.cycles += 1;
        c.lwc1(21, (c.g[S0][0] as u32));
        break 'b_16afc8;
        }
        c.cycles += 2;
        c.sub_s(23, 0, 23);
        c.mtc1(0, 22);
        break 'b_16afd0;
        }
        'b_16b058: {
        'b_16b054: {
        'b_16b014: {
        c.cycles += 6;
        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x5c));
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64e4));
        c.c_eq_s(21, 22);
        c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0x4));
        let t = !c.fcc();
        c.mul_s(24, 1, 0);
        if t {
        break 'b_16b014;
        } else {
        c.cycles += 4;
        c.c_eq_s(20, 22);
        let t = c.fcc();
        c.mov_s(0, 20);
        if t {
        break 'b_16b054;
        } else {
        c.cycles += 4;
        c.c_le_s(22, 20);
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64e0));
        if !c.fcc() {
        c.add_s(1, 24, 0);
        break 'b_16b058;
        } else {
        c.cycles += 3;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64dc));
        let t = true;
        c.add_s(1, 24, 0);
        if t {
        break 'b_16b058;
        } else {
        break 'b_16b014;
        }
        }
        }
        }
        }
        'b_16b050: {
        c.cycles += 7;
        c.div_s(12, 20, 21);
        c.g[RA][0] = sx(0x0016b020);
        call_ptr(c, 0x0031c228, 0x0016b020)?;
        c.c_lt_s(21, 22);
        let t = !c.fcc();
        c.mov_s(12, 0);
        if t {
        break 'b_16b050;
        } else {
        'b_16b048: {
        c.cycles += 3;
        c.c_lt_s(22, 20);
        let t = !c.fcc();
        
        if t {
        break 'b_16b048;
        } else {
        c.cycles += 3;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64d8));
        let t = true;
        c.add_s(12, 12, 0);
        if t {
        break 'b_16b050;
        } else {
        break 'b_16b048;
        }
        }
        }
        c.cycles += 2;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64d4));
        c.sub_s(12, 12, 0);
        break 'b_16b050;
        }
        }
        c.cycles += 1;
        c.mov_s(0, 12);
        break 'b_16b054;
        }
        c.cycles += 1;
        c.add_s(1, 24, 0);
        break 'b_16b058;
        }
        c.cycles += 2;
        let t = true;
        c.lwc1(20, (c.g[S1][0] as u32).wrapping_add(0x54));
        if t {
        break 'b_16b070;
        } else {
        break 'b_16b060;
        }
        }
        }
        c.cycles += 4;
        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x5c));
        c.mul_s(23, 0, 24);
        c.lwc1(20, (c.g[S1][0] as u32).wrapping_add(0x54));
        c.mul_s(1, 1, 24);
        break 'b_16b070;
        }
        c.cycles += 55;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x84));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x70));
        c.lwc1(12, (c.g[V0][0] as u32).wrapping_add(0x78));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x74));
        c.add_s(12, 1, 12);
        c.g[RA][0] = sx(0x0016b094);
        c.add_s(23, 23, 0);
        call_ptr(c, 0x0031be50, 0x0016b094)?;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x88));
        c.mov_s(12, 23);
        c.g[RA][0] = sx(0x0016b0a4);
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8c));
        call_ptr(c, 0x0031be50, 0x0016b0a4)?;
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8c));
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x84));
        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x88));
        c.mul_s(0, 2, 0);
        c.mul_s(2, 2, 3);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
        c.neg_s(1, 1);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.mfc1(3, 20);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x68));
        c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = sx(0x0016b114);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        call_ptr(c, 0x00160028, 0x0016b114)?;
        c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x6c));
        c.g[RA][0] = sx(0x0016b120);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00160130, 0x0016b120)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0xe8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.pc = t; return Ok(());
    });
}

#[allow(unused_labels, unreachable_code, unused_assignments)]
/// `int get_cCTBoundObjEllipse(reader, obj)` (0x171a20): reads an ellipse trigger
/// bound (position, scale, heading, pitch, roll) through the object's field
/// accessors and returns the sum of the reads. Sets `obj->Type = 0`.
pub fn camera_ellipse_bound_obj_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 59;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171a54);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171a54)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171a60);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00171a60)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171a78);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171a78)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171a84);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00171a84)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171a9c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171a9c)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171aa8);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171aa8)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171ac0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171ac0)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171acc);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171acc)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171ae4);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171ae4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171af0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171af0)?;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());
    });
}

#[allow(unused_labels, unreachable_code, unused_assignments)]
/// `int get_cCTBoundObjBox(reader, obj)` (0x171b10): reads a box trigger bound
/// (the same five fields as the ellipse). Sets `obj->Type = 1`.
pub fn camera_box_bound_obj_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 60;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0x1;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171b48);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171b48)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171b54);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00171b54)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171b6c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171b6c)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171b78);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00171b78)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171b90);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171b90)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171b9c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171b9c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171bb4);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171bb4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171bc0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171bc0)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171bd8);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171bd8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171be4);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171be4)?;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());
    });
}

#[allow(unused_labels, unreachable_code, unused_assignments)]
/// `int get_cCTBoundObjLine(reader, obj)` (0x171c00): reads a line trigger bound
/// (the five fields, then the two end points). Sets `obj->Type = 2`.
pub fn camera_line_bound_obj_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 68;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0x2;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171c38);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171c38)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171c44);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00171c44)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171c5c);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171c5c)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171c68);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00171c68)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171c80);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171c80)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171c8c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171c8c)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171ca4);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171ca4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171cb0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171cb0)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171cc8);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171cc8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171cd4);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171cd4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2c));
        c.g[RA][0] = sx(0x00171ce4);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        call_ptr(c, 0x002d25a0, 0x00171ce4)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171cf4);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x002d25a0, 0x00171cf4)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());
    });
}

#[allow(unused_labels, unreachable_code, unused_assignments)]
/// `void get_camspline(reader, out)` (0x171d58): allocates a spline trigger
/// (`str_Camera_Trigger_Spline`, 0x58 bytes), copies its template, stores it in
/// `*out` and reads the five object fields plus the four control points.
pub fn camera_trigger_read_spline(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 148;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[A2][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A3][0] = 0u64;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = 0x58;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x00171d8c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3e90));
        call_ptr(c, 0x00317d70, 0x00171d8c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[V0][0] = 0x460000;
        c.g[V1][0] = 0x500000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3c10));
        c.g[A3][0] = c.g[V1][0];
        c.sw((c.g[A1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
        c.g[T0][0] = 0xffffffffffffffff;
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = c.g[A1][0];
        c.swc1(0, (c.g[SP][0] as u32));
        c.g[A0][0] = 0x3;
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xf28));
        c.ldl(9, (c.g[T4][0] as u32).wrapping_add(0x7));
        c.ldr(9, (c.g[T4][0] as u32));
        c.g[T2][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x8));
        c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.sdr(9, (c.g[A1][0] as u32));
        c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T2][0]);
        c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x14), 0u64);
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(3, (c.g[SP][0] as u32));
        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(3, (c.g[A1][0] as u32).wrapping_add(0x1f));
        c.sdr(3, (c.g[A1][0] as u32).wrapping_add(0x18));
        c.sw((c.g[A1][0] as u32).wrapping_add(0x20), c.g[T1][0]);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf28));
        c.ldl(12, (c.g[A0][0] as u32).wrapping_add(0x7));
        c.ldr(12, (c.g[A0][0] as u32));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.sdl(12, (c.g[A2][0] as u32).wrapping_add(0x2f));
        c.sdr(12, (c.g[A2][0] as u32).wrapping_add(0x28));
        c.sw((c.g[A2][0] as u32).wrapping_add(0x30), c.g[V0][0]);
        c.g[T3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf28));
        c.ldl(8, (c.g[T3][0] as u32).wrapping_add(0x7));
        c.ldr(8, (c.g[T3][0] as u32));
        c.g[T1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x8));
        c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0x3b));
        c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x34));
        c.sw((c.g[A2][0] as u32).wrapping_add(0x3c), c.g[T1][0]);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf28));
        c.ldl(12, (c.g[A0][0] as u32).wrapping_add(0x7));
        c.ldr(12, (c.g[A0][0] as u32));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.sdl(12, (c.g[A2][0] as u32).wrapping_add(0x47));
        c.sdr(12, (c.g[A2][0] as u32).wrapping_add(0x40));
        c.sw((c.g[A2][0] as u32).wrapping_add(0x48), c.g[V0][0]);
        c.g[T3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf28));
        c.ldl(8, (c.g[T3][0] as u32).wrapping_add(0x7));
        c.ldr(8, (c.g[T3][0] as u32));
        c.g[T1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x8));
        c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0x53));
        c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x4c));
        c.sw((c.g[A2][0] as u32).wrapping_add(0x54), c.g[T1][0]);
        c.sw((c.g[S1][0] as u32), c.g[A1][0]);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171e90);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171e90)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00171e9c);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002d25a0, 0x00171e9c)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171eb8);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171eb8)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00171ec4);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002d25a0, 0x00171ec4)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171ee0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171ee0)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00171eec);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002d2510, 0x00171eec)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171f08);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171f08)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00171f14);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002d2510, 0x00171f14)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171f30);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00171f30)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00171f3c);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002d2510, 0x00171f3c)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[RA][0] = sx(0x00171f50);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x002d25a0, 0x00171f50)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[RA][0] = sx(0x00171f64);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x34));
        call_ptr(c, 0x002d25a0, 0x00171f64)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[RA][0] = sx(0x00171f78);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x002d25a0, 0x00171f78)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171f8c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4c));
        call_ptr(c, 0x002d25a0, 0x00171f8c)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fakes, fill, DATA, GAME_GP};
    use ssx3_ee_rt::cpu::reg::*;
    use std::sync::atomic::AtomicUsize;

    /// The bound readers' object (the reader's stream cursor at `READER`).
    pub(crate) const OC637_OBJ: u32 = DATA + 0x6000;
    pub(crate) const OC637_READER: u32 = DATA + 0x6100;
    pub(crate) const OC637_STREAM: u32 = DATA + 0x6200;
    pub(crate) const OC637_VT: u32 = DATA + 0x6400;
    /// The spline's `*out` cell.
    pub(crate) const OC637_OUT: u32 = DATA + 0x6800;
    /// The block `EA_MemAllocNamed` returns to the spline (gate on).
    pub(crate) const OC637_NEW_BLOCK: u32 = DATA + 0x21_0000;
    /// The fake allocator runs only when `OC637_GATE` holds `OC637_GATE_MAGIC`.
    pub(crate) const OC637_GATE: u32 = DATA + 0x20_5000;
    pub(crate) const OC637_GATE_MAGIC: u32 = 0x0637_c0de;
    /// The vtable the spline stores into its object (`0x45c3f0`, in the game image).
    pub(crate) const OC637_SPLINE_VT: u32 = 0x0045_c3f0;
    static OC637_NEW_PREV: AtomicUsize = AtomicUsize::new(0);

    /// The camera reset's objects: the algorithm, its script object, the camera
    /// entity, the rider (reached through the world), its anim and vtable.
    pub(crate) const OC637_ALG: u32 = DATA + 0x4000;
    pub(crate) const OC637_SCRIPT: u32 = DATA + 0x4400;
    pub(crate) const OC637_CAM: u32 = DATA + 0x4500;
    pub(crate) const OC637_ENT: u32 = DATA + 0x4600;
    pub(crate) const OC637_RIDER: u32 = DATA + 0x1800;
    pub(crate) const OC637_ANIM: u32 = DATA + 0x2000;
    pub(crate) const OC637_VT2: u32 = DATA + 0x2400;
    pub(crate) const OC637_WORLD: u32 = DATA + 0x2800;
    pub(crate) const OC637_W2: u32 = DATA + 0x2900;
    pub(crate) const OC637_W3: u32 = DATA + 0x2a00;

    /// Five accessor slots (the offsets the readers and the spline call), each a
    /// fake callee: the `miss` hook logs the call and returns a pointer.
    pub(crate) fn oc637_vtable(c: &mut Cpu, r: &mut crate::difftest::Rng, vt: u32) {
        for (i, off) in [0x08u32, 0x28, 0x10, 0x18, 0x20].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
    }

    /// The readers' fixture: the object with its accessor vtable, the reader whose
    /// first word is the stream cursor (the float and vec3 reads are real guest
    /// code), and random words in the stream.
    pub(crate) fn oc637_reader_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = fakes::machine(c, seed, 0x637_0001, 5);
        c.pc = entry;
        c.g[A0][0] = OC637_READER as u64;
        c.g[A1][0] = OC637_OBJ as u64;
        fill(c, &mut r, OC637_OBJ, 0x40);
        oc637_vtable(c, &mut r, OC637_VT);
        c.bus.write32(OC637_OBJ + 0x24, OC637_VT);
        c.bus.write32(OC637_READER, OC637_STREAM);
        fill(c, &mut r, OC637_STREAM, 0x40);
    }

    #[test]
    fn camera_ellipse_bound_obj_read_matches() {
        fakes::covered(ssx3_ee::s02::f_00171a20__generated, camera_ellipse_bound_obj_read, 200, &|c, seed| {
            oc637_reader_fixture(c, seed, 0x0017_1a20)
        });
    }

    #[test]
    fn camera_box_bound_obj_read_matches() {
        fakes::covered(ssx3_ee::s02::f_00171b10__generated, camera_box_bound_obj_read, 200, &|c, seed| {
            oc637_reader_fixture(c, seed, 0x0017_1b10)
        });
    }

    #[test]
    fn camera_line_bound_obj_read_matches() {
        fakes::covered(ssx3_ee::s02::f_00171c00__generated, camera_line_bound_obj_read, 200, &|c, seed| {
            oc637_reader_fixture(c, seed, 0x0017_1c00)
        });
    }

    /// The fake `EA_MemAllocNamed` for the spline's `new` (gate on: the block).
    fn oc637_fake_new(c: &mut Cpu) {
        if c.bus.read32(OC637_GATE) != OC637_GATE_MAGIC {
            return fakes::fall_back(c, &OC637_NEW_PREV, ssx3_ee::s10::f_00317d70__generated);
        }
        c.g[V0][0] = OC637_NEW_BLOCK as u64;
        ret(c);
    }

    fn oc637_install_new() {
        fakes::hook(
            std::ptr::addr_of_mut!(ssx3_ee::s10::OVERRIDE_00317D70),
            oc637_fake_new,
            &OC637_NEW_PREV,
        );
    }

    /// The spline fixture: the reader stream, the `*out` cell, and the vtable at
    /// the address the body stores (its slots are the fake accessors).
    pub(crate) fn oc637_spline_fixture(c: &mut Cpu, seed: u32) {
        oc637_install_new();
        let mut r = fakes::machine(c, seed, 0x637_0002, 5);
        c.pc = 0x0017_1d58;
        c.bus.write32(OC637_GATE, OC637_GATE_MAGIC);
        c.g[A0][0] = OC637_READER as u64;
        c.g[A1][0] = OC637_OUT as u64;
        c.bus.write32(OC637_OUT, r.next_u32());
        c.bus.write32(OC637_READER, OC637_STREAM);
        fill(c, &mut r, OC637_STREAM, 0x40);
        fill(c, &mut r, OC637_NEW_BLOCK, 0x20);
        oc637_vtable(c, &mut r, OC637_SPLINE_VT);
    }

    #[test]
    fn camera_trigger_read_spline_matches() {
        fakes::covered(ssx3_ee::s02::f_00171d58__generated, camera_trigger_read_spline, 200, &|c, seed| {
            oc637_spline_fixture(c, seed)
        });
    }

    /// The camera reset's fixture: the algorithm (A0), the chain algorithm ->
    /// script object -> camera entity, the world (`gp-0x848`) -> rider, the
    /// rider's anim and object vtable. The script engine (`gp-0x84c`) stays zero,
    /// so the rider comes from the world.
    pub(crate) fn oc637_reset_fixture(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x637_0003, 6);
        c.pc = 0x0016_ad98;
        c.g[A0][0] = OC637_ALG as u64;
        fill(c, &mut r, OC637_ALG, 0x80);
        fill(c, &mut r, OC637_SCRIPT, 0x40);
        fill(c, &mut r, OC637_CAM, 0x40);
        fill(c, &mut r, OC637_ENT, 0x80);
        fill(c, &mut r, OC637_RIDER, 0x200);
        fill(c, &mut r, OC637_ANIM, 0x80);
        fill(c, &mut r, OC637_VT2, 0x80);
        fill(c, &mut r, OC637_WORLD, 0x40);
        fill(c, &mut r, OC637_W2, 0x40);
        fill(c, &mut r, OC637_W3, 0x40);
        oc637_vtable(c, &mut r, OC637_VT2);
        c.bus.write32(OC637_ALG + 0x14, OC637_SCRIPT);
        c.bus.write32(OC637_SCRIPT + 0x14, OC637_CAM);
        c.bus.write32(OC637_CAM + 0x8, OC637_ENT);
        c.bus.write32(GAME_GP - 0x84c, 0);
        c.bus.write32(GAME_GP - 0x848, OC637_WORLD);
        c.bus.write32(OC637_WORLD + 0x84, OC637_W2);
        c.bus.write32(OC637_W2 + 0xc, OC637_W3);
        c.bus.write32(OC637_W3 + 0x28, OC637_RIDER);
        c.bus.write32(OC637_RIDER + 0x780, OC637_ANIM);
        c.bus.write32(OC637_RIDER + 0x6c0, OC637_VT2);
    }

    #[test]
    fn algorithmic_script_camera_algo_reset_for_rider_matches() {
        fakes::covered(
            ssx3_ee::s02::f_0016ad98__generated,
            algorithmic_script_camera_algo_reset_for_rider,
            200,
            &|c, seed| oc637_reset_fixture(c, seed),
        );
    }
}
