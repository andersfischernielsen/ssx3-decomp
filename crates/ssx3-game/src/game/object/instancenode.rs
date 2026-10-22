//! `src/object/instancenode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::sp;
use crate::call::VSlot;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::render::lightman::addu;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p as p_2;
pub(crate) use crate::game::be::beintplayer::lh;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0034_fb90, world_obj_ctor_2, "WorldObj_ctor_2"),
    (0x0034_fe28, world_obj_replace_with_dead_node, "WorldObj_ReplaceWithDeadNode"),
    (0x0034_fe90, world_obj_save, "WorldObj_Save"),
    (0x0035_0698, instance_has_anim_nodes, "Instance_HasAnimNodes"),
];

// ---------------------------------------------------------------------------
// Callees and field offsets.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::render::lightman::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_FLAG_MANAGER;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_AI_FRAME_COUNT;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_NASH;
    pub(crate) use crate::game::render::lightman::callee::LIGHT_CONTRIBUTION;
    pub(crate) use crate::game::render::lightman::callee::MATH_SIN_COS;
    pub(crate) use crate::game::render::lightman::callee::SPATIAL_HASH_UPDATE;
    pub(crate) use crate::game::render::lightman::callee::FLAG_MANAGER_GROUP_COMPUTE_VERTS;
    pub(crate) use crate::game::render::lightman::callee::INTERSECT_INFO_EVALUATE;
    pub(crate) use crate::game::render::lightman::callee::TYPED_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::MULTI_SPLINE_MODIFIER_DEBUG_DRAW_PATH;
    pub(crate) use crate::game::render::lightman::callee::EA_MEMSET;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_FROM_SCRATCHPAD;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_TO_SCRATCHPAD;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::put_words;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use ssx3_ee as ee;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::object::debouncenode::tests::vtable_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;


    #[test]
    fn world_obj_save_matches() {
        covered(ssx3_ee::s11::f_0034fe90__generated, world_obj_save, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x34fe90, 3);
            let this = DATA + 0x100;
            let stream = DATA + 0x400;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            let vt = vtable(c, &mut r, &[8, 0x30]);
            c.bus.write32(stream, vt);
        });
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
        check(ee::s11::f_0034fb90__generated, world_obj_ctor_2, 40, &|c, seed| {
            let mut r = base(c, seed, 0x34fb_90);
            let s = DATA + 0x100;
            clear(c, s, 0x100);
            let stream = DATA + 0x300;
            clear(c, stream, 0x20);
            let vt = DATA + 0x400;
            vtable_2(
                c,
                vt,
                &[(0x10, FAKE + 0x10), (0x14, FAKE + 0x20), (0x20, FAKE + 0x30), (0x24, FAKE + 0x40)],
            );
            w(c, stream, vt as u32);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = sx(stream);
        });
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
}
