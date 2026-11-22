//! Native RollerModifier_Update (0x0035e850), with explicit call-return PCs.
//!
//! Used by the existing game::object::railmodifier entry. Original native
//! update semantics and tick-rate rules stay intact; imported-body ownership
//! and lifetime policy belong to that entry's caller-side guard.

use crate::call::{call_ptr, guest, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(ENTRY, roller_modifier_update, "RollerModifier_Update")];
const ENTRY: u32 = 0x0035_e850;
const END: u32 = 0x0035_ed88;
const FRAME_BYTES: u32 = 0xa0;
const AFTER_BODY: u32 = 0x0035_ed6c;
const EPILOGUE: u32 = 0x0035_ed70;
const TRANSLATE: u32 = 0x0035_ed90;
const GET_POSE: u32 = 0x0035_e770;
const INTEGRATE: u32 = 0x0035_d340;
const UPDATE_INERTIA: u32 = 0x0035_e248;
const UPDATE_MOMENTUM: u32 = 0x0035_d288;
const COLLIDE_WORLD: u32 = 0x0035_edc8;
const SET_TREE_TRANSFORM: u32 = 0x0032_c648;

#[inline]
fn r32(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}
#[inline]
fn sp(c: &Cpu) -> u32 {
    r32(c, SP)
}
#[inline]
fn gp(c: &Cpu, offset: i32) -> u32 {
    r32(c, GP).wrapping_add(offset as u32)
}

/// `void RollerModifier_Update(RollerModifier* self)`.
/// Advance the timer, stop on low activity/timeout or integrate the body and
/// synchronize its collision/render transforms. Re-entry at a callee return
/// uses the still-open native frame. The 16s native disposal path is retained.
pub fn roller_modifier_update(c: &mut Cpu) {
    use roller::*;
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 16;
                    Frame::open(c, FRAME_BYTES);
                    c.lwc1(0, gp(c, -0x29ec));
                    frame.save(c, S2, 0x60);
                    frame.save(c, S0, 0x80);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S1, 0x70);
                    frame.save_ra(c, 0x50);
                    frame.save_fpr(c, 20, 0x90);
                    let this = r32(c, S2);
                    c.g[AT][0] = 0x4130_0000;
                    c.mtc1(AT, 2);
                    c.lwc1(1, this.wrapping_add(TIMER));
                    c.add_s(1, 1, 0);
                    c.c_le_s(2, 1);
                    let waiting = c.fcc();
                    c.swc1(1, this.wrapping_add(TIMER));
                    if waiting {
                        c.cycles += 6;
                        c.g[AT][0] = 0x4180_0000;
                        c.mtc1(AT, 0);
                        c.c_le_s(0, 1);
                        let reset = c.fcc();
                        frame.restore(c, S0, 0x80);
                        if reset {
                            c.cycles += 51;
                            let stack = sp(c);
                            c.g[AT][0] = 0xffff_ffff_c120_0000;
                            c.mtc1(AT, 0);
                            c.g[A1][0] = sx(stack);
                            Addr(stack).sw(c, 0);
                            c.swc1(0, stack.wrapping_add(8));
                            Addr(stack).off(4).sw(c, 0);
                            Addr(stack).off(0xc).sw(c, 0);
                            call_ptr(c, TRANSLATE, 0x0035_e8c8)?;
                            pc = 0x0035_e8c8;
                        } else {
                            pc = EPILOGUE;
                        }
                    } else if prepare_body(c) {
                        call_ptr(c, INTEGRATE, 0x0035_ec20)?;
                        pc = 0x0035_ec20;
                    } else {
                        pc = AFTER_BODY;
                    }
                }
                0x0035_e8c8 => {
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
                    call_ptr(c, GET_POSE, 0x0035_e8d4)?;
                    pc = 0x0035_e8d4;
                }
                0x0035_e8d4 => {
                    c.g[A1][0] = sx(r32(c, S2).wrapping_add(TRANSFORM));
                    roller_pose_matrix(c, sp(c).wrapping_add(0x10), r32(c, A1));
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 12);
                    c.g[A0][0] = sx(r32(c, S2).wrapping_add(TREE));
                    call_ptr(c, SET_TREE_TRANSFORM, 0x0035_e96c)?;
                    pc = 0x0035_e96c;
                }
                0x0035_e96c => {
                    frame.restore(c, S0, 0x80);
                    pc = EPILOGUE;
                }
                0x0035_ec20 => {
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, UPDATE_INERTIA, 0x0035_ec28)?;
                    pc = 0x0035_ec28;
                }
                0x0035_ec28 => {
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, UPDATE_MOMENTUM, 0x0035_ec30)?;
                    pc = 0x0035_ec30;
                }
                0x0035_ec30 => {
                    c.g[S0][0] = sx(sp(c).wrapping_add(0x20));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, GET_POSE, 0x0035_ec40)?;
                    pc = 0x0035_ec40;
                }
                0x0035_ec40 => {
                    roller_pose_matrix(c, sp(c).wrapping_add(0x20), r32(c, S1));
                    c.mov_s(12, 20);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = sx(r32(c, S2).wrapping_add(TREE));
                    call_ptr(c, SET_TREE_TRANSFORM, 0x0035_ecd4)?;
                    pc = 0x0035_ecd4;
                }
                0x0035_ecd4 => {
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, COLLIDE_WORLD, 0x0035_ecdc)?;
                    pc = 0x0035_ecdc;
                }
                0x0035_ecdc => {
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, GET_POSE, 0x0035_ece8)?;
                    pc = 0x0035_ece8;
                }
                0x0035_ece8 => {
                    roller_pose_matrix(c, sp(c).wrapping_add(0x20), r32(c, S1));
                    pc = AFTER_BODY;
                }
                AFTER_BODY => {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x80);
                    pc = EPILOGUE;
                }
                EPILOGUE => {
                    c.cycles += 6;
                    frame.restore(c, S1, 0x70);
                    frame.restore(c, S2, 0x60);
                    frame.restore_ra(c, 0x50);
                    frame.restore_fpr(c, 20, 0x90);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}

/// `RollerModifier` fields used by the update.
mod roller {
    /// Running average of the kinetic energy (below `gp-0x29e0` = 10000 the
    /// roller stops).
    pub const ENERGY_AVG: u32 = 0x4;
    /// State timer (s): 10 stopped, 11..16 waiting, 16 reset.
    pub const TIMER: u32 = 0x20;
    /// Velocity damping (decays per tick, `tick::PER_TICK`).
    pub const DAMPING: u32 = 0x28;
    /// Linear and angular momentum, mass, linear/angular velocity.
    pub const MOMENTUM: u32 = 0x50;
    pub const ANGULAR: u32 = 0x60;
    pub const MASS: u32 = 0x70;
    pub const VELOCITY: u32 = 0xb0;
    pub const SPIN: u32 = 0xc0;
    /// The collision sphere tree and its transform.
    pub const TREE: u32 = 0xe0;
    pub const TRANSFORM: u32 = 0x240;
}

/// The VU0 sequence that turns the pose at `pose` (quaternion at `+0`,
/// position at `+0x10`) into the 4x4 transform at `out` (rotation rows
/// `+0..+0x20`, position `+0x30`).
fn roller_pose_matrix(c: &mut Cpu, pose: u32, out: u32) {
    c.lqc2(4, pose.wrapping_add(0x10));
    c.lqc2(3, pose);
    c.vu0().add(0b1110, 1, 0, Src::Bc(0, W));
    c.vu0().add(0b1110, 5, 4, Src::Vec(4));
    c.vu0().sub(0b0001, 10, 10, Src::Vec(10));
    c.vu0().sub(0b0001, 11, 11, Src::Vec(11));
    c.vu0().sub(0b0001, 12, 12, Src::Vec(12));
    c.vu0().mul(0b1110, 6, 5, Src::Vec(4));
    c.vu0().mul(0b1110, 7, 5, Src::Bc(4, W));
    c.vu0().opmula(5, 4);
    c.vu0().madd(0b1110, 8, 0, Src::Vec(0));
    c.vu0().suba(0b1000, 1, Src::Bc(6, Y));
    c.vu0().msub(0b1000, 10, 1, Src::Bc(6, Z));
    c.vu0().suba(0b0100, 1, Src::Bc(6, Z));
    c.vu0().msub(0b0100, 11, 1, Src::Bc(6, X));
    c.vu0().suba(0b0010, 1, Src::Bc(6, X));
    c.vu0().msub(0b0010, 12, 1, Src::Bc(6, Y));
    c.vu0().adda(0b0100, 0, Src::Bc(8, Z));
    c.vu0().madd(0b0100, 10, 1, Src::Bc(7, Z));
    c.vu0().adda(0b0010, 0, Src::Bc(8, X));
    c.vu0().madd(0b0010, 11, 1, Src::Bc(7, X));
    c.vu0().adda(0b0100, 0, Src::Bc(8, X));
    c.vu0().msub(0b0100, 12, 1, Src::Bc(7, X));
    c.vu0().adda(0b0010, 0, Src::Bc(8, Y));
    c.vu0().msub(0b0010, 10, 1, Src::Bc(7, Y));
    c.vu0().adda(0b1000, 0, Src::Bc(8, Z));
    c.vu0().msub(0b1000, 11, 1, Src::Bc(7, Z));
    c.vu0().adda(0b1000, 0, Src::Bc(8, Y));
    c.vu0().madd(0b1000, 12, 1, Src::Bc(7, Y));
    c.sqc2(3, out.wrapping_add(0x30));
    c.sqc2(10, out);
    c.sqc2(11, out.wrapping_add(0x10));
    c.sqc2(12, out.wrapping_add(0x20));
}

/// `w = dot(a, b)` of two quadwords (VU0, the `.w` lane of vf4), moved to
/// `f{fr}` through v0.
fn roller_dot(c: &mut Cpu, a: u32, b: u32, fr: usize) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(V0, 4);
    c.mtc1(V0, fr);
}

fn prepare_body(c: &mut Cpu) -> bool {
    use roller::*;
    let this = r32(c, S2);
    c.cycles += 24;
    c.lwc1(3, this.wrapping_add(MASS));
    roller_dot(
        c,
        this.wrapping_add(VELOCITY),
        this.wrapping_add(MOMENTUM),
        1,
    );
    c.mtc1(ZERO, 2);
    roller_dot(c, this.wrapping_add(SPIN), this.wrapping_add(ANGULAR), 0);
    c.c_eq_s(3, 2);
    let massless = c.fcc();
    c.add_s(5, 1, 0);
    if massless {
        c.cycles += 2;
        c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
        c.mtc1(AT, 4);
    } else {
        c.cycles += 5;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.div_s(4, 0, 3);
    }

    // avg = avg * 0.95 + energy * 0.05 (per tick).
    c.cycles += 12;
    let hz = crate::tick::rate(c);
    let kept = if hz == 60 {
        None
    } else {
        let entry = gp(c, -0x29e4);
        let k = f32::from_bits(c.bus.read32(entry));
        let k = k.powf(60.0 / hz as f32);
        Some((FpuF32::from_f32(k), FpuF32::from_f32(1.0 - k)))
    };
    c.lwc1(1, gp(c, -0x29e8));
    if let Some((_, w)) = kept {
        c.f[1] = w.0;
    }
    c.lwc1(0, this.wrapping_add(ENERGY_AVG));
    c.mul_s(1, 5, 1);
    c.lwc1(2, gp(c, -0x29e4));
    if let Some((k, _)) = kept {
        c.f[2] = k.0;
    }
    c.lwc1(3, gp(c, -0x29e0));
    c.mul_s(0, 0, 2);
    c.div_s(1, 1, 4);
    c.add_s(0, 0, 1);
    c.c_lt_s(0, 3);
    let slow = c.fcc();
    c.swc1(0, this.wrapping_add(ENERGY_AVG));
    let mut stop = slow;
    if !slow {
        c.cycles += 7;
        c.lwc1(3, this.wrapping_add(TIMER));
        c.g[AT][0] = 0x4120_0000; // 10.0
        c.mtc1(AT, 5);
        c.c_le_s(5, 3);
        stop = c.fcc();
        if !stop {
            c.lwc1(0, this.wrapping_add(DAMPING));
        }
    }
    if stop {
        roller_stop(c, this);
        false
    } else {
        prepare_step(c, this);
        true
    }
}

fn roller_stop(c: &mut Cpu, this: u32) {
    use roller::*;
    c.cycles += 59;
    let s = sp(c);
    for k in 0..4 {
        Addr(s).off(4 * k).sw(c, 0);
    }
    c.g[AT][0] = 0x4130_0000; // 11.0
    c.mtc1(AT, 0);
    c.lq(V0, s);
    c.swc1(0, this.wrapping_add(TIMER));
    c.sq(V0, this.wrapping_add(MOMENTUM));
    c.lwc1(0, this.wrapping_add(MASS));
    c.mfc1(V0, 0);
    c.lqc2(4, s);
    c.qmtc2(V0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x20));
    for k in 0..4 {
        Addr(s).off(4 * k).sw(c, 0);
    }
    c.lq(V0, s);
    c.lwc1(3, this.wrapping_add(0x80));
    c.sq(V0, this.wrapping_add(ANGULAR));
    c.lwc1(9, this.wrapping_add(0x8c));
    c.lwc1(2, this.wrapping_add(0x60));
    c.lwc1(4, this.wrapping_add(0x64));
    c.lwc1(0, this.wrapping_add(0x84));
    c.mul_s(3, 3, 2);
    c.lwc1(7, this.wrapping_add(0x90));
    c.mul_s(9, 9, 4);
    c.lwc1(1, this.wrapping_add(0x88));
    c.mul_s(0, 0, 2);
    c.lwc1(8, this.wrapping_add(0x94));
    c.mul_s(7, 7, 4);
    c.lwc1(6, this.wrapping_add(0x68));
    c.mul_s(1, 1, 2);
    c.lwc1(5, this.wrapping_add(0x98));
    c.mul_s(8, 8, 4);
    c.lwc1(2, this.wrapping_add(0x9c));
    c.add_s(3, 3, 9);
    c.lwc1(4, this.wrapping_add(0xa0));
    c.mul_s(5, 5, 6);
    c.add_s(0, 0, 7);
    c.lq(V0, s.wrapping_add(0x20));
    c.mul_s(2, 2, 6);
    c.add_s(1, 1, 8);
    c.sq(V0, this.wrapping_add(VELOCITY));
    c.mul_s(4, 4, 6);
    c.add_s(3, 3, 5);
    c.add_s(0, 0, 2);
    c.add_s(1, 1, 4);
    c.swc1(3, s.wrapping_add(0x20));
    c.swc1(0, s.wrapping_add(0x24));
    c.swc1(1, s.wrapping_add(0x28));
    c.lwc1(0, this.wrapping_add(0x6c));
    c.swc1(0, s.wrapping_add(0x2c));
    c.lq(V0, s.wrapping_add(0x20));
    c.sq(V0, this.wrapping_add(SPIN));
    c.sq(V0, s.wrapping_add(0x10));
}

fn prepare_step(c: &mut Cpu, this: u32) {
    use roller::*;
    // The damping decays; the drag scale is min(damping * 10, 2) (0 when
    // negative).
    c.cycles += 13;
    c.lwc1(1, gp(c, -0x29dc));
    c.mtc1(ZERO, 4);
    c.mul_s(0, 0, 1);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 20);
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(AT, 2);
    c.mov_s(6, 4);
    c.mul_s(1, 0, 5);
    c.c_le_s(4, 1);
    let positive = c.fcc();
    c.swc1(0, this.wrapping_add(DAMPING));
    if positive {
        c.cycles += 1;
        c.min_s(6, 1, 2);
    }

    // Forces: gravity (-980 / mass on z) and drag on both momenta, then
    // integrate one step.
    c.cycles += 131;
    let s = sp(c);
    c.lwc1(2, this.wrapping_add(MASS));
    c.add_s(0, 3, 3);
    c.g[AT][0] = 0xffff_ffff_c475_0000; // -980.0
    c.mtc1(AT, 1);
    c.g[S0][0] = sx(this.wrapping_add(0x30));
    c.swc1(4, s.wrapping_add(0x10));
    c.g[A1][0] = sx(s);
    c.div_s(1, 1, 2);
    c.swc1(4, s.wrapping_add(0x14));
    c.add_s(0, 0, 20);
    c.swc1(4, s.wrapping_add(0x1c));
    c.lwc1(12, gp(c, -0x29d8));
    c.g[A2][0] = sx(s.wrapping_add(0x10));
    c.g[A0][0] = c.g[S0][0];
    c.g[S1][0] = sx(this.wrapping_add(TRANSFORM));
    c.mul_s(0, 6, 0);
    c.swc1(1, s.wrapping_add(0x18));
    c.mfc1(V1, 0);
    c.lqc2(4, this.wrapping_add(MOMENTUM));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x30));
    c.lq(V0, s.wrapping_add(0x30));
    c.sq(V0, s.wrapping_add(0x20));
    c.lqc2(3, s.wrapping_add(0x10));
    c.lqc2(4, s.wrapping_add(0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x30));
    c.lq(V0, s.wrapping_add(0x30));
    c.swc1(4, s.wrapping_add(0x2c));
    c.sq(V0, s);
    c.swc1(4, s.wrapping_add(0x20));
    c.swc1(4, s.wrapping_add(0x24));
    c.swc1(4, s.wrapping_add(0x28));
    c.mfc1(V1, 0);
    c.lqc2(4, this.wrapping_add(ANGULAR));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x40));
    c.lq(V0, s.wrapping_add(0x40));
    c.sq(V0, s.wrapping_add(0x30));
    c.lqc2(3, s.wrapping_add(0x20));
    c.lqc2(4, s.wrapping_add(0x30));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x40));
    c.lq(V0, s.wrapping_add(0x40));
    c.sq(V0, s.wrapping_add(0x10));
}
