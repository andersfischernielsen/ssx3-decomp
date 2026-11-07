//! `src/object/deadfadenode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::render::particle::p as p_2;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_0b98, dead_fade_ctor, "DeadFade_ctor"),
    (0x0035_0be0, dead_fade_dtor, "DeadFade_dtor"),
    (0x0035_0c38, dead_fade_update, "DeadFade_Update"),
    (0x0035_0c90, dead_fade_kill, "DeadFade_Kill"),
    (0x0035_0e90, hide_ctor, "Hide_ctor"),
    (0x0035_0f08, hide_ctor_2, "Hide_ctor_2"),
    (0x0035_0fd8, restore_node_ctor_2, "RestoreNode_ctor_2"),
    (0x0035_0f40, hide_save, "Hide_Save"),
];

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {

    pub(crate) use crate::game::be::bedbreward::table_callee::WORLD_CREATE_DEAD_NODE;
}

// ---------------------------------------------------------------------------
// Intra-shard callees the generator called directly (its `jal` aliases).

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR_3;

    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;

    pub(crate) use crate::game::world::wscriptcache::callee::CINSTANCE_NODE_CTOR_V;

    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_OBJ_CTOR;
}

/// Cross-shard call targets (the generator emits `call_indirect` for them).
pub(crate) mod table {

    pub(crate) use crate::game::ai::computer::table::EA_MEM_FREE;
}

/// `void DeadFade_Kill(DeadFade *this)` (0x350c90): spawn a dead node in the
/// world (`this->world` at `+0x18`) at `this + 0x1c`.
pub fn dead_fade_kill(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c));
        call_ptr(c, table_callee::WORLD_CREATE_DEAD_NODE, 0x0035_0ca8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Derived-class constructors: base constructor, then the class vtable.

/// `self = BaseCtor(self); self->vptr = vtable; return self`: the shape of the
/// five node constructors (0x34ae68, 0x350f08, 0x350fd8, 0x356e60, 0x357918).
pub(crate) fn ctor_with_vtable(c: &mut Cpu, ret: u32, base: Callee, vtable: u32) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    let obj = c.g[A0][0];
    c.g[S0][0] = obj;
    base.call(c, ret)?;
    c.g[V1][0] = sx(vtable);
    c.g[V0][0] = c.g[S0][0];
    frame.restore_ra(c, 0);
    Addr(obj as u32).off(0xc).sw(c, c.g[V1][0]);
    frame.restore(c, S0, 0x10);
    frame.close(c);
    Ok(())
}

/// `void* Hide_ctor_2(Hide* self)` (0x350f08).
pub fn hide_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        ctor_with_vtable(c, 0x0035_0f1c, callee::WORLD_OBJ_CTOR, 0x0049_1800)
    })
}

/// `void* RestoreNode_ctor_2(RestoreNode* self)` (0x350fd8).
pub fn restore_node_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        ctor_with_vtable(c, 0x0035_0fec, callee::WORLD_OBJ_CTOR, 0x0049_1680)
    })
}

/// `void* DeadFade_ctor(DeadFade* self)` (0x350b98): a dead-fade node: base
/// instance node, vtable 0x491980, `+0x1c = -1`, `+0x38 = 0`. Returns `self`.
pub fn dead_fade_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CINSTANCE_NODE_CTOR_V.call(c, 0x0035_0bac)?;
        c.g[V1][0] = 0x490000;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_add(0x1980));
        c.g[A0][0] |= 0xffff;
        c.sw(r32(c, S0).wrapping_add(0xc), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(r32(c, S0).wrapping_add(0x1c), c.g[A0][0]);
        c.sw(r32(c, S0).wrapping_add(0x38), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DeadFade_dtor(DeadFade* f, int flags)` (0x350be0): set the vtable,
/// free the fade node at `+0x38` and run the WorldObj destructor.
pub fn dead_fade_dtor(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0049_1980);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.sw(p(c, S0, 0xc), c.g[V0][0]);
    c.g[A0][0] = Addr(p(c, S0, 0x38)).lw(c);
    let null = c.g[A0][0] == 0;
    c.g[S1][0] = c.g[A1][0];
    if !null {
        c.cycles += 2;
        if call_ptr(c, table::EA_MEM_FREE, 0x0035_0c14).is_err() {
            return;
        }
    }
    c.cycles += 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    if callee::WORLD_OBJ_DTOR.call(c, 0x0035_0c20).is_err() {
        return;
    }
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void DeadFade_Update(DeadFade* self)` (0x350c38): fade the alpha down by
/// its step until it reaches zero, then spawn the dead node.
pub fn dead_fade_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.mtc1(0, 0);
        let a1 = r32(c, A1);
        c.lwc1(1, p_2(a1, 0x30));
        c.lwc1(2, p_2(a1, 0x34));
        c.lwc1(3, p_2(a1, 0x28));
        c.add_s(1, 1, 2);
        c.c_lt_s(0, 3);
        let expired = !c.fcc();
        c.swc1(1, p_2(a1, 0x30));
        if expired {
            c.cycles += 3;
            c.g[A0][0] = Addr(r32(c, A1)).off(0x18).lw(c);
            c.g[RA][0] = sx(0x0035_0c84);
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1c));
            call_ptr(c, callee::WORLD_CREATE_DEAD_NODE, 0x0035_0c84)?;
        } else {
            c.cycles += 4;
            c.lwc1(0, p_2(a1, 0x2c));
            c.sub_s(0, 3, 0);
            c.swc1(0, p_2(a1, 0x28));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_00350E90` (0x350e90): the hide-object constructor. Same shape as the
/// dead node's, with `WorldObj_ctor` kind 1 / flags 0x10 and vtable `0x491800`.
/// Signature: `void ctor(this, a, b)`.
pub fn hide_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x1;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0035_0eb0);
        c.g[A2][0] = 0x10;
        callee::WORLD_OBJ_CTOR_3.call(c, 0x0035_0eb0)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A2][0] = lw_r(c, S0, 0x18);
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x1800));
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        sw_r(c, S0, 0xc, c.g[V1][0]);
        c.g[A0][0] = c.g[A0][0] | 0xff9f;
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = lw_r(c, A2, 0x8);
        c.g[A1][0] = c.g[A1][0] | 0xfffd;
        c.g[RA][0] = c.ld(low(c, SP));
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        sw_r(c, A2, 0x8, c.g[V1][0]);
        c.g[A0][0] = lw_r(c, S0, 0x18);
        frame.restore(c, S0, 0x10);
        c.g[V1][0] = lw_r(c, A0, 0x8);
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V1][0] = c.g[V1][0] | 0x4;
        sw_r(c, A0, 0x8, c.g[V1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Hide_Save (0x350f40).

/// `WorldObj_Save(self, stream)`.
pub(crate) const HIDE_WORLD_OBJ_SAVE: Callee = Callee {
    addr: 0x0034_fe90,
    f: ee::s11::WorldObj_Save,
};

/// `void Hide_Save(Hide* self, Stream* stream)` (0x350f40): a hide node
/// saves only its world-object part: `WorldObj_Save(self, stream)`.
/// Frame 0x10 (ra +0).
pub fn hide_save(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(HIDE_WORLD_OBJ_SAVE),
        0x0035_0f50,
    );
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;

    use crate::difftest::fill;

    use crate::difftest::DATA;

    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::bx::bxstring2::tests::game;

    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::common;

    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;

    pub(crate) use crate::game::worldpainter::quadtree::tests::put_vtable;

    #[test]
    fn dead_fade_kill_matches() {
        check(
            ee::s11::f_00350c90__generated,
            dead_fade_kill,
            60,
            &|c, seed| {
                let mut r = game(c, seed, 0x350c90, 1);
                c.g[A0][0] = SCRATCH as u64;
                let world = SCRATCH + 0x2000;
                c.bus.write32(SCRATCH + 0x18, world);
                for i in 0..3u32 {
                    c.bus.write32(SCRATCH + 0x1c + 4 * i, r.f32(20.0));
                }
                c.bus.write32(world + 0xc, 0);
            },
        );
    }

    // ---------------------------------------------- node constructors

    pub(crate) fn ctor_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32, src: u32) {
        let mut r = machine(c, seed, salt, 0);
        let vt = put_vtable(c, &mut r, DATA + 0x800, &[slot]);
        c.bus.write32(src, vt);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = sx(r.next_u32());
        c.g[A2][0] = sx(src);
        fill(c, &mut r, DATA + 0x1000, 0x100);
        // ModObj's ctor reads a word the fake read leaves alone; keep it zero
        // so the modifier-block path is skipped.
        c.bus.write32(STACK_TOP - 0x60, 0);
    }

    #[test]
    fn hide_ctor_2_matches() {
        check(
            ee::s11::f_00350f08__generated,
            hide_ctor_2,
            80,
            &|c, seed| {
                ctor_setup(c, seed, 0x350f_08, 0x20, DATA + 0x300);
            },
        );
    }

    #[test]
    fn restore_node_ctor_2_matches() {
        check(
            ee::s11::f_00350fd8__generated,
            restore_node_ctor_2,
            80,
            &|c, seed| {
                ctor_setup(c, seed, 0x350f_d8, 0x20, DATA + 0x300);
            },
        );
    }

    #[test]
    fn dead_fade_ctor_matches() {
        covered(
            ee::s11::f_00350b98__generated,
            dead_fade_ctor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_0b98, 1);
                let obj = DATA + 0x1000;
                let owner = DATA + 0x3000;
                let vt = vtable(c, &mut r, &[0x10, 0x20]);
                c.bus.write32(owner, vt);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = 0;
                c.g[A2][0] = sx(owner);
            },
        );
    }

    #[test]
    fn dead_fade_dtor_matches() {
        check(
            ssx3_ee::s11::f_00350be0__generated,
            dead_fade_dtor,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x350be0, 0);
                heap_init(c);
                let this = alloc(c, 0x100);
                let node = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
                c.bus.write32(this + 0x38, node);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn dead_fade_update_matches() {
        check(
            ee::s11::f_00350c38__generated,
            dead_fade_update,
            SEEDS,
            &|c, seed| {
                let mut r = common(c, seed, 0x350c38, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                for off in [0x28u32, 0x2c, 0x30, 0x34] {
                    let v = if (seed as u32 + off) % 3 == 0 {
                        0
                    } else {
                        r.edge_f32()
                    };
                    c.bus.write32(this + off, v);
                }
                c.bus.write32(this + 0x18, DATA + 0x200);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn hide_ctor_matches() {
        check(
            ee::s11::f_00350e90__generated,
            hide_ctor,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x350e90, 4);
                be_library(c);
                world(c);
                heap_init_2(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                c.g[A2][0] = (DATA + 0x500) as u64;
                w(c, this + 0x18, DATA + 0x600);
            },
        );
    }

    /// `Hide_Save` through the real `WorldObj_Save` (and
    /// `SyncProcess_Save`), the stream's writers fakes (some unwind).
    #[test]
    fn hide_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        covered(
            ee::s11::f_00350f40__generated,
            hide_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_0f40, 3);
                c.pc = 0x0035_0f40;
                let this = DATA + 0x100;
                let stream = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = stream as u64;
                let vt = vtable(c, &mut r, &[8, 0x30]);
                c.bus.write32(stream, vt);
            },
        );
    }
}
