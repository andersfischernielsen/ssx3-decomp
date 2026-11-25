//! Staging module oc569 (wave232 s2): hand-written sphere-tree and triangle
//! collision replacements, each with its differential test against the
//! generated body. Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc569`.

#![allow(unused_imports)]

use crate::abi;
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
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
        0x0032_cdb0,
        sphere_tree_collide_sphere_node,
        "SphereTree_CollideSphereNode",
    ),
    (
        0x0032_d440,
        sphere_tree_point_in_triangle,
        "SphereTree_PointInTriangle",
    ),
    (
        0x0032_de20,
        world_free_sphere_tree_cache,
        "World_FreeSphereTreeCache",
    ),
    (
        0x0032_fe78,
        coll_sphere_collide_triangle,
        "CollSphere_CollideTriangle",
    ),
    (
        0x0032_ffa0,
        coll_sphere_collide_triangle_2sided,
        "CollSphere_CollideTriangle2Sided",
    ),
    (
        0x0033_0128,
        closest_point_on_segment,
        "ClosestPointOnSegment",
    ),
    (
        0x0033_03f0,
        coll_sphere_tree_construct,
        "CollSphereTree_ctor",
    ),
];

/// `VU0_TriangleIntersect` (0x32b6a8), reached through the function table.
const TRI_INTERSECT_ADDR: u32 = 0x0032_b6a8;
/// `EA_MemFree` (0x317e98), through the function table.
const EA_FREE_ADDR: u32 = 0x0031_7e98;
/// `EA_MemAllocNamed` (0x317d70), through the function table.
const EA_ALLOC_NAMED_ADDR: u32 = 0x0031_7d70;
/// `SphereTree_ctor` (0x32c508), through the function table.
const SPHERE_TREE_CTOR_ADDR: u32 = 0x0032_c508;
/// `SphereTree_GetRadius` (0x32c590), through the function table.
const SPHERE_TREE_RADIUS_ADDR: u32 = 0x0032_c590;

/// `CollSphere_PlaneDistance` (0x330360), a direct call.
const COLL_PLANE_DIST_CALLEE: Callee = Callee {
    addr: 0x0033_0360,
    f: ee::s11::f_00330360,
};
/// `CollSphere_HitsTriangleEdges` (0x330250), a direct call.
const COLL_HITS_EDGES_CALLEE: Callee = Callee {
    addr: 0x0033_0250,
    f: ee::s11::f_00330250,
};
/// The sphere-tree node itself (0x32cdb0): the recursive child call.
const SPHERE_NODE_CALLEE: Callee = Callee {
    addr: 0x0032_cdb0,
    f: ee::s10::f_0032cdb0,
};

/// `gp + off` for the game's gp-relative globals (`off` may be negative).
fn gp_word_addr(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `int SphereTree_PointInTriangle(a0, a1, a2, a3, t0)` (0x32d440): a tail
/// wrapper around `VU0_TriangleIntersect`, the tolerance (`gp-0x2d80`) passed
/// in f12. The callee's result is returned unchanged.
pub fn sphere_tree_point_in_triangle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[A0][0] = c.g[A1][0];
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A2][0];
        let tol = gp_word_addr(c, -0x2d80);
        c.lwc1(12, tol);
        c.g[A2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[T0][0];
        call_ptr(c, TRI_INTERSECT_ADDR, 0x0032_d460)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void World_FreeSphereTreeCache(void)` (0x32de20): frees the cached
/// sphere-tree node arrays and single entries through `EA_MemFree`, clearing
/// each pointer. The first array is at `gp+0xed8` (count `gp+0xee0`), the
/// single at `gp+0xedc`, the second array at `gp+0xee8` (count `gp+0xef0`),
/// and the last single at `gp+0xeec`.
pub fn world_free_sphere_tree_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        let a = gp_word_addr(c, 0xed8);
        c.g[A0][0] = c.lw(a);
        frame.save(c, S0, 0x10);
        let list_empty = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !list_empty {
            // First array: elements 0 .. count-1 at gp+0xed8.
            c.cycles += 3;
            let a = gp_word_addr(c, 0xee0);
            c.g[V0][0] = c.lw(a);
            let counted = (c.g[V0][0] as i64) > 0;
            c.g[S0][0] = 0;
            if counted {
                loop {
                    c.cycles += 6;
                    let a = gp_word_addr(c, 0xed8);
                    c.g[V1][0] = c.lw(a);
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let slot = c.g[V0][0] as u32;
                    c.g[A0][0] = c.lw(slot);
                    let empty = c.g[A0][0] == 0;
                    let a = gp_word_addr(c, 0xee0);
                    c.g[V0][0] = c.lw(a);
                    if !empty {
                        c.cycles += 3;
                        call_ptr(c, EA_FREE_ADDR, 0x0032_de60)?;
                        let a = gp_word_addr(c, 0xee0);
                        c.g[V0][0] = c.lw(a);
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    let a = gp_word_addr(c, 0xed8);
                    c.g[A0][0] = c.lw(a);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_de40);
                    }
                }
            }
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                let a = gp_word_addr(c, 0xed8);
                c.sw(a, 0u64);
            } else {
                c.cycles += 3;
                call_ptr(c, EA_FREE_ADDR, 0x0032_de84)?;
                let a = gp_word_addr(c, 0xed8);
                c.sw(a, 0u64);
            }
        }
        // The single at gp+0xedc, then the second array's head at gp+0xee8.
        c.cycles += 3;
        let a = gp_word_addr(c, 0xedc);
        c.g[A0][0] = c.lw(a);
        if c.g[A0][0] == 0 {
            let a = gp_word_addr(c, 0xee8);
            c.g[A0][0] = c.lw(a);
        } else {
            c.cycles += 4;
            call_ptr(c, EA_FREE_ADDR, 0x0032_de9c)?;
            let a = gp_word_addr(c, 0xedc);
            c.sw(a, 0u64);
            let a = gp_word_addr(c, 0xee8);
            c.g[A0][0] = c.lw(a);
        }
        // Second array: elements at gp+0xee8 (count gp+0xef0).
        c.cycles += 2;
        let second_empty = c.g[A0][0] == 0;
        let a = gp_word_addr(c, 0xef0);
        c.g[V0][0] = c.lw(a);
        if !second_empty {
            c.cycles += 2;
            let counted = (c.g[V0][0] as i64) > 0;
            c.g[S0][0] = 0;
            if counted {
                c.cycles += 1;
                loop {
                    c.cycles += 6;
                    let a = gp_word_addr(c, 0xee8);
                    c.g[V1][0] = c.lw(a);
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let slot = c.g[V0][0] as u32;
                    c.g[A0][0] = c.lw(slot);
                    let empty = c.g[A0][0] == 0;
                    let a = gp_word_addr(c, 0xef0);
                    c.g[V0][0] = c.lw(a);
                    if !empty {
                        c.cycles += 3;
                        call_ptr(c, EA_FREE_ADDR, 0x0032_ded8)?;
                        let a = gp_word_addr(c, 0xef0);
                        c.g[V0][0] = c.lw(a);
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    let a = gp_word_addr(c, 0xee8);
                    c.g[A0][0] = c.lw(a);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_deb8);
                    }
                }
            }
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                let a = gp_word_addr(c, 0xee8);
                c.sw(a, 0u64);
            } else {
                c.cycles += 3;
                call_ptr(c, EA_FREE_ADDR, 0x0032_defc)?;
                let a = gp_word_addr(c, 0xee8);
                c.sw(a, 0u64);
            }
        }
        // The single at gp+0xeec, then the epilogue.
        c.cycles += 3;
        let a = gp_word_addr(c, 0xeec);
        c.g[A0][0] = c.lw(a);
        let single_empty = c.g[A0][0] == 0;
        frame.restore(c, S0, 0x10);
        if !single_empty {
            c.cycles += 4;
            call_ptr(c, EA_FREE_ADDR, 0x0032_df14)?;
            let a = gp_word_addr(c, 0xeec);
            c.sw(a, 0u64);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The one-sided test of a sphere against a triangle that the centre is in
/// front of ('b_32ff50' of 0x32fe78): the centre projected into the plane
/// (`VU0_TriangleIntersect`, tolerance `gp-0x2d70`), then the triangle edges
/// (`CollSphere_HitsTriangleEdges`) when it is outside. True on a hit. On a
/// miss it leaves v0 = 0 and returns false.
fn collide_triangle_front_one_sided(c: &mut Cpu, frame: &Frame) -> Result<bool, Unwound> {
    c.cycles += 22;
    c.g[A3][0] = c.g[S3][0];
    c.mfc1(2, 0);
    let s0 = c.g[S0][0] as u32;
    c.lqc2(4, s0);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let t20 = frame.at(c, 0x20);
    c.sqc2(5, t20);
    let tol = gp_word_addr(c, -0x2d70);
    c.lwc1(12, tol);
    c.lq(2, t20);
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S4][0];
    c.g[A2][0] = c.g[S5][0];
    let t10 = frame.at(c, 0x10);
    c.sq(2, t10);
    let s1_60 = (c.g[S1][0] as u32).wrapping_add(0x60);
    c.lqc2(3, s1_60);
    c.lqc2(4, t10);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, t20);
    c.lq(2, t20);
    let s3 = c.g[S3][0] as u32;
    c.sq(2, s3);
    let sp0 = frame.at(c, 0);
    c.sq(2, sp0);
    call_ptr(c, TRI_INTERSECT_ADDR, 0x0032_ff28)?;
    if c.g[V0][0] != 0 {
        let s0 = c.g[S0][0] as u32;
        c.lq(3, s0);
        return Ok(true);
    }
    c.cycles += 7;
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S4][0];
    c.g[A3][0] = c.g[S5][0];
    c.g[A0][0] = c.g[S1][0];
    COLL_HITS_EDGES_CALLEE.call(c, 0x0032_ff44)?;
    let missed = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if missed {
        return Ok(false);
    }
    c.cycles += 1;
    let s0 = c.g[S0][0] as u32;
    c.lq(3, s0);
    Ok(true)
}

/// `int CollSphere_CollideTriangle(self, a1, a2, a3, t0, t1, t2, t3)`
/// (0x32fe78): one-sided sphere-triangle collision. Tests the sphere centre
/// against the triangle's plane (`CollSphere_PlaneDistance`, distance at
/// sp+0x30); on the front side it tests the centre inside the triangle and
/// then the edges. A hit writes the penetration-scaled contact normal to `t2`
/// and the corrected centre to `t3`, and returns 1.
pub fn coll_sphere_collide_triangle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S0][0] = c.g[T0][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[S4][0] = c.g[A2][0];
        c.g[S5][0] = c.g[A3][0];
        c.g[S3][0] = c.g[T1][0];
        c.g[S6][0] = c.g[T2][0];
        c.g[S7][0] = c.g[T3][0];

        // Plane distance of the centre (written to sp+0x30).
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S2][0];
        let dist = frame.at(c, 0x30);
        c.g[A3][0] = sx(dist);
        COLL_PLANE_DIST_CALLEE.call(c, 0x0032_fed0)?;
        let dist = frame.at(c, 0x30);
        let plane_front = c.g[V0][0] != 0;
        c.lwc1(0, dist);
        if !plane_front {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else if collide_triangle_front_one_sided(c, &frame)? {
            c.cycles += 7;
            c.g[V0][0] = 1;
            let dist = frame.at(c, 0x30);
            c.lwc1(1, dist);
            let s6 = c.g[S6][0] as u32;
            c.sq(3, s6);
            let s1_70 = (c.g[S1][0] as u32).wrapping_add(0x70);
            c.lwc1(0, s1_70);
            c.sub_s(0, 0, 1);
            let s7 = c.g[S7][0] as u32;
            c.swc1(0, s7);
        }

        // Epilogue.
        c.cycles += 11;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The two-sided test of a sphere against a triangle ('b_3300b0' of
/// 0x32ffa0): the centre projected into the plane (`VU0_TriangleIntersect`,
/// tolerance `gp-0x2d6c`), then the triangle edges when it is outside. True on
/// a hit; on a miss the caller sets v0 = 0 and charges one cycle.
fn collide_triangle_front_two_sided(c: &mut Cpu, frame: &Frame) -> Result<bool, Unwound> {
    c.cycles += 22;
    c.g[A3][0] = c.g[S2][0];
    c.mfc1(2, 0);
    let s = frame.at(c, 0);
    c.lqc2(4, s);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let t30 = frame.at(c, 0x30);
    c.sqc2(5, t30);
    let tol = gp_word_addr(c, -0x2d6c);
    c.lwc1(12, tol);
    c.lq(2, t30);
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S4][0];
    let t20 = frame.at(c, 0x20);
    c.sq(2, t20);
    let s0_60 = (c.g[S0][0] as u32).wrapping_add(0x60);
    c.lqc2(3, s0_60);
    c.lqc2(4, t20);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, t30);
    c.lq(2, t30);
    let s2 = c.g[S2][0] as u32;
    c.sq(2, s2);
    let t10 = frame.at(c, 0x10);
    c.sq(2, t10);
    call_ptr(c, TRI_INTERSECT_ADDR, 0x0033_008c)?;
    let inside = c.g[V0][0] != 0;
    let s = frame.at(c, 0);
    c.lq(3, s);
    if inside {
        return Ok(true);
    }
    c.cycles += 7;
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[A3][0] = c.g[S4][0];
    c.g[A0][0] = c.g[S0][0];
    COLL_HITS_EDGES_CALLEE.call(c, 0x0033_00a8)?;
    let missed = c.g[V0][0] == 0;
    let s = frame.at(c, 0);
    c.lq(3, s);
    Ok(!missed)
}

/// `int CollSphere_CollideTriangle2Sided(self, a1, a2, a3, t0, t1, t2, t3)`
/// (0x32ffa0): as the one-sided test, but the plane's normal is flipped when
/// the centre is behind it, so both faces collide. A hit writes the contact
/// normal to `t2` and the corrected centre to `t3`, and returns 1.
pub fn coll_sphere_collide_triangle_2sided(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0xd0);
        let t0 = c.g[T0][0] as u32;
        c.lq(2, t0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save_ra(c, 0x50);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A2][0];
        c.g[S4][0] = c.g[A3][0];
        c.g[S2][0] = c.g[T1][0];
        c.g[S5][0] = c.g[T2][0];
        c.g[S6][0] = c.g[T3][0];
        let s = frame.at(c, 0);
        c.sq(2, s);

        // Plane distance of the centre (written to sp+0x40).
        c.g[A1][0] = c.g[T0][0];
        c.g[A2][0] = c.g[S1][0];
        let dist = frame.at(c, 0x40);
        c.g[A3][0] = sx(dist);
        COLL_PLANE_DIST_CALLEE.call(c, 0x0032_fff8)?;
        let plane_front = c.g[V0][0] != 0;
        let dist = frame.at(c, 0x40);
        c.lwc1(1, dist);
        let mut hit = false;
        if plane_front {
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_lt_s(1, 0);
            let below = c.fcc();
            let dist = frame.at(c, 0x40);
            c.lwc1(0, dist);
            if below {
                // Behind the plane: flip the normal (and the distance) to -1.
                c.cycles += 10;
                c.neg_s(0, 1);
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.mtc1(1, 1);
                c.swc1(0, dist);
                c.mfc1(2, 1);
                let s = frame.at(c, 0);
                c.lqc2(4, s);
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, s);
                c.lwc1(0, dist);
            }
            hit = collide_triangle_front_two_sided(c, &frame)?;
        }
        if hit {
            c.cycles += 7;
            c.g[V0][0] = 1;
            let dist = frame.at(c, 0x40);
            c.lwc1(1, dist);
            let s5 = c.g[S5][0] as u32;
            c.sq(3, s5);
            let s0_70 = (c.g[S0][0] as u32).wrapping_add(0x70);
            c.lwc1(0, s0_70);
            c.sub_s(0, 0, 1);
            let s6 = c.g[S6][0] as u32;
            c.swc1(0, s6);
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }

        // Epilogue.
        c.cycles += 10;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `Vector ClosestPointOnSegment(out, a, b, p)` (0x330128): the point on the
/// segment `a`..`b` nearest `p`, written to `out`, and `out` returned. The
/// projection parameter is clamped to the ends: `a` when the parameter is
/// <= 0 and `b` when the far end is <= the parameter.
pub fn closest_point_on_segment(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x60);
        c.mtc1(0, 0);
        // p - a (sp+0x10 and sp+0), then b - a (sp+0x10 and sp+0x20).
        let p = c.g[A3][0] as u32;
        c.lqc2(3, p);
        let a = c.g[A1][0] as u32;
        c.lqc2(4, a);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        let t10 = frame.at(c, 0x10);
        c.sqc2(5, t10);
        c.lq(2, t10);
        let s0 = frame.at(c, 0);
        c.sq(2, s0);
        let b = c.g[A2][0] as u32;
        c.lqc2(3, b);
        let a = c.g[A1][0] as u32;
        c.lqc2(4, a);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        let t20 = frame.at(c, 0x20);
        c.sqc2(5, t20);
        c.lq(2, t20);
        c.sq(2, t10);
        // The unit direction (b - a) / |b - a| (sp+0x20).
        c.lqc2(3, t10);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        let t30 = frame.at(c, 0x30);
        c.sqc2(5, t30);
        c.lq(2, t30);
        c.sq(2, t20);
        // t = unit direction . (p - a) (f1).
        c.lqc2(3, t20);
        c.lqc2(5, s0);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 1);
        // |b - a| (f2) against the parameter.
        c.lqc2(3, t10);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 2);
        c.c_le_s(1, 0);
        if c.fcc() {
            // t <= 0: the result is a.
            c.cycles += 3;
            let a = c.g[A1][0] as u32;
            c.lq(2, a);
            let out = c.g[A0][0] as u32;
            c.sq(2, out);
        } else {
            c.cycles += 3;
            c.c_le_s(2, 1);
            if c.fcc() {
                // |b - a| <= t: the result is b.
                c.cycles += 3;
                let b = c.g[A2][0] as u32;
                c.lq(2, b);
                let out = c.g[A0][0] as u32;
                c.sq(2, out);
            } else {
                // a + t * unit direction.
                c.cycles += 14;
                c.mfc1(2, 1);
                c.lqc2(4, t20);
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                let t50 = frame.at(c, 0x50);
                c.sqc2(5, t50);
                c.lq(3, t50);
                let t40 = frame.at(c, 0x40);
                c.sq(3, t40);
                let a = c.g[A1][0] as u32;
                c.lqc2(3, a);
                c.lqc2(4, t40);
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, t50);
                c.lq(2, t50);
                let out = c.g[A0][0] as u32;
                c.sq(2, out);
                let t30 = frame.at(c, 0x30);
                c.sq(2, t30);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A0][0];
        frame.close(c);
        Ok(())
    })
}

/// `CollSphereTree` constructor (0x3303f0): sets the vtable and the counters,
/// copies the owner's sphere (`owner+0x80`, radius from `SphereTree_GetRadius`)
/// into the tree, derives the plus/minus boxes, then allocates the node
/// storage (0x140 bytes) and constructs the sphere tree over it.
pub fn coll_sphere_tree_construct(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S1, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1a10));
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let s1 = c.g[S1][0] as u32;
        let vtable = c.g[V0][0];
        c.sw(s1.wrapping_add(0x50), vtable);
        c.sw(s1.wrapping_add(0x8), 0u64);
        let no_flag = c.g[A2][0] == 0;
        c.sw(s1.wrapping_add(0x4), 0u64);
        if no_flag {
            c.cycles += 2;
            c.g[V0][0] = 1;
            let one = c.g[V0][0];
            c.sw(s1, one);
        } else {
            c.cycles += 2;
            c.sw(s1, 0u64);
        }

        c.cycles += 47;
        let owner = c.g[A1][0];
        c.sw(s1.wrapping_add(0x64), owner);
        c.g[A0][0] = c.g[A1][0];
        c.sw(s1.wrapping_add(0x60), owner);
        let owner_sphere = (c.g[A1][0] as u32).wrapping_add(0x80);
        c.lq(2, owner_sphere);
        let s = frame.at(c, 0);
        c.sq(2, s);
        call_ptr(c, SPHERE_TREE_RADIUS_ADDR, 0x0033_0444)?;

        // Plus and minus boxes of the sphere, from its radius.
        let s = frame.at(c, 0);
        c.lwc1(3, s);
        c.g[A1][0] = 0x49_0000;
        let s4 = frame.at(c, 4);
        c.lwc1(2, s4);
        c.g[A2][0] = 0;
        let s8 = frame.at(c, 8);
        c.lwc1(1, s8);
        c.add_s(4, 3, 0);
        c.add_s(5, 2, 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 7);
        c.add_s(6, 1, 0);
        let s = frame.at(c, 0);
        c.lq(2, s);
        let f10 = frame.at(c, 0x10);
        c.swc1(4, f10);
        c.sub_s(1, 1, 0);
        let f14 = frame.at(c, 0x14);
        c.swc1(5, f14);
        c.sub_s(3, 3, 0);
        let f18 = frame.at(c, 0x18);
        c.swc1(6, f18);
        c.sub_s(2, 2, 0);
        let f1c = frame.at(c, 0x1c);
        c.swc1(7, f1c);
        c.g[A3][0] = 0;
        c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1ab0));
        c.g[A0][0] = 0x140;
        c.lq(3, f10);
        c.swc1(3, f10);
        c.swc1(2, f14);
        c.swc1(1, f18);
        c.swc1(7, f1c);
        c.sq(3, (c.g[S1][0] as u32).wrapping_add(0x20));
        c.lq(2, f10);
        c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x30));
        call_ptr(c, EA_ALLOC_NAMED_ADDR, 0x0033_04c0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, SPHERE_TREE_CTOR_ADDR, 0x0033_04cc)?;
        let node = c.g[S0][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x68), node);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `int SphereTree_CollideSphereNode(self, spherePos, radius, pos, depth,
/// base, outA, outB)` (0x32cdb0): the recursive sphere-against-tree test.
/// Returns 0 when the sphere misses this node's bound; at a leaf (or a node
/// with no children) it writes the contact normal to `outA` and the corrected
/// centre to `outB` and returns 1; otherwise it recurses into the children
/// whose bit is set in the node mask.
pub fn sphere_tree_collide_sphere_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0x110);
        c.g[V0][0] = 0xc;
        frame.save(c, S1, 0xe0);
        frame.save(c, S3, 0xc0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S4, 0xb0);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S5, 0xa0);
        // A3 = depth * 12 (the level stride).
        c.mult(0, A3, S3, V0);
        frame.save(c, S6, 0x90);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S7, 0x80);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, FP, 0x70);
        c.g[S4][0] = c.g[T0][0];
        frame.save_fpr(c, 21, 0x108);
        c.g[S7][0] = c.g[T1][0];
        frame.save(c, S0, 0xf0);
        c.mov_s(21, 12);
        frame.save(c, S2, 0xd0);
        c.g[FP][0] = c.g[T2][0];
        frame.save_ra(c, 0x60);
        frame.save_fpr(c, 20, 0x100);

        // Bound test: the centre distance against radius + the node radius.
        let info = (c.g[S1][0] as u32).wrapping_add(0x98);
        c.g[V1][0] = c.lw(info);
        let scale = (c.g[S1][0] as u32).wrapping_add(0x90);
        c.lwc1(1, scale);
        let levels_ptr = (c.g[V1][0] as u32).wrapping_add(0x20);
        c.g[V0][0] = c.lw(levels_ptr);
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let level = c.g[V0][0] as u32;
        c.lwc1(0, level);
        let s6 = c.g[S6][0] as u32;
        c.lqc2(3, s6);
        let s5 = c.g[S5][0] as u32;
        c.lqc2(4, s5);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        let t10 = frame.at(c, 0x10);
        c.sqc2(5, t10);
        c.mul_s(0, 0, 1);
        c.lq(2, t10);
        let s0 = frame.at(c, 0);
        c.sq(2, s0);
        c.add_s(1, 21, 0);
        c.lqc2(3, s0);
        c.lqc2(5, t10);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 2);
        c.mul_s(0, 1, 1);
        c.c_lt_s(2, 0);
        let near = c.fcc();
        c.g[V0][0] = 0;
        if near {
            let leaf = sphere_node_walk(c, &frame)?;
            if leaf {
                sphere_node_leaf(c, &frame);
            }
        }

        // Epilogue.
        c.cycles += 14;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 21, 0x108);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// The child walk of the sphere-tree node ('b_32cf54' and its loop). Returns
/// `Ok(true)` when the node is a leaf (the caller writes the contact); `Ok(false)`
/// when the result is already in v0 (a child hit, or no child hit).
fn sphere_node_walk(c: &mut Cpu, frame: &Frame) -> Result<bool, Unwound> {
    c.cycles += 7;
    let info = (c.g[S1][0] as u32).wrapping_add(0x98);
    c.g[A0][0] = c.lw(info);
    let masks_ptr = (c.g[A0][0] as u32).wrapping_add(0x28);
    c.g[V0][0] = c.lw(masks_ptr);
    let depth_ptr = (c.g[A0][0] as u32).wrapping_add(0xc);
    c.g[V1][0] = c.lw(depth_ptr);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
    c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let below_depth = c.g[V1][0] != 0;
    let mask_addr = c.g[V0][0] as u32;
    c.g[S2][0] = c.lbu(mask_addr);
    if !below_depth {
        return Ok(true);
    }
    c.cycles += 2;
    let no_children = c.g[S2][0] == 0;
    c.g[V1][0] = 0;
    if no_children {
        return Ok(true);
    }
    c.cycles += 5;
    let levels_ptr = (c.g[A0][0] as u32).wrapping_add(0x20);
    c.g[V0][0] = c.lw(levels_ptr);
    let a3_save = frame.at(c, 0x50);
    let depth_stride = c.g[A3][0];
    c.sw(a3_save, depth_stride);
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    // distance = levels[depth + 1].offset (f20).
    let next_level = (c.g[V0][0] as u32).wrapping_add(0x10);
    c.lwc1(20, next_level);
    c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> ((c.g[V1][0] as u32) & 31)) as u32);

    loop {
        // 'b_32cf3c: child i is present when bit i of the mask is set.
        c.cycles += 3;
        c.g[V0][0] &= 0x1;
        let absent = c.g[V0][0] == 0;
        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if !absent {
            c.cycles += 32;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A1][0] = c.g[S6][0];
            c.mfc1(3, 20);
            let axis = c.g[V0][0] as u32;
            c.lqc2(4, axis);
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            let t30 = frame.at(c, 0x30);
            c.sqc2(5, t30);
            c.mov_s(12, 21);
            c.lq(2, t30);
            let child_pos = frame.at(c, 0x10);
            c.g[A2][0] = sx(child_pos);
            c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            c.g[T1][0] = c.g[S7][0];
            let t20 = frame.at(c, 0x20);
            c.sq(2, t20);
            c.g[T2][0] = c.g[FP][0];
            let s5 = c.g[S5][0] as u32;
            c.lqc2(3, s5);
            c.lqc2(4, t20);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, t30);
            c.lq(3, t30);
            let info = (c.g[S1][0] as u32).wrapping_add(0x98);
            c.g[T0][0] = c.lw(info);
            c.sq(3, child_pos);
            let stride_slot = frame.at(c, 0x50);
            c.g[V1][0] = c.lw(stride_slot);
            let levels_ptr = (c.g[T0][0] as u32).wrapping_add(0x20);
            c.g[V0][0] = c.lw(levels_ptr);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let stride_field = (c.g[V0][0] as u32).wrapping_add(0x8);
            c.g[T0][0] = c.lw(stride_field);
            c.mult(0, 2, 16, 8);
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
            SPHERE_NODE_CALLEE.call(c, 0x0032_cf34)?;
            let child_hit = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if child_hit {
                return Ok(false);
            }
        }
        // Next child: v1 = i + 1 while i + 1 < 8.
        c.cycles += 4;
        c.g[V1][0] = c.g[S0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 8) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> ((c.g[V1][0] as u32) & 31)) as u32);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0032_ceb0);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            return Ok(false);
        }
    }
}

/// The leaf of the sphere-tree node ('cycles += 38' of 0x32cdb0): the contact
/// normal (the centre offset, normalised) scaled by the penetration goes to
/// `outA`, and the corrected centre to `outB`. Leaves v0 = 1.
fn sphere_node_leaf(c: &mut Cpu, frame: &Frame) {
    c.cycles += 38;
    let delta = frame.at(c, 0);
    c.lqc2(3, delta);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    let normal = frame.at(c, 0x20);
    c.sqc2(5, normal);
    c.sqrt_s(0, 2);
    c.lq(3, normal);
    // distance = sum - |delta| (f20); v0 = 1.
    c.sub_s(20, 1, 0);
    c.g[V0][0] = 1;
    let n_slot = frame.at(c, 0x10);
    c.sq(3, n_slot);
    c.mfc1(4, 20);
    c.lqc2(4, n_slot);
    c.qmtc2(4, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let scaled = frame.at(c, 0x30);
    c.sqc2(5, scaled);
    // radius - distance (f0): the normal's length left for the centre.
    c.sub_s(0, 21, 20);
    c.lq(3, scaled);
    let out_a = c.g[S7][0] as u32;
    c.sq(3, out_a);
    c.sq(3, normal);
    c.mfc1(4, 0);
    c.lqc2(4, n_slot);
    c.qmtc2(4, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let pushed = frame.at(c, 0x40);
    c.sqc2(5, pushed);
    c.lq(3, pushed);
    c.sq(3, scaled);
    let s6 = c.g[S6][0] as u32;
    c.lqc2(3, s6);
    c.lqc2(4, scaled);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, pushed);
    c.lq(3, pushed);
    let out_b = c.g[FP][0] as u32;
    c.sq(3, out_b);
    c.sq(3, normal);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The fakes below run only when the machine holds `MAGIC` at `CTRL`.
    const CTRL: u32 = DATA + 0x20_0000;
    const MAGIC: u32 = 0x0569_f00d;

    static FREE_PREV: AtomicUsize = AtomicUsize::new(0);
    static ALLOC_PREV: AtomicUsize = AtomicUsize::new(0);
    static CTOR_PREV: AtomicUsize = AtomicUsize::new(0);

    /// `EA_MemFree`: a leaf that returns (gate on).
    fn oc569_fake_mem_free(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &FREE_PREV, ee::s10::f_00317e98__generated);
        }
        abi::ret(c);
    }

    /// `EA_MemAllocNamed`: returns a fixed block in the scratch area (gate on).
    fn oc569_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &ALLOC_PREV, ee::s10::f_00317d70__generated);
        }
        c.g[V0][0] = (DATA + 0x30_0000) as u64;
        abi::ret(c);
    }

    /// `SphereTree_ctor`: a leaf that returns (gate on).
    fn oc569_fake_tree_ctor(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &CTOR_PREV, ee::s10::f_0032c508__generated);
        }
        abi::ret(c);
    }

    fn hook_fakes() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc569_fake_mem_free,
            &FREE_PREV,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc569_fake_alloc,
            &ALLOC_PREV,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_0032C508),
            oc569_fake_tree_ctor,
            &CTOR_PREV,
        );
    }

    /// Scrambled registers and the game gp.
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x5690_0000_0000_0001 ^ ((seed as u64) << 8) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// `words` random float bit patterns at `addr`.
    fn floats(c: &mut Cpu, r: &mut Rng, addr: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(addr + 4 * i, r.edge_f32());
        }
    }

    /// Self at DATA+0x1000, vectors at DATA+0x2000..; pointers in the argument
    /// registers of the triangle tests.
    fn triangle_fixture(c: &mut Cpu, r: &mut Rng) {
        floats(c, r, DATA + 0x1000, 0x180);
        floats(c, r, DATA + 0x2000, 0x180);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = sx(DATA + 0x2000);
        c.g[A2][0] = sx(DATA + 0x2100);
        c.g[A3][0] = sx(DATA + 0x2200);
        c.g[T0][0] = sx(DATA + 0x2300);
        c.g[T1][0] = sx(DATA + 0x2400);
        c.g[T2][0] = sx(DATA + 0x2500);
        c.g[T3][0] = sx(DATA + 0x2600);
    }

    #[test]
    fn sphere_tree_point_in_triangle_matches() {
        check(
            ee::s11::f_0032d440__generated,
            sphere_tree_point_in_triangle,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 0x32d4);
                c.pc = 0x0032_d440;
                floats(c, &mut r, DATA + 0x100, 0x40);
                c.bus.write32(GAME_GP - 0x2d80, r.f32(1.0e-4));
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x200);
                c.g[A2][0] = sx(DATA + 0x300);
                c.g[A3][0] = sx(DATA + 0x400);
                c.g[T0][0] = sx(DATA + 0x500);
            },
        );
    }

    #[test]
    fn world_free_sphere_tree_cache_matches() {
        hook_fakes();
        check(
            ee::s11::f_0032de20__generated,
            world_free_sphere_tree_cache,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 0x32de);
                c.pc = 0x0032_de20;
                c.bus.write32(CTRL, MAGIC);
                let arr1 = DATA + 0x2000;
                let arr2 = DATA + 0x2100;
                c.bus.write32(GAME_GP + 0xed8, arr1);
                c.bus.write32(GAME_GP + 0xee0, seed % 4);
                c.bus.write32(GAME_GP + 0xee8, arr2);
                c.bus.write32(GAME_GP + 0xef0, (seed / 4) % 4);
                let single = if seed % 3 == 0 { 0 } else { DATA + 0x2200 };
                c.bus.write32(GAME_GP + 0xedc, single);
                let single2 = if seed % 5 == 0 { 0 } else { DATA + 0x2300 };
                c.bus.write32(GAME_GP + 0xeec, single2);
                for i in 0..4u32 {
                    let e1 = if (seed + i) % 3 == 0 {
                        0
                    } else {
                        DATA + 0x2400 + 0x10 * i
                    };
                    let e2 = if (seed + i) % 4 == 0 {
                        0
                    } else {
                        DATA + 0x2500 + 0x10 * i
                    };
                    c.bus.write32(arr1 + 4 * i, e1);
                    c.bus.write32(arr2 + 4 * i, e2);
                }
            },
        );
    }

    #[test]
    fn coll_sphere_collide_triangle_matches() {
        check(
            ee::s11::f_0032fe78__generated,
            coll_sphere_collide_triangle,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 0x32fe);
                c.pc = 0x0032_fe78;
                triangle_fixture(c, &mut r);
            },
        );
    }

    #[test]
    fn coll_sphere_collide_triangle_2sided_matches() {
        check(
            ee::s11::f_0032ffa0__generated,
            coll_sphere_collide_triangle_2sided,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 0x32ff);
                c.pc = 0x0032_ffa0;
                triangle_fixture(c, &mut r);
            },
        );
    }

    #[test]
    fn closest_point_on_segment_matches() {
        check(
            ee::s11::f_00330128__generated,
            closest_point_on_segment,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 0x3301);
                c.pc = 0x0033_0128;
                floats(c, &mut r, DATA + 0x3100, 0x40);
                c.g[A0][0] = sx(DATA + 0x3000);
                c.g[A1][0] = sx(DATA + 0x3100);
                c.g[A2][0] = sx(DATA + 0x3200);
                c.g[A3][0] = sx(DATA + 0x3300);
            },
        );
    }

    #[test]
    fn coll_sphere_tree_construct_matches() {
        hook_fakes();
        check(
            ee::s11::f_003303f0__generated,
            coll_sphere_tree_construct,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 0x3303);
                c.pc = 0x0033_03f0;
                c.bus.write32(CTRL, MAGIC);
                let this = DATA + 0x1000;
                let owner = DATA + 0x2000;
                floats(c, &mut r, owner, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(owner);
                c.g[A2][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn sphere_tree_collide_sphere_node_matches() {
        check(
            ee::s10::f_0032cdb0__generated,
            sphere_tree_collide_sphere_node,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 0x32cd);
                c.pc = 0x0032_cdb0;
                let this = DATA + 0x1000;
                let info = DATA + 0x1100;
                let levels = DATA + 0x1200;
                let masks = DATA + 0x1300;
                // Axes (16 bytes each at self+0), scale at +0x90, tree at +0x98.
                floats(c, &mut r, this, 0x10);
                c.bus.write32(this + 0x90, r.f32(2.0));
                c.bus.write32(this + 0x98, info);
                c.bus.write32(info + 0xc, 2);
                c.bus.write32(info + 0x20, levels);
                c.bus.write32(info + 0x28, masks);
                floats(c, &mut r, levels, 9);
                for k in 0..3u32 {
                    c.bus.write32(levels + 12 * k + 8, 0x40 * (k + 1));
                }
                c.bus.write32(masks, r.next_u32());
                let pos = DATA + 0x1400;
                let centre = DATA + 0x1420;
                floats(c, &mut r, pos, 4);
                floats(c, &mut r, centre, 4);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pos);
                c.g[A2][0] = sx(centre);
                c.g[A3][0] = sx(seed % 3);
                c.g[T0][0] = 0;
                c.g[T1][0] = sx(DATA + 0x1440);
                c.g[T2][0] = sx(DATA + 0x1460);
                c.f[12] = r.f32(8.0);
            },
        );
    }
}
