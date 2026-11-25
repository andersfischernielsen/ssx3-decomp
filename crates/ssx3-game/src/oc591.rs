//! Staging module oc591 (wave wave233): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc591`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0035_d340,
        roller_modifier_integrate,
        "RollerModifier_Integrate",
    ),
    (
        0x0035_d4a0,
        roller_modifier_resolve_contact,
        "RollerModifier_ResolveContact",
    ),
    (
        0x0035_d908,
        roller_modifier_apply_impulse,
        "RollerModifier_ApplyImpulse",
    ),
    (
        0x0035_dde8,
        roller_modifier_init_body,
        "RollerModifier_InitBody",
    ),
    (0x0036_0a28, dead_node_v01, "DeadNode_v01"),
    (0x0036_0a70, hide_dtor, "Hide_dtor"),
    (
        0x0036_0c78,
        dynamic_particle_modifier_v04,
        "DynamicParticleModifier_v04",
    ),
    (
        0x0036_0de8,
        mod_obj_call_transform_v16,
        "ModObj_CallTransformV16",
    ),
    (
        0x0036_0ed0,
        mod_obj_get_uv_scroll_matrix,
        "ModObj_GetUVScrollMatrix",
    ),
    (
        0x0036_0f20,
        mod_obj_free_transform_modifier,
        "ModObj_FreeTransformModifier",
    ),
    (
        0x0036_0f48,
        mod_obj_free_uv_scroll_modifier,
        "ModObj_FreeUVScrollModifier",
    ),
    (
        0x0036_0f70,
        mod_obj_free_tex_flip_modifier,
        "ModObj_FreeTexFlipModifier",
    ),
    (
        0x0036_0f98,
        mod_obj_free_boost_modifier,
        "ModObj_FreeBoostModifier",
    ),
    (
        0x0036_0fc0,
        mod_obj_remove_dead_particles,
        "ModObj_RemoveDeadParticles",
    ),
    (0x0036_0fe8, mod_obj_free_particles, "ModObj_FreeParticles"),
    (
        0x0036_1010,
        mod_obj_reset_particles,
        "ModObj_ResetParticles",
    ),
];

/// `void cRigidBody_stepMotion(RigidBody* body, Vec4* force, Vec4* torque, float dt)`
/// (0x35d340): one explicit step of a rigid body. Fields: `+0x00` position,
/// `+0x10` orientation quaternion, `+0x20` linear momentum, `+0x30` angular
/// momentum, `+0x80` velocity, `+0x90` angular velocity. Position moves by
/// `V*dt`, the quaternion by its derivative (renormalised), the momenta by
/// `force*dt` and `torque*dt`. The VU0 sequences are issued as the original.
pub fn roller_modifier_integrate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 87;
        let body = r32(c, A0);
        let force = r32(c, A1);
        let torque = r32(c, A2);
        let frame = Frame::open(c, 0x30);
        let s = sp(c);
        c.g[V1][0] = 0x3f00_0000; // 0.5 in the VU broadcast lanes
                                  // step = V * dt, position += step (copied through the stack).
        c.mfc1(2, 12);
        c.lqc2(4, body + 0x80);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s + 0x10);
        c.lq(2, s + 0x10);
        c.sq(2, s);
        c.lqc2(3, body);
        c.lqc2(4, s);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, body);
        // Quaternion derivative from Q and W, scaled by dt, added to Q.
        c.qmtc2(3, 1);
        c.lqc2(4, body + 0x10);
        c.lqc2(5, body + 0x90);
        c.vu0().suba(0b0001, 0, Src::Vec(0));
        c.vu0().msub(0b0001, 2, 0, Src::Bc(1, X));
        c.vu0().mul(0b1110, 7, 4, Src::Vec(5));
        c.vu0().mul(0b1110, 3, 5, Src::Bc(4, W));
        c.vu0().opmula(5, 4);
        c.vu0().opmsub(6, 4, 5);
        c.vu0().mula(0b0001, 2, Src::Bc(7, X));
        c.vu0().madda(0b0001, 2, Src::Bc(7, Y));
        c.vu0().madd(0b0001, 8, 2, Src::Bc(7, Z));
        c.vu0().mula(0b1110, 3, Src::Bc(1, X));
        c.vu0().madd(0b1110, 8, 6, Src::Bc(1, X));
        let dq = frame.at(c, 0x20);
        c.sqc2(8, dq);
        c.lq(2, dq);
        c.sq(2, s + 0x10);
        c.mfc1(3, 12);
        c.lqc2(4, s + 0x10);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, dq);
        c.lq(2, dq);
        c.lwc1(1, body + 0x10);
        c.sq(2, s);
        c.lwc1(4, s);
        c.lwc1(5, s + 0x4);
        c.lwc1(6, s + 0x8);
        c.add_s(1, 1, 4);
        c.lwc1(2, body + 0x14);
        c.lwc1(3, body + 0x18);
        c.lwc1(4, s + 0xc);
        c.add_s(2, 2, 5);
        c.lwc1(0, body + 0x1c);
        c.add_s(3, 3, 6);
        c.swc1(1, body + 0x10);
        c.add_s(0, 0, 4);
        c.swc1(2, body + 0x14);
        c.swc1(3, body + 0x18);
        c.swc1(0, body + 0x1c);
        // Renormalise Q (rsqrt of its squared length).
        c.lqc2(3, body + 0x10);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, body + 0x10);
        // Linear momentum += force * dt.
        c.mfc1(3, 12);
        c.lqc2(4, force);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s + 0x10);
        c.lq(2, s + 0x10);
        c.sq(2, s);
        c.lqc2(3, body + 0x20);
        c.lqc2(4, s);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, body + 0x20);
        // Angular momentum += torque * dt.
        c.mfc1(3, 12);
        c.lqc2(4, torque);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s + 0x10);
        c.lq(2, s + 0x10);
        c.sq(2, s);
        c.lqc2(3, body + 0x30);
        c.lqc2(4, s);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, body + 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void RollerModifier_ApplyImpulse(RollerModifier* self, Vec4* arm, Vec4* impulse)`
/// (0x35d908). Adds the impulse to the linear momentum at `+0x20` and `arm x
/// impulse` to the angular momentum at `+0x30`, then refreshes the derived
/// state: `+0x80` = momentum * invMass (`+0x40`), `+0x90..0x9c` = the
/// angular velocity from `+0x30` through the inverse inertia (`+0x50..+0x70`),
/// `+0x9c` = copy of `+0x3c`.
pub fn roller_modifier_apply_impulse(c: &mut Cpu) {
    c.cycles += 62;
    let body = r32(c, A0);
    let arm = r32(c, A1);
    let imp = r32(c, A2);
    c.lqc2(3, body + 0x20);
    c.lqc2(4, imp);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, body + 0x20);
    let frame = Frame::open(c, 0x20);
    let s = sp(c);
    // torque = arm x impulse, stored at the scratch slot +0x10.
    c.lqc2(4, arm);
    c.lqc2(5, imp);
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, s + 0x10);
    c.lq(2, s + 0x10);
    c.sq(2, s);
    c.lqc2(3, body + 0x30);
    c.lqc2(4, s);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, body + 0x30);
    // +0x80 = momentum * invMass.
    c.lwc1(0, body + 0x40);
    c.mfc1(2, 0);
    c.lqc2(4, body + 0x20);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s + 0x10);
    // Angular velocity from the inverse inertia tensor and L (+0x30).
    c.lwc1(3, body + 0x30);
    c.lwc1(5, body + 0x34);
    c.lwc1(2, body + 0x50);
    c.lwc1(9, body + 0x5c);
    c.lwc1(0, body + 0x54);
    c.mul_s(2, 2, 3);
    c.lwc1(7, body + 0x60);
    c.mul_s(9, 9, 5);
    c.lwc1(8, body + 0x64);
    c.mul_s(0, 0, 3);
    c.lwc1(1, body + 0x58);
    c.mul_s(7, 7, 5);
    c.lwc1(6, body + 0x38);
    c.mul_s(8, 8, 5);
    c.lwc1(4, body + 0x68);
    c.mul_s(1, 1, 3);
    c.lwc1(5, body + 0x6c);
    c.add_s(2, 2, 9);
    c.lwc1(3, body + 0x70);
    c.mul_s(4, 4, 6);
    c.add_s(0, 0, 7);
    c.lq(2, s + 0x10);
    c.mul_s(5, 5, 6);
    c.add_s(1, 1, 8);
    c.sq(2, body + 0x80);
    c.mul_s(3, 3, 6);
    c.add_s(2, 2, 4);
    c.add_s(0, 0, 5);
    c.add_s(1, 1, 3);
    c.swc1(2, s + 0x10);
    c.swc1(0, s + 0x14);
    c.swc1(1, s + 0x18);
    c.lwc1(0, body + 0x3c);
    c.swc1(0, s + 0x1c);
    c.lq(2, s + 0x10);
    c.sq(2, body + 0x90);
    c.sq(2, s);
    frame.close(c);
    ret(c);
}

/// `void cRigidBody_rigidImpact(RigidBody* body, Vec4* point, Vec4* n, float
/// restitution, float mu)` (0x35d4a0): resolve one contact. Friction first
/// (only when `mu != 0` and the tangential speed exceeds 0.001), then the
/// normal impulse with `(restitution + 1)`. Each impulse is applied through
/// `RollerModifier_ApplyImpulse`. A contact that still separates after the
/// friction impulse returns early.
pub fn roller_modifier_resolve_contact(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 51;
        let frame = Frame::open(c, 0x100);
        let s = sp(c);
        c.mov_s(10, 13); // f10 = mu
        c.sq(16, s + 0xe0);
        c.sq(18, s + 0xc0);
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, s + 0xd0);
        c.g[S2][0] = c.g[A1][0];
        c.swc1(21, s + 0xf8);
        c.g[S1][0] = c.g[A2][0];
        c.swc1(20, s + 0xf0);
        c.sd(s + 0xb0, c.g[RA][0]);
        c.mov_s(20, 12); // f20 = restitution
        let body = r32(c, S0);
        let point = r32(c, S2);
        let normal = r32(c, S1);
        let gp = r32(c, GP);

        let returned_early = 'friction: {
            // arm = point - position (stored at s); dir = normal * normalVel
            c.lqc2(3, point);
            c.lqc2(4, body);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s + 0x10);
            c.mtc1(0, 21); // f21 = 0.0
            c.lq(2, s + 0x10);
            c.c_eq_s(10, 21); // mu == 0
            c.sq(2, s);
            c.lqc2(3, point);
            c.lqc2(4, body);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s + 0x40);
            c.lq(2, s + 0x40);
            c.sq(2, s + 0x30);
            c.lqc2(4, body + 0x90);
            c.lqc2(5, s + 0x30);
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, s + 0x40);
            c.lq(2, s + 0x40);
            c.sq(2, s + 0x20);
            c.lqc2(3, body + 0x80);
            c.lqc2(4, s + 0x20);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s + 0x40);
            c.lq(2, s + 0x40);
            c.sq(2, s + 0x10);
            c.lqc2(3, s + 0x10);
            c.lqc2(5, normal);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 12); // f12 = normalVel
            if c.fcc() {
                // mu == 0: no friction.
                break 'friction false;
            }
            c.cycles += 27;
            // tangent = arm - normal * normalVel (the scaled normal lands in s+0x40)
            c.mfc1(2, 12);
            c.lqc2(4, normal);
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, s + 0x40);
            c.lwc1(0, gp.wrapping_sub(0x2a00)); // 0.001
            c.lq(2, s + 0x40);
            c.sq(2, s + 0x30);
            c.lqc2(3, s + 0x10);
            c.lqc2(4, s + 0x30);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s + 0x40);
            c.lq(2, s + 0x40);
            c.sq(2, s + 0x20);
            c.lqc2(3, s + 0x20);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 13); // f13 = tangential speed
            c.c_lt_s(0, 13); // 0.001 < speed
            if !c.fcc() {
                break 'friction false;
            }
            c.cycles += 117;
            // dir = tangent / speed; j = -mu * speed / (invMass + ...)
            c.mfc1(2, 13);
            c.qmtc2(2, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, s + 0x20);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 4, 4, Src::Q);
            c.sqc2(4, s + 0x40);
            c.neg_s(10, 10);
            c.lq(2, s + 0x40);
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = sx(s.wrapping_add(0x40));
            c.sq(2, s + 0x30);
            c.mul_s(10, 10, 13);
            c.lqc2(4, s);
            c.lqc2(5, s + 0x30);
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, s + 0x70);
            c.lq(2, s + 0x70);
            c.lwc1(4, body + 0x50);
            c.sq(2, s + 0x60);
            c.lwc1(9, body + 0x5c);
            c.lwc1(2, body + 0x54);
            c.lwc1(8, body + 0x60);
            c.lwc1(0, body + 0x58);
            c.lwc1(3, body + 0x64);
            c.lwc1(1, s + 0x60);
            c.lwc1(5, s + 0x64);
            c.mul_s(4, 4, 1);
            c.lwc1(7, body + 0x68);
            c.mul_s(8, 8, 5);
            c.lwc1(6, s + 0x68);
            c.mul_s(9, 9, 5);
            c.mul_s(2, 2, 1);
            c.mul_s(0, 0, 1);
            c.mul_s(3, 3, 5);
            c.lwc1(1, body + 0x70);
            c.lwc1(5, body + 0x6c);
            c.add_s(2, 2, 8);
            c.mul_s(1, 1, 6);
            c.lwc1(8, s + 0x6c);
            c.add_s(4, 4, 9);
            c.add_s(0, 0, 3);
            c.swc1(8, s + 0x7c);
            c.mul_s(7, 7, 6);
            c.mul_s(5, 5, 6);
            c.add_s(0, 0, 1);
            c.add_s(4, 4, 7);
            c.add_s(2, 2, 5);
            c.swc1(0, s + 0x78);
            c.swc1(4, s + 0x70);
            c.swc1(2, s + 0x74);
            c.lq(2, s + 0x70);
            c.sq(2, s + 0x50);
            c.lqc2(4, s + 0x50);
            c.lqc2(5, s);
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, s + 0x70);
            c.lq(2, s + 0x70);
            c.sq(2, s + 0x40);
            c.lwc1(0, body + 0x40);
            c.lqc2(3, s + 0x30);
            c.lqc2(5, s + 0x40);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 1);
            c.add_s(0, 0, 1);
            c.div_s(10, 10, 0); // j
            c.mfc1(3, 10);
            c.lqc2(4, s + 0x30);
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, s + 0x50);
            c.lq(2, s + 0x50);
            c.sq(2, s + 0x40);
            call_ptr(c, 0x0035_d908, 0x0035_d724)?; // ApplyImpulse(body, arm, friction impulse)
                                                    // Relative velocity along the normal after the friction impulse.
            c.lqc2(3, point);
            c.lqc2(4, body);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s + 0xa0);
            c.lq(2, s + 0xa0);
            c.sq(2, s + 0x90);
            c.lqc2(4, body + 0x90);
            c.lqc2(5, s + 0x90);
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, s + 0xa0);
            c.lq(2, s + 0xa0);
            c.sq(2, s + 0x80);
            c.lqc2(3, body + 0x80);
            c.lqc2(4, s + 0x80);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s + 0xa0);
            c.lq(2, s + 0xa0);
            c.sq(2, s + 0x10);
            c.sq(2, s + 0x50);
            c.lqc2(3, s + 0x10);
            c.lqc2(5, normal);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 12);
            c.c_lt_s(21, 12); // 0 < normalVel: separating, stop here
            c.fcc()
        };

        if !returned_early {
            c.cycles += 79;
            c.lqc2(4, s);
            c.lqc2(5, normal);
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, s + 0x50);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 9);
            c.lq(2, s + 0x50);
            c.g[A0][0] = c.g[S0][0];
            c.lwc1(5, body + 0x50);
            c.add_s(9, 20, 9); // 1 + restitution
            c.sq(2, s + 0x40);
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = sx(s.wrapping_add(0x20));
            c.lwc1(10, body + 0x5c);
            c.neg_s(9, 9);
            c.lwc1(2, body + 0x54);
            c.lwc1(8, body + 0x60);
            c.lwc1(0, body + 0x58);
            c.mul_s(9, 9, 12);
            c.lwc1(3, body + 0x64);
            c.lwc1(1, s + 0x40);
            c.lwc1(4, s + 0x44);
            c.mul_s(5, 5, 1);
            c.lwc1(7, body + 0x68);
            c.mul_s(8, 8, 4);
            c.lwc1(6, s + 0x48);
            c.mul_s(10, 10, 4);
            c.mul_s(2, 2, 1);
            c.mul_s(0, 0, 1);
            c.mul_s(3, 3, 4);
            c.lwc1(1, body + 0x70);
            c.lwc1(4, body + 0x6c);
            c.add_s(2, 2, 8);
            c.mul_s(1, 1, 6);
            c.lwc1(8, s + 0x4c);
            c.add_s(5, 5, 10);
            c.add_s(0, 0, 3);
            c.swc1(8, s + 0x5c);
            c.mul_s(7, 7, 6);
            c.mul_s(4, 4, 6);
            c.add_s(0, 0, 1);
            c.add_s(5, 5, 7);
            c.add_s(2, 2, 4);
            c.swc1(0, s + 0x58);
            c.swc1(5, s + 0x50);
            c.swc1(2, s + 0x54);
            c.lq(2, s + 0x50);
            c.sq(2, s + 0x30);
            c.lqc2(4, s + 0x30);
            c.lqc2(5, s);
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, s + 0x50);
            c.lq(2, s + 0x50);
            c.lwc1(0, body + 0x40);
            c.sq(2, s + 0x20);
            c.lqc2(3, normal);
            c.lqc2(5, s + 0x20);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 1);
            c.add_s(0, 0, 1);
            c.div_s(9, 9, 0); // j
            c.mfc1(3, 9);
            c.lqc2(4, normal);
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, s + 0x30);
            c.lq(2, s + 0x30);
            c.sq(2, s + 0x20);
            call_ptr(c, 0x0035_d908, 0x0035_d8e4)?; // ApplyImpulse(body, arm, normal impulse)
        }
        c.lq(16, s + 0xe0);
        c.cycles += 7;
        c.lq(17, s + 0xd0);
        c.lq(18, s + 0xc0);
        c.g[RA][0] = c.ld(s + 0xb0);
        c.lwc1(21, s + 0xf8);
        c.lwc1(20, s + 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// `void RollerModifier_InitBody(RollerModifier* self, Vec4* a1)` (0x35dde8):
/// sets up the body's derived terms: a clamped factor `min(100 + gp_const /
/// self->+0x8, 600)` (f3), the inverse-inertia rows from `a1`'s vectors
/// (`+0x10`, `+0x20`, `+0x30`), then `RollerModifier_SetAngularMomentum` and
/// the two dot-product rows that fill `+0x4` and `+0x50..+0x70`.
pub fn roller_modifier_init_body(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x90);
        let s = sp(c);
        let gp = r32(c, GP);
        c.lwc1(0, gp.wrapping_sub(0x29f0));
        c.sq(16, s + 0x80);
        c.g[A3][0] = c.g[A1][0];
        c.sd(s + 0x70, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        let body = r32(c, S0);
        let a1 = r32(c, A3);
        // f3 = clamp(f0 / body[+8] + 100, <= 600)
        c.g[AT][0] = 0x42c8_0000; // 100.0
        c.mtc1(1, 2);
        c.lwc1(1, body + 0x8);
        c.div_s(0, 0, 1);
        c.g[AT][0] = 0x4416_0000; // 600.0
        c.mtc1(1, 1);
        c.add_s(3, 0, 2);
        c.c_lt_s(1, 3);
        c.sw(body + 0x28, 0);
        if c.fcc() {
            c.cycles += 1;
            c.mov_s(3, 1);
        }
        c.cycles += 79;
        let v1 = body + 0x30;
        c.lwc1(0, a1 + 0x30);
        c.g[V1][0] = sx(v1);
        c.mfc1(2, 0);
        c.lqc2(4, a1 + 0x10);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s + 0x30);
        c.g[AT][0] = 0x4000_0000; // 2.0
        c.mtc1(1, 1);
        c.lq(2, s + 0x30);
        c.g[A0][0] = c.g[V1][0];
        c.g[AT][0] = 0x4120_0000; // 10.0
        c.mtc1(1, 2);
        c.g[A1][0] = c.g[SP][0];
        c.sq(2, s + 0x20);
        c.mfc1(6, 1);
        c.qmtc2(6, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, s + 0x20);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 4, 4, Src::Q);
        c.sqc2(4, s + 0x30);
        c.lq(2, s + 0x30);
        c.swc1(3, s + 0x38);
        c.sq(2, s + 0x10);
        c.sw(s + 0x30, 0);
        c.sw(s + 0x34, 0);
        c.sw(s + 0x3c, 0);
        c.lqc2(3, s + 0x10);
        c.lqc2(4, s + 0x30);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, s + 0x40);
        c.lq(2, s + 0x40);
        c.sq(2, s);
        c.lwc1(0, v1 + 0x40);
        c.mfc1(2, 0);
        c.qmtc2(2, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, s);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 4, 4, Src::Q);
        c.sqc2(4, s + 0x60);
        c.lq(3, s + 0x60);
        c.lq(2, s);
        c.sq(3, body + 0x50);
        c.sq(2, body + 0xb0);
        c.sq(3, s + 0x50);
        c.mfc1(6, 2);
        c.lqc2(4, a1 + 0x20);
        c.qmtc2(6, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s + 0x10);
        c.lq(2, s + 0x10);
        c.sq(2, s);
        // SetAngularMomentum(body + 0x30, sp, 10.0)
        call_ptr(c, 0x0035_cff0, 0x0035_df0c)?;
        c.lqc2(3, body + 0xb0);
        c.lqc2(5, body + 0x50);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.lqc2(3, body + 0xc0);
        c.lqc2(5, body + 0x60);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 1);
        c.add_s(0, 0, 1);
        c.g[RA][0] = c.ld(s + 0x70);
        c.swc1(0, body + 0x4);
        c.lq(16, s + 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `void DeadNode_v01(DeadNode* self)` (0x360a28): install the DeadNode vtable
/// (0x491b00) at `+0xc`, then run the base destructor 0x34fbf0 on `self`.
pub fn dead_node_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x0049_1b00;
        c.g[V1][0] = c.g[A0][0];
        let obj = r32(c, V1);
        c.sw(obj.wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x0034_fbf0, 0x0036_0a44)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Hide_dtor(Hide* self)` (0x360a70): as `DeadNode_v01`, with the
/// vtable 0x491800 and the base destructor 0x34fbf0.
pub fn hide_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x0049_1800;
        c.g[V1][0] = c.g[A0][0];
        let obj = r32(c, V1);
        c.sw(obj.wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x0034_fbf0, 0x0036_0a8c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DynamicParticleModifier_v04(DynamicParticleModifier* self)`
/// (0x360c78): the deleting-destructor step, `DynamicEmitter_dtor(self)`.
pub fn dynamic_particle_modifier_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0037_0c08, 0x0036_0c88)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ModObj_CallTransformV16(ModObj* self)` (0x360de8): when the modifier
/// chain `self->+0x1c -> *p -> *q` is set, call the vtable entry `+0x84` of
/// `q` with `q + sext16(vtable[+0x80])`. Returns the callee's result, else 0.
pub fn mod_obj_call_transform_v16(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let owner = r32(c, A0);
        c.g[V1][0] = Addr(owner.wrapping_add(0x1c)).lw(c);
        c.g[V0][0] = 0;
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            let p = r32(c, V1);
            c.g[A1][0] = Addr(p).lw(c);
            let q_null = c.g[A1][0] == 0;
            frame.restore_ra(c, 0);
            if !q_null {
                c.cycles += 7;
                let q = r32(c, A1);
                c.g[V1][0] = Addr(q).lw(c);
                let vt = r32(c, V1);
                c.g[A0][0] = Addr(vt.wrapping_add(0x80)).lh(c);
                c.g[V0][0] = Addr(vt.wrapping_add(0x84)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx(q.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0036_0e1c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `float* ModObj_GetUVScrollMatrix(ModObj* self)` (0x360ed0): when
/// `self->+0x1c` and its `+0x4` are set and `*(+0x4)` is non-zero, forward to
/// `UVScrollModifier_GetMatrix` on `self->+0x1c->+0x4`. Otherwise returns 0.
pub fn mod_obj_get_uv_scroll_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let owner = r32(c, A0);
        c.g[A0][0] = Addr(owner.wrapping_add(0x1c)).lw(c);
        c.g[V0][0] = 0;
        let mut positive = 0u64;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            let obj = r32(c, A0);
            c.g[V0][0] = Addr(obj.wrapping_add(0x4)).lw(c);
            positive = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        let t = positive == 0;
        c.g[V0][0] = 0;
        if t {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            let obj = r32(c, A0);
            c.g[A0][0] = Addr(obj.wrapping_add(0x4)).lw(c);
            let scroll_null = c.g[A0][0] == 0;
            frame.restore_ra(c, 0);
            if !scroll_null {
                c.cycles += 4;
                call_ptr(c, 0x0035_fc20, 0x0036_0f08)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// Shared shape of the `ModObj_Free*` helpers: when `self->+0x1c` is set, run
/// its destructor `destructor` (returning to `ret`) on that child; otherwise
/// only the frame is torn down.
fn free_child_at_1c(c: &mut Cpu, destructor: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let owner = r32(c, A0);
        c.g[A0][0] = Addr(owner.wrapping_add(0x1c)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, destructor, ret)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void ModObj_FreeTransformModifier(ModObj* self)` (0x360f20).
pub fn mod_obj_free_transform_modifier(c: &mut Cpu) {
    free_child_at_1c(c, 0x0035_30d0, 0x0036_0f3c);
}

/// `void ModObj_FreeUVScrollModifier(ModObj* self)` (0x360f48).
pub fn mod_obj_free_uv_scroll_modifier(c: &mut Cpu) {
    free_child_at_1c(c, 0x0035_3118, 0x0036_0f64);
}

/// `void ModObj_FreeTexFlipModifier(ModObj* self)` (0x360f70).
pub fn mod_obj_free_tex_flip_modifier(c: &mut Cpu) {
    free_child_at_1c(c, 0x0035_3150, 0x0036_0f8c);
}

/// `void ModObj_FreeBoostModifier(ModObj* self)` (0x360f98).
pub fn mod_obj_free_boost_modifier(c: &mut Cpu) {
    free_child_at_1c(c, 0x0035_3188, 0x0036_0fb4);
}

/// `void ModObj_RemoveDeadParticles(ModObj* self)` (0x360fc0).
pub fn mod_obj_remove_dead_particles(c: &mut Cpu) {
    free_child_at_1c(c, 0x0035_3278, 0x0036_0fdc);
}

/// `void ModObj_FreeParticles(ModObj* self)` (0x360fe8).
pub fn mod_obj_free_particles(c: &mut Cpu) {
    free_child_at_1c(c, 0x0035_31d0, 0x0036_1004);
}

/// `void ModObj_ResetParticles(ModObj* self)` (0x361010).
pub fn mod_obj_reset_particles(c: &mut Cpu) {
    free_child_at_1c(c, 0x0035_3228, 0x0036_102c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, DATA};

    /// Gate word in the machine's RAM: the leaf fakes below act only where it
    /// was written, and fall back to the real body everywhere else.
    const GATE: u32 = DATA + 0x0020_0000;
    const GATE_MAGIC: u32 = 0x0591_0591;

    /// A leaf callee that returns at once; installed in the callee's override
    /// slot, so the generated and the replacement side both reach it.
    macro_rules! oc591_leaf {
        ($fake:ident, $generated:path) => {
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(GATE) != GATE_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                }
                ret(c)
            }
        };
    }

    oc591_leaf!(oc591_leaf_34fbf0, ee::s11::f_0034fbf0);
    oc591_leaf!(oc591_leaf_35cff0, ee::s11::f_0035cff0);
    oc591_leaf!(oc591_leaf_370c08, ee::s12::f_00370c08);
    oc591_leaf!(oc591_leaf_35fc20, ee::s12::f_0035fc20);
    oc591_leaf!(oc591_leaf_3530d0, ee::s11::f_003530d0);
    oc591_leaf!(oc591_leaf_353118, ee::s11::f_00353118);
    oc591_leaf!(oc591_leaf_353150, ee::s11::f_00353150);
    oc591_leaf!(oc591_leaf_353188, ee::s11::f_00353188);
    oc591_leaf!(oc591_leaf_353278, ee::s11::f_00353278);
    oc591_leaf!(oc591_leaf_3531d0, ee::s11::f_003531d0);
    oc591_leaf!(oc591_leaf_353228, ee::s11::f_00353228);

    fn install_leaves() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FBF0),
            oc591_leaf_34fbf0,
            fakes::prev_cell(oc591_leaf_34fbf0),
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0035CFF0),
            oc591_leaf_35cff0,
            fakes::prev_cell(oc591_leaf_35cff0),
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00370C08),
            oc591_leaf_370c08,
            fakes::prev_cell(oc591_leaf_370c08),
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0035FC20),
            oc591_leaf_35fc20,
            fakes::prev_cell(oc591_leaf_35fc20),
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003530D0),
            oc591_leaf_3530d0,
            fakes::prev_cell(oc591_leaf_3530d0),
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00353118),
            oc591_leaf_353118,
            fakes::prev_cell(oc591_leaf_353118),
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00353150),
            oc591_leaf_353150,
            fakes::prev_cell(oc591_leaf_353150),
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00353188),
            oc591_leaf_353188,
            fakes::prev_cell(oc591_leaf_353188),
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00353278),
            oc591_leaf_353278,
            fakes::prev_cell(oc591_leaf_353278),
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003531D0),
            oc591_leaf_3531d0,
            fakes::prev_cell(oc591_leaf_3531d0),
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00353228),
            oc591_leaf_353228,
            fakes::prev_cell(oc591_leaf_353228),
        );
    }

    fn gate_on(c: &mut Cpu) {
        c.bus.write32(GATE, GATE_MAGIC);
    }

    #[test]
    fn roller_modifier_integrate_matches() {
        check(
            ee::s11::f_0035d340__generated,
            roller_modifier_integrate,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0d340, 0);
                c.pc = 0x0035_d340;
                fill(c, &mut r, DATA, 0x80);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x140);
                c.f[12] = r.f32(2.0);
            },
        );
    }

    #[test]
    fn roller_modifier_apply_impulse_matches() {
        check(
            ee::s11::f_0035d908__generated,
            roller_modifier_apply_impulse,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0d908, 0);
                c.pc = 0x0035_d908;
                fill(c, &mut r, DATA, 0x80);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x140);
            },
        );
    }

    #[test]
    fn roller_modifier_resolve_contact_matches() {
        install_leaves();
        check(
            ee::s11::f_0035d4a0__generated,
            roller_modifier_resolve_contact,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0d4a0, 0);
                c.pc = 0x0035_d4a0;
                fill(c, &mut r, DATA, 0x80);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x140);
                c.f[12] = r.f32(1.0);
                c.f[13] = if seed % 4 == 0 { 0 } else { r.f32(1.0) };
            },
        );
    }

    #[test]
    fn roller_modifier_init_body_matches() {
        install_leaves();
        check(
            ee::s11::f_0035dde8__generated,
            roller_modifier_init_body,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0dde8, 0);
                gate_on(c);
                c.pc = 0x0035_dde8;
                fill(c, &mut r, DATA, 0x80);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
            },
        );
    }

    /// The leaf-callee dtor family: DeadNode_v01 and Hide_dtor share 0x34fbf0.
    #[test]
    fn dead_node_v01_matches() {
        install_leaves();
        check(ee::s12::f_00360a28__generated, dead_node_v01, 60, &|c, seed| {
            let _ = machine(c, seed, 0x0a28, 0);
            gate_on(c);
            c.pc = 0x0036_0a28;
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn hide_dtor_matches() {
        install_leaves();
        check(ee::s12::f_00360a70__generated, hide_dtor, 60, &|c, seed| {
            let _ = machine(c, seed, 0x0a70, 0);
            gate_on(c);
            c.pc = 0x0036_0a70;
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn dynamic_particle_modifier_v04_matches() {
        install_leaves();
        check(
            ee::s12::f_00360c78__generated,
            dynamic_particle_modifier_v04,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x0c78, 0);
                gate_on(c);
                c.pc = 0x0036_0c78;
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn mod_obj_call_transform_v16_matches() {
        check(
            ee::s12::f_00360de8__generated,
            mod_obj_call_transform_v16,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0de8, 1);
                c.pc = 0x0036_0de8;
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x1c, if seed % 5 == 0 { 0 } else { DATA + 0x100 });
                c.bus
                    .write32(DATA + 0x100, if seed % 7 == 0 { 0 } else { DATA + 0x200 });
                c.bus.write32(DATA + 0x200, DATA + 0x300);
                c.bus.write32(DATA + 0x380, r.next_u32());
                c.bus.write32(DATA + 0x384, fakes::FAKE);
            },
        );
    }

    #[test]
    fn mod_obj_get_uv_scroll_matrix_matches() {
        install_leaves();
        check(
            ee::s12::f_00360ed0__generated,
            mod_obj_get_uv_scroll_matrix,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x0ed0, 0);
                gate_on(c);
                c.pc = 0x0036_0ed0;
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x1c, if seed % 4 == 0 { 0 } else { DATA + 0x100 });
                c.bus.write32(
                    DATA + 0x100 + 0x4,
                    if seed % 3 == 0 { 0 } else { DATA + 0x200 },
                );
                c.bus.write32(
                    DATA + 0x200 + 0x4,
                    if seed % 5 == 0 { 0 } else { DATA + 0x300 },
                );
            },
        );
    }

    /// The `ModObj_Free*` family: one child set or null (seed % 3), the
    /// child's destructor replaced by the leaf fake.
    fn check_free_child(generated: GuestFn, hand: GuestFn, entry: u32, salt: u64) {
        install_leaves();
        check(generated, hand, 120, &|c, seed| {
            let _ = machine(c, seed, salt, 0);
            gate_on(c);
            c.pc = entry;
            c.g[A0][0] = sx(DATA);
            c.bus
                .write32(DATA + 0x1c, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
        });
    }

    #[test]
    fn mod_obj_free_transform_modifier_matches() {
        check_free_child(
            ee::s12::f_00360f20__generated,
            mod_obj_free_transform_modifier,
            0x0036_0f20,
            0x0f20,
        );
    }

    #[test]
    fn mod_obj_free_uv_scroll_modifier_matches() {
        check_free_child(
            ee::s12::f_00360f48__generated,
            mod_obj_free_uv_scroll_modifier,
            0x0036_0f48,
            0x0f48,
        );
    }

    #[test]
    fn mod_obj_free_tex_flip_modifier_matches() {
        check_free_child(
            ee::s12::f_00360f70__generated,
            mod_obj_free_tex_flip_modifier,
            0x0036_0f70,
            0x0f70,
        );
    }

    #[test]
    fn mod_obj_free_boost_modifier_matches() {
        check_free_child(
            ee::s12::f_00360f98__generated,
            mod_obj_free_boost_modifier,
            0x0036_0f98,
            0x0f98,
        );
    }

    #[test]
    fn mod_obj_remove_dead_particles_matches() {
        check_free_child(
            ee::s12::f_00360fc0__generated,
            mod_obj_remove_dead_particles,
            0x0036_0fc0,
            0x0fc0,
        );
    }

    #[test]
    fn mod_obj_free_particles_matches() {
        check_free_child(
            ee::s12::f_00360fe8__generated,
            mod_obj_free_particles,
            0x0036_0fe8,
            0x0fe8,
        );
    }

    #[test]
    fn mod_obj_reset_particles_matches() {
        check_free_child(
            ee::s12::f_00361010__generated,
            mod_obj_reset_particles,
            0x0036_1010,
            0x1010,
        );
    }
}
