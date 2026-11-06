//! `src/object/objnode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
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
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_4788, typed_process_ctor_2, "TypedProcess_ctor_2"),
    (0x0035_48c0, sync_process_ctor_2, "SyncProcess_ctor_2"),
    (0x0035_4948, typed_process_save, "TypedProcess_Save"),
    (0x0035_4980, sync_process_save, "SyncProcess_Save"),
    (0x0035_46c8, game_process_dtor, "GameProcess_dtor"),
    (0x0035_4720, typed_process_ctor, "TypedProcess_ctor"),
    (0x0035_4808, typed_process_dtor, "TypedProcess_dtor"),
    (0x0035_4850, sync_process_ctor, "SyncProcess_ctor"),
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

/// `TypedProcess_ctor` (0x354720).
pub(crate) const TYPED_PROCESS_CTOR: Callee = Callee {
    addr: 0x0035_4720,
    f: ee::s11::TypedProcess_ctor,
};

/// The `SyncProcess` vtable (0x491e00).
pub(crate) const SYNC_PROCESS_VTABLE: u32 = 0x0049_1e00;

/// Process field: vtable pointer.
pub(crate) const PROCESS_VTABLE: u32 = 0xc;

/// `SyncProcess` field: its serial id.
pub(crate) const SYNC_PROCESS_ID: u32 = 0x14;

/// The type of sync processes that take their ids from the second counter.
pub(crate) const SYNC_TYPE_SECONDARY: u64 = 6;

/// Next-id counters (gp-relative): the common one and the one for type 6.
pub(crate) const SYNC_NEXT_ID: u32 = 0x2a88;

pub(crate) const SYNC_NEXT_ID_TYPE6: u32 = 0x2a8c;

/// `SyncProcess* SyncProcess_ctor(SyncProcess* self, int priority, int type)`
/// (0x354850): constructs the typed process (registers it in the process
/// list), installs the `SyncProcess` vtable and gives it the next serial id,
/// type 6 from its own counter, all others from the common one. Returns
/// `self`.
pub fn sync_process_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        TYPED_PROCESS_CTOR.call(c, 0x0035_486c)?;
        let this = r32(c, S1);
        c.g[V1][0] = sx(SYNC_PROCESS_VTABLE);
        c.g[V0][0] = SYNC_TYPE_SECONDARY;
        let counter = if c.g[S0][0] == SYNC_TYPE_SECONDARY {
            c.cycles += 5;
            SYNC_NEXT_ID_TYPE6
        } else {
            c.cycles += 4;
            SYNC_NEXT_ID
        };
        Addr(this).off(PROCESS_VTABLE).sw(c, c.g[V1][0]);
        let at = Addr(r32(c, GP).wrapping_add(counter));
        let id = at.lw(c);
        c.g[V1][0] = id;
        c.g[V0][0] = sx((id as u32).wrapping_add(1));
        at.sw(c, c.g[V0][0]);
        c.cycles += 7;
        Addr(this).off(SYNC_PROCESS_ID).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ProcessList_Remove` (0x354e48).
pub(crate) const PROCESS_LIST_REMOVE: Callee = Callee {
    addr: 0x0035_4e48,
    f: ee::s11::ProcessList_Remove,
};

/// `operator delete` (`__builtin_delete`, 0x317e50; called through the table).
pub(crate) const OC286_BUILTIN_DELETE: u32 = 0x0031_7e50;

/// The `GameProcess` vtable (0x491f00).
pub(crate) const GAME_PROCESS_VTABLE: u32 = 0x0049_1f00;

/// The global process list (gp-relative).
pub(crate) const PROCESS_LIST: u32 = 0x2898;

/// `void GameProcess_dtor(GameProcess* self, int flags)` (0x3546c8):
/// reinstalls the `GameProcess` vtable, unlinks the process from the global
/// process list and, with bit 0 of `flags` (deleting destructor), frees it.
pub fn game_process_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(GAME_PROCESS_VTABLE);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(r32(c, S1)).off(PROCESS_VTABLE).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(r32(c, GP).wrapping_add(PROCESS_LIST));
        c.g[A1][0] = c.g[S1][0];
        PROCESS_LIST_REMOVE.call(c, 0x0035_46f8)?;
        c.g[S0][0] &= 1;
        let delete = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, OC286_BUILTIN_DELETE, 0x0035_470c)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameProcess_dtor` (0x3546c8).
pub(crate) const GAME_PROCESS_DTOR: Callee = Callee {
    addr: 0x0035_46c8,
    f: ee::s11::GameProcess_dtor,
};

/// The `TypedProcess` vtable (0x491e80).
pub(crate) const TYPED_PROCESS_VTABLE: u32 = 0x0049_1e80;

/// `TypedProcess` field: its type (i16).
pub(crate) const TYPED_PROCESS_TYPE: u32 = 0x10;

/// Live processes per type (i32 each, 0x44aff0 = 0x450000 - 0x5010).
pub(crate) const PROCESS_TYPE_COUNTS_HI: u32 = 0x0045_0000;

pub(crate) const PROCESS_TYPE_COUNTS: u32 = PROCESS_TYPE_COUNTS_HI - 0x5010;

/// `void TypedProcess_dtor(TypedProcess* self, int flags)` (0x354808):
/// reinstalls the `TypedProcess` vtable, decrements the live count of the
/// process's type, then runs the `GameProcess` destructor (with `flags`).
pub fn typed_process_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(TYPED_PROCESS_VTABLE);
        frame.save_ra(c, 0);
        let this = Addr(r32(c, A0));
        this.off(PROCESS_VTABLE).sw(c, c.g[V0][0]);
        c.g[A2][0] = this.off(TYPED_PROCESS_TYPE).lh(c);
        c.g[V1][0] = sx(PROCESS_TYPE_COUNTS);
        c.g[A2][0] = sx((r32(c, A2) << 2).wrapping_add(PROCESS_TYPE_COUNTS));
        let count = Addr(r32(c, A2));
        c.g[V0][0] = sx((count.lw(c) as u32).wrapping_sub(1));
        count.sw(c, c.g[V0][0]);
        GAME_PROCESS_DTOR.call(c, 0x0035_4840)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `TypedProcess` fields: vtable at +0xc, type at +0x10 (s16), +0x12 (s16).
pub(crate) const TYPED_PROCESS_VTABLE_2: u32 = 0xc;

pub(crate) const TYPED_PROCESS_FLAGS: u32 = 0x12;

/// The `TypedProcess` vtable.
pub(crate) const TYPED_PROCESS_VT: u32 = 0x0049_1e80;

/// Live process count per type (`int[]`).
pub(crate) const TYPED_PROCESS_COUNTS: u32 = 0x0044_aff0;

/// `TypedProcess* TypedProcess_ctor(TypedProcess* self, void* list_arg,
/// int type)` (0x354720): construct the base `GameProcess` (registers it in
/// its process list), then set the vtable, store the type, clear +0x12 and
/// count one more live process of that type.
pub fn typed_process_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        Callee {
            addr: 0x0035_4648,
            f: ee::s11::GameProcess_ctor,
        }
        .call(c, 0x0035_473c)?;

        let this = Addr(c.g[S0][0] as u32);
        let ty = c.g[S1][0] as u32;
        this.off(TYPED_PROCESS_TYPE).sh(c, ty as u64);
        this.off(TYPED_PROCESS_VTABLE_2)
            .sw(c, TYPED_PROCESS_VT as u64);
        this.off(TYPED_PROCESS_FLAGS).sh(c, 0);
        let slot = TYPED_PROCESS_COUNTS.wrapping_add(((ty << 16) as i32 >> 14) as u32);
        c.g[A0][0] = sx(slot);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        let n = sx((Addr(slot).lw(c) as u32).wrapping_add(1));
        c.g[V1][0] = n;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        Addr(slot).sw(c, n);
        frame.close(c);
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
    use super::*;
    use super::*;
    use crate::difftest::{scramble, RA as SENTINEL};

    /// The process list at `gp + 0x2898`: its per-priority node array (two
    /// sub-lists per priority, chosen by bit 0 of the entry's first word) with
    /// valid tail pointers, and the per-type process counts at 0x44aff0.
    pub(crate) fn oc286_process_list(c: &mut Cpu, r: &mut Rng) {
        let nodes = DATA + 0x1000;
        c.bus.write32(GAME_GP + 0x2898 + 4, nodes);
        for i in 0..16u32 {
            let e = nodes + i * 0x44;
            c.bus.write32(e, r.next_u32());
            c.bus.write32(e + 0x8, DATA + 0x3000 + i * 0x10);
            c.bus.write32(e + 0x28, DATA + 0x3800 + i * 0x10);
            c.bus.write32(0x0044_aff0 + i * 4, r.next_u32() % 50);
        }
    }

    /// Both id counters, for type 6 and others, after a real typed-process
    /// construction (process-list insertion in both sub-lists).
    #[test]
    fn sync_process_ctor_matches() {
        check(
            ee::s11::f_00354850__generated,
            sync_process_ctor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_4850, 0);
                oc286_process_list(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (r.next_u32() % 16) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    6
                } else {
                    (r.next_u32() % 16) as u64
                };
                c.bus.write32(GAME_GP + SYNC_NEXT_ID, r.next_u32());
                c.bus.write32(GAME_GP + SYNC_NEXT_ID_TYPE6, r.next_u32());
                c.pc = 0x0035_4850;
            },
        );
    }

    /// A process in a real heap block (so `operator delete` can free it),
    /// linked into a list (prev/next at +4/+8, either may be null), with its
    /// type (+0x10) and the per-type counts. Returns the process.
    pub(crate) fn oc286_heap_process(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        use crate::game::render::particle::tests::heap_block;
        use crate::game::worldpainter::quadtree::tests::{heap_init, pool_allocator};
        pool_allocator(c);
        heap_init(c);
        let this = heap_block(c, 0x80);
        let prev = if seed % 3 == 0 { 0 } else { DATA + 0x5000 };
        let next = if seed % 5 == 0 { 0 } else { DATA + 0x5100 };
        c.bus.write32(this + 4, prev);
        c.bus.write32(this + 8, next);
        c.bus.write16(this + 0x10, (r.next_u32() % 16) as u16);
        for i in 0..16u32 {
            c.bus.write32(0x0044_aff0 + i * 4, r.next_u32() % 50);
        }
        this
    }

    /// Unlink from the process list (prev/next present or not) and the
    /// delete flag clear or set (a real heap free).
    #[test]
    fn game_process_dtor_matches() {
        check(
            ee::s11::f_003546c8__generated,
            game_process_dtor,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_46c8, 0);
                let this = oc286_heap_process(c, &mut r, seed);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx((seed & 3) as u32);
                c.pc = 0x0035_46c8;
            },
        );
    }

    /// Type-count decrement, then the real `GameProcess` destructor: unlink
    /// (prev/next present or not) and the delete flag clear or set.
    #[test]
    fn typed_process_dtor_matches() {
        check(
            ee::s11::f_00354808__generated,
            typed_process_dtor,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_4808, 0);
                let this = oc286_heap_process(c, &mut r, seed);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx((seed & 3) as u32);
                c.pc = 0x0035_4808;
            },
        );
    }

    /// `TypedProcess_ctor` with the real `GameProcess_ctor`/`ProcessList_Add`
    /// on a small process list (both list kinds, by the entry's flag bit),
    /// types with junk in the upper half and negative types.
    #[test]
    fn typed_process_ctor_matches() {
        use crate::difftest::GAME_GP;
        const LIST: u32 = GAME_GP + 0x2898;
        check(
            ee::s11::f_00354720__generated,
            typed_process_ctor,
            300,
            &|c, seed| {
                let mut r = Rng(0x3547_2000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let table = DATA + 0x2000;
                c.bus.write32(LIST + 4, table);
                let kind = r.next_u32() % 4;
                for k in 0..4 {
                    let e = table + 0x44 * k;
                    c.bus.write32(e, r.next_u32());
                    c.bus.write32(e + 0x8, DATA + 0x3000 + 0x40 * k);
                    c.bus.write32(e + 0x28, DATA + 0x3400 + 0x40 * k);
                }
                let ty = match seed % 4 {
                    0 => r.next_u32() % 16,
                    1 => (r.next_u32() << 16) | (r.next_u32() % 16),
                    2 => 0xffff_0000 | (0x1_0000 - 1 - r.next_u32() % 4),
                    _ => r.next_u32() % 64,
                };
                let base = TYPED_PROCESS_COUNTS.wrapping_add(((ty << 16) as i32 >> 14) as u32);
                c.bus.write32(base, r.next_u32() % 100);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = kind as u64;
                c.g[A2][0] = ty as u64 | ((r.next_u32() as u64) << 32);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0035_4720;
            },
        );
    }
}
