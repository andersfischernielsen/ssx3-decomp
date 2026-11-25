//! Staging module oc665 (wave g45.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc665`.

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
    (0x0035_da70, roller_modifier_ctor, "RollerModifier_ctor"),
    (0x0035_df70, roller_modifier_ctor_2, "RollerModifier_ctor_2"),
    (0x0035_edc8, roller_modifier_collide, "RollerModifier_CollideWorld"),
];

/// `RollerModifier_MemberCtorStub` (0x35cfe8): the member constructor for the
/// body at `this + 0x30`.
pub(crate) const ROLLER_MEMBER_CTOR_STUB: u32 = 0x0035_cfe8;
/// `SphereTree_ctor` (0x32c508): the bounding sphere tree at `this + 0xe0`.
pub(crate) const SPHERE_TREE_CTOR: u32 = 0x0032_c508;
/// `RollerModifier_CopyMatrixBlock` (0x327cc8): copy a 4x4 block into the
/// bounding sphere tree.
pub(crate) const ROLLER_COPY_MATRIX_BLOCK: u32 = 0x0032_7cc8;
/// `Mat4_ToPosQuat` (0x31b748).
pub(crate) const MAT4_TO_POS_QUAT: u32 = 0x0031_b748;
/// `RollerModifier_GetPose` (0x35e770).
pub(crate) const ROLLER_GET_POSE: u32 = 0x0035_e770;
/// `RollerModifier_UpdateInertia` (0x35e248).
pub(crate) const ROLLER_UPDATE_INERTIA: u32 = 0x0035_e248;
/// `RollerModifier_UpdateMomentum` (0x35d288).
pub(crate) const ROLLER_UPDATE_MOMENTUM: u32 = 0x0035_d288;
/// `RollerModifier_InitBody` (0x35dde8).
pub(crate) const ROLLER_INIT_BODY: u32 = 0x0035_dde8;
/// `RollerModifier_LoadBody` (0x35da38), the replay-frame load.
pub(crate) const ROLLER_LOAD_BODY: u32 = 0x0035_da38;

/// `cRollerModifier::cRollerModifier(const tAddRollerData& rdata,
/// const tIntersectInfo* ii, const tCollideInfo* ci)` (0x35da70): build the
/// modifier at `this` (a0). It sets the vtable, constructs the member body at
/// `this + 0x30` and the bounding tree at `this + 0xe0`, copies the matrix
/// block from the intersect info (`ii + 0x48`), derives the body's inverse
/// radius from the rdata (`rdata + 4`) and copies the sample pose from the
/// body at `ii + 0x50` (quad moves and the VU0 transform of the inertia), then
/// computes the pose and inertia and initialises the body with the collide
/// info (`ci`).
pub fn roller_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 221;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
        let frame_sp = c.g[SP][0] as u32;
        c.g[V0][0] = 0x490000;
        c.sq(16, frame_sp.wrapping_add(0xd0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x13f8));
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.sq(21, frame_sp.wrapping_add(0x80));
        c.sq(17, frame_sp.wrapping_add(0xc0));
        c.g[S5][0] = sx(this.wrapping_add(0x30));
        c.sq(18, frame_sp.wrapping_add(0xb0));
        c.g[A0][0] = c.g[S5][0];
        c.sq(19, frame_sp.wrapping_add(0xa0));
        c.g[S2][0] = c.g[A1][0];
        c.sq(20, frame_sp.wrapping_add(0x90));
        c.g[S1][0] = sx(this.wrapping_add(0xe0));
        c.sq(22, frame_sp.wrapping_add(0x70));
        c.g[S4][0] = c.g[A2][0];
        c.sd(frame_sp.wrapping_add(0x60), c.g[RA][0]);
        c.g[S6][0] = c.g[A3][0];
        c.sw(this, c.g[V0][0]);
        c.g[S3][0] = sx(this.wrapping_add(0x4));
        call_ptr(c, ROLLER_MEMBER_CTOR_STUB, 0x0035_dac4)?;

        c.g[V0][0] = 0x490000;
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf80));
        c.sw(this, c.g[V0][0]);
        call_ptr(c, SPHERE_TREE_CTOR, 0x0035_dad8)?;

        c.sw(this.wrapping_add(0x20), 0);
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x50));
        c.sw(this.wrapping_add(0xd0), c.g[V0][0]);
        c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x48));
        call_ptr(c, ROLLER_COPY_MATRIX_BLOCK, 0x0035_daf0)?;

        c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = sx(frame_sp.wrapping_add(0x10));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.swc1(0, this.wrapping_add(0x8));
        c.g[V1][0] = c.lw(this.wrapping_add(0x178));
        c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x8));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 3);
        c.swc1(2, this.wrapping_add(0xc));
        c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.div_s(1, 1, 0);
        c.swc1(1, this.wrapping_add(0x70));
        let body = c.g[V1][0] as u32;
        c.lwc1(0, body.wrapping_add(0x2c));
        c.swc1(0, frame_sp);
        c.lwc1(1, body.wrapping_add(0x30));
        c.swc1(1, frame_sp.wrapping_add(0x4));
        c.lwc1(0, body.wrapping_add(0x34));
        c.sw(frame_sp.wrapping_add(0xc), 0);
        c.swc1(0, frame_sp.wrapping_add(0x8));
        // The 12-byte sample (x, y, z, w) is moved with unaligned quad moves.
        c.lq(2, frame_sp);
        c.sq(2, this.wrapping_add(0x2b0));
        c.ldl(2, body.wrapping_add(0x63));
        c.ldr(2, body.wrapping_add(0x5c));
        c.ldl(5, body.wrapping_add(0x6b));
        c.ldr(5, body.wrapping_add(0x64));
        c.ldl(6, body.wrapping_add(0x73));
        c.ldr(6, body.wrapping_add(0x6c));
        c.ldl(7, body.wrapping_add(0x7b));
        c.ldr(7, body.wrapping_add(0x74));
        c.sdl(2, this.wrapping_add(0x287));
        c.sdr(2, this.wrapping_add(0x280));
        c.sdl(5, this.wrapping_add(0x28f));
        c.sdr(5, this.wrapping_add(0x288));
        c.sdl(6, this.wrapping_add(0x297));
        c.sdr(6, this.wrapping_add(0x290));
        c.sdl(7, this.wrapping_add(0x29f));
        c.sdr(7, this.wrapping_add(0x298));
        c.g[V0][0] = c.lw(body.wrapping_add(0x7c));
        c.sw(this.wrapping_add(0x2a0), c.g[V0][0]);
        // Scale the four halves of the sample by 0.5 (f3).
        c.lwc1(5, this.wrapping_add(0x280));
        c.lwc1(7, this.wrapping_add(0x284));
        c.lwc1(8, this.wrapping_add(0x288));
        c.mul_s(5, 5, 3);
        c.lwc1(6, this.wrapping_add(0x28c));
        c.mul_s(7, 7, 3);
        c.lwc1(4, this.wrapping_add(0x290));
        c.mul_s(8, 8, 3);
        c.lwc1(2, this.wrapping_add(0x294));
        c.mul_s(6, 6, 3);
        c.lwc1(0, this.wrapping_add(0x298));
        c.mul_s(4, 4, 3);
        c.lwc1(1, this.wrapping_add(0x29c));
        c.mul_s(2, 2, 3);
        c.mul_s(0, 0, 3);
        c.swc1(5, this.wrapping_add(0x280));
        c.mul_s(1, 1, 3);
        c.swc1(7, this.wrapping_add(0x284));
        c.swc1(8, this.wrapping_add(0x288));
        c.swc1(6, this.wrapping_add(0x28c));
        c.swc1(4, this.wrapping_add(0x290));
        c.swc1(2, this.wrapping_add(0x294));
        c.swc1(0, this.wrapping_add(0x298));
        c.swc1(1, this.wrapping_add(0x29c));
        c.lwc1(0, this.wrapping_add(0x2a0));
        c.g[A1][0] = c.lw(this.wrapping_add(0xd0));
        c.mul_s(0, 0, 3);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        c.swc1(0, this.wrapping_add(0x2a0));
        call_ptr(c, MAT4_TO_POS_QUAT, 0x0035_dc08)?;

        // Quaternion to matrix (VU0): the pose at sp+0x10 and the sample.
        c.lqc2(4, frame_sp.wrapping_add(0x20));
        c.lqc2(5, this.wrapping_add(0x2b0));
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
        c.sqc2(8, frame_sp.wrapping_add(0x30));
        c.g[A0][0] = sx(frame_sp.wrapping_add(0x30));
        c.lq(2, frame_sp.wrapping_add(0x30));
        c.g[A1][0] = c.g[S0][0];
        c.sq(2, frame_sp);
        c.lqc2(4, frame_sp.wrapping_add(0x20));
        c.lqc2(5, frame_sp);
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
        c.sqc2(8, frame_sp.wrapping_add(0x50));
        c.lq(2, frame_sp.wrapping_add(0x50));
        c.sq(2, frame_sp.wrapping_add(0x40));
        c.lqc2(3, frame_sp.wrapping_add(0x10));
        c.lqc2(4, frame_sp.wrapping_add(0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame_sp.wrapping_add(0x10));
        c.lq(2, frame_sp.wrapping_add(0x10));
        c.lq(3, frame_sp.wrapping_add(0x20));
        c.sq(2, this.wrapping_add(0x30));
        c.sq(3, this.wrapping_add(0x40));
        call_ptr(c, ROLLER_GET_POSE, 0x0035_dcac)?;

        // Pose matrix into the body's transform block at +0x240.
        c.g[V0][0] = sx(this.wrapping_add(0x240));
        c.lqc2(4, frame_sp.wrapping_add(0x40));
        c.lqc2(3, frame_sp.wrapping_add(0x30));
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
        let pose = c.g[V0][0] as u32;
        c.sqc2(3, pose.wrapping_add(0x30));
        c.sqc2(10, pose);
        c.sqc2(11, pose.wrapping_add(0x10));
        c.sqc2(12, pose.wrapping_add(0x20));

        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, ROLLER_UPDATE_INERTIA, 0x0035_dd3c)?;
        c.g[A0][0] = c.g[S5][0];
        call_ptr(c, ROLLER_UPDATE_MOMENTUM, 0x0035_dd44)?;

        // Default damping and limits from the globals, then the body init.
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x29fc));
        c.g[A3][0] = 0xffff_ffff_ffff_0000;
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x29f8));
        c.g[A3][0] = c.g[A3][0] | 0xffdf;
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x29f4));
        c.g[T0][0] = 0xffff_ffff_ffff_0000;
        c.swc1(0, this.wrapping_add(0x14));
        c.g[T0][0] = c.g[T0][0] | 0xfffd;
        c.swc1(1, this.wrapping_add(0x18));
        c.g[A1][0] = c.g[S6][0];
        c.swc1(2, this.wrapping_add(0x1c));
        c.g[A2][0] = c.g[S4][0];
        c.sw(this.wrapping_add(0x10), 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = c.lw(this.wrapping_add(0xd0));
        let owner_body = c.g[V1][0] as u32;
        c.g[V0][0] = c.lw(owner_body.wrapping_add(0x8));
        c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
        c.g[V0][0] = c.g[V0][0] | 0x40;
        c.sw(owner_body.wrapping_add(0x8), c.g[V0][0]);
        c.g[A3][0] = c.lw(this.wrapping_add(0xd0));
        let a3_body = c.g[A3][0] as u32;
        c.g[V0][0] = c.lw(a3_body.wrapping_add(0x8));
        c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
        c.g[V0][0] = c.g[V0][0] | 0x4;
        c.sw(a3_body.wrapping_add(0x8), c.g[V0][0]);
        c.g[V1][0] = c.lw(this.wrapping_add(0xd0));
        let v1_body = c.g[V1][0] as u32;
        c.g[V0][0] = c.lw(v1_body.wrapping_add(0x8));
        c.g[V0][0] = c.g[V0][0] | 0x1;
        c.sw(v1_body.wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, ROLLER_INIT_BODY, 0x0035_ddb8)?;

        c.g[V0][0] = c.g[S0][0];
        c.lq(17, frame_sp.wrapping_add(0xc0));
        c.lq(16, frame_sp.wrapping_add(0xd0));
        c.lq(18, frame_sp.wrapping_add(0xb0));
        c.lq(19, frame_sp.wrapping_add(0xa0));
        c.lq(20, frame_sp.wrapping_add(0x90));
        c.lq(21, frame_sp.wrapping_add(0x80));
        c.lq(22, frame_sp.wrapping_add(0x70));
        c.g[RA][0] = c.ld(frame_sp.wrapping_add(0x60));
        c.g[SP][0] = sx(frame_sp.wrapping_add(0xe0));
        ret(c);
        Ok(())
    })
}

/// `cRollerModifier::cRollerModifier(cReplayFramePtr* s)` (0x35df70): the
/// replay-frame constructor. It builds the same member body and bounding tree
/// as [`roller_modifier_ctor`], loads the body from the replay frame
/// (`RollerModifier_LoadBody`), copies the sample and the matrix block from
/// the frame's record (`*s` at `+0x50`, `+0x48`), recomputes the pose and the
/// inertia, and returns the object in `v0`.
pub fn roller_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 156;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        let frame_sp = c.g[SP][0] as u32;
        c.g[V0][0] = 0x490000;
        c.sq(16, frame_sp.wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x13f8));
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.sq(19, frame_sp.wrapping_add(0x40));
        c.sq(17, frame_sp.wrapping_add(0x60));
        c.g[S3][0] = sx(this.wrapping_add(0x30));
        c.sq(18, frame_sp.wrapping_add(0x50));
        c.g[S1][0] = c.g[A1][0];
        c.sd(frame_sp.wrapping_add(0x30), c.g[RA][0]);
        c.g[A0][0] = c.g[S3][0];
        c.sw(this, c.g[V0][0]);
        c.g[S2][0] = sx(this.wrapping_add(0xe0));
        call_ptr(c, ROLLER_MEMBER_CTOR_STUB, 0x0035_dfac)?;

        c.g[V1][0] = 0x490000;
        c.g[A0][0] = c.g[S2][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xf80));
        c.sw(this, c.g[V1][0]);
        call_ptr(c, SPHERE_TREE_CTOR, 0x0035_dfc0)?;

        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, ROLLER_LOAD_BODY, 0x0035_dfcc)?;

        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        c.g[A2][0] = 0x28;
        c.g[A1][0] = sx(this.wrapping_add(0x4));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let t = c.g[A3][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0035_dfe8)?;

        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0035_dffc)?;

        c.sw(this.wrapping_add(0xd0), c.g[V0][0]);
        call_ptr(c, 0x002d_1bd8, 0x0035_e004)?; // World_GetInstance
        c.g[A1][0] = c.lw(this.wrapping_add(0xd0));
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x003a_6cd8, 0x0035_e010)?; // World_GetObjectSectionData_2
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x003a_6ce8, 0x0035_e01c)?; // World_GetSphereTreeAt
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, ROLLER_COPY_MATRIX_BLOCK, 0x0035_e028)?;

        c.g[V1][0] = c.lw(this.wrapping_add(0x178));
        c.g[A1][0] = c.g[S0][0];
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 2);
        c.g[A0][0] = sx(frame_sp.wrapping_add(0x10));
        let body = c.g[V1][0] as u32;
        c.lwc1(0, body.wrapping_add(0x2c));
        c.swc1(0, frame_sp);
        c.lwc1(1, body.wrapping_add(0x30));
        c.swc1(1, frame_sp.wrapping_add(0x4));
        c.lwc1(0, body.wrapping_add(0x34));
        c.sw(frame_sp.wrapping_add(0xc), 0);
        c.swc1(0, frame_sp.wrapping_add(0x8));
        c.lq(2, frame_sp);
        c.sq(2, this.wrapping_add(0x2b0));
        c.ldl(2, body.wrapping_add(0x63));
        c.ldr(2, body.wrapping_add(0x5c));
        c.ldl(6, body.wrapping_add(0x6b));
        c.ldr(6, body.wrapping_add(0x64));
        c.ldl(7, body.wrapping_add(0x73));
        c.ldr(7, body.wrapping_add(0x6c));
        c.ldl(8, body.wrapping_add(0x7b));
        c.ldr(8, body.wrapping_add(0x74));
        c.sdl(2, this.wrapping_add(0x287));
        c.sdr(2, this.wrapping_add(0x280));
        c.sdl(6, this.wrapping_add(0x28f));
        c.sdr(6, this.wrapping_add(0x288));
        c.sdl(7, this.wrapping_add(0x297));
        c.sdr(7, this.wrapping_add(0x290));
        c.sdl(8, this.wrapping_add(0x29f));
        c.sdr(8, this.wrapping_add(0x298));
        c.g[V0][0] = c.lw(body.wrapping_add(0x7c));
        c.sw(this.wrapping_add(0x2a0), c.g[V0][0]);
        // Scale the sample halves by 0.5 (f2).
        c.lwc1(6, this.wrapping_add(0x280));
        c.lwc1(8, this.wrapping_add(0x284));
        c.lwc1(9, this.wrapping_add(0x288));
        c.mul_s(6, 6, 2);
        c.lwc1(7, this.wrapping_add(0x28c));
        c.mul_s(8, 8, 2);
        c.lwc1(5, this.wrapping_add(0x290));
        c.mul_s(9, 9, 2);
        c.lwc1(4, this.wrapping_add(0x294));
        c.mul_s(7, 7, 2);
        c.lwc1(3, this.wrapping_add(0x298));
        c.mul_s(5, 5, 2);
        c.lwc1(1, this.wrapping_add(0x29c));
        c.mul_s(4, 4, 2);
        c.lwc1(0, this.wrapping_add(0x2a0));
        c.mul_s(3, 3, 2);
        c.mul_s(1, 1, 2);
        c.swc1(6, this.wrapping_add(0x280));
        c.mul_s(0, 0, 2);
        c.swc1(8, this.wrapping_add(0x284));
        c.swc1(9, this.wrapping_add(0x288));
        c.swc1(7, this.wrapping_add(0x28c));
        c.swc1(5, this.wrapping_add(0x290));
        c.swc1(4, this.wrapping_add(0x294));
        c.swc1(3, this.wrapping_add(0x298));
        c.swc1(1, this.wrapping_add(0x29c));
        c.swc1(0, this.wrapping_add(0x2a0));
        call_ptr(c, ROLLER_GET_POSE, 0x0035_e118)?;

        c.g[A1][0] = sx(this.wrapping_add(0x240));
        c.lqc2(4, frame_sp.wrapping_add(0x20));
        c.lqc2(3, frame_sp.wrapping_add(0x10));
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
        let pose = c.g[A1][0] as u32;
        c.sqc2(3, pose.wrapping_add(0x30));
        c.sqc2(10, pose);
        c.sqc2(11, pose.wrapping_add(0x10));
        c.sqc2(12, pose.wrapping_add(0x20));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0032_c648, 0x0035_e1b0)?; // SphereTree_SetTransform
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, ROLLER_UPDATE_INERTIA, 0x0035_e1b8)?;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, ROLLER_UPDATE_MOMENTUM, 0x0035_e1c0)?;

        c.g[V0][0] = c.g[S0][0];
        c.lq(17, frame_sp.wrapping_add(0x60));
        c.lq(16, frame_sp.wrapping_add(0x70));
        c.lq(18, frame_sp.wrapping_add(0x50));
        c.lq(19, frame_sp.wrapping_add(0x40));
        c.g[RA][0] = c.ld(frame_sp.wrapping_add(0x30));
        c.g[SP][0] = sx(frame_sp.wrapping_add(0x80));
        ret(c);
        Ok(())
    })
}

/// `cRollerModifier::collide(void)` (0x35edc8): one collision step of a roller
/// against the world. It builds a temporary bounding tree at the stack, finds
/// the closest contact of the roller's nash (`World_GetNash`) within the
/// radius, moves the roller out of the contact (`RollerModifier_Translate`,
/// `RollerModifier_ResolveContact`) and updates the contact's depth and
/// timer, then destroys the temporary tree.
pub fn roller_modifier_collide(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1a0));
        let frame_sp = c.g[SP][0] as u32;
        c.g[A2][0] = 0x1;
        c.sq(18, frame_sp.wrapping_add(0x160));
        c.sq(16, frame_sp.wrapping_add(0x180));
        c.g[S2][0] = c.g[A0][0];
        let this = c.g[S2][0] as u32;
        c.sq(17, frame_sp.wrapping_add(0x170));
        c.g[S0][0] = sx(frame_sp.wrapping_add(0x80));
        c.g[S1][0] = sx(this.wrapping_add(0xe0));
        c.sq(19, frame_sp.wrapping_add(0x150));
        c.swc1(21, frame_sp.wrapping_add(0x198));
        c.g[A0][0] = c.g[S0][0];
        c.swc1(20, frame_sp.wrapping_add(0x190));
        c.g[A1][0] = c.g[S1][0];
        c.sd(frame_sp.wrapping_add(0x140), c.g[RA][0]);
        c.g[S3][0] = c.g[S0][0];
        call_ptr(c, 0x0033_03f0, 0x0035_ee08)?; // CollSphereTree_ctor

        c.mtc1(0, 21);
        call_ptr(c, 0x002d_1be0, 0x0035_ee14)?; // World_GetNash
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = sx(this.wrapping_add(0x2c0));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x0033_6850, 0x0035_ee28)?; // Nash_FindClosestContact
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0032_c5a8, 0x0035_ee34)?; // SphereTree_GetHalfLeafRadius
        c.sub_s(20, 20, 0);
        c.c_lt_s(21, 20);
        let t = !c.fcc();
        c.g[A0][0] = c.g[S2][0];
        'b088: {
            'b084: {
                if t {
                    break 'b084;
                }
                c.cycles += 110;
                c.g[AT][0] = 0x4000_0000;
                c.mtc1(1, 0);
                c.g[A1][0] = sx(frame_sp.wrapping_add(0xf0));
                c.add_s(0, 20, 0);
                c.mfc1(3, 0);
                c.lqc2(4, frame_sp.wrapping_add(0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame_sp.wrapping_add(0x100));
                c.lq(2, frame_sp.wrapping_add(0x100));
                c.sq(2, frame_sp.wrapping_add(0xf0));
                call_ptr(c, 0x0035_ed90, 0x0035_ee78)?; // RollerModifier_Translate
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x29d4));
                c.g[A0][0] = sx(this.wrapping_add(0x30));
                c.mfc1(3, 10);
                c.lqc2(4, this.wrapping_add(0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame_sp.wrapping_add(0x110));
                c.lq(2, frame_sp.wrapping_add(0x110));
                c.lwc1(4, this.wrapping_add(0x80));
                c.sq(2, this.wrapping_add(0x60));
                c.lwc1(9, this.wrapping_add(0x8c));
                c.lwc1(1, this.wrapping_add(0x60));
                c.lwc1(5, this.wrapping_add(0x64));
                c.lwc1(2, this.wrapping_add(0x84));
                c.mul_s(4, 4, 1);
                c.lwc1(6, this.wrapping_add(0x90));
                c.mul_s(9, 9, 5);
                c.lwc1(0, this.wrapping_add(0x88));
                c.mul_s(2, 2, 1);
                c.lwc1(3, this.wrapping_add(0x94));
                c.mul_s(6, 6, 5);
                c.mul_s(0, 0, 1);
                c.lwc1(7, this.wrapping_add(0x68));
                c.mul_s(3, 3, 5);
                c.lwc1(8, this.wrapping_add(0x98));
                c.lwc1(5, this.wrapping_add(0x9c));
                c.add_s(2, 2, 6);
                c.lwc1(1, this.wrapping_add(0xa0));
                c.add_s(4, 4, 9);
                c.add_s(0, 0, 3);
                c.lwc1(6, this.wrapping_add(0x6c));
                c.mul_s(1, 1, 7);
                c.sq(2, frame_sp.wrapping_add(0x100));
                c.mul_s(8, 8, 7);
                c.swc1(6, frame_sp.wrapping_add(0x13c));
                c.mul_s(5, 5, 7);
                c.add_s(0, 0, 1);
                c.add_s(4, 4, 8);
                c.add_s(2, 2, 5);
                c.swc1(0, frame_sp.wrapping_add(0x138));
                c.swc1(4, frame_sp.wrapping_add(0x130));
                c.swc1(2, frame_sp.wrapping_add(0x134));
                c.lq(2, frame_sp.wrapping_add(0x130));
                c.sq(2, this.wrapping_add(0xc0));
                c.sq(2, frame_sp.wrapping_add(0x120));
                c.mfc1(3, 10);
                c.lqc2(4, this.wrapping_add(0xb0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame_sp.wrapping_add(0x110));
                c.lq(2, frame_sp.wrapping_add(0x110));
                c.sq(2, frame_sp.wrapping_add(0x100));
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x40));
                c.mfc1(2, 0);
                c.qmtc2(2, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, frame_sp.wrapping_add(0x100));
                c.vu0().waitq();
                c.vu0().mul(0b1111, 4, 4, Src::Q);
                c.sqc2(4, frame_sp.wrapping_add(0x120));
                c.lq(3, frame_sp.wrapping_add(0x120));
                c.lq(2, frame_sp.wrapping_add(0x100));
                c.sq(3, this.wrapping_add(0x50));
                c.sq(2, this.wrapping_add(0xb0));
                c.sq(3, frame_sp.wrapping_add(0x110));
                c.lqc2(3, frame_sp);
                c.lqc2(4, this.wrapping_add(0x30));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame_sp.wrapping_add(0x130));
                c.lq(2, frame_sp.wrapping_add(0x130));
                c.sq(2, frame_sp.wrapping_add(0x120));
                c.lqc2(4, this.wrapping_add(0xc0));
                c.lqc2(5, frame_sp.wrapping_add(0x120));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, frame_sp.wrapping_add(0x130));
                c.lq(2, frame_sp.wrapping_add(0x130));
                c.sq(2, frame_sp.wrapping_add(0x110));
                c.lqc2(3, this.wrapping_add(0xb0));
                c.lqc2(4, frame_sp.wrapping_add(0x110));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame_sp.wrapping_add(0x130));
                c.lq(2, frame_sp.wrapping_add(0x130));
                c.sq(2, frame_sp.wrapping_add(0x100));
                c.lqc2(3, frame_sp.wrapping_add(0x100));
                c.lqc2(5, frame_sp.wrapping_add(0x10));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(3, 4);
                c.mtc1(3, 0);
                c.c_lt_s(0, 21);
                if !c.fcc() {
                    c.g[A0][0] = c.g[S3][0];
                    break 'b088;
                }
                'b044: {
                    c.cycles += 16;
                    c.lwc1(0, this.wrapping_add(0x1c));
                    c.lqc2(3, this.wrapping_add(0xb0));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(2, 22);
                    c.mtc1(2, 1);
                    c.lwc1(12, this.wrapping_add(0x14));
                    c.c_lt_s(1, 0);
                    let t = !c.fcc();
                    c.lwc1(13, this.wrapping_add(0x18));
                    if t {
                        break 'b044;
                    }
                    c.cycles += 1;
                    c.mov_s(12, 21);
                }
                c.cycles += 13;
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = sx(frame_sp.wrapping_add(0x10));
                call_ptr(c, 0x0035_d4a0, 0x0035_f050)?; // RollerModifier_ResolveContact
                c.lwc1(0, this.wrapping_add(0x28));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x29d0));
                c.lwc1(3, this.wrapping_add(0x20));
                c.g[AT][0] = 0x40a0_0000;
                c.mtc1(1, 1);
                c.add_s(0, 0, 2);
                c.c_lt_s(1, 3);
                let t = !c.fcc();
                c.swc1(0, this.wrapping_add(0x28));
                if t {
                    break 'b084;
                }
                c.cycles += 3;
                c.g[AT][0] = 0x4120_0000;
                c.mtc1(1, 0);
                c.swc1(0, this.wrapping_add(0x20));
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[S3][0];
        }
        c.cycles += 11;
        c.g[RA][0] = sx(0x0035_f090);
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0033_04e8, 0x0035_f090)?; // CollSphereTree_dtor
        c.lq(16, frame_sp.wrapping_add(0x180));
        c.lq(17, frame_sp.wrapping_add(0x170));
        c.lq(18, frame_sp.wrapping_add(0x160));
        c.lq(19, frame_sp.wrapping_add(0x150));
        c.g[RA][0] = c.ld(frame_sp.wrapping_add(0x140));
        c.lwc1(21, frame_sp.wrapping_add(0x198));
        c.lwc1(20, frame_sp.wrapping_add(0x190));
        c.g[SP][0] = sx(frame_sp.wrapping_add(0x1a0));
        ret(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, DATA, GAME_GP};

    #[test]
    fn roller_modifier_ctor_matches() {
        check(ee::s11::f_0035da70__generated, roller_modifier_ctor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x35da_70, 0);
            let this = DATA + 0x1000;
            let rdata = DATA + 0x2000;
            let ii = DATA + 0x3000;
            let ci = DATA + 0x3800;
            let body = DATA + 0x5000;
            let owner = DATA + 0x6000;
            let matrix = DATA + 0x7000;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(rdata);
            c.g[A2][0] = sx(ii);
            c.g[A3][0] = sx(ci);
            fill(c, &mut r, this, 0x80);
            fill(c, &mut r, rdata, 0x10);
            fill(c, &mut r, ii, 0x40);
            fill(c, &mut r, ci, 0x40);
            fill(c, &mut r, body, 0x40);
            fill(c, &mut r, owner, 0x40);
            fill(c, &mut r, matrix, 0x40);
            c.bus.write32(this + 0x178, body);
            c.bus.write32(ii + 0x50, owner);
            c.bus.write32(ii + 0x48, matrix);
            c.bus.write32(rdata + 4, r.edge_f32());
            c.bus.write32(rdata + 8, r.edge_f32());
            for g in [0x29fc, 0x29f8, 0x29f4] {
                c.bus.write32(GAME_GP - g, r.edge_f32());
            }
        });
    }

    #[test]
    fn roller_modifier_ctor_2_matches() {
        check(ee::s11::f_0035df70__generated, roller_modifier_ctor_2, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x35df_70, 0);
            let this = DATA + 0x1000;
            let frame = DATA + 0x3000;
            let body = DATA + 0x5000;
            let owner = DATA + 0x6000;
            let matrix = DATA + 0x7000;
            let inner = DATA + 0x8000;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(frame);
            fill(c, &mut r, this, 0x80);
            fill(c, &mut r, frame, 0x40);
            fill(c, &mut r, body, 0x40);
            fill(c, &mut r, owner, 0x40);
            fill(c, &mut r, matrix, 0x40);
            fill(c, &mut r, inner, 0x40);
            // The replay frame: a body pointer at +0 and a load-body record.
            c.bus.write32(frame, body);
            c.bus.write32(body + 0x10, owner);
            c.bus.write32(body + 0x1c, 0);
            c.bus.write32(this + 0x178, body);
            c.bus.write32(this + 0xd0, owner);
            c.bus.write32(GAME_GP.wrapping_sub(0x848), inner);
            for g in [0x29fc, 0x29f8, 0x29f4] {
                c.bus.write32(GAME_GP - g, r.edge_f32());
            }
        });
    }

    #[test]
    fn roller_modifier_collide_matches() {
        check(ee::s11::f_0035edc8__generated, roller_modifier_collide, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x35ed_c8, 0);
            let this = DATA + 0x1000;
            let world = DATA + 0x8000;
            c.g[A0][0] = sx(this);
            fill(c, &mut r, this, 0xc0);
            fill(c, &mut r, world, 0x40);
            for off in (0..0x300u32).step_by(4) {
                c.bus.write32(this + off, r.f32(4.0));
            }
            c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
            for g in [0x29d4, 0x29d0] {
                c.bus.write32(GAME_GP - g, r.edge_f32());
            }
        });
    }
}
