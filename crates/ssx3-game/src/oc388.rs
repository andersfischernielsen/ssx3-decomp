//! Staging module oc388 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc388`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0012_4788,
    rider_apply_script_track,
    "Rider_ApplyScriptTrack",
)];

// ---------------------------------------------------------------------------
// Rider_ApplyScriptTrack (0x00124788)
// ---------------------------------------------------------------------------

/// `Rider` fields read and written by the script track.
pub(crate) const AST_POS: u32 = 0x110;
pub(crate) const AST_ROT: u32 = 0x120;
pub(crate) const AST_FRONT: u32 = 0x1b0;
pub(crate) const AST_VEL: u32 = 0x1e0;
/// `mSlewed[18]` (target, rate, current): the start-gate knee offset.
pub(crate) const AST_KNEE_SLEW: u32 = 0x2c8;
/// Whether the rider's animation heading is flipped (0 or 1).
pub(crate) const AST_FLIPPED: u32 = 0x2d8;
pub(crate) const AST_SCRIPT_OBJ: u32 = 0x6d0;
pub(crate) const AST_SCRIPT_MATRIX: u32 = 0x700;
pub(crate) const AST_ANIM_MODEL: u32 = 0x780;
pub(crate) const AST_RIDER_ANIM: u32 = 0x784;
pub(crate) const AST_BONE_HIPS: u32 = 0x89c;
pub(crate) const AST_BONE_BOARD_ROOT: u32 = 0x8a0;
/// The track's last position (x, y, z, 1) and Euler angles (radians).
pub(crate) const AST_TRACK_POS: u32 = 0xae0;
pub(crate) const AST_TRACK_ANG: u32 = 0xaf0;
/// Velocity from the board-root bone instead of the position delta.
pub(crate) const AST_BONE_VELOCITY: u32 = 0xafc;

/// `.lit4` (gp-relative, subtracted): 1/60 (rate frames -> seconds), pi,
/// 1/60 (start frame -> seconds), degrees -> radians.
pub(crate) const AST_GP_RATE_STEP: u32 = 0x7890;
pub(crate) const AST_GP_PI: u32 = 0x788c;
pub(crate) const AST_GP_START_STEP: u32 = 0x7888;
pub(crate) const AST_GP_DEG2RAD: u32 = 0x7884;

/// The static origin vec4 the root-motion transforms pivot about
/// (`0x500000 - 0xed0`).
pub(crate) const AST_PIVOT_BASE: u32 = 0x0050_0000;

const AST_SCRIPT_GET_ANIM: u32 = 0x0028_0520;
const AST_SAMPLE_ANIM_POSE: u32 = 0x0031_2660;
const AST_MODEL_ROOT_XFORM: u32 = 0x0030_ecd8;
const AST_CHANGE_ORIENT_OFFSET: u32 = 0x0031_1bf0;
const AST_CHANGE_HEADING_OFFSET: u32 = 0x0031_1b48;
const AST_PLAY_ANIM_DIRECT: u32 = 0x0031_29e0;
const AST_GET_CONTROLLER: u32 = 0x0031_1b20;
const AST_SET_TRACK_TIME: u32 = 0x0031_3cf0;
const AST_ANIM_UPDATE: u32 = 0x0031_2598;
const AST_ANIM_PLAY: u32 = 0x0031_28e8;
const AST_EVENT_PENDING: u32 = 0x0014_46a0;
const AST_EVENT_FIRED: u32 = 0x0014_46b8;
const AST_EVENT_CLEAR: u32 = 0x0014_4670;
const AST_MODEL_EVENT_ON: u32 = 0x0030_e9e0;
const AST_MODEL_EVENT_OFF: u32 = 0x0030_ea80;

const AST_BUILD_WORLD_MATRIX: Callee = Callee {
    addr: 0x0011_e098,
    f: ee::s00::Rider_BuildWorldMatrix,
};
const AST_GATE_CHARGE_ANGLE: Callee = Callee {
    addr: 0x0011_5ab0,
    f: ee::s00::Rider_GetGateChargeAngle,
};
const AST_START_POSE_ANGLE: Callee = Callee {
    addr: 0x0011_5b08,
    f: ee::s00::Rider_GetStartPoseAngle,
};
const AST_QUAT_FROM_EULER: Callee = Callee {
    addr: 0x0012_41c0,
    f: ee::s00::Quat_FromEuler,
};
const AST_TICK_ANIMATION: Callee = Callee {
    addr: 0x0011_eb60,
    f: ee::s00::Rider_TickAnimation,
};
const AST_UPDATE_SKELETON: Callee = Callee {
    addr: 0x0011_eb98,
    f: ee::s00::Rider_UpdateSkeleton,
};
const AST_UPDATE_BOUNDS: Callee = Callee {
    addr: 0x0011_e150,
    f: ee::s00::Rider_UpdateBounds,
};
const AST_UPDATE_SPHERE_TREE: Callee = Callee {
    addr: 0x0010_6828,
    f: ee::s00::Rider_UpdateSphereTree,
};

/// `lwc1 f, a`.
#[inline(always)]
fn ast_ld(c: &mut Cpu, f: usize, a: u32) -> FpuF32 {
    let v = Addr(a).read_fpu(c);
    c.f[f] = v.0;
    v
}

/// `swc1 f, a`.
#[inline(always)]
fn ast_st(c: &mut Cpu, f: usize, a: u32) {
    let v = FpuF32(c.f[f]);
    Addr(a).write_fpu(c, v);
}

/// FPU register `f`.
#[inline(always)]
fn ast_f(c: &Cpu, f: usize) -> FpuF32 {
    FpuF32(c.f[f])
}

/// `out = a - b` (vf3 = a, vf4 = b -> vf5).
fn ast_vsub(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `out = a + b` (vf3 = a, vf4 = b -> vf5).
fn ast_vadd(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `out = v * fS` (the scalar moved through the GPR `via` into vf3.x).
fn ast_vscale(c: &mut Cpu, via: usize, f: usize, v: u32, out: u32) {
    c.mfc1(via, f);
    c.lqc2(4, v);
    c.qmtc2(via, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
}

/// `lq v0, from; sq v0, to...` (a quadword copied through v0).
fn ast_copy(c: &mut Cpu, from: u32, to: &[u32]) {
    c.lq(V0, from);
    for &t in to {
        c.sq(V0, t);
    }
}

/// `void Rider_ApplyScriptTrack(Rider* rider, ScriptFrame* frame, int events,
/// float currFrame, float frameDelta)` (0x00124788): drives a rider from a
/// script track (cutscene / start gate).
///
/// Event bit 0 (a new main animation on this frame): when the frame's
/// animation info starts at `currFrame`, look the animation up in the
/// rider's script object; with root motion, sample the pose at the
/// animation's start, move the rider by the root bone's transform (and
/// push the inverse into the rider animation's orientation offset); with
/// the drift flag, set the knee slew to the gate charge angle and push the
/// rider forward by the start-pose angle; flip the animation heading when
/// the flag changed; play the animation direct at its start time, set its
/// track time and update it at the animation speed. Event bit 1 with no
/// active frame plays animation 5.
///
/// Then: move the rider by the track position delta (through the script
/// matrix), turn it by the Euler angle delta, derive the velocity (from
/// the board-root bone when flagged; per second when `frameDelta > 0`),
/// tick the animation and skeleton, fire / clear the controller's model
/// events 0 and 1, and update the bounds and sphere tree.
pub fn rider_apply_script_track(c: &mut Cpu) {
    guest(c, |c| {
        // 0x00124788: frame, saves.
        c.cycles += 16;
        let frame = Frame::open(c, 0x1f0);
        c.g[V0][0] = c.g[A2][0] & 0x1;
        frame.save(c, S1, 0x1b0);
        frame.save(c, S2, 0x1a0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x1d8);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x1c0);
        c.f[21] = c.f[13];
        frame.save(c, S3, 0x190);
        frame.save(c, S4, 0x180);
        frame.save(c, S5, 0x170);
        frame.save_ra(c, 0x160);
        frame.save_fpr(c, 22, 0x1e0);
        frame.save_fpr(c, 20, 0x1d0);

        if c.g[V0][0] != 0 {
            // A new main animation: only when the frame is active and the
            // animation starts on this frame.
            c.cycles += 3;
            let fi = r32(c, S2);
            c.g[V0][0] = c.lw(fi);
            frame.restore(c, S0, 0x1c0);
            if c.g[V0][0] == 0 {
                ast_epilogue(c, frame);
                return Ok(());
            }
            c.cycles += 7;
            c.g[S3][0] = c.lw(fi.wrapping_add(4));
            let info = r32(c, S3);
            let start = FpuF32::from_i32(c.lw(info.wrapping_add(4)) as u32 as i32);
            c.f[0] = start.0;
            let same = start.eq_s(ast_f(c, 12));
            crate::abi::set_fcc(c, same);
            c.g[S5][0] = c.lw(info.wrapping_add(0x1c));
            if !same {
                ast_epilogue(c, frame);
                return Ok(());
            }

            // 0x00124bf8 block: look the animation up (animID, sourceID).
            c.cycles += 14;
            let rate = ast_ld(c, 1, info.wrapping_add(0x10));
            let step = Addr(r32(c, GP).wrapping_sub(AST_GP_RATE_STEP)).read_fpu(c);
            c.f[0] = step.0;
            let rider = r32(c, S1);
            c.g[A0][0] = sx(rider.wrapping_add(AST_SCRIPT_OBJ));
            c.g[A1][0] = c.lw(info);
            let s = sp(c);
            c.g[A2][0] = sx(s.wrapping_add(0x150));
            c.f[22] = (rate * step).0;
            c.g[A3][0] = sx(s.wrapping_add(0x154));
            call_ptr(c, AST_SCRIPT_GET_ANIM, 0x0012_4810)?;
            let s = sp(c);
            c.g[V0][0] = c.lw(s.wrapping_add(0x150));
            c.g[A0][0] = c.lbu(s.wrapping_add(0x154));
            c.g[S4][0] = sx((c.g[V0][0] as u32) << 8);
            let flags = r32(c, S5);
            c.g[V1][0] = c.lb(flags.wrapping_add(1));
            c.g[S4][0] |= c.g[A0][0];
            if c.g[V1][0] != 0 {
                ast_root_motion(c)?;
            }

            // 0x00124bf8: the drift flag (start gate).
            c.cycles += 3;
            let flags = r32(c, S5);
            c.g[V0][0] = c.lb(flags.wrapping_add(2));
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.lb(flags);
            } else {
                c.cycles += 24;
                c.g[A0][0] = c.g[S1][0];
                AST_GATE_CHARGE_ANGLE.call(c, 0x0012_4c0c)?;
                let rider = r32(c, S1);
                let s = sp(c);
                c.lq(V0, rider.wrapping_add(AST_FRONT));
                c.g[A0][0] = c.g[S1][0];
                c.sw(rider.wrapping_add(AST_KNEE_SLEW + 4), 0);
                ast_st(c, 0, rider.wrapping_add(AST_KNEE_SLEW));
                c.sq(V0, s.wrapping_add(0x10));
                ast_st(c, 0, rider.wrapping_add(AST_KNEE_SLEW + 8));
                AST_START_POSE_ANGLE.call(c, 0x0012_4c28)?;
                let s = sp(c);
                let rider = r32(c, S1);
                ast_vscale(c, V0, 0, s.wrapping_add(0x10), s.wrapping_add(0x20));
                c.g[A0][0] = c.g[S1][0];
                ast_copy(c, s.wrapping_add(0x20), &[s]);
                ast_vadd(
                    c,
                    rider.wrapping_add(AST_POS),
                    s,
                    rider.wrapping_add(AST_POS),
                );
                AST_BUILD_WORLD_MATRIX.call(c, 0x0012_4c60)?;
                let flags = r32(c, S5);
                c.g[V0][0] = c.lb(flags);
            }

            // 0x00124c64: flip the heading when the flag changed.
            c.cycles += 4;
            let rider = r32(c, S1);
            c.g[V1][0] = c.lw(rider.wrapping_add(AST_FLIPPED));
            c.g[S0][0] = (0u64 < c.g[V0][0]) as u64;
            if c.g[S0][0] != c.g[V1][0] {
                c.cycles += 3;
                c.f[12] = 0;
                if c.g[S0][0] != 0 {
                    c.cycles += 1;
                    let pi = Addr(r32(c, GP).wrapping_sub(AST_GP_PI)).read_fpu(c);
                    c.f[12] = pi.0;
                }
                c.cycles += 4;
                c.g[A0][0] = c.lw(rider.wrapping_add(AST_RIDER_ANIM));
                call_ptr(c, AST_CHANGE_HEADING_OFFSET, 0x0012_4c8c)?;
                let rider = r32(c, S1);
                c.sw(rider.wrapping_add(AST_FLIPPED), c.g[S0][0]);
            }
            let info = r32(c, S3);
            c.g[V0][0] = c.lw(info.wrapping_add(0x18));

            // 0x00124c94: the start frame as an unsigned float.
            c.cycles += 2;
            let start = c.g[V0][0];
            c.g[V1][0] = sx((start as u32) >> 1);
            if (start as i64) >= 0 {
                c.cycles += 4;
                c.f[1] = FpuF32::from_i32(start as u32 as i32).0;
            } else {
                c.cycles += 6;
                c.g[V0][0] = (start & 1) | c.g[V1][0];
                let half = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
                c.f[1] = (half + half).0;
            }
            ast_ld(c, 0, info.wrapping_add(0x14));

            // 0x00124cc4: play it at its speed, set the track time, update.
            c.cycles += 23;
            c.g[A2][0] = c.g[S4][0];
            let rider = r32(c, S1);
            c.g[V0][0] = c.lw(rider.wrapping_add(AST_RIDER_ANIM));
            c.g[A1][0] = 0x1b0;
            let step = Addr(r32(c, GP).wrapping_sub(AST_GP_START_STEP)).read_fpu(c);
            let anim = c.g[V0][0] as u32;
            ast_st(c, 0, anim.wrapping_add(0x1c));
            c.f[12] = (ast_f(c, 1) * step).0;
            c.g[A0][0] = c.lw(rider.wrapping_add(AST_RIDER_ANIM));
            call_ptr(c, AST_PLAY_ANIM_DIRECT, 0x0012_4ce4)?;
            let rider = r32(c, S1);
            c.g[A0][0] = c.lw(rider.wrapping_add(AST_RIDER_ANIM));
            c.g[A1][0] = 0x2;
            call_ptr(c, AST_GET_CONTROLLER, 0x0012_4cf0)?;
            c.g[A0][0] = c.g[V0][0];
            c.f[12] = c.f[22];
            c.g[A1][0] = 0;
            call_ptr(c, AST_SET_TRACK_TIME, 0x0012_4d00)?;
            let rider = r32(c, S1);
            c.g[A0][0] = c.lw(rider.wrapping_add(AST_RIDER_ANIM));
            c.g[A1][0] = 0;
            call_ptr(c, AST_ANIM_UPDATE, 0x0012_4d0c)?;
            let rider = r32(c, S1);
            c.g[V0][0] = c.lw(rider.wrapping_add(AST_RIDER_ANIM));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            let anim = c.g[V0][0] as u32;
            ast_st(c, 0, anim.wrapping_add(0x1c));
            c.cycles += 1;
        } else {
            // 0x00124d20: event bit 1 with no active frame plays anim 5.
            c.cycles += 3;
            c.g[V0][0] = c.g[A2][0] & 0x2;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, S2));
                if c.g[V0][0] == 0 {
                    c.cycles += 6;
                    let rider = r32(c, S1);
                    c.g[A0][0] = c.lw(rider.wrapping_add(AST_RIDER_ANIM));
                    c.g[A1][0] = 0x5;
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 12);
                    c.g[A2][0] = 0;
                    call_ptr(c, AST_ANIM_PLAY, 0x0012_4d50)?;
                    c.cycles += 1;
                }
            }
        }
        let fi = r32(c, S2);
        ast_ld(c, 3, fi.wrapping_add(0x18));

        ast_follow_track(c)?;

        // 0x00124fd4: bounds from the hips bone at the rider's depth.
        c.cycles += 14;
        let rider = r32(c, S1);
        let s = sp(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.lw(rider.wrapping_add(AST_BONE_HIPS));
        c.g[A1][0] = sx(s.wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 5);
        ast_ld(c, 0, rider.wrapping_add(AST_POS + 8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        let hips = c.g[V1][0] as u32;
        ast_copy(c, hips, &[s.wrapping_add(0x40)]);
        ast_st(c, 0, s.wrapping_add(0x48));
        AST_UPDATE_BOUNDS.call(c, 0x0012_5000)?;
        c.g[A0][0] = c.g[S1][0];
        AST_UPDATE_SPHERE_TREE.call(c, 0x0012_5008)?;
        frame.restore(c, S0, 0x1c0);
        ast_epilogue(c, frame);
        Ok(())
    })
}

/// 0x0012500c: restore and return (s0 is already restored on every path).
fn ast_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, 0x1b0);
    frame.restore(c, S2, 0x1a0);
    frame.restore(c, S3, 0x190);
    frame.restore(c, S4, 0x180);
    frame.restore(c, S5, 0x170);
    frame.restore_ra(c, 0x160);
    frame.restore_fpr(c, 22, 0x1e0);
    frame.restore_fpr(c, 21, 0x1d8);
    frame.restore_fpr(c, 20, 0x1d0);
    frame.close(c);
}

/// 0x00124820: root motion of the new animation. Samples the pose at the
/// start (into sp+0), the model's root transform `a` (sp+0x50 pos, sp+0x60
/// rot), the hips bone `node` (scaled position, rotation); `y = node *
/// inverse(a)` (rotations about the pivot, scaled by -1), the rider's
/// transform combined with `y`, the world matrix rebuilt, and `inverse(y)`
/// passed to the rider animation's orientation offset.
fn ast_root_motion(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 244;
    let rider = r32(c, S1);
    c.g[A1][0] = c.lw(rider.wrapping_add(AST_RIDER_ANIM));
    c.g[A0][0] = c.g[SP][0];
    c.f[12] = c.f[22];
    c.g[A2][0] = 0x1b0;
    c.g[A3][0] = c.g[S4][0];
    c.g[S0][0] = AST_PIVOT_BASE as u64;
    call_ptr(c, AST_SAMPLE_ANIM_POSE, 0x0012_4844)?;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 20);
    let rider = r32(c, S1);
    let s = sp(c);
    c.g[A1][0] = c.lw(rider.wrapping_add(AST_ANIM_MODEL));
    c.g[A0][0] = sx(s.wrapping_add(0x50));
    c.g[A2][0] = c.g[SP][0];
    c.g[A3][0] = 0;
    call_ptr(c, AST_MODEL_ROOT_XFORM, 0x0012_4860)?;

    // node.pos = hips local position * the model's scale; node.rot.
    let rider = r32(c, S1);
    let s = sp(c);
    c.g[V1][0] = c.lw(rider.wrapping_add(AST_ANIM_MODEL));
    let model = c.g[V1][0] as u32;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.lw(rider.wrapping_add(AST_BONE_HIPS));
    c.g[V0][0] = c.lw(model.wrapping_add(0x24));
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
    ast_ld(c, 4, model.wrapping_add(0x140));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let lp = c.g[V0][0] as u32;
    ast_ld(c, 3, model.wrapping_add(0x148));
    ast_ld(c, 0, lp);
    ast_ld(c, 6, model.wrapping_add(0x144));
    c.f[0] = (ast_f(c, 0) * ast_f(c, 4)).0;
    ast_ld(c, 5, lp.wrapping_add(0xc));
    ast_ld(c, 1, lp.wrapping_add(4));
    ast_ld(c, 2, lp.wrapping_add(8));
    ast_ld(c, 4, model.wrapping_add(0x14c));
    c.f[1] = (ast_f(c, 1) * ast_f(c, 6)).0;
    c.f[2] = (ast_f(c, 2) * ast_f(c, 3)).0;
    ast_st(c, 0, s.wrapping_add(0x90));
    c.f[5] = (ast_f(c, 5) * ast_f(c, 4)).0;
    ast_ld(c, 3, s.wrapping_add(0x60));
    ast_st(c, 1, s.wrapping_add(0x94));
    ast_st(c, 2, s.wrapping_add(0x98));
    c.f[3] = (-ast_f(c, 3)).0;
    ast_st(c, 5, s.wrapping_add(0x9c));
    ast_ld(c, 1, s.wrapping_add(0x64));
    c.lq(V0, s.wrapping_add(0x90));
    ast_ld(c, 0, s.wrapping_add(0x68));
    c.f[1] = (-ast_f(c, 1)).0;
    c.sq(V0, s.wrapping_add(0x70));
    c.f[0] = (-ast_f(c, 0)).0;
    ast_ld(c, 2, s.wrapping_add(0x6c));
    c.g[V0][0] = c.lw(model.wrapping_add(0x28));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.lq(V1, c.g[V0][0] as u32);
    // inverse(a).rot = conjugate (sp+0x110).
    ast_st(c, 2, s.wrapping_add(0x11c));
    ast_st(c, 3, s.wrapping_add(0x110));
    ast_st(c, 1, s.wrapping_add(0x114));
    ast_st(c, 0, s.wrapping_add(0x118));
    c.sq(V1, s.wrapping_add(0x80));
    ast_copy(
        c,
        s.wrapping_add(0x110),
        &[s.wrapping_add(0xf0), s.wrapping_add(0x100)],
    );

    let pivot = r32(c, S0).wrapping_sub(0xed0);
    // inverse(a).pos = pivot - rot'(a.pos - pivot).
    ast_vsub(c, s.wrapping_add(0x50), pivot, s.wrapping_add(0x140));
    ast_copy(c, s.wrapping_add(0x140), &[s.wrapping_add(0x130)]);
    crate::game::ai::rider::q_rotate(
        c,
        s.wrapping_add(0xf0),
        s.wrapping_add(0x130),
        s.wrapping_add(0x140),
    );
    ast_copy(c, s.wrapping_add(0x140), &[s.wrapping_add(0x120)]);
    ast_vscale(c, V1, 20, s.wrapping_add(0x120), s.wrapping_add(0x140));
    ast_copy(c, s.wrapping_add(0x140), &[s.wrapping_add(0x110)]);
    ast_vadd(c, s.wrapping_add(0x110), pivot, s.wrapping_add(0x140));
    c.lq(V0, s.wrapping_add(0x140));
    c.lq(V1, s.wrapping_add(0xf0));
    c.sq(V0, s.wrapping_add(0x100));
    c.sq(V0, s.wrapping_add(0xc0));
    c.sq(V1, s.wrapping_add(0xd0));
    c.sq(V0, s.wrapping_add(0xe0));

    // y = node * inverse(a) (sp+0xa0 pos, sp+0xb0 rot).
    crate::game::ai::rider::q_mul(
        c,
        s.wrapping_add(0x80),
        s.wrapping_add(0xd0),
        s.wrapping_add(0x110),
    );
    ast_copy(
        c,
        s.wrapping_add(0x110),
        &[s.wrapping_add(0xf0), s.wrapping_add(0x100)],
    );
    ast_vsub(c, s.wrapping_add(0xc0), pivot, s.wrapping_add(0x130));
    ast_copy(c, s.wrapping_add(0x130), &[s.wrapping_add(0x120)]);
    crate::game::ai::rider::q_rotate(
        c,
        s.wrapping_add(0x80),
        s.wrapping_add(0x120),
        s.wrapping_add(0x130),
    );
    ast_copy(c, s.wrapping_add(0x130), &[s.wrapping_add(0x110)]);
    ast_vadd(
        c,
        s.wrapping_add(0x110),
        s.wrapping_add(0x70),
        s.wrapping_add(0x130),
    );
    c.lq(V0, s.wrapping_add(0x130));
    c.lq(V1, s.wrapping_add(0xf0));
    c.sq(V0, s.wrapping_add(0x100));
    c.sq(V0, s.wrapping_add(0xa0));
    c.sq(V1, s.wrapping_add(0xb0));
    c.sq(V0, s.wrapping_add(0xe0));

    // rider = rider * y.
    let rot = rider.wrapping_add(AST_ROT);
    let pos = rider.wrapping_add(AST_POS);
    crate::game::ai::rider::q_mul(c, rot, s.wrapping_add(0xb0), s.wrapping_add(0x110));
    ast_copy(
        c,
        s.wrapping_add(0x110),
        &[s.wrapping_add(0xf0), s.wrapping_add(0x100)],
    );
    ast_vsub(c, s.wrapping_add(0xa0), pivot, s.wrapping_add(0x130));
    ast_copy(c, s.wrapping_add(0x130), &[s.wrapping_add(0x120)]);
    crate::game::ai::rider::q_rotate(c, rot, s.wrapping_add(0x120), s.wrapping_add(0x130));
    ast_copy(c, s.wrapping_add(0x130), &[s.wrapping_add(0x110)]);
    ast_vadd(c, s.wrapping_add(0x110), pos, s.wrapping_add(0x130));
    c.lq(V0, s.wrapping_add(0x130));
    c.lq(V1, s.wrapping_add(0xf0));
    c.sq(V0, pos);
    c.sq(V1, rot);
    c.sq(V0, s.wrapping_add(0x100));
    c.sq(V0, s.wrapping_add(0xe0));
    c.sq(V0, s.wrapping_add(0xc0));
    c.sq(V1, s.wrapping_add(0xd0));
    AST_BUILD_WORLD_MATRIX.call(c, 0x0012_4b1c)?;

    // inverse(y) (sp+0xc0 pos, sp+0xd0 rot) to the orientation offset.
    let s = sp(c);
    ast_ld(c, 3, s.wrapping_add(0xb0));
    c.g[A1][0] = sx(s.wrapping_add(0xc0));
    ast_ld(c, 0, s.wrapping_add(0xb4));
    ast_ld(c, 1, s.wrapping_add(0xb8));
    c.f[3] = (-ast_f(c, 3)).0;
    c.f[0] = (-ast_f(c, 0)).0;
    ast_ld(c, 2, s.wrapping_add(0xbc));
    c.f[1] = (-ast_f(c, 1)).0;
    ast_st(c, 2, s.wrapping_add(0x11c));
    ast_st(c, 3, s.wrapping_add(0x110));
    ast_st(c, 0, s.wrapping_add(0x114));
    ast_st(c, 1, s.wrapping_add(0x118));
    ast_copy(
        c,
        s.wrapping_add(0x110),
        &[s.wrapping_add(0xf0), s.wrapping_add(0x100)],
    );
    let pivot = r32(c, S0).wrapping_sub(0xed0);
    ast_vsub(c, s.wrapping_add(0xa0), pivot, s.wrapping_add(0x140));
    ast_copy(c, s.wrapping_add(0x140), &[s.wrapping_add(0x130)]);
    crate::game::ai::rider::q_rotate(
        c,
        s.wrapping_add(0xf0),
        s.wrapping_add(0x130),
        s.wrapping_add(0x140),
    );
    ast_copy(c, s.wrapping_add(0x140), &[s.wrapping_add(0x120)]);
    ast_vscale(c, V1, 20, s.wrapping_add(0x120), s.wrapping_add(0x140));
    ast_copy(c, s.wrapping_add(0x140), &[s.wrapping_add(0x110)]);
    ast_vadd(c, s.wrapping_add(0x110), pivot, s.wrapping_add(0x140));
    c.lq(V0, s.wrapping_add(0x140));
    c.sq(V0, s.wrapping_add(0x100));
    c.lq(V1, s.wrapping_add(0xf0));
    let rider = r32(c, S1);
    c.g[A0][0] = c.lw(rider.wrapping_add(AST_RIDER_ANIM));
    c.sq(V0, s.wrapping_add(0xc0));
    c.sq(V1, s.wrapping_add(0xd0));
    c.sq(V0, s.wrapping_add(0xe0));
    call_ptr(c, AST_CHANGE_ORIENT_OFFSET, 0x0012_4bf8)
}

/// 0x00124d54 (f3 = frame->pos[0]): follow the track. The track position
/// (x, y, z, 1) and Euler angles (degrees -> radians) replace the stored
/// ones; the position moves by the script matrix times the position delta;
/// velocity = new - old position; the orientation turns by the Euler delta
/// (`Quat_FromEuler(z, y, x)` premultiplied); world matrix, animation tick,
/// skeleton; the board-root bone velocity when flagged; velocity per second
/// when `frameDelta > 0`; the controller's model events 0 and 1. Leaves
/// v0 = the anim model.
fn ast_follow_track(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 98;
    let rider = r32(c, S1);
    let fi = r32(c, S2);
    let s = sp(c);
    c.g[A2][0] = sx(rider.wrapping_add(AST_SCRIPT_MATRIX));
    ast_ld(c, 2, fi.wrapping_add(0x1c));
    c.g[A1][0] = sx(s.wrapping_add(0x20));
    ast_ld(c, 1, fi.wrapping_add(0x20));
    c.g[A0][0] = c.g[S1][0];
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    ast_st(c, 2, s.wrapping_add(0x14));
    ast_st(c, 3, s.wrapping_add(0x10));
    ast_st(c, 1, s.wrapping_add(0x18));
    ast_st(c, 0, s.wrapping_add(0x1c));
    let d2r_at = r32(c, GP).wrapping_sub(AST_GP_DEG2RAD);
    let d2r = ast_ld(c, 2, d2r_at);
    let track = rider.wrapping_add(AST_TRACK_POS);
    let ang = rider.wrapping_add(AST_TRACK_ANG);
    c.lq(V0, s.wrapping_add(0x10));
    c.lq(V1, track);
    c.sq(V0, track);
    ast_ld(c, 5, ang);
    ast_ld(c, 0, fi.wrapping_add(0x24));
    ast_ld(c, 4, ang.wrapping_add(4));
    c.f[0] = (ast_f(c, 0) * d2r).0;
    ast_ld(c, 3, ang.wrapping_add(8));
    c.lq(V0, rider.wrapping_add(AST_POS));
    c.sq(V1, s);
    ast_st(c, 0, ang);
    c.sq(V0, s.wrapping_add(0x10));
    ast_ld(c, 0, fi.wrapping_add(0x28));
    c.f[0] = (ast_f(c, 0) * d2r).0;
    ast_st(c, 0, ang.wrapping_add(4));
    ast_ld(c, 1, fi.wrapping_add(0x2c));
    c.f[1] = (ast_f(c, 1) * d2r).0;
    ast_st(c, 1, ang.wrapping_add(8));

    // pos += scriptMatrix * (track - oldTrack).
    ast_vsub(c, track, s, s.wrapping_add(0x40));
    ast_copy(c, s.wrapping_add(0x40), &[s.wrapping_add(0x30)]);
    let m = c.g[A2][0] as u32;
    c.lqc2(8, s.wrapping_add(0x30));
    c.lqc2(4, m);
    c.lqc2(5, m.wrapping_add(0x10));
    c.lqc2(6, m.wrapping_add(0x20));
    c.lqc2(7, m.wrapping_add(0x30));
    {
        let v = c.vu0();
        v.mula(0b1111, 4, Src::Bc(8, X));
        v.madda(0b1111, 5, Src::Bc(8, Y));
        v.madda(0b1111, 6, Src::Bc(8, Z));
        v.madd(0b1111, 12, 7, Src::Bc(8, W));
    }
    c.sqc2(12, s.wrapping_add(0x40));
    ast_copy(c, s.wrapping_add(0x40), &[s.wrapping_add(0x20)]);
    let pos = rider.wrapping_add(AST_POS);
    ast_vadd(c, pos, s.wrapping_add(0x20), pos);
    ast_vsub(c, pos, s.wrapping_add(0x10), s.wrapping_add(0x30));

    // The Euler delta (z, y, x) -> sp+0x20.
    ast_ld(c, 12, ang.wrapping_add(8));
    ast_ld(c, 13, ang.wrapping_add(4));
    ast_ld(c, 14, ang);
    c.f[12] = (ast_f(c, 12) - ast_f(c, 3)).0;
    c.lq(V0, s.wrapping_add(0x30));
    c.f[13] = (ast_f(c, 13) - ast_f(c, 4)).0;
    c.f[14] = (ast_f(c, 14) - ast_f(c, 5)).0;
    c.sq(V0, rider.wrapping_add(AST_VEL));
    c.sq(V0, s.wrapping_add(0x20));
    AST_QUAT_FROM_EULER.call(c, 0x0012_4e5c)?;
    let s = sp(c);
    let rider = r32(c, S1);
    crate::game::ai::rider::q_mul(
        c,
        s.wrapping_add(0x20),
        rider.wrapping_add(AST_ROT),
        s.wrapping_add(0x40),
    );
    c.g[A0][0] = c.g[S1][0];
    c.lq(V0, s.wrapping_add(0x40));
    c.sq(V0, rider.wrapping_add(AST_ROT));
    c.sq(V0, s.wrapping_add(0x30));
    AST_BUILD_WORLD_MATRIX.call(c, 0x0012_4ea0)?;

    // The board-root bone before the animation tick.
    let rider = r32(c, S1);
    let s = sp(c);
    c.g[V0][0] = c.lw(rider.wrapping_add(AST_ANIM_MODEL));
    c.g[A0][0] = c.g[S1][0];
    c.g[V1][0] = c.lw(rider.wrapping_add(AST_BONE_BOARD_ROOT));
    c.f[12] = c.f[21];
    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
    c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let board = r32(c, S0);
    ast_copy(c, board, &[s.wrapping_add(0x30)]);
    AST_TICK_ANIMATION.call(c, 0x0012_4ec8)?;
    c.g[A0][0] = c.g[S1][0];
    AST_UPDATE_SKELETON.call(c, 0x0012_4ed0)?;
    let rider = r32(c, S1);
    c.g[V0][0] = c.lw(rider.wrapping_add(AST_BONE_VELOCITY));
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        let s = sp(c);
        let board = r32(c, S0);
        ast_vsub(c, board, s.wrapping_add(0x30), s.wrapping_add(0x50));
        c.lq(V0, s.wrapping_add(0x50));
        c.sq(V0, rider.wrapping_add(AST_VEL));
        c.sq(V0, s.wrapping_add(0x40));
    }

    // 0x00124ef8: velocity per second.
    c.cycles += 5;
    c.f[0] = 0;
    let dt = ast_f(c, 21);
    let positive = FpuF32::ZERO.lt(dt);
    crate::abi::set_fcc(c, positive);
    if positive {
        c.cycles += 10;
        let vel = rider.wrapping_add(AST_VEL);
        ast_ld(c, 0, vel);
        ast_ld(c, 1, vel.wrapping_add(4));
        ast_ld(c, 2, vel.wrapping_add(8));
        c.f[0] = ast_f(c, 0).div(dt).0;
        c.f[1] = ast_f(c, 1).div(dt).0;
        c.f[2] = ast_f(c, 2).div(dt).0;
        ast_st(c, 0, vel);
        ast_st(c, 1, vel.wrapping_add(4));
        ast_st(c, 2, vel.wrapping_add(8));
    }
    c.g[A0][0] = c.lw(rider.wrapping_add(AST_RIDER_ANIM));

    // 0x00124f34: the controller's model events 0 (on) and 1 (off).
    c.cycles += 9;
    c.g[A1][0] = 0x2;
    c.g[S2][0] = 0;
    call_ptr(c, AST_GET_CONTROLLER, 0x0012_4f40)?;
    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, AST_EVENT_PENDING, 0x0012_4f50)?;
    c.g[A0][0] = c.g[S0][0];
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[A1][0] = 0;
        call_ptr(c, AST_EVENT_FIRED, 0x0012_4f60)?;
        c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
    }

    c.cycles += 2;
    c.g[A1][0] = 0xb;
    if c.g[S2][0] != 0 {
        c.cycles += 8;
        let rider = r32(c, S1);
        c.g[A0][0] = c.lw(rider.wrapping_add(AST_ANIM_MODEL));
        c.g[A2][0] = 0;
        call_ptr(c, AST_MODEL_EVENT_ON, 0x0012_4f78)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, AST_EVENT_CLEAR, 0x0012_4f84)?;
        let rider = r32(c, S1);
        c.g[V0][0] = c.lw(rider.wrapping_add(AST_ANIM_MODEL));
        return Ok(());
    }

    c.cycles += 6;
    c.g[S2][0] = 0;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0x1;
    call_ptr(c, AST_EVENT_PENDING, 0x0012_4f9c)?;
    c.g[A0][0] = c.g[S0][0];
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[A1][0] = 0x1;
        call_ptr(c, AST_EVENT_FIRED, 0x0012_4fac)?;
        c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
    }

    c.cycles += 2;
    c.g[A1][0] = 0xb;
    if c.g[S2][0] != 0 {
        c.cycles += 6;
        let rider = r32(c, S1);
        c.g[A0][0] = c.lw(rider.wrapping_add(AST_ANIM_MODEL));
        c.g[A2][0] = 0;
        call_ptr(c, AST_MODEL_EVENT_OFF, 0x0012_4fc4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, AST_EVENT_CLEAR, 0x0012_4fd0)?;
    }
    c.cycles += 1;
    let rider = r32(c, S1);
    c.g[V0][0] = c.lw(rider.wrapping_add(AST_ANIM_MODEL));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, fall_back, hook, machine, prev_cell};
    use crate::difftest::{put_vec, put_words, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate of the Rider_ApplyScriptTrack fakes: they fake only while
    /// `AST_CTRL` holds `AST_MAGIC` (in this test's machines).
    pub(crate) const AST_CTRL: u32 = DATA + 0xc_0000;
    pub(crate) const AST_MAGIC: u32 = 0x4153_5452;
    /// Bits (by event index 0/1) that `AnimEvent_IsPending` / `_HasFired`
    /// return.
    pub(crate) const AST_PENDING_BITS: u32 = AST_CTRL + 4;
    pub(crate) const AST_FIRED_BITS: u32 = AST_CTRL + 8;
    /// What the root-transform fake writes to its output (pos, rot) and the
    /// Euler-quaternion fake to its output.
    pub(crate) const AST_XFORM_TEMPLATE: u32 = AST_CTRL + 0x100;
    pub(crate) const AST_EULER_TEMPLATE: u32 = AST_CTRL + 0x200;

    /// A logged fake (`fakes::fake`) with the results the paths need: the
    /// event queries from the control bits, `BE_GetRiderAltCharacter` 0 and
    /// `BENewPlayerInt_IsSharingCharRecord` 1 (the short path of the real
    /// `BE_GetRiderSizeScale` under the real gate/start-pose angles), and
    /// the two transforms' outputs.
    pub(crate) fn ast_fake(c: &mut Cpu, target: u32) {
        let (a0, a1) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        let bit = |c: &mut Cpu, at: u32| ((c.bus.read32(at) >> (a1 & 1)) & 1) as u64;
        match target {
            AST_EVENT_PENDING => c.g[V0][0] = bit(c, AST_PENDING_BITS),
            AST_EVENT_FIRED => c.g[V0][0] = bit(c, AST_FIRED_BITS),
            0x0014_a0b0 => c.g[V0][0] = 0,
            0x0014_77e8 => c.g[V0][0] = 1,
            AST_MODEL_ROOT_XFORM => {
                for i in 0..8 {
                    let w = c.bus.read32(AST_XFORM_TEMPLATE + 4 * i);
                    c.bus.write32(a0.wrapping_add(4 * i), w);
                }
            }
            0x0012_41c0 => {
                for i in 0..4 {
                    let w = c.bus.read32(AST_EULER_TEMPLATE + 4 * i);
                    c.bus.write32(a1.wrapping_add(4 * i), w);
                }
            }
            _ => {}
        }
    }

    macro_rules! ast_gated {
        ($name:ident, $addr:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(AST_CTRL) != AST_MAGIC {
                    return fall_back(c, prev_cell($name), $gen);
                }
                ast_fake(c, $addr);
            }
        };
    }

    ast_gated!(
        ast_fake_sample_pose,
        0x0031_2660,
        ee::s10::f_00312660__generated
    );
    ast_gated!(
        ast_fake_root_xform,
        0x0030_ecd8,
        ee::s10::f_0030ecd8__generated
    );
    ast_gated!(
        ast_fake_orient_offset,
        0x0031_1bf0,
        ee::s10::f_00311bf0__generated
    );
    ast_gated!(
        ast_fake_heading_offset,
        0x0031_1b48,
        ee::s10::f_00311b48__generated
    );
    ast_gated!(
        ast_fake_play_direct,
        0x0031_29e0,
        ee::s10::f_003129e0__generated
    );
    ast_gated!(
        ast_fake_get_controller,
        0x0031_1b20,
        ee::s10::f_00311b20__generated
    );
    ast_gated!(
        ast_fake_track_time,
        0x0031_3cf0,
        ee::s10::f_00313cf0__generated
    );
    ast_gated!(
        ast_fake_anim_update,
        0x0031_2598,
        ee::s10::f_00312598__generated
    );
    ast_gated!(
        ast_fake_anim_play,
        0x0031_28e8,
        ee::s10::f_003128e8__generated
    );
    ast_gated!(
        ast_fake_event_on,
        0x0030_e9e0,
        ee::s10::f_0030e9e0__generated
    );
    ast_gated!(
        ast_fake_event_off,
        0x0030_ea80,
        ee::s10::f_0030ea80__generated
    );
    ast_gated!(
        ast_fake_world_matrix,
        0x0011_e098,
        ee::s00::f_0011e098__generated
    );
    ast_gated!(
        ast_fake_quat_euler,
        0x0012_41c0,
        ee::s00::f_001241c0__generated
    );
    ast_gated!(
        ast_fake_tick_anim,
        0x0011_eb60,
        ee::s00::f_0011eb60__generated
    );
    ast_gated!(
        ast_fake_skeleton,
        0x0011_eb98,
        ee::s00::f_0011eb98__generated
    );
    ast_gated!(ast_fake_bounds, 0x0011_e150, ee::s00::f_0011e150__generated);
    ast_gated!(
        ast_fake_sphere_tree,
        0x0010_6828,
        ee::s00::f_00106828__generated
    );
    ast_gated!(
        ast_fake_pending,
        0x0014_46a0,
        ee::s01::f_001446a0__generated
    );
    ast_gated!(ast_fake_fired, 0x0014_46b8, ee::s01::f_001446b8__generated);
    ast_gated!(ast_fake_clear, 0x0014_4670, ee::s01::f_00144670__generated);
    ast_gated!(ast_fake_be, 0x0014_dc80, ee::s01::f_0014dc80__generated);
    ast_gated!(
        ast_fake_interface,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );
    ast_gated!(
        ast_fake_character,
        0x0014_a080,
        ee::s01::f_0014a080__generated
    );
    ast_gated!(
        ast_fake_alt_character,
        0x0014_a0b0,
        ee::s01::f_0014a0b0__generated
    );
    ast_gated!(
        ast_fake_sharing,
        0x0014_77e8,
        ee::s01::f_001477e8__generated
    );

    /// Hook the gated fakes in their callees' slots (once per process).
    fn ast_install_fakes() {
        let slots: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 25] = [
            (
                addr_of_mut!(ee::s10::OVERRIDE_00312660),
                ast_fake_sample_pose,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_0030ECD8),
                ast_fake_root_xform,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00311BF0),
                ast_fake_orient_offset,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00311B48),
                ast_fake_heading_offset,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_003129E0),
                ast_fake_play_direct,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00311B20),
                ast_fake_get_controller,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00313CF0),
                ast_fake_track_time,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00312598),
                ast_fake_anim_update,
            ),
            (addr_of_mut!(ee::s10::OVERRIDE_003128E8), ast_fake_anim_play),
            (addr_of_mut!(ee::s10::OVERRIDE_0030E9E0), ast_fake_event_on),
            (addr_of_mut!(ee::s10::OVERRIDE_0030EA80), ast_fake_event_off),
            (
                addr_of_mut!(ee::s00::OVERRIDE_0011E098),
                ast_fake_world_matrix,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_001241C0),
                ast_fake_quat_euler,
            ),
            (addr_of_mut!(ee::s00::OVERRIDE_0011EB60), ast_fake_tick_anim),
            (addr_of_mut!(ee::s00::OVERRIDE_0011EB98), ast_fake_skeleton),
            (addr_of_mut!(ee::s00::OVERRIDE_0011E150), ast_fake_bounds),
            (
                addr_of_mut!(ee::s00::OVERRIDE_00106828),
                ast_fake_sphere_tree,
            ),
            (addr_of_mut!(ee::s01::OVERRIDE_001446A0), ast_fake_pending),
            (addr_of_mut!(ee::s01::OVERRIDE_001446B8), ast_fake_fired),
            (addr_of_mut!(ee::s01::OVERRIDE_00144670), ast_fake_clear),
            (addr_of_mut!(ee::s01::OVERRIDE_0014DC80), ast_fake_be),
            (addr_of_mut!(ee::s01::OVERRIDE_0014DD58), ast_fake_interface),
            (addr_of_mut!(ee::s01::OVERRIDE_0014A080), ast_fake_character),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014A0B0),
                ast_fake_alt_character,
            ),
            (addr_of_mut!(ee::s01::OVERRIDE_001477E8), ast_fake_sharing),
        ];
        for (slot, fake) in slots {
            hook(slot, fake, prev_cell(fake));
        }
    }

    /// Event bits 0-3; an inactive frame; the animation starting on this
    /// frame or not; with and without root motion and the drift (start
    /// gate) flag, with the real `Rider_GetGateChargeAngle` /
    /// `Rider_GetStartPoseAngle` / `BE_GetRiderSizeScale`; the heading
    /// flag changing (to set or clear) or not; start frames with the top
    /// bit set (unsigned conversion); the board-root velocity flag;
    /// `frameDelta` positive or not; every combination of the model-event
    /// queries. The animation callees are logged fakes (one may unwind);
    /// the script object's lookup runs for real.
    #[test]
    fn rider_apply_script_track_matches() {
        ast_install_fakes();
        covered(
            ee::s00::f_00124788__generated,
            rider_apply_script_track,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0012_4788, 30);
                c.pc = 0x0012_4788;
                c.bus.write32(AST_CTRL, AST_MAGIC);
                const RIDER: u32 = DATA + 0x4_0000;
                const SOBJ: u32 = DATA + 0x4_2300;
                const TABLE: u32 = DATA + 0x4_2400;
                const MODEL: u32 = DATA + 0x4_2600;
                const ANIM: u32 = DATA + 0x4_2800;
                const LPOS: u32 = DATA + 0x4_3000;
                const LROT: u32 = DATA + 0x4_3100;
                const BONES: u32 = DATA + 0x4_3200;
                const FI: u32 = DATA + 0x4_4000;
                const INFO: u32 = DATA + 0x4_4100;
                const FLAGS: u32 = DATA + 0x4_4200;
                let vec = |c: &mut Cpu, r: &mut Rng, at: u32, k: f32| {
                    put_vec(c, at, [r.f32(k), r.f32(k), r.f32(k), r.f32(k)]);
                };

                // .lit4 constants, the pivot, BE_GetRiderSizeScale's result.
                let gp_lits = [
                    (AST_GP_RATE_STEP, 1.0f32 / 60.0),
                    (AST_GP_PI, std::f32::consts::PI),
                    (AST_GP_START_STEP, 1.0 / 60.0),
                    (AST_GP_DEG2RAD, std::f32::consts::PI / 180.0),
                    (0x6df0, 1.0),
                ];
                for (off, v) in gp_lits {
                    c.bus.write32(GAME_GP - off, v.to_bits());
                }
                vec(c, &mut r, AST_PIVOT_BASE - 0xed0, 4.0);

                // The fakes' outputs and results.
                for i in 0..2 {
                    vec(c, &mut r, AST_XFORM_TEMPLATE + 0x10 * i, 2.0);
                }
                vec(c, &mut r, AST_EULER_TEMPLATE, 1.0);
                c.bus.write32(AST_PENDING_BITS, r.next_u32());
                c.bus.write32(AST_FIRED_BITS, r.next_u32());

                // The rider.
                for o in [
                    AST_POS,
                    AST_ROT,
                    AST_FRONT,
                    AST_VEL,
                    AST_KNEE_SLEW,
                    AST_TRACK_POS,
                    AST_TRACK_ANG,
                ] {
                    vec(c, &mut r, RIDER + o, 8.0);
                }
                for i in 0..4 {
                    vec(c, &mut r, RIDER + AST_SCRIPT_MATRIX + 0x10 * i, 1.5);
                }
                c.bus.write32(RIDER + AST_FLIPPED, r.next_u32() % 2);
                c.bus.write32(RIDER + AST_SCRIPT_OBJ + 0x1c, SOBJ);
                c.bus.write32(RIDER + AST_ANIM_MODEL, MODEL);
                c.bus.write32(RIDER + AST_RIDER_ANIM, ANIM);
                c.bus.write32(RIDER + AST_BONE_HIPS, r.next_u32() % 4);
                c.bus.write32(RIDER + AST_BONE_BOARD_ROOT, r.next_u32() % 4);
                c.bus.write32(RIDER + 0x86c, r.next_u32() % 4);
                c.bus.write32(RIDER + AST_BONE_VELOCITY, r.next_u32() % 2);
                // The script object (anim name -> id, source) and anim model.
                c.bus.write32(SOBJ + 4, r.next_u32());
                c.bus.write32(SOBJ + 0x10, TABLE);
                for i in 0..8 {
                    c.bus.write32(TABLE + 4 * i, r.next_u32());
                }
                put_words(c, MODEL + 0x24, &[LPOS, LROT, BONES]);
                vec(c, &mut r, MODEL + 0x140, 2.0);
                for i in 0..4 {
                    vec(c, &mut r, LPOS + 0x10 * i, 3.0);
                    vec(c, &mut r, LROT + 0x10 * i, 1.0);
                    vec(c, &mut r, BONES + 0x20 * i, 6.0);
                    vec(c, &mut r, BONES + 0x20 * i + 0x10, 1.0);
                }
                vec(c, &mut r, ANIM + 0x10, 1.0);

                // The script frame and its animation info.
                let active = if r.next_u32() % 5 == 0 { 0 } else { 1 };
                c.bus.write32(FI, active);
                c.bus.write32(FI + 4, INFO);
                for o in [0x18u32, 0x1c, 0x20] {
                    c.bus.write32(FI + o, r.f32(50.0));
                }
                for o in [0x24u32, 0x28, 0x2c] {
                    c.bus.write32(FI + o, r.f32(30.0));
                }
                let frame = r.next_u32() % 200;
                c.bus.write32(INFO, r.next_u32() % 8);
                c.bus.write32(INFO + 4, frame);
                c.bus.write32(INFO + 0x10, r.f32(60.0));
                c.bus.write32(INFO + 0x14, r.f32(2.0));
                let start = match r.next_u32() % 4 {
                    0 => 0x8000_0000 | r.next_u32() % 300,
                    _ => r.next_u32() % 300,
                };
                c.bus.write32(INFO + 0x18, start);
                c.bus.write32(INFO + 0x1c, FLAGS);
                let byte = |r: &mut Rng| match r.next_u32() % 4 {
                    0 | 1 => 0u8,
                    2 => 1,
                    _ => 0xff,
                };
                for i in 0..3 {
                    let b = byte(&mut r);
                    c.bus.write8(FLAGS + i, b);
                }

                // Arguments.
                c.g[A0][0] = sx(RIDER);
                c.g[A1][0] = sx(FI);
                c.g[A2][0] = (seed % 4) as u64;
                let cur = if r.next_u32() % 5 == 0 {
                    frame + 1
                } else {
                    frame
                };
                c.f[12] = (cur as f32).to_bits();
                c.f[13] = [0.0f32, 1.0 / 60.0, 0.5, -1.0][(seed / 4 % 4) as usize].to_bits();
            },
        );
    }
}
