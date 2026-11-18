//! Staging module oc412 (wave wave218): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc412`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0027_d970,
    script_obj_cam_build_matrix_from_script,
    "ScriptObjCam_BuildMatrixFromScript",
)];

/// `base + off` of a guest register, as the compiled code forms addresses.
macro_rules! cam_ea {
    ($c:ident, $r:expr, $o:expr) => {
        ($c.g[$r][0] as u32).wrapping_add($o)
    };
}

macro_rules! cam_ea_sub {
    ($c:ident, $r:expr, $o:expr) => {
        ($c.g[$r][0] as u32).wrapping_sub($o)
    };
}

/// A guest register plus a (possibly negative) offset.
fn cam_at(c: &Cpu, (reg, off): (usize, i32)) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off as u32)
}

/// `dst = a * b` for 4x4 float matrices in guest memory (row vectors, VU0
/// `vmula/vmadda/vmadd` chain per row of `b`); `dst` may alias `a` (all
/// loads happen first).
fn cam_mat_mul(c: &mut Cpu, a: (usize, i32), b: (usize, i32), dst: (usize, i32)) {
    let (pa, pb, pd) = (cam_at(c, a), cam_at(c, b), cam_at(c, dst));
    for i in 0..4u32 {
        c.lqc2((4 + i) as usize, pa.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.lqc2((8 + i) as usize, pb.wrapping_add(0x10 * i));
    }
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.vu0().mula(0b1111, 4, Src::Bc(9, X));
    c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
    c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
    c.vu0().mula(0b1111, 4, Src::Bc(10, X));
    c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
    c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
    c.vu0().mula(0b1111, 4, Src::Bc(11, X));
    c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
    c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
    for i in 0..4u32 {
        c.sqc2((12 + i) as usize, pd.wrapping_add(0x10 * i));
    }
}

/// Axis-angle rotation matrix (the camera's per-axis rotation): with the
/// `Math_SinCos` results at `sin`/`cos` (f8 = sin, f11 = cos, f1 = 1 - cos)
/// and the axis `(x, y, z)` read from `k0` (x), `k + 4` (y), `k + 8` (z), it
/// writes the 4x4 matrix at `dst` (translation row zero, `f20` = 1.0 in the
/// corner). Leaves f0..f13 as the generated sequence does.
fn cam_rot_core(
    c: &mut Cpu,
    sin: (usize, i32),
    cos: (usize, i32),
    k: (usize, i32),
    k0: (usize, i32),
    dst: (usize, i32),
) {
    let (ps, pc, pk, pk0, pd) = (
        cam_at(c, sin),
        cam_at(c, cos),
        cam_at(c, k),
        cam_at(c, k0),
        cam_at(c, dst),
    );
    c.lwc1(11, pc);
    c.lwc1(3, pk.wrapping_add(0x8));
    c.sub_s(1, 20, 11);
    c.lwc1(0, pk0);
    c.lwc1(6, pk.wrapping_add(0x4));
    c.lwc1(8, ps);
    c.mul_s(2, 1, 3);
    c.mul_s(4, 1, 0);
    c.mul_s(1, 1, 6);
    c.mul_s(13, 8, 3);
    c.mul_s(12, 8, 0);
    c.mul_s(9, 2, 3);
    c.mul_s(10, 4, 3);
    c.mul_s(5, 2, 0);
    c.mul_s(7, 4, 0);
    c.mul_s(3, 1, 3);
    c.mul_s(0, 1, 0);
    c.mul_s(8, 8, 6);
    c.mul_s(2, 2, 6);
    c.mul_s(4, 4, 6);
    c.mul_s(1, 1, 6);
    c.add_s(5, 5, 8);
    c.sub_s(0, 0, 13);
    c.sub_s(2, 2, 12);
    c.add_s(9, 9, 11);
    c.add_s(7, 7, 11);
    c.add_s(4, 4, 13);
    c.sub_s(10, 10, 8);
    c.add_s(1, 1, 11);
    c.add_s(3, 3, 12);
    for off in [0x30, 0x34, 0x38, 0xc, 0x1c, 0x2c] {
        c.sw(pd.wrapping_add(off), 0u64);
    }
    c.swc1(20, pd.wrapping_add(0x3c));
    for (reg, off) in [
        (7, 0x0),
        (0, 0x4),
        (5, 0x8),
        (4, 0x10),
        (1, 0x14),
        (2, 0x18),
        (10, 0x20),
        (3, 0x24),
        (9, 0x28),
    ] {
        c.swc1(reg, pd.wrapping_add(off));
    }
}

/// `void ScriptObjCam_BuildMatrixFromScript(ScriptObjCam* cam, ScriptArg* arg)`
/// (0x27d970): builds the scripted camera's matrix (`cam + 0x30`, 4x4) and
/// its euler offsets (`cam + 0x70..0x7c`) from the script channel's params.
///
/// The channel id of `arg->script` selects the mode: 0 manual, 1 target,
/// 2 subject, 3 algorithmic (anything else leaves the matrix at the
/// identity copy and returns). Each mode reads its params block
/// (`script->params`), forms the offset vector, rotates by the three
/// per-axis angles (`Math_SinCos` + [`cam_rot_core`] + [`cam_mat_mul`]) and,
/// for the pose modes, multiplies in `ScriptEngine_GetTargetPose`'s result.
/// The block structure of the generated body (shared mode tails) is kept.
pub fn script_obj_cam_build_matrix_from_script(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0027_d970);
    guest(c, |c| {
        'b_27f93c: {
            'b_27f938: {
                'b_27f934: {
                    'b_27f538: {
                        'b_27f510: {
                            'b_27ec1c: {
                                'b_27e324: {
                                    c.cycles += 34;
                                    c.g[SP][0] = sx(cam_ea_sub!(c, SP, 0x250));
                                    c.g[V0][0] = 0x500000;
                                    c.sq(18, cam_ea!(c, SP, 0x210));
                                    c.sq(22, cam_ea!(c, SP, 0x1d0));
                                    c.g[S2][0] = sx(cam_ea_sub!(c, V0, 0xe60));
                                    c.sq(30, cam_ea!(c, SP, 0x1b0));
                                    c.sq(16, cam_ea!(c, SP, 0x230));
                                    c.g[FP][0] = c.g[A0][0];
                                    c.sq(17, cam_ea!(c, SP, 0x220));
                                    c.g[S6][0] = sx(cam_ea!(c, FP, 0x30));
                                    c.sq(19, cam_ea!(c, SP, 0x200));
                                    c.sq(20, cam_ea!(c, SP, 0x1f0));
                                    c.sq(21, cam_ea!(c, SP, 0x1e0));
                                    c.sd(cam_ea!(c, SP, 0x1a0), c.g[RA][0]);
                                    c.swc1(20, cam_ea!(c, SP, 0x240));
                                    c.sq(23, cam_ea!(c, SP, 0x1c0));
                                    c.g[S7][0] = c.lw(cam_ea!(c, A1, 0x8));
                                    c.lqc2(1, cam_ea!(c, S2, 0));
                                    c.lqc2(2, cam_ea!(c, S2, 0x10));
                                    c.lqc2(3, cam_ea!(c, S2, 0x20));
                                    c.lqc2(4, cam_ea!(c, S2, 0x30));
                                    c.sqc2(1, cam_ea!(c, S6, 0));
                                    c.sqc2(2, cam_ea!(c, S6, 0x10));
                                    c.sqc2(3, cam_ea!(c, S6, 0x20));
                                    c.sqc2(4, cam_ea!(c, S6, 0x30));
                                    c.sw(cam_ea!(c, FP, 0x70), 0u64);
                                    c.g[A0][0] = c.g[S7][0];
                                    c.sw(cam_ea!(c, FP, 0x74), 0u64);
                                    c.sw(cam_ea!(c, FP, 0x78), 0u64);
                                    call_ptr(c, 0x002743c8, 0x0027d9e8)?; // InSeq_ChannelGetId
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                    if t {
                                        break 'b_27e324;
                                    } else {
                                        'b_27da28: {
                                            'b_27da10: {
                                                c.cycles += 2;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[V0][0] = 0x2;
                                                    break 'b_27da10;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.lq(16, cam_ea!(c, SP, 0x230));
                                                    if t {
                                                        break 'b_27da28;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.lq(17, cam_ea!(c, SP, 0x220));
                                                        break 'b_27f93c;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_27ec1c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == c.g[V0][0];

                                                if t {
                                                    break 'b_27f538;
                                                } else {
                                                    c.cycles += 2;
                                                    c.lq(16, cam_ea!(c, SP, 0x230));
                                                    break 'b_27f938;
                                                }
                                            }
                                        }
                                        c.cycles += 575;
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 20);
                                        c.g[A0][0] = c.g[S7][0];
                                        call_ptr(c, 0x0027c098, 0x0027da38)?; // ScriptCam_GetManualParams
                                        c.g[S0][0] = c.g[V0][0];
                                        c.lwc1(2, cam_ea_sub!(c, GP, 0x479c));
                                        c.lwc1(1, cam_ea!(c, S0, 0x1c));
                                        c.g[S1][0] = sx(cam_ea!(c, SP, 0x10));
                                        c.mul_s(1, 1, 2);
                                        c.swc1(1, cam_ea!(c, FP, 0x70));
                                        c.lwc1(0, cam_ea!(c, S0, 0x18));
                                        c.mul_s(0, 0, 2);
                                        c.swc1(0, cam_ea!(c, FP, 0x74));
                                        c.lwc1(1, cam_ea!(c, S0, 0x14));
                                        c.mul_s(1, 1, 2);
                                        c.swc1(1, cam_ea!(c, FP, 0x78));
                                        c.lwc1(0, cam_ea!(c, S0, 0x8));
                                        c.lwc1(1, cam_ea!(c, S0, 0xc));
                                        c.lwc1(2, cam_ea!(c, S0, 0x10));
                                        c.swc1(0, cam_ea!(c, SP, 0));
                                        c.swc1(1, cam_ea!(c, SP, 0x4));
                                        c.swc1(2, cam_ea!(c, SP, 0x8));
                                        c.swc1(20, cam_ea!(c, SP, 0xc));
                                        c.lqc2(1, cam_ea!(c, S2, 0));
                                        c.lqc2(2, cam_ea!(c, S2, 0x10));
                                        c.lqc2(3, cam_ea!(c, S2, 0x20));
                                        c.lqc2(4, cam_ea!(c, S2, 0x30));
                                        c.sqc2(1, cam_ea!(c, S1, 0));
                                        c.sqc2(2, cam_ea!(c, S1, 0x10));
                                        c.sqc2(3, cam_ea!(c, S1, 0x20));
                                        c.sqc2(4, cam_ea!(c, S1, 0x30));
                                        c.lqc2(4, cam_ea!(c, S1, 0));
                                        c.lqc2(5, cam_ea!(c, S1, 0x10));
                                        c.lqc2(6, cam_ea!(c, S1, 0x20));
                                        c.lqc2(7, cam_ea!(c, S1, 0x30));
                                        c.lqc2(8, cam_ea!(c, SP, 0));
                                        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                                        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                                        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                                        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                                        c.sqc2(12, cam_ea!(c, S1, 0x30));
                                        c.lwc1(12, cam_ea!(c, FP, 0x78));
                                        c.g[V0][0] = 0x500000;
                                        c.g[S5][0] = sx(cam_ea_sub!(c, V0, 0xea0));
                                        c.g[A0][0] = sx(cam_ea!(c, SP, 0xd0));
                                        c.neg_s(12, 12);
                                        c.g[A1][0] = sx(cam_ea!(c, SP, 0xd4));
                                        call_ptr(c, 0x0031be50, 0x0027dae8)?; // Math_SinCos
                                        c.g[V1][0] = 0x500000;
                                        c.g[S2][0] = sx(cam_ea!(c, SP, 0x50));
                                        cam_rot_core(
                                            c,
                                            (SP, 0xd0),
                                            (SP, 0xd4),
                                            (S5, 0x0),
                                            (V1, -0xea0),
                                            (SP, 0x50),
                                        );
                                        cam_mat_mul(c, (S1, 0x0), (S2, 0x0), (S1, 0x0));
                                        c.lwc1(12, cam_ea!(c, FP, 0x74));
                                        c.g[A0][0] = sx(cam_ea!(c, SP, 0xd8));
                                        c.g[A1][0] = sx(cam_ea!(c, SP, 0xdc));
                                        call_ptr(c, 0x0031be50, 0x0027dc28)?; // Math_SinCos
                                        c.g[V0][0] = 0x500000;
                                        c.g[S4][0] = sx(cam_ea_sub!(c, V0, 0xeb0));
                                        cam_rot_core(
                                            c,
                                            (SP, 0xd8),
                                            (SP, 0xdc),
                                            (S4, 0x0),
                                            (V0, -0xeb0),
                                            (SP, 0x50),
                                        );
                                        cam_mat_mul(c, (S1, 0x0), (S2, 0x0), (S1, 0x0));
                                        c.lwc1(12, cam_ea!(c, FP, 0x70));
                                        c.g[A0][0] = sx(cam_ea!(c, SP, 0xe0));
                                        c.g[A1][0] = sx(cam_ea!(c, SP, 0xe4));
                                        call_ptr(c, 0x0031be50, 0x0027dd68)?; // Math_SinCos
                                        c.g[V1][0] = 0x500000;
                                        c.g[S3][0] = sx(cam_ea_sub!(c, V1, 0xec0));
                                        cam_rot_core(
                                            c,
                                            (SP, 0xe0),
                                            (SP, 0xe4),
                                            (S3, 0x0),
                                            (V1, -0xec0),
                                            (SP, 0x50),
                                        );
                                        cam_mat_mul(c, (S1, 0x0), (S2, 0x0), (S1, 0x0));
                                        c.g[A1][0] = c.lb(cam_ea!(c, S0, 0x20));
                                        c.g[A3][0] = c.g[S2][0];
                                        c.g[A2][0] = c.lw(cam_ea!(c, S7, 0x8));
                                        c.g[T0][0] = sx(cam_ea!(c, SP, 0xe8));
                                        c.g[A0][0] = c.lw(cam_ea_sub!(c, GP, 0x84c));
                                        c.g[T1][0] = sx(cam_ea!(c, SP, 0xec));
                                        c.g[T2][0] = sx(cam_ea!(c, SP, 0xf0));
                                        call_ptr(c, 0x0027a0d8, 0x0027deb8)?; // ScriptEngine_GetTargetPose
                                        c.lqc2(4, cam_ea!(c, S6, 0));
                                        c.lqc2(5, cam_ea!(c, S6, 0x10));
                                        c.lqc2(6, cam_ea!(c, S6, 0x20));
                                        c.lqc2(7, cam_ea!(c, S6, 0x30));
                                        c.lqc2(8, cam_ea!(c, SP, 0x50));
                                        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                                        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                                        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                                        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                                        c.sqc2(12, cam_ea!(c, S6, 0x30));
                                        c.lwc1(12, cam_ea!(c, SP, 0xf0));
                                        c.g[A0][0] = sx(cam_ea!(c, SP, 0xf4));
                                        c.g[A1][0] = sx(cam_ea!(c, SP, 0xf8));
                                        c.g[S0][0] = c.g[S6][0];
                                        c.neg_s(12, 12);
                                        call_ptr(c, 0x0031be50, 0x0027def8)?; // Math_SinCos
                                        c.g[V1][0] = 0x500000;
                                        c.g[V0][0] = sx(cam_ea!(c, SP, 0x90));
                                        cam_rot_core(
                                            c,
                                            (SP, 0xf4),
                                            (SP, 0xf8),
                                            (S5, 0x0),
                                            (V1, -0xea0),
                                            (SP, 0x90),
                                        );
                                        cam_mat_mul(c, (S0, 0x0), (V0, 0x0), (S0, 0x0));
                                        c.lwc1(12, cam_ea!(c, SP, 0xec));
                                        c.g[A0][0] = sx(cam_ea!(c, SP, 0xfc));
                                        c.g[A1][0] = sx(cam_ea!(c, SP, 0x100));
                                        call_ptr(c, 0x0031be50, 0x0027e038)?; // Math_SinCos
                                        c.g[V0][0] = 0x500000;
                                        c.g[S2][0] = sx(cam_ea!(c, SP, 0x60));
                                        cam_rot_core(
                                            c,
                                            (SP, 0xfc),
                                            (SP, 0x100),
                                            (S4, 0x0),
                                            (V0, -0xeb0),
                                            (SP, 0x60),
                                        );
                                        cam_mat_mul(c, (S0, 0x0), (S2, 0x0), (S0, 0x0));
                                        c.lwc1(12, cam_ea!(c, SP, 0xe8));
                                        c.g[A0][0] = sx(cam_ea!(c, SP, 0x104));
                                        c.g[A1][0] = sx(cam_ea!(c, SP, 0x108));
                                        call_ptr(c, 0x0031be50, 0x0027e178)?; // Math_SinCos
                                        c.g[V1][0] = 0x500000;
                                        cam_rot_core(
                                            c,
                                            (SP, 0x104),
                                            (SP, 0x108),
                                            (S3, 0x0),
                                            (V1, -0xec0),
                                            (SP, 0x60),
                                        );
                                        cam_mat_mul(c, (S0, 0x0), (S2, 0x0), (S0, 0x0));
                                        cam_mat_mul(c, (S0, 0x0), (S1, 0x0), (S0, 0x0));
                                        c.lwc1(3, cam_ea!(c, SP, 0xe8));
                                        c.lwc1(4, cam_ea!(c, SP, 0xec));
                                        c.lwc1(5, cam_ea!(c, SP, 0xf0));
                                        break 'b_27f510;
                                    }
                                }
                                c.cycles += 574;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 20);
                                c.g[A0][0] = c.g[S7][0];
                                call_ptr(c, 0x0027c2a8, 0x0027e334)?; // ScriptCam_GetTargetParams
                                c.g[S0][0] = c.g[V0][0];
                                c.lwc1(2, cam_ea_sub!(c, GP, 0x4798));
                                c.lwc1(1, cam_ea!(c, S0, 0x1c));
                                c.g[S1][0] = sx(cam_ea!(c, SP, 0x10));
                                c.mul_s(1, 1, 2);
                                c.swc1(1, cam_ea!(c, FP, 0x70));
                                c.lwc1(0, cam_ea!(c, S0, 0x18));
                                c.mul_s(0, 0, 2);
                                c.swc1(0, cam_ea!(c, FP, 0x74));
                                c.lwc1(1, cam_ea!(c, S0, 0x14));
                                c.mul_s(1, 1, 2);
                                c.swc1(1, cam_ea!(c, FP, 0x78));
                                c.lwc1(0, cam_ea!(c, S0, 0x8));
                                c.lwc1(1, cam_ea!(c, S0, 0xc));
                                c.lwc1(2, cam_ea!(c, S0, 0x10));
                                c.swc1(0, cam_ea!(c, SP, 0));
                                c.swc1(1, cam_ea!(c, SP, 0x4));
                                c.swc1(2, cam_ea!(c, SP, 0x8));
                                c.swc1(20, cam_ea!(c, SP, 0xc));
                                c.lqc2(1, cam_ea!(c, S2, 0));
                                c.lqc2(2, cam_ea!(c, S2, 0x10));
                                c.lqc2(3, cam_ea!(c, S2, 0x20));
                                c.lqc2(4, cam_ea!(c, S2, 0x30));
                                c.sqc2(1, cam_ea!(c, S1, 0));
                                c.sqc2(2, cam_ea!(c, S1, 0x10));
                                c.sqc2(3, cam_ea!(c, S1, 0x20));
                                c.sqc2(4, cam_ea!(c, S1, 0x30));
                                c.lqc2(4, cam_ea!(c, S1, 0));
                                c.lqc2(5, cam_ea!(c, S1, 0x10));
                                c.lqc2(6, cam_ea!(c, S1, 0x20));
                                c.lqc2(7, cam_ea!(c, S1, 0x30));
                                c.lqc2(8, cam_ea!(c, SP, 0));
                                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                                c.sqc2(12, cam_ea!(c, S1, 0x30));
                                c.lwc1(12, cam_ea!(c, FP, 0x78));
                                c.g[V0][0] = 0x500000;
                                c.g[S5][0] = sx(cam_ea_sub!(c, V0, 0xea0));
                                c.g[A0][0] = sx(cam_ea!(c, SP, 0x10c));
                                c.neg_s(12, 12);
                                c.g[A1][0] = sx(cam_ea!(c, SP, 0x110));
                                call_ptr(c, 0x0031be50, 0x0027e3e4)?; // Math_SinCos
                                c.g[V1][0] = 0x500000;
                                c.g[S2][0] = sx(cam_ea!(c, SP, 0x50));
                                cam_rot_core(
                                    c,
                                    (SP, 0x10c),
                                    (SP, 0x110),
                                    (S5, 0x0),
                                    (V1, -0xea0),
                                    (SP, 0x50),
                                );
                                cam_mat_mul(c, (S1, 0x0), (S2, 0x0), (S1, 0x0));
                                c.lwc1(12, cam_ea!(c, FP, 0x74));
                                c.g[A0][0] = sx(cam_ea!(c, SP, 0x114));
                                c.g[A1][0] = sx(cam_ea!(c, SP, 0x118));
                                call_ptr(c, 0x0031be50, 0x0027e524)?; // Math_SinCos
                                c.g[V0][0] = 0x500000;
                                c.g[S4][0] = sx(cam_ea_sub!(c, V0, 0xeb0));
                                cam_rot_core(
                                    c,
                                    (SP, 0x114),
                                    (SP, 0x118),
                                    (S4, 0x0),
                                    (V0, -0xeb0),
                                    (SP, 0x50),
                                );
                                cam_mat_mul(c, (S1, 0x0), (S2, 0x0), (S1, 0x0));
                                c.lwc1(12, cam_ea!(c, FP, 0x70));
                                c.g[A0][0] = sx(cam_ea!(c, SP, 0x11c));
                                c.g[A1][0] = sx(cam_ea!(c, SP, 0x120));
                                call_ptr(c, 0x0031be50, 0x0027e664)?; // Math_SinCos
                                c.g[V1][0] = 0x500000;
                                c.g[S3][0] = sx(cam_ea_sub!(c, V1, 0xec0));
                                cam_rot_core(
                                    c,
                                    (SP, 0x11c),
                                    (SP, 0x120),
                                    (S3, 0x0),
                                    (V1, -0xec0),
                                    (SP, 0x50),
                                );
                                cam_mat_mul(c, (S1, 0x0), (S2, 0x0), (S1, 0x0));
                                c.g[A1][0] = c.lb(cam_ea!(c, S0, 0x20));
                                c.g[A3][0] = c.g[S2][0];
                                c.g[A2][0] = c.lw(cam_ea!(c, S7, 0x8));
                                c.g[T0][0] = sx(cam_ea!(c, SP, 0x124));
                                c.g[A0][0] = c.lw(cam_ea_sub!(c, GP, 0x84c));
                                c.g[T1][0] = sx(cam_ea!(c, SP, 0x128));
                                c.g[T2][0] = sx(cam_ea!(c, SP, 0x12c));
                                call_ptr(c, 0x0027a0d8, 0x0027e7b4)?; // ScriptEngine_GetTargetPose
                                c.lqc2(4, cam_ea!(c, S6, 0));
                                c.lqc2(5, cam_ea!(c, S6, 0x10));
                                c.lqc2(6, cam_ea!(c, S6, 0x20));
                                c.lqc2(7, cam_ea!(c, S6, 0x30));
                                c.lqc2(8, cam_ea!(c, SP, 0x50));
                                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                                c.sqc2(12, cam_ea!(c, S6, 0x30));
                                c.lwc1(12, cam_ea!(c, SP, 0x12c));
                                c.g[A0][0] = sx(cam_ea!(c, SP, 0x130));
                                c.g[A1][0] = sx(cam_ea!(c, SP, 0x134));
                                c.g[S0][0] = c.g[S6][0];
                                c.neg_s(12, 12);
                                call_ptr(c, 0x0031be50, 0x0027e7f4)?; // Math_SinCos
                                c.g[V0][0] = 0x500000;
                                c.g[S2][0] = sx(cam_ea!(c, SP, 0x60));
                                cam_rot_core(
                                    c,
                                    (SP, 0x130),
                                    (SP, 0x134),
                                    (S5, 0x0),
                                    (V0, -0xea0),
                                    (SP, 0x60),
                                );
                                cam_mat_mul(c, (S0, 0x0), (S2, 0x0), (S0, 0x0));
                                c.lwc1(12, cam_ea!(c, SP, 0x128));
                                c.g[A0][0] = sx(cam_ea!(c, SP, 0x138));
                                c.g[A1][0] = sx(cam_ea!(c, SP, 0x13c));
                                call_ptr(c, 0x0031be50, 0x0027e934)?; // Math_SinCos
                                c.g[V1][0] = 0x500000;
                                cam_rot_core(
                                    c,
                                    (SP, 0x138),
                                    (SP, 0x13c),
                                    (S4, 0x0),
                                    (V1, -0xeb0),
                                    (SP, 0x60),
                                );
                                cam_mat_mul(c, (S0, 0x0), (S2, 0x0), (S0, 0x0));
                                c.lwc1(12, cam_ea!(c, SP, 0x124));
                                c.g[A0][0] = sx(cam_ea!(c, SP, 0x140));
                                c.g[A1][0] = sx(cam_ea!(c, SP, 0x144));
                                call_ptr(c, 0x0031be50, 0x0027ea70)?; // Math_SinCos
                                c.g[V0][0] = 0x500000;
                                cam_rot_core(
                                    c,
                                    (SP, 0x140),
                                    (SP, 0x144),
                                    (S3, 0x0),
                                    (V0, -0xec0),
                                    (SP, 0x60),
                                );
                                cam_mat_mul(c, (S0, 0x0), (S2, 0x0), (S0, 0x0));
                                cam_mat_mul(c, (S0, 0x0), (S1, 0x0), (S0, 0x0));
                                c.lwc1(3, cam_ea!(c, SP, 0x124));
                                c.lwc1(4, cam_ea!(c, SP, 0x128));
                                c.lwc1(5, cam_ea!(c, SP, 0x12c));
                                break 'b_27f510;
                            }
                            c.cycles += 573;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 20);
                            c.g[A0][0] = c.g[S7][0];
                            call_ptr(c, 0x0027c4b8, 0x0027ec2c)?; // ScriptCam_GetSubjectParams
                            c.g[S0][0] = c.g[V0][0];
                            c.lwc1(2, cam_ea_sub!(c, GP, 0x4794));
                            c.lwc1(1, cam_ea!(c, S0, 0x1c));
                            c.g[S1][0] = sx(cam_ea!(c, SP, 0x10));
                            c.mul_s(1, 1, 2);
                            c.swc1(1, cam_ea!(c, FP, 0x70));
                            c.lwc1(0, cam_ea!(c, S0, 0x18));
                            c.mul_s(0, 0, 2);
                            c.swc1(0, cam_ea!(c, FP, 0x74));
                            c.lwc1(1, cam_ea!(c, S0, 0x14));
                            c.mul_s(1, 1, 2);
                            c.swc1(1, cam_ea!(c, FP, 0x78));
                            c.lwc1(0, cam_ea!(c, S0, 0x8));
                            c.lwc1(1, cam_ea!(c, S0, 0xc));
                            c.lwc1(2, cam_ea!(c, S0, 0x10));
                            c.swc1(0, cam_ea!(c, SP, 0));
                            c.swc1(1, cam_ea!(c, SP, 0x4));
                            c.swc1(2, cam_ea!(c, SP, 0x8));
                            c.swc1(20, cam_ea!(c, SP, 0xc));
                            c.lqc2(1, cam_ea!(c, S2, 0));
                            c.lqc2(2, cam_ea!(c, S2, 0x10));
                            c.lqc2(3, cam_ea!(c, S2, 0x20));
                            c.lqc2(4, cam_ea!(c, S2, 0x30));
                            c.sqc2(1, cam_ea!(c, S1, 0));
                            c.sqc2(2, cam_ea!(c, S1, 0x10));
                            c.sqc2(3, cam_ea!(c, S1, 0x20));
                            c.sqc2(4, cam_ea!(c, S1, 0x30));
                            c.lqc2(4, cam_ea!(c, S1, 0));
                            c.lqc2(5, cam_ea!(c, S1, 0x10));
                            c.lqc2(6, cam_ea!(c, S1, 0x20));
                            c.lqc2(7, cam_ea!(c, S1, 0x30));
                            c.lqc2(8, cam_ea!(c, SP, 0));
                            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                            c.sqc2(12, cam_ea!(c, S1, 0x30));
                            c.lwc1(12, cam_ea!(c, FP, 0x78));
                            c.g[V1][0] = 0x500000;
                            c.g[S5][0] = sx(cam_ea_sub!(c, V1, 0xea0));
                            c.g[A0][0] = sx(cam_ea!(c, SP, 0x148));
                            c.neg_s(12, 12);
                            c.g[A1][0] = sx(cam_ea!(c, SP, 0x14c));
                            call_ptr(c, 0x0031be50, 0x0027ecdc)?; // Math_SinCos
                            c.g[V0][0] = 0x500000;
                            c.g[S2][0] = sx(cam_ea!(c, SP, 0x50));
                            cam_rot_core(
                                c,
                                (SP, 0x148),
                                (SP, 0x14c),
                                (S5, 0x0),
                                (V0, -0xea0),
                                (SP, 0x50),
                            );
                            cam_mat_mul(c, (S1, 0x0), (S2, 0x0), (S1, 0x0));
                            c.lwc1(12, cam_ea!(c, FP, 0x74));
                            c.g[A0][0] = sx(cam_ea!(c, SP, 0x150));
                            c.g[A1][0] = sx(cam_ea!(c, SP, 0x154));
                            call_ptr(c, 0x0031be50, 0x0027ee1c)?; // Math_SinCos
                            c.g[V1][0] = 0x500000;
                            c.g[S4][0] = sx(cam_ea_sub!(c, V1, 0xeb0));
                            cam_rot_core(
                                c,
                                (SP, 0x150),
                                (SP, 0x154),
                                (S4, 0x0),
                                (V1, -0xeb0),
                                (SP, 0x50),
                            );
                            cam_mat_mul(c, (S1, 0x0), (S2, 0x0), (S1, 0x0));
                            c.lwc1(12, cam_ea!(c, FP, 0x70));
                            c.g[A0][0] = sx(cam_ea!(c, SP, 0x158));
                            c.g[A1][0] = sx(cam_ea!(c, SP, 0x15c));
                            call_ptr(c, 0x0031be50, 0x0027ef5c)?; // Math_SinCos
                            c.g[V0][0] = 0x500000;
                            c.g[S3][0] = sx(cam_ea_sub!(c, V0, 0xec0));
                            cam_rot_core(
                                c,
                                (SP, 0x158),
                                (SP, 0x15c),
                                (S3, 0x0),
                                (V0, -0xec0),
                                (SP, 0x50),
                            );
                            cam_mat_mul(c, (S1, 0x0), (S2, 0x0), (S1, 0x0));
                            c.g[A1][0] = c.lb(cam_ea!(c, S0, 0x20));
                            c.g[A3][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw(cam_ea!(c, S7, 0x8));
                            c.g[T0][0] = sx(cam_ea!(c, SP, 0x160));
                            c.g[A0][0] = c.lw(cam_ea_sub!(c, GP, 0x84c));
                            c.g[T1][0] = sx(cam_ea!(c, SP, 0x164));
                            c.g[T2][0] = sx(cam_ea!(c, SP, 0x168));
                            call_ptr(c, 0x0027a0d8, 0x0027f0ac)?; // ScriptEngine_GetTargetPose
                            c.lqc2(4, cam_ea!(c, S6, 0));
                            c.lqc2(5, cam_ea!(c, S6, 0x10));
                            c.lqc2(6, cam_ea!(c, S6, 0x20));
                            c.lqc2(7, cam_ea!(c, S6, 0x30));
                            c.lqc2(8, cam_ea!(c, SP, 0x50));
                            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                            c.sqc2(12, cam_ea!(c, S6, 0x30));
                            c.lwc1(12, cam_ea!(c, SP, 0x168));
                            c.g[A0][0] = sx(cam_ea!(c, SP, 0x16c));
                            c.g[A1][0] = sx(cam_ea!(c, SP, 0x170));
                            c.g[S0][0] = c.g[S6][0];
                            c.neg_s(12, 12);
                            call_ptr(c, 0x0031be50, 0x0027f0ec)?; // Math_SinCos
                            c.g[V1][0] = 0x500000;
                            c.g[S2][0] = sx(cam_ea!(c, SP, 0x60));
                            cam_rot_core(
                                c,
                                (SP, 0x16c),
                                (SP, 0x170),
                                (S5, 0x0),
                                (V1, -0xea0),
                                (SP, 0x60),
                            );
                            cam_mat_mul(c, (S0, 0x0), (S2, 0x0), (S0, 0x0));
                            c.lwc1(12, cam_ea!(c, SP, 0x164));
                            c.g[A0][0] = sx(cam_ea!(c, SP, 0x174));
                            c.g[A1][0] = sx(cam_ea!(c, SP, 0x178));
                            call_ptr(c, 0x0031be50, 0x0027f22c)?; // Math_SinCos
                            c.g[V0][0] = 0x500000;
                            cam_rot_core(
                                c,
                                (SP, 0x174),
                                (SP, 0x178),
                                (S4, 0x0),
                                (V0, -0xeb0),
                                (SP, 0x60),
                            );
                            cam_mat_mul(c, (S0, 0x0), (S2, 0x0), (S0, 0x0));
                            c.lwc1(12, cam_ea!(c, SP, 0x160));
                            c.g[A0][0] = sx(cam_ea!(c, SP, 0x17c));
                            c.g[A1][0] = sx(cam_ea!(c, SP, 0x180));
                            call_ptr(c, 0x0031be50, 0x0027f368)?; // Math_SinCos
                            c.g[V1][0] = 0x500000;
                            cam_rot_core(
                                c,
                                (SP, 0x17c),
                                (SP, 0x180),
                                (S3, 0x0),
                                (V1, -0xec0),
                                (SP, 0x60),
                            );
                            cam_mat_mul(c, (S0, 0x0), (S2, 0x0), (S0, 0x0));
                            cam_mat_mul(c, (S0, 0x0), (S1, 0x0), (S0, 0x0));
                            c.lwc1(3, cam_ea!(c, SP, 0x160));
                            c.lwc1(4, cam_ea!(c, SP, 0x164));
                            c.lwc1(5, cam_ea!(c, SP, 0x168));
                            break 'b_27f510;
                        }
                        c.cycles += 10;
                        c.lwc1(0, cam_ea!(c, FP, 0x70));
                        c.lwc1(1, cam_ea!(c, FP, 0x74));
                        c.lwc1(2, cam_ea!(c, FP, 0x78));
                        c.add_s(0, 0, 3);
                        c.add_s(1, 1, 4);
                        c.add_s(2, 2, 5);
                        c.swc1(0, cam_ea!(c, FP, 0x70));
                        c.swc1(1, cam_ea!(c, FP, 0x74));
                        c.swc1(2, cam_ea!(c, FP, 0x78));
                        break 'b_27f934;
                    }
                    c.cycles += 222;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 20);
                    c.g[A0][0] = c.g[S7][0];
                    call_ptr(c, 0x0027c6a0, 0x0027f548)?; // ScriptCam_GetAlgorithmicParams
                    c.g[S1][0] = 0x500000;
                    c.lwc1(2, cam_ea_sub!(c, GP, 0x4790));
                    c.g[A0][0] = sx(cam_ea!(c, SP, 0x184));
                    c.lwc1(1, cam_ea!(c, V0, 0x10));
                    c.g[A1][0] = sx(cam_ea!(c, SP, 0x188));
                    c.g[S0][0] = sx(cam_ea_sub!(c, S1, 0xea0));
                    c.g[S2][0] = c.g[S6][0];
                    c.mul_s(1, 1, 2);
                    c.swc1(1, cam_ea!(c, FP, 0x70));
                    c.lwc1(0, cam_ea!(c, V0, 0xc));
                    c.mul_s(0, 0, 2);
                    c.swc1(0, cam_ea!(c, FP, 0x74));
                    c.lwc1(1, cam_ea!(c, V0, 0x8));
                    c.mul_s(1, 1, 2);
                    c.neg_s(12, 1);
                    c.swc1(1, cam_ea!(c, FP, 0x78));
                    call_ptr(c, 0x0031be50, 0x0027f58c)?; // Math_SinCos
                    cam_rot_core(
                        c,
                        (SP, 0x184),
                        (SP, 0x188),
                        (S0, 0x0),
                        (S1, -0xea0),
                        (SP, 0x0),
                    );
                    cam_mat_mul(c, (S2, 0x0), (SP, 0x0), (S2, 0x0));
                    c.lwc1(12, cam_ea!(c, FP, 0x74));
                    c.g[A0][0] = sx(cam_ea!(c, SP, 0x18c));
                    c.g[A1][0] = sx(cam_ea!(c, SP, 0x190));
                    c.g[S0][0] = 0x500000;
                    call_ptr(c, 0x0031be50, 0x0027f6c8)?; // Math_SinCos
                    c.g[V0][0] = sx(cam_ea_sub!(c, S0, 0xeb0));
                    cam_rot_core(
                        c,
                        (SP, 0x18c),
                        (SP, 0x190),
                        (V0, 0x0),
                        (S0, -0xeb0),
                        (SP, 0x0),
                    );
                    cam_mat_mul(c, (S2, 0x0), (SP, 0x0), (S2, 0x0));
                    c.lwc1(12, cam_ea!(c, FP, 0x70));
                    c.g[A0][0] = sx(cam_ea!(c, SP, 0x194));
                    c.g[A1][0] = sx(cam_ea!(c, SP, 0x198));
                    c.g[S0][0] = 0x500000;
                    call_ptr(c, 0x0031be50, 0x0027f808)?; // Math_SinCos
                    c.lwc1(11, cam_ea!(c, SP, 0x198));
                    c.g[V0][0] = sx(cam_ea_sub!(c, S0, 0xec0));
                    c.lwc1(3, cam_ea!(c, V0, 0x8));
                    c.sub_s(1, 20, 11);
                    c.lwc1(0, cam_ea_sub!(c, S0, 0xec0));
                    c.lwc1(6, cam_ea!(c, V0, 0x4));
                    c.lwc1(8, cam_ea!(c, SP, 0x194));
                    c.mul_s(2, 1, 3);
                    c.swc1(20, cam_ea!(c, SP, 0x3c));
                    c.mul_s(4, 1, 0);
                    c.sw(cam_ea!(c, SP, 0x30), 0u64);
                    c.mul_s(1, 1, 6);
                    c.sw(cam_ea!(c, SP, 0x34), 0u64);
                    c.mul_s(13, 8, 3);
                    c.sw(cam_ea!(c, SP, 0x38), 0u64);
                    c.mul_s(12, 8, 0);
                    c.sw(cam_ea!(c, SP, 0xc), 0u64);
                    c.mul_s(9, 2, 3);
                    c.sw(cam_ea!(c, SP, 0x1c), 0u64);
                    c.mul_s(10, 4, 3);
                    c.sw(cam_ea!(c, SP, 0x2c), 0u64);
                    c.mul_s(5, 2, 0);
                    c.mul_s(7, 4, 0);
                    c.mul_s(3, 1, 3);
                    c.mul_s(0, 1, 0);
                    c.mul_s(8, 8, 6);
                    c.mul_s(2, 2, 6);
                    c.mul_s(4, 4, 6);
                    c.mul_s(1, 1, 6);
                    c.add_s(5, 5, 8);
                    c.sub_s(0, 0, 13);
                    c.sub_s(2, 2, 12);
                    c.add_s(9, 9, 11);
                    c.swc1(5, cam_ea!(c, SP, 0x8));
                    c.add_s(7, 7, 11);
                    c.swc1(0, cam_ea!(c, SP, 0x4));
                    c.add_s(4, 4, 13);
                    c.swc1(2, cam_ea!(c, SP, 0x18));
                    c.sub_s(10, 10, 8);
                    c.swc1(9, cam_ea!(c, SP, 0x28));
                    c.add_s(1, 1, 11);
                    c.swc1(7, cam_ea!(c, SP, 0));
                    break 'b_27f934;
                }
                c.cycles += 1;
                c.lq(16, cam_ea!(c, SP, 0x230));
                break 'b_27f938;
            }
            c.cycles += 1;
            c.lq(17, cam_ea!(c, SP, 0x220));
            break 'b_27f93c;
        }
        c.cycles += 11;
        c.lq(18, cam_ea!(c, SP, 0x210));
        c.lq(19, cam_ea!(c, SP, 0x200));
        c.lq(20, cam_ea!(c, SP, 0x1f0));
        c.lq(21, cam_ea!(c, SP, 0x1e0));
        c.lq(22, cam_ea!(c, SP, 0x1d0));
        c.lq(23, cam_ea!(c, SP, 0x1c0));
        c.lq(30, cam_ea!(c, SP, 0x1b0));
        c.g[RA][0] = c.ld(cam_ea!(c, SP, 0x1a0));
        c.lwc1(20, cam_ea!(c, SP, 0x240));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(cam_ea!(c, SP, 0x250));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the channel-id fake (+4: the id it returns).
    pub(crate) const CAM_CTRL: u32 = DATA + 0xf_f000;
    pub(crate) const CAM_MAGIC: u32 = 0x4341_4d42;
    pub(crate) const CAM_OBJ: u32 = DATA + 0x1000;
    pub(crate) const CAM_ARG: u32 = DATA + 0x1100;
    pub(crate) const CAM_SCRIPT: u32 = DATA + 0x1200;
    pub(crate) const CAM_PARAMS: u32 = DATA + 0x1300;
    pub(crate) static CAM_PREV_ID: AtomicUsize = AtomicUsize::new(0);

    /// `InSeq_ChannelGetId` fake: returns the id the fixture chose (and
    /// unwinds at the scheduled call).
    pub(crate) fn cam_fake_channel_id(c: &mut Cpu) {
        if c.bus.read32(CAM_CTRL) != CAM_MAGIC {
            return fakes::fall_back(c, &CAM_PREV_ID, ee::s07::f_002743c8__generated);
        }
        let ra = c.g[RA][0] as u32;
        let id = c.bus.read32(CAM_CTRL + 4);
        fakes::fake(c, 0x0027_43c8, ra);
        if c.pc == ra {
            c.g[V0][0] = sx(id);
        }
    }

    /// `ScriptObjCam_BuildMatrixFromScript` against the generated body: all
    /// four modes plus the ids that leave early (4, -1, 7, i32::MIN), random
    /// params and constants, pose kind byte (0 or out of range: the real
    /// `ScriptEngine_GetTargetPose` zeroes its outputs), real
    /// `Math_SinCos` and param getters, and a channel-id fake that
    /// sometimes unwinds.
    #[test]
    fn script_obj_cam_build_matrix_from_script_matches() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002743C8),
            cam_fake_channel_id,
            &CAM_PREV_ID,
        );
        check(
            ee::s07::f_0027d970__generated,
            script_obj_cam_build_matrix_from_script,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27_d970, 1);
                c.pc = 0x0027_d970;
                let ids: [u32; 8] = [0, 1, 2, 3, 4, 0xffff_ffff, 7, 0x8000_0000];
                c.bus.write32(CAM_CTRL, CAM_MAGIC);
                c.bus.write32(CAM_CTRL + 4, ids[(seed % 8) as usize]);
                c.g[A0][0] = sx(CAM_OBJ);
                c.g[A1][0] = sx(CAM_ARG);
                c.bus.write32(CAM_ARG + 8, CAM_SCRIPT);
                c.bus.write32(CAM_SCRIPT + 8, r.next_u32());
                c.bus.write32(CAM_SCRIPT + 0x20, CAM_PARAMS);
                for k in 0..0x40 {
                    c.bus.write32(CAM_OBJ + 4 * k, r.next_u32());
                }
                // Params block: floats at +8..+0x1c, pose kind byte at +0x20.
                for off in (8..0x20).step_by(4) {
                    c.bus.write32(CAM_PARAMS + off, r.f32(3.0));
                }
                let kind: [u8; 4] = [0, 0x70, 0xf0, 0];
                c.bus
                    .write8(CAM_PARAMS + 0x20, kind[((seed / 8) % 4) as usize]);
                // Static data: constant matrix, axis vectors, angle scales.
                for off in (0..0x40).step_by(4) {
                    c.bus.write32(0x004f_f1a0 + off, r.f32(2.0));
                }
                for base in [0x004f_f140u32, 0x004f_f150, 0x004f_f160] {
                    for off in [0u32, 4, 8] {
                        c.bus.write32(base + off, r.f32(1.0));
                    }
                }
                for off in [0x479cu32, 0x4798, 0x4794, 0x4790] {
                    c.bus.write32(GAME_GP - off, r.f32(2.0));
                }
                c.bus.write32(GAME_GP - 0x84c, r.next_u32());
            },
        );
    }
}
