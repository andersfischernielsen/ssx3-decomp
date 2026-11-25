//! Staging module oc602 (wave wave233): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc602`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0034_6e38,
        flex_bridge_apply_force,
        "FlexBridge_ApplyForce",
    ),
    (
        0x0034_70d0,
        flex_bridge_update_intersect_info,
        "FlexBridge_UpdateIntersectInfo",
    ),
    (0x0034_7268, flex_bridge_intersect, "FlexBridge_Intersect"),
];

/// The 32-bit address `r + off` (the generated `(c.g[r][0] as u32).wrapping_add(off)`).
#[inline(always)]
fn fb_ea(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `void FlexBridgeNode::applyForce(const tIntersectInfo &, const t4Vector &)`
/// (0x346e38): skipped when the weight at `intInfo+0x60` is negative; otherwise
/// transforms the force by the node's matrix (vtable call on `this+0xc`).
pub fn flex_bridge_apply_force(c: &mut Cpu) {
    guest(c, apply_force)
}

/// `void FlexBridgeNode::updateIntersectInfo(tIntersectInfo &)` (0x3470d0).
pub fn flex_bridge_update_intersect_info(c: &mut Cpu) {
    guest(c, update_intersect_info)
}

/// `int FlexBridgeNode::intersect(cWorldIntersectType &, tIntersectInfo *, int, int)` (0x347268).
pub fn flex_bridge_intersect(c: &mut Cpu) {
    guest(c, intersect)
}

fn apply_force(c: &mut Cpu) -> Flow {
    'b_3470b4: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
        c.sq(16, fb_ea(c, SP, 0xd0));
        c.sq(17, fb_ea(c, SP, 0xc0));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, fb_ea(c, SP, 0xb0));
        c.g[S1][0] = c.g[A0][0];
        c.sd(fb_ea(c, SP, 0xa0), c.g[RA][0]);
        c.g[V0][0] = c.lw(fb_ea(c, S0, 0x60));
        let t = (c.g[V0][0] as i64) < 0;
        c.g[S2][0] = c.g[A2][0];
        if t {
            break 'b_3470b4;
        } else {
            c.cycles += 149;
            c.g[V1][0] = c.lw(fb_ea(c, S1, 0xc));
            c.g[A0][0] = c.lh(fb_ea(c, V1, 0xc0));
            c.g[V0][0] = c.lw(fb_ea(c, V1, 0xc4));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00346e74);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00346e74)?;
            c.lq(3, fb_ea(c, V0, 0x30));
            c.g[T0][0] = 0x50;
            c.sq(3, fb_ea(c, SP, 0x10));
            c.lqc2(3, (c.g[S0][0] as u32));
            c.lqc2(4, fb_ea(c, SP, 0x10));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, SP, 0x20));
            c.lq(3, fb_ea(c, SP, 0x20));
            c.sq(3, (c.g[SP][0] as u32));
            c.lqc2(3, fb_ea(c, S1, 0x30));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, fb_ea(c, SP, 0x30));
            c.lq(3, fb_ea(c, SP, 0x30));
            c.sq(3, fb_ea(c, SP, 0x20));
            c.lqc2(8, fb_ea(c, SP, 0x20));
            c.lqc2(4, (c.g[V0][0] as u32));
            c.lqc2(5, fb_ea(c, V0, 0x10));
            c.lqc2(6, fb_ea(c, V0, 0x20));
            c.lqc2(7, fb_ea(c, V0, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, fb_ea(c, SP, 0x30));
            c.g[V1][0] = c.lw(fb_ea(c, S0, 0x60));
            c.g[A1][0] = c.lw(fb_ea(c, S1, 0x80));
            c.lq(4, fb_ea(c, SP, 0x30));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            c.sq(4, fb_ea(c, SP, 0x10));
            c.lqc2(8, (c.g[V1][0] as u32));
            c.lqc2(4, (c.g[V0][0] as u32));
            c.lqc2(5, fb_ea(c, V0, 0x10));
            c.lqc2(6, fb_ea(c, V0, 0x20));
            c.lqc2(7, fb_ea(c, V0, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, fb_ea(c, SP, 0x30));
            c.lq(4, fb_ea(c, SP, 0x30));
            c.g[A1][0] = c.lw(fb_ea(c, S0, 0x60));
            c.sq(4, fb_ea(c, SP, 0x20));
            c.lqc2(3, (c.g[SP][0] as u32));
            c.lqc2(5, fb_ea(c, SP, 0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 8);
            c.g[V1][0] = sx((c.g[A1][0] as u32) >> 31);
            c.lq(4, fb_ea(c, V0, 0x30));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[A2][0] = c.lw(fb_ea(c, S1, 0x74));
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 1) as u32);
            c.lwc1(0, fb_ea(c, S2, 0xc));
            c.mult(0, 3, 5, 8);
            c.lwc1(7, fb_ea(c, S1, 0x64));
            c.sq(4, fb_ea(c, SP, 0x80));
            c.mult(0, 7, 5, 8);
            c.mfc1(8, 0);
            c.lqc2(4, fb_ea(c, SP, 0x80));
            c.qmtc2(8, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, fb_ea(c, SP, 0x90));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
            c.lq(3, fb_ea(c, SP, 0x90));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A2][0] as u32)));
            c.sq(3, fb_ea(c, SP, 0x70));
            c.lqc2(3, (c.g[S2][0] as u32));
            c.lqc2(4, fb_ea(c, SP, 0x70));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, SP, 0x90));
            c.lq(3, fb_ea(c, SP, 0x90));
            c.lwc1(6, fb_ea(c, S2, 0xc));
            c.sq(3, fb_ea(c, SP, 0x60));
            c.lwc1(1, (c.g[V0][0] as u32));
            c.lwc1(2, fb_ea(c, V0, 0x4));
            c.lwc1(4, fb_ea(c, SP, 0x60));
            c.lwc1(5, fb_ea(c, SP, 0x64));
            c.mul_s(1, 1, 4);
            c.lwc1(0, fb_ea(c, V0, 0x8));
            c.mul_s(2, 2, 5);
            c.lwc1(3, fb_ea(c, SP, 0x68));
            c.mul_s(0, 0, 3);
            c.add_s(1, 1, 2);
            c.add_s(1, 1, 0);
            c.swc1(1, fb_ea(c, SP, 0x50));
            c.lwc1(0, fb_ea(c, V0, 0x10));
            c.lwc1(2, fb_ea(c, V0, 0x14));
            c.mul_s(0, 0, 4);
            c.lwc1(1, fb_ea(c, V0, 0x18));
            c.mul_s(2, 2, 5);
            c.mul_s(1, 1, 3);
            c.add_s(0, 0, 2);
            c.add_s(0, 0, 1);
            c.swc1(0, fb_ea(c, SP, 0x54));
            c.lwc1(1, fb_ea(c, V0, 0x20));
            c.lwc1(2, fb_ea(c, V0, 0x24));
            c.mul_s(1, 1, 4);
            c.lwc1(0, fb_ea(c, V0, 0x28));
            c.mul_s(2, 2, 5);
            c.swc1(6, fb_ea(c, SP, 0x5c));
            c.mul_s(0, 0, 3);
            c.add_s(1, 1, 2);
            c.add_s(1, 1, 0);
            c.swc1(1, fb_ea(c, SP, 0x58));
            c.lq(2, fb_ea(c, SP, 0x50));
            c.sq(2, fb_ea(c, SP, 0x40));
            c.mfc1(3, 7);
            c.lqc2(4, fb_ea(c, SP, 0x40));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, fb_ea(c, SP, 0x50));
            c.lq(2, fb_ea(c, SP, 0x50));
            c.sq(2, fb_ea(c, SP, 0x30));
            c.lqc2(3, fb_ea(c, A3, 0x30));
            c.lqc2(4, fb_ea(c, SP, 0x30));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, A3, 0x30));
            c.lwc1(0, fb_ea(c, S1, 0x68));
            c.lqc2(3, fb_ea(c, SP, 0x20));
            c.lqc2(5, (c.g[S2][0] as u32));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 2);
            c.lwc1(1, fb_ea(c, A1, 0x40));
            c.mul_s(0, 0, 2);
            c.mul_s(0, 0, 8);
            c.add_s(1, 1, 0);
            c.swc1(1, fb_ea(c, A1, 0x40));
            break 'b_3470b4;
        }
    }
    c.cycles += 6;
    c.lq(16, fb_ea(c, SP, 0xd0));
    c.lq(17, fb_ea(c, SP, 0xc0));
    c.lq(18, fb_ea(c, SP, 0xb0));
    c.g[RA][0] = c.ld(fb_ea(c, SP, 0xa0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(fb_ea(c, SP, 0xe0));
    c.pc = t;
    Ok(())
}

fn update_intersect_info(c: &mut Cpu) -> Flow {
    'b_34724c: {
        c.cycles += 9;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sq(18, fb_ea(c, SP, 0x70));
        c.sq(16, fb_ea(c, SP, 0x90));
        c.g[S2][0] = c.g[A1][0];
        c.sq(17, fb_ea(c, SP, 0x80));
        c.sd(fb_ea(c, SP, 0x60), c.g[RA][0]);
        c.g[V0][0] = c.lw(fb_ea(c, S2, 0x60));
        let t = (c.g[V0][0] as i64) < 0;
        c.g[A1][0] = c.g[A0][0];
        if t {
            break 'b_34724c;
        } else {
            c.cycles += 86;
            c.g[V1][0] = c.lw(fb_ea(c, A1, 0xc));
            c.g[S0][0] = sx((c.g[V0][0] as u32) >> 31);
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.g[S1][0] = c.lw(fb_ea(c, A1, 0x74));
            c.g[A0][0] = c.lh(fb_ea(c, V1, 0xc0));
            c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 1) as u32);
            c.g[V0][0] = c.lw(fb_ea(c, V1, 0xc4));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00347118);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00347118)?;
            c.g[V1][0] = 0x50;
            c.mult(0, 16, 16, 3);
            c.g[V1][0] = 0x500000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xec0));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
            c.lwc1(0, fb_ea(c, S0, 0x24));
            c.mfc1(4, 0);
            c.lqc2(4, (c.g[V1][0] as u32));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, fb_ea(c, SP, 0x20));
            c.lq(3, fb_ea(c, SP, 0x20));
            c.sq(3, fb_ea(c, SP, 0x10));
            c.lqc2(8, fb_ea(c, SP, 0x10));
            c.lqc2(4, (c.g[V0][0] as u32));
            c.lqc2(5, fb_ea(c, V0, 0x10));
            c.lqc2(6, fb_ea(c, V0, 0x20));
            c.lqc2(7, fb_ea(c, V0, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, fb_ea(c, SP, 0x20));
            c.lq(3, fb_ea(c, SP, 0x20));
            c.sq(3, (c.g[SP][0] as u32));
            c.lqc2(8, (c.g[S0][0] as u32));
            c.lqc2(4, (c.g[V0][0] as u32));
            c.lqc2(5, fb_ea(c, V0, 0x10));
            c.lqc2(6, fb_ea(c, V0, 0x20));
            c.lqc2(7, fb_ea(c, V0, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, fb_ea(c, SP, 0x30));
            c.lq(3, fb_ea(c, SP, 0x30));
            c.sq(3, fb_ea(c, SP, 0x20));
            c.lqc2(3, (c.g[S2][0] as u32));
            c.lqc2(4, fb_ea(c, SP, 0x20));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, SP, 0x30));
            c.lq(3, fb_ea(c, SP, 0x30));
            c.sq(3, fb_ea(c, SP, 0x10));
            c.lqc2(8, fb_ea(c, S0, 0x10));
            c.lqc2(4, (c.g[V0][0] as u32));
            c.lqc2(5, fb_ea(c, V0, 0x10));
            c.lqc2(6, fb_ea(c, V0, 0x20));
            c.lqc2(7, fb_ea(c, V0, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, fb_ea(c, SP, 0x40));
            c.lq(2, fb_ea(c, SP, 0x40));
            c.sq(2, fb_ea(c, SP, 0x30));
            c.lqc2(4, fb_ea(c, SP, 0x10));
            c.lqc2(5, (c.g[SP][0] as u32));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, fb_ea(c, SP, 0x50));
            c.lq(2, fb_ea(c, SP, 0x50));
            c.sq(2, fb_ea(c, SP, 0x40));
            c.lqc2(3, fb_ea(c, SP, 0x30));
            c.lqc2(4, fb_ea(c, SP, 0x40));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, SP, 0x50));
            c.lq(2, fb_ea(c, SP, 0x50));
            c.sq(2, fb_ea(c, SP, 0x20));
            c.lqc2(3, fb_ea(c, S2, 0x20));
            c.lqc2(4, fb_ea(c, SP, 0x20));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, S2, 0x20));
            c.lqc2(3, fb_ea(c, S2, 0x30));
            c.lqc2(4, (c.g[SP][0] as u32));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, fb_ea(c, S2, 0x30));
            break 'b_34724c;
        }
    }
    c.cycles += 6;
    c.lq(16, fb_ea(c, SP, 0x90));
    c.lq(17, fb_ea(c, SP, 0x80));
    c.lq(18, fb_ea(c, SP, 0x70));
    c.g[RA][0] = c.ld(fb_ea(c, SP, 0x60));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(fb_ea(c, SP, 0xa0));
    c.pc = t;
    Ok(())
}

fn intersect(c: &mut Cpu) -> Flow {
    'b_347584: {
        'b_3472a8: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
            c.g[T0][0] = c.g[T0][0] & 0x2;
            c.sq(16, fb_ea(c, SP, 0xf0));
            c.sq(17, fb_ea(c, SP, 0xe0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, fb_ea(c, SP, 0xd0));
            c.g[S1][0] = c.g[A1][0];
            c.sq(20, fb_ea(c, SP, 0xb0));
            c.g[S2][0] = c.g[A3][0];
            c.sq(19, fb_ea(c, SP, 0xc0));
            c.g[S4][0] = c.g[A2][0];
            c.sd(fb_ea(c, SP, 0xa0), c.g[RA][0]);
            let t = c.g[T0][0] != 0u64;
            c.swc1(20, fb_ea(c, SP, 0x100));
            if t {
                break 'b_3472a8;
            } else {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_347584;
                } else {
                    break 'b_3472a8;
                }
            }
        }
        'b_347580: {
            c.cycles += 93;
            c.g[V1][0] = c.lw(fb_ea(c, S0, 0x50));
            c.g[A0][0] = c.lw(fb_ea(c, S0, 0x7c));
            c.g[A2][0] = c.lw(fb_ea(c, S0, 0x84));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.g[A1][0] = c.lw(fb_ea(c, S0, 0x80));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            c.sh((c.g[SP][0] as u32), c.g[V0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
            c.sw(fb_ea(c, SP, 0x4), c.g[A2][0]);
            c.sw(fb_ea(c, SP, 0xc), c.g[A0][0]);
            c.sw(fb_ea(c, SP, 0x10), c.g[A1][0]);
            c.sh(fb_ea(c, SP, 0x2), c.g[V1][0]);
            c.g[V1][0] = c.lw(fb_ea(c, S0, 0xc));
            c.g[A0][0] = c.lh(fb_ea(c, V1, 0xc0));
            c.g[V0][0] = c.lw(fb_ea(c, V1, 0xc4));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003472ec);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x003472ec)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V1][0] = sx(fb_ea(c, SP, 0x20));
            c.lqc2(1, (c.g[S3][0] as u32));
            c.lqc2(2, fb_ea(c, S3, 0x10));
            c.lqc2(3, fb_ea(c, S3, 0x20));
            c.lqc2(4, fb_ea(c, S3, 0x30));
            c.sqc2(1, (c.g[V1][0] as u32));
            c.sqc2(2, fb_ea(c, V1, 0x10));
            c.sqc2(3, fb_ea(c, V1, 0x20));
            c.sqc2(4, fb_ea(c, V1, 0x30));
            c.lwc1(3, fb_ea(c, SP, 0x24));
            c.g[A2][0] = sx(fb_ea(c, SP, 0x60));
            c.lwc1(2, fb_ea(c, SP, 0x30));
            c.g[A1][0] = c.g[V1][0];
            c.swc1(3, fb_ea(c, SP, 0x30));
            c.lwc1(3, fb_ea(c, SP, 0x28));
            c.lwc1(1, fb_ea(c, SP, 0x40));
            c.swc1(3, fb_ea(c, SP, 0x40));
            c.lwc1(0, fb_ea(c, SP, 0x44));
            c.lq(2, fb_ea(c, SP, 0x50));
            c.lwc1(3, fb_ea(c, SP, 0x38));
            c.swc1(0, fb_ea(c, SP, 0x38));
            c.sq(2, fb_ea(c, SP, 0x70));
            c.swc1(2, fb_ea(c, SP, 0x24));
            c.swc1(1, fb_ea(c, SP, 0x28));
            c.swc1(3, fb_ea(c, SP, 0x44));
            c.g[AT][0] = 0xffffffffbf800000;
            c.mtc1(1, 0);
            c.mfc1(4, 0);
            c.lqc2(4, fb_ea(c, SP, 0x70));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, fb_ea(c, SP, 0x80));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 20);
            c.lq(2, fb_ea(c, SP, 0x80));
            c.sq(2, fb_ea(c, SP, 0x60));
            c.sw(fb_ea(c, SP, 0x6c), 0u64);
            c.lqc2(8, fb_ea(c, SP, 0x60));
            c.lqc2(4, (c.g[V1][0] as u32));
            c.lqc2(5, fb_ea(c, V1, 0x10));
            c.lqc2(6, fb_ea(c, V1, 0x20));
            c.lqc2(7, fb_ea(c, V1, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, fb_ea(c, A2, 0x20));
            c.lq(2, fb_ea(c, SP, 0x80));
            c.g[A0][0] = c.lw(fb_ea(c, S0, 0x18));
            c.sq(2, fb_ea(c, SP, 0x60));
            c.sq(2, fb_ea(c, SP, 0x70));
            c.swc1(20, fb_ea(c, A2, 0xc));
            c.lq(2, fb_ea(c, SP, 0x60));
            c.g[V1][0] = c.lw(fb_ea(c, S1, 0x50));
            c.sq(2, fb_ea(c, SP, 0x50));
            c.lwc1(12, fb_ea(c, A0, 0x84));
            c.g[A0][0] = c.lh(fb_ea(c, V1, 0x8));
            c.div_s(12, 20, 12);
            c.g[V0][0] = c.lw(fb_ea(c, V1, 0xc));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003473e4);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x003473e4)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[T0][0] = c.g[S2][0];
            c.g[A2][0] = 0x1;
            c.g[A3][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x003473fc);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x0032c0f8, 0x003473fc)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(fb_ea(c, S1, 0x50));
            c.g[A0][0] = c.lh(fb_ea(c, V0, 0x10));
            c.g[V1][0] = c.lw(fb_ea(c, V0, 0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00347414);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00347414)?;
            let t = (c.g[S2][0] as i64) <= 0;
            c.g[V0][0] = 0x500000;
            if t {
                break 'b_347580;
            } else {
                c.cycles += 9;
                c.mov_s(12, 20);
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
                c.g[A2][0] = c.g[S4][0];
                c.g[A3][0] = 0xffffffffffffffff;
                c.g[T3][0] = 0x2;
                c.g[T2][0] = 0x1;
                c.g[T1][0] = 0xc;
                c.g[A1][0] = c.g[S2][0];
                'l_347440: loop {
                    c.cycles += 80;
                    c.sw(fb_ea(c, A2, 0x54), 0u64);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.sw(fb_ea(c, A2, 0x50), 0u64);
                    c.sw(fb_ea(c, A2, 0x58), 0u64);
                    c.sw(fb_ea(c, A2, 0x5c), c.g[A3][0]);
                    c.sw(fb_ea(c, A2, 0x60), c.g[A3][0]);
                    c.sw(fb_ea(c, A2, 0x64), c.g[A3][0]);
                    c.sw(fb_ea(c, A2, 0x44), c.g[T3][0]);
                    c.sw(fb_ea(c, A2, 0x4c), c.g[A3][0]);
                    c.lwc1(4, (c.g[A2][0] as u32));
                    c.lq(2, (c.g[T0][0] as u32));
                    c.lwc1(3, fb_ea(c, A2, 0x4));
                    c.sq(2, fb_ea(c, A2, 0x20));
                    c.lwc1(2, fb_ea(c, A2, 0x8));
                    c.lq(2, (c.g[T0][0] as u32));
                    c.sw(fb_ea(c, A2, 0x68), 0u64);
                    c.sq(2, fb_ea(c, A2, 0x30));
                    c.sw(fb_ea(c, A2, 0x6c), 0u64);
                    c.sw(fb_ea(c, A2, 0x70), 0u64);
                    c.lwc1(1, fb_ea(c, A2, 0xc));
                    c.g[V0][0] = c.lw(fb_ea(c, S0, 0x18));
                    c.lwc1(0, fb_ea(c, V0, 0x84));
                    c.swc1(1, fb_ea(c, SP, 0x7c));
                    c.mul_s(2, 2, 0);
                    c.swc1(0, fb_ea(c, SP, 0x80));
                    c.mul_s(4, 4, 0);
                    c.swc1(0, fb_ea(c, SP, 0x84));
                    c.mul_s(3, 3, 0);
                    c.swc1(0, fb_ea(c, SP, 0x88));
                    c.swc1(2, fb_ea(c, SP, 0x78));
                    c.swc1(4, fb_ea(c, SP, 0x70));
                    c.swc1(3, fb_ea(c, SP, 0x74));
                    c.swc1(12, fb_ea(c, SP, 0x8c));
                    c.lqc2(8, fb_ea(c, SP, 0x70));
                    c.lqc2(4, (c.g[S3][0] as u32));
                    c.lqc2(5, fb_ea(c, S3, 0x10));
                    c.lqc2(6, fb_ea(c, S3, 0x20));
                    c.lqc2(7, fb_ea(c, S3, 0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, fb_ea(c, SP, 0x90));
                    c.lq(2, fb_ea(c, SP, 0x90));
                    c.sq(2, fb_ea(c, SP, 0x60));
                    c.sq(2, (c.g[A2][0] as u32));
                    c.lqc2(8, fb_ea(c, A2, 0x10));
                    c.lqc2(4, (c.g[S3][0] as u32));
                    c.lqc2(5, fb_ea(c, S3, 0x10));
                    c.lqc2(6, fb_ea(c, S3, 0x20));
                    c.lqc2(7, fb_ea(c, S3, 0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, fb_ea(c, SP, 0x70));
                    c.lq(2, fb_ea(c, SP, 0x70));
                    c.sw(fb_ea(c, A2, 0x44), c.g[T2][0]);
                    c.sq(2, fb_ea(c, A2, 0x10));
                    c.sq(2, fb_ea(c, SP, 0x60));
                    c.g[V1][0] = c.lw(fb_ea(c, S0, 0x18));
                    c.sw(fb_ea(c, A2, 0x50), c.g[V1][0]);
                    c.g[V0][0] = c.lw(fb_ea(c, S0, 0x18));
                    c.g[V1][0] = c.lw(fb_ea(c, V0, 0x80));
                    c.g[A0][0] = c.lw(fb_ea(c, V1, 0x4));
                    c.sw(fb_ea(c, A2, 0x48), 0u64);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.sw(fb_ea(c, A2, 0x5c), c.g[A0][0]);
                    c.mult(0, 4, 4, 9);
                    c.lq(2, (c.g[T0][0] as u32));
                    c.sq(2, fb_ea(c, A2, 0x20));
                    c.lq(3, (c.g[T0][0] as u32));
                    c.sq(3, fb_ea(c, A2, 0x30));
                    c.g[V0][0] = c.lw(fb_ea(c, S0, 0x18));
                    c.g[V1][0] = c.lw(fb_ea(c, V0, 0x88));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = c.lh(fb_ea(c, A0, 0x1a));
                    c.sw(fb_ea(c, A2, 0x4c), c.g[V0][0]);
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx(fb_ea(c, A2, 0x80));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00347440);
                        }
                        continue 'l_347440;
                    } else {
                        break 'b_347580;
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S2][0];
        break 'b_347584;
    }
    c.cycles += 9;
    c.lq(16, fb_ea(c, SP, 0xf0));
    c.lq(17, fb_ea(c, SP, 0xe0));
    c.lq(18, fb_ea(c, SP, 0xd0));
    c.lq(19, fb_ea(c, SP, 0xc0));
    c.lq(20, fb_ea(c, SP, 0xb0));
    c.g[RA][0] = c.ld(fb_ea(c, SP, 0xa0));
    c.lwc1(20, fb_ea(c, SP, 0x100));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(fb_ea(c, SP, 0x110));
    c.pc = t;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{fill, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Fixture layout in the test data area (clear of the fakes' log at 0x900000).
    const NODE: u32 = DATA + 0x20_0000;
    const INFO: u32 = DATA + 0x20_0100;
    const FORCE: u32 = DATA + 0x20_0200;
    const VT: u32 = DATA + 0x20_0300;
    const VT2: u32 = DATA + 0x20_0400;
    const DEF: u32 = DATA + 0x20_0500;
    const DEF_V: u32 = DATA + 0x20_0600;
    const OUT: u32 = DATA + 0x20_1000;
    const FB_CTRL: u32 = DATA + 0x20_2000;
    const FB_MAGIC: u32 = 0x6f62_3630;
    static FB_PREV_MESH: AtomicUsize = AtomicUsize::new(0);

    /// A vtable slot at `vt + off`: a small delta and the function `fake`.
    fn fb_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, fake: u32) {
        c.bus.write16(vt + off, (r.next_u32() as u16) & !3);
        c.bus.write32(vt + off + 4, fake);
    }

    fn apply_force_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x0602_0043, 1);
        fill(c, &mut r, NODE, 0x40);
        fill(c, &mut r, INFO, 0x40);
        fill(c, &mut r, FORCE, 0x40);
        c.bus.write32(NODE + 0xc, VT);
        fb_vtable(c, &mut r, VT, 0xc0, fakes::FAKE);
        c.bus.write32(NODE + 0x74, 0x40);
        c.bus.write32(NODE + 0x80, OUT);
        let weight = if seed % 4 == 0 {
            0x8000_0000
        } else {
            r.next_u32() & 0x3f
        };
        c.bus.write32(INFO + 0x60, weight);
        c.g[A0][0] = sx(NODE);
        c.g[A1][0] = sx(INFO);
        c.g[A2][0] = sx(FORCE);
        c.pc = 0x0034_6e38;
    }

    fn update_intersect_info_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x0602_0042, 1);
        fill(c, &mut r, NODE, 0x40);
        fill(c, &mut r, INFO, 0x40);
        c.bus.write32(NODE + 0xc, VT);
        fb_vtable(c, &mut r, VT, 0xc0, fakes::FAKE);
        c.bus.write32(NODE + 0x74, OUT);
        let weight = if seed % 4 == 0 {
            0x8000_0000
        } else {
            r.next_u32() & 0x3e
        };
        c.bus.write32(INFO + 0x60, weight);
        c.g[A0][0] = sx(NODE);
        c.g[A1][0] = sx(INFO);
        c.pc = 0x0034_70d0;
    }

    /// Gated IsMeshInt (`0x32c0f8`): logs the call and returns the count the setup wrote.
    fn fb_mesh_fake(c: &mut Cpu) {
        if c.bus.read32(FB_CTRL) != FB_MAGIC {
            return fakes::fall_back(c, &FB_PREV_MESH, ee::s10::f_0032c0f8__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_c0f8, ra);
        if c.pc == ra {
            c.g[V0][0] = c.bus.read32(FB_CTRL + 4) as i32 as i64 as u64;
        }
    }

    fn fb_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0032C0F8),
            fb_mesh_fake,
            &FB_PREV_MESH,
        );
    }

    fn intersect_setup(c: &mut Cpu, seed: u32) {
        fb_install();
        let mut r = fakes::machine(c, seed, 0x0602_0039, 5);
        fill(c, &mut r, NODE, 0x40);
        fill(c, &mut r, INFO, 0x40);
        fill(c, &mut r, OUT, 0x80);
        fill(c, &mut r, DEF, 0x40);
        c.bus.write32(NODE + 0xc, VT);
        fb_vtable(c, &mut r, VT, 0xc0, fakes::FAKE);
        c.bus.write32(NODE + 0x18, DEF);
        c.bus.write32(DEF + 0x80, DEF_V);
        c.bus.write32(DEF + 0x88, 0x40);
        c.bus.write32(DEF_V + 4, DATA + 0x100);
        c.bus.write32(INFO + 0x50, VT2);
        fb_vtable(c, &mut r, VT2, 0x8, fakes::FAKE);
        fb_vtable(c, &mut r, VT2, 0x10, fakes::FAKE + 0x10);
        c.bus.write32(FB_CTRL, FB_MAGIC);
        let count = [0u32, 1, 2, 3, u32::MAX][((seed / 7) % 5) as usize];
        c.bus.write32(FB_CTRL + 4, count);
        c.g[A0][0] = sx(NODE);
        c.g[A1][0] = sx(INFO);
        c.g[A2][0] = sx(OUT);
        c.g[A3][0] = r.next_u32() as u64;
        let early = if seed % 7 == 0 { 0 } else { 2 };
        c.g[T0][0] = (r.next_u32() as u64 & !2) | early;
        c.pc = 0x0034_7268;
    }

    #[test]
    fn flex_bridge_apply_force_matches() {
        fakes::covered(
            ee::s11::f_00346e38__generated,
            flex_bridge_apply_force,
            120,
            &apply_force_setup,
        );
    }

    #[test]
    fn flex_bridge_update_intersect_info_matches() {
        fakes::covered(
            ee::s11::f_003470d0__generated,
            flex_bridge_update_intersect_info,
            120,
            &update_intersect_info_setup,
        );
    }

    #[test]
    fn flex_bridge_intersect_matches() {
        fakes::covered(
            ee::s11::f_00347268__generated,
            flex_bridge_intersect,
            120,
            &intersect_setup,
        );
    }
}
