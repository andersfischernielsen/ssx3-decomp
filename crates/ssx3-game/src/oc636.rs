//! Staging module oc636 (wave 234, list oc636): hand-written replacements, each
//! with its differential test against the generated body. Moved into the
//! source-tree units by `tools/reorg_modules.py apply oc636`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0016_a510, subject_script_camera_algo_update, "SubjectScriptCameraAlgo_Update"),
];

/// The 32-bit value of GPR `r` (the generated code's `c.g[r][0] as u32`).
#[inline(always)]
fn camera_gpr(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `off(sp)` as an address (read from `sp` each time, as the generated code does).
#[inline(always)]
fn camera_frame_at(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// The address `GP - off` (the game's gp-relative data).
#[inline(always)]
fn camera_gp_addr(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// `lwc1 f<reg>, GP - off`: a float constant or global from the gp data.
#[inline(always)]
fn camera_gp_f32(c: &mut Cpu, reg: usize, off: u32) {
    let a = camera_gp_addr(c, off);
    c.lwc1(reg, a);
}

/// The chase-camera epilogue shared by the two mid chase updates: restores
/// s0 (`lq 16`), `ra`, f20 and f21, pops the 0xa0 frame and returns.
#[inline(always)]
fn mid_chase_epilogue(c: &mut Cpu) {
    let a80 = camera_frame_at(c, 0x80);
    c.lq(16, a80);
    let a70 = camera_frame_at(c, 0x70);
    c.g[RA][0] = c.ld(a70);
    let a98 = camera_frame_at(c, 0x98);
    c.lwc1(21, a98);
    let a90 = camera_frame_at(c, 0x90);
    c.lwc1(20, a90);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0xa0));
    c.pc = t;
}

/// `cMidRightChaseAlgorithm_update(self)` (0x168a70): the right mid chase
/// camera's per-frame update. It runs the chase pipeline in order on one
/// stack `cChaseSetup` (`info` at sp+0): setup, handplant blend, half pipe,
/// target, direction, follow distance, drop, FOV, handplant, late target,
/// air, half pipe, rail, view, slow speed and lateral shift. Each step gets
/// its tuning constants from the gp data; the callee-saved registers and the
/// scratch registers at each call are the generated body's.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn mid_right_chase_camera_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 120;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0xa0));
        camera_gp_f32(c, 19, 0x66bc);
        let a80 = camera_frame_at(c, 0x80);
        c.sq(16, a80);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        camera_gp_f32(c, 12, 0x66b8);
        camera_gp_f32(c, 13, 0x66b4);
        camera_gp_f32(c, 14, 0x66b0);
        camera_gp_f32(c, 15, 0x66ac);
        camera_gp_f32(c, 16, 0x66a8);
        camera_gp_f32(c, 17, 0x66a4);
        camera_gp_f32(c, 18, 0x66a0);
        let a70 = camera_frame_at(c, 0x70);
        c.sd(a70, c.g[RA][0]);
        let a98 = camera_frame_at(c, 0x98);
        c.swc1(21, a98);
        let a90 = camera_frame_at(c, 0x90);
        c.swc1(20, a90);
        // setupChaseData(self, &info, the eight tuning floats)
        call_ptr(c, 0x0016_2568, 0x0016_8ab0)?;
        c.mtc1(0, 20);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2998, 0x0016_8ac0)?;
        camera_gp_f32(c, 21, 0x669c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2a20, 0x0016_8ad0)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b80, 0x0016_8adc)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2c78, 0x0016_8ae8)?;
        camera_gp_f32(c, 17, 0x6698);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 18, 0x6694);
        camera_gp_f32(c, 12, 0x6690);
        camera_gp_f32(c, 13, 0x668c);
        camera_gp_f32(c, 14, 0x6688);
        camera_gp_f32(c, 15, 0x6684);
        camera_gp_f32(c, 16, 0x6680);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3010, 0x0016_8b10)?;
        camera_gp_f32(c, 16, 0x667c);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 12, 0x6678);
        camera_gp_f32(c, 13, 0x6674);
        camera_gp_f32(c, 14, 0x6670);
        camera_gp_f32(c, 15, 0x666c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3270, 0x0016_8b30)?;
        camera_gp_f32(c, 13, 0x6668);
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 14);
        c.g[A0][0] = c.g[S0][0];
        camera_gp_f32(c, 15, 0x6664);
        c.mov_s(12, 20);
        call_ptr(c, 0x0016_33b0, 0x0016_8b50)?;
        camera_gp_f32(c, 18, 0x6660);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 19, 0x665c);
        c.mov_s(17, 21);
        camera_gp_f32(c, 12, 0x6658);
        c.mov_s(16, 20);
        camera_gp_f32(c, 13, 0x6654);
        camera_gp_f32(c, 14, 0x6650);
        camera_gp_f32(c, 15, 0x664c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3450, 0x0016_8b7c)?;
        camera_gp_f32(c, 16, 0x6648);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 12, 0x6644);
        camera_gp_f32(c, 13, 0x6640);
        camera_gp_f32(c, 14, 0x663c);
        camera_gp_f32(c, 15, 0x6638);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b90, 0x0016_8b9c)?;
        camera_gp_f32(c, 14, 0x6634);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 15, 0x6630);
        camera_gp_f32(c, 13, 0x662c);
        camera_gp_f32(c, 12, 0x6628);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_35f8, 0x0016_8bb8)?;
        camera_gp_f32(c, 12, 0x6624);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_4878, 0x0016_8bc8)?;
        camera_gp_f32(c, 12, 0x6620);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 13, 0x661c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_46a0, 0x0016_8bdc)?;
        c.g[AT][0] = 0x4416_0000;
        c.mtc1(1, 12);
        c.mov_s(15, 21);
        c.g[AT][0] = 0x4396_0000;
        c.mtc1(1, 13);
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(1, 14);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_41c0, 0x0016_8c04)?;
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(1, 13);
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x4170_0000;
        c.mtc1(1, 12);
        c.g[A1][0] = c.g[SP][0];
        c.mov_s(14, 13);
        call_ptr(c, 0x0016_43a8, 0x0016_8c24)?;
        camera_gp_f32(c, 12, 0x6618);
        c.g[A0][0] = c.g[S0][0];
        camera_gp_f32(c, 13, 0x6614);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0016_3158, 0x0016_8c38)?;
        mid_chase_epilogue(c);
        Ok(())
    })
}

/// `cMidLeftChaseAlgorithm_update(self)` (0x168d80): the left mid chase
/// camera's per-frame update. The same chase pipeline as the right one
/// ([`mid_right_chase_camera_update`]), with the left camera's gp constants
/// and return addresses.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn mid_left_chase_camera_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 120;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0xa0));
        camera_gp_f32(c, 19, 0x65f8);
        let a80 = camera_frame_at(c, 0x80);
        c.sq(16, a80);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        camera_gp_f32(c, 12, 0x65f4);
        camera_gp_f32(c, 13, 0x65f0);
        camera_gp_f32(c, 14, 0x65ec);
        camera_gp_f32(c, 15, 0x65e8);
        camera_gp_f32(c, 16, 0x65e4);
        camera_gp_f32(c, 17, 0x65e0);
        camera_gp_f32(c, 18, 0x65dc);
        let a70 = camera_frame_at(c, 0x70);
        c.sd(a70, c.g[RA][0]);
        let a98 = camera_frame_at(c, 0x98);
        c.swc1(21, a98);
        let a90 = camera_frame_at(c, 0x90);
        c.swc1(20, a90);
        // setupChaseData(self, &info, the eight tuning floats)
        call_ptr(c, 0x0016_2568, 0x0016_8dc0)?;
        c.mtc1(0, 20);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2998, 0x0016_8dd0)?;
        camera_gp_f32(c, 21, 0x65d8);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2a20, 0x0016_8de0)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b80, 0x0016_8dec)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2c78, 0x0016_8df8)?;
        camera_gp_f32(c, 17, 0x65d4);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 18, 0x65d0);
        camera_gp_f32(c, 12, 0x65cc);
        camera_gp_f32(c, 13, 0x65c8);
        camera_gp_f32(c, 14, 0x65c4);
        camera_gp_f32(c, 15, 0x65c0);
        camera_gp_f32(c, 16, 0x65bc);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3010, 0x0016_8e20)?;
        camera_gp_f32(c, 16, 0x65b8);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 12, 0x65b4);
        camera_gp_f32(c, 13, 0x65b0);
        camera_gp_f32(c, 14, 0x65ac);
        camera_gp_f32(c, 15, 0x65a8);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3270, 0x0016_8e40)?;
        camera_gp_f32(c, 13, 0x65a4);
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 14);
        c.g[A0][0] = c.g[S0][0];
        camera_gp_f32(c, 15, 0x65a0);
        c.mov_s(12, 20);
        call_ptr(c, 0x0016_33b0, 0x0016_8e60)?;
        camera_gp_f32(c, 18, 0x659c);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 19, 0x6598);
        c.mov_s(17, 21);
        camera_gp_f32(c, 12, 0x6594);
        c.mov_s(16, 20);
        camera_gp_f32(c, 13, 0x6590);
        camera_gp_f32(c, 14, 0x658c);
        camera_gp_f32(c, 15, 0x6588);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3450, 0x0016_8e8c)?;
        camera_gp_f32(c, 16, 0x6584);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 12, 0x6580);
        camera_gp_f32(c, 13, 0x657c);
        camera_gp_f32(c, 14, 0x6578);
        camera_gp_f32(c, 15, 0x6574);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b90, 0x0016_8eac)?;
        camera_gp_f32(c, 14, 0x6570);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 15, 0x656c);
        camera_gp_f32(c, 13, 0x6568);
        camera_gp_f32(c, 12, 0x6564);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_35f8, 0x0016_8ec8)?;
        camera_gp_f32(c, 12, 0x6560);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_4878, 0x0016_8ed8)?;
        camera_gp_f32(c, 12, 0x655c);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 13, 0x6558);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_46a0, 0x0016_8eec)?;
        c.g[AT][0] = 0x4416_0000;
        c.mtc1(1, 12);
        c.mov_s(15, 21);
        c.g[AT][0] = 0x4396_0000;
        c.mtc1(1, 13);
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(1, 14);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_41c0, 0x0016_8f14)?;
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(1, 13);
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x4170_0000;
        c.mtc1(1, 12);
        c.g[A1][0] = c.g[SP][0];
        c.mov_s(14, 13);
        call_ptr(c, 0x0016_43a8, 0x0016_8f34)?;
        camera_gp_f32(c, 12, 0x6554);
        c.g[A0][0] = c.g[S0][0];
        camera_gp_f32(c, 13, 0x6550);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0016_3158, 0x0016_8f48)?;
        mid_chase_epilogue(c);
        Ok(())
    })
}

/// `cScriptTargetAlgorithm_update(algorithm)` (0x16a510): the script camera
/// target's per-frame update. It reads the camera subject's header and frame
/// (`GetSSXCameraSubjectHeader` / `...Frame`) and the subject rider (or the
/// first AI rider when the engine finds none), writes Near/Far/View/Roll and
/// the position, transforms the position by the base transform, takes the
/// rider's look-at target from its virtual method (vtable slot 5) and its
/// lead vectors, and derives the heading and pitch from the look vector.
/// Then it updates the positional and roll jitterers.
///
/// The vector maths runs on VU0 (`lqc2`/`sqc2`/macro ops) exactly as the
/// generated code issues it, through the 128-bit stack slots it uses.
pub fn subject_script_camera_algo_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x100));
        let a_c0 = camera_frame_at(c, 0xc0);
        c.sq(17, a_c0);
        let a_b0 = camera_frame_at(c, 0xb0);
        c.sq(18, a_b0);
        c.g[S1][0] = c.g[A0][0];
        let a_a0 = camera_frame_at(c, 0xa0);
        c.sq(19, a_a0);
        let a_90 = camera_frame_at(c, 0x90);
        c.sq(20, a_90);
        let a_d0 = camera_frame_at(c, 0xd0);
        c.sq(16, a_d0);
        let a_80 = camera_frame_at(c, 0x80);
        c.sd(a_80, c.g[RA][0]);
        let a_f0 = camera_frame_at(c, 0xf0);
        c.swc1(22, a_f0);
        let a_e8 = camera_frame_at(c, 0xe8);
        c.swc1(21, a_e8);
        let a_e0 = camera_frame_at(c, 0xe0);
        c.swc1(20, a_e0);

        // cam = algorithm->ScriptObj->mpCameraEntity->mpEntity; header = subject header
        c.g[V0][0] = Addr(camera_gpr(c, S1).wrapping_add(0x14)).lw(c);
        c.g[V1][0] = Addr(camera_gpr(c, V0).wrapping_add(0x14)).lw(c);
        c.g[S0][0] = Addr(camera_gpr(c, V1).wrapping_add(0x8)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0027_c4b8, 0x0016_a550)?;
        c.g[S2][0] = c.g[V0][0];
        // rider = cSSXScriptEngine_GetSubjectRider(ScriptEngine, header[0x22], cam + 8)
        c.g[A0][0] = Addr(camera_gp_addr(c, 0x84c)).lw(c);
        c.g[V0][0] = Addr(camera_gpr(c, S1).wrapping_add(0x14)).lw(c);
        c.g[A1][0] = Addr(camera_gpr(c, S2).wrapping_add(0x22)).lb(c);
        c.g[A2][0] = Addr(camera_gpr(c, S0).wrapping_add(0x8)).lw(c);
        c.g[S4][0] = sx(camera_gpr(c, V0).wrapping_add(0x30));
        call_ptr(c, 0x0027_9f18, 0x0016_a56c)?;
        c.g[S3][0] = c.g[V0][0];
        let rider_found = c.g[S3][0] != 0;
        c.lwc1(1, camera_gpr(c, S2));
        if !rider_found {
            c.cycles += 4;
            c.g[V0][0] = Addr(camera_gp_addr(c, 0x848)).lw(c);
            c.g[V1][0] = Addr(camera_gpr(c, V0).wrapping_add(0x84)).lw(c);
            c.g[A0][0] = Addr(camera_gpr(c, V1).wrapping_add(0xc)).lw(c);
            c.g[S3][0] = Addr(camera_gpr(c, A0).wrapping_add(0x28)).lw(c);
        }

        'b_16a77c: {
            'b_16a774: {
                'b_16a738: {
                    c.cycles += 97;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(camera_gpr(c, S1).wrapping_add(0x4c));
                    c.mtc1(0, 22);
                    c.swc1(1, camera_gpr(c, S1).wrapping_add(0x4));
                    c.lwc1(0, camera_gpr(c, S2).wrapping_add(0x4));
                    c.swc1(0, camera_gpr(c, S1).wrapping_add(0x8));
                    // GetSSXCameraSubjectFrame(cam, &self->FrameInfo)
                    call_ptr(c, 0x0027_c510, 0x0016_a5a4)?;
                    c.g[V1][0] = Addr(camera_gpr(c, S3).wrapping_add(0x780)).lw(c);
                    c.g[A1][0] = sx(camera_gpr(c, S3).wrapping_add(0x6c0));
                    c.lwc1(0, camera_gpr(c, S1).wrapping_add(0x64));
                    camera_gp_f32(c, 2, 0x651c);
                    let a_rot = camera_gpr(c, V1).wrapping_add(0x140);
                    c.lq(2, a_rot);
                    c.mul_s(0, 0, 2);
                    c.g[A0][0] = Addr(camera_gpr(c, S1).wrapping_add(0x14)).lw(c);
                    c.lwc1(1, camera_gpr(c, S1).wrapping_add(0x60));
                    let a_sp0 = camera_frame_at(c, 0x0);
                    c.sq(2, a_sp0);
                    c.swc1(0, camera_gpr(c, S1));
                    c.mul_s(1, 1, 2);
                    c.lwc1(5, camera_gpr(c, S1).wrapping_add(0x3c));
                    c.lwc1(0, camera_gpr(c, A0).wrapping_add(0x70));
                    c.lwc1(2, camera_gpr(c, S1).wrapping_add(0x4c));
                    c.lwc1(3, camera_gpr(c, S1).wrapping_add(0x50));
                    c.add_s(1, 1, 0);
                    c.lwc1(4, camera_gpr(c, S1).wrapping_add(0x54));
                    let a10 = camera_frame_at(c, 0x10);
                    c.swc1(2, a10);
                    let a14 = camera_frame_at(c, 0x14);
                    c.swc1(3, a14);
                    let a18 = camera_frame_at(c, 0x18);
                    c.swc1(4, a18);
                    let a1c = camera_frame_at(c, 0x1c);
                    c.swc1(5, a1c);
                    c.swc1(1, camera_gpr(c, S1).wrapping_add(0x40));
                    c.lq(2, camera_frame_at(c, 0x10));
                    c.sq(2, camera_gpr(c, S1).wrapping_add(0x30));
                    // position = baseTransform * position (VU0)
                    c.lqc2(8, camera_gpr(c, S1).wrapping_add(0x30));
                    c.lqc2(4, camera_gpr(c, S4));
                    c.lqc2(5, camera_gpr(c, S4).wrapping_add(0x10));
                    c.lqc2(6, camera_gpr(c, S4).wrapping_add(0x20));
                    c.lqc2(7, camera_gpr(c, S4).wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    let a20 = camera_frame_at(c, 0x20);
                    c.sqc2(12, a20);
                    c.lq(2, camera_frame_at(c, 0x20));
                    c.sq(2, camera_gpr(c, S1).wrapping_add(0x30));
                    c.sq(2, camera_frame_at(c, 0x10));
                    // target = vt[5](obj) + lead terms (vthis(obj, vt[5]) call)
                    c.g[V1][0] = Addr(camera_gpr(c, S3).wrapping_add(0x6c0)).lw(c);
                    c.g[A0][0] = Addr(camera_gpr(c, V1).wrapping_add(0x28)).lh(c);
                    c.g[V0][0] = Addr(camera_gpr(c, V1).wrapping_add(0x2c)).lw(c);
                    let vfn = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(camera_gpr(c, A1).wrapping_add(camera_gpr(c, A0)));
                    call_ptr(c, vfn, 0x0016_a64c)?;
                    c.lq(3, camera_gpr(c, S3).wrapping_add(0x1b0));
                    c.lwc1(0, camera_gpr(c, S1).wrapping_add(0x58));
                    c.lwc1(1, camera_frame_at(c, 0x4));
                    let a40 = camera_frame_at(c, 0x40);
                    c.sq(3, a40);
                    c.mul_s(0, 0, 1);
                    c.mfc1(4, 0);
                    c.lqc2(4, camera_frame_at(c, 0x40));
                    c.qmtc2(4, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    let a50 = camera_frame_at(c, 0x50);
                    c.sqc2(5, a50);
                    c.lq(3, camera_frame_at(c, 0x50));
                    c.sq(3, camera_frame_at(c, 0x30));
                    c.lqc2(3, camera_gpr(c, V0));
                    c.lqc2(4, camera_frame_at(c, 0x30));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, camera_frame_at(c, 0x50));
                    c.lq(2, camera_frame_at(c, 0x50));
                    c.lwc1(0, camera_gpr(c, S1).wrapping_add(0x5c));
                    c.sq(2, camera_frame_at(c, 0x20));
                    c.lwc1(1, camera_frame_at(c, 0x0));
                    c.lq(2, camera_gpr(c, S3).wrapping_add(0x1c0));
                    c.mul_s(0, 0, 1);
                    let a60 = camera_frame_at(c, 0x60);
                    c.sq(2, a60);
                    c.mfc1(3, 0);
                    c.lqc2(4, camera_frame_at(c, 0x60));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, camera_frame_at(c, 0x70));
                    c.lq(2, camera_frame_at(c, 0x70));
                    c.sq(2, camera_frame_at(c, 0x50));
                    c.lqc2(3, camera_frame_at(c, 0x20));
                    c.lqc2(4, camera_frame_at(c, 0x50));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, camera_frame_at(c, 0x70));
                    c.lq(2, camera_frame_at(c, 0x70));
                    c.sq(2, camera_gpr(c, S1).wrapping_add(0x20));
                    c.sq(2, camera_frame_at(c, 0x10));
                    // look = target - position
                    c.lqc2(3, camera_gpr(c, S1).wrapping_add(0x20));
                    c.lqc2(4, camera_gpr(c, S1).wrapping_add(0x30));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, camera_frame_at(c, 0x20));
                    c.lq(2, camera_frame_at(c, 0x20));
                    c.sq(2, camera_frame_at(c, 0x10));
                    c.lwc1(20, camera_frame_at(c, 0x10));
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    c.lwc1(21, camera_frame_at(c, 0x14));
                    if t {
                        break 'b_16a738;
                    }
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let t = c.fcc();
                    c.mov_s(12, 21);
                    if t {
                        break 'b_16a774;
                    }
                    c.cycles += 4;
                    c.c_le_s(22, 21);
                    camera_gp_f32(c, 12, 0x6518);
                    let t = !c.fcc();
                    c.lwc1(0, camera_frame_at(c, 0x10));
                    if t {
                        break 'b_16a774;
                    }
                    c.cycles += 3;
                    camera_gp_f32(c, 12, 0x6514);
                    c.lwc1(1, camera_frame_at(c, 0x14));
                    break 'b_16a77c;
                }
                // heading = atan2(look.y, look.x) with the quadrant fix-ups
                c.cycles += 7;
                c.div_s(12, 21, 20);
                call_ptr(c, 0x0031_c228, 0x0016_a744)?;
                c.c_lt_s(20, 22);
                let t = !c.fcc();
                c.mov_s(12, 0);
                if t {
                    break 'b_16a774;
                }
                'b_16a76c: {
                    c.cycles += 3;
                    c.c_lt_s(22, 21);
                    let t = !c.fcc();
                    if t {
                        break 'b_16a76c;
                    }
                    c.cycles += 3;
                    camera_gp_f32(c, 0, 0x6510);
                    c.add_s(12, 12, 0);
                    break 'b_16a774;
                }
                c.cycles += 2;
                camera_gp_f32(c, 0, 0x650c);
                c.sub_s(12, 12, 0);
            }
            c.cycles += 2;
            c.lwc1(0, camera_frame_at(c, 0x10));
            c.lwc1(1, camera_frame_at(c, 0x14));
        }

        'b_16a820: {
            'b_16a818: {
                'b_16a7d8: {
                    c.cycles += 12;
                    c.mul_s(2, 0, 0);
                    c.swc1(12, camera_gpr(c, S1).wrapping_add(0x44));
                    c.mul_s(3, 1, 1);
                    c.swc1(0, camera_frame_at(c, 0x20));
                    c.mtc1(0, 22);
                    c.swc1(1, camera_frame_at(c, 0x24));
                    c.add_s(2, 2, 3);
                    c.lwc1(21, camera_frame_at(c, 0x18));
                    c.sqrt_s(20, 2);
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    if t {
                        break 'b_16a7d8;
                    }
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let t = c.fcc();
                    c.mov_s(0, 21);
                    if t {
                        break 'b_16a818;
                    }
                    c.cycles += 4;
                    c.c_le_s(22, 21);
                    camera_gp_f32(c, 0, 0x6508);
                    let t = !c.fcc();
                    c.g[S0][0] = sx(camera_gpr(c, S1).wrapping_add(0x70));
                    if t {
                        break 'b_16a818;
                    }
                    c.cycles += 3;
                    camera_gp_f32(c, 0, 0x6504);
                    c.lwc1(12, camera_gpr(c, S1).wrapping_add(0x68));
                    break 'b_16a820;
                }
                // pitch = atan2(look.z, horizontal length)
                'b_16a814: {
                    c.cycles += 7;
                    c.div_s(12, 21, 20);
                    call_ptr(c, 0x0031_c228, 0x0016_a7e4)?;
                    c.c_lt_s(20, 22);
                    let t = !c.fcc();
                    c.mov_s(12, 0);
                    if t {
                        break 'b_16a814;
                    }
                    'b_16a80c: {
                        c.cycles += 3;
                        c.c_lt_s(22, 21);
                        let t = !c.fcc();
                        if t {
                            break 'b_16a80c;
                        }
                        c.cycles += 3;
                        camera_gp_f32(c, 0, 0x6500);
                        c.add_s(12, 12, 0);
                        break 'b_16a814;
                    }
                    c.cycles += 2;
                    camera_gp_f32(c, 0, 0x64fc);
                    c.sub_s(12, 12, 0);
                }
                c.cycles += 1;
                c.mov_s(0, 12);
            }
            c.cycles += 2;
            c.g[S0][0] = sx(camera_gpr(c, S1).wrapping_add(0x70));
            c.lwc1(12, camera_gpr(c, S1).wrapping_add(0x68));
        }

        c.cycles += 17;
        c.swc1(0, camera_gpr(c, S1).wrapping_add(0x48));
        c.g[A0][0] = c.g[S0][0];
        // cCameraJitterer_updatePositionalJitter(&Jitterer, jitterFocus)
        call_ptr(c, 0x0016_0028, 0x0016_a82c)?;
        c.lwc1(12, camera_gpr(c, S1).wrapping_add(0x6c));
        c.g[A0][0] = c.g[S0][0];
        // cCameraJitterer_updateRollJitter(&Jitterer, jitterRoll)
        call_ptr(c, 0x0016_0130, 0x0016_a838)?;
        c.lq(16, camera_frame_at(c, 0xd0));
        c.lq(17, camera_frame_at(c, 0xc0));
        c.lq(18, camera_frame_at(c, 0xb0));
        c.lq(19, camera_frame_at(c, 0xa0));
        c.lq(20, camera_frame_at(c, 0x90));
        c.g[RA][0] = c.ld(camera_frame_at(c, 0x80));
        c.lwc1(22, camera_frame_at(c, 0xf0));
        c.lwc1(21, camera_frame_at(c, 0xe8));
        c.lwc1(20, camera_frame_at(c, 0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x100));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, DATA, GAME_GP};

    /// Fixture for the mid chase updates: `self` at DATA+0x1000 with random
    /// float fields, in the machine's scrambled registers.
    fn mid_chase_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = fakes::machine(c, seed, 0x0e37_0636 ^ entry as u64, 16);
        c.pc = entry;
        let this = DATA + 0x1000;
        c.g[A0][0] = sx(this);
        for i in 0..0x180 {
            c.bus.write32(this + i * 4, r.edge_f32());
        }
    }





    /// Fixture for the script camera: the script object chain
    /// (`ScriptObj` -> camera entity -> entity), the script engine and the
    /// AI rider table, a subject rider with its target vtable, and the base
    /// transform. Every 4th seed zeroes the transform and the target so the
    /// look vector can be zero (the `atan2` zero-x branches).
    fn subject_script_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x0e37_0636 ^ 0x16a510, 8);
        c.pc = 0x0016_a510;
        let algo = DATA + 0x1000;
        let script = DATA + 0x2000;
        let entity = DATA + 0x2400;
        let cam = DATA + 0x2800;
        let engine = DATA + 0x3000;
        let app = DATA + 0x3200;
        let game = DATA + 0x3300;
        let ai = DATA + 0x3400;
        let rider = DATA + 0x4000;
        let rot_block = DATA + 0x5000;
        let obj = rider + 0x6c0;
        c.g[A0][0] = sx(algo);
        for i in 0..0x40 {
            c.bus.write32(algo + i * 4, r.edge_f32());
            c.bus.write32(script + i * 4, r.edge_f32());
            c.bus.write32(cam + i * 4, r.edge_f32());
            c.bus.write32(engine + i * 4, r.edge_f32());
        }
        for i in 0..0x10 {
            c.bus.write32(entity + i * 4, r.edge_f32());
        }
        // ScriptObj -> mpCameraEntity -> mpEntity (the camera).
        c.bus.write32(algo + 0x14, script);
        c.bus.write32(script + 0x14, entity);
        c.bus.write32(entity + 8, cam);
        // A zero seed zeroes the base transform, so the target and the look
        // vector can come out zero (the atan2 zero branches).
        let zero = seed % 4 == 0;
        for i in 0..0x10 {
            let v = if zero { 0 } else { r.edge_f32() };
            c.bus.write32(script + 0x30 + i * 4, v);
        }
        // The engine, the app, the game and the AI rider table.
        c.bus.write32(GAME_GP.wrapping_sub(0x84c), engine);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), app);
        c.bus.write32(app + 0x84, game);
        c.bus.write32(game + 0xc, ai);
        c.bus.write32(ai + 0x28, rider);
        // The subject rider: rotation block, lead vectors, target vtable.
        c.bus.write32(rider + 0x780, rot_block);
        for i in 0..4 {
            let v = if zero { 0 } else { r.edge_f32() };
            c.bus.write32(rot_block + 0x140 + i * 4, v);
            c.bus.write32(rider + 0x1b0 + i * 4, if zero { 0 } else { r.edge_f32() });
            c.bus.write32(rider + 0x1c0 + i * 4, if zero { 0 } else { r.edge_f32() });
        }
        let vt = fakes::vtable(c, &mut r, &[0x28]);
        c.bus.write32(obj, vt);
    }

    #[test]
    fn subject_script_camera_algo_update_matches() {
        check(
            ssx3_ee::s02::f_0016a510__generated,
            subject_script_camera_algo_update,
            200,
            &|c, seed| subject_script_setup(c, seed),
        );
    }
}
