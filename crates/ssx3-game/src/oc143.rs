//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for 8 cold-path functions — the two rail "grow to fit"
//! update helpers, two collision tests, HUD meter layout, the SHP allocation
//! sizer, BE profile-event recording and world-trigger insertion — each with
//! a differential test against the generated body.
//!
//! Arithmetic is performed with the same micro-operations as the generated
//! code (`c.add_s`/`c.mul_s`/`c.c_lt_s`, `c.mult`, `c.div`, the `ldl`/`ldr`
//! pairs) so the FPU flags, LO/HI and the scratch registers come out
//! identical; calls go through [`crate::call`] with the same registers and
//! the unwind check after each one.

use crate::abi;
use crate::call::{guest, Callee};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0034_9110, float_rail_v50, "FloatRail_v50"),
    (0x0034_9f18, spring_rail_v50, "SpringRail_v50"),
    (
        0x0032_e288,
        coll_segment_overlaps_aabb,
        "CollSegment_OverlapsAABB",
    ),(0x003b_26b8, shp_calc_alloc_size, "SHP_CalcAllocSize"),(
        0x002b_5c68,
        world_trigger_manager_add_trigger,
        "WorldTriggerManager_AddTrigger",
    ),
    (
        0x0033_0250,
        coll_sphere_hits_triangle_edges,
        "CollSphere_HitsTriangleEdges",
    ),
];

/// Direct-call targets (dossier callees, found by address in `generated/ee`).
mod callee {
    use super::{ee, Callee};

    pub const BE_GET_EVENT_RECORD_INDEX: Callee = Callee {
        addr: 0x0014_ab68,
        f: ee::s01::BE_GetEventRecordIndex,
    };
    pub const WORLD_TRIGGER_FIND_INSTANCE: Callee = Callee {
        addr: 0x002b_5bc0,
        f: ee::s08::WorldTriggerManager_FindInstance,
    };
    pub const WORLD_TRIGGER_FIND_FREE: Callee = Callee {
        addr: 0x002b_5b90,
        f: ee::s08::WorldTriggerManager_FindFreeInstance,
    };
    pub const SHP_NORMALIZE_BPP: Callee = Callee {
        addr: 0x003b_2528,
        f: ee::s13::SHP_NormalizeBpp,
    };
    pub const SHP_CALC_IMAGE_SIZE: Callee = Callee {
        addr: 0x003b_2558,
        f: ee::s13::SHP_CalcImageSize,
    };
    pub const SHP_PALETTE_ENTRIES_FOR_BPP: Callee = Callee {
        addr: 0x003b_2688,
        f: ee::s13::SHP_PaletteEntriesForBpp,
    };
    pub const CLOSEST_POINT_ON_SEGMENT: Callee = Callee {
        addr: 0x0033_0128,
        f: ee::s11::ClosestPointOnSegment,
    };
}

// ---------------------------------------------------------------------------
// Helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn at(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global address.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// A 32-bit register value.
#[inline(always)]
fn r32v(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// `off(sp)`.
#[inline(always)]
fn spa(c: &Cpu, off: u32) -> u32 {
    r32v(c, SP).wrapping_add(off)
}

// ---------------------------------------------------------------------------
// Rails.

/// `void FloatRail_v50(FloatRail* rail)` (0x349110): grow the rail's bounding
/// box (`+0x30`: min, `+0x40`: max) over the points at `*(rail+0x70)`
/// (`+0x20` of that array is the count, points are 0x50 bytes each; the first
/// point is skipped), then widen it by 200 units in every direction.
pub fn float_rail_v50(c: &mut Cpu) {
    c.cycles += 9;
    let this = r32v(c, A0);
    // *(rail+0x70): point array; *(rail+0x20): point count.
    c.g[A1][0] = Addr(at(this, 0x70)).lw(c);
    c.g[V1][0] = 1;
    c.g[A2][0] = Addr(at(this, 0x20)).lw(c);
    let points = r32v(c, A1);
    let count = c.g[A2][0];
    c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x10));
    // Seed both corners from the second point (+0x20 of the array).
    c.lq(2, at(points, 0x20));
    c.g[V1][0] = (1i64 < count as i64) as u64;
    c.sq(2, at(this, 0x30));
    let one_point = c.g[V1][0] == 0;
    c.sq(2, at(this, 0x40));
    if !one_point {
        c.cycles += 3;
        c.g[V0][0] = sx(at(points, 0x70));
        let mut p = at(points, 0x70);
        let mut n = r32v(c, A2).wrapping_sub(1);
        loop {
            // min: keep the smaller of each axis (x, y, z).
            c.cycles += 6;
            c.lwc1(1, p);
            c.lwc1(0, at(this, 0x30));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at(this, 0x30));
            }
            c.cycles += 6;
            c.lwc1(1, at(p, 4));
            c.lwc1(0, at(this, 0x34));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at(this, 0x34));
            }
            c.cycles += 6;
            c.lwc1(1, at(p, 8));
            c.lwc1(0, at(this, 0x38));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at(this, 0x38));
            }
            // max: keep the larger of each axis.
            c.cycles += 6;
            c.lwc1(1, p);
            c.lwc1(0, at(this, 0x40));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(this, 0x40));
            }
            c.cycles += 6;
            c.lwc1(1, at(p, 4));
            c.lwc1(0, at(this, 0x44));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(this, 0x44));
            }
            c.cycles += 6;
            c.lwc1(1, at(p, 8));
            c.lwc1(0, at(this, 0x48));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(this, 0x48));
            }
            c.cycles += 3;
            n = n.wrapping_sub(1);
            let more = n != 0;
            c.g[V1][0] = sx(n);
            c.g[V0][0] = sx(p.wrapping_add(0x50));
            if more && event_due(c) {
                deliver_event_here(c, 0x0034_9140);
            }
            p = p.wrapping_add(0x50);
            if !more {
                break;
            }
        }
    }
    c.cycles += 16;
    // Widen by 200 units on each side: one VU0 subtract, one add.
    c.g[AT][0] = 0x4348_0000;
    c.mtc1(1, 0);
    c.sw(spa(c, 0xc), 0);
    c.swc1(0, spa(c, 8));
    c.swc1(0, spa(c, 0));
    c.swc1(0, spa(c, 4));
    c.lqc2(3, at(this, 0x30));
    c.lqc2(4, spa(c, 0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(this, 0x30));
    c.lqc2(3, at(this, 0x40));
    c.lqc2(4, spa(c, 0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(this, 0x40));
    c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x10));
    abi::ret(c);
}

/// `void SpringRail_v50(SpringRail* rail)` (0x349f18): as
/// [`float_rail_v50`], but the points live at `*(rail+0x70)` themselves and
/// are 0x60 bytes each (six floats: position and velocity).
pub fn spring_rail_v50(c: &mut Cpu) {
    c.cycles += 9;
    let this = r32v(c, A0);
    c.g[A1][0] = Addr(at(this, 0x70)).lw(c);
    c.g[V1][0] = 1;
    c.g[A2][0] = Addr(at(this, 0x20)).lw(c);
    let points = r32v(c, A1);
    let count = c.g[A2][0];
    c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x10));
    c.lq(2, points);
    c.g[V1][0] = (1i64 < count as i64) as u64;
    c.sq(2, at(this, 0x30));
    let one_point = c.g[V1][0] == 0;
    c.sq(2, at(this, 0x40));
    if !one_point {
        c.cycles += 3;
        c.g[V0][0] = sx(at(points, 0x60));
        let mut p = at(points, 0x60);
        let mut n = r32v(c, A2).wrapping_sub(1);
        loop {
            c.cycles += 6;
            c.lwc1(1, p);
            c.lwc1(0, at(this, 0x30));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at(this, 0x30));
            }
            c.cycles += 6;
            c.lwc1(1, at(p, 4));
            c.lwc1(0, at(this, 0x34));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at(this, 0x34));
            }
            c.cycles += 6;
            c.lwc1(1, at(p, 8));
            c.lwc1(0, at(this, 0x38));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, at(this, 0x38));
            }
            c.cycles += 6;
            c.lwc1(1, p);
            c.lwc1(0, at(this, 0x40));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(this, 0x40));
            }
            c.cycles += 6;
            c.lwc1(1, at(p, 4));
            c.lwc1(0, at(this, 0x44));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(this, 0x44));
            }
            c.cycles += 6;
            c.lwc1(1, at(p, 8));
            c.lwc1(0, at(this, 0x48));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(this, 0x48));
            }
            c.cycles += 3;
            n = n.wrapping_sub(1);
            let more = n != 0;
            c.g[V1][0] = sx(n);
            c.g[V0][0] = sx(p.wrapping_add(0x60));
            if more && event_due(c) {
                deliver_event_here(c, 0x0034_9f48);
            }
            p = p.wrapping_add(0x60);
            if !more {
                break;
            }
        }
    }
    c.cycles += 16;
    c.g[AT][0] = 0x4348_0000;
    c.mtc1(1, 0);
    c.sw(spa(c, 0xc), 0);
    c.swc1(0, spa(c, 8));
    c.swc1(0, spa(c, 0));
    c.swc1(0, spa(c, 4));
    c.lqc2(3, at(this, 0x30));
    c.lqc2(4, spa(c, 0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(this, 0x30));
    c.lqc2(3, at(this, 0x40));
    c.lqc2(4, spa(c, 0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(this, 0x40));
    c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Collision.

/// `bool CollSegment_OverlapsAABB(Segment* seg, Vec3* boxMin, Vec3* boxMax)`
/// (0x32e288): the segment's centre (`+0x60`) plus its extent (`+0x70`) is its
/// AABB; the test rejects the pair on each axis as the original does.
pub fn coll_segment_overlaps_aabb(c: &mut Cpu) {
    c.cycles += 12;
    let seg = r32v(c, A0);
    let lo = r32v(c, A1);
    let hi = r32v(c, A2);
    c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x20));
    c.lqc2(3, at(seg, 0x60));
    c.lqc2(4, at(seg, 0x70));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, spa(c, 0x10));
    // First: the segment min against the box min.
    c.lwc1(2, at(seg, 0x60));
    c.lwc1(1, lo);
    c.lq(2, spa(c, 0x10));
    c.c_lt_s(2, 1);
    let mut separated = false;
    let skips = !c.fcc();
    c.sq(2, spa(c, 0));
    if !skips {
        c.cycles += 5;
        c.lwc1(0, spa(c, 0));
        c.c_lt_s(0, 1);
        separated = c.fcc();
    }
    // Then the box max against the segment min/max.
    if !separated {
        c.cycles += 5;
        c.lwc1(1, hi);
        c.c_lt_s(1, 2);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 0));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(1, 0);
            separated = c.fcc();
        }
    }
    // y.
    if !separated {
        c.cycles += 6;
        c.lwc1(2, at(seg, 0x64));
        c.lwc1(1, at(lo, 4));
        c.c_lt_s(2, 1);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 4));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(0, 1);
            separated = c.fcc();
        }
    }
    if !separated {
        c.cycles += 5;
        c.lwc1(1, at(hi, 4));
        c.c_lt_s(1, 2);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 4));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(1, 0);
            separated = c.fcc();
        }
    }
    // z.
    if !separated {
        c.cycles += 6;
        c.lwc1(2, at(seg, 0x68));
        c.lwc1(1, at(lo, 8));
        c.c_lt_s(2, 1);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 8));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(0, 1);
            separated = c.fcc();
        }
    }
    if !separated {
        c.cycles += 5;
        c.lwc1(1, at(hi, 8));
        c.c_lt_s(1, 2);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 8));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(1, 0);
            separated = c.fcc();
        }
    }
    if separated {
        c.g[V0][0] = 0;
    } else {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x20));
    abi::ret(c);
}

/// `bool CollSphere_HitsTriangleEdges(Sphere*, Vec3 a, Vec3 b, Vec3 c)`
/// (0x330250): for each of the three triangle edges, clamp the sphere centre
/// onto the edge and compare the distance with the sphere radius.
pub fn coll_sphere_hits_triangle_edges(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let sphere = r32v(c, A0);
        c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0xc0));
        c.sq(17, spa(c, 0xa0));
        c.sq(18, spa(c, 0x90));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, spa(c, 0x80));
        c.sq(20, spa(c, 0x70));
        c.sq(16, spa(c, 0xb0));
        c.sd(spa(c, 0x60), c.g[RA][0]);
        // The three vertices are passed by value; spill them to the frame.
        c.lq(2, r32v(c, A1));
        c.g[A1][0] = 0;
        c.sq(2, spa(c, 0x10));
        c.lq(2, r32v(c, A2));
        c.sq(2, spa(c, 0x20));
        c.lq(3, r32v(c, A3));
        c.sq(3, spa(c, 0x30));
        c.g[S2][0] = sx(spa(c, 0x10));
        c.g[S3][0] = 3;
        c.g[S4][0] = sx(spa(c, 0x40));
        // s0 walks the edges (0, 1, 2), a1 the start vertex, s3 the vertex
        // count for the div below.
        c.g[S0][0] = 1;
        loop {
            c.cycles += 2;
            c.cycles += 32;
            c.div(0, 16, 19);
            c.g[A1][0] = sx(r32v(c, A1) << 4);
            c.g[A1][0] = sx(r32v(c, S2).wrapping_add(r32v(c, A1)));
            c.g[A0][0] = c.g[S4][0];
            c.g[A3][0] = sx(at(sphere, 0x60));
            c.g[A2][0] = c.hi[0];
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 4);
            c.g[A2][0] = sx(r32v(c, S2).wrapping_add(r32v(c, A2)));
            callee::CLOSEST_POINT_ON_SEGMENT.call(c, 0x0033_02cc)?;
            // d = closest - centre; |d|^2 against r^2 (VU0 dot product).
            c.lq(2, spa(c, 0x40));
            c.sq(2, spa(c, 0));
            c.lqc2(3, at(sphere, 0x60));
            c.lqc2(4, spa(c, 0));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, spa(c, 0x50));
            c.lq(2, spa(c, 0x50));
            c.lwc1(1, at(sphere, 0x70));
            c.sq(2, spa(c, 0x40));
            c.lqc2(3, spa(c, 0x40));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 0);
            c.c_lt_s(0, 1);
            let hit = c.fcc();
            c.g[V0][0] = 1;
            if hit {
                break;
            }
            c.cycles += 4;
            let i = c.g[S0][0];
            c.g[A1][0] = i;
            c.g[V0][0] = ((i as i64) < 3i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((i as u32).wrapping_add(1));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0033_02a0);
                }
                continue;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
            break;
        }
        c.cycles += 8;
        c.lq(16, spa(c, 0xb0));
        c.lq(17, spa(c, 0xa0));
        c.lq(18, spa(c, 0x90));
        c.lq(19, spa(c, 0x80));
        c.lq(20, spa(c, 0x70));
        c.g[RA][0] = c.ld(spa(c, 0x60));
        c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0xc0));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// HUD.

/// `void HUDMeter_SetRect(HUDMeter* m, int unused, Rect* src)` (0x20ec18):
/// store the 200-unit-wide meter rectangle scaled/offset from `src` into the
/// meter's layout rects.
pub fn hud_meter_set_rect(c: &mut Cpu) {
    c.cycles += 3;
    let meter = r32v(c, A0);
    let src = r32v(c, A2);
    c.g[V0][0] = Addr(gp(c, -0x988)).lw(c);
    let absent = c.g[V0][0] == 0;
    c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x10));
    if !absent {
        c.cycles += 8;
        c.g[V0][0] = Addr(gp(c, -0x98c)).lw(c);
        c.lwc1(2, gp(c, -0x54f4));
        c.lwc1(0, at(r32v(c, V0), 0x14));
        c.cvt_s_w(0, 0);
        c.lwc1(1, at(r32v(c, V0), 0x34));
        c.mul_s(0, 0, 1);
        c.mul_s(7, 0, 2);
    }
    // Both paths converge on f7 = 30.0.
    c.cycles += 2;
    c.g[AT][0] = 0x41f0_0000;
    c.mtc1(1, 7);
    c.cycles += 53;
    // The source rect is `{x, y, w, h}` at +0, +4, +8, +0xc.
    c.lwc1(2, at(src, 0xc));
    c.g[V0][0] = 0x4d_0000;
    c.lwc1(3, at(src, 8));
    c.lwc1(0, at(src, 4));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(1, 5);
    c.add_s(0, 0, 2);
    c.lwc1(1, src);
    c.mul_s(3, 3, 5);
    c.lwc1(4, 0x004c_8ca0);
    c.lwc1(6, gp(c, -0x54f0));
    c.sub_s(0, 0, 7);
    c.add_s(1, 1, 3);
    c.swc1(0, spa(c, 4));
    c.swc1(1, spa(c, 0));
    // `{x1, y1}` as one doubleword at +0.
    c.ldl(2, spa(c, 7));
    c.ldr(2, spa(c, 0));
    c.sdl(2, at(meter, 7));
    c.sdr(2, at(meter, 0));
    c.lwc1(3, at(src, 4));
    c.swc1(3, at(meter, 8));
    c.lwc1(0, at(src, 0xc));
    c.lwc1(1, at(src, 4));
    c.add_s(1, 1, 0);
    c.sub_s(1, 1, 7);
    c.sub_s(1, 1, 4);
    c.swc1(1, at(meter, 0xc));
    c.sub_s(3, 1, 3);
    c.lwc1(0, at(src, 8));
    c.lwc1(2, at(src, 0));
    c.sub_s(3, 3, 4);
    c.mul_s(0, 0, 5);
    c.mul_s(3, 3, 6);
    c.add_s(2, 2, 0);
    c.swc1(2, at(meter, 0x10));
    c.lwc1(0, at(src, 4));
    c.swc1(3, at(meter, 0x1c));
    c.add_s(0, 0, 4);
    c.swc1(1, at(meter, 0x18));
    c.swc1(0, at(meter, 0x14));
    // `{y2, x2}` at +0x20 and `{h', w'}` at +0x28, copied as doublewords.
    c.ldl(2, at(src, 7));
    c.ldr(2, at(src, 0));
    c.ldl(3, at(src, 0xf));
    c.ldr(3, at(src, 8));
    c.sdl(2, at(meter, 0x27));
    c.sdr(2, at(meter, 0x20));
    c.sdl(3, at(meter, 0x2f));
    c.sdr(3, at(meter, 0x28));
    c.lwc1(0, at(meter, 0x2c));
    c.mul_s(0, 0, 6);
    c.swc1(0, at(meter, 0x30));
    c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Texture / SHP.

/// `int SHP_CalcAllocSize(int w, int h, int bpp, int flags, int pad0,
/// int pad1, Vec4* mip)` (0x3b26b8): raw image size plus the mip-chain and
/// header overhead, rounded as the original does.
pub fn shp_calc_alloc_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x90));
        c.sq(16, spa(c, 0x80));
        c.sq(18, spa(c, 0x60));
        c.g[S0][0] = c.g[A2][0];
        c.sq(17, spa(c, 0x70));
        c.g[S2][0] = c.g[A0][0];
        c.sq(19, spa(c, 0x50));
        c.g[A0][0] = c.g[S0][0];
        c.sq(20, spa(c, 0x40));
        c.g[S3][0] = c.g[T0][0];
        c.sq(22, spa(c, 0x20));
        c.g[S1][0] = c.g[A1][0];
        c.sq(23, spa(c, 0x10));
        c.g[S4][0] = c.g[A3][0];
        c.sq(21, spa(c, 0x30));
        c.g[S6][0] = c.g[T1][0];
        c.sd(spa(c, 0), c.g[RA][0]);
        c.g[RA][0] = sx(0x003b_2704);
        c.g[S7][0] = c.g[T2][0];
        callee::SHP_NORMALIZE_BPP.call(c, 0x003b_2704)?;
        c.g[S5][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x003b_271c);
        c.g[A3][0] = c.g[S3][0];
        callee::SHP_CALC_IMAGE_SIZE.call(c, 0x003b_271c)?;
        c.g[S0][0] = sx(r32v(c, V0).wrapping_add(0x10));
        c.g[V0][0] = ((c.g[S5][0] as i64) < 9) as u64;
        let plain_bpp = c.g[V0][0] == 0;
        if !plain_bpp {
            c.cycles += 2;
            let no_flags = c.g[S4][0] == 0;
            c.g[V1][0] = 0x10;
            if !no_flags {
                // Palette alignment: keep 16-byte blocks, at least one.
                c.cycles += 3;
                c.g[V0][0] = c.g[S4][0] ^ 0xf;
                let no_blocks = c.g[S0][0] == 0;
                if c.g[V0][0] == 0 {
                    c.g[S4][0] = c.g[V1][0];
                }
                if !no_blocks {
                    c.cycles += 3;
                    c.g[V1][0] = sx(r32v(c, S0).wrapping_add(0xf));
                    c.g[V0][0] = 0xffff_fff0;
                    c.g[S0][0] = c.g[V1][0] & c.g[V0][0];
                }
                c.cycles += 13;
                c.g[RA][0] = sx(0x003b_2754);
                c.g[A0][0] = c.g[S5][0];
                callee::SHP_PALETTE_ENTRIES_FOR_BPP.call(c, 0x003b_2754)?;
                // v0 = v0 * flags (the palette bytes), rounded up to 8.
                c.mult(0, 2, 2, 20);
                c.g[A1][0] = sx(r32v(c, S0).wrapping_add(0xf));
                c.g[V1][0] = 0xffff_fff0;
                c.g[A0][0] = 0xffff_ffff_ffff_ffff;
                c.g[A1][0] = c.g[A1][0] & c.g[V1][0];
                c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[V1][0] = sx(r32v(c, V0).wrapping_add(7));
                if c.g[A0][0] != 0 {
                    c.g[V1][0] = c.g[V0][0];
                }
                c.g[V1][0] = sx(((r32v(c, V1) as i32) >> 3) as u32);
                c.g[V1][0] = sx(r32v(c, V1).wrapping_add(0x10));
                c.g[S0][0] = sx(r32v(c, A1).wrapping_add(r32v(c, V1)));
            }
        }
        // Mip chain and the two optional header blocks.
        c.cycles += 2;
        let no_mip = c.g[S6][0] == 0;
        c.g[V0][0] = sx(r32v(c, S0).wrapping_add(8));
        if !no_mip {
            c.cycles += 1;
            c.g[S0][0] = sx(r32v(c, V0).wrapping_add(r32v(c, S6)));
        }
        c.cycles += 2;
        let no_tail = c.g[S7][0] == 0;
        c.g[V0][0] = sx(r32v(c, S0).wrapping_add(0x10));
        if !no_tail {
            c.cycles += 1;
            c.g[S0][0] = sx(r32v(c, V0).wrapping_add(r32v(c, S7)));
        }
        c.cycles += 12;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, spa(c, 0x70));
        c.lq(16, spa(c, 0x80));
        c.lq(18, spa(c, 0x60));
        c.lq(19, spa(c, 0x50));
        c.lq(20, spa(c, 0x40));
        c.lq(21, spa(c, 0x30));
        c.lq(22, spa(c, 0x20));
        c.lq(23, spa(c, 0x10));
        c.g[RA][0] = c.ld(spa(c, 0));
        c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x90));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World triggers.

/// `void WorldTriggerManager_AddTrigger(WorldTriggerManager* mgr, int a1,
/// int a2, int a3, int count, int a5, Vec4* pos)` (0x2b5c68): update the
/// matching trigger instance's radius, or fill the first free instance from
/// the arguments, and bump the instance's use counter.
pub fn world_trigger_manager_add_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0xb0));
        c.sq(16, spa(c, 0x90));
        c.sq(17, spa(c, 0x80));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, spa(c, 0x70));
        c.g[S1][0] = c.g[A2][0];
        c.sq(19, spa(c, 0x60));
        c.g[S2][0] = c.g[A3][0];
        c.sq(20, spa(c, 0x50));
        c.g[S3][0] = c.g[A1][0];
        c.sq(21, spa(c, 0x40));
        c.g[S4][0] = c.g[T0][0];
        c.swc1(20, spa(c, 0xa0));
        c.g[S5][0] = c.g[T1][0];
        c.sd(spa(c, 0x30), c.g[RA][0]);
        c.mov_s(20, 12);
        // The position is passed by value; keep its 12 bytes on the frame.
        let mgr = r32v(c, S0);
        c.g[V0][0] = Addr(at(mgr, 0x334)).lw(c);
        c.ldl(3, at(r32v(c, T2), 7));
        c.ldr(3, at(r32v(c, T2), 0));
        c.g[A0][0] = Addr(at(r32v(c, T2), 8)).lw(c);
        c.sdl(3, spa(c, 7));
        c.sdr(3, spa(c, 0));
        let no_instances = c.g[V0][0] == 0;
        c.sw(spa(c, 8), c.g[A0][0]);
        if !no_instances {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x002b_5cd8);
            c.g[A2][0] = c.g[S1][0];
            callee::WORLD_TRIGGER_FIND_INSTANCE.call(c, 0x002b_5cd8)?;
            let found = c.g[V0][0];
            c.g[V1][0] = found;
            if found != 0 {
                // Existing instance: just take the new radius.
                c.swc1(20, at(found as u32, 0x10));
            } else {
                c.cycles += 24;
                c.lwc1(3, spa(c, 0));
                c.g[A0][0] = c.g[S0][0];
                c.lwc1(0, spa(c, 4));
                c.lwc1(1, spa(c, 8));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 2);
                c.swc1(3, spa(c, 0x20));
                c.swc1(0, spa(c, 0x24));
                c.swc1(1, spa(c, 0x28));
                c.swc1(2, spa(c, 0x2c));
                c.lq(2, spa(c, 0x20));
                c.g[RA][0] = sx(0x002b_5d18);
                c.sq(2, spa(c, 0x10));
                callee::WORLD_TRIGGER_FIND_FREE.call(c, 0x002b_5d18)?;
                let inst = c.g[V0][0] as u32;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = 1;
                c.sw(at(inst, 4), c.g[S3][0]);
                c.sw(at(inst, 0), c.g[V0][0]);
                c.sw(at(inst, 8), c.g[S1][0]);
                c.sw(at(inst, 0x3c), c.g[S2][0]);
                c.sw(at(inst, 0xc), c.g[S4][0]);
                c.swc1(20, at(inst, 0x10));
                c.sw(at(inst, 0x14), c.g[S5][0]);
                c.lq(2, spa(c, 0x10));
                c.sq(2, at(inst, 0x20));
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            c.sw(at(r32v(c, V1), 0x34), c.g[V0][0]);
        }
        c.cycles += 10;
        c.lq(16, spa(c, 0x90));
        c.lq(17, spa(c, 0x80));
        c.lq(18, spa(c, 0x70));
        c.lq(19, spa(c, 0x60));
        c.lq(20, spa(c, 0x50));
        c.lq(21, spa(c, 0x40));
        c.g[RA][0] = c.ld(spa(c, 0x30));
        c.lwc1(20, spa(c, 0xa0));
        c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0xb0));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE profile events.

/// The per-event-record table `BE_GetEventRecordIndex` scans (2 entries of
/// `{id, index}` per event id), and the per-place record table this function
/// indexes by place (`+0x54` of each 100-byte record).
const EVENT_TABLE: u32 = 0x0045_a2f8;
const PLACE_TABLE: u32 = 0x0043_d950;

/// `int BE_RecordCharProfileEventPlace(Profile* p, int event, int place,
/// int value)` (0x152528): record that `event` happened at `place` with
/// `value` in the character profile — bump the per-place counter, keep the
/// best (smallest) value, and bump the event's record count. Returns 1 when
/// recorded, 0 when the event is unknown or the value is worse.
pub fn be_record_char_profile_event_place(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x60));
        c.sq(16, spa(c, 0x50));
        c.sq(17, spa(c, 0x40));
        c.g[S0][0] = c.g[A2][0];
        c.sq(18, spa(c, 0x30));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, spa(c, 0x20));
        c.g[S2][0] = c.g[A3][0];
        c.sq(20, spa(c, 0x10));
        c.g[S3][0] = c.g[A1][0];
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        let no_value = c.g[S2][0] == c.g[S4][0];
        c.sd(spa(c, 0), c.g[RA][0]);
        if no_value {
            // No value: nothing recorded.
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x0015_2568);
            c.g[A1][0] = c.g[S0][0];
            callee::BE_GET_EVENT_RECORD_INDEX.call(c, 0x0015_2568)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = 0x1a;
            let unknown = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = sx(r32v(c, V1) << 3);
            if unknown {
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                // The per-event byte table at profile+0xad0: mark the record
                // seen, and count the places when this value is >= 2.
                let rec = r32v(c, S1).wrapping_add(r32v(c, V0)).wrapping_add(0xad0);
                c.g[A2][0] = sx(rec);
                c.cycles += 6;
                c.g[A0][0] = 1;
                c.g[V1][0] = (c.g[S2][0] < 2) as u64;
                let count_place = c.g[V1][0] == 0;
                c.g[A2][0] = sx(rec);
                c.sb(rec, c.g[A0][0]);
                if count_place {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(at(rec, 2)).lhu(c);
                    c.g[V0][0] = sx(r32v(c, V0).wrapping_add(1));
                    c.sh(at(rec, 2), c.g[V0][0]);
                }
                // Keep the best (smallest) value for the record: 0 = the
                // record was unset (run the per-place counters), 1 = it was
                // improved, 2 = the new value is worse (nothing recorded).
                let mut outcome = 0u32;
                c.cycles += 3;
                c.g[V0][0] = Addr(at(rec, 1)).lb(c);
                let unset = c.g[V0][0] == c.g[S4][0];
                let better = (c.g[S2][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = better as u64;
                if !unset {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        outcome = 2;
                    } else {
                        c.cycles += 2;
                        c.sb(at(rec, 1), c.g[S2][0]);
                        outcome = 1;
                    }
                }
                if outcome != 2 {
                    if outcome == 0 {
                        // Per-place record: 100 bytes each, the counter index
                        // at +0x54, counters at profile+0xba0 (place 0) or
                        // +0xba4 (places 1..3); places >= 4 have no counter.
                        c.cycles += 7;
                        c.g[V1][0] = 0x64;
                        c.g[V0][0] = 0x44_0000;
                        c.mult(0, 3, 19, 3);
                        c.g[V0][0] = sx(r32v(c, V0).wrapping_sub(0x26b0));
                        c.g[V0][0] = sx(r32v(c, V0).wrapping_add(r32v(c, V1)));
                        let have_place = c.g[S0][0] != 0;
                        c.g[V1][0] = Addr(at(r32v(c, V0), 0x54)).lw(c);
                        let mut bump = have_place;
                        if !have_place {
                            c.cycles += 3;
                            c.g[A0][0] = sx(r32v(c, V1) << 3);
                            c.g[V1][0] = sx(r32v(c, S1).wrapping_add(0xba0));
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = sx(r32v(c, S0).wrapping_sub(1));
                            c.g[V0][0] = (c.g[V0][0] < 3) as u64;
                            let no_counter = c.g[V0][0] == 0;
                            c.g[A0][0] = sx(r32v(c, V1) << 3);
                            if no_counter {
                                bump = false;
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = sx(r32v(c, S1).wrapping_add(0xba4));
                            }
                        }
                        if bump {
                            c.cycles += 4;
                            c.g[V1][0] = sx(r32v(c, V1).wrapping_add(r32v(c, A0)));
                            c.g[V0][0] = Addr(r32v(c, V1)).lw(c);
                            c.g[V0][0] = sx(r32v(c, V0).wrapping_add(1));
                            c.sw(r32v(c, V1), c.g[V0][0]);
                        }
                    }
                    c.cycles += 1;
                    c.sb(at(rec, 1), c.g[S2][0]);
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                } else {
                    c.g[V0][0] = 0;
                }
            }
        }
        c.cycles += 8;
        c.lq(16, spa(c, 0x50));
        c.lq(17, spa(c, 0x40));
        c.lq(18, spa(c, 0x30));
        c.lq(19, spa(c, 0x20));
        c.lq(20, spa(c, 0x10));
        c.g[RA][0] = c.ld(spa(c, 0));
        c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x60));
        abi::ret(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{compare, scramble, seed_count, Rng, DATA, GAME_GP};

    /// Scrambled registers and a game-like `gp`.
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x143_0000 ^ ((seed as u64) << 11) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// Compare hard state, caller-saved "soft" state and LO/HI for every seed.
    fn check_all(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seed_count(seeds) {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lohi: {:#?}", r.lohi);
        }
    }

    fn put3(c: &mut Cpu, addr: u32, v: [u32; 3]) {
        for (i, x) in v.into_iter().enumerate() {
            c.bus.write32(addr + 4 * i as u32, x);
        }
    }

    fn floats(c: &mut Cpu, r: &mut Rng, addr: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(addr + 4 * i, r.edge_f32());
        }
    }

    // -----------------------------------------------------------------------

    #[test]
    fn float_rail_v50_matches() {
        check_all(ee::s11::f_00349110__generated, float_rail_v50, 300, &|c, seed| {
            let mut r = base(c, seed, 0x34_9110);
            c.g[A0][0] = sx(DATA);
            let count = if seed % 5 == 0 {
                seed % 2
            } else {
                2 + r.next_u32() % 40
            };
            c.bus.write32(DATA + 0x70, DATA + 0x2000);
            c.bus.write32(DATA + 0x20, count);
            // The first point comes from +0x20 of the array, the rest from
            // +0x70 with a 0x50 stride.
            floats(c, &mut r, DATA + 0x2000, count * 0x50 / 4 + 8);
            floats(c, &mut r, DATA, 2); // the `lq` seeds from +0x30/from +0x40
            floats(c, &mut r, DATA + 0x10, 2);
            for off in [0x30u32, 0x34, 0x38, 0x40, 0x44, 0x48] {
                c.bus.write32(DATA + off, r.edge_f32());
            }
        });
    }

    #[test]
    fn spring_rail_v50_matches() {
        check_all(ee::s11::f_00349f18__generated, spring_rail_v50, 300, &|c, seed| {
            let mut r = base(c, seed, 0x34_9f18);
            c.g[A0][0] = sx(DATA);
            let count = if seed % 5 == 0 {
                seed % 2
            } else {
                2 + r.next_u32() % 40
            };
            c.bus.write32(DATA + 0x70, DATA + 0x2000);
            c.bus.write32(DATA + 0x20, count);
            floats(c, &mut r, DATA + 0x2000, count * 0x60 / 4 + 8);
            for off in [0x30u32, 0x34, 0x38, 0x40, 0x44, 0x48] {
                c.bus.write32(DATA + off, r.edge_f32());
            }
        });
    }

    #[test]
    fn coll_segment_overlaps_aabb_matches() {
        check_all(
            ee::s11::f_0032e288__generated,
            coll_segment_overlaps_aabb,
            300,
            &|c, seed| {
                let mut r = base(c, seed, 0x32_e288);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x120);
                match seed % 3 {
                    0 => {
                        // Overlapping: centre 10, half extents 1, box [0, 20].
                        put3(
                            c,
                            DATA + 0x60,
                            [10.0f32.to_bits(), 10.0f32.to_bits(), 10.0f32.to_bits()],
                        );
                        put3(
                            c,
                            DATA + 0x70,
                            [1.0f32.to_bits(), 1.0f32.to_bits(), 1.0f32.to_bits()],
                        );
                        put3(c, DATA + 0x100, [0; 3]);
                        put3(c, DATA + 0x120, [20.0f32.to_bits(); 3]);
                    }
                    1 => {
                        // Separated on the first axis.
                        put3(c, DATA + 0x60, [10.0f32.to_bits(); 3]);
                        put3(c, DATA + 0x70, [1.0f32.to_bits(); 3]);
                        put3(c, DATA + 0x100, [100.0f32.to_bits(); 3]);
                        put3(c, DATA + 0x120, [200.0f32.to_bits(); 3]);
                    }
                    _ => {
                        floats(c, &mut r, DATA + 0x60, 8);
                        floats(c, &mut r, DATA + 0x100, 3);
                        floats(c, &mut r, DATA + 0x120, 3);
                    }
                }
                // The VU0 sum is stored to the frame first.
                floats(c, &mut r, DATA + 0x80, 8);
            },
        );
    }

        #[test]
    fn coll_sphere_hits_triangle_edges_matches() {
        check_all(
            ee::s11::f_00330250__generated,
            coll_sphere_hits_triangle_edges,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 0x33_0250);
                c.g[A0][0] = sx(DATA); // sphere: centre +0x60, radius +0x70
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x120);
                c.g[A3][0] = sx(DATA + 0x140);
                match seed % 3 {
                    0 => {
                        // Tiny radius: every edge is tested.
                        put3(c, DATA + 0x60, [0; 3]);
                        c.bus.write32(DATA + 0x70, 0.001f32.to_bits());
                    }
                    1 => {
                        // Huge radius: the first edge already hits.
                        put3(c, DATA + 0x60, [0; 3]);
                        c.bus.write32(DATA + 0x70, 1.0e6f32.to_bits());
                    }
                    _ => {
                        floats(c, &mut r, DATA + 0x60, 4);
                        c.bus.write32(DATA + 0x70, r.edge_f32());
                    }
                }
                for base_off in [0x100u32, 0x120, 0x140] {
                    match seed % 3 {
                        0 => put3(
                            c,
                            DATA + base_off,
                            [
                                (base_off as f32).to_bits(),
                                2.0f32.to_bits(),
                                3.0f32.to_bits(),
                            ],
                        ),
                        _ => floats(c, &mut r, DATA + base_off, 4),
                    }
                }
            },
        );
    }

    #[test]
    fn shp_calc_alloc_size_matches() {
        check_all(ee::s13::f_003b26b8__generated, shp_calc_alloc_size, 300, &|c, seed| {
            let mut r = base(c, seed, 0x3b_26b8);
            let w = 1 + (r.next_u32() % 64);
            let h = 1 + (r.next_u32() % 64);
            let bpp = match seed % 4 {
                0 => r.next_u32() % 8,
                1 => 8 + r.next_u32() % 32,
                2 => 4,
                _ => 32,
            };
            let flags = match seed % 3 {
                0 => 0,
                1 => 0xf,
                _ => r.next_u32() & 0xf0,
            };
            c.g[A0][0] = w as u64;
            c.g[A1][0] = h as u64;
            c.g[A2][0] = bpp as u64;
            c.g[A3][0] = flags as u64;
            c.g[T0][0] = (r.next_u32() % 0x40) as u64;
            c.g[T1][0] = if seed % 4 == 0 {
                0
            } else {
                (r.next_u32() % 0x40) as u64
            };
            c.g[T2][0] = sx(DATA);
            floats(c, &mut r, DATA, 4);
        });
    }

    #[test]
    fn world_trigger_manager_add_trigger_matches() {
        check_all(
            ee::s08::f_002b5c68__generated,
            world_trigger_manager_add_trigger,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 0x2b_5c68);
                let mgr = DATA;
                let a1 = 0x11u64;
                let a2 = 0x22u64;
                let a3 = 0x33u64;
                c.g[A0][0] = sx(mgr);
                c.g[A1][0] = a1;
                c.g[A2][0] = a2;
                c.g[A3][0] = a3;
                c.g[T0][0] = (r.next_u32() % 8) as u64;
                c.g[T1][0] = (r.next_u32() % 8) as u64;
                c.g[T2][0] = sx(DATA + 0x3000);
                for i in 0..4 {
                    c.bus.write32(DATA + 0x3000 + 4 * i, r.next_u32());
                }
                c.f[12] = r.f32(100.0);
                match seed % 4 {
                    0 => c.bus.write32(mgr + 0x334, 0),
                    _ => {
                        c.bus.write32(mgr + 0x334, DATA + 0x2000);
                        // Instance 0: 0x70 bytes; 0 means free.
                        for i in 0..(40 * 0x70 / 4) {
                            c.bus.write32(DATA + 0x2000 + 4 * i, 0);
                        }
                        if seed % 2 == 0 {
                            // A matching instance: the found path.
                            c.bus.write32(DATA + 0x2000 + 4, a1 as u32);
                            c.bus.write32(DATA + 0x2000 + 8, a2 as u32);
                            c.bus.write32(DATA + 0x2000 + 0x3c, a3 as u32);
                        } else {
                            c.bus.write32(DATA + 0x2000, 1);
                            c.bus.write32(DATA + 0x2000 + 4, 0x99);
                        }
                    }
                }
            },
        );
    }

    }
