//! Staging module oc384 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc384`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0035_c698,
    railmodifier_find_closest_point,
    "RailModifier_FindClosestPoint",
)];

// ---------------------------------------------------------------------------
// RailModifier_FindClosestPoint (0x35c698)

/// Spline record fields (`sRmSpline`): the collision mask, the segment
/// count, the first segment and the spline's id.
pub(crate) const RMFCP_SPLINE_MASK: u32 = 0x1c;
pub(crate) const RMFCP_SPLINE_NUM_SEGS: u32 = 0x20;
pub(crate) const RMFCP_SPLINE_SEGS: u32 = 0x24;
pub(crate) const RMFCP_SPLINE_ID: u32 = 0x28;
/// Segment fields: the cubic's coefficient matrix and the next segment.
pub(crate) const RMFCP_SEG_COEF: u32 = 0x10;
pub(crate) const RMFCP_SEG_NEXT: u32 = 0x64;
/// The golden-section start brackets (five floats each, by sample index).
#[cfg(test)]
pub(crate) const RMFCP_BRACKET_LO: u32 = 0x0048_e7d8;
#[cfg(test)]
pub(crate) const RMFCP_BRACKET_HI: u32 = 0x0048_e7f0;
/// `t4vect_Zero`.
pub(crate) const RMFCP_VEC_ZERO: u32 = 0x004f_f120;

/// `vf12 = vec * M` (row vector times the 4x4 matrix at `mat`), stored at
/// `out`: a point (or tangent) of the cubic segment whose transformed
/// coefficients are `mat`.
fn rmfcp_mtx_vec(c: &mut Cpu, mat: u32, vec: u32, out: u32) {
    c.lqc2(8, vec);
    c.lqc2(4, mat);
    c.lqc2(5, mat.wrapping_add(0x10));
    c.lqc2(6, mat.wrapping_add(0x20));
    c.lqc2(7, mat.wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, out);
}

/// `[out] = [a] - [b]` (vf3 - vf4 into vf5).
fn rmfcp_vsub(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `vf4.x = dot(vf3, vf{other})` of the quadword in vf3 (the VU0 sequence
/// shared by the dot product and the length).
fn rmfcp_dot_x(c: &mut Cpu, other: u8) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(other));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `v1 = |[v]|` (4-component length through Q), left in `v1`.
fn rmfcp_length(c: &mut Cpu, v: u32) {
    c.lqc2(3, v);
    rmfcp_dot_x(c, 3);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V1, 22);
}

/// `vf4.x = dot([a], [b])`.
fn rmfcp_dot(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    rmfcp_dot_x(c, 5);
}

/// `void RailModifier_FindClosestPoint(RailModifier* self, Box*, Box*,
/// const Vec4* point, u32 mask, int* found, float* bestDist, Hit* out)`
/// (0x35c698): look up the rail's spline in the world tables; if its mask
/// matches, transform each cubic segment by the modifier's world matrix,
/// sample it at five points (t = 0, 0.25, .., 1) to pick the nearest chord,
/// then refine t with a golden-section search (at most 24 steps, to 0.0005)
/// inside that sample's bracket. A distance closer than `*bestDist` (or the
/// first hit) records the point, the normalised tangent, the spline id, the
/// model, the bone, t and the spline in the hit.
pub fn railmodifier_find_closest_point(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x1d0);
        frame.save(c, S0, 0x1c0);
        frame.save(c, S1, 0x1b0);
        c.g[S0][0] = c.g[T3][0];
        frame.save(c, S2, 0x1a0);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S4, 0x180);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S5, 0x170);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S6, 0x160);
        c.g[S5][0] = c.g[T2][0];
        frame.save(c, S3, 0x190);
        frame.save(c, S7, 0x150);
        frame.save_ra(c, 0x140);
        c.g[S6][0] = c.g[T1][0];
        // World_GetInstance.
        call_ptr(c, 0x002d_1bd8, 0x0035_c6dc)?;

        // The spline: world->sets[id & 0xff]->refs[id >> 8] >> 8 << 2.
        c.g[V1][0] = c.lw(r32(c, V0));
        c.g[A1][0] = c.lw(r32(c, S4).wrapping_add(0x30));
        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[V1][0] = c.lw(r32(c, V0));
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x44));
            c.cycles += 7;
            c.g[V0][0] = sx(r32(c, A1) >> 8);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = c.lw(r32(c, V0));
            c.g[V0][0] = sx(r32(c, V1) >> 8);
            let present = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            if !present {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[S3][0] = c.g[V0][0];
        } else {
            c.cycles += 2;
            c.g[S3][0] = 0;
        }

        c.cycles += 4;
        c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(RMFCP_SPLINE_MASK));
        c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x1c0);
            rmfcp_epilogue(c, frame);
            return Ok(());
        }

        // The modifier's world matrix at sp+0.
        c.cycles += 7;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[SP][0];
        Callee {
            addr: 0x0035_c5a0,
            f: ee::s11::RailModifier_GetWorldMatrix,
        }
        .call(c, 0x0035_c748)?;
        c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(RMFCP_SPLINE_NUM_SEGS));
        c.g[V1][0] = 0;
        c.g[T3][0] = c.lw(r32(c, S3).wrapping_add(RMFCP_SPLINE_SEGS));
        if (c.g[V0][0] as i64) <= 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0x1c0);
            rmfcp_epilogue(c, frame);
            return Ok(());
        }

        c.cycles += 18;
        let s = sp(c);
        let at = |o: u32| s.wrapping_add(o);
        c.g[V0][0] = 0x50_0000;
        c.g[AT][0] = 0x3e80_0000;
        c.mtc1(AT, 18); // f18 = 0.25 (sample step)
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 12); // f12 = 1.0
        c.g[T6][0] = sx(RMFCP_VEC_ZERO);
        c.mtc1(0, 17); // f17 = 0.0
        c.g[T1][0] = sx(at(0xb0));
        c.g[T8][0] = c.g[SP][0];
        c.g[A1][0] = sx(at(0x70));
        c.g[T4][0] = sx(at(0xc0));
        c.g[T9][0] = 0x49_0000;
        c.g[S7][0] = 0x49_0000;
        c.g[S2][0] = sx(at(0xd0));
        c.g[T0][0] = sx(at(0x100));
        c.g[T2][0] = u64::MAX;
        c.g[T7][0] = sx(at(0x120));
        let coefs = at(0x70); // the segment's world coefficients (a1)

        loop {
            // One segment: coefs = world * seg->coef.
            c.cycles += 76;
            c.mtc1(0, 9);
            let seg = r32(c, T3);
            c.g[V0][0] = sx(seg.wrapping_add(RMFCP_SEG_COEF));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 11);
            let world = r32(c, T8);
            let coef = r32(c, V0);
            c.lqc2(4, world);
            c.lqc2(5, world.wrapping_add(0x10));
            c.lqc2(6, world.wrapping_add(0x20));
            c.lqc2(7, world.wrapping_add(0x30));
            c.lqc2(8, coef);
            c.lqc2(9, coef.wrapping_add(0x10));
            c.lqc2(10, coef.wrapping_add(0x20));
            c.lqc2(11, coef.wrapping_add(0x30));
            for (row, fd) in [(8u8, 12u8), (9, 13), (10, 14), (11, 15)] {
                c.vu0().mula(0b1111, 4, Src::Bc(row, X));
                c.vu0().madda(0b1111, 5, Src::Bc(row, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(row, Z));
                c.vu0().madd(0b1111, fd, 7, Src::Bc(row, W));
            }
            let tmp = r32(c, T1);
            c.sqc2(12, tmp);
            c.sqc2(13, tmp.wrapping_add(0x10));
            c.sqc2(14, tmp.wrapping_add(0x20));
            c.sqc2(15, tmp.wrapping_add(0x30));
            c.lqc2(1, tmp);
            c.lqc2(2, tmp.wrapping_add(0x10));
            c.lqc2(3, tmp.wrapping_add(0x20));
            c.lqc2(4, tmp.wrapping_add(0x30));
            c.sqc2(1, coefs);
            c.sqc2(2, coefs.wrapping_add(0x10));
            c.sqc2(3, coefs.wrapping_add(0x20));
            c.sqc2(4, coefs.wrapping_add(0x30));
            // pos = eval(0); best = |point - pos|.
            c.swc1(11, at(0xcc));
            c.g[T5][0] = sx(r32(c, V1).wrapping_add(1));
            c.swc1(9, at(0xc0));
            c.g[A2][0] = 0;
            c.swc1(9, at(0xc8));
            c.g[A0][0] = 1;
            c.swc1(9, at(0xc4));
            let sample = r32(c, T4).wrapping_add(0x10);
            rmfcp_mtx_vec(c, coefs, at(0xc0), sample);
            c.lq(V0, at(0xd0));
            c.sq(V0, at(0xb0));
            c.sq(V0, at(0x40));
            let point = r32(c, S1);
            rmfcp_vsub(c, point, at(0x40), at(0xc0));
            c.lq(V0, at(0xc0));
            c.sq(V0, at(0xb0));
            rmfcp_length(c, at(0xb0));
            c.mtc1(V1, 3);

            // Samples k = 1..4: the nearest point on each chord.
            loop {
                c.cycles += 59;
                c.mtc1(A0, 0);
                c.cvt_s_w(0, 0);
                c.swc1(12, at(0xcc));
                c.mov_s(2, 17);
                c.mul_s(0, 0, 18);
                c.mul_s(1, 0, 0);
                c.swc1(0, at(0xc8));
                c.mul_s(0, 0, 1);
                c.swc1(1, at(0xc4));
                c.swc1(0, at(0xc0));
                let sample = r32(c, T4).wrapping_add(0x10);
                rmfcp_mtx_vec(c, coefs, at(0xc0), sample);
                c.lq(V0, at(0xd0));
                c.sq(V0, at(0xb0));
                c.sq(V0, at(0x50)); // next
                rmfcp_vsub(c, at(0x50), at(0x40), at(0xc0));
                c.lq(V0, at(0xc0));
                c.sq(V0, at(0xb0));
                c.sq(V0, at(0x60)); // dir = next - pos
                let point = r32(c, S1);
                rmfcp_vsub(c, point, at(0x40), at(0xc0));
                c.lq(V0, at(0xc0));
                c.sq(V0, at(0xb0));
                rmfcp_dot(c, at(0xb0), at(0x60));
                c.qmfc2(V1, 4);
                c.mtc1(V1, 1);
                rmfcp_dot(c, at(0x60), at(0x60));
                c.qmfc2(V0, 4);
                c.mtc1(V0, 0);
                c.div_s(1, 1, 0);
                c.c_le_s(17, 1);
                if c.fcc() {
                    c.min_s(2, 1, 12); // u clamped to [0, 1]
                }

                c.cycles += 34;
                c.mfc1(V1, 2);
                c.lqc2(4, at(0x60));
                c.qmtc2(V1, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(0xd0));
                c.lq(V0, at(0xd0));
                c.sq(V0, at(0xc0));
                c.lqc2(3, at(0x40));
                c.lqc2(4, at(0xc0));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(0xd0));
                c.lq(V0, at(0xd0));
                c.sq(V0, at(0xb0));
                c.sq(V0, at(0x60));
                let point = r32(c, S1);
                rmfcp_vsub(c, point, at(0x60), at(0xc0));
                c.lq(V0, at(0xc0));
                c.sq(V0, at(0xb0));
                rmfcp_length(c, at(0xb0));
                c.mtc1(V1, 0);
                c.c_lt_s(0, 3);
                let closer = c.fcc();
                c.lq(V0, at(0x50));
                if closer {
                    c.cycles += 2;
                    c.mov_s(3, 0);
                    c.g[A2][0] = c.g[A0][0];
                }

                c.cycles += 4;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                c.g[V1][0] = ((c.g[A0][0] as i64) < 5) as u64;
                let more = c.g[V1][0] != 0;
                c.sq(V0, at(0x40)); // pos = next
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_c8d0);
                }
            }

            // Golden-section search inside the best sample's bracket.
            c.cycles += 165;
            c.g[A3][0] = sx(r32(c, T9).wrapping_sub(0x1828));
            let lo = r32(c, A3);
            c.ldl(V0, lo.wrapping_add(0x7));
            c.ldr(V0, lo);
            c.ldl(V1, lo.wrapping_add(0xf));
            c.ldr(V1, lo.wrapping_add(0x8));
            c.g[A0][0] = c.lw(lo.wrapping_add(0x10));
            c.sdl(V0, at(0xb7));
            c.sdr(V0, at(0xb0));
            c.sdl(V1, at(0xbf));
            c.sdr(V1, at(0xb8));
            c.sw(at(0xc0), c.g[A0][0]);
            c.g[V0][0] = sx(r32(c, A2) << 2);
            c.lwc1(3, r32(c, GP).wrapping_sub(0x2a0c)); // 0.381966
            c.g[A3][0] = sx(r32(c, S7).wrapping_sub(0x1810));
            let hi = r32(c, A3);
            c.ldl(V1, hi.wrapping_add(0x7));
            c.ldr(V1, hi);
            c.ldl(A0, hi.wrapping_add(0xf));
            c.ldr(A0, hi.wrapping_add(0x8));
            c.g[A2][0] = c.lw(hi.wrapping_add(0x10));
            c.sdl(V1, at(0xd7));
            c.sdr(V1, at(0xd0));
            c.sdl(A0, at(0xdf));
            c.sdr(A0, at(0xd8));
            c.sw(at(0xe0), c.g[A2][0]);
            c.g[V1][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
            c.mov_s(15, 3);
            c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, V0)));
            c.lwc1(11, r32(c, V1)); // b
            c.lwc1(9, r32(c, V0)); // a
            c.g[A0][0] = 0;
            c.lwc1(1, r32(c, GP).wrapping_sub(0x2a08)); // 0.618034
            c.mul_s(5, 11, 11);
            c.sub_s(0, 11, 9);
            c.swc1(12, at(0x10c));
            c.mul_s(2, 9, 9);
            c.swc1(9, at(0x108));
            c.mov_s(14, 1);
            c.mul_s(6, 11, 5);
            c.mul_s(3, 0, 3);
            c.lwc1(16, r32(c, GP).wrapping_sub(0x2a04)); // 0.0005
            c.mul_s(4, 9, 2);
            c.swc1(2, at(0x104));
            c.mul_s(0, 0, 1);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 13);
            c.add_s(7, 9, 3); // tc
            c.g[A3][0] = c.g[T0][0];
            c.swc1(4, at(0x100));
            c.g[A2][0] = c.g[A1][0];
            // f(a)
            let (mat, probe) = (r32(c, A1), r32(c, T0).wrapping_add(0x10));
            rmfcp_mtx_vec(c, mat, at(0x100), probe);
            c.add_s(4, 9, 0); // td
            c.lq(V0, at(0x110));
            c.mul_s(1, 7, 7);
            c.sq(V0, at(0xf0));
            c.mul_s(3, 4, 4);
            c.sq(V0, at(0x40));
            c.mul_s(0, 7, 1);
            let point = r32(c, S1);
            rmfcp_vsub(c, at(0x40), point, at(0x100));
            c.mov_s(10, 4);
            c.lq(V0, at(0x100));
            c.mul_s(2, 4, 3);
            c.swc1(12, at(0x10c));
            c.sq(V0, at(0xf0));
            c.swc1(0, at(0x100));
            c.swc1(7, at(0x108));
            c.swc1(1, at(0x104));
            // f(tc)
            let (mat, probe) = (r32(c, A1), r32(c, T0).wrapping_add(0x10));
            rmfcp_mtx_vec(c, mat, at(0x100), probe);
            c.lq(V0, at(0x110));
            c.sq(V0, at(0x40));
            c.sq(V0, at(0xf0));
            let point = r32(c, S1);
            rmfcp_vsub(c, at(0x40), point, at(0x100));
            c.lq(V0, at(0x100));
            c.sq(V0, at(0xf0));
            rmfcp_length(c, at(0xf0));
            c.mtc1(V1, 8);
            c.swc1(2, at(0x100));
            c.swc1(12, at(0x10c));
            c.swc1(4, at(0x108));
            c.swc1(3, at(0x104));
            // f(td)
            let (mat, probe) = (r32(c, A1), r32(c, T0).wrapping_add(0x10));
            rmfcp_mtx_vec(c, mat, at(0x100), probe);
            c.lq(V0, at(0x110));
            c.sq(V0, at(0xf0));
            c.sq(V0, at(0x40));
            let point = r32(c, S1);
            rmfcp_vsub(c, at(0x40), point, at(0x100));
            c.lq(V0, at(0x100));
            c.sq(V0, at(0xf0));
            rmfcp_length(c, at(0xf0));
            c.mtc1(V1, 3);
            c.swc1(6, at(0x100));
            c.mov_s(2, 3);
            c.swc1(12, at(0x10c));
            c.swc1(11, at(0x108));
            c.swc1(5, at(0x104));
            // f(b)
            let (mat, probe) = (r32(c, A1), r32(c, T0).wrapping_add(0x10));
            rmfcp_mtx_vec(c, mat, at(0x100), probe);
            c.lq(V0, at(0x110));
            c.sq(V0, at(0xf0));
            c.sq(V0, at(0x40));
            let point = r32(c, S1);
            rmfcp_vsub(c, at(0x40), point, at(0x100));
            c.lq(V0, at(0x100));
            c.sq(V0, at(0xf0));

            loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 24) as u64;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.lw(r32(c, S6));
                    break;
                }
                c.cycles += 5;
                c.sub_s(0, 11, 9);
                c.abs_s(0, 0);
                c.c_le_s(0, 16);
                if c.fcc() {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r32(c, S6));
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_ccf0);
                }
                c.cycles += 4;
                c.c_lt_s(3, 8);
                if c.fcc() {
                    // fd < fc: a = tc, tc = td, td = tc * 0.618 + b * 0.382.
                    c.cycles += 45;
                    c.mov_s(9, 7);
                    c.mul_s(2, 11, 15);
                    c.mov_s(7, 4);
                    c.swc1(13, at(0x10c));
                    c.mul_s(1, 7, 14);
                    c.mov_s(8, 3);
                    c.add_s(4, 1, 2);
                    c.mul_s(0, 4, 4);
                    c.swc1(4, at(0x108));
                    c.mov_s(10, 4);
                    c.mul_s(1, 4, 0);
                    c.swc1(0, at(0x104));
                    c.swc1(1, at(0x100));
                    rmfcp_golden_eval(c, at);
                    c.mov_s(3, 0);
                    c.mov_s(2, 3);
                } else {
                    // b = td, td = tc, tc = td * 0.618 + a * 0.382.
                    c.mov_s(11, 4);
                    c.cycles += 43;
                    c.mul_s(2, 9, 15);
                    c.mov_s(4, 7);
                    c.swc1(13, at(0x10c));
                    c.mul_s(1, 4, 14);
                    c.mov_s(3, 8);
                    c.add_s(7, 1, 2);
                    c.mul_s(0, 7, 7);
                    c.swc1(7, at(0x108));
                    c.mov_s(10, 7);
                    c.mul_s(1, 7, 0);
                    c.swc1(0, at(0x104));
                    c.swc1(1, at(0x100));
                    rmfcp_golden_eval(c, at);
                    c.mov_s(8, 0);
                    c.mov_s(2, 8);
                }
                c.cycles += 1;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
            }

            // Record the hit when it is the first or the closest so far.
            c.cycles += 2;
            let none_yet = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            let record = if none_yet {
                true
            } else {
                c.cycles += 5;
                c.lwc1(0, r32(c, S5));
                c.c_lt_s(2, 0);
                if !c.fcc() {
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(RMFCP_SPLINE_NUM_SEGS));
                }
                c.fcc()
            };
            if record {
                rmfcp_record_hit(c, at);
            }

            c.cycles += 4;
            c.g[V1][0] = c.g[T5][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[T3][0] = c.lw(r32(c, T3).wrapping_add(RMFCP_SEG_NEXT));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_c7a0);
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x1c0);
        rmfcp_epilogue(c, frame);
        Ok(())
    })
}

/// One golden-section probe: the point at the parameter just built at
/// sp+0x100, its offset from the query point, and its distance into f0.
fn rmfcp_golden_eval(c: &mut Cpu, at: impl Fn(u32) -> u32) {
    let (mat, probe) = (r32(c, A2), r32(c, A3).wrapping_add(0x10));
    rmfcp_mtx_vec(c, mat, at(0x100), probe);
    c.lq(V0, at(0x110));
    c.sq(V0, at(0xf0));
    c.sq(V0, at(0x40));
    let point = r32(c, S1);
    rmfcp_vsub(c, at(0x40), point, at(0x100));
    c.lq(V0, at(0x100));
    c.sq(V0, at(0xf0));
    rmfcp_length(c, at(0xf0));
    c.mtc1(V1, 0);
}

/// Fill the hit record (`s0`) from the refined point: `*found = 1`,
/// `*bestDist = dist`, type 2, the position, the normalised tangent at t,
/// the spline id, model, bone, t and spline.
fn rmfcp_record_hit(c: &mut Cpu, at: impl Fn(u32) -> u32) {
    c.cycles += 64;
    let hit = r32(c, S0);
    c.g[V1][0] = 0x2;
    c.sw(r32(c, S6), c.g[V0][0]);
    c.add_s(1, 10, 10);
    c.swc1(2, r32(c, S5));
    c.sw(hit.wrapping_add(0x44), c.g[V1][0]);
    c.sw(hit.wrapping_add(0x54), 0);
    c.sw(hit.wrapping_add(0x50), 0);
    c.sw(hit.wrapping_add(0x58), 0);
    c.sw(hit.wrapping_add(0x5c), c.g[T2][0]);
    c.sw(hit.wrapping_add(0x60), c.g[T2][0]);
    c.sw(hit.wrapping_add(0x64), c.g[T2][0]);
    c.sw(hit.wrapping_add(0x4c), c.g[T2][0]);
    c.g[AT][0] = 0x4040_0000;
    c.mtc1(AT, 0); // 3.0
    c.lq(V0, r32(c, T6));
    c.lq(V1, at(0x40));
    c.mul_s(0, 10, 0);
    c.sq(V0, hit.wrapping_add(0x20));
    c.swc1(1, at(0x124));
    c.lq(V0, r32(c, T6));
    c.mul_s(0, 10, 0);
    c.sq(V1, hit);
    c.sq(V0, hit.wrapping_add(0x30));
    c.swc1(0, at(0x120));
    c.sw(hit.wrapping_add(0x68), 0);
    c.sw(hit.wrapping_add(0x6c), 0);
    c.sw(hit.wrapping_add(0x70), 0);
    c.sw(at(0x12c), 0);
    c.swc1(12, at(0x128));
    // The tangent (3t^2, 2t, 1, 0) through the coefficients, normalised.
    let (mat, tangent) = (r32(c, A1), r32(c, T7).wrapping_add(0x10));
    rmfcp_mtx_vec(c, mat, at(0x120), tangent);
    c.lq(V0, at(0x130));
    c.sq(V0, at(0x100));
    c.lqc2(3, at(0x100));
    rmfcp_dot_x(c, 3);
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, at(0x110));
    c.lq(V0, at(0x110));
    c.sq(V0, at(0xf0));
    c.sq(V0, hit.wrapping_add(0x10));
    c.g[V1][0] = c.lw(r32(c, S3).wrapping_add(RMFCP_SPLINE_ID));
    c.sw(hit.wrapping_add(0x4c), c.g[V1][0]);
    c.g[V0][0] = c.lw(r32(c, S4).wrapping_add(0x40));
    c.sw(hit.wrapping_add(0x50), c.g[V0][0]);
    c.g[V1][0] = c.lw(r32(c, S4).wrapping_add(0x34));
    c.swc1(10, hit.wrapping_add(0x68));
    c.sw(hit.wrapping_add(0x5c), c.g[V1][0]);
    c.sw(hit.wrapping_add(0x58), c.g[S3][0]);
    c.sw(hit.wrapping_add(0x54), 0);
    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(RMFCP_SPLINE_NUM_SEGS));
}

/// Restore s1..s7 and ra, pop the 0x1d0 frame and return.
fn rmfcp_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 10;
    frame.restore(c, S1, 0x1b0);
    frame.restore(c, S2, 0x1a0);
    frame.restore(c, S3, 0x190);
    frame.restore(c, S4, 0x180);
    frame.restore(c, S5, 0x170);
    frame.restore(c, S6, 0x160);
    frame.restore(c, S7, 0x150);
    frame.restore_ra(c, 0x140);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RailModifier_FindClosestPoint: a rail whose world tables give no set,
    /// no spline, or a spline (mask matching or not, 0..3 segments of random
    /// cubics), a random query point, a first or a later hit (closer or
    /// not). Some seeds widen the golden-section brackets so the search
    /// stops at its 24-step limit instead of the 0.0005 tolerance.
    #[test]
    fn railmodifier_find_closest_point_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, put_vec, put_words, DATA, GAME_GP};
        const SELF: u32 = DATA;
        const INST: u32 = DATA + 0x200;
        const SKEL: u32 = DATA + 0x300;
        const WORLD: u32 = DATA + 0x400;
        const SETS: u32 = DATA + 0x500;
        const SET: u32 = DATA + 0x900;
        const REFS: u32 = DATA + 0xa00;
        const SPLINE: u32 = DATA + 0x1000;
        const SEGS: u32 = DATA + 0x2000;
        const POINT: u32 = DATA + 0x3000;
        const FOUND: u32 = DATA + 0x3100;
        const DIST: u32 = DATA + 0x3104;
        const HIT: u32 = DATA + 0x3200;
        check(
            ee::s11::f_0035c698__generated,
            railmodifier_find_closest_point,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35c698, 0);
                c.pc = 0x0035_c698;
                // .lit4 constants and the bracket tables.
                put_words(
                    c,
                    GAME_GP - 0x2a0c,
                    &[0x3ec3_910c, 0x3f1e_377a, 0x3a03_126f],
                );
                if seed % 5 == 4 {
                    // Wide brackets: the search runs its 24 steps.
                    for i in 0..5 {
                        c.bus.write32(RMFCP_BRACKET_LO + 4 * i, r.f32(1000.0));
                        c.bus.write32(RMFCP_BRACKET_HI + 4 * i, r.f32(1000.0));
                    }
                } else {
                    put_words(c, RMFCP_BRACKET_LO, &[0, 0, 0, 0x3e80_0000, 0x3e80_0000]);
                    put_words(
                        c,
                        RMFCP_BRACKET_HI,
                        &[
                            0x3f40_0000,
                            0x3f40_0000,
                            0x3f80_0000,
                            0x3f80_0000,
                            0x3f80_0000,
                        ],
                    );
                }
                for i in 0..4 {
                    c.bus.write32(RMFCP_VEC_ZERO + 4 * i, r.f32(2.0));
                }
                // World tables: gp+0x16c8 -> &world; world+8 -> sets.
                c.bus.write32(GAME_GP + 0x16c8, WORLD);
                c.bus.write32(WORLD, WORLD + 0x10);
                c.bus.write32(WORLD + 0x10 + 8, SETS);
                let set_idx = r.next_u32() & 0xff;
                let ref_idx = r.next_u32() & 0x3f;
                c.bus.write32(SELF + 0x30, ref_idx << 8 | set_idx);
                let lookup = seed % 8;
                c.bus
                    .write32(SETS + 4 * set_idx, if lookup == 0 { 0 } else { SET });
                c.bus.write32(SET + 0x44, REFS);
                let r_word = if lookup == 1 {
                    r.next_u32() & 0xff
                } else {
                    (SPLINE >> 2) << 8 | (r.next_u32() & 0xff)
                };
                c.bus.write32(REFS + 4 * ref_idx, r_word);
                // The modifier: bone -1, its instance (no owner: the world
                // matrix is the instance's own), its transform at +0x50.
                c.bus.write32(SELF + 0x34, u32::MAX);
                c.bus.write32(SELF + 0x40, INST);
                for k in 0..16 {
                    c.bus.write32(SELF + 0x50 + 4 * k, r.f32(2.0));
                    c.bus.write32(INST + 0x10 + 4 * k, r.f32(2.0));
                }
                c.bus.write32(INST + 0xc, 0);
                c.bus.write32(INST + 0x80, SKEL);
                c.bus.write32(SKEL + 8, 0);
                // The spline.
                let mask = r.next_u32();
                c.bus.write32(SPLINE + RMFCP_SPLINE_MASK, mask);
                let query_mask = if seed % 8 == 2 {
                    !mask
                } else {
                    r.next_u32() | mask
                };
                let segs = match seed % 8 {
                    3 => 0,
                    _ => 1 + r.next_u32() % 3,
                };
                c.bus.write32(SPLINE + RMFCP_SPLINE_NUM_SEGS, segs);
                c.bus.write32(SPLINE + RMFCP_SPLINE_SEGS, SEGS);
                c.bus.write32(SPLINE + RMFCP_SPLINE_ID, r.next_u32());
                for i in 0..3 {
                    let seg = SEGS + 0x100 * i;
                    for k in 0..16 {
                        c.bus.write32(seg + RMFCP_SEG_COEF + 4 * k, r.f32(20.0));
                    }
                    c.bus.write32(seg + RMFCP_SEG_NEXT, seg + 0x100);
                }
                put_vec(
                    c,
                    POINT,
                    [r.f32(30.0), r.f32(30.0), r.f32(30.0), 0x3f80_0000],
                );
                c.bus.write32(FOUND, if seed % 3 == 0 { 0 } else { 1 });
                let best = match seed % 4 {
                    0 => (0.5f32).to_bits(),
                    1 => (1.0e6f32).to_bits(),
                    _ => r.f32(40.0) & 0x7fff_ffff,
                };
                c.bus.write32(DIST, best);
                for k in 0..0x20 {
                    c.bus.write32(HIT + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(DATA + 0x3400);
                c.g[A2][0] = sx(DATA + 0x3500);
                c.g[A3][0] = sx(POINT);
                c.g[T0][0] = sx(query_mask);
                c.g[T1][0] = sx(FOUND);
                c.g[T2][0] = sx(DIST);
                c.g[T3][0] = sx(HIT);
            },
        );
    }
}
