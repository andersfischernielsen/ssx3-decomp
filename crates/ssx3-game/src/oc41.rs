//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for rider update, AI decisions and rider rendering, each with
//! a differential test against the generated body.
//!
//! - **Sphere tree bounds** (`RiderSphereTree_GetBounds`): the bounding box of
//!   a sphere-tree node — either the node's own sphere (leaf) or the union of
//!   the spheres its child bitmask selects.
//! - **Wrong-way / stuck test** (`ComputerActorRider_ShouldResetPath`): the
//!   computer rider's heading error against its velocity, and whether it has
//!   been wrong-way or stuck long enough that the path must be reset.
//! - **Lateral ground response** (`RiderGroundState_ComputeCarveAccel`): the
//!   carve acceleration from the speed curve and the edging stat.
//!
//! Calls follow [`crate::call`] (the same registers and cycle blocks at the
//! same points, the unwind check after every call); FPU arithmetic uses
//! [`FpuF32`] and writes the same `c.f` registers; VU0 ops are issued on
//! `c.vu0()` in the original's order.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_9dc8, rider_sphere_tree_get_bounds, "RiderSphereTree_GetBounds"),
    (0x0010_d1a0, computer_actor_rider_should_reset_path, "ComputerActorRider_ShouldResetPath"),
    (0x0013_d028, rider_ground_state_compute_carve_accel, "RiderGroundState_ComputeCarveAccel"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    pub const GET_MAIN_STATE: Callee = Callee { addr: 0x0011_fe98, f: ee::s00::Rider_GetMainState };
    pub const GET_SUB_STATE: Callee = Callee { addr: 0x0011_fee8, f: ee::s00::Rider_GetSubState };
    pub const BE_LIBRARY_GET_INSTANCE: Callee =
        Callee { addr: 0x0014_dc80, f: ee::s01::BELibrary_GetInstance };
}

/// `Math_Atan`, through the function table (and its override slot).
const MATH_ATAN: u32 = 0x0031_c228;
/// `BELibrary_GetInterface`, through the function table.
const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
/// `BECharInt_GetAttrib3Factor_2`, through the function table.
const BE_CHAR_ATTRIB3_FACTOR: u32 = 0x0014_8e68;
/// `D_004A1140`: pointer to the four-point speed curve.
const SPEED_CURVE: u32 = 0x004a_1140;
/// The back-end's phase-interface table (`BELibrary_GetInterface`).
const BE_PHASE_INTERFACES: u32 = 0x0053_08a8;

/// gp offsets of the globals this module reads.
pub mod global {
    /// The `BELibrary` singleton.
    pub const BE_LIBRARY: u32 = 0x1f04;
    /// The race-phase index `BELibrary_GetInterface` selects.
    pub const BE_LIBRARY_INDEX: u32 = 0x1efc;
}

// ---------------------------------------------------------------------------
// Register, memory and FPU helpers.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative address (globals and the `.lit4` constant pool).
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// FPU register `n`.
#[inline(always)]
fn fr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `fN = v`.
#[inline(always)]
fn setf(c: &mut Cpu, n: usize, v: FpuF32) -> FpuF32 {
    c.f[n] = v.0;
    v
}

/// `lwc1 fN, a`.
#[inline(always)]
fn ldf(c: &mut Cpu, n: usize, a: u32) -> FpuF32 {
    let v = Addr(a).read_fpu(c);
    setf(c, n, v)
}

/// `swc1 fN, a`.
#[inline(always)]
fn stf(c: &mut Cpu, n: usize, a: u32) {
    let v = fr(c, n);
    Addr(a).write_fpu(c, v);
}

/// `li at, bits; mtc1 at, fN` (a constant moved through `at`, as the original
/// does); returns the register's value.
#[inline(always)]
fn li_f(c: &mut Cpu, n: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = bits as u64;
    c.mtc1(1, n);
    fr(c, n)
}

/// `c.lt.s fA, fB`, leaving FCR31 set.
#[inline(always)]
fn lt(c: &mut Cpu, a: usize, b: usize) -> bool {
    let r = fr(c, a).lt(fr(c, b));
    abi::set_fcc(c, r);
    r
}

/// `c.le.s fA, fB`, leaving FCR31 set.
#[inline(always)]
fn le(c: &mut Cpu, a: usize, b: usize) -> bool {
    let r = fr(c, a).le(fr(c, b));
    abi::set_fcc(c, r);
    r
}

/// `c.eq.s fA, fB`, leaving FCR31 set.
#[inline(always)]
fn eq(c: &mut Cpu, a: usize, b: usize) -> bool {
    let r = fr(c, a).eq_s(fr(c, b));
    abi::set_fcc(c, r);
    r
}

/// The generated loop back-edge event check (`if cycles >= next_event`).
#[inline(always)]
fn event_check(c: &mut Cpu, head: u32) {
    if c.cycles >= c.next_event {
        c.pc = head;
        crate::events::deliver_event_here(c, head);
    }
}

// ---------------------------------------------------------------------------
// Sphere tree (`sSphereBox_9DC8`, 0x20-byte nodes).

mod sphere_tree {
    /// The node's own sphere: centre (vec4) and radius.
    pub const CENTER: u32 = 0x10;
    pub const RADIUS: u32 = 0x20;
    /// Bit `i` selects child `i`; 0 means a leaf.
    pub const MASK: u32 = 0x28;
    pub const COUNT: u32 = 0x2c;
    /// Child `i`'s fields are read from the slot base `tree + i * 0x20`: the
    /// position quadword at `+0x30`, the radius one quadword further at
    /// `+0x40`.
    pub const KID_POS: u32 = 0x30;
    pub const KID_RADIUS: u32 = 0x40;
    pub const KID_STRIDE: u32 = 0x20;
}

/// `void RiderSphereTree_GetBounds(tree, Vec4* mn, Vec4* mx)` (0x329dc8):
/// `*mn`/`*mx` start as ±1e10 (w = 1). A leaf node (mask 0) yields its own
/// sphere: centre ∓/± radius with w = centre.w. Otherwise the box grows over
/// the children its mask selects, one float at a time. The VU0 difference and
/// sum go through `vf3`/`vf4` into `vf5`; every test uses the FPU condition
/// bit.
pub fn rider_sphere_tree_get_bounds(c: &mut Cpu) {
    use sphere_tree::*;
    c.cycles += 19;
    let frame = Frame::open(c, 0x40);
    let tree = r32(c, A0);
    let mn = r32(c, A1);
    let mx = r32(c, A2);
    let sp0 = sp(c);
    // mn = (1e10, 1e10, 1e10, 1.0)
    ldf(c, 0, gp(c, -0x2de8));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    stf(c, 0, p(sp0, 8));
    stf(c, 0, sp0);
    stf(c, 0, p(sp0, 4));
    stf(c, 1, p(sp0, 0xc));
    // mx = (-1e10, -1e10, -1e10, 1.0)
    ldf(c, 0, gp(c, -0x2de4));
    c.lq(3, sp0);
    stf(c, 0, p(sp0, 8));
    stf(c, 0, sp0);
    stf(c, 0, p(sp0, 4));
    c.sq(3, mn);
    c.lq(2, sp0);
    c.sq(2, mx);
    c.g[V1][0] = lw(c, p(tree, MASK));
    if c.g[V1][0] != 0 {
        c.g[V0][0] = lw(c, p(tree, COUNT));
    } else {
        c.cycles += 25;
        // mn = centre - (r, r, r, 0)
        ldf(c, 0, p(tree, RADIUS));
        c.sw(p(sp0, 0x1c), 0);
        stf(c, 0, p(sp0, 0x18));
        stf(c, 0, p(sp0, 0x10));
        stf(c, 0, p(sp0, 0x14));
        c.lqc2(3, p(tree, CENTER));
        c.lqc2(4, p(sp0, 0x10));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(sp0, 0x20));
        c.lq(2, p(sp0, 0x20));
        c.sw(p(sp0, 0x1c), 0);
        c.sq(2, mn);
        c.sq(2, sp0);
        // mx = centre + (r, r, r, 0)
        ldf(c, 0, p(tree, RADIUS));
        stf(c, 0, p(sp0, 0x18));
        stf(c, 0, p(sp0, 0x10));
        stf(c, 0, p(sp0, 0x14));
        c.lqc2(3, p(tree, CENTER));
        c.lqc2(4, p(sp0, 0x10));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(sp0, 0x20));
        c.lq(2, p(sp0, 0x20));
        c.sq(2, mx);
        c.sq(2, sp0); // also mirrored into the frame's low quadword
        // A leaf returns straight to the epilogue: the count test and the
        // child loop belong to the masked-node path only.
        c.cycles += 2;
        frame.close(c);
        return;
    }
    c.cycles += 2;
    c.g[T0][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 4;
        c.mov_s(2, 1); // f2 = 1.0 (the children's w)
        c.g[T1][0] = 1;
        c.g[A3][0] = c.g[A0][0];
        loop {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(tree, MASK));
            c.g[V1][0] = sx((c.g[T1][0] as u32) << ((c.g[T0][0] as u32) & 31));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            if c.g[V0][0] != 0 {
                accumulate_kid(c, r32(c, A3), mn, mx);
                c.cycles += 1;
                c.g[V0][0] = lw(c, p(tree, COUNT));
            } else {
                c.g[V0][0] = lw(c, p(tree, COUNT));
            }
            c.cycles += 4;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(KID_STRIDE));
            if !more {
                break;
            }
            event_check(c, 0x0032_9e90);
        }
    }
    c.cycles += 2;
    frame.close(c);
}

/// Grow `mn`/`mx` by child `kid`'s sphere: `pos - r` against the minimum
/// (17 cycles, each further axis test 6) and `pos + r` against the maximum
/// (13, each further axis test 6). The radius quadword `(r, r, r, 1)` sits at
/// the frame's `sp + 0` throughout.
fn accumulate_kid(c: &mut Cpu, kid: u32, mn: u32, mx: u32) {
    use sphere_tree::*;
    let sp0 = sp(c);
    c.cycles += 17;
    ldf(c, 0, p(kid, KID_RADIUS));
    stf(c, 2, p(sp0, 0xc));
    stf(c, 0, p(sp0, 8));
    stf(c, 0, sp0);
    stf(c, 0, p(sp0, 4));
    c.lqc2(3, p(kid, KID_POS));
    c.lqc2(4, sp0);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp0, 0x20));
    c.lq(2, p(sp0, 0x20));
    ldf(c, 0, mn); // f0 = mn.x
    c.sq(2, p(sp0, 0x10));
    ldf(c, 1, p(sp0, 0x10)); // f1 = t.x
    if lt(c, 1, 0) {
        stf(c, 1, mn);
    }
    c.cycles += 6;
    ldf(c, 1, p(sp0, 0x14));
    ldf(c, 0, p(mn, 4));
    if lt(c, 1, 0) {
        stf(c, 1, p(mn, 4));
    }
    c.cycles += 6;
    ldf(c, 1, p(sp0, 0x18));
    ldf(c, 0, p(mn, 8));
    if lt(c, 1, 0) {
        stf(c, 1, p(mn, 8));
    }
    c.cycles += 13;
    c.lqc2(3, p(kid, KID_POS));
    c.lqc2(4, sp0);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp0, 0x30));
    c.lq(2, p(sp0, 0x30));
    ldf(c, 0, mx); // f0 = mx.x
    c.sq(2, p(sp0, 0x10));
    c.sq(2, p(sp0, 0x20));
    ldf(c, 1, p(sp0, 0x10)); // f1 = t.x
    if lt(c, 0, 1) {
        stf(c, 1, mx);
    }
    c.cycles += 6;
    ldf(c, 1, p(sp0, 0x14));
    ldf(c, 0, p(mx, 4));
    if lt(c, 0, 1) {
        stf(c, 1, p(mx, 4));
    }
    c.cycles += 6;
    ldf(c, 1, p(sp0, 0x18));
    ldf(c, 0, p(mx, 8));
    if lt(c, 0, 1) {
        stf(c, 1, p(mx, 8));
    }
}

// ---------------------------------------------------------------------------
// Rider and computer-rider fields.

mod rider {
    /// Physical velocity (vec4).
    pub const VELOCITY: u32 = 0x1b0;
    /// Speed (the obstacle-phase gate reads it below 1000).
    pub const SPEED: u32 = 0x4c8;
    /// Route heading (fall line).
    pub const HEADING: u32 = 0x4cc;
    /// `RiderBase*`.
    pub const BASE: u32 = 0x77c;
    /// The air predictor object.
    pub const AIR_PREDICTOR: u32 = 0x788;
    /// The rider's player slot (edging stat lookups).
    pub const PLAYER: u32 = 0x86c;
    /// The per-player value word the attrib factor divides.
    pub const ATTRIB_VALUE: u32 = 0xb34;
    /// Ground speed factor the carve response divides by.
    pub const SPEED_FACTOR: u32 = 0x2fc;
}

mod rider_base {
    /// Main state (`Rider_GetMainState`).
    pub const MAIN_STATE: u32 = 0xde0;
    /// Sub state (`Rider_GetSubState`).
    pub const SUB_STATE: u32 = 0xde4;
}

mod air_predictor {
    /// Predicted landing height.
    pub const LANDING: u32 = 0x98;
    /// Height above the ground the predictor expects.
    pub const HEIGHT: u32 = 0xa0;
    /// Mode: 1 or 3 means "airborne and predicting".
    pub const MODE: u32 = 0xac;
}

/// The computer rider's AI bookkeeping.
mod computer {
    /// The rider it drives.
    pub const RIDER: u32 = 0x18;
    /// Movement direction along the course (< 0 = the wrong way).
    pub const DIR: u32 = 0xe70;
    /// Frames spent stuck.
    pub const STUCK_TIME: u32 = 0xe74;
    /// Frames spent facing the wrong way.
    pub const WRONG_TIME: u32 = 0xe78;
}

/// `.lit4` constants of the angle wrap (`gp` offsets).
const TWO_PI: i32 = -0x7d04; // 6.2831855
const INV_TWO_PI: i32 = -0x7d08; // 0.15915494
const PI_SUB: i32 = -0x7d0c; // 3.1415927, for y <= 0 with x < 0
const PI_ADD: i32 = -0x7d10; // 3.1415927, for y > 0 with x < 0
const HALF_PI_POS: i32 = -0x7d14; // 1.5707964
const HALF_PI_NEG: i32 = -0x7d18; // -1.5707964
const WRONG_WAY_LIMIT: i32 = -0x7d00; // 1.0471976 (pi/3)

/// The function's epilogue: `lq s0`, the other saved callee-saved registers,
/// `ra`, then `sp += 0x80` and `jr ra`. `extra` carries the block's own cycles
/// before the shared tail (1 on the normal path, 2 at the predictor return).
fn leave(c: &mut Cpu, frame: Frame, extra: u64) {
    c.cycles += extra;
    frame.restore(c, S0, 0x50);
    frame.restore_fpr(c, 22, 0x70);
    frame.restore_fpr(c, 21, 0x68);
    frame.restore_fpr(c, 20, 0x60);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore_ra(c, 0x20);
    c.cycles += 8;
    frame.close(c);
}

/// `int ComputerActorRider_ShouldResetPath(ComputerActorRider* self)`
/// (0x10d1a0): `d = wrap(heading - atan2(vel.y, vel.x))`. While the course
/// runs the other way (`dir < 0`) a heading error over pi/3 counts a
/// wrong-way frame (`+0xe78`), otherwise the counter clears. Then the rider
/// state machine: main state 2 adds a stuck frame (`+0xe74`); state 4 with
/// wrong-way frames resets (returns 1); state 1 or a speed below 1000 clears
/// the counters, and a predictor height over 30 resets; anything else adds a
/// stuck frame. Reports stuck when the wrong-way frames reach 0x3d or the
/// stuck counter 0xdf, except in sub state 8, where only the wrong-way
/// direction counts.
pub fn computer_actor_rider_should_reset_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x50);
        frame.save_fpr(c, 22, 0x70);
        let actor = r32(c, A0);
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.mtc1(0, 22); // f22 = 0
        let rider = lw(c, p(actor, computer::RIDER)) as u32;
        let sp0 = sp(c);
        c.lq(3, p(rider, rider::VELOCITY));
        c.sq(3, sp0);
        c.lq(3, p(rider, rider::VELOCITY));
        ldf(c, 21, p(sp0, 4)); // f21 = vel.y
        c.sq(3, p(sp0, 0x10));
        ldf(c, 20, p(sp0, 0x10)); // f20 = vel.x
        // The angle in f12: 0 or ±pi/2 when x is 0, else atan(y/x) with the
        // quadrant fixup added in line. Reaching the shared tail through
        // `b_10d258` costs one cycle and reloads v0.
        let mut via_shared = true;
        if !eq(c, 20, 22) {
            c.cycles += 7;
            setf(c, 12, fr(c, 21).div(fr(c, 20))); // y / x
            call_ptr(c, MATH_ATAN, 0x0010_d228)?;
            let x_negative = lt(c, 20, 22);
            setf(c, 12, fr(c, 0)); // mov.s f12, f0
            if x_negative {
                c.cycles += 3;
                if lt(c, 22, 21) {
                    c.cycles += 3;
                    let pi = ldf(c, 0, gp(c, PI_ADD));
                    setf(c, 12, fr(c, 12) + pi);
                } else {
                    c.cycles += 2;
                    let pi = ldf(c, 0, gp(c, PI_SUB));
                    setf(c, 12, fr(c, 12) - pi);
                }
            }
        } else {
            c.cycles += 4;
            let y_zero = eq(c, 21, 22);
            setf(c, 12, fr(c, 21));
            if !y_zero {
                c.cycles += 4;
                let up = le(c, 22, 21);
                ldf(c, 12, gp(c, HALF_PI_NEG));
                c.g[V0][0] = lw(c, p(actor, computer::RIDER));
                via_shared = false;
                if up {
                    c.cycles += 3;
                    ldf(c, 12, gp(c, HALF_PI_POS));
                }
            }
        }
        if via_shared {
            c.cycles += 1;
            c.g[V0][0] = lw(c, p(actor, computer::RIDER));
        }
        // d = x - floor(x / (2 pi) + 1/2) * 2 pi in f0; f1 keeps the unwrapped
        // error, which the tail only uses for the absolute test.
        c.cycles += 12;
        let inv = ldf(c, 0, gp(c, INV_TWO_PI));
        ldf(c, 1, p(rider, rider::HEADING));
        setf(c, 2, FpuF32(0x3f00_0000)); // 0.5
        let err = setf(c, 1, fr(c, 1) - fr(c, 12));
        let turns = setf(c, 0, err * inv + fr(c, 2));
        let whole = setf(c, 2, FpuF32::from_i32(turns.to_i32()));
        if lt(c, 0, 2) {
            c.cycles += 3;
            li_f(c, 0, 0x3f80_0000); // 1.0
            setf(c, 2, whole - fr(c, 0));
        }
        c.cycles += 5;
        let two_pi = ldf(c, 0, gp(c, TWO_PI));
        let dir = lw(c, p(actor, computer::DIR));
        setf(c, 0, fr(c, 1) - fr(c, 2) * two_pi);
        if (dir as i64) >= 0 {
            c.cycles += 1;
            c.sw(p(actor, computer::WRONG_TIME), 0);
        } else {
            c.cycles += 6;
            setf(c, 1, fr(c, 0).abs());
            let limit = ldf(c, 0, gp(c, WRONG_WAY_LIMIT));
            if lt(c, 0, 1) {
                c.cycles += 4;
                c.g[V0][0] = lw(c, p(actor, computer::WRONG_TIME));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.sw(p(actor, computer::WRONG_TIME), c.g[V0][0]);
            } else {
                c.sw(p(actor, computer::WRONG_TIME), 0);
            }
        }
        // wrong = wrongTime >= 0x3d
        c.cycles += 8;
        let wrong_time = lw(c, p(actor, computer::WRONG_TIME));
        c.g[V0][0] = wrong_time;
        c.g[A0][0] = lw(c, p(actor, computer::RIDER));
        c.g[V0][0] = ((wrong_time as i64) < 61) as u64;
        c.g[S2][0] = c.g[V0][0] ^ 1;
        callee::GET_MAIN_STATE.call(c, 0x0010_d2ec)?;
        c.g[V1][0] = 2;
        let mut add_stuck_frame = false;
        if c.g[V0][0] != c.g[V1][0] {
            // State 4 with wrong-way frames resets at once.
            c.cycles += 5;
            c.g[A0][0] = lw(c, p(actor, computer::RIDER));
            callee::GET_MAIN_STATE.call(c, 0x0010_d30c)?;
            c.g[V1][0] = 4;
            if c.g[V0][0] == c.g[V1][0] {
                c.cycles += 3;
                c.g[V0][0] = lw(c, p(actor, computer::WRONG_TIME));
                let wrong_way = (c.g[V0][0] as i64) > 0;
                c.g[V0][0] = 1;
                if wrong_way {
                    leave(c, frame, 1);
                    return Ok(());
                }
                c.cycles += 1;
                c.g[A0][0] = lw(c, p(actor, computer::RIDER));
            }
            // State 1 or a speed below 1000 clears the counters.
            c.cycles += 4;
            c.g[S1][0] = 1;
            callee::GET_MAIN_STATE.call(c, 0x0010_d330)?;
            let mut skip_predictor = false;
            if c.g[V0][0] == c.g[S1][0] {
                c.sw(p(actor, computer::STUCK_TIME), 0);
            } else {
                c.cycles += 8;
                c.g[V0][0] = lw(c, p(actor, computer::RIDER));
                let thousand = li_f(c, 1, 0x447a_0000); // 1000.0
                ldf(c, 0, p(c.g[V0][0] as u32, rider::SPEED));
                if lt(c, 0, 1) {
                    c.cycles += 1;
                    c.sw(p(actor, computer::STUCK_TIME), 0);
                } else {
                    // One more stuck frame instead.
                    c.g[V0][0] = lw(c, p(actor, computer::STUCK_TIME));
                    add_stuck_frame = true;
                    skip_predictor = true;
                }
                let _ = thousand;
            }
            if !skip_predictor {
                // State 1 with a predictor above 30 resets.
                c.cycles += 5;
                c.g[S2][0] = 0;
                c.g[A0][0] = lw(c, p(actor, computer::RIDER));
                let one = c.g[S1][0];
                callee::GET_MAIN_STATE.call(c, 0x0010_d368)?;
                if c.g[V0][0] == one {
                    c.cycles += 9;
                    c.g[V1][0] = lw(c, p(actor, computer::RIDER));
                    li_f(c, 1, 0x41f0_0000); // 30.0
                    c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, rider::AIR_PREDICTOR));
                    ldf(c, 0, p(c.g[V0][0] as u32, air_predictor::HEIGHT));
                    let high = lt(c, 1, 0);
                    c.g[V0][0] = 1;
                    if high {
                        leave(c, frame, 2);
                        return Ok(());
                    }
                }
            }
        } else {
            // State 2 adds a stuck frame and clears the wrong-way flag.
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(actor, computer::STUCK_TIME));
            c.g[S2][0] = 0;
            add_stuck_frame = true;
        }
        if add_stuck_frame {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sw(p(actor, computer::STUCK_TIME), c.g[V0][0]);
        }
        // stuck = wrong || stuckTime >= 0xdf
        c.cycles += 2;
        let wrong = c.g[S2][0] != 0;
        c.g[S1][0] = 0;
        if wrong {
            c.cycles += 1;
            c.g[S1][0] = 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = lw(c, p(actor, computer::STUCK_TIME));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 223) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[S1][0] = 1;
            }
        }
        // Sub state 8 keeps the verdict; any other needs dir < 0.
        c.cycles += 5;
        c.g[A0][0] = lw(c, p(actor, computer::RIDER));
        callee::GET_SUB_STATE.call(c, 0x0010_d3c8)?;
        c.g[V1][0] = 8;
        let reset_sub = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = c.g[S1][0];
        if !reset_sub {
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                c.g[V0][0] = 0;
                c.cycles += 2;
                c.g[S1][0] = c.g[V0][0];
                c.g[V0][0] = c.g[S1][0];
            } else {
                c.cycles += 2;
                c.g[V0][0] = lw(c, p(actor, computer::DIR));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 0) as u64;
                c.cycles += 2;
                c.g[S1][0] = c.g[V0][0];
                c.g[V0][0] = c.g[S1][0];
            }
        }
        leave(c, frame, 1);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Speed curve and the carve response.

/// `sCurvePt_13D028` pairs: four `{x, y}` points, x ascending.
mod speed_curve {
    pub const P0_X: u32 = 0x00;
    pub const P0_Y: u32 = 0x04;
    pub const P1_X: u32 = 0x08;
    pub const P1_Y: u32 = 0x0c;
    pub const P2_X: u32 = 0x10;
    pub const P2_Y: u32 = 0x14;
    pub const P3_X: u32 = 0x18;
    pub const P3_Y: u32 = 0x1c;
}

/// The carve curve's `x = |b| * 0.036` scale, the edging stat's reciprocal
/// scale, and the pointer variable holding the curve.
const CURVE_SPEED_SCALE: i32 = -0x7070; // 0.036
const STAT_SPEED_SCALE: i32 = -0x706c; // 4.0086374
const SPEED_CURVE_PTR: i32 = -0x1fb0; // `D_004A1140`

/// `float RiderGroundState_ComputeCarveAccel(ground, const Vec4* v, float a,
/// float b, float c)` (0x13d028): when the lateral speed `a` and the carve
/// input `c` point the same way, `a * c` (or `a * -c` for the other sign) is
/// scaled by the speed curve at `|b| * 0.036`, by `-v[2]`, and — when the
/// ground's `+0x2fc` factor is positive — by `1 / (1 + 4.0086 * factor)`.
/// The edging stat comes from `BECharInt_GetAttrib3Factor_2`. The curve
/// search is the inlined `hpCurve`: outside `p0..p3` the end point's y is
/// used without the lerp, inside a band the lerp uses that band's endpoints.
pub fn rider_ground_state_compute_carve_accel(c: &mut Cpu) {
    use speed_curve::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.mtc1(0, 0); // f0 = 0
        frame.save_fpr(c, 20, 0x20);
        c.mov_s(20, 12); // f20 = a
        frame.save(c, S0, 0x10);
        let a_negative = lt(c, 20, 0);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        // `a` and `c` must point the same way: `a * c`, `a * -c`, else 0.
        let mut same_way = false;
        let mut second_test = true;
        if a_negative {
            c.cycles += 3;
            if lt(c, 0, 14) {
                c.cycles += 2;
                setf(c, 20, fr(c, 20) * fr(c, 14));
                same_way = true;
                second_test = false;
            }
        }
        if second_test {
            c.cycles += 3;
            if lt(c, 0, 20) {
                c.cycles += 3;
                if lt(c, 14, 0) {
                    c.cycles += 3;
                    let c_neg = -fr(c, 14);
                    setf(c, 0, c_neg);
                    setf(c, 20, fr(c, 20) * c_neg);
                    same_way = true;
                }
            }
        }
        if !same_way {
            // No carve: return +0.0.
            c.cycles += 3;
            c.mtc1(0, 0);
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.restore_fpr(c, 20, 0x20);
            frame.close(c);
            return Ok(());
        }
        // The curve at |b| * 0.036: f3 = x, f1 receives the value.
        c.cycles += 9;
        let scale = ldf(c, 1, gp(c, CURVE_SPEED_SCALE));
        let abs_b = setf(c, 0, fr(c, 13).abs());
        let curve = lw(c, gp(c, SPEED_CURVE_PTR)) as u32;
        setf(c, 3, abs_b * scale);
        ldf(c, 4, p(curve, P1_X));
        let mut lerp = false;
        if !lt(c, 4, 3) {
            // x <= p1.x: the p0..p1 band, or the clamp at p0.
            ldf(c, 1, p(curve, P0_X));
            c.cycles += 4;
            if lt(c, 3, 1) {
                c.cycles += 2;
                ldf(c, 1, p(curve, P0_Y));
            } else {
                ldf(c, 2, p(curve, P0_Y));
                c.cycles += 2;
                setf(c, 3, fr(c, 3) - fr(c, 1));
                ldf(c, 0, p(curve, P1_Y));
                c.cycles += 1;
                setf(c, 1, fr(c, 4) - fr(c, 1));
                lerp = true;
            }
        } else {
            c.cycles += 5;
            ldf(c, 1, p(curve, P2_X));
            if !lt(c, 1, 3) {
                ldf(c, 2, p(curve, P1_Y));
                c.cycles += 4;
                setf(c, 3, fr(c, 3) - fr(c, 4));
                ldf(c, 0, p(curve, P2_Y));
                setf(c, 1, fr(c, 1) - fr(c, 4));
                lerp = true;
            } else {
                c.cycles += 5;
                ldf(c, 4, p(curve, P3_X));
                if !lt(c, 4, 3) {
                    ldf(c, 2, p(curve, P2_Y));
                    c.cycles += 3;
                    setf(c, 3, fr(c, 3) - fr(c, 1));
                    ldf(c, 0, p(curve, P3_Y));
                    c.cycles += 1;
                    setf(c, 1, fr(c, 4) - fr(c, 1));
                    lerp = true;
                } else {
                    c.cycles += 2;
                    ldf(c, 1, p(curve, P3_Y));
                }
            }
        }
        if lerp {
            c.cycles += 4;
            let f0 = setf(c, 0, fr(c, 0) - fr(c, 2));
            let f0 = setf(c, 0, f0 * fr(c, 3));
            let f0 = setf(c, 0, f0.div(fr(c, 1)));
            setf(c, 1, fr(c, 2) + f0);
        }
        // a = a * curve * -v[2]
        c.cycles += 22;
        let v = r32(c, A1);
        ldf(c, 0, p(v, 0x8));
        setf(c, 20, fr(c, 20) * fr(c, 1));
        let v2 = -fr(c, 0);
        setf(c, 0, v2);
        setf(c, 20, fr(c, 20) * v2);
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0013_d148)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x0013_d154)?;
        c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, computer::RIDER));
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = lw(c, p(c.g[V1][0] as u32, rider::ATTRIB_VALUE));
        c.g[A1][0] = lw(c, p(c.g[V1][0] as u32, rider::PLAYER));
        call_ptr(c, BE_CHAR_ATTRIB3_FACTOR, 0x0013_d168)?;
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, computer::RIDER));
        c.mtc1(0, 0);
        ldf(c, 1, p(c.g[V0][0] as u32, rider::SPEED_FACTOR));
        li_f(c, 2, 0x3f80_0000); // 1.0
        let positive = lt(c, 0, 1);
        c.mov_s(0, 20); // f0 = the acceleration so far
        if positive {
            c.cycles += 6;
            let k = ldf(c, 0, gp(c, STAT_SPEED_SCALE));
            let denom = fr(c, 1) * k + fr(c, 2);
            let ratio = fr(c, 2).div(denom);
            setf(c, 20, fr(c, 20) * ratio);
            c.mov_s(0, 20);
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, put_vec, put_words, scramble, Rng, DATA, GAME_GP};

    /// The `gp`-relative `.lit4` constants these functions read (a test
    /// machine's RAM starts empty; the game image holds them).
    fn literals(c: &mut Cpu) {
        put_words(c, GAME_GP - 0x2de8, &[0x5015_02f9, 0xd015_02f9]);
        // The angle-wrap pool around `gp - 0x7d18`.
        put_words(
            c,
            GAME_GP - 0x7d18,
            &[0xbfc9_0fdb, 0x3fc9_0fdb, 0x4049_0fdb, 0x4049_0fdb, 0x3e22_f983, 0x40c9_0fdb, 0x3f86_0a93],
        );
        // 0.036 / 4.0086374
        put_words(c, GAME_GP - 0x7070, &[0x3d13_74bc, 0x4080_46c2]);
    }

    #[test]
    fn rider_sphere_tree_get_bounds_matches() {
        check(ssx3_ee::s10::f_00329dc8__generated, rider_sphere_tree_get_bounds, 300, &|c, seed| {
            let mut r = Rng(0x51f3_9a2b ^ seed as u64);
            scramble(c, &mut r);
            literals(c);
            c.g[GP][0] = GAME_GP as u64;
            let tree = DATA;
            c.g[A0][0] = tree as u64;
            c.g[A1][0] = (tree + 0x100) as u64;
            c.g[A2][0] = (tree + 0x110) as u64;
            // A leaf every fourth seed; otherwise a mask that selects some (or
            // none) of the children, with 0..2 of them counted.
            let leaf = seed % 4 == 0;
            let mask = if leaf { 0 } else { (seed / 4) % 4 + 1 };
            let count = (seed / 16) % 3;
            c.bus.write32(tree + 0x28, mask);
            c.bus.write32(tree + 0x2c, count);
            put_vec(c, tree + 0x10, [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.edge_f32()]);
            c.bus.write32(tree + 0x20, r.f32(50.0));
            for i in 0..2 {
                let kid = tree + 0x30 + i * 0x20;
                put_vec(c, kid, [r.f32(200.0), r.f32(200.0), r.f32(200.0), r.edge_f32()]);
                c.bus.write32(kid + 0x10, r.f32(30.0));
            }
        });
    }

    #[test]
    fn computer_actor_rider_should_reset_path_matches() {
        check(ssx3_ee::s00::f_0010d1a0__generated, computer_actor_rider_should_reset_path, 300, &|c, seed| {
            let mut r = Rng(0x2c1b_3e4d ^ seed as u64);
            scramble(c, &mut r);
            literals(c);
            c.g[GP][0] = GAME_GP as u64;
            let actor = DATA;
            let rider = DATA + 0x200;
            let base = DATA + 0x400;
            let predictor = DATA + 0x300;
            c.g[A0][0] = actor as u64;
            c.bus.write32(actor + computer::RIDER, rider);
            let dir = if seed % 3 == 0 {
                -(1 + (r.next_u32() % 4) as i32)
            } else {
                (r.next_u32() % 5) as i32
            };
            c.bus.write32(actor + computer::DIR, dir as u32);
            let stuck = match seed % 4 {
                0 => 0,
                1 => 222,
                2 => 230,
                _ => r.next_u32() % 400,
            };
            c.bus.write32(actor + computer::STUCK_TIME, stuck);
            let wrong = match seed % 5 {
                0 => 0,
                1 => 60,
                2 => 61,
                3 => r.next_u32() % 100,
                _ => 1000,
            };
            c.bus.write32(actor + computer::WRONG_TIME, wrong);
            // Velocity: zero x, zero y, and both signs.
            let (vx, vy) = match seed % 6 {
                0 => (0, 0),
                1 => (0, r.edge_f32()),
                2 => (0, r.edge_f32() | 0x8000_0000),
                3 => (r.f32(30.0), 0),
                4 => (r.f32(30.0), r.f32(30.0)),
                _ => (r.edge_f32(), r.edge_f32()),
            };
            put_vec(c, rider + rider::VELOCITY, [vx, vy, r.f32(10.0), r.f32(1.0)]);
            let speed = match seed % 4 {
                0 => r.f32(900.0),
                1 => 0x447a_0000, // exactly 1000
                2 => 0x4520_0000, // 2560
                _ => r.f32(2000.0),
            };
            c.bus.write32(rider + rider::SPEED, speed);
            c.bus.write32(rider + rider::HEADING, r.f32(6.3));
            c.bus.write32(rider + rider::BASE, base);
            c.bus.write32(rider + rider::AIR_PREDICTOR, predictor);
            c.bus.write32(base + rider_base::MAIN_STATE, seed % 6);
            c.bus.write32(base + rider_base::SUB_STATE, if seed % 5 == 0 { 8 } else { seed % 9 });
            let height = match seed % 3 {
                0 => r.f32(20.0),
                1 => 0x41f0_0000, // exactly 30
                _ => 0x4240_0000, // 48
            };
            c.bus.write32(predictor + air_predictor::HEIGHT, height);
            c.bus.write32(predictor + air_predictor::LANDING, r.f32(50.0));
            c.bus.write32(predictor + air_predictor::MODE, seed % 4);
        });
    }

    #[test]
    fn rider_ground_state_compute_carve_accel_matches() {
        check(ssx3_ee::s01::f_0013d028__generated, rider_ground_state_compute_carve_accel, 300, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x0013_d028, 0);
            literals(c);
            let ground = DATA;
            let rider = DATA + 0x200;
            let v = DATA + 0x300;
            c.g[A0][0] = ground as u64;
            c.g[A1][0] = v as u64;
            c.bus.write32(ground + computer::RIDER, rider);
            // The edging stat chain (`BECharInt_GetAttrib3Factor_2`): the
            // player's record has a profile, character 0, no fixed flag.
            c.bus.write32(rider + rider::PLAYER, 0);
            c.bus.write32(rider + rider::ATTRIB_VALUE, r.next_u32() % 400);
            c.bus.write32(rider + rider::SPEED_FACTOR, if seed % 3 == 0 { 0 } else { r.edge_f32() });
            c.bus.write8(0x0053_5538 + 3, (r.next_u32() % 250) as u8);
            c.bus.write8(0x0053_08d8 + 8 + 3, 1 + (r.next_u32() % 250) as u8);
            put_vec(c, v, [r.f32(10.0), r.f32(10.0), r.f32(10.0), r.f32(1.0)]);
            // The speed curve: four ascending points.
            let curve = DATA + 0x800;
            c.bus.write32(SPEED_CURVE, curve);
            let mut x = 0i32;
            for i in 0..4u32 {
                x += 1 + (r.next_u32() % 20) as i32;
                c.bus.write32(curve + 8 * i, x as u32);
                c.bus.write32(curve + 8 * i + 4, r.f32(1000.0));
            }
            // The back-end singleton and the interface's vtable (a fake: the
            // interface value is not used by the attrib chain).
            c.bus.write32(GAME_GP - global::BE_LIBRARY, DATA + 0x600);
            c.bus.write32(GAME_GP - global::BE_LIBRARY_INDEX, 0);
            c.bus.write32(BE_PHASE_INTERFACES, DATA + 0x700);
            c.bus.write16(DATA + 0x908, 0); // vtable slot 1's delta (at vt+8)
            c.bus.write32(DATA + 0x90c, fakes::FAKE); // ... and its function
            c.bus.write32(DATA + 0x704, DATA + 0x900); // lib->vtable
            // Arguments: a, b, c in f12-f14.
            c.f[12] = if seed % 4 == 0 { r.edge_f32() | 0x8000_0000 } else { r.edge_f32() };
            c.f[13] = r.edge_f32();
            c.f[14] = match seed % 4 {
                0 => r.edge_f32() | 0x8000_0000,
                1 => 0,
                _ => r.edge_f32(),
            };
        });
    }
}
