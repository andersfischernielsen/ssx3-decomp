//! Staging wave 208 (oc266): hand-written replacements for the hottest
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

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_b590, computer_actor_rider_think_natural_air, "ComputerActorRider_ThinkNaturalAir"),
    (0x0032_f760, coll_hull_overlaps_aabb, "CollHull_OverlapsAABB"),
    (0x0013_7860, rider_wipeout_state_post_update_air, "RiderWipeoutState_PostUpdateAir"),
    (0x002b_b6b8, c_world_painter_man_debug_draw, "cWorldPainterMan_DebugDraw"),
    (0x0031_4588, anim_seq_list_push_looping_anim, "AnimSeqList_PushLoopingAnim"),
    (0x0013_5be0, rider_spin_sub_try_reverse, "RiderSpinSub_TryReverse"),
    (0x0022_57e0, c_luno_table_write, "cLunoTable_Write"),
    (0x0037_2500, vfx_tube_update, "VfxTube_Update"),
    (0x0030_a060, wscript_man_on_object_trigger, "WScriptMan_OnObjectTrigger"),
];

// ---------------------------------------------------------------------------
// ComputerActorRider_ThinkNaturalAir (0x10b590)

/// `Rand_NextGame` (the AI's random generator, called through the table).
const NATURAL_AIR_RAND: u32 = 0x0031_7810;
const NATURAL_AIR_GET_STEER_TARGET: Callee = Callee {
    addr: 0x0010_0610,
    f: ee::s00::ComputerActorRider_GetSteerTarget,
};
const NATURAL_AIR_COMPUTE_STEER: Callee = Callee {
    addr: 0x0010_0348,
    f: ee::s00::ComputerActorRider_ComputeSteer,
};

/// AI think state (the `ComputerActorRider`).
const AI_RIDER: u32 = 0x18;
const AI_TRICK: u32 = 0xe0c;
const AI_IN_TRICK: u32 = 0xe10;
const AI_E14: u32 = 0xe14;
const AI_E18: u32 = 0xe18;
const AI_E20: u32 = 0xe20;
const AI_SPIN_DIR: u32 = 0xe38;
const AI_FLIP_DIR: u32 = 0xe3c;
/// Rider -> air predictor; its state and the height above the landing.
const RIDER_AIR: u32 = 0x788;
const AIR_HEIGHT: u32 = 0x98;
const AIR_GROUND: u32 = 0xa0;
const AIR_STATE: u32 = 0xac;

/// `void ComputerActorRider_ThinkNaturalAir(ComputerActorRider *self, Pad *pad)`
/// (0x10b590): the AI's "natural" air (no trick planned). With the air
/// predictor in state 1 or 3 the height above the landing decides: above 1
/// unit it spins (random direction), above 3 it also flips; below 0.5 with
/// neither, it steers toward the path target (`ComputeSteer * -31` into the
/// pad's stick bits 24..29). The trick slot (`self+0xe0c`, -1) goes to pad
/// byte 2.
pub fn computer_actor_rider_think_natural_air(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        c.g[V1][0] = 1;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x40);
        let ai = r32(c, S0);
        let rider = Addr(ai).off(AI_RIDER).lw(c);
        c.g[V0][0] = rider;
        let air = Addr(rider as u32).off(RIDER_AIR).lw(c);
        c.g[A0][0] = air;
        let state = Addr(air as u32).off(AIR_STATE).lw(c);
        c.g[A1][0] = state;
        // State 1 or 3: the predictor has a landing.
        let landing = if state == 1 {
            c.cycles += 1;
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 3;
            if state == 3 {
                c.cycles += 1;
            }
            state == 3
        };
        c.g[A2][0] = landing as u64;

        c.cycles += 2;
        let height = if landing {
            c.cycles += 4;
            let ground = Addr(air as u32).off(AIR_GROUND).read_fpu(c);
            let h = Addr(air as u32).off(AIR_HEIGHT).read_fpu(c);
            c.f[1] = ground.0;
            c.f[0] = h.0;
            h - ground
        } else {
            c.cycles += 1;
            FpuF32::ZERO
        };
        c.f[20] = height.0;

        // Defaults: no trick, no spin/flip.
        c.cycles += 11;
        let ai = Addr(ai);
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = FpuF32::ONE.0;
        c.g[V1][0] = 1;
        c.g[V0][0] = u64::MAX;
        ai.off(AI_IN_TRICK).sw(c, 0);
        let spin = abi::c_lt_s(c, FpuF32::ONE, height);
        ai.off(AI_TRICK).sw(c, u64::MAX);
        ai.off(AI_E14).sw(c, 0);
        ai.off(AI_E18).sw(c, 1);
        ai.off(AI_E20).sw(c, 0);
        if spin {
            c.cycles += 6;
            ai.off(AI_IN_TRICK).sw(c, 1);
            ai.off(AI_SPIN_DIR).write_fpu(c, FpuF32::ONE);
            call_ptr(c, NATURAL_AIR_RAND, 0x0010_b628)?;
            natural_air_random_sign(c, AI_SPIN_DIR);
        } else {
            c.cycles += 1;
            Addr(r32(c, S0)).off(AI_SPIN_DIR).sw(c, 0);
        }

        c.cycles += 6;
        let three = FpuF32::from_f32(3.0);
        c.g[AT][0] = 0x4040_0000;
        c.f[0] = three.0;
        let flip = abi::c_lt_s(c, three, height);
        c.g[V0][0] = 1;
        if flip {
            c.cycles += 8;
            c.g[AT][0] = 0x3f80_0000;
            c.f[0] = FpuF32::ONE.0;
            let ai = Addr(r32(c, S0));
            ai.off(AI_IN_TRICK).sw(c, 1);
            ai.off(AI_FLIP_DIR).write_fpu(c, FpuF32::ONE);
            call_ptr(c, NATURAL_AIR_RAND, 0x0010_b674)?;
            natural_air_random_sign(c, AI_FLIP_DIR);
        } else {
            c.cycles += 1;
            Addr(r32(c, S0)).off(AI_FLIP_DIR).sw(c, 0);
        }

        // Low and not tricking: steer toward the path.
        c.cycles += 6;
        let half = FpuF32::from_f32(0.5);
        c.g[AT][0] = 0x3f00_0000;
        c.f[0] = half.0;
        if abi::c_lt_s(c, height, half) {
            c.cycles += 6;
            let flip_dir = Addr(r32(c, S0)).off(AI_FLIP_DIR).read_fpu(c);
            c.f[0] = flip_dir.0;
            c.f[1] = 0;
            let still = flip_dir.eq_s(FpuF32::ZERO);
            abi::set_fcc(c, still);
            if still {
                c.cycles += 5;
                let spin_dir = Addr(r32(c, S0)).off(AI_SPIN_DIR).read_fpu(c);
                c.f[0] = spin_dir.0;
                let still = spin_dir.eq_s(FpuF32::ZERO);
                abi::set_fcc(c, still);
                c.g[A0][0] = c.g[SP][0];
                if still {
                    c.cycles += 21;
                    c.g[A1][0] = c.g[S0][0];
                    NATURAL_AIR_GET_STEER_TARGET.call(c, 0x0010_b6e0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[SP][0];
                    NATURAL_AIR_COMPUTE_STEER.call(c, 0x0010_b6ec)?;
                    let steer = -FpuF32(c.f[0]);
                    let k = FpuF32::from_f32(31.0);
                    c.g[AT][0] = 0x41f8_0000;
                    c.f[1] = k.0;
                    let pad = Addr(r32(c, S1));
                    let word = pad.lw(c);
                    c.g[V1][0] = 0xffff_ffff_c0ff_ffff;
                    c.g[A1][0] = u64::MAX;
                    let stick = steer * k;
                    c.f[0] = stick.0;
                    let bits = stick.to_i32() as u32;
                    c.f[1] = bits;
                    let field = sx((bits & 0x3f) << 24);
                    c.g[V0][0] = field;
                    let word = (word & 0xffff_ffff_c0ff_ffff) | field;
                    c.g[A0][0] = word;
                    pad.sw(c, word);
                    Addr(r32(c, S0)).off(AI_TRICK).sw(c, u64::MAX);
                }
                c.cycles += 1;
            }
        }
        c.g[V0][0] = Addr(r32(c, S0)).off(AI_TRICK).lbu(c);

        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        let trick = c.g[V0][0];
        Addr(r32(c, S1)).off(2).sb(c, trick);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// After `Rand_NextGame`: an odd result negates the direction at `field`.
fn natural_air_random_sign(c: &mut Cpu, field: u32) {
    c.g[V0][0] &= 1;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        let dir = Addr(r32(c, S0)).off(field);
        let v = -dir.read_fpu(c);
        c.f[0] = v.0;
        dir.write_fpu(c, v);
    }
}

// ---------------------------------------------------------------------------
// CollHull_OverlapsAABB (0x32f760)

/// Collision hull -> its bounding sphere (`{center: vec4 @0x10, radius @0x20}`).
const HULL_SPHERE: u32 = 0x60;
const HULL_SPHERE_CENTER: u32 = 0x10;
const HULL_SPHERE_RADIUS: u32 = 0x20;

/// `c.le.s a, b` with the condition bit latched.
fn hull_le(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let le = a.le(b);
    abi::set_fcc(c, le);
    le
}

/// `bool CollHull_OverlapsAABB(CollHull *hull, const vec4 *min, const vec4 *max)`
/// (0x32f760): whether the box around the hull's bounding sphere
/// (`center -+ radius` on x, y, z) overlaps the box `[min, max]`. The box
/// corners are built on the stack with VU0 (`vsub`/`vadd`), then x, y and z
/// are compared in turn, stopping at the first separated axis.
pub fn coll_hull_overlaps_aabb(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::Src;
    c.cycles += 26;
    let hull = Addr(abi::arg(c, 0));
    let min = Addr(abi::arg(c, 1));
    let max = Addr(abi::arg(c, 2));
    let sphere = hull.off(HULL_SPHERE).lw(c);
    c.g[V1][0] = sphere;
    let frame = Frame::open(c, 0x40);
    c.g[A3][0] = 0;
    let radius = Addr(sphere as u32).off(HULL_SPHERE_RADIUS).read_fpu(c);
    c.f[0] = radius.0;
    // (r, r, r, 0) at sp.
    let extent = frame.at(c, 0);
    Addr(extent).off(0xc).sw(c, 0);
    Addr(extent).off(8).write_fpu(c, radius);
    Addr(extent).write_fpu(c, radius);
    Addr(extent).off(4).write_fpu(c, radius);
    // Low corner: center - extent (to sp+0x20, copied to sp+0x10).
    let lo = frame.at(c, 0x10);
    let hi = frame.at(c, 0x20);
    let tmp_hi = frame.at(c, 0x30);
    c.lqc2(3, (sphere as u32).wrapping_add(HULL_SPHERE_CENTER));
    c.lqc2(4, extent);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, hi);
    c.lq(V0, hi);
    let sphere = hull.off(HULL_SPHERE).lw(c);
    c.g[V1][0] = sphere;
    c.sq(V0, lo);
    // High corner: center + extent (to sp+0x30, copied to sp+0x20).
    c.lqc2(3, (sphere as u32).wrapping_add(HULL_SPHERE_CENTER));
    c.lqc2(4, extent);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, tmp_hi);
    let max_x = max.read_fpu(c);
    c.f[1] = max_x.0;
    let lo_x = Addr(lo).read_fpu(c);
    c.f[0] = lo_x.0;
    c.lq(V0, tmp_hi);
    let mut overlap = hull_le(c, lo_x, max_x);
    c.sq(V0, hi);

    // Per axis: lo <= max, then min <= hi (the next value loads in the
    // branch delay slot, also when the test fails).
    if overlap {
        for axis in 0..3u32 {
            if axis > 0 {
                c.cycles += 5;
                let lo_a = FpuF32(c.f[0]);
                let max_a = max.off(4 * axis).read_fpu(c);
                c.f[1] = max_a.0;
                overlap = hull_le(c, lo_a, max_a);
                let hi_a = Addr(hi).off(4 * axis).read_fpu(c);
                c.f[0] = hi_a.0;
                if !overlap {
                    break;
                }
            } else {
                c.cycles += 1;
            }
            c.cycles += 5;
            let min_a = min.off(4 * axis).read_fpu(c);
            c.f[1] = min_a.0;
            let hi_a = if axis == 0 {
                let h = Addr(hi).read_fpu(c);
                c.f[0] = h.0;
                h
            } else {
                FpuF32(c.f[0])
            };
            overlap = hull_le(c, min_a, hi_a);
            if axis < 2 {
                let lo_next = Addr(lo).off(4 * (axis + 1)).read_fpu(c);
                c.f[0] = lo_next.0;
            }
            if !overlap {
                break;
            }
        }
    }
    if overlap {
        c.g[A3][0] = 1;
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[A3][0];
    frame.close(c);
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_PostUpdateAir (0x137860)

/// The wipeout state's rider.
const PUA_STATE_RIDER: u32 = 0x40;
/// Rider fields.
const PUA_R_POS: u32 = 0x110;
const PUA_R_VEL: u32 = 0x1e0;
const PUA_R_GROUND_NORMAL: u32 = 0x370;
const PUA_R_3D0: u32 = 0x3d0;
const PUA_R_SURFACE: u32 = 0x438;
const PUA_R_GROUND_HEIGHT: u32 = 0x454;
const PUA_R_CONTACT_POINT: u32 = 0x460;
const PUA_R_VTABLE: u32 = 0x6c0;
const PUA_R_HANDLER: u32 = 0x77c;
const PUA_R_AIR: u32 = 0x788;
const PUA_R_OVERLAP: u32 = 0x860;
const PUA_R_WALL_HINT: u32 = 0x868;
const PUA_R_SPHERE_TREE: u32 = 0xaa0;
/// Rider handler: "landed" flag and landing velocity.
const PUA_H_LANDED: u32 = 0x314;
const PUA_H_LAND_VEL: u32 = 0x320;
/// Stack frame: hit A (the sphere's closest contact; its normal at +0x10),
/// the collision hull, hit B (the ground ray: position, normal, +0x20,
/// surface at +0x4c, object at +0x50) and three vector temporaries.
const PUA_HIT_A_NORMAL: u32 = 0x10;
const PUA_HULL: u32 = 0x80;
const PUA_HIT_B: u32 = 0x3a0;
const PUA_HIT_B_NORMAL: u32 = 0x3b0;
const PUA_HIT_B_20: u32 = 0x3c0;
const PUA_HIT_B_SURFACE: u32 = 0x3ec;
const PUA_HIT_B_OBJECT: u32 = 0x3f0;
const PUA_T420: u32 = 0x420;
const PUA_T430: u32 = 0x430;
const PUA_T440: u32 = 0x440;
const PUA_T450: u32 = 0x450;

const PUA_CAST_GROUND_RAY: Callee = Callee {
    addr: 0x0013_8960,
    f: ee::s01::RiderWipeoutState_CastGroundRay,
};
const PUA_COLL_HULL_CTOR: u32 = 0x0032_f650;
const PUA_FIND_CLOSEST_CONTACT: u32 = 0x0033_42d0;
const PUA_CLEAR_AIR_TIME: u32 = 0x0011_3998;
const PUA_TRIGGER_SPRAY: u32 = 0x0011_1aa0;
const PUA_TRANSLATE: u32 = 0x0010_6538;
const PUA_AIR_PRIME: u32 = 0x0011_35b8;
const PUA_UPDATE_BOUNDS: u32 = 0x0011_e150;
const PUA_WALL_COLLISION: u32 = 0x0010_5398;
const PUA_RIDER_COLLISIONS: u32 = 0x0010_7888;
const PUA_COLL_HULL_DTOR: u32 = 0x0032_f708;

/// The game's VU0 dot-product macro sequence: `vf3 = *a`, `vf5 = *b`,
/// x*x + y*y + z*z into vf4.w, moved to GPR `gpr` (`qmfc2`).
fn pua_dot(c: &mut Cpu, a: u32, b: u32, gpr: usize) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(gpr, 4);
}

/// `*dst = *v * f[fs]` through VU0 (`mfc1 gpr; qmtc2 gpr, vf3; vmul.xyzw
/// vf5, vf4, vf3x`).
fn pua_scale(c: &mut Cpu, v: u32, fs: usize, gpr: usize, dst: u32) {
    use ssx3_ee_rt::vu0::{Src, X};
    c.mfc1(gpr, fs);
    c.lqc2(4, v);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, dst);
}

/// `*dst = *a - *b` (`vsub`) or `*a + *b` (`vadd`) through VU0.
fn pua_add_sub(c: &mut Cpu, a: u32, b: u32, dst: u32, sub: bool) {
    use ssx3_ee_rt::vu0::Src;
    c.lqc2(3, a);
    c.lqc2(4, b);
    if sub {
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    } else {
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    }
    c.sqc2(5, dst);
}

/// `self->rider` (read from s2 each time, as the original does).
fn pua_rider(c: &mut Cpu) -> u64 {
    Addr(r32(c, S2)).off(PUA_STATE_RIDER).lw(c)
}

/// `void RiderWipeoutState_PostUpdateAir(RiderWipeoutState *self)`
/// (0x137860): after a wipeout tumble step in the air, test the rider's
/// collision sphere against the world. On a contact, cast a ground ray
/// along the contact normal: a ground hit (ray distance >= 0.4) the rider
/// moves into (`vel . n <= 0`) within 10 units lands him (air time cleared,
/// dynamic spray, the handler's landed flag and landing velocity, ground
/// normal / surface / contact point / height, the normal velocity removed,
/// the hit object pushed by the impulse scaled by the world's int at +0x10,
/// pushed out of the ground, and `self->state = 0`). Otherwise the rider is
/// pushed out of the contact, loses the velocity into it, and the air
/// predictor is re-primed. Then always: bounds, wall and rider collisions,
/// and the hull is destroyed.
pub fn rider_wipeout_state_post_update_air(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x4d0);
        let at = |c: &Cpu, o: u32| sp(c).wrapping_add(o);
        frame.save(c, S1, 0x490);
        frame.save(c, S2, 0x480);
        c.g[S1][0] = sx(at(c, PUA_HULL));
        frame.save(c, S3, 0x470);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 23, 0x4c8);
        c.g[S3][0] = c.g[S1][0];
        frame.save_fpr(c, 21, 0x4b8);
        frame.save_ra(c, 0x460);
        frame.save_fpr(c, 22, 0x4c0);
        frame.save_fpr(c, 20, 0x4b0);
        frame.save(c, S0, 0x4a0);
        c.g[S0][0] = pua_rider(c);
        c.mtc1(0, 23);
        // is_ai = rider->vtable->IsHuman() ^ 1 (slot 0x40).
        let rider = r32(c, S0);
        c.g[V1][0] = Addr(rider).off(PUA_R_VTABLE).lw(c);
        c.g[A1][0] = sx(rider.wrapping_add(PUA_R_VTABLE));
        c.g[A0][0] = Addr(r32(c, V1)).off(0x40).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x44).lw(c);
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0013_78b4)?;
        // CollHull hull(rider->sphere_tree, !human)
        c.g[A1][0] = Addr(r32(c, S0)).off(PUA_R_SPHERE_TREE).lw(c);
        c.g[A2][0] = c.g[V0][0] ^ 1;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, PUA_COLL_HULL_CTOR, 0x0013_78c4)?;
        // d = FindClosestContact(rider->overlap, &hull, &hit_a, wall_hint)
        c.g[V1][0] = pua_rider(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = Addr(r32(c, V1)).off(PUA_R_WALL_HINT).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(PUA_R_OVERLAP).lw(c);
        call_ptr(c, PUA_FIND_CLOSEST_CONTACT, 0x0013_78dc)?;
        c.mov_s(21, 0);
        c.c_le_s(23, 21);
        let contact = c.fcc();
        c.g[S0][0] = sx(at(c, PUA_HIT_B));
        if contact {
            pua_contact(c)?;
        }

        c.cycles += 22;
        c.g[A0][0] = pua_rider(c);
        c.g[A1][0] = 0;
        call_ptr(c, PUA_UPDATE_BOUNDS, 0x0013_7cc8)?;
        c.g[A0][0] = pua_rider(c);
        c.g[A1][0] = 0;
        call_ptr(c, PUA_WALL_COLLISION, 0x0013_7cd4)?;
        c.g[A0][0] = pua_rider(c);
        call_ptr(c, PUA_RIDER_COLLISIONS, 0x0013_7cdc)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 2;
        call_ptr(c, PUA_COLL_HULL_DTOR, 0x0013_7ce8)?;
        frame.restore(c, S0, 0x4a0);
        frame.restore(c, S1, 0x490);
        frame.restore(c, S2, 0x480);
        frame.restore(c, S3, 0x470);
        frame.restore_ra(c, 0x460);
        frame.restore_fpr(c, 23, 0x4c8);
        frame.restore_fpr(c, 22, 0x4c0);
        frame.restore_fpr(c, 21, 0x4b8);
        frame.restore_fpr(c, 20, 0x4b0);
        frame.close(c);
        Ok(())
    })
}

/// The sphere touched something (f21 = its distance): cast the ground ray
/// and land, or push out of the contact and re-prime the air predictor.
fn pua_contact(c: &mut Cpu) -> crate::call::Flow {
    let s = sp(c);
    // t = CastGroundRay(self, &hit_b, &hit_a.normal)
    c.cycles += 11;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = sx(s + PUA_HIT_A_NORMAL);
    PUA_CAST_GROUND_RAY.call(c, 0x0013_7900)?;
    c.mov_s(22, 23);
    c.mov_s(20, 0);
    c.lwc1(0, r32(c, GP).wrapping_sub(0x7268)); // 0.4
    c.c_le_s(0, 20);
    let ground = c.fcc();
    c.mov_s(1, 22);
    if ground {
        // f22 = vel . n, f1 = (pos - hit) . n
        c.cycles += 26;
        c.g[V0][0] = pua_rider(c);
        let rider = r32(c, V0);
        pua_dot(c, rider.wrapping_add(PUA_R_VEL), s + PUA_HIT_B_NORMAL, V1);
        c.mtc1(V1, 22);
        pua_add_sub(c, rider.wrapping_add(PUA_R_POS), s + PUA_HIT_B, s + PUA_T430, true);
        c.lq(V1, s + PUA_T430);
        c.sq(V1, s + PUA_T420);
        pua_dot(c, s + PUA_T420, s + PUA_HIT_B_NORMAL, V0);
        c.mtc1(V0, 1);
        c.c_le_s(0, 20);
    }

    c.cycles += 2;
    let mut land = c.fcc();
    if land {
        c.cycles += 3;
        c.c_le_s(22, 23);
        land = c.fcc();
    }
    if land {
        c.cycles += 5;
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(AT, 0); // 10.0
        c.c_le_s(1, 0);
        land = c.fcc();
    }
    if land {
        pua_land(c)?;
        return Ok(());
    }

    // Push out along hit_a's normal by d * k.
    c.cycles += 23;
    c.lwc1(0, r32(c, GP).wrapping_sub(0x7264));
    c.g[A1][0] = sx(s + PUA_T420);
    pua_scale(c, s + PUA_HIT_A_NORMAL, 0, V1, s + PUA_T440);
    c.lq(V0, s + PUA_T440);
    c.sq(V0, s + PUA_T430);
    pua_scale(c, s + PUA_T430, 21, V1, s + PUA_T440);
    c.lq(V0, s + PUA_T440);
    c.g[A0][0] = pua_rider(c);
    c.sq(V0, s + PUA_T420);
    call_ptr(c, PUA_TRANSLATE, 0x0013_7bf0)?;
    c.mtc1(0, 2);
    c.c_lt_s(20, 2);
    // Kill the velocity into hit_a's normal unless the ray hit something
    // facing it (n_b . n_a >= k).
    let mut kill = c.fcc();
    if !kill {
        c.cycles += 14;
        c.lwc1(0, r32(c, GP).wrapping_sub(0x7260));
        pua_dot(c, s + PUA_HIT_B_NORMAL, s + PUA_HIT_A_NORMAL, V0);
        c.mtc1(V0, 1);
        c.c_lt_s(1, 0);
        kill = c.fcc();
        if kill {
            c.cycles += 1;
        }
    }
    if kill {
        c.g[V0][0] = pua_rider(c);
        c.cycles += 13;
        let rider = r32(c, V0);
        pua_dot(c, s + PUA_HIT_A_NORMAL, rider.wrapping_add(PUA_R_VEL), V1);
        c.mtc1(V1, 0);
        c.c_lt_s(0, 2);
        if c.fcc() {
            // vel -= n_a * (vel . n_a)
            c.cycles += 13;
            pua_scale(c, s + PUA_HIT_A_NORMAL, 0, V0, s + PUA_T440);
            c.lq(V1, s + PUA_T440);
            c.g[V0][0] = pua_rider(c);
            c.sq(V1, s + PUA_T430);
            let rider = r32(c, V0);
            pua_add_sub(
                c,
                rider.wrapping_add(PUA_R_VEL),
                s + PUA_T430,
                rider.wrapping_add(PUA_R_VEL),
                true,
            );
        }
    }
    c.g[A1][0] = pua_rider(c);

    // AirPredictor_Prime(rider->air, &rider->pos, &rider->vel, k)
    c.cycles += 5;
    c.lwc1(12, r32(c, GP).wrapping_sub(0x725c));
    let rider = r32(c, A1);
    c.g[A0][0] = Addr(rider).off(PUA_R_AIR).lw(c);
    c.g[A2][0] = sx(rider.wrapping_add(PUA_R_VEL));
    c.g[A1][0] = sx(rider.wrapping_add(PUA_R_POS));
    call_ptr(c, PUA_AIR_PRIME, 0x0013_7cbc)?;
    Ok(())
}

/// Landing on hit_b (f22 = vel . n): see the parent's doc.
fn pua_land(c: &mut Cpu) -> crate::call::Flow {
    let s = sp(c);
    c.cycles += 65;
    // AirPredictor_ClearAirTime(rider->air, &rider->pos, &rider->vel)
    c.g[A1][0] = pua_rider(c);
    let rider = r32(c, A1);
    c.g[A0][0] = Addr(rider).off(PUA_R_AIR).lw(c);
    c.g[A2][0] = sx(rider.wrapping_add(PUA_R_VEL));
    c.g[A1][0] = sx(rider.wrapping_add(PUA_R_POS));
    call_ptr(c, PUA_CLEAR_AIR_TIME, 0x0013_79c0)?;
    // RiderBase_TriggerDynamicSpray(handler, &hit_b, &hit_b.normal, surface, vn)
    c.g[V0][0] = pua_rider(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A3][0] = Addr(s + PUA_HIT_B_SURFACE).lw(c);
    c.g[A2][0] = sx(s + PUA_HIT_B_NORMAL);
    c.g[A0][0] = Addr(r32(c, V0)).off(PUA_R_HANDLER).lw(c);
    c.mov_s(12, 22);
    call_ptr(c, PUA_TRIGGER_SPRAY, 0x0013_79dc)?;
    // handler->landed = 1, handler->land_vel = rider->vel
    c.g[A1][0] = pua_rider(c);
    c.g[A0][0] = 1;
    c.neg_s(1, 22);
    let rider = r32(c, A1);
    c.g[V1][0] = Addr(rider).off(PUA_R_HANDLER).lw(c);
    Addr(r32(c, V1)).off(PUA_H_LANDED).sw(c, 1);
    c.lq(V0, rider.wrapping_add(PUA_R_VEL));
    c.sq(V0, r32(c, V1).wrapping_add(PUA_H_LAND_VEL));
    // Ground state from hit_b.
    c.g[A0][0] = pua_rider(c);
    c.lq(V0, s + PUA_HIT_B_20);
    c.sq(V0, r32(c, A0).wrapping_add(PUA_R_3D0));
    c.g[V1][0] = pua_rider(c);
    c.lq(V0, s + PUA_HIT_B_NORMAL);
    c.sq(V0, r32(c, V1).wrapping_add(PUA_R_GROUND_NORMAL));
    c.g[A0][0] = pua_rider(c);
    c.g[V0][0] = Addr(s + PUA_HIT_B_SURFACE).lw(c);
    let surface = c.g[V0][0];
    Addr(r32(c, A0)).off(PUA_R_SURFACE).sw(c, surface);
    c.g[A1][0] = pua_rider(c);
    c.lq(V0, s + PUA_HIT_B);
    c.sq(V0, r32(c, A1).wrapping_add(PUA_R_CONTACT_POINT));
    // rider->ground_height = (pos - hit) . ground_normal
    c.g[V1][0] = pua_rider(c);
    let rider = r32(c, V1);
    pua_add_sub(c, rider.wrapping_add(PUA_R_POS), s + PUA_HIT_B, s + PUA_T430, true);
    c.lq(V0, s + PUA_T430);
    c.g[V1][0] = pua_rider(c);
    c.sq(V0, s + PUA_T420);
    let rider = r32(c, V1);
    pua_dot(c, s + PUA_T420, rider.wrapping_add(PUA_R_GROUND_NORMAL), A0);
    c.mtc1(A0, 0);
    c.swc1(0, r32(c, V1).wrapping_add(PUA_R_GROUND_HEIGHT));
    // impulse = ground_normal * -vn; vel += impulse
    c.g[V0][0] = pua_rider(c);
    let rider = r32(c, V0);
    pua_scale(c, rider.wrapping_add(PUA_R_GROUND_NORMAL), 1, A0, s + PUA_T430);
    c.lq(V1, s + PUA_T430);
    c.g[V0][0] = pua_rider(c);
    c.sq(V1, s + PUA_T420);
    let rider = r32(c, V0);
    pua_add_sub(
        c,
        rider.wrapping_add(PUA_R_VEL),
        s + PUA_T420,
        rider.wrapping_add(PUA_R_VEL),
        false,
    );
    // The hit object's owner feels -impulse * world int (vtable slot 0x158).
    c.g[A0][0] = Addr(s + PUA_HIT_B_OBJECT).lw(c);
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.g[T0][0] = Addr(r32(c, A0)).off(0xc).lw(c);
        c.g[A1][0] = c.g[S0][0];
        if c.g[T0][0] != 0 {
            c.cycles += 26;
            let owner = r32(c, T0);
            c.g[A3][0] = Addr(owner).off(0xc).lw(c);
            let vt = r32(c, A3);
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 0); // -1.0
            c.g[A2][0] = sx(s + PUA_T430);
            c.g[A0][0] = Addr(vt).off(0x158).lh(c);
            pua_scale(c, s + PUA_T420, 0, V1, s + PUA_T450);
            c.lq(V0, s + PUA_T450);
            c.g[A0][0] = sx(owner.wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
            c.sq(V0, s + PUA_T440);
            c.lwc1(0, r32(c, V1).wrapping_add(0x10));
            c.cvt_s_w(0, 0);
            pua_scale(c, s + PUA_T440, 0, T0, s + PUA_T450);
            c.lq(V0, s + PUA_T450);
            c.sq(V0, s + PUA_T430);
            c.g[V1][0] = Addr(vt).off(0x15c).lw(c);
            let method = r32(c, V1);
            call_ptr(c, method, 0x0013_7b24)?;
        }
        c.cycles += 1;
    }
    c.g[V0][0] = pua_rider(c);

    // Push out of the ground: pen = (pos - hit) . n; if < 0, translate by -pen * n.
    c.cycles += 20;
    let rider = r32(c, V0);
    pua_add_sub(c, rider.wrapping_add(PUA_R_POS), s + PUA_HIT_B, s + PUA_T440, true);
    c.mtc1(0, 0);
    c.lq(V0, s + PUA_T440);
    c.sq(V0, s + PUA_T430);
    pua_dot(c, s + PUA_T430, s + PUA_HIT_B_NORMAL, V1);
    c.mtc1(V1, 1);
    c.c_lt_s(1, 0);
    let below = c.fcc();
    c.g[A1][0] = sx(s + PUA_T430);
    if below {
        c.cycles += 10;
        c.neg_s(0, 1);
        pua_scale(c, s + PUA_HIT_B_NORMAL, 0, V1, s + PUA_T440);
        c.lq(V0, s + PUA_T440);
        c.g[A0][0] = pua_rider(c);
        c.sq(V0, s + PUA_T430);
        call_ptr(c, PUA_TRANSLATE, 0x0013_7ba0)?;
    }
    c.cycles += 2;
    Addr(r32(c, S2)).sw(c, 0);
    Ok(())
}

// ---------------------------------------------------------------------------
// cWorldPainterMan_DebugDraw (0x2bb6b8)

const WPM_DEBUG_DRAW_CURRENT: Callee = Callee {
    addr: 0x002b_b6e0,
    f: ee::s08::cWorldPainterMan_DebugDrawCurrent,
};
/// World (`gp-0x848`) -> world painter manager (+0x84) -> current painter
/// (+0x10); the painter's +8 is the node set drawn.
const WPM_WORLD: i32 = -0x848;
const WORLD_PAINTER_MAN: u32 = 0x84;
const WPM_CURRENT: u32 = 0x10;
const PAINTER_NODES: u32 = 0x8;

/// `void cWorldPainterMan_DebugDraw(void)` (0x2bb6b8): debug-draw the
/// world's current world painter
/// (`DebugDrawCurrent(painter->nodes, painter)`).
pub fn c_world_painter_man_debug_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(WPM_WORLD as u32)).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, V0)).off(WORLD_PAINTER_MAN).lw(c);
        c.g[A1][0] = Addr(r32(c, V1)).off(WPM_CURRENT).lw(c);
        c.g[A0][0] = Addr(r32(c, A1)).off(PAINTER_NODES).lw(c);
        WPM_DEBUG_DRAW_CURRENT.call(c, 0x002b_b6d4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimSeqList_PushLoopingAnim (0x314588)

const ASL_PUSH_NEW: Callee = Callee {
    addr: 0x0031_42f8,
    f: ee::s10::AnimSeqList_PushNew,
};
const ASL_SET_TRACK_ANIM: Callee = Callee {
    addr: 0x0031_3c50,
    f: ee::s10::AnimSequence_SetTrackAnim,
};
const ASL_SET_TRACK_TIME: Callee = Callee {
    addr: 0x0031_3cf0,
    f: ee::s10::AnimSequence_SetTrackTime,
};
const ASL_SET_TRACK_LOOP: Callee = Callee {
    addr: 0x0031_3d40,
    f: ee::s10::AnimSequence_SetTrackLoop,
};

/// `AnimSequence *AnimSeqList_PushLoopingAnim(AnimSeqList *list, int arg,
/// int anim, float time)` (0x314588): push a new sequence (`PushNew(list,
/// arg)`) whose track 0 plays `anim` from `time`, looping. Returns the
/// sequence.
pub fn anim_seq_list_push_looping_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x10);
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A2][0]; // anim
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.mov_s(20, 12); // time
        ASL_PUSH_NEW.call(c, 0x0031_45a8)?;
        c.g[S0][0] = c.g[V0][0]; // the new sequence
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        ASL_SET_TRACK_ANIM.call(c, 0x0031_45bc)?;
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        ASL_SET_TRACK_TIME.call(c, 0x0031_45cc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 1;
        ASL_SET_TRACK_LOOP.call(c, 0x0031_45dc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderSpinSub_TryReverse (0x135be0)

/// `Rider_CanReverseSpin`-style query on the rider (0x114db8).
const SPIN_CAN_REVERSE: u32 = 0x0011_4db8;
/// `cRiderAnimBase::play(anim, id, flags, blend)` (0x3128e8).
const SPIN_ANIM_PLAY: u32 = 0x0031_28e8;
/// Spin sub-state fields.
const SPIN_SUB_10: u32 = 0x10;
const SPIN_SUB_18: u32 = 0x18;
const SPIN_SUB_ANGLE: u32 = 0x20;
const SPIN_SUB_RATE: u32 = 0x24;
const SPIN_SUB_2C: u32 = 0x2c;
const SPIN_SUB_RIDER: u32 = 0x58;
const RIDER_ANIM: u32 = 0x784;
/// The reverse-spin animation.
const SPIN_REVERSE_ANIM: u64 = 0x120;

/// `bool RiderSpinSub_TryReverse(RiderSpinSub *self)` (0x135be0): if the
/// rider may reverse (0x114db8), turn the spin around: the angle plus pi
/// wrapped back into one turn (`a - 2pi * floor(a / 2pi + 0.5)`), the rate
/// and the three other signed fields negated, and the reverse animation
/// (0x120) played with blend -1. Returns whether it reversed.
pub fn rider_spin_sub_try_reverse(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).off(SPIN_SUB_RIDER).lw(c);
        call_ptr(c, SPIN_CAN_REVERSE, 0x0013_5bf8)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            let sub = Addr(r32(c, S0));
            let gp = r32(c, GP);
            c.cycles += 13;
            let pi = Addr(gp.wrapping_sub(0x733c)).read_fpu(c);
            let angle = sub.off(SPIN_SUB_ANGLE).read_fpu(c);
            let inv_two_pi = Addr(gp.wrapping_sub(0x7338)).read_fpu(c);
            let turned = angle + pi;
            let half = FpuF32::from_f32(0.5);
            c.g[AT][0] = 0x3f00_0000;
            let turns = turned * inv_two_pi + half;
            let mut whole = FpuF32::from_i32(turns.to_i32());
            c.f[3] = angle.0;
            c.f[5] = turned.0;
            c.f[2] = half.0;
            c.f[0] = turns.0;
            if abi::c_lt_s(c, turns, whole) {
                c.cycles += 3;
                c.g[AT][0] = 0x3f80_0000;
                c.f[0] = FpuF32::ONE.0;
                whole = whole - FpuF32::ONE;
            }
            c.f[1] = whole.0;

            c.cycles += 24;
            let two_pi = Addr(gp.wrapping_sub(0x7334)).read_fpu(c);
            c.g[A1][0] = SPIN_REVERSE_ANIM;
            let rate = -sub.off(SPIN_SUB_RATE).read_fpu(c);
            c.g[A2][0] = 0;
            let f2c = -sub.off(SPIN_SUB_2C).read_fpu(c);
            let f18 = -sub.off(SPIN_SUB_18).read_fpu(c);
            let f10 = -sub.off(SPIN_SUB_10).read_fpu(c);
            let wrapped = turned - whole * two_pi;
            c.g[V0][0] = sub.off(SPIN_SUB_RIDER).lw(c);
            sub.off(SPIN_SUB_RATE).write_fpu(c, rate);
            sub.off(SPIN_SUB_2C).write_fpu(c, f2c);
            sub.off(SPIN_SUB_ANGLE).write_fpu(c, wrapped);
            sub.off(SPIN_SUB_18).write_fpu(c, f18);
            sub.off(SPIN_SUB_10).write_fpu(c, f10);
            c.f[3] = rate.0;
            c.f[0] = f2c.0;
            c.f[4] = f18.0;
            c.f[1] = f10.0;
            c.f[2] = wrapped.0;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.f[12] = 0xbf80_0000; // blend -1.0
            c.g[A0][0] = Addr(r32(c, V0)).off(RIDER_ANIM).lw(c);
            call_ptr(c, SPIN_ANIM_PLAY, 0x0013_5c9c)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cLunoTable_Write (0x2257e0)

const LUNO_REGISTER_WRITE: Callee = Callee {
    addr: 0x0022_5be8,
    f: ee::s05::LunoVMRegister_Write,
};
/// Table: highest bucket index (-1 without a hash part) and the node array.
const LUNO_TABLE_MODULUS: u32 = 0xc;
const LUNO_TABLE_NODES: u32 = 0x10;
/// Hash node (0x20 bytes): key register (+0), value register (+0xc, its
/// type tag at +0x14) and the chain link (+0x18).
const LUNO_NODE_VALUE: u32 = 0xc;
const LUNO_NODE_VALUE_TYPE: u32 = 0x14;
const LUNO_NODE_NEXT: u32 = 0x18;
const LUNO_NODE_SIZE: u32 = 0x20;

/// `r += 1` on a 32-bit counter register (`addiu r, r, 1`).
fn luno_table_inc(c: &mut Cpu, r: usize) {
    c.g[r][0] = sx((c.g[r][0] as u32).wrapping_add(1));
}

/// `stream->Write(data, n)` (vtable slot 8) returning to `ret`; `vt`/`m`
/// are the registers the original loads the vtable and method into.
fn luno_table_stream_write(c: &mut Cpu, vt: usize, m: usize, ret: u32) -> crate::call::Flow {
    let stream = r32(c, S2);
    c.g[vt][0] = Addr(stream).lw(c);
    let table = r32(c, vt);
    c.g[A0][0] = Addr(table).off(8).lh(c);
    c.g[m][0] = Addr(table).off(0xc).lw(c);
    let method = r32(c, m);
    c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

/// `void cLunoTable_Write(cLunoTableRef *self, Stream *stream, int base)`
/// (0x2257e0): save a Luno table: the table pointer (8 bytes), the number
/// of live entries (non-nil values over every bucket and its chain), then
/// each live entry's key and value register (`LunoVMRegister_Write`).
pub fn c_luno_table_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A1][0]; // stream
        frame.save(c, S6, 0x20);
        c.g[S3][0] = c.g[A2][0]; // base
        frame.save(c, S0, 0x80);
        c.g[S6][0] = c.g[A0][0]; // self
        frame.save(c, S1, 0x70);
        c.g[A2][0] = 8;
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x10);
        // stream->Write(self->table, 8) (the a1 load sits before the method's).
        let stream = r32(c, S2);
        c.g[V1][0] = Addr(stream).lw(c);
        c.g[A1][0] = Addr(r32(c, S6)).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(8).lh(c);
        c.g[V0][0] = Addr(vt).off(0xc).lw(c);
        let method = r32(c, V0);
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0022_582c)?;

        // Count the live entries into the local at sp+0 (a1; a3 = bucket,
        // t0 = modulus, a0 = node, a2 = next bucket).
        c.g[V1][0] = Addr(r32(c, S6)).lw(c);
        c.g[A3][0] = 0;
        let count_at = Addr(sp(c));
        count_at.sw(c, 0);
        let table = r32(c, V1);
        c.g[V0][0] = Addr(table).off(LUNO_TABLE_MODULUS).lw(c);
        c.g[A0][0] = Addr(table).off(LUNO_TABLE_NODES).lw(c);
        if (c.g[V0][0] as i64) >= 0 {
            c.cycles += 3;
            c.g[T0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A0)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                if c.g[V0][0] != 0 {
                    luno_table_inc(c, A1);
                }
                c.cycles += 4;
                let node = r32(c, A0);
                c.g[V1][0] = Addr(node).off(LUNO_NODE_NEXT).lw(c);
                luno_table_inc(c, A3);
                c.g[A2][0] = sx(node.wrapping_add(LUNO_NODE_SIZE));
                if c.g[V1][0] != 0 {
                    // The chain (a0 counts, v1 walks).
                    c.cycles += 3;
                    c.g[A0][0] = c.g[A1][0];
                    c.g[V0][0] = Addr(r32(c, V1)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                    loop {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            luno_table_inc(c, A0);
                        }
                        c.cycles += 3;
                        c.g[V1][0] = Addr(r32(c, V1)).off(LUNO_NODE_NEXT).lw(c);
                        if c.g[V1][0] != 0 {
                            c.g[V0][0] = Addr(r32(c, V1)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                            if event_due(c) {
                                deliver_event_here(c, 0x0022_5878);
                            }
                        } else {
                            c.cycles += 1;
                            c.g[A1][0] = c.g[A0][0];
                            break;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                c.g[A0][0] = c.g[A2][0];
                if c.g[V0][0] == 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_5850);
                    }
                } else {
                    c.cycles += 1;
                    let count = c.g[A1][0];
                    Addr(sp(c)).sw(c, count);
                    break;
                }
            }
        }

        // stream->Write(&count, 4)
        c.cycles += 10;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        luno_table_stream_write(c, V0, V1, 0x0022_58bc)?;
        c.g[V0][0] = Addr(sp(c)).lw(c);
        c.g[S5][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S6)).lw(c);
            let table = r32(c, V0);
            c.g[V1][0] = Addr(table).off(LUNO_TABLE_MODULUS).lw(c);
            c.g[S1][0] = Addr(table).off(LUNO_TABLE_NODES).lw(c);
            if (c.g[V1][0] as i64) >= 0 {
                // s1 = bucket node, s0 = chain node / value, s5 = bucket
                // index, s4 = next bucket.
                loop {
                    c.cycles += 3;
                    let node = r32(c, S1);
                    c.g[V0][0] = Addr(node).off(LUNO_NODE_VALUE_TYPE).lw(c);
                    c.g[S0][0] = sx(node.wrapping_add(LUNO_NODE_VALUE));
                    if c.g[V0][0] != 0 {
                        c.cycles += 11;
                        luno_table_write_entry(c, S1, S0, 0x0022_5900, 0x0022_5910)?;
                    }
                    c.cycles += 4;
                    let node = r32(c, S1);
                    c.g[S0][0] = Addr(node).off(LUNO_NODE_NEXT).lw(c);
                    luno_table_inc(c, S5);
                    c.g[S4][0] = sx(node.wrapping_add(LUNO_NODE_SIZE));
                    if c.g[S0][0] != 0 {
                        c.cycles += 2;
                        c.g[V0][0] = Addr(r32(c, S0)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                        loop {
                            c.cycles += 2;
                            let link = r32(c, S0);
                            c.g[S1][0] = sx(link.wrapping_add(LUNO_NODE_VALUE));
                            if c.g[V0][0] != 0 {
                                c.cycles += 11;
                                luno_table_write_entry(c, S0, S1, 0x0022_594c, 0x0022_595c)?;
                            }
                            c.cycles += 3;
                            c.g[S0][0] = Addr(r32(c, S0)).off(LUNO_NODE_NEXT).lw(c);
                            if c.g[S0][0] != 0 {
                                c.g[V0][0] = Addr(r32(c, S0)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                                if event_due(c) {
                                    deliver_event_here(c, 0x0022_5928);
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V1][0] = Addr(r32(c, S6)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).off(LUNO_TABLE_MODULUS).lw(c);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    c.g[S1][0] = c.g[S4][0];
                    if c.g[V0][0] == 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_58d8);
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// One live entry: `--count`, then `LunoVMRegister_Write(key)` and
/// `LunoVMRegister_Write(value)` (registers `key`/`value` hold them; `ret_*`
/// are the two return addresses).
fn luno_table_write_entry(
    c: &mut Cpu,
    key: usize,
    value: usize,
    ret_key: u32,
    ret_value: u32,
) -> crate::call::Flow {
    let count_at = Addr(sp(c));
    c.g[V0][0] = count_at.lw(c);
    c.g[A0][0] = c.g[key][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    let count = c.g[V0][0];
    count_at.sw(c, count);
    LUNO_REGISTER_WRITE.call(c, ret_key)?;
    c.g[A0][0] = c.g[value][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    LUNO_REGISTER_WRITE.call(c, ret_value)
}

// ---------------------------------------------------------------------------
// VfxTube_Update (0x372500)

/// `void VfxTube_Update(void)` (0x372500): per-frame tube effect update,
/// which only refreshes the colour table (`VfxTube_UpdateColorTable`).
pub fn vfx_tube_update(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(Callee {
            addr: 0x0037_25b0,
            f: ee::s12::VfxTube_UpdateColorTable,
        }),
        0x0037_2510,
    );
}

// ---------------------------------------------------------------------------
// WScriptMan_OnObjectTrigger (0x30a060)

/// Script manager fields: the current object id (+0), the cache (+0x28c),
/// the trigger being run (+0x290, from the trigger volume's +0x50), its
/// source (+0x294) and volume (+0x298), and the script owner (+0x2a4) or
/// owning instance (+0x2a8, whose owner is at +0x3c).
const WSM_CURRENT_ID: u32 = 0x0;
const WSM_CACHE: u32 = 0x28c;
const WSM_TRIGGER: u32 = 0x290;
const WSM_SOURCE: u32 = 0x294;
const WSM_VOLUME: u32 = 0x298;
const WSM_OWNER: u32 = 0x2a4;
const WSM_INSTANCE: u32 = 0x2a8;
const WSM_ARGS: u32 = 0x24;

const WSM_POST_TRIGGER_EVENT: Callee = Callee {
    addr: 0x0030_97e0,
    f: ee::s10::WScriptMan_PostTriggerEvent,
};
const WSM_CALL_FUNC: Callee = Callee {
    addr: 0x0030_9c88,
    f: ee::s10::WScriptMan_CallFunc,
};
const WSM_FIND_FUNC: u32 = 0x003a_6b78;
const WSM_FUNC_ENTRY: u32 = 0x003a_6d18;
const WSM_VALUE_FROM_FUNC: u32 = 0x0022_6600;
const WSM_CALL_INIT: u32 = 0x0022_24a8;
const WSM_CALL_RUN: u32 = 0x0022_24b8;
const WSM_ALLOC: u32 = 0x0031_7d70;
const WSM_TABLE_CTOR: u32 = 0x0022_4da0;
const WSM_TABLE_DTOR: u32 = 0x0022_4df0;
const WSM_VALUE_RELEASE: u32 = 0x0022_5b90;
/// The allocation name (`0x4898d8`).
const WSM_TABLE_NAME: u32 = 0x0049_0000 - 0x6728;

/// `obj->vtable[slot](obj)`: `lw vt; lh delta; lw method` with the
/// registers `vt`/`m` the original uses, returning to `ret`.
fn wsm_virtual(c: &mut Cpu, obj: usize, slot: u32, vt: usize, m: usize, ret: u32) -> crate::call::Flow {
    let this = r32(c, obj);
    c.g[vt][0] = Addr(this).lw(c);
    let table = r32(c, vt);
    c.g[A0][0] = Addr(table).off(slot).lh(c);
    c.g[m][0] = Addr(table).off(slot + 4).lw(c);
    let method = r32(c, m);
    c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

/// `void WScriptMan_OnObjectTrigger(WScriptMan *self, void *source,
/// TriggerVolume *volume, Object *obj)` (0x30a060): an object entered a
/// trigger volume. Makes the object current (its id, vtable slot 0x38),
/// records the trigger, and for an object that says so (slot 0x40) posts
/// the trigger event, unless it is a rider who is already finished outside
/// event mode 4. Then runs the trigger's script function (`id != -1`):
/// directly without an owner, else through a Luno call with the owner's
/// table (or a fresh one, freed after). Finally the trigger's effect, if
/// any, is played at 0.5 (slot 0x148), and the current id and trigger
/// state are restored / cleared.
pub fn wscript_man_on_object_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A1][0]; // source
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A2][0]; // volume
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A0][0]; // self
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A3][0]; // obj
        frame.save_ra(c, 0x30);
        // self->current = obj->GetId(); s4 = the previous one.
        let obj = r32(c, S3);
        c.g[V1][0] = Addr(obj).lw(c);
        c.g[S4][0] = Addr(r32(c, S2)).off(WSM_CURRENT_ID).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(0x38).lh(c);
        c.g[V0][0] = Addr(vt).off(0x3c).lw(c);
        let method = r32(c, V0);
        c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0030_a0a4)?;
        let wsm = Addr(r32(c, S2));
        let id = c.g[V0][0];
        wsm.off(WSM_CURRENT_ID).sw(c, id);
        c.g[V0][0] = Addr(r32(c, S1)).off(0x50).lw(c);
        let (source, trigger, volume) = (c.g[S0][0], c.g[V0][0], c.g[S1][0]);
        wsm.off(WSM_SOURCE).sw(c, source);
        wsm.off(WSM_TRIGGER).sw(c, trigger);
        wsm.off(WSM_VOLUME).sw(c, volume);
        wsm_virtual(c, S3, 0x40, V1, V0, 0x0030_a0cc)?;
        let posts = c.g[V0][0] != 0;
        c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        let mut run = true;
        if posts {
            c.cycles += 5;
            c.g[A1][0] = 4;
            c.g[V0][0] = Addr(r32(c, A0)).off(0x84).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x214).lw(c);
            let event_mode = c.g[V1][0] == 4;
            // The rider that owns obj (its interface at +0x6c0).
            c.g[V0][0] = sx(r32(c, S3).wrapping_sub(0x6c0));
            if !event_mode {
                c.cycles += 4;
                if c.g[S3][0] == 0 {
                    c.g[V0][0] = 0;
                }
                c.g[V1][0] = Addr(r32(c, V0)).off(0x480).lw(c);
                if c.g[V1][0] != 0 {
                    let prev = c.g[S4][0];
                    Addr(r32(c, S2)).sw(c, prev);
                    run = false;
                }
            }
            if run {
                c.cycles += 4;
                c.g[V0][0] = Addr(r32(c, S2)).off(WSM_TRIGGER).lw(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = Addr(r32(c, V0)).off(0x78).lw(c);
                WSM_POST_TRIGGER_EVENT.call(c, 0x0030_a108)?;
            }
        }
        if run {
            wsm_run_trigger_func(c)?;
            // The trigger's effect (+0xc), played at 0.5.
            c.cycles += 3;
            c.g[A1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
            if c.g[A1][0] != 0 {
                c.cycles += 8;
                let fx = r32(c, A1);
                c.g[V0][0] = Addr(fx).off(0xc).lw(c);
                c.g[AT][0] = 0x3f00_0000;
                c.f[12] = 0x3f00_0000;
                let vt = r32(c, V0);
                c.g[A0][0] = Addr(vt).off(0x148).lh(c);
                c.g[V1][0] = Addr(vt).off(0x14c).lw(c);
                let method = r32(c, V1);
                c.g[A0][0] = sx(fx.wrapping_add(r32(c, A0)));
                call_ptr(c, method, 0x0030_a240)?;
            }
            let prev = c.g[S4][0];
            Addr(r32(c, S2)).sw(c, prev);
        }

        c.cycles += 11;
        let wsm = Addr(r32(c, S2));
        wsm.off(WSM_TRIGGER).sw(c, 0);
        wsm.off(WSM_SOURCE).sw(c, 0);
        wsm.off(WSM_VOLUME).sw(c, 0);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// Find and run the trigger's script function; leaves `v0 = self->trigger`.
/// Stack: the argument value at sp+0, the result at sp+0x10, the Luno call
/// at sp+0x20, the function id at sp+0x24.
fn wsm_run_trigger_func(c: &mut Cpu) -> crate::call::Flow {
    let s = sp(c);
    c.cycles += 8;
    c.g[A1][0] = Addr(r32(c, S2)).off(WSM_TRIGGER).lw(c);
    c.g[A2][0] = 2;
    c.g[A0][0] = Addr(r32(c, S2)).off(WSM_CACHE).lw(c);
    call_ptr(c, WSM_FIND_FUNC, 0x0030_a118)?;
    c.g[A1][0] = sx(s + WSM_ARGS);
    c.g[V1][0] = !c.g[V0][0];
    let id = c.g[V0][0];
    Addr(s + WSM_ARGS).sw(c, id);
    if c.g[V1][0] == 0 {
        // No function (-1).
        c.cycles += 1;
        c.g[V0][0] = Addr(r32(c, S2)).off(WSM_TRIGGER).lw(c);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, S2)).off(WSM_OWNER).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S2)).off(WSM_INSTANCE).lw(c);
        if c.g[V0][0] == 0 {
            // No owner: a plain call.
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            WSM_CALL_FUNC.call(c, 0x0030_a148)?;
            c.g[V0][0] = Addr(r32(c, S2)).off(WSM_TRIGGER).lw(c);
            return Ok(());
        }
    }
    // A Luno call with the owner's table as `self`.
    c.cycles += 14;
    c.g[A0][0] = Addr(r32(c, S2)).off(WSM_CACHE).lw(c);
    call_ptr(c, WSM_FUNC_ENTRY, 0x0030_a158)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = 0;
    call_ptr(c, WSM_VALUE_FROM_FUNC, 0x0030_a168)?;
    c.g[A0][0] = sx(s + 0x20);
    Addr(s + 0x18).sw(c, 0);
    c.g[S3][0] = c.g[A0][0];
    Addr(s + 0x10).sw(c, 0);
    call_ptr(c, WSM_CALL_INIT, 0x0030_a17c)?;
    c.g[V0][0] = Addr(r32(c, S2)).off(WSM_INSTANCE).lw(c);
    if c.g[V0][0] == 0 {
        c.g[V0][0] = Addr(r32(c, S2)).off(WSM_OWNER).lw(c);
    } else {
        c.cycles += 1;
        c.g[V0][0] = Addr(r32(c, V0)).off(0x3c).lw(c);
    }
    c.cycles += 3;
    c.g[S1][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
    c.g[V0][0] = sx(s + 0x10);
    if c.g[S1][0] != 0 {
        c.cycles += 8;
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A3][0] = c.g[V0][0];
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, WSM_CALL_RUN, 0x0030_a1b0)?;
        c.g[A0][0] = c.g[S0][0];
    } else {
        // No table yet: a temporary one, released after the call.
        c.cycles += 18;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(WSM_TABLE_NAME);
        c.g[A0][0] = 4;
        call_ptr(c, WSM_ALLOC, 0x0030_a1d0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 8;
        call_ptr(c, WSM_TABLE_CTOR, 0x0030_a1dc)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[S0][0] = sx(s + 0x10);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S0][0];
        call_ptr(c, WSM_CALL_RUN, 0x0030_a1f8)?;
        c.g[A0][0] = c.g[S1][0];
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, WSM_TABLE_DTOR, 0x0030_a208)?;
        }
        c.cycles += 1;
        c.g[A0][0] = c.g[S0][0];
    }
    // Release the result value.
    c.cycles += 2;
    c.g[A1][0] = 2;
    call_ptr(c, WSM_VALUE_RELEASE, 0x0030_a214)?;
    c.cycles += 1;
    c.g[V0][0] = Addr(r32(c, S2)).off(WSM_TRIGGER).lw(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{check, elf_image, fill, put_words, scramble, Rng, DATA, GAME_GP};

    // ------------------------------------------- ComputerActorRider_ThinkNaturalAir

    fn natural_air_f32s(c: &mut Cpu, a: u32, v: [f32; 4]) {
        put_words(c, a, &v.map(f32::to_bits));
    }

    /// The AI in the air with the predictor in each state and heights around
    /// the 0.5 / 1 / 3 thresholds (spin, flip, steer); the real
    /// `Rand_NextGame` and `GetSteerTarget` run, `ComputeSteer`'s two virtual
    /// calls are fakes (one may unwind).
    #[test]
    fn computer_actor_rider_think_natural_air_matches() {
        covered(
            ee::s00::f_0010b590__generated,
            computer_actor_rider_think_natural_air,
            240,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0010_b590, 2);
                c.pc = 0x0010_b590;
                let ai = DATA + 0x8_0000;
                let rider = DATA + 0x9_0000;
                let air = DATA + 0xa_0000;
                let vt = DATA + 0xb_0000;
                let axis = DATA + 0xc_0000;
                let motion = DATA + 0xc_1000;
                let pad = DATA + 0xd_0000;
                c.g[A0][0] = sx(ai);
                c.g[A1][0] = sx(pad);
                fill(c, &mut r, pad, 4);
                // The game's random generator state (both parities).
                fill(c, &mut r, 0x004f_f030, 6);
                fill(c, &mut r, ai + 0xe00, 0x10);
                c.bus.write32(ai + AI_RIDER, rider);
                c.bus.write32(rider + RIDER_AIR, air);
                c.bus
                    .write32(air + AIR_STATE, [1u32, 3, 0, 2][(seed % 4) as usize]);
                let heights = [-1.0f32, 0.0, 0.2, 0.5, 0.7, 1.0, 2.0, 3.0, 5.0];
                let h = heights[((seed / 4) as usize) % heights.len()];
                let ground = f32::from_bits(r.f32(10.0));
                c.bus.write32(air + AIR_GROUND, ground.to_bits());
                c.bus.write32(air + AIR_HEIGHT, (ground + h).to_bits());
                // GetSteerTarget's inputs.
                for off in [0x110u32, 0x490, 0x4b0] {
                    natural_air_f32s(
                        c,
                        rider + off,
                        [
                            f32::from_bits(r.f32(2.0)),
                            f32::from_bits(r.f32(2.0)),
                            0.0,
                            0.0,
                        ],
                    );
                }
                // ComputeSteer: model axis, two virtual calls (fakes).
                c.bus.write32(rider + 0x6c0, vt);
                c.bus
                    .write16(vt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x14, FAKE);
                c.bus
                    .write16(vt + 0x28, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x2c, FAKE + 0x10);
                c.bus.write32(fakes::LOG_RESULTS, axis);
                c.bus.write32(fakes::LOG_RESULTS + 4, motion);
                natural_air_f32s(c, motion, [0.0; 4]);
                natural_air_f32s(
                    c,
                    rider + 0x1c0,
                    [
                        f32::from_bits(r.f32(2.0)),
                        f32::from_bits(r.f32(2.0)),
                        if seed % 3 == 0 { 1.0 } else { -f32::from_bits(r.f32(1.0)) },
                        0.0,
                    ],
                );
                natural_air_f32s(
                    c,
                    axis,
                    [
                        f32::from_bits(r.f32(2.0)),
                        f32::from_bits(r.f32(2.0)),
                        f32::from_bits(r.f32(2.0)),
                        0.0,
                    ],
                );
            },
        );
    }

    // ------------------------------------------------------- CollHull_OverlapsAABB

    /// A hull's bounding sphere against a box that overlaps it, with one of
    /// the six axis tests made to fail (`seed % 7`; 6 = overlap), plus some
    /// edge-case floats.
    #[test]
    fn coll_hull_overlaps_aabb_matches() {
        check(
            ee::s11::f_0032f760__generated,
            coll_hull_overlaps_aabb,
            280,
            &|c, seed| {
                let mut r = Rng(0x0032_f760 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.pc = 0x0032_f760;
                let hull = DATA;
                let sphere = DATA + 0x100;
                let min = DATA + 0x200;
                let max = DATA + 0x300;
                fill(c, &mut r, hull, 0x20);
                c.bus.write32(hull + HULL_SPHERE, sphere);
                let center = [
                    (r.next_u32() % 16) as f32 - 8.0,
                    (r.next_u32() % 16) as f32 - 8.0,
                    (r.next_u32() % 16) as f32 - 8.0,
                    f32::from_bits(r.f32(4.0)),
                ];
                let rad = (r.next_u32() % 4) as f32 + 0.5;
                put_words(c, sphere + HULL_SPHERE_CENTER, &center.map(f32::to_bits));
                c.bus.write32(sphere + HULL_SPHERE_RADIUS, rad.to_bits());
                let mut lo = [0f32; 3];
                let mut hi = [0f32; 3];
                let mut mn = [0f32; 3];
                let mut mx = [0f32; 3];
                for a in 0..3 {
                    lo[a] = center[a] - rad;
                    hi[a] = center[a] + rad;
                    // Touching exactly on some seeds (the `<=`).
                    mn[a] = if seed % 5 == 0 { hi[a] } else { center[a] - 0.25 };
                    mx[a] = if seed % 5 == 1 { lo[a] } else { center[a] + 0.25 };
                }
                let fail = seed % 7;
                if fail < 6 {
                    let a = (fail / 2) as usize;
                    if fail % 2 == 0 {
                        mx[a] = lo[a] - 1.0;
                    } else {
                        mn[a] = hi[a] + 1.0;
                    }
                }
                let mut put = |c: &mut Cpu, at: u32, v: [f32; 3]| {
                    put_words(c, at, &v.map(f32::to_bits));
                    c.bus.write32(at + 12, r.next_u32());
                };
                put(c, min, mn);
                put(c, max, mx);
                if seed % 11 == 10 {
                    let mut r2 = Rng(seed as u64 * 77 + 1);
                    for i in 0..3 {
                        c.bus.write32(min + 4 * i, r2.edge_f32());
                        c.bus.write32(max + 4 * i, r2.edge_f32());
                    }
                    c.bus.write32(sphere + HULL_SPHERE_RADIUS, r2.edge_f32());
                }
                c.g[A0][0] = sx(hull);
                c.g[A1][0] = sx(min);
                c.g[A2][0] = sx(max);
            },
        );
    }

    // ------------------------------------------- RiderWipeoutState_PostUpdateAir

    /// Control block of the gated fakes below: magic word (without it they
    /// run the generated body, so other tests are unaffected), the contact
    /// distance `FindClosestContact` returns, the ray distance
    /// `CastGroundRay` returns, and the hits they write.
    const PUA_CTRL: u32 = DATA + 0x5_8000;
    const PUA_MAGIC: u32 = 0x1378_60a1;
    const PUA_HIT_A_TEMPLATE: u32 = PUA_CTRL + 0x100;
    const PUA_HIT_B_TEMPLATE: u32 = PUA_CTRL + 0x200;

    /// A logged fake (`fakes::fake`); the two collision queries also return
    /// their chosen distance in f0 and write their hit.
    fn pua_fake(c: &mut Cpu, target: u32) {
        let (a1, a2) = (c.g[A1][0] as u32, c.g[A2][0] as u32);
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        let (template, hit, dist) = match target {
            PUA_FIND_CLOSEST_CONTACT => (PUA_HIT_A_TEMPLATE, a2, PUA_CTRL + 4),
            0x0013_8960 => (PUA_HIT_B_TEMPLATE, a1, PUA_CTRL + 8),
            _ => return,
        };
        for i in 0..0x18 {
            let w = c.bus.read32(template + 4 * i);
            c.bus.write32(hit + 4 * i, w);
        }
        c.f[0] = c.bus.read32(dist);
    }

    macro_rules! pua_gated {
        ($name:ident, $addr:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(PUA_CTRL) != PUA_MAGIC {
                    return $gen(c);
                }
                pua_fake(c, $addr);
            }
        };
    }
    pua_gated!(pua_fake_hull_ctor, PUA_COLL_HULL_CTOR, ee::s11::f_0032f650__generated);
    pua_gated!(pua_fake_closest, PUA_FIND_CLOSEST_CONTACT, ee::s11::f_003342d0__generated);
    pua_gated!(pua_fake_hull_dtor, PUA_COLL_HULL_DTOR, ee::s11::f_0032f708__generated);
    pua_gated!(pua_fake_clear_air, PUA_CLEAR_AIR_TIME, ee::s00::f_00113998__generated);
    pua_gated!(pua_fake_translate, PUA_TRANSLATE, ee::s00::f_00106538__generated);
    pua_gated!(pua_fake_bounds, PUA_UPDATE_BOUNDS, ee::s00::f_0011e150__generated);
    pua_gated!(pua_fake_wall, PUA_WALL_COLLISION, ee::s00::f_00105398__generated);
    pua_gated!(pua_fake_riders, PUA_RIDER_COLLISIONS, ee::s00::f_00107888__generated);
    pua_gated!(pua_fake_prime, PUA_AIR_PRIME, ee::s00::f_001135b8__generated);
    pua_gated!(pua_fake_spray, 0x002e_23e0, ee::s09::f_002e23e0__generated);
    pua_gated!(pua_fake_ground_ray, 0x0013_8960, ee::s01::f_00138960__generated);

    /// Install the gated fakes in their callees' slots once per test process.
    fn pua_install_fakes() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| unsafe {
            ee::s11::OVERRIDE_0032F650 = pua_fake_hull_ctor;
            ee::s11::OVERRIDE_003342D0 = pua_fake_closest;
            ee::s11::OVERRIDE_0032F708 = pua_fake_hull_dtor;
            ee::s00::OVERRIDE_00113998 = pua_fake_clear_air;
            ee::s00::OVERRIDE_00106538 = pua_fake_translate;
            ee::s00::OVERRIDE_0011E150 = pua_fake_bounds;
            ee::s00::OVERRIDE_00105398 = pua_fake_wall;
            ee::s00::OVERRIDE_00107888 = pua_fake_riders;
            ee::s00::OVERRIDE_001135B8 = pua_fake_prime;
            ee::s09::OVERRIDE_002E23E0 = pua_fake_spray;
            ee::s01::OVERRIDE_00138960 = pua_fake_ground_ray;
        });
    }

    /// The sphere contact missed / hit; the ground ray short, behind, the
    /// rider moving away from or far above the ground, or a landing (hit
    /// object none / ownerless / with a push handler, pushed out of the
    /// ground or not); the push-out path with and without the velocity
    /// kill. The callees are logged fakes (one may unwind); the real
    /// `RiderBase_TriggerDynamicSpray` wrapper runs.
    #[test]
    fn rider_wipeout_state_post_update_air_matches() {
        pua_install_fakes();
        covered(
            ee::s01::f_00137860__generated,
            rider_wipeout_state_post_update_air,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0013_7860, 12);
                c.pc = 0x0013_7860;
                let state = DATA + 0x5_0000;
                let rider = DATA + 0x5_1000;
                let vt = DATA + 0x5_3000;
                let handler = DATA + 0x5_4000;
                let obj = DATA + 0x5_6000;
                let owner = DATA + 0x5_6100;
                let owner_vt = DATA + 0x5_6200;
                let world = DATA + 0x5_7000;
                let pick = |r: &mut Rng, v: &[f32]| v[r.next_u32() as usize % v.len()].to_bits();
                let comps = [-12.0f32, -3.0, -1.0, -0.5, 0.0, 0.5, 1.0, 3.0, 12.0];
                let vec = |c: &mut Cpu, r: &mut Rng, at: u32| {
                    for i in 0..3 {
                        c.bus.write32(at + 4 * i, pick(r, &comps));
                    }
                    c.bus.write32(at + 12, pick(r, &[0.0, 1.0]));
                };
                c.bus.write32(PUA_CTRL, PUA_MAGIC);
                c.g[A0][0] = sx(state);
                fill(c, &mut r, state, 4);
                c.bus.write32(state + PUA_STATE_RIDER, rider);
                // Rider.
                c.bus.write32(rider + PUA_R_VTABLE, vt);
                c.bus.write16(vt + 0x40, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x44, FAKE);
                for off in [PUA_R_AIR, PUA_R_OVERLAP, PUA_R_WALL_HINT, PUA_R_SPHERE_TREE] {
                    c.bus.write32(rider + off, r.next_u32());
                }
                c.bus.write32(rider + PUA_R_HANDLER, handler);
                vec(c, &mut r, rider + PUA_R_POS);
                vec(c, &mut r, rider + PUA_R_VEL);
                vec(c, &mut r, rider + PUA_R_GROUND_NORMAL);
                // The distances: the sphere contact, then the ground ray.
                c.bus.write32(PUA_CTRL + 4, pick(&mut r, &[-1.0, 0.0, 0.25, 2.0, 2.0]));
                c.bus
                    .write32(PUA_CTRL + 8, pick(&mut r, &[-1.0, 0.2, 0.4, 0.5, 3.0, 3.0, 3.0]));
                fill(c, &mut r, PUA_HIT_A_TEMPLATE, 0x18);
                vec(c, &mut r, PUA_HIT_A_TEMPLATE + 0x10);
                fill(c, &mut r, PUA_HIT_B_TEMPLATE, 0x18);
                vec(c, &mut r, PUA_HIT_B_TEMPLATE);
                vec(c, &mut r, PUA_HIT_B_TEMPLATE + 0x10);
                // Hit object: none, ownerless, or an owner with a push handler.
                let object = match r.next_u32() % 3 {
                    0 => 0,
                    1 => {
                        c.bus.write32(obj + 0xc, 0);
                        obj
                    }
                    _ => {
                        c.bus.write32(obj + 0xc, owner);
                        obj
                    }
                };
                c.bus.write32(PUA_HIT_B_TEMPLATE + 0x50, object);
                c.bus.write32(owner + 0xc, owner_vt);
                c.bus
                    .write16(owner_vt + 0x158, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(owner_vt + 0x15c, FAKE + 0x20);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x10, r.next_u32() % 5);
            },
        );
    }

    // ------------------------------------------------ cWorldPainterMan_DebugDraw

    /// The world's painter manager and current painter; the real
    /// `DebugDrawCurrent` runs with the debug draw switched off
    /// (`gp+0x13c4` = 0, as in the game).
    #[test]
    fn c_world_painter_man_debug_draw_matches() {
        check(
            ee::s08::f_002bb6b8__generated,
            c_world_painter_man_debug_draw,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_b6b8, 0);
                c.pc = 0x002b_b6b8;
                let world = DATA + 0x1_0000;
                let man = DATA + 0x1_1000;
                let painter = DATA + 0x1_2000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + WORLD_PAINTER_MAN, man);
                c.bus.write32(man + WPM_CURRENT, painter);
                c.bus.write32(painter + PAINTER_NODES, r.next_u32());
                c.bus.write32(GAME_GP + 0x13c4, 0);
            },
        );
    }

    // ------------------------------------------------ AnimSeqList_PushLoopingAnim

    /// Gate of the plain logged fakes below (without it they run the
    /// generated body).
    const ASL_CTRL: u32 = DATA + 0x5_9000;
    const ASL_MAGIC: u32 = 0x3145_88a5;

    macro_rules! asl_gated {
        ($name:ident, $addr:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(ASL_CTRL) != ASL_MAGIC {
                    return $gen(c);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }
    asl_gated!(asl_fake_alloc, 0x0031_7d70, ee::s10::f_00317d70__generated);
    asl_gated!(asl_fake_ctor, 0x0031_3508, ee::s10::f_00313508__generated);
    asl_gated!(asl_fake_track_anim, 0x0031_3c50, ee::s10::f_00313c50__generated);
    asl_gated!(asl_fake_track_time, 0x0031_3cf0, ee::s10::f_00313cf0__generated);
    asl_gated!(asl_fake_track_loop, 0x0031_3d40, ee::s10::f_00313d40__generated);

    fn asl_install_fakes() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| unsafe {
            ee::s10::OVERRIDE_00317D70 = asl_fake_alloc;
            ee::s10::OVERRIDE_00313508 = asl_fake_ctor;
            ee::s10::OVERRIDE_00313C50 = asl_fake_track_anim;
            ee::s10::OVERRIDE_00313CF0 = asl_fake_track_time;
            ee::s10::OVERRIDE_00313D40 = asl_fake_track_loop;
        });
    }

    /// A list with a few sequences; the real `PushNew` links the new one in
    /// (its allocator and constructor are logged fakes returning a node),
    /// the three track setters are logged fakes (one may unwind).
    #[test]
    fn anim_seq_list_push_looping_anim_matches() {
        asl_install_fakes();
        covered(
            ee::s10::f_00314588__generated,
            anim_seq_list_push_looping_anim,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_4588, 5);
                c.pc = 0x0031_4588;
                c.bus.write32(ASL_CTRL, ASL_MAGIC);
                let list = DATA + 0x5_0000;
                c.bus.write32(list, r.next_u32() % 8);
                c.bus.write32(list + 4, if seed % 3 == 0 { 0 } else { DATA + 0x5_1000 });
                c.g[A0][0] = sx(list);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(r.next_u32() % 0x400);
                c.f[12] = r.edge_f32();
            },
        );
    }

    // --------------------------------------------------- RiderSpinSub_TryReverse

    const SPIN_CTRL: u32 = DATA + 0x5_a000;
    const SPIN_MAGIC: u32 = 0x135b_e0a5;

    /// The reverse query: a logged fake whose answer (0 or 1) the fixture
    /// picks at `SPIN_CTRL + 4`.
    fn spin_fake_can_reverse(c: &mut Cpu) {
        if c.bus.read32(SPIN_CTRL) != SPIN_MAGIC {
            return ee::s00::f_00114db8__generated(c);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, SPIN_CAN_REVERSE, ra);
        if c.pc == ra {
            c.g[V0][0] = c.bus.read32(SPIN_CTRL + 4) as u64;
        }
    }

    fn spin_fake_play(c: &mut Cpu) {
        if c.bus.read32(SPIN_CTRL) != SPIN_MAGIC {
            return ee::s10::f_003128e8__generated(c);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, SPIN_ANIM_PLAY, ra);
    }

    fn spin_install_fakes() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| unsafe {
            ee::s00::OVERRIDE_00114DB8 = spin_fake_can_reverse;
            ee::s10::OVERRIDE_003128E8 = spin_fake_play;
        });
    }

    /// No reverse, or a reverse of angles around +-pi (the floor's
    /// correction when the rounding goes up), with edge floats in the other
    /// fields; the game's pi constants from the image.
    #[test]
    fn rider_spin_sub_try_reverse_matches() {
        spin_install_fakes();
        covered(
            ee::s01::f_00135be0__generated,
            rider_spin_sub_try_reverse,
            240,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0013_5be0, 2);
                c.pc = 0x0013_5be0;
                c.bus.write32(SPIN_CTRL, SPIN_MAGIC);
                c.bus.write32(SPIN_CTRL + 4, (seed % 4 != 0) as u32);
                let sub = DATA + 0x5_0000;
                let rider = DATA + 0x5_1000;
                c.g[A0][0] = sx(sub);
                c.bus.write32(sub + SPIN_SUB_RIDER, rider);
                c.bus.write32(rider + RIDER_ANIM, r.next_u32());
                let angles = [
                    0.0f32,
                    3.1415927,
                    -3.1415927,
                    6.2831855,
                    -6.2831855,
                    9.424778,
                    -9.424778,
                    1.0,
                    -2.5,
                    100.0,
                ];
                let angle = if seed % 5 == 4 {
                    r.edge_f32()
                } else {
                    angles[(seed as usize / 4) % angles.len()].to_bits()
                };
                c.bus.write32(sub + SPIN_SUB_ANGLE, angle);
                for off in [SPIN_SUB_10, SPIN_SUB_18, SPIN_SUB_RATE, SPIN_SUB_2C] {
                    c.bus.write32(sub + off, r.edge_f32());
                }
            },
        );
    }

    // ------------------------------------------------------------ cLunoTable_Write

    /// Tables without a hash part, or with 1..4 buckets whose nodes and
    /// chains (0..3 links) hold nil and non-nil values; plain register
    /// types only (the real `LunoVMRegister_Write` writes them directly).
    /// The stream's Write is a fake (some calls unwind).
    #[test]
    fn c_luno_table_write_matches() {
        covered(
            ee::s05::f_002257e0__generated,
            c_luno_table_write,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_57e0, 8);
                c.pc = 0x0022_57e0;
                let this = DATA + 0x5_0000;
                let stream = DATA + 0x5_0100;
                let table = DATA + 0x5_0200;
                let nodes = DATA + 0x5_1000;
                let links = DATA + 0x5_2000;
                let vt = fakes::vtable(c, &mut r, &[8]);
                c.bus.write32(stream, vt);
                c.bus.write32(this, table);
                let modulus = [-1i32, 0, 1, 3][(seed % 4) as usize];
                c.bus.write32(table + LUNO_TABLE_MODULUS, modulus as u32);
                c.bus.write32(table + LUNO_TABLE_NODES, nodes);
                let types = [0u32, 0, 1, 2, 3, 6];
                let node = |c: &mut Cpu, r: &mut Rng, at: u32, next: u32| {
                    c.bus.write32(at, r.next_u32());
                    c.bus.write32(at + 8, types[r.next_u32() as usize % types.len()]);
                    c.bus.write32(at + LUNO_NODE_VALUE, r.next_u32());
                    c.bus
                        .write32(at + LUNO_NODE_VALUE_TYPE, types[r.next_u32() as usize % types.len()]);
                    c.bus.write32(at + LUNO_NODE_NEXT, next);
                };
                let mut link = links;
                for b in 0..4u32 {
                    let mut next = 0;
                    for _ in 0..r.next_u32() % 4 {
                        node(c, &mut r, link, next);
                        next = link;
                        link += LUNO_NODE_SIZE;
                    }
                    node(c, &mut r, nodes + LUNO_NODE_SIZE * b, next);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    // ------------------------------------------------------------- VfxTube_Update

    /// The real `VfxTube_UpdateColorTable` runs: the tube colour unchanged
    /// (nothing to do) or changed in one component (the table refill).
    #[test]
    fn vfx_tube_update_matches() {
        check(ee::s12::f_00372500__generated, vfx_tube_update, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x0037_2500, 0);
            c.pc = 0x0037_2500;
            let rgb: [u32; 3] = [r.next_u32() % 256, r.next_u32() % 256, r.next_u32() % 256];
            for (i, v) in rgb.iter().enumerate() {
                c.bus.write32(GAME_GP + 0x10f0 + 4 * i as u32, *v);
                // The last colour the table was built for.
                let last = if seed % 4 == i as u32 { *v + 1 } else { *v };
                c.bus.write32(GAME_GP + 0x1140 + 4 * i as u32, (last as f32).to_bits());
            }
        });
    }

    // ------------------------------------------------ WScriptMan_OnObjectTrigger

    const WSM_CTRL: u32 = DATA + 0x5_b000;
    const WSM_MAGIC: u32 = 0x30a0_60a7;

    /// The function lookup: a logged fake returning the id the fixture
    /// picks at `WSM_CTRL + 4` (-1 = none).
    fn wsm_fake_find_func(c: &mut Cpu) {
        if c.bus.read32(WSM_CTRL) != WSM_MAGIC {
            return ee::s13::f_003a6b78__generated(c);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, WSM_FIND_FUNC, ra);
        if c.pc == ra {
            c.g[V0][0] = sx(c.bus.read32(WSM_CTRL + 4));
        }
    }

    macro_rules! wsm_gated {
        ($name:ident, $addr:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(WSM_CTRL) != WSM_MAGIC {
                    return $gen(c);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }
    wsm_gated!(wsm_fake_post, 0x0030_97e0, ee::s10::f_003097e0__generated);
    wsm_gated!(wsm_fake_value, WSM_VALUE_FROM_FUNC, ee::s05::f_00226600__generated);
    wsm_gated!(wsm_fake_call_init, WSM_CALL_INIT, ee::s05::f_002224a8__generated);
    wsm_gated!(wsm_fake_call_run, WSM_CALL_RUN, ee::s05::f_002224b8__generated);
    wsm_gated!(wsm_fake_table_ctor, WSM_TABLE_CTOR, ee::s05::f_00224da0__generated);
    wsm_gated!(wsm_fake_release, WSM_VALUE_RELEASE, ee::s05::f_00225b90__generated);
    wsm_gated!(wsm_fake_cache_entry, 0x003a_dc50, ee::s13::f_003adc50__generated);
    wsm_gated!(wsm_fake_free, 0x0031_7e50, ee::s10::f_00317e50__generated);

    fn wsm_install_fakes() {
        // The allocator's slot is shared with the PushLoopingAnim fakes.
        asl_install_fakes();
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| unsafe {
            ee::s13::OVERRIDE_003A6B78 = wsm_fake_find_func;
            ee::s10::OVERRIDE_003097E0 = wsm_fake_post;
            ee::s05::OVERRIDE_00226600 = wsm_fake_value;
            ee::s05::OVERRIDE_002224A8 = wsm_fake_call_init;
            ee::s05::OVERRIDE_002224B8 = wsm_fake_call_run;
            ee::s05::OVERRIDE_00224DA0 = wsm_fake_table_ctor;
            ee::s05::OVERRIDE_00225B90 = wsm_fake_release;
            ee::s13::OVERRIDE_003ADC50 = wsm_fake_cache_entry;
            ee::s10::OVERRIDE_00317E50 = wsm_fake_free;
        });
    }

    /// Objects that post the trigger event or not (a real leaf returning 0),
    /// event mode 4 or not, finished riders, no rider; no function (-1), no
    /// owner (the real `CallFunc`), an owner or an instance with a table or
    /// without (a temporary one, the real table release); with and without a
    /// trigger effect. Script-VM callees are logged fakes (one may unwind).
    #[test]
    fn wscript_man_on_object_trigger_matches() {
        wsm_install_fakes();
        covered(
            ee::s10::f_0030a060__generated,
            wscript_man_on_object_trigger,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_a060, 10);
                c.pc = 0x0030_a060;
                c.bus.write32(WSM_CTRL, WSM_MAGIC);
                c.bus.write32(ASL_CTRL, ASL_MAGIC);
                let wsm = DATA + 0x5_0000;
                let volume = DATA + 0x5_1000;
                let trigger = DATA + 0x5_1100;
                let rider = DATA + 0x5_2000;
                let obj = rider + 0x6c0;
                let obj_vt = DATA + 0x5_3000;
                let world = DATA + 0x5_4000;
                let events = DATA + 0x5_4100;
                let owner = DATA + 0x5_5000;
                let instance = DATA + 0x5_5100;
                let fx = DATA + 0x5_6000;
                let fx_vt = DATA + 0x5_6100;
                let cache = DATA + 0x5_7000;
                fill(c, &mut r, wsm, 4);
                c.bus.write32(wsm + WSM_CACHE, cache);
                c.bus.write32(cache + 4, DATA + 0x5_7100);
                for i in 0..0x40 {
                    c.bus.write32(wsm + 0x3cc + 4 * i, r.next_u32());
                }
                c.bus.write32(volume + 0x50, trigger);
                c.bus.write32(trigger + 0x78, r.next_u32());
                c.bus
                    .write32(trigger + 0xc, if r.next_u32() % 3 == 0 { 0 } else { fx });
                c.bus.write32(fx + 0xc, fx_vt);
                c.bus.write16(fx_vt + 0x148, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(fx_vt + 0x14c, FAKE + 0x20);
                // obj: id (slot 0x38) and "posts the event" (slot 0x40).
                c.bus.write32(obj, obj_vt);
                c.bus.write16(obj_vt + 0x38, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(obj_vt + 0x3c, FAKE);
                c.bus.write16(obj_vt + 0x40, 0);
                c.bus.write32(
                    obj_vt + 0x44,
                    if seed % 3 == 0 { 0x0014_06c0 } else { FAKE + 0x10 },
                );
                c.bus.write32(rider + 0x480, (r.next_u32() % 3 == 0) as u32);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, events);
                c.bus
                    .write32(events + 0x214, if r.next_u32() % 3 == 0 { 4 } else { 1 });
                // The function and its owner.
                c.bus.write32(
                    WSM_CTRL + 4,
                    if r.next_u32() % 4 == 0 { u32::MAX } else { r.next_u32() % 0x400 },
                );
                let (o, i) = match (seed / 3) % 4 {
                    0 => (0, 0),
                    1 => (owner, 0),
                    2 => (0, instance),
                    _ => (owner, instance),
                };
                c.bus.write32(wsm + WSM_OWNER, o);
                c.bus.write32(wsm + WSM_INSTANCE, i);
                c.bus.write32(instance + 0x3c, owner);
                c.bus
                    .write32(owner + 0x1c, if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() });
                c.g[A0][0] = sx(wsm);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(volume);
                c.g[A3][0] = sx(if r.next_u32() % 8 == 0 { 0 } else { obj });
            },
        );
    }
}
