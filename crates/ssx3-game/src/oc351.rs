//! Staging module oc351 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc351`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
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
    0x0015_e668,
    game_camera_compute_view,
    "GameCamera_ComputeView",
)];

// -------------------------------------------------- GameCamera_ComputeView

/// `GameCamera` fields read and written by `GameCamera_ComputeView`.
/// The listener position (x, y, z) at +0x0..+0x8, clamped to
/// `[0, +0xc]` (x) and `[+0x10, +0x14]` (y, z).
pub(crate) const GCV_LISTEN: u32 = 0x0;
pub(crate) const GCV_LISTEN_X_MAX: u32 = 0xc;
pub(crate) const GCV_LISTEN_LO: u32 = 0x10;
pub(crate) const GCV_LISTEN_HI: u32 = 0x14;
/// The world painter (its camera block is at +6).
pub(crate) const GCV_PAINTER: u32 = 0x18;
/// The view: eye position (+0x20) and orientation quaternion (+0x30).
pub(crate) const GCV_EYE: u32 = 0x20;
pub(crate) const GCV_QUAT: u32 = 0x30;
/// The view matrix (4 quadwords) and the copy of its third column (+0x80).
pub(crate) const GCV_MATRIX: u32 = 0x40;
pub(crate) const GCV_FORWARD: u32 = 0x80;
/// The target controller (`+8` -> target list) and the current one.
pub(crate) const GCV_TARGET_CTRL: u32 = 0xa0;
pub(crate) const GCV_CURRENT_CTRL: u32 = 0xa8;
/// The chase algorithm, its look-at (+0xe0) and eye (+0x100) quadwords.
pub(crate) const GCV_CHASE: u32 = 0xc0;
pub(crate) const GCV_CHASE_LOOK_AT: u32 = 0xe0;
pub(crate) const GCV_CHASE_EYE: u32 = 0x100;

/// Target list node: target object, listener weight, view weight, next.
pub(crate) const GCV_NODE_OBJ: u32 = 0x0;
pub(crate) const GCV_NODE_LISTEN_W: u32 = 0x4;
pub(crate) const GCV_NODE_VIEW_W: u32 = 0xc;
pub(crate) const GCV_NODE_NEXT: u32 = 0x14;
/// Target object: position (+0x0), type (+0xc, 0x4a: a fixed view), vtable
/// (+0x10; slot +0x10 gets the view, slot +0x30 the look-at point).
pub(crate) const GCV_OBJ_TYPE: u32 = 0xc;
pub(crate) const GCV_OBJ_VTABLE: u32 = 0x10;
pub(crate) const GCV_VT_GET_VIEW: u32 = 0x10;
pub(crate) const GCV_VT_GET_LOOK_AT: u32 = 0x30;

/// Static data: the zero vector, the origin (the default eye), the identity
/// matrix; `.lit4`: the minimum view weight (`gp-0x6c70`) and the painter's
/// view argument (`gp-0x6c6c`); the clamped listener height (`gp+0xa78`).
pub(crate) const GCV_ZERO_VEC: u32 = 0x004f_f120;
pub(crate) const GCV_ORIGIN: u32 = 0x004f_f130;
/// (`s6 + 0x53a0` in the body.)
#[cfg(test)]
pub(crate) const GCV_IDENTITY_MATRIX: u32 = 0x004c_53a0;
pub(crate) const GCV_MIN_WEIGHT_GP: u32 = 0x6c70;
pub(crate) const GCV_PAINTER_ARG_GP: u32 = 0x6c6c;
pub(crate) const GCV_LISTEN_Z_GP: u32 = 0xa78;

/// `ChaseCameraAlgorithm_ComputeAngles(algo, snap)`.
pub(crate) const GCV_COMPUTE_ANGLES: u32 = 0x0016_6640;
/// `ChaseCameraAlgorithm_BuildOutputView(algo)`.
pub(crate) const GCV_BUILD_OUTPUT_VIEW: u32 = 0x0016_6f90;
/// `ChaseCameraAlgorithm_GetView(out, algo)`.
pub(crate) const GCV_GET_VIEW: u32 = 0x0016_73a0;
/// `WorldPainter_UpdateView(painter_cam, x, y, arg)`.
pub(crate) const GCV_PAINTER_UPDATE_VIEW: u32 = 0x002e_d490;
/// `WorldPainter_GetCamera08(painter_cam)` (f0).
pub(crate) const GCV_PAINTER_GET_08: u32 = 0x002e_e3b8;

/// The game camera's direct callees (through their override slots).
const GCV_AVOID_COLLISION: Callee = Callee {
    addr: 0x0015_ee00,
    f: ee::s01::GameCamera_AvoidCollision,
};
const GCV_UPDATE_SHAKE: Callee = Callee {
    addr: 0x0015_e460,
    f: ee::s01::f_0015e460,
};

/// `lw` + `.read_fpu` at `base + off`.
#[inline(always)]
fn gcv_f(c: &mut Cpu, base: u32, off: u32) -> FpuF32 {
    Addr(base.wrapping_add(off)).read_fpu(c)
}

/// The virtual call `obj->vtable[slot](out, obj + delta)` with the
/// registers in the order `GameCamera_ComputeView` loads them for the
/// target-object calls inside its blend loop (`a2 = obj`, `v0 = vtable`,
/// `a1 = delta`, `v1 = method`).
fn gcv_vcall_a2(c: &mut Cpu, node: u32, slot: u32, ret: u32) -> Flow {
    let obj = Addr(node).off(GCV_NODE_OBJ).lw(c);
    c.g[A2][0] = obj;
    let vt = Addr(obj as u32).off(GCV_OBJ_VTABLE).lw(c);
    c.g[V0][0] = vt;
    c.g[A1][0] = Addr(vt as u32).off(slot).lh(c);
    let method = Addr(vt as u32).off(slot + 4).lw(c);
    c.g[V1][0] = method;
    c.g[A1][0] = crate::abi::addu(c.g[A2][0], c.g[A1][0]);
    call_ptr(c, method as u32, ret)
}

/// `vadd.xyzw vf5, vf3(a), vf4(b)` through the stack temporaries:
/// `lqc2 vf3, a; lqc2 vf4, b; vadd; sqc2 vf5, tmp; lq v0, tmp`, then the
/// quadword stored to `dst1` and `dst2`.
fn gcv_vadd_to(c: &mut Cpu, a: u32, b: u32, tmp: u32, dst1: u32, dst2: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, tmp);
    c.lq(V0, tmp);
    c.sq(V0, dst1);
    c.sq(V0, dst2);
}

/// `acc = m0*v.x + m1*v.y + m2*v.z; vf[dst] = acc + m3*v.w` with the matrix
/// columns in vf4..vf7 and `v` in vf[v].
fn gcv_mat_vec(c: &mut Cpu, dst: u8, v: u8) {
    c.vu0().mula(0b1111, 4, Src::Bc(v, X));
    c.vu0().madda(0b1111, 5, Src::Bc(v, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(v, Z));
    c.vu0().madd(0b1111, dst, 7, Src::Bc(v, W));
}

/// `min(v, hi)` if `lo <= v`, else `lo`, as the original's
/// `c.le.s lo, v; mov.s d, lo; bc1f; min.s d, v, hi` (the `min.s` costs one
/// cycle).
#[inline(always)]
fn gcv_clamp(c: &mut Cpu, lo: FpuF32, v: FpuF32, hi: FpuF32) -> FpuF32 {
    let take = lo.le(v);
    crate::abi::set_fcc(c, take);
    if take {
        c.cycles += 1;
        v.min(hi)
    } else {
        lo
    }
}

/// `void GameCamera_ComputeView(GameCamera* cam)` (0x15e668): compute the
/// camera's view for this tick.
///
/// The view starts at the identity (`+0x20` = origin, `+0x30` = identity
/// quaternion). With a single target, or a target of type 0x4a, the
/// target's own view (vtable +0x10) is taken; if that controller is the
/// current one, the chase algorithm is seeded with the target's eye
/// (`+0x100`) and look-at point (vtable +0x30, `+0xe0`). Otherwise the eyes
/// and look-at points of all targets whose view weight is above the minimum
/// are blended by running weight into the chase algorithm, which then
/// snaps its angles and builds the view. For the current controller the
/// chase camera then avoids collisions, shakes, and builds the view again.
///
/// The view matrix (`+0x40`) is the identity times the rotation of the view
/// quaternion, translated by `-eye`; its third column is kept at `+0x80`.
/// The listener (`+0x0`) is the blend of the target positions by their
/// listener weights, clamped to the bounds at `+0xc..+0x14` after the
/// world painter got the eye (`WorldPainter_UpdateView`) and its height
/// limit (`WorldPainter_GetCamera08`, also stored at `gp+0xa78`).
pub fn game_camera_compute_view(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: the zero/identity view in the frame's view temporary.
        c.cycles += 36;
        let frame = Frame::open(c, 0x150);
        frame.save_fpr(c, 22, 0x130);
        frame.save_fpr(c, 21, 0x128);
        c.f[22] = 0;
        c.g[AT][0] = 0x3f80_0000;
        c.f[21] = 0x3f80_0000;
        let view = frame.at(c, 0x20);
        Addr(view).write_fpu(c, FpuF32::ZERO);
        Addr(view).off(0xc).write_fpu(c, FpuF32::ONE);
        Addr(view).off(0x4).write_fpu(c, FpuF32::ZERO);
        Addr(view).off(0x8).write_fpu(c, FpuF32::ZERO);
        frame.save(c, S1, 0x100);
        c.lq(V0, view);
        c.g[S1][0] = c.g[A0][0];
        let cam = r32(c, S1);
        frame.save(c, S2, 0xf0);
        frame.save(c, S0, 0x110);
        c.g[A0][0] = sx(GCV_ORIGIN);
        frame.save(c, S3, 0xe0);
        c.g[S2][0] = sx(view);
        frame.save(c, S4, 0xd0);
        frame.save(c, S5, 0xc0);
        frame.save(c, S6, 0xb0);
        frame.save_ra(c, 0xa0);
        frame.save_fpr(c, 24, 0x140);
        frame.save_fpr(c, 23, 0x138);
        frame.save_fpr(c, 20, 0x120);
        c.sq(V0, cam.wrapping_add(GCV_QUAT));
        let target_ctrl = Addr(cam).off(GCV_TARGET_CTRL).lw(c);
        c.g[A1][0] = target_ctrl;
        c.lq(V1, GCV_ORIGIN);
        c.sq(V1, cam.wrapping_add(GCV_EYE));
        let list = Addr(target_ctrl as u32).off(8).lw(c);
        c.g[A0][0] = list;
        let count = Addr(list as u32).off(4).lw(c);
        let first = Addr(list as u32).lw(c);
        c.g[V0][0] = count ^ 1;
        c.g[S0][0] = Addr(first as u32).lw(c);

        // A single target (or a fixed view): take its view.
        let single = if count == 1 {
            true
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, S0)).off(GCV_OBJ_TYPE).lw(c);
            c.g[V0][0] = 0x4a;
            c.g[A0][0] = 0x50_0000;
            c.g[V1][0] == 0x4a
        };

        if single {
            c.cycles += 16;
            let obj = r32(c, S0);
            let vt = Addr(obj).off(GCV_OBJ_VTABLE).lw(c);
            c.g[V1][0] = vt;
            let out = frame.at(c, 0x30);
            c.g[A0][0] = sx(out);
            c.g[S4][0] = sx(cam.wrapping_add(GCV_MATRIX));
            c.g[S6][0] = 0x4c_0000;
            c.g[A1][0] = Addr(vt as u32).off(GCV_VT_GET_VIEW).lh(c);
            c.g[S5][0] = sx(cam.wrapping_add(GCV_QUAT));
            let method = Addr(vt as u32).off(GCV_VT_GET_VIEW + 4).lw(c);
            c.g[V0][0] = method;
            c.g[A1][0] = crate::abi::addu(c.g[S0][0], c.g[A1][0]);
            call_ptr(c, method as u32, 0x0015_e72c)?;
            let cam = r32(c, S1);
            let (eye_at, quat_at) = (frame.at(c, 0x30), frame.at(c, 0x40));
            c.lq(A1, eye_at);
            c.lq(V0, quat_at);
            c.g[A0][0] = Addr(cam).off(GCV_TARGET_CTRL).lw(c);
            c.g[V1][0] = Addr(cam).off(GCV_CURRENT_CTRL).lw(c);
            c.sq(V0, cam.wrapping_add(GCV_QUAT));
            let current = c.g[A0][0] == c.g[V1][0];
            c.sq(A1, cam.wrapping_add(GCV_EYE));
            if !current {
                c.cycles += 1;
                c.g[V0][0] = sx(r32(c, S6).wrapping_add(0x53a0));
                return gcv_finish(c, frame);
            }
            // Seed the chase algorithm with the target's eye and look-at.
            c.cycles += 10;
            c.sq(A1, cam.wrapping_add(GCV_CHASE_EYE));
            c.g[A0][0] = c.g[S2][0];
            let vt = Addr(r32(c, S0)).off(GCV_OBJ_VTABLE).lw(c);
            c.g[V0][0] = vt;
            c.g[A1][0] = Addr(vt as u32).off(GCV_VT_GET_LOOK_AT).lh(c);
            let method = Addr(vt as u32).off(GCV_VT_GET_LOOK_AT + 4).lw(c);
            c.g[V1][0] = method;
            c.g[A1][0] = crate::abi::addu(c.g[S0][0], c.g[A1][0]);
            call_ptr(c, method as u32, 0x0015_e764)?;
            let look_at = frame.at(c, 0x20);
            c.lq(V0, look_at);
            let dst = r32(c, S1).wrapping_add(GCV_CHASE_LOOK_AT);
            c.sq(V0, dst);
        } else {
            // Blend the targets' eyes and look-at points into the chase
            // algorithm, starting from zero.
            c.cycles += 12;
            c.g[S3][0] = sx(cam.wrapping_add(GCV_CHASE));
            c.g[A0][0] = sx(GCV_ZERO_VEC);
            c.g[S4][0] = sx(cam.wrapping_add(GCV_MATRIX));
            c.lq(V1, GCV_ZERO_VEC);
            c.g[S6][0] = 0x4c_0000;
            c.sq(V1, cam.wrapping_add(GCV_CHASE_LOOK_AT));
            c.lq(V0, GCV_ZERO_VEC);
            c.sq(V0, cam.wrapping_add(GCV_CHASE_EYE));
            let list = Addr(r32(c, A1)).off(8).lw(c);
            c.g[V1][0] = list;
            c.g[S0][0] = Addr(list as u32).lw(c);
            c.g[S5][0] = sx(cam.wrapping_add(GCV_QUAT));
            if c.g[S0][0] != 0 {
                c.cycles += 4;
                let lit = r32(c, GP).wrapping_sub(GCV_MIN_WEIGHT_GP);
                c.f[24] = Addr(lit).read_u32(c);
                c.f[23] = c.f[21];
                let node = r32(c, S0);
                c.f[20] = gcv_f(c, node, GCV_NODE_VIEW_W).0;
                loop {
                    c.cycles += 4;
                    let min_w = FpuF32(c.f[24]);
                    let w = FpuF32(c.f[20]);
                    let take = min_w.lt(w);
                    crate::abi::set_fcc(c, take);
                    c.g[A0][0] = c.g[S2][0];
                    if take {
                        c.cycles += 66;
                        // Running weight: the share of this target.
                        let total = FpuF32(c.f[22]) + w;
                        c.f[22] = total.0;
                        let share = w.div(total);
                        c.f[20] = share.0;
                        let node = r32(c, S0);
                        gcv_vcall_a2(c, node, GCV_VT_GET_VIEW, 0x0015_e7e0)?;
                        let cam = r32(c, S1);
                        let (s00, s10, s20, s30) = (
                            frame.at(c, 0x0),
                            frame.at(c, 0x10),
                            frame.at(c, 0x20),
                            frame.at(c, 0x30),
                        );
                        let (s40, s50, s60, s70, s80, s90) = (
                            frame.at(c, 0x40),
                            frame.at(c, 0x50),
                            frame.at(c, 0x60),
                            frame.at(c, 0x70),
                            frame.at(c, 0x80),
                            frame.at(c, 0x90),
                        );
                        c.lq(V1, cam.wrapping_add(GCV_CHASE_EYE));
                        c.g[A0][0] = sx(s40);
                        c.lq(V0, s30);
                        let rest = FpuF32(c.f[23]) - FpuF32(c.f[20]);
                        c.f[21] = rest.0;
                        c.lq(A1, s20);
                        c.sq(V0, s10);
                        c.sq(V1, s30);
                        c.sq(A1, s00);
                        // eye = target eye * share + eye * (1 - share)
                        crate::game::camera::camera::vu_scale_to(c, 20, s20, s60, s50);
                        crate::game::camera::camera::vu_scale_to(c, 21, s30, s70, s60);
                        gcv_vadd_to(c, s50, s60, s70, cam.wrapping_add(GCV_CHASE_EYE), s40);
                        let node = r32(c, S0);
                        gcv_vcall_a2(c, node, GCV_VT_GET_LOOK_AT, 0x0015_e86c)?;
                        let cam = r32(c, S1);
                        c.lq(V0, cam.wrapping_add(GCV_CHASE_LOOK_AT));
                        c.sq(V0, s50);
                        // look-at = target look-at * share + look-at * (1 - share)
                        crate::game::camera::camera::vu_scale_to(c, 20, s40, s80, s70);
                        crate::game::camera::camera::vu_scale_to(c, 21, s50, s90, s80);
                        gcv_vadd_to(c, s70, s80, s90, cam.wrapping_add(GCV_CHASE_LOOK_AT), s60);
                    }
                    c.cycles += 3;
                    let next = Addr(r32(c, S0)).off(GCV_NODE_NEXT).lw(c);
                    c.g[S0][0] = next;
                    if next == 0 {
                        break;
                    }
                    c.f[20] = gcv_f(c, next as u32, GCV_NODE_VIEW_W).0;
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_e7b0);
                    }
                }
            }
            // Snap the chase angles and build its view.
            c.cycles += 16;
            c.g[AT][0] = 0x3f80_0000;
            c.f[0] = 0x3f80_0000;
            c.g[A1][0] = 1;
            c.g[A0][0] = c.g[S3][0];
            let cam = r32(c, S1);
            Addr(cam)
                .off(GCV_CHASE_LOOK_AT + 0xc)
                .write_fpu(c, FpuF32::ONE);
            Addr(cam).off(GCV_CHASE_EYE + 0xc).write_fpu(c, FpuF32::ONE);
            call_ptr(c, GCV_COMPUTE_ANGLES, 0x0015_e8f0)?;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, GCV_BUILD_OUTPUT_VIEW, 0x0015_e8f8)?;
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, GCV_GET_VIEW, 0x0015_e904)?;
            gcv_take_view(c, frame);
        }

        // The current controller: avoid collisions, shake, rebuild.
        c.cycles += 4;
        let cam = r32(c, S1);
        c.g[V1][0] = Addr(cam).off(GCV_TARGET_CTRL).lw(c);
        c.g[V0][0] = Addr(cam).off(GCV_CURRENT_CTRL).lw(c);
        let current = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = sx(r32(c, S6).wrapping_add(0x53a0));
        if !current {
            return gcv_finish(c, frame);
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = sx(cam.wrapping_add(GCV_CHASE));
        GCV_AVOID_COLLISION.call(c, 0x0015_e930)?;
        c.g[A0][0] = c.g[S1][0];
        GCV_UPDATE_SHAKE.call(c, 0x0015_e938)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, GCV_COMPUTE_ANGLES, 0x0015_e944)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, GCV_BUILD_OUTPUT_VIEW, 0x0015_e94c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, GCV_GET_VIEW, 0x0015_e958)?;
        gcv_take_view(c, frame);
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, S6).wrapping_add(0x53a0));
        gcv_finish(c, frame)
    })
}

/// The chase view (eye, quaternion) from the frame temporary into the
/// camera.
fn gcv_take_view(c: &mut Cpu, frame: Frame) {
    let (eye_at, quat_at) = (frame.at(c, 0x20), frame.at(c, 0x30));
    c.lq(V0, eye_at);
    c.lq(V1, quat_at);
    let cam = r32(c, S1);
    c.sq(V0, cam.wrapping_add(GCV_EYE));
    c.sq(V1, cam.wrapping_add(GCV_QUAT));
}

/// The rest of `GameCamera_ComputeView` from 0x15e96c (`v0` = the identity
/// matrix): the view matrix, the listener blend, the world painter, the
/// clamps, the epilogue.
fn gcv_finish(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 113;
    let ident = r32(c, V0);
    let cam = r32(c, S1);
    let mat = r32(c, S4);
    let quat = r32(c, S5);
    let tmp = r32(c, S2);
    // matrix = identity
    c.lqc2(1, ident);
    c.lqc2(2, ident.wrapping_add(0x10));
    c.lqc2(3, ident.wrapping_add(0x20));
    c.lqc2(4, ident.wrapping_add(0x30));
    c.sqc2(1, mat);
    c.sqc2(2, mat.wrapping_add(0x10));
    c.sqc2(3, mat.wrapping_add(0x20));
    c.sqc2(4, mat.wrapping_add(0x30));
    // The rotation matrix of the quaternion (vf10..vf12, vf0 as the last
    // row) into the frame temporary.
    c.lqc2(4, quat);
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
    c.sqc2(0, tmp.wrapping_add(0x30));
    c.sqc2(10, tmp);
    c.sqc2(11, tmp.wrapping_add(0x10));
    c.sqc2(12, tmp.wrapping_add(0x20));
    // matrix = matrix * rotation
    c.lqc2(4, mat);
    c.lqc2(5, mat.wrapping_add(0x10));
    c.lqc2(6, mat.wrapping_add(0x20));
    c.lqc2(7, mat.wrapping_add(0x30));
    c.lqc2(8, tmp);
    c.lqc2(9, tmp.wrapping_add(0x10));
    c.lqc2(10, tmp.wrapping_add(0x20));
    c.lqc2(11, tmp.wrapping_add(0x30));
    gcv_mat_vec(c, 12, 8);
    gcv_mat_vec(c, 13, 9);
    gcv_mat_vec(c, 14, 10);
    gcv_mat_vec(c, 15, 11);
    c.sqc2(12, mat);
    c.sqc2(13, mat.wrapping_add(0x10));
    c.sqc2(14, mat.wrapping_add(0x20));
    c.sqc2(15, mat.wrapping_add(0x30));
    // Translation: matrix row 3 = matrix * (-eye, 1).
    let ex = gcv_f(c, cam, GCV_EYE);
    let ey = gcv_f(c, cam, GCV_EYE + 4);
    let ez = gcv_f(c, cam, GCV_EYE + 8);
    c.f[0] = ex.0;
    c.f[1] = ey.0;
    c.f[3] = ez.0;
    c.f[4] = (-ex).0;
    c.f[5] = (-ey).0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[6] = 0x3f80_0000;
    c.f[2] = (-ez).0;
    let s = |c: &Cpu, off: u32| frame.at(c, off);
    let (s60, s70, s80) = (s(c, 0x60), s(c, 0x70), s(c, 0x80));
    Addr(s80).write_fpu(c, ex);
    Addr(s80 + 4).write_fpu(c, ey);
    Addr(s60).write_fpu(c, -ex);
    Addr(s60 + 4).write_fpu(c, -ey);
    Addr(s60 + 8).write_fpu(c, -ez);
    Addr(s80 + 8).write_fpu(c, ez);
    Addr(s70).write_fpu(c, -ex);
    Addr(s70 + 4).write_fpu(c, -ey);
    Addr(s70 + 8).write_fpu(c, -ez);
    Addr(s60 + 0xc).write_fpu(c, FpuF32::ONE);
    c.lqc2(4, mat);
    c.lqc2(5, mat.wrapping_add(0x10));
    c.lqc2(6, mat.wrapping_add(0x20));
    c.lqc2(7, mat.wrapping_add(0x30));
    c.lqc2(8, s60);
    gcv_mat_vec(c, 12, 8);
    c.sqc2(12, mat.wrapping_add(0x30));
    // The forward column (matrix x/y/z of column 2, w = 0) at +0x80.
    let fx = gcv_f(c, cam, 0x48);
    let fy = gcv_f(c, cam, 0x58);
    let fz = gcv_f(c, cam, 0x68);
    c.f[0] = fx.0;
    c.f[1] = fy.0;
    c.f[2] = fz.0;
    Addr(s60 + 0xc).sw(c, 0);
    Addr(s60).write_fpu(c, fx);
    Addr(s60 + 4).write_fpu(c, fy);
    Addr(s60 + 8).write_fpu(c, fz);
    // The listener: blend of the target positions by listener weight.
    Addr(cam).off(GCV_LISTEN).sw(c, 0);
    let ctrl = Addr(cam).off(GCV_TARGET_CTRL).lw(c);
    c.g[V1][0] = ctrl;
    c.lq(V0, s60);
    Addr(cam).off(GCV_LISTEN + 4).sw(c, 0);
    c.sq(V0, cam.wrapping_add(GCV_FORWARD));
    Addr(cam).off(GCV_LISTEN + 8).sw(c, 0);
    let list = Addr(ctrl as u32).off(8).lw(c);
    c.g[V0][0] = list;
    c.g[S0][0] = Addr(list as u32).lw(c);
    c.f[22] = Addr(s60 + 0xc).read_u32(c);
    if c.g[S0][0] != 0 {
        c.cycles += 4;
        c.f[7] = c.f[6];
        c.f[6] = c.f[22];
        let node = r32(c, S0);
        c.f[1] = gcv_f(c, node, GCV_NODE_LISTEN_W).0;
        loop {
            c.cycles += 4;
            let w = FpuF32(c.f[1]);
            let zero = w.eq_s(FpuF32(c.f[6]));
            crate::abi::set_fcc(c, zero);
            let node = r32(c, S0);
            if !zero {
                c.cycles += 25;
                let total = FpuF32(c.f[22]) + w;
                c.f[22] = total.0;
                let obj = Addr(node).lw(c);
                c.g[V0][0] = obj;
                let lx = gcv_f(c, cam, GCV_LISTEN);
                c.f[2] = lx.0;
                let px = gcv_f(c, obj as u32, 0);
                let share = w.div(total);
                c.f[1] = share.0;
                let ly = gcv_f(c, cam, GCV_LISTEN + 4);
                let lz = gcv_f(c, cam, GCV_LISTEN + 8);
                c.f[5] = lz.0;
                let rest = FpuF32(c.f[7]) - share;
                let (rx, ry, rz) = (rest * lx, rest * ly, rest * lz);
                c.f[2] = rx.0;
                c.f[4] = ry.0;
                c.f[3] = rz.0;
                let x = share * px + rx;
                Addr(cam).off(GCV_LISTEN).write_fpu(c, x);
                let obj = Addr(node).lw(c);
                let py = gcv_f(c, obj as u32, 4);
                let y = share * py + ry;
                Addr(cam).off(GCV_LISTEN + 4).write_fpu(c, y);
                let obj = Addr(node).lw(c);
                c.g[V0][0] = obj;
                let pz = gcv_f(c, obj as u32, 8);
                let z = share * pz + rz;
                c.f[0] = pz.0;
                c.f[1] = z.0;
                Addr(cam).off(GCV_LISTEN + 8).write_fpu(c, z);
            }
            c.g[S0][0] = Addr(node).off(GCV_NODE_NEXT).lw(c);
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                break;
            }
            let next = r32(c, S0);
            c.f[1] = gcv_f(c, next, GCV_NODE_LISTEN_W).0;
            if event_due(c) {
                deliver_event_here(c, 0x0015_eb40);
            }
        }
    }

    // The world painter gets the eye; its height limit caps the listener.
    c.cycles += 15;
    let cam = r32(c, S1);
    c.g[A0][0] = Addr(cam).off(GCV_PAINTER).lw(c);
    c.f[12] = gcv_f(c, cam, GCV_EYE).0;
    c.f[13] = gcv_f(c, cam, GCV_EYE + 4).0;
    let lit = r32(c, GP).wrapping_sub(GCV_PAINTER_ARG_GP);
    c.f[14] = Addr(lit).read_u32(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(6));
    call_ptr(c, GCV_PAINTER_UPDATE_VIEW, 0x0015_ebd4)?;
    let cam = r32(c, S1);
    c.g[A0][0] = Addr(cam).off(GCV_PAINTER).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(6));
    call_ptr(c, GCV_PAINTER_GET_08, 0x0015_ebe0)?;
    let cam = r32(c, S1);
    let top = FpuF32(c.f[0]);
    let lo = gcv_f(c, cam, GCV_LISTEN_LO);
    c.f[2] = lo.0;
    let z = gcv_f(c, cam, GCV_LISTEN + 8);
    c.f[1] = z.0;
    let z = gcv_clamp(c, lo, z, top);
    c.f[3] = z.0;

    c.cycles += 8;
    c.f[0] = 0;
    let x = gcv_f(c, cam, GCV_LISTEN);
    c.f[1] = x.0;
    Addr(cam).off(GCV_LISTEN + 8).write_fpu(c, z);
    let listen_z = r32(c, GP).wrapping_add(GCV_LISTEN_Z_GP);
    Addr(listen_z).write_fpu(c, z);
    let x_max = gcv_f(c, cam, GCV_LISTEN_X_MAX);
    c.f[0] = x_max.0;
    let x = gcv_clamp(c, FpuF32::ZERO, x, x_max);
    c.f[5] = x.0;

    c.cycles += 7;
    let y = gcv_f(c, cam, GCV_LISTEN + 4);
    c.f[1] = y.0;
    Addr(cam).off(GCV_LISTEN).write_fpu(c, x);
    let hi = gcv_f(c, cam, GCV_LISTEN_HI);
    c.f[0] = hi.0;
    let y = gcv_clamp(c, lo, y, hi);
    c.f[4] = y.0;

    c.cycles += 4;
    Addr(cam).off(GCV_LISTEN + 4).write_fpu(c, y);
    let z = gcv_clamp(c, lo, z, hi);
    c.f[1] = z.0;

    c.cycles += 16;
    Addr(cam).off(GCV_LISTEN + 8).write_fpu(c, z);
    frame.restore(c, S0, 0x110);
    frame.restore(c, S1, 0x100);
    frame.restore(c, S2, 0xf0);
    frame.restore(c, S3, 0xe0);
    frame.restore(c, S4, 0xd0);
    frame.restore(c, S5, 0xc0);
    frame.restore(c, S6, 0xb0);
    frame.restore_ra(c, 0xa0);
    frame.restore_fpr(c, 24, 0x140);
    frame.restore_fpr(c, 23, 0x138);
    frame.restore_fpr(c, 22, 0x130);
    frame.restore_fpr(c, 21, 0x128);
    frame.restore_fpr(c, 20, 0x120);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake, fall_back, hook, machine, FAKE};
    use crate::difftest::{fill, put_vec, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The gate of this test's callee fakes (a word in the test machine's
    /// RAM): with it off, the slots run what they held before.
    const GCV_CTRL: u32 = DATA + 0xf_ff00;
    const GCV_MAGIC: u32 = 0x6763_7631;

    macro_rules! gcv_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(GCV_CTRL) == GCV_MAGIC {
                    let ra = c.g[RA][0] as u32;
                    return fake(c, $addr, ra);
                }
                fall_back(c, &$prev, $gen)
            }
        };
    }

    gcv_gated!(
        gcv_fake_angles,
        GCV_PREV_ANGLES,
        GCV_COMPUTE_ANGLES,
        ee::s02::f_00166640__generated
    );
    gcv_gated!(
        gcv_fake_build,
        GCV_PREV_BUILD,
        GCV_BUILD_OUTPUT_VIEW,
        ee::s02::f_00166f90__generated
    );
    gcv_gated!(
        gcv_fake_get_view,
        GCV_PREV_GET_VIEW,
        GCV_GET_VIEW,
        ee::s02::f_001673a0__generated
    );
    gcv_gated!(
        gcv_fake_painter_view,
        GCV_PREV_PAINTER_VIEW,
        GCV_PAINTER_UPDATE_VIEW,
        ee::s09::f_002ed490__generated
    );
    gcv_gated!(
        gcv_fake_painter_08,
        GCV_PREV_PAINTER_08,
        GCV_PAINTER_GET_08,
        ee::s09::f_002ee3b8__generated
    );
    gcv_gated!(
        gcv_fake_avoid,
        GCV_PREV_AVOID,
        0x0015_ee00,
        ee::s01::f_0015ee00__generated
    );
    gcv_gated!(
        gcv_fake_shake,
        GCV_PREV_SHAKE,
        0x0015_e460,
        ee::s01::f_0015e460__generated
    );

    #[allow(unused_unsafe)]
    fn gcv_install_fakes() {
        use std::ptr::addr_of_mut;
        unsafe {
            hook(
                addr_of_mut!(ee::s02::OVERRIDE_00166640),
                gcv_fake_angles,
                &GCV_PREV_ANGLES,
            );
            hook(
                addr_of_mut!(ee::s02::OVERRIDE_00166F90),
                gcv_fake_build,
                &GCV_PREV_BUILD,
            );
            hook(
                addr_of_mut!(ee::s02::OVERRIDE_001673A0),
                gcv_fake_get_view,
                &GCV_PREV_GET_VIEW,
            );
            hook(
                addr_of_mut!(ee::s09::OVERRIDE_002ED490),
                gcv_fake_painter_view,
                &GCV_PREV_PAINTER_VIEW,
            );
            hook(
                addr_of_mut!(ee::s09::OVERRIDE_002EE3B8),
                gcv_fake_painter_08,
                &GCV_PREV_PAINTER_08,
            );
            hook(
                addr_of_mut!(ee::s01::OVERRIDE_0015EE00),
                gcv_fake_avoid,
                &GCV_PREV_AVOID,
            );
            hook(
                addr_of_mut!(ee::s01::OVERRIDE_0015E460),
                gcv_fake_shake,
                &GCV_PREV_SHAKE,
            );
        }
    }

    /// A weight: zero, negative, small or large.
    fn gcv_weight(r: &mut Rng) -> u32 {
        match r.next_u32() % 5 {
            0 => 0,
            1 => (-((r.next_u32() % 3 + 1) as f32) / 4.0).to_bits(),
            2 => r.edge_f32(),
            _ => r.f32(3.0),
        }
    }

    /// `GameCamera_ComputeView`: one target (count 1 or a type-0x4a first
    /// target) or a list of 0-4 targets with zero / negative / edge / normal
    /// view and listener weights (below and above the minimum weight);
    /// the target controller being the current one or not; listener bounds
    /// that clamp or not. The target objects' view methods and every direct
    /// callee (the chase algorithm, collision avoidance, shake, the world
    /// painter) are logged fakes (gated on this test's machine), one of which
    /// may unwind; the painter's height limit is the fake's random `f0`.
    #[test]
    fn game_camera_compute_view_matches() {
        gcv_install_fakes();
        covered(
            ee::s01::f_0015e668__generated,
            game_camera_compute_view,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_e668, 12);
                c.pc = 0x0015_e668;
                c.bus.write32(GCV_CTRL, GCV_MAGIC);
                // Static data and literals.
                for (at, n) in [(GCV_ZERO_VEC, 1), (GCV_ORIGIN, 1), (GCV_IDENTITY_MATRIX, 4)] {
                    for k in 0..n {
                        let v = [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(2.0)];
                        put_vec(c, at + 0x10 * k, v);
                    }
                }
                let min_w = if seed % 3 == 0 { 0 } else { r.f32(0.5) };
                c.bus.write32(GAME_GP - GCV_MIN_WEIGHT_GP, min_w);
                let painter_arg = r.f32(100.0);
                c.bus.write32(GAME_GP - GCV_PAINTER_ARG_GP, painter_arg);
                // The camera: random contents, bounds, the painter, controllers.
                let cam = DATA + 0x8000;
                fill(c, &mut r, cam, 0x120 / 4);
                let x_max = r.f32(200.0);
                let lo = r.f32(200.0);
                let hi = r.f32(200.0);
                c.bus.write32(cam + GCV_LISTEN_X_MAX, x_max);
                c.bus.write32(cam + GCV_LISTEN_LO, lo);
                c.bus.write32(cam + GCV_LISTEN_HI, hi);
                c.bus.write32(cam + GCV_PAINTER, DATA + 0x5000);
                let ctrl = DATA + 0x1000;
                let current = if seed % 3 == 1 { DATA + 0x1080 } else { ctrl };
                c.bus.write32(cam + GCV_TARGET_CTRL, ctrl);
                c.bus.write32(cam + GCV_CURRENT_CTRL, current);
                // The target list.
                let list = DATA + 0x1100;
                c.bus.write32(ctrl + 8, list);
                let n = if seed % 4 == 0 { 1 } else { r.next_u32() % 5 };
                let count = if seed % 4 == 0 || r.next_u32() % 4 == 0 {
                    n
                } else {
                    n + 2
                };
                c.bus.write32(list + 4, count);
                let vt = DATA + 0x1400;
                for (i, slot) in [GCV_VT_GET_VIEW, GCV_VT_GET_LOOK_AT]
                    .into_iter()
                    .enumerate()
                {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(vt + slot, delta as u16);
                    c.bus.write32(vt + slot + 4, FAKE + 0x10 * i as u32);
                }
                let mut next = 0;
                for i in (0..n).rev() {
                    let node = DATA + 0x1200 + 0x20 * i;
                    let obj = DATA + 0x1800 + 0x40 * i;
                    c.bus.write32(node + GCV_NODE_OBJ, obj);
                    let lw = gcv_weight(&mut r);
                    c.bus.write32(node + GCV_NODE_LISTEN_W, lw);
                    let vw = gcv_weight(&mut r);
                    c.bus.write32(node + GCV_NODE_VIEW_W, vw);
                    c.bus.write32(node + GCV_NODE_NEXT, next);
                    next = node;
                    for k in 0..3 {
                        let p = r.f32(300.0);
                        c.bus.write32(obj + 4 * k, p);
                    }
                    let ty = if r.next_u32() % 4 == 0 {
                        0x4a
                    } else {
                        r.next_u32() % 0x60
                    };
                    c.bus.write32(obj + GCV_OBJ_TYPE, ty);
                    c.bus.write32(obj + GCV_OBJ_VTABLE, vt);
                }
                c.bus.write32(list, next);
                c.g[A0][0] = sx(cam);
            },
        );
    }
}
