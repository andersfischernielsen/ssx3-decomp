//! Native Rider_Place (0x0011d660), from generated/ee/s00/src/lib.rs.
//!
//! A0 = Rider*, A1 = position vec4*, A2 = facing vec4*, A3 = animation ID,
//! F12 = ground clearance. A3 is passed directly to RiderAnimBase_play; it
//! is not a reset-flags mask. A negative clearance disables the ground ray
//! and preserves the supplied position. Nonnegative clearance probes the
//! terrain and adds that clearance along the world-up axis, even on a miss.
//!
//! Placement always clears linear velocity, angular rates, impulse, contact
//! hints and input smoothing, rebuilds the pose/bounds, refreshes overlaps
//! and lights, resets rider vibration and resets the active camera.
//! It marks the per-rider RaceProgress cache dirty, but does not directly
//! overwrite the rider's path cursors or restart race timing. It does not
//! itself select Ground/Air or directly prime AirPredictor. The marker
//! wrapper selects Ground/substate zero and animation 5, then uses clearance
//! zero. A teleporter can supply its authored exit and use negative
//! clearance for an exact placement; selecting its outgoing physics state
//! and velocity remains the caller's responsibility.
//!
//! This standalone port is intentionally not installed and has no automated
//! test, in accordance with the user's development instructions.

use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

use crate::call::{call_ptr, guest, Frame};

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0011_d660, rider_place, "Rider_Place")];

const WORLD_UP: u32 = 0x004f_f160;
const ZERO_VECTOR: u32 = 0x004f_f120;
const IDENTITY_QUATERNION: u32 = 0x004f_f130;

#[inline]
fn vu_scale(c: &mut Cpu, input: u32, scalar: usize, output: u32) {
    c.lqc2(4, input);
    c.qmtc2(scalar, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, output);
}

#[inline]
fn vu_add(c: &mut Cpu, left: u32, right: u32, output: u32) {
    c.lqc2(3, left);
    c.lqc2(4, right);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, output);
}

#[inline]
fn vu_sub(c: &mut Cpu, left: u32, right: u32, output: u32) {
    c.lqc2(3, left);
    c.lqc2(4, right);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, output);
}

#[inline]
fn vu_cross(c: &mut Cpu, left: u32, right: u32, output: u32) {
    c.lqc2(4, left);
    c.lqc2(5, right);
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, output);
}

/// Dot vf3 with vf{rhs}, retaining the original W lane and VU accumulator.
#[inline]
fn vu_dot(c: &mut Cpu, rhs: u8) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(rhs));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `void Rider_Place(Rider*, Vec4* position, Vec4* facing,
///                   int animation, float ground_clearance)`.
pub fn rider_place(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x230);
        let sp = c.g[SP][0] as u32;
        let gp = c.g[GP][0] as u32;

        c.g[V1][0] = 0x500000;
        frame.save(c, S1, 0x1f0);
        frame.save(c, S2, 0x1e0);
        c.g[S1][0] = c.g[A0][0];
        let rider = c.g[S1][0] as u32;
        frame.save(c, S3, 0x1d0);
        c.g[S2][0] = c.g[A1][0];
        let position = c.g[S2][0] as u32;
        frame.save(c, S4, 0x1c0);
        c.g[S3][0] = c.g[V1][0];
        frame.save_fpr(c, 23, 0x228);
        c.g[V1][0] = sx(WORLD_UP);
        frame.save_fpr(c, 22, 0x220);
        c.mov_s(23, 12);
        frame.save(c, S0, 0x200);
        c.g[S4][0] = c.g[A3][0];
        frame.save_ra(c, 0x1b0);
        frame.save_fpr(c, 21, 0x218);
        frame.save_fpr(c, 20, 0x210);
        c.mtc1(0, 22);

        // Clear the line and wall query hints before moving the rider.
        c.g[V0][0] = c.lw(rider.wrapping_add(0x864));
        c.lq(A0, WORLD_UP);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0);
        c.sw(c.g[V0][0] as u32, 0);
        c.sh((c.g[V0][0] as u32).wrapping_add(4), 0);
        c.sh((c.g[V0][0] as u32).wrapping_add(6), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(8), 0);
        c.g[V0][0] = c.lw(rider.wrapping_add(0x868));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0);
        c.sw(c.g[V0][0] as u32, 0);
        c.sh((c.g[V0][0] as u32).wrapping_add(4), 0);
        c.sh((c.g[V0][0] as u32).wrapping_add(6), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(8), 0);
        c.sq(A0, sp.wrapping_add(0x10));

        // Facing is reduced to XY heading; terrain supplies the tilt later.
        c.lwc1(20, c.g[A2][0] as u32);
        c.lq(V0, position);
        c.c_eq_s(20, 22);
        c.lwc1(21, (c.g[A2][0] as u32).wrapping_add(4));
        let facing_x_zero = c.fcc();
        c.sq(V0, sp);
        if facing_x_zero {
            c.cycles += 4;
            c.c_eq_s(21, 22);
            let facing_y_zero = c.fcc();
            c.mov_s(20, 21);
            if !facing_y_zero {
                c.cycles += 4;
                c.c_le_s(22, 21);
                c.lwc1(20, gp.wrapping_sub(0x79b0));
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(20, gp.wrapping_sub(0x79ac));
                }
            }
        } else {
            c.cycles += 7;
            c.div_s(12, 21, 20);
            call_ptr(c, 0x0031_c228, 0x0011_d734)?; // atan
            c.c_lt_s(20, 22);
            let negative_x = c.fcc();
            c.mov_s(12, 0);
            if negative_x {
                c.cycles += 3;
                c.c_lt_s(22, 21);
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(0, gp.wrapping_sub(0x79a8));
                    c.add_s(12, 12, 0);
                } else {
                    c.cycles += 2;
                    c.lwc1(0, gp.wrapping_sub(0x79a4));
                    c.sub_s(12, 12, 0);
                }
            }
            c.cycles += 1;
            c.mov_s(20, 12);
        }

        c.cycles += 7;
        c.mtc1(0, 21);
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] |= 0xffff;
        c.c_le_s(21, 23);
        let probe_ground = c.fcc();
        c.sw(rider.wrapping_add(0x5b8), c.g[V0][0]);
        if probe_ground {
            c.cycles += 47;
            c.lwc1(0, gp.wrapping_sub(0x79a0));
            c.g[S0][0] = sx(sp.wrapping_add(0xe0));
            c.mfc1(V0, 0);
            vu_scale(c, WORLD_UP, V0, sp.wrapping_add(0x40));
            c.g[AT][0] = 0x43480000;
            c.mtc1(AT, 1);
            c.lq(V0, sp.wrapping_add(0x40));
            c.g[A1][0] = sx(sp.wrapping_add(0x20));
            c.lwc1(12, gp.wrapping_sub(0x799c));
            c.g[A2][0] = sx(sp.wrapping_add(0x30));
            c.sq(V0, sp.wrapping_add(0x30));
            c.g[A0][0] = c.g[S0][0];
            vu_add(c, position, sp.wrapping_add(0x30), sp.wrapping_add(0x40));
            c.g[A3][0] = 2;
            c.lq(V0, sp.wrapping_add(0x40));
            c.sq(V0, sp.wrapping_add(0x20));
            c.mfc1(V1, 1);
            vu_scale(c, WORLD_UP, V1, sp.wrapping_add(0x50));
            c.lq(V0, sp.wrapping_add(0x50));
            c.sq(V0, sp.wrapping_add(0x40));
            vu_add(c, position, sp.wrapping_add(0x40), sp.wrapping_add(0x50));
            c.lq(V0, sp.wrapping_add(0x50));
            c.sq(V0, sp.wrapping_add(0x30));
            call_ptr(c, 0x0032_e100, 0x0011_d814)?; // Build ground ray.
            c.g[V1][0] = c.lw(gp.wrapping_sub(0x848));
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = sx(sp.wrapping_add(0x60));
            c.g[A3][0] = 0;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20));
            call_ptr(c, 0x0033_6850, 0x0011_d830)?;
            c.c_le_s(21, 0);
            let hit_ground = c.fcc();
            c.lq(V0, sp.wrapping_add(0x60));
            if hit_ground {
                c.cycles += 3;
                c.lq(V1, sp.wrapping_add(0x70));
                c.sq(V0, sp);
                c.sq(V1, sp.wrapping_add(0x10));
            }
            c.cycles += 11;
            c.mfc1(V1, 23);
            vu_scale(c, WORLD_UP, V1, sp.wrapping_add(0x50));
            c.lq(V0, sp.wrapping_add(0x50));
            c.sq(V0, sp.wrapping_add(0x40));
            vu_add(c, sp, sp.wrapping_add(0x40), sp);
        }

        // Heading quaternion, followed by an optional ground-normal rotation.
        c.cycles += 48;
        c.lwc1(12, gp.wrapping_sub(0x7998));
        c.g[A0][0] = sx(sp.wrapping_add(0x190));
        c.g[AT][0] = 0x3f000000;
        c.mtc1(AT, 21);
        c.g[A1][0] = sx(sp.wrapping_add(0x194));
        c.sub_s(12, 20, 12);
        c.lq(V0, sp);
        c.g[S2][0] = sx(WORLD_UP);
        c.sq(V0, rider.wrapping_add(0x110));
        c.mul_s(12, 12, 21);
        call_ptr(c, 0x0031_be50, 0x0011_d8a4)?; // sincos
        c.lwc1(1, WORLD_UP.wrapping_add(4));
        c.lwc1(0, sp.wrapping_add(0x190));
        c.lwc1(2, WORLD_UP);
        c.lwc1(4, WORLD_UP.wrapping_add(8));
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.lwc1(3, sp.wrapping_add(0x194));
        c.mul_s(0, 0, 4);
        c.sw(rider.wrapping_add(0x150), 0);
        c.swc1(1, sp.wrapping_add(0x24));
        c.swc1(3, sp.wrapping_add(0x2c));
        c.swc1(2, sp.wrapping_add(0x20));
        c.swc1(0, sp.wrapping_add(0x28));
        c.lwc1(1, gp.wrapping_sub(0x7994));
        c.lq(V0, sp.wrapping_add(0x20));
        c.sq(V0, rider.wrapping_add(0x120));
        vu_cross(c, WORLD_UP, sp.wrapping_add(0x10), sp.wrapping_add(0x30));
        c.lq(V0, sp.wrapping_add(0x30));
        c.sq(V0, sp.wrapping_add(0x20));
        c.lqc2(3, sp.wrapping_add(0x20));
        vu_dot(c, 3);
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V1, 22);
        c.mtc1(V1, 0);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.cycles += 44;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(AT, 12);
            c.mfc1(V0, 0);
            c.qmtc2(V0, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, sp.wrapping_add(0x20));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            c.sqc2(5, sp.wrapping_add(0x20));
            c.min_s(12, 0, 12);
            call_ptr(c, 0x0031_c128, 0x0011_d964)?; // asin
            c.mul_s(12, 0, 21);
            c.g[A0][0] = sx(sp.wrapping_add(0x198));
            c.g[A1][0] = sx(sp.wrapping_add(0x19c));
            call_ptr(c, 0x0031_be50, 0x0011_d974)?;
            c.lwc1(0, sp.wrapping_add(0x198));
            c.lwc1(1, sp.wrapping_add(0x20));
            c.lwc1(2, sp.wrapping_add(0x24));
            c.lwc1(4, sp.wrapping_add(0x28));
            c.mul_s(1, 0, 1);
            c.mul_s(2, 0, 2);
            c.lwc1(3, sp.wrapping_add(0x19c));
            c.mul_s(0, 0, 4);
            c.swc1(3, sp.wrapping_add(0x4c));
            c.swc1(1, sp.wrapping_add(0x40));
            c.swc1(2, sp.wrapping_add(0x44));
            c.swc1(0, sp.wrapping_add(0x48));
            c.lq(V0, sp.wrapping_add(0x40));
            c.sq(V0, sp.wrapping_add(0x30));
            c.lqc2(4, sp.wrapping_add(0x30));
            c.lqc2(5, rider.wrapping_add(0x120));
            c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().mula(0b1110, 4, Src::Bc(5, W));
            c.vu0().madda(0b1110, 5, Src::Bc(4, W));
            c.vu0().suba(0b0001, 7, Src::Bc(7, X));
            c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
            c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
            c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
            c.sqc2(8, sp.wrapping_add(0x50));
            c.lq(V0, sp.wrapping_add(0x50));
            c.sq(V0, rider.wrapping_add(0x120));
            c.sq(V0, sp.wrapping_add(0x40));
        }

        c.cycles += 151;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(AT, 20);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0011_e098, 0x0011_d9f8)?; // Rider_BuildWorldMatrix
        c.lq(V0, sp.wrapping_add(0x10));
        c.g[A0][0] = 0x500000;
        c.lq(V1, rider.wrapping_add(0x1b0));
        c.g[A0][0] = sx(ZERO_VECTOR);
        c.sq(V0, rider.wrapping_add(0x370));
        c.g[A1][0] = u64::MAX;
        c.sq(V1, sp.wrapping_add(0x70));
        c.lq(V0, ZERO_VECTOR);
        c.sq(V1, sp.wrapping_add(0x50));
        c.sq(V0, rider.wrapping_add(0x3d0));
        c.lqc2(3, sp.wrapping_add(0x70));
        c.lqc2(5, rider.wrapping_add(0x370));
        vu_dot(c, 5);
        c.qmfc2(V1, 4);
        c.mtc1(V1, 0);
        c.mfc1(A2, 0);
        vu_scale(c, rider.wrapping_add(0x370), A2, sp.wrapping_add(0x80));
        c.lq(V0, sp.wrapping_add(0x80));
        c.sq(V0, sp.wrapping_add(0x60));
        vu_sub(
            c,
            sp.wrapping_add(0x50),
            sp.wrapping_add(0x60),
            sp.wrapping_add(0x80),
        );
        c.lq(V0, sp.wrapping_add(0x80));
        c.sq(V0, sp.wrapping_add(0x40));
        c.lqc2(3, sp.wrapping_add(0x40));
        vu_dot(c, 3);
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, sp.wrapping_add(0x80));
        c.lq(V0, sp.wrapping_add(0x80));
        c.sq(V0, rider.wrapping_add(0x3a0));
        c.sq(V0, sp.wrapping_add(0x30));
        vu_cross(
            c,
            rider.wrapping_add(0x370),
            rider.wrapping_add(0x3a0),
            sp.wrapping_add(0x40),
        );
        c.lq(V0, sp.wrapping_add(0x40));
        c.sq(V0, sp.wrapping_add(0x30));
        c.sq(V0, rider.wrapping_add(0x3b0));
        c.lq(V1, rider.wrapping_add(0x110));
        c.lq(V0, ZERO_VECTOR);
        c.sq(V1, rider.wrapping_add(0x460));
        c.sq(V0, rider.wrapping_add(0x1e0)); // Velocity, always reset.
        c.sw(rider.wrapping_add(0x438), 0);
        c.sw(rider.wrapping_add(0x454), 0);
        c.lq(V0, ZERO_VECTOR);
        c.sw(rider.wrapping_add(0x3f0), 0);
        c.sq(V0, rider.wrapping_add(0x3e0)); // Impulse.
        c.sw(rider.wrapping_add(0x3f4), 0);

        // Nineteen smoothing channels: {current, rate, target}. Channel 9
        // is the sole exception: current/target 1, rate 0. Store
        // ordering matches the original, without changing guest registers.
        for channel in (0x1f0..=0x250).step_by(12) {
            c.sw(rider.wrapping_add(channel + 8), 0);
            c.sw(rider.wrapping_add(channel), 0);
            c.sw(rider.wrapping_add(channel + 4), 0);
        }
        c.swc1(20, rider.wrapping_add(0x264));
        c.swc1(20, rider.wrapping_add(0x25c));
        c.sw(rider.wrapping_add(0x260), 0);
        for channel in (0x268..=0x2c8).step_by(12) {
            c.sw(rider.wrapping_add(channel + 8), 0);
            c.sw(rider.wrapping_add(channel), 0);
            c.sw(rider.wrapping_add(channel + 4), 0);
        }
        c.sw(rider.wrapping_add(0x2dc), 0); // Spin rate.
        c.sw(rider.wrapping_add(0x2e0), 0); // Flip rate.
        c.sw(rider.wrapping_add(0x2fc), 0); // Boost strength.
        c.swc1(20, rider.wrapping_add(0x300)); // Time scale = 1.
        c.swc1(20, rider.wrapping_add(0x318));
        c.sw(rider.wrapping_add(0x31c), 0);
        c.sw(rider.wrapping_add(0x328), 0);
        c.sw(rider.wrapping_add(0x32c), 0);
        c.sw(rider.wrapping_add(0x330), 0);
        c.g[V1][0] = c.lw(rider.wrapping_add(0x324));
        c.lq(V0, ZERO_VECTOR);
        c.g[A0][0] = c.lw(rider.wrapping_add(0x784));
        c.sw(rider.wrapping_add(0x350), 0);
        c.sw(rider.wrapping_add(0x358), 0);
        c.sw(rider.wrapping_add(0x35c), 0);
        c.sq(V0, rider.wrapping_add(0x340));
        c.sw(rider.wrapping_add(0x320), c.g[V1][0]);
        c.sw(rider.wrapping_add(0x354), c.g[A1][0]);
        call_ptr(c, 0x0031_1a50, 0x0011_dc28)?; // Reset rider animation.
        c.g[A0][0] = c.lw(rider.wrapping_add(0x784));
        c.g[V1][0] = c.lw(rider.wrapping_add(0x320));
        c.mtc1(0, 12);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
        c.g[V0][0] = c.lw(rider.wrapping_add(0x320));
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.lwc1(12, gp.wrapping_sub(0x7990));
        }
        c.neg_s(12, 12);

        c.cycles += 58;
        c.g[V0][0] = 0x500000;
        c.g[V0][0] = sx(IDENTITY_QUATERNION);
        c.g[S0][0] = c.lw(rider.wrapping_add(0x784));
        c.lq(V1, IDENTITY_QUATERNION);
        c.g[A0][0] = sx(sp.wrapping_add(0x1a0));
        c.mul_s(12, 12, 21);
        c.g[A1][0] = sx(sp.wrapping_add(0x1a4));
        c.sq(V1, (c.g[S0][0] as u32).wrapping_add(0x30));
        call_ptr(c, 0x0031_be50, 0x0011_dc70)?;
        c.lwc1(0, sp.wrapping_add(0x1a0));
        c.g[A2][0] = 0;
        c.lwc1(1, WORLD_UP.wrapping_add(8));
        c.g[A1][0] = c.g[S4][0]; // Animation ID, not a reset mask.
        c.lwc1(2, WORLD_UP);
        c.lwc1(4, WORLD_UP.wrapping_add(4));
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.lwc1(3, sp.wrapping_add(0x1a4));
        c.mul_s(0, 0, 4);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 12);
        c.swc1(3, sp.wrapping_add(0x3c));
        c.swc1(1, sp.wrapping_add(0x38));
        c.swc1(2, sp.wrapping_add(0x30));
        c.swc1(0, sp.wrapping_add(0x34));
        c.lq(V0, sp.wrapping_add(0x30));
        c.sq(V0, (c.g[S0][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = c.lw(rider.wrapping_add(0x784));
        call_ptr(c, 0x0031_28e8, 0x0011_dcc0)?; // RiderAnimBase_play
        c.g[A0][0] = c.lw(rider.wrapping_add(0x784));
        c.g[A1][0] = 0;
        call_ptr(c, 0x0031_2598, 0x0011_dccc)?;
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0011_eb60, 0x0011_dcd8)?; // Rider_TickAnimation
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0011_eb98, 0x0011_dce0)?; // Rider_UpdateSkeleton
        c.g[A0][0] = c.lw(rider.wrapping_add(0x780));
        call_ptr(c, 0x0031_03f0, 0x0011_dce8)?; // RiderModel_BuildBoneMatrices
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0011_e150, 0x0011_dcf4)?; // Rider_UpdateBounds
        c.g[A0][0] = c.lw(rider.wrapping_add(0x860));
        c.g[A1][0] = sx(rider.wrapping_add(0x400));
        call_ptr(c, 0x0033_2db8, 0x0011_dd00)?; // Refresh overlap list.
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0012_2088, 0x0011_dd08)?; // Rider_FindNearbyLights
        c.g[A0][0] = c.lw(rider.wrapping_add(0x86c));
        call_ptr(c, 0x0021_05b0, 0x0011_dd10)?; // RaceProgress_ResetRider

        c.g[V0][0] = c.lw(rider.wrapping_add(0x6c0));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        let reset_vibration = c.g[V1][0] as u32;
        c.g[A0][0] = sx(rider.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, reset_vibration, 0x0011_dd24)?;

        // A local player's view has a camera reset virtual method.
        c.g[A0][0] = c.lw(rider.wrapping_add(0x870));
        c.g[V0][0] = (c.g[A0][0] < 2) as u64;
        let has_view = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
        if has_view {
            c.cycles += 4;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            let has_views = c.g[V1][0] != 0;
            c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
            if has_views {
                c.cycles += 8;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                let reset_camera = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, reset_camera, 0x0011_dd64)?;
            }
        }

        c.cycles += 12;
        frame.restore(c, S0, 0x200);
        frame.restore(c, S1, 0x1f0);
        frame.restore(c, S2, 0x1e0);
        frame.restore(c, S3, 0x1d0);
        frame.restore(c, S4, 0x1c0);
        frame.restore_ra(c, 0x1b0);
        frame.restore_fpr(c, 23, 0x228);
        frame.restore_fpr(c, 22, 0x220);
        frame.restore_fpr(c, 21, 0x218);
        frame.restore_fpr(c, 20, 0x210);
        frame.close(c);
        Ok(())
    });
}
