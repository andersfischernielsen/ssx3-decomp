//! `src/object/deadfadenode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::object::floatingnode::oc604_ea;
pub(crate) use crate::game::object::floatingnode::oc604_quad_load;
pub(crate) use crate::game::object::floatingnode::oc604_quad_store;
pub(crate) use crate::game::render::particle::p as p_2;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
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
    (0x0035_0b98, dead_fade_ctor, "DeadFade_ctor"),
    (0x0035_0be0, dead_fade_dtor, "DeadFade_dtor"),
    (0x0035_0c38, dead_fade_update, "DeadFade_Update"),
    (0x0035_0c90, dead_fade_kill, "DeadFade_Kill"),
    (0x0035_0e90, hide_ctor, "Hide_ctor"),
    (0x0035_0f08, hide_ctor_2, "Hide_ctor_2"),
    (0x0035_0fd8, restore_node_ctor_2, "RestoreNode_ctor_2"),
    (0x0035_0f40, hide_save, "Hide_Save"),
    (0x0035_1010, restore_node_save, "RestoreNode_Save"),
    (0x0035_0f60, restore_node_ctor, "RestoreNode_ctor"),
    (0x0035_0750, dead_fade_ctor_2, "DeadFade_ctor_2"),
    (0x0035_0cb8, dead_fade_render, "DeadFade_Render"),
    (0x0035_0e70, dead_fade_save, "DeadFade_Save"),
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

// ---------------------------------------------------------------------------
// RestoreNode_Save (0x351010).

/// `WorldObj_Save(self, stream)`.
pub(crate) const RESTORE_NODE_WORLD_OBJ_SAVE: Callee = Callee {
    addr: 0x0034_fe90,
    f: ee::s11::WorldObj_Save,
};

/// `void RestoreNode_Save(RestoreNode* self, Stream* stream)` (0x351010): a
/// restore node saves only its world-object part: `WorldObj_Save(self,
/// stream)`. Frame 0x10 (ra +0).
pub fn restore_node_save(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(RESTORE_NODE_WORLD_OBJ_SAVE),
        0x0035_1020,
    );
}

// ---------------------------------------------------------------------------
// RestoreNode_ctor (0x350f60)

pub(crate) const RESTORENODE_WORLD_OBJ_CTOR: Callee = Callee {
    addr: 0x0034_fb00,
    f: ee::s11::f_0034fb00,
};

/// `RestoreNode* RestoreNode_ctor(RestoreNode* self, void* node)` (0x350f60):
/// construct the world object as kind 0x13 with `node`, install the
/// `RestoreNode` vtable (`0x491680`), then clear bits 5/6 of the node's
/// flags (+8) and set bit 2 (restore/dormant behaviour).
pub fn restore_node_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 1;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x13;
        RESTORENODE_WORLD_OBJ_CTOR.call(c, 0x0035_0f80)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A2][0] = Addr(r32(c, S0)).off(0x18).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1680));
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        let at = r32(c, S0);
        Addr(at).off(0xc).sw(c, c.g[V1][0]);
        c.g[A0][0] |= 0xff9f; // ~0x60
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = Addr(r32(c, A2)).off(0x8).lw(c);
        c.g[A1][0] |= 0xfffd; // ~2
        frame.restore_ra(c, 0);
        c.g[V1][0] &= c.g[A0][0];
        let node = r32(c, A2);
        Addr(node).off(0x8).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(r32(c, S0)).off(0x18).lw(c);
        frame.restore(c, S0, 0x10);
        c.g[V1][0] = Addr(r32(c, A0)).off(0x8).lw(c);
        c.g[V1][0] &= c.g[A1][0];
        c.g[V1][0] |= 0x4;
        let node = r32(c, A0);
        Addr(node).off(0x8).sw(c, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

// Callees (direct calls go through the readable alias, which dispatches
// through the callee's override slot).
pub(crate) const WORLD_OBJ_SAVE_CALLEE: Callee = Callee {
    addr: 0x0034_fe90,
    f: ee::s11::f_0034fe90,
};

/// `void DeadFade_Save(DeadFade *self)` (0x350e70): save the world object
/// part through `WorldObj_Save` and return.
pub fn dead_fade_save(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(WORLD_OBJ_SAVE_CALLEE), 0x0035_0e80);
}

// ---------------------------------------------------------------------------
// DeadFade_Render (0x350cb8)
// ---------------------------------------------------------------------------

/// `DeadFade_Render` (0x350cb8): draws the fade of the dead-rider overlay.
/// The frame is checked first (byte at +0x7d of the state, the table entry
/// at `[GP+0x16c8]`); when the fade is active it reads the timing from the
/// game table at `[S4+0x10d8]` (`S4` = `[GP+0x2a90]`), computes the fade
/// colour, and calls the two draw hooks through the function table.
pub(crate) fn dead_fade_render_body(c: &mut Cpu) -> Flow {
    c.cycles += 17;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0xe0));
    c.g[V0][0] = c.lw(oc604_ea(c.g[GP][0], 0x16c8));
    c.sq(21, sp(c).wrapping_add(0x70));
    c.sq(16, sp(c).wrapping_add(0xc0));
    c.g[S5][0] = c.g[A0][0];
    c.sq(17, sp(c).wrapping_add(0xb0));
    c.g[A0][0] = 0xff;
    c.sq(18, sp(c).wrapping_add(0xa0));
    c.sq(19, sp(c).wrapping_add(0x90));
    c.sq(20, sp(c).wrapping_add(0x80));
    c.sd(sp(c).wrapping_add(0x60), c.g[RA][0]);
    c.swc1(20, sp(c).wrapping_add(0xd0));
    c.g[A1][0] = c.lw(oc604_ea(c.g[S5][0], 0x18));
    c.g[A2][0] = c.lw(c.g[V0][0] as u32);
    c.g[V1][0] = c.lbu(oc604_ea(c.g[A1][0], 0x7d));
    let t = c.g[V1][0] == c.g[A0][0];
    c.g[V0][0] = 0x1;
    if !t {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(oc604_ea(c.g[A1][0], 0x7d));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = c.lw(oc604_ea(c.g[V0][0], 0x24));
        c.g[V1][0] ^= 0x6;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V0][0] = c.lh(oc604_ea(c.g[A1][0], 0x7e));
            c.g[A0][0] = 0x18;
            c.mult(0, 3, 2, 4);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V1][0] = c.lw(oc604_ea(c.g[V0][0], 0x3f0));
            c.g[V1][0] ^= 0x3;
            c.g[V0][0] = (c.g[V1][0] < 0x1) as u64;
        }
    }

    // The fade itself, when the first stage returned non-zero.
    c.cycles += 2;
    let active = c.g[V0][0] != 0;
    c.g[S4][0] = c.lw(oc604_ea(c.g[GP][0], 0x2a90));
    if active {
        c.cycles += 37;
        c.g[S2][0] = sx(sp(c).wrapping_add(0x20));
        c.g[V0][0] = c.lw(oc604_ea(c.g[S5][0], 0x18));
        c.g[S3][0] = sx(sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.lw(oc604_ea(c.g[S4][0], 0x10d8));
        c.g[A0][0] = c.g[S2][0];
        c.lwc1(0, oc604_ea(c.g[V0][0], 0x60));
        c.g[S0][0] = c.lh(oc604_ea(c.g[S1][0], 0x2e8));
        c.swc1(0, sp(c));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20);
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.lwc1(0, oc604_ea(c.g[V0][0], 0x64));
        c.swc1(0, sp(c).wrapping_add(0x4));
        c.lwc1(1, oc604_ea(c.g[V0][0], 0x68));
        c.swc1(20, sp(c).wrapping_add(0xc));
        c.swc1(1, sp(c).wrapping_add(0x8));
        c.lwc1(0, oc604_ea(c.g[V0][0], 0x6c));
        c.swc1(0, sp(c).wrapping_add(0x10));
        c.lwc1(1, oc604_ea(c.g[V0][0], 0x70));
        c.swc1(1, sp(c).wrapping_add(0x14));
        c.lwc1(0, oc604_ea(c.g[V0][0], 0x74));
        c.swc1(20, sp(c).wrapping_add(0x1c));
        c.swc1(0, sp(c).wrapping_add(0x18));
        c.g[V0][0] = c.lw(oc604_ea(c.g[S4][0], 0x10d8));
        c.g[A1][0] = c.lh(oc604_ea(c.g[V0][0], 0x158));
        c.g[V1][0] = c.lw(oc604_ea(c.g[V0][0], 0x15c));
        let t = c.g[V1][0] as u32;
        c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, t, 0x0035_0dac)?;
        c.g[V0][0] = c.lw(oc604_ea(c.g[S1][0], 0x2ec));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, t, 0x0035_0dc4)?;
        c.g[V1][0] = 0x1;
        let not_one = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0x500000;
        if not_one {
            c.cycles += 29;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
            oc604_quad_load(c, 1, V0);
            oc604_quad_store(c, 1, SP);
            c.lwc1(0, oc604_ea(c.g[S5][0], 0x30));
            c.g[A3][0] = c.g[SP][0];
            c.swc1(20, sp(c).wrapping_add(0x4c));
            c.g[T1][0] = 0;
            c.sw(sp(c).wrapping_add(0x40), 0u64);
            c.neg_s(0, 0);
            c.sw(sp(c).wrapping_add(0x44), 0u64);
            c.g[T2][0] = 0;
            c.g[A1][0] = c.lw(oc604_ea(c.g[S5][0], 0x18));
            c.swc1(0, sp(c).wrapping_add(0x48));
            c.lq(2, sp(c).wrapping_add(0x40));
            c.g[T0][0] = c.lw(oc604_ea(c.g[S5][0], 0x38));
            c.lwc1(12, oc604_ea(c.g[S5][0], 0x28));
            c.sq(2, sp(c).wrapping_add(0x30));
            c.g[V1][0] = c.lw(oc604_ea(c.g[S4][0], 0x10d8));
            c.g[A2][0] = c.lw(oc604_ea(c.g[A1][0], 0x94));
            c.g[A0][0] = c.lh(oc604_ea(c.g[V1][0], 0x2f8));
            c.g[V0][0] = c.lw(oc604_ea(c.g[V1][0], 0x2fc));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0035_0e44)?;
        }
    }
    c.cycles += 10;
    c.lq(16, sp(c).wrapping_add(0xc0));
    c.lq(17, sp(c).wrapping_add(0xb0));
    c.lq(18, sp(c).wrapping_add(0xa0));
    c.lq(19, sp(c).wrapping_add(0x90));
    c.lq(20, sp(c).wrapping_add(0x80));
    c.lq(21, sp(c).wrapping_add(0x70));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x60));
    c.lwc1(20, sp(c).wrapping_add(0xd0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0xe0));
    c.pc = t;
    Ok(())
}

/// `DeadFade_Render` (0x350cb8), see [`dead_fade_render_body`].
pub fn dead_fade_render(c: &mut Cpu) {
    guest(c, dead_fade_render_body)
}

/// `cInstanceNode_cInstanceNode1` (0x34fb00): the world-object base of the
/// fade node, kind 0x16 (the generated body calls it directly).
pub(crate) const DEAD_FADE_CTOR_2_BASE: Callee = Callee {
    addr: 0x0034_fb00,
    f: ee::s11::f_0034fb00,
};

/// The blend of one part's four rows (`src`, the part's source quads) with
/// the matrix `mat` (four quads), written through the stack scratch at `sp`
/// and into the output block at `T1`, with the `T3` matrix. The two
/// `DeadFade_ctor_2` loop bodies differ only in where `src` and `mat` come
/// from and in the register side effects their callers set up.
pub(crate) fn dead_fade_ctor_2_blend_rows(c: &mut Cpu, src: u32, mat: u32) {
    let s = sp(c);
    let t3 = r32(c, T3);
    let t1 = r32(c, T1);
    c.lwc1(6, s);
    c.lq(2, src);
    c.lwc1(7, s.wrapping_add(4));
    c.sq(2, s.wrapping_add(0x90));
    c.sq(2, s.wrapping_add(0x50));
    c.lwc1(5, s.wrapping_add(8));
    c.lq(2, src.wrapping_add(0x10));
    c.lwc1(4, s.wrapping_add(0xc));
    c.sq(2, s.wrapping_add(0x90));
    c.sq(2, s.wrapping_add(0x60));
    c.lq(3, src.wrapping_add(0x20));
    c.sq(3, s.wrapping_add(0x90));
    c.sq(3, s.wrapping_add(0x70));
    c.lq(2, src.wrapping_add(0x30));
    c.sq(2, s.wrapping_add(0xa0));
    c.lwc1(1, s.wrapping_add(0xa0));
    c.lwc1(2, s.wrapping_add(0xa4));
    c.lwc1(3, s.wrapping_add(0xa8));
    c.mul_s(1, 1, 6);
    c.lwc1(0, s.wrapping_add(0xac));
    c.mul_s(2, 2, 7);
    c.mul_s(3, 3, 5);
    c.mul_s(0, 0, 4);
    c.swc1(1, s.wrapping_add(0x90));
    c.swc1(2, s.wrapping_add(0x94));
    c.swc1(3, s.wrapping_add(0x98));
    c.swc1(0, s.wrapping_add(0x9c));
    c.lq(2, s.wrapping_add(0x90));
    c.sq(2, s.wrapping_add(0x80));
    c.lqc2(4, mat);
    c.lqc2(5, mat.wrapping_add(0x10));
    c.lqc2(6, mat.wrapping_add(0x20));
    c.lqc2(7, mat.wrapping_add(0x30));
    c.lqc2(8, t3);
    c.lqc2(9, t3.wrapping_add(0x10));
    c.lqc2(10, t3.wrapping_add(0x20));
    c.lqc2(11, t3.wrapping_add(0x30));
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
    c.sqc2(12, t1);
    c.sqc2(13, t1.wrapping_add(0x10));
    c.sqc2(14, t1.wrapping_add(0x20));
    c.sqc2(15, t1.wrapping_add(0x30));
}

/// `void cDeadFadeNode::cDeadFadeNode(self, bucket, obj, desc, other)`
/// (0x350750): builds a dead-fade node in place. The world-object part comes
/// from `cInstanceNode` (kind 0x16) with `obj`; then the node takes the
/// vtable 0x491980, its fade rate (`getUpdateTime / desc->FadeTime`) and
/// distance rate (`desc->ZVel * getUpdateTime`), and starts at distance 0 and
/// fade 1. Its per-part animation matrices (`AnimMats`, one 64-byte row per
/// model part) are copied from `other` when it has them, else composed from
/// the instance transform and the model's parts (parent -1 starts from the
/// instance's root transform). `other` is deleted afterwards and the
/// instance's flags get bit 2 (bits 5 and 6 cleared).
/// Frame 0x150: `ra` at +0xf0, s0..s4 at +0x140..+0x100.
pub fn dead_fade_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0x150);
        frame.save(c, S0, 0x140);
        frame.save(c, S1, 0x130);
        c.g[S0][0] = c.g[A3][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x120);
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S3, 0x110);
        frame.save(c, S4, 0x100);
        frame.save_ra(c, 0xf0);
        c.g[S2][0] = c.g[T0][0];
        c.g[A2][0] = 0x16;
        c.g[S4][0] = sx(r32(c, S1).wrapping_add(0x38));
        DEAD_FADE_CTOR_2_BASE.call(c, 0x0035_0788)?;

        c.g[V1][0] = 0x49_0000;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x1980));
        c.g[V0][0] |= 0xffff;
        c.sw(r32(c, S1).wrapping_add(0xc), c.g[V1][0]);
        c.sw(r32(c, S1).wrapping_add(0x1c), c.g[V0][0]);
        c.ldl(2, r32(c, S0).wrapping_add(0x7));
        c.ldr(2, r32(c, S0));
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x8));
        c.sdl(2, r32(c, S1).wrapping_add(0x23));
        c.sdr(2, r32(c, S1).wrapping_add(0x1c));
        c.sw(r32(c, S1).wrapping_add(0x24), c.g[V1][0]);

        // FadeRate = getUpdateTime / desc->FadeTime.
        call_ptr(c, 0x002d_1c70, 0x0035_07c0)?;
        c.lwc1(1, r32(c, S0).wrapping_add(0x8));
        c.div_s(0, 0, 1);
        c.swc1(0, r32(c, S1).wrapping_add(0x2c));
        // DistRate = desc->ZVel * getUpdateTime.
        call_ptr(c, 0x002d_1c70, 0x0035_07d0)?;
        c.lwc1(1, r32(c, S0).wrapping_add(0xc));
        c.g[A1][0] = 0x49_0000;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 2);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1758));
        c.mul_s(1, 1, 0);
        c.sw(r32(c, S1).wrapping_add(0x30), 0u64);
        c.swc1(2, r32(c, S1).wrapping_add(0x28));
        c.g[A2][0] = 0x2000_0000;
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x18));
        c.g[A3][0] = 0u64;
        c.swc1(1, r32(c, S1).wrapping_add(0x34));
        c.g[S3][0] = c.lw(r32(c, V0).wrapping_add(0x80));
        c.g[S0][0] = c.lw(r32(c, S3).wrapping_add(0x4));
        c.g[A0][0] = sx(r32(c, S0) << 6);
        // AnimMats = new (str_DeadFadeAnimMats, 0x20000000, 0) [count].
        call_ptr(c, 0x0031_7e30, 0x0035_0810)?;
        let no_parts = c.g[S0][0] == 0u64;
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(0x1));
        if !no_parts {
            c.cycles += 2;
            c.g[A0][0] = 0xffff_ffff_ffff_ffff;
        }

        'b_350b24: {
            'b_350878: {
                'b_350874: {
                    c.cycles += 2;
                    let other_is_null = c.g[S2][0] == 0u64;
                    c.sw(r32(c, S4), c.g[V0][0]);
                    if other_is_null {
                        break 'b_350874;
                    }
                    // other->hasAnimMats()
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(0xc));
                    c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0xd0));
                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xd4));
                    let has_anim_mats = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
                    call_ptr(c, has_anim_mats, 0x0035_083c)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x18));
                        break 'b_350878;
                    }
                    // other->getAnimMats(): copy model->count rows of 64 bytes.
                    c.cycles += 12;
                    c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(0xc));
                    c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0xd8));
                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xdc));
                    let get_anim_mats = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
                    call_ptr(c, get_anim_mats, 0x0035_0858)?;
                    c.g[A2][0] = c.lw(r32(c, S3).wrapping_add(0x4));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x38));
                    c.g[A2][0] = sx(r32(c, A2) << 6);
                    call_ptr(c, 0x003e_6574, 0x0035_086c)?;
                    break 'b_350b24;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x18));
            }

            // base transform: the instance's root transform when `other` gave none.
            'b_350898: {
                c.cycles += 2;
                let other_is_null = c.g[S2][0] == 0u64;
                c.g[T4][0] = sx(r32(c, V0).wrapping_add(0x10));
                if other_is_null {
                    break 'b_350898;
                }
                c.cycles += 6;
                c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(0xc));
                c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0xc0));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xc4));
                let root = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
                call_ptr(c, root, 0x0035_0894)?;
                c.g[T4][0] = c.g[V0][0];
            }

            c.cycles += 12;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x18));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.lwc1(0, r32(c, V0).wrapping_add(0x84));
            c.g[A3][0] = c.lw(r32(c, S3).wrapping_add(0x8));
            let s = sp(c);
            c.swc1(0, s.wrapping_add(0x8));
            c.swc1(1, s.wrapping_add(0xc));
            c.swc1(0, s);
            c.swc1(0, s.wrapping_add(0x4));
            c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x4));
            let no_parts = (c.g[V0][0] as i64) <= 0;
            c.g[T0][0] = 0u64;
            if no_parts {
                break 'b_350b24;
            }
            c.cycles += 4;
            c.g[T5][0] = 0xffff_ffff_ffff_ffff;
            c.g[T2][0] = sx(s.wrapping_add(0x10));
            c.g[T1][0] = sx(s.wrapping_add(0xb0));
            c.g[T3][0] = sx(s.wrapping_add(0x50));
            'l_3508d8: loop {
                c.cycles += 3;
                c.g[A2][0] = c.lw(r32(c, A3));
                let parent_is_none = c.g[A2][0] == c.g[T5][0];
                c.g[A2][0] = sx(r32(c, A2) << 6);
                if parent_is_none {
                    // Root part: the source is the part's own quads, the
                    // matrix the base transform in T4.
                    c.cycles += 58;
                    c.g[A0][0] = c.lw(r32(c, A3).wrapping_add(0xc));
                    let src = r32(c, A0);
                    let base = r32(c, T4);
                    dead_fade_ctor_2_blend_rows(c, src, base);
                } else {
                    // Child part: the matrix is the parent's row in AnimMats.
                    c.cycles += 62;
                    c.g[A1][0] = c.lw(r32(c, A3).wrapping_add(0xc));
                    c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x38));
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    let src = r32(c, A1);
                    let parent_row = r32(c, A2);
                    dead_fade_ctor_2_blend_rows(c, src, parent_row);
                }
                // Copy the blended row to the output block and advance.
                c.cycles += 24;
                c.lqc2(1, r32(c, T1));
                c.lqc2(2, r32(c, T1).wrapping_add(0x10));
                c.lqc2(3, r32(c, T1).wrapping_add(0x20));
                c.lqc2(4, r32(c, T1).wrapping_add(0x30));
                c.sqc2(1, r32(c, T2));
                c.sqc2(2, r32(c, T2).wrapping_add(0x10));
                c.sqc2(3, r32(c, T2).wrapping_add(0x20));
                c.sqc2(4, r32(c, T2).wrapping_add(0x30));
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x38));
                c.g[V0][0] = sx(r32(c, T0) << 6);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.lqc2(1, r32(c, T2));
                c.lqc2(2, r32(c, T2).wrapping_add(0x10));
                c.lqc2(3, r32(c, T2).wrapping_add(0x20));
                c.lqc2(4, r32(c, T2).wrapping_add(0x30));
                c.sqc2(1, r32(c, V0));
                c.sqc2(2, r32(c, V0).wrapping_add(0x10));
                c.sqc2(3, r32(c, V0).wrapping_add(0x20));
                c.sqc2(4, r32(c, V0).wrapping_add(0x30));
                c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x4));
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more_parts = c.g[V0][0] != 0u64;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x10));
                if more_parts {
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_08d8);
                    }
                    continue 'l_3508d8;
                }
                break 'b_350b24;
            }
        }

        'b_350b40: {
            c.cycles += 2;
            let other_is_null = c.g[S2][0] == 0u64;
            c.g[A1][0] = 0x3;
            if other_is_null {
                break 'b_350b40;
            }
            // The C++ `delete other`: a virtual call through other's vtable
            // slot at +0xc (delta at +8, as for the calls above).
            c.cycles += 5;
            c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(0xc));
            c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x8));
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
            let delete_slot = r32(c, V1);
            c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
            call_ptr(c, delete_slot, 0x0035_0b40)?;
        }

        // Instance flags: clear bits 5 and 6, set bit 2.
        c.cycles += 22;
        c.g[A2][0] = c.lw(r32(c, S1).wrapping_add(0x18));
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[A0][0] |= 0xff9f;
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x8));
        c.g[A1][0] |= 0xfffd;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x140);
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        frame.restore(c, S2, 0x120);
        c.sw(r32(c, A2).wrapping_add(0x8), c.g[V1][0]);
        frame.restore(c, S3, 0x110);
        c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x18));
        frame.restore(c, S1, 0x130);
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x8));
        frame.restore(c, S4, 0x100);
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        frame.restore_ra(c, 0xf0);
        c.g[V1][0] |= 0x4;
        c.sw(r32(c, A0).wrapping_add(0x8), c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
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
    pub(crate) use crate::game::bx::bxstringctor::tests::game;

    pub(crate) use crate::game::bx::bxstringctor::tests::SCRATCH;
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

    /// `RestoreNode_Save` through the real `WorldObj_Save` (and
    /// `SyncProcess_Save`), the stream's writers fakes (some unwind).
    #[test]
    fn restore_node_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        covered(
            ee::s11::f_00351010__generated,
            restore_node_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_1010, 3);
                c.pc = 0x0035_1010;
                let this = DATA + 0x100;
                let stream = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = stream as u64;
                let vt = vtable(c, &mut r, &[8, 0x30]);
                c.bus.write32(stream, vt);
            },
        );
    }

    use crate::difftest::fakes::fake;

    pub(crate) use crate::game::sound::bankmonitor::tests::OC329_CTRL;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC329_MAGIC;

    // -- RestoreNode_ctor -------------------------------------------------

    static OC329_PREV_WORLD_OBJ_CTOR: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn oc329_fake_world_obj_ctor(c: &mut Cpu) {
        if c.bus.read32(OC329_CTRL) != OC329_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &OC329_PREV_WORLD_OBJ_CTOR,
                ee::s11::f_0034fb00__generated,
            );
        }
        fake(c, 0x0034_fb00, c.g[RA][0] as u32);
    }

    #[test]
    fn restore_node_ctor_matches() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0034FB00),
            oc329_fake_world_obj_ctor,
            &OC329_PREV_WORLD_OBJ_CTOR,
        );
        check(
            ee::s11::f_00350f60__generated,
            restore_node_ctor,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_0f60, 0);
                c.pc = 0x0035_0f60;
                c.bus.write32(OC329_CTRL, OC329_MAGIC);
                let node = DATA + 0x200;
                let selfp = DATA + 0x100;
                c.bus.write32(node + 8, r.next_u32());
                c.bus.write32(selfp + 0x18, node);
                c.g[A0][0] = sx(selfp);
                c.g[A1][0] = sx(node);
            },
        );
    }

    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::scramble;
    use crate::difftest::Rng;

    pub(crate) use crate::game::object::instanceman::tests::gate_on;
    pub(crate) use crate::game::object::instanceman::tests::SEEDS as SEEDS_3;

    pub(crate) use crate::game::object::floatingnode::tests::LEAF;
    pub(crate) use crate::game::object::instanceman::tests::install_fakes;

    #[test]
    fn dead_fade_save_matches() {
        install_fakes();
        check(
            ee::s11::f_00350e70__generated,
            dead_fade_save,
            SEEDS_3,
            &|c, seed| {
                let mut r = Rng(0x587_0e70 ^ seed as u64);
                scramble(c, &mut r);
                gate_on(c);
                c.pc = 0x0035_0e70;
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 3) * 0x40);
            },
        );
    }

    #[test]
    fn dead_fade_render_matches() {
        check(
            ee::s11::f_00350cb8__generated,
            dead_fade_render,
            200,
            &|c, seed| {
                let mut r = Rng(0xdead ^ seed as u64);
                scramble(c, &mut r);
                let gp = crate::difftest::GAME_GP;
                let state = DATA + 0x100;
                let table = DATA + 0x2000;
                let timing = DATA + 0x2400;
                c.g[GP][0] = sx(gp);
                c.g[S5][0] = sx(state);
                c.bus.write32(gp + 0x16c8, DATA + 0x2800);
                c.bus.write32(gp + 0x2a90, DATA + 0x2c00);
                c.bus.write32(DATA + 0x2800, table);
                c.bus.write32(state + 0x18, DATA + 0x200);
                c.bus.write8(
                    DATA + 0x200 + 0x7d,
                    if seed % 4 == 0 {
                        0xff
                    } else {
                        (seed % 3) as u8
                    },
                );
                c.bus.write16(DATA + 0x200 + 0x7e, (seed % 5) as u16);
                c.bus.write32(table, table + 0x100);
                c.bus.write32(table + 0x100 + 0x24, (seed % 2) as u32 * 6);
                c.bus.write32(DATA + 0x2c00 + 0x10d8, timing);
                c.bus.write16(timing + 0x2e8, 0);
                c.bus.write16(timing + 0x158, 0);
                c.bus.write32(timing + 0x15c, LEAF);
                c.bus.write16(timing + 0x2f8, 0);
                c.bus.write32(timing + 0x2fc, LEAF);
                c.bus.write32(timing + 0x2ec, LEAF);
                c.bus.write32(DATA + 0x200 + 0x94, DATA + 0x2400);
                for k in 0..0x40 {
                    c.bus.write32(DATA + 0x200 + 4 * k, r.next_u32());
                }
            },
        );
    }

    /// Word that arms the test fake of the world-object constructor.
    pub(crate) const OC605_CTRL: u32 = DATA + 0xf_0000;

    pub(crate) const OC605_MAGIC: u32 = 0x0605_0750;

    pub(crate) static OC605_PREV_WORLD_OBJ_CTOR: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    /// `cInstanceNode_cInstanceNode1` with the gate on: a logged fake call
    /// (it can unwind); with the gate off, the body the slot held before.
    pub(crate) fn oc605_fake_world_obj_ctor(c: &mut Cpu) {
        if c.bus.read32(OC605_CTRL) != OC605_MAGIC {
            return fall_back(
                c,
                &OC605_PREV_WORLD_OBJ_CTOR,
                ee::s11::f_0034fb00__generated,
            );
        }
        let ret = c.g[RA][0] as u32;
        fake(c, 0x0034_fb00, ret);
    }

    /// A node with `seed % 4` parts (the instance, model and parts written
    /// out), `other` null on every third seed, and on every fifth seed its
    /// `hasAnimMats` slot returns zero (the real leaf at 0x1406c0).
    pub(crate) fn oc605_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x35_0750, 6);
        c.pc = 0x0035_0750;
        c.bus.write32(OC605_CTRL, OC605_MAGIC);
        crate::game::scripter::ssxscriptengine::tests::heap_init(c);

        let this = DATA + 0x100;
        let obj = DATA + 0x1000;
        let desc = DATA + 0x2000;
        let other = if seed % 3 == 0 { 0 } else { DATA + 0x3000 };
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(r.next_u32());
        c.g[A2][0] = sx(obj);
        c.g[A3][0] = sx(desc);
        c.g[T0][0] = sx(other);

        c.bus.write32(desc + 0x8, r.f32(20.0));
        c.bus.write32(desc + 0xc, r.f32(20.0));

        let inst = DATA + 0x4000;
        let model = DATA + 0x4400;
        let parts = DATA + 0x4800;
        c.bus.write32(this + 0x18, inst);
        c.bus.write32(inst + 0x80, model);
        c.bus.write32(inst + 0x84, r.f32(2.0));
        c.bus.write32(model + 0x4, seed % 4);
        c.bus.write32(model + 0x8, parts);
        for i in 0..3u32 {
            let part = parts + 0x10 * i;
            let parent = if (seed + i) % 2 == 0 { u32::MAX } else { 0 };
            c.bus.write32(part, parent);
            c.bus.write32(part + 0xc, DATA + 0x5000 + 0x100 * i);
        }
        fill(c, &mut r, DATA + 0x5000, 0x40);

        if other != 0 {
            let vt = vtable(c, &mut r, &[0xc0, 0xd0, 0xd8]);
            if seed % 5 == 0 {
                c.bus.write32(vt + 0xd4, 0x0014_06c0);
            }
            c.bus.write32(other + 0xc, vt);
        }
    }

    #[test]
    fn dead_fade_ctor_2_matches() {
        hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0034FB00),
            oc605_fake_world_obj_ctor,
            &OC605_PREV_WORLD_OBJ_CTOR,
        );
        check(
            ee::s11::f_00350750__generated,
            dead_fade_ctor_2,
            200,
            &|c, seed| oc605_setup(c, seed),
        );
    }
}
