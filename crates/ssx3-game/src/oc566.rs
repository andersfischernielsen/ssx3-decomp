//! Staging module oc566 (wave wave232): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc566`.

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
    (0x0033_00f8, vec3_neg_dot, "Vec3_NegDot"),
    (
        0x0032_e5e8,
        coll_segment_collide_triangle_2sided,
        "CollSegment_CollideTriangle2Sided",
    ),
];

/// `float Vec3_NegDot(const Vec3* a, const Vec3* b)` (0x3300f8): the negated
/// dot product of the vectors at `a0` and `a1`, computed on VU0 and returned
/// in f0.
pub fn vec3_neg_dot(c: &mut Cpu) {
    c.cycles += 11;
    c.lqc2(3, c.g[A0][0] as u32);
    c.lqc2(5, c.g[A1][0] as u32);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 0);
    c.neg_s(0, 0);
    ret(c);
}

/// `int CollSegment_CollideTriangle2Sided(seg, ..., t0 = triangle vector)`
/// (0x32e5e8). The segment's vector at `seg+0x70` is dotted with the vector
/// at `t0` on VU0. When that dot is below zero, the vector at `t0` is
/// negated (copied to the stack) and passed on; the callback at `+0x24` of
/// the object at `seg+0x50` is then called with `a0 = seg + lh(obj+0x20)`.
pub fn coll_segment_collide_triangle_2sided(c: &mut Cpu) {
    let frame = Frame::open(c, 0x30);
    guest(c, |c| coll_segment_body(c, frame));
}

fn coll_segment_body(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 17;
    c.g[T4][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);
    c.mtc1(0, 0);
    c.lqc2(3, (c.g[T4][0] as u32).wrapping_add(0x70));
    c.lqc2(5, c.g[T0][0] as u32);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 1);
    c.c_lt_s(1, 0);
    let negative = c.fcc();
    c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x50));
    if negative {
        c.cycles += 13;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.mfc1(2, 0);
        c.lqc2(4, c.g[T0][0] as u32);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        let spill = frame.at(c, 0x10);
        c.sqc2(5, spill);
        c.lq(2, spill);
        c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[T0][0] = c.g[SP][0];
        let copy = frame.at(c, 0);
        c.sq(2, copy);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[A0][0] as u32));
    }
    c.cycles += 5;
    let target = c.g[V0][0] as u32;
    call_ptr(c, target, 0x0032_e678)?;
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `void SphereTree_SortChildOrder(SphereTree* node, Vec4* dir)` (0x32db40).
/// Four child centres at `node+0..0x40` are projected on `dir` (scaled by
/// -1000 on VU0) and truncated to int keys; the keys are stored at
/// `node+0x9c` (index order) and `node+0xb8` (descending), then sorted
/// through `ShellSortDesc_KeyVal` (0x32da40) on the stack.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn sphere_tree_sort_child_order(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    guest(c, |c| sort_child_order_body(c, frame));
}

fn sort_child_order_body(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 20;
    c.g[AT][0] = 0xffff_ffff_c47a_0000;
    c.mtc1(1, 0);
    frame.save_ra(c, 0x40);
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9c));
    c.mfc1(2, 0);
    c.lqc2(4, c.g[A1][0] as u32);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let spill = frame.at(c, 0x30);
    c.sqc2(5, spill);
    c.g[T2][0] = 0;
    c.lq(2, spill);
    c.g[A1][0] = c.g[A0][0];
    c.g[T4][0] = 7;
    c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb8));
    let scaled = frame.at(c, 0x20);
    c.sq(2, scaled);
    c.g[T1][0] = c.g[A2][0];
    c.g[T0][0] = sx(frame.at(c, 0x1c));
    c.g[A3][0] = sx(frame.at(c, 0));
    loop {
        c.cycles += 25;
        c.lqc2(3, c.g[A1][0] as u32);
        c.lqc2(5, scaled);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.g[V1][0] = sx((c.g[T4][0] as u32).wrapping_sub(c.g[T2][0] as u32));
        c.cvt_w_s(1, 0);
        c.mfc1(4, 1);
        c.sw(c.g[T1][0] as u32, c.g[T2][0]);
        c.sw(c.g[T3][0] as u32, c.g[V1][0]);
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x4));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
        c.sw(c.g[A3][0] as u32, c.g[A0][0]);
        c.sw(c.g[T0][0] as u32, c.g[V0][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4));
        c.g[V0][0] = ((c.g[T2][0] as i64) < 4i64) as u64;
        let more = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_db90);
        }
    }
    c.cycles += 6;
    c.g[A0][0] = 0x8;
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, 0x0032_da40, 0x0032_dc00)?;
    frame.restore_ra(c, 0x40);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    #[test]
    fn vec3_neg_dot_matches() {
        check(ssx3_ee::s11::f_003300f8__generated, vec3_neg_dot, 200, &|c, seed| {
            let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0033_00f8;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x20);
            for k in 0..4u32 {
                c.bus.write32(DATA + 4 * k, r.f32(100.0));
                c.bus.write32(DATA + 0x20 + 4 * k, r.f32(100.0));
            }
        });
    }

    /// The callback the segment test calls: a real leaf that returns 0.
    const LEAF_RET0: u32 = 0x0014_06c0;

    #[test]
    fn coll_segment_collide_triangle_2sided_matches() {
        check(
            ssx3_ee::s11::f_0032e5e8__generated,
            coll_segment_collide_triangle_2sided,
            200,
            &|c, seed| {
                let mut r = Rng(0x2c6b_9d31_5a7e_1f03 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0032_e5e8;
                let seg = DATA + 0x100;
                let tri = DATA + 0x200;
                let obj = DATA + 0x300;
                c.g[A0][0] = sx(seg);
                c.g[T0][0] = sx(tri);
                for k in 0..4u32 {
                    c.bus.write32(seg + 0x70 + 4 * k, r.f32(10.0));
                    c.bus.write32(tri + 4 * k, r.f32(10.0));
                }
                c.bus.write32(seg + 0x50, obj);
                c.bus.write16(obj + 0x20, r.next_u32() as u16);
                c.bus.write32(obj + 0x24, LEAF_RET0);
            },
        );
    }
}
