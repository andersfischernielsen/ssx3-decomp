//! Staging wave 201/202 (oc219): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    // The port already lives in its source unit (camera.cpp); it was kept
    // uninstalled for lack of a bounded fixture (live spline, heap). The test
    // below supplies one.
    (
        0x0016_4878,
        crate::game::camera::camera::chase_camera_apply_air_swing,
        "ChaseCameraAlgorithm_ApplyAirSwing",
    ),
    (0x0010_8a48, rider_find_rail, "Rider_FindRail"),
    (0x0038_9840, irradiance_rotate, "Irradiance_Rotate"),
    (
        0x0014_77e8,
        be_new_player_int_is_sharing_char_record,
        "BENewPlayerInt_IsSharingCharRecord",
    ),
    (0x0021_e750, hud_align_x, "HUD_AlignX"),
    (
        0x0013_4dd0,
        rider_spin_sub_apply_pose_rotation,
        "RiderSpinSub_ApplyPoseRotation",
    ),
];

// ---------------------------------------------------------------------------
// Rider_FindRail (0x108a48, seg_1218.cpp).

/// `Rider_GetMainState(rider)`.
const RIDER_GET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe98,
    f: ee::s00::Rider_GetMainState,
};
/// `Rider_ProbeRail(rider, pos, pos + 0x10)`.
const RIDER_PROBE_RAIL: Callee = Callee {
    addr: 0x0010_86b8,
    f: ee::s00::Rider_ProbeRail,
};
/// `OverlapList_FindNearestRail(list, pos, out, mask, radius)` (cross-shard).
const OVERLAP_LIST_FIND_NEAREST_RAIL: u32 = 0x0033_4680;
/// `AnimController_GetLayerAnimEntry(ctrl, layer)` (cross-shard).
const ANIM_GET_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;
/// `AnimController_GetLayerSequence(ctrl, layer)` (cross-shard).
const ANIM_GET_LAYER_SEQUENCE: u32 = 0x0031_1b20;
/// `Flags64_Test(flags, bit)` (cross-shard).
const FLAGS64_TEST: u32 = 0x0014_46a0;

/// Rider fields used by the rail search.
mod rail_rider {
    /// Velocity (`vec4`).
    pub const VELOCITY: u32 = 0x1e0;
    /// The rider's path (`+0x2c`: nodes, 32 bytes each).
    pub const PATH: u32 = 0x780;
    /// The animation controller.
    pub const ANIM: u32 = 0x784;
    /// The overlap list of objects near the rider.
    pub const OVERLAP: u32 = 0x860;
    /// Current path node index.
    pub const NODE: u32 = 0x8a0;
    /// Offset from the path node to the rider's probe point (`vec4`).
    pub const PROBE_OFFSET: u32 = 0x9d0;
}

/// `int Rider_FindRail(Rider* self, sVec4 out[2])` (0x108a48): in the main
/// states 0/1, look for a rail near the rider's probe point (path node +
/// offset, radius 300). Rejects a rail behind the rider (closer than -0.2 x
/// speed along the velocity) and, when the animation on layer 2 is a grab
/// (kind 0x12) or a trick (0x13/0x14), a rail the sequence's flags rule out;
/// otherwise returns `Rider_ProbeRail(self, out, out + 1)`. Returns 0 when
/// there is none.
pub fn rider_find_rail(c: &mut Cpu) {
    use rail_rider::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        let me = c.g[S0][0] as u32;
        let lw = |c: &mut Cpu, a: u32| c.lw(a);
        RIDER_GET_MAIN_STATE.call(c, 0x0010_8a64)?;
        'out: {
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                RIDER_GET_MAIN_STATE.call(c, 0x0010_8a74)?;
                c.g[V1][0] = 1;
                let other = c.g[V0][0] != 1;
                c.g[V0][0] = 0;
                if other {
                    break 'out;
                }
                c.cycles += 1;
            }
            c.g[A0][0] = lw(c, me.wrapping_add(PATH));

            // 0x108a84: probe point = path node + offset; nearest rail.
            c.cycles += 19;
            c.g[A1][0] = c.g[SP][0];
            c.g[V0][0] = lw(c, me.wrapping_add(NODE));
            c.g[A2][0] = c.g[S1][0];
            c.g[V1][0] = lw(c, (c.g[A0][0] as u32).wrapping_add(0x2c));
            c.g[A3][0] = 1;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
            c.g[AT][0] = 0x4396_0000; // 300.0
            c.mtc1(1, 12);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lqc2(3, c.g[V1][0] as u32);
            c.lqc2(4, me.wrapping_add(PROBE_OFFSET));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp(c).wrapping_add(0x10));
            c.lq(2, sp(c).wrapping_add(0x10));
            c.g[A0][0] = lw(c, me.wrapping_add(OVERLAP));
            c.sq(2, sp(c));
            call_ptr(c, OVERLAP_LIST_FIND_NEAREST_RAIL, 0x0010_8ac8)?;
            let none = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if none {
                break 'out;
            }

            // 0x108ad0: |velocity| (VU0 length).
            c.cycles += 15;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7e38));
            c.lqc2(3, me.wrapping_add(VELOCITY));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 2);
            c.c_lt_s(0, 2);
            if c.fcc() {
                // 0x108b0c: moving: reject a rail behind the rider.
                c.cycles += 21;
                let out = c.g[S1][0] as u32;
                c.lqc2(3, out);
                c.lqc2(4, sp(c));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x20));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7e34));
                c.lq(2, sp(c).wrapping_add(0x20));
                c.mul_s(1, 2, 1);
                c.sq(2, sp(c).wrapping_add(0x10));
                c.lqc2(3, sp(c).wrapping_add(0x10));
                c.lqc2(5, me.wrapping_add(VELOCITY));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(3, 4);
                c.mtc1(3, 0);
                c.c_lt_s(0, 1);
                let behind = c.fcc();
                c.g[V0][0] = 0;
                if behind {
                    break 'out;
                }
                c.cycles += 1;
            }
            c.g[A0][0] = lw(c, me.wrapping_add(ANIM));

            // 0x108b64: the animation on layer 2 may forbid the rail.
            c.cycles += 5;
            c.g[A1][0] = 2;
            call_ptr(c, ANIM_GET_LAYER_ANIM_ENTRY, 0x0010_8b6c)?;
            c.g[V1][0] = 0x12;
            let grab = c.g[V0][0] != 0x12;
            c.g[A0][0] = lw(c, me.wrapping_add(ANIM));
            let probe;
            if !grab {
                // Kind 0x12: allowed when flag 2 is set or flag 0 is clear.
                c.cycles += 7;
                c.g[A1][0] = 2;
                call_ptr(c, ANIM_GET_LAYER_SEQUENCE, 0x0010_8b80)?;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                c.g[A1][0] = 2;
                call_ptr(c, FLAGS64_TEST, 0x0010_8b8c)?;
                let flag2 = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if flag2 {
                    probe = true;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = lw(c, me.wrapping_add(ANIM));
                    c.g[A1][0] = 2;
                    call_ptr(c, ANIM_GET_LAYER_SEQUENCE, 0x0010_8ba0)?;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = 0;
                    call_ptr(c, FLAGS64_TEST, 0x0010_8bac)?;
                    probe = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                }
            } else {
                // 0x108bbc: kinds 0x13/0x14 need flag 2.
                c.cycles += 5;
                c.g[A1][0] = 2;
                call_ptr(c, ANIM_GET_LAYER_ANIM_ENTRY, 0x0010_8bc4)?;
                c.g[V1][0] = 0x13;
                let is_13 = c.g[V0][0] == 0x13;
                c.g[A0][0] = lw(c, me.wrapping_add(ANIM));
                let mut check = true;
                if !is_13 {
                    c.cycles += 5;
                    c.g[A1][0] = 2;
                    call_ptr(c, ANIM_GET_LAYER_ANIM_ENTRY, 0x0010_8bd8)?;
                    c.g[V1][0] = 0x14;
                    check = c.g[V0][0] == 0x14;
                    c.g[A1][0] = c.g[S1][0];
                    if check {
                        c.cycles += 1;
                        c.g[A0][0] = lw(c, me.wrapping_add(ANIM));
                    }
                }
                if check {
                    // 0x108be8
                    c.cycles += 7;
                    c.g[A1][0] = 2;
                    call_ptr(c, ANIM_GET_LAYER_SEQUENCE, 0x0010_8bf0)?;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = 2;
                    call_ptr(c, FLAGS64_TEST, 0x0010_8bfc)?;
                    probe = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[S1][0];
                    // The branch back to 0x108bb4 is a back edge: an event
                    // may be due there (and may unwind; re-entry at 0x108bb4
                    // runs the generated code).
                    if !probe && crate::events::back_edge_unwinds(c, 0x0010_8bb4) {
                        return Err(crate::call::Unwound);
                    }
                } else {
                    probe = true;
                }
            }
            if probe {
                // 0x108c04
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                RIDER_PROBE_RAIL.call(c, 0x0010_8c10)?;
            } else {
                // 0x108bb4
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        // 0x108c10
        c.cycles += 5;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Irradiance_Rotate (0x389840, particle.cpp; assembly only in the reference).

/// FPU registers the function saves (f20..f31) and their frame slots.
const IRR_SAVED_FPRS: [(usize, u32); 12] = [
    (31, 0x98),
    (30, 0x90),
    (29, 0x88),
    (28, 0x80),
    (27, 0x78),
    (26, 0x70),
    (25, 0x68),
    (24, 0x60),
    (23, 0x58),
    (22, 0x50),
    (21, 0x48),
    (20, 0x40),
];

/// `void Irradiance_Rotate(Irradiance* irr, const float rot[3][3], int first,
/// int last)` (0x389840): rotate the irradiance's spherical-harmonic
/// coefficients by the 3x3 matrix `rot`, for the colour channels
/// `first..=last` (one float column per channel; the coefficient rows are
/// `irr + 0x10 .. irr + 0x90`, 16 bytes apart). The SH rotation is the
/// original's fully scheduled FPU sequence (207 cycles per channel), kept
/// instruction for instruction so every scratch FPR and the stack scratch
/// (`sp+0x00..0x34`: the matrix row 0 copy and products) end as it leaves
/// them.
pub fn irradiance_rotate(c: &mut Cpu) {
    c.cycles += 31;
    let frame = Frame::open(c, 0xa0);
    let s = sp(c);
    // t4 = irr, t5 = channel, t7 = last channel.
    c.g[T7][0] = c.g[A3][0];
    c.g[T5][0] = c.g[A2][0];
    c.g[T4][0] = c.g[A0][0];
    c.g[V0][0] = ((c.g[T7][0] as i64) < (c.g[T5][0] as i64)) as u64;
    for (f, off) in IRR_SAVED_FPRS {
        frame.save_fpr(c, f, off);
    }
    // The packed 3x3 matrix: rot[0][0], rot[0][1], rot[1][0] and rot[2][0]
    // to the stack, the other five in f27..f31.
    let m = c.g[A1][0] as u32;
    c.lwc1(0, m);
    c.lwc1(30, m.wrapping_add(0x20));
    c.swc1(0, s);
    c.lwc1(1, m.wrapping_add(0x4));
    c.swc1(1, s.wrapping_add(0x4));
    c.lwc1(2, m.wrapping_add(0xc));
    c.lwc1(29, m.wrapping_add(0x8));
    c.swc1(2, s.wrapping_add(0x8));
    c.lwc1(4, m.wrapping_add(0x18));
    c.lwc1(27, m.wrapping_add(0x10));
    c.lwc1(31, m.wrapping_add(0x14));
    c.swc1(4, s.wrapping_add(0xc));
    c.lwc1(28, m.wrapping_add(0x1c));
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        loop {
            c.cycles += 207;
            // The channel's column in each coefficient row.
            let irr = c.g[T4][0] as u32;
            let col = (c.g[T5][0] as u32) << 2;
            c.g[V0][0] = sx(col);
            let row = |off: u32| irr.wrapping_add(off).wrapping_add(col);
            let (c10, c20, c30, c40) = (row(0x10), row(0x20), row(0x30), row(0x40));
            let (c50, c60, c70, c80, c90) = (row(0x50), row(0x60), row(0x70), row(0x80), row(0x90));
            c.g[A0][0] = sx(c10);
            c.g[A3][0] = sx(c20);
            c.g[A1][0] = sx(c30);
            c.g[T2][0] = sx(c40);
            c.g[T1][0] = sx(c50);
            c.g[T3][0] = sx(c60);
            c.g[V1][0] = sx(c70);
            c.g[T0][0] = sx(c80);
            c.g[A2][0] = sx(c90);
            c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T7][0] as i64) < (c.g[T5][0] as i64)) as u64;
            c.lwc1(12, c20);
            c.lwc1(13, c10);
            c.add_s(5, 12, 12);
            c.lwc1(6, s.wrapping_add(0x8));
            c.lwc1(8, s.wrapping_add(0xc));
            c.swc1(5, s.wrapping_add(0x2c));
            c.lwc1(16, c40);
            c.lwc1(20, c60);
            c.mul_s(6, 16, 6);
            c.mul_s(8, 16, 8);
            c.lwc1(7, s.wrapping_add(0x4));
            c.mul_s(11, 20, 28);
            c.lwc1(19, s);
            c.swc1(6, s.wrapping_add(0x18));
            c.mul_s(26, 5, 7);
            c.mul_s(2, 6, 7);
            c.lwc1(9, c30);
            c.swc1(8, s.wrapping_add(0x20));
            c.mul_s(16, 16, 19);
            c.lwc1(10, c50);
            c.add_s(22, 13, 13);
            c.swc1(11, s.wrapping_add(0x24));
            c.mul_s(4, 26, 27);
            c.mul_s(25, 10, 19);
            c.lwc1(15, s.wrapping_add(0x8));
            c.lwc1(0, c80);
            c.mul_s(1, 8, 7);
            c.lwc1(19, s.wrapping_add(0x4));
            c.mul_s(3, 22, 15);
            c.swc1(0, s.wrapping_add(0x14));
            c.add_s(2, 2, 4);
            c.lwc1(5, s.wrapping_add(0x24));
            c.mul_s(0, 12, 19);
            c.lwc1(6, c70);
            c.add_s(11, 9, 9);
            c.lwc1(17, c90);
            c.mul_s(8, 5, 29);
            c.swc1(6, s.wrapping_add(0x10));
            c.mul_s(4, 25, 31);
            c.mul_s(6, 9, 29);
            c.lwc1(7, s.wrapping_add(0xc));
            c.add_s(1, 1, 8);
            c.lwc1(15, s.wrapping_add(0x18));
            c.mul_s(0, 0, 19);
            c.lwc1(8, s.wrapping_add(0x8));
            c.mul_s(6, 6, 29);
            c.mul_s(24, 10, 7);
            c.mul_s(3, 3, 7);
            c.mul_s(23, 11, 29);
            c.mul_s(7, 16, 28);
            c.add_s(2, 2, 4);
            c.mul_s(21, 20, 27);
            c.lwc1(4, s);
            c.mul_s(10, 10, 8);
            c.add_s(0, 0, 6);
            c.lwc1(6, s.wrapping_add(0x4));
            c.mul_s(5, 15, 28);
            c.mul_s(8, 23, 31);
            c.mul_s(22, 22, 4);
            c.add_s(1, 1, 7);
            c.mul_s(20, 20, 6);
            c.mul_s(14, 10, 31);
            c.mul_s(18, 16, 6);
            c.mul_s(4, 21, 31);
            c.lwc1(6, s);
            c.mul_s(7, 12, 27);
            c.mul_s(15, 24, 31);
            c.add_s(3, 3, 5);
            c.mul_s(11, 11, 31);
            c.mul_s(5, 9, 30);
            c.add_s(2, 2, 8);
            c.mul_s(19, 25, 30);
            c.mul_s(8, 13, 6);
            c.add_s(4, 4, 14);
            c.mul_s(7, 7, 27);
            c.mul_s(9, 9, 31);
            c.mul_s(6, 21, 29);
            c.add_s(3, 3, 15);
            c.mul_s(11, 11, 30);
            c.mul_s(15, 24, 30);
            c.mul_s(5, 5, 30);
            c.add_s(1, 1, 19);
            c.add_s(4, 4, 7);
            c.lwc1(19, s.wrapping_add(0xc));
            c.mul_s(9, 9, 31);
            c.lwc1(7, s.wrapping_add(0x8));
            c.add_s(3, 3, 11);
            c.add_s(2, 2, 6);
            c.lwc1(11, s.wrapping_add(0x24));
            c.lwc1(6, s);
            c.add_s(5, 5, 15);
            c.mul_s(14, 22, 19);
            c.lwc1(15, s.wrapping_add(0x20));
            c.add_s(4, 4, 9);
            c.mul_s(19, 11, 31);
            c.lwc1(9, s.wrapping_add(0x18));
            c.mul_s(8, 8, 6);
            c.add_s(0, 0, 18);
            c.mul_s(11, 15, 28);
            c.mul_s(15, 20, 31);
            c.mul_s(6, 13, 7);
            c.mul_s(9, 9, 27);
            c.mul_s(7, 20, 29);
            c.add_s(3, 3, 19);
            c.add_s(0, 0, 8);
            c.lwc1(19, s.wrapping_add(0xc));
            c.lwc1(8, s.wrapping_add(0x8));
            c.add_s(5, 5, 11);
            c.swc1(9, s.wrapping_add(0x1c));
            c.add_s(2, 2, 15);
            c.mul_s(12, 12, 28);
            c.lwc1(15, s.wrapping_add(0x24));
            c.mul_s(11, 10, 30);
            c.add_s(0, 0, 7);
            c.mul_s(25, 25, 29);
            c.mul_s(10, 10, 29);
            c.mul_s(15, 15, 30);
            c.mul_s(12, 12, 28);
            c.mul_s(13, 13, 19);
            c.add_s(0, 0, 25);
            c.swc1(15, s.wrapping_add(0x28));
            c.mul_s(6, 6, 8);
            c.add_s(3, 3, 11);
            c.add_s(2, 2, 10);
            c.lwc1(11, s.wrapping_add(0x2c));
            c.add_s(1, 1, 14);
            c.lwc1(10, s.wrapping_add(0x20));
            c.mul_s(26, 26, 28);
            c.swc1(0, c10);
            c.add_s(5, 5, 12);
            c.lwc1(0, s.wrapping_add(0x4));
            c.mul_s(13, 13, 19);
            c.add_s(4, 4, 6);
            c.lwc1(19, s.wrapping_add(0x14));
            c.mul_s(22, 22, 8);
            c.add_s(1, 1, 26);
            c.lwc1(8, s.wrapping_add(0x1c));
            c.mul_s(23, 23, 30);
            c.mul_s(21, 21, 30);
            c.mul_s(16, 16, 27);
            c.add_s(5, 5, 13);
            c.mul_s(7, 10, 27);
            c.add_s(4, 4, 8);
            c.lwc1(10, s.wrapping_add(0x10));
            c.add_s(1, 1, 23);
            c.mul_s(24, 24, 29);
            c.add_s(3, 3, 21);
            c.mul_s(12, 11, 27);
            c.add_s(5, 5, 15);
            c.lwc1(11, s.wrapping_add(0x8));
            c.add_s(2, 2, 16);
            c.swc1(4, c20);
            c.add_s(3, 3, 7);
            c.add_s(1, 1, 24);
            c.swc1(5, c30);
            c.mul_s(20, 20, 30);
            c.mul_s(9, 19, 0);
            c.lwc1(15, s);
            c.mul_s(7, 19, 28);
            c.add_s(2, 2, 22);
            c.mul_s(19, 19, 27);
            c.add_s(1, 1, 20);
            c.swc1(2, c40);
            c.mul_s(6, 17, 29);
            c.swc1(19, s.wrapping_add(0x30));
            c.mul_s(8, 17, 31);
            c.swc1(1, c50);
            c.mul_s(0, 10, 11);
            c.lwc1(1, s.wrapping_add(0xc));
            c.mul_s(12, 12, 28);
            c.mul_s(17, 17, 30);
            c.add_s(6, 6, 9);
            c.mul_s(4, 10, 15);
            c.mul_s(1, 10, 1);
            c.add_s(0, 0, 8);
            c.add_s(3, 3, 12);
            c.add_s(17, 17, 7);
            c.swc1(1, s.wrapping_add(0x34));
            c.add_s(6, 6, 4);
            c.add_s(0, 0, 19);
            c.swc1(3, c60);
            c.add_s(17, 17, 1);
            c.swc1(6, c70);
            c.swc1(0, c80);
            // (In the original this last store sits after the loop test,
            // in the branch delay slot.)
            c.swc1(17, c90);
            if c.g[V0][0] != 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0038_98c0);
            }
        }
    }
    c.cycles += 14;
    for (f, off) in IRR_SAVED_FPRS {
        frame.restore_fpr(c, f, off);
    }
    frame.close(c);
}

// ---------------------------------------------------------------------------
// BENewPlayerInt_IsSharingCharRecord (0x1477e8, beintnewrace.cpp).

/// `BENewPlayerInt_GetSlot(player)` (same shard).
const BE_NEW_PLAYER_INT_GET_SLOT: Callee = Callee {
    addr: 0x0014_7798,
    f: ee::s01::BENewPlayerInt_GetSlot,
};
/// Per-slot character records (0x1c bytes each); flags word at +0x10, bit 2
/// "uses slot 0's character record".
const CHAR_RECORDS: u32 = 0x0053_4fe0;
const CHAR_RECORD_SIZE: u32 = 0x1c;
const CHAR_RECORD_FLAGS: u32 = 0x10;

/// `bool BENewPlayerInt_IsSharingCharRecord(int player)` (0x1477e8): whether
/// the character record of the player's race slot is shared with slot 0
/// (flags bit 2). 15 cycles plus the slot lookup; LO/HI keep the record
/// offset (`mult`), v1 the table base.
pub fn be_new_player_int_is_sharing_char_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        BE_NEW_PLAYER_INT_GET_SLOT.call(c, 0x0014_77f8)?;
        c.g[A0][0] = CHAR_RECORD_SIZE as u64;
        c.g[V1][0] = sx(CHAR_RECORDS);
        c.mult(0, V0, V0, A0); // v0 = slot * 0x1c
        frame.restore_ra(c, 0);
        let record = (c.g[V0][0] as u32).wrapping_add(CHAR_RECORDS);
        let flags = c.lw(record.wrapping_add(CHAR_RECORD_FLAGS)) as u32;
        c.g[V0][0] = ((flags >> 2) & 1) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// HUD_AlignX (0x21e750, ovstatehud.cpp).

/// The HUD's virtual screen width (640.0).
const HUD_SCREEN_WIDTH: u32 = 0x4420_0000;
/// 0.5.
const HALF: u32 = 0x3f00_0000;

/// `float HUD_AlignX(float x, int align, int flags, float width, float
/// scale)` (0x21e750): the left edge of an element of `width * scale` at
/// `x`. Flag 1 mirrors the layout (`x = 640 - x`, and the alignment flips
/// unless flag 2 is set); alignment 0 centres the element on `x`, positive
/// alignment right-aligns it, negative leaves `x` as the left edge.
pub fn hud_align_x(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    c.cycles += 3;
    let align = c.g[A1][0];
    let flags = c.g[A2][0];
    let mut x = FpuF32(c.f[12]);
    c.g[V0][0] = flags & 1;
    c.g[V1][0] = align;
    if flags & 1 != 0 {
        c.cycles += 6;
        c.g[AT][0] = HUD_SCREEN_WIDTH as u64;
        c.f[0] = HUD_SCREEN_WIDTH;
        c.g[V0][0] = flags & 2;
        c.g[V1][0] = sx(0u32.wrapping_sub(align as u32));
        x = FpuF32(HUD_SCREEN_WIDTH) - x;
        if flags & 2 != 0 {
            c.g[V1][0] = align;
        }
    }
    c.cycles += 2;
    let v = c.g[V1][0];
    let size = FpuF32(c.f[14]) * FpuF32(c.f[13]);
    let shift = if v == 0 {
        // Centred: half the size.
        c.cycles += 5;
        c.g[AT][0] = HALF as u64;
        c.f[1] = HALF;
        Some(size * FpuF32(HALF))
    } else {
        c.cycles += 2;
        if (v as i64) > 0 {
            c.cycles += 1;
            Some(size)
        } else {
            None
        }
    };
    if let Some(shift) = shift {
        c.cycles += 1;
        c.f[0] = shift.0;
        x = x - shift;
    }
    c.cycles += 2;
    c.f[12] = x.0;
    c.f[0] = x.0;
    ret(c);
}

// ---------------------------------------------------------------------------
// RiderSpinSub_ApplyPoseRotation (0x134dd0, ai.cpp; assembly only).

/// `void Math_SinCos(float* sin, float* cos, float angle)` (cross-shard).
const MATH_SIN_COS: u32 = 0x0031_be50;
/// `float RiderSpinSub_GetSpinFactor(RiderSpinSub*)` (same shard).
const RIDER_SPIN_SUB_GET_SPIN_FACTOR: Callee = Callee {
    addr: 0x0013_5180,
    f: ee::s01::RiderSpinSub_GetSpinFactor,
};

/// `void RiderSpinSub_ApplyPoseRotation(RiderSpinSub* self, Pose* pose)`
/// (0x134dd0): turns the rider's pose (`pose`: position at +0, rotation
/// quaternion at +0x10) for the spin sub-state. It
/// 1. scales the current path node's offset (path `+0x24`, node `+0x89c`,
///    16 bytes each) by the path's scale (`+0x140..+0x14c`), takes it
///    relative to the spin pivot (0x4ff130), rotates it by the pose
///    quaternion and adds it to the position;
/// 2. rotates the pose by the spin angle (`-(self+0x20 + self+0x28) / 2`
///    about world up, `Math_SinCos`);
/// 3. eases the lean factor `self+0x40` towards `GetSpinFactor()` by a step
///    proportional to the distance (`gp-0x7384`);
/// 4. rotates the pose by half of `self+0x24 + self+0x2c` about the axis
///    blended from 0x4ff150 and 0x4ff140 by the lean factor (normalised),
///    and moves the position back by the rotated pivot.
/// All vector work is the original's VU0 macro code (quaternion products
/// `opmula/opmsub`, rotations), issued in its order with its stack scratch.
pub fn rider_spin_sub_apply_pose_rotation(c: &mut Cpu) {
    guest(c, |c| {
        let me = c.g[A0][0] as u32;
        let pose = c.g[A1][0] as u32;
        // 1-2: offset to the pivot, rotated into the pose; spin angle.
        c.cycles += 116;
        let frame = Frame::open(c, 0xe0);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 12);
        c.sq(16, sp(c).wrapping_add(0xc0));
        c.sq(17, sp(c).wrapping_add(0xb0));
        c.g[S0][0] = 0x500000;
        c.sq(18, sp(c).wrapping_add(0xa0));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xea0));
        c.sq(19, sp(c).wrapping_add(0x90));
        c.g[S2][0] = c.g[A0][0];
        c.swc1(20, sp(c).wrapping_add(0xd0));
        c.g[S3][0] = c.g[A1][0];
        c.sd(sp(c).wrapping_add(0x80), c.g[RA][0]);
        c.g[A0][0] = sx(sp(c).wrapping_add(0x70));
        c.g[A1][0] = sx(sp(c).wrapping_add(0x74));
        c.g[A2][0] = c.lw(me.wrapping_add(0x58));
        c.lwc1(6, me.wrapping_add(0x28));
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x780));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x89c));
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x14c));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x140));
        c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(1, c.g[V0][0] as u32);
        c.mul_s(4, 4, 0);
        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.mul_s(1, 1, 3);
        c.lwc1(5, (c.g[V1][0] as u32).wrapping_add(0x144));
        c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x148));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.mul_s(2, 2, 5);
        c.lwc1(7, me.wrapping_add(0x20));
        c.g[V0][0] = 0x500000;
        c.mul_s(0, 0, 3);
        c.swc1(4, sp(c).wrapping_add(0x1c));
        c.swc1(2, sp(c).wrapping_add(0x14));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
        c.swc1(1, sp(c).wrapping_add(0x10));
        c.add_s(7, 7, 6);
        c.swc1(0, sp(c).wrapping_add(0x18));
        c.lwc1(0, me.wrapping_add(0x2c));
        c.lwc1(1, me.wrapping_add(0x24));
        c.neg_s(7, 7);
        c.lqc2(3, sp(c).wrapping_add(0x10));
        c.lqc2(4, c.g[V0][0] as u32);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x20));
        c.lq(2, sp(c).wrapping_add(0x20));
        c.add_s(20, 1, 0);
        c.mul_s(12, 7, 12);
        c.sq(2, sp(c));
        c.lqc2(4, pose.wrapping_add(0x10));
        c.lqc2(5, sp(c));
        c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().opmula(4, 6);
        c.vu0().opmsub(7, 6, 4);
        c.vu0().mula(0b1110, 5, Src::Bc(0, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 7, Src::Bc(0, W));
        c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
        c.sqc2(8, sp(c).wrapping_add(0x20));
        c.lq(2, sp(c).wrapping_add(0x20));
        c.sq(2, sp(c).wrapping_add(0x10));
        c.lqc2(3, pose);
        c.lqc2(4, sp(c).wrapping_add(0x10));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, pose);
        call_ptr(c, MATH_SIN_COS, 0x0013_4ef8)?;
        // Spin quaternion (axis = world up x sin, w = cos) times the pose.
        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.g[S2][0];
        c.lwc1(0, sp(c).wrapping_add(0x70));
        c.lwc1(2, (c.g[S0][0] as u32).wrapping_sub(0xea0));
        c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x4));
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.lwc1(3, sp(c).wrapping_add(0x74));
        c.mul_s(0, 0, 4);
        c.swc1(3, sp(c).wrapping_add(0x4c));
        c.swc1(1, sp(c).wrapping_add(0x48));
        c.swc1(2, sp(c).wrapping_add(0x40));
        c.swc1(0, sp(c).wrapping_add(0x44));
        c.lq(2, sp(c).wrapping_add(0x40));
        c.sq(2, sp(c).wrapping_add(0x30));
        c.lqc2(4, pose.wrapping_add(0x10));
        c.lqc2(5, sp(c).wrapping_add(0x30));
        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
        c.sqc2(8, sp(c).wrapping_add(0x60));
        c.lq(2, sp(c).wrapping_add(0x60));
        c.sq(2, pose.wrapping_add(0x10));
        c.sq(2, sp(c).wrapping_add(0x50));
        RIDER_SPIN_SUB_GET_SPIN_FACTOR.call(c, 0x0013_4f74)?;
        // 3: ease the lean factor (self+0x40, f4) towards the spin factor
        // (f3) by step = |current - target| * k (f2).
        c.lwc1(4, me.wrapping_add(0x40));
        c.mov_s(3, 0);
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x7384));
        c.sub_s(0, 4, 3);
        c.abs_s(0, 0);
        c.mul_s(2, 0, 2);
        c.add_s(1, 3, 2);
        c.c_lt_s(1, 4);
        if !c.fcc() {
            // target + step >= current: step up if current < target - step,
            // else snap to the target.
            c.sub_s(0, 3, 2);
            c.cycles += 4;
            c.c_lt_s(4, 0);
            let below = c.fcc();
            c.mov_s(1, 3);
            if below {
                c.cycles += 1;
                c.add_s(1, 4, 2);
            }
        } else {
            // current > target + step: step down.
            c.cycles += 2;
            c.sub_s(1, 4, 2);
        }
        // 4: the lean axis and its half-angle rotation.
        c.cycles += 112;
        c.g[V1][0] = 0x500000;
        c.swc1(1, me.wrapping_add(0x40));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xeb0));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.mfc1(2, 1);
        c.lqc2(4, c.g[V1][0] as u32);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp(c).wrapping_add(0x40));
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 12);
        c.lq(2, sp(c).wrapping_add(0x40));
        c.g[A0][0] = sx(sp(c).wrapping_add(0x78));
        c.lwc1(1, me.wrapping_add(0x40));
        c.mul_s(12, 20, 12);
        c.sq(2, sp(c).wrapping_add(0x30));
        c.g[A1][0] = sx(sp(c).wrapping_add(0x7c));
        c.g[V0][0] = 0x500000;
        c.sub_s(0, 0, 1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
        c.mfc1(3, 0);
        c.lqc2(4, c.g[V0][0] as u32);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp(c).wrapping_add(0x50));
        c.lq(2, sp(c).wrapping_add(0x50));
        c.sq(2, sp(c).wrapping_add(0x40));
        c.lqc2(3, sp(c).wrapping_add(0x30));
        c.lqc2(4, sp(c).wrapping_add(0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x50));
        c.lq(2, sp(c).wrapping_add(0x50));
        c.sq(2, sp(c).wrapping_add(0x20));
        c.lqc2(3, sp(c).wrapping_add(0x20));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, sp(c).wrapping_add(0x50));
        c.lq(2, sp(c).wrapping_add(0x50));
        c.sq(2, sp(c).wrapping_add(0x10));
        call_ptr(c, MATH_SIN_COS, 0x0013_5078)?;
        c.lwc1(0, sp(c).wrapping_add(0x78));
        c.lwc1(1, sp(c).wrapping_add(0x10));
        c.lwc1(2, sp(c).wrapping_add(0x14));
        c.lwc1(4, sp(c).wrapping_add(0x18));
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.lwc1(3, sp(c).wrapping_add(0x7c));
        c.mul_s(0, 0, 4);
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 5);
        c.swc1(3, sp(c).wrapping_add(0x3c));
        c.swc1(1, sp(c).wrapping_add(0x30));
        c.swc1(2, sp(c).wrapping_add(0x34));
        c.swc1(0, sp(c).wrapping_add(0x38));
        c.lq(16, sp(c).wrapping_add(0xc0));
        c.lq(2, sp(c).wrapping_add(0x30));
        c.lq(17, sp(c).wrapping_add(0xb0));
        c.sq(2, sp(c).wrapping_add(0x20));
        c.lqc2(4, pose.wrapping_add(0x10));
        c.lqc2(5, sp(c).wrapping_add(0x20));
        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
        c.sqc2(8, sp(c).wrapping_add(0x50));
        c.lq(18, sp(c).wrapping_add(0xa0));
        c.lq(2, sp(c).wrapping_add(0x50));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x80));
        c.sq(2, pose.wrapping_add(0x10));
        c.sq(2, sp(c).wrapping_add(0x40));
        c.mfc1(3, 5);
        c.lqc2(4, sp(c));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp(c).wrapping_add(0x30));
        c.lwc1(20, sp(c).wrapping_add(0xd0));
        c.lq(2, sp(c).wrapping_add(0x30));
        c.sq(2, sp(c).wrapping_add(0x20));
        c.lqc2(4, pose.wrapping_add(0x10));
        c.lqc2(5, sp(c).wrapping_add(0x20));
        c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().opmula(4, 6);
        c.vu0().opmsub(7, 6, 4);
        c.vu0().mula(0b1110, 5, Src::Bc(0, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 7, Src::Bc(0, W));
        c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
        c.sqc2(8, sp(c).wrapping_add(0x40));
        c.lq(2, sp(c).wrapping_add(0x40));
        c.sq(2, sp(c).wrapping_add(0x30));
        c.lqc2(3, pose);
        c.lqc2(4, sp(c).wrapping_add(0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, pose);
        c.lq(19, sp(c).wrapping_add(0x90));
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, FAKE, LOG_UNWIND_AT};
    use crate::difftest::{elf_image, put_vec, Rng, DATA};

    // ------------------------------------------------------------------
    // ChaseCameraAlgorithm_ApplyAirSwing (0x164878).

    /// The chase-camera state, its swing info, the camera target ("rig",
    /// first word its vtable), the vtable and the rider the target points at.
    /// All above the fake results (`DATA..DATA+0x40000`), the heap arena
    /// (`DATA+0x40000..+0x80000`) and the fake log / world (`DATA+0x10_0000`).
    const AS_ME: u32 = DATA + 0x20_0000;
    const AS_INFO: u32 = DATA + 0x20_1000;
    const AS_RIG: u32 = DATA + 0x20_2000;
    const AS_VT: u32 = DATA + 0x20_2800;
    const AS_RIDER: u32 = DATA + 0x20_3000;
    /// Fields of the state (`ChaseCameraAlgorithm`).
    const AS_EYE_TARGET: u32 = 0x20;
    const AS_EYE: u32 = 0x40;
    const AS_TARGET: u32 = 0x30;
    const AS_SWING_CLOCK: u32 = 0x22c;
    const AS_DURATION: u32 = 0x230;
    const AS_TIME: u32 = 0x234;
    const AS_BLEND_OUT: u32 = 0x23c;
    const AS_BLEND_A: u32 = 0x240;
    const AS_BLEND_B: u32 = 0x244;
    const AS_AIRBORNE: u32 = 0x2d0;
    const AS_SWINGING: u32 = 0x2d8;
    const AS_SPLINE: u32 = 0x334;
    /// A real leaf that returns 0 (`v0 = 0; jr ra`), for the "no swing" answer
    /// of vtable slot 0x98 (the fakes never return 0).
    const RETURN_ZERO: u32 = 0x0014_06c0;
    /// `cCSICubicSpline_InitWithStartSlope` / `cCSICubicSpline_AddPoint`.
    const SPLINE_INIT: u32 = 0x0031_d738;
    const SPLINE_ADD: u32 = 0x0031_d7e0;

    /// Call a real guest function from the setup with `f12`/`f13` set.
    fn setup_call(c: &mut Cpu, addr: u32, f: GuestFn, a0: u32, a1: u64, f12: f32, f13: f32) {
        c.f[12] = f12.to_bits();
        c.f[13] = f13.to_bits();
        crate::game::camera::camera::tests::run(c, addr, f, &[sx(a0), a1]);
    }

    /// One air-swing frame: airborne or not, swing running or about to start
    /// (rig says yes / no), the spline initialised in the call itself or
    /// built beforehand (a live 9-point spline on the test heap), blend-out
    /// and the final minimum-distance clamp; every rig query is a logging fake
    /// (some unwind), `CheckDistanceReset` and the spline/maths are real.
    #[test]
    fn chase_camera_algorithm_apply_air_swing_matches() {
        covered(
            ee::s01::f_00164878__generated,
            crate::game::camera::camera::chase_camera_apply_air_swing,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = crate::game::camera::camera::tests::alloc_base(c, seed, 0x0016_4878);
                // Fake calls: 0x98 (n=0), 0x60 (1, 2), 0x80 (3), reset slot 8.
                let unwind = if seed % 3 == 0 { r.next_u32() % 5 } else { u32::MAX };
                c.bus.write32(LOG_UNWIND_AT, unwind);
                for i in 0..0x360 / 4 {
                    c.bus.write32(AS_ME + 4 * i, 0);
                }
                // The rig: vtable, rider pointer (CheckDistanceReset's reset).
                c.bus.write32(AS_RIG, AS_VT);
                c.bus.write32(AS_RIG + 4, AS_RIDER);
                for (i, slot) in [0x08u32, 0x60, 0x80, 0x98].into_iter().enumerate() {
                    let delta = (r.next_u32() % 0x40) & !3;
                    c.bus.write16(AS_VT + slot, delta as u16);
                    c.bus.write32(AS_VT + slot + 4, FAKE + 0x10 * i as u32);
                }
                if seed % 5 == 0 {
                    c.bus.write32(AS_VT + 0x98 + 4, RETURN_ZERO);
                }
                put_vec(c, AS_RIDER + 0x1b0, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                c.bus.write32(AS_ME + AS_TARGET, AS_RIG);
                // World up (D_004FF160) and random vectors where the fake
                // results point (below the pool allocator's tables at
                // DATA+0x20000; above them the fakes read allocator words).
                put_vec(c, 0x004f_f160, [0, 0, 1.0f32.to_bits(), 0]);
                for i in 0..0x2_0000 / 4 {
                    let v = if i % 4 == 3 { 0 } else { r.f32(1.0) };
                    c.bus.write32(DATA + 4 * i, v);
                }
                let big = if seed % 7 == 0 { 30.0 } else { 600.0 };
                put_vec(c, AS_ME + AS_EYE_TARGET, [r.f32(big), r.f32(big), r.f32(big), 0]);
                put_vec(c, AS_ME + AS_EYE, [r.f32(big), r.f32(big), r.f32(big), 0]);
                for off in [0x110u32, 0x120, 0x130, 0x140, 0x150, 0x160, 0x170, 0x180] {
                    put_vec(c, AS_ME + off, [r.f32(300.0), r.f32(300.0), r.f32(300.0), 0]);
                }
                put_vec(c, AS_INFO + 0x10, [r.f32(600.0), r.f32(600.0), r.f32(600.0), 0]);
                put_vec(c, AS_INFO + 0x20, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                let airborne = seed % 4 != 0;
                let swinging = seed % 3 == 1;
                c.bus.write32(AS_ME + AS_AIRBORNE, airborne as u32);
                c.bus.write32(AS_ME + AS_SWINGING, swinging as u32);
                c.bus.write32(AS_ME + AS_SWING_CLOCK, r.f32(1.0));
                c.bus.write32(AS_ME + AS_DURATION, (2.3 + (r.next_u32() % 100) as f32 / 50.0).to_bits());
                let time = (r.next_u32() % 400) as f32 / 100.0;
                c.bus.write32(AS_ME + AS_TIME, time.to_bits());
                c.bus.write32(AS_ME + 0x238, r.f32(1.5));
                let blend = |r: &mut Rng, zero: bool| if zero { 0 } else { ((r.next_u32() % 100) as f32 / 100.0).to_bits() };
                let z = seed % 6 == 2;
                let v = blend(&mut r, z);
                c.bus.write32(AS_ME + AS_BLEND_OUT, v);
                let v = blend(&mut r, seed % 5 == 3);
                c.bus.write32(AS_ME + AS_BLEND_A, v);
                let v = blend(&mut r, seed % 7 == 4);
                c.bus.write32(AS_ME + AS_BLEND_B, v);
                if swinging {
                    // A live spline, as the start of the swing leaves it.
                    let spl = AS_ME + AS_SPLINE;
                    setup_call(c, SPLINE_INIT, ee::s10::f_0031d738, spl, 9, 0.0, 0.0);
                    let mut y = 0.0f32;
                    for i in 0..9 {
                        y += (r.next_u32() % 100) as f32 / 800.0;
                        setup_call(c, SPLINE_ADD, ee::s10::f_0031d7e0, spl, 0, i as f32 / 8.0, y);
                    }
                }
                // Arguments: (this, info, blend-out time).
                c.g[A0][0] = sx(AS_ME);
                c.g[A1][0] = sx(AS_INFO);
                c.f[12] = ((r.next_u32() % 100) as f32 / 100.0 + 0.1).to_bits();
                c.g[RA][0] = crate::difftest::RA as u64;
                c.pc = 0x0016_4878;
            },
        );
    }

    // ------------------------------------------------------------------
    // Rider_FindRail (0x108a48).

    /// The rider, its state object, path, nodes, overlap list with one plain
    /// rail object (bounds, matrix, flags), animation controller and
    /// sequence list; the output pair of vectors.
    const FR_ME: u32 = DATA + 0x1000;
    const FR_STATE: u32 = DATA + 0x2000;
    const FR_PATH: u32 = DATA + 0x3000;
    const FR_NODES: u32 = DATA + 0x3100;
    const FR_LIST: u32 = DATA + 0x4000;
    const FR_RAIL: u32 = DATA + 0x5000;
    const FR_RAIL_FLAGS: u32 = DATA + 0x5200;
    const FR_ANIM: u32 = DATA + 0x6000;
    const FR_SEQS: u32 = DATA + 0x6100;
    const FR_SEQ: u32 = DATA + 0x6200;
    const FR_OUT: u32 = DATA + 0x7000;
    /// `AnimTable` (0x446990, 0x1c bytes per animation): the test writes the
    /// kind of animation 7.
    const ANIM_TABLE: u32 = 0x0044_6990;

    /// Main state 0/1/other; one rail in reach or none (or masked out); the
    /// rider still or moving (rail ahead / behind); layer-2 animation none,
    /// grab (0x12), trick (0x13/0x14) or other, with every combination of
    /// sequence flags 0 and 2. All callees are real (bounded on this data).
    #[test]
    fn rider_find_rail_matches() {
        use crate::difftest::fakes::machine;
        crate::difftest::check(ee::s00::f_00108a48__generated, rider_find_rail, 300, &|c, seed| {
            elf_image(c);
            let mut r = machine(c, seed, 0x0010_8a48, 0);
            let me = FR_ME;
            c.bus.write32(me + 0x77c, FR_STATE);
            c.bus.write32(FR_STATE + 0xde0, [0, 1, 0, 1, 2][(seed % 5) as usize]);
            // Path node + probe offset.
            c.bus.write32(me + rail_rider::PATH, FR_PATH);
            c.bus.write32(FR_PATH + 0x2c, FR_NODES);
            let node = r.next_u32() % 4;
            c.bus.write32(me + rail_rider::NODE, node);
            for i in 0..4 {
                put_vec(c, FR_NODES + 0x20 * i, [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0]);
            }
            put_vec(c, me + rail_rider::PROBE_OFFSET, [r.f32(5.0), r.f32(5.0), r.f32(5.0), 0]);
            let vel = if seed % 5 == 3 {
                [0, 0, 0, 0]
            } else {
                [r.f32(50.0), r.f32(50.0), r.f32(50.0), 0]
            };
            put_vec(c, me + rail_rider::VELOCITY, vel);
            c.bus.write32(me + 0x25c, r.f32(10.0));
            c.bus.write32(me + 0x330, r.next_u32() & 1);
            // The overlap list: none, or one plain rail (type 1) whose flags
            // pass the mask (1) or not.
            c.bus.write32(me + rail_rider::OVERLAP, FR_LIST);
            c.bus.write32(FR_LIST + 0x210, (seed % 7 != 0) as u32);
            c.bus.write32(FR_LIST + 0x214, FR_RAIL);
            c.bus.write32(FR_RAIL + 8, 1);
            c.bus.write32(FR_RAIL + 0x68, FR_RAIL_FLAGS);
            c.bus.write32(FR_RAIL_FLAGS + 0x1c, (seed % 11 != 5) as u32);
            c.bus.write32(FR_RAIL_FLAGS + 0x28, r.next_u32());
            for off in [0x6c, 0x70, 0x74] {
                c.bus.write32(FR_RAIL + off, (-1.0e4f32).to_bits());
            }
            for off in [0x78, 0x7c, 0x80] {
                c.bus.write32(FR_RAIL + off, 1.0e4f32.to_bits());
            }
            for i in 0..4 {
                let w = if i == 3 { 1.0f32.to_bits() } else { 0 };
                put_vec(c, FR_RAIL + 0x10 + 0x10 * i, [r.f32(100.0), r.f32(100.0), r.f32(100.0), w]);
            }
            // Layer-2 animation (0x1b6 = none) and its sequence flags.
            c.bus.write32(me + rail_rider::ANIM, FR_ANIM);
            c.bus.write32(FR_ANIM + 8, if seed % 6 == 0 { 0x1b6 } else { 7 });
            let kind = [0x12u32, 0x13, 0x14, 0x12, 5][(r.next_u32() % 5) as usize];
            c.bus.write32(ANIM_TABLE + 7 * 0x1c, kind);
            c.bus.write32(FR_ANIM + 0x50, FR_SEQS);
            c.bus.write32(FR_SEQS + 0x10 + 4, FR_SEQ);
            c.bus.write64(FR_SEQ + 0xb0, (r.next_u32() & 0b101) as u64);
            put_vec(c, FR_OUT, [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0]);
            c.g[A0][0] = sx(me);
            c.g[A1][0] = sx(FR_OUT);
            c.pc = 0x0010_8a48;
        });
    }

    // ------------------------------------------------------------------
    // Irradiance_Rotate (0x389840).

    /// Random coefficients and matrix (edge-case floats too), channel ranges
    /// empty, single and up to four channels.
    #[test]
    fn irradiance_rotate_matches() {
        use crate::difftest::{scramble, Rng};
        const IRR: u32 = DATA + 0x100;
        const ROT: u32 = DATA + 0x400;
        crate::difftest::check(ee::s12::f_00389840__generated, irradiance_rotate, 300, &|c, seed| {
            let mut r = Rng(0x3898_40a1 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            for i in 0..0xa0 / 4 {
                let v = if seed % 9 == 0 { r.edge_f32() } else { r.f32(4.0) };
                c.bus.write32(IRR + 4 * i, v);
            }
            for i in 0..9 {
                let v = if seed % 7 == 0 { r.edge_f32() } else { r.f32(1.0) };
                c.bus.write32(ROT + 4 * i, v);
            }
            let first = r.next_u32() % 4;
            let last = if seed % 5 == 0 { first as i64 - 1 } else { (first + r.next_u32() % (4 - first)) as i64 };
            c.g[A0][0] = sx(IRR);
            c.g[A1][0] = sx(ROT);
            c.g[A2][0] = first as u64;
            c.g[A3][0] = last as u64;
            c.pc = 0x0038_9840;
        });
    }

    // ------------------------------------------------------------------
    // BENewPlayerInt_IsSharingCharRecord (0x1477e8).

    /// Players 0-3 (slot table 0x5305b0), random slots 0-3 and record flags.
    #[test]
    fn be_new_player_int_is_sharing_char_record_matches() {
        use crate::difftest::{scramble, Rng};
        crate::difftest::check_lohi(ee::s01::f_001477e8__generated, be_new_player_int_is_sharing_char_record, 200, &|c, seed| {
            let mut r = Rng(0x1477_e8 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            let player = seed % 4;
            for p in 0..4 {
                c.bus.write32(0x0053_05b0 + 4 * p, r.next_u32() % 4);
            }
            for slot in 0..4 {
                c.bus.write32(CHAR_RECORDS + CHAR_RECORD_SIZE * slot + CHAR_RECORD_FLAGS, r.next_u32());
            }
            c.g[A0][0] = player as u64;
            c.pc = 0x0014_77e8;
        });
    }

    // ------------------------------------------------------------------
    // HUD_AlignX (0x21e750).

    /// Alignments -2..2, all four flag combinations, random and edge floats.
    #[test]
    fn hud_align_x_matches() {
        use crate::difftest::{scramble, Rng};
        crate::difftest::check(ee::s05::f_0021e750__generated, hud_align_x, 300, &|c, seed| {
            let mut r = Rng(0x21e7_50 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            c.g[A1][0] = sx((seed % 5) as i32 as u32).wrapping_sub(2);
            c.g[A2][0] = ((seed / 5) % 4) as u64 | (r.next_u32() as u64 & !3) << 8;
            for f in [12, 13, 14] {
                c.f[f] = if seed % 6 == 0 { r.edge_f32() } else { r.f32(400.0) };
            }
            c.pc = 0x0021_e750;
        });
    }

    // ------------------------------------------------------------------
    // RiderSpinSub_ApplyPoseRotation (0x134dd0).

    /// Random spin state, rider path node and pose (unit-ish quaternion);
    /// the lean factor below, near and above the spin factor so all three
    /// easing branches run. `Math_SinCos`, `GetSpinFactor` (and its
    /// `Math_Atan`) are real, with the game's `.lit4` constants.
    #[test]
    fn rider_spin_sub_apply_pose_rotation_matches() {
        use crate::difftest::fakes::machine;
        const SELF: u32 = DATA + 0x100;
        const RIDER: u32 = DATA + 0x1000;
        const PATH: u32 = DATA + 0x2000;
        const NODES: u32 = DATA + 0x3000;
        const POSE: u32 = DATA + 0x4000;
        crate::difftest::check(ee::s01::f_00134dd0__generated, rider_spin_sub_apply_pose_rotation, 300, &|c, seed| {
            elf_image(c);
            let mut r = machine(c, seed, 0x0013_4dd0, 0);
            c.bus.write32(SELF + 0x58, RIDER);
            for off in [0x20u32, 0x24, 0x28, 0x2c] {
                c.bus.write32(SELF + off, r.f32(3.2));
            }
            // Lean factor: anywhere in [-2, 2] (the spin factor is atan-based).
            let lean = if seed % 4 == 0 { r.edge_f32() } else { r.f32(2.0) };
            c.bus.write32(SELF + 0x40, lean);
            c.bus.write32(RIDER + 0x2dc, r.f32(20.0));
            c.bus.write32(RIDER + 0x2e0, r.f32(20.0));
            c.bus.write32(RIDER + 0x780, PATH);
            let node = r.next_u32() % 4;
            c.bus.write32(RIDER + 0x89c, node);
            c.bus.write32(PATH + 0x24, NODES);
            for off in [0x140u32, 0x144, 0x148, 0x14c] {
                c.bus.write32(PATH + off, r.f32(2.0));
            }
            for i in 0..4 {
                put_vec(c, NODES + 0x10 * i, [r.f32(10.0), r.f32(10.0), r.f32(10.0), r.f32(1.0)]);
            }
            // The spin pivot, the two lean axes and world up.
            for a in [0x004f_f130u32, 0x004f_f140, 0x004f_f150] {
                put_vec(c, a, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
            }
            put_vec(c, 0x004f_f160, [0, 0, 1.0f32.to_bits(), 0]);
            put_vec(c, POSE, [r.f32(100.0), r.f32(100.0), r.f32(100.0), 1.0f32.to_bits()]);
            put_vec(c, POSE + 0x10, [r.f32(0.7), r.f32(0.7), r.f32(0.7), r.f32(1.0)]);
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(POSE);
            c.pc = 0x0013_4dd0;
        });
    }
}
