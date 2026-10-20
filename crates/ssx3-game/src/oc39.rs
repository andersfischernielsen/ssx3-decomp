//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for renderer objects and view setup (a modifier-block
//! transform query, two object-slot virtual calls, the graphics manager's
//! view stack: push/pop/copy of the view state, the viewport setter, and the
//! per-frame view-matrix/depth update), each with a differential test against
//! the generated body.
//!
//! - **Objects**: `ModObj_v20` (`vcall 26 == 2`), `ModObj_v03` (a tail call
//!   through slot 0x190), and the two modifier-block transform queries.
//! - **Views**: `PS2GraphicsMan_PushView` / `PopView` copy the current view
//!   state to/from the `views[]` stack; `SetViewport` clamps the rectangle,
//!   stores it in the current view and hands it to the defer manager;
//!   `CopyViewState` copies one `sPtView`; `UpdateViewMatrices` rebuilds the
//!   view/projection matrices through VU0; `PS2GraphicsMan_v27` computes the
//!   view's depth along a point and rebuilds the matrices when it changed.
//!
//! The replacements keep their state in the guest registers the originals
//! use where callers, callees or event handlers can see them, follow the call
//! protocol of [`crate::call`], and issue the original's VU0 macro ops.

use crate::abi;
use crate::call::{call_ptr, sp, Callee, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_6bf0, mod_obj_v20, "ModObj_v20"),
    (
        0x0037_d968,
        ps2_graphics_man_update_view_matrices,
        "PS2GraphicsMan_UpdateViewMatrices",
    ),
    (
        0x0035_2e50,
        modifier_block_has_transform_changed,
        "ModifierBlock_HasTransformChanged",
    ),
    (
        0x0035_2ed0,
        modifier_block_is_transform_finished,
        "ModifierBlock_IsTransformFinished",
    ),
    (0x0036_0da0, mod_obj_v03, "ModObj_v03"),
    (0x0037_7278, ps2_graphics_man_v27, "PS2GraphicsMan_v27"),
    (
        0x0037_6768,
        ps2_graphics_man_pop_view,
        "PS2GraphicsMan_PopView",
    ),
    (
        0x0037_65b8,
        ps2_graphics_man_push_view,
        "PS2GraphicsMan_PushView",
    ),
    (
        0x0037_6a70,
        ps2_graphics_man_set_viewport,
        "PS2GraphicsMan_SetViewport",
    ),
    (
        0x0039_50c0,
        ps2_graphics_man_copy_view_state,
        "PS2GraphicsMan_CopyViewState",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    /// Same shard: a `jal` to the generated alias.
    pub const UPDATE_VIEW_MATRICES: Callee = Callee {
        addr: 0x0037_d968,
        f: ee::s12::PS2GraphicsMan_UpdateViewMatrices,
    };
    pub const INTERN_MATRIX_RECORD: Callee = Callee {
        addr: 0x0036_45b8,
        f: ee::s12::DeferMan_InternMatrixRecord,
    };
    pub const SET_SCISSOR: Callee = Callee {
        addr: 0x0036_4b88,
        f: ee::s12::DeferMan_SetScissor,
    };
    pub const SET_SCISSOR_CLAMPED: Callee = Callee {
        addr: 0x0037_6938,
        f: ee::s12::PS2GraphicsMan_SetScissorClamped,
    };
    /// Cross-shard: a `jal` the generator emitted as `call_indirect`.
    pub const COPY_VIEW_STATE: u32 = 0x0039_50c0;
}

// ---------------------------------------------------------------------------
// Layout of the graphics manager (`PS2GraphicsMan`, the particle/view
// manager these functions belong to).

mod man {
    /// Display object used for virtual queries (`lh +0x50/+0x58` slots).
    pub const DISPLAY: u32 = 0x10d8;
    /// Index of the current view in `views[]`.
    pub const CUR_VIEW: u32 = 0x10dc;
    /// Pointer to the camera matrix the view matrices are built from.
    pub const CAMERA: u32 = 0x13e4;
    /// The defer manager (scissor and matrix records).
    pub const DEFER: u32 = 0x18f0;
    /// World matrix (projection `WORLD * CAMERA` goes to `VIEW_PROJ`).
    pub const WORLD: u32 = 0x6af0;
    pub const VIEW_PROJ: u32 = 0x6b30;
    /// Scissor rectangle, key, and the two view-space depths.
    pub const SCISSOR: u32 = 0x5900;
    pub const F5908: u32 = 0x5908;
    pub const F590C: u32 = 0x590c;
    pub const F5910: u32 = 0x5910;
    pub const Q5920: u32 = 0x5920;
    /// Matrices: camera copy, local, product, view matrix.
    pub const CAM_MAT: u32 = 0x5780;
    pub const LOCAL_MAT: u32 = 0x57c0;
    pub const CAM_MAT2: u32 = 0x5800;
    pub const MAT5860: u32 = 0x5860;
    pub const VIEW_MAT: u32 = 0x58a0;
    /// Quads pushed with the view state.
    pub const Q6B70: u32 = 0x6b70;
    pub const Q6B80: u32 = 0x6b80;
    pub const Q5840: u32 = 0x5840;
    pub const Q5850: u32 = 0x5850;
    pub const Q58E0: u32 = 0x58e0;
    pub const Q58F0: u32 = 0x58f0;
    /// `k + depth` values written by `PS2GraphicsMan_v27`.
    pub const F5858: u32 = 0x5858;
    pub const F58E8: u32 = 0x58e8;
    pub const F58F8: u32 = 0x58f8;
    /// Viewport clamp mode and top/bottom ratios.
    pub const CLAMP_MODE: u32 = 0x6b94;
    pub const CLAMP_TOP: u32 = 0x6b98;
    pub const CLAMP_BOTTOM: u32 = 0x6b9c;
    /// Set when the view matrices are current.
    pub const TEX_READY: u32 = 0x6b90;
    /// Drawing mode (0x31 selects the large depth bias).
    pub const MODE: u32 = 0x5a40;
    /// `sPtView views[]`.
    pub const VIEWS: u32 = 0x6d20;
    pub const VIEW_STRIDE: u32 = 0x230;
}

/// One saved view (`sPtView`).
mod vw {
    pub const DEPTH: u32 = 0x1c;
    pub const F24: u32 = 0x24;
    pub const F28: u32 = 0x28;
    pub const F2C: u32 = 0x2c;
    pub const Q30: u32 = 0x30;
    pub const Q40: u32 = 0x40;
    pub const Q50: u32 = 0x50;
    pub const M60: u32 = 0x60;
    pub const MA0: u32 = 0xa0;
    pub const ME0: u32 = 0xe0;
    pub const M120: u32 = 0x120;
    pub const Q160: u32 = 0x160;
    pub const Q170: u32 = 0x170;
    pub const M180: u32 = 0x180;
    pub const M1C0: u32 = 0x1c0;
    pub const Q200: u32 = 0x200;
    pub const Q210: u32 = 0x210;
    /// Scissor rectangle (four shorts) and the defer-record key.
    pub const SCISSOR: u32 = 0x220;
    pub const KEY: u32 = 0x228;
}

// ---------------------------------------------------------------------------
// Small helpers.

/// `x + off` on a 32-bit guest address.
#[inline(always)]
fn p(x: u32, off: u32) -> u32 {
    x.wrapping_add(off)
}

#[inline(always)]
fn fr(c: &Cpu, f: usize) -> FpuF32 {
    FpuF32(c.f[f])
}

#[inline(always)]
fn setf(c: &mut Cpu, f: usize, v: FpuF32) {
    c.f[f] = v.0;
}

#[inline(always)]
fn fmul(c: &mut Cpu, fd: usize, fs: usize, ft: usize) {
    let v = fr(c, fs) * fr(c, ft);
    setf(c, fd, v);
}

#[inline(always)]
fn fadd(c: &mut Cpu, fd: usize, fs: usize, ft: usize) {
    let v = fr(c, fs) + fr(c, ft);
    setf(c, fd, v);
}

#[inline(always)]
fn fsub(c: &mut Cpu, fd: usize, fs: usize, ft: usize) {
    let v = fr(c, fs) - fr(c, ft);
    setf(c, fd, v);
}

#[inline(always)]
fn f_lt(c: &mut Cpu, fs: usize, ft: usize) -> bool {
    let r = fr(c, fs).lt(fr(c, ft));
    abi::set_fcc(c, r);
    r
}

#[inline(always)]
fn f_le(c: &mut Cpu, fs: usize, ft: usize) -> bool {
    let r = fr(c, fs).le(fr(c, ft));
    abi::set_fcc(c, r);
    r
}

#[inline(always)]
fn f_eq(c: &mut Cpu, fs: usize, ft: usize) -> bool {
    let r = fr(c, fs).eq_s(fr(c, ft));
    abi::set_fcc(c, r);
    r
}

/// `lq gpr, from; sq gpr, to` (a quadword moved through a GPR).
#[inline(always)]
fn copy_q(c: &mut Cpu, gpr: usize, from: u32, to: u32) {
    c.lq(gpr, from);
    c.sq(gpr, to);
}

/// A 4x4 matrix copied through `vf1`..`vf4`.
#[inline(always)]
fn copy_mat(c: &mut Cpu, from: u32, to: u32) {
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, p(from, 0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, p(to, 0x10 * i));
    }
}

/// `dst = m1 * m2` the way gcc emitted it: VU0 multiply into `tmp`, then
/// `tmp -> sp+0 -> dst` (the intermediate quadwords are visible in RAM).
#[inline(always)]
fn mul_mat(c: &mut Cpu, dst: u32, m1: u32, m2: u32, tmp: u32) {
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, p(m1, 0x10 * i));
    }
    for i in 0..4u32 {
        c.lqc2(8 + i as usize, p(m2, 0x10 * i));
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
        c.sqc2(12 + i as usize, p(tmp, 0x10 * i));
    }
    let scratch = sp(c);
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, p(tmp, 0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, p(scratch, 0x10 * i));
    }
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, p(scratch, 0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, p(dst, 0x10 * i));
    }
}

// ---------------------------------------------------------------------------
// Objects.

/// `bool ModObj_v20(ModObj* o)` (0x356bf0): runs the object's slot-0xd0
/// method (an object type query) and reports whether it returned 2.
///
/// The reference calls it `VCall_6BF0(o, 26) == 2` through the slot table at
/// `*(o+0x1c) -> *slot -> vtable[26]`.
pub fn mod_obj_v20(c: &mut Cpu) {
    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    let this = abi::arg64(c, 0);
    c.g[V1][0] = Addr(this as u32).off(0x1c).lw(c);
    c.g[V0][0] = 0;
    // (The "no object" branch goes straight to the epilogue: it skips the
    // tail increment and RA reload below.)
    let mut straight_out = false;
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
        let obj = c.g[A1][0];
        c.g[RA][0] = Addr(sp(c)).ld(c);
        if obj == 0 {
            straight_out = true;
        } else {
            c.cycles += 9;
            c.g[V1][0] = Addr(obj as u32).lw(c);
            let vt = c.g[V1][0];
            c.g[A0][0] = Addr(vt as u32).off(0xd0).lh(c);
            c.g[V0][0] = Addr(vt as u32).off(0xd4).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0035_6c24);
            c.g[A0][0] = abi::addu(obj, c.g[A0][0]);
            if call_ptr(c, target, 0x0035_6c24).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = 0x2;
            let same = c.g[A0][0] == c.g[V1][0];
            c.g[V0][0] = 1;
            if !same {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
    }
    if !straight_out {
        c.cycles += 1;
        c.g[RA][0] = Addr(sp(c)).ld(c);
    }
    c.cycles += 2;
    frame.close(c);
}

/// `bool ModifierBlock_HasTransformChanged(ModifierBlock* b)` (0x352e50):
/// false when the block has no bound-box node, else slot 0x88's answer.
pub fn modifier_block_has_transform_changed(c: &mut Cpu) {
    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = Addr(abi::arg(c, 0)).lw(c);
    let node = c.g[A1][0];
    if node == 0 {
        c.g[V0][0] = 0;
    } else {
        c.cycles += 8;
        c.g[V1][0] = Addr(node as u32).lw(c);
        let vt = c.g[V1][0];
        c.g[A0][0] = Addr(vt as u32).off(0x88).lh(c);
        c.g[V0][0] = Addr(vt as u32).off(0x8c).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0035_2e78);
        c.g[A0][0] = abi::addu(node, c.g[A0][0]);
        if call_ptr(c, target, 0x0035_2e78).is_err() {
            return;
        }
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 3;
    c.g[RA][0] = Addr(sp(c)).ld(c);
    frame.close(c);
}

/// `bool ModifierBlock_IsTransformFinished(ModifierBlock* b)` (0x352ed0):
/// false without a node, else the node's slot 0x30 result.
pub fn modifier_block_is_transform_finished(c: &mut Cpu) {
    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = Addr(abi::arg(c, 0)).lw(c);
    let node = c.g[A1][0];
    if node == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.g[V1][0] = Addr(node as u32).lw(c);
        let vt = c.g[V1][0];
        c.cycles += 4;
        c.g[A0][0] = Addr(vt as u32).off(0x30).lh(c);
        c.g[V0][0] = Addr(vt as u32).off(0x34).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0035_2efc);
        c.g[A0][0] = abi::addu(node, c.g[A0][0]);
        if call_ptr(c, target, 0x0035_2efc).is_err() {
            return;
        }
    }
    c.cycles += 3;
    c.g[RA][0] = Addr(sp(c)).ld(c);
    frame.close(c);
}

/// `void ModObj_v03(ModObj* o)` (0x360da0): a tail call to the node at
/// `*(o+0xc)`'s slot 0x190 method.
pub fn mod_obj_v03(c: &mut Cpu) {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    let this = abi::arg64(c, 0);
    c.g[V0][0] = Addr(this as u32).off(0xc).lw(c);
    let node = c.g[V0][0];
    c.g[V1][0] = Addr(node as u32).off(0x190).lh(c);
    c.g[A1][0] = Addr(node as u32).off(0x194).lw(c);
    let target = c.g[A1][0] as u32;
    c.g[RA][0] = sx(0x0036_0dbc);
    c.g[A0][0] = abi::addu(this, c.g[V1][0]);
    if call_ptr(c, target, 0x0036_0dbc).is_err() {
        return;
    }
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// View matrices and depth.

/// `void PS2GraphicsMan_UpdateViewMatrices(PS2GraphicsMan* g)` (0x37d968):
/// mark the matrices current, rebuild the view-projection matrix and the two
/// dependent products, then hand the new view matrix to the defer manager.
pub fn ps2_graphics_man_update_view_matrices(c: &mut Cpu) {
    c.cycles += 159;
    let frame = Frame::open(c, 0x90);
    let gfx = abi::arg(c, 0);
    let sp0 = sp(c);
    c.g[V0][0] = 1;
    frame.save_ra(c, 0x80);
    c.g[A2][0] = sx(p(sp0, 0x40));
    c.sw(p(gfx, man::TEX_READY), c.g[V0][0]);
    c.g[V1][0] = sx(p(gfx, man::WORLD));
    c.g[V0][0] = Addr(gfx).off(man::CAMERA).lw(c);
    let tmp = c.g[A2][0] as u32;
    let world = c.g[V1][0] as u32;
    let cam = c.g[V0][0] as u32;
    // view-projection = world * camera
    mul_mat(c, p(gfx, man::VIEW_PROJ), world, cam, tmp);
    c.g[A1][0] = sx(p(gfx, man::CAM_MAT));
    c.g[V0][0] = Addr(gfx).off(man::CAMERA).lw(c);
    let cam = c.g[V0][0] as u32;
    let cam_mat = c.g[A1][0] as u32;
    copy_mat(c, cam, cam_mat);
    c.g[V1][0] = sx(p(gfx, man::LOCAL_MAT));
    let local = c.g[V1][0] as u32;
    let cam_mat = c.g[A1][0] as u32;
    mul_mat(c, p(gfx, man::CAM_MAT2), local, cam_mat, tmp);
    c.g[V1][0] = sx(p(gfx, man::MAT5860));
    let m5860 = c.g[V1][0] as u32;
    let cam_mat = c.g[A1][0] as u32;
    mul_mat(c, p(gfx, man::VIEW_MAT), m5860, cam_mat, tmp);
    c.g[V0][0] = sx(p(gfx, man::VIEW_MAT));
    c.g[RA][0] = sx(0x0037_dbd8);
    c.g[A0][0] = Addr(gfx).off(man::DEFER).lw(c);
    if callee::INTERN_MATRIX_RECORD.call(c, 0x0037_dbd8).is_err() {
        return;
    }
    frame.restore_ra(c, 0x80);
    frame.close(c);
}

/// `void PS2GraphicsMan_v27(PS2GraphicsMan* g, const Vec4* pos, float dist)`
/// (0x377278): project `pos` (and `pos + dist * camera-direction`) through the
/// view matrix, derive the view depth, and when it differs from the saved
/// depth, store `k + depth` (k = 32767.5, or 8388467.5 in mode 0x31) and mark
/// the matrices stale.
pub fn ps2_graphics_man_v27(c: &mut Cpu) {
    c.cycles += 16;
    let frame = Frame::open(c, 0xb0);
    let gfx = abi::arg(c, 0);
    let pos = abi::arg(c, 1);
    let sp0 = sp(c);
    c.g[V1][0] = 0x230;
    frame.save(c, S0, 0x90);
    frame.save(c, S1, 0x80);
    c.g[S0][0] = abi::arg64(c, 0);
    frame.save(c, S2, 0x70);
    frame.save_fpr(c, 20, 0xa0);
    c.g[S2][0] = abi::arg64(c, 1);
    frame.save_ra(c, 0x60);
    c.mov_s(20, 12);
    c.g[V0][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    c.g[A0][0] = Addr(gfx).off(man::TEX_READY).lw(c);
    c.mult(0, 2, 2, 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(man::VIEWS));
    let ready = c.g[A0][0];
    c.g[S1][0] = sx((gfx).wrapping_add(c.g[V0][0] as u32));
    if ready == 0 {
        c.cycles += 2;
        c.g[RA][0] = sx(0x0037_72c0);
        c.g[A0][0] = c.g[S0][0];
        if callee::UPDATE_VIEW_MATRICES.call(c, 0x0037_72c0).is_err() {
            return;
        }
    }
    let view = c.g[S1][0] as u32;

    // The depth offset: `f1` becomes 0 when the point is at the camera or
    // the projection ratio leaves [-1, 1].
    c.cycles += 5;
    c.mtc1(0, 3);
    let at_camera = f_eq(c, 20, 3);
    c.g[V1][0] = sx(p(gfx, man::VIEW_MAT));
    let mut computed = false;
    if !at_camera {
        c.cycles += 56;
        c.g[V0][0] = Addr(gfx).off(man::CAMERA).lw(c);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 4);
        c.lwc1(2, p(c.g[V0][0] as u32, 0x28));
        c.lwc1(0, p(c.g[V0][0] as u32, 0x8));
        c.lwc1(1, p(c.g[V0][0] as u32, 0x18));
        c.swc1(0, p(sp0, 0x0));
        c.swc1(1, p(sp0, 0x4));
        c.swc1(2, p(sp0, 0x8));
        c.swc1(3, p(sp0, 0xc));
        // view * pos
        c.lqc2(8, pos);
        c.lqc2(4, p(gfx, man::VIEW_MAT));
        c.lqc2(5, p(gfx, man::VIEW_MAT + 0x10));
        c.lqc2(6, p(gfx, man::VIEW_MAT + 0x20));
        c.lqc2(7, p(gfx, man::VIEW_MAT + 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, p(sp0, 0x20));
        c.lq(2, p(sp0, 0x20));
        c.sq(2, p(sp0, 0x10));
        // pos + dist * (camera direction)
        c.mfc1(4, 20);
        c.lqc2(4, p(sp0, 0x0));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, p(sp0, 0x50));
        c.lq(2, p(sp0, 0x50));
        c.sq(2, p(sp0, 0x40));
        c.lqc2(3, pos);
        c.lqc2(4, p(sp0, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(sp0, 0x50));
        c.lq(2, p(sp0, 0x50));
        c.sq(2, p(sp0, 0x30));
        // view * (pos + dist * dir)
        c.lqc2(8, p(sp0, 0x30));
        c.lqc2(4, p(gfx, man::VIEW_MAT));
        c.lqc2(5, p(gfx, man::VIEW_MAT + 0x10));
        c.lqc2(6, p(gfx, man::VIEW_MAT + 0x20));
        c.lqc2(7, p(gfx, man::VIEW_MAT + 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, p(sp0, 0x50));
        c.lwc1(1, p(sp0, 0x18));
        c.lwc1(0, p(sp0, 0x1c));
        c.lq(2, p(sp0, 0x50));
        c.div_s(2, 1, 0);
        c.sq(2, p(sp0, 0x20));
        c.lwc1(1, p(sp0, 0x28));
        c.lwc1(0, p(sp0, 0x2c));
        c.c_le_s(4, 2);
        c.div_s(1, 1, 0);
        if c.fcc() {
            c.cycles += 5;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            if f_le(c, 2, 0) {
                c.cycles += 3;
                if f_le(c, 4, 1) {
                    c.cycles += 3;
                    if f_le(c, 1, 0) {
                        c.cycles += 4;
                        fsub(c, 1, 1, 2);
                        c.lwc1(0, p(gfx, man::F58E8));
                        fmul(c, 1, 0, 1);
                        computed = true;
                    }
                }
            }
        }
    }
    if !computed {
        c.cycles += 1;
        c.mtc1(0, 1);
    }

    // Compare with the depth saved in the view; rebuild when it changed.
    c.cycles += 5;
    c.lwc1(0, p(view, vw::DEPTH));
    if f_eq(c, 1, 0) {
        c.lq(16, p(sp0, 0x90));
    } else {
        c.cycles += 6;
        c.swc1(1, p(view, vw::DEPTH));
        c.g[V1][0] = 0x31;
        let gp = c.g[GP][0] as u32;
        c.lwc1(2, gp.wrapping_sub(0x279c));
        c.g[V0][0] = Addr(gfx).off(man::MODE).lw(c);
        if c.g[V0][0] != c.g[V1][0] {
            fadd(c, 1, 2, 1);
        } else {
            c.cycles += 2;
            let gp = c.g[GP][0] as u32;
            c.lwc1(2, gp.wrapping_sub(0x2798));
            fadd(c, 1, 2, 1);
        }
        c.cycles += 6;
        c.swc1(1, p(gfx, man::F58F8));
        c.lwc1(0, p(view, vw::DEPTH));
        c.sw(p(gfx, man::TEX_READY), 0);
        fadd(c, 0, 2, 0);
        c.swc1(0, p(gfx, man::F5858));
        c.lq(16, p(sp0, 0x90));
    }
    c.cycles += 6;
    c.lq(17, p(sp0, 0x80));
    c.lq(18, p(sp0, 0x70));
    c.g[RA][0] = c.ld(p(sp0, 0x60));
    c.lwc1(20, p(sp0, 0xa0));
    c.g[SP][0] = sx(p(sp0, 0xb0));
    c.pc = c.g[RA][0] as u32;
}

// ---------------------------------------------------------------------------
// The view stack.

/// `void PS2GraphicsMan_PushView(PS2GraphicsMan* g)` (0x3765b8): save the
/// current view state (scissor, key, depths, matrices, quads) into
/// `views[curView]`, bump the index, and copy that state into the new view.
pub fn ps2_graphics_man_push_view(c: &mut Cpu) {
    c.cycles += 108;
    let frame = Frame::open(c, 0x10);
    let gfx = abi::arg(c, 0);
    c.g[T1][0] = 0x230;
    frame.save_ra(c, 0);
    c.g[A1][0] = 0x60000;
    c.g[A1][0] |= 0x8000;
    c.g[T0][0] = sx(p(gfx, man::WORLD));
    c.g[A2][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    c.mult(0, 6, 6, 9);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(man::VIEWS));
    c.g[A2][0] = sx((gfx).wrapping_add(c.g[A2][0] as u32));
    let view = c.g[A2][0] as u32;
    c.ldl(2, p(gfx, man::SCISSOR + 7));
    c.ldr(2, p(gfx, man::SCISSOR));
    c.sdl(2, p(view, vw::SCISSOR + 7));
    c.sdr(2, p(view, vw::SCISSOR));
    c.g[A3][0] = sx(p(view, vw::M60));
    c.g[V0][0] = Addr(gfx).off(man::DEFER).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1cc4).lw(c);
    c.sw(p(view, vw::KEY), c.g[V1][0]);
    c.lwc1(0, p(gfx, man::F590C));
    c.swc1(0, p(view, vw::F24));
    c.lwc1(1, p(gfx, man::F5908));
    c.swc1(1, p(view, vw::F28));
    copy_mat(c, c.g[T0][0] as u32, c.g[A3][0] as u32);
    c.lq(3, p(gfx, man::Q6B70));
    c.g[A3][0] = sx(p(view, vw::MA0));
    c.g[A1][0] = sx(p(gfx, man::CAM_MAT));
    c.sq(3, p(view, vw::Q30));
    c.lq(2, p(gfx, man::Q6B80));
    c.sq(2, p(view, vw::Q40));
    copy_mat(c, c.g[A1][0] as u32, c.g[A3][0] as u32);
    c.g[V1][0] = sx(p(view, vw::ME0));
    c.g[V0][0] = sx(p(gfx, man::LOCAL_MAT));
    copy_mat(c, c.g[V0][0] as u32, c.g[V1][0] as u32);
    c.g[A1][0] = sx(p(view, vw::M120));
    c.g[V0][0] = sx(p(gfx, man::CAM_MAT2));
    copy_mat(c, c.g[V0][0] as u32, c.g[A1][0] as u32);
    c.lq(3, p(gfx, man::Q5840));
    c.g[T0][0] = sx(p(view, vw::M180));
    c.g[A3][0] = sx(p(gfx, man::MAT5860));
    c.sq(3, p(view, vw::Q160));
    c.lq(2, p(gfx, man::Q5850));
    c.sq(2, p(view, vw::Q170));
    copy_mat(c, c.g[A3][0] as u32, c.g[T0][0] as u32);
    c.g[A1][0] = sx(p(view, vw::M1C0));
    c.g[V0][0] = sx(p(gfx, man::VIEW_MAT));
    copy_mat(c, c.g[V0][0] as u32, c.g[A1][0] as u32);
    c.lq(3, p(gfx, man::Q58E0));
    c.g[A1][0] = c.g[A2][0];
    c.sq(3, p(view, vw::Q200));
    c.lq(2, p(gfx, man::Q58F0));
    c.sq(2, p(view, vw::Q210));
    c.lwc1(0, p(gfx, man::F5910));
    c.swc1(0, p(view, vw::F2C));
    c.lq(2, p(gfx, man::Q5920));
    c.sq(2, p(view, vw::Q50));
    c.g[V1][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.mult(0, 9, 3, 9);
    c.sw(p(gfx, man::CUR_VIEW), c.g[V1][0]);
    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(man::VIEWS));
    c.g[RA][0] = sx(0x0037_675c);
    c.g[A0][0] = sx((gfx).wrapping_add(c.g[T1][0] as u32));
    if call_ptr(c, callee::COPY_VIEW_STATE, 0x0037_675c).is_err() {
        return;
    }
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void PS2GraphicsMan_PopView(PS2GraphicsMan* g)` (0x376768): pop the view
/// index, restore the saved view state from `views[curView]` and clear the
/// "matrices current" flag.
pub fn ps2_graphics_man_pop_view(c: &mut Cpu) {
    c.cycles += 20;
    let frame = Frame::open(c, 0x30);
    c.g[V1][0] = 0x230;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = abi::arg64(c, 0);
    frame.save_ra(c, 0);
    c.g[A0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(man::CUR_VIEW).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    c.mult(0, 3, 2, 3);
    c.sw(p(c.g[S0][0] as u32, man::CUR_VIEW), c.g[V0][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(man::VIEWS));
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let gfx = c.g[S0][0] as u32;
    let view = c.g[S1][0] as u32;
    c.ldl(2, p(view, vw::SCISSOR + 7));
    c.ldr(2, p(view, vw::SCISSOR));
    c.sdl(2, p(gfx, man::SCISSOR + 7));
    c.sdr(2, p(gfx, man::SCISSOR));
    c.g[A2][0] = Addr(view).off(vw::KEY).lw(c);
    c.g[A1][0] = sx(p(gfx, man::SCISSOR));
    if c.g[A2][0] == c.g[A0][0] {
        c.cycles += 4;
        c.g[RA][0] = sx(0x0037_67c0);
        c.g[A0][0] = Addr(gfx).off(man::DEFER).lw(c);
        if callee::SET_SCISSOR.call(c, 0x0037_67c0).is_err() {
            return;
        }
        c.lwc1(1, p(view, vw::F24));
    } else {
        c.cycles += 6;
        c.g[V1][0] = Addr(gfx).off(man::DEFER).lw(c);
        c.g[V0][0] = 0x60000;
        c.g[V0][0] |= 0x8000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(p(c.g[V1][0] as u32, 0x1cc4), c.g[A2][0]);
        c.lwc1(1, p(view, vw::F24));
    }
    c.cycles += 85;
    c.g[A0][0] = sx(p(gfx, man::WORLD));
    c.g[V0][0] = sx(p(view, vw::M60));
    c.swc1(1, p(gfx, man::F590C));
    c.lwc1(0, p(view, vw::F28));
    c.swc1(0, p(gfx, man::F5908));
    copy_mat(c, c.g[V0][0] as u32, c.g[A0][0] as u32);
    c.lq(3, p(view, vw::Q30));
    c.g[A1][0] = sx(p(gfx, man::CAM_MAT));
    c.g[A0][0] = sx(p(view, vw::MA0));
    c.sq(3, p(gfx, man::Q6B70));
    c.lq(2, p(view, vw::Q40));
    c.sq(2, p(gfx, man::Q6B80));
    copy_mat(c, c.g[A0][0] as u32, c.g[A1][0] as u32);
    c.g[V1][0] = sx(p(gfx, man::LOCAL_MAT));
    c.g[V0][0] = sx(p(view, vw::ME0));
    copy_mat(c, c.g[V0][0] as u32, c.g[V1][0] as u32);
    c.g[A0][0] = sx(p(gfx, man::CAM_MAT2));
    c.g[V0][0] = sx(p(view, vw::M120));
    copy_mat(c, c.g[V0][0] as u32, c.g[A0][0] as u32);
    c.lq(3, p(view, vw::Q160));
    c.g[A2][0] = sx(p(gfx, man::MAT5860));
    c.g[A1][0] = sx(p(view, vw::M180));
    c.sq(3, p(gfx, man::Q5840));
    c.lq(2, p(view, vw::Q170));
    c.sq(2, p(gfx, man::Q5850));
    copy_mat(c, c.g[A1][0] as u32, c.g[A2][0] as u32);
    c.g[A0][0] = sx(p(gfx, man::VIEW_MAT));
    c.g[V1][0] = sx(p(view, vw::M1C0));
    copy_mat(c, c.g[V1][0] as u32, c.g[A0][0] as u32);
    c.lq(2, p(view, vw::Q200));
    frame.restore_ra(c, 0);
    c.sq(2, p(gfx, man::Q58E0));
    c.lq(3, p(view, vw::Q210));
    c.sq(3, p(gfx, man::Q58F0));
    c.lwc1(0, p(view, vw::F2C));
    c.swc1(0, p(gfx, man::F5910));
    c.lq(2, p(view, vw::Q50));
    c.sw(p(gfx, man::TEX_READY), 0);
    c.sq(2, p(gfx, man::Q5920));
    c.lq(16, p(sp(c), 0x20));
    c.lq(17, p(sp(c), 0x10));
    frame.close(c);
}

/// `void PS2GraphicsMan_SetViewport(PS2GraphicsMan* g, float x, float y,
/// float w, float h)` (0x376a70): clamp `y`/`h` by the display height in
/// clamp modes 0..2, store the rectangle in the current view, and queue the
/// scissor.
pub fn ps2_graphics_man_set_viewport(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x10);
    frame.save_fpr(c, 23, 0x38);
    c.g[S0][0] = abi::arg64(c, 0);
    frame.save_fpr(c, 22, 0x30);
    c.mov_s(23, 14);
    frame.save_fpr(c, 21, 0x28);
    c.mov_s(22, 12);
    frame.save_fpr(c, 20, 0x20);
    c.mov_s(21, 15);
    frame.save_ra(c, 0);
    c.mov_s(20, 13);
    let gfx = abi::arg(c, 0);
    c.g[V1][0] = Addr(gfx).off(man::DISPLAY).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[RA][0] = sx(0x0037_6ab4);
    c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
    if call_ptr(c, target, 0x0037_6ab4).is_err() {
        return;
    }
    c.g[V1][0] = Addr(gfx).off(man::CLAMP_MODE).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
    let mut clamped = true;
    if c.g[V0][0] == 0 {
        clamped = false;
    } else {
        c.cycles += 2;
        if (c.g[V1][0] as i64) < 0 {
            clamped = false;
        }
    }
    if !clamped {
        c.g[V1][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    } else {
        c.cycles += 8;
        c.mtc1(4, 1);
        c.cvt_s_w(1, 1);
        c.lwc1(0, p(gfx, man::CLAMP_TOP));
        fmul(c, 0, 0, 1);
        if f_lt(c, 20, 0) {
            c.mov_s(20, 0);
        }
        c.cycles += 6;
        c.lwc1(0, p(gfx, man::CLAMP_BOTTOM));
        fmul(c, 0, 0, 1);
        if f_lt(c, 0, 21) {
            c.mov_s(21, 0);
        }
        c.cycles += 1;
        c.g[V1][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    }
    c.cycles += 33;
    c.g[A1][0] = 0x230;
    c.mov_s(12, 22);
    c.mov_s(13, 20);
    c.mult(0, 2, 3, 5);
    c.mov_s(14, 23);
    c.mov_s(15, 21);
    c.g[A0][0] = c.g[S0][0];
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
    c.swc1(22, p(c.g[V1][0] as u32, man::VIEWS));
    c.g[V0][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    c.mult(0, 3, 2, 5);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
    c.swc1(20, p(c.g[V0][0] as u32, man::VIEWS + 0x4));
    c.g[V1][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    c.mult(0, 2, 3, 5);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
    c.swc1(23, p(c.g[V1][0] as u32, man::VIEWS + 0x8));
    c.g[V0][0] = Addr(gfx).off(man::CUR_VIEW).lw(c);
    c.mult(0, 3, 2, 5);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
    c.g[RA][0] = sx(0x0037_6b64);
    c.swc1(21, p(c.g[V0][0] as u32, man::VIEWS + 0xc));
    if callee::SET_SCISSOR_CLAMPED.call(c, 0x0037_6b64).is_err() {
        return;
    }
    c.g[A0][0] = Addr(gfx).off(man::DEFER).lw(c);
    c.g[RA][0] = sx(0x0037_6b70);
    c.g[A1][0] = sx(p(gfx, man::SCISSOR));
    if callee::SET_SCISSOR.call(c, 0x0037_6b70).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 23, 0x38);
    frame.restore_fpr(c, 22, 0x30);
    frame.restore_fpr(c, 21, 0x28);
    frame.restore_fpr(c, 20, 0x20);
    frame.close(c);
}

/// `void PS2GraphicsMan_CopyViewState(sPtView* dst, const sPtView* src)`
/// (0x3950c0): copy one saved view (scissor/shorts, floats, matrices, quads,
/// key). Returns `dst` in v0 and the copied key in v1.
pub fn ps2_graphics_man_copy_view_state(c: &mut Cpu) {
    c.cycles += 106;
    let dst = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    c.g[A3][0] = sx(p(dst, vw::M60));
    c.g[A2][0] = sx(p(src, vw::M60));
    for &(off, fpr) in &[
        (0x0u32, 1usize),
        (0x4, 0),
        (0x8, 1),
        (0xc, 0),
        (0x10, 1),
        (0x14, 0),
        (0x18, 1),
        (0x1c, 0),
    ] {
        c.lwc1(fpr, p(src, off));
        c.swc1(fpr, p(dst, off));
    }
    c.g[V0][0] = Addr(src).off(0x20).lw(c);
    c.sw(p(dst, 0x20), c.g[V0][0]);
    for &(off, fpr) in &[(0x24u32, 0usize), (0x28, 1), (0x2c, 0)] {
        c.lwc1(fpr, p(src, off));
        c.swc1(fpr, p(dst, off));
    }
    copy_q(c, 2, p(src, vw::Q30), p(dst, vw::Q30));
    copy_q(c, 3, p(src, vw::Q40), p(dst, vw::Q40));
    copy_q(c, 2, p(src, vw::Q50), p(dst, vw::Q50));
    copy_mat(c, c.g[A2][0] as u32, c.g[A3][0] as u32);
    c.g[V1][0] = sx(p(dst, vw::MA0));
    c.g[V0][0] = sx(p(src, vw::MA0));
    copy_mat(c, c.g[V0][0] as u32, c.g[V1][0] as u32);
    c.g[A2][0] = sx(p(dst, vw::ME0));
    c.g[V0][0] = sx(p(src, vw::ME0));
    copy_mat(c, c.g[V0][0] as u32, c.g[A2][0] as u32);
    c.g[A3][0] = sx(p(dst, vw::M120));
    c.g[V0][0] = sx(p(src, vw::M120));
    copy_mat(c, c.g[V0][0] as u32, c.g[A3][0] as u32);
    c.lq(3, p(src, vw::Q160));
    c.g[A3][0] = sx(p(dst, vw::M180));
    c.g[A2][0] = sx(p(src, vw::M180));
    c.sq(3, p(dst, vw::Q160));
    c.lq(2, p(src, vw::Q170));
    c.sq(2, p(dst, vw::Q170));
    copy_mat(c, c.g[A2][0] as u32, c.g[A3][0] as u32);
    c.g[V1][0] = sx(p(dst, vw::M1C0));
    c.g[V0][0] = sx(p(src, vw::M1C0));
    copy_mat(c, c.g[V0][0] as u32, c.g[V1][0] as u32);
    c.lq(6, p(src, vw::Q200));
    c.g[V0][0] = c.g[A0][0];
    c.sq(6, p(dst, vw::Q200));
    c.lq(3, p(src, vw::Q210));
    c.sq(3, p(dst, vw::Q210));
    c.ldl(3, p(src, vw::SCISSOR + 7));
    c.ldr(3, p(src, vw::SCISSOR));
    c.sdl(3, p(dst, vw::SCISSOR + 7));
    c.sdr(3, p(dst, vw::SCISSOR));
    c.g[V1][0] = Addr(src).off(vw::KEY).lw(c);
    c.sw(p(dst, vw::KEY), c.g[V1][0]);
    c.pc = c.g[RA][0] as u32;
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 250;
    /// A generated function that returns 0 (a `bool`-returning callee).
    const RETURNS_ZERO: u32 = 0x0032_e688;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 8) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// The test vtable (`DATA + 0x3000`); entries are `{delta: 0, fn}`.
    fn vt() -> u32 {
        DATA + 0x3000
    }

    fn slot(c: &mut Cpu, off: u32, target: u32) {
        c.bus.write16(vt() + off, 0);
        c.bus.write32(vt() + off + 4, target);
    }

    /// A DeferMan stand-in at `DATA + 0x8000` with empty record lists.
    fn put_defer(c: &mut Cpu) -> u32 {
        let d = DATA + 0x8000;
        let rec = d + 0x68000;
        c.bus.write32(rec + 0x1cc4, 0);
        c.bus.write32(rec + 0x1cc8, 0);
        c.bus.write32(rec + 0x2ed0, 0);
        d
    }

    fn identity(c: &mut Cpu, at: u32) {
        for i in 0..4u32 {
            for k in 0..4u32 {
                let v = if i == k { f(1.0) } else { 0 };
                c.bus.write32(at + 0x10 * i + 4 * k, v);
            }
        }
    }

    #[test]
    fn mod_obj_v20_matches() {
        covered(ee::s11::f_00356bf0__generated, mod_obj_v20, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x35_6bf0, 1);
            let block = DATA;
            let h = DATA + 0x3100;
            let obj = DATA + 0x3200;
            slot(c, 0xd0, FAKE);
            c.bus.write32(obj, vt());
            if seed % 4 == 0 {
                c.bus.write32(block + 0x1c, 0);
            } else {
                c.bus.write32(block + 0x1c, h);
                if seed % 5 == 0 {
                    c.bus.write32(h, 0);
                } else {
                    c.bus.write32(h, obj);
                    // Force a "type 2" answer for half of the calls.
                    c.bus.write32(
                        crate::difftest::fakes::LOG_RESULTS,
                        if seed % 2 == 0 { 2 } else { 0 },
                    );
                }
            }
            c.g[A0][0] = sx(block);
        });
    }

    #[test]
    fn modifier_block_has_transform_changed_matches() {
        covered(
            ee::s11::f_00352e50__generated,
            modifier_block_has_transform_changed,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x35_2e50, 1);
                let block = DATA;
                let node = DATA + 0x3100;
                slot(c, 0x88, if seed % 3 == 0 { RETURNS_ZERO } else { FAKE });
                c.bus.write32(node, vt());
                if seed % 5 == 0 {
                    c.bus.write32(block, 0);
                } else {
                    c.bus.write32(block, node);
                }
                c.g[A0][0] = sx(block);
            },
        );
    }

    #[test]
    fn modifier_block_is_transform_finished_matches() {
        covered(
            ee::s11::f_00352ed0__generated,
            modifier_block_is_transform_finished,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x35_2ed0, 1);
                let block = DATA;
                let node = DATA + 0x3100;
                slot(c, 0x30, if seed % 4 == 0 { RETURNS_ZERO } else { FAKE });
                c.bus.write32(node, vt());
                if seed % 5 == 0 {
                    c.bus.write32(block, 0);
                } else {
                    c.bus.write32(block, node);
                }
                c.g[A0][0] = sx(block);
            },
        );
    }

    #[test]
    fn mod_obj_v03_matches() {
        covered(ee::s12::f_00360da0__generated, mod_obj_v03, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x36_0da0, 1);
            let block = DATA;
            let node = DATA + 0x3100;
            slot(c, 0x190, if seed % 3 == 0 { RETURNS_ZERO } else { FAKE });
            c.bus.write32(node, vt());
            c.bus.write32(block + 0xc, node);
            c.g[A0][0] = sx(block);
        });
    }

    #[test]
    fn ps2_graphics_man_update_view_matrices_matches() {
        check(
            ee::s12::f_0037d968__generated,
            ps2_graphics_man_update_view_matrices,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x37_d968);
                let gfx = DATA;
                let cam = DATA + 0x1000;
                let defer = put_defer(c);
                for i in 0..4u32 {
                    for k in 0..4u32 {
                        c.bus.write32(cam + 0x10 * i + 4 * k, r.edge_f32());
                        c.bus
                            .write32(gfx + man::WORLD + 0x10 * i + 4 * k, r.edge_f32());
                        c.bus
                            .write32(gfx + man::LOCAL_MAT + 0x10 * i + 4 * k, r.edge_f32());
                        c.bus
                            .write32(gfx + man::MAT5860 + 0x10 * i + 4 * k, r.edge_f32());
                    }
                }
                c.bus.write32(gfx + man::CAMERA, cam);
                c.bus.write32(gfx + man::DEFER, defer);
                c.g[A0][0] = sx(gfx);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_push_view_matches() {
        check(ee::s12::f_003765b8__generated, ps2_graphics_man_push_view, 200, &|c, seed| {
            let mut r = base(c, seed, 0x37_65b8);
            let gfx = DATA;
            let defer = put_defer(c);
            fill(c, &mut r, gfx + man::CAM_MAT, 0x200);
            fill(c, &mut r, gfx + man::WORLD, 0xd0);
            fill(c, &mut r, gfx + man::VIEWS, 0x460);
            c.bus.write32(gfx + man::CUR_VIEW, (seed % 3) as u32);
            c.bus.write32(gfx + man::DEFER, defer);
            c.bus.write32(defer + 0x68000 + 0x1cc4, r.next_u32());
            c.g[A0][0] = sx(gfx);
        });
    }

    #[test]
    fn ps2_graphics_man_pop_view_matches() {
        check(ee::s12::f_00376768__generated, ps2_graphics_man_pop_view, 200, &|c, seed| {
            let mut r = base(c, seed, 0x37_6768);
            let gfx = DATA;
            let defer = put_defer(c);
            fill(c, &mut r, gfx + man::CAM_MAT, 0x200);
            fill(c, &mut r, gfx + man::WORLD, 0xd0);
            fill(c, &mut r, gfx + man::VIEWS, 0x460);
            let cur = 1 + (seed % 2) as u32;
            c.bus.write32(gfx + man::CUR_VIEW, cur);
            let view = gfx + man::VIEWS + (cur - 1) * man::VIEW_STRIDE;
            let key = if seed % 3 == 0 {
                0xffff_ffff
            } else {
                r.next_u32()
            };
            c.bus.write32(view + vw::KEY, key);
            c.bus.write32(gfx + man::DEFER, defer);
            c.bus.write32(defer + 0x68000 + 0x1cc4, 0);
            c.g[A0][0] = sx(gfx);
        });
    }

    #[test]
    fn ps2_graphics_man_set_viewport_matches() {
        covered(
            ee::s12::f_00376a70__generated,
            ps2_graphics_man_set_viewport,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37_6a70, 4);
                let gfx = DATA;
                let defer = put_defer(c);
                fill(c, &mut r, gfx + man::CAM_MAT, 0x200);
                fill(c, &mut r, gfx + man::WORLD, 0xd0);
                fill(c, &mut r, gfx + man::VIEWS, 0x460);
                slot(c, 0x50, FAKE);
                slot(c, 0x58, FAKE + 0x10);
                c.bus.write32(gfx + man::DISPLAY, vt());
                c.bus.write32(gfx + man::CUR_VIEW, (seed % 2) as u32);
                c.bus.write32(gfx + man::DEFER, defer);
                let mode = match seed % 4 {
                    0 => 0,
                    1 => 2,
                    2 => 3,
                    _ => 0xffff_ffff,
                };
                c.bus.write32(gfx + man::CLAMP_MODE, mode);
                c.bus.write32(gfx + man::CLAMP_TOP, f(0.75));
                c.bus.write32(gfx + man::CLAMP_BOTTOM, f(0.5));
                for i in 0..4 {
                    c.f[12 + i] = r.edge_f32();
                }
                c.g[A0][0] = sx(gfx);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_copy_view_state_matches() {
        check(
            ee::s13::f_003950c0__generated,
            ps2_graphics_man_copy_view_state,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x39_50c0);
                let dst = DATA;
                let src = DATA + 0x4000;
                fill(c, &mut r, src, 0x100);
                fill(c, &mut r, dst, 0x100);
                c.g[A0][0] = sx(dst);
                c.g[A1][0] = sx(src);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_v27_matches() {
        check(ee::s12::f_00377278__generated, ps2_graphics_man_v27, 200, &|c, seed| {
            let mut r = base(c, seed, 0x37_7278);
            let gfx = DATA;
            let pos = DATA + 0x1000;
            let cam = DATA + 0x2000;
            let defer = put_defer(c);
            // The camera's third column scales the point along the view's z.
            identity(c, cam);
            c.bus.write32(cam + 0x28, f(0.25));
            identity(c, gfx + man::MAT5860);
            identity(c, gfx + man::WORLD);
            identity(c, gfx + man::LOCAL_MAT);
            identity(c, gfx + man::VIEW_MAT);
            c.bus.write32(gfx + man::VIEW_MAT + 0x28, f(0.25));
            let (z, dist) = if seed % 7 == 3 {
                (5.0f32, 1.0f32)
            } else {
                match seed % 4 {
                    0 => (0.0, 0.0),
                    1 => (0.0, 1.0),
                    2 => (0.0, 2.0),
                    _ => (0.0, 1.0),
                }
            };
            c.bus.write32(pos, 0);
            c.bus.write32(pos + 4, 0);
            c.bus.write32(pos + 8, f(z));
            c.bus.write32(pos + 0xc, f(1.0));
            c.bus.write32(gfx + man::CAMERA, cam);
            c.bus.write32(gfx + man::DEFER, defer);
            let idx = (seed % 2) as u32;
            c.bus.write32(gfx + man::CUR_VIEW, idx);
            c.bus
                .write32(gfx + man::MODE, if seed % 3 == 0 { 0x31 } else { 0x10 });
            c.bus
                .write32(gfx + man::TEX_READY, if seed % 5 == 0 { 0 } else { 1 });
            c.bus.write32(gfx + man::F58E8, f(100.0));
            c.bus.write32(GAME_GP - 0x279c, f(32767.5));
            c.bus.write32(GAME_GP - 0x2798, f(8388467.5));
            let d = if z == 0.0 { 6.25 * dist } else { 0.0 };
            let depth = if seed % 2 == 0 { f(d) } else { r.edge_f32() };
            let view = gfx + man::VIEWS + idx * man::VIEW_STRIDE;
            c.bus.write32(view + vw::DEPTH, depth);
            c.g[A0][0] = sx(gfx);
            c.g[A1][0] = sx(pos);
            c.f[12] = f(dist);
        });
    }
}
