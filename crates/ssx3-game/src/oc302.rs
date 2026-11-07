//! Staging wave 212 (oc302): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};

use crate::abi::addu;
use crate::guest::Addr;
use crate::ps2f::FpuF32;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_1b40, mesh_anim_ctor_2, "MeshAnim_ctor_2"),
];

// ---------------------------------------------------------------------------
// MeshAnim_ctor_2 (0x351b40)
// ---------------------------------------------------------------------------

/// The MeshAnim vtable (`D_0048F6D8`).
pub(crate) const MESH_ANIM_VTABLE: u32 = 0x0048_f6d8;

/// MeshAnim fields: vtable, model (world node), copied def values, the
/// per-tick spin, the bind-pose quaternions (+0x30) and positions (+0x3b0)
/// of each bone, the two def vectors (+0x390, +0x3a0).
pub(crate) const MESH_ANIM_VT: u32 = 0xc;
pub(crate) const MESH_ANIM_MODEL: u32 = 0x18;
pub(crate) const MESH_ANIM_F1C: u32 = 0x1c;
pub(crate) const MESH_ANIM_SPIN: u32 = 0x20;
pub(crate) const MESH_ANIM_F24: u32 = 0x24;
pub(crate) const MESH_ANIM_QUATS: u32 = 0x30;
pub(crate) const MESH_ANIM_VEC_A: u32 = 0x390;
pub(crate) const MESH_ANIM_VEC_B: u32 = 0x3a0;
pub(crate) const MESH_ANIM_POSITIONS: u32 = 0x3b0;
pub(crate) const MESH_ANIM_F6B0: u32 = 0x6b0;
pub(crate) const MESH_ANIM_F6B4: u32 = 0x6b4;

/// Mesh-animation definition fields.
pub(crate) const MESH_ANIM_DEF_F4: u32 = 0x4;
pub(crate) const MESH_ANIM_DEF_F8: u32 = 0x8;
pub(crate) const MESH_ANIM_DEF_SRC_SCALE: u32 = 0xc;
pub(crate) const MESH_ANIM_DEF_F10: u32 = 0x10;
pub(crate) const MESH_ANIM_DEF_VEC_A: u32 = 0x14;
pub(crate) const MESH_ANIM_DEF_VEC_B: u32 = 0x20;
pub(crate) const MESH_ANIM_DEF_ANGLE: u32 = 0x2c;
pub(crate) const MESH_ANIM_DEF_F30: u32 = 0x30;
pub(crate) const MESH_ANIM_DEF_MODEL: u32 = 0x34;
pub(crate) const MESH_ANIM_DEF_BONE: u32 = 0x38;

/// Model (world node) fields: flags, root matrix, skeleton, uniform scale.
pub(crate) const MESH_ANIM_NODE_FLAGS: u32 = 0x8;
pub(crate) const MESH_ANIM_NODE_ROOT: u32 = 0x10;
pub(crate) const MESH_ANIM_NODE_SKEL: u32 = 0x80;
pub(crate) const MESH_ANIM_NODE_SCALE: u32 = 0x84;

/// Skeleton: bone count and bone array (0x10 per bone: parent index at
/// +0, bind matrix pointer at +0xc).
pub(crate) const MESH_ANIM_SKEL_COUNT: u32 = 0x4;
pub(crate) const MESH_ANIM_SKEL_BONES: u32 = 0x8;
pub(crate) const MESH_ANIM_BONE_MTX: u32 = 0xc;

/// Owner object virtual slots: root matrix, "has bone matrices", bone
/// matrices, and the notification (vtable +8, called with 3).
pub(crate) const MESH_ANIM_OWNER_ROOT: u32 = 0xc0;
pub(crate) const MESH_ANIM_OWNER_HAS_MATS: u32 = 0xd0;
pub(crate) const MESH_ANIM_OWNER_MATS: u32 = 0xd8;
pub(crate) const MESH_ANIM_OWNER_NOTIFY: u32 = 0x8;

/// Frame layout: 24 local bone matrices at 0, scratch at 0x600.. (the
/// scale vector / Mat4_ToPosQuat output at 0x600, the copied matrix at
/// 0x610, the scaled bind matrix at 0x650, temporaries at 0x690/0x6a0, the
/// product at 0x6b0).
const MESH_ANIM_FRAME: u32 = 0x790;
const MESH_ANIM_SCRATCH: u32 = 0x600;

/// The array-constructor countdown (`v1` from 0x17 down to -1, a0 = -1),
/// with the delay-slot instruction `delay`, back-edge event checks at
/// `head`.
fn mesh_anim_ctor_2_countdown(c: &mut Cpu, head: u32, delay: fn(&mut Cpu)) {
    loop {
        c.cycles += 7;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let more = c.g[V1][0] != c.g[A0][0];
        delay(c);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `def->model.get()`: resolve the model handle (low byte: table entry,
/// rest: slot) through the world instance; v0 = the instance or 0. Runs
/// after `World_GetInstance` returned the world in v0.
fn mesh_anim_ctor_2_resolve_handle(c: &mut Cpu) {
    let def = r32(c, S4);
    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
    c.g[A1][0] = Addr(def + MESH_ANIM_DEF_MODEL).lw(c);
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx(((c.g[A1][0] & 0xff) as u32) << 2);
    c.g[V0][0] = addu(c.g[V0][0], c.g[A0][0]);
    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
    c.g[V0][0] = sx(r32(c, A1) >> 8);
    if c.g[V1][0] != 0 {
        c.cycles += 7;
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = addu(c.g[V0][0], c.g[A0][0]);
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[V0][0] = sx(r32(c, V1) >> 8);
        let live = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r32(c, V0) << 2);
        if live {
            return;
        }
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
}

/// Pick the object the bones hang off: `owner` (s0), replaced by the
/// owner of the def's model instance when the def names one. Also sets
/// the default root matrix (s2 = model + 0x10).
fn mesh_anim_ctor_2_pick_owner(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 6;
    let this = r32(c, S3);
    let def = r32(c, S4);
    c.g[V1][0] = Addr(this + MESH_ANIM_MODEL).lw(c);
    c.g[V0][0] = Addr(def + MESH_ANIM_DEF_MODEL).lw(c);
    c.g[S0][0] = c.g[S6][0];
    c.g[V0][0] = !c.g[V0][0];
    c.g[S2][0] = addu(c.g[V1][0], MESH_ANIM_NODE_ROOT as u64);
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 11;
    call_ptr(c, 0x002d_1bd8, 0x0035_1c5c)?; // World_GetInstance
    mesh_anim_ctor_2_resolve_handle(c);
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[S0][0] = c.g[V0][0];
        }
    }
    Ok(())
}

/// Virtual call on the bone owner (s0): `vt = *(s0 + 0xc)`, `this + delta`.
fn mesh_anim_ctor_2_owner_call(c: &mut Cpu, slot: u32, ret_to: u32) -> crate::call::Flow {
    let obj = c.g[S0][0];
    c.g[V1][0] = Addr((obj as u32).wrapping_add(0xc)).lw(c);
    let vt = r32(c, V1);
    c.g[A0][0] = Addr(vt.wrapping_add(slot)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(slot + 4)).lw(c);
    c.g[A0][0] = addu(obj, c.g[A0][0]);
    let method = r32(c, V0);
    call_ptr(c, method, ret_to)
}

/// Ask the owner for the root matrix (s2) and, when it has them, its bone
/// matrices (s1); a def bone index instead roots the skeleton at that
/// owner bone and builds local matrices (s1 = sp).
fn mesh_anim_ctor_2_owner_matrices(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 2;
    if c.g[S0][0] == 0 {
        return Ok(());
    }
    c.cycles += 13;
    mesh_anim_ctor_2_owner_call(c, MESH_ANIM_OWNER_ROOT, 0x0035_1ccc)?;
    c.g[S2][0] = c.g[V0][0];
    mesh_anim_ctor_2_owner_call(c, MESH_ANIM_OWNER_HAS_MATS, 0x0035_1ce4)?;
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 10;
    mesh_anim_ctor_2_owner_call(c, MESH_ANIM_OWNER_MATS, 0x0035_1d00)?;
    c.g[S1][0] = c.g[V0][0];
    let def = r32(c, S4);
    c.g[V1][0] = Addr(def + MESH_ANIM_DEF_BONE).lw(c);
    c.g[V0][0] = u64::MAX;
    let no_bone = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = sx(r32(c, V1) << 6);
    if !no_bone {
        c.cycles += 2;
        c.g[S2][0] = addu(c.g[S1][0], c.g[V0][0]);
        c.g[S1][0] = c.g[SP][0];
    }
    Ok(())
}

/// VU0 4x4 product `out = a * b` (rows of `b` combine the rows of `a`):
/// vf4-7 = a, vf8-11 = b, vf12-15 = result.
fn mesh_anim_ctor_2_mtx_mul(c: &mut Cpu, a: u32, b: u32, out: u32) {
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, a.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.lqc2(8 + i as usize, b.wrapping_add(0x10 * i));
    }
    for row in 0..4u8 {
        let bv = 8 + row;
        c.vu0().mula(0b1111, 4, Src::Bc(bv, X));
        c.vu0().madda(0b1111, 5, Src::Bc(bv, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(bv, Z));
        c.vu0().madd(0b1111, 12 + row, 7, Src::Bc(bv, W));
    }
    for i in 0..4u32 {
        c.sqc2(12 + i as usize, out.wrapping_add(0x10 * i));
    }
}

/// Copy a matrix through vf1-vf4.
fn mesh_anim_ctor_2_mtx_copy(c: &mut Cpu, from: u32, to: u32) {
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, from.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, to.wrapping_add(0x10 * i));
    }
}

/// One local bone matrix: the bone's bind matrix (bone at a2) with its
/// translation row scaled by the model scale (scratch 0x600), times
/// `parent` (the parent's local matrix, or the root), into the local
/// matrix at `dst` (via the product at a3 and the copy at t0). `from_root`
/// picks the original's FPU register order (f6/f7 swapped).
fn mesh_anim_ctor_2_bone(c: &mut Cpu, parent: u32, from_root: bool, dst: usize) {
    let s = sp(c);
    c.g[A0][0] = Addr(r32(c, A2).wrapping_add(MESH_ANIM_BONE_MTX)).lw(c);
    let bind = r32(c, A0);
    let (sx_reg, sy_reg) = if from_root { (6, 7) } else { (7, 6) };
    c.lwc1(sx_reg, s + 0x600);
    c.lwc1(sy_reg, s + 0x604);
    c.lwc1(5, s + 0x608);
    c.lwc1(4, s + 0x60c);
    // Rows 0-2 to the scratch matrix at 0x650 (through 0x690).
    c.lq(V0, bind);
    c.sq(V0, s + 0x690);
    c.sq(V0, s + 0x650);
    c.lq(V0, bind + 0x10);
    c.sq(V0, s + 0x690);
    c.sq(V0, s + 0x660);
    c.lq(V1, bind + 0x20);
    c.sq(V1, s + 0x690);
    c.sq(V1, s + 0x670);
    // Row 3 (translation) scaled lane by lane.
    c.lq(V0, bind + 0x30);
    c.sq(V0, s + 0x6a0);
    c.lwc1(1, s + 0x6a0);
    c.lwc1(2, s + 0x6a4);
    c.lwc1(3, s + 0x6a8);
    c.lwc1(0, s + 0x6ac);
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[sx_reg])).0;
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[sy_reg])).0;
    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[5])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[4])).0;
    c.swc1(1, s + 0x690);
    c.swc1(2, s + 0x694);
    c.swc1(3, s + 0x698);
    c.swc1(0, s + 0x69c);
    c.lq(V0, s + 0x690);
    c.sq(V0, s + 0x680);
    let (t4, a3, t0) = (r32(c, T4), r32(c, A3), r32(c, T0));
    mesh_anim_ctor_2_mtx_mul(c, parent, t4, a3);
    mesh_anim_ctor_2_mtx_copy(c, a3, t0);
    let to = r32(c, dst);
    mesh_anim_ctor_2_mtx_copy(c, t0, to);
    let skel = r32(c, S5);
    c.g[V0][0] = Addr(skel + MESH_ANIM_SKEL_COUNT).lw(c);
}

/// Without owner bone matrices (s1 == sp): build the local matrices of
/// all bones on the stack, each from its parent's (or the root s2), with
/// the model's uniform scale. Leaves v0 = the bone count.
fn mesh_anim_ctor_2_local_matrices(c: &mut Cpu) {
    c.cycles += 2;
    let skel = r32(c, S5);
    if c.g[S1][0] != c.g[SP][0] {
        c.g[V0][0] = Addr(skel + MESH_ANIM_SKEL_COUNT).lw(c);
        return;
    }
    c.cycles += 12;
    let s = sp(c);
    c.g[V0][0] = Addr(r32(c, S3) + MESH_ANIM_MODEL).lw(c);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    c.lwc1(0, r32(c, V0) + MESH_ANIM_NODE_SCALE);
    c.g[A2][0] = Addr(skel + MESH_ANIM_SKEL_BONES).lw(c);
    c.swc1(0, s + 0x608);
    c.swc1(1, s + 0x60c);
    c.swc1(0, s + 0x600);
    c.swc1(0, s + 0x604);
    c.g[V0][0] = Addr(skel + MESH_ANIM_SKEL_COUNT).lw(c);
    c.g[T3][0] = 0;
    if (c.g[V0][0] as i64) <= 0 {
        return;
    }
    c.cycles += 7;
    c.g[T5][0] = u64::MAX;
    c.g[T0][0] = sx(s + 0x610);
    c.g[A3][0] = sx(s + 0x6b0);
    c.g[T4][0] = sx(s + 0x650);
    c.g[T2][0] = c.g[SP][0];
    c.g[T1][0] = c.g[SP][0];
    loop {
        c.cycles += 3;
        c.g[A1][0] = Addr(r32(c, A2)).lw(c);
        let root = c.g[A1][0] == c.g[T5][0];
        c.g[A1][0] = sx(r32(c, A1) << 6);
        if !root {
            c.cycles += 77;
            c.g[A1][0] = addu(c.g[A1][0], c.g[SP][0]);
            let parent = r32(c, A1);
            mesh_anim_ctor_2_bone(c, parent, false, T1);
        } else {
            c.cycles += 75;
            let root_mtx = r32(c, S2);
            mesh_anim_ctor_2_bone(c, root_mtx, true, T2);
        }
        c.cycles += 6;
        c.g[T3][0] = sx(r32(c, T3).wrapping_add(1));
        c.g[T2][0] = sx(r32(c, T2).wrapping_add(0x40));
        c.g[T1][0] = sx(r32(c, T1).wrapping_add(0x40));
        c.g[V0][0] = ((c.g[T3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
        if !more {
            c.cycles += 1;
            let skel = r32(c, S5);
            c.g[V0][0] = Addr(skel + MESH_ANIM_SKEL_COUNT).lw(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0035_1d70);
        }
    }
}

/// Split each bone matrix (s1, 0x40 apart) with `Mat4_ToPosQuat` into the
/// bind position (+0x3b0) and quaternion (+0x30) arrays.
fn mesh_anim_ctor_2_store_bind_pose(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 2;
    let count = c.g[V0][0];
    c.g[S2][0] = 0;
    if (count as i64) <= 0 {
        return Ok(());
    }
    c.cycles += 2;
    c.g[S0][0] = c.g[S3][0];
    loop {
        c.cycles += 13;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = sx(sp(c) + MESH_ANIM_SCRATCH);
        call_ptr(c, 0x0031_b748, 0x0035_2014)?; // Mat4_ToPosQuat
        let s = sp(c);
        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
        c.lq(V0, s + 0x600);
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x40));
        c.lq(V1, s + 0x610);
        let slot = r32(c, S0);
        c.sq(V0, slot + MESH_ANIM_POSITIONS);
        c.sq(V1, slot + MESH_ANIM_QUATS);
        c.g[V0][0] = Addr(r32(c, S5) + MESH_ANIM_SKEL_COUNT).lw(c);
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S0][0] = sx(slot.wrapping_add(0x10));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0035_2008);
        }
    }
}

/// Tell the constructing owner (s6) we are attached: `owner->vt[1](3)`.
fn mesh_anim_ctor_2_notify_owner(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 2;
    c.g[A1][0] = 3;
    let owner = c.g[S6][0];
    if owner == 0 {
        return Ok(());
    }
    c.cycles += 5;
    c.g[V0][0] = Addr((owner as u32).wrapping_add(0xc)).lw(c);
    let vt = r32(c, V0);
    c.g[A0][0] = Addr(vt.wrapping_add(MESH_ANIM_OWNER_NOTIFY)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(MESH_ANIM_OWNER_NOTIFY + 4)).lw(c);
    c.g[A0][0] = addu(owner, c.g[A0][0]);
    let method = r32(c, V1);
    call_ptr(c, method, 0x0035_2058)
}

/// Copy a def xyz triple into the scratch quad (w = 0) and from there
/// (through v0) to `dst`.
fn mesh_anim_ctor_2_copy_xyz(c: &mut Cpu, from: u32, dst: u32, clear_w: bool) {
    let s = sp(c);
    c.lwc1(0, from);
    c.lwc1(1, from + 4);
    c.lwc1(2, from + 8);
    c.swc1(0, s + 0x600);
    c.swc1(1, s + 0x604);
    c.swc1(2, s + 0x608);
    if clear_w {
        Addr(s + 0x60c).sw(c, 0);
    }
    c.lq(V0, s + 0x600);
    if clear_w {
        Addr(s + 0x60c).sw(c, 0);
    }
    c.sq(V0, dst);
}

/// The two def vectors (+0x390, +0x3a0); with a source, the second one
/// gets `src->v * |src->s| * def->fC` added (VU0).
fn mesh_anim_ctor_2_def_vectors(c: &mut Cpu) {
    c.cycles += 19;
    let this = r32(c, S3);
    let def = r32(c, S4);
    mesh_anim_ctor_2_copy_xyz(c, def + MESH_ANIM_DEF_VEC_A, this + MESH_ANIM_VEC_A, true);
    mesh_anim_ctor_2_copy_xyz(c, def + MESH_ANIM_DEF_VEC_B, this + MESH_ANIM_VEC_B, false);
    let src = r32(c, S7);
    if c.g[S7][0] == 0 {
        return;
    }
    c.cycles += 21;
    let s = sp(c);
    c.lwc1(0, src + 0x30);
    c.f[0] = FpuF32(c.f[0]).abs().0;
    c.mfc1(V1, 0);
    c.lqc2(4, src + 0x10);
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s + 0x620);
    c.lq(V0, s + 0x620);
    c.lwc1(0, def + MESH_ANIM_DEF_SRC_SCALE);
    c.sq(V0, s + 0x610);
    c.mfc1(V1, 0);
    c.lqc2(4, s + 0x610);
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s + 0x620);
    c.lq(V0, s + 0x620);
    c.sq(V0, s + 0x600);
    c.lqc2(3, this + MESH_ANIM_VEC_B);
    c.lqc2(4, s + 0x600);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, this + MESH_ANIM_VEC_B);
}

/// `MeshAnim* MeshAnim_ctor_2(MeshAnim* self, int list, Node* model,
/// MeshAnimDef* def, Source* src, Owner* owner)` (0x351b40): builds the
/// world object (kind 0xb), installs the MeshAnim vtable, copies the def's
/// parameters (the spin is `def->angle` in degrees per second turned into
/// radians per tick), computes every bone's bind matrix (from the owner's
/// bone matrices when it provides them, otherwise the skeleton's bind
/// matrices chained from the root and scaled by the model scale) and
/// stores them as positions and quaternions, notifies the owner, sets the
/// two def vectors (the second pushed along the source), updates the model
/// flags and randomises the nodes. Returns `self`.
pub fn mesh_anim_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, MESH_ANIM_FRAME);
        frame.save(c, S4, 0x730);
        frame.save(c, S3, 0x740);
        c.g[S4][0] = c.g[A3][0];
        frame.save(c, S6, 0x710);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S7, 0x700);
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S0, 0x770);
        frame.save(c, S1, 0x760);
        c.g[S7][0] = c.g[T0][0];
        frame.save(c, S2, 0x750);
        c.g[S6][0] = c.g[T1][0];
        frame.save(c, S5, 0x720);
        c.g[A2][0] = 0xb;
        frame.save_ra(c, 0x6f0);
        frame.save_fpr(c, 20, 0x780);
        Callee {
            addr: 0x0034_fb00,
            f: ee::s11::WorldObj_ctor,
        }
        .call(c, 0x0035_1b88)?;

        // Vtable, then the (empty) constructors of the bone vector arrays.
        let this = r32(c, S3);
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = sx(MESH_ANIM_VTABLE);
        c.g[V1][0] = 0x17;
        Addr(this + MESH_ANIM_VT).sw(c, sx(MESH_ANIM_VTABLE));
        mesh_anim_ctor_2_countdown(c, 0x0035_1ba0, |c| c.g[V0][0] = 0x17);

        // Copied def parameters; the spin per tick.
        c.cycles += 25;
        let def = r32(c, S4);
        c.g[A0][0] = Addr(def + MESH_ANIM_DEF_F4).lw(c);
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 0x17;
        let f4 = c.g[A0][0];
        Addr(this + MESH_ANIM_F6B0).sw(c, f4);
        let gp = r32(c, GP);
        c.lwc1(2, gp.wrapping_sub(0x2a8c)); // degrees to radians
        c.lwc1(0, def + MESH_ANIM_DEF_F10);
        c.swc1(0, this + MESH_ANIM_F1C);
        c.lwc1(1, def + MESH_ANIM_DEF_F30);
        c.swc1(1, this + MESH_ANIM_F24);
        c.lwc1(0, def + MESH_ANIM_DEF_F8);
        c.swc1(0, this + MESH_ANIM_F6B4);
        c.lwc1(20, def + MESH_ANIM_DEF_ANGLE);
        c.f[20] = (FpuF32(c.f[20]) * FpuF32(c.f[2])).0;
        call_ptr(c, 0x002d_1c70, 0x0035_1c04)?; // World_GetSecondsPerTick
        let this = r32(c, S3);
        c.g[V0][0] = Addr(this + MESH_ANIM_MODEL).lw(c);
        c.g[A0][0] = u64::MAX;
        c.f[20] = (FpuF32(c.f[20]) * FpuF32(c.f[0])).0;
        c.g[V1][0] = 0x17;
        c.swc1(20, this + MESH_ANIM_SPIN);
        c.g[S5][0] = Addr(r32(c, V0) + MESH_ANIM_NODE_SKEL).lw(c);
        mesh_anim_ctor_2_countdown(c, 0x0035_1c20, |c| c.g[S1][0] = c.g[SP][0]);

        // Bone matrices: the owner's, or local ones built on the stack.
        mesh_anim_ctor_2_pick_owner(c)?;
        mesh_anim_ctor_2_owner_matrices(c)?;
        mesh_anim_ctor_2_local_matrices(c);
        mesh_anim_ctor_2_store_bind_pose(c)?;
        mesh_anim_ctor_2_notify_owner(c)?;
        mesh_anim_ctor_2_def_vectors(c);

        // Model flags: clear 2 and 0x60, set 4.
        c.cycles += 28;
        let this = r32(c, S3);
        c.g[A2][0] = Addr(this + MESH_ANIM_MODEL).lw(c);
        c.g[V1][0] = 0xffff_ffff_ffff_fffd;
        c.g[A1][0] = 0xffff_ffff_ffff_ff9f;
        let node = r32(c, A2);
        c.g[V0][0] = Addr(node + MESH_ANIM_NODE_FLAGS).lw(c);
        c.g[A0][0] = c.g[S3][0];
        c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | 4;
        let flags = c.g[V0][0];
        Addr(node + MESH_ANIM_NODE_FLAGS).sw(c, flags);
        c.g[V1][0] = Addr(this + MESH_ANIM_MODEL).lw(c);
        let node = r32(c, V1);
        c.g[V0][0] = Addr(node + MESH_ANIM_NODE_FLAGS).lw(c);
        c.g[V0][0] &= c.g[A1][0];
        let flags = c.g[V0][0];
        Addr(node + MESH_ANIM_NODE_FLAGS).sw(c, flags);
        Callee {
            addr: 0x0035_2230,
            f: ee::s11::MeshAnim_RandomizeNodes,
        }
        .call(c, 0x0035_2134)?;

        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x770);
        frame.restore(c, S1, 0x760);
        frame.restore(c, S2, 0x750);
        frame.restore(c, S3, 0x740);
        frame.restore(c, S4, 0x730);
        frame.restore(c, S5, 0x720);
        frame.restore(c, S6, 0x710);
        frame.restore(c, S7, 0x700);
        frame.restore_ra(c, 0x6f0);
        frame.restore_fpr(c, 20, 0x780);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{Rng, DATA, GAME_GP};

    /// Fixture layout (all inside DATA..DATA+0x4000, away from the fakes' log).
    const MA_SELF: u32 = DATA;
    const MA_NODE: u32 = DATA + 0x800;
    const MA_SKEL: u32 = DATA + 0x900;
    const MA_BONES: u32 = DATA + 0xa00;
    const MA_BIND: u32 = DATA + 0xc00;
    const MA_DEF: u32 = DATA + 0x1000;
    const MA_SRC: u32 = DATA + 0x1100;
    const MA_OWNER: u32 = DATA + 0x1200;
    const MA_OWNER_VT: u32 = DATA + 0x1300;
    const MA_MODEL_OWNER: u32 = DATA + 0x1400;
    const MA_MODEL_OWNER_VT: u32 = DATA + 0x1500;
    const MA_WORLD: u32 = DATA + 0x2000;
    const MA_RAND_CFG: u32 = DATA + 0x2100;
    const MA_INST: u32 = DATA + 0x3000;
    const MA_INST_HDR: u32 = DATA + 0x3100;
    const MA_TABLE: u32 = DATA + 0x3200;
    const MA_ENTRIES: u32 = DATA + 0x3300;
    const MA_SLOTS: u32 = DATA + 0x3500;
    const MA_MODELS: u32 = DATA + 0x3800;
    /// A real leaf returning 0 (for the "has bone matrices" slot).
    const MA_RETURNS_ZERO: u32 = 0x0014_06c0;

    /// An owner vtable: slots 8 (notify), 0xc0 (root), 0xd0 (has matrices),
    /// 0xd8 (matrices) with random deltas and fake methods; `no_mats` makes
    /// the "has matrices" slot return 0.
    fn mesh_anim_ctor_2_owner(c: &mut Cpu, r: &mut Rng, obj: u32, vt: u32, no_mats: bool) {
        c.bus.write32(obj + 0xc, vt);
        for (i, off) in [0x8u32, 0xc0, 0xd0, 0xd8].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
        }
        if no_mats {
            c.bus.write32(vt + 0xd4, MA_RETURNS_ZERO);
        }
    }

    /// Memory for one seed: the object, its model node (focus flag set so
    /// WorldObj_ctor takes its short path), a skeleton of 0-5 bones, the
    /// def (model handle -1 or resolving to an instance with or without an
    /// owner), source and owner (each sometimes null), the world and the
    /// handle tables. Returns (owner, src).
    fn mesh_anim_ctor_2_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) -> (u32, u32) {
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        for i in 0..0x1b0 {
            w(c, MA_SELF + 4 * i, r.next_u32());
        }
        // Model node.
        w(c, MA_NODE + MESH_ANIM_NODE_FLAGS, 0x100 | (r.next_u32() & !0x100));
        for i in 0..16 {
            let v = r.f32(3.0);
            w(c, MA_NODE + MESH_ANIM_NODE_ROOT + 4 * i, v);
        }
        w(c, MA_NODE + MESH_ANIM_NODE_SKEL, MA_SKEL);
        let scale = r.f32(2.0);
        w(c, MA_NODE + MESH_ANIM_NODE_SCALE, scale);
        // Skeleton: parents are -1 or an earlier bone.
        let count = seed % 6;
        w(c, MA_SKEL + MESH_ANIM_SKEL_COUNT, count);
        w(c, MA_SKEL + MESH_ANIM_SKEL_BONES, MA_BONES);
        for i in 0..5u32 {
            let parent = if i == 0 || r.next_u32() % 3 == 0 {
                u32::MAX
            } else {
                r.next_u32() % i
            };
            w(c, MA_BONES + 0x10 * i, parent);
            w(c, MA_BONES + 0x10 * i + MESH_ANIM_BONE_MTX, MA_BIND + 0x40 * i);
            for k in 0..16 {
                let v = r.f32(2.0);
                w(c, MA_BIND + 0x40 * i + 4 * k, v);
            }
        }
        // Def.
        for i in 0..0xd {
            let v = r.f32(100.0);
            w(c, MA_DEF + 4 * i, v);
        }
        let handle = match seed % 3 {
            0 => u32::MAX,
            _ => (r.next_u32() % 4) << 8 | r.next_u32() % 4,
        };
        w(c, MA_DEF + MESH_ANIM_DEF_MODEL, handle);
        let bone = if (seed / 3) % 2 == 0 {
            u32::MAX
        } else {
            r.next_u32() % 24
        };
        w(c, MA_DEF + MESH_ANIM_DEF_BONE, bone);
        // Source.
        let src = if seed % 4 == 0 { 0 } else { MA_SRC };
        for i in 0..0x10 {
            let v = r.f32(10.0);
            w(c, MA_SRC + 4 * i, v);
        }
        // Owner (the constructing object).
        let owner = if (seed / 2) % 3 == 0 { 0 } else { MA_OWNER };
        mesh_anim_ctor_2_owner(c, r, MA_OWNER, MA_OWNER_VT, seed % 5 == 0);
        mesh_anim_ctor_2_owner(c, r, MA_MODEL_OWNER, MA_MODEL_OWNER_VT, seed % 7 == 0);
        // World: seconds per tick, degrees-to-radians literal, the random
        // node config of MeshAnim_RandomizeNodes.
        w(c, GAME_GP - 0x848, MA_WORLD);
        w(c, MA_WORLD + 0x14, (1.0f32 / 60.0).to_bits());
        w(c, GAME_GP - 0x2a8c, 0.017_453_292f32.to_bits());
        w(c, GAME_GP + 0x2a74, MA_RAND_CFG);
        w(c, MA_RAND_CFG + 0x10, 1 + r.next_u32() % 50);
        let v = r.f32(4.0);
        w(c, MA_RAND_CFG + 0x14, v);
        // World instance -> header -> entry table -> slot tables -> models.
        w(c, GAME_GP + 0x16c8, MA_INST);
        w(c, MA_INST, MA_INST_HDR);
        w(c, MA_INST_HDR + 8, MA_TABLE);
        for i in 0..4u32 {
            let entry = MA_ENTRIES + 0x40 * i;
            w(c, MA_TABLE + 4 * i, if r.next_u32() % 5 == 0 { 0 } else { entry });
            w(c, entry + 0x1c, MA_SLOTS + 0x40 * i);
            for j in 0..4u32 {
                let model = MA_MODELS + 0x20 * (4 * i + j);
                let word = match r.next_u32() % 5 {
                    0 => r.next_u32() & 0xff,
                    _ => (model >> 2) << 8 | (r.next_u32() & 0xff),
                };
                w(c, MA_SLOTS + 0x40 * i + 4 * j, word);
                let mo = if r.next_u32() % 3 == 0 { 0 } else { MA_MODEL_OWNER };
                w(c, model + 0xc, mo);
            }
        }
        (owner, src)
    }

    /// Argument registers: self, list 0, model node, def, source, owner.
    fn mesh_anim_ctor_2_args(c: &mut Cpu, owner: u32, src: u32) {
        c.pc = 0x0035_1b40;
        c.g[A0][0] = sx(MA_SELF);
        c.g[A1][0] = 0;
        c.g[A2][0] = sx(MA_NODE);
        c.g[A3][0] = sx(MA_DEF);
        c.g[T0][0] = sx(src);
        c.g[T1][0] = sx(owner);
    }

    /// MeshAnim_ctor_2: owner bone matrices or local ones from the
    /// skeleton, def model handle (none / missing entry / dead slot /
    /// instance with or without owner), def bone index, source and owner
    /// present or null; fake owner methods unwind at random calls.
    #[test]
    fn mesh_anim_ctor_2_matches() {
        covered(ee::s11::f_00351b40__generated, mesh_anim_ctor_2, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x35_1b40, 4);
            let (owner, src) = mesh_anim_ctor_2_fixture(c, &mut r, seed);
            mesh_anim_ctor_2_args(c, owner, src);
        });
    }

    /// The same with events due inside the loops (array countdowns, the
    /// local bone matrices, the bind-pose split).
    #[test]
    fn mesh_anim_ctor_2_events_match() {
        check_events(ee::s11::f_00351b40__generated, mesh_anim_ctor_2, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x35_1b41, 0);
            let (owner, src) = mesh_anim_ctor_2_fixture(c, &mut r, seed);
            let mode = schedule(c, &mut r, seed, 0x0035_1b40);
            // Spread the first due event over the whole body (identically
            // for both runs).
            let k = 40 + (r.next_u32() % 900) as u64;
            if mode != crate::difftest::events::Events::Off {
                c.next_event = c.cycles + k;
            }
            mesh_anim_ctor_2_args(c, owner, src);
            mode
        });
    }
}
