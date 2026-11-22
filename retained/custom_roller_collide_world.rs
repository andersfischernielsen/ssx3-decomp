//! Native RollerModifier_CollideWorld (0x0035edc8), retained uninstalled.
//!
//! Generated EE is the authority. No tests or builds were added or run;
//! the user has disabled automated validation. No physics policy is changed.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0035_edc8;
const END: u32 = 0x0035_f0b8;
const FRAME_BYTES: u32 = 0x1a0;
const CLEANUP: u32 = 0x0035_f084;
const DESTROY_QUERY: u32 = 0x0035_f088;

const POSITION: u32 = 0x30;
const MOMENTUM: u32 = 0x50;
const ANGULAR_MOMENTUM: u32 = 0x60;
const VELOCITY: u32 = 0xb0;
const ANGULAR_VELOCITY: u32 = 0xc0;
const SPHERE_TREE: u32 = 0xe0;
const QUERY_CACHE: u32 = 0x2c0;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void RollerModifier_CollideWorld(RollerModifier* self)`.
/// Build a temporary sphere-tree query, find the closest world contact and
/// move the body out by `(penetration + 2) * normal`. Damp both momenta and
/// reconstruct the contact-point velocity. If it points into the contact,
/// pass the query's position/normal and the body's response coefficients to
/// ResolveContact, then increase damping and possibly advance the stop timer.
/// This function has no contact-script dispatch or target-instance argument.
/// Calls retain the original registers and native return continuations.
pub fn roller_modifier_collide_world(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 32;
                    Frame::open(c, FRAME_BYTES);
                    c.g[A2][0] = 1;
                    frame.save(c, S2, 0x160);
                    frame.save(c, S0, 0x180);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S1, 0x170);
                    c.g[S0][0] = sx(at(c, SP, 0x80));
                    c.g[S1][0] = sx(at(c, S2, SPHERE_TREE));
                    frame.save(c, S3, 0x150);
                    frame.save_fpr(c, 21, 0x198);
                    c.g[A0][0] = c.g[S0][0];
                    frame.save_fpr(c, 20, 0x190);
                    c.g[A1][0] = c.g[S1][0];
                    frame.save_ra(c, 0x140);
                    c.g[S3][0] = c.g[S0][0];
                    call_ptr(c, 0x0033_03f0, 0x0035_ee08)?; // CollSphereTree_ctor
                    pc = 0x0035_ee08;
                }
                0x0035_ee08 => {
                    c.mtc1(ZERO, 21);
                    call_ptr(c, 0x002d_1be0, 0x0035_ee14)?; // current Nash
                    pc = 0x0035_ee14;
                }
                0x0035_ee14 => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = sx(at(c, S2, QUERY_CACHE));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[SP][0];
                    call_ptr(c, 0x0033_6850, 0x0035_ee28)?; // Nash_FindClosestContact
                    pc = 0x0035_ee28;
                }
                0x0035_ee28 => {
                    c.mov_s(20, 0);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0032_c5a8, 0x0035_ee34)?;
                    pc = 0x0035_ee34;
                }
                0x0035_ee34 => {
                    c.sub_s(20, 20, 0);
                    c.c_lt_s(21, 20);
                    c.g[A0][0] = c.g[S2][0];
                    if !c.fcc() {
                        pc = CLEANUP;
                        continue;
                    }
                    // The generated block charges all following operations
                    // through the normal-velocity comparison before this call.
                    c.cycles += 110;
                    c.g[AT][0] = 0x4000_0000;
                    c.mtc1(AT, 0);
                    c.g[A1][0] = sx(at(c, SP, 0xf0));
                    c.add_s(0, 20, 0);
                    c.mfc1(V1, 0);
                    c.lqc2(4, at(c, SP, 0x10));
                    c.qmtc2(V1, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at(c, SP, 0x100));
                    c.lq(V0, at(c, SP, 0x100));
                    c.sq(V0, at(c, SP, 0xf0));
                    call_ptr(c, 0x0035_ed90, 0x0035_ee78)?; // RollerModifier_Translate
                    pc = 0x0035_ee78;
                }
                0x0035_ee78 => {
                    damp_body_and_contact_velocity(c);
                    c.c_lt_s(0, 21);
                    if !c.fcc() {
                        // This direct branch skips CLEANUP's one-cycle block.
                        c.g[A0][0] = c.g[S3][0];
                        pc = DESTROY_QUERY;
                        continue;
                    }
                    c.cycles += 16;
                    c.lwc1(0, at(c, S2, 0x1c));
                    c.lqc2(3, at(c, S2, VELOCITY));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, X);
                    c.vu0().waitq();
                    c.cfc2(V0, 22);
                    c.mtc1(V0, 1);
                    c.lwc1(12, at(c, S2, 0x14));
                    c.c_lt_s(1, 0);
                    let slow = c.fcc();
                    c.lwc1(13, at(c, S2, 0x18));
                    if slow {
                        c.cycles += 1;
                        c.mov_s(12, 21);
                    }
                    c.cycles += 13;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = sx(at(c, SP, 0x10));
                    // A0 remains self+POSITION from the damping block.
                    call_ptr(c, 0x0035_d4a0, 0x0035_f050)?; // ResolveContact
                    pc = 0x0035_f050;
                }
                0x0035_f050 => {
                    c.lwc1(0, at(c, S2, 0x28));
                    c.lwc1(2, r(c, GP).wrapping_sub(0x29d0));
                    c.lwc1(3, at(c, S2, 0x20));
                    c.g[AT][0] = 0x40a0_0000;
                    c.mtc1(AT, 1);
                    c.add_s(0, 0, 2);
                    c.c_lt_s(1, 3);
                    let near_stop = c.fcc();
                    c.swc1(0, at(c, S2, 0x28));
                    if near_stop {
                        c.cycles += 3;
                        c.g[AT][0] = 0x4120_0000;
                        c.mtc1(AT, 0);
                        c.swc1(0, at(c, S2, 0x20));
                    }
                    pc = CLEANUP;
                }
                CLEANUP => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S3][0];
                    pc = DESTROY_QUERY;
                }
                DESTROY_QUERY => {
                    c.cycles += 11;
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0033_04e8, 0x0035_f090)?; // CollSphereTree_dtor
                    pc = 0x0035_f090;
                }
                0x0035_f090 => {
                    frame.restore(c, S0, 0x180);
                    frame.restore(c, S1, 0x170);
                    frame.restore(c, S2, 0x160);
                    frame.restore(c, S3, 0x150);
                    frame.restore_ra(c, 0x140);
                    frame.restore_fpr(c, 21, 0x198);
                    frame.restore_fpr(c, 20, 0x190);
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

/// Native VU/FPU sequence: damp angular momentum, apply the world inverse
/// inertia tensor, damp velocity/rebuild momentum, then form
/// dot(velocity + angularVelocity cross (contactPosition - position), normal).
/// Preserve the generated scratch stores and V0/V1/FPU/VU results verbatim.
fn damp_body_and_contact_velocity(c: &mut Cpu) {
    c.lwc1(10, r(c, GP).wrapping_sub(0x29d4));
    c.g[A0][0] = sx(at(c, S2, POSITION));
    c.mfc1(V1, 10);
    c.lqc2(4, at(c, S2, ANGULAR_MOMENTUM));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at(c, SP, 0x110));
    c.lq(V0, at(c, SP, 0x110));
    c.lwc1(4, at(c, S2, 0x80));
    c.sq(V0, at(c, S2, ANGULAR_MOMENTUM));
    c.lwc1(9, at(c, S2, 0x8c));
    c.lwc1(1, at(c, S2, ANGULAR_MOMENTUM));
    c.lwc1(5, at(c, S2, 0x64));
    c.lwc1(2, at(c, S2, 0x84));
    c.mul_s(4, 4, 1);
    c.lwc1(6, at(c, S2, 0x90));
    c.mul_s(9, 9, 5);
    c.lwc1(0, at(c, S2, 0x88));
    c.mul_s(2, 2, 1);
    c.lwc1(3, at(c, S2, 0x94));
    c.mul_s(6, 6, 5);
    c.mul_s(0, 0, 1);
    c.lwc1(7, at(c, S2, 0x68));
    c.mul_s(3, 3, 5);
    c.lwc1(8, at(c, S2, 0x98));
    c.lwc1(5, at(c, S2, 0x9c));
    c.add_s(2, 2, 6);
    c.lwc1(1, at(c, S2, 0xa0));
    c.add_s(4, 4, 9);
    c.add_s(0, 0, 3);
    c.lwc1(6, at(c, S2, 0x6c));
    c.mul_s(1, 1, 7);
    c.sq(V0, at(c, SP, 0x100));
    c.mul_s(8, 8, 7);
    c.swc1(6, at(c, SP, 0x13c));
    c.mul_s(5, 5, 7);
    c.add_s(0, 0, 1);
    c.add_s(4, 4, 8);
    c.add_s(2, 2, 5);
    c.swc1(0, at(c, SP, 0x138));
    c.swc1(4, at(c, SP, 0x130));
    c.swc1(2, at(c, SP, 0x134));
    c.lq(V0, at(c, SP, 0x130));
    c.sq(V0, at(c, S2, ANGULAR_VELOCITY));
    c.sq(V0, at(c, SP, 0x120));
    c.mfc1(V1, 10);
    c.lqc2(4, at(c, S2, VELOCITY));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at(c, SP, 0x110));
    c.lq(V0, at(c, SP, 0x110));
    c.sq(V0, at(c, SP, 0x100));
    c.lwc1(0, at(c, A0, 0x40));
    c.mfc1(V0, 0);
    c.qmtc2(V0, 3);
    c.vu0().div(0, W, 3, X);
    c.lqc2(4, at(c, SP, 0x100));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, at(c, SP, 0x120));
    c.lq(V1, at(c, SP, 0x120));
    c.lq(V0, at(c, SP, 0x100));
    c.sq(V1, at(c, S2, MOMENTUM));
    c.sq(V0, at(c, S2, VELOCITY));
    c.sq(V1, at(c, SP, 0x110));
    c.lqc2(3, r(c, SP));
    c.lqc2(4, at(c, S2, POSITION));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, 0x130));
    c.lq(V0, at(c, SP, 0x130));
    c.sq(V0, at(c, SP, 0x120));
    c.lqc2(4, at(c, S2, ANGULAR_VELOCITY));
    c.lqc2(5, at(c, SP, 0x120));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, at(c, SP, 0x130));
    c.lq(V0, at(c, SP, 0x130));
    c.sq(V0, at(c, SP, 0x110));
    c.lqc2(3, at(c, S2, VELOCITY));
    c.lqc2(4, at(c, SP, 0x110));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, 0x130));
    c.lq(V0, at(c, SP, 0x130));
    c.sq(V0, at(c, SP, 0x100));
    c.lqc2(3, at(c, SP, 0x100));
    c.lqc2(5, at(c, SP, 0x10));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
}
