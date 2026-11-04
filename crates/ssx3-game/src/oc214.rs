//! Staging wave 201/202 (oc214): hand-written replacements for the hottest
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
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0033_4680,
        overlap_list_find_nearest_rail,
        "OverlapList_FindNearestRail",
    ),
    (
        0x0013_f488,
        rider_ground_state_collide_walls,
        "RiderGroundState_CollideWalls",
    ),
    (
        0x0035_ac20,
        multi_spline_modifier_update_nodes,
        "MultiSplineModifier_UpdateNodes",
    ),
    (
        0x0036_d428,
        emitter_update_descriptor,
        "Emitter_UpdateDescriptor",
    ),
    (
        0x0013_f178,
        rider_ground_state_post_update,
        "RiderGroundState_PostUpdate",
    ),
    (
        0x003c_a460,
        snd_dec_eaxa_decode_stream,
        "SND_DecEaxa_DecodeStream",
    ),
    (
        0x0038_9308,
        irradiance_add_directional_scaled,
        "Irradiance_AddDirectionalScaled",
    ),
    (
        0x003c_9520,
        snd_dec_utk_decode_loop,
        "SND_DecUtk_DecodeLoop",
    ),
    (0x002b_af90, wp_data_find_cell_index, "WPData_FindCellIndex"),
    (
        0x0038_6bd0,
        ps2_graphics_man_blend_bone_matrices,
        "PS2GraphicsMan_v111",
    ),
];

// ---------------------------------------------------------------------------
// OverlapList_FindNearestRail (0x334680, worldsphtree.cpp)

/// `RailModifier_FindClosestPoint` (type-2 objects).
const RAIL_MODIFIER_FIND_CLOSEST_POINT: Callee = Callee {
    addr: 0x0035_c698,
    f: ee::s11::RailModifier_FindClosestPoint,
};
/// `FlexRail_FindClosestPoint` (type-3 objects).
const FLEX_RAIL_FIND_CLOSEST_POINT: Callee = Callee {
    addr: 0x0034_8290,
    f: ee::s11::FlexRail_FindClosestPoint,
};
/// `Rail_FindNearestPoint` (type-1 objects).
const RAIL_FIND_NEAREST_POINT: Callee = Callee {
    addr: 0x0033_5128,
    f: ee::s11::Rail_FindNearestPoint,
};

/// Overlap list: number of objects (`u32`) and the object pointer array.
const OVERLAP_COUNT: u32 = 0x210;
const OVERLAP_ITEMS: u32 = 0x214;
/// World object: its type (1 rail, 2 rail modifier, 3 flex rail), the
/// modifier/flex data it carries, and its instance (flags at +0x1c).
const OBJ_TYPE: u32 = 0x8;
const OBJ_DATA: u32 = 0xc;
const OBJ_INSTANCE: u32 = 0x68;
const INSTANCE_FLAGS: u32 = 0x1c;
/// The query frame: box min (0x00), box max (0x10), extent (0x20),
/// contact count (0x30), nearest distance (0x34).
const Q_BOX_MIN: u32 = 0x00;
const Q_BOX_MAX: u32 = 0x10;
const Q_EXT: u32 = 0x20;
const Q_COUNT: u32 = 0x30;
const Q_DIST: u32 = 0x34;

/// `int OverlapList_FindNearestRail(sOverlapList* self, const sVec4* pos,
/// void* out, int mask, float r)` (0x334680): builds the query box
/// `pos -/+ (r, r, r, 0)` on the stack, then asks every rail-like object of
/// the overlap list for its closest point to `pos` (rail modifiers and flex
/// rails always; plain rails only when their instance flags pass `mask`).
/// The callees count contacts and keep the nearest distance in the frame;
/// returns the contact count.
pub fn overlap_list_find_nearest_rail(c: &mut Cpu) {
    use crate::guest::Addr;
    use ssx3_ee_rt::vu0::Src;
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0xe0);
        c.mtc1(0, 0); // f0 = 0.0
        for (reg, off) in [
            (S0, 0xd0),
            (S2, 0xb0),
            (S3, 0xa0),
            (S4, 0x90),
            (S5, 0x80),
            (S1, 0xc0),
            (S6, 0x70),
            (S7, 0x60),
            (FP, 0x50),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x40);
        c.g[S0][0] = c.g[A1][0]; // pos
        c.g[S2][0] = 0; // i
        c.g[S3][0] = c.g[A3][0]; // mask
        c.g[S4][0] = c.g[A2][0]; // out
        c.g[S5][0] = c.g[A0][0]; // self

        // box = { pos - ext, pos + ext }, ext = (r, r, r, 0); count = 0,
        // dist = 0.
        c.lq(2, r32(c, S0));
        c.sq(2, frame.at(c, Q_BOX_MIN));
        c.sq(2, frame.at(c, Q_BOX_MAX));
        for off in [0, 4, 8] {
            let a = frame.at(c, Q_EXT + off);
            c.swc1(12, a);
        }
        let a = frame.at(c, Q_EXT + 0xc);
        c.swc1(0, a);
        Addr(frame.at(c, Q_COUNT)).sw(c, 0);
        let a = frame.at(c, Q_DIST);
        c.swc1(0, a);
        c.lqc2(3, frame.at(c, Q_BOX_MIN));
        c.lqc2(4, frame.at(c, Q_EXT));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, Q_BOX_MIN));
        c.lqc2(3, frame.at(c, Q_BOX_MAX));
        c.lqc2(4, frame.at(c, Q_EXT));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, Q_BOX_MAX));

        let list = Addr(r32(c, S5));
        c.g[V0][0] = list.off(OVERLAP_COUNT).lw(c);
        c.g[FP][0] = 2; // rail modifier
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[S7][0] = 3; // flex rail
            c.g[S6][0] = 1; // plain rail
            c.g[S1][0] = sx(list.0.wrapping_add(OVERLAP_ITEMS));
            loop {
                c.cycles += 4;
                let obj = Addr(r32(c, S1)).lw(c);
                c.g[A0][0] = obj;
                let obj = Addr(obj as u32);
                c.g[V0][0] = obj.off(OBJ_TYPE).lw(c);
                c.g[A1][0] = c.g[SP][0]; // &box
                if c.g[V0][0] == c.g[FP][0] {
                    // RailModifier_FindClosestPoint(data, &box, &box, pos,
                    // mask, &count, &dist, out)
                    c.cycles += 10;
                    c.g[A0][0] = obj.off(OBJ_DATA).lw(c);
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = c.g[S3][0];
                    c.g[T1][0] = sx(frame.at(c, Q_COUNT));
                    c.g[T2][0] = sx(frame.at(c, Q_DIST));
                    c.g[T3][0] = c.g[S4][0];
                    RAIL_MODIFIER_FIND_CLOSEST_POINT.call(c, 0x0033_4750)?;
                } else {
                    c.cycles += 2;
                    c.g[A2][0] = c.g[SP][0];
                    if c.g[V0][0] == c.g[S7][0] {
                        // FlexRail_FindClosestPoint(data, &box, &box, pos,
                        // mask, &count, &dist, out)
                        c.cycles += 9;
                        c.g[A0][0] = obj.off(OBJ_DATA).lw(c);
                        c.g[A3][0] = c.g[S0][0];
                        c.g[T0][0] = c.g[S3][0];
                        c.g[T1][0] = sx(frame.at(c, Q_COUNT));
                        c.g[T2][0] = sx(frame.at(c, Q_DIST));
                        c.g[T3][0] = c.g[S4][0];
                        FLEX_RAIL_FIND_CLOSEST_POINT.call(c, 0x0033_477c)?;
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] == c.g[S6][0] {
                            // A plain rail, only when its instance flags pass.
                            c.cycles += 5;
                            c.g[V1][0] = obj.off(OBJ_INSTANCE).lw(c);
                            let flags = Addr(r32(c, V1)).off(INSTANCE_FLAGS).lw(c);
                            c.g[V0][0] = flags & c.g[S3][0];
                            c.g[A1][0] = c.g[SP][0];
                            if c.g[V0][0] != 0 {
                                // Rail_FindNearestPoint(obj, &box, pos,
                                // &count, &dist, out)
                                c.cycles += 5;
                                c.g[A2][0] = c.g[S0][0];
                                c.g[A3][0] = sx(frame.at(c, Q_COUNT));
                                c.g[T0][0] = sx(frame.at(c, Q_DIST));
                                c.g[T1][0] = c.g[S4][0];
                                RAIL_FIND_NEAREST_POINT.call(c, 0x0033_47b4)?;
                            }
                            c.cycles += 1;
                        }
                    }
                }
                let list = Addr(r32(c, S5));
                c.g[V0][0] = list.off(OVERLAP_COUNT).lw(c);

                c.cycles += 4;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0033_4720);
                }
            }
        }

        c.cycles += 13;
        c.g[V0][0] = Addr(frame.at(c, Q_COUNT)).lw(c);
        for (reg, off) in [
            (S0, 0xd0),
            (S1, 0xc0),
            (S2, 0xb0),
            (S3, 0xa0),
            (S4, 0x90),
            (S5, 0x80),
            (S6, 0x70),
            (S7, 0x60),
            (FP, 0x50),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderGroundState_CollideWalls (0x13f488, handplantcontrol.cpp)

/// Called functions (all through the function table, as the original).
const RIDER_GET_SUB_STATE: u32 = 0x0011_fee8;
const COLL_HULL_CTOR: u32 = 0x0032_f650;
const COLL_HULL_DTOR: u32 = 0x0032_f708;
const OVERLAP_LIST_FIND_CLOSEST_CONTACT: u32 = 0x0033_42d0;
const RIDER_TRANSLATE: u32 = 0x0010_6538;
const RIDER_DEFLECT_ALONG_WALL: u32 = 0x0010_65b0;
const RIDER_BUILD_WORLD_MATRIX: u32 = 0x0011_e098;
const RIDER_HANDLE_IMPACT: u32 = 0x0010_5d98;
const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;
const SSX_AUDIO_PLAY_GROUND_WALL_HIT_SOUND: u32 = 0x0029_89a8;

/// The ground state's rider.
const GS_RIDER: u32 = 0x18;
/// Rider fields.
const R_POS_Y: u32 = 0x118;
const R_VEL: u32 = 0x1e0;
const R_FORWARD: u32 = 0x370;
const R_GROUND_Y: u32 = 0x498;
const R_SPEED: u32 = 0x4c8;
/// The rider's controller object (vtable slot 0x40: "is in contact").
const R_CONTROLLER: u32 = 0x6c0;
const R_OVERLAP_LIST: u32 = 0x860;
const R_OVERLAP_HINT: u32 = 0x868;
const R_HULL: u32 = 0xaa0;
/// `.lit4` constants (gp-relative, subtracted): -0.9999 (head-on
/// threshold), 1.1 (push-out factor), 0.05 (bounce factor), 27.777779 (the
/// minimal bounce speed, 100 km/h in m/s).
const GP_HEAD_ON: u32 = 0x6fa0;
const GP_PUSH_OUT: u32 = 0x6f9c;
const GP_BOUNCE_SCALE: u32 = 0x6f98;
const GP_MIN_BOUNCE: u32 = 0x6f94;
/// Frame layout: the contact (0x00: pos, 0x10: normal, 0x20: the hit
/// surface's velocity,
/// 0x4c: material, 0x50: object), the hull volume (0x80, +0x10 its
/// direction), the impact info (0x3f0) and VU spill slots.
const F_HIT: u32 = 0x00;
const F_HIT_NORMAL: u32 = 0x10;
const F_HIT_VELOCITY: u32 = 0x20;
const F_HIT_MATERIAL: u32 = 0x4c;
const F_HIT_OBJECT: u32 = 0x50;
const F_HULL: u32 = 0x80;
const F_SPILL_A: u32 = 0x3a0;
const F_SPILL_B: u32 = 0x3b0;
const F_SPILL_C: u32 = 0x3c0;
const F_SPILL_D: u32 = 0x3d0;
const F_SPILL_E: u32 = 0x3e0;
const F_IMPACT: u32 = 0x3f0;

/// `vf4.x = vf3 . vf<b>` (xyz, through the `vf6.x = 1 + vf0.w` trick the
/// compiler's inline dot product uses).
#[inline]
fn vu_dot3(c: &mut Cpu, b: u8) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `vf5 = vf3 * rsqrt(vf3 . vf3)` (the inline normalise).
#[inline]
fn vu_normalize3(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::Src;
    vu_dot3(c, 3);
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
}

/// `vf5 = q(at) * x` with `x` passed through gpr `gpr` (`mfc1`/`qmtc2`).
#[inline]
fn vu_scale(c: &mut Cpu, gpr: usize, at: u32) {
    use ssx3_ee_rt::vu0::{Src, X};
    c.lqc2(4, at);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
}

/// `vf5 = q(a) - q(b)`.
#[inline]
fn vu_sub(c: &mut Cpu, a: u32, b: u32) {
    use ssx3_ee_rt::vu0::Src;
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
}

/// `sqc2 vf5, tmp; lq v0, tmp; sq v0, dst...` (the compiler's VU spill).
#[inline]
fn vu_spill(c: &mut Cpu, frame: Frame, tmp: u32, dst: &[u32]) {
    let t = frame.at(c, tmp);
    c.sqc2(5, t);
    c.lq(2, t);
    for &d in dst {
        let a = frame.at(c, d);
        c.sq(2, a);
    }
}

/// `void RiderGroundState_CollideWalls(RiderGroundState* self)` (0x13f488):
/// the ground state's wall collision. Skipped in sub-state 9, and for a slow
/// rider (< 50) close to the ground that is not in contact. Otherwise the
/// rider's hull is swept along its forward direction against its overlap
/// list; on a hit (depth >= 0) the hit object is notified (vtable 0x150),
/// and unless the rider runs head-on into the wall (forward . normal <
/// -0.9999) the rider is pushed out by 1.1 * depth along the normal
/// (flattened against the forward direction when it faces the wall). If the
/// rider then still moves into the wall it is bounced off (by the into-wall
/// speed plus 5% of it, at least 27.78), deflected along the wall, its
/// matrix rebuilt, the impact handled and the wall-hit sound played. The
/// hull is destroyed on every path after its construction.
pub fn rider_ground_state_collide_walls(c: &mut Cpu) {
    use crate::abi::{c_lt_s, set_fpr};
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use ssx3_ee_rt::vu0::Src;

    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x490);
        frame.save(c, S2, 0x450);
        frame.save(c, S0, 0x470);
        frame.save(c, S1, 0x460);
        frame.save(c, S3, 0x440);
        frame.save_ra(c, 0x430);
        frame.save_fpr(c, 21, 0x488);
        frame.save_fpr(c, 20, 0x480);
        c.g[S2][0] = c.g[A0][0]; // self
        let this = Addr(r32(c, S2));

        // Rider_GetSubState(rider) == 9: nothing to do.
        c.g[A0][0] = this.off(GS_RIDER).lw(c);
        call_ptr(c, RIDER_GET_SUB_STATE, 0x0013_f4b4)?;
        c.g[V1][0] = 9;
        let skip = if c.g[V0][0] == c.g[V1][0] {
            true
        } else {
            // In contact? Else only when fast or off the ground.
            c.cycles += 9;
            let rider = this.off(GS_RIDER).lw(c) as u32;
            controller_in_contact(c, rider, true, 0x0013_f4dc)?;
            if c.g[V0][0] != 0 {
                false
            } else {
                c.cycles += 8;
                let rider = this.off(GS_RIDER).lw(c);
                c.g[V0][0] = rider;
                let rider = Addr(rider as u32);
                c.g[AT][0] = 0x4248_0000;
                let fifty = FpuF32(0x4248_0000);
                set_fpr(c, 2, fifty);
                let speed = rider.off(R_SPEED).read_fpu(c);
                set_fpr(c, 0, speed);
                if !c_lt_s(c, speed, fifty) {
                    false
                } else {
                    c.cycles += 8;
                    let y = rider.off(R_POS_Y).read_fpu(c);
                    let ground = rider.off(R_GROUND_Y).read_fpu(c);
                    set_fpr(c, 1, y);
                    let height = (ground - y).abs();
                    set_fpr(c, 0, height);
                    if c_lt_s(c, height, fifty) {
                        true
                    } else {
                        c.cycles += 1;
                        false
                    }
                }
            }
        };
        if !skip {
            c.g[S0][0] = this.off(GS_RIDER).lw(c);
            collide_walls_hit(c, frame)?;
        }
        frame.restore(c, S0, 0x470);

        c.cycles += 8;
        frame.restore(c, S1, 0x460);
        frame.restore(c, S2, 0x450);
        frame.restore(c, S3, 0x440);
        frame.restore_ra(c, 0x430);
        frame.restore_fpr(c, 21, 0x488);
        frame.restore_fpr(c, 20, 0x480);
        frame.close(c);
        Ok(())
    });

    /// `rider->controller->vt[0x40]()` ("in contact"), returning to `ret`.
    /// The two call sites leave different registers: the first keeps the
    /// `this` delta in a1, the second the controller address.
    fn controller_in_contact(c: &mut Cpu, rider: u32, first: bool, ret: u32) -> crate::call::Flow {
        let ctl = rider.wrapping_add(R_CONTROLLER);
        let vt = Addr(ctl).lw(c);
        c.g[V1][0] = vt;
        let delta = Addr(vt as u32).off(0x40).lh(c);
        c.g[A1][0] = if first { delta } else { sx(ctl) };
        let method = Addr(vt as u32).off(0x44).lw(c);
        c.g[V0][0] = method;
        c.g[A0][0] = sx(ctl.wrapping_add(delta as u32));
        call_ptr(c, method as u32, ret)
    }

    /// The hull sweep and the response (rider in s0, self in s2).
    fn collide_walls_hit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
        c.cycles += 28;
        c.g[S1][0] = sx(frame.at(c, F_HULL));
        c.g[S3][0] = c.g[S1][0];
        let rider = r32(c, S0);
        // CollHull_ctor(&hull, rider->hull, !in_contact)
        controller_in_contact(c, rider, false, 0x0013_f548)?;
        c.g[A1][0] = Addr(rider).off(R_HULL).lw(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0] ^ 1;
        call_ptr(c, COLL_HULL_CTOR, 0x0013_f558)?;
        // hull.f8 = 1; hull.dir = rider->forward;
        // d = OverlapList_FindClosestContact(rider->list, &hull, &hit, hint)
        let rider = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        c.g[V1][0] = rider;
        let rider = Addr(rider as u32);
        c.g[A0][0] = 1;
        Addr(frame.at(c, F_HULL + 8)).sw(c, 1);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        c.lq(2, rider.off(R_FORWARD).0);
        let a = frame.at(c, F_HULL + 0x10);
        c.sq(2, a);
        c.g[A3][0] = rider.off(R_OVERLAP_HINT).lw(c);
        c.g[A0][0] = rider.off(R_OVERLAP_LIST).lw(c);
        call_ptr(c, OVERLAP_LIST_FIND_CLOSEST_CONTACT, 0x0013_f580)?;
        let depth = FpuF32(c.f[0]);
        set_fpr(c, 20, depth);
        set_fpr(c, 0, FpuF32::ZERO);
        let missed = c_lt_s(c, depth, FpuF32::ZERO);
        c.g[A0][0] = c.g[S3][0];
        if missed {
            return destroy_hull(c);
        }

        // Tell the hit object (vtable 0x150 of its owner).
        c.cycles += 3;
        let obj = Addr(frame.at(c, F_HIT_OBJECT)).lw(c);
        c.g[A0][0] = obj;
        if obj != 0 {
            c.cycles += 3;
            let owner = Addr(obj as u32).off(0xc).lw(c);
            c.g[A2][0] = owner;
            c.g[A1][0] = c.g[SP][0];
            if owner != 0 {
                c.cycles += 5;
                let vt = Addr(owner as u32).off(0xc).lw(c);
                c.g[V0][0] = vt;
                let delta = Addr(vt as u32).off(0x150).lh(c);
                let method = Addr(vt as u32).off(0x154).lw(c);
                c.g[V1][0] = method;
                c.g[A0][0] = sx((owner as u32).wrapping_add(delta as u32));
                call_ptr(c, method as u32, 0x0013_f5c4)?;
            }
            c.cycles += 1;
        }
        let rider = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        c.g[V1][0] = rider;
        let rider = rider as u32;

        // dot = forward . normal; head-on: give up.
        c.cycles += 14;
        c.lwc1(0, r32(c, GP).wrapping_sub(GP_HEAD_ON));
        c.lqc2(3, rider.wrapping_add(R_FORWARD));
        c.lqc2(5, frame.at(c, F_HIT_NORMAL));
        vu_dot3(c, 5);
        c.qmfc2(2, 4);
        c.mtc1(2, 1);
        c.c_lt_s(1, 0);
        let head_on = c.fcc();
        c.g[A0][0] = c.g[S3][0];
        if head_on {
            return destroy_hull(c);
        }
        c.cycles += 4;
        c.mtc1(0, 21); // f21 = 0
        let facing = c_lt_s(c, FpuF32(c.f[1]), FpuF32::ZERO);
        if facing {
            // normal = normalize(normal - forward * dot)
            c.cycles += 26;
            c.mfc1(2, 1);
            vu_scale(c, 2, rider.wrapping_add(R_FORWARD));
            vu_spill(c, frame, F_SPILL_D, &[F_SPILL_C]);
            vu_sub(c, frame.at(c, F_HIT_NORMAL), frame.at(c, F_SPILL_C));
            vu_spill(c, frame, F_SPILL_D, &[F_SPILL_B]);
            c.lqc2(3, frame.at(c, F_SPILL_B));
            vu_normalize3(c);
            vu_spill(c, frame, F_SPILL_D, &[F_HIT_NORMAL, F_SPILL_A]);
        }

        // Rider_Translate(rider, normal * 1.1 * depth)
        c.cycles += 47;
        c.lwc1(0, r32(c, GP).wrapping_sub(GP_PUSH_OUT));
        c.g[A1][0] = sx(frame.at(c, F_SPILL_A));
        c.mfc1(3, 0);
        vu_scale(c, 3, frame.at(c, F_HIT_NORMAL));
        vu_spill(c, frame, F_SPILL_C, &[F_SPILL_B]);
        c.mfc1(3, 20);
        vu_scale(c, 3, frame.at(c, F_SPILL_B));
        let t = frame.at(c, F_SPILL_C);
        c.sqc2(5, t);
        c.lq(2, t);
        c.g[A0][0] = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        let a = frame.at(c, F_SPILL_A);
        c.sq(2, a);
        call_ptr(c, RIDER_TRANSLATE, 0x0013_f6c0)?;

        // rel = rider->vel - hit.velocity; into = -normal . rel
        let rider = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        c.g[V0][0] = rider;
        let rider = rider as u32;
        vu_sub(c, rider.wrapping_add(R_VEL), frame.at(c, F_HIT_VELOCITY));
        let t = frame.at(c, F_SPILL_C);
        c.sqc2(5, t);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0); // f0 = -1.0
        c.lq(2, t);
        let a = frame.at(c, F_SPILL_B);
        c.sq(2, a);
        c.mfc1(3, 0);
        vu_scale(c, 3, frame.at(c, F_HIT_NORMAL));
        vu_spill(c, frame, F_SPILL_D, &[F_SPILL_C]);
        c.lqc2(3, frame.at(c, F_SPILL_C));
        c.lqc2(5, frame.at(c, F_SPILL_B));
        vu_dot3(c, 5);
        c.qmfc2(3, 4);
        c.mtc1(3, 20);
        let into = FpuF32(c.f[20]);
        let receding = c_lt_s(c, into, FpuF32(c.f[21]));
        c.g[A0][0] = c.g[S3][0];
        if receding {
            return destroy_hull(c);
        }

        // dir = normalize(rel); push = max(into * 0.05, 27.78)
        c.cycles += 18;
        c.lwc1(0, r32(c, GP).wrapping_sub(GP_BOUNCE_SCALE));
        c.lqc2(3, frame.at(c, F_SPILL_B));
        vu_normalize3(c);
        let t = frame.at(c, F_SPILL_D);
        c.sqc2(5, t);
        let min_push = Addr(r32(c, GP).wrapping_sub(GP_MIN_BOUNCE)).read_fpu(c);
        set_fpr(c, 1, min_push);
        let mut push = into * FpuF32(c.f[0]);
        set_fpr(c, 0, push);
        c.lq(2, t);
        let small = c_lt_s(c, push, min_push);
        let a = frame.at(c, F_SPILL_C);
        c.sq(2, a);
        if small {
            c.cycles += 1;
            push = min_push;
            set_fpr(c, 0, push);
        }

        // rider->vel += normal * (into + push)
        c.cycles += 38;
        set_fpr(c, 0, into + push);
        c.g[A1][0] = c.g[SP][0];
        c.mfc1(3, 0);
        vu_scale(c, 3, frame.at(c, F_HIT_NORMAL));
        let t = frame.at(c, F_SPILL_E);
        c.sqc2(5, t);
        c.lq(2, t);
        let rider = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        c.g[V1][0] = rider;
        let rider = rider as u32;
        let a = frame.at(c, F_SPILL_D);
        c.sq(2, a);
        c.lqc2(3, rider.wrapping_add(R_VEL));
        c.lqc2(4, frame.at(c, F_SPILL_D));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, rider.wrapping_add(R_VEL));
        // Rider_DeflectAlongWall(rider, &hit); Rider_BuildWorldMatrix(rider)
        c.g[A0][0] = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        call_ptr(c, RIDER_DEFLECT_ALONG_WALL, 0x0013_f7d0)?;
        c.g[A0][0] = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        call_ptr(c, RIDER_BUILD_WORLD_MATRIX, 0x0013_f7d8)?;
        // Rider_HandleImpact(rider, &{ hit.pos, dir, hit.normal, into },
        // hit.material)
        c.lq(7, frame.at(c, F_HIT));
        c.g[A1][0] = sx(frame.at(c, F_IMPACT));
        c.lq(2, frame.at(c, F_SPILL_C));
        c.lq(3, frame.at(c, F_HIT_NORMAL));
        c.g[A0][0] = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        c.g[A2][0] = Addr(frame.at(c, F_HIT_MATERIAL)).lw(c);
        let a = frame.at(c, F_IMPACT);
        c.sq(7, a);
        c.sq(2, a + 0x10);
        c.sq(3, a + 0x20);
        frame_swc1(c, frame, 20, F_IMPACT + 0x30);
        call_ptr(c, RIDER_HANDLE_IMPACT, 0x0013_f804)?;
        // SSXAudioSystem_PlayGroundWallHitSound(GetInstance(), rider)
        call_ptr(c, SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0013_f80c)?;
        c.g[A1][0] = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, SSX_AUDIO_PLAY_GROUND_WALL_HIT_SOUND, 0x0013_f818)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 2;
        call_ptr(c, COLL_HULL_DTOR, 0x0013_f824)
    }

    /// `swc1 f, off(sp)`.
    fn frame_swc1(c: &mut Cpu, frame: Frame, f: usize, off: u32) {
        let a = frame.at(c, off);
        c.swc1(f, a);
    }

    /// `CollHull_dtor(&hull, 2)` (a0 already holds the hull).
    fn destroy_hull(c: &mut Cpu) -> crate::call::Flow {
        c.cycles += 4;
        c.g[A1][0] = 2;
        call_ptr(c, COLL_HULL_DTOR, 0x0013_f73c)
    }
}

// ---------------------------------------------------------------------------
// MultiSplineModifier_UpdateNodes (0x35ac20, splinemodifier.cpp)

const MATH_SIN_COS: u32 = 0x0031_be50;
const MATH_ATAN: u32 = 0x0031_c228;
const SPLINE_EVAL_PATH_AT_DISTANCE: Callee = Callee {
    addr: 0x0034_5248,
    f: ee::s11::SplineModifier_EvalPathAtDistance,
};

/// MultiSplineModifier fields.
const MSM_COUNT: u32 = 0x04;
const MSM_MODE: u32 = 0x08;
const MSM_SPIN: u32 = 0x0c;
const MSM_START: u32 = 0x10;
const MSM_DIRTY: u32 = 0x30;
const MSM_STATES: u32 = 0x3c;
const MSM_NODES: u32 = 0x44;
const MSM_PATH: u32 = 0x48;
const MSM_LENGTH: u32 = 0x54;
/// Per-node state (0x60 bytes): position (0x00), direction (0x10),
/// third output (0x20), squared direction x/z (0x50/0x54).
const NS_DIR: u32 = 0x10;
const NS_DIR_SQ: u32 = 0x50;
const NS_STRIDE: u32 = 0x60;
/// The world up axis (a global vec3).
const UP_AXIS: u32 = 0x004f_f160;
/// gp-relative `.lit4`: pi (atan quadrant fix), the +-pi/2 yaw and pitch
/// values for a zero denominator.
const GP_PI: u32 = 0x2a24;
const GP_YAW_NEG: u32 = 0x2a20;
const GP_YAW_POS: u32 = 0x2a1c;
const GP_PITCH_NEG: u32 = 0x2a18;
const GP_PITCH_POS: u32 = 0x2a14;
/// Frame: per-node distances (0x000), spin matrix (0x400), node matrix
/// scratch (0x450, 0x490), angles (0x440..), sin/cos outputs (0x4d0..).
const MSM_F_SPIN: u32 = 0x400;
const MSM_F_TMP_A: u32 = 0x450;
const MSM_F_TMP_B: u32 = 0x490;

/// `void MultiSplineModifier_UpdateNodes(MultiSplineModifier* self)`
/// (0x35ac20): spreads the `count` nodes evenly over the path (`start +
/// i * length / count`, wrapped into `[0, length)`), evaluates the path at
/// each distance, derives the node's yaw (modes 2/3) and pitch (modes 1/3)
/// from the path direction and writes the node object's matrix (rows from
/// the angles' sines/cosines, translation = path position); a non-zero spin
/// angle additionally rotates every node about the world up axis. Clears
/// the dirty flag.
pub fn multi_spline_modifier_update_nodes(c: &mut Cpu) {
    use crate::guest::Addr;
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x5d0);
        c.g[V0][0] = 1;
        for (reg, off) in [
            (S2, 0x560),
            (S0, 0x580),
            (S1, 0x570),
            (S3, 0x550),
            (S4, 0x540),
            (S5, 0x530),
            (S6, 0x520),
            (S7, 0x510),
            (FP, 0x500),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x4f0);
        for (f, off) in [
            (26, 0x5c0),
            (25, 0x5b8),
            (24, 0x5b0),
            (23, 0x5a8),
            (22, 0x5a0),
            (21, 0x598),
            (20, 0x590),
        ] {
            frame.save_fpr(c, f, off);
        }
        c.g[S2][0] = c.g[A0][0];
        let this = r32(c, S2);
        let at = |o: u32| this.wrapping_add(o);

        // dist[0] = start; step = length / count
        c.g[V1][0] = Addr(at(MSM_COUNT)).lw(c);
        c.lwc1(2, at(MSM_LENGTH));
        c.mtc1(V1, 1);
        c.cvt_s_w(1, 1);
        c.lwc1(0, at(MSM_START));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.div_s(2, 2, 1);
        let a = frame.at(c, 0);
        c.swc1(0, a);
        if c.g[V0][0] != 0 {
            // dist[i] = dist[i - 1] + step
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V1).wrapping_sub(1));
            c.g[V1][0] = sx(frame.at(c, 4));
            loop {
                c.cycles += 6;
                c.lwc1(0, r32(c, V1).wrapping_sub(4));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.add_s(0, 0, 2);
                c.swc1(0, r32(c, V1));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_aca0);
                }
            }
        }

        // Wrap every distance into [0, length).
        c.cycles += 3;
        c.g[V0][0] = Addr(at(MSM_COUNT)).lw(c);
        c.g[A0][0] = sx(at(MSM_COUNT));
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[V1][0] = c.g[V0][0];
            c.mtc1(0, 2); // f2 = 0
            c.g[V0][0] = c.g[SP][0];
            loop {
                c.cycles += 5;
                c.lwc1(1, r32(c, V0));
                c.c_lt_s(1, 2);
                let negative = c.fcc();
                c.lwc1(0, at(MSM_LENGTH));
                let store = if negative {
                    c.cycles += 2;
                    c.add_s(0, 1, 0);
                    true
                } else {
                    c.cycles += 4;
                    c.c_le_s(0, 1);
                    if c.fcc() {
                        c.cycles += 1;
                        c.sub_s(0, 1, 0);
                        true
                    } else {
                        false
                    }
                };
                if store {
                    c.cycles += 2;
                    c.swc1(0, r32(c, V0));
                }
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                c.cycles += 2;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
                if c.g[V1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_acd0);
                }
            }
        }

        // The spin matrix: a rotation by `spin` about the up axis.
        c.cycles += 6;
        c.lwc1(0, at(MSM_SPIN));
        c.mtc1(0, 20); // f20 = 0
        c.c_eq_s(0, 20);
        if !c.fcc() {
            c.cycles += 55;
            c.lwc1(12, r32(c, A0).wrapping_add(8));
            c.g[A1][0] = sx(frame.at(c, 0x4d4));
            c.g[A0][0] = sx(frame.at(c, 0x4d0));
            c.g[S0][0] = 0x50_0000;
            call_ptr(c, MATH_SIN_COS, 0x0035_ad3c)?;
            msm_spin_matrix(c, frame);
        }
        c.g[V0][0] = Addr(at(MSM_COUNT)).lw(c);

        // Every node.
        c.cycles += 3;
        c.g[S4][0] = 0;
        c.g[S1][0] = Addr(at(MSM_STATES)).lw(c);
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 6;
            c.lwc1(25, r32(c, GP).wrapping_sub(GP_PI));
            c.mov_s(26, 20); // roll = 0
            c.g[S7][0] = sx(frame.at(c, MSM_F_TMP_B));
            c.g[FP][0] = sx(frame.at(c, MSM_F_SPIN));
            c.g[S6][0] = sx(frame.at(c, MSM_F_TMP_A));
            c.g[S5][0] = c.g[SP][0];
            loop {
                // EvalPathAtDistance(&path, dist[i], &st->pos, &st->dir, &st->c)
                c.cycles += 19;
                let st = r32(c, S1);
                c.lwc1(12, r32(c, S5));
                c.g[A0][0] = sx(at(MSM_PATH));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = sx(st.wrapping_add(NS_DIR));
                c.mtc1(0, 23); // f23 = 0
                c.g[A3][0] = sx(st.wrapping_add(0x20));
                SPLINE_EVAL_PATH_AT_DISTANCE.call(c, 0x0035_ae44)?;
                c.g[S0][0] = sx(r32(c, S4) << 2);
                c.lwc1(21, st.wrapping_add(NS_DIR));
                c.lwc1(20, st.wrapping_add(NS_DIR + 4));
                c.mul_s(0, 21, 21);
                c.mul_s(1, 20, 20);
                c.swc1(0, st.wrapping_add(NS_DIR_SQ));
                c.swc1(1, st.wrapping_add(NS_DIR_SQ + 4));
                c.g[V0][0] = Addr(at(MSM_MODE)).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(2));
                c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                c.mov_s(24, 23); // pitch = 0
                if c.g[V0][0] != 0 {
                    // yaw = -atan2(dir.y, dir.x) (modes 2, 3)
                    msm_angle(c, 21, 20, 23, GP_YAW_NEG, GP_YAW_POS, 0x0035_aeb8, false)?;
                }

                c.cycles += 4;
                c.g[V1][0] = Addr(at(MSM_MODE)).lw(c);
                c.g[V0][0] = 1;
                let mut pitch = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 3;
                if !pitch {
                    c.cycles += 2;
                    pitch = c.g[V1][0] == c.g[V0][0];
                }
                if pitch {
                    // pitch = atan2(dir.z, |dir.xy|) (modes 1, 3)
                    c.cycles += 9;
                    c.lwc1(0, st.wrapping_add(NS_DIR_SQ));
                    c.lwc1(1, st.wrapping_add(NS_DIR_SQ + 4));
                    c.mtc1(0, 22); // f22 = 0
                    c.add_s(0, 0, 1);
                    c.lwc1(21, st.wrapping_add(NS_DIR + 8));
                    c.sqrt_s(20, 0);
                    msm_angle(c, 20, 21, 22, GP_PITCH_NEG, GP_PITCH_POS, 0x0035_af54, true)?;
                    c.cycles += 1;
                }
                c.g[V0][0] = Addr(at(MSM_NODES)).lw(c);

                // The node's matrix from (roll, pitch, yaw).
                c.cycles += 89;
                let a0 = frame.at(c, 0x4d8);
                c.g[A0][0] = sx(a0);
                let a = frame.at(c, 0x444);
                c.swc1(24, a);
                c.g[A1][0] = sx(frame.at(c, 0x4dc));
                let a = frame.at(c, 0x448);
                c.swc1(23, a);
                c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                let a = frame.at(c, 0x440);
                c.swc1(26, a);
                c.mov_s(12, 26);
                c.g[S0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, MATH_SIN_COS, 0x0035_afa4)?;
                c.lwc1(12, frame.at(c, 0x444));
                c.g[A0][0] = sx(frame.at(c, 0x4e0));
                c.g[A1][0] = sx(frame.at(c, 0x4e4));
                c.g[S3][0] = sx(r32(c, S0).wrapping_add(0x10));
                call_ptr(c, MATH_SIN_COS, 0x0035_afb8)?;
                c.lwc1(12, frame.at(c, 0x448));
                c.g[A0][0] = sx(frame.at(c, 0x4e8));
                c.g[A1][0] = sx(frame.at(c, 0x4ec));
                call_ptr(c, MATH_SIN_COS, 0x0035_afc8)?;
                msm_node_matrix(c, frame);
                c.lwc1(0, at(MSM_SPIN));
                c.c_eq_s(0, 26);
                if !c.fcc() {
                    // node = node * spin (through two scratch copies)
                    c.cycles += 45;
                    let (m, spin) = (r32(c, S3), r32(c, FP));
                    for i in 0..4 {
                        c.lqc2(4 + i, m + 0x10 * i as u32);
                    }
                    for i in 0..4 {
                        c.lqc2(8 + i, spin + 0x10 * i as u32);
                    }
                    for i in 0..4 {
                        let row = 8 + i;
                        c.vu0().mula(0b1111, 4, Src::Bc(row, X));
                        c.vu0().madda(0b1111, 5, Src::Bc(row, Y));
                        c.vu0().madda(0b1111, 6, Src::Bc(row, Z));
                        c.vu0().madd(0b1111, 12 + i, 7, Src::Bc(row, W));
                    }
                    let (b, a) = (r32(c, S7), r32(c, S6));
                    for i in 0..4 {
                        c.sqc2(12 + i, b + 0x10 * i as u32);
                    }
                    for (from, to) in [(b, a), (a, m)] {
                        for i in 0..4 {
                            c.lqc2(1 + i, from + 0x10 * i as u32);
                        }
                        for i in 0..4 {
                            c.sqc2(1 + i, to + 0x10 * i as u32);
                        }
                    }
                }
                c.g[V0][0] = Addr(at(MSM_COUNT)).lw(c);

                c.cycles += 5;
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(4));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(NS_STRIDE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_ae28);
                }
            }
        }

        c.cycles += 20;
        Addr(at(MSM_DIRTY)).sw(c, 0);
        for (reg, off) in [
            (S0, 0x580),
            (S1, 0x570),
            (S2, 0x560),
            (S3, 0x550),
            (S4, 0x540),
            (S5, 0x530),
            (S6, 0x520),
            (S7, 0x510),
            (FP, 0x500),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x4f0);
        for (f, off) in [
            (26, 0x5c0),
            (25, 0x5b8),
            (24, 0x5b0),
            (23, 0x5a8),
            (22, 0x5a0),
            (21, 0x598),
            (20, 0x590),
        ] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

/// One angle of a node: `atan(num / den)` fixed up by +-pi into the right
/// quadrant, or +-pi/2 (by the sign of `num`) when `den` is 0, 0 when both
/// are. `zero` holds 0.0. The yaw form (`pitch == false`, den = dir.x in
/// f21, num = dir.y in f20) leaves `f23 = f24 - angle`; the pitch form (den
/// = |dir.xy| in f20, num = dir.z in f21) leaves `f24 += angle`. The
/// pitch form's final block cycle is charged by the caller.
#[allow(clippy::too_many_arguments)]
fn msm_angle(
    c: &mut Cpu,
    den: usize,
    num: usize,
    zero: usize,
    gp_neg: u32,
    gp_pos: u32,
    ret_addr: u32,
    pitch: bool,
) -> crate::call::Flow {
    // The angle accumulates into f23 (yaw) or f24 (pitch).
    let apply = |c: &mut Cpu| {
        if pitch {
            c.add_s(24, 24, 0);
        } else {
            c.sub_s(23, 24, 0);
        }
    };
    if !pitch {
        c.cycles += 3;
    }
    c.c_eq_s(den, zero);
    if c.fcc() {
        // A zero denominator.
        c.cycles += 4;
        c.c_eq_s(num, zero);
        let both_zero = c.fcc();
        c.mov_s(0, num);
        if both_zero {
            c.cycles += 1;
            apply(c);
            return Ok(());
        }
        c.cycles += 4;
        c.c_le_s(zero, num);
        c.lwc1(0, r32(c, GP).wrapping_sub(gp_neg));
        if c.fcc() {
            c.cycles += 3;
            c.lwc1(0, r32(c, GP).wrapping_sub(gp_pos));
        }
        apply(c);
        return Ok(());
    }
    c.cycles += 7;
    c.div_s(12, num, den);
    call_ptr(c, MATH_ATAN, ret_addr)?;
    c.c_lt_s(den, zero);
    if !c.fcc() {
        apply(c);
        return Ok(());
    }
    c.cycles += 4;
    c.c_lt_s(zero, num);
    if c.fcc() {
        c.cycles += 1;
        c.add_s(0, 0, 25);
    } else {
        c.sub_s(0, 0, 25);
    }
    c.cycles += 1;
    apply(c);
    Ok(())
}

/// The spin rotation about the up axis from `sin`/`cos` (frame 0x4d0/0x4d4)
/// into the frame's 4x4 matrix at 0x400 (bottom row 0, 0, 0, 1). Verbatim
/// register order of the original (f0..f13).
fn msm_spin_matrix(c: &mut Cpu, frame: Frame) {
    let sp = frame.at(c, 0);
    let s = |o: u32| sp.wrapping_add(o);
    c.lwc1(11, s(0x4d4));
    c.g[V0][0] = sx(r32(c, S0).wrapping_sub(0xea0));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 5);
    c.lwc1(3, UP_AXIS + 8);
    c.sub_s(1, 5, 11);
    c.lwc1(0, UP_AXIS);
    c.lwc1(6, UP_AXIS + 4);
    c.lwc1(8, s(0x4d0));
    c.mul_s(2, 1, 3);
    c.swc1(5, s(0x43c));
    c.mul_s(4, 1, 0);
    c.swc1(20, s(0x430));
    c.mul_s(1, 1, 6);
    c.swc1(20, s(0x434));
    c.mul_s(13, 8, 3);
    c.swc1(20, s(0x438));
    c.mul_s(12, 8, 0);
    c.swc1(20, s(0x40c));
    c.mul_s(9, 2, 3);
    c.swc1(20, s(0x41c));
    c.mul_s(10, 4, 3);
    c.swc1(20, s(0x42c));
    c.mul_s(5, 2, 0);
    c.mul_s(7, 4, 0);
    c.mul_s(3, 1, 3);
    c.mul_s(0, 1, 0);
    c.mul_s(8, 8, 6);
    c.mul_s(2, 2, 6);
    c.mul_s(4, 4, 6);
    c.mul_s(1, 1, 6);
    c.add_s(5, 5, 8);
    c.sub_s(0, 0, 13);
    c.sub_s(2, 2, 12);
    c.add_s(9, 9, 11);
    c.swc1(5, s(0x408));
    c.add_s(7, 7, 11);
    c.swc1(0, s(0x404));
    c.add_s(4, 4, 13);
    c.swc1(2, s(0x418));
    c.sub_s(10, 10, 8);
    c.swc1(9, s(0x428));
    c.add_s(1, 1, 11);
    c.swc1(7, s(0x400));
    c.add_s(3, 3, 12);
    c.swc1(4, s(0x410));
    c.swc1(10, s(0x420));
    c.swc1(1, s(0x414));
    c.swc1(3, s(0x424));
}

/// The node object's matrix (s0 + 0x10): rotation rows from the sines and
/// cosines of roll (0x4d8/0x4dc), pitch (0x4e0/0x4e4) and yaw (0x4e8/0x4ec),
/// translation = the node state's position (s1), w = 1. Verbatim register
/// order of the original.
fn msm_node_matrix(c: &mut Cpu, frame: Frame) {
    let sp = frame.at(c, 0);
    let s = |o: u32| sp.wrapping_add(o);
    let n = r32(c, S0);
    let m = |o: u32| n.wrapping_add(o);
    let st = r32(c, S1);
    c.lwc1(0, s(0x4ec));
    c.lwc1(1, s(0x4e4));
    c.lwc1(2, s(0x4e0));
    c.mul_s(1, 1, 0);
    c.lwc1(3, s(0x4d8));
    c.lwc1(0, s(0x4e8));
    c.lwc1(4, s(0x4dc));
    c.mul_s(3, 3, 2);
    c.swc1(1, m(0x10));
    c.mul_s(4, 4, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 5);
    c.lwc1(0, s(0x4e4));
    c.lwc1(1, s(0x4e8));
    c.neg_s(0, 0);
    c.mul_s(0, 0, 1);
    c.swc1(0, m(0x14));
    c.lwc1(1, s(0x4e0));
    c.sw(m(0x1c), 0);
    c.swc1(1, m(0x18));
    c.lwc1(0, s(0x4ec));
    c.mul_s(0, 3, 0);
    c.add_s(0, 0, 4);
    c.swc1(0, m(0x20));
    c.lwc1(0, s(0x4ec));
    c.lwc1(2, s(0x4e8));
    c.lwc1(1, s(0x4dc));
    c.mul_s(3, 3, 2);
    c.mul_s(1, 1, 0);
    c.sub_s(1, 1, 3);
    c.swc1(1, m(0x24));
    c.lwc1(0, s(0x4d8));
    c.lwc1(1, s(0x4e4));
    c.neg_s(0, 0);
    c.sw(m(0x2c), 0);
    c.mul_s(0, 0, 1);
    c.swc1(0, m(0x28));
    c.lwc1(2, s(0x4e0));
    c.lwc1(1, s(0x4dc));
    c.lwc1(0, s(0x4d8));
    c.mul_s(1, 1, 2);
    c.lwc1(3, s(0x4e8));
    c.lwc1(2, s(0x4ec));
    c.mul_s(0, 0, 3);
    c.mul_s(1, 1, 2);
    c.sub_s(0, 0, 1);
    c.swc1(0, m(0x30));
    c.lwc1(2, s(0x4e0));
    c.lwc1(1, s(0x4ec));
    c.lwc1(0, s(0x4d8));
    c.mul_s(4, 4, 2);
    c.mul_s(0, 0, 1);
    c.add_s(4, 4, 0);
    c.swc1(4, m(0x34));
    c.lwc1(1, s(0x4e4));
    c.lwc1(0, s(0x4dc));
    c.mul_s(0, 0, 1);
    c.swc1(0, m(0x38));
    c.sw(m(0x3c), 0);
    c.lwc1(0, st);
    c.swc1(0, m(0x40));
    c.lwc1(1, st.wrapping_add(4));
    c.swc1(1, m(0x44));
    c.lwc1(2, st.wrapping_add(8));
    c.swc1(5, m(0x4c));
    c.swc1(2, m(0x48));
}

// ---------------------------------------------------------------------------
// Emitter_UpdateDescriptor (0x36d428, graphicsman.cpp)

const EMITTER_ADVANCE_DESC_TIME: Callee = Callee {
    addr: 0x0036_d3e8,
    f: ee::s12::Emitter_AdvanceDescTime,
};
const VU0_RAND_NEXT: Callee = Callee {
    addr: 0x0036_d400,
    f: ee::s12::Vu0_RandNext,
};

/// Emitter descriptor fields: the emission count (int), the time
/// accumulator, the emission interval, the per-emission random seeds, the
/// emission position and its per-emission step.
const ED_COUNT: u32 = 0x00;
const ED_TIME: u32 = 0x08;
const ED_INTERVAL: u32 = 0x0c;
const ED_POS: u32 = 0xb0;
const ED_POS_STEP: u32 = 0xe0;
/// The random seeds re-rolled per emission, with the return address of
/// each `Vu0_RandNext` call.
const ED_RANDOMS: [(u32, u32); 9] = [
    (0x38, 0x0036_d488),
    (0x3c, 0x0036_d490),
    (0x40, 0x0036_d498),
    (0x44, 0x0036_d4a0),
    (0x48, 0x0036_d4a8),
    (0x50, 0x0036_d4b0),
    (0x54, 0x0036_d4b8),
    (0x58, 0x0036_d4c0),
    (0x5c, 0x0036_d4c8),
];

/// `void Emitter_UpdateDescriptor(EmitterDesc* d, float dt)` (0x36d428):
/// advances the descriptor's clock (`Emitter_AdvanceDescTime`), then while
/// `count * interval < time` steps it back by one interval, moves the
/// emission position by its step (VU0) and re-rolls the nine random seeds
/// (`Vu0_RandNext`).
pub fn emitter_update_descriptor(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::Src;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        EMITTER_ADVANCE_DESC_TIME.call(c, 0x0036_d43c)?;
        let d = r32(c, S0);
        let due = |c: &mut Cpu, f: usize| {
            c.lwc1(f, d.wrapping_add(ED_COUNT));
            c.cvt_s_w(f, f);
            c.lwc1(3, d.wrapping_add(ED_INTERVAL));
            c.lwc1(2, d.wrapping_add(ED_TIME));
            c.mul_s(f, f, 3);
            c.c_lt_s(f, 2);
            c.fcc()
        };
        if due(c, 0) {
            c.cycles += 2;
            c.sub_s(0, 2, 3); // time - interval
            loop {
                c.cycles += 33;
                c.g[A0][0] = sx(d.wrapping_add(ED_RANDOMS[0].0));
                c.swc1(0, d.wrapping_add(ED_TIME));
                c.lqc2(3, d.wrapping_add(ED_POS));
                c.lqc2(4, d.wrapping_add(ED_POS_STEP));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, d.wrapping_add(ED_POS));
                for (off, ret_addr) in ED_RANDOMS {
                    c.g[A0][0] = sx(d.wrapping_add(off));
                    VU0_RAND_NEXT.call(c, ret_addr)?;
                }
                if !due(c, 1) {
                    c.cycles += 1;
                    break;
                }
                c.sub_s(0, 2, 3);
                if event_due(c) {
                    deliver_event_here(c, 0x0036_d468);
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderGroundState_PostUpdate (0x13f178, handplantcontrol.cpp)

const RIDER_LAUNCH_INTO_AIR: u32 = 0x0011_4298;
const RIDER_START_WIPEOUT: u32 = 0x0010_eb30;
const RIDER_CHECK_RESET_PATH: u32 = 0x0011_6120;
const RIDER_UPDATE_BOUNDS: u32 = 0x0011_e150;
const RIDER_HANDLE_WALL_COLLISION: u32 = 0x0010_5398;
const RIDER_CHECK_RIDER_COLLISIONS: u32 = 0x0010_7888;
const RIDER_GET_MAIN_STATE: u32 = 0x0011_fe98;
const RIDER_SET_MAIN_STATE: u32 = 0x0011_fe78;
const SSX_AUDIO_PLAY_TAKEOFF_SOUND: u32 = 0x0029_4170;
const RIDER_GROUND_STATE_COLLIDE_WALLS: Callee = Callee {
    addr: 0x0013_f488,
    f: ee::s01::RiderGroundState_CollideWalls,
};

/// The ground state: "leaving the ground" flag.
const GS_LEAVING: u32 = 0x0c;
/// Rider fields.
const R_POS: u32 = 0x110;
const R_SURFACE: u32 = 0x438;
const R_CONTACT_FLAGS: u32 = 0x2d4;
const R_MAX_SPEED: u32 = 0x2e4;
const R_SMOOTH_UP: u32 = 0x390;
const R_SPEED_FRACTION: u32 = 0x75c;
const R_ABS_VEL_Z: u32 = 0x754;
const R_ABS_SPIN: u32 = 0x750;
/// The surface type that wipes the rider out, and the surface table's
/// entry size and "resets the path" word.
const SURFACE_WIPEOUT: u64 = 0x12;
const SURFACE_ENTRY: u64 = 0xb0;
const SURFACE_RESETS: u32 = 0x44;
/// gp-relative globals: the world, the hull's ground sphere mask, the audio
/// system; `.lit4` 3333.3335 (the speed of a full speed fraction).
const GP_WORLD: u32 = 0x848;
const GP_HULL_GROUND_MASK: u32 = 0x1f94;
const GP_AUDIO: u32 = 0x410;
const GP_FULL_SPEED: u32 = 0x6fac;

/// `void RiderGroundState_PostUpdate(RiderGroundState* self)` (0x13f178):
/// after the ground physics step. A rider leaving the ground is launched
/// (`Rider_LaunchIntoAir(-1)`); otherwise surface type 0x12 starts a
/// wipeout (position, normalised velocity, forward), and a surface that
/// resets the path (or contact flag 2) checks the reset path. Then the
/// bounds are updated and the walls collided with the hull's ground spheres
/// masked in; wall and rider collisions are handled; a rider leaving the
/// ground while in main state 0 gets main state 1 and the take-off sound. Finally the smoothed up vector follows forward (normalise(up +
/// forward / 2)), the velocity is clamped to the maximal speed, and the
/// speed fraction (|v| / 3333.33, 0..1) and the absolute z velocity and spin
/// are stored.
pub fn rider_ground_state_post_update(c: &mut Cpu) {
    use crate::guest::Addr;
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save_ra(c, 0x60);
        c.g[S0][0] = c.g[A0][0];
        let this = Addr(r32(c, S0));
        let rider = |c: &mut Cpu| this.off(GS_RIDER).lw(c);

        c.g[V0][0] = this.off(GS_LEAVING).lw(c);
        if c.g[V0][0] != 0 {
            // Rider_LaunchIntoAir(rider, -1.0)
            c.cycles += 6;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 12);
            c.g[A0][0] = rider(c);
            call_ptr(c, RIDER_LAUNCH_INTO_AIR, 0x0013_f1a4)?;
        } else {
            c.g[A3][0] = rider(c);
            let r = r32(c, A3);
            // entry = &world->surfaces->table[rider->surface]
            c.cycles += 9;
            c.g[V1][0] = SURFACE_ENTRY;
            let world = Addr(r32(c, GP).wrapping_sub(GP_WORLD)).lw(c);
            c.g[V0][0] = world;
            c.g[A2][0] = SURFACE_WIPEOUT;
            c.g[A1][0] = Addr(r).off(R_SURFACE).lw(c);
            c.g[A0][0] = Addr(world as u32).off(0x84).lw(c);
            c.mult(0, V1, A1, V1);
            c.g[V0][0] = Addr(r32(c, A0)).off(0x44).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            if c.g[A1][0] == c.g[A2][0] {
                // Rider_StartWipeout(rider, 0x168, 0, surface, &info)
                c.cycles += 27;
                c.lq(2, r.wrapping_add(R_POS));
                c.g[A1][0] = 0x168;
                c.g[A2][0] = 0;
                c.g[T0][0] = c.g[SP][0];
                let a = frame.at(c, 0);
                c.sq(2, a);
                c.lqc2(3, r.wrapping_add(R_VEL));
                vu_normalize3(c);
                vu_spill(c, frame, 0x50, &[0x10, 0x40]);
                let r = rider(c);
                c.g[V1][0] = r;
                c.g[A0][0] = r;
                c.lq(2, (r as u32).wrapping_add(R_FORWARD));
                Addr(frame.at(c, 0x30)).sw(c, 0);
                let a = frame.at(c, 0x20);
                c.sq(2, a);
                c.g[A3][0] = Addr(r as u32).off(R_SURFACE).lw(c);
                call_ptr(c, RIDER_START_WIPEOUT, 0x0013_f234)?;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, V0)).off(SURFACE_RESETS).lw(c);
                c.g[A0][0] = c.g[A3][0];
                let mut reset = c.g[V0][0] != 0;
                if !reset {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(r).off(R_CONTACT_FLAGS).lh(c) & 2;
                    reset = c.g[V0][0] != 0;
                }
                if reset {
                    // Rider_CheckResetPath(rider, 0, 1)
                    c.cycles += 4;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 1;
                    call_ptr(c, RIDER_CHECK_RESET_PATH, 0x0013_f264)?;
                }
            }
        }
        c.g[A0][0] = rider(c);

        // Rider_UpdateBounds(rider, 0); the walls with the ground spheres.
        c.cycles += 21;
        c.g[A1][0] = 0;
        call_ptr(c, RIDER_UPDATE_BOUNDS, 0x0013_f270)?;
        c.g[A1][0] = rider(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(GP_HULL_GROUND_MASK)).lw(c);
        c.g[V0][0] = Addr(r32(c, A1)).off(R_HULL).lw(c);
        let mask = c.g[V1][0];
        Addr(r32(c, V0)).off(0x28).sw(c, mask);
        RIDER_GROUND_STATE_COLLIDE_WALLS.call(c, 0x0013_f288)?;
        c.g[A0][0] = rider(c);
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = 0;
        c.g[V1][0] = Addr(r32(c, A0)).off(R_HULL).lw(c);
        let all = c.g[V0][0];
        Addr(r32(c, V1)).off(0x28).sw(c, all);
        // Rider_HandleWallCollision(rider, 0); Rider_CheckRiderCollisions(rider)
        c.g[A0][0] = rider(c);
        call_ptr(c, RIDER_HANDLE_WALL_COLLISION, 0x0013_f2a8)?;
        c.g[A0][0] = rider(c);
        call_ptr(c, RIDER_CHECK_RIDER_COLLISIONS, 0x0013_f2b0)?;
        c.g[V0][0] = this.off(GS_LEAVING).lw(c);
        if c.g[V0][0] != 0 {
            // Main state 0: set 1 and play the take-off sound.
            c.cycles += 4;
            c.g[A0][0] = rider(c);
            call_ptr(c, RIDER_GET_MAIN_STATE, 0x0013_f2c4)?;
            let state = c.g[V0][0];
            c.g[A0][0] = rider(c);
            if state == 0 {
                c.cycles += 6;
                c.g[A1][0] = 1;
                call_ptr(c, RIDER_SET_MAIN_STATE, 0x0013_f2d4)?;
                c.g[A0][0] = Addr(r32(c, GP).wrapping_add(GP_AUDIO)).lw(c);
                c.g[A1][0] = rider(c);
                call_ptr(c, SSX_AUDIO_PLAY_TAKEOFF_SOUND, 0x0013_f2e0)?;
            }
        }
        c.g[A0][0] = rider(c);

        // up = normalize(up + forward * 0.5)
        c.cycles += 44;
        let r = r32(c, A0);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        c.mfc1(V1, 0);
        vu_scale(c, V1, r.wrapping_add(R_FORWARD));
        vu_spill(c, frame, 0x30, &[0x20]);
        c.g[V1][0] = rider(c);
        c.lqc2(3, r32(c, V1).wrapping_add(R_SMOOTH_UP));
        c.lqc2(4, frame.at(c, 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        vu_spill(c, frame, 0x30, &[0x10]);
        c.lqc2(3, frame.at(c, 0x10));
        vu_normalize3(c);
        vu_spill(c, frame, 0x30, &[0]);
        c.sq(2, r.wrapping_add(R_SMOOTH_UP));
        // |v|, clamped to the maximal speed.
        let r = rider(c);
        c.g[V1][0] = r;
        let r = r as u32;
        c.lqc2(3, r.wrapping_add(R_VEL));
        vu_dot3(c, 3);
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V0, 22);
        c.mtc1(V0, 1); // f1 = |v|
        c.lwc1(0, r.wrapping_add(R_MAX_SPEED));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.cycles += 6;
            c.div_s(0, 0, 1);
            c.mfc1(V0, 0);
            c.lqc2(4, r.wrapping_add(R_VEL));
            c.qmtc2(V0, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, r.wrapping_add(R_VEL));
        }
        // fraction = 0 below 0, |v| / full below full, else 1.
        c.cycles += 6;
        c.mtc1(0, 0);
        c.lwc1(2, r32(c, GP).wrapping_sub(GP_FULL_SPEED));
        c.c_le_s(0, 1);
        c.g[V0][0] = rider(c);
        if c.fcc() {
            c.cycles += 5;
            c.c_le_s(1, 2);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            if c.fcc() {
                c.cycles += 1;
                c.div_s(0, 1, 2);
            }
        }
        c.cycles += 13;
        frame.restore_ra(c, 0x60);
        c.swc1(0, r32(c, V0).wrapping_add(R_SPEED_FRACTION));
        let r = rider(c);
        c.g[V1][0] = r;
        c.lwc1(0, (r as u32).wrapping_add(0x1f0));
        c.abs_s(0, 0);
        c.swc1(0, (r as u32).wrapping_add(R_ABS_VEL_Z));
        let r = rider(c);
        c.g[V0][0] = r;
        frame.restore(c, S0, 0x70);
        c.lwc1(0, (r as u32).wrapping_add(0x214));
        c.abs_s(0, 0);
        c.swc1(0, (r as u32).wrapping_add(R_ABS_SPIN));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_DecEaxa_DecodeStream (0x3ca460, EA SND library)

const SNDSTRM_RELEASE_BYTES: u32 = 0x003c_7010;
const SNDSTRM_POP_BLOCK: u32 = 0x003c_6de0;
const MEMSET: u32 = 0x0041_6210;
const SND_EAXA_DECODE: Callee = Callee {
    addr: 0x003c_cf90,
    f: ee::s14::SND_EaxaDecode,
};
const SND_EAXA_SET_HISTORY: Callee = Callee {
    addr: 0x003c_d190,
    f: ee::s14::SND_EaxaSetHistory,
};
const SND_EAXA_SET_INPUT: Callee = Callee {
    addr: 0x003c_cf60,
    f: ee::s14::SND_EaxaSetInput,
};

/// The EA-XA stream decoder's fields: its ADPCM decoder, the current block
/// (sample data), the block's sample count, samples decoded from it, the
/// codec version (<= 2: blocks start with two history samples), the
/// stream and channel, and the samples not yet released to the stream.
const DX_DECODER: u32 = 0x1c;
const DX_BLOCK: u32 = 0x20;
const DX_BLOCK_SAMPLES: u32 = 0x24;
const DX_BLOCK_POS: u32 = 0x28;
const DX_VERSION: u32 = 0x2c;
const DX_STREAM: u32 = 0x30;
const DX_UNRELEASED: u32 = 0x34;
const DX_CHANNEL: u32 = 0x38;

/// `int SND_DecEaxa_DecodeStream(DecEaxa* d, int n, int, short** out)`
/// (0x3ca460, the output cursor is passed through the frame): releases the
/// samples consumed by the last call to the stream, then decodes `n`
/// samples: whatever the ADPCM decoder still holds, then block after block
/// popped from the stream (`SNDSTRM_PopBlock`; old-version blocks seed the
/// decoder history from their first two samples). When the stream runs dry
/// the rest of the output is cleared (only if something was decoded since
/// the last release). Returns the number of samples decoded.
pub fn snd_dec_eaxa_decode_stream(c: &mut Cpu) {
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0]; // d
        c.g[S1][0] = c.g[A1][0]; // n
        c.g[S2][0] = 0; // seed the history
        c.g[S3][0] = 0; // decoded
        let d = r32(c, S0);
        let at = |o: u32| Addr(d.wrapping_add(o));
        let out = frame.at(c, 0x10);
        c.g[A2][0] = at(DX_UNRELEASED).lw(c);
        let a3 = c.g[A3][0];
        Addr(out).sw(c, a3);
        if c.g[A2][0] != 0 {
            // SNDSTRM_ReleaseBytes(stream, channel, unreleased)
            c.cycles += 4;
            c.g[A0][0] = at(DX_STREAM).lw(c);
            c.g[A1][0] = at(DX_CHANNEL).lbu(c);
            call_ptr(c, SNDSTRM_RELEASE_BYTES, 0x003c_a4a4)?;
            at(DX_UNRELEASED).sw(c, 0);
        }

        c.cycles += 2;
        c.g[S4][0] = 1;
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 2;
            c.g[A0][0] = at(DX_DECODER).lw(c);
            loop {
                // k = SND_EaxaDecode(decoder, &out, n)
                c.cycles += 17;
                c.g[A1][0] = sx(out);
                c.g[A2][0] = c.g[S1][0];
                SND_EAXA_DECODE.call(c, 0x003c_a4c4)?;
                let k = r32(c, V0);
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = Addr(out).lw(c);
                c.g[V0][0] = at(DX_BLOCK_POS).lw(c);
                c.g[A1][0] = sx(k << 2);
                c.g[V1][0] = at(DX_UNRELEASED).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(k << 2));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(k));
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(k));
                let v = c.g[V0][0];
                at(DX_BLOCK_POS).sw(c, v);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(k));
                let a0 = c.g[A0][0];
                Addr(out).sw(c, a0);
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let v1 = c.g[V1][0];
                at(DX_UNRELEASED).sw(c, v1);
                if c.g[V0][0] == 0 {
                    // Satisfied by the decoder alone.
                    c.cycles += 1;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_sub(k));
                } else {
                    // The decoder ran dry: the next block.
                    c.cycles += 9;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_sub(k));
                    c.g[A0][0] = at(DX_STREAM).lw(c);
                    c.g[A1][0] = at(DX_CHANNEL).lbu(c);
                    c.g[A2][0] = sx(d.wrapping_add(DX_BLOCK_SAMPLES));
                    c.g[A3][0] = sx(frame.at(c, 0x14));
                    call_ptr(c, SNDSTRM_POP_BLOCK, 0x003c_a514)?;
                    c.g[A2][0] = c.g[V0][0];
                    let block = c.g[A2][0];
                    at(DX_BLOCK).sw(c, block);
                    if block != 0 {
                        c.cycles += 6;
                        c.g[V1][0] = at(DX_VERSION).lw(c);
                        c.g[V0][0] = 2;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            c.g[S2][0] = c.g[S4][0];
                        }
                        c.g[A1][0] = c.g[SP][0];
                        if c.g[S2][0] != 0 {
                            // SND_EaxaSetHistory(decoder, {p[0], p[1]}); p += 2
                            c.cycles += 13;
                            let p = Addr(block as u32);
                            c.g[V0][0] = p.lh(c);
                            c.g[A0][0] = at(DX_DECODER).lw(c);
                            c.mtc1(V0, 0);
                            c.cvt_s_w(0, 0);
                            let a = frame.at(c, 0);
                            c.swc1(0, a);
                            c.g[V0][0] = p.off(2).lh(c);
                            c.mtc1(V0, 0);
                            c.cvt_s_w(0, 0);
                            let a = frame.at(c, 4);
                            c.swc1(0, a);
                            SND_EAXA_SET_HISTORY.call(c, 0x003c_a560)?;
                            c.g[V0][0] = at(DX_BLOCK).lw(c);
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
                            let v = c.g[V0][0];
                            at(DX_BLOCK).sw(c, v);
                        }
                        // SND_EaxaSetInput(decoder, p, samples * 4, samples)
                        c.cycles += 9;
                        c.g[A2][0] = at(DX_BLOCK_SAMPLES).lw(c);
                        at(DX_BLOCK_POS).sw(c, 0);
                        c.g[A3][0] = c.g[A2][0];
                        c.g[A0][0] = at(DX_DECODER).lw(c);
                        c.g[A1][0] = at(DX_BLOCK).lw(c);
                        c.g[A2][0] = sx(r32(c, A2) << 2);
                        SND_EAXA_SET_INPUT.call(c, 0x003c_a588)?;
                    } else {
                        // The stream ran dry: clear the rest, stop.
                        c.cycles += 2;
                        if (c.g[S1][0] as i64) > 0 {
                            c.cycles += 3;
                            c.g[V0][0] = at(DX_UNRELEASED).lw(c);
                            c.g[A0][0] = Addr(out).lw(c);
                            if c.g[V0][0] != 0 {
                                // memset(out, 0, n * 4)
                                c.cycles += 3;
                                c.g[A2][0] = sx(r32(c, S1) << 2);
                                c.g[A1][0] = 0;
                                call_ptr(c, MEMSET, 0x003c_a5b0)?;
                            }
                            c.cycles += 2;
                        }
                        c.g[S1][0] = 0;
                    }
                }
                c.cycles += 2;
                if (c.g[S1][0] as i64) <= 0 {
                    break;
                }
                c.g[A0][0] = at(DX_DECODER).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003c_a4b8);
                }
            }
        }

        c.cycles += 9;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Irradiance_AddDirectionalScaled (0x389308, particle.cpp)

/// gp-relative `.lit4`: the spherical-harmonic basis constants (band 0,
/// band 1 and the five band-2 terms, with their per-term scales).
const GP_SH_CONSTS: [(usize, u32); 11] = [
    (30, 0x272c),
    (29, 0x2728),
    (28, 0x2724),
    (27, 0x2720),
    (18, 0x271c),
    (17, 0x2718),
    (26, 0x2714),
    (25, 0x2710),
    (24, 0x270c),
    (23, 0x2708),
    (22, 0x2704),
];
/// The callee-saved FPU registers the function uses, with their frame
/// slots.
const SH_SAVED: [(usize, u32); 11] = [
    (20, 0x30),
    (30, 0x80),
    (29, 0x78),
    (28, 0x70),
    (27, 0x68),
    (26, 0x60),
    (25, 0x58),
    (24, 0x50),
    (23, 0x48),
    (22, 0x40),
    (21, 0x38),
];

/// `void Irradiance_AddDirectionalScaled(SH9Color* sh, const float rgb[3],
/// float scale, float x, float y, float z)` (0x389308): adds a directional
/// light of colour `rgb * scale` from direction (x, y, z) to the 9-term
/// spherical-harmonic irradiance `sh` (per channel: band 0 at +0x00,
/// band 2 terms at +0x10..+0x60 in 0x10 strides, band 1 at +0x70/+0x80/
/// +0x90). The per-channel terms are spilled to the frame as the original
/// does (0x00..0x20); every FPU temporary is left as the original leaves
/// it. Leaf.
pub fn irradiance_add_directional_scaled(c: &mut Cpu) {
    c.cycles += 40;
    // f0 = 3 z^2 - 1 (f19), f20 = x^2 - y^2; f16 = x.
    c.g[AT][0] = 0x4040_0000;
    c.mtc1(AT, 0);
    c.mov_s(16, 13);
    c.mul_s(2, 16, 16);
    let frame = Frame::open(c, 0x90);
    c.mul_s(0, 15, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3);
    c.mul_s(1, 14, 14);
    for (f, off) in SH_SAVED {
        frame.save_fpr(c, f, off);
    }
    c.g[T0][0] = c.g[A0][0]; // sh
    c.mul_s(0, 15, 0);
    c.sub_s(20, 2, 1);
    c.g[A3][0] = c.g[T0][0]; // band 0 cursor
    c.sub_s(19, 0, 3);
    c.g[T1][0] = c.g[A1][0]; // rgb cursor
    c.g[T2][0] = 0; // channel
    c.g[A2][0] = sx(r32(c, A3).wrapping_add(0x60)); // band 2 cursor
    c.mov_s(21, 12); // scale
    for (f, off) in GP_SH_CONSTS {
        c.lwc1(f, r32(c, GP).wrapping_sub(off));
    }
    loop {
        c.cycles += 81;
        let sp = frame.at(c, 0);
        c.lwc1(0, r32(c, T1));
        c.g[V0][0] = sx(r32(c, T2) << 2);
        c.lwc1(3, r32(c, A3));
        c.g[V1][0] = sx(r32(c, T0).wrapping_add(0x70));
        c.mul_s(0, 0, 21); // colour * scale
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = sx(r32(c, T0).wrapping_add(0x80));
        c.g[A1][0] = sx(r32(c, T0).wrapping_add(0x90));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.mul_s(4, 0, 30);
        c.g[T2][0] = sx(r32(c, T2).wrapping_add(1));
        c.g[T1][0] = sx(r32(c, T1).wrapping_add(4));
        c.g[V0][0] = ((c.g[T2][0] as i64) < 3) as u64;
        c.swc1(4, sp);
        c.mul_s(4, 0, 29);
        c.lwc1(2, sp);
        c.mul_s(11, 4, 16);
        c.mul_s(12, 4, 14);
        c.mul_s(13, 4, 15);
        c.mul_s(4, 0, 28);
        c.swc1(11, sp + 0xc);
        c.mul_s(2, 2, 25);
        c.swc1(12, sp + 0x4);
        c.swc1(13, sp + 0x8);
        c.mul_s(11, 11, 17);
        c.mul_s(6, 4, 16);
        c.mul_s(8, 4, 14);
        c.mul_s(4, 0, 27);
        c.mul_s(9, 6, 15);
        c.mul_s(8, 8, 15);
        c.mul_s(7, 4, 19);
        c.mul_s(4, 0, 26);
        c.swc1(9, sp + 0x1c);
        c.mul_s(6, 6, 14);
        c.swc1(8, sp + 0x14);
        c.mul_s(1, 7, 24);
        c.swc1(7, sp + 0x18);
        c.mul_s(0, 4, 20);
        c.swc1(6, sp + 0x10);
        c.mul_s(7, 7, 22);
        c.sub_s(2, 2, 1);
        c.swc1(0, sp + 0x20);
        c.mul_s(10, 0, 23);
        c.mul_s(6, 6, 18);
        c.add_s(3, 3, 2);
        c.mul_s(9, 9, 18);
        c.mul_s(8, 8, 18);
        c.swc1(3, r32(c, A3)); // band 0
        c.mul_s(12, 12, 17);
        c.mul_s(13, 13, 17);
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
        // Band 2: the five terms of this channel (0x10 strides).
        let b2 = r32(c, A2);
        c.lwc1(2, b2.wrapping_sub(0x50));
        c.lwc1(4, b2.wrapping_sub(0x40));
        c.lwc1(5, b2.wrapping_sub(0x30));
        c.add_s(2, 2, 10);
        c.lwc1(3, b2.wrapping_sub(0x20));
        c.sub_s(4, 4, 10);
        c.lwc1(1, b2.wrapping_sub(0x10));
        c.add_s(5, 5, 7);
        c.lwc1(0, b2);
        c.add_s(3, 3, 6);
        c.add_s(1, 1, 9);
        c.swc1(2, b2.wrapping_sub(0x50));
        c.add_s(0, 0, 8);
        c.swc1(4, b2.wrapping_sub(0x40));
        c.swc1(5, b2.wrapping_sub(0x30));
        c.swc1(3, b2.wrapping_sub(0x20));
        c.swc1(1, b2.wrapping_sub(0x10));
        c.swc1(0, b2);
        c.g[A2][0] = sx(b2.wrapping_add(4));
        // Band 1: x, y, z terms.
        for (reg, term) in [(V1, 11), (A0, 12), (A1, 13)] {
            let a = r32(c, reg);
            let f = if term == 12 { 1 } else { 0 };
            c.lwc1(f, a);
            c.add_s(f, f, term);
            c.swc1(f, a);
        }
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_93a8);
        }
    }
    c.cycles += 13;
    for (f, off) in SH_SAVED {
        frame.restore_fpr(c, f, off);
    }
    frame.close(c);
}

// ---------------------------------------------------------------------------
// SND_DecUtk_DecodeLoop (0x3c9520, EA SND library)

const SND_UTK_DECODE: Callee = Callee {
    addr: 0x003c_de68,
    f: ee::s14::SND_UtkDecode,
};
const SND_UTK_SET_STATE: Callee = Callee {
    addr: 0x003c_e0b8,
    f: ee::s14::SND_UtkSetState,
};
const SND_UTK_SET_INPUT: Callee = Callee {
    addr: 0x003c_dde8,
    f: ee::s14::SND_UtkSetInput,
};

/// The looping MicroTalk decoder's fields: its UTK decoder, the sample
/// position, the loop start and end samples, the stream data.
const DU_DECODER: u32 = 0x1c;
const DU_POS: u32 = 0x20;
const DU_LOOP_START: u32 = 0x24;
const DU_LOOP_END: u32 = 0x28;
const DU_DATA: u32 = 0x2c;

/// `int SND_DecUtk_DecodeLoop(DecUtk* d, int n, int, float* out)`
/// (0x3c9520, the output cursor is passed through the frame): decodes `n`
/// samples of a looping MicroTalk sound. Whenever the decoder runs out
/// before the request is met, the position jumps back to the loop start,
/// the decoder state is reset (`SND_UtkSetState({0, 0, 1})`) and the loop's
/// `end - start + 1` samples are queued again (`SND_UtkSetInput`). Returns
/// 1.
pub fn snd_dec_utk_decode_loop(c: &mut Cpu) {
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0]; // d
        c.g[S1][0] = c.g[A1][0]; // n
        let d = r32(c, S0);
        let at = |o: u32| Addr(d.wrapping_add(o));
        let out = frame.at(c, 0x10);
        let a3 = c.g[A3][0];
        Addr(out).sw(c, a3);
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S2][0] = 1;
            c.g[A0][0] = at(DU_DECODER).lw(c);
            loop {
                // k = SND_UtkDecode(decoder, &out, n)
                c.cycles += 13;
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = sx(out);
                SND_UTK_DECODE.call(c, 0x003c_955c)?;
                let k = r32(c, V0);
                c.g[A0][0] = Addr(out).lw(c);
                c.g[V1][0] = sx(k << 2);
                c.g[A1][0] = at(DU_POS).lw(c);
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(k));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(k << 2));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(k));
                let a0 = c.g[A0][0];
                Addr(out).sw(c, a0);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let a1 = c.g[A1][0];
                at(DU_POS).sw(c, a1);
                if c.g[V0][0] != 0 {
                    // Back to the loop start: reset and requeue the loop.
                    c.cycles += 18;
                    c.g[V0][0] = at(DU_LOOP_START).lw(c);
                    c.g[A2][0] = 0;
                    c.g[A0][0] = at(DU_DECODER).lw(c);
                    c.g[A1][0] = c.g[SP][0];
                    let start = c.g[V0][0];
                    at(DU_POS).sw(c, start);
                    Addr(frame.at(c, 4)).sw(c, 0);
                    Addr(frame.at(c, 0)).sw(c, 0);
                    let one = c.g[S2][0];
                    Addr(frame.at(c, 8)).sw(c, one);
                    SND_UTK_SET_STATE.call(c, 0x003c_95a8)?;
                    c.g[A2][0] = at(DU_LOOP_END).lw(c);
                    c.g[V0][0] = at(DU_LOOP_START).lw(c);
                    c.g[A0][0] = at(DU_DECODER).lw(c);
                    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, V0)));
                    c.g[A1][0] = at(DU_DATA).lw(c);
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                    c.g[A3][0] = c.g[A2][0];
                    c.g[A2][0] = sx(r32(c, A2) << 2);
                    SND_UTK_SET_INPUT.call(c, 0x003c_95cc)?;
                }
                c.cycles += 2;
                if (c.g[S1][0] as i64) <= 0 {
                    break;
                }
                c.g[A0][0] = at(DU_DECODER).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003c_9550);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WPData_FindCellIndex (0x2baf90)

const QUAD_TREE_FIND_LEAF: Callee = Callee {
    addr: 0x002c_1cd8,
    f: ee::s08::QuadTree_FindLeaf,
};

/// `int WPData_FindCellIndex(WPData* wp, float x, float z)` (0x2baf90): the
/// world-painter cell at (x, z): the leaf of the data's quadtree
/// (`QuadTree_FindLeaf(wp->tree, x, z)`) and its cell index (`leaf + 4`).
pub fn wp_data_find_cell_index(c: &mut Cpu) {
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, A0)).lw(c);
        QUAD_TREE_FIND_LEAF.call(c, 0x002b_afa0)?;
        c.g[V0][0] = Addr(r32(c, V0)).off(4).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_v111 (0x386bd0, particle.cpp): blended skinning matrices

/// The blend weights are hundredths (`0.01` in VU0's I register).
const BLEND_WEIGHT_SCALE: u32 = 0x3c23_d70a;

/// One weighted bone of a blend: `vf_acc (+)= matrix(bone) * weight / 100`
/// (`first`: plain scale into vf1..vf4; else accumulate through ACC with
/// vf6..vf9). `inf` points at `{s16 weight; u8 bone}`.
#[inline]
fn blend_bone(c: &mut Cpu, inf: u32, first: bool) {
    use crate::guest::Addr;
    use ssx3_ee_rt::vu0::{Src, W, X};
    c.g[V0][0] = Addr(inf).off(2).lbu(c);
    c.g[V1][0] = Addr(inf).lh(c);
    c.g[V0][0] = sx(r32(c, V0) << 6);
    c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
    let m = r32(c, V0);
    c.qmtc2(V1, 5);
    c.vu0().itof(0, 0b1000, 5, 5);
    let base = if first { 1 } else { 6 };
    c.lqc2(base, m);
    c.lqc2(base + 1, m.wrapping_add(0x10));
    c.vu0().mul(0b1000, 5, 5, Src::I);
    c.lqc2(base + 2, m.wrapping_add(0x20));
    c.lqc2(base + 3, m.wrapping_add(0x30));
    for row in 0..4 {
        if first {
            c.vu0().mul(0b1111, 1 + row, 1 + row, Src::Bc(5, X));
        } else {
            c.vu0().mula(0b1111, 1 + row, Src::Bc(0, W));
            c.vu0().madd(0b1111, 1 + row, 6 + row, Src::Bc(5, X));
        }
    }
}

/// `void PS2GraphicsMan_v111(PS2GraphicsMan* gm, const Matrix* bones,
/// int, const Blend* blends, int count, Matrix* out)` (0x386bd0, a
/// leaf; `out` defaults to the manager's buffer at +0x13e8 when null):
/// for each of `count` blends (12 bytes: number of influences, pointer to
/// `{s16 weight; u8 bone}` influences), writes the weighted sum of the
/// bones' matrices (weights in hundredths) into the next 4x4 output
/// matrix. VU0 macro code; leaves LO/HI as its index multiplies do.
pub fn ps2_graphics_man_blend_bone_matrices(c: &mut Cpu) {
    use crate::guest::Addr;
    c.cycles += 4;
    let frame = Frame::open(c, 0x10);
    c.g[T5][0] = c.g[A3][0]; // blends
    frame.save(c, S5, 0);
    if c.g[T1][0] == 0 {
        c.cycles += 1;
        c.g[T1][0] = Addr(r32(c, A0)).off(0x13e8).lw(c);
    }
    c.cycles += 5;
    c.g[V0][0] = BLEND_WEIGHT_SCALE as u64;
    c.ctc2(V0, 21); // I = 0.01
    c.g[A3][0] = 0;
    if (c.g[T0][0] as i64) > 0 {
        c.cycles += 2;
        c.g[T4][0] = 0xc;
        c.mult(0, V0, A3, T4);
        loop {
            c.cycles += 25;
            c.g[T2][0] = c.g[T1][0]; // this output matrix
            c.g[T1][0] = sx(r32(c, T2).wrapping_add(0x40));
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, T5))); // blend
            c.g[A2][0] = Addr(r32(c, A0)).off(4).lw(c);
            blend_bone(c, r32(c, A2), true);
            c.g[A0][0] = Addr(r32(c, A0)).lw(c); // influences
            c.g[T3][0] = sx(r32(c, A3).wrapping_add(1));
            c.g[A3][0] = 1;
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.g[A3][0] = sx(r32(c, A0).wrapping_sub(1));
                loop {
                    c.cycles += 22;
                    blend_bone(c, r32(c, A2), false);
                    c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
                    if c.g[A3][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0038_6c68);
                    }
                }
            }
            c.cycles += 8;
            let out = r32(c, T2);
            for row in 0..4u32 {
                c.sqc2(1 + row as usize, out.wrapping_add(0x10 * row));
            }
            c.g[A3][0] = c.g[T3][0];
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.mult(0, V0, A3, T4);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0038_6c00);
            }
        }
    }
    c.cycles += 3;
    frame.restore(c, S5, 0);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// OverlapList_FindNearestRail: lists of 0-4 objects of every type
    /// (rail modifier, flex rail, plain rail with flags passing or failing
    /// the mask, and non-rail types). The real callees take their short
    /// exits: the rail modifier's piece lookup finds no piece (its mask test
    /// reads flag 0), the flex rail's flags fail the mask, and the plain
    /// rail's bounds lie beyond the query box.
    #[test]
    fn overlap_list_find_nearest_rail_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, put_vec, DATA, GAME_GP};
        const LIST: u32 = DATA + 0x1000;
        const POS: u32 = DATA + 0x2000;
        const OUT: u32 = DATA + 0x2100;
        const OBJS: u32 = DATA + 0x4000;
        // The getter 0x2d1bd8 returns the word at gp+0x16c8: a table whose
        // piece array (+8) is empty, so the modifier finds no piece.
        const PIECES: u32 = DATA + 0x3000;
        check(
            ee::s11::f_00334680__generated,
            overlap_list_find_nearest_rail,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3346_80, 0);
                c.pc = 0x0033_4680;
                c.bus.write32(GAME_GP + 0x16c8, PIECES);
                c.bus.write32(PIECES, PIECES + 0x10);
                c.bus.write32(PIECES + 0x10 + 8, PIECES + 0x40);
                for i in 0..0x100 {
                    c.bus.write32(PIECES + 0x40 + 4 * i, 0);
                }
                let count = if seed % 7 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 4
                };
                c.bus.write32(LIST + 0x210, count);
                for i in 0..count {
                    let obj = OBJS + 0x200 * i;
                    let ty = r.next_u32() % 6; // 0, 4, 5: no rail
                    c.bus.write32(LIST + 0x214 + 4 * i, obj);
                    c.bus.write32(obj + 8, ty);
                    // Rail modifier / flex rail data.
                    let data = obj + 0x100;
                    c.bus.write32(obj + 0xc, data);
                    c.bus.write32(data + 0x30, r.next_u32()); // piece index
                    c.bus.write32(data + 0x24, 0x8000_0000); // flex flags
                                                             // Plain rail: instance flags, bounds beyond the box.
                    c.bus.write32(obj + 0x68, obj + 0x180);
                    let flags = if r.next_u32() % 2 == 0 { 0x10 } else { 0x4 };
                    c.bus.write32(obj + 0x180 + 0x1c, flags);
                    c.bus.write32(obj + 0x6c, 1000.0f32.to_bits());
                }
                put_vec(
                    c,
                    POS,
                    [4.0f32.to_bits(), 3.0f32.to_bits(), 2.0f32.to_bits(), 0],
                );
                c.g[A0][0] = sx(LIST);
                c.g[A1][0] = sx(POS);
                c.g[A2][0] = sx(OUT);
                c.g[A3][0] = sx(0x7fff_fff0 | (seed & 0xf)); // mask: 0x10 passes
                c.f[12] = 1.5f32.to_bits();
            },
        );
    }

    /// RiderGroundState_CollideWalls fixture. `impact`: the rider moves into
    /// the wall after the push-out (reaches the bounce, the deflection and
    /// the impact/sound callees); otherwise it recedes from it.
    ///
    /// The sub-state is 9 or not; the controller's "in contact" method is the
    /// real getter 0x2d1bd8 (returns the word at gp+0x16c8, set to 0 or 1);
    /// speed and height pick the slow-on-the-ground early exit. The overlap
    /// list holds no object (no contact: depth -1) or one static object with
    /// a huge box whose sub-object's collide method (a fake) reports one
    /// contact; the contact record then comes from the closest-contact
    /// query's frame, which the fixture fills (depth, normal, surface velocity,
    /// hit object with or without an owner to notify).
    fn collide_walls_setup(c: &mut Cpu, seed: u32, impact: bool) {
        use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
        use crate::difftest::{put_vec, DATA, GAME_GP, SP};
        let f = |x: f32| x.to_bits();
        let mut r = machine(c, seed, 0x13f4_88 ^ impact as u64, 4);
        c.pc = 0x0013_f488;
        const SELF: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const SUB: u32 = DATA + 0x3000;
        const CTL_VT: u32 = DATA + 0x3100;
        const HULL: u32 = DATA + 0x3200;
        const LIST: u32 = DATA + 0x3400;
        const OBJ: u32 = DATA + 0x3600;
        const SUBOBJ: u32 = DATA + 0x3700;
        const SUB_VT: u32 = DATA + 0x3800;
        const HIT_OBJ: u32 = DATA + 0x3a00;
        const OWNER: u32 = DATA + 0x3b00;
        const OWNER_VT: u32 = DATA + 0x3c00;
        // The closest-contact query's frame (0x2050 below ours, 0x490).
        const RECORD: u32 = SP - 0x490 - 0x2050;

        // .lit4 constants.
        c.bus.write32(GAME_GP - 0x6fa0, f(-0.999_9));
        c.bus.write32(GAME_GP - 0x6f9c, f(1.1));
        c.bus.write32(GAME_GP - 0x6f98, f(0.05));
        c.bus.write32(GAME_GP - 0x6f94, f(27.777_779));

        c.g[A0][0] = sx(SELF);
        c.bus.write32(SELF + 0x18, RIDER);
        c.bus.write32(RIDER + 0x77c, SUB);
        let sub_state = if !impact && seed % 9 == 0 {
            9
        } else {
            r.next_u32() % 8
        };
        c.bus.write32(SUB + 0xde4, sub_state);
        // Controller: vt[0x40] = { delta, 0x2d1bd8 }.
        c.bus.write32(RIDER + 0x6c0, CTL_VT);
        c.bus.write16(CTL_VT + 0x40, (r.next_u32() % 4 * 4) as u16);
        c.bus.write32(CTL_VT + 0x44, 0x002d_1bd8);
        let in_contact = impact || r.next_u32() % 2 == 0;
        c.bus.write32(GAME_GP + 0x16c8, in_contact as u32);
        c.bus.write32(RIDER + 0x4c8, f((r.next_u32() % 100) as f32));
        let y = (r.next_u32() % 200) as f32 - 100.0;
        c.bus.write32(RIDER + 0x118, f(y));
        c.bus
            .write32(RIDER + 0x498, f(y + (r.next_u32() % 100) as f32 - 50.0));
        c.bus.write32(RIDER + 0xaa0, HULL);
        c.bus.write32(RIDER + 0x860, LIST);
        c.bus.write32(RIDER + 0x868, r.next_u32() % 0x40);
        // Hull: no spheres (+0x28 mask, +0x2c count), radius at +0x20.
        c.bus.write32(HULL + 0x20, f(0.5));

        // Overlap list: no instances, zero or one object.
        let contact = impact || seed % 5 != 1;
        c.bus.write32(LIST + 0x8, contact as u32);
        c.bus.write32(LIST + 0xc, LIST + 0x40);
        c.bus.write32(LIST + 0x40, OBJ);
        c.bus.write32(OBJ + 0x8, 0x20);
        for k in 0..3 {
            c.bus.write32(OBJ + 0x60 + 4 * k, f(-1.0e30));
            c.bus.write32(OBJ + 0x6c + 4 * k, f(1.0e30));
        }
        c.bus.write32(OBJ + 0x88, 1);
        c.bus.write32(OBJ + 0xc, SUBOBJ);
        c.bus.write32(SUBOBJ + 0xc, SUB_VT);
        c.bus.write32(SUB_VT + 0x134, FAKE);
        c.bus.write32(SUB_VT + 0x13c, FAKE + 0x10);
        // Fake call 1 (vt 0x13c) reports one contact.
        c.bus.write32(LOG_RESULTS + 4, 1);

        // The contact record.
        let unit = |r: &mut crate::difftest::Rng| {
            let v = [
                (r.next_u32() % 200) as f32 - 100.0,
                (r.next_u32() % 200) as f32 - 100.0,
                (r.next_u32() % 200) as f32 - 100.0 + 0.5,
            ];
            let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            [v[0] / n, v[1] / n, v[2] / n]
        };
        let fwd = unit(&mut r);
        let normal = if !impact && seed % 7 == 3 {
            [-fwd[0], -fwd[1], -fwd[2]] // head-on
        } else {
            unit(&mut r)
        };
        put_vec(c, RIDER + 0x370, [f(fwd[0]), f(fwd[1]), f(fwd[2]), 0]);
        put_vec(
            c,
            RECORD + 0x10,
            [f(normal[0]), f(normal[1]), f(normal[2]), 0],
        );
        let point = [r.f32(50.0), r.f32(50.0), r.f32(50.0), 0];
        put_vec(c, RECORD, [r.f32(50.0), r.f32(50.0), r.f32(50.0), f(1.0)]);
        put_vec(c, RECORD + 0x20, point);
        for k in 0x30..0x40 {
            c.bus.write8(RECORD + k, r.next_u32() as u8);
        }
        let depth = if !impact && seed % 6 == 2 {
            -((r.next_u32() % 10) as f32) - 0.5
        } else {
            (r.next_u32() % 10) as f32 * 0.25
        };
        c.bus.write32(RECORD + 0x40, f(depth));
        c.bus.write32(RECORD + 0x44, r.next_u32());
        c.bus.write32(RECORD + 0x48, r.next_u32());
        c.bus.write32(RECORD + 0x4c, r.next_u32() % 16);
        let hit_obj = match seed % 3 {
            0 => 0,
            _ => HIT_OBJ,
        };
        c.bus.write32(RECORD + 0x50, hit_obj);
        for k in 0x54..0x80 {
            c.bus.write8(RECORD + k, r.next_u32() as u8);
        }
        c.bus
            .write32(HIT_OBJ + 0xc, if seed % 3 == 1 { 0 } else { OWNER });
        c.bus.write32(OWNER + 0xc, OWNER_VT);
        c.bus.write16(OWNER_VT + 0x150, 8);
        c.bus.write32(OWNER_VT + 0x154, FAKE + 0x20);

        // The rider's velocity relative to the hit surface's: along the
        // normal (receding: no bounce) or against it (impact).
        let k = (1 + r.next_u32() % 20) as f32 * if impact { -1.0 } else { 1.0 };
        let p = |i: usize| f32::from_bits(point[i]);
        put_vec(
            c,
            RIDER + 0x1e0,
            [
                f(p(0) + normal[0] * k),
                f(p(1) + normal[1] * k),
                f(p(2) + normal[2] * k),
                0,
            ],
        );
        for off in [0x110, 0x400, 0x410, 0x9d0] {
            put_vec(
                c,
                RIDER + off,
                [r.f32(100.0), r.f32(100.0), r.f32(100.0), f(1.0)],
            );
        }
    }

    /// RiderGroundState_CollideWalls up to the bounce decision: skipped
    /// sub-state, slow-on-the-ground exit, no contact, head-on, facing and
    /// not facing the wall (normal flattening), the hit-object notification,
    /// and a rider receding from the wall after the push-out. Fakes unwind
    /// at chosen calls.
    #[test]
    fn rider_ground_state_collide_walls_matches() {
        crate::difftest::check(
            ee::s01::f_0013f488__generated,
            rider_ground_state_collide_walls,
            300,
            &|c, seed| collide_walls_setup(c, seed, false),
        );
    }

    /// RiderGroundState_CollideWalls through the bounce: velocity update,
    /// Rider_DeflectAlongWall, Rider_BuildWorldMatrix, Rider_HandleImpact
    /// and the wall-hit sound (real callees on a mostly zeroed rider; their
    /// virtual calls through empty memory land in the fakes).
    #[test]
    fn rider_ground_state_collide_walls_impact_matches() {
        crate::difftest::check(
            ee::s01::f_0013f488__generated,
            rider_ground_state_collide_walls,
            64,
            &|c, seed| collide_walls_setup(c, seed, true),
        );
    }

    /// MultiSplineModifier_UpdateNodes: 0-5 nodes, every mode (0: neither
    /// angle, 1: pitch, 2: yaw, 3: both, 4: neither), with and without a
    /// spin angle, start distances below 0 and beyond the length (both
    /// wraps). The path is one straight-forward segment whose direction
    /// components are zeroed per seed (dir.x/dir.y/dir.z = 0 reach the
    /// zero-denominator and zero-angle branches; negative components the
    /// quadrant fixes). Real callees (EvalPathAtDistance, Math_SinCos,
    /// Math_Atan) on the game image's constants.
    #[test]
    fn multi_spline_modifier_update_nodes_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{elf_image, put_vec, DATA};
        const SELF: u32 = DATA;
        const STATES: u32 = DATA + 0x1000;
        const NODES: u32 = DATA + 0x2000;
        const NODE_OBJS: u32 = DATA + 0x2100;
        const SEG: u32 = DATA + 0x3000;
        crate::difftest::check(
            ee::s11::f_0035ac20__generated,
            multi_spline_modifier_update_nodes,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35ac_20, 0);
                elf_image(c);
                c.pc = 0x0035_ac20;
                let f = |x: f32| x.to_bits();
                let n = match seed % 8 {
                    0 => 0,
                    1 => 1,
                    _ => 2 + r.next_u32() % 4,
                };
                let len = 10.0 + (r.next_u32() % 100) as f32;
                let start = match seed % 3 {
                    0 => -((r.next_u32() % 50) as f32) - 0.5,
                    1 => len + (r.next_u32() % 50) as f32,
                    _ => (r.next_u32() % 10) as f32,
                };
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + 0x4, n);
                c.bus.write32(SELF + 0x8, (seed / 3) % 5);
                let spin = if seed % 4 == 0 { 0 } else { r.f32(3.0) };
                c.bus.write32(SELF + 0xc, spin);
                c.bus.write32(SELF + 0x10, f(start));
                c.bus.write32(SELF + 0x30, 1);
                c.bus.write32(SELF + 0x3c, STATES);
                c.bus.write32(SELF + 0x44, NODES);
                c.bus.write32(SELF + 0x54, f(len));
                for i in 0..6 {
                    c.bus.write32(NODES + 4 * i, NODE_OBJS + 0x80 * i);
                }
                // The path: one segment covering everything.
                c.bus.write32(SELF + 0x48 + 0x4, 0);
                c.bus.write32(SELF + 0x48 + 0x8, SEG);
                c.bus.write32(SELF + 0x48 + 0xc, f(len * 2.0));
                c.bus.write32(SEG + 0x84, 0);
                c.bus.write32(SEG + 0x60, 0);
                c.bus.write32(SEG + 0x64, 0);
                for k in 0..4 {
                    c.bus
                        .write32(SEG + 0x50 + 4 * k, f((r.next_u32() % 8) as f32 * 0.125));
                }
                // Direction columns, with components zeroed by seed.
                let zero = (seed / 15) % 8; // bit 0: x, 1: y, 2: z
                for row in 0..4u32 {
                    let mut v = [
                        r.f32(4.0),
                        r.f32(4.0),
                        r.f32(4.0),
                        f(if row == 3 { 1.0 } else { 0.0 }),
                    ];
                    for k in 0..3usize {
                        if zero & (1 << k) != 0 {
                            v[k] = 0;
                        }
                    }
                    put_vec(c, SEG + 0x10 + 0x10 * row, v);
                }
                // The world up axis.
                put_vec(c, 0x004f_f160, [f(0.0), f(0.0), f(1.0), 0]);
                if seed % 5 == 0 {
                    put_vec(c, 0x004f_f160, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                }
            },
        );
    }

    /// Emitter_UpdateDescriptor: no emission due, one, or up to five
    /// (count * interval against the advanced time; positive intervals).
    #[test]
    fn emitter_update_descriptor_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::DATA;
        crate::difftest::check(
            ee::s12::f_0036d428__generated,
            emitter_update_descriptor,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36d4_28, 0);
                c.pc = 0x0036_d428;
                let f = |x: f32| x.to_bits();
                let d = DATA;
                for k in 0..0x40 {
                    c.bus.write32(d + 4 * k, r.next_u32());
                }
                let count = (r.next_u32() % 6) as i32 - 1;
                let interval = 0.25 + (r.next_u32() % 8) as f32 * 0.25;
                c.bus.write32(d + 0x0, count as u32);
                c.bus.write32(d + 0xc, f(interval));
                let behind = (seed % 7) as f32 - 1.5; // emissions due, roughly
                c.bus
                    .write32(d + 0x8, f(count as f32 * interval + behind * interval));
                c.bus
                    .write32(d + 0x4c, f(1.0 + (r.next_u32() % 4) as f32 * 0.5));
                for k in 0..8 {
                    c.bus.write32(d + 0xb0 + 4 * k, r.f32(100.0));
                    c.bus.write32(d + 0xe0 + 4 * k, r.f32(10.0));
                }
                c.g[A0][0] = sx(d);
                c.f[12] = f((r.next_u32() % 4) as f32 * 0.0625);
            },
        );
    }

    /// RiderGroundState_PostUpdate fixture. `branches == false`: the rider
    /// stays on the ground on an ordinary surface (no wipeout, no path
    /// reset); `true`: leaving the ground (`Rider_LaunchIntoAir`) in a main
    /// state other than 0. Sub-state 9 keeps the wall collisions
    /// (`RiderGroundState_CollideWalls`, `Rider_HandleWallCollision`) on
    /// their early exits; the rider's animation table answers "no bump"
    /// (entry 0x1b6); the zeroed BELibrary dummy at gp-0x1f04 keeps
    /// `BELibrary_GetInstance` (from `Rider_GetBumpStrength` /
    /// `Rider_CheckRiderCollisions`) from building the library, which loads
    /// files from disc (the runaway at 0x3de768). Velocities below and above
    /// the maximal speed and the full-speed constant.
    ///
    /// Not reached (unbounded on a test machine): the wipeout surface
    /// (`Rider_StartWipeout`), path resets (`Rider_CheckResetPath`) and
    /// main state 0 on leaving (`Rider_SetMainState` + take-off sound):
    /// their audio callees reach `BankMonitor_StopHandle` / sound playback,
    /// which reach the same disc-loading request loop.
    fn post_update_setup(c: &mut Cpu, seed: u32, branches: bool) {
        use crate::difftest::fakes::machine;
        use crate::difftest::{put_vec, DATA, GAME_GP};
        let f = |x: f32| x.to_bits();
        let mut r = machine(
            c,
            seed,
            0x13f1_78 ^ branches as u64,
            if branches { 3 } else { 0 },
        );
        c.pc = 0x0013_f178;
        const SELF: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const SUB: u32 = DATA + 0x3000;
        const HULL: u32 = DATA + 0x4000;
        const WORLD: u32 = DATA + 0x4100;
        const W84: u32 = DATA + 0x4200;
        const SURF: u32 = DATA + 0x5000;
        const ANIM: u32 = DATA + 0x6000;
        const BE_LIBRARY: u32 = DATA + 0x8000;
        for k in 0..0x400 {
            c.bus.write32(BE_LIBRARY + 4 * k, 0);
        }
        c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
        c.bus.write32(GAME_GP - 0x6fac, f(3333.3335));
        c.bus.write32(GAME_GP - 0x1f94, r.next_u32());
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, W84);
        c.bus.write32(W84 + 0x44, SURF);
        c.g[A0][0] = sx(SELF);
        c.bus.write32(SELF + 0x18, RIDER);
        let leaving = branches;
        c.bus.write32(SELF + 0xc, leaving as u32);
        c.bus.write32(RIDER + 0x77c, SUB);
        c.bus.write32(SUB + 0xde4, 9);
        c.bus.write32(
            SUB + 0xde0,
            if seed % 2 == 0 && !leaving {
                0
            } else {
                1 + r.next_u32() % 5
            },
        );
        c.bus.write32(RIDER + 0xaa0, HULL);
        c.bus.write32(RIDER + 0x784, ANIM);
        c.bus.write32(ANIM + 4, 0x1b6);
        let surface = r.next_u32() % 0x12;
        c.bus.write32(RIDER + 0x438, surface);
        let reset = false;
        let by_table = reset && seed % 2 == 0;
        c.bus.write32(SURF + surface * 0xb0 + 0x44, by_table as u32);
        let flags = (r.next_u32() & 0xfffd) as u16 | if reset && !by_table { 2 } else { 0 };
        c.bus.write16(RIDER + 0x2d4, flags);
        for off in [0x110, 0x370, 0x390] {
            put_vec(c, RIDER + off, [r.f32(1.0), r.f32(1.0), r.f32(1.0), f(1.0)]);
        }
        let speed = [10.0f32, 5000.0][(seed / 2 % 2) as usize];
        put_vec(
            c,
            RIDER + 0x1e0,
            [r.f32(speed), r.f32(speed), r.f32(speed), 0],
        );
        c.bus.write32(
            RIDER + 0x2e4,
            f([100.0f32, 8000.0][(seed / 4 % 2) as usize]),
        );
        c.bus.write32(RIDER + 0x1f0, r.f32(50.0));
        c.bus.write32(RIDER + 0x214, r.f32(5.0));
    }

    /// RiderGroundState_PostUpdate on the ground: no launch, wipeout or
    /// path reset; bounds, walls (masked hull spheres), rider collisions,
    /// up smoothing, speed clamp and fraction.
    #[test]
    fn rider_ground_state_post_update_matches() {
        crate::difftest::check(
            ee::s01::f_0013f178__generated,
            rider_ground_state_post_update,
            160,
            &|c, seed| post_update_setup(c, seed, false),
        );
    }

    /// RiderGroundState_PostUpdate leaving the ground: `Rider_LaunchIntoAir`
    /// (real, on a mostly empty rider; virtual calls through empty memory
    /// land in the fakes, some of which unwind), main state already set.
    #[test]
    fn rider_ground_state_post_update_launch_matches() {
        crate::difftest::check(
            ee::s01::f_0013f178__generated,
            rider_ground_state_post_update,
            96,
            &|c, seed| post_update_setup(c, seed, true),
        );
    }

    /// SND_DecEaxa_DecodeStream: an empty ADPCM decoder that is refilled
    /// from a real stream (`SNDSTRM_PopBlock` on a one-stream fixture whose
    /// channel queue holds 0-2 blocks of 28 or 56 samples), old and new
    /// codec versions (history seeding), requests satisfied mid-block and
    /// requests the stream cannot satisfy (cleared tail with and without
    /// unreleased samples), and a pending release on entry.
    #[test]
    fn snd_dec_eaxa_decode_stream_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{fill, DATA};
        const SELF: u32 = DATA;
        const DECODER: u32 = DATA + 0x200;
        const STREAM: u32 = DATA + 0x1000;
        const BLOCKS: u32 = DATA + 0x2000;
        const OUT: u32 = DATA + 0x8000;
        /// The stream table (`0x5157b8 + 0x304 + 4 * stream`).
        const STREAMS: u32 = 0x0051_57b8 + 0x304;
        crate::difftest::check(
            ee::s14::f_003ca460__generated,
            snd_dec_eaxa_decode_stream,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ca4_60, 0);
                c.pc = 0x003c_a460;
                c.bus.write32(STREAMS, STREAM);
                // The decoder: empty (no samples left, nothing buffered).
                for k in 0..0x30 {
                    c.bus.write32(DECODER + 4 * k, 0);
                }
                c.bus.write32(SELF + 0x1c, DECODER);
                c.bus.write32(SELF + 0x20, 0);
                c.bus.write32(SELF + 0x24, 0);
                c.bus.write32(SELF + 0x28, r.next_u32() % 100);
                c.bus.write32(SELF + 0x2c, r.next_u32() % 5);
                c.bus.write32(SELF + 0x30, 0);
                let pending = if seed % 3 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 64
                };
                c.bus.write32(SELF + 0x34, pending);
                c.bus.write8(SELF + 0x38, 0);
                // The stream: no extra channels, channel 0 is 0x42's, a ring of
                // four block descriptors.
                for k in 0..0x40 {
                    c.bus.write32(STREAM + 4 * k, 0);
                }
                c.bus
                    .write8(STREAM + 0x42, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write16(STREAM + 0x40, 4);
                c.bus.write16(STREAM + 0x44, (r.next_u32() % 4) as u16);
                let queued = seed % 3;
                c.bus.write16(STREAM + 0x34, queued as u16);
                c.bus.write16(STREAM + 0x28, 0);
                c.bus.write32(STREAM + 0x4c, 0x1000);
                for d in 0..4u32 {
                    let desc = STREAM + 0x7c + 0x20 * d;
                    let samples = if r.next_u32() % 2 == 0 { 28 } else { 56 };
                    c.bus
                        .write32(desc + 4, samples | (r.next_u32() & 0x8000_0000));
                    c.bus.write32(desc + 8, BLOCKS + 0x100 * d);
                }
                fill(c, &mut r, BLOCKS, 0x100);
                let n = match seed % 5 {
                    0 => 0,
                    1 => 10,
                    2 => 28,
                    _ => 20 + r.next_u32() % 100,
                };
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = n as u64;
                c.g[A3][0] = sx(OUT);
            },
        );
    }

    /// Irradiance_AddDirectionalScaled: random coefficients, colours,
    /// scales and directions (edge floats on every fourth seed), the
    /// game's SH constants written at their gp slots.
    #[test]
    fn irradiance_add_directional_scaled_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{fill, DATA, GAME_GP};
        const SH: u32 = DATA;
        const RGB: u32 = DATA + 0x200;
        crate::difftest::check(
            ee::s12::f_00389308__generated,
            irradiance_add_directional_scaled,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3893_08, 0);
                c.pc = 0x0038_9308;
                fill(c, &mut r, GAME_GP - 0x272c, 11);
                for k in 0..0x30 {
                    c.bus.write32(SH + 4 * k, r.f32(10.0));
                }
                for k in 0..3 {
                    c.bus.write32(RGB + 4 * k, r.f32(2.0));
                }
                c.g[A0][0] = sx(SH);
                c.g[A1][0] = sx(RGB);
                let edge = seed % 4 == 0;
                for f in 12..16 {
                    c.f[f] = if edge { r.edge_f32() } else { r.f32(1.5) };
                }
                if !edge {
                    for k in 0..11 {
                        c.bus.write32(GAME_GP - 0x272c + 4 * k, r.f32(1.0));
                    }
                }
            },
        );
    }

    /// SND_DecUtk_DecodeLoop with the real MicroTalk decoder (game image
    /// tables): requests met by the decoded frame alone, and requests that
    /// run the stream dry (LEFT below n, or 0) so the loop restarts
    /// (`SND_UtkSetState`, `SND_UtkSetInput` with a 20..320-sample loop and
    /// header parsing on random bits) once or several times; n <= 0.
    #[test]
    fn snd_dec_utk_decode_loop_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{elf_image, fill, DATA};
        use crate::game::lib::snd::utk::*;
        use crate::game::lib::snd::utk_frame as f;
        const SELF: u32 = DATA;
        const U: u32 = DATA + 0x1000;
        const INPUT: u32 = DATA + 0x8000;
        const OUT: u32 = DATA + 0x1_0000;
        crate::difftest::check(
            ee::s14::f_003c9520__generated,
            snd_dec_utk_decode_loop,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c95_20, 0);
                elf_image(c);
                c.pc = 0x003c_9520;
                fill(c, &mut r, INPUT, 0x800);
                // The decoder (as the SND_UtkDecode test builds it), mode 0.
                c.bus.write32(U + BIT_PTR, INPUT + r.next_u32() % 8);
                c.bus.write32(U + BIT_BUF, r.next_u32());
                c.bus.write32(U + BIT_COUNT, 8 + r.next_u32() % 25);
                c.bus
                    .write32(U + f::REDUCED_BW, if r.next_u32() % 2 == 0 { 0 } else { 1 });
                c.bus.write32(U + f::VOICED_THRESHOLD, r.next_u32() % 72);
                for i in 0..64 {
                    let v = r.f32(2.0);
                    c.bus.write32(U + f::FIXED_GAINS + 4 * i, v);
                }
                for i in 0..12 {
                    let v = r.f32(0.9);
                    c.bus.write32(U + f::RC + 4 * i, v);
                    let h = r.f32(100.0);
                    c.bus.write32(U + 0x144 + 4 * i, h);
                }
                for i in 0..324 {
                    let v = r.f32(1000.0);
                    c.bus.write32(U + f::ADAPT_CB + 4 * i, v);
                }
                fill(c, &mut r, U + DECODED, 432);
                c.bus.write32(U + MODE, 0);
                let avail = r.next_u32() % 433;
                c.bus.write32(U + AVAIL, avail);
                let n: i32 = match seed % 6 {
                    0 => -((r.next_u32() % 3) as i32),
                    1 => (r.next_u32() % 50) as i32 + 1,
                    _ => 50 + (r.next_u32() % 400) as i32,
                };
                let left = match seed % 4 {
                    0 => 0,
                    1 => avail.min(n.max(0) as u32 / 2),
                    _ => 2000,
                };
                c.bus.write32(U + LEFT, left);
                // The loop: samples 0..len-1 of the data.
                let start = r.next_u32() % 100;
                let len = 20 + r.next_u32() % 300;
                c.bus.write32(SELF + 0x1c, U);
                c.bus.write32(SELF + 0x20, r.next_u32() % 1000);
                c.bus.write32(SELF + 0x24, start);
                c.bus.write32(SELF + 0x28, start + len - 1);
                c.bus
                    .write32(SELF + 0x2c, INPUT + 0x400 + r.next_u32() % 16);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(n as u32);
                c.g[A3][0] = sx(OUT);
            },
        );
    }

    /// WPData_FindCellIndex on a real quadtree (QuadTree_FindLeaf): a root
    /// leaf, or a tree of up to three internal levels with random leaf
    /// cells; points inside the tree and outside it (negative or too large
    /// grid coordinates: no leaf).
    #[test]
    fn wp_data_find_cell_index_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::DATA;
        const WP: u32 = DATA;
        const TREE: u32 = DATA + 0x100;
        const NODES: u32 = DATA + 0x1000;
        crate::difftest::check(
            ee::s08::f_002baf90__generated,
            wp_data_find_cell_index,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2baf_90, 0);
                c.pc = 0x002b_af90;
                let f = |x: f32| x.to_bits();
                c.bus.write32(WP, TREE);
                // Grid: 0x8000 cells over 0..1024 world units.
                c.bus.write32(TREE, f(32.0));
                c.bus.write32(TREE + 4, f(0.0));
                c.bus.write32(TREE + 8, f(0.0));
                c.bus.write16(TREE + 0x14, 0);
                c.bus.write32(TREE + 0x20, NODES);
                // Nodes (8 bytes): internal = four child halfwords (index << 1,
                // bit 0 of the first set), leaf = even first halfword and its
                // cell index at +4.
                let depth = seed % 4;
                let mut next = 1u32;
                let mut level = vec![0u32];
                for d in 0..=depth {
                    let mut below = Vec::new();
                    for &n in &level {
                        let node = NODES + 8 * n;
                        if d < depth && r.next_u32() % 4 != 0 {
                            for k in 0..4u32 {
                                let child = next;
                                next += 1;
                                below.push(child);
                                c.bus
                                    .write16(node + 2 * k, (child << 1 | (k == 0) as u32) as u16);
                            }
                        } else {
                            c.bus.write16(node, (r.next_u32() as u16) & !1);
                            c.bus.write32(node + 4, r.next_u32() % 5000);
                        }
                    }
                    level = below;
                }
                let coord = |r: &mut crate::difftest::Rng| match r.next_u32() % 8 {
                    0 => -((r.next_u32() % 100) as f32) - 1.0,
                    1 => 1024.0 + (r.next_u32() % 100) as f32,
                    _ => (r.next_u32() % 1024) as f32 + 0.5,
                };
                c.f[12] = f(coord(&mut r));
                c.f[13] = f(coord(&mut r));
                c.g[A0][0] = sx(WP);
            },
        );
    }

    /// PS2GraphicsMan_v111: 0-4 blends of 0-4 influences (the first
    /// influence is always applied, even for a count of 0 or 1) over eight
    /// random bone matrices, into a given output buffer or the manager's
    /// default one (+0x13e8). LO/HI checked too.
    #[test]
    fn ps2_graphics_man_blend_bone_matrices_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check_lohi, DATA};
        const GM: u32 = DATA;
        const BONES: u32 = DATA + 0x2000;
        const BLENDS: u32 = DATA + 0x3000;
        const INFS: u32 = DATA + 0x3100;
        const OUT: u32 = DATA + 0x4000;
        const DEFAULT_OUT: u32 = DATA + 0x5000;
        check_lohi(
            ee::s12::f_00386bd0__generated,
            ps2_graphics_man_blend_bone_matrices,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x386b_d0, 0);
                c.pc = 0x0038_6bd0;
                c.bus.write32(GM + 0x13e8, DEFAULT_OUT);
                for k in 0..0x80 {
                    c.bus.write32(BONES + 4 * k, r.f32(4.0));
                }
                let count = seed % 5;
                for b in 0..4u32 {
                    let n = r.next_u32() % 5;
                    c.bus.write32(BLENDS + 12 * b, n);
                    c.bus.write32(BLENDS + 12 * b + 4, INFS + 0x20 * b);
                    for i in 0..5u32 {
                        let inf = INFS + 0x20 * b + 4 * i;
                        c.bus.write16(inf, (r.next_u32() % 201) as u16);
                        c.bus.write8(inf + 2, (r.next_u32() % 8) as u8);
                        c.bus.write8(inf + 3, r.next_u32() as u8);
                    }
                }
                c.g[A0][0] = sx(GM);
                c.g[A1][0] = sx(BONES);
                c.g[A3][0] = sx(BLENDS);
                c.g[T0][0] = if seed % 11 == 0 {
                    sx(0xffff_fffe)
                } else {
                    count as u64
                };
                c.g[T1][0] = if seed % 3 == 0 { 0 } else { sx(OUT) };
            },
        );
    }
}
