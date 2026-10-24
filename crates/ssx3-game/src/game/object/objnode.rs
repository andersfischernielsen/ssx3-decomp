//! `src/object/objnode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::be::vt_call;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p;
pub(crate) use crate::game::render::lightman::addu;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_4788, typed_process_ctor_2, "TypedProcess_ctor_2"),
    (0x0035_48c0, sync_process_ctor_2, "SyncProcess_ctor_2"),
    (0x0035_4948, typed_process_save, "TypedProcess_Save"),
    (0x0035_4980, sync_process_save, "SyncProcess_Save"),
];

// ---------------------------------------------------------------------------
// Callees and field offsets.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_FROM_SCRATCHPAD;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_TO_SCRATCHPAD;
    pub(crate) use crate::game::render::lightman::callee::EA_MEMSET;
    pub(crate) use crate::game::render::lightman::callee::FLAG_MANAGER_GROUP_COMPUTE_VERTS;
    pub(crate) use crate::game::render::lightman::callee::INTERSECT_INFO_EVALUATE;
    pub(crate) use crate::game::render::lightman::callee::LIGHT_CONTRIBUTION;
    pub(crate) use crate::game::render::lightman::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::render::lightman::callee::MATH_SIN_COS;
    pub(crate) use crate::game::render::lightman::callee::MULTI_SPLINE_MODIFIER_DEBUG_DRAW_PATH;
    pub(crate) use crate::game::render::lightman::callee::SPATIAL_HASH_UPDATE;
    pub(crate) use crate::game::render::lightman::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::TYPED_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_AI_FRAME_COUNT;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_FLAG_MANAGER;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_NASH;
}

/// `void TypedProcess_Save(TypedProcess *self, void *obj)` (0x00354948):
/// `obj->vt[1](obj + delta, self + 0x10, 4)`.
pub fn typed_process_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        let this = abi::arg(c, 0);
        let obj = abi::arg(c, 1);
        c.g[A3][0] = sx(obj);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(off(this, 0x10));
        c.g[A2][0] = 4;
        let vt = c.ldw(obj);
        c.g[V1][0] = vt;
        c.g[A0][0] = c.ldh(off(vt as u32, 8));
        c.g[V0][0] = c.ldw(off(vt as u32, 0xc));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = addu(c.g[A3][0], c.g[A0][0]);
        call_ptr(c, target, 0x0035_4970)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SyncProcess_Save(SyncProcess *self, void *obj)` (0x00354980):
/// `TypedProcess_Save(self, obj)`, then `obj->vt[1](obj + delta,
/// self + 0x14, 4)`.
pub fn sync_process_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        let obj = abi::arg(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(obj);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::TYPED_PROCESS_SAVE.call(c, 0x0035_499c)?;
        c.g[V1][0] = c.ldw(c.g[S0][0] as u32);
        c.g[A1][0] = sx(off(c.g[S1][0] as u32, 0x14));
        c.g[A2][0] = 4;
        c.g[A0][0] = c.ldh(off(c.g[V1][0] as u32, 8));
        c.g[V0][0] = c.ldw(off(c.g[V1][0] as u32, 0xc));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, target, 0x0035_49b8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SyncProcess* SyncProcess_ctor_2(SyncProcess* self, int a, Stream* stream)`
/// (0x3548c0): typed-process base, its vtable and the stream registration.
pub fn sync_process_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0035_4788, 0x0035_48dc)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1e00));
        c.g[A2][0] = 4;
        Addr(p(c.g[S0][0], 0xc)).sw(c, c.g[V1][0]);
        c.g[V0][0] = lw(c, c.g[S1][0] as u32);
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x10)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_4904)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void TypedProcess_ctor_2(TypedProcess* p, int, TypeInfo* ti)`
/// (0x00354788): build the base process, install the type descriptor from
/// `[ti]`, then bump the constructor count in the type table.
pub fn typed_process_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0035_4688, 0x0035_47a4)?; // GameProcess_ctor_2
        c.g[V1][0] = sx(0x0049_1e80);
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 4;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
        let vt = c.g[V0][0];
        let obj = c.g[S1][0];
        vt_call(c, vt, 0x10, obj, V1, 0x0035_47cc)?;
        c.g[A1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx(0x0044_aff0);
        c.g[V0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[RA][0] = c.ld(sp(c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.sw(c.g[A1][0] as u32, c.g[V1][0]);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::object::debouncenode::tests::vtable_2;
    use ssx3_ee as ee;

    #[test]
    fn typed_process_save_matches() {
        covered(
            ssx3_ee::s11::f_00354948__generated,
            typed_process_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x354948, 1);
                let this = DATA + 0x100;
                let obj = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = obj as u64;
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(obj, vt);
                c.bus.write32(this + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn sync_process_save_matches() {
        covered(
            ssx3_ee::s11::f_00354980__generated,
            sync_process_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x354980, 2);
                let this = DATA + 0x100;
                let obj = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = obj as u64;
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(obj, vt);
            },
        );
    }

    #[test]
    fn sync_process_ctor_2_matches() {
        check(
            ee::s11::f_003548c0__generated,
            sync_process_ctor_2,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x3548_c0);
                let s = DATA + 0x100;
                clear(c, s, 0x100);
                let stream = DATA + 0x300;
                clear(c, stream, 0x20);
                let vt = DATA + 0x400;
                vtable_2(c, vt, &[(0x10, FAKE + 0x10), (0x14, FAKE + 0x20)]);
                w(c, stream, vt as u32);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(stream);
            },
        );
    }

    #[test]
    fn typed_process_ctor_2_matches() {
        check(
            ee::s11::f_00354788__generated,
            typed_process_ctor_2,
            80,
            &|c, seed| {
                let _ = base_2(c, seed, 0x3547, 2);
                let proc = DATA;
                let ti = DATA + 0x600;
                w_2(c, ti, DATA + 0x700);
                slot(c, DATA + 0x700, 0x10, FAKE);
                wh(c, proc + 0x10, (seed % 8) as u16);
                c.g[A0][0] = sx(proc);
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = sx(ti);
            },
        );
    }
}
