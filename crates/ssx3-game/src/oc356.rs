//! Staging module oc356 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc356`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0013_7d18,
    rider_wipeout_state_update_ground,
    "RiderWipeoutState_UpdateGround",
)];

// ---------------------------------------------------------------------------
// RiderWipeoutState_UpdateGround (0x137d18, wipeoutmotion.cpp)

/// Wipeout state: the spin vector (rescaled toward the speed) and the rider.
pub(crate) const WUG_SPIN: u32 = 0x30;

pub(crate) const WUG_RIDER: u32 = 0x40;

/// Wipeout state: 1 once the rider is launched into the air.
pub(crate) const WUG_IN_AIR: u32 = 0x4;

/// Rider fields.
pub(crate) const WUG_R_POS: u32 = 0x110;

pub(crate) const WUG_R_ROOT_POSE_OVERRIDE: u32 = 0x150;

pub(crate) const WUG_R_FRONT: u32 = 0x1b0;

pub(crate) const WUG_R_VEL: u32 = 0x1e0;

pub(crate) const WUG_R_TIME_SCALE: u32 = 0x300;

pub(crate) const WUG_R_GROUND_NORMAL: u32 = 0x370;

pub(crate) const WUG_R_GROUND_VEL: u32 = 0x3d0;

pub(crate) const WUG_R_SURFACE: u32 = 0x438;

pub(crate) const WUG_R_GROUND_HEIGHT: u32 = 0x454;

pub(crate) const WUG_R_CONTACT_POINT: u32 = 0x460;

pub(crate) const WUG_R_ANIM_MODEL: u32 = 0x780;

pub(crate) const WUG_R_HANDLER: u32 = 0x77c;

pub(crate) const WUG_R_ANIM: u32 = 0x784;

pub(crate) const WUG_R_AIR: u32 = 0x788;

/// Surface material record (0xb0 bytes): penetration factor, "reset rider"
/// flag, and the friction curve's four (x, y) points.
pub(crate) const WUG_MAT_SIZE: u32 = 0xb0;

pub(crate) const WUG_MAT_PEN: u32 = 0x18;

pub(crate) const WUG_MAT_RESET: u32 = 0x44;

pub(crate) const WUG_MAT_CURVE: u32 = 0x90;

/// The ground ray's hit record on the stack (`sp + 0xb0`): position, normal,
/// ground velocity, material, object, triangle.
pub(crate) const WUG_HIT: u32 = 0xb0;

pub(crate) const WUG_HIT_NORMAL: u32 = 0xc0;

pub(crate) const WUG_HIT_VEL: u32 = 0xd0;

pub(crate) const WUG_HIT_MATERIAL: u32 = 0xfc;

pub(crate) const WUG_HIT_OBJECT: u32 = 0x100;

pub(crate) const WUG_HIT_TRI: u32 = 0x104;

const WUG_GET_LAYER_ANIM: u32 = 0x0031_1ae8;

const WUG_UPDATE_ORIENTATION: u32 = 0x0012_1aa0;

const WUG_TRIGGER_SPRAY: u32 = 0x0011_1aa0;

const WUG_LAUNCH_INTO_AIR: u32 = 0x0011_35b8;

const WUG_CHECK_RESET: u32 = 0x0011_6120;

const WUG_BUILD_WORLD_MATRIX: u32 = 0x0011_e098;

const WUG_CAST_GROUND_RAY: Callee = Callee {
    addr: 0x0013_8960,
    f: ee::s01::RiderWipeoutState_CastGroundRay,
};

/// `|*v|` with the game's VU0 length sequence (4 components, `vsqrt`), out
/// through GPR `gpr` (`cfc2 gpr, Q`) into `f[fd]`.
fn wug_len(c: &mut Cpu, v: u32, gpr: usize, fd: usize) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.lqc2(3, v);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(gpr, 22);
    c.mtc1(gpr, fd);
}

/// `*a . *b` (4 components) through VU0, out through GPR `gpr` into `f[fd]`.
fn wug_dot(c: &mut Cpu, a: u32, b: u32, gpr: usize, fd: usize) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(gpr, 4);
    c.mtc1(gpr, fd);
}

/// `*dst = *v * f[fs]` through VU0 (the scalar via GPR `gpr`).
fn wug_scale(c: &mut Cpu, v: u32, fs: usize, gpr: usize, dst: u32) {
    use ssx3_ee_rt::vu0::{Src, X};
    c.mfc1(gpr, fs);
    c.lqc2(4, v);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, dst);
}

/// `*dst = *a + *b` (`add`) or `*a - *b` through VU0.
fn wug_add_sub(c: &mut Cpu, a: u32, b: u32, dst: u32, add: bool) {
    use ssx3_ee_rt::vu0::Src;
    c.lqc2(3, a);
    c.lqc2(4, b);
    if add {
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    } else {
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    }
    c.sqc2(5, dst);
}

/// `self->rider` (from s0, as the original reads it each time).
fn wug_rider(c: &mut Cpu) -> u64 {
    Addr(r32(c, S0)).off(WUG_RIDER).lw(c)
}

/// The friction curve (f1 = curve(f2), x in f2, f4 = p1.x on entry; s1 the
/// material): clamped to the first / last point's y, else the linear
/// interpolation between the two points around x.
fn wug_friction_curve(c: &mut Cpu) {
    let m = r32(c, S1);
    let p = |i: u32| m.wrapping_add(WUG_MAT_CURVE + 8 * i);
    c.c_lt_s(4, 2);
    if !c.fcc() {
        // x <= p1.x.
        c.lwc1(1, p(0));
        c.cycles += 4;
        c.c_lt_s(2, 1);
        if c.fcc() {
            // x < p0.x: p0.y.
            c.cycles += 2;
            c.lwc1(1, p(0) + 4);
            return;
        }
        c.lwc1(3, p(0) + 4);
        c.cycles += 2;
        c.sub_s(2, 2, 1);
        c.lwc1(0, p(1) + 4);
        c.cycles += 1;
        c.sub_s(1, 4, 1);
    } else {
        c.cycles += 5;
        c.lwc1(1, p(2));
        c.c_lt_s(1, 2);
        if !c.fcc() {
            // p1.x < x <= p2.x.
            c.lwc1(3, p(1) + 4);
            c.cycles += 4;
            c.sub_s(2, 2, 4);
            c.lwc1(0, p(2) + 4);
            c.sub_s(1, 1, 4);
        } else {
            c.cycles += 5;
            c.lwc1(4, p(3));
            c.c_lt_s(4, 2);
            if c.fcc() {
                // x > p3.x: p3.y.
                c.cycles += 2;
                c.lwc1(1, p(3) + 4);
                return;
            }
            // p2.x < x <= p3.x.
            c.lwc1(3, p(2) + 4);
            c.cycles += 3;
            c.sub_s(2, 2, 1);
            c.lwc1(0, p(3) + 4);
            c.cycles += 1;
            c.sub_s(1, 4, 1);
        }
    }
    // y = y0 + (y1 - y0) * (x - x0) / (x1 - x0).
    c.cycles += 4;
    c.sub_s(0, 0, 3);
    c.mul_s(0, 0, 2);
    c.div_s(0, 0, 1);
    c.add_s(1, 3, 0);
}

/// `void RiderWipeoutState_UpdateGround(RiderWipeoutState* self)` (0x137d18):
/// one tick of a wiped-out rider tumbling along the ground (`dt` = time
/// scale / 60). The spin vector eases toward the speed; the sliding
/// velocity (relative to the ground's) loses its tangential part through
/// the surface's friction curve (stronger at low speed in the stick poses
/// 0x1d / 0x19 or with the root pose overridden, scaled by how flat the
/// ground is, at most 1/dt); gravity; position and velocity integrate, the
/// speed is clamped to 3333.33; the rider turns toward front + velocity.
/// Then a ground ray: a hit takes the ground's velocity, normal, surface and
/// contact point, and by the rider's penetration either pushes him out
/// (deep: velocity into the ground removed, spray), springs him back
/// (shallow: spray), or launches him (more than 10 above); a hit object's
/// owner gets the impulse (vtable slot 0x158); a "reset" surface or a
/// triangle with flag 2 checks for a reset. No hit launches him into the
/// air. Finally the world matrix is rebuilt.
pub fn rider_wipeout_state_update_ground(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 44;
        let frame = Frame::open(c, 0x190);
        let s = sp(c);
        c.lwc1(2, gp.wrapping_sub(0x7258)); // 1/60
        frame.save(c, S0, 0x160);
        frame.save_fpr(c, 22, 0x180);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x150);
        frame.save(c, S2, 0x140);
        frame.save_ra(c, 0x130);
        frame.save_fpr(c, 21, 0x178);
        frame.save_fpr(c, 20, 0x170);
        let me = r32(c, S0);

        // Spin length eases toward |vel| * 0.0125: len + (target - len) * 5 * dt.
        wug_len(c, me + WUG_SPIN, V1, 5);
        c.lwc1(3, gp.wrapping_sub(0x7254));
        c.g[V0][0] = Addr(me + WUG_RIDER).lw(c);
        let rider = r32(c, V0);
        wug_len(c, rider + WUG_R_VEL, V1, 0);
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(AT, 4);
        c.mul_s(0, 0, 2);
        c.lwc1(1, rider + WUG_R_TIME_SCALE);
        c.lwc1(2, gp.wrapping_sub(0x7250));
        c.mul_s(22, 1, 3); // dt
        c.sub_s(0, 0, 5);
        c.c_lt_s(2, 5);
        c.mul_s(0, 0, 4);
        c.mul_s(0, 0, 22);
        let rescale = c.fcc();
        c.add_s(0, 5, 0);
        if rescale {
            c.cycles += 6;
            c.div_s(0, 0, 5);
            wug_scale(c, me + WUG_SPIN, 0, V0, me + WUG_SPIN);
        }

        // Tangential sliding velocity and the surface's friction.
        c.cycles += 54;
        c.g[A1][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
        c.g[A2][0] = WUG_MAT_SIZE as u64;
        c.g[V0][0] = Addr(me + WUG_RIDER).lw(c);
        c.g[V1][0] = Addr(r32(c, A1) + 0x84).lw(c);
        let rider = r32(c, V0);
        c.g[A0][0] = Addr(rider + WUG_R_SURFACE).lw(c);
        c.g[A1][0] = Addr(r32(c, V1) + 0x44).lw(c);
        wug_add_sub(
            c,
            rider + WUG_R_VEL,
            rider + WUG_R_GROUND_VEL,
            s + 0x10,
            false,
        );
        c.mult(0, A0, A0, A2);
        c.lq(V1, s + 0x10);
        c.g[V0][0] = Addr(me + WUG_RIDER).lw(c);
        c.sq(V1, s);
        let rider = r32(c, V0);
        wug_dot(c, s, rider + WUG_R_GROUND_NORMAL, A2, 0);
        c.lwc1(1, gp.wrapping_sub(0x724c));
        wug_scale(c, rider + WUG_R_GROUND_NORMAL, 0, V1, s + 0x20);
        c.g[S1][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        c.lq(V0, s + 0x20);
        c.sq(V0, s + 0x10);
        wug_add_sub(c, s, s + 0x10, s + 0x30, false);
        c.lq(V0, s + 0x30);
        c.sq(V0, s + 0x20); // tangent
        wug_len(c, s + 0x20, V1, 21);
        c.mul_s(2, 21, 1);
        c.lwc1(4, r32(c, S1) + WUG_MAT_CURVE + 8);
        wug_friction_curve(c);

        // friction = curve * 3 + 1.5; at low speed in a stick pose, blend
        // toward 40 by (1 - tl * 0.0009)^4.
        c.cycles += 10;
        c.g[AT][0] = 0x4040_0000;
        c.mtc1(AT, 0);
        c.lwc1(2, gp.wrapping_sub(0x7248));
        c.mul_s(0, 1, 0);
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(AT, 1);
        c.c_lt_s(21, 2);
        let slow = c.fcc();
        c.add_s(20, 0, 1);
        let mut reload = true;
        if slow {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, S0) + WUG_RIDER).lw(c);
            c.g[V0][0] = Addr(r32(c, V1) + WUG_R_ROOT_POSE_OVERRIDE).lw(c);
            let mut stick = c.g[V0][0] != 0;
            if !stick {
                c.cycles += 6;
                c.g[A0][0] = Addr(r32(c, V1) + WUG_R_ANIM).lw(c);
                c.g[A1][0] = 2;
                call_ptr(c, WUG_GET_LAYER_ANIM, 0x0013_7f7c)?;
                c.g[V1][0] = 0x1d;
                stick = c.g[V0][0] == c.g[V1][0];
                c.g[A1][0] = 2;
                if !stick {
                    c.cycles += 6;
                    c.g[V0][0] = wug_rider(c);
                    c.g[A0][0] = Addr(r32(c, V0) + WUG_R_ANIM).lw(c);
                    call_ptr(c, WUG_GET_LAYER_ANIM, 0x0013_7f94)?;
                    c.g[V1][0] = 0x19;
                    stick = c.g[V0][0] == c.g[V1][0];
                    if !stick {
                        c.g[V0][0] = wug_rider(c);
                        reload = false;
                    }
                }
            }
            if stick {
                c.cycles += 13;
                c.lwc1(0, gp.wrapping_sub(0x7244));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 1);
                c.mul_s(0, 21, 0);
                c.g[AT][0] = 0x4220_0000;
                c.mtc1(AT, 2);
                c.sub_s(0, 1, 0);
                c.mul_s(0, 0, 0);
                c.mul_s(0, 0, 0);
                c.sub_s(1, 1, 0);
                c.mul_s(0, 0, 2);
                c.mul_s(1, 1, 20);
                c.add_s(20, 0, 1);
            }
        }
        if reload {
            c.cycles += 1;
            c.g[V0][0] = wug_rider(c);
        }

        // friction *= clamp(normal.z + 0.3, 0, 1); at most 1 / dt. f21 = 0.
        c.cycles += 10;
        c.lwc1(1, gp.wrapping_sub(0x7240));
        c.lwc1(0, r32(c, V0) + WUG_R_GROUND_NORMAL + 8);
        c.mtc1(0, 21);
        c.add_s(0, 0, 1);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.c_le_s(21, 0);
        let positive = c.fcc();
        c.mov_s(2, 21);
        if positive {
            c.cycles += 1;
            c.min_s(2, 0, 1);
        }
        c.cycles += 6;
        c.div_s(0, 1, 22);
        c.mul_s(20, 20, 2);
        c.c_lt_s(0, 20);
        if c.fcc() {
            c.mov_s(20, 0);
        }

        // acc = gravity * -1800 + tangent * -friction; pos += vel * dt;
        // vel += acc * dt; |vel| <= 3333.33.
        c.cycles += 66;
        c.g[V1][0] = 0x50_0000;
        c.g[AT][0] = 0xffff_ffff_c4e1_0000;
        c.mtc1(AT, 0);
        c.g[V1][0] = sx(0x004f_f160);
        c.neg_s(1, 20);
        wug_scale(c, 0x004f_f160, 0, V0, s + 0x40);
        c.lwc1(20, gp.wrapping_sub(0x723c));
        c.lq(V0, s + 0x40);
        c.sq(V0, s + 0x30);
        wug_scale(c, s + 0x20, 1, V1, s + 0x50);
        c.lq(V0, s + 0x50);
        c.sq(V0, s + 0x40);
        wug_add_sub(c, s + 0x30, s + 0x40, s + 0x60, true);
        c.lq(V0, s + 0x60);
        c.g[V1][0] = wug_rider(c);
        c.sq(V0, s + 0x50);
        let rider = r32(c, V1);
        wug_scale(c, rider + WUG_R_VEL, 22, A0, s + 0x70);
        c.lq(V0, s + 0x70);
        c.g[V1][0] = wug_rider(c);
        c.sq(V0, s + 0x60);
        let rider = r32(c, V1);
        wug_add_sub(c, rider + WUG_R_POS, s + 0x60, rider + WUG_R_POS, true);
        wug_scale(c, s + 0x50, 22, A0, s + 0x70);
        c.lq(V0, s + 0x70);
        c.sq(V0, s + 0x60);
        c.g[V1][0] = wug_rider(c);
        let rider = r32(c, V1);
        wug_add_sub(c, rider + WUG_R_VEL, s + 0x60, rider + WUG_R_VEL, true);
        c.g[V1][0] = wug_rider(c);
        let rider = r32(c, V1);
        wug_len(c, rider + WUG_R_VEL, V0, 0);
        c.c_lt_s(20, 0);
        if c.fcc() {
            c.cycles += 8;
            c.div_s(0, 20, 0);
            wug_scale(c, rider + WUG_R_VEL, 0, V0, rider + WUG_R_VEL);
            c.g[V1][0] = wug_rider(c);
        }
        c.g[A2][0] = sx(s + 0x60);

        // Turn toward normalize(front + vel * 0.0036).
        c.cycles += 46;
        let rider = r32(c, V1);
        c.lwc1(0, gp.wrapping_sub(0x7238));
        c.lq(V0, rider + WUG_R_FRONT);
        c.lwc1(13, gp.wrapping_sub(0x7234));
        c.sq(V0, s + 0x80);
        wug_scale(c, rider + WUG_R_VEL, 0, A0, s + 0xa0);
        c.lq(V0, s + 0xa0);
        c.sq(V0, s + 0x90);
        wug_add_sub(c, s + 0x80, s + 0x90, s + 0xa0, true);
        c.lq(V0, s + 0xa0);
        c.sq(V0, s + 0x70);
        {
            use ssx3_ee_rt::vu0::{Src, W, Y, Z};
            c.lqc2(3, s + 0x70);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, s + 0xa0);
        }
        c.lq(V0, s + 0xa0);
        c.g[V1][0] = wug_rider(c);
        c.sq(V0, s + 0x60);
        c.g[A0][0] = c.g[V1][0];
        let rider = r32(c, V1);
        c.lwc1(12, rider + WUG_R_TIME_SCALE);
        c.g[A1][0] = sx(rider + WUG_R_GROUND_NORMAL);
        c.mul_s(13, 12, 13);
        c.add_s(12, 12, 12);
        call_ptr(c, WUG_UPDATE_ORIENTATION, 0x0013_81d4)?;

        // The ground ray.
        c.g[A2][0] = wug_rider(c);
        c.g[A1][0] = sx(sp(c) + WUG_HIT);
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx(r32(c, A2) + WUG_R_GROUND_NORMAL);
        WUG_CAST_GROUND_RAY.call(c, 0x0013_81ec)?;
        let s = sp(c);
        c.c_le_s(21, 0);
        let hit = c.fcc();
        c.lq(V0, s + WUG_HIT_VEL);
        if hit {
            wug_ground_hit(c)?;
        } else {
            c.cycles += 8;
            c.g[A1][0] = wug_rider(c);
            wug_launch(c, 0x0013_8608)?;
        }

        c.cycles += 11;
        c.g[A0][0] = wug_rider(c);
        call_ptr(c, WUG_BUILD_WORLD_MATRIX, 0x0013_8618)?;
        frame.restore(c, S0, 0x160);
        frame.restore(c, S1, 0x150);
        frame.restore(c, S2, 0x140);
        frame.restore_ra(c, 0x130);
        frame.restore_fpr(c, 22, 0x180);
        frame.restore_fpr(c, 21, 0x178);
        frame.restore_fpr(c, 20, 0x170);
        frame.close(c);
        Ok(())
    })
}

/// `AirPredictor_StartLaunchIntoAir(rider->air, &rider->pos, &rider->vel,
/// 3333.33)` (a1 = rider), then `self->inAir = 1`.
fn wug_launch(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.mov_s(12, 20);
    let rider = r32(c, A1);
    c.g[A0][0] = Addr(rider + WUG_R_AIR).lw(c);
    c.g[A2][0] = sx(rider + WUG_R_VEL);
    c.g[A1][0] = sx(rider + WUG_R_POS);
    call_ptr(c, WUG_LAUNCH_INTO_AIR, ret)?;
    c.g[V0][0] = 1;
    c.sw(r32(c, S0) + WUG_IN_AIR, 1);
    Ok(())
}

/// The ground ray hit (v0 = the hit's ground velocity, 128 bits): take the
/// ground, react to the penetration, push the hit object's owner, check
/// for a reset.
fn wug_ground_hit(c: &mut Cpu) -> crate::call::Flow {
    let s = sp(c);
    c.cycles += 53;
    c.g[V1][0] = wug_rider(c);
    c.sq(V0, r32(c, V1) + WUG_R_GROUND_VEL);
    c.g[A0][0] = wug_rider(c);
    c.lq(V0, s + WUG_HIT_NORMAL);
    c.sq(V0, r32(c, A0) + WUG_R_GROUND_NORMAL);
    c.g[V1][0] = wug_rider(c);
    c.g[V0][0] = Addr(s + WUG_HIT_MATERIAL).lw(c);
    c.sw(r32(c, V1) + WUG_R_SURFACE, c.g[V0][0]);
    c.g[A0][0] = wug_rider(c);
    c.lq(V0, s + WUG_HIT);
    c.sq(V0, r32(c, A0) + WUG_R_CONTACT_POINT);
    c.g[V1][0] = wug_rider(c);
    let rider = r32(c, V1);
    wug_add_sub(c, rider + WUG_R_POS, s + WUG_HIT, s + 0x80, false);
    c.lq(A0, s + 0x80);
    c.g[V0][0] = wug_rider(c);
    c.sq(A0, s + 0x70);
    let rider = r32(c, V0);
    // Height above the ground.
    wug_dot(c, s + 0x70, rider + WUG_R_GROUND_NORMAL, V1, 0);
    c.swc1(0, rider + WUG_R_GROUND_HEIGHT);
    c.g[V1][0] = wug_rider(c);
    let rider = r32(c, V1);
    c.lwc1(1, r32(c, S1) + WUG_MAT_PEN);
    c.g[V0][0] = Addr(rider + WUG_R_ANIM_MODEL).lw(c);
    c.lwc1(0, r32(c, V0) + 0x140);
    wug_add_sub(c, rider + WUG_R_POS, s + WUG_HIT, s + 0x80, false);
    c.mul_s(2, 0, 1); // limit
    c.lq(V0, s + 0x80);
    c.sq(V0, s + 0x70);
    c.neg_s(0, 2);
    // Penetration along the hit normal (f1).
    wug_dot(c, s + 0x70, s + WUG_HIT_NORMAL, V0, 1);
    c.c_lt_s(1, 0);
    let spray;
    if c.fcc() {
        // Deep: drop the velocity into the ground, push out to the limit.
        c.cycles += 24;
        c.g[V0][0] = wug_rider(c);
        let rider = r32(c, V0);
        wug_len(c, s + WUG_HIT_VEL, V1, 12);
        wug_dot(c, rider + WUG_R_VEL, s + WUG_HIT_NORMAL, A0, 0);
        c.c_lt_s(0, 21);
        if c.fcc() {
            c.cycles += 13;
            wug_scale(c, s + WUG_HIT_NORMAL, 0, V0, s + 0x80);
            c.lq(V1, s + 0x80);
            c.g[V0][0] = wug_rider(c);
            c.sq(V1, s + 0x70);
            let rider = r32(c, V0);
            wug_add_sub(c, rider + WUG_R_VEL, s + 0x70, rider + WUG_R_VEL, false);
        }
        c.add_s(0, 1, 2);
        c.cycles += 16;
        c.g[A1][0] = c.g[S2][0];
        wug_scale(c, s + WUG_HIT_NORMAL, 0, V0, s + 0x80);
        c.g[A2][0] = sx(s + WUG_HIT_NORMAL);
        c.lq(V1, s + 0x80);
        c.g[V0][0] = wug_rider(c);
        c.sq(V1, s + 0x70);
        let rider = r32(c, V0);
        wug_add_sub(c, rider + WUG_R_POS, s + 0x70, rider + WUG_R_POS, false);
        c.g[V1][0] = wug_rider(c);
        spray = true;
    } else {
        c.cycles += 4;
        c.c_lt_s(1, 21);
        let below = c.fcc();
        c.g[A1][0] = c.g[S2][0];
        if below {
            // Shallow: spring back, vel += n * dt * (pen * -2700 / limit - vel . n).
            c.cycles += 39;
            c.lwc1(0, r32(c, GP).wrapping_sub(0x7230));
            c.g[V0][0] = wug_rider(c);
            c.g[A2][0] = sx(s + WUG_HIT_NORMAL);
            c.mul_s(0, 1, 0);
            let rider = r32(c, V0);
            wug_len(c, rider + WUG_R_VEL, A0, 12);
            wug_dot(c, rider + WUG_R_VEL, rider + WUG_R_GROUND_NORMAL, V1, 1);
            c.div_s(0, 0, 2);
            c.sub_s(0, 0, 1);
            c.mul_s(0, 22, 0);
            wug_scale(c, rider + WUG_R_GROUND_NORMAL, 0, A0, s + 0x80);
            c.lq(V1, s + 0x80);
            c.g[V0][0] = wug_rider(c);
            c.sq(V1, s + 0x70);
            let rider = r32(c, V0);
            wug_add_sub(c, rider + WUG_R_VEL, s + 0x70, rider + WUG_R_VEL, true);
            c.g[V1][0] = wug_rider(c);
            spray = true;
        } else {
            spray = false;
            c.cycles += 6;
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(AT, 0);
            c.c_lt_s(0, 1);
            let above = c.fcc();
            c.g[A0][0] = Addr(s + WUG_HIT_OBJECT).lw(c);
            if above {
                // More than 10 above: drop the velocity into the ground, launch.
                c.cycles += 14;
                c.g[V1][0] = wug_rider(c);
                let rider = r32(c, V1);
                wug_dot(c, rider + WUG_R_VEL, rider + WUG_R_GROUND_NORMAL, V0, 0);
                c.c_lt_s(0, 21);
                if c.fcc() {
                    c.cycles += 13;
                    wug_scale(c, rider + WUG_R_GROUND_NORMAL, 0, V0, s + 0x80);
                    c.lq(V1, s + 0x80);
                    c.g[V0][0] = wug_rider(c);
                    c.sq(V1, s + 0x70);
                    let rider = r32(c, V0);
                    wug_add_sub(c, rider + WUG_R_VEL, s + 0x70, rider + WUG_R_VEL, false);
                }
                c.g[A1][0] = wug_rider(c);
                c.cycles += 8;
                wug_launch(c, 0x0013_84fc)?;
                c.g[A0][0] = Addr(s + WUG_HIT_OBJECT).lw(c);
            }
        }
    }
    if spray {
        // RiderBase_TriggerDynamicSpray(rider->handler, &hit, &hit.normal,
        // hit.material, speed).
        c.cycles += 5;
        c.g[A3][0] = Addr(s + WUG_HIT_MATERIAL).lw(c);
        c.g[A0][0] = Addr(r32(c, V1) + WUG_R_HANDLER).lw(c);
        call_ptr(c, WUG_TRIGGER_SPRAY, 0x0013_845c)?;
        c.g[A0][0] = Addr(s + WUG_HIT_OBJECT).lw(c);
    }

    // The hit object's owner takes the impulse (v - vel + groundVel) / dt.
    c.cycles += 2;
    let object = c.g[A0][0];
    c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
    if object != 0 {
        c.cycles += 3;
        c.g[T0][0] = Addr(object as u32 + 0xc).lw(c);
        c.g[A1][0] = c.g[S2][0];
        if c.g[T0][0] != 0 {
            c.cycles += 31;
            let owner = r32(c, T0);
            c.g[A3][0] = Addr(owner + 0xc).lw(c);
            c.g[V1][0] = wug_rider(c);
            c.g[A2][0] = sx(s + 0x70);
            let vt = r32(c, A3);
            c.g[A0][0] = Addr(vt + 0x158).lh(c);
            let rider = r32(c, V1);
            wug_add_sub(c, s, rider + WUG_R_VEL, s + 0xa0, false);
            c.lq(V0, s + 0xa0);
            c.g[A0][0] = sx(owner.wrapping_add(r32(c, A0)));
            c.g[V1][0] = wug_rider(c);
            c.sq(V0, s + 0x90);
            let rider = r32(c, V1);
            wug_add_sub(c, s + 0x90, rider + WUG_R_GROUND_VEL, s + 0xa0, true);
            c.lq(V0, s + 0xa0);
            c.sq(V0, s + 0x80);
            {
                use ssx3_ee_rt::vu0::Src;
                c.mfc1(V1, 22);
                c.qmtc2(V1, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, s + 0x80);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 4, 4, Src::Q);
                c.sqc2(4, s + 0xa0);
            }
            c.lq(V0, s + 0xa0);
            c.sq(V0, s + 0x70);
            c.g[V1][0] = Addr(vt + 0x15c).lw(c);
            let method = r32(c, V1);
            call_ptr(c, method, 0x0013_8594)?;
            c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        }
    }

    // A "reset" ground material, or a triangle flagged 2: check for a reset.
    c.cycles += 9;
    c.g[A1][0] = WUG_MAT_SIZE as u64;
    c.g[V1][0] = Addr(s + WUG_HIT_MATERIAL).lw(c);
    c.g[A0][0] = Addr(r32(c, V0) + 0x84).lw(c);
    c.mult(0, V1, V1, A1);
    c.g[V0][0] = Addr(r32(c, A0) + 0x44).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.g[A0][0] = Addr(r32(c, V0) + WUG_MAT_RESET).lw(c);
    let mut reset = c.g[A0][0] != 0;
    if !reset {
        c.cycles += 3;
        c.g[V0][0] = Addr(s + WUG_HIT_TRI).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, V0) + 0xa).lh(c);
            c.g[V0][0] &= 2;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                reset = true;
            }
        }
    }
    if reset {
        c.g[A0][0] = wug_rider(c);
        c.cycles += 5;
        c.g[A1][0] = 0;
        c.g[A2][0] = 1;
        call_ptr(c, WUG_CHECK_RESET, 0x0013_85e8)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{elf_image, put_vec, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's fakes (off: the slot's previous body runs).
    pub(crate) const WUG_CTRL: u32 = DATA + 0x6_8000;

    pub(crate) const WUG_MAGIC: u32 = 0x137d_18a5;

    macro_rules! wug_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(WUG_CTRL) != WUG_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    static WUG_PREV_LAYER_ANIM: AtomicUsize = AtomicUsize::new(0);

    /// `AnimController_GetLayerAnimEntry`: logged, returns the anim id the
    /// fixture puts at `WUG_CTRL + 4`.
    fn wug_fake_layer_anim(c: &mut Cpu) {
        if c.bus.read32(WUG_CTRL) != WUG_MAGIC {
            return fakes::fall_back(c, &WUG_PREV_LAYER_ANIM, ee::s10::f_00311ae8__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, WUG_GET_LAYER_ANIM);
        if c.pc == ra {
            c.g[V0][0] = c.bus.read32(WUG_CTRL + 4) as u64;
        }
    }

    wug_gated!(
        wug_fake_orientation,
        WUG_PREV_ORIENTATION,
        WUG_UPDATE_ORIENTATION,
        ee::s00::f_00121aa0__generated
    );

    wug_gated!(
        wug_fake_spray,
        WUG_PREV_SPRAY,
        WUG_TRIGGER_SPRAY,
        ee::s00::f_00111aa0__generated
    );

    wug_gated!(
        wug_fake_check_reset,
        WUG_PREV_CHECK_RESET,
        WUG_CHECK_RESET,
        ee::s00::f_00116120__generated
    );

    wug_gated!(
        wug_fake_world_matrix,
        WUG_PREV_WORLD_MATRIX,
        WUG_BUILD_WORLD_MATRIX,
        ee::s00::f_0011e098__generated
    );

    #[allow(unused_unsafe)]
    fn wug_install_fakes() {
        use std::ptr::addr_of_mut;
        // The ground ray and the air launch: PostUpdateAir's gated fakes.
        crate::game::ai::motion::wipeoutmotion::tests::pua_install_fakes();
        unsafe {
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00311AE8),
                wug_fake_layer_anim,
                &WUG_PREV_LAYER_ANIM,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_00121AA0),
                wug_fake_orientation,
                &WUG_PREV_ORIENTATION,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_00111AA0),
                wug_fake_spray,
                &WUG_PREV_SPRAY,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_00116120),
                wug_fake_check_reset,
                &WUG_PREV_CHECK_RESET,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0011E098),
                wug_fake_world_matrix,
                &WUG_PREV_WORLD_MATRIX,
            );
        }
    }

    fn wug_dot3(a: [f32; 4], b: [f32; 4]) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
    }

    /// RiderWipeoutState_UpdateGround: spin rescaled or not; the friction
    /// curve's five ranges (points placed around this seed's tangential
    /// speed); stick poses (root pose override, layer anim 0x1d / 0x19 /
    /// other / none) at low and high speed; ground steep, mid and flat; friction above 1/dt;
    /// the speed clamp; the ray missing or hitting with a deep, shallow,
    /// small or large (launch) penetration and the velocity into or out of
    /// the ground; hit object none / ownerless / with an impulse handler;
    /// reset material or flagged triangle. Callees are gated logged fakes
    /// (PostUpdateAir's for the ground ray and the launch), one may unwind.
    #[test]
    fn rider_wipeout_state_update_ground_matches() {
        use crate::game::ai::computer::tests::{WT_CTRL, WT_MAGIC};
        use crate::game::ai::motion::wipeoutmotion::tests::{
            PUA_CTRL, PUA_HIT_B_TEMPLATE, PUA_MAGIC,
        };
        const SELF: u32 = DATA + 0x6_0000;
        const RIDER: u32 = DATA + 0x6_1000;
        const MODEL: u32 = DATA + 0x6_2000;
        const WORLD: u32 = DATA + 0x6_3000;
        const MATS: u32 = DATA + 0x6_4000;
        const TRI: u32 = DATA + 0x6_5000;
        const OBJ: u32 = DATA + 0x6_6000;
        const OWNER: u32 = OBJ + 0x100;
        const OWNER_VT: u32 = OBJ + 0x200;
        wug_install_fakes();
        covered(
            ee::s01::f_00137d18__generated,
            rider_wipeout_state_update_ground,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0013_7d18, 8);
                c.pc = 0x0013_7d18;
                c.bus.write32(WUG_CTRL, WUG_MAGIC);
                c.bus.write32(PUA_CTRL, PUA_MAGIC);
                // ComputerActorRider's wipeout fakes (put back by their test on
                // 0x111aa0 / 0x11e098) log just like ours.
                c.bus.write32(WT_CTRL, WT_MAGIC);
                let pick = |r: &mut Rng, v: &[f32]| v[r.next_u32() as usize % v.len()];
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + WUG_IN_AIR, r.next_u32());
                c.bus.write32(SELF + WUG_RIDER, RIDER);
                let spin = [0.0f32, 0.0001, -3.0, 5.0];
                put_vec(
                    c,
                    SELF + WUG_SPIN,
                    [
                        pick(&mut r, &spin).to_bits(),
                        pick(&mut r, &spin).to_bits(),
                        pick(&mut r, &spin).to_bits(),
                        0,
                    ],
                );

                // Ground velocity u, sliding velocity t, vel = t + u.
                let big = [0.0f32, 0.0, 50.0, -50.0, 3000.0, -3000.0];
                let u = [pick(&mut r, &big), pick(&mut r, &big), 0.0, 0.0];
                let tc = [
                    -800.0f32, -300.0, -40.0, -10.0, 0.0, 10.0, 40.0, 300.0, 800.0,
                ];
                let vz = [-600.0f32, -50.0, 0.0, 50.0, 600.0];
                let t = [
                    pick(&mut r, &tc),
                    pick(&mut r, &tc) + 1.0,
                    pick(&mut r, &vz),
                    0.0,
                ];
                let vel = [t[0] + u[0], t[1] + u[1], t[2] + u[2], 0.0];
                let n = [
                    pick(&mut r, &[0.0, 0.0, 0.3]),
                    pick(&mut r, &[0.0, 0.3]),
                    pick(&mut r, &[1.0, 1.0, 0.2, -0.5]),
                    0.0,
                ];
                let bits = |v: [f32; 4]| v.map(f32::to_bits);
                put_vec(c, RIDER + WUG_R_VEL, bits(vel));
                put_vec(c, RIDER + WUG_R_GROUND_VEL, bits(u));
                put_vec(c, RIDER + WUG_R_GROUND_NORMAL, bits(n));
                let ts = pick(&mut r, &[1.0, 1.0, 0.5, 2.0]);
                c.bus.write32(RIDER + WUG_R_TIME_SCALE, ts.to_bits());
                put_vec(
                    c,
                    RIDER + WUG_R_FRONT,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0],
                );

                // The friction curve around this seed's x = |tangent| * 0.036.
                let v = [vel[0] - u[0], vel[1] - u[1], vel[2] - u[2], 0.0];
                let vn = wug_dot3(v, n);
                let tang = [v[0] - n[0] * vn, v[1] - n[1] * vn, v[2] - n[2] * vn, 0.0];
                let x = wug_dot3(tang, tang).sqrt() * 0.036;
                let ranges = [
                    [1.5f32, 2.0, 3.0, 4.0],
                    [0.5, 1.5, 2.0, 3.0],
                    [0.3, 0.6, 1.5, 2.0],
                    [0.2, 0.4, 0.7, 1.5],
                    [0.1, 0.2, 0.3, 0.5],
                ];
                let xs = ranges[(seed % 5) as usize];
                let lim_k = pick(&mut r, &[0.5, 1.0, 2.0]);
                for m in 0..4 {
                    let mat = MATS + m * WUG_MAT_SIZE;
                    for i in 0..4u32 {
                        let p = mat + WUG_MAT_CURVE + 8 * i;
                        c.bus.write32(p, (x * xs[i as usize]).to_bits());
                        c.bus
                            .write32(p + 4, (f32::from_bits(r.f32(1.0)).abs() * 30.0).to_bits());
                    }
                    c.bus.write32(mat + WUG_MAT_PEN, lim_k.to_bits());
                    c.bus
                        .write32(mat + WUG_MAT_RESET, (r.next_u32() % 3 == 0) as u32);
                }
                c.bus.write32(GAME_GP_WORLD, WORLD);
                c.bus.write32(WORLD + 0x84, WORLD + 0x100);
                c.bus.write32(WORLD + 0x100 + 0x44, MATS);
                c.bus.write32(RIDER + WUG_R_SURFACE, r.next_u32() % 4);

                // Stick pose: root pose override, or the layer anim's id.
                c.bus.write32(
                    RIDER + WUG_R_ROOT_POSE_OVERRIDE,
                    (r.next_u32() % 4 == 0) as u32,
                );
                c.bus.write32(RIDER + WUG_R_ANIM, r.next_u32());
                c.bus.write32(
                    WUG_CTRL + 4,
                    [0x1du32, 0x19, 3, 0][r.next_u32() as usize % 4],
                );
                c.bus.write32(RIDER + WUG_R_ANIM_MODEL, MODEL);
                c.bus.write32(MODEL + 0x140, 1.0f32.to_bits());
                c.bus.write32(RIDER + WUG_R_HANDLER, r.next_u32());
                c.bus.write32(RIDER + WUG_R_AIR, r.next_u32());

                // The ray: miss, or a hit at the origin with normal (0,0,s)
                // and the rider at penetration `pen` after the move.
                c.bus
                    .write32(PUA_CTRL + 8, pick(&mut r, &[-1.0, 0.0, 2.0, 2.0]).to_bits());
                let sign = pick(&mut r, &[1.0, -1.0]);
                let lim = lim_k;
                let pen = pick(&mut r, &[-3.0 * lim, -0.5 * lim, 5.0, 25.0]);
                let dt = ts / 60.0;
                let pz = pen * sign - vel[2] * dt;
                put_vec(
                    c,
                    RIDER + WUG_R_POS,
                    [r.f32(100.0), r.f32(100.0), pz.to_bits(), 0],
                );
                let hit = PUA_HIT_B_TEMPLATE;
                for w in 0..0x18 {
                    c.bus.write32(hit + 4 * w, r.next_u32());
                }
                put_vec(c, hit, [0, 0, 0, 0]);
                put_vec(c, hit + 0x10, bits([0.0, 0.0, sign, 0.0]));
                put_vec(c, hit + 0x20, [r.f32(50.0), r.f32(50.0), r.f32(5.0), 0]);
                c.bus.write32(hit + 0x4c, r.next_u32() % 4);
                let object = match r.next_u32() % 3 {
                    0 => 0,
                    1 => {
                        c.bus.write32(OBJ + 0xc, 0);
                        OBJ
                    }
                    _ => {
                        c.bus.write32(OBJ + 0xc, OWNER);
                        OBJ
                    }
                };
                c.bus.write32(hit + 0x50, object);
                c.bus.write32(OWNER + 0xc, OWNER_VT);
                c.bus
                    .write16(OWNER_VT + 0x158, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(OWNER_VT + 0x15c, fakes::FAKE + 0x20);
                let tri = if r.next_u32() % 2 == 0 { 0 } else { TRI };
                c.bus.write32(hit + 0x54, tri);
                c.bus.write16(TRI + 0xa, r.next_u32() as u16);
            },
        );
    }

    /// `gp - 0x848` (the world) with the test gp.
    const GAME_GP_WORLD: u32 = crate::difftest::GAME_GP - 0x848;
}
