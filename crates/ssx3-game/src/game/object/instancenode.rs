//! `src/object/instancenode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintplayer::lh;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p as p_2;
pub(crate) use crate::game::render::lightman::addu;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0034_fb90, world_obj_ctor_2, "WorldObj_ctor_2"),
    (
        0x0034_fe28,
        world_obj_replace_with_dead_node,
        "WorldObj_ReplaceWithDeadNode",
    ),
    (0x0034_fe90, world_obj_save, "WorldObj_Save"),
    (
        0x0035_0698,
        instance_has_anim_nodes,
        "Instance_HasAnimNodes",
    ),
    (
        0x0034_fed8,
        instance_get_node_matrix,
        "Instance_GetNodeMatrix",
    ),
    (0x0034_fbf0, world_obj_dtor, "WorldObj_dtor"),
    (0x0034_fd90, world_obj_kill, "WorldObj_Kill"),
];

// ---------------------------------------------------------------------------
// Callees and field offsets.

pub(crate) mod callee {

    pub(crate) use crate::game::render::lightman::callee::SYNC_PROCESS_SAVE;
}

/// `void WorldObj_Save(WorldObj *self, void *stream)` (0x0034fe90):
/// `SyncProcess_Save(self, stream)`, then `stream->vt[6](stream + delta,
/// self)`.
pub fn world_obj_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        let stream = abi::arg(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(stream);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::SYNC_PROCESS_SAVE.call(c, 0x0034_feac)?;
        c.g[V0][0] = c.ldw(c.g[S0][0] as u32);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 0x30));
        c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0x34));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, target, 0x0034_fec4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool Instance_HasAnimNodes(Instance* self)` (0x350698): true when any of
/// the anim-nodes list's entries (`+0x80`, count `+4`, entries `+8`, stride
/// 0x10) has a non-zero field at `+8`.
pub fn instance_has_anim_nodes(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x80).lw(c);
    c.g[A0][0] = 0;
    c.g[A1][0] = Addr(r32(c, V0)).off(4).lw(c);
    let none = (c.g[A1][0] as i64) <= 0;
    c.g[V1][0] = Addr(r32(c, V0)).off(8).lw(c);
    if none {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, V1)).off(8).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        if !empty {
            c.cycles += 2;
            let ra = r32(c, RA);
            c.g[V0][0] = 1;
            c.pc = ra;
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0035_06b0);
        }
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.g[V0][0] = 0;
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Process nodes.

/// `WorldObj* WorldObj_ctor_2(WorldObj* self, int a1, Stream* stream)`
/// (0x34fb90): construct the process, install the world-object vtable and ask
/// the stream for the object's id.
pub fn world_obj_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0035_48c0, 0x0034_fbac)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1c80));
        Addr(p(c.g[S0][0], 0xc)).sw(c, c.g[V1][0]);
        c.g[A2][0] = lw(c, c.g[S1][0] as u32);
        c.g[A0][0] = Addr(p(c.g[A2][0], 0x20)).lh(c);
        c.g[V0][0] = lw(c, p(c.g[A2][0], 0x24));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_fbd0)?;
        Addr(p(c.g[S0][0], 0x18)).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `WorldObj_ReplaceWithDeadNode(WorldObj* self)` (0x34fe28): tell the node
/// dtor (vtable slot 1) to release, allocate a 0x1c-byte dead node named
/// `D_0048E738` and construct it with the object's node index.
pub fn world_obj_replace_with_dead_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        let this = c.g[A0][0];
        c.g[A2][0] = this;
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        let null = c.g[A2][0] == 0;
        c.g[S0][0] = lw(c, p_2(this as u32, 0x18));
        if !null {
            c.cycles += 6;
            c.g[V0][0] = lw(c, p_2(this as u32, 0xc));
            c.g[A1][0] = 3;
            c.g[A0][0] = lh(c, p_2(c.g[V0][0] as u32, 8));
            c.g[V1][0] = lw(c, p_2(c.g[V0][0] as u32, 0xc));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0034_fe58)?;
        }
        c.cycles += 13;
        c.g[A1][0] = 0x0049_0000;
        c.g[A0][0] = 0x1c;
        c.g[A1][0] = 0x0048_e738;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0034_fe70)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0035_06d8, 0x0034_fe7c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The body of `WorldObj_BuildMatrices`' loop: compose bone matrix `mat` with
/// the scale at `sp+0..0xc` (the two paths of the original keep the first two
/// scale components in opposite FPU registers, hence `swap`) and copy the
/// product through the scratch matrices at `sp+0x50`/`sp+0x80`/`sp+0x90` to
/// `out` -- the VU0 ops are issued exactly as the original's COP2 macros.
pub(crate) fn world_obj_bone(c: &mut Cpu, mat: u32, swap: bool) {
    let sp = c.g[SP][0] as u32;
    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
    if swap {
        c.lwc1(6, sp);
        c.lq(2, mat);
        c.lwc1(7, sp.wrapping_add(0x4));
        c.sq(2, sp.wrapping_add(0x90));
        c.sq(2, sp.wrapping_add(0x50));
        c.lwc1(5, sp.wrapping_add(0x8));
        c.lq(2, mat.wrapping_add(0x10));
        c.lwc1(4, sp.wrapping_add(0xc));
        c.sq(2, sp.wrapping_add(0x90));
        c.sq(2, sp.wrapping_add(0x60));
        c.lq(3, mat.wrapping_add(0x20));
        c.sq(3, sp.wrapping_add(0x90));
        c.sq(3, sp.wrapping_add(0x70));
        c.lq(2, mat.wrapping_add(0x30));
        c.sq(2, sp.wrapping_add(0xa0));
        c.lwc1(1, sp.wrapping_add(0xa0));
        c.lwc1(2, sp.wrapping_add(0xa4));
        c.lwc1(3, sp.wrapping_add(0xa8));
        c.mul_s(1, 1, 6);
        c.lwc1(0, sp.wrapping_add(0xac));
        c.mul_s(2, 2, 7);
    } else {
        c.lwc1(7, sp);
        c.lq(2, mat);
        c.lwc1(6, sp.wrapping_add(0x4));
        c.sq(2, sp.wrapping_add(0x90));
        c.sq(2, sp.wrapping_add(0x50));
        c.lwc1(5, sp.wrapping_add(0x8));
        c.lq(2, mat.wrapping_add(0x10));
        c.lwc1(4, sp.wrapping_add(0xc));
        c.sq(2, sp.wrapping_add(0x90));
        c.sq(2, sp.wrapping_add(0x60));
        c.lq(3, mat.wrapping_add(0x20));
        c.sq(3, sp.wrapping_add(0x90));
        c.sq(3, sp.wrapping_add(0x70));
        c.lq(2, mat.wrapping_add(0x30));
        c.sq(2, sp.wrapping_add(0xa0));
        c.lwc1(1, sp.wrapping_add(0xa0));
        c.lwc1(2, sp.wrapping_add(0xa4));
        c.lwc1(3, sp.wrapping_add(0xa8));
        c.mul_s(1, 1, 7);
        c.lwc1(0, sp.wrapping_add(0xac));
        c.mul_s(2, 2, 6);
    }
    c.mul_s(3, 3, 5);
    c.mul_s(0, 0, 4);
    c.swc1(1, sp.wrapping_add(0x90));
    c.swc1(2, sp.wrapping_add(0x94));
    c.swc1(3, sp.wrapping_add(0x98));
    c.swc1(0, sp.wrapping_add(0x9c));
    c.lq(2, sp.wrapping_add(0x90));
    c.sq(2, sp.wrapping_add(0x80));
    let src = c.g[A1][0] as u32;
    let out = c.g[T0][0] as u32;
    let m2 = c.g[T5][0] as u32;
    let dst = c.g[T1][0] as u32;
    let dst2 = c.g[T2][0] as u32;
    c.lqc2(4, src);
    c.lqc2(5, src.wrapping_add(0x10));
    c.lqc2(6, src.wrapping_add(0x20));
    c.lqc2(7, src.wrapping_add(0x30));
    c.lqc2(8, m2);
    c.lqc2(9, m2.wrapping_add(0x10));
    c.lqc2(10, m2.wrapping_add(0x20));
    c.lqc2(11, m2.wrapping_add(0x30));
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
    c.sqc2(12, out);
    c.sqc2(13, out.wrapping_add(0x10));
    c.sqc2(14, out.wrapping_add(0x20));
    c.sqc2(15, out.wrapping_add(0x30));
    c.lqc2(1, out);
    c.lqc2(2, out.wrapping_add(0x10));
    c.lqc2(3, out.wrapping_add(0x20));
    c.lqc2(4, out.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
    c.lqc2(1, dst);
    c.lqc2(2, dst.wrapping_add(0x10));
    c.lqc2(3, dst.wrapping_add(0x20));
    c.lqc2(4, dst.wrapping_add(0x30));
    c.sqc2(1, dst2);
    c.sqc2(2, dst2.wrapping_add(0x10));
    c.sqc2(3, dst2.wrapping_add(0x20));
    c.sqc2(4, dst2.wrapping_add(0x30));
}

/// `WorldObj_BuildMatrices` (0x00350288): build the world matrices of a
/// world object's skeleton: for every bone (0x10-byte records at
/// `skel+8`, count at `skel+4`) compose the bone matrix with the object's
/// uniform scale (`skel+0x84`) over either its parent's already-built
/// matrix `out[bone->parent]` (`+0`) or the caller's `parent` matrix, and
/// store the result in `out[i]`.
pub fn world_obj_build_matrices(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1); // f1 = 1.0
    c.g[T7][0] = c.g[A1][0]; // the caller's parent matrix
    c.g[T6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x80)); // skeleton
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x84)); // uniform scale
    c.g[A3][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x8)); // bones
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
    c.swc1(0, c.g[SP][0] as u32);
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x4)); // bone count
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[T4][0] = 0u64;
    if !empty {
        c.cycles += 7;
        c.g[T2][0] = c.g[A2][0]; // out
        c.g[T8][0] = 0xffff_ffff_ffff_ffff;
        c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[T5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[T3][0] = c.g[T2][0];
        loop {
            c.cycles += 3;
            c.g[A1][0] = c.lw(c.g[A3][0] as u32); // bone->parent
            let no_parent = c.g[A1][0] == c.g[T8][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
            if no_parent {
                c.cycles += 75;
                world_obj_bone(c, c.g[T7][0] as u32, true);
            } else {
                c.cycles += 77;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                world_obj_bone(c, c.g[A1][0] as u32, false);
            }
            c.cycles += 6;
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x40));
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_02e0);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
    c.pc = ra;
}

pub(crate) mod inode {

    pub const OWNER: u32 = 0xc;

    pub const WORLD: u32 = 0x10;

    pub const SKELETON: u32 = 0x80;

    pub const SCALE: u32 = 0x84;

    /// Skeleton: node table (0x10-byte entries `{ int parent; ...; Mat*
    /// local at +0xc }`).
    pub const NODES: u32 = 0x8;

    /// Owner v-table slots: `HasNodeMatrix(node)`, `GetNodeMatrix(node)`,
    /// `GetWorldMatrix()`.
    pub const VT_HAS_NODE: u32 = 0xd0;

    pub const VT_NODE_MATRIX: u32 = 0xe8;

    pub const VT_WORLD: u32 = 0xc0;
}

// ---------------------------------------------------------------------------
// Instance_GetNodeMatrix

/// Copy a 4x4 matrix through vf1-vf4 (`lqc2` all four rows, then `sqc2`).
pub(crate) fn vu0_copy_matrix(c: &mut Cpu, from: u32, to: u32) {
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, from.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, to.wrapping_add(0x10 * i));
    }
}

/// `out = b * a` row by row on VU0 (vf4-vf7 = a, vf8-vf11 = b, result in
/// vf12-vf15): each row of b combines a's rows by its x, y, z, w.
pub(crate) fn vu0_mul_matrix(c: &mut Cpu, a: u32, b: u32, out: u32) {
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, a.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.lqc2(8 + i as usize, b.wrapping_add(0x10 * i));
    }
    for i in 0..4u8 {
        c.vu0().mula(0b1111, 4, Src::Bc(8 + i, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8 + i, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8 + i, Z));
        c.vu0().madd(0b1111, 12 + i, 7, Src::Bc(8 + i, W));
    }
    for i in 0..4u32 {
        c.sqc2(12 + i as usize, out.wrapping_add(0x10 * i));
    }
}

/// Virtual call on the owner in a1 (v-table in v1): `{ i16 delta; fn }` at
/// `vt + slot`.
pub(crate) fn owner_vcall(c: &mut Cpu, slot: u32, ret: u32) -> Flow {
    let vt = r32(c, V1);
    c.g[A0][0] = c.lh(vt.wrapping_add(slot));
    c.g[V0][0] = c.lw(vt.wrapping_add(slot + 4));
    let f = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, f, ret)
}

/// `void Instance_GetNodeMatrix(Instance* inst, int node, Mat4* out)`
/// (0x34fed8): the world matrix of one skeleton node. An instance with an
/// owner object asks it (the owner's own node matrix when it has one, else
/// its world matrix stands in for the instance's). For node -1 the result
/// is the world matrix; otherwise the node's local matrices are chained up
/// to the root (translations scaled by the instance scale) and then
/// multiplied by the world matrix.
pub fn instance_get_node_matrix(c: &mut Cpu) {
    guest(c, |c| {
        use inode::*;
        c.cycles += 13;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S0, 0x150);
        frame.save(c, S1, 0x140);
        frame.save(c, S2, 0x130);
        frame.save(c, S3, 0x120);
        frame.save_ra(c, 0x110);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        let inst = r32(c, S1);
        let out = r32(c, S2);
        c.g[T1][0] = sx(inst.wrapping_add(WORLD));
        c.g[A1][0] = c.lw(inst.wrapping_add(OWNER));
        c.g[S3][0] = c.lw(inst.wrapping_add(SKELETON));
        if c.g[A1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            let world_only = c.g[S0][0] == c.g[V0][0];
            c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(OWNER));
            if !world_only {
                c.cycles += 6;
                owner_vcall(c, VT_HAS_NODE, 0x0034_ff28)?;
                let has = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S0][0];
                if has {
                    // The owner's own node matrix.
                    c.cycles += 16;
                    c.g[A2][0] = c.lw(inst.wrapping_add(OWNER));
                    let owner = r32(c, A2);
                    c.g[V1][0] = c.lw(owner.wrapping_add(OWNER));
                    let vt = r32(c, V1);
                    c.g[A0][0] = c.lh(vt.wrapping_add(VT_NODE_MATRIX));
                    c.g[V0][0] = c.lw(vt.wrapping_add(VT_NODE_MATRIX + 4));
                    let f = r32(c, V0);
                    c.g[A0][0] = sx(owner.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, f, 0x0034_ff48)?;
                    vu0_copy_matrix(c, r32(c, V0), out);
                    frame.restore(c, S0, 0x150);
                    return node_matrix_epilogue(c, frame);
                }
                c.cycles += 2;
                c.g[A1][0] = c.lw(inst.wrapping_add(OWNER));
                c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(OWNER));
            }
            c.cycles += 5;
            owner_vcall(c, VT_WORLD, 0x0034_ff88)?;
            c.g[T1][0] = c.g[V0][0];
        }

        c.cycles += 2;
        c.g[T0][0] = c.lw(r32(c, S3).wrapping_add(NODES));
        if (c.g[S0][0] as i64) < 0 {
            c.cycles += 9;
            vu0_copy_matrix(c, r32(c, T1), out);
            frame.restore(c, S0, 0x150);
            return node_matrix_epilogue(c, frame);
        }

        // Start from the node's local matrix with its translation scaled by
        // (s, s, s, 1).
        let s = sp(c);
        c.cycles += 43;
        c.lwc1(0, inst.wrapping_add(SCALE));
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 4);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.swc1(0, s.wrapping_add(0x8));
        c.swc1(1, s.wrapping_add(0xc));
        c.swc1(0, s);
        c.swc1(0, s.wrapping_add(0x4));
        let node = r32(c, V1);
        c.g[V0][0] = c.lw(node.wrapping_add(0xc));
        vu0_copy_matrix(c, r32(c, V0), out);
        c.lq(V0, out.wrapping_add(0x30));
        c.g[T3][0] = sx(s.wrapping_add(0x50));
        c.lwc1(0, s);
        c.sq(V0, s.wrapping_add(0x20));
        c.g[A0][0] = u64::MAX;
        c.lwc1(4, s.wrapping_add(0x4));
        c.lwc1(1, s.wrapping_add(0x20));
        c.lwc1(2, s.wrapping_add(0x24));
        c.mul_s(1, 1, 0);
        c.lwc1(3, s.wrapping_add(0x28));
        c.mul_s(2, 2, 4);
        c.lwc1(5, s.wrapping_add(0x8));
        c.lwc1(0, s.wrapping_add(0x2c));
        c.lwc1(4, s.wrapping_add(0xc));
        c.mul_s(3, 3, 5);
        c.swc1(1, s.wrapping_add(0x10));
        c.mul_s(0, 0, 4);
        c.swc1(2, s.wrapping_add(0x14));
        c.swc1(3, s.wrapping_add(0x18));
        c.swc1(0, s.wrapping_add(0x1c));
        c.lq(V0, s.wrapping_add(0x10));
        c.sq(V0, out.wrapping_add(0x30));
        c.g[S0][0] = c.lw(node);
        let root = c.g[S0][0] == c.g[A0][0];
        c.g[T5][0] = sx(s.wrapping_add(0x10));
        if !root {
            // Chain the parents: out = out * parent (scaled translation).
            c.cycles += 6;
            c.g[A3][0] = sx(s.wrapping_add(0x30));
            c.g[A2][0] = sx(s.wrapping_add(0xd0));
            c.g[T2][0] = sx(s.wrapping_add(0x70));
            c.g[T4][0] = u64::MAX;
            c.g[A1][0] = sx((c.g[S0][0] as u32) << 4);
            loop {
                c.cycles += 78;
                c.lwc1(6, s);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.lwc1(7, s.wrapping_add(0x4));
                c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(0xc));
                let m = r32(c, A0);
                c.lwc1(5, s.wrapping_add(0x8));
                c.lq(V1, m);
                c.lwc1(4, s.wrapping_add(0xc));
                c.sq(V1, s.wrapping_add(0xb0));
                c.sq(V1, s.wrapping_add(0x70));
                c.lq(V0, m.wrapping_add(0x10));
                c.sq(V0, s.wrapping_add(0xb0));
                c.sq(V0, s.wrapping_add(0x80));
                c.lq(V1, m.wrapping_add(0x20));
                c.sq(V1, s.wrapping_add(0xb0));
                c.sq(V1, s.wrapping_add(0x90));
                c.lq(V0, m.wrapping_add(0x30));
                c.sq(V0, s.wrapping_add(0xc0));
                c.lwc1(1, s.wrapping_add(0xc0));
                c.lwc1(2, s.wrapping_add(0xc4));
                c.lwc1(3, s.wrapping_add(0xc8));
                c.mul_s(1, 1, 6);
                c.lwc1(0, s.wrapping_add(0xcc));
                c.mul_s(2, 2, 7);
                c.mul_s(3, 3, 5);
                c.mul_s(0, 0, 4);
                c.swc1(1, s.wrapping_add(0xb0));
                c.swc1(2, s.wrapping_add(0xb4));
                c.swc1(3, s.wrapping_add(0xb8));
                c.swc1(0, s.wrapping_add(0xbc));
                c.lq(V0, s.wrapping_add(0xb0));
                c.sq(V0, s.wrapping_add(0xa0));
                vu0_mul_matrix(c, s.wrapping_add(0x70), out, s.wrapping_add(0xd0));
                vu0_copy_matrix(c, s.wrapping_add(0xd0), s.wrapping_add(0x30));
                vu0_copy_matrix(c, s.wrapping_add(0x30), out);
                c.g[S0][0] = c.lw(r32(c, A1));
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 4);
                if c.g[S0][0] == c.g[T4][0] {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_0058);
                }
            }
        }
        // out = out * world.
        c.cycles += 47;
        c.g[V0][0] = c.g[T5][0];
        vu0_mul_matrix(c, r32(c, T1), out, s.wrapping_add(0x50));
        vu0_copy_matrix(c, s.wrapping_add(0x50), s.wrapping_add(0x10));
        vu0_copy_matrix(c, s.wrapping_add(0x10), out);
        frame.restore(c, S0, 0x150);
        node_matrix_epilogue(c, frame)
    })
}

pub(crate) fn node_matrix_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    frame.restore(c, S1, 0x140);
    frame.restore(c, S2, 0x130);
    frame.restore(c, S3, 0x120);
    frame.restore_ra(c, 0x110);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// WorldObj_dtor (0x34fbf0, instancenode.cpp)

/// WorldObj fields: vtable (+0xc) and model instance (+0x18; its flags at
/// +8). `D_00491C80` is the WorldObj vtable.
pub(crate) const WOD_VTABLE: u32 = 0xc;

pub(crate) const WOD_INSTANCE: u32 = 0x18;

pub(crate) const WOD_INST_FLAGS: u32 = 0x8;

pub(crate) const WOD_VTABLE_ADDR: u64 = 0x0049_1c80;

pub(crate) const WOD_UNLINK: Callee = Callee {
    addr: 0x0034_fc80,
    f: ee::s11::WorldObj_UnlinkInstance,
};

pub(crate) const WOD_BASE_DTOR: Callee = Callee {
    addr: 0x0035_4920,
    f: ee::s11::SyncProcess_dtor,
};

/// `WorldObj::~WorldObj(WorldObj* self, int flags)` (0x34fbf0): unlinks the
/// model instance from the object, resets its flags (keeps the high half
/// and bits 8-9, then ORs the high half into the low half and sets bit 1),
/// and chains to `SyncProcess_dtor`. Frame 0x30 (ra at 0, s1 / s0 at
/// 0x10 / 0x20).
pub fn world_obj_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = WOD_VTABLE_ADDR;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let this = r32(c, S0);
        c.sw(this.wrapping_add(WOD_VTABLE), c.g[V0][0]);
        c.g[A1][0] = c.lw(this.wrapping_add(WOD_INSTANCE));
        WOD_UNLINK.call(c, 0x0034_fc1c)?;
        let this = r32(c, S0);
        c.g[A3][0] = c.lw(this.wrapping_add(WOD_INSTANCE));
        c.g[V1][0] = 0xffff_ffff_ffff_0300;
        c.g[A0][0] = c.g[S0][0];
        let inst = r32(c, A3).wrapping_add(WOD_INST_FLAGS);
        c.g[V0][0] = c.lw(inst);
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] &= c.g[V1][0];
        c.sw(inst, c.g[V0][0]);
        c.g[A2][0] = c.lw(this.wrapping_add(WOD_INSTANCE));
        let inst = r32(c, A2).wrapping_add(WOD_INST_FLAGS);
        c.g[V0][0] = c.lw(inst);
        c.g[V1][0] = sx((r32(c, V0) as i32 >> 16) as u32);
        c.g[V0][0] |= c.g[V1][0];
        c.sw(inst, c.g[V0][0]);
        c.g[V1][0] = c.lw(this.wrapping_add(WOD_INSTANCE));
        let inst = r32(c, V1).wrapping_add(WOD_INST_FLAGS);
        c.g[V0][0] = c.lw(inst) | 2;
        c.sw(inst, c.g[V0][0]);
        WOD_BASE_DTOR.call(c, 0x0034_fc64)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldObj::Kill()` (0x34fd90): in the instance node (+0x18) flags
/// word (+8) the low half is replaced by the high half and bit 2 (dead) is
/// set; then `this->SetState(3)` through vtable (+0xc) slot +8.
pub fn world_obj_kill(c: &mut Cpu) {
    const NODE: u32 = 0x18;
    const FLAGS: u32 = 0x8;
    const VTABLE: u32 = 0xc;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        let me = c.g[A0][0];
        c.g[A2][0] = me;
        frame.save_ra(c, 0);
        let mask = 0xffff_ffff_ffff_0000u64;
        let node = c.lw((me as u32).wrapping_add(NODE));
        let flags = c.lw((node as u32).wrapping_add(FLAGS)) & mask;
        c.sw((node as u32).wrapping_add(FLAGS), flags);
        let node = c.lw((me as u32).wrapping_add(NODE));
        c.g[A0][0] = node;
        let flags = c.lw((node as u32).wrapping_add(FLAGS));
        let high = sx(((flags as u32) as i32 >> 16) as u32);
        c.sw((node as u32).wrapping_add(FLAGS), flags | high);
        let node = c.lw((me as u32).wrapping_add(NODE));
        c.g[V1][0] = node;
        let flags = c.lw((node as u32).wrapping_add(FLAGS)) | 0x2;
        c.g[V0][0] = flags;
        c.sw((node as u32).wrapping_add(FLAGS), flags);
        if me != 0 {
            c.cycles += 6;
            let vt = c.lw((me as u32).wrapping_add(VTABLE));
            c.g[V0][0] = vt;
            c.g[A1][0] = 0x3;
            let delta = c.lh((vt as u32).wrapping_add(0x8));
            let method = c.lw((vt as u32).wrapping_add(0xc));
            c.g[V1][0] = method;
            c.g[A0][0] = sx((me as u32).wrapping_add(delta as u32));
            call_ptr(c, method as u32, 0x0034_fdf0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::DATA;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::object::debouncenode::tests::vtable_2;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn world_obj_save_matches() {
        covered(
            ssx3_ee::s11::f_0034fe90__generated,
            world_obj_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34fe90, 3);
                let this = DATA + 0x100;
                let stream = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = stream as u64;
                let vt = vtable(c, &mut r, &[8, 0x30]);
                c.bus.write32(stream, vt);
            },
        );
    }

    #[test]
    fn instance_has_anim_nodes_matches() {
        check(
            ee::s11::f_00350698__generated,
            instance_has_anim_nodes,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x350698, 0);
                let self_at = DATA + 0x200;
                let list = DATA + 0x300;
                let entries = DATA + 0x400;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x80, list);
                c.bus.write32(list + 4, (seed % 4) as u32);
                c.bus.write32(list + 8, entries);
                for k in 0..3u32 {
                    let hit = seed % 2 == 0 && k == seed % 3;
                    c.bus.write32(entries + 0x10 * k + 8, hit as u32);
                }
            },
        );
    }

    #[test]
    fn world_obj_ctor_2_matches() {
        check(
            ee::s11::f_0034fb90__generated,
            world_obj_ctor_2,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x34fb_90);
                let s = DATA + 0x100;
                clear(c, s, 0x100);
                let stream = DATA + 0x300;
                clear(c, stream, 0x20);
                let vt = DATA + 0x400;
                vtable_2(
                    c,
                    vt,
                    &[
                        (0x10, FAKE + 0x10),
                        (0x14, FAKE + 0x20),
                        (0x20, FAKE + 0x30),
                        (0x24, FAKE + 0x40),
                    ],
                );
                w(c, stream, vt as u32);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(stream);
            },
        );
    }

    #[test]
    fn world_obj_replace_with_dead_node_matches() {
        check(
            ssx3_ee::s11::f_0034fe28__generated,
            world_obj_replace_with_dead_node,
            80,
            &|c, seed| {
                let mut r = setup(c, seed, 0x34fe28);
                c.g[A0][0] = (DATA + 0x800) as u64;
                c.bus.write32(DATA + 0x800 + 0x18, r.next_u32());
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x800 + 0xc, 0);
                } else {
                    let vt = vtable(c, &mut r, &[8]);
                    c.bus.write32(DATA + 0x800 + 0xc, vt);
                }
            },
        );
    }

    /// Instance_GetNodeMatrix: no owner, or an owner (fake v-table) that
    /// has the node's matrix or not; node -1, the root node or a node with
    /// one or two parents; random matrices and scale.
    #[test]
    fn instance_get_node_matrix_matches() {
        covered(
            ee::s11::f_0034fed8__generated,
            instance_get_node_matrix,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34fed8, 2);
                c.pc = 0x0034_fed8;
                let inst = DATA;
                let skel = DATA + 0x400;
                let nodes = DATA + 0x500;
                let mats = DATA + 0x1000;
                let out = DATA + 0x3000;
                let owner = DATA + 0x600;
                for k in 0..0x10 {
                    c.bus.write32(inst + 0x10 + 4 * k, r.f32(4.0));
                }
                c.bus.write32(inst + 0x84, r.f32(2.0));
                c.bus.write32(inst + 0x80, skel);
                c.bus.write32(skel + 8, nodes);
                // Nodes 0..3: 0 is the root, 1 -> 0, 2 -> 1, 3 -> 0.
                for (n, parent) in [(0u32, u32::MAX), (1, 0), (2, 1), (3, 0)] {
                    c.bus.write32(nodes + 0x10 * n, parent);
                    c.bus.write32(nodes + 0x10 * n + 0xc, mats + 0x40 * n);
                }
                for k in 0..0x40 {
                    c.bus.write32(mats + 4 * k, r.f32(4.0));
                }
                let with_owner = seed % 3 == 0;
                if with_owner {
                    let vt = fakes::vtable(c, &mut r, &[0xc0, 0xd0, 0xe8]);
                    c.bus.write32(inst + 0xc, owner);
                    c.bus.write32(owner + 0xc, vt);
                    // HasNodeMatrix: a fake answering non-zero, or the real
                    // `Rider_ReturnZero_25b8` (0x1125b8). The matrix calls answer
                    // matrix pointers.
                    if r.next_u32() % 2 == 0 {
                        c.bus.write32(vt + 0xd4, 0x0011_25b8);
                        c.bus.write32(fakes::LOG_RESULTS, mats + 0x40);
                    } else {
                        c.bus.write32(fakes::LOG_RESULTS, mats + 0x80);
                        c.bus.write32(fakes::LOG_RESULTS + 4, mats + 0x40);
                    }
                }
                let node = match r.next_u32() % 5 {
                    0 => u32::MAX,
                    k => k - 1,
                };
                c.g[A0][0] = sx(inst);
                c.g[A1][0] = sx(node);
                c.g[A2][0] = sx(out);
            },
        );
    }

    /// WorldObj_dtor: an object at DATA whose instance (DATA+0x100, random
    /// flags) points back at it or not; the real base destructors run
    /// (process type counter, process-list unlink with/without neighbours;
    /// flags without the delete bit, so nothing is freed).
    #[test]
    fn world_obj_dtor_matches() {
        use crate::difftest::{check, fill, scramble, Rng, GAME_GP};
        check(
            ee::s11::f_0034fbf0__generated,
            world_obj_dtor,
            300,
            &|c, seed| {
                let mut r = Rng(0x34_fbf0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0034_fbf0;
                c.g[GP][0] = GAME_GP as u64;
                let obj = DATA;
                let inst = DATA + 0x100;
                fill(c, &mut r, obj, 8);
                c.bus.write16(obj + 0x10, (r.next_u32() % 8) as u16);
                if seed % 3 == 0 {
                    c.bus.write32(obj + 4, 0);
                } else {
                    c.bus.write32(obj + 4, DATA + 0x200);
                    c.bus
                        .write32(obj + 8, if seed % 3 == 1 { 0 } else { DATA + 0x300 });
                }
                c.bus
                    .write32(obj + WOD_INSTANCE, if seed % 7 == 0 { 0 } else { inst });
                c.bus.write32(inst + WOD_INST_FLAGS, r.next_u32());
                c.bus
                    .write32(inst + 0xc, if seed % 2 == 0 { obj } else { r.next_u32() });
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (r.next_u32() & !1) as u64;
            },
        );
    }

    /// `WorldObj_Kill`: an object with a node whose flags word is random
    /// (high half sign bit set or not) and a vtable whose slot +8 is a fake;
    /// on 1 in 8 seeds the object pointer is null (the flag writes go
    /// through low RAM and no call is made).
    #[test]
    fn world_obj_kill_matches() {
        const ME: u32 = DATA;
        const NODE: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        covered(
            ee::s11::f_0034fd90__generated,
            world_obj_kill,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34fd90, 1);
                c.pc = 0x0034_fd90;
                c.bus.write32(ME + 0x18, NODE);
                c.bus.write32(NODE + 8, r.next_u32());
                c.bus.write32(ME + 0xc, VT);
                c.bus.write16(VT + 8, (r.next_u32() % 0x20) as u16 & !3);
                c.bus.write32(VT + 0xc, FAKE);
                c.bus
                    .write32(0x18, if seed % 2 == 0 { 0 } else { NODE + 0x40 });
                c.g[A0][0] = if seed % 8 == 0 { 0 } else { sx(ME) };
            },
        );
    }
}
