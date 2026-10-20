//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for path, race progress and collision constructors, each with
//! a differential test against the generated body.
//!
//! - **Path queries** (`Path_QueryEvents`, `Path_GetPointAtDistance`): the
//!   range-list filter and the walk along a track's segments.
//! - **Race progress** (`RaceProgress_UpdateRider`,
//!   `RaceProgress_ProjectRider`): the per-rider distance along the course,
//!   its clamped step and the lateral offset from the zone line.
//! - **Collision** (`CollSegment_ctor`, `CollHull_ctor`): the capsule and the
//!   hull constructors.
//! - **Render** (`PS2BezierMan_v21`, `PS2BezierMan_TestPatchBounds`,
//!   `PS2GraphicsMan_CullAgainstLights`): the bezier manager's virtual
//!   entry, the patch-bounds test and the per-light culling pass.
//! - **Sound** (`PATH_PlatRefreshStatus`): the monitor slot refresh.
//!
//! The functions keep the original's VU0 macro operations (through
//! [`ssx3_ee_rt::vu0`]) and its FPU instructions (through the `Cpu` `*_s`
//! helpers, which are the hardware model, never native `f32`), so VF/ACC/Q
//! and the FPU registers end as the original leaves them. Calls follow
//! [`crate::call`]: registers at each call as the original has them, the
//! unwind check after every call, and the original's event checks at loop
//! back edges.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002a_efa8,
        path_plat_refresh_status,
        "PATH_PlatRefreshStatus",
    ),
    (
        0x0021_0618,
        race_progress_update_rider,
        "RaceProgress_UpdateRider",
    ),
    (
        0x0021_0820,
        race_progress_project_rider,
        "RaceProgress_ProjectRider",
    ),
    (0x0026_aa80, path_query_events, "Path_QueryEvents"),
    (
        0x0026_ab20,
        path_get_point_at_distance,
        "Path_GetPointAtDistance",
    ),
    (0x0032_e100, coll_segment_ctor, "CollSegment_ctor"),
    (0x0032_f650, coll_hull_ctor, "CollHull_ctor"),
    (0x0038_b338, ps2bezier_man_v21, "PS2BezierMan_v21"),
    (
        0x0038_ca08,
        ps2bezier_man_test_patch_bounds,
        "PS2BezierMan_TestPatchBounds",
    ),
    (
        0x0037_df88,
        ps2graphics_man_cull_against_lights,
        "PS2GraphicsMan_CullAgainstLights",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    /// `PS2GraphicsMan_CullAgainstLights`.
    pub const CULL_AGAINST_LIGHTS: Callee = Callee {
        addr: 0x0037_df88,
        f: ee::s12::PS2GraphicsMan_CullAgainstLights,
    };
    /// `float RaceProgress_ProjectRider(int i)`.
    pub const PROJECT_RIDER: Callee = Callee {
        addr: 0x0021_0820,
        f: ee::s05::RaceProgress_ProjectRider,
    };

    // Cross-shard calls the generator emitted as `call_indirect`.
    pub const RIDER_GET_DIST_TO_FINISH: u32 = 0x0011_3128;
    pub const RIDER_SPHERE_TREE_CTOR: u32 = 0x0032_9910;
    pub const RIDER_SPHERE_TREE_GET_BOUNDS: u32 = 0x0032_9dc8;
    pub const SND_LOCK: u32 = 0x003b_58a0;
    pub const SND_UNLOCK: u32 = 0x003b_58d8;
    pub const SND_STREAM_POSITION: u32 = 0x003b_6670;
    pub const SND_SUB_TIME: u32 = 0x003b_8ad8;
    pub const SND_SUB_STATUS: u32 = 0x003b_86c0;
    pub const SND_STREAM_INFO: u32 = 0x003b_7b48;
    pub const SND_SUB_STATE: u32 = 0x003b_7c40;
    pub const SND_STATUS: u32 = 0x003b_7f18;
}

/// The monitor slots `PATH_PlatRefreshStatus` walks (`D_004D3E98`, 24
/// pointers) and the fields of one slot (`sMonSlot`) and its sub-streams.
mod monitor {
    pub const TABLE: u32 = 0x004d_3e98;
    pub const SLOTS: u64 = 24;

    pub const H: u32 = 0x0;
    pub const SUB: u32 = 0x4;
    pub const STATUS: u32 = 0xc;
    pub const F48: u32 = 0x48;
    pub const F4C: u32 = 0x4c;
    pub const F50: u32 = 0x50;
    pub const F54: u32 = 0x54;
    pub const BUSY: u32 = 0x58;
    pub const F5C: u32 = 0x5c;
    pub const F60: u32 = 0x60;
    pub const IS_MEM: u32 = 0x64;
    pub const DIRTY: u32 = 0x65;
    pub const COUNT: u32 = 0x66;
    pub const SUBS: u32 = 0x6c;

    /// `sMonSub`: size, id, pos.
    pub const SUB_SIZE: u32 = 0x0;
    pub const SUB_ID: u32 = 0x4;
    pub const SUB_POS: u32 = 0x8;
    pub const SUB_STRIDE: u32 = 0xc;

    /// `sStrmState` on the stack: state, a, b, c.
    pub const ST_STATE: u32 = 0x10;
    pub const ST_A: u32 = 0x14;
    pub const ST_B: u32 = 0x18;
    pub const ST_C: u32 = 0x1c;

    /// `sStrmHdr.rate` and `sStrmPos.samples` within the stack block.
    pub const HDR_RATE: u32 = 0x20;
    pub const POS_SAMPLES: u32 = 0xa0;
}

/// The race-progress state: the rider slots (`0x4d0000-0x7438`, stride 0x18),
/// the zone/course-segment table at `gp-0x99c`, the course at `gp-0x9a0` and
/// the world at `gp-0x848`.
mod progress {
    pub const SLOTS: u32 = 0x004c_8bc8;
    pub const ZONES: i32 = -0x99c;
    pub const COURSE: i32 = -0x9a0;
    pub const WORLD: i32 = -0x848;
    /// The step limit (138.8889) at `gp-0x54ac`.
    pub const STEP_LIMIT: i32 = -0x54ac;

    pub const ACTIVE: u32 = 0x0;
    pub const SEG: u32 = 0x4;
    pub const DIST: u32 = 0x8;
    pub const POS: u32 = 0xc;
    pub const E: u32 = 0x14;
    pub const STRIDE: u32 = 0x18;

    /// Course: segment count at +0, distance at +0x10.
    pub const COURSE_COUNT: u32 = 0x0;
    pub const COURSE_DIST: u32 = 0x10;
    /// One segment (`sZone`): scale x/y at +0/+4, origin x/y at +8/+0xc,
    /// distance at +0x10 (stride 0x14).
    pub const SEG_DIST: u32 = 0x10;
    pub const SEG_STRIDE: u32 = 0x14;

    /// Race manager: riders at +0x28, rider count at +0x78.
    pub const RIDERS: u32 = 0x28;
    pub const RIDER_COUNT: u32 = 0x78;
    pub const RIDER_POS: u32 = 0x6c0;
    pub const RIDER_DIST: u32 = 0x4d0;

    pub const ZONE_SX: u32 = 0x0;
    pub const ZONE_SY: u32 = 0x4;
    pub const ZONE_X: u32 = 0x8;
    pub const ZONE_Y: u32 = 0xc;
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// A float's bit pattern (test fixtures).
#[inline(always)]
#[cfg(test)]
fn f(x: f32) -> u32 {
    x.to_bits()
}

// ---------------------------------------------------------------------------
// Path_QueryEvents.

/// `int Path_QueryEvents(const PathRangeList* list, PathRange* out, int max,
/// float lo, float hi)` (0x26aa80): copies the ranges that overlap
/// `[lo, hi]` into `out` (16 bytes each), stopping at `max`, and returns how
/// many were copied. A range list is `{int count; Range* items}` and a range
/// `{u32 a, b; float min, max}`. Nothing is copied when `hi < lo`.
pub fn path_query_events(c: &mut Cpu) {
    c.cycles += 4;
    c.c_lt_s(13, 12); // hi < lo
    if c.fcc() {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    let list = r32(c, A0);
    c.g[V0][0] = c.lw(list); // count
    c.cycles += 3;
    c.g[T0][0] = 0; // i
    let positive = (c.g[V0][0] as i64) > 0;
    c.g[A3][0] = 0; // copied
    if positive {
        c.cycles += 1;
        c.g[V0][0] = c.lw(list.wrapping_add(4)); // items
        'walk: loop {
            'skip: {
                c.cycles += 7;
                let item = r32(c, V0).wrapping_add(r32(c, T0) << 4);
                c.g[V1][0] = sx(item);
                c.lwc1(0, item.wrapping_add(8)); // range.min
                c.c_lt_s(13, 0); // hi < min
                if c.fcc() {
                    c.g[V0][0] = c.lw(list);
                    break 'skip;
                }
                c.cycles += 5;
                c.lwc1(0, item.wrapping_add(0xc)); // range.max
                c.c_lt_s(0, 12); // max < lo
                if c.fcc() {
                    c.g[V0][0] = c.lw(list);
                    break 'skip;
                }
                c.cycles += 11;
                // `*out++ = *item` (the original's ldl/ldr + sdl/sdr pair).
                let dst = r32(c, A1);
                c.ldl(2, item.wrapping_add(7));
                c.ldr(2, item);
                c.ldl(9, item.wrapping_add(0xf));
                c.ldr(9, item.wrapping_add(8));
                c.sdl(2, dst.wrapping_add(7));
                c.sdr(2, dst);
                c.sdl(9, dst.wrapping_add(0xf));
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.sdr(9, dst.wrapping_add(8));
                let full = c.g[A3][0] == c.g[A2][0];
                c.g[A1][0] = sx(dst.wrapping_add(0x10));
                if full {
                    break 'walk;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(list);
            }
            c.cycles += 4;
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
            let more = (c.g[T0][0] as i64) < (c.g[V0][0] as i64);
            c.g[V0][0] = more as u64;
            if !more {
                break;
            }
            c.g[V0][0] = c.lw(list.wrapping_add(4));
            if event_due(c) {
                deliver_event_here(c, 0x0026_aaa8);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Path_GetPointAtDistance.

/// `Vec4 Path_GetPointAtDistance(const PathTrack* track, bool* in_range,
/// float t)` (0x26ab20): the point `t` along the track, written through the
/// hidden struct-return pointer in `a0`. The track has its start point at
/// +0xc (x, y, z), its segment count at +8 and its segments at +0x18; a
/// segment is `{dir x, dir y, dir z, len}`. `*in_range` is 1 while `t` is on
/// the track and 0 once it runs past the end (when it is not null).
pub fn path_get_point_at_distance(c: &mut Cpu) {
    c.cycles += 12;
    let track = r32(c, A1);
    let frame = Frame::open(c, 0x40);
    let sp = sp(c);
    // The start point, w = 1.
    c.lwc1(2, track.wrapping_add(0xc));
    c.g[T0][0] = c.g[A0][0];
    c.lwc1(1, track.wrapping_add(0x10));
    c.lwc1(0, track.wrapping_add(0x14));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.swc1(2, sp);
    c.swc1(1, sp.wrapping_add(4));
    c.swc1(0, sp.wrapping_add(8));
    let no_range = c.g[A2][0] == 0;
    c.swc1(3, sp.wrapping_add(0xc));
    if !no_range {
        c.cycles += 2;
        c.g[V0][0] = 1;
        c.sw(r32(c, A2), 1);
    }
    c.cycles += 2;
    c.g[A3][0] = 0; // segment index
    c.g[V0][0] = c.lw(track.wrapping_add(0x18)); // segments
    loop {
        // The segment, or the end of the track.
        c.cycles += 4;
        c.g[V1][0] = sx(r32(c, A3) << 4);
        c.g[A0][0] = c.lw(track.wrapping_add(8)); // segment count
        let past_end = c.g[A3][0] == c.g[A0][0];
        let seg = r32(c, V0).wrapping_add(r32(c, V1));
        c.g[V0][0] = sx(seg);
        if past_end {
            c.cycles += 2;
            if c.g[A2][0] != 0 {
                c.sw(r32(c, A2), 0);
            }
            c.cycles += 2;
            c.lq(2, sp); // the point built so far
            break;
        }
        // `t < seg.len`: the point lies on this segment.
        c.cycles += 5;
        c.lwc1(2, seg.wrapping_add(0xc));
        c.c_lt_s(12, 2);
        if !c.fcc() {
            // Advance to the segment's end: `pos += dir * len`.
            c.cycles += 22;
            c.lwc1(1, seg);
            c.sub_s(12, 12, 2);
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
            c.swc1(1, sp.wrapping_add(0x20));
            c.lwc1(0, seg.wrapping_add(4));
            c.swc1(0, sp.wrapping_add(0x24));
            c.lwc1(1, seg.wrapping_add(8));
            c.sw(sp.wrapping_add(0x2c), 0);
            c.swc1(1, sp.wrapping_add(0x28));
            c.mfc1(3, 2);
            c.lqc2(4, sp.wrapping_add(0x20));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp.wrapping_add(0x30));
            c.lq(2, sp.wrapping_add(0x30));
            c.sq(2, sp.wrapping_add(0x10));
            c.lqc2(3, sp);
            c.lqc2(4, sp.wrapping_add(0x10));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp);
            c.g[V0][0] = c.lw(track.wrapping_add(0x18));
            if event_due(c) {
                deliver_event_here(c, 0x0026_ab60);
            }
            continue;
        }
        // The point on this segment: `pos += dir * t`.
        c.cycles += 18;
        c.lwc1(0, seg);
        c.swc1(0, sp.wrapping_add(0x20));
        c.lwc1(1, seg.wrapping_add(4));
        c.swc1(1, sp.wrapping_add(0x24));
        c.lwc1(0, seg.wrapping_add(8));
        c.sw(sp.wrapping_add(0x2c), 0);
        c.swc1(0, sp.wrapping_add(0x28));
        c.mfc1(3, 12);
        c.lqc2(4, sp.wrapping_add(0x20));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp.wrapping_add(0x30));
        c.lq(2, sp.wrapping_add(0x30));
        c.sq(2, sp.wrapping_add(0x10));
        c.lqc2(3, sp);
        c.lqc2(4, sp.wrapping_add(0x10));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp.wrapping_add(0x30));
        c.lq(2, sp.wrapping_add(0x30));
        break;
    }
    c.cycles += 4;
    c.sq(2, r32(c, T0));
    c.g[V0][0] = c.g[T0][0];
    frame.close(c);
}

// ---------------------------------------------------------------------------
// RaceProgress_ProjectRider.

/// `float RaceProgress_ProjectRider(int i)` (0x210820): the rider's lateral
/// offset from its zone line. The zone index is the slot's `+4`, the zone
/// `{scale x, scale y, origin x, origin y, dist}` at `zones + zone * 0x14`;
/// the offset is `(pos.x - origin.x) * scale.x + (pos.y - origin.y) *
/// scale.y`, with the rider position read through `rider+0x6c0`'s virtual
/// slot at `+0x2c`. An active slot also has its `+0x14` cleared.
pub fn race_progress_project_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0x40);
        let world = c.lw(gp(c, progress::WORLD));
        c.g[A1][0] = world;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        let idx = r32(c, A0);
        c.g[S2][0] = c.g[A0][0];
        c.g[A0][0] = sx((idx as u32) << 2);
        let zones = c.lw(gp(c, progress::ZONES));
        c.g[S1][0] = zones;
        c.g[V1][0] = c.lw((world as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xc));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(progress::RIDERS));
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(progress::RIDER_POS));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(progress::RIDER_POS));
        c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x2c));
        let v05 = r32(c, V0);
        c.g[RA][0] = sx(0x0021_0868);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        call_ptr(c, v05, 0x0021_0868)?;
        // The slot (0x18 bytes) and the zone offset (0x14 bytes); the
        // original's `mult`s leave the two products in a3 and v1.
        c.g[V1][0] = 0x18;
        c.g[A0][0] = 0x4d_0000;
        c.mult(0, S2, S2, V1);
        c.g[T1][0] = sx(0x4d_0000u32.wrapping_sub(0x7438));
        c.lq(3, r32(c, V0)); // the position the virtual call returned
        c.g[A1][0] = progress::SEG_STRIDE as u64;
        let sp = sp(c);
        c.sq(3, sp);
        c.g[A0][0] = sx(r32(c, T1).wrapping_add(r32(c, S2)));
        c.g[A2][0] = sx(r32(c, S2).wrapping_add(r32(c, T1)));
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(progress::SEG));
        c.lwc1(2, sp.wrapping_add(4));
        c.mult(0, A3, V0, A1);
        c.mult(0, V1, V0, A1);
        c.lwc1(1, sp);
        c.g[T0][0] = c.lw(r32(c, A2));
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, S1)));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S1)));
        c.lwc1(0, r32(c, A0).wrapping_add(progress::ZONE_X));
        c.g[A3][0] = c.g[A0][0];
        c.lwc1(3, r32(c, V1).wrapping_add(progress::ZONE_Y));
        c.g[V0][0] = c.g[A0][0];
        c.sub_s(1, 1, 0);
        c.lwc1(4, r32(c, V0).wrapping_add(progress::ZONE_SY));
        c.sub_s(2, 2, 3);
        c.lwc1(0, r32(c, A3));
        c.mul_s(1, 1, 0);
        c.mul_s(2, 2, 4);
        let inactive = c.g[T0][0] == 0;
        c.add_s(0, 2, 1);
        if !inactive {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, S2)));
            c.sw(r32(c, V0).wrapping_add(progress::E), 0);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RaceProgress_UpdateRider.

/// `void RaceProgress_UpdateRider(int i)` (0x210618): the rider's distance
/// along the course. `d = course.dist - Rider_GetDistToFinish(rider)` is
/// clamped to the slot's previous distance ±138.8889 unless the slot is
/// active; the slot's segment is then found on the zone/segment table
/// (jumping to the last segment past the end, else walking to the interval),
/// and the slot's distance plus `RaceProgress_ProjectRider`'s offset are
/// stored.
pub fn race_progress_update_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        let course = c.lw(gp(c, progress::COURSE));
        c.g[S3][0] = course;
        let idx = r32(c, A0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        let no_course = course == 0;
        let segs = c.lw(gp(c, progress::ZONES));
        c.g[S0][0] = segs;
        if !no_course {
            c.cycles += 7;
            c.g[V0][0] = c.lw(gp(c, progress::WORLD));
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
            c.g[V1][0] = c.lw(r32(c, V1).wrapping_add(0xc));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(progress::RIDER_COUNT));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let known = c.g[V0][0] != 0;
            c.g[V0][0] = sx(idx << 2);
            if known {
                // `d = course.dist - Rider_GetDistToFinish(rider)`.
                c.cycles += 13;
                c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[RA][0] = sx(0x0021_0668);
                c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(progress::RIDERS));
                call_ptr(c, callee::RIDER_GET_DIST_TO_FINISH, 0x0021_0668)?;
                c.g[V1][0] = 0x18;
                c.g[A0][0] = 0x4d_0000;
                c.mult(0, A0, S2, V1); // a0 = i * 0x18
                c.g[A1][0] = sx(0x4d_0000u32.wrapping_sub(0x7438));
                c.g[S1][0] = 0x4d_0000;
                let course_dist = r32(c, S3).wrapping_add(progress::COURSE_DIST);
                c.lwc1(1, course_dist);
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                c.g[V1][0] = c.lw(r32(c, V0));
                let active = c.g[V1][0] != 0;
                c.sub_s(1, 1, 0); // d
                if !active {
                    // Clamp the step against the previous distance.
                    c.cycles += 8;
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    c.lwc1(4, gp(c, progress::STEP_LIMIT));
                    c.lwc1(2, r32(c, V0).wrapping_add(progress::DIST));
                    c.sub_s(3, 1, 2); // d - previous
                    c.c_lt_s(4, 3); // limit < step
                    if c.fcc() {
                        c.cycles += 2;
                        c.add_s(1, 2, 4);
                    } else {
                        c.neg_s(0, 4);
                        c.cycles += 4;
                        c.c_lt_s(3, 0);
                        if c.fcc() {
                            c.sub_s(1, 2, 4);
                        }
                    }
                }

                'branches: {
                    // `d <= 0`: the start of the course.
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_le_s(1, 0);
                    let ahead = !c.fcc();
                    c.g[A0][0] = c.g[S2][0];
                    if !ahead {
                        c.cycles += 8;
                        c.g[S0][0] = progress::STRIDE as u64;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x7438));
                        c.mult(0, S0, S2, S0); // s0 = i * 0x18
                        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                        c.sw(r32(c, V0).wrapping_add(progress::SEG), 0);
                        c.g[V1][0] = c.g[V0][0];
                        c.swc1(0, r32(c, V1).wrapping_add(progress::DIST));
                        break 'branches;
                    }
                    // Past the last segment?
                    c.cycles += 10;
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(progress::COURSE_COUNT));
                    c.g[A0][0] = progress::SEG_STRIDE as u64;
                    c.g[A1][0] = sx(r32(c, V0).wrapping_sub(1)); // last
                    c.mult(0, V0, A1, A0); // v0 = last * 0x14
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                    c.lwc1(0, r32(c, V1).wrapping_add(progress::SEG_DIST));
                    c.c_le_s(0, 1);
                    let before_last = !c.fcc();
                    c.g[V0][0] = progress::STRIDE as u64;
                    if !before_last {
                        c.cycles += 10;
                        c.g[S0][0] = progress::STRIDE as u64;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x7438));
                        c.mult(0, S0, S2, S0);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                        c.sw(r32(c, V0).wrapping_add(progress::SEG), c.g[A1][0]);
                        c.g[V1][0] = c.g[V0][0];
                        c.lwc1(0, course_dist);
                        c.swc1(0, r32(c, V1).wrapping_add(progress::DIST));
                        break 'branches;
                    }
                    // Walk from the slot's segment to the interval holding `d`.
                    c.cycles += 11;
                    c.g[V1][0] = sx(r32(c, S1).wrapping_sub(0x7438));
                    c.mult(0, V0, S2, V0); // v0 = i * 0x18
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[A1][0] = c.lw(r32(c, V1).wrapping_add(progress::SEG)); // k
                    c.mult(0, V1, A1, A0); // v1 = k * 0x14
                    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S0)));
                    c.lwc1(0, r32(c, V0).wrapping_add(progress::SEG_DIST));
                    c.c_le_s(0, 1);
                    let after = !c.fcc();
                    c.g[V0][0] = sx(r32(c, V1).wrapping_add(progress::SEG_DIST));
                    if !after {
                        c.cycles += 2;
                        let mut dist_at = r32(c, S0).wrapping_add(r32(c, V0));
                        loop {
                            c.cycles += 6;
                            dist_at = dist_at.wrapping_add(progress::SEG_STRIDE);
                            c.lwc1(0, dist_at);
                            c.c_le_s(0, 1);
                            let more = c.fcc();
                            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                            if !more {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x0021_0780);
                            }
                        }
                    }
                    // `d < segs[k].dist`: step back to the lower segment.
                    c.cycles += 8;
                    c.g[V0][0] = progress::SEG_STRIDE as u64;
                    c.mult(0, V0, A1, V0); // v0 = k * 0x14
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                    c.lwc1(0, r32(c, V1).wrapping_add(progress::SEG_DIST));
                    c.c_lt_s(1, 0);
                    let below = !c.fcc();
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(progress::SEG_DIST));
                    if !below {
                        c.cycles += 2;
                        let mut dist_at = r32(c, S0).wrapping_add(r32(c, V0));
                        loop {
                            c.cycles += 6;
                            dist_at = dist_at.wrapping_sub(progress::SEG_STRIDE);
                            c.lwc1(0, dist_at);
                            c.c_lt_s(1, 0);
                            let more = c.fcc();
                            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                            if !more {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x0021_07c0);
                            }
                        }
                    }
                    c.cycles += 8;
                    c.g[S0][0] = progress::STRIDE as u64;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x7438));
                    c.mult(0, S0, S2, S0);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                    c.g[V1][0] = c.g[V0][0];
                    c.sw(r32(c, V0).wrapping_add(progress::SEG), c.g[A1][0]);
                    c.swc1(1, r32(c, V1).wrapping_add(progress::DIST));
                }
                // The slot's lateral offset.
                c.cycles += 3;
                c.g[RA][0] = sx(0x0021_0800);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                callee::PROJECT_RIDER.call(c, 0x0021_0800)?;
                c.swc1(0, r32(c, S1).wrapping_add(progress::POS));
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CollSegment_ctor.

/// `CollSegment* CollSegment_ctor(CollSegment* seg, const Vec4* p0,
/// const Vec4* p1, int state, float r)` (0x32e100): a capsule: the state and
/// vtable (`0x48e6a8`), the start point `p0` (raw and as the base), the
/// direction `p1 - p0`, the radius, the AABB of the two points and its
/// centre.
pub fn coll_segment_ctor(c: &mut Cpu) {
    c.cycles += 26;
    let seg = r32(c, A0);
    c.g[T0][0] = c.g[A0][0];
    c.g[V0][0] = sx(0x490000u32.wrapping_sub(0x1958));
    c.sw(seg, c.g[A3][0]); // state
    c.sw(seg.wrapping_add(0x50), c.g[V0][0]); // vtable
    let frame = Frame::open(c, 0x30);
    let sp = sp(c);
    c.sw(seg.wrapping_add(8), 0);
    c.sw(seg.wrapping_add(4), 0);
    let p0 = r32(c, A1);
    let p1 = r32(c, A2);
    c.lq(2, p0);
    c.sq(2, seg.wrapping_add(0x60)); // p0b
    c.sq(2, seg.wrapping_add(0x80)); // p0
    c.lqc2(3, p1);
    c.lqc2(4, p0);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp.wrapping_add(0x10));
    c.lq(2, sp.wrapping_add(0x10));
    c.swc1(12, seg.wrapping_add(0xa0)); // radius
    c.sq(2, seg.wrapping_add(0x70)); // dirb
    c.sq(2, seg.wrapping_add(0x90)); // dir
    c.sq(2, sp);

    // max = component-wise greater of p0 and p1, w = 1.
    c.lwc1(0, p1);
    c.lwc1(1, p0);
    c.c_lt_s(0, 1);
    let lt = c.fcc();
    c.mov_s(4, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(4, 1);
    }
    c.cycles += 6;
    c.lwc1(0, p1.wrapping_add(4));
    c.lwc1(1, p0.wrapping_add(4));
    c.c_lt_s(0, 1);
    let lt = c.fcc();
    c.mov_s(3, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(3, 1);
    }
    c.cycles += 6;
    c.lwc1(0, p1.wrapping_add(8));
    c.lwc1(1, p0.wrapping_add(8));
    c.c_lt_s(0, 1);
    let lt = c.fcc();
    c.mov_s(2, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(2, 1);
    }
    c.cycles += 14;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 5);
    c.swc1(4, sp);
    c.swc1(3, sp.wrapping_add(4));
    c.swc1(2, sp.wrapping_add(8));
    c.swc1(5, sp.wrapping_add(0xc));
    c.lq(2, sp);
    c.sq(2, seg.wrapping_add(0x20)); // max

    // min = component-wise lesser of p0 and p1, w = 1.
    c.lwc1(0, p1);
    c.lwc1(1, p0);
    c.c_lt_s(1, 0);
    let lt = c.fcc();
    c.mov_s(4, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(4, 1);
    }
    c.cycles += 6;
    c.lwc1(0, p1.wrapping_add(4));
    c.lwc1(1, p0.wrapping_add(4));
    c.c_lt_s(1, 0);
    let lt = c.fcc();
    c.mov_s(3, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(3, 1);
    }
    c.cycles += 6;
    c.lwc1(0, p1.wrapping_add(8));
    c.lwc1(1, p0.wrapping_add(8));
    c.c_lt_s(1, 0);
    let lt = c.fcc();
    c.mov_s(2, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(2, 1);
    }

    // center = 0.5 * (min + max).
    c.cycles += 27;
    c.swc1(4, sp);
    c.g[V0][0] = c.g[T0][0];
    c.swc1(3, sp.wrapping_add(4));
    c.swc1(2, sp.wrapping_add(8));
    c.swc1(5, sp.wrapping_add(0xc));
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(1, 0);
    c.lq(3, sp);
    c.sq(3, seg.wrapping_add(0x30)); // min
    c.lqc2(3, seg.wrapping_add(0x30));
    c.lqc2(4, seg.wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp.wrapping_add(0x20));
    c.lq(3, sp.wrapping_add(0x20));
    c.sq(3, sp.wrapping_add(0x10));
    c.mfc1(4, 0);
    c.qmtc2(4, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, sp.wrapping_add(0x10));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, sp.wrapping_add(0x20));
    c.lq(3, sp.wrapping_add(0x20));
    c.sq(3, seg.wrapping_add(0x40)); // center
    c.sq(3, sp);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// CollHull_ctor.

/// `CollHull* CollHull_ctor(CollHull* hull, RiderSphereTree* tree, int flag)`
/// (0x32f650): the base at +0x50 (vtable `0x48e590`), the embedded sphere
/// tree node at +0x70, the tree at +0x60/+0x64, the state `flag == 0`, the
/// tree's bounds at +0x20/+0x30 and their centre at +0x40.
pub fn coll_hull_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x490000;
        frame.save(c, S0, 0x60);
        c.g[V0][0] = sx(0x490000u32.wrapping_sub(0x1a70));
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        let hull = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.sw(hull.wrapping_add(8), 0);
        c.g[A0][0] = sx(hull.wrapping_add(0x70));
        c.sw(hull.wrapping_add(4), 0);
        c.g[S1][0] = c.g[A1][0];
        c.sw(hull.wrapping_add(0x50), c.g[V0][0]); // vtable
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, callee::RIDER_SPHERE_TREE_CTOR, 0x0032_f68c)?;
        c.sw(hull.wrapping_add(0x64), c.g[S1][0]); // tree2
        c.g[S2][0] = (c.g[S2][0] < 1) as u64;
        c.sw(hull.wrapping_add(0x60), c.g[S1][0]); // tree
        c.g[A0][0] = c.g[S1][0];
        c.sw(hull, c.g[S2][0]); // state
        c.g[A1][0] = sx(hull.wrapping_add(0x30));
        c.g[A2][0] = sx(hull.wrapping_add(0x20));
        call_ptr(c, callee::RIDER_SPHERE_TREE_GET_BOUNDS, 0x0032_f6ac)?;

        // center = 0.5 * (min + max).
        c.lqc2(3, hull.wrapping_add(0x30));
        c.lqc2(4, hull.wrapping_add(0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        let sp = sp(c);
        c.sqc2(5, sp.wrapping_add(0x20));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 0); // f0 = 0.5
        c.lq(3, sp.wrapping_add(0x20));
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x50);
        c.sq(3, sp.wrapping_add(0x10));
        c.mfc1(4, 0);
        c.lqc2(4, sp.wrapping_add(0x10));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp.wrapping_add(0x20));
        frame.restore(c, S2, 0x40);
        c.lq(3, sp.wrapping_add(0x20));
        frame.restore_ra(c, 0x30);
        c.sq(3, hull.wrapping_add(0x40)); // center
        frame.restore(c, S0, 0x60);
        c.sq(3, sp);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2BezierMan_v21.

/// `void PS2BezierMan_v21(PS2BezierMan* self, void* a, void* b, void* c)`
/// (0x38b338): the manager's entry, which forwards to its renderer (vtable
/// slot 22, a `{short delta; void (*fn)();}` entry at `*(self+4)+0xb0`)
/// unless the global at `0x4a44d4` disables it.
pub fn ps2bezier_man_v21(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw(gp(c, 0x13e4));
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[T2][0] = c.g[A0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 6;
            let vt = c.lw(self_.wrapping_add(4));
            c.g[V0][0] = vt;
            c.g[T1][0] = 0;
            c.g[A0][0] = Addr((vt as u32).wrapping_add(0xb0)).lh(c);
            let func = c.lw((vt as u32).wrapping_add(0xb4));
            c.g[V1][0] = func;
            c.g[A0][0] = sx(self_.wrapping_add(r32(c, A0)));
            call_ptr(c, func as u32, 0x0038_b364)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2BezierMan_TestPatchBounds.

/// `void PS2BezierMan_TestPatchBounds(PS2BezierMan* self, Patch* patch)`
/// (0x38ca08): the patch's two corner points (+0x158 and +0x164, w = 1) are
/// handed to the graphics manager at `*(0x4a5b80)`'s light cull.
pub fn ps2bezier_man_test_patch_bounds(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let sp = sp(c);
        let self_ = r32(c, A1);
        c.g[V0][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = c.g[SP][0]; // a = first point
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 5); // f5 = 1.0
        c.g[A2][0] = sx(sp.wrapping_add(0x10)); // b = second point
        c.lwc1(6, self_.wrapping_add(0x16c));
        c.lwc1(4, self_.wrapping_add(0x158));
        c.lwc1(0, self_.wrapping_add(0x15c));
        c.lwc1(1, self_.wrapping_add(0x160));
        c.lwc1(2, self_.wrapping_add(0x164));
        c.lwc1(3, self_.wrapping_add(0x168));
        c.g[A0][0] = c.lw(gp(c, 0x2a90));
        c.swc1(4, sp);
        c.swc1(0, sp.wrapping_add(4));
        c.swc1(1, sp.wrapping_add(8));
        c.swc1(2, sp.wrapping_add(0x10));
        c.swc1(3, sp.wrapping_add(0x14));
        c.swc1(6, sp.wrapping_add(0x18));
        c.swc1(5, sp.wrapping_add(0x1c));
        c.g[RA][0] = sx(0x0038_ca64);
        c.swc1(5, sp.wrapping_add(0xc));
        callee::CULL_AGAINST_LIGHTS.call(c, 0x0038_ca64)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_CullAgainstLights.

/// `unsigned int PS2GraphicsMan_CullAgainstLights(PS2GraphicsMan* self,
/// const Vec4* a, const Vec4* b)` (0x37df88): every light item (0xa0 bytes at
/// `self+0x1400`, count at `self+0x13ec`) is tested against the segment
/// `a`-`b` by the method in the vtable at `self+0x10d8` (entry 93); the
/// result is a bitmask of the items the method rejected (it returned != 1).
pub fn ps2graphics_man_cull_against_lights(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = 0;
        frame.save(c, S5, 0x20);
        c.g[S3][0] = 0;
        frame.save(c, S6, 0x10);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S4, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(self_.wrapping_add(0x13ec));
        let t = c.g[V0][0] == 0;
        c.g[S6][0] = c.g[A1][0];
        if !t {
            c.cycles += 2;
            c.g[S4][0] = 1;
            c.g[S1][0] = sx(self_.wrapping_add(0x1400));
            loop {
                c.cycles += 10;
                let vt = c.lw(self_.wrapping_add(0x10d8));
                c.g[V1][0] = vt;
                c.g[A1][0] = c.g[S6][0];
                c.g[A2][0] = c.g[S5][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[A0][0] = Addr((vt as u32).wrapping_add(0x2e8)).lh(c);
                let func = c.lw((vt as u32).wrapping_add(0x2ec));
                c.g[V0][0] = func;
                c.g[RA][0] = sx(0x0037_dff0);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, func as u32, 0x0037_dff0)?;
                let kept = c.g[V0][0] == c.g[S4][0];
                c.g[V0][0] = sx(r32(c, S4) << (r32(c, S2) & 31));
                if !kept {
                    c.cycles += 1;
                    c.g[S3][0] = c.g[S3][0] | c.g[V0][0];
                }
                c.cycles += 5;
                c.g[V0][0] = c.lw(self_.wrapping_add(0x13ec));
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0xa0));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0037_dfd0);
                }
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH_PlatRefreshStatus.

/// `void PATH_PlatRefreshStatus(int reset)` (0x2aefa8): the per-frame refresh
/// of the 24 sound-bank monitor slots: a `reset` marks every slot for
/// restart; otherwise each slot with a stream (`h >= 0`) updates its
/// in-memory or streaming state and its `f48`/`f4C`/`f60`/`dirty` fields.
pub fn path_plat_refresh_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x170);
        let frame_sp = sp(c);
        frame.save(c, S1, 0x150);
        frame.save(c, S7, 0xf0);
        c.g[S1][0] = 0;
        frame.save(c, FP, 0xe0);
        c.g[S7][0] = u64::MAX; // -1
        frame.save(c, S0, 0x160);
        c.g[FP][0] = 1;
        frame.save(c, S2, 0x140);
        frame.save(c, S3, 0x130);
        frame.save(c, S4, 0x120);
        frame.save(c, S5, 0x110);
        frame.save(c, S6, 0x100);
        frame.save_ra(c, 0xd0);
        c.g[RA][0] = sx(0x002a_efe8);
        c.sw(frame_sp.wrapping_add(0xc0), c.g[A0][0]); // reset
        call_ptr(c, callee::SND_LOCK, 0x002a_efe8)?;
        c.g[V0][0] = 0x4d_0000;
        loop {
            // m = D_004D3E98[i]
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, V0).wrapping_add(0x3e98));
            c.g[V0][0] = sx(r32(c, S1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[S0][0] = c.lw(r32(c, V0));
            let null = c.g[S0][0] == 0;
            c.g[S6][0] = sx(r32(c, S1).wrapping_add(1));
            if !null {
                c.cycles += 3;
                c.g[V1][0] = c.lw(frame_sp.wrapping_add(0xc0));
                let restart = c.g[V1][0] != 0;
                c.g[V0][0] = 1;
                if restart {
                    c.cycles += 3;
                    let m = r32(c, S0);
                    c.sw(m.wrapping_add(monitor::F60), 0);
                    c.sb(m.wrapping_add(monitor::DIRTY), c.g[V0][0]);
                } else {
                    let m = r32(c, S0);
                    // The negative-sub path stores `f48` itself and leaves by
                    // `b_2af29c`, skipping the shared tail's store and cycle.
                    let mut stored48 = false;
                    'body: {
                        // The slot's working values: f60, f48, f4C.
                        c.cycles += 6;
                        c.g[A0][0] = c.lw(m.wrapping_add(monitor::H));
                        c.g[S5][0] = sx(0xffff_fff8); // -8
                        c.g[S4][0] = 0;
                        c.g[S2][0] = 0;
                        let no_stream = (c.g[A0][0] as i64) < 0;
                        c.sw(m.wrapping_add(monitor::BUSY), c.g[FP][0]);
                        if no_stream {
                            break 'body;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(m.wrapping_add(monitor::IS_MEM)).lb(c);
                        let in_memory = c.g[V0][0] != 0;
                        if !in_memory {
                            // Streaming state machine: stream info, then each
                            // sub-stream's state, then the slot status.
                            c.cycles += 9;
                            c.g[RA][0] = sx(0x002a_f134);
                            c.g[A1][0] = c.g[SP][0];
                            call_ptr(c, callee::SND_STREAM_INFO, 0x002a_f134)?;
                            c.sw(m.wrapping_add(monitor::F50), 0);
                            c.g[S5][0] = c.g[V0][0]; // f60
                            c.g[V1][0] = Addr(m.wrapping_add(monitor::COUNT)).lb(c);
                            c.g[V0][0] = c.lw(frame_sp);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                c.sw(m.wrapping_add(monitor::BUSY), 0);
                            }
                            c.cycles += 3;
                            c.g[V0][0] = Addr(m.wrapping_add(monitor::COUNT)).lb(c);
                            let none = (c.g[V0][0] as i64) <= 0;
                            c.g[S3][0] = 0; // j
                            if !none {
                                c.cycles += 1;
                                c.g[S1][0] = 0; // j * 0xc
                                loop {
                                    'sub: {
                                        c.cycles += 8;
                                        c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                        c.sw(r32(c, V0).wrapping_add(monitor::SUB_POS), 0);
                                        c.g[V1][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                        c.g[V1][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
                                        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(monitor::SUB_ID));
                                        if (c.g[A0][0] as i64) < 0 {
                                            c.g[V0][0] = Addr(m.wrapping_add(monitor::COUNT)).lb(c);
                                            break 'sub;
                                        }
                                        c.cycles += 5;
                                        c.g[RA][0] = sx(0x002a_f188);
                                        c.g[A1][0] = sx(frame_sp.wrapping_add(monitor::ST_STATE));
                                        call_ptr(c, callee::SND_SUB_STATE, 0x002a_f188)?;
                                        let state = c.lw(frame_sp.wrapping_add(monitor::ST_STATE));
                                        c.g[V1][0] = state;
                                        // `switch (state)`: 1 plays the
                                        // sub-stream to its end, 0 and 2
                                        // advance it by st.a/st.b, anything
                                        // else drops it.
                                        let mut store = false;
                                        if state == 1 {
                                            c.cycles += 18;
                                            c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                            c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                            c.g[V1][0] = c.lw(r32(c, V0));
                                            c.sw(
                                                r32(c, V0).wrapping_add(monitor::SUB_POS),
                                                c.g[V1][0],
                                            );
                                            c.g[A1][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                            c.g[V1][0] = c.lw(m.wrapping_add(monitor::F54));
                                            c.g[A1][0] = sx(r32(c, S1).wrapping_add(r32(c, A1)));
                                            c.g[A0][0] = c.lw(m.wrapping_add(monitor::F50));
                                            c.g[V0][0] = c.lw(r32(c, A1));
                                            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                                            c.sw(m.wrapping_add(monitor::F54), c.g[V1][0]);
                                            c.g[V0][0] = c.lw(frame_sp.wrapping_add(monitor::ST_C));
                                            c.sw(m.wrapping_add(monitor::BUSY), 0);
                                            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                                            c.sw(m.wrapping_add(monitor::F50), c.g[A0][0]);
                                            c.g[V0][0] =
                                                c.lw(r32(c, A1).wrapping_add(monitor::SUB_ID));
                                            c.sw(m.wrapping_add(monitor::F5C), c.g[V0][0]);
                                        } else {
                                            c.cycles += 2;
                                            if state == 0 || state == 2 {
                                                c.cycles += 2;
                                                c.g[S4][0] =
                                                    c.lw(frame_sp.wrapping_add(monitor::ST_A));
                                                store = true;
                                            } else {
                                                // b_2af1ac's exit, the else,
                                                // then the default block.
                                                c.cycles += 2;
                                                c.cycles += 2;
                                                c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                                c.cycles += 2;
                                                c.g[V0][0] =
                                                    sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                                c.sw(
                                                    r32(c, V0).wrapping_add(monitor::SUB_ID),
                                                    c.g[S7][0],
                                                );
                                            }
                                        }
                                        if store {
                                            // st.b, falling back to the
                                            // sub-stream's size when both
                                            // are zero.
                                            c.cycles += 2;
                                            c.g[S2][0] = c.lw(frame_sp.wrapping_add(monitor::ST_B));
                                            if c.g[S4][0] != 0 {
                                                c.cycles += 1;
                                                c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                            } else {
                                                c.cycles += 2;
                                                c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                                if c.g[S2][0] == 0 {
                                                    c.cycles += 3;
                                                    c.g[S4][0] = c.g[S2][0];
                                                    c.g[V0][0] =
                                                        sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                                    c.g[S2][0] = c.lw(r32(c, V0));
                                                    c.cycles += 1;
                                                    c.g[V0][0] =
                                                        c.lw(m.wrapping_add(monitor::SUBS));
                                                }
                                            }
                                            c.cycles += 14;
                                            c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                            c.sw(
                                                r32(c, V0).wrapping_add(monitor::SUB_POS),
                                                c.g[S2][0],
                                            );
                                            c.g[V1][0] = c.lw(m.wrapping_add(monitor::F54));
                                            c.g[A0][0] = c.lw(m.wrapping_add(monitor::F50));
                                            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S2)));
                                            c.sw(m.wrapping_add(monitor::F54), c.g[V1][0]);
                                            c.g[V0][0] = c.lw(frame_sp.wrapping_add(monitor::ST_C));
                                            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                                            c.sw(m.wrapping_add(monitor::F50), c.g[A0][0]);
                                            c.g[V0][0] = c.lw(frame_sp.wrapping_add(monitor::ST_C));
                                            c.g[V1][0] = c.lw(frame_sp.wrapping_add(monitor::ST_B));
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            if c.g[V0][0] != 0 {
                                                c.sw(m.wrapping_add(monitor::BUSY), 0);
                                            }
                                            c.cycles += 5;
                                            c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                            c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                            c.g[V1][0] =
                                                c.lw(r32(c, V0).wrapping_add(monitor::SUB_ID));
                                            c.sw(m.wrapping_add(monitor::F5C), c.g[V1][0]);
                                        }
                                        // Every path but the dropped
                                        // sub-stream reloads the sub count
                                        // here (the switch's exit).
                                        c.cycles += 1;
                                        c.g[V0][0] = Addr(m.wrapping_add(monitor::COUNT)).lb(c);
                                    }
                                    // The sub-stream loop's tail.
                                    c.cycles += 4;
                                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let more = c.g[V0][0] != 0;
                                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(monitor::SUB_STRIDE));
                                    if !more {
                                        break;
                                    }
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002a_f160);
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x002a_f294);
                            c.g[A0][0] = c.lw(m);
                            call_ptr(c, callee::SND_STATUS, 0x002a_f294)?;
                            c.sw(m.wrapping_add(monitor::STATUS), c.g[V0][0]);
                            break 'body;
                        }

                        // In-memory stream: skip a negative sub or a sub
                        // stream that is gone.
                        c.cycles += 3;
                        c.g[A1][0] = c.lw(m.wrapping_add(monitor::SUB));
                        if (c.g[A1][0] as i64) < 0 {
                            c.sw(m.wrapping_add(monitor::F48), c.g[S4][0]);
                            stored48 = true;
                            break 'body;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(monitor::SUB_ID));
                        let gone = (c.g[V1][0] as i64) < 0;
                        c.g[A2][0] = sx(frame_sp.wrapping_add(0x20)); // header
                        if gone {
                            break 'body;
                        }
                        c.cycles += 6;
                        c.g[A3][0] = sx(frame_sp.wrapping_add(0x30)); // buffer
                        c.g[RA][0] = sx(0x002a_f06c);
                        c.g[T0][0] = sx(frame_sp.wrapping_add(monitor::POS_SAMPLES));
                        call_ptr(c, callee::SND_STREAM_POSITION, 0x002a_f06c)?;
                        c.g[S5][0] = c.g[V0][0];
                        let failed = (c.g[S5][0] as i64) < 0;
                        c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                        if !failed {
                            c.cycles += 5;
                            c.g[RA][0] = sx(0x002a_f080);
                            c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(monitor::SUB_ID));
                            call_ptr(c, callee::SND_SUB_TIME, 0x002a_f080)?;
                            c.g[S2][0] = c.g[V0][0];
                            let ok = (c.g[S2][0] as i64) >= 0;
                            c.g[V0][0] = 0xa;
                            if !ok {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                c.g[S2][0] = 0;
                                c.sw(r32(c, V0).wrapping_add(monitor::SUB_ID), c.g[S7][0]);
                                break 'body;
                            }
                            // f4C = time * 10; f48 = ms(samples / rate) - f4C.
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(frame_sp.wrapping_add(monitor::POS_SAMPLES));
                            let negative = (c.g[A0][0] as i64) < 0;
                            c.mult(0, S2, S2, V0);
                            if negative {
                                c.cycles += 7;
                                c.g[V1][0] = sx(r32(c, A0) >> 1);
                                c.g[V0][0] = c.g[A0][0] & 0x1;
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.mtc1(2, 2);
                                c.cvt_s_w(2, 2);
                                c.add_s(2, 2, 2);
                            } else {
                                c.cycles += 4;
                                c.mtc1(4, 2);
                                c.cvt_s_w(2, 2);
                            }
                            c.g[V0][0] = Addr(frame_sp.wrapping_add(monitor::HDR_RATE)).lhu(c);
                            c.cycles += 10;
                            c.g[AT][0] = 0x447a_0000;
                            c.mtc1(1, 1); // f1 = 1000.0
                            c.mtc1(2, 0); // f0 = the rate
                            c.cvt_s_w(0, 0);
                            c.div_s(0, 2, 0);
                            c.mul_s(0, 0, 1);
                            c.cvt_w_s(1, 0);
                            c.mfc1(2, 1);
                            c.g[S4][0] = sx(r32(c, V0).wrapping_sub(r32(c, S2)));
                            c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                        }
                        // The sub-stream's position, size, status and id.
                        c.cycles += 12;
                        c.g[S6][0] = sx(r32(c, S1).wrapping_add(1));
                        c.sw(r32(c, V0).wrapping_add(monitor::SUB_POS), c.g[S2][0]);
                        c.g[V1][0] = c.lw(m.wrapping_add(monitor::SUBS));
                        c.g[V0][0] = c.lw(r32(c, V1));
                        c.sw(m.wrapping_add(monitor::F50), c.g[V0][0]);
                        c.g[RA][0] = sx(0x002a_f118);
                        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(monitor::SUB_ID));
                        call_ptr(c, callee::SND_SUB_STATUS, 0x002a_f118)?;
                        c.sw(m.wrapping_add(monitor::STATUS), c.g[V0][0]);
                        c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(monitor::SUB_ID));
                        c.sw(m.wrapping_add(monitor::F5C), c.g[V1][0]);
                        break 'body;
                    }
                    // done: the slot's working values and its dirty flag.
                    if !stored48 {
                        c.cycles += 1;
                        c.sw(m.wrapping_add(monitor::F48), c.g[S4][0]);
                    }
                    c.cycles += 3;
                    c.sw(m.wrapping_add(monitor::F4C), c.g[S2][0]);
                    c.sw(m.wrapping_add(monitor::F60), c.g[S5][0]);
                    c.sb(m.wrapping_add(monitor::DIRTY), 0);
                }
            }
            // The next slot.
            c.cycles += 4;
            c.g[S1][0] = c.g[S6][0];
            c.g[V0][0] = ((c.g[S1][0] as i64) < monitor::SLOTS as i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = 0x4d_0000;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_eff0);
            }
        }
        c.cycles += 14;
        c.g[RA][0] = sx(0x002a_f2c0);
        call_ptr(c, callee::SND_UNLOCK, 0x002a_f2c0)?;
        frame.restore(c, S0, 0x160);
        frame.restore(c, S1, 0x150);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore(c, S5, 0x110);
        frame.restore(c, S6, 0x100);
        frame.restore(c, S7, 0xf0);
        frame.restore(c, FP, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, vtable, FAKE};
    use crate::difftest::{check, put_vec, scramble, Rng, DATA, GAME_GP};

    fn uf(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        f(lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo))
    }

    // Path_QueryEvents -----------------------------------------------------

    const RANGE_LIST: u32 = DATA;
    const RANGES: u32 = DATA + 0x400;
    const RANGE_OUT: u32 = DATA + 0x800;

    #[test]
    fn path_query_events_matches() {
        check(
            ssx3_ee::s07::f_0026aa80__generated,
            path_query_events,
            200,
            &|c, seed| {
                let mut r = Rng(0x0026_aa80 ^ seed as u64);
                scramble(c, &mut r);
                let count = (seed % 5) as u32; // 0..4
                c.g[A0][0] = RANGE_LIST as u64;
                c.g[A1][0] = RANGE_OUT as u64;
                c.g[A2][0] = (seed % 3) as u64; // max
                c.bus.write32(RANGE_LIST, count);
                c.bus.write32(RANGE_LIST + 4, RANGES);
                for i in 0..5 {
                    let lo = f32::from_bits(uf(&mut r, -20.0, 20.0));
                    let hi = lo + if seed % 4 == 0 { -5.0 } else { 3.0 };
                    c.bus.write32(RANGES + 16 * i, r.next_u32());
                    c.bus.write32(RANGES + 16 * i + 4, r.next_u32());
                    c.bus.write32(RANGES + 16 * i + 8, f(lo));
                    c.bus.write32(RANGES + 16 * i + 0xc, f(hi));
                }
                c.f[12] = uf(&mut r, -10.0, 10.0);
                c.f[13] = if seed % 7 == 0 {
                    f(-200.0)
                } else {
                    uf(&mut r, -10.0, 10.0)
                };
            },
        );
    }

    // Path_GetPointAtDistance ---------------------------------------------

    const TRACK: u32 = DATA;
    const TRACK_SEGS: u32 = DATA + 0x400;
    const POINT: u32 = DATA + 0x800;
    const IN_RANGE: u32 = DATA + 0x900;

    fn put_track(c: &mut Cpu, r: &mut Rng, count: u32) {
        c.bus.write32(TRACK + 8, count);
        c.bus.write32(TRACK + 0xc, r.f32(100.0));
        c.bus.write32(TRACK + 0x10, r.f32(100.0));
        c.bus.write32(TRACK + 0x14, r.f32(100.0));
        c.bus.write32(TRACK + 0x18, TRACK_SEGS);
        for i in 0..4 {
            put_vec(
                c,
                TRACK_SEGS + 16 * i,
                [r.f32(2.0), r.f32(2.0), r.f32(2.0), uf(r, 10.0, 60.0)],
            );
        }
    }

    #[test]
    fn path_get_point_at_distance_matches() {
        check(
            ssx3_ee::s07::f_0026ab20__generated,
            path_get_point_at_distance,
            200,
            &|c, seed| {
                let mut r = Rng(0x0026_ab20 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = POINT as u64;
                c.g[A1][0] = TRACK as u64;
                c.g[A2][0] = if seed % 5 == 0 { 0 } else { IN_RANGE as u64 };
                put_track(c, &mut r, (seed % 4) as u32);
                // Inside the track half the time, past its end otherwise.
                c.f[12] = if seed % 2 == 0 {
                    uf(&mut r, 0.0, 120.0)
                } else {
                    uf(&mut r, 200.0, 400.0)
                };
            },
        );
    }

    // CollSegment_ctor -----------------------------------------------------

    const SEGMENT: u32 = DATA;
    const SEG_P0: u32 = DATA + 0x100;
    const SEG_P1: u32 = DATA + 0x200;

    #[test]
    fn coll_segment_ctor_matches() {
        check(
            ssx3_ee::s11::f_0032e100__generated,
            coll_segment_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x0032_e100 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = SEGMENT as u64;
                c.g[A1][0] = SEG_P0 as u64;
                c.g[A2][0] = SEG_P1 as u64;
                c.g[A3][0] = (seed % 3) as u64;
                put_vec(
                    c,
                    SEG_P0,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    SEG_P1,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                c.f[12] = r.f32(3.0);
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    // CollHull_ctor --------------------------------------------------------

    const HULL: u32 = DATA;
    const HULL_TREE: u32 = DATA + 0x1000;

    #[test]
    fn coll_hull_ctor_matches() {
        check(ssx3_ee::s11::f_0032f650__generated, coll_hull_ctor, 200, &|c, seed| {
            let mut r = Rng(0x0032_f650 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = HULL as u64;
            c.g[A1][0] = HULL_TREE as u64;
            c.g[A2][0] = (seed % 2) as u64;
            // An empty sphere tree: the bounds come from the empty-box
            // globals and there is no per-sphere loop.
            c.bus.write32(HULL_TREE + 0x28, 0);
            c.bus.write32(GAME_GP - 0x2de8, f(1.5));
            c.bus.write32(GAME_GP - 0x2de4, f(-2.5));
        });
    }

    // PS2BezierMan_v21 -----------------------------------------------------

    const BEZIER: u32 = DATA;

    #[test]
    fn ps2bezier_man_v21_matches() {
        covered(
            ssx3_ee::s12::f_0038b338__generated,
            ps2bezier_man_v21,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38b338, 1);
                c.g[A0][0] = BEZIER as u64;
                c.bus
                    .write32(GAME_GP + 0x13e4, if seed % 3 == 0 { 0 } else { 1 });
                let vt = vtable(c, &mut r, &[0xb0]);
                c.bus.write32(BEZIER + 4, vt);
            },
        );
    }

    // PS2BezierMan_TestPatchBounds ----------------------------------------

    const PATCH: u32 = DATA + 0x2000;
    const GMAN: u32 = DATA + 0x4000;
    const LIGHT_VT: u32 = DATA + 0x5000;

    #[test]
    fn ps2bezier_man_test_patch_bounds_matches() {
        covered(
            ssx3_ee::s12::f_0038ca08__generated,
            ps2bezier_man_test_patch_bounds,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38ca08, 1);
                c.g[A1][0] = PATCH as u64;
                c.bus.write32(GAME_GP + 0x2a90, GMAN);
                for k in 0..7 {
                    c.bus.write32(PATCH + 0x158 + 4 * k, r.edge_f32());
                }
                // The manager's light list: empty half the time, else one item
                // whose method is a fake.
                let count = (seed % 2) as u32;
                c.bus.write32(GMAN + 0x13ec, count);
                if count != 0 {
                    c.bus.write32(GMAN + 0x10d8, LIGHT_VT);
                    c.bus.write16(LIGHT_VT + 0x2e8, 0);
                    c.bus.write32(LIGHT_VT + 0x2ec, FAKE);
                }
            },
        );
    }

    // PS2GraphicsMan_CullAgainstLights ------------------------------------

    const CULL: u32 = DATA;
    const CULL_VT: u32 = DATA + 0x2000;

    #[test]
    fn ps2graphics_man_cull_against_lights_matches() {
        covered(
            ssx3_ee::s12::f_0037df88__generated,
            ps2graphics_man_cull_against_lights,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37df88, 1);
                c.g[A0][0] = CULL as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = (DATA + 0x900) as u64;
                let count = (seed % 4) as u32;
                c.bus.write32(CULL + 0x13ec, count);
                if count != 0 {
                    c.bus.write32(CULL + 0x10d8, CULL_VT);
                    c.bus.write16(CULL_VT + 0x2e8, 0);
                    c.bus.write32(CULL_VT + 0x2ec, FAKE);
                }
                for i in 0..4 {
                    for k in 0..4 {
                        c.bus
                            .write32(CULL + 0x1400 + 0xa0 * i + 4 * k, r.next_u32());
                    }
                }
            },
        );
    }

    // RaceProgress_ProjectRider ------------------------------------------

    const WORLD: u32 = DATA + 0x1000;
    const RACERS: u32 = DATA + 0x1100;
    const RACE: u32 = DATA + 0x1200;
    const RIDER: u32 = DATA + 0x2000;
    const ZONES: u32 = DATA + 0x3000;
    const POS_VT: u32 = DATA + 0x4000;

    /// The world → race manager chain and the zone table pointer.
    fn put_world(c: &mut Cpu) {
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, RACERS);
        c.bus.write32(RACERS + 0xc, RACE);
        c.bus.write32(RACE + 0x78, 4);
        c.bus.write32(GAME_GP - 0x99c, ZONES);
    }

    /// The rider `idx`: its position object's virtual method is a fake.
    fn put_rider(c: &mut Cpu, idx: u32) {
        let rider = RIDER + 0x100 * idx;
        c.bus.write32(RACE + 0x28 + 4 * idx, rider);
        c.bus.write32(rider + 0x6c0, POS_VT);
        c.bus.write16(POS_VT + 0x28, 0);
        c.bus.write32(POS_VT + 0x2c, FAKE);
    }

    #[test]
    fn race_progress_project_rider_matches() {
        covered(
            ssx3_ee::s05::f_00210820__generated,
            race_progress_project_rider,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x210820, 1);
                put_world(c);
                let idx = (seed % 2) as u32;
                put_rider(c, idx);
                c.g[A0][0] = idx as u64;
                // The slot and the zone it points at.
                let slot = 0x004c_8bc8 + 0x18 * idx;
                let zone = idx;
                c.bus.write32(slot, (seed % 3) as u32); // active
                c.bus.write32(slot + 4, zone);
                for i in 0..5 {
                    c.bus.write32(ZONES + 0x14 * zone + 4 * i, r.edge_f32());
                }
            },
        );
    }

    // RaceProgress_UpdateRider -------------------------------------------

    const COURSE: u32 = DATA + 0x5000;
    const COURSE_SEGS: u32 = DATA + 0x6000;

    fn put_course(c: &mut Cpu) {
        c.bus.write32(COURSE, 4);
        c.bus.write32(COURSE + 0x10, f(1000.0));
        c.bus.write32(GAME_GP - 0x9a0, COURSE);
        for i in 0..4u32 {
            c.bus.write32(COURSE_SEGS + 0x14 * i, 0);
            c.bus.write32(COURSE_SEGS + 0x14 * i + 4, 0);
            c.bus.write32(COURSE_SEGS + 0x14 * i + 8, 0);
            c.bus.write32(COURSE_SEGS + 0x14 * i + 0xc, 0);
            c.bus
                .write32(COURSE_SEGS + 0x14 * i + 0x10, f(100.0 * i as f32));
        }
    }

    #[test]
    fn race_progress_update_rider_matches() {
        covered(
            ssx3_ee::s05::f_00210618__generated,
            race_progress_update_rider,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x210618, 1);
                put_world(c);
                put_course(c);
                c.bus.write32(GAME_GP - 0x99c, COURSE_SEGS);
                c.bus.write32(GAME_GP - 0x54ac, f(138.8889));
                let idx = (seed % 2) as u32;
                put_rider(c, idx);
                c.g[A0][0] = idx as u64;
                c.bus.write32(RIDER + 0x100 * idx + 0x4d0, f(700.0));
                let slot = 0x004c_8bc8 + 0x18 * idx;
                // `d = course.dist - 700`, chosen per branch.
                let (active, seg, d, prev) = match seed % 8 {
                    0 => (0, 0, 100.0, 0.0),    // (course is nulled below)
                    1 => (0, 0, 100.0, 0.0),    // (rider count is zeroed below)
                    2 => (1, 0, 900.0, 0.0),    // active: no clamp, past the last
                    3 => (0, 0, 1000.0, 500.0), // clamp up
                    4 => (0, 0, 100.0, 5000.0), // clamp down
                    5 => (0, 0, 150.0, 100.0),  // no clamp, walk
                    6 => (0, 0, -10.0, 0.0),    // d <= 0
                    _ => (0, 0, 250.0, 0.0),    // walk
                };
                c.bus.write32(COURSE + 0x10, f(700.0 + d));
                c.bus.write32(slot, active);
                c.bus.write32(slot + 4, seg);
                c.bus.write32(slot + 8, f(prev));
                if seed % 8 == 0 {
                    c.bus.write32(GAME_GP - 0x9a0, 0);
                }
                if seed % 8 == 1 {
                    c.bus.write32(RACE + 0x78, 0);
                }
                let _ = r.next_u32();
            },
        );
    }

    // PATH_PlatRefreshStatus ---------------------------------------------

    const MON_TABLE: u32 = 0x004d_3e98;
    const MON: u32 = DATA + 0x7000;
    const MON_SUBS: u32 = DATA + 0x7800;

    #[test]
    fn path_plat_refresh_status_matches() {
        covered(
            ssx3_ee::s08::f_002aefa8__generated,
            path_plat_refresh_status,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2aefa8, 1);
                c.g[A0][0] = (seed % 3 == 0) as u64; // reset
                for i in 0..24 {
                    let m = if i == (seed % 24) as u32 { MON } else { 0 };
                    c.bus.write32(MON_TABLE + 4 * i, m);
                }
                let m = MON;
                // One sub-stream on the slot's list.
                c.bus.write32(m + 0x6c, MON_SUBS);
                c.bus.write32(MON_SUBS, 0x40);
                c.bus.write32(MON_SUBS + 8, 0);
                match seed % 6 {
                    0 => c.bus.write32(m + 0x0, u32::MAX), // no stream
                    1 => {
                        // In memory, but the slot's sub index is negative.
                        c.bus.write8(m + 0x64, 1);
                        c.bus.write32(m + 0x4, u32::MAX);
                    }
                    2 => {
                        // Streaming with no sub-streams.
                        c.bus.write8(m + 0x64, 0);
                        c.bus.write8(m + 0x66, 0);
                    }
                    3 => {
                        // In memory, but its sub-stream is gone.
                        c.bus.write8(m + 0x64, 1);
                        c.bus.write32(MON_SUBS + 4, u32::MAX);
                    }
                    4 => {
                        // In memory with a live sub-stream.
                        c.bus.write8(m + 0x64, 1);
                        c.bus.write32(MON_SUBS + 4, 1);
                        c.bus.write32(m + 0x54, 0);
                        c.bus.write32(m + 0x50, 0);
                    }
                    _ => {
                        // Streaming with one live sub-stream.
                        c.bus.write8(m + 0x64, 0);
                        c.bus.write8(m + 0x66, 1);
                        c.bus.write32(MON_SUBS + 4, 1);
                    }
                }
            },
        );
    }
}
