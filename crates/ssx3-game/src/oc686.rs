//! Staging module oc686 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc686`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0034_2538, anim_teeter_v43, "AnimTeeter_v43"),
    (0x0034_3130, conveyor_v42, "Conveyor_v42"),
    (0x0034_3588, conveyor_render, "Conveyor_Render"),
    (
        0x0034_4aa0,
        crowd_man3d_group_build_matrices,
        "CrowdMan3d_GroupBuildMatrices",
    ),
    (
        0x0034_8c48,
        flex_rail_unregister_space_node,
        "FlexRail_UnregisterSpaceNode",
    ),
    (0x0034_8d98, float_rail_ctor, "FloatRail_ctor"),
    (
        0x0034_9220,
        float_rail_reset_pos_from_instance,
        "FloatRail_ResetPosFromInstance",
    ),
    (0x0034_9b48, spring_rail_ctor, "SpringRail_ctor"),
    (0x0034_9db0, spring_rail_ctor_2, "SpringRail_ctor_2"),
];

/// `void AnimTeeter_v43(AnimTeeter* self, Hit* hit, Vec4* force)` (0x342538,
/// `cAnimTeeterNode_applyForce`): torque from a force applied to a hit bone;
/// when the hit's mesh index (+0x5c) is non-negative and the model's bone
/// table entry has an animation, asks the object (vtable slot 0xe8/0xec) for
/// the bone matrix, and if the force is longer than 100 builds the lever arm
/// from the hit position and adds `dot(yaxis * bone, lever x force)` to
/// the torque sum.
pub fn anim_teeter_v43(c: &mut Cpu) {
    guest(c, |c| {
        'b_342700: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S1][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
            let t = (c.g[A3][0] as i64) < 0;
            c.g[S2][0] = c.g[A2][0];
            if t {
                break 'b_342700;
            } else {
                c.cycles += 9;
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x44));
                c.g[A1][0] = sx((c.g[A3][0] as u32) << 4);
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.g[A3][0];
                if t {
                    break 'b_342700;
                } else {
                    c.cycles += 22;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xe8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xec));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00342598);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00342598)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[AT][0] = 0x42c80000;
                    c.mtc1(1, 1);
                    c.lqc2(3, (c.g[S2][0] as u32));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(2, 22);
                    c.mtc1(2, 0);
                    c.c_lt_s(1, 0);
                    let t = !c.fcc();
                    c.g[A0][0] = c.g[V1][0];
                    if t {
                        break 'b_342700;
                    } else {
                        'b_342638: {
                            c.cycles += 13;
                            c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x30));
                            c.mtc1(0, 1);
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.lqc2(3, (c.g[S1][0] as u32));
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.c_lt_s(0, 1);
                            let t = !c.fcc();
                            c.sq(2, (c.g[SP][0] as u32));
                            if t {
                                break 'b_342638;
                            } else {
                                c.cycles += 10;
                                c.g[AT][0] = 0xffffffffbf800000;
                                c.mtc1(1, 0);
                                c.mfc1(3, 0);
                                c.lqc2(4, (c.g[SP][0] as u32));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.sq(2, (c.g[SP][0] as u32));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                break 'b_342638;
                            }
                        }
                        'b_34267c: {
                            c.cycles += 6;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32
                                >> ((c.g[V1][0] as u32) & 31))
                                as u32);
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x500000;
                            if t {
                                break 'b_34267c;
                            } else {
                                c.cycles += 11;
                                c.g[AT][0] = 0xffffffffbf800000;
                                c.mtc1(1, 0);
                                c.mfc1(3, 0);
                                c.lqc2(4, (c.g[SP][0] as u32));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.sq(2, (c.g[SP][0] as u32));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = 0x500000;
                                break 'b_34267c;
                            }
                        }
                        c.cycles += 33;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xeb0));
                        c.lqc2(8, (c.g[V0][0] as u32));
                        c.lqc2(4, (c.g[A0][0] as u32));
                        c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                        c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                        c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                        c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lqc2(4, (c.g[SP][0] as u32));
                        c.lqc2(5, (c.g[S2][0] as u32));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                        c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lwc1(0, (c.g[S0][0] as u32));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(3, 4);
                        c.mtc1(3, 1);
                        c.add_s(0, 0, 1);
                        c.swc1(0, (c.g[S0][0] as u32));
                        break 'b_342700;
                    }
                }
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `void Conveyor_v42(Conveyor* self, Hit* hit)` (0x343130,
/// `cConveyorNode_updateIntersectInfo`): for a hit on a conveyor bone, builds
/// the bone's world matrix (bone chain times the object matrix from vtable
/// vtable +0xc0/0xc4) and adds the push as a torque (flagged bones) or impulse to the
/// hit's velocity and angular velocity. Resumable body: a resume at any
/// other `pc` goes to `__entries` (the orchestrator switches it to
/// `f_00343130__generated` at install).
pub fn conveyor_v42(c: &mut Cpu) {
    if c.pc != 0x00343130 {
        return ee::s11::f_00343130__generated(c);
    }
    guest(c, |c| {
        'b_3434d8: {
            'b_3434d4: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[S0][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                let t = (c.g[V0][0] as i64) < 0;
                c.g[S1][0] = c.g[A0][0];
                if t {
                    break 'b_3434d4;
                } else {
                    'b_3432a4: {
                        c.cycles += 30;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x00343164);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00343164)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        c.g[T4][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x80));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                        c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        c.lqc2(1, (c.g[V1][0] as u32));
                        c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                        c.sqc2(1, (c.g[SP][0] as u32));
                        c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                        c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                        c.g[V1][0] = 0xffffffffffffffff;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                        let t = c.g[V0][0] == c.g[V1][0];
                        c.g[T3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_3432a4;
                        } else {
                            c.cycles += 6;
                            c.g[A2][0] = c.g[T0][0];
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = c.g[T2][0];
                            c.g[T1][0] = 0xffffffffffffffff;
                            c.g[V1][0] = sx((c.g[V0][0] as u32) << 4);
                            'l_3431e0: loop {
                                c.cycles += 49;
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                c.lqc2(4, (c.g[V0][0] as u32));
                                c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x10));
                                c.lqc2(6, (c.g[V0][0] as u32).wrapping_add(0x20));
                                c.lqc2(7, (c.g[V0][0] as u32).wrapping_add(0x30));
                                c.lqc2(8, (c.g[A0][0] as u32));
                                c.lqc2(9, (c.g[A0][0] as u32).wrapping_add(0x10));
                                c.lqc2(10, (c.g[A0][0] as u32).wrapping_add(0x20));
                                c.lqc2(11, (c.g[A0][0] as u32).wrapping_add(0x30));
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
                                c.sqc2(12, (c.g[A2][0] as u32));
                                c.sqc2(13, (c.g[A2][0] as u32).wrapping_add(0x10));
                                c.sqc2(14, (c.g[A2][0] as u32).wrapping_add(0x20));
                                c.sqc2(15, (c.g[A2][0] as u32).wrapping_add(0x30));
                                c.lqc2(1, (c.g[A2][0] as u32));
                                c.lqc2(2, (c.g[A2][0] as u32).wrapping_add(0x10));
                                c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
                                c.lqc2(4, (c.g[A2][0] as u32).wrapping_add(0x30));
                                c.sqc2(1, (c.g[A1][0] as u32));
                                c.sqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                                c.sqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
                                c.sqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
                                c.lqc2(1, (c.g[A1][0] as u32));
                                c.lqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                                c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
                                c.lqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
                                c.sqc2(1, (c.g[A0][0] as u32));
                                c.sqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                                c.sqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                                c.sqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                let t = c.g[V0][0] != c.g[T1][0];
                                c.g[V1][0] = sx((c.g[V0][0] as u32) << 4);
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003431e0);
                                    }
                                    continue 'l_3431e0;
                                } else {
                                    break 'b_3432a4;
                                }
                            }
                        }
                    }
                    'b_34341c: {
                        c.cycles += 50;
                        c.g[V0][0] = c.g[T2][0];
                        c.lqc2(4, (c.g[T4][0] as u32));
                        c.lqc2(5, (c.g[T4][0] as u32).wrapping_add(0x10));
                        c.lqc2(6, (c.g[T4][0] as u32).wrapping_add(0x20));
                        c.lqc2(7, (c.g[T4][0] as u32).wrapping_add(0x30));
                        c.lqc2(8, (c.g[SP][0] as u32));
                        c.lqc2(9, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lqc2(10, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lqc2(11, (c.g[SP][0] as u32).wrapping_add(0x30));
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
                        c.sqc2(12, (c.g[T0][0] as u32));
                        c.sqc2(13, (c.g[T0][0] as u32).wrapping_add(0x10));
                        c.sqc2(14, (c.g[T0][0] as u32).wrapping_add(0x20));
                        c.sqc2(15, (c.g[T0][0] as u32).wrapping_add(0x30));
                        c.lqc2(1, (c.g[T0][0] as u32));
                        c.lqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                        c.lqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                        c.lqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                        c.sqc2(1, (c.g[V0][0] as u32));
                        c.sqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                        c.sqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                        c.sqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                        c.lqc2(1, (c.g[V0][0] as u32));
                        c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                        c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                        c.sqc2(1, (c.g[SP][0] as u32));
                        c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        c.g[V1][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[V0][0] = c.lb((c.g[V1][0] as u32));
                        let t = c.g[V0][0] == 0u64;
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_34341c;
                        } else {
                            c.cycles += 44;
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.lqc2(3, (c.g[S0][0] as u32));
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.vu0().sqrt(4, 0);
                            c.vu0().waitq();
                            c.cfc2(3, 22);
                            c.mtc1(3, 1);
                            c.div_s(0, 0, 1);
                            c.mfc1(3, 0);
                            c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x10));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.vu0().opmula(4, 5);
                            c.vu0().opmsub(6, 5, 4);
                            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                            c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x70));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.lqc2(3, (c.g[S0][0] as u32).wrapping_add(0x20));
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, (c.g[S0][0] as u32).wrapping_add(0x20));
                            c.lqc2(3, (c.g[S0][0] as u32).wrapping_add(0x30));
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, (c.g[S0][0] as u32).wrapping_add(0x30));
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                            if t {
                                break 'b_3434d8;
                            } else {
                                break 'b_34341c;
                            }
                        }
                    }
                    c.cycles += 46;
                    c.g[V0][0] = 0x500000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xeb0));
                    c.lqc2(8, (c.g[V0][0] as u32));
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(6, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(7, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().rsqrt(0, 3, 4, 0);
                    c.vu0().waitq();
                    c.vu0().mul(0b1111, 5, 3, Src::Q);
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x10));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                    c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x1c));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, (c.g[S0][0] as u32).wrapping_add(0x20));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[S0][0] as u32).wrapping_add(0x20));
                    break 'b_3434d4;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
            break 'b_3434d8;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.pc = t;
        return Ok(());
    });
}

/// `void Conveyor_Render(Conveyor* self)` (0x343588, `cConveyorNode_render`):
/// when the instance is visible and the graphics manager's cull test (vtable
/// slot 93) does not reject its box, draws the instance (vtable slot 96) with
/// the identity matrix translated by the belt's UV scroll.
pub fn conveyor_render(c: &mut Cpu) {
    guest(c, |c| {
        'b_343600: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[S5][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[A0][0] = 0xff;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x18));
            c.g[A2][0] = c.lw((c.g[V0][0] as u32));
            c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x7d));
            let t = c.g[V1][0] == c.g[A0][0];
            c.g[V0][0] = 0x1;
            if t {
                break 'b_343600;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x7d));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                c.g[V1][0] = c.g[V1][0] ^ 0x6;
                if c.g[V1][0] != 0u64 {
                    c.g[V0][0] = 0u64;
                    break 'b_343600;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x7e));
                    c.g[A0][0] = 0x18;
                    c.mult(0, 3, 2, 4);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3f0));
                    c.g[V1][0] = c.g[V1][0] ^ 0x3;
                    c.g[V0][0] = (c.g[V1][0] < 0x1u64) as u64;
                    break 'b_343600;
                }
            }
        }
        'b_3436f4: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[S4][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
            if t {
                break 'b_3436f4;
            } else {
                c.cycles += 37;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x18));
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.g[S2][0];
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x60));
                c.g[S0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x2e8));
                c.swc1(0, (c.g[SP][0] as u32));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 2);
                c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x64));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x68));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x6c));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x70));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x74));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10d8));
                c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x158));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x15c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00343678);
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                call_ptr(c, t, 0x00343678)?;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2ec));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S2][0];
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00343690);
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, t, 0x00343690)?;
                c.g[V1][0] = 0x1;
                let t = c.g[V0][0] == c.g[V1][0];
                c.g[V0][0] = 0x500000;
                if t {
                    break 'b_3436f4;
                } else {
                    c.cycles += 22;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
                    c.lqc2(1, (c.g[V0][0] as u32));
                    c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                    c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                    c.sqc2(1, (c.g[SP][0] as u32));
                    c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lwc1(0, (c.g[S5][0] as u32).wrapping_add(0x40));
                    c.g[A2][0] = 0u64;
                    c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0x44));
                    c.g[A3][0] = c.g[SP][0];
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[T0][0] = 0x2;
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10d8));
                    c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x300));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x304));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x003436f4);
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x003436f4)?;
                    break 'b_3436f4;
                }
            }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        return Ok(());
    });
}

/// `void CrowdMan3d_GroupBuildMatrices(CrowdGroup* self)` (0x344aa0): looks up
/// the group's instance in the world table, then for each of its nodes builds
/// the node's 4x4 matrix into the matrix array (`+0x1e8`) from the bone
/// matrices on VU0, with the instance's scale (+0x84) applied.
pub fn crowd_man3d_group_build_matrices(c: &mut Cpu) {
    guest(c, |c| {
        'b_344b04: {
            'b_344b00: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x120));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[S0][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f0));
                c.g[RA][0] = sx(0x00344ac0);
                c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                call_ptr(c, 0x002d1bd8, 0x00344ac0)?;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[A1][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                if t {
                    break 'b_344b00;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        break 'b_344b04;
                    } else {
                        break 'b_344b00;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_344b04;
        }
        'b_344e00: {
            c.cycles += 11;
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 1);
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
            c.swc1(0, (c.g[SP][0] as u32));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f0));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            let t = (c.g[V1][0] as i64) <= 0;
            c.g[T4][0] = 0u64;
            if t {
                break 'b_344e00;
            } else {
                c.cycles += 8;
                c.g[T6][0] = 0xffffffffffffffff;
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[T5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A3][0] = 0u64;
                c.g[T3][0] = 0u64;
                c.g[T2][0] = 0u64;
                'l_344b50: loop {
                    'b_344de8: {
                        'b_344d90: {
                            'b_344d60: {
                                'b_344c80: {
                                    'b_344c18: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A2][0] = c.lw((c.g[S1][0] as u32));
                                        if t {
                                            break 'b_344c18;
                                        } else {
                                            'b_344bd4: {
                                                c.cycles += 2;
                                                let t = c.g[A2][0] == c.g[T6][0];
                                                c.g[A2][0] = sx((c.g[A2][0] as u32) << 6);
                                                if t {
                                                    break 'b_344bd4;
                                                } else {
                                                    c.cycles += 28;
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x1ec)
                                                        );
                                                    c.lwc1(7, (c.g[SP][0] as u32));
                                                    c.g[T3][0] =
                                                        sx((c.g[T3][0] as u32).wrapping_add(0xd0));
                                                    c.g[A1][0] = sx((c.g[T2][0] as u32)
                                                        .wrapping_add((c.g[A1][0] as u32)));
                                                    c.lwc1(
                                                        6,
                                                        (c.g[SP][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.lq(2, (c.g[A1][0] as u32).wrapping_add(0x90));
                                                    c.g[T2][0] =
                                                        sx((c.g[T2][0] as u32).wrapping_add(0xd0));
                                                    c.lwc1(
                                                        5,
                                                        (c.g[SP][0] as u32).wrapping_add(0x8),
                                                    );
                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                    c.lwc1(
                                                        4,
                                                        (c.g[SP][0] as u32).wrapping_add(0xc),
                                                    );
                                                    c.lq(2, (c.g[A1][0] as u32).wrapping_add(0xa0));
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x1e8)
                                                        );
                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                    c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    c.lq(3, (c.g[A1][0] as u32).wrapping_add(0xb0));
                                                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                                    c.lq(2, (c.g[A1][0] as u32).wrapping_add(0xc0));
                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                                    c.lwc1(
                                                        1,
                                                        (c.g[SP][0] as u32).wrapping_add(0xa0),
                                                    );
                                                    c.lwc1(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0xa4),
                                                    );
                                                    c.lwc1(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0xa8),
                                                    );
                                                    c.mul_s(1, 1, 7);
                                                    c.lwc1(
                                                        0,
                                                        (c.g[SP][0] as u32).wrapping_add(0xac),
                                                    );
                                                    let t = true;
                                                    c.mul_s(2, 2, 6);
                                                    if t {
                                                        break 'b_344c80;
                                                    } else {
                                                        break 'b_344bd4;
                                                    }
                                                }
                                            }
                                            c.cycles += 17;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x1ec));
                                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0xd0));
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x1e8));
                                            c.g[V0][0] = sx((c.g[T3][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[V1][0] = sx((c.g[A3][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x90));
                                            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0xd0));
                                            c.lqc2(1, (c.g[V0][0] as u32));
                                            c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                                            c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                                            c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                                            c.sqc2(1, (c.g[V1][0] as u32));
                                            c.sqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                                            c.sqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                                            c.sqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                                            let t = true;
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x1e8));
                                            if t {
                                                break 'b_344d90;
                                            } else {
                                                break 'b_344c18;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[A2][0] == c.g[T6][0];
                                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 6);
                                    if t {
                                        break 'b_344d60;
                                    } else {
                                        c.cycles += 24;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                        c.lwc1(6, (c.g[SP][0] as u32));
                                        c.lq(2, (c.g[A1][0] as u32));
                                        c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.lq(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                                        c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0xc));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                        c.lq(3, (c.g[A1][0] as u32).wrapping_add(0x20));
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e8));
                                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x90));
                                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                        c.g[A2][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        c.lq(2, (c.g[A1][0] as u32).wrapping_add(0x30));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xa4));
                                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0xa8));
                                        c.mul_s(1, 1, 6);
                                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xac));
                                        c.mul_s(2, 2, 7);
                                        break 'b_344c80;
                                    }
                                }
                                c.cycles += 56;
                                c.mul_s(3, 3, 5);
                                c.mul_s(0, 0, 4);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x90));
                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x94));
                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x98));
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x9c));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                                c.lqc2(4, (c.g[A2][0] as u32));
                                c.lqc2(5, (c.g[A2][0] as u32).wrapping_add(0x10));
                                c.lqc2(6, (c.g[A2][0] as u32).wrapping_add(0x20));
                                c.lqc2(7, (c.g[A2][0] as u32).wrapping_add(0x30));
                                c.lqc2(8, (c.g[T5][0] as u32));
                                c.lqc2(9, (c.g[T5][0] as u32).wrapping_add(0x10));
                                c.lqc2(10, (c.g[T5][0] as u32).wrapping_add(0x20));
                                c.lqc2(11, (c.g[T5][0] as u32).wrapping_add(0x30));
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
                                c.sqc2(12, (c.g[T0][0] as u32));
                                c.sqc2(13, (c.g[T0][0] as u32).wrapping_add(0x10));
                                c.sqc2(14, (c.g[T0][0] as u32).wrapping_add(0x20));
                                c.sqc2(15, (c.g[T0][0] as u32).wrapping_add(0x30));
                                c.lqc2(1, (c.g[T0][0] as u32));
                                c.lqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                                c.lqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                                c.lqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                                c.sqc2(1, (c.g[T1][0] as u32));
                                c.sqc2(2, (c.g[T1][0] as u32).wrapping_add(0x10));
                                c.sqc2(3, (c.g[T1][0] as u32).wrapping_add(0x20));
                                c.sqc2(4, (c.g[T1][0] as u32).wrapping_add(0x30));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e8));
                                c.g[V0][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.lqc2(1, (c.g[T1][0] as u32));
                                c.lqc2(2, (c.g[T1][0] as u32).wrapping_add(0x10));
                                c.lqc2(3, (c.g[T1][0] as u32).wrapping_add(0x20));
                                c.lqc2(4, (c.g[T1][0] as u32).wrapping_add(0x30));
                                c.sqc2(1, (c.g[V0][0] as u32));
                                c.sqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                                c.sqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                                c.sqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f0));
                                if t {
                                    break 'b_344de8;
                                } else {
                                    break 'b_344d60;
                                }
                            }
                            c.cycles += 12;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e8));
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.lqc2(1, (c.g[V1][0] as u32));
                            c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                            c.sqc2(1, (c.g[V0][0] as u32));
                            c.sqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                            c.sqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                            c.sqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e8));
                            break 'b_344d90;
                        }
                        c.cycles += 22;
                        c.lwc1(6, (c.g[SP][0] as u32));
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                        c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x24));
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x28));
                        c.mul_s(1, 1, 6);
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c));
                        c.mul_s(2, 2, 7);
                        c.mul_s(3, 3, 5);
                        c.mul_s(0, 0, 4);
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x18));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f0));
                        break 'b_344de8;
                    }
                    c.cycles += 6;
                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00344b50);
                        }
                        continue 'l_344b50;
                    } else {
                        break 'b_344e00;
                    }
                }
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xf0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
        c.pc = t;
        return Ok(());
    });
}

/// `void FlexRail_UnregisterSpaceNode(FlexRail* self)` (0x348c48,
/// `cFlexRailNode_remSpaceHash`): removes the rail's space node from the
/// Nash: bumps the delete count, finds the octant by the bounding box key,
/// removes the node from the cell, frees the root cell when it is empty,
/// then deletes the node and clears the pointer.
pub fn flex_rail_unregister_space_node(c: &mut Cpu) {
    if c.pc != 0x00348c48 {
        return ee::s11::f_00348c48__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00348c48;
        loop {
            match pc {
                0x00348c48 => {
                    c.cycles += 38;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[RA][0] = sx(0x00348c64);
                    c.g[S2][0] = c.g[A0][0];
                    call_ptr(c, 0x002d1be0, 0x00348c64)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x00348c84);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                    call_ptr(c, 0x00328f28, 0x00348c84)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = 0xffffffffffffffff;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A0][0] = ((c.g[A3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[A2][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = 0x2;
                    if c.g[A3][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00348cd0);
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x003284b8, 0x00348cd0)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00348d5c;
                        continue;
                    }
                    pc = 0x00348ce0;
                    continue;
                }
                0x00348ce0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00348d5c;
                        continue;
                    }
                    pc = 0x00348cec;
                    continue;
                }
                0x00348cec => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00348d5c;
                        continue;
                    }
                    pc = 0x00348cf8;
                    continue;
                }
                0x00348cf8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00348d5c;
                        continue;
                    }
                    pc = 0x00348d04;
                    continue;
                }
                0x00348d04 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00348d5c;
                        continue;
                    }
                    pc = 0x00348d10;
                    continue;
                }
                0x00348d10 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00348d5c;
                        continue;
                    }
                    pc = 0x00348d1c;
                    continue;
                }
                0x00348d1c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00348d5c;
                        continue;
                    }
                    pc = 0x00348d28;
                    continue;
                }
                0x00348d28 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x00348d3c;
                        continue;
                    }
                    pc = 0x00348d34;
                    continue;
                }
                0x00348d34 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00348d5c;
                        continue;
                    }
                    pc = 0x00348d3c;
                    continue;
                }
                0x00348d3c => {
                    c.cycles += 1;
                    c.g[A0][0] = 0u64;
                    pc = 0x00348d40;
                    continue;
                }
                0x00348d40 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x00348d34) {
                            return Ok(());
                        }
                        pc = 0x00348d34;
                        continue;
                    }
                    pc = 0x00348d4c;
                    continue;
                }
                0x00348d4c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x00348d40) {
                            return Ok(());
                        }
                        pc = 0x00348d40;
                        continue;
                    }
                    pc = 0x00348d58;
                    continue;
                }
                0x00348d58 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00348d5c;
                    continue;
                }
                0x00348d5c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x00348d70;
                        continue;
                    }
                    pc = 0x00348d64;
                    continue;
                }
                0x00348d64 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00348d6c);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317e50, 0x00348d6c)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                    pc = 0x00348d70;
                    continue;
                }
                0x00348d70 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x00348d78);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                    call_ptr(c, 0x00317e50, 0x00348d78)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x54), 0u64);
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00348c48, 0x00348d94);
                    return Ok(());
                }
            }
        }
    });
}

/// `void* FloatRail_ctor(FloatRail* self, void* bucket, Desc* desc)`
/// (0x348d98, `cFloatRailNode_cFloatRailNode`): builds the rail on the
/// FlexRail base (type 20), sets the vtable and the spring constants from the
/// descriptor, packs the descriptor's used instance ids and meshes, allocates
/// one node per id, places each from its instance
/// (`FloatRail_ResetPosFromInstance`) and finishes with the matrices, the
/// bound box and the space-hash registration. Returns `self`.
pub fn float_rail_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S0][0] = c.g[A2][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[A1][0];
        c.g[A1][0] = 0x14;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
        c.g[RA][0] = sx(0x00348dcc);
        c.g[A3][0] = c.g[S0][0];
        Callee {
            addr: 0x00347d90,
            f: ee::s11::FlexRail_ctor,
        }
        .call(c, 0x00348dcc)?;
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = 0xffffffffffff0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xd0));
        c.g[A1][0] = 0xffffffffffffffff;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[V1][0] = c.g[SP][0];
        c.g[V0][0] = 0x7;
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x48));
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x60));
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4c));
        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x64));
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x50));
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x68));
        'l_348e08: loop {
            c.cycles += 7;
            c.sw((c.g[V1][0] as u32), c.g[A0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00348e08);
                }
                continue 'l_348e08;
            } else {
                c.cycles += 7;
                c.g[A3][0] = 0xffffffffffff0000;
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x70));
                c.g[A3][0] = c.g[A3][0] | 0xffff;
                c.g[T0][0] = 0x7;
                c.g[A0][0] = c.g[SP][0];
                'l_348e40: loop {
                    'b_348e5c: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                        let t = c.g[V1][0] == c.g[A3][0];
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_348e5c;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                            c.sw((c.g[A0][0] as u32), c.g[V1][0]);
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            break 'b_348e5c;
                        }
                    }
                    c.cycles += 3;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                    let t = (c.g[T0][0] as i64) >= 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00348e40);
                        }
                        continue 'l_348e40;
                    } else {
                        'b_348ec4: {
                            c.cycles += 12;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[A0][0] = 0x50;
                            c.g[A1][0] = 0x490000;
                            c.g[A2][0] = 0x20000000;
                            c.mult(0, 0, 16, 4);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17e0));
                            c.g[A0][0] = c.lo[0];
                            c.g[RA][0] = sx(0x00348e8c);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317e30, 0x00348e8c)?;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.g[V1][0] = c.g[V0][0];
                            if t {
                                break 'b_348ec4;
                            } else {
                                c.cycles += 4;
                                c.g[A1][0] = 0xffffffffffff0000;
                                c.g[A2][0] = 0xffffffffffffffff;
                                c.g[A1][0] = c.g[A1][0] | 0xffff;
                                'l_348ea8: loop {
                                    c.cycles += 7;
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[A1][0]);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                    let t = c.g[A0][0] != c.g[A2][0];
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x50));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00348ea8);
                                        }
                                        continue 'l_348ea8;
                                    } else {
                                        break 'b_348ec4;
                                    }
                                }
                            }
                        }
                        'b_348f60: {
                            c.cycles += 4;
                            c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[S3][0] = 0u64;
                            if t {
                                break 'b_348f60;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = 0x500000;
                                c.g[S2][0] = c.g[SP][0];
                                c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
                                c.g[S0][0] = 0u64;
                                'l_348ee8: loop {
                                    c.cycles += 30;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x40), c.g[A1][0]);
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                                    c.g[V1][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[A1][0]);
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.lq(3, (c.g[S4][0] as u32));
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sq(3, (c.g[V0][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.lq(2, (c.g[S4][0] as u32));
                                    c.g[A1][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    c.sq(2, (c.g[A1][0] as u32));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[RA][0] = sx(0x00348f40);
                                    c.g[A1][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                                    Callee {
                                        addr: 0x00349220,
                                        f: ee::s11::FloatRail_ResetPosFromInstance,
                                    }
                                    .call(c, 0x00348f40)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                                    c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x30));
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00348ee8);
                                        }
                                        continue 'l_348ee8;
                                    } else {
                                        break 'b_348f60;
                                    }
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[RA][0] = sx(0x00348f68);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x003492a8,
                            f: ee::s11::FloatRail_v48,
                        }
                        .call(c, 0x00348f68)?;
                        c.g[RA][0] = sx(0x00348f70);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00349110,
                            f: ee::s11::FloatRail_v50,
                        }
                        .call(c, 0x00348f70)?;
                        c.g[RA][0] = sx(0x00348f78);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00348b90,
                            f: ee::s11::FlexRail_RegisterSpaceNode,
                        }
                        .call(c, 0x00348f78)?;
                        c.g[V0][0] = c.g[S1][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `void FloatRail_ResetPosFromInstance(FloatRail* self, Node* node)`
/// (0x349220, `cFloatRailNode_getPosition`): looks the node's instance up in
/// the world table (low byte, then the high bits of its id), fetches the
/// node's bone matrix (`Instance_GetNodeMatrix`) and copies the matrix's
/// translation row to the node's position and previous position.
pub fn float_rail_reset_pos_from_instance(c: &mut Cpu) {
    guest(c, |c| {
        'b_34927c: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
            c.g[RA][0] = sx(0x00349234);
            c.g[S0][0] = c.g[A1][0];
            call_ptr(c, 0x002d1bd8, 0x00349234)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.g[A1][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            let t = c.g[V1][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_34927c;
            } else {
                c.cycles += 8;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = 0u64;
                if t {
                    break 'b_34927c;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                    break 'b_34927c;
                }
            }
        }
        c.cycles += 11;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x44));
        c.g[RA][0] = sx(0x00349288);
        c.g[A2][0] = c.g[SP][0];
        Callee {
            addr: 0x0034fed8,
            f: ee::s11::Instance_GetNodeMatrix,
        }
        .call(c, 0x00349288)?;
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x30));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `void* SpringRail_ctor(SpringRail* self, void* bucket, Desc* desc)`
/// (0x349b48, `cSpringRailNode_cSpringRailNode`): the spring rail's
/// constructor: FlexRail base, vtable and spring constants, the used
/// instance ids packed into a frame scratch, the 0x60-byte node array
/// allocated and each node placed from its instance, then matrices, bound box
/// and space-hash registration. Returns `self`.
pub fn spring_rail_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S0][0] = c.g[A2][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[A1][0];
        c.g[A1][0] = 0x15;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
        c.g[RA][0] = sx(0x00349b7c);
        c.g[A3][0] = c.g[S0][0];
        Callee {
            addr: 0x00347d90,
            f: ee::s11::FlexRail_ctor,
        }
        .call(c, 0x00349b7c)?;
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = 0xffffffffffff0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x270));
        c.g[A1][0] = 0xffffffffffffffff;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[V1][0] = c.g[SP][0];
        c.g[V0][0] = 0x7;
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x48));
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x60));
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4c));
        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x64));
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x50));
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x68));
        'l_349bb8: loop {
            c.cycles += 7;
            c.sw((c.g[V1][0] as u32), c.g[A0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00349bb8);
                }
                continue 'l_349bb8;
            } else {
                c.cycles += 7;
                c.g[A3][0] = 0xffffffffffff0000;
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x70));
                c.g[A3][0] = c.g[A3][0] | 0xffff;
                c.g[T0][0] = 0x7;
                c.g[A0][0] = c.g[SP][0];
                'l_349bf0: loop {
                    'b_349c0c: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                        let t = c.g[V1][0] == c.g[A3][0];
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_349c0c;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                            c.sw((c.g[A0][0] as u32), c.g[V1][0]);
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            break 'b_349c0c;
                        }
                    }
                    c.cycles += 3;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                    let t = (c.g[T0][0] as i64) >= 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00349bf0);
                        }
                        continue 'l_349bf0;
                    } else {
                        'b_349c74: {
                            c.cycles += 12;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[A0][0] = 0x60;
                            c.g[A1][0] = 0x490000;
                            c.g[A2][0] = 0x20000000;
                            c.mult(0, 0, 16, 4);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17d0));
                            c.g[A0][0] = c.lo[0];
                            c.g[RA][0] = sx(0x00349c3c);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317e30, 0x00349c3c)?;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.g[V1][0] = c.g[V0][0];
                            if t {
                                break 'b_349c74;
                            } else {
                                c.cycles += 4;
                                c.g[A1][0] = 0xffffffffffff0000;
                                c.g[A2][0] = 0xffffffffffffffff;
                                c.g[A1][0] = c.g[A1][0] | 0xffff;
                                'l_349c58: loop {
                                    c.cycles += 7;
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                    let t = c.g[A0][0] != c.g[A2][0];
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00349c58);
                                        }
                                        continue 'l_349c58;
                                    } else {
                                        break 'b_349c74;
                                    }
                                }
                            }
                        }
                        'b_349d24: {
                            c.cycles += 4;
                            c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[S4][0] = 0u64;
                            if t {
                                break 'b_349d24;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = 0x500000;
                                c.g[S2][0] = c.g[SP][0];
                                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
                                c.g[S0][0] = 0u64;
                                'l_349c98: loop {
                                    c.cycles += 34;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                                    c.g[V1][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x54), c.g[A1][0]);
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.lq(3, (c.g[S3][0] as u32));
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sq(3, (c.g[V0][0] as u32).wrapping_add(0x30));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.lq(2, (c.g[S3][0] as u32));
                                    c.g[A1][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x40));
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.lq(2, (c.g[S3][0] as u32));
                                    c.g[V1][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x20));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.lq(2, (c.g[S3][0] as u32));
                                    c.g[A1][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[RA][0] = sx(0x00349d10);
                                    c.g[A1][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                                    Callee {
                                        addr: 0x0034a028,
                                        f: ee::s11::SpringRail_InitPosFromInstance,
                                    }
                                    .call(c, 0x00349d10)?;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x60));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00349c98);
                                        }
                                        continue 'l_349c98;
                                    } else {
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                        break 'b_349d24;
                                    }
                                }
                            }
                        }
                        'b_349d74: {
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_349d74;
                            } else {
                                c.cycles += 2;
                                c.g[A2][0] = 0u64;
                                'l_349d38: loop {
                                    c.cycles += 15;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x60));
                                    c.lqc2(4, (c.g[V0][0] as u32));
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                    c.sq(3, (c.g[V0][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                    c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                    let t = c.g[A0][0] != 0u64;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00349d38);
                                        }
                                        continue 'l_349d38;
                                    } else {
                                        break 'b_349d74;
                                    }
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[RA][0] = sx(0x00349d7c);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x0034a0b0,
                            f: ee::s11::SpringRail_v48,
                        }
                        .call(c, 0x00349d7c)?;
                        c.g[RA][0] = sx(0x00349d84);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00349f18,
                            f: ee::s11::SpringRail_v50,
                        }
                        .call(c, 0x00349d84)?;
                        c.g[RA][0] = sx(0x00349d8c);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00348b90,
                            f: ee::s11::FlexRail_RegisterSpaceNode,
                        }
                        .call(c, 0x00349d8c)?;
                        c.g[V0][0] = c.g[S1][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `void* SpringRail_ctor_2(SpringRail* self, void* bucket, Stream* s)`
/// (0x349db0, `cSpringRailNode_cSpringRailNode1`): the stream form: second
/// FlexRail base constructor, the spring constants and the node array read
/// from the stream, then matrices and the space-hash registration. Returns
/// `self`.
pub fn spring_rail_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        'b_349e5c: {
            c.cycles += 32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x00349dd4);
            c.g[S2][0] = c.g[A2][0];
            Callee {
                addr: 0x00347ea8,
                f: ee::s11::FlexRail_ctor_2,
            }
            .call(c, 0x00349dd4)?;
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = 0x490000;
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x60));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x270));
            c.g[A2][0] = 0xc;
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00349e00);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00349e00)?;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = 0x60;
            c.g[A1][0] = 0x490000;
            c.g[A2][0] = 0x20000000;
            c.mult(0, 0, 16, 4);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17d0));
            c.g[A0][0] = c.lo[0];
            c.g[RA][0] = sx(0x00349e24);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x00349e24)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = c.g[V0][0];
            if t {
                break 'b_349e5c;
            } else {
                c.cycles += 4;
                c.g[A1][0] = 0xffffffffffff0000;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                'l_349e40: loop {
                    c.cycles += 7;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A0][0] != c.g[A2][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00349e40);
                        }
                        continue 'l_349e40;
                    } else {
                        break 'b_349e5c;
                    }
                }
            }
        }
        c.cycles += 23;
        c.sw((c.g[S3][0] as u32), c.g[V0][0]);
        c.g[V0][0] = 0x60;
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.mult(0, 0, 6, 2);
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        c.g[A2][0] = c.lo[0];
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00349e88);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00349e88)?;
        c.g[RA][0] = sx(0x00349e90);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0034a0b0,
            f: ee::s11::SpringRail_v48,
        }
        .call(c, 0x00349e90)?;
        c.g[RA][0] = sx(0x00349e98);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x00348b90,
            f: ee::s11::FlexRail_RegisterSpaceNode,
        }
        .call(c, 0x00349e98)?;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the oc686 callee fakes (in the machine's own RAM).
    const OC686_CTRL: u32 = DATA + 0x000e_8000;
    const OC686_MAGIC: u32 = 0x0686_0686;
    /// What the gated `World_GetInstance` fake returns.
    const OC686_WORLD: u32 = DATA + 0x0001_0000;

    /// The instance object the second table level points to (its scale at +0x84).
    const OC686_INST_OBJ: u32 = DATA + 0x0001_8000;

    /// A gated logging fake for one callee slot: with the gate off the body
    /// that was in the slot runs.
    macro_rules! oc686_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path, $post:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC686_CTRL) != OC686_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let args = [c.g[A0][0], c.g[A1][0], c.g[A2][0], c.g[A3][0]];
                fakes::fake_call(c, $addr);
                let post: fn(&mut Cpu, [u64; 4]) = $post;
                post(c, args);
            }
        };
    }

    oc686_fake!(
        oc686_f_world,
        OC686_PREV_WORLD,
        0x002d_1bd8,
        ee::s09::f_002d1bd8__generated,
        |c, _| c.g[V0][0] = OC686_WORLD as u64
    );
    // `Instance_GetNodeMatrix(inst, mesh, out)`: a deterministic matrix at a2.
    oc686_fake!(
        oc686_f_node_matrix,
        OC686_PREV_NODE_MATRIX,
        0x0034_fed8,
        ee::s11::f_0034fed8__generated,
        |c, a| {
            for i in 0..16u32 {
                let w = fakes::mix(a[0] ^ a[1] << 32 ^ i as u64) as u32;
                c.bus.write32(a[2] as u32 + 4 * i, w);
            }
        }
    );

    fn oc686_install() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D1BD8),
            oc686_f_world,
            &OC686_PREV_WORLD,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FED8),
            oc686_f_node_matrix,
            &OC686_PREV_NODE_MATRIX,
        );
        oc686_install_more();
    }

    /// The world's two-level instance table the node placement walks:
    /// `World_GetInstance()` -> object (+0) -> table (+8) indexed by the low
    /// byte of an instance id -> entry (+0x1c) -> table indexed by the rest
    /// of the id; ids are `(a << 8) | b` with `a, b < 4`. `gaps` leaves some
    /// slots empty on either level.
    fn oc686_world_tables(c: &mut Cpu, r: &mut Rng, seed: u32, gaps: bool) {
        let (obj, tbl, ent, tbl2) = (DATA + 0x3000, DATA + 0x4000, DATA + 0x5000, DATA + 0x6000);
        c.bus.write32(OC686_WORLD, obj);
        c.bus.write32(obj + 8, tbl);
        let has_ent = !gaps || seed % 5 != 0;
        for i in 0..4u32 {
            c.bus.write32(tbl + 4 * i, if has_ent { ent } else { 0 });
        }
        c.bus.write32(ent + 0x1c, tbl2);
        for i in 0..4u32 {
            let low = r.next_u32() & 0xff;
            let v = if gaps && (seed / 5 + i) % 3 == 0 {
                low
            } else {
                ((OC686_INST_OBJ >> 2) << 8) | low
            };
            c.bus.write32(tbl2 + 4 * i, v);
        }
    }

    /// `Nash_FindClosestObjectContact(nash, query, out, full)`: with no hits
    /// (count 0 at `nash+0x24`; the Nash has no root cell, so the real
    /// `Nash_CollideObjects` just returns the count) the result is -1.0.
    /// Otherwise the hit whose `Iparam` is closest to the query's ideal value
    /// (a virtual call, fake, unwinding on some seeds) wins; equal distances
    /// go through the tie-break on the hits' object pointers (+0x50/+0x54);
    /// the winner is copied to `out`. The hits are the frame buffer the
    /// callee left, so the test pre-fills that stack memory.

    /// `AnimTeeter_v43(self, hit, force)`: a force on a teeter bone adds to
    /// the torque sum (`self+0`). Paths: a negative mesh index, a bone with
    /// no animation (+8 of its table entry is zero), a short force (length
    /// not above 100), and the full computation with the lever arm negated
    /// by the torque scale's sign and by the mesh's animation sign bit. The
    /// bone matrix comes from a virtual call (fake method at vtable
    /// `+0xe8/0xec`, unwinding on some seeds).
    #[test]
    fn anim_teeter_v43_matches() {
        covered(
            ee::s11::f_00342538__generated,
            anim_teeter_v43,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_2538, 1);
                c.pc = 0x0034_2538;
                let (this, hit, force, vt, inst, model, ents, amat) = (
                    DATA + 0x1000,
                    DATA + 0x1400,
                    DATA + 0x1800,
                    DATA + 0x1c00,
                    DATA + 0x2000,
                    DATA + 0x2400,
                    DATA + 0x2800,
                    DATA + 0x3000,
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(hit);
                c.g[A2][0] = sx(force);
                let mesh = if seed % 9 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32() % 6
                };
                c.bus.write32(hit + 0x5c, mesh);
                oc686_put_floats(c, &mut r, hit, 4, 50.0);
                // The force: long (above 100) on most seeds.
                let len = if seed % 4 == 1 { 30.0 } else { 160.0 };
                for k in 0..3u32 {
                    let v = r.f32(len);
                    c.bus.write32(force + 4 * k, v);
                }
                c.bus.write32(this + 0x50, vt);
                c.bus.write16(vt + 0xe8, (4 * (r.next_u32() % 3)) as u16);
                c.bus.write32(vt + 0xec, fakes::FAKE);
                c.bus.write32(fakes::LOG_RESULTS, amat);
                oc686_put_floats(c, &mut r, amat, 16, 30.0);
                c.bus.write32(this + 0x5c, inst);
                c.bus.write32(inst + 0x80, model);
                c.bus.write32(model + 8, ents);
                for i in 0..6u32 {
                    c.bus.write32(
                        ents + 16 * i + 8,
                        if seed % 7 == 2 { 0 } else { r.next_u32() | 1 },
                    );
                }
                // Torque sum, torque scale (sign) and the animation sign bits.
                c.bus.write32(this, r.f32(5.0));
                c.bus.write32(this + 8, r.f32(3.0));
                c.bus.write32(this + 0x2c, r.next_u32());
                oc686_put_floats(c, &mut r, 0x004f_f150, 4, 1.0);
            },
        );
    }

    /// Sixteen random floats at `addr` (a 4x4 matrix or four vectors).
    fn oc686_put_floats(c: &mut Cpu, r: &mut Rng, addr: u32, n: u32, range: f32) {
        for i in 0..n {
            let v = r.f32(range);
            c.bus.write32(addr + 4 * i, v);
        }
    }

    /// `Conveyor_v42(self, hit)`: with a negative mesh index nothing happens.
    /// Otherwise the bone's world matrix is built from the bone chain
    /// (0..3 parents, from the bone table in the instance's model) and the
    /// object matrix returned by the virtual call at vtable `+0xc0/0xc4`
    /// (fake method, unwinding on some seeds); a flagged bone pushes the hit
    /// as a torque and impulse about its position, an unflagged one adds a
    /// torque about the bone's up axis.
    #[test]
    fn conveyor_v42_matches() {
        covered(
            ee::s11::f_00343130__generated,
            conveyor_v42,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_3130, 1);
                c.pc = 0x0034_3130;
                let (this, hit, vt, inst, model, bones, wmat, mats) = (
                    DATA + 0x1000,
                    DATA + 0x1400,
                    DATA + 0x1800,
                    DATA + 0x2000,
                    DATA + 0x2400,
                    DATA + 0x9000,
                    DATA + 0xb000,
                    DATA + 0xa000,
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(hit);
                let mesh = if seed % 7 == 0 {
                    0xffff_fff0 | (r.next_u32() & 7)
                } else {
                    r.next_u32() % 5
                };
                c.bus.write32(hit + 0x5c, mesh);
                oc686_put_floats(c, &mut r, hit, 16, 4.0);
                c.bus.write32(this + 0xc, vt);
                c.bus.write16(vt + 0xc0, 0);
                c.bus.write32(vt + 0xc4, fakes::FAKE);
                c.bus.write32(fakes::LOG_RESULTS, wmat);
                c.bus.write32(this + 0x18, inst);
                c.bus.write32(inst + 0x80, model);
                c.bus.write32(model + 8, bones);
                // Bone i: parent id (-1 or a lower bone) and its matrix pointer.
                for i in 0..5u32 {
                    let parent = if i == 0 || r.next_u32() % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % i
                    };
                    c.bus.write32(bones + 16 * i, parent);
                    c.bus.write32(bones + 16 * i + 0xc, mats + 0x40 * i);
                    oc686_put_floats(c, &mut r, mats + 0x40 * i, 16, 2.0);
                }
                oc686_put_floats(c, &mut r, wmat, 16, 2.0);
                // Per-bone flags (inline at +0x20) and the push strength.
                for i in 0..8u32 {
                    c.bus
                        .write8(this + 0x20 + i, if r.next_u32() % 2 == 0 { 0 } else { 1 });
                }
                c.bus.write32(this + 0x1c, r.f32(10.0));
                // The up axis constant the unflagged branch transforms.
                oc686_put_floats(c, &mut r, 0x004f_f150, 4, 1.0);
            },
        );
    }

    /// `Conveyor_Render(self)`: the instance is drawn only if it is visible
    /// (a record byte of 0xff, or the camera's table entry says so: both
    /// table checks are exercised), the graphics manager's cull test (virtual
    /// call, fake; result 1 culls it on some seeds) passes, and then it makes
    /// the draw virtual call with the identity matrix scrolled by the UV.
    #[test]
    fn conveyor_render_matches() {
        covered(
            ee::s11::f_00343588__generated,
            conveyor_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_3588, 3);
                c.pc = 0x0034_3588;
                let (this, inst, holder, cam, gfx, gvt) = (
                    DATA + 0x1000,
                    DATA + 0x2000,
                    DATA + 0x3000,
                    DATA + 0x4000,
                    DATA + 0x5000,
                    DATA + 0x6000,
                );
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x18, inst);
                oc686_put_floats(c, &mut r, this + 0x40, 2, 8.0);
                c.bus.write32(GAME_GP + 0x16c8, holder);
                c.bus.write32(holder, cam);
                c.bus.write32(GAME_GP + 0x2a90, gfx);
                c.bus.write32(gfx + 0x10d8, gvt);
                for off in [0x158, 0x2e8, 0x300] {
                    c.bus.write16(gvt + off, (4 * (r.next_u32() % 4)) as u16);
                }
                for off in [0x15c, 0x2ec, 0x304] {
                    c.bus.write32(gvt + off, fakes::FAKE + 0x20);
                }
                // The cull test's result: 1 culls the instance.
                c.bus
                    .write32(fakes::LOG_RESULTS + 4, (seed % 3 == 0) as u32);
                oc686_put_floats(c, &mut r, inst + 0x60, 6, 20.0);
                // Visibility: record byte 0xff, else the camera's table entries.
                let byte = if seed % 4 == 0 {
                    0xff
                } else {
                    r.next_u32() % 4
                };
                let idx = r.next_u32() % 4;
                c.bus.write8(inst + 0x7d, byte as u8);
                c.bus.write16(inst + 0x7e, idx as u16);
                let e1 = if seed % 5 < 3 { 6 } else { r.next_u32() };
                let e2 = if seed % 3 < 2 { 3 } else { r.next_u32() };
                c.bus.write32(cam + 0x24 + 8 * (byte % 4), e1);
                c.bus.write32(cam + 0x3f0 + 0x18 * idx, e2);
                oc686_put_floats(c, &mut r, 0x004f_f1a0, 16, 2.0);
            },
        );
    }

    oc686_fake!(
        oc686_f_nash,
        OC686_PREV_NASH,
        0x002d_1be0,
        ee::s09::f_002d1be0__generated,
        |_, _| {}
    );
    oc686_fake!(
        oc686_f_find_key,
        OC686_PREV_FIND_KEY,
        0x0032_8f28,
        ee::s10::f_00328f28__generated,
        |_, _| {}
    );
    oc686_fake!(
        oc686_f_cell_remove,
        OC686_PREV_CELL_REMOVE,
        0x0032_84b8,
        ee::s10::f_003284b8__generated,
        |_, _| {}
    );
    oc686_fake!(
        oc686_f_delete,
        OC686_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated,
        |_, _| {}
    );
    oc686_fake!(
        oc686_f_new,
        OC686_PREV_NEW,
        0x0031_7e30,
        ee::s10::f_00317e30__generated,
        |c, _| c.g[V0][0] = sx(OC686_ALLOC)
    );

    /// The block `operator_new` hands out.
    const OC686_ALLOC: u32 = DATA + 0x000a_0000;

    fn oc686_install_more() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D1BE0),
            oc686_f_nash,
            &OC686_PREV_NASH,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328F28),
            oc686_f_find_key,
            &OC686_PREV_FIND_KEY,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003284B8),
            oc686_f_cell_remove,
            &OC686_PREV_CELL_REMOVE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc686_f_delete,
            &OC686_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc686_f_new,
            &OC686_PREV_NEW,
        );
    }

    /// `FlexRail_UnregisterSpaceNode(self)`: bumps the Nash's delete count,
    /// takes the node out of the cell the bounding box key selects (one of
    /// eight octants, by the key's signs, read from uninitialised frame
    /// memory), deletes the octant's root cell when its 11 slots are all
    /// empty, then deletes the node. The Nash lookup, key finder, cell
    /// removal and `operator_delete` are gated fakes; unwinds at any of them.
    #[test]
    fn flex_rail_unregister_space_node_matches() {
        oc686_install();
        covered(
            ee::s11::f_00348c48__generated,
            flex_rail_unregister_space_node,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_8c48, 5);
                c.pc = 0x0034_8c48;
                c.bus.write32(OC686_CTRL, OC686_MAGIC);
                let (this, nash, root) = (DATA + 0x1000, DATA + 0x4000, DATA + 0x5000);
                c.g[A0][0] = sx(this);
                c.bus.write32(fakes::LOG_RESULTS, nash);
                c.bus.write32(this + 0x54, r.next_u32());
                fill(c, &mut r, this + 0x30, 6);
                c.bus.write32(nash + 0xa0, r.next_u32());
                // Every octant points at one root cell: empty, or with one
                // used slot among the 11 the emptiness test reads.
                for o in 0..8u32 {
                    c.bus.write32(nash + 0x14 * o + 0x10, root);
                }
                for i in 0..12u32 {
                    c.bus.write32(root + 4 * i, 0);
                }
                let used = seed % 12;
                if used < 11 {
                    c.bus.write32(root + 4 * used, r.next_u32() | 1);
                }
            },
        );
    }

    /// `CrowdMan3d_GroupBuildMatrices(self)`: reads the group's instance scale
    /// through the world table (the instance may be missing: scale from
    /// address 0x84), then for each of 0..4 entries builds its output matrix:
    /// copied from the entry's own matrix or from the source array
    /// (entry flag), or (parent index set) multiplied by an already built
    /// parent matrix, with the translation row scaled. `World_GetInstance`
    /// is a gated fake; unwinds at it.
    #[test]
    fn crowd_man3d_group_build_matrices_matches() {
        oc686_install();
        covered(
            ee::s11::f_00344aa0__generated,
            crowd_man3d_group_build_matrices,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_4aa0, 1);
                c.pc = 0x0034_4aa0;
                c.bus.write32(OC686_CTRL, OC686_MAGIC);
                let (this, group, list, out, src, mats) = (
                    DATA + 0x1000,
                    DATA + 0x2_0000,
                    DATA + 0x2_0400,
                    DATA + 0x2_1000,
                    DATA + 0x2_2000,
                    DATA + 0x2_3000,
                );
                c.g[A0][0] = sx(this);
                let id = (r.next_u32() % 4) << 8 | (r.next_u32() % 4);
                c.bus.write32(this + 4, id);
                c.bus.write32(this + 0x1e8, out);
                c.bus.write32(this + 0x1ec, src);
                c.bus.write32(this + 0x1f0, group);
                let n = r.next_u32() % 5;
                c.bus.write32(group + 4, n);
                c.bus.write32(group + 8, list);
                oc686_world_tables(c, &mut r, seed, seed % 6 == 0);
                if seed % 6 == 0 || seed % 11 == 0 {
                    // No instance scale: the read hits address 0x84 instead.
                } else {
                    oc686_put_floats(c, &mut r, OC686_INST_OBJ + 0x84, 1, 3.0);
                }
                oc686_put_floats(c, &mut r, out, 0x40, 2.0);
                oc686_put_floats(c, &mut r, src, 0xd0, 2.0);
                for i in 0..4u32 {
                    let e = list + 0x10 * i;
                    // Parent: none, or an earlier entry.
                    let parent = if i == 0 || r.next_u32() % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % i
                    };
                    c.bus.write32(e, parent);
                    c.bus.write32(
                        e + 8,
                        if r.next_u32() % 2 == 0 {
                            0
                        } else {
                            r.next_u32() | 1
                        },
                    );
                    c.bus.write32(e + 0xc, mats + 0x40 * i);
                    oc686_put_floats(c, &mut r, mats + 0x40 * i, 16, 2.0);
                }
            },
        );
    }

    oc686_fake!(
        oc686_f_flex_ctor,
        OC686_PREV_FLEX_CTOR,
        0x0034_7d90,
        ee::s11::f_00347d90__generated,
        |_, _| {}
    );
    oc686_fake!(
        oc686_f_flex_ctor_2,
        OC686_PREV_FLEX_CTOR_2,
        0x0034_7ea8,
        ee::s11::f_00347ea8__generated,
        |_, _| {}
    );
    oc686_fake!(
        oc686_f_float_mats,
        OC686_PREV_FLOAT_MATS,
        0x0034_92a8,
        ee::s11::f_003492a8__generated,
        |_, _| {}
    );
    oc686_fake!(
        oc686_f_float_bounds,
        OC686_PREV_FLOAT_BOUNDS,
        0x0034_9110,
        ee::s11::f_00349110__generated,
        |_, _| {}
    );
    oc686_fake!(
        oc686_f_space_register,
        OC686_PREV_SPACE_REGISTER,
        0x0034_8b90,
        ee::s11::f_00348b90__generated,
        |_, _| {}
    );

    fn oc686_install_rails() {
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00347D90),
            oc686_f_flex_ctor,
            &OC686_PREV_FLEX_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00347EA8),
            oc686_f_flex_ctor_2,
            &OC686_PREV_FLEX_CTOR_2,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003492A8),
            oc686_f_float_mats,
            &OC686_PREV_FLOAT_MATS,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00349110),
            oc686_f_float_bounds,
            &OC686_PREV_FLOAT_BOUNDS,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00348B90),
            oc686_f_space_register,
            &OC686_PREV_SPACE_REGISTER,
        );
    }

    /// A rail descriptor: `used` of the eight instance id slots (+4) hold ids
    /// (`(a << 8) | b`, `a, b < 4`), the others are -1; meshes at +0x24, the
    /// three spring constants at +0x48.
    fn oc686_rail_desc(c: &mut Cpu, r: &mut Rng, desc: u32, used: u32) {
        let mut left = used;
        for i in 0..8u32 {
            // Spread the used slots: slot i is used with the odds left/(8-i).
            let take = left > 0 && r.next_u32() % (8 - i) < left;
            let id = if take {
                left -= 1;
                (r.next_u32() % 4) << 8 | (r.next_u32() % 4)
            } else {
                0xffff_ffff
            };
            c.bus.write32(desc + 4 + 4 * i, id);
            c.bus.write32(desc + 0x24 + 4 * i, r.next_u32() % 16);
        }
        for k in 0..3u32 {
            c.bus.write32(desc + 0x48 + 4 * k, r.edge_f32());
        }
    }

    /// `FloatRail_ctor(self, bucket, desc)`: the base constructor, the vtable
    /// and spring constants, the descriptor's ids packed into a frame scratch
    /// (0..8 used), the node block allocated and each node placed through
    /// `FloatRail_ResetPosFromInstance` (real, with the world fakes), then
    /// matrices, bound box and space registration. The base constructor, the
    /// allocator and the three finishers are gated fakes; unwinds at any call.
    #[test]
    fn float_rail_ctor_matches() {
        oc686_install();
        oc686_install_rails();
        covered(
            ee::s11::f_00348d98__generated,
            float_rail_ctor,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_8d98, 7);
                c.pc = 0x0034_8d98;
                c.bus.write32(OC686_CTRL, OC686_MAGIC);
                let (this, desc) = (DATA + 0x1000, DATA + 0x1800);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x1400);
                c.g[A2][0] = sx(desc);
                let used = seed % 9;
                oc686_rail_desc(c, &mut r, desc, used);
                // The base constructor (a fake) leaves the node count in the rail.
                c.bus.write32(this + 0x20, used);
                oc686_world_tables(c, &mut r, seed, false);
            },
        );
    }

    /// `FloatRail_ResetPosFromInstance(self, node)`: the node's instance is
    /// found through the world's two-level table (it may be missing at either
    /// level), its bone matrix is fetched into the frame and the translation
    /// row is stored into the node. `World_GetInstance` and
    /// `Instance_GetNodeMatrix` are gated fakes; unwinds at either call.
    #[test]
    fn float_rail_reset_pos_from_instance_matches() {
        oc686_install();
        covered(
            ee::s11::f_00349220__generated,
            float_rail_reset_pos_from_instance,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_9220, 2);
                c.pc = 0x0034_9220;
                c.bus.write32(OC686_CTRL, OC686_MAGIC);
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(node);
                // InstId: low byte indexes the first table, the rest the second.
                let id = (r.next_u32() % 4) << 8 | (r.next_u32() % 4);
                c.bus.write32(node + 0x40, id);
                c.bus.write32(node + 0x44, r.next_u32());
                oc686_world_tables(c, &mut r, seed, true);
            },
        );
    }

    oc686_fake!(
        oc686_f_spring_bounds,
        OC686_PREV_SPRING_BOUNDS,
        0x0034_9f18,
        ee::s11::f_00349f18__generated,
        |_, _| {}
    );

    /// The rail fixture both spring constructors share: the matrix array
    /// (`self+0x50`, written by the real `SpringRail_v48`), the constants
    /// it reads and the node block `operator_new` returns.
    fn oc686_spring_rail(c: &mut Cpu, r: &mut Rng, this: u32, nodes: u32) {
        c.bus.write32(this + 0x20, nodes);
        c.bus.write32(this + 0x50, DATA + 0x7000);
        fill(c, r, DATA + 0x7000, 0x100);
        fill(c, r, OC686_ALLOC, 0xc0);
        fill(c, r, 0x004f_fac0, 16);
    }

    /// `SpringRail_ctor(self, bucket, desc)`: the spring rail builder. The
    /// descriptor's used ids are packed into the frame, the node block
    /// (0x60 bytes per node) allocated, each node placed by the real
    /// `SpringRail_InitPosFromInstance`, the segment vectors built, then the
    /// real `SpringRail_v48` (matrices) and the fakes for the base
    /// constructor, allocator, `SpringRail_v50` and the space registration.
    #[test]
    fn spring_rail_ctor_matches() {
        oc686_install();
        oc686_install_rails();
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00349F18),
            oc686_f_spring_bounds,
            &OC686_PREV_SPRING_BOUNDS,
        );
        covered(
            ee::s11::f_00349b48__generated,
            spring_rail_ctor,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_9b48, 8);
                c.pc = 0x0034_9b48;
                c.bus.write32(OC686_CTRL, OC686_MAGIC);
                let (this, desc) = (DATA + 0x1000, DATA + 0x1800);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x1400);
                c.g[A2][0] = sx(desc);
                let used = seed % 9;
                oc686_rail_desc(c, &mut r, desc, used);
                oc686_spring_rail(c, &mut r, this, used);
                oc686_world_tables(c, &mut r, seed, false);
            },
        );
    }

    /// `SpringRail_ctor_2(self, bucket, stream)`: the stream form. The two
    /// stream reads (spring constants, then the node block) are virtual
    /// calls into fake methods; the base constructor, the allocator and the
    /// space registration are gated fakes; `SpringRail_v48` runs for real.
    #[test]
    fn spring_rail_ctor_2_matches() {
        oc686_install();
        oc686_install_rails();
        covered(
            ee::s11::f_00349db0__generated,
            spring_rail_ctor_2,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_9db0, 6);
                c.pc = 0x0034_9db0;
                c.bus.write32(OC686_CTRL, OC686_MAGIC);
                let (this, stream, record) = (DATA + 0x1000, DATA + 0x2800, DATA + 0x2900);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x1400);
                c.g[A2][0] = sx(stream);
                oc686_spring_rail(c, &mut r, this, seed % 7);
                // The stream's read method: a vtable slot (delta, fake address).
                c.bus.write32(stream, record);
                c.bus.write16(record + 0x10, (4 * (seed % 8)) as u16);
                c.bus.write32(record + 0x14, fakes::FAKE + 0x40);
            },
        );
    }
}
