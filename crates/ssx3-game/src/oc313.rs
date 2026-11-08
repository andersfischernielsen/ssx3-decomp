//! Staging wave 213 (oc313): hand-written replacements for functions with no
//! C++ in the reference, ported with Ghidra's decompilation as the readable
//! aid. Each reproduces the generated body's registers, cycles, memory and
//! call order.

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
    (
        0x0015_f780,
        rider_camera_target_v23,
        "RiderCameraTarget_v23",
    ),
    (
        0x0035_c0e8,
        rail_modifier_build_rail_nodes,
        "RailModifier_BuildRailNodes",
    ),
];

// ---------------------------------------------------------------------------
// RiderCameraTarget_v23 (0x15f780)

/// `RiderCameraTarget::mRider` (the rider object).
pub(crate) const RCT_V23_RIDER: u32 = 0x4;
/// `Rider + 0x788`: the landing record (velocity at +0x10, normal at +0x20).
pub(crate) const RCT_V23_LANDING: u32 = 0x788;
pub(crate) const RCT_V23_LANDING_VELOCITY: u32 = 0x10;
pub(crate) const RCT_V23_LANDING_NORMAL: u32 = 0x20;
/// `.lit4` at gp-0x6c50: minimum |n|·|v| (0.001) for a meaningful angle.
pub(crate) const RCT_V23_MIN_LEN: u32 = 0x6c50;
/// `.lit4` at gp-0x6c4c: pi/2.
pub(crate) const RCT_V23_HALF_PI: u32 = 0x6c4c;
/// `Math_Asin` (0x31c128, another shard: through the table).
pub(crate) const RCT_V23_MATH_ASIN: u32 = 0x0031_c128;

/// gp-relative address `gp - off`.
#[inline]
fn rct_v23_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// The compiled inline dot product of the quadwords at `a` and `b`: leaves
/// the result in vf4 (lane w), with vf3/vf5/vf6 and ACC as the original.
#[inline]
fn rct_v23_dot(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `float RiderCameraTarget_v23(RiderCameraTarget* self)` (0x15f780): the
/// landing angle of the rider. Takes the rider's landing normal `n` and
/// landing velocity `v`; if both have positive squared length and
/// |n|·|v| >= 0.001, returns `pi/2 - asin(clamp(n·(-v) / (|n||v|), -1, 1))`
/// (the angle between the surface and the incoming direction), else 0.
pub fn rider_camera_target_v23(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        c.mtc1(0, 1);
        frame.save_ra(c, 0x40);
        c.g[A1][0] = 0;
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(RCT_V23_RIDER)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(RCT_V23_LANDING)).lw(c);
        let landing = c.g[A0][0] as u32;
        // sp+0x00: the normal, sp+0x10: the velocity.
        c.lq(2, landing.wrapping_add(RCT_V23_LANDING_NORMAL));
        c.sq(2, frame.at(c, 0));
        c.lq(3, landing.wrapping_add(RCT_V23_LANDING_VELOCITY));
        c.sq(3, frame.at(c, 0x10));
        let (n, v) = (frame.at(c, 0), frame.at(c, 0x10));
        rct_v23_dot(c, n, n);
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.cycles += 13;
            rct_v23_dot(c, v, v);
            c.qmfc2(2, 4);
            c.mtc1(2, 0);
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.g[A1][0] = 1;
            }
        }
        c.cycles += 2;
        if c.g[A1][0] == 0 {
            // No usable vectors: 0.
            c.cycles += 1;
            c.mtc1(0, 0);
            c.cycles += 1;
            frame.restore_ra(c, 0x40);
        } else {
            c.cycles += 34;
            // sp+0x20 (via sp+0x30): -v.
            c.g[AT][0] = sx(0xbf80_0000);
            c.mtc1(1, 3);
            c.mfc1(2, 3);
            c.lqc2(4, v);
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x30));
            let lit = rct_v23_gp(c, RCT_V23_MIN_LEN);
            c.lwc1(2, lit);
            c.lq(2, frame.at(c, 0x30));
            c.sq(2, frame.at(c, 0x20));
            let neg_v = frame.at(c, 0x20);
            rct_v23_dot(c, neg_v, neg_v);
            c.qmfc2(2, 4);
            c.mtc1(2, 1);
            rct_v23_dot(c, n, n);
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.mul_s(0, 0, 1);
            c.sqrt_s(1, 0);
            c.c_lt_s(1, 2);
            c.mtc1(0, 0);
            let too_short = c.fcc();
            frame.restore_ra(c, 0x40);
            if !too_short {
                c.cycles += 14;
                rct_v23_dot(c, n, neg_v);
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.div_s(12, 0, 1);
                // clamp(cos, -1, 1) (f3 = -1)
                c.c_le_s(3, 12);
                if !c.fcc() {
                    c.mov_s(12, 3);
                } else {
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.min_s(12, 12, 0);
                }
                c.cycles += 5;
                call_ptr(c, RCT_V23_MATH_ASIN, 0x0015_f8e8)?;
                let lit = rct_v23_gp(c, RCT_V23_HALF_PI);
                c.lwc1(1, lit);
                c.sub_s(0, 1, 0);
                c.cycles += 1;
                frame.restore_ra(c, 0x40);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RailModifier_BuildRailNodes (0x35c0e8)

/// `cRailModifier + 0x34`: the starting rail node index (< 0: none).
pub(crate) const RM_BUILD_NODE: u32 = 0x34;
/// `cRailModifier + 0x40`: the rail object (matrix at +0x10, node table
/// header at +0x80, scale at +0x84).
pub(crate) const RM_BUILD_RAIL: u32 = 0x40;
/// `cRailModifier + 0x44`: the allocated 0x10-byte link.
pub(crate) const RM_BUILD_LINK: u32 = 0x44;
/// `cRailModifier + 0x50`: the built transform (4 rows).
pub(crate) const RM_BUILD_XFORM: u32 = 0x50;
pub(crate) const RM_BUILD_RAIL_MATRIX: u32 = 0x10;
pub(crate) const RM_BUILD_RAIL_TABLE: u32 = 0x80;
pub(crate) const RM_BUILD_RAIL_SCALE: u32 = 0x84;
/// `EA_MemAllocNamed` / `cMemMan_alloc` (0x317d70, through the table).
pub(crate) const RM_BUILD_MEM_ALLOC: u32 = 0x0031_7d70;

/// `lqc2 vf1..vf4` from `src`, then `sqc2 vf1..vf4` to `dst` (a 4x4 matrix
/// copy through VU0).
#[inline]
fn rm_build_copy_matrix(c: &mut Cpu, src: u32, dst: u32) {
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, src.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, dst.wrapping_add(0x10 * i));
    }
}

/// vf12..vf15 = rows vf8..vf11 times the matrix vf4..vf7 (the compiled
/// matrix multiply).
#[inline]
fn rm_build_mul_matrix(c: &mut Cpu) {
    for row in 0..4u8 {
        let r = 8 + row;
        c.vu0().mula(0b1111, 4, Src::Bc(r, X));
        c.vu0().madda(0b1111, 5, Src::Bc(r, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(r, Z));
        c.vu0().madd(0b1111, 12 + row, 7, Src::Bc(r, W));
    }
}

/// `lqc2 vf4..vf7` from `a`, `lqc2 vf8..vf11` from `b`, multiply, store
/// vf12..vf15 at `out`, then copy `out` -> `tmp` -> `b`.
#[inline]
fn rm_build_concat(c: &mut Cpu, a: u32, b: u32, out: u32, tmp: u32) {
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, a.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.lqc2(8 + i as usize, b.wrapping_add(0x10 * i));
    }
    rm_build_mul_matrix(c);
    for i in 0..4u32 {
        c.sqc2(12 + i as usize, out.wrapping_add(0x10 * i));
    }
    rm_build_copy_matrix(c, out, tmp);
    rm_build_copy_matrix(c, tmp, b);
}

/// `void cRailModifier_buildXform(cRailModifier* self)` (0x35c0e8): builds
/// the modifier's transform at +0x50. Without a start node it is the rail's
/// own matrix; otherwise the start node's matrix (translation scaled by the
/// rail's scale), concatenated with each following node's matrix (same
/// scaling) down the node chain (`next == -1` ends it), then with the rail's
/// matrix. The 3x3 part is then transposed and the translation replaced by
/// `-t` through the transposed rows (an inverse rigid transform), and a
/// 0x10-byte link `{.., .., 2, self}` is allocated into +0x44.
pub fn rail_modifier_build_rail_nodes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save_ra(c, 0x110);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = Addr(this.wrapping_add(RM_BUILD_RAIL)).lw(c);
        c.g[A0][0] = Addr(this.wrapping_add(RM_BUILD_NODE)).lw(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(RM_BUILD_RAIL_TABLE)).lw(c);
        c.g[T2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lw(c);
        let s = sp(c);
        if (c.g[A0][0] as i64) < 0 {
            // No node: the rail's matrix.
            c.cycles += 13;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(RM_BUILD_RAIL_MATRIX));
            c.g[V0][0] = sx(this.wrapping_add(RM_BUILD_XFORM));
            let (src, dst) = (c.g[V1][0] as u32, c.g[V0][0] as u32);
            rm_build_copy_matrix(c, src, dst);
            c.g[T1][0] = c.g[V0][0];
            c.g[T7][0] = 0x0049_0000;
        } else {
            c.cycles += 46;
            // sp+0x00: the scale vector {s, s, s, 1}.
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(RM_BUILD_RAIL_SCALE));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T2][0] as u32));
            c.swc1(0, s.wrapping_add(0x8));
            c.g[V1][0] = sx(this.wrapping_add(RM_BUILD_XFORM));
            c.swc1(1, s.wrapping_add(0xc));
            c.swc1(0, s);
            c.swc1(0, s.wrapping_add(0x4));
            let node = c.g[A0][0] as u32;
            c.g[V0][0] = Addr(node.wrapping_add(0xc)).lw(c);
            let (src, dst) = (c.g[V0][0] as u32, c.g[V1][0] as u32);
            rm_build_copy_matrix(c, src, dst);
            // Row 3 (translation) *= scale.
            c.lq(2, this.wrapping_add(0x80));
            c.g[T1][0] = c.g[V1][0];
            c.lwc1(0, s);
            c.g[T4][0] = sx(s.wrapping_add(0x50));
            c.sq(2, s.wrapping_add(0x20));
            c.g[T6][0] = sx(s.wrapping_add(0x10));
            c.lwc1(4, s.wrapping_add(0x4));
            c.g[V1][0] = u64::MAX;
            c.lwc1(1, s.wrapping_add(0x20));
            c.lwc1(2, s.wrapping_add(0x24));
            c.mul_s(1, 1, 0);
            c.lwc1(3, s.wrapping_add(0x28));
            c.mul_s(2, 2, 4);
            c.lwc1(5, s.wrapping_add(0x8));
            c.lwc1(0, s.wrapping_add(0x2c));
            c.lwc1(4, s.wrapping_add(0xc));
            c.mul_s(3, 3, 5);
            c.swc1(1, s.wrapping_add(0x10));
            c.mul_s(0, 0, 4);
            c.swc1(2, s.wrapping_add(0x14));
            c.swc1(3, s.wrapping_add(0x18));
            c.swc1(0, s.wrapping_add(0x1c));
            c.lq(2, s.wrapping_add(0x10));
            c.sq(2, this.wrapping_add(0x80));
            c.g[A0][0] = Addr(node).lw(c);
            c.g[T7][0] = 0x0049_0000;
            if c.g[A0][0] != c.g[V1][0] {
                c.cycles += 6;
                c.g[T0][0] = c.g[T1][0];
                c.g[A3][0] = sx(s.wrapping_add(0x30));
                c.g[A2][0] = sx(s.wrapping_add(0xd0));
                c.g[T3][0] = sx(s.wrapping_add(0x70));
                c.g[T5][0] = u64::MAX;
                c.g[A1][0] = sx((c.g[A0][0] as u32) << 4);
                loop {
                    // Next node: its matrix (sp+0x70) with the translation
                    // scaled (sp+0xa0), then xform = node * xform.
                    c.cycles += 78;
                    c.lwc1(6, s);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.lwc1(7, s.wrapping_add(0x4));
                    let node = c.g[A1][0] as u32;
                    c.g[A0][0] = Addr(node.wrapping_add(0xc)).lw(c);
                    c.lwc1(5, s.wrapping_add(0x8));
                    let m = c.g[A0][0] as u32;
                    c.lq(3, m);
                    c.lwc1(4, s.wrapping_add(0xc));
                    c.sq(3, s.wrapping_add(0xb0));
                    c.sq(3, s.wrapping_add(0x70));
                    c.lq(2, m.wrapping_add(0x10));
                    c.sq(2, s.wrapping_add(0xb0));
                    c.sq(2, s.wrapping_add(0x80));
                    c.lq(3, m.wrapping_add(0x20));
                    c.sq(3, s.wrapping_add(0xb0));
                    c.sq(3, s.wrapping_add(0x90));
                    c.lq(2, m.wrapping_add(0x30));
                    c.sq(2, s.wrapping_add(0xc0));
                    c.lwc1(1, s.wrapping_add(0xc0));
                    c.lwc1(2, s.wrapping_add(0xc4));
                    c.lwc1(3, s.wrapping_add(0xc8));
                    c.mul_s(1, 1, 6);
                    c.lwc1(0, s.wrapping_add(0xcc));
                    c.mul_s(2, 2, 7);
                    c.mul_s(3, 3, 5);
                    c.mul_s(0, 0, 4);
                    c.swc1(1, s.wrapping_add(0xb0));
                    c.swc1(2, s.wrapping_add(0xb4));
                    c.swc1(3, s.wrapping_add(0xb8));
                    c.swc1(0, s.wrapping_add(0xbc));
                    c.lq(2, s.wrapping_add(0xb0));
                    c.sq(2, s.wrapping_add(0xa0));
                    let (t3, t0, a2, a3) = (
                        c.g[T3][0] as u32,
                        c.g[T0][0] as u32,
                        c.g[A2][0] as u32,
                        c.g[A3][0] as u32,
                    );
                    rm_build_concat(c, t3, t0, a2, a3);
                    c.g[A0][0] = Addr(node).lw(c);
                    let more = c.g[A0][0] != c.g[T5][0];
                    c.g[A1][0] = sx((c.g[A0][0] as u32) << 4);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_c210);
                    }
                }
            }
            // xform = rail matrix * xform.
            c.cycles += 47;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(RM_BUILD_RAIL)).lw(c);
            c.g[V1][0] = c.g[T6][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(RM_BUILD_RAIL_MATRIX));
            let (v0, t1, t4, v1) = (
                c.g[V0][0] as u32,
                c.g[T1][0] as u32,
                c.g[T4][0] as u32,
                c.g[V1][0] as u32,
            );
            rm_build_concat(c, v0, t1, t4, v1);
        }
        // Transpose the 3x3 part.
        c.cycles += 55;
        let this = c.g[S0][0] as u32;
        let s = sp(c);
        let f = |o: u32| this.wrapping_add(o);
        c.lwc1(3, f(0x54));
        c.g[A1][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x15a0));
        c.lwc1(2, f(0x60));
        c.g[A0][0] = 0x10;
        c.swc1(3, f(0x60));
        c.g[A2][0] = 0x2000_0000;
        c.lwc1(3, f(0x58));
        c.g[A3][0] = 0;
        c.lwc1(1, f(0x70));
        c.swc1(3, f(0x70));
        c.lq(2, f(0x80));
        c.swc1(2, f(0x54));
        c.lwc1(3, f(0x68));
        c.lwc1(0, f(0x74));
        c.sq(2, s.wrapping_add(0x10));
        c.swc1(0, f(0x68));
        c.swc1(1, f(0x58));
        c.swc1(3, f(0x74));
        // Translation: -t (sp+0x20), w = 0, through the transposed rows.
        c.g[AT][0] = sx(0xbf80_0000);
        c.mtc1(1, 0);
        c.mfc1(2, 0);
        c.lqc2(4, s.wrapping_add(0x10));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s.wrapping_add(0x20));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.lq(2, s.wrapping_add(0x20));
        c.sq(2, s);
        Addr(s.wrapping_add(0xc)).sw(c, 0);
        c.lqc2(8, s);
        let t1 = c.g[T1][0] as u32;
        for i in 0..4u32 {
            c.lqc2(4 + i as usize, t1.wrapping_add(0x10 * i));
        }
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, s.wrapping_add(0x20));
        c.lq(3, s.wrapping_add(0x20));
        c.sq(3, s);
        c.swc1(1, s.wrapping_add(0xc));
        c.sq(3, s.wrapping_add(0x10));
        c.lq(2, s);
        c.sq(2, f(0x80));
        // The link: EA_MemAllocNamed(0x10, "...", 0x20000000, 0).
        call_ptr(c, RM_BUILD_MEM_ALLOC, 0x0035_c4c0)?;
        c.g[V1][0] = 0x2;
        let (link, owner, kind) = (c.g[V0][0], c.g[S0][0], c.g[V1][0]);
        Addr((link as u32).wrapping_add(0xc)).sw(c, owner);
        Addr((link as u32).wrapping_add(0x8)).sw(c, kind);
        Addr((owner as u32).wrapping_add(RM_BUILD_LINK)).sw(c, link);
        frame.restore(c, S0, 0x120);
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, put_vec, put_words, Rng, DATA, GAME_GP};

    /// A float vector: mostly ordinary values, sometimes edge cases.
    fn oc313_vec(r: &mut Rng, edge: bool) -> [u32; 4] {
        let mut v = [0u32; 4];
        for x in v.iter_mut() {
            *x = if edge { r.edge_f32() } else { r.f32(50.0) };
        }
        v
    }

    /// Landing angle: zero vectors (both early outs), tiny vectors (below
    /// the 0.001 length), parallel/antiparallel vectors (clamp edges) and
    /// random ones; the real `Math_Asin` runs with its `.lit4` constants.
    #[test]
    fn rider_camera_target_v23_matches() {
        check(
            ee::s01::f_0015f780__generated,
            rider_camera_target_v23,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_f780, 0);
                c.pc = 0x0015_f780;
                // .lit4: the function's and Math_Asin's.
                let lit = |c: &mut Cpu, off: i32, v: f32| {
                    c.bus.write32(GAME_GP.wrapping_add(off as u32), v.to_bits())
                };
                lit(c, -0x6c50, 0.001);
                lit(c, -0x6c4c, std::f32::consts::FRAC_PI_2);
                lit(c, -0x2e60, -std::f32::consts::FRAC_1_SQRT_2);
                lit(c, -0x2e58, std::f32::consts::FRAC_1_SQRT_2);
                lit(c, -0x2e54, std::f32::consts::FRAC_PI_2);
                put_words(
                    c,
                    GAME_GP + 0xdf0,
                    &[
                        0.167_702_27f32.to_bits(),
                        0.067_658_506f32.to_bits(),
                        0.052_005_03f32.to_bits(),
                        0.054_984_73f32.to_bits(),
                    ],
                );
                let target = DATA;
                let rider = DATA + 0x100;
                let landing = DATA + 0x1000;
                c.bus.write32(target + RCT_V23_RIDER, rider);
                c.bus.write32(rider + RCT_V23_LANDING, landing);
                c.g[A0][0] = sx(target);
                let edge = seed % 7 == 6;
                let mut n = oc313_vec(&mut r, edge);
                let mut v = oc313_vec(&mut r, edge);
                match seed % 8 {
                    0 => n = [0; 4],
                    1 => v = [0; 4],
                    2 => {
                        // tiny: |n||v| < 0.001
                        for x in n.iter_mut().chain(v.iter_mut()) {
                            *x = r.f32(0.01);
                        }
                    }
                    3 => {
                        // v parallel or antiparallel to n
                        let k = if r.next_u32() & 1 == 0 { 3.0f32 } else { -2.5 };
                        for i in 0..4 {
                            v[i] = (f32::from_bits(n[i]) * k).to_bits();
                        }
                    }
                    _ => {}
                }
                put_vec(c, landing + RCT_V23_LANDING_VELOCITY, v);
                put_vec(c, landing + RCT_V23_LANDING_NORMAL, n);
            },
        );
    }

    /// Rail transform: no start node (rail matrix), a single node, and
    /// chains of up to four nodes; the link is allocated from the pools.
    #[test]
    fn rail_modifier_build_rail_nodes_matches() {
        check(
            ee::s11::f_0035c0e8__generated,
            rail_modifier_build_rail_nodes,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_c0e8, 0);
                c.pc = 0x0035_c0e8;
                crate::game::fe::debugfe::tests::pool_allocator(c);
                let this = DATA;
                let rail = DATA + 0x1000;
                let table = DATA + 0x2000;
                let nodes = DATA + 0x3000;
                let mats = DATA + 0x4000;
                let edge = seed % 9 == 8;
                for i in 0..0x24u32 {
                    c.bus.write32(this + 4 * i, r.next_u32());
                }
                for i in 0..4u32 {
                    let v = oc313_vec(&mut r, edge);
                    put_vec(c, rail + RM_BUILD_RAIL_MATRIX + 0x10 * i, v);
                    put_vec(
                        c,
                        this + RM_BUILD_XFORM + 0x10 * i,
                        oc313_vec(&mut r, false),
                    );
                }
                c.bus.write32(this + RM_BUILD_RAIL, rail);
                c.bus.write32(rail + RM_BUILD_RAIL_TABLE, table);
                c.bus.write32(rail + RM_BUILD_RAIL_SCALE, r.f32(4.0));
                c.bus.write32(table + 0x8, nodes);
                // Node chain: start, then `len` more, then -1.
                let len = seed % 5;
                let mut idx = 1 + r.next_u32() % 3;
                let start = idx;
                for k in 0..=len {
                    let next = if k == len {
                        u32::MAX
                    } else {
                        idx + 1 + r.next_u32() % 2
                    };
                    let node = nodes + 0x10 * idx;
                    c.bus.write32(node, next);
                    c.bus.write32(node + 0xc, mats + 0x40 * k);
                    for i in 0..4u32 {
                        let v = oc313_vec(&mut r, edge);
                        put_vec(c, mats + 0x40 * k + 0x10 * i, v);
                    }
                    idx = next;
                }
                let first = if seed % 4 == 0 {
                    u32::MAX - r.next_u32() % 4
                } else {
                    start
                };
                c.bus.write32(this + RM_BUILD_NODE, first);
                c.g[A0][0] = sx(this);
            },
        );
    }
}
