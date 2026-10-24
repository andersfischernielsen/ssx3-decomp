//! `src/fe/ovstates7.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp_2;
pub(crate) use crate::game::fe::festaterewards::p;
pub(crate) use crate::game::fe::festaterewards::r;
pub(crate) use crate::game::object::flexrailnode::at_3 as at_2;
pub(crate) use crate::game::object::flexrailnode::r32v;
pub(crate) use crate::game::object::flexrailnode::spa;
#[cfg(test)]
pub(crate) use crate::game::render::bezierman::f;
pub(crate) use crate::game::render::bezierman::gp;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
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
    (
        0x0021_08f8,
        race_progress_reset_all,
        "RaceProgress_ResetAll",
    ),
    (
        0x0021_0f40,
        ov_screen_read_private_message_open_keyboard,
        "OVScreenReadPrivateMessage_OpenKeyboard",
    ),
    (
        0x0021_0fa0,
        ovscreen_read_private_message_create_keyboard_popup,
        "OVScreenReadPrivateMessage_CreateKeyboardPopup",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::render::bezierman::callee::CULL_AGAINST_LIGHTS;
    pub(crate) use crate::game::render::bezierman::callee::PROJECT_RIDER;
    pub(crate) use crate::game::render::bezierman::callee::RIDER_GET_DIST_TO_FINISH;
    pub(crate) use crate::game::render::bezierman::callee::RIDER_SPHERE_TREE_CTOR;
    pub(crate) use crate::game::render::bezierman::callee::RIDER_SPHERE_TREE_GET_BOUNDS;
    pub(crate) use crate::game::render::bezierman::callee::SND_LOCK;
    pub(crate) use crate::game::render::bezierman::callee::SND_STATUS;
    pub(crate) use crate::game::render::bezierman::callee::SND_STREAM_INFO;
    pub(crate) use crate::game::render::bezierman::callee::SND_STREAM_POSITION;
    pub(crate) use crate::game::render::bezierman::callee::SND_SUB_STATE;
    pub(crate) use crate::game::render::bezierman::callee::SND_SUB_STATUS;
    pub(crate) use crate::game::render::bezierman::callee::SND_SUB_TIME;
    pub(crate) use crate::game::render::bezierman::callee::SND_UNLOCK;
}

/// The race-progress state: the rider slots (`0x4d0000-0x7438`, stride 0x18),
/// the zone/course-segment table at `gp-0x99c`, the course at `gp-0x9a0` and
/// the world at `gp-0x848`.
pub(crate) mod progress {

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
// Race progress reset.

/// `void RaceProgress_ResetAll()` (0x2108f8): clear the six race-progress
/// records at 0x4c8bc8 (0x18 bytes each: an "in use" word then five zero
/// words) and set the "progress initialised" flag (`gp_2-0x990`).
pub fn race_progress_reset_all(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x4d_0000;
    c.g[A1][0] = 1;
    c.g[V0][0] = sx(0x4d_0000u32.wrapping_sub(0x7438));
    c.g[V1][0] = c.g[V0][0];
    let first = c.g[V1][0] as u32;
    c.g[A0][0] = sx(first.wrapping_add(0x90));
    let one = c.g[A1][0];
    Addr(first).sw(c, one);
    let mut record = first;
    loop {
        c.cycles += 9;
        Addr(record.wrapping_add(4)).sw(c, 0);
        Addr(record.wrapping_add(8)).sw(c, 0);
        Addr(record.wrapping_add(0xc)).sw(c, 0);
        Addr(record.wrapping_add(0x10)).sw(c, 0);
        Addr(record.wrapping_add(0x14)).sw(c, 0);
        record = record.wrapping_add(0x18);
        c.g[V1][0] = sx(record);
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        Addr(record).sw(c, one);
        if event_due(c) {
            deliver_event_here(c, 0x0021_0910);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = 1;
    Addr(gp_2(c, -0x990)).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void OVScreenReadPrivateMessage_OpenKeyboard(...)` (0x210f40): open the
/// keyboard popup for the private-message overlay.
pub fn ov_screen_read_private_message_open_keyboard(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let a2 = c.g[A2][0];
        frame.save(c, S2, 0x10);
        let self_ = c.g[A0][0];
        c.g[S0][0] = a2;
        c.g[A2][0] = c.g[T0][0];
        c.g[S1][0] = self_;
        let a3 = c.g[A3][0];
        c.g[S2][0] = a3;
        frame.save_ra(c, 0);
        c.g[T0][0] = c.g[T2][0];
        c.g[A3][0] = c.g[T1][0];
        call_ptr(c, 0x0021_0fa0, 0x0021_0f70)?;
        c.g[A0][0] = Addr(at(c, S1, 0xe0)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001c_cf00, 0x0021_0f7c)?;
        c.g[A0][0] = Addr(at(c, S1, 0xe0)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x001c_b418, 0x0021_0f88)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Keyboard popup.

/// `cKeyboardPopup* OVScreenReadPrivateMessage_CreateKeyboardPopup(
/// self, arg1, arg2, arg3, arg4)` (0x210fa0): allocate a 0x444-byte keyboard
/// popup, construct it, push it on the screen's state manager, set its
/// max length / key set / function keys, and remember it on the screen.
pub fn ovscreen_read_private_message_create_keyboard_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 58;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S3][0] = c.g[A3][0];
        c.g[A1][0] = 0x470000;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[T0][0];
        c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x1030));
        c.g[A0][0] = 0x444;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0021_0fe8)?;

        c.g[A1][0] = Addr(r(c, S0)).off(0x10).lw(c);
        c.g[T0][0] = 0xf;
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001c_b030, 0x0021_1000)?;

        c.g[V1][0] = Addr(r(c, S0)).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        c.sw(Addr(r(c, S0)).off(0xe0).0, c.g[V0][0]);
        c.g[V0][0] = Addr(r(c, V1)).off(0xd0).lw(c);
        c.g[A0][0] = Addr(r(c, V0)).off(0x10).lw(c);
        c.g[A0][0] = sx(p(r(c, A0), 0x18));
        call_ptr(c, 0x0039_f290, 0x0021_101c)?;

        c.g[V0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.sw(Addr(r(c, V0)).off(0x18).0, c.g[S3][0]);
        c.g[A0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        call_ptr(c, 0x001c_d088, 0x0021_1030)?;

        c.g[V0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        c.g[A2][0] = 1;
        c.g[A3][0] = 2;
        c.g[A1][0] = 0x4b;
        c.sw(Addr(r(c, V0)).off(0x43c).0, 0);
        c.g[A0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        call_ptr(c, 0x001c_e3c8, 0x0021_104c)?;

        c.g[A0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        c.g[A1][0] = 1;
        call_ptr(c, 0x001c_e468, 0x0021_1058)?;

        c.g[V1][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        c.g[V0][0] = 1;
        c.lq(17, p(r(c, SP), 0x40));
        c.sw(Addr(r(c, V1)).off(0x440).0, c.g[V0][0]);
        c.sw(Addr(r(c, S0)).off(0xdc).0, c.g[S4][0]);
        c.lq(16, p(r(c, SP), 0x50));
        c.lq(18, p(r(c, SP), 0x30));
        c.lq(19, p(r(c, SP), 0x20));
        c.lq(20, p(r(c, SP), 0x10));
        c.g[RA][0] = c.ld(r(c, SP));
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x60));
        c.pc = ra;
        Ok(())
    })
}

/// A `gp_3`-relative global address.
#[inline(always)]
pub(crate) fn gp_3(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
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
    c.g[V0][0] = Addr(gp_3(c, -0x988)).lw(c);
    let absent = c.g[V0][0] == 0;
    c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x10));
    if !absent {
        c.cycles += 8;
        c.g[V0][0] = Addr(gp_3(c, -0x98c)).lw(c);
        c.lwc1(2, gp_3(c, -0x54f4));
        c.lwc1(0, at_2(r32v(c, V0), 0x14));
        c.cvt_s_w(0, 0);
        c.lwc1(1, at_2(r32v(c, V0), 0x34));
        c.mul_s(0, 0, 1);
        c.mul_s(7, 0, 2);
    }
    // Both paths converge on f7 = 30.0.
    c.cycles += 2;
    c.g[AT][0] = 0x41f0_0000;
    c.mtc1(1, 7);
    c.cycles += 53;
    // The source rect is `{x, y, w, h}` at_2 +0, +4, +8, +0xc.
    c.lwc1(2, at_2(src, 0xc));
    c.g[V0][0] = 0x4d_0000;
    c.lwc1(3, at_2(src, 8));
    c.lwc1(0, at_2(src, 4));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(1, 5);
    c.add_s(0, 0, 2);
    c.lwc1(1, src);
    c.mul_s(3, 3, 5);
    c.lwc1(4, 0x004c_8ca0);
    c.lwc1(6, gp_3(c, -0x54f0));
    c.sub_s(0, 0, 7);
    c.add_s(1, 1, 3);
    c.swc1(0, spa(c, 4));
    c.swc1(1, spa(c, 0));
    // `{x1, y1}` as one doubleword at_2 +0.
    c.ldl(2, spa(c, 7));
    c.ldr(2, spa(c, 0));
    c.sdl(2, at_2(meter, 7));
    c.sdr(2, at_2(meter, 0));
    c.lwc1(3, at_2(src, 4));
    c.swc1(3, at_2(meter, 8));
    c.lwc1(0, at_2(src, 0xc));
    c.lwc1(1, at_2(src, 4));
    c.add_s(1, 1, 0);
    c.sub_s(1, 1, 7);
    c.sub_s(1, 1, 4);
    c.swc1(1, at_2(meter, 0xc));
    c.sub_s(3, 1, 3);
    c.lwc1(0, at_2(src, 8));
    c.lwc1(2, at_2(src, 0));
    c.sub_s(3, 3, 4);
    c.mul_s(0, 0, 5);
    c.mul_s(3, 3, 6);
    c.add_s(2, 2, 0);
    c.swc1(2, at_2(meter, 0x10));
    c.lwc1(0, at_2(src, 4));
    c.swc1(3, at_2(meter, 0x1c));
    c.add_s(0, 0, 4);
    c.swc1(1, at_2(meter, 0x18));
    c.swc1(0, at_2(meter, 0x14));
    // `{y2, x2}` at_2 +0x20 and `{h', w'}` at_2 +0x28, copied as doublewords.
    c.ldl(2, at_2(src, 7));
    c.ldr(2, at_2(src, 0));
    c.ldl(3, at_2(src, 0xf));
    c.ldr(3, at_2(src, 8));
    c.sdl(2, at_2(meter, 0x27));
    c.sdr(2, at_2(meter, 0x20));
    c.sdl(3, at_2(meter, 0x2f));
    c.sdr(3, at_2(meter, 0x28));
    c.lwc1(0, at_2(meter, 0x2c));
    c.mul_s(0, 0, 6);
    c.swc1(0, at_2(meter, 0x30));
    c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x10));
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::be::beintstat::tests::allocator;
    pub(crate) use crate::game::be::beintstat::tests::call;
    pub(crate) use crate::game::be::beintstat::tests::pool_allocator as pool_allocator_2;
    pub(crate) use crate::game::be::beintstat::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::debugfe::tests::pool_allocator;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_2;
    pub(crate) use crate::game::fe::festaterewards::tests::BE_7 as BE;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::W;

    // RaceProgress_ProjectRider ------------------------------------------

    pub(crate) const WORLD: u32 = DATA + 0x1000;

    pub(crate) const RACERS: u32 = DATA + 0x1100;

    pub(crate) const RACE: u32 = DATA + 0x1200;

    pub(crate) const RIDER: u32 = DATA + 0x2000;

    pub(crate) const ZONES: u32 = DATA + 0x3000;

    pub(crate) const POS_VT: u32 = DATA + 0x4000;

    /// The world → race manager chain and the zone table pointer.
    pub(crate) fn put_world(c: &mut Cpu) {
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, RACERS);
        c.bus.write32(RACERS + 0xc, RACE);
        c.bus.write32(RACE + 0x78, 4);
        c.bus.write32(GAME_GP - 0x99c, ZONES);
    }

    /// The rider `idx`: its position object's virtual method is a fake.
    pub(crate) fn put_rider(c: &mut Cpu, idx: u32) {
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

    pub(crate) const COURSE: u32 = DATA + 0x5000;

    pub(crate) const COURSE_SEGS: u32 = DATA + 0x6000;

    pub(crate) fn put_course(c: &mut Cpu) {
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

    #[test]
    fn race_progress_reset_all_matches() {
        check(
            ee::s05::f_002108f8__generated,
            race_progress_reset_all,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x2108_f8, 0);
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_open_keyboard_matches() {
        check(
            ssx3_ee::s05::f_00210f40__generated,
            ov_screen_read_private_message_open_keyboard,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x210f40, 0);
                pool_allocator(c);
                w(c, OBJ + 0xe0, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = OBJ2 as u64;
                c.g[A3][0] = OBJ3 as u64;
                c.g[T0][0] = seed.wrapping_mul(3) as u64;
                c.g[T1][0] = seed.wrapping_mul(5) as u64;
                c.g[T2][0] = seed.wrapping_mul(7) as u64;
            },
        );
    }

    // -------------------------------------------------------- lobby pages

    #[test]
    fn ovscreen_read_private_message_create_keyboard_popup_matches() {
        strict(
            ee::s05::f_00210fa0__generated,
            ovscreen_read_private_message_create_keyboard_popup,
            40,
            &|c, seed| {
                let _ = base(c, seed, 0x210f, 6);
                allocator(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = ((seed % 3) + 1) as u64;
                c.g[A3][0] = (seed * 3) as u64;
                c.g[T0][0] = (seed % 5) as u64;
                w_2(c, DATA + 0x10, DATA + 0x900);
            },
        );
    }
}
