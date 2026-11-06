//! Staging wave 210 (oc280): hand-written replacements for the hottest
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
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_57b8, rider_bounce_off_wall, "Rider_BounceOffWall"),
];

// ---------------------------------------------------------------------------
// Rider_BounceOffWall (0x1057b8)

/// `Rider` fields read or written by `Rider_BounceOffWall`.
mod bow_rider {
    /// `+0x180` the board's facing direction (`unk_180`).
    pub const FACING: u32 = 0x180;
    /// `+0x1e0 mVelocity`.
    pub const VEL: u32 = 0x1e0;
    /// `+0x2e4` the value handed to `AirPredictor_Prime` (`unk_2E4`).
    pub const AIR_PRIME_ARG: u32 = 0x2e4;
    /// `+0x320` current stance, `+0x324 mNaturalStance`.
    pub const STANCE: u32 = 0x320;
    pub const NATURAL_STANCE: u32 = 0x324;
    /// `+0x3f4 mImpactAccum2`: slow wall hits accumulate here.
    pub const IMPACT_ACCUM: u32 = 0x3f4;
    /// `+0x438 mSurfaceType`.
    pub const SURFACE_TYPE: u32 = 0x438;
    /// `+0x6c0` the `RiderIface` sub-object (its vtable pointer first).
    pub const IFACE: u32 = 0x6c0;
    /// `+0x77c mRiderHandler` (`+0x30` its control mode).
    pub const HANDLER: u32 = 0x77c;
    /// `+0x784 mRiderAnim`, `+0x788 mAirPredictor`, `+0x790 mMetrix`.
    pub const ANIM: u32 = 0x784;
    pub const AIR_PREDICTOR: u32 = 0x788;
    pub const METRIX: u32 = 0x790;
}

/// The `RiderIface` vtable slots (`{i16 delta; fn}` pairs): `+0x88`
/// vibrationBump(amount), `+0x28` getPosition, `+0x10` getVelocity.
mod bow_iface {
    pub const VIBRATION_BUMP: u32 = 0x88;
    pub const GET_POSITION: u32 = 0x28;
    pub const GET_VELOCITY: u32 = 0x10;
}

/// The wall hit record (`a3`).
mod bow_hit {
    /// `+0x10` the wall normal, `+0x20` the wall's own velocity.
    pub const NORMAL: u32 = 0x10;
    pub const NORMAL_Z: u32 = 0x18;
    pub const VEL: u32 = 0x20;
    /// `+0x4c` the material handed to `Rider_HandleImpact`.
    pub const MATERIAL: u32 = 0x4c;
    /// `+0x5c` index into the collision object's surface table.
    pub const SURFACE: u32 = 0x5c;
}

/// The collision object (`a1`): `+0xc` an optional handle whose vtable slot
/// `+0x70` says whether the rider is pushed out of the wall, `+0x88` the
/// surface table (12-byte `{bounce, grip, ...}` entries from `+0x10`).
mod bow_obj {
    pub const HANDLE: u32 = 0xc;
    pub const SURFACES: u32 = 0x88;
    pub const HANDLE_PUSH_SLOT: u32 = 0x70;
}

/// `.lit4` constants (gp-relative).
mod bow_lit {
    /// 1.1: how far past the wall the rider is pushed out (times depth).
    pub const PUSH_SCALE: u32 = 0x7f30;
    /// 55.6 / 1388.9: the clamp of the grip rebound.
    pub const REBOUND_MIN: u32 = 0x7f2c;
    pub const REBOUND_MAX: u32 = 0x7f28;
    /// 833.3: below this speed a hit adds to the impact accumulator.
    pub const SLOW_SPEED: u32 = 0x7f24;
    /// 0.2: the floor of the normal's z in the accumulator.
    pub const MIN_NORMAL_Z: u32 = 0x7f20;
    /// 0.3: the facing threshold of the launch.
    pub const LAUNCH_FACING: u32 = 0x7f1c;
    /// 277.8: the least outward speed of a launch.
    pub const LAUNCH_MIN_OUT: u32 = 0x7f18;
    /// 555.6 / 27.8: the launch's along-wall speed rule.
    pub const LAUNCH_SPEED_ADD: u32 = 0x7f14;
    pub const LAUNCH_SPEED_MIN: u32 = 0x7f10;
    /// 0.2: blend past which the bail animation restarts.
    pub const BAIL_BLEND: u32 = 0x7f0c;
}

/// Direct calls (same shard) of `Rider_BounceOffWall`.
mod bow_callee {
    use super::{ee, Callee};
    pub const GET_SUB_STATE: Callee = Callee { addr: 0x0011_fee8, f: ee::s00::Rider_GetSubState };
    pub const GET_MAIN_STATE: Callee = Callee { addr: 0x0011_fe98, f: ee::s00::Rider_GetMainState };
    pub const SET_SUB_STATE: Callee = Callee { addr: 0x0011_fec8, f: ee::s00::Rider_SetSubState };
    pub const PLAY_BUMP_ANIM: Callee = Callee { addr: 0x0010_8388, f: ee::s00::Rider_PlayBumpAnim };
    pub const TRANSLATE: Callee = Callee { addr: 0x0010_6538, f: ee::s00::Rider_Translate };
    pub const DEFLECT_ALONG_WALL: Callee =
        Callee { addr: 0x0010_65b0, f: ee::s00::Rider_DeflectAlongWall };
    pub const BUILD_WORLD_MATRIX: Callee =
        Callee { addr: 0x0011_e098, f: ee::s00::Rider_BuildWorldMatrix };
    pub const BANK_TRICK_BOOST: Callee =
        Callee { addr: 0x0010_e910, f: ee::s00::Rider_BankTrickBoost };
    pub const INTERRUPT_TRICK: Callee =
        Callee { addr: 0x0011_9e38, f: ee::s00::RiderMetrix_InterruptTrick };
    pub const AIR_PREDICTOR_PRIME: Callee =
        Callee { addr: 0x0011_35b8, f: ee::s00::AirPredictor_Prime };
    pub const HANDLE_IMPACT: Callee = Callee { addr: 0x0010_5d98, f: ee::s00::Rider_HandleImpact };
}

/// Cross-shard calls (through the function table).
mod bow_table {
    /// `RiderAnimBase_getCurrentAnim(anim, layer)`.
    pub const ANIM_CURRENT: u32 = 0x0031_1ae8;
    /// `RiderAnimBase_getAnimState(anim, layer)` / `..._getBlend` (f0).
    pub const ANIM_STATE: u32 = 0x0031_2aa0;
    pub const ANIM_BLEND: u32 = 0x0031_2ab0;
    /// `cRiderAnimBase_play(anim, id, layer, speed)`.
    pub const ANIM_PLAY: u32 = 0x0031_28e8;
}

/// The bail animation the launch plays (and checks for).
const BOW_BAIL_ANIM: u64 = 0x10c;

#[inline(always)]
fn bow_f(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

#[inline(always)]
fn bow_set_f(c: &mut Cpu, n: usize, v: FpuF32) {
    abi::set_fpr(c, n, v)
}

/// A gp-relative `.lit4` constant.
#[inline(always)]
fn bow_lit(c: &mut Cpu, neg_off: u32) -> FpuF32 {
    let gp = r32(c, GP);
    Addr(gp.wrapping_sub(neg_off)).read_fpu(c)
}

/// The original's dot macro: `vf4.x = vf[a] . vf[b]` over all four lanes
/// (`vf6.x = 1`, `ACC = x + y`, `+ z`, `+ w`).
fn bow_dot4(c: &mut Cpu, a: u8, b: u8) {
    let v = c.vu0();
    v.add(0b1000, 6, 0, Src::Bc(0, W));
    v.mul(0b1111, 4, a, Src::Vec(b));
    v.adda(0b1000, 4, Src::Bc(4, Y));
    v.madda(0b1000, 6, Src::Bc(4, Z));
    v.madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// The length of `vf3` into `v{reg}` (`sqrt` of the dot, read back as Q).
fn bow_length3(c: &mut Cpu, reg: usize) {
    bow_dot4(c, 3, 3);
    let v = c.vu0();
    v.sqrt(4, 0);
    v.waitq();
    c.cfc2(reg, 22);
}

/// `vf5 = vf4 * s` with `s` moved through `v{reg}` into `vf3`
/// (`mfc1; qmtc2; vmulx`).
fn bow_scale_vf4(c: &mut Cpu, reg: usize, s_freg: usize) {
    c.mfc1(reg, s_freg);
    c.qmtc2(reg, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
}

/// The closing speed `n . (hit.vel - rider.vel)` into `v1` (the copy of
/// the difference goes through `scratch(sp)` into `slot(sp)`).
fn bow_closing_speed(c: &mut Cpu, scratch: u32, slot: u32) {
    let (rider, hit) = (r32(c, S2), r32(c, S1));
    c.lqc2(3, hit.wrapping_add(bow_hit::VEL));
    c.lqc2(4, rider.wrapping_add(bow_rider::VEL));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(scratch));
    c.lq(V0, sp(c).wrapping_add(scratch));
    c.sq(V0, sp(c).wrapping_add(slot));
    c.lqc2(3, hit.wrapping_add(bow_hit::NORMAL));
    c.lqc2(5, sp(c).wrapping_add(slot));
    bow_dot4(c, 3, 5);
    c.qmfc2(V1, 4);
}

/// Restore the frame (s0 first, as each path does after its last call) and
/// return.
fn bow_epilogue(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, 0x80);
    c.cycles += 10;
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore_ra(c, 0x40);
    frame.restore_fpr(c, 23, 0xa8);
    frame.restore_fpr(c, 22, 0xa0);
    frame.restore_fpr(c, 21, 0x98);
    frame.restore_fpr(c, 20, 0x90);
    frame.close(c);
}

/// A call through the `{i16 delta; fn}` slot `off` of the vtable at `vt`
/// on the object `this` (`a0 = this + delta`).
fn bow_virtual(c: &mut Cpu, this: u32, vt: u32, off: u32, ret: u32) -> crate::call::Flow {
    let delta = Addr(vt.wrapping_add(off)).lh(c);
    c.g[A0][0] = delta;
    let f = Addr(vt.wrapping_add(off + 4)).lw(c);
    c.g[V0][0] = f;
    c.g[A0][0] = sx(this.wrapping_add(delta as u32));
    call_ptr(c, f as u32, ret)
}

/// The grip of the hit surface, clamped to [0, 1] (f22); leaves the
/// surface entry's address in a0 and f0/f1/f12 as the original does.
fn bow_surface_grip(c: &mut Cpu) -> u32 {
    c.cycles += 13;
    let (obj, hit) = (r32(c, S0), r32(c, S1));
    c.g[V0][0] = Addr(hit.wrapping_add(bow_hit::SURFACE)).lw(c);
    c.g[A0][0] = Addr(obj.wrapping_add(bow_obj::SURFACES)).lw(c);
    // v1 = 12 (the entry size).
    c.mult(0, V0, V0, V1);
    c.f[12] = 0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[1] = 0x3f80_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
    let entry = (c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32);
    c.g[A0][0] = sx(entry);
    let raw = Addr(entry.wrapping_add(4)).read_fpu(c);
    bow_set_f(c, 0, raw);
    let positive = FpuF32::ZERO.le(raw);
    abi::set_fcc(c, positive);
    let mut grip = FpuF32::ZERO;
    if positive {
        c.cycles += 1;
        grip = raw.min(FpuF32::ONE);
    }
    bow_set_f(c, 22, grip);
    entry
}

/// Pushed out of a solid wall: translate by `n * depth * 1.1`, the rebound
/// `vn + clamp(grip * vn)` into f20, deflect along the wall on the ground,
/// rebuild the world matrix.
fn bow_push_out(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 17;
    let scale = bow_lit(c, bow_lit::PUSH_SCALE);
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[SP][0];
    let push = bow_f(c, 23) * scale;
    bow_set_f(c, 0, push);
    let hit = r32(c, S1);
    c.lqc2(4, hit.wrapping_add(bow_hit::NORMAL));
    bow_scale_vf4(c, V1, 0);
    c.sqc2(5, sp(c).wrapping_add(0x10));
    c.lq(V0, sp(c).wrapping_add(0x10));
    c.sq(V0, sp(c));
    bow_callee::TRANSLATE.call(c, 0x0010_594c)?;
    c.f[0] = 0;
    let vn = bow_f(c, 21);
    if abi::c_lt_s(c, vn, FpuF32::ZERO) {
        bow_set_f(c, 21, FpuF32::ZERO);
    }

    c.cycles += 7;
    let vn = bow_f(c, 21);
    let rebound = bow_f(c, 22) * vn;
    bow_set_f(c, 1, rebound);
    let lo = bow_lit(c, bow_lit::REBOUND_MIN);
    bow_set_f(c, 0, lo);
    let hi = bow_lit(c, bow_lit::REBOUND_MAX);
    bow_set_f(c, 2, hi);
    let above = lo.le(rebound);
    abi::set_fcc(c, above);
    let mut clamped = lo;
    if above {
        c.cycles += 1;
        clamped = rebound.min(hi);
    }
    bow_set_f(c, 3, clamped);

    c.cycles += 5;
    bow_set_f(c, 20, vn + clamped);
    c.g[A0][0] = c.g[S2][0];
    bow_callee::GET_MAIN_STATE.call(c, 0x0010_598c)?;
    let on_ground = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S2][0];
    if on_ground {
        c.cycles += 2;
        c.g[A1][0] = c.g[S1][0];
        bow_callee::DEFLECT_ALONG_WALL.call(c, 0x0010_599c)?;
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    bow_callee::BUILD_WORLD_MATRIX.call(c, 0x0010_59a4)?;
    Ok(())
}

/// A soft wall: the rebound `sqrt(max(b^2 - c, 0)) - b` with
/// `k = 1 / (100 / bounce + 1)`, `b = vn * k`, `c = (1 - grip) * k` (f20).
fn bow_soft_rebound(c: &mut Cpu) {
    c.cycles += 17;
    c.g[AT][0] = 0x42c8_0000;
    let hundred = FpuF32(0x42c8_0000);
    c.g[AT][0] = 0x3f80_0000;
    let one = FpuF32::ONE;
    bow_set_f(c, 1, one);
    let bounce = bow_f(c, 20);
    let grip = bow_f(c, 22);
    let vn = bow_f(c, 21);
    let q = hundred.div(bounce);
    let slip = one - grip;
    c.f[4] = 0;
    let q = q + one;
    let k = one.div(q);
    bow_set_f(c, 1, k);
    let b = vn * k;
    bow_set_f(c, 2, b);
    let slip = slip * k;
    bow_set_f(c, 3, slip);
    let d = (b * b - slip).max(FpuF32::ZERO).sqrt();
    bow_set_f(c, 0, d);
    bow_set_f(c, 20, d - b);
}

/// Slow hits (main state 1, or 2 with the handler in mode 1) add
/// `(2.5 - facing) * max(n.z, 0.2)` to the impact accumulator when the
/// rider is slower than 833. Leaves a0 = the animation.
fn bow_accumulate_impact(c: &mut Cpu) {
    c.cycles += 15;
    let limit = bow_lit(c, bow_lit::SLOW_SPEED);
    bow_set_f(c, 0, limit);
    let rider = r32(c, S2);
    c.lqc2(3, rider.wrapping_add(bow_rider::VEL));
    bow_length3(c, V0);
    c.mtc1(V0, 1);
    let speed = bow_f(c, 1);
    let slow = abi::c_lt_s(c, speed, limit);
    let accum = rider.wrapping_add(bow_rider::IMPACT_ACCUM);
    c.g[V0][0] = sx(accum);
    if slow {
        c.cycles += 8;
        let hit = r32(c, S1);
        let nz = Addr(hit.wrapping_add(bow_hit::NORMAL_Z)).read_fpu(c);
        bow_set_f(c, 2, nz);
        let floor = bow_lit(c, bow_lit::MIN_NORMAL_Z);
        bow_set_f(c, 3, floor);
        c.g[AT][0] = 0x4020_0000;
        let steep = floor.le(nz);
        abi::set_fcc(c, steep);
        let old = Addr(accum).read_fpu(c);
        bow_set_f(c, 1, old);
        let weight = FpuF32(0x4020_0000) - bow_f(c, 22);
        let add = if steep {
            c.cycles += 2;
            weight * nz
        } else {
            c.cycles += 1;
            weight * floor
        };
        c.cycles += 2;
        let sum = old + add;
        bow_set_f(c, 0, sum);
        Addr(accum).write_fpu(c, sum);
    }
    c.cycles += 1;
    c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::ANIM)).lw(c);
}

/// The launch off the wall: keep the along-wall velocity (rescaled to
/// `(speed + 555.6) / 2 / max(speed, 27.8)`), push out at
/// `max(rebound - vn, 277.8)`, bump the pad, restart the bail animation
/// (unless it is already playing early), and prime the air predictor.
fn bow_launch(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 35;
    let (rider, hit) = (r32(c, S2), r32(c, S1));
    c.lqc2(3, rider.wrapping_add(bow_rider::VEL));
    c.lqc2(4, hit.wrapping_add(bow_hit::VEL));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x20));
    let out = bow_f(c, 20) - bow_f(c, 21);
    bow_set_f(c, 1, out);
    c.lq(V0, sp(c).wrapping_add(0x20));
    let min_out = bow_lit(c, bow_lit::LAUNCH_MIN_OUT);
    bow_set_f(c, 0, min_out);
    c.sq(V0, sp(c).wrapping_add(0x10));
    c.lqc2(4, hit.wrapping_add(bow_hit::NORMAL));
    bow_scale_vf4(c, V1, 21);
    c.sqc2(5, sp(c).wrapping_add(0x30));
    let faster = abi::c_lt_s(c, min_out, out);
    c.lq(V0, sp(c).wrapping_add(0x30));
    bow_set_f(c, 3, min_out);
    c.sq(V0, sp(c).wrapping_add(0x20));
    c.lqc2(3, sp(c).wrapping_add(0x10));
    c.lqc2(4, sp(c).wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x30));
    c.lq(V0, sp(c).wrapping_add(0x30));
    c.sq(V0, sp(c));
    c.lqc2(3, sp(c));
    bow_length3(c, V1);
    c.mtc1(V1, 20);
    if faster {
        bow_set_f(c, 3, out);
    }

    c.cycles += 9;
    let add = bow_lit(c, bow_lit::LAUNCH_SPEED_ADD);
    let min_speed = bow_lit(c, bow_lit::LAUNCH_SPEED_MIN);
    bow_set_f(c, 2, min_speed);
    let speed = bow_f(c, 20);
    c.g[AT][0] = 0x3f00_0000;
    bow_set_f(c, 1, FpuF32(0x3f00_0000));
    let slow = speed.le(min_speed);
    abi::set_fcc(c, slow);
    let half = (speed + add) * FpuF32(0x3f00_0000);
    let scale = if slow {
        c.cycles += 3;
        half.div(min_speed)
    } else {
        c.cycles += 1;
        half.div(speed)
    };
    bow_set_f(c, 0, scale);

    c.cycles += 31;
    c.lqc2(4, sp(c));
    bow_scale_vf4(c, V0, 0);
    c.sqc2(5, sp(c));
    c.f[12] = c.f[21];
    c.lqc2(4, hit.wrapping_add(bow_hit::NORMAL));
    bow_scale_vf4(c, V1, 3);
    c.sqc2(5, sp(c).wrapping_add(0x30));
    c.lq(V0, sp(c).wrapping_add(0x30));
    c.sq(V0, sp(c).wrapping_add(0x20));
    c.lqc2(3, sp(c));
    c.lqc2(4, sp(c).wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x30));
    c.lq(V0, sp(c).wrapping_add(0x30));
    let vt = Addr(rider.wrapping_add(bow_rider::IFACE)).lw(c);
    c.g[V1][0] = vt;
    c.sq(V0, rider.wrapping_add(bow_rider::VEL));
    c.sq(V0, sp(c).wrapping_add(0x10));
    bow_virtual(c, rider, vt as u32, bow_iface::VIBRATION_BUMP, 0x0010_5c40)?;

    let rider = r32(c, S2);
    c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::ANIM)).lw(c);
    c.g[A1][0] = 2;
    call_ptr(c, bow_table::ANIM_STATE, 0x0010_5c4c)?;
    c.g[V1][0] = BOW_BAIL_ANIM;
    let restart = if c.g[V0][0] != BOW_BAIL_ANIM {
        true
    } else {
        c.cycles += 8;
        let rider = r32(c, S2);
        c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::ANIM)).lw(c);
        c.g[A1][0] = 2;
        call_ptr(c, bow_table::ANIM_BLEND, 0x0010_5c64)?;
        let limit = bow_lit(c, bow_lit::BAIL_BLEND);
        bow_set_f(c, 1, limit);
        let blend = bow_f(c, 0);
        let late = abi::c_lt_s(c, limit, blend);
        if late {
            c.cycles += 1;
        }
        late
    };
    let rider = r32(c, S2);
    if restart {
        Addr(rider.wrapping_add(bow_rider::SURFACE_TYPE)).sw(c, 0);
        bow_restart_bail(c)?;
    }
    let rider = r32(c, S2);
    c.g[V1][0] = Addr(rider.wrapping_add(bow_rider::IFACE)).lw(c);

    c.cycles += 18;
    let iface = rider.wrapping_add(bow_rider::IFACE);
    c.g[S0][0] = sx(iface);
    let vt = c.g[V1][0] as u32;
    bow_virtual(c, iface, vt, bow_iface::GET_POSITION, 0x0010_5cfc)?;
    let rider = r32(c, S2);
    let vt = Addr(rider.wrapping_add(bow_rider::IFACE)).lw(c);
    c.g[V1][0] = vt;
    c.g[S1][0] = c.g[V0][0];
    let iface = r32(c, S0);
    bow_virtual(c, iface, vt as u32, bow_iface::GET_VELOCITY, 0x0010_5d14)?;
    let rider = r32(c, S2);
    c.f[12] = Addr(rider.wrapping_add(bow_rider::AIR_PRIME_ARG)).read_fpu(c).0;
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::AIR_PREDICTOR)).lw(c);
    c.g[A2][0] = c.g[V0][0];
    bow_callee::AIR_PREDICTOR_PRIME.call(c, 0x0010_5d28)?;
    Ok(())
}

/// The launch's trick interruption: bank the trick boost, interrupt the
/// trick, sub states 0xd then 5, play the bail animation at speed -1.
fn bow_restart_bail(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 27;
    c.f[12] = c.f[20];
    c.g[T0][0] = c.g[S1][0];
    c.g[A3][0] = 0;
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = 0;
    bow_callee::BANK_TRICK_BOOST.call(c, 0x0010_5c98)?;
    let rider = r32(c, S2);
    c.g[V0][0] = Addr(rider.wrapping_add(bow_rider::NATURAL_STANCE)).lw(c);
    c.g[A2][0] = 0;
    c.g[A1][0] = Addr(rider.wrapping_add(bow_rider::STANCE)).lw(c);
    c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::METRIX)).lw(c);
    let switched = c.g[A1][0] ^ c.g[V0][0];
    c.g[A1][0] = (switched != 0) as u64;
    bow_callee::INTERRUPT_TRICK.call(c, 0x0010_5cb4)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 0xd;
    bow_callee::SET_SUB_STATE.call(c, 0x0010_5cc0)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 5;
    bow_callee::SET_SUB_STATE.call(c, 0x0010_5ccc)?;
    let rider = r32(c, S2);
    c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::ANIM)).lw(c);
    c.g[A1][0] = BOW_BAIL_ANIM;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.f[12] = 0xbf80_0000;
    c.g[A2][0] = 1;
    call_ptr(c, bow_table::ANIM_PLAY, 0x0010_5ce4)
}

/// `void Rider_BounceOffWall(Rider* rider, CollObj* obj, void* info,
/// WallHit* hit, float depth)` (0x1057b8): the rider's response to a wall
/// hit. Nothing in sub state 9; a surface without bounce only plays the
/// bump animation. Otherwise the closing speed `vn` gives either a push out
/// of a solid wall (rebound `vn + clamp(grip * vn)`) or a soft rebound;
/// slow hits feed the impact accumulator; in main state 1 with a cruising
/// animation and the board facing the wall, the rider launches off it into
/// the air; else the rebound is added along the normal and
/// `Rider_HandleImpact` runs.
pub fn rider_bounce_off_wall(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save_fpr(c, 23, 0xa8);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 22, 0xa0);
        frame.save_fpr(c, 21, 0x98);
        frame.save_fpr(c, 20, 0x90);
        c.g[S2][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A2][0];
        c.f[23] = c.f[12];
        bow_callee::GET_SUB_STATE.call(c, 0x0010_57f8)?;
        let sub_state = c.g[V0][0];
        c.g[V1][0] = 0xc;
        if sub_state == 9 {
            c.cycles += 1;
            bow_epilogue(c, frame);
            return Ok(());
        }

        let entry = bow_surface_grip(c);
        c.cycles += 5;
        let bounce = Addr(entry).read_fpu(c);
        bow_set_f(c, 20, bounce);
        let no_bounce = bounce.eq_s(FpuF32::ZERO);
        abi::set_fcc(c, no_bounce);
        c.g[A0][0] = c.g[S2][0];
        if no_bounce {
            c.cycles += 5;
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = 1;
            bow_callee::PLAY_BUMP_ANIM.call(c, 0x0010_585c)?;
            bow_epilogue(c, frame);
            return Ok(());
        }

        // The closing speed vn (f21), 0 when moving away from the wall.
        c.cycles += 19;
        bow_closing_speed(c, 0x10, 0x00);
        c.mtc1(V1, 0);
        let toward = FpuF32::ZERO.le(bow_f(c, 0));
        abi::set_fcc(c, toward);
        c.f[21] = 0;
        if toward {
            c.cycles += 17;
            bow_closing_speed(c, 0x20, 0x10);
            c.mtc1(V1, 21);
        } else {
            c.cycles += 1;
        }
        let obj = r32(c, S0);
        c.g[A1][0] = Addr(obj.wrapping_add(bow_obj::HANDLE)).lw(c);

        // Solid (no handle, or its slot +0x70 says so) or soft.
        c.cycles += 2;
        let handle = c.g[A1][0] as u32;
        let solid = if c.g[A1][0] == 0 {
            true
        } else {
            c.cycles += 7;
            let vt = Addr(handle.wrapping_add(0xc)).lw(c);
            c.g[V1][0] = vt;
            bow_virtual(c, handle, vt as u32, bow_obj::HANDLE_PUSH_SLOT, 0x0010_5914)?;
            c.g[V0][0] != 0
        };
        if solid {
            bow_push_out(c)?;
        } else {
            bow_soft_rebound(c);
        }
        let rider = r32(c, S2);
        c.lq(V0, rider.wrapping_add(bow_rider::FACING));

        // facing = board direction . n (f22).
        c.cycles += 16;
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = 1;
        c.sq(V0, sp(c));
        c.lqc2(3, sp(c));
        let hit = r32(c, S1);
        c.lqc2(5, hit.wrapping_add(bow_hit::NORMAL));
        bow_dot4(c, 3, 5);
        c.qmfc2(V1, 4);
        c.mtc1(V1, 22);
        bow_callee::GET_MAIN_STATE.call(c, 0x0010_5a28)?;
        let slow_hit_state = if c.g[V0][0] == c.g[S0][0] {
            true
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            bow_callee::GET_MAIN_STATE.call(c, 0x0010_5a38)?;
            c.g[V1][0] = 2;
            if c.g[V0][0] != 2 {
                false
            } else {
                c.cycles += 4;
                let rider = r32(c, S2);
                let handler = Addr(rider.wrapping_add(bow_rider::HANDLER)).lw(c);
                c.g[V0][0] = handler;
                c.g[V1][0] = Addr((handler as u32).wrapping_add(0x30)).lw(c);
                c.g[V1][0] == c.g[S0][0]
            }
        };
        if slow_hit_state {
            bow_accumulate_impact(c);
        } else {
            let rider = r32(c, S2);
            c.g[A0][0] = Addr(rider.wrapping_add(bow_rider::ANIM)).lw(c);
        }

        // The launch test: main state 1, animation 9/2/0xb/1, facing > 0.3.
        c.cycles += 9;
        c.g[A1][0] = 2;
        call_ptr(c, bow_table::ANIM_CURRENT, 0x0010_5ad0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        bow_callee::GET_MAIN_STATE.call(c, 0x0010_5adc)?;
        let state = c.g[V0][0];
        c.g[V1][0] = state;
        c.g[V0][0] = 9;
        let mut launch = false;
        if state == 1 {
            let anim = c.g[S0][0];
            c.cycles += 2;
            c.g[V0][0] = 2;
            let cruising = anim == 9 || {
                c.cycles += 2;
                c.g[V0][0] = 0xb;
                anim == 2 || {
                    c.cycles += 2;
                    anim == 0xb || {
                        c.cycles += 2;
                        anim == state
                    }
                }
            };
            if cruising {
                c.cycles += 4;
                let threshold = bow_lit(c, bow_lit::LAUNCH_FACING);
                bow_set_f(c, 0, threshold);
                let facing = bow_f(c, 22);
                launch = abi::c_lt_s(c, threshold, facing);
            }
        }
        if launch {
            bow_launch(c)?;
            bow_epilogue(c, frame);
            return Ok(());
        }

        // vel += n * rebound; then the impact.
        c.cycles += 15;
        let hit = r32(c, S1);
        c.lqc2(4, hit.wrapping_add(bow_hit::NORMAL));
        bow_scale_vf4(c, V0, 20);
        c.sqc2(5, sp(c).wrapping_add(0x10));
        c.g[A0][0] = c.g[S2][0];
        c.lq(V0, sp(c).wrapping_add(0x10));
        c.g[A1][0] = c.g[S3][0];
        c.sq(V0, sp(c));
        let rider = r32(c, S2);
        c.lqc2(3, rider.wrapping_add(bow_rider::VEL));
        c.lqc2(4, sp(c));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, rider.wrapping_add(bow_rider::VEL));
        c.g[A2][0] = Addr(hit.wrapping_add(bow_hit::MATERIAL)).lw(c);
        bow_callee::HANDLE_IMPACT.call(c, 0x0010_5d6c)?;
        c.cycles += 1;
        bow_epilogue(c, frame);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------- Rider_BounceOffWall

    use crate::difftest::fakes::{self as bow_fakes, covered as bow_covered, machine as bow_machine, FAKE as BOW_FAKE};
    use crate::difftest::{put_vec as bow_put_vec, put_words as bow_put_words, DATA as BOW_DATA, GAME_GP as BOW_GP};
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Control block of the gated fakes: the magic word (without it they
    /// run the slot's previous function, so other tests are unaffected),
    /// the queue cursors and the scripted answers.
    const BOW_CTRL: u32 = BOW_DATA + 0x20_0000;
    const BOW_MAGIC: u32 = 0x1057_b8a1;
    const BOW_SUB_NEXT: u32 = BOW_CTRL + 0x04;
    const BOW_MAIN_NEXT: u32 = BOW_CTRL + 0x08;
    /// `RiderAnimBase` answers: current animation, state, blend (f0).
    const BOW_ANIM_CURRENT: u32 = BOW_CTRL + 0x0c;
    const BOW_ANIM_STATE: u32 = BOW_CTRL + 0x10;
    const BOW_ANIM_BLEND: u32 = BOW_CTRL + 0x14;
    /// Eight answers each of `Rider_GetSubState` / `Rider_GetMainState`.
    const BOW_SUB_QUEUE: u32 = BOW_CTRL + 0x40;
    const BOW_MAIN_QUEUE: u32 = BOW_CTRL + 0x80;

    /// Fixture objects.
    const BOW_RIDER: u32 = BOW_DATA + 0x1000;
    const BOW_IFACE_VT: u32 = BOW_DATA + 0x0800;
    const BOW_HANDLER: u32 = BOW_DATA + 0x0c00;
    const BOW_OBJ: u32 = BOW_DATA + 0x3000;
    const BOW_HIT: u32 = BOW_DATA + 0x3400;
    const BOW_SURFACES: u32 = BOW_DATA + 0x3800;
    const BOW_HANDLE: u32 = BOW_DATA + 0x3c00;
    const BOW_HANDLE_VT: u32 = BOW_DATA + 0x3d00;
    const BOW_METRIX: u32 = BOW_DATA + 0x4000;

    /// A real leaf that returns 0 (the soft-wall answer of the handle).
    const BOW_RETURNS_ZERO: u32 = 0x0014_06c0;
    /// `RiderMetrix_FinishTrick`: always unwinds (it is reached only
    /// inside `Rider_BankTrickBoost`, whose remaining callees have no
    /// short path on test data).
    const BOW_FINISH_TRICK: u32 = 0x0011_a228;

    fn bow_next(c: &mut Cpu, cursor: u32, queue: u32) -> u64 {
        let n = c.bus.read32(cursor);
        c.bus.write32(cursor, n + 1);
        c.bus.read32(queue + 4 * n.min(7)) as u64
    }

    /// A logged fake (`fakes::fake`, which may unwind) with the scripted
    /// answers.
    fn bow_fake(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        bow_fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        match target {
            0x0011_fee8 => {
                let v = bow_next(c, BOW_SUB_NEXT, BOW_SUB_QUEUE);
                c.g[V0][0] = v;
            }
            0x0011_fe98 => {
                let v = bow_next(c, BOW_MAIN_NEXT, BOW_MAIN_QUEUE);
                c.g[V0][0] = v;
            }
            0x0031_1ae8 => c.g[V0][0] = c.bus.read32(BOW_ANIM_CURRENT) as u64,
            0x0031_2aa0 => c.g[V0][0] = c.bus.read32(BOW_ANIM_STATE) as u64,
            0x0031_2ab0 => c.f[0] = c.bus.read32(BOW_ANIM_BLEND),
            BOW_FINISH_TRICK => c.pc = 0xdead_f000,
            _ => {}
        }
    }

    macro_rules! bow_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(BOW_CTRL) == BOW_MAGIC {
                    return bow_fake(c, $addr);
                }
                let p = $prev.load(Ordering::Relaxed);
                if p == 0 || p == $name as usize {
                    return $gen(c);
                }
                // SAFETY: `p` was read from the same slot (a `fn(&mut Cpu)`).
                let f = unsafe { std::mem::transmute::<usize, fn(&mut Cpu)>(p) };
                f(c)
            }
        };
    }

    bow_gated!(bow_fake_sub_state, BOW_PREV_SUB_STATE, 0x0011_fee8, ee::s00::f_0011fee8__generated);
    bow_gated!(bow_fake_main_state, BOW_PREV_MAIN_STATE, 0x0011_fe98, ee::s00::f_0011fe98__generated);
    bow_gated!(bow_fake_set_sub_state, BOW_PREV_SET_SUB_STATE, 0x0011_fec8, ee::s00::f_0011fec8__generated);
    bow_gated!(bow_fake_translate, BOW_PREV_TRANSLATE, 0x0010_6538, ee::s00::f_00106538__generated);
    bow_gated!(bow_fake_world_matrix, BOW_PREV_WORLD_MATRIX, 0x0011_e098, ee::s00::f_0011e098__generated);
    bow_gated!(bow_fake_air_prime, BOW_PREV_AIR_PRIME, 0x0011_35b8, ee::s00::f_001135b8__generated);
    bow_gated!(bow_fake_finish_trick, BOW_PREV_FINISH_TRICK, BOW_FINISH_TRICK, ee::s00::f_0011a228__generated);
    bow_gated!(bow_fake_anim_current, BOW_PREV_ANIM_CURRENT, 0x0031_1ae8, ee::s10::f_00311ae8__generated);
    bow_gated!(bow_fake_anim_state, BOW_PREV_ANIM_STATE, 0x0031_2aa0, ee::s10::f_00312aa0__generated);
    bow_gated!(bow_fake_anim_blend, BOW_PREV_ANIM_BLEND, 0x0031_2ab0, ee::s10::f_00312ab0__generated);
    bow_gated!(bow_fake_anim_play, BOW_PREV_ANIM_PLAY, 0x0031_28e8, ee::s10::f_003128e8__generated);

    /// Put `fake` in `slot` unless it is there, chaining to what was there.
    fn bow_hook(slot: *mut fn(&mut Cpu), fake: fn(&mut Cpu), prev: &AtomicUsize) {
        // SAFETY: the slots are plain `fn` pointers; tests swap them the
        // same way (gated fakes fall back to the previous function).
        unsafe {
            let cur = *slot;
            if cur as usize != fake as usize {
                prev.store(cur as usize, Ordering::Relaxed);
                *slot = fake;
            }
        }
    }

    #[allow(unused_unsafe)]
    fn bow_install_fakes() {
        unsafe {
            use std::ptr::addr_of_mut;
            bow_hook(addr_of_mut!(ee::s00::OVERRIDE_0011FEE8), bow_fake_sub_state, &BOW_PREV_SUB_STATE);
            bow_hook(addr_of_mut!(ee::s00::OVERRIDE_0011FE98), bow_fake_main_state, &BOW_PREV_MAIN_STATE);
            bow_hook(addr_of_mut!(ee::s00::OVERRIDE_0011FEC8), bow_fake_set_sub_state, &BOW_PREV_SET_SUB_STATE);
            bow_hook(addr_of_mut!(ee::s00::OVERRIDE_00106538), bow_fake_translate, &BOW_PREV_TRANSLATE);
            bow_hook(addr_of_mut!(ee::s00::OVERRIDE_0011E098), bow_fake_world_matrix, &BOW_PREV_WORLD_MATRIX);
            bow_hook(addr_of_mut!(ee::s00::OVERRIDE_001135B8), bow_fake_air_prime, &BOW_PREV_AIR_PRIME);
            bow_hook(addr_of_mut!(ee::s00::OVERRIDE_0011A228), bow_fake_finish_trick, &BOW_PREV_FINISH_TRICK);
            bow_hook(addr_of_mut!(ee::s10::OVERRIDE_00311AE8), bow_fake_anim_current, &BOW_PREV_ANIM_CURRENT);
            bow_hook(addr_of_mut!(ee::s10::OVERRIDE_00312AA0), bow_fake_anim_state, &BOW_PREV_ANIM_STATE);
            bow_hook(addr_of_mut!(ee::s10::OVERRIDE_00312AB0), bow_fake_anim_blend, &BOW_PREV_ANIM_BLEND);
            bow_hook(addr_of_mut!(ee::s10::OVERRIDE_003128E8), bow_fake_anim_play, &BOW_PREV_ANIM_PLAY);
        }
    }

    /// A random quadword of floats in [-range, range).
    fn bow_vec(c: &mut Cpu, r: &mut crate::difftest::Rng, at: u32, range: f32) {
        let v = [r.f32(range), r.f32(range), r.f32(range), r.f32(range)];
        bow_put_vec(c, at, v);
    }

    /// Sub state 9 (nothing); a bounceless surface (bump animation, its
    /// real `Rider_PlayBumpAnim` takes the short path); moving toward or away
    /// from the wall; solid walls (no handle, or the handle's slot answers
    /// non-zero: push out, with and without the ground deflection, the
    /// real `Rider_DeflectAlongWall` runs) and soft ones (the slot is a real
    /// leaf returning 0); slow-hit states 1 / 2 with the handler in mode 1
    /// or not, slow or fast riders, steep or flat normals; the launch test
    /// (state, animation 9/2/0xb/1/other, facing) and the launch with and
    /// without the bail restart (`Rider_BankTrickBoost` runs for real until
    /// its `RiderMetrix_FinishTrick`, which unwinds); the plain rebound
    /// with the real `Rider_HandleImpact` (sub state 9: its short path).
    /// The other callees are logged fakes, one of which may unwind.
    #[test]
    fn rider_bounce_off_wall_matches() {
        bow_install_fakes();
        bow_covered(ee::s00::f_001057b8__generated, rider_bounce_off_wall, 300, &|c, seed| {
            bow_install_fakes();
            let mut r = bow_machine(c, seed, 0x0010_57b8, 8);
            c.pc = 0x0010_57b8;

            // .lit4 constants at gp-0x7f30 .. gp-0x7f0c.
            bow_put_words(
                c,
                BOW_GP - 0x7f30,
                &[
                    0x3f8c_cccd, 0x425e_38e4, 0x44ad_9c72, 0x4450_5556, 0x3e4c_cccd,
                    0x3e99_999a, 0x438a_e38e, 0x440a_e38e, 0x41de_38e4, 0x3e4c_cccd,
                ],
            );

            // The scripted answers.
            c.bus.write32(BOW_CTRL, BOW_MAGIC);
            let sub0 = if seed % 10 == 0 { 9 } else { [0, 1, 2, 5][(r.next_u32() % 4) as usize] };
            c.bus.write32(BOW_SUB_QUEUE, sub0);
            for i in 1..8 {
                c.bus.write32(BOW_SUB_QUEUE + 4 * i, 9);
            }
            let no_bounce = seed % 7 == 1;
            for i in 0..8 {
                let v = if no_bounce {
                    [1, 2, 3][(r.next_u32() % 3) as usize]
                } else {
                    [0, 1, 1, 2, 3][(r.next_u32() % 5) as usize]
                };
                c.bus.write32(BOW_MAIN_QUEUE + 4 * i, v);
            }
            c.bus.write32(BOW_ANIM_CURRENT, [9, 2, 0xb, 1, 5][(r.next_u32() % 5) as usize]);
            c.bus.write32(BOW_ANIM_STATE, if r.next_u32() % 2 == 0 { 0x10c } else { 0x10b });
            let blend = r.f32(0.5);
            c.bus.write32(BOW_ANIM_BLEND, blend);

            // Arguments: rider, collision object, info, hit, depth.
            c.g[A0][0] = BOW_RIDER as u64;
            c.g[A1][0] = BOW_OBJ as u64;
            c.g[A2][0] = (BOW_DATA + 0x5000) as u64;
            c.g[A3][0] = BOW_HIT as u64;
            c.f[12] = r.f32(20.0);

            // The hit: normal, wall velocity, material, surface index.
            bow_vec(c, &mut r, BOW_HIT + 0x10, 1.0);
            bow_vec(c, &mut r, BOW_HIT + 0x20, 600.0);
            let mat = r.next_u32();
            c.bus.write32(BOW_HIT + 0x4c, mat);
            let surf = r.next_u32() % 4;
            c.bus.write32(BOW_HIT + 0x5c, surf);
            for i in 0..4 {
                let entry = BOW_SURFACES + 0x10 + 12 * i;
                let bounce = if no_bounce { 0 } else { r.f32(80.0) };
                c.bus.write32(entry, bounce);
                let grip = r.f32(1.5);
                c.bus.write32(entry + 4, grip);
            }

            // The collision object: surface table, optional handle whose
            // slot +0x70 answers solid (fake: non-zero) or soft (0).
            c.bus.write32(BOW_OBJ + 0x88, BOW_SURFACES);
            let handle = if seed % 3 == 0 { 0 } else { BOW_HANDLE };
            c.bus.write32(BOW_OBJ + 0xc, handle);
            c.bus.write32(BOW_HANDLE + 0xc, BOW_HANDLE_VT);
            c.bus.write16(BOW_HANDLE_VT + 0x70, ((r.next_u32() % 8) * 4) as u16);
            let push = if r.next_u32() % 2 == 0 { BOW_FAKE + 0x80 } else { BOW_RETURNS_ZERO };
            c.bus.write32(BOW_HANDLE_VT + 0x74, push);

            // The rider.
            if r.next_u32() % 2 == 0 {
                // Facing the wall's normal.
                let n = c.bus.read128(BOW_HIT + 0x10);
                c.bus.write128(BOW_RIDER + 0x180, n);
            } else {
                bow_vec(c, &mut r, BOW_RIDER + 0x180, 1.0);
            }
            let speed = if r.next_u32() % 2 == 0 { 300.0 } else { 1200.0 };
            bow_vec(c, &mut r, BOW_RIDER + 0x1e0, speed);
            let w = r.f32(5.0);
            c.bus.write32(BOW_RIDER + 0x2e4, w);
            let (stance, natural) = (r.next_u32() % 2, r.next_u32() % 2);
            c.bus.write32(BOW_RIDER + 0x320, stance);
            c.bus.write32(BOW_RIDER + 0x324, natural);
            let accum = r.f32(10.0);
            c.bus.write32(BOW_RIDER + 0x3f4, accum);
            let surface_type = r.next_u32();
            c.bus.write32(BOW_RIDER + 0x438, surface_type);
            c.bus.write32(BOW_RIDER + 0x6c0, BOW_IFACE_VT);
            c.bus.write32(BOW_RIDER + 0x77c, BOW_HANDLER);
            c.bus.write32(BOW_HANDLER + 0x30, r.next_u32() % 2);
            let anim = bow_fakes::ptr(&mut r);
            c.bus.write32(BOW_RIDER + 0x784, anim);
            let predictor = bow_fakes::ptr(&mut r);
            c.bus.write32(BOW_RIDER + 0x788, predictor);
            c.bus.write32(BOW_RIDER + 0x790, BOW_METRIX);
            for off in [0x14u32, 0x70, 0x114, 0x134, 0x198] {
                let v = r.next_u32() % 16;
                c.bus.write32(BOW_METRIX + off, v);
            }
            // RiderIface vtable: vibrationBump, getPosition, getVelocity.
            for (i, off) in [0x88u32, 0x28, 0x10].into_iter().enumerate() {
                c.bus.write16(BOW_IFACE_VT + off, ((r.next_u32() % 8) * 4) as u16);
                c.bus.write32(BOW_IFACE_VT + off + 4, BOW_FAKE + 0x10 * i as u32);
            }
        });
    }
}
