//! Staging wave 201 (oc209): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size),
//! one function per writer. Each reproduces the generated body's registers,
//! cycles, memory and call order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_9e50, world_cache_point_box_distance, "cWorldCache_PointBoxDistance"),
];

// ---------------------------------------------------------------------------
// cWorldCache_PointBoxDistance (0x3a9e50, worldview.cpp)

/// `c.le.s a, b`, latched in the FCC bit as the original leaves it.
#[inline]
fn fpu_le(c: &mut Cpu, a: crate::ps2f::FpuF32, b: crate::ps2f::FpuF32) -> bool {
    let le = a.le(b);
    crate::abi::set_fcc(c, le);
    le
}

/// One axis of the outside distance: `pos - max` when the point lies above
/// the box on that axis (the difference is not negative), else `min - pos`.
/// `x_axis` selects the x variant, whose `bmin` reload costs two cycles; the
/// y/z variants compute it in a branch-likely delay slot from the saved
/// registers. Returns the (squared if `squared`) term; leaves f0/f1 as the
/// original does.
#[inline]
fn point_box_axis(
    c: &mut Cpu,
    x_axis: bool,
    pos: crate::ps2f::FpuF32,
    min: crate::ps2f::FpuF32,
    bmax: crate::guest::Addr,
    bmin: crate::guest::Addr,
    squared: bool,
) -> crate::ps2f::FpuF32 {
    use crate::ps2f::FpuF32;
    c.cycles += 7;
    let max = bmax.read_fpu(c);
    crate::abi::set_fpr(c, 1, FpuF32::ZERO);
    let mut d = pos - max;
    if crate::abi::c_lt_s(c, d, FpuF32::ZERO) {
        if x_axis {
            c.cycles += 2;
            let min = bmin.read_fpu(c);
            d = min - pos;
        } else {
            d = min - pos;
        }
    }
    c.cycles += 2;
    if squared {
        d = d * d;
    }
    crate::abi::set_fpr(c, 0, d);
    c.cycles += 1;
    d
}

/// `float cWorldCache_PointBoxDistance(void* self, const Vec3* pos,
/// const Vec3* bmin, const Vec3* bmax)` (0x3a9e50): how far a point lies
/// outside an axis-aligned box, used by the world cache to rank streaming
/// nodes. Returns -1.0 when the point is inside (bounds inclusive). When it
/// is outside on exactly one axis the result is that axis' plain distance;
/// outside on two or three axes it is the Euclidean distance to the box
/// (squared terms, then `sqrt.s`).
///
/// Register leftovers reproduced: t3/t0/v1 = inside flags per axis; on the
/// inside path at = -1.0 bits; otherwise a0/t1/t2 = outside flags per axis,
/// v0 = "more than one axis outside"; f0..f7 and FCC as the original.
pub fn world_cache_point_box_distance(c: &mut Cpu) {
    use crate::abi::{set_fpr, set_reg};
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;

    let pos = Addr(r32(c, A1));
    let bmin = Addr(r32(c, A2));
    let bmax = Addr(r32(c, A3));

    // X: f1 = pos.x, f0 = bmin.x (then bmax.x).
    c.cycles += 6;
    let px = pos.read_fpu(c);
    let min_x = bmin.read_fpu(c);
    set_fpr(c, 1, px);
    set_fpr(c, 0, min_x);
    let mut in_x = fpu_le(c, min_x, px);
    if in_x {
        c.cycles += 5;
        let max_x = bmax.read_fpu(c);
        set_fpr(c, 0, max_x);
        in_x = fpu_le(c, px, max_x);
    }

    // Y: f6 = bmin.y, f4 = pos.y.
    c.cycles += 8;
    let min_y = bmin.off(4).read_fpu(c);
    let py = pos.off(4).read_fpu(c);
    set_fpr(c, 1, min_y);
    set_fpr(c, 0, py);
    set_fpr(c, 6, min_y);
    set_fpr(c, 4, py);
    let mut in_y = fpu_le(c, min_y, py);
    if in_y {
        c.cycles += 5;
        let max_y = bmax.off(4).read_fpu(c);
        set_fpr(c, 0, max_y);
        in_y = fpu_le(c, py, max_y);
    }

    // Z: f7 = bmin.z, f5 = pos.z.
    c.cycles += 8;
    let min_z = bmin.off(8).read_fpu(c);
    let pz = pos.off(8).read_fpu(c);
    set_fpr(c, 1, min_z);
    set_fpr(c, 0, pz);
    set_fpr(c, 7, min_z);
    set_fpr(c, 5, pz);
    let mut in_z = fpu_le(c, min_z, pz);
    if in_z {
        c.cycles += 5;
        let max_z = bmax.off(8).read_fpu(c);
        set_fpr(c, 0, max_z);
        in_z = fpu_le(c, pz, max_z);
    }

    set_reg(c, T3, in_x as u64);
    set_reg(c, T0, in_y as u64);
    set_reg(c, V1, in_z as u64);

    // Inside on every axis: -1.0.
    c.cycles += 2;
    if in_x {
        c.cycles += 2;
        if in_y {
            c.cycles += 2;
            if in_z {
                c.cycles += 4;
                set_reg(c, AT, 0xffff_ffff_bf80_0000);
                set_fpr(c, 0, FpuF32(0xbf80_0000));
                ret(c);
                return;
            }
        }
    }

    // Which axes to measure, and whether to combine them Euclidean-style.
    let (out_x, out_y, out_z) = (!in_x, !in_y, !in_z);
    let squared = (out_x as u32 + out_y as u32 + out_z as u32) > 1;
    c.cycles += 6 + if squared { 7 } else { 4 };
    set_fpr(c, 2, FpuF32::ZERO);
    set_reg(c, A0, out_x as u64);
    set_reg(c, T1, out_y as u64);
    set_reg(c, T2, out_z as u64);
    set_reg(c, V0, squared as u64);

    let mut sum = FpuF32::ZERO;
    c.cycles += 2;
    if out_x {
        // The x term reloads pos.x into f3.
        let px = pos.read_fpu(c);
        set_fpr(c, 3, px);
        let d = point_box_axis(c, true, px, min_x, bmax, bmin, squared);
        sum = sum + d;
        set_fpr(c, 2, sum);
    }
    c.cycles += 2;
    if out_y {
        let d = point_box_axis(c, false, py, min_y, bmax.off(4), bmin.off(4), squared);
        sum = sum + d;
        set_fpr(c, 2, sum);
    }
    c.cycles += 2;
    if out_z {
        let d = point_box_axis(c, false, pz, min_z, bmax.off(8), bmin.off(8), squared);
        sum = sum + d;
        set_fpr(c, 2, sum);
    }

    c.cycles += 2;
    if squared {
        c.cycles += 1;
        sum = sum.sqrt();
        set_fpr(c, 2, sum);
    }
    c.cycles += 2;
    set_fpr(c, 0, sum);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// cWorldCache_PointBoxDistance: per axis the point lies below, inside
    /// or above the box (seed enumerates all 27 combinations, so inside,
    /// one-axis and two/three-axis Euclidean paths are all reached), plus
    /// edge-case floats (±0, denormals, exponent 255, inverted boxes) on
    /// every fourth seed.
    #[test]
    fn world_cache_point_box_distance_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        const POS: u32 = DATA;
        const BMIN: u32 = DATA + 0x10;
        const BMAX: u32 = DATA + 0x20;
        check(
            ee::s13::f_003a9e50__generated,
            world_cache_point_box_distance,
            300,
            &|c, seed| {
                let mut r = Rng(0x3a9e_5000 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x003a_9e50;
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(POS);
                c.g[A2][0] = sx(BMIN);
                c.g[A3][0] = sx(BMAX);
                let mut mode = seed % 27;
                for axis in 0..3u32 {
                    let (p, lo, hi) = if seed % 4 == 3 {
                        (r.edge_f32(), r.edge_f32(), r.edge_f32())
                    } else {
                        let lo = (r.next_u32() % 200) as f32 - 100.0;
                        let hi = lo + (r.next_u32() % 50) as f32 + 0.25;
                        let p = match mode % 3 {
                            0 => lo - 1.0 - (r.next_u32() % 300) as f32 * 0.5,
                            1 => match r.next_u32() % 4 {
                                0 => lo,
                                1 => hi,
                                _ => (lo + hi) * 0.5,
                            },
                            _ => hi + 1.0 + (r.next_u32() % 300) as f32 * 0.5,
                        };
                        (p.to_bits(), lo.to_bits(), hi.to_bits())
                    };
                    mode /= 3;
                    c.bus.write32(POS + axis * 4, p);
                    c.bus.write32(BMIN + axis * 4, lo);
                    c.bus.write32(BMAX + axis * 4, hi);
                }
            },
        );
    }
}
