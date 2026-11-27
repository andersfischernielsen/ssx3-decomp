//! `src/object/modifierblock.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ridermetrix::swc1_r;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::main::gameload::vcall;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::misc::lit;
pub(crate) use crate::game::movie::movieplayer::add32;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::object::floatingnode::oc604_ea;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::world::world::oc435_callee;
pub(crate) use crate::game::wscript::wscriptman::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_1c20, world_get_camera_eye, "World_GetCameraEye"),
    (
        0x002d_1c58,
        world_get_current_view_index,
        "World_GetCurrentViewIndex",
    ),
    (0x0035_2aa8, modifier_block_ctor, "ModifierBlock_ctor"),
    (
        0x0035_2bf8,
        modifier_block_get_transform_radius,
        "ModifierBlock_GetTransformRadius",
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
    (
        0x0035_2f40,
        modifier_block_handle_event,
        "ModifierBlock_HandleEvent",
    ),
    (
        0x0035_3020,
        modifier_block_forward_v43,
        "ModifierBlock_ForwardV43",
    ),
    (
        0x0035_3098,
        modifier_block_forward_v42,
        "ModifierBlock_ForwardV42",
    ),
    (
        0x0035_3228,
        modifier_block_reset_particles,
        "ModifierBlock_ResetParticles",
    ),
    (
        0x0035_3278,
        modifier_block_remove_dead_particles,
        "ModifierBlock_RemoveDeadParticles",
    ),
    (
        0x0035_3418,
        modifier_block_has_rail_modifier,
        "ModifierBlock_HasRailModifier",
    ),
    (0x0035_3ac0, sky_box_ctor, "SkyBox_ctor"),
    (
        0x0035_3da8,
        flex_bridge_springs_ctor,
        "FlexBridge_SpringsCtor",
    ),
    (
        0x0035_3db8,
        flex_bridge_springs_dtor,
        "FlexBridge_SpringsDtor",
    ),
    (
        0x0035_3e08,
        flex_bridge_springs_init,
        "FlexBridge_SpringsInit",
    ),
    (
        0x0035_3e80,
        flex_bridge_springs_load,
        "FlexBridge_SpringsLoad",
    ),
    (0x0035_3448, modifier_block_save, "ModifierBlock_Save"),
    (0x0035_3b10, skybox_render, "SkyBox_Render"),
    (
        0x0035_2bc0,
        modifier_block_set_transform_bounds,
        "ModifierBlock_SetTransformBounds",
    ),
    (0x0035_2ae8, modifier_block_dtor, "ModifierBlock_dtor"),
    (
        0x0035_3300,
        modifier_block_free_rail_modifiers,
        "ModifierBlock_FreeRailModifiers",
    ),
    (
        0x0035_2c38,
        modifier_block_set_transform_radius,
        "ModifierBlock_SetTransformRadius",
    ),
    (
        0x0035_3398,
        modifier_block_take_particle_list,
        "ModifierBlock_TakeParticleList",
    ),
    (
        0x0035_2f08,
        modifier_block_clear_transform_finished,
        "ModifierBlock_ClearTransformFinished",
    ),
    (0x0035_3cf0, sky_box_set_section_2, "SkyBox_SetSection"),
    (
        0x0035_2e98,
        modifier_block_update_transform,
        "ModifierBlock_UpdateTransform",
    ),
    (0x0035_3ae8, sky_box_dtor, "SkyBox_dtor"),
    (0x0035_36d8, modifier_block_load_3, "ModifierBlock_Load"),
];

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
}

// ---------------------------------------------------------------------------
// Callees (generated shard functions, reached through their override slots).

pub(crate) mod callee {

    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;

    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES as FLEX_BRIDGE_ALLOC_SPRING_NODES_2;
}

// ---------------------------------------------------------------------------
// World.

/// `int World_GetCurrentViewIndex(void)` (0x2d1c58): follows three pointers out
/// of the world singleton (at `gp - 0x848`, then +0x84, +0x84) and returns the
/// view index at +0x14.
pub fn world_get_current_view_index(c: &mut Cpu) {
    c.cycles += 5;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = world;
    let views = Addr(world as u32).off(0x84).lw(c);
    c.g[V1][0] = views;
    let view = Addr(views as u32).off(0x84).lw(c);
    c.g[A0][0] = view;
    c.g[V0][0] = Addr(view as u32).off(0x14).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World camera

/// `Vec3* World_GetCameraEye(World* w, int view)` (0x2d1c20): return
/// `view[view].eye + 0x20`.
pub fn world_get_camera_eye(c: &mut Cpu) {
    c.cycles += 8;
    let view = abi::arg(c, 0) as u32;
    let world = Addr(c.g[GP][0] as u32 - 0x848).lw(c);
    let viewer = Addr(world as u32).off(0x84).lw(c);
    let table = Addr(viewer as u32).off(0x84).lw(c);
    let entry = (table as u32).wrapping_add(view.wrapping_shl(2));
    c.g[A1][0] = world;
    c.g[A0][0] = sx(view.wrapping_shl(2));
    c.g[V1][0] = sx(entry);
    let eye = Addr(entry).off(0x4).lw(c);
    c.g[V0][0] = sx((eye as u32).wrapping_add(0x20));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Assorted constructors.

/// `ModifierBlock* ModifierBlock_ctor(ModifierBlock *b)` (0x352aa8): install
/// the vtable pointers and clear the block.
pub fn modifier_block_ctor(c: &mut Cpu) {
    c.cycles += 16;
    let b = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x490000;
    c.g[V1][0] = 0x490000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1340));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1200));
    b.off(0x18).sw(c, c.g[V0][0]);
    b.off(0x24).sw(c, c.g[V1][0]);
    c.g[V0][0] = c.g[A0][0];
    b.off(0x10).sw(c, 0);
    b.off(0x14).sw(c, 0);
    b.off(0x1c).sw(c, 0);
    b.off(0x20).sw(c, 0);
    b.sw(c, 0);
    b.off(4).sw(c, 0);
    b.off(8).sw(c, 0);
    b.off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `SkyBox* SkyBox_ctor(SkyBox *s)` (0x353ac0): install the vtable pointer and
/// clear the sky box.
pub fn sky_box_ctor(c: &mut Cpu) {
    c.cycles += 10;
    let s = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x490000;
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xff8));
    s.off(0x10).sw(c, c.g[V1][0]);
    s.off(0xc).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    s.off(4).sw(c, 0);
    s.off(8).sw(c, 0);
    s.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `FlexBridge_Springs* FlexBridge_SpringsCtor(FlexBridge_Springs *s)`
/// (0x353da8): return the object with its spring count cleared.
pub fn flex_bridge_springs_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).off(0x24).sw(c, 0);
    abi::ret(c);
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

/// `float ModifierBlock_GetTransformRadius(ModifierBlock* mb)` (0x352bf8):
/// virtual slot 14 of the block's node, 0 when there is none.
pub fn modifier_block_get_transform_radius(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(r32(c, A0));
        if c.g[A1][0] != 0 {
            c.g[V0][0] = c.lw(r32(c, A1));
        } else {
            c.cycles += 3;
            c.mtc1(0, 0);
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = c.lh(at(c, V0, 0x70));
        c.g[V1][0] = c.lw(at(c, V0, 0x74));
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_2c28)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `ModifierBlock_HasRailModifier(block, id)` (0x353418): walk the modifier
/// list at `+0x1c` for a node whose id (`+0x30`) matches.
pub fn modifier_block_has_rail_modifier(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
    if c.g[V1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lw(c);
    loop {
        c.cycles += 2;
        if c.g[V0][0] != c.g[A1][0] {
            c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lw(c);
                if events::event_due(c) {
                    events::deliver_event_here(c, 0x0035_3428);
                }
                continue;
            }
            break;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void ModifierBlock_ForwardV42(ModifierBlock* block, int arg)`
/// (0x353098): call the wrapped node's `v02(arg)`, if there is a node.
pub fn modifier_block_forward_v42(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(abi::arg(c, 0)).lw(c);
        let node = c.g[A2][0];
        frame.restore_ra(c, 0);
        if node != 0 {
            c.cycles += 6;
            let vt = Addr(node as u32).lw(c);
            c.g[V0][0] = vt;
            let m = VSlot::read(c, vt, 0xb0);
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(node);
            call_ptr(c, m.func as u32, 0x0035_30c0)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void ModifierBlock_ResetParticles(ModifierBlock* self)` (0x353228):
/// call the third virtual method of every modifier link.
pub fn modifier_block_reset_particles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = Addr(r32(c, A1)).off(8).lw(c);
            loop {
                c.cycles += 8;
                c.g[S0][0] = Addr(r32(c, A1)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).off(0x18).lh(c);
                c.g[V1][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0035_325c)?;
                c.g[A1][0] = c.g[S0][0];
                if c.g[A1][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V0][0] = Addr(r32(c, A1)).off(8).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_3248);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void FlexBridge_SpringsDtor(FlexBridge* self, int flags)` (0x353db8):
/// free the spring array and, when `flags & 1`, the object.
pub fn flex_bridge_springs_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(p(this, 0x24)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, table::BUILTIN_VEC_DELETE, 0x0035_3de0)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0035_3df4)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_00353020` (0x353020): `ModifierBlock_ForwardV43`. Forwards a modifier
/// call to the block's first child (`this->+0` through `vtable[0xc8]`) and then
/// to the block itself (`child->+0xc` through `vtable[0x158]`, with the two
/// saved arguments). Signature: `void forward(this, a1, a2)`.
pub fn modifier_block_forward_v43(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = lw_r(c, A0, 0);
        let none = c.g[A3][0] == 0;
        c.g[S1][0] = c.g[A2][0];
        if none {
            c.cycles += 1;
            c.g[A3][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V1][0] = Addr(low(c, A3)).lw(c);
            c.g[A0][0] = lh_r(c, V1, 0xc8);
            c.g[V0][0] = lw_r(c, V1, 0xcc);
            let target = low(c, V0);
            c.g[RA][0] = sx(0x0035_3054);
            c.g[A0][0] = sx(low(c, A3).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0035_3054)?;
            c.g[A3][0] = c.g[V0][0];
        }
        c.cycles += 2;
        let skip = c.g[A3][0] == 0;
        c.g[A1][0] = c.g[S0][0];
        if !skip {
            c.cycles += 6;
            c.g[V0][0] = lw_r(c, A3, 0xc);
            c.g[A2][0] = c.g[S1][0];
            c.g[A0][0] = lh_r(c, V0, 0x158);
            c.g[V1][0] = lw_r(c, V0, 0x15c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0035_3080);
            c.g[A0][0] = sx(low(c, A3).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0035_3080)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FlexBridge_SpringsInit(self, desc)` (0x353e08): copy the five spring
/// parameter floats, scale the sixth, compute the segment length and build the
/// spring nodes.
pub fn flex_bridge_springs_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[V1][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.lwc1(2, gpo(c, -0x2a88));
        c.g[V0][0] = lw_r(c, A1, 0x1c);
        sw_r(c, V1, 0, c.g[V0][0]);
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(1));
        c.lwc1(0, low(c, A1));
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        swc1_r(c, 0, V1, 0x4);
        c.div_s(12, 12, 1);
        c.lwc1(0, low(c, A1).wrapping_add(0x4));
        swc1_r(c, 0, V1, 0x8);
        c.lwc1(1, low(c, A1).wrapping_add(0x8));
        swc1_r(c, 1, V1, 0xc);
        c.lwc1(0, low(c, A1).wrapping_add(0xc));
        swc1_r(c, 0, V1, 0x10);
        c.lwc1(1, low(c, A1).wrapping_add(0x10));
        swc1_r(c, 1, V1, 0x14);
        c.lwc1(0, low(c, A1).wrapping_add(0x14));
        c.mul_s(0, 0, 2);
        swc1_r(c, 0, V1, 0x18);
        c.lwc1(1, low(c, A1).wrapping_add(0x18));
        swc1_r(c, 12, V1, 0x20);
        swc1_r(c, 1, V1, 0x1c);
        callee::FLEX_BRIDGE_ALLOC_SPRING_NODES.call(c, 0x0035_3e74)?;
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void* FlexBridge_SpringsLoad(self, stream)` (0x00353e80): load the spring
/// nodes and tell the owner.
pub fn flex_bridge_springs_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x24;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(at(c, S1, 0)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x10)).lh(c);
        c.g[V0][0] = Addr(at(c, V1, 0x14)).lw(c);
        c.g[A0][0] = add32(word(c, S1), word(c, A0));
        call_ptr(c, word(c, V0), 0x0035_3eb4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::FLEX_BRIDGE_ALLOC_SPRING_NODES_2.call(c, 0x0035_3ebc)?;
        c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
        c.g[A2][0] = 0x50;
        c.g[V1][0] = Addr(at(c, S1, 0)).lw(c);
        c.mult(0, V0, V0, A2);
        c.g[A1][0] = Addr(at(c, S0, 0x24)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x10)).lh(c);
        c.g[V0][0] = Addr(at(c, V1, 0x14)).lw(c);
        c.g[A2][0] = c.lo[0];
        c.g[A0][0] = add32(word(c, S1), word(c, A0));
        call_ptr(c, word(c, V0), 0x0035_3ee4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Modifier blocks.

/// `void ModifierBlock_RemoveDeadParticles(ModifierBlock* self)` (0x353278):
/// walk the block's particle links; a live one (its virtual `state()` is 3)
/// is deleted and unlinked.
pub fn modifier_block_remove_dead_particles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(r32(c, A0), 0x10)).lw(c);
        let null = c.g[A0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if !null {
            c.cycles += 2;
            c.g[S2][0] = 0x3;
            c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
            loop {
                c.cycles += 7;
                c.g[S1][0] = Addr(r32(c, S0)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
                let target = r32(c, V0);
                c.g[RA][0] = sx(0x0035_32b4);
                c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                call_ptr(c, target, 0x0035_32b4)?;
                if c.g[V0][0] != c.g[S2][0] {
                    c.g[S0][0] = c.g[S1][0];
                } else {
                    c.cycles += 2;
                    let dead = c.g[S0][0] == 0;
                    c.g[A1][0] = 0x3;
                    if !dead {
                        c.cycles += 5;
                        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                        c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
                        c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
                        let target = r32(c, V1);
                        c.g[RA][0] = sx(0x0035_32d8);
                        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                        call_ptr(c, target, 0x0035_32d8)?;
                    }
                    c.cycles += 1;
                    c.g[S0][0] = c.g[S1][0];
                }
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_32a0);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SkyBox_SetSection(self, int id)` (0x353cf0): set the section id and,
/// when it is valid, look up the model reference's index.
pub fn sky_box_set_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let valid = (c.g[S0][0] as i64) >= 0;
        Addr(p(r32(c, S1), 0x10)).sw(c, c.g[S0][0]);
        let mut one = false;
        if !valid {
            c.cycles += 4;
            call_ptr(c, 0x0035_3d98, 0x00353d18)?;
        } else {
            one = true;
            c.cycles += 11;
            call_ptr(c, 0x002d_1bd8, 0x00353d28)?;
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = 0x58;
            c.g[V1][0] = Addr(r32(c, A2)).lw(c);
            c.mult(0, 2, 16, 2);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
            c.g[V0][0] = sx(p(r32(c, V0), r32(c, A0)));
            c.g[V1][0] = Addr(p(r32(c, V0), 0x26)).lh(c);
            let none = (c.g[V1][0] as i64) <= 0;
            c.g[A1][0] = c.g[S0][0] & 0xff;
            if !none {
                c.cycles += 6;
                c.g[V1][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[V0][0] = sx(r32(c, A1) << 2);
                c.g[V0][0] = sx(p(r32(c, V0), r32(c, V1)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x1c)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                    c.g[V0][0] = sx(r32(c, V0) >> 8);
                    let nonzero = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if !nonzero {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                }
                c.cycles += 1;
                Addr(p(r32(c, S1), 0x14)).sw(c, c.g[V0][0]);
            }
        }
        if one {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModifierBlock_HandleEvent (0x0000352f40).

/// `ModifierBlock_HandleEvent`: dispatch an event to the modifier block's handlers.
pub fn modifier_block_handle_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_352f7c: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A1][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
            let t = c.g[A2][0] == 0u64;
            c.mov_s(20, 12);
            if t {
                break 'b_352f7c;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[RA][0] = sx(0x00352f7c);
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x00352f7c {
                    return Err(crate::call::Unwound);
                }
                break 'b_352f7c;
            }
        }
        'b_352f90: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_352f90;
            } else {
                c.cycles += 2;
                c.mov_s(12, 20);
                call_ptr(c, 0x0035fb30, 0x00352f90)?;
                break 'b_352f90;
            }
        }
        'b_352fa4: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_352fa4;
            } else {
                c.cycles += 2;
                c.mov_s(12, 20);
                call_ptr(c, 0x0035f598, 0x00352fa4)?;
                break 'b_352fa4;
            }
        }
        'b_352fc8: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_352fc8;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.mov_s(12, 20);
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[RA][0] = sx(0x00352fc8);
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x00352fc8 {
                    return Err(crate::call::Unwound);
                }
                break 'b_352fc8;
            }
        }
        'b_35300c: {
            c.cycles += 3;
            c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            if c.g[S0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_35300c;
            } else {
                c.cycles += 3;
                c.g[A2][0] = c.g[S0][0];
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                'l_352fe0: loop {
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = c.lw(c.g[A2][0] as u32);
                    c.mov_s(12, 20);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[RA][0] = sx(0x00352ffc);
                    c.pc = t;
                    ssx3_ee_rt::funcs::call_indirect(c, t);
                    if c.pc != 0x00352ffc {
                        return Err(crate::call::Unwound);
                    }
                    c.g[A2][0] = c.g[S0][0];
                    if c.g[A2][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        if event_due(c) {
                            deliver_event_here(c, 0x00352fe0);
                        }
                        continue 'l_352fe0;
                    } else {
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_35300c;
                    }
                }
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}

pub(crate) mod skybox {

    /// gp+0x143c: sky drawing disabled; gp+0x2a90: the renderer.
    pub const GP_SKY_OFF: u32 = 0x143c;

    pub const GP_RENDERER: u32 = 0x2a90;

    /// Renderer: render-state stack top (0x14-byte records), vtable.
    pub const RS_TOP: u32 = 0xe84;

    pub const RENDERER_VT: u32 = 0x10d8;

    /// `DrawModel(renderer, model, 1.0, textures, matrix, 0, 0, 0x5420)`.
    pub const SLOT_DRAW_MODEL: u32 = 0x2f8;

    pub const RS_SIZE: u32 = 0x14;

    /// Record word 2: layer bits 5..9.
    pub const RS_LAYER_WORD: u32 = 0x8;

    pub const LAYER_KEEP: u64 = 0xffff_ffff_ffff_fc1f;

    /// The view matrix the sky is drawn with (eye replaced by the camera's).
    pub const VIEW_MATRIX: u32 = 0x004f_f1a0;

    /// `SkyBox`: the model (`+8` flags, `+0x80` its texture set at +0xc).
    pub const MODEL: u32 = 0x14;

    /// `World_GetCurrentViewIndex()`, `World_GetCameraEye(view)`.
    pub const GET_VIEW_INDEX: u32 = 0x002d_1c58;

    pub const GET_CAMERA_EYE: u32 = 0x002d_1c20;
}

/// Modifier block: the modifiers it holds.
pub(crate) const MB_PRIMARY: u32 = 0x0;

pub(crate) const MB_UV_SCROLL: u32 = 0x4;

pub(crate) const MB_RAIL: u32 = 0x8;

pub(crate) const MB_SECONDARY: u32 = 0xc;

/// Intrusive lists (`next` at +0, the object's vtable at +8).
pub(crate) const MB_ID_LIST: u32 = 0x10;

pub(crate) const MB_EXTRA_LIST: u32 = 0x1c;

/// The 0x34-byte header at sp+0: flags, two ids, two counts, 32 id bytes.
pub(crate) const HDR_FLAGS: u32 = 0x0;

pub(crate) const HDR_PRIMARY_ID: u32 = 0x4;

pub(crate) const HDR_SECONDARY_ID: u32 = 0x8;

pub(crate) const HDR_ID_COUNT: u32 = 0xc;

pub(crate) const HDR_EXTRA_COUNT: u32 = 0x10;

pub(crate) const HDR_IDS: u32 = 0x14;

/// `void ModifierBlock_Save(ModifierBlock* block, Stream* stream)`
/// (0x353448): write a header (which modifiers are present, the ids of the
/// primary and secondary ones, the id byte of each node in the id list, the
/// length of the extra list) with `stream->Write(&hdr, 0x34)`, then let every
/// present modifier and list node save itself to the stream.
pub fn modifier_block_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        let hdr = sp(c);
        c.g[V1][0] = 0x1f;
        frame.save(c, S1, 0x70);
        c.g[V0][0] = sx(hdr + HDR_IDS + 0x1f);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        c.g[S3][0] = sx(hdr + HDR_IDS);
        frame.save_ra(c, 0x40);
        Addr(hdr).off(HDR_FLAGS).sw(c, 0);
        Addr(hdr).off(HDR_SECONDARY_ID).sw(c, 0);
        Addr(hdr).off(HDR_PRIMARY_ID).sw(c, 0);
        Addr(hdr).off(HDR_ID_COUNT).sw(c, 0);
        Addr(hdr).off(HDR_EXTRA_COUNT).sw(c, 0);
        let block = r32(c, S1);
        let stream = c.g[S2][0];

        // Clear the 32 id bytes, last to first.
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sb(c, 0);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            if (c.g[V1][0] as i64) < 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_3488);
            }
        }

        // Flags and ids.
        c.cycles += 3;
        c.g[A1][0] = Addr(block).off(MB_PRIMARY).lw(c);
        c.g[V0][0] = Addr(hdr).off(HDR_FLAGS).lw(c);
        if c.g[A1][0] != 0 {
            c.cycles += 8;
            or_flag(c, hdr, 0x1);
            let obj = c.g[A1][0];
            c.g[V1][0] = Addr(obj as u32).lw(c);
            let vt = c.g[V1][0];
            vcall(c, obj, vt, 0xd0, V0, 0x0035_34cc)?;
            let id = c.g[V0][0];
            Addr(sp(c)).off(HDR_PRIMARY_ID).sw(c, id);
        }
        for (field, bit) in [(MB_UV_SCROLL, 0x2), (MB_RAIL, 0x4)] {
            c.cycles += 3;
            let present = Addr(block).off(field).lw(c) != 0;
            c.g[V0][0] = Addr(hdr).off(HDR_FLAGS).lw(c);
            if present {
                c.cycles += 2;
                or_flag(c, hdr, bit);
            }
        }
        c.cycles += 3;
        c.g[A1][0] = Addr(block).off(MB_SECONDARY).lw(c);
        c.g[V0][0] = Addr(hdr).off(HDR_FLAGS).lw(c);
        if c.g[A1][0] != 0 {
            c.cycles += 8;
            or_flag(c, hdr, 0x8);
            let obj = c.g[A1][0];
            c.g[V1][0] = Addr(obj as u32).off(4).lw(c);
            let vt = c.g[V1][0];
            vcall(c, obj, vt, 0x28, V0, 0x0035_3520)?;
            let id = c.g[V0][0];
            Addr(sp(c)).off(HDR_SECONDARY_ID).sw(c, id);
        }

        // One id byte per node of the id list.
        c.cycles += 3;
        c.g[S0][0] = Addr(block).off(MB_ID_LIST).lw(c);
        c.g[V0][0] = Addr(hdr).off(HDR_ID_COUNT).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V1][0] = Addr(r32(c, S0)).off(8).lw(c);
            loop {
                c.cycles += 12;
                let (node, vt) = (c.g[S0][0], c.g[V1][0]);
                vcall(c, node, vt, 0x10, V0, 0x0035_3548)?;
                let hdr = sp(c);
                c.g[A0][0] = Addr(hdr).off(HDR_ID_COUNT).lw(c);
                c.g[V1][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
                let id = c.g[V0][0];
                Addr(r32(c, V1)).sb(c, id);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                let n = c.g[A0][0];
                Addr(hdr).off(HDR_ID_COUNT).sw(c, n);
                c.g[S0][0] = Addr(r32(c, S0)).lw(c);
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(hdr).off(HDR_ID_COUNT).lw(c);
                    break;
                }
                c.g[V1][0] = Addr(r32(c, S0)).off(8).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_3538);
                }
            }
        }
        let hdr = sp(c);
        c.cycles += 2;
        let has_ids = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = Addr(hdr).off(HDR_FLAGS).lw(c);
        if has_ids {
            c.cycles += 2;
            or_flag(c, hdr, 0x10);
        }

        // Count the extra list.
        c.cycles += 3;
        c.g[V1][0] = Addr(block).off(MB_EXTRA_LIST).lw(c);
        c.g[V0][0] = Addr(hdr).off(HDR_EXTRA_COUNT).lw(c);
        if c.g[V1][0] != 0 {
            loop {
                c.cycles += 7;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                let n = c.g[V0][0];
                Addr(hdr).off(HDR_EXTRA_COUNT).sw(c, n);
                c.g[V1][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = Addr(hdr).off(HDR_EXTRA_COUNT).lw(c);
                if c.g[V1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_3588);
                }
            }
        }
        c.cycles += 2;
        let has_extra = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = Addr(hdr).off(HDR_FLAGS).lw(c);
        if has_extra {
            c.cycles += 2;
            or_flag(c, hdr, 0x20);
        }

        // stream->Write(&hdr, 0x34).
        c.cycles += 10;
        c.g[V0][0] = Addr(stream as u32).lw(c);
        c.g[A2][0] = 0x34;
        c.g[A1][0] = sx(hdr);
        let vt = c.g[V0][0];
        vcall(c, stream, vt, 0x8, V1, 0x0035_35d0)?;
        c.g[A2][0] = Addr(block).off(MB_PRIMARY).lw(c);
        c.g[A1][0] = stream;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            let obj = c.g[A2][0];
            c.g[V0][0] = Addr(obj as u32).lw(c);
            let vt = c.g[V0][0];
            vcall(c, obj, vt, 0xd8, V1, 0x0035_35f0)?;
        }

        // The UV-scroll and rail modifiers save themselves.
        c.cycles += 3;
        c.g[A0][0] = Addr(block).off(MB_UV_SCROLL).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = stream;
            Callee {
                addr: 0x0035_fd98,
                f: ee::s12::f_0035fd98,
            }
            .call(c, 0x0035_3604)?;
        }
        c.g[A0][0] = Addr(block).off(MB_RAIL).lw(c);
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = stream;
            Callee {
                addr: 0x0035_f688,
                f: ee::s12::f_0035f688,
            }
            .call(c, 0x0035_3618)?;
        }
        c.g[A2][0] = Addr(block).off(MB_SECONDARY).lw(c);
        c.cycles += 2;
        c.g[A1][0] = stream;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            let obj = c.g[A2][0];
            c.g[V0][0] = Addr(obj as u32).off(4).lw(c);
            let vt = c.g[V0][0];
            vcall(c, obj, vt, 0x30, V1, 0x0035_3638)?;
        }

        // The id-list nodes, if any had ids.
        let hdr = sp(c);
        c.cycles += 3;
        let id_count = Addr(hdr).off(HDR_ID_COUNT).lw(c);
        c.g[V0][0] = Addr(hdr).off(HDR_EXTRA_COUNT).lw(c);
        if (id_count as i64) > 0 {
            c.cycles += 3;
            c.g[S0][0] = Addr(block).off(MB_ID_LIST).lw(c);
            if c.g[S0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = stream;
                    let (node, vt) = (c.g[S0][0], c.g[V0][0]);
                    vcall(c, node, vt, 0x20, V1, 0x0035_366c)?;
                    c.g[S0][0] = Addr(r32(c, S0)).lw(c);
                    if c.g[S0][0] == 0 {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(sp(c)).off(HDR_EXTRA_COUNT).lw(c);
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_3658);
                    }
                }
            }
        }

        // The extra-list nodes, if any.
        c.cycles += 2;
        let has_extra = (c.g[V0][0] as i64) > 0;
        frame.restore(c, S0, 0x80);
        if has_extra {
            c.cycles += 3;
            c.g[S0][0] = Addr(block).off(MB_EXTRA_LIST).lw(c);
            if c.g[S0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = stream;
                    let (node, vt) = (c.g[S0][0], c.g[V0][0]);
                    vcall(c, node, vt, 0x10, V1, 0x0035_36ac)?;
                    c.g[S0][0] = Addr(r32(c, S0)).lw(c);
                    if c.g[S0][0] == 0 {
                        c.cycles += 1;
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_3698);
                    }
                }
            }
            frame.restore(c, S0, 0x80);
        }

        c.cycles += 6;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });

    /// `v0 |= bit; hdr.flags = v0` (v0 holds the flags just loaded).
    fn or_flag(c: &mut Cpu, hdr: u32, bit: u64) {
        c.g[V0][0] |= bit;
        let v = c.g[V0][0];
        Addr(hdr).off(HDR_FLAGS).sw(c, v);
    }
}

/// `void SkyBox_Render(SkyBox* sky)` (0x353b10): unless disabled or
/// without a model, draw the sky model with the view matrix moved to the
/// camera eye, under a pushed render state (layer 0, z test on, blend 2,
/// z write off, alpha ref 0x14 in the state's mode word) restored with
/// the old layer afterwards; the model's flag bit 1 is cleared and bit 2
/// set. Frame 0xa0 (matrix at +0, s0-s3).
pub fn skybox_render(c: &mut Cpu) {
    use skybox::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(GP_SKY_OFF));
        frame.save(c, S1, 0x80);
        frame.save(c, S0, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save_ra(c, 0x50);
        let sky = r32(c, S1);
        'out: {
            if c.g[V0][0] != 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(sky.wrapping_add(MODEL));
            c.g[S0][0] = c.lw(r32(c, GP).wrapping_add(GP_RENDERER));
            if c.g[V0][0] == 0 {
                break 'out;
            }
            let rend = r32(c, S0);
            c.cycles += 74;
            // The view matrix, eye replaced by the camera's.
            c.g[V0][0] = sx(VIEW_MATRIX);
            let s = sp(c);
            for k in 0..4u8 {
                c.lqc2(1 + k as usize, VIEW_MATRIX + 0x10 * k as u32);
            }
            for k in 0..4u8 {
                c.sqc2(1 + k as usize, s + 0x10 * k as u32);
            }
            c.g[S3][0] = LAYER_KEEP;
            call_ptr(c, GET_VIEW_INDEX, 0x0035_3b70)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, GET_CAMERA_EYE, 0x0035_3b78)?;
            let s = sp(c);
            c.lq(3, r32(c, V0));
            c.g[A1][0] = 0xffff_ffff_ffbf_ffff;
            c.g[T2][0] = 0x40_0000;
            c.sq(3, s.wrapping_add(0x30));
            c.g[A2][0] = 0xffff_ffff_fe7f_ffff;
            c.sq(3, s.wrapping_add(0x40));
            c.g[T3][0] = 0x100_0000;
            c.g[A3][0] = 0xffff_ffff_ffcf_ffff;
            // Push: copy the top record over the next one.
            let top = c.lw(rend.wrapping_add(RS_TOP));
            c.g[V0][0] = top;
            let top = top as u32;
            c.g[T0][0] = 0xffff_ffff_fff0_0000;
            c.g[T1][0] = 0x1_0000;
            c.ldl(V1, top.wrapping_add(0x7));
            c.ldr(V1, top);
            c.ldl(A0, top.wrapping_add(0xf));
            c.ldr(A0, top.wrapping_add(0x8));
            c.g[T4][0] = c.lw(top.wrapping_add(0x10));
            c.sdl(V1, top.wrapping_add(0x1b));
            c.sdr(V1, top.wrapping_add(0x14));
            c.sdl(A0, top.wrapping_add(0x23));
            c.sdr(A0, top.wrapping_add(0x1c));
            c.sw(top.wrapping_add(0x24), c.g[T4][0]);
            c.g[T0][0] |= 0xfff;
            c.g[T1][0] |= 0x4000;
            c.g[A0][0] = c.lw(rend.wrapping_add(RS_TOP));
            c.g[V1][0] = sx(at(c, A0, RS_SIZE));
            c.sw(rend.wrapping_add(RS_TOP), c.g[V1][0]);
            // Layer 0, remembering the old one (s2).
            c.g[V0][0] = c.lw(at(c, A0, RS_SIZE + RS_LAYER_WORD));
            c.g[V1][0] = c.g[V0][0] & c.g[S3][0];
            c.sw(at(c, A0, RS_SIZE + RS_LAYER_WORD), c.g[V1][0]);
            c.g[V0][0] &= 0x3e0;
            c.g[S2][0] = sx(r32(c, V0) >> 5);
            // Word 0 |= 0xc; word 1: z test on, blend 2, z write off,
            // alpha ref 0x14.
            c.g[V1][0] = c.lw(rend.wrapping_add(RS_TOP));
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[V0][0] |= 0xc;
            c.sw(r32(c, V1), c.g[V0][0]);
            for (base, keep, set) in [
                (A0, A1, Some(T2)),
                (A1, A2, Some(T3)),
                (V1, A3, None),
                (A0, T0, Some(T1)),
            ] {
                c.g[base][0] = c.lw(rend.wrapping_add(RS_TOP));
                c.g[V0][0] = c.lw(at(c, base, 4));
                c.g[V0][0] &= c.g[keep][0];
                if let Some(set) = set {
                    c.g[V0][0] |= c.g[set][0];
                }
                c.sw(at(c, base, 4), c.g[V0][0]);
            }
            c.g[A0][0] = c.lw(sky.wrapping_add(MODEL));
            c.g[A3][0] = c.g[SP][0];
            if c.g[A0][0] != 0 {
                c.cycles += 19;
                c.g[V1][0] = c.lw(at(c, A0, 8));
                c.g[V0][0] = 0xffff_ffff_ffff_fffd;
                lit(c, 12, 0x3f80_0000);
                c.g[V1][0] &= c.g[V0][0];
                c.g[V1][0] |= 0x4;
                c.g[T0][0] = 0;
                c.sw(at(c, A0, 8), c.g[V1][0]);
                c.g[T1][0] = 0;
                c.g[T2][0] = 0x5420;
                c.g[A1][0] = c.lw(sky.wrapping_add(MODEL));
                c.g[V0][0] = c.lw(rend.wrapping_add(RENDERER_VT));
                c.g[T3][0] = c.lw(at(c, A1, 0x80));
                c.g[A0][0] = c.lh(at(c, V0, SLOT_DRAW_MODEL));
                c.g[V1][0] = c.lw(at(c, V0, SLOT_DRAW_MODEL + 4));
                c.g[A2][0] = c.lw(at(c, T3, 0xc));
                let method = r32(c, V1);
                c.g[A0][0] = sx(rend.wrapping_add(r32(c, A0)));
                call_ptr(c, method, 0x0035_3cb4)?;
            }
            // Pop, restoring the old layer.
            c.cycles += 8;
            c.g[A0][0] = c.lw(rend.wrapping_add(RS_TOP));
            c.g[A1][0] = sx(r32(c, S2) << 5);
            c.g[V1][0] = sx(r32(c, A0).wrapping_sub(RS_SIZE));
            c.sw(rend.wrapping_add(RS_TOP), c.g[V1][0]);
            let word = r32(c, A0).wrapping_sub(RS_SIZE - RS_LAYER_WORD);
            c.g[V0][0] = c.lw(word);
            c.g[V0][0] &= c.g[S3][0];
            c.g[V0][0] |= c.g[A1][0];
            c.sw(word, c.g[V0][0]);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------ ModifierBlock_SetTransformBounds

/// `sBoundBoxNode` vtable slot 13 (`setBoundBox`): this-delta (s16) at
/// +0x68, method at +0x6c.
pub(crate) const MB_SET_BOUND_BOX_SLOT: u32 = 0x68;

/// `void ModifierBlock_SetTransformBounds(tModifierBlock* self,
/// BoundBox* box)` (0x352bc0): forward the box to the block's bound node
/// (`self->node`, at +0) through its virtual `setBoundBox`, if it has one.
/// Frame 0x10 (ra only; reloaded before the null test as the original does).
pub fn modifier_block_set_transform_bounds(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let node = Addr(r32(c, A0)).lw(c);
        c.g[A2][0] = node;
        frame.restore_ra(c, 0);
        if node != 0 {
            c.cycles += 6;
            let node = node as u32;
            let vt = Addr(node).lw(c);
            c.g[V0][0] = vt;
            let delta = Addr((vt as u32).wrapping_add(MB_SET_BOUND_BOX_SLOT)).lh(c);
            let method = Addr((vt as u32).wrapping_add(MB_SET_BOUND_BOX_SLOT + 4)).lw(c);
            c.g[V1][0] = method;
            // node->setBoundBox(box): a1 passes through.
            c.g[A0][0] = sx(node.wrapping_add(delta as u32));
            call_ptr(c, method as u32, 0x0035_2be8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00352ae8 ModifierBlock_dtor

/// ModifierBlock fields written by [`modifier_block_dtor`]: the vtables of
/// the embedded rail-modifier list (`+0x1c`, vtable at `+0x24`) and
/// modifier list node (`+0x10`, vtable at `+0x18`).
pub(crate) const MB_MOD_LIST: u32 = 0x10;

pub(crate) const MB_MOD_LIST_VTABLE: u32 = 0x18;

pub(crate) const MB_RAIL_LIST: u32 = 0x1c;

pub(crate) const MB_RAIL_LIST_VTABLE: u32 = 0x24;

/// `ModifierBlock* ModifierBlock_dtor(ModifierBlock* self, int flags)`
/// (0x352ae8): free the transform, UV-scroll, texture-flip, boost and
/// particle modifiers and every rail modifier (owner -1), restore the two
/// embedded lists' base vtables and unlink them; `operator delete` when
/// `flags & 1`.
pub fn modifier_block_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        let frees = [
            Callee {
                addr: 0x0035_30d0,
                f: ee::s11::ModifierBlock_FreeTransform,
            },
            Callee {
                addr: 0x0035_3118,
                f: ee::s11::ModifierBlock_FreeUVScroll,
            },
            Callee {
                addr: 0x0035_3150,
                f: ee::s11::ModifierBlock_FreeTexFlip,
            },
            Callee {
                addr: 0x0035_3188,
                f: ee::s11::ModifierBlock_FreeBoost,
            },
            Callee {
                addr: 0x0035_31d0,
                f: ee::s11::ModifierBlock_FreeParticles,
            },
        ];
        for (i, free) in frees.into_iter().enumerate() {
            if i > 0 {
                c.g[A0][0] = c.g[S1][0];
            }
            free.call(c, 0x0035_2b04 + 8 * i as u32)?;
        }
        // Every rail modifier, whatever its owner (-1).
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        Callee {
            addr: 0x0035_3300,
            f: ee::s11::ModifierBlock_FreeRailModifiers,
        }
        .call(c, 0x0035_2b34)?;
        let this = c.g[S1][0] as u32;
        c.g[V0][0] = 0x0049_1200;
        c.g[A0][0] = sx(this.wrapping_add(MB_RAIL_LIST));
        c.sw(this.wrapping_add(MB_RAIL_LIST_VTABLE), 0x0049_1200);
        Callee {
            addr: 0x0035_b6d0,
            f: ee::s11::RailModifier_ListUnlink,
        }
        .call(c, 0x0035_2b48)?;
        let this = c.g[S1][0] as u32;
        c.g[V0][0] = 0x0049_1340;
        c.g[A0][0] = sx(this.wrapping_add(MB_MOD_LIST));
        c.sw(this.wrapping_add(MB_MOD_LIST_VTABLE), 0x0049_1340);
        Callee {
            addr: 0x0034_5760,
            f: ee::s11::ModifierList_Unlink,
        }
        .call(c, 0x0035_2b5c)?;
        c.g[S0][0] &= 0x1;
        let delete = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0035_2b70)?; // operator delete
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ModifierBlock`: head of the rail modifier list at +0x1c. A list link:
/// next at +0, vtable at +0x8 (`Release(link, how)` at slot 0x8), id at
/// +0x30.
pub(crate) const MOD_BLOCK_RAIL_HEAD: u32 = 0x1c;

pub(crate) const MOD_BLOCK_LINK_NEXT: u32 = 0x0;

pub(crate) const MOD_BLOCK_LINK_VT: u32 = 0x8;

pub(crate) const MOD_BLOCK_LINK_ID: u32 = 0x30;

pub(crate) const MOD_BLOCK_RELEASE: u32 = 0x8;

/// `link->Release(3)` through the link's vtable (link in a2), returning to
/// `back`.
pub(crate) fn mod_block_release_link(c: &mut Cpu, back: u32) -> crate::call::Flow {
    let link = c.g[A2][0] as u32;
    let vt = Addr(link.wrapping_add(MOD_BLOCK_LINK_VT)).lw(c);
    c.g[V0][0] = vt;
    c.g[A1][0] = 3;
    let delta = Addr((vt as u32).wrapping_add(MOD_BLOCK_RELEASE)).lh(c);
    let release = Addr((vt as u32).wrapping_add(MOD_BLOCK_RELEASE + 4)).lw(c);
    c.g[V1][0] = release;
    c.g[A0][0] = sx(link.wrapping_add(delta as u32));
    call_ptr(c, release as u32, back)
}

/// `void ModifierBlock_FreeRailModifiers(ModifierBlock* self, u32 id)`
/// (0x353300): release (`Release(3)`) the rail modifier with `id`, or every
/// rail modifier when `id` is -1. Both list walks check events at their
/// heads (0x353320, 0x353358).
pub fn modifier_block_free_rail_modifiers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let head = Addr((c.g[A0][0] as u32).wrapping_add(MOD_BLOCK_RAIL_HEAD)).lw(c);
        c.g[A0][0] = head;
        let id = c.g[A1][0];
        c.g[V0][0] = !id;
        let mut free_all_done = head == 0;
        if head != 0 {
            c.cycles += 2;
            c.g[A2][0] = head;
            if !id != 0 {
                // Release the one modifier whose id matches.
                loop {
                    c.cycles += 2;
                    if c.g[A2][0] == 0 {
                        break;
                    }
                    c.cycles += 3;
                    let link_id = Addr((c.g[A2][0] as u32).wrapping_add(MOD_BLOCK_LINK_ID)).lw(c);
                    c.g[V0][0] = link_id;
                    if link_id == c.g[A1][0] {
                        c.cycles += 8;
                        mod_block_release_link(c, 0x0035_334c)?;
                        break;
                    }
                    c.g[A2][0] = Addr((c.g[A2][0] as u32).wrapping_add(MOD_BLOCK_LINK_NEXT)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_3320);
                    }
                }
            } else {
                // Release every modifier (the next link read first).
                loop {
                    c.cycles += 2;
                    let link = c.g[A2][0];
                    c.g[S0][0] = Addr((link as u32).wrapping_add(MOD_BLOCK_LINK_NEXT)).lw(c);
                    if link != 0 {
                        c.cycles += 6;
                        mod_block_release_link(c, 0x0035_3378)?;
                    }
                    c.cycles += 3;
                    c.g[A2][0] = c.g[S0][0];
                    if c.g[A2][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_3358);
                    }
                }
                free_all_done = true;
            }
        }
        if free_all_done {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModifierBlock_TakeParticleList (0x353398).

/// `void ModifierBlock_TakeParticleList(ModifierBlock *self, List *list)`
/// (0x353398): for each particle node in `list`, call its vtable slot +0x18
/// method on `node + delta`, push the node onto the block's list
/// (`self + 0x10`), then clear the source list's head and tail.
pub fn modifier_block_take_particle_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = Addr(r32(c, S2)).lw(c);
        let empty = c.g[S1][0] == 0;
        c.g[S3][0] = c.g[A0][0];
        if !empty {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
            'nodes: loop {
                c.cycles += 11;
                c.g[S0][0] = Addr(r32(c, S1)).lw(c);
                let slot = crate::call::VSlot::read(c, c.g[V0][0], 0x18);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(slot.delta as u32));
                c.g[V1][0] = slot.func;
                call_ptr(c, slot.func as u32, 0x0035_33dc)?;
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0035_33e8);
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(0x10));
                MODIFIER_LIST_PUSH_BACK.call(c, 0x0035_33e8)?;
                c.g[S1][0] = c.g[S0][0];
                if c.g[S1][0] == 0 {
                    break 'nodes;
                }
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_33c8);
                }
            }
        }
        c.cycles += 9;
        c.sw(r32(c, S2).wrapping_add(0x4), 0);
        c.sw(r32(c, S2), 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ModifierList_PushBack(List *list, void *node)` 0x345720.
pub(crate) const MODIFIER_LIST_PUSH_BACK: Callee = Callee {
    addr: 0x0034_5720,
    f: ee::s11::f_00345720,
};

/// `void tModifierBlock_setRadius(tModifierBlock* self, ...)` (0x352c38): call
/// the handler at slot `+0x7c` of the vtable held by `*(*self)` with the
/// adjusted `this` (`*(*self) + slot +0x78`).
pub fn modifier_block_set_transform_radius(c: &mut Cpu) {
    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = Addr(r32(c, A0)).lw(c);
    frame.restore_ra(c, 0);
    if c.g[A1][0] != 0 {
        c.cycles += 6;
        c.g[V0][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x78).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x7c).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0035_2c60);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        if call_ptr(c, target, 0x0035_2c60).is_err() {
            return;
        }
        frame.restore_ra(c, 0);
    }
    c.cycles += 2;
    frame.close(c);
}

// ----------------------------------- ModifierBlock_ClearTransformFinished

/// Vtable entry 7 (`{delta, fn}` at +0x38): the transform modifier's
/// "clear finished" method.
pub(crate) const MODIFIER_BLOCK_CLEAR_FINISHED_SLOT: u32 = 0x38;

/// `void ModifierBlock_ClearTransformFinished(tModifierBlock* b)`
/// (0x00352f08, `tModifierBlock::clearComplete`): if the block has a
/// transform modifier (+0), call its virtual "clear finished" method.
pub fn modifier_block_clear_transform_finished(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(r32(c, A0)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[A1][0] != 0 {
            c.cycles += 6;
            let modifier = r32(c, A1);
            c.g[V0][0] = Addr(modifier).lw(c);
            let vt = r32(c, V0);
            c.g[A0][0] = Addr(vt.wrapping_add(MODIFIER_BLOCK_CLEAR_FINISHED_SLOT)).lh(c);
            c.g[V1][0] = Addr(vt.wrapping_add(MODIFIER_BLOCK_CLEAR_FINISHED_SLOT + 4)).lw(c);
            c.g[A0][0] = sx(modifier.wrapping_add(r32(c, A0)));
            let method = r32(c, V1);
            call_ptr(c, method, 0x0035_2f30)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void SkyBox_SetSection(SkyBox *self, int section)` (0x353cf0): stores the
/// section at `+0x10`; a negative one clears the box (`SkyBox_Clear`), else
/// looks the section's world mesh up (via 0x2d1bd8 -> manager, entry `section *
/// 0x58`) and, when it has meshes (`+0x26 > 0`) and the table entry
/// `table[section & 0xff]` exists, stores `(entry->+0x1c->word >> 8) * 4` (or
/// 0) at `+0x14`. Frame 0x30 (s0, s1).
pub fn sky_box_set_section_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let non_negative = (c.g[S0][0] as i64) >= 0;
        Addr(r32(c, S1).wrapping_add(0x10)).sw(c, c.g[S0][0]);
        if !non_negative {
            c.cycles += 4;
            oc435_callee!(0x0035_3d98, ee::s11::SkyBox_Clear).call(c, 0x0035_3d18)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 11;
            call_ptr(c, 0x002d_1bd8, 0x0035_3d28)?;
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = 0x58;
            c.g[V1][0] = Addr(r32(c, A2)).lw(c);
            c.mult(0, 2, 16, 2);
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x26)).lh(c);
            let no_meshes = (c.g[V1][0] as i64) <= 0;
            c.g[A1][0] = c.g[S0][0] & 0xff;
            if !no_meshes {
                c.cycles += 6;
                c.g[V1][0] = Addr(r32(c, A2).wrapping_add(8)).lw(c);
                c.g[V0][0] = sx(r32(c, A1) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x1c)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                    c.g[V0][0] = sx(r32(c, V0) >> 8);
                    let nonzero = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if !nonzero {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                }
                c.cycles += 1;
                Addr(r32(c, S1).wrapping_add(0x14)).sw(c, c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const GAME_PROCESS_DTOR_CALLEE: Callee = Callee {
    addr: 0x0035_46c8,
    f: ee::s11::f_003546c8,
};

/// `void ModifierBlock_UpdateTransform(ModifierBlock **self)` (0x352e98): when
/// the block `*self` is set, call the hook at `*(*block) + 0x1c` with
/// `a0 = block + lh(*(*block) + 0x18)` and `a1 = block`.
pub fn modifier_block_update_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(c.g[A0][0] as u32).lw(c);
        let null = c.g[A1][0] == 0;
        frame.restore_ra(c, 0);
        if !null {
            c.cycles += 6;
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0035_2ec0)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void SkyBox_dtor(SkyBox *self)` (0x353ae8): set the vtable word at
/// `self+0xc` and run `GameProcess_dtor` (0x3546c8) on it.
pub fn sky_box_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0xff8));
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        let vt = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, vt);
        GAME_PROCESS_DTOR_CALLEE.call(c, 0x0035_3b04)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModifierBlock_Load (0x3536d8): not installed yet (see the note below).
// ---------------------------------------------------------------------------

/// `ModifierBlock_Load` (0x3536d8): reads the modifier type word of a block and
/// builds the typed modifier objects it lists (spline, multi-spline, parent,
/// position, roller, magnet, ava-spline, and the sub-lists: boost, particles,
/// halo, rails), storing each into the block and pushing list entries.
///
/// Not in `OVERRIDES`: its eight constructors and the pool allocator
/// (`0x317d70`) need a real heap in the fixture, and the constructors
/// depend on the modifier table, so the differential test is not set up yet.
/// The port is kept here, written from the generated body.
pub fn modifier_block_load(c: &mut Cpu) {
    guest(c, modifier_block_load_body)
}

/// The typed constructor of one modifier group: `size` bytes from the pool
/// (`0x490000 - off` names the type), the constructor `ctor` on the block in
/// `S1`, the result stored at `[S2 + store]`. `cyc` is the group's cycle
/// charge on the allocation path.
pub(crate) fn oc604_mb_make(
    c: &mut Cpu,
    size: u64,
    off: u32,
    alloc_ret: u32,
    ctor: u32,
    ctor_ret: u32,
    store: u32,
    cyc: u64,
) -> Flow {
    c.cycles += cyc;
    c.g[A1][0] = 0x490000;
    c.g[A0][0] = size;
    c.g[A1][0] = sx(0x490000u32.wrapping_sub(off));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, alloc_ret)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, ctor, ctor_ret)?;
    c.sw(oc604_ea(c.g[S2][0], store), c.g[V0][0]);
    Ok(())
}

/// One group of the type chain: `V0` holds the type it compares with (set by
/// the caller), `next` is what `V0` becomes. True when the group matched.
pub(crate) fn oc604_mb_group(
    c: &mut Cpu,
    entry_cycles: u64,
    next: u64,
    size: u64,
    off: u32,
    alloc_ret: u32,
    ctor: u32,
    ctor_ret: u32,
) -> Result<bool, Unwound> {
    c.cycles += entry_cycles;
    let other = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = next;
    if other {
        return Ok(false);
    }
    oc604_mb_make(c, size, off, alloc_ret, ctor, ctor_ret, 0, 11)?;
    Ok(true)
}

pub(crate) fn modifier_block_load_body(c: &mut Cpu) -> Flow {
    c.cycles += 28;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0xe0));
    c.g[A2][0] = 0x34;
    c.sq(18, sp(c).wrapping_add(0xb0));
    c.sq(17, sp(c).wrapping_add(0xc0));
    c.g[S2][0] = c.g[A0][0];
    c.sq(16, sp(c).wrapping_add(0xd0));
    c.g[S1][0] = c.g[A1][0];
    c.sq(19, sp(c).wrapping_add(0xa0));
    c.g[A1][0] = c.g[SP][0];
    c.sq(20, sp(c).wrapping_add(0x90));
    c.sq(21, sp(c).wrapping_add(0x80));
    c.sq(22, sp(c).wrapping_add(0x70));
    c.sq(23, sp(c).wrapping_add(0x60));
    c.sq(30, sp(c).wrapping_add(0x50));
    c.sd(sp(c).wrapping_add(0x40), c.g[RA][0]);
    c.sw(c.g[S2][0] as u32, 0u64);
    c.sw(oc604_ea(c.g[S2][0], 0x4), 0u64);
    c.sw(oc604_ea(c.g[S2][0], 0x8), 0u64);
    c.sw(oc604_ea(c.g[S2][0], 0xc), 0u64);
    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
    c.g[A0][0] = c.lh(oc604_ea(c.g[V0][0], 0x10));
    c.g[V1][0] = c.lw(oc604_ea(c.g[V0][0], 0x14));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x0035_3738)?;
    c.g[V0][0] = c.lw(sp(c) as u32);
    c.g[V0][0] &= 0x1;
    let none = c.g[V0][0] == 0;
    c.g[V1][0] = c.lw(sp(c).wrapping_add(0x4));
    let mut matched = false;
    if !none {
        // Group 1: the spline.
        c.g[V0][0] = 0x1;
        matched = oc604_mb_group(c, 3, 2, 0xf0, 0x1730, 0x0035_376c, 0x0035_95d8, 0x0035_3778)?;
        if !matched {
            // Group 2: the multi-spline.
            matched = oc604_mb_group(c, 2, 4, 0x58, 0x1730, 0x0035_37a0, 0x0035_a118, 0x0035_37ac)?;
        }
        if !matched {
            // Group 3: the parent (type 4 in the table).
            matched = oc604_mb_group(c, 2, 3, 0xb0, 0x1720, 0x0035_37d4, 0x0035_7090, 0x0035_37e0)?;
        }
        if !matched {
            // Group 4: the position modifier.
            matched = oc604_mb_group(c, 2, 5, 0x90, 0x1710, 0x0035_3808, 0x0035_6f68, 0x0035_3814)?;
        }
        if !matched {
            // Group 5: the roller.
            matched = oc604_mb_group(
                c,
                2,
                6,
                0x2d0,
                0x16f8,
                0x0035_383c,
                0x0035_df70,
                0x0035_3848,
            )?;
        }
        if !matched {
            // Group 6: the magnet.
            matched = oc604_mb_group(c, 2, 7, 0xb0, 0x16e8, 0x0035_3870, 0x0035_7358, 0x0035_387c)?;
        }
        if !matched {
            // Group 7: the ava-spline (compares with 7).
            c.cycles += 2;
            let other = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.lw(sp(c) as u32);
            if !other {
                oc604_mb_make(
                    c,
                    0x90,
                    0x16d8,
                    0x0035_38a4,
                    0x0035_7798,
                    0x0035_38b0,
                    0,
                    10,
                )?;
                matched = true;
            }
        }
    }
    if none || matched {
        c.cycles += 1;
        c.g[V0][0] = c.lw(sp(c) as u32);
    }

    // Stage 2: the sub-objects from the flags word (bit 1, 2, 3).
    c.cycles += 3;
    c.g[V0][0] &= 0x2;
    let t = c.g[V0][0] == 0;
    c.g[A0][0] = 0x50;
    if !t {
        oc604_mb_make(
            c,
            0x50,
            0x16c0,
            0x0035_38d8,
            0x0035_f788,
            0x0035_38e4,
            0x4,
            9,
        )?;
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw(sp(c) as u32);
    c.g[V0][0] &= 0x4;
    let t = c.g[V0][0] == 0;
    c.g[A0][0] = 0xe8;
    if !t {
        oc604_mb_make(
            c,
            0xe8,
            0x16a8,
            0x0035_390c,
            0x0035_f1a8,
            0x0035_3918,
            0x8,
            9,
        )?;
    }

    // Stage 3: boost (flag 8), with the second flag word.
    c.cycles += 4;
    c.g[V0][0] = c.lw(sp(c) as u32) & 0x8;
    let no_boost = c.g[V0][0] == 0;
    c.g[V1][0] = c.lw(sp(c).wrapping_add(0x8));
    let mut boost_done = no_boost;
    if !no_boost {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        let skip = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0xc));
        if !skip {
            oc604_mb_make(
                c,
                0x50,
                0x1698,
                0x0035_3950,
                0x0034_2808,
                0x0035_395c,
                0xc,
                10,
            )?;
            boost_done = true;
        }
    }
    if boost_done {
        c.cycles += 1;
    }
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0xc));

    // Stage 4: the object list (count at [sp+0xc]), particles, halo, then
    // the rail list (count at [sp+0x10]).
    c.cycles += 2;
    let count = c.g[V0][0] as i64;
    c.g[S0][0] = 0;
    if count > 0 {
        c.cycles += 7;
        c.g[S3][0] = sx(sp(c).wrapping_add(0x14));
        c.g[FP][0] = 0x490000;
        c.g[S7][0] = 0x2;
        c.g[S6][0] = 0x490000;
        c.g[S5][0] = 0x3;
        c.g[S4][0] = 0x490000;
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
        loop {
            let mut pushed = false;
            c.cycles += 4;
            c.g[V1][0] = 0x1;
            c.g[V0][0] = c.lb(c.g[V0][0] as u32);
            let other = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = 0x1f0;
            if !other {
                c.cycles += 9;
                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x1688));
                c.g[A2][0] = 0x2000_0000;
                c.g[RA][0] = sx(0x0035_39a8);
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0035_39a8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0034_59a8, 0x0035_39b4)?;
                c.g[A1][0] = c.g[V0][0];
                pushed = true;
            }
            if !pushed {
                c.cycles += 2;
                let other = c.g[V0][0] != c.g[S7][0];
                c.g[A0][0] = 0x270;
                if !other {
                    c.cycles += 9;
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1670));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[RA][0] = sx(0x0035_39d4);
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, 0x0035_39d4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0034_5d80, 0x0035_39e0)?;
                    c.g[A1][0] = c.g[V0][0];
                    pushed = true;
                }
            }
            if !pushed {
                c.cycles += 2;
                let other = c.g[V0][0] != c.g[S5][0];
                c.g[V0][0] = 0;
                if !other {
                    c.cycles += 10;
                    c.g[A0][0] = 0x40;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1658));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[RA][0] = sx(0x0035_3a04);
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, 0x0035_3a04)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0034_61c0, 0x0035_3a10)?;
                    c.g[A1][0] = c.g[V0][0];
                    pushed = true;
                }
                if !pushed {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[V0][0];
                }
            }
            c.cycles += 7;
            c.g[RA][0] = sx(0x0035_3a24);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x0034_5720, 0x0035_3a24)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = c.lw(sp(c).wrapping_add(0xc));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_3988);
            }
        }
    }
    c.cycles += 3;
    c.g[S0][0] = 0;
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 3;
        c.g[S3][0] = 0x490000;
        c.g[A2][0] = 0x2000_0000;
        loop {
            c.cycles += 15;
            c.g[A3][0] = 0;
            c.g[A0][0] = 0x90;
            c.g[RA][0] = sx(0x0035_3a60);
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1648));
            call_ptr(c, 0x0031_7d70, 0x0035_3a60)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0035_ba88, 0x0035_3a70)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1c));
            call_ptr(c, 0x0035_b690, 0x0035_3a7c)?;
            c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = 0x2000_0000;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_3a50);
            }
        }
    }
    c.cycles += 12;
    c.lq(16, sp(c).wrapping_add(0xd0));
    c.lq(17, sp(c).wrapping_add(0xc0));
    c.lq(18, sp(c).wrapping_add(0xb0));
    c.lq(19, sp(c).wrapping_add(0xa0));
    c.lq(20, sp(c).wrapping_add(0x90));
    c.lq(21, sp(c).wrapping_add(0x80));
    c.lq(22, sp(c).wrapping_add(0x70));
    c.lq(23, sp(c).wrapping_add(0x60));
    c.lq(30, sp(c).wrapping_add(0x50));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0xe0));
    c.pc = t;
    Ok(())
}

/// `cMemMan_alloc` (0x317d70, shard s10): called through the function table.
pub(crate) const OC664_ALLOC: u32 = 0x0031_7d70;

/// The UV scroll and tex flip modifiers (shard s12): called through the table.
pub(crate) const OC664_UV_SCROLL_CTOR: u32 = 0x0035_f788;

pub(crate) const OC664_TEX_FLIP_CTOR: u32 = 0x0035_f1a8;

/// Constructors, appenders and helpers the bodies call by name (shard s11).
pub(crate) const MB_SPLINE_CTOR: Callee = Callee {
    addr: 0x0035_95d8,
    f: ee::s11::f_003595d8,
};

pub(crate) const MB_MULTI_SPLINE_CTOR: Callee = Callee {
    addr: 0x0035_a118,
    f: ee::s11::f_0035a118,
};

pub(crate) const MB_PARENT_CTOR: Callee = Callee {
    addr: 0x0035_7090,
    f: ee::s11::f_00357090,
};

pub(crate) const MB_POSITION_CTOR: Callee = Callee {
    addr: 0x0035_6f68,
    f: ee::s11::f_00356f68,
};

pub(crate) const MB_ROLLER_CTOR: Callee = Callee {
    addr: 0x0035_df70,
    f: ee::s11::f_0035df70,
};

pub(crate) const MB_MAGNET_CTOR: Callee = Callee {
    addr: 0x0035_7358,
    f: ee::s11::f_00357358,
};

pub(crate) const MB_AVA_SPLINE_CTOR: Callee = Callee {
    addr: 0x0035_7798,
    f: ee::s11::f_00357798,
};

pub(crate) const MB_BOOST_CTOR: Callee = Callee {
    addr: 0x0034_2808,
    f: ee::s11::f_00342808,
};

pub(crate) const MB_PARTICLE_CTOR: Callee = Callee {
    addr: 0x0034_59a8,
    f: ee::s11::f_003459a8,
};

pub(crate) const MB_DYNAMIC_PARTICLE_CTOR: Callee = Callee {
    addr: 0x0034_5d80,
    f: ee::s11::f_00345d80,
};

pub(crate) const MB_HALO_CTOR: Callee = Callee {
    addr: 0x0034_61c0,
    f: ee::s11::f_003461c0,
};

pub(crate) const MB_EFFECT_APPEND: Callee = Callee {
    addr: 0x0034_5720,
    f: ee::s11::f_00345720,
};

pub(crate) const MB_RAIL_CTOR: Callee = Callee {
    addr: 0x0035_ba88,
    f: ee::s11::f_0035ba88,
};

pub(crate) const MB_RAIL_APPEND: Callee = Callee {
    addr: 0x0035_b690,
    f: ee::s11::f_0035b690,
};

/// One arm of the `hdr.type` chain: compares V1 with `kind` (V0 holds it),
/// leaves `next` in V0, and on a match allocates `size` bytes named
/// `0x490000 - name` and constructs the modifier with `ctor`.
#[derive(Clone, Copy)]
pub(crate) struct Oc664RootArm {
    kind: u64,
    next: u64,
    entry: u64,
    size: u64,
    name: u32,
    alloc_back: u32,
    ctor: Callee,
    ctor_back: u32,
}

pub(crate) const OC664_ROOT_ARMS: [Oc664RootArm; 6] = [
    Oc664RootArm {
        kind: 1,
        next: 2,
        entry: 3,
        size: 0xf0,
        name: 0x1730,
        alloc_back: 0x0035_376c,
        ctor: MB_SPLINE_CTOR,
        ctor_back: 0x0035_3778,
    },
    Oc664RootArm {
        kind: 2,
        next: 4,
        entry: 2,
        size: 0x58,
        name: 0x1730,
        alloc_back: 0x0035_37a0,
        ctor: MB_MULTI_SPLINE_CTOR,
        ctor_back: 0x0035_37ac,
    },
    Oc664RootArm {
        kind: 4,
        next: 3,
        entry: 2,
        size: 0xb0,
        name: 0x1720,
        alloc_back: 0x0035_37d4,
        ctor: MB_PARENT_CTOR,
        ctor_back: 0x0035_37e0,
    },
    Oc664RootArm {
        kind: 3,
        next: 5,
        entry: 2,
        size: 0x90,
        name: 0x1710,
        alloc_back: 0x0035_3808,
        ctor: MB_POSITION_CTOR,
        ctor_back: 0x0035_3814,
    },
    Oc664RootArm {
        kind: 5,
        next: 6,
        entry: 2,
        size: 0x2d0,
        name: 0x16f8,
        alloc_back: 0x0035_383c,
        ctor: MB_ROLLER_CTOR,
        ctor_back: 0x0035_3848,
    },
    Oc664RootArm {
        kind: 6,
        next: 7,
        entry: 2,
        size: 0xb0,
        name: 0x16e8,
        alloc_back: 0x0035_3870,
        ctor: MB_MAGNET_CTOR,
        ctor_back: 0x0035_387c,
    },
];

/// `cMemMan_alloc(size, str, 0x20000000, 0)` where the name string sits at
/// `0x490000 - name_off`; returns to `back` with v0 = the block.
pub(crate) fn oc664_alloc(c: &mut Cpu, size: u64, name_off: u32, back: u32) -> Flow {
    c.g[A0][0] = size;
    c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(name_off));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, OC664_ALLOC, back)
}

/// `stream->read(a1, a2)` through the stream's vtable entry 2 (`{short delta;
/// fn}` at +0x10). The vtable pointer goes to register `vt`, the method to
/// `method`, and a0 = stream + delta; the caller sets a1 and a2 first. Which
/// registers hold them differs per call site in the generated code.
pub(crate) fn oc664_stream_read(
    c: &mut Cpu,
    stream: u32,
    vt: usize,
    method: usize,
    back: u32,
) -> Flow {
    c.g[vt][0] = Addr(stream).lw(c);
    let table = c.g[vt][0] as u32;
    c.g[A0][0] = Addr(table.wrapping_add(0x10)).lh(c);
    c.g[method][0] = Addr(table.wrapping_add(0x14)).lw(c);
    let target = c.g[method][0] as u32;
    c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, back)
}

/// One arm of the root-transform chain (`'b_353780` ... `'b_353884`). Returns
/// true when the type matched (the arm built its modifier).
pub(crate) fn oc664_root_arm(c: &mut Cpu, arm: Oc664RootArm) -> Result<bool, Unwound> {
    c.cycles += arm.entry;
    c.g[V0][0] = arm.kind;
    let mismatch = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = arm.next;
    if mismatch {
        return Ok(false);
    }
    c.cycles += 11;
    oc664_alloc(c, arm.size, arm.name, arm.alloc_back)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    arm.ctor.call(c, arm.ctor_back)?;
    let made = c.g[V0][0];
    Addr(c.g[S2][0] as u32).sw(c, made);
    Ok(true)
}

/// The root transform by `hdr.type` (V1), for `flags & 1`. Returns true when
/// the body goes on to its tail: a match, or type 7 is not taken and the tail
/// is skipped (false) when no arm matched.
pub(crate) fn oc664_root_transform(c: &mut Cpu) -> Result<bool, Unwound> {
    for arm in OC664_ROOT_ARMS {
        if oc664_root_arm(c, arm)? {
            return Ok(true);
        }
    }
    // Type 7 (`'b_353884`'s successor): the flags word is reloaded before the
    // branch, so the constructor call sees it in v0.
    let hdr = sp(c);
    c.cycles += 2;
    let mismatch = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = Addr(hdr).lw(c);
    if mismatch {
        return Ok(false);
    }
    c.cycles += 10;
    oc664_alloc(c, 0x90, 0x16d8, 0x0035_38a4)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    MB_AVA_SPLINE_CTOR.call(c, 0x0035_38b0)?;
    let made = c.g[V0][0];
    Addr(c.g[S2][0] as u32).sw(c, made);
    Ok(true)
}

/// One numA effect (`'b_353a1c`): the effect type is in v0 (a byte address
/// on entry); a1 leaves with the effect (or 0) for the link.
pub(crate) fn oc664_effect(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[V1][0] = 1;
    c.g[V0][0] = Addr(c.g[V0][0] as u32).lb(c);
    let mismatch = c.g[V0][0] != c.g[V1][0];
    c.g[A0][0] = 0x1f0;
    if !mismatch {
        c.cycles += 9;
        oc664_alloc(c, 0x1f0, 0x1688, 0x0035_39a8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        MB_PARTICLE_CTOR.call(c, 0x0035_39b4)?;
        c.g[A1][0] = c.g[V0][0];
        return Ok(());
    }
    c.cycles += 2;
    let mismatch = c.g[V0][0] != c.g[S7][0];
    c.g[A0][0] = 0x270;
    if !mismatch {
        c.cycles += 9;
        oc664_alloc(c, 0x270, 0x1670, 0x0035_39d4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        MB_DYNAMIC_PARTICLE_CTOR.call(c, 0x0035_39e0)?;
        c.g[A1][0] = c.g[V0][0];
        return Ok(());
    }
    c.cycles += 2;
    let mismatch = c.g[V0][0] != c.g[S5][0];
    c.g[V0][0] = 0;
    if !mismatch {
        c.cycles += 10;
        c.g[A0][0] = 0x40;
        oc664_alloc(c, 0x40, 0x1658, 0x0035_3a04)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        MB_HALO_CTOR.call(c, 0x0035_3a10)?;
        c.g[A1][0] = c.g[V0][0];
        return Ok(());
    }
    c.cycles += 1;
    c.g[A1][0] = c.g[V0][0];
    Ok(())
}

/// `void ModifierBlock_Load(ModifierBlock* self, Stream* s)` (0x3536d8): reads
/// the 52-byte modifier header and builds the typed objects it lists: the root
/// transform (spline, multi-spline, parent, position, roller, magnet or
/// ava-spline by `type`), the UV scroll, tex flip and boost modifiers by
/// `flags`, `numA` effects (particle, dynamic particle, halo) pushed on the
/// effect list and `numB` rail modifiers pushed on the rail list. Each object
/// is a named pool allocation (`EA_MemAllocNamed`) plus its constructor.
pub fn modifier_block_load_3(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_3538b8: {
            'b_3538b4: {
                c.cycles += 28;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
                c.g[A2][0] = 0x34;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[S1][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[A1][0] = c.g[SP][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                c.sw(c.g[S2][0] as u32, 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00353738);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00353738)?;
                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                if t {
                    break 'b_3538b4;
                } else {
                    'b_353780: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_353780;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = 0x490000;
                            c.g[A0][0] = 0xf0;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1730));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x0035376c);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x0035376c)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00353778);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x003595d8,
                                f: ee::s11::SplineModifier_ctor_2,
                            }
                            .call(c, 0x00353778)?;
                            let t = true;
                            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_3538b4;
                            } else {
                                break 'b_353780;
                            }
                        }
                    }
                    'b_3537b4: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x4;
                        if t {
                            break 'b_3537b4;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = 0x490000;
                            c.g[A0][0] = 0x58;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1730));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x003537a0);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x003537a0)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x003537ac);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x0035a118,
                                f: ee::s11::MultiSplineModifier_ctor_2,
                            }
                            .call(c, 0x003537ac)?;
                            let t = true;
                            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_3538b4;
                            } else {
                                break 'b_3537b4;
                            }
                        }
                    }
                    'b_3537e8: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x3;
                        if t {
                            break 'b_3537e8;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = 0x490000;
                            c.g[A0][0] = 0xb0;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1720));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x003537d4);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x003537d4)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x003537e0);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00357090,
                                f: ee::s11::ParentModifier_ctor_2,
                            }
                            .call(c, 0x003537e0)?;
                            let t = true;
                            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_3538b4;
                            } else {
                                break 'b_3537e8;
                            }
                        }
                    }
                    'b_35381c: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x5;
                        if t {
                            break 'b_35381c;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = 0x490000;
                            c.g[A0][0] = 0x90;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1710));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x00353808);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x00353808)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00353814);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00356f68,
                                f: ee::s11::PositionModifier_ctor_3,
                            }
                            .call(c, 0x00353814)?;
                            let t = true;
                            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_3538b4;
                            } else {
                                break 'b_35381c;
                            }
                        }
                    }
                    'b_353850: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x6;
                        if t {
                            break 'b_353850;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = 0x490000;
                            c.g[A0][0] = 0x2d0;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16f8));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x0035383c);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x0035383c)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00353848);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x0035df70,
                                f: ee::s11::RollerModifier_ctor_2,
                            }
                            .call(c, 0x00353848)?;
                            let t = true;
                            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_3538b4;
                            } else {
                                break 'b_353850;
                            }
                        }
                    }
                    'b_353884: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x7;
                        if t {
                            break 'b_353884;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = 0x490000;
                            c.g[A0][0] = 0xb0;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16e8));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x00353870);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x00353870)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0035387c);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00357358,
                                f: ee::s11::MagnetModifier_ctor_2,
                            }
                            .call(c, 0x0035387c)?;
                            let t = true;
                            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_3538b4;
                            } else {
                                break 'b_353884;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        break 'b_3538b8;
                    } else {
                        c.cycles += 10;
                        c.g[A1][0] = 0x490000;
                        c.g[A0][0] = 0x90;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16d8));
                        c.g[A2][0] = 0x20000000;
                        c.g[RA][0] = sx(0x003538a4);
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x003538a4)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x003538b0);
                        c.g[A1][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00357798,
                            f: ee::s11::AvaSplineModifier_ctor_2,
                        }
                        .call(c, 0x003538b0)?;
                        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                        break 'b_3538b4;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            break 'b_3538b8;
        }
        'b_3538e8: {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] & 0x2;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0x50;
            if t {
                break 'b_3538e8;
            } else {
                c.cycles += 9;
                c.g[A1][0] = 0x490000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16c0));
                c.g[A2][0] = 0x20000000;
                c.g[RA][0] = sx(0x003538d8);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x003538d8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x003538e4);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0035f788, 0x003538e4)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                break 'b_3538e8;
            }
        }
        'b_35391c: {
            c.cycles += 4;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.g[V0][0] = c.g[V0][0] & 0x4;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0xe8;
            if t {
                break 'b_35391c;
            } else {
                c.cycles += 9;
                c.g[A1][0] = 0x490000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16a8));
                c.g[A2][0] = 0x20000000;
                c.g[RA][0] = sx(0x0035390c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0035390c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00353918);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0035f1a8, 0x00353918)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                break 'b_35391c;
            }
        }
        'b_353964: {
            'b_353960: {
                c.cycles += 4;
                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                c.g[V0][0] = c.g[V0][0] & 0x8;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                if t {
                    break 'b_353960;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    if t {
                        break 'b_353964;
                    } else {
                        c.cycles += 10;
                        c.g[A1][0] = 0x490000;
                        c.g[A0][0] = 0x50;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1698));
                        c.g[A2][0] = 0x20000000;
                        c.g[RA][0] = sx(0x00353950);
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00353950)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x0035395c);
                        c.g[A1][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00342808,
                            f: ee::s11::BoostModifier_ctor_2,
                        }
                        .call(c, 0x0035395c)?;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                        break 'b_353960;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
            break 'b_353964;
        }
        'b_353a38: {
            c.cycles += 2;
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_353a38;
            } else {
                c.cycles += 7;
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
                c.g[FP][0] = 0x490000;
                c.g[S7][0] = 0x2;
                c.g[S6][0] = 0x490000;
                c.g[S5][0] = 0x3;
                c.g[S4][0] = 0x490000;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
                'l_353988: loop {
                    'b_353a1c: {
                        'b_3539bc: {
                            c.cycles += 4;
                            c.g[V1][0] = 0x1;
                            c.g[V0][0] = c.lb(c.g[V0][0] as u32);
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[A0][0] = 0x1f0;
                            if t {
                                break 'b_3539bc;
                            } else {
                                c.cycles += 9;
                                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x1688));
                                c.g[A2][0] = 0x20000000;
                                c.g[RA][0] = sx(0x003539a8);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x003539a8)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x003539b4);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x003459a8,
                                    f: ee::s11::ParticleModifier_ctor_2,
                                }
                                .call(c, 0x003539b4)?;
                                let t = true;
                                c.g[A1][0] = c.g[V0][0];
                                if t {
                                    break 'b_353a1c;
                                } else {
                                    break 'b_3539bc;
                                }
                            }
                        }
                        'b_3539e8: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != c.g[S7][0];
                            c.g[A0][0] = 0x270;
                            if t {
                                break 'b_3539e8;
                            } else {
                                c.cycles += 9;
                                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1670));
                                c.g[A2][0] = 0x20000000;
                                c.g[RA][0] = sx(0x003539d4);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x003539d4)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x003539e0);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x00345d80,
                                    f: ee::s11::DynamicParticleModifier_ctor_2,
                                }
                                .call(c, 0x003539e0)?;
                                let t = true;
                                c.g[A1][0] = c.g[V0][0];
                                if t {
                                    break 'b_353a1c;
                                } else {
                                    break 'b_3539e8;
                                }
                            }
                        }
                        'b_353a18: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != c.g[S5][0];
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_353a18;
                            } else {
                                c.cycles += 10;
                                c.g[A0][0] = 0x40;
                                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1658));
                                c.g[A2][0] = 0x20000000;
                                c.g[RA][0] = sx(0x00353a04);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x00353a04)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x00353a10);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x003461c0,
                                    f: ee::s11::HaloModifier_ctor_2,
                                }
                                .call(c, 0x00353a10)?;
                                let t = true;
                                c.g[A1][0] = c.g[V0][0];
                                if t {
                                    break 'b_353a1c;
                                } else {
                                    break 'b_353a18;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A1][0] = c.g[V0][0];
                        break 'b_353a1c;
                    }
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00353a24);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                    Callee {
                        addr: 0x00345720,
                        f: ee::s11::ModifierList_PushBack,
                    }
                    .call(c, 0x00353a24)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00353988);
                        }
                        continue 'l_353988;
                    } else {
                        break 'b_353a38;
                    }
                }
            }
        }
        'b_353a8c: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_353a8c;
            } else {
                c.cycles += 3;
                c.g[S3][0] = 0x490000;
                c.g[A2][0] = 0x20000000;
                'l_353a50: loop {
                    c.cycles += 15;
                    c.g[A3][0] = 0u64;
                    c.g[A0][0] = 0x90;
                    c.g[RA][0] = sx(0x00353a60);
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1648));
                    call_ptr(c, 0x00317d70, 0x00353a60)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00353a70);
                    c.g[A1][0] = c.g[S1][0];
                    Callee {
                        addr: 0x0035ba88,
                        f: ee::s11::RailModifier_ctor_2,
                    }
                    .call(c, 0x00353a70)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00353a7c);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1c));
                    Callee {
                        addr: 0x0035b690,
                        f: ee::s11::RailModifier_ListPushBack,
                    }
                    .call(c, 0x00353a7c)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = 0x20000000;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00353a50);
                        }
                        continue 'l_353a50;
                    } else {
                        break 'b_353a8c;
                    }
                }
            }
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::events::check_events;

    use crate::difftest::events::schedule;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::uf;
    pub(crate) use crate::game::bx::bxstring::tests::heap_init;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::bx::memblockman::tests::fake_slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_4;

    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::movie::movieplayer::tests::ret_slot;

    pub(crate) use crate::game::movie::movieplayer::tests::RET_ZERO;

    pub(crate) use crate::game::object::animnode::tests::heap_block;
    pub(crate) use crate::game::render::particle::tests::slot;
    pub(crate) use crate::game::render::particle::tests::vt;
    pub(crate) use crate::game::render::particle::tests::SEEDS;
    pub(crate) use crate::game::worldpainter::quadtree::tests::put_vtable;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_3;
    use ssx3_ee as ee;

    #[test]
    fn world_get_current_view_index_matches() {
        check(
            ssx3_ee::s09::f_002d1c58__generated,
            world_get_current_view_index,
            300,
            &|c, seed| {
                let mut r = Rng(0x1234_5678 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let base = GAME_GP.wrapping_sub(0x848);
                let p1 = DATA + 0x400;
                let p2 = DATA + 0x500;
                let p3 = DATA + 0x600;
                c.bus.write32(base, p1);
                c.bus.write32(p1 + 0x84, p2);
                c.bus.write32(p2 + 0x84, p3);
                c.bus.write32(p3 + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn world_get_camera_eye_matches() {
        check(
            ssx3_ee::s09::f_002d1c20__generated,
            world_get_camera_eye,
            200,
            &|c, seed| {
                let mut r = Rng(0x8888 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = DATA + 0x100;
                let viewer = DATA + 0x200;
                let table = DATA + 0x400;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, viewer);
                c.bus.write32(viewer + 0x84, table);
                for k in 0..16u32 {
                    c.bus.write32(table + 4 * k, r.next_u32());
                }
                c.g[A0][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    #[test]
    fn modifier_block_ctor_matches() {
        check(
            ssx3_ee::s11::f_00352aa8,
            modifier_block_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x101c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..8u32 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn sky_box_ctor_matches() {
        check(ssx3_ee::s11::f_00353ac0, sky_box_ctor, 200, &|c, seed| {
            let mut r = Rng(0x101d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..8u32 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn flex_bridge_springs_ctor_matches() {
        check(
            ssx3_ee::s11::f_00353da8,
            flex_bridge_springs_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x101e ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x24, r.next_u32());
            },
        );
    }

    /// A generated function that returns 0 (a `bool`-returning callee).
    pub(crate) const RETURNS_ZERO: u32 = 0x0032_e688;

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
    fn modifier_block_get_transform_radius_matches() {
        covered(
            ssx3_ee::s11::f_00352bf8__generated,
            modifier_block_get_transform_radius,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 71, 1);
                let mb = DATA + 0x100;
                if seed % 2 == 0 {
                    c.bus.write32(mb, 0);
                } else {
                    let node = DATA + 0x400;
                    let vt = vtable(c, &mut r, &[0x70]);
                    c.bus.write32(node, vt);
                    c.bus.write32(mb, node);
                }
                c.g[A0][0] = sx(mb);
            },
        );
    }

    #[test]
    fn modifier_block_has_rail_modifier_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x353418, 0);
            c.g[A0][0] = DATA as u64;
            if seed % 5 == 0 {
                c.bus.write32(DATA + 0x1c, 0);
                return;
            }
            let n0 = DATA + 0x100;
            let n1 = DATA + 0x140;
            c.bus.write32(DATA + 0x1c, n0);
            c.bus.write32(n0, n1);
            c.bus.write32(n1, 0);
            c.bus.write32(n0 + 0x30, 0x21);
            c.bus.write32(n1 + 0x30, 0x22);
            c.g[A1][0] = (0x20 + seed % 4) as u64;
        };
        check(
            ee::s11::f_00353418__generated,
            modifier_block_has_rail_modifier,
            SEEDS_2,
            &setup,
        );
    }

    #[test]
    fn modifier_block_forward_v42_matches() {
        check(
            ee::s11::f_00353098__generated,
            modifier_block_forward_v42,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3530_98, 0);
                let block = DATA + 0x100;
                let node = DATA + 0x300;
                let vt = put_vtable(c, &mut r, DATA + 0x800, &[0xb0]);
                c.bus.write32(block, if seed % 4 == 0 { 0 } else { node });
                c.bus.write32(node, vt);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = sx(r.next_u32());
                fill(c, &mut r, node, 8);
            },
        );
    }

    #[test]
    fn modifier_block_reset_particles_matches() {
        check(
            ee::s11::f_00353228__generated,
            modifier_block_reset_particles,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3532_28, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                let nodes = (seed % 3) as u32;
                let mut next = 0u32;
                for i in (0..nodes).rev() {
                    let node = DATA + 0x800 + 0x100 * i;
                    clear(c, node, 0x80);
                    c.bus.write32(node, next);
                    let vt = vtable(c, &mut r, &[0x18]);
                    c.bus.write32(node + 8, vt);
                    next = node;
                }
                c.bus.write32(this + 0x10, next);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn flex_bridge_springs_dtor_matches() {
        check(
            ee::s11::f_00353db8__generated,
            flex_bridge_springs_dtor,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x353d_b8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x60);
                let owned = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                c.bus.write32(this + 0x24, owned);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }

    #[test]
    fn modifier_block_forward_v43_matches() {
        check(
            ee::s11::f_00353020__generated,
            modifier_block_forward_v43,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x353020, 4);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = (DATA + 0x900) as u64;
                if seed % 2 == 0 {
                    w(c, DATA, 0);
                } else {
                    let child = DATA + 0x400;
                    w(c, DATA, child);
                    fake_slot(c, child, DATA + 0x2000, 0xc8);
                }
            },
        );
    }

    #[test]
    fn flex_bridge_springs_init_matches() {
        check(
            ee::s11::f_00353e08__generated,
            flex_bridge_springs_init,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x353e08, 1);
                be_library(c);
                heap_init(c);
                let desc = DATA + 0x800;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = desc as u64;
                c.f[12] = r.edge_f32();
                for i in 0..7u32 {
                    w(c, desc + 4 * i, r.edge_f32());
                }
                w(c, desc + 0x1c, (2 + seed % 4) as u32);
            },
        );
    }

    #[test]
    fn flex_bridge_springs_load_matches() {
        check(
            ee::s11::f_00353e80__generated,
            flex_bridge_springs_load,
            8,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x353e_80, 6);
                let self_ = DATA;
                let stream = DATA + 0x1000;
                let vtable = DATA + 0x1200;
                w(c, self_, 1 + r.next_u32() % 4);
                w(c, self_ + 0x24, DATA + 0x2000);
                w(c, stream, vtable);
                ret_slot(c, vtable, 0x10, RET_ZERO);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = stream as u64;
            },
        );
    }

    #[test]
    fn modifier_block_remove_dead_particles_matches() {
        check(
            ssx3_ee::s11::f_00353278__generated,
            modifier_block_remove_dead_particles,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x353278, 6);
                be_library(c);
                world(c);
                let blk = DATA + 0x100;
                c.g[A0][0] = blk as u64;
                c.g[A1][0] = r.next_u32() as u64;
                if seed % 4 == 0 {
                    w(c, blk + 0x10, 0);
                } else {
                    let n1 = DATA + 0x300;
                    let n2 = DATA + 0x380;
                    let vt1 = DATA + 0x500;
                    let vt2 = DATA + 0x600;
                    w(c, blk + 0x10, n1);
                    w(c, n1, n2);
                    w(c, n1 + 8, vt1);
                    w(c, n2, 0);
                    w(c, n2 + 8, vt2);
                    slot_3(c, vt1, 0x10, 0, FAKE);
                    slot_3(c, vt1, 0x14, 0, FAKE + 0x10);
                    slot_3(c, vt2, 0x10, 0, FAKE + 0x20);
                    slot_3(c, vt2, 0x14, 0, FAKE + 0x30);
                    // The state getter returns 3 (dead) on some seeds.
                    if seed % 2 == 0 {
                        w(c, 0x0090_0010, 3);
                    }
                }
            },
        );
    }

    #[test]
    fn modifier_block_handle_event_matches() {
        diff(
            ee::s11::f_00352f40__generated,
            modifier_block_handle_event,
            30,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x35_2f40);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.f[12] = uf(&mut r, 10.0);
                w(c, DATA, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
                w(c, DATA + 0x1000, 0); // the handler's vtable is null
                w(c, DATA + 4, if seed % 3 == 0 { 0 } else { DATA + 0x1100 });
                w(c, DATA + 8, if seed % 3 == 0 { DATA + 0x1200 } else { 0 });
                w(c, DATA + 0xc, if seed % 5 == 0 { DATA + 0x1300 } else { 0 });
                w(c, DATA + 0x1100, r.next_u32());
            },
        );
    }

    /// Every modifier present or absent, id and extra lists of 0-4 nodes;
    /// all virtual calls go to fakes (some unwind), the two direct callees
    /// (UV-scroll and rail save) run for real into the fake stream.
    #[test]
    fn modifier_block_save_matches() {
        covered(
            ee::s11::f_00353448__generated,
            modifier_block_save,
            300,
            &|c, seed| {
                c.pc = 0x0035_3448;
                let mut r = machine(c, seed, 0x353448, 14);
                let block = DATA;
                let stream = DATA + 0x800;
                let vt = |i: u32| DATA + 0x2000 + 0x400 * i;
                let mut fake = 0;
                let mut fslot = |c: &mut Cpu, r: &mut Rng, vt: u32, off: u32| {
                    crate::game::main::gamemode::tests::slot(
                        c,
                        vt,
                        off,
                        (r.next_u32() as i16) & !3,
                        FAKE + 0x10 * fake,
                    );
                    fake += 1;
                };
                // Stream: Write (0x8) and the rail modifier's 0x28.
                c.bus.write32(stream, vt(0));
                fslot(c, &mut r, vt(0), 0x8);
                fslot(c, &mut r, vt(0), 0x28);
                let bits = r.next_u32();
                let present = |i: u32| seed % 11 == 0 || (bits >> i) & 1 != 0;
                // Primary (vtable at +0: id 0xd0, save 0xd8).
                let primary = if present(0) { DATA + 0x100 } else { 0 };
                c.bus.write32(DATA + 0x100, vt(1));
                fslot(c, &mut r, vt(1), 0xd0);
                fslot(c, &mut r, vt(1), 0xd8);
                // UV scroll / rail (direct callees; the rail one reads +0x18).
                let uv = if present(1) { DATA + 0x300 } else { 0 };
                let rail = if present(2) { DATA + 0x380 } else { 0 };
                c.bus.write32(DATA + 0x380 + 0x18, r.next_u32());
                // Secondary (vtable at +4: id 0x28, save 0x30).
                let secondary = if present(3) { DATA + 0x200 } else { 0 };
                c.bus.write32(DATA + 0x204, vt(2));
                fslot(c, &mut r, vt(2), 0x28);
                fslot(c, &mut r, vt(2), 0x30);
                // Lists: next at +0, vtable at +8.
                fslot(c, &mut r, vt(3), 0x10);
                fslot(c, &mut r, vt(3), 0x20);
                fslot(c, &mut r, vt(4), 0x10);
                let list = |c: &mut Cpu, base: u32, n: u32, vtab: u32| -> u32 {
                    for i in 0..n {
                        let node = base + 0x20 * i;
                        c.bus.write32(node, if i + 1 < n { node + 0x20 } else { 0 });
                        c.bus.write32(node + 8, vtab);
                    }
                    if n == 0 {
                        0
                    } else {
                        base
                    }
                };
                let ids = list(c, DATA + 0x400, (seed / 3) % 5, vt(3));
                let extra = list(c, DATA + 0x600, (seed / 7) % 5, vt(4));
                put_words(c, block, &[primary, uv, rail, secondary, ids]);
                c.bus.write32(block + MB_EXTRA_LIST, extra);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    // ------------------------------------------------------------ SkyBox_Render

    /// SkyBox_Render: sky drawing disabled, no model, or drawn (the draw
    /// method a fake that may unwind); random render-state records and
    /// layers, view index 0..3 with its camera eye, a random view matrix.
    #[test]
    fn skybox_render_matches() {
        use crate::difftest::fakes::{covered, vtable};
        use skybox::*;
        const SKY: u32 = 0x00a0_0000;
        const REND: u32 = 0x00a0_1000;
        const RS: u32 = 0x00a0_3000;
        const MODEL_AT: u32 = 0x00a0_4000;
        const APP: u32 = 0x00a0_5000;
        const GM: u32 = 0x00a0_5100;
        const WX: u32 = 0x00a0_5200;
        const CAMS: u32 = 0x00a0_6000;
        covered(
            ee::s11::f_00353b10__generated,
            skybox_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_3b10, 1);
                c.pc = 0x0035_3b10;
                c.bus.write32(GAME_GP + GP_SKY_OFF, (seed % 9 == 4) as u32);
                c.bus.write32(GAME_GP + GP_RENDERER, REND);
                let top = RS + RS_SIZE * (r.next_u32() % 4);
                c.bus.write32(REND + RS_TOP, top);
                fill(c, &mut r, RS, 0x40);
                let vt = vtable(c, &mut r, &[SLOT_DRAW_MODEL]);
                c.bus.write32(REND + RENDERER_VT, vt);
                c.bus
                    .write32(SKY + MODEL, if seed % 9 == 7 { 0 } else { MODEL_AT });
                fill(c, &mut r, MODEL_AT, 0x40);
                c.bus.write32(MODEL_AT + 0x80, MODEL_AT + 0x40);
                for i in 0..16 {
                    c.bus.write32(VIEW_MATRIX + 4 * i, r.f32(100.0));
                }
                c.bus.write32(GAME_GP - 0x848, APP);
                c.bus.write32(APP + 0x84, GM);
                c.bus.write32(GM + 0x84, WX);
                c.bus.write32(WX + 0x14, r.next_u32() % 4);
                for v in 0..4 {
                    c.bus.write32(WX + 4 + 4 * v, CAMS + 0x100 * v);
                    fill(c, &mut r, CAMS + 0x100 * v + 0x20, 4);
                }
                c.g[A0][0] = SKY as u64;
            },
        );
    }

    /// ModifierBlock_SetTransformBounds: no node, or a node whose vtable
    /// slot 13 is a fake method (random this-delta) that may unwind.
    #[test]
    fn modifier_block_set_transform_bounds_matches() {
        covered(
            ee::s11::f_00352bc0__generated,
            modifier_block_set_transform_bounds,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_2bc0, 1);
                c.pc = 0x0035_2bc0;
                let block = DATA + 0x2_0000;
                let node = if seed % 5 == 0 {
                    0
                } else {
                    let vt = crate::difftest::fakes::vtable(c, &mut r, &[0x68]);
                    let n = DATA + 0x3_0000 + (r.next_u32() & 0xff0);
                    c.bus.write32(n, vt);
                    n
                };
                c.bus.write32(block, node);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = sx(DATA + 0x4_0000);
            },
        );
    }

    use crate::difftest::RA as SENTINEL;

    /// ModifierBlock_dtor: transform / boost / particle modifiers present
    /// or not (their release methods are fakes; some unwind), no UV-scroll,
    /// texture-flip or rail modifiers, the modifier list linked to a node or
    /// empty; delete flag on (real operator delete over the test heap) or off.
    #[test]
    fn modifier_block_dtor_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::DATA;
        use crate::game::be::belibrary::tests::alloc_ready;
        const THIS: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x800;
        const TRANSFORM: u32 = DATA + 0x900;
        const BOOST: u32 = DATA + 0xa00;
        const NODE: u32 = DATA + 0xb00;
        check(
            ee::s11::f_00352ae8__generated,
            modifier_block_dtor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_2ae8, 3);
                alloc_ready(c);
                c.pc = 0x0035_2ae8;
                for k in 0..0x40u32 {
                    c.bus.write32(THIS - 0x10 + 4 * k, 0);
                }
                c.bus.write16(VT + 0x8, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(VT + 0xc, FAKE);
                c.bus.write32(TRANSFORM, VT);
                c.bus.write32(BOOST + 4, VT);
                c.bus.write32(NODE, 0);
                c.bus.write32(NODE + 4, 0);
                c.bus.write32(NODE + 8, VT);
                let present = |bit: u32, p: u32| if (seed >> bit) & 1 != 0 { p } else { 0 };
                c.bus.write32(THIS, present(1, TRANSFORM));
                c.bus.write32(THIS + 0xc, present(2, BOOST));
                c.bus.write32(THIS + MB_MOD_LIST, present(3, NODE));
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = ((seed & 1) | (r.next_u32() & 6)) as u64;
            },
        );
    }

    /// A modifier block whose rail list holds 0..5 links (each with a
    /// fake `Release`), asked to free one id (present or not) or all (-1).
    pub(crate) fn mod_block_free_rail_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::{machine, FAKE};
        let n = seed / 3 % 6;
        let mut r = machine(c, seed, 0x3533_0000, n.max(1));
        let block = DATA;
        let links = DATA + 0x1000;
        let mut ids = Vec::new();
        for i in 0..n {
            let link = links + i * 0x100;
            let next = if i + 1 < n { link + 0x100 } else { 0 };
            let vt = DATA + 0x8000 + i * 0x40;
            c.bus.write32(link + MOD_BLOCK_LINK_NEXT, next);
            c.bus.write32(link + MOD_BLOCK_LINK_VT, vt);
            let id = r.next_u32() & 0x7fff_ffff;
            ids.push(id);
            c.bus.write32(link + MOD_BLOCK_LINK_ID, id);
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + MOD_BLOCK_RELEASE, delta as u16);
            c.bus.write32(vt + MOD_BLOCK_RELEASE + 4, FAKE + 0x10 * i);
        }
        c.bus
            .write32(block + MOD_BLOCK_RAIL_HEAD, if n > 0 { links } else { 0 });
        // Word 0 (the free-all walk reads `next` of a null link there).
        c.bus.write32(0, r.next_u32());
        c.g[A1][0] = match seed % 3 {
            0 => u64::MAX,
            1 if n > 0 => ids[(r.next_u32() % n) as usize] as u64,
            _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        };
        c.g[A0][0] = sx(block);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0035_3300;
    }

    /// `ModifierBlock_FreeRailModifiers`: empty list, one id found or not,
    /// free all; fakes unwinding.
    #[test]
    fn modifier_block_free_rail_modifiers_matches() {
        check(
            ee::s11::f_00353300__generated,
            modifier_block_free_rail_modifiers,
            300,
            &mod_block_free_rail_fixture,
        );
    }

    /// Event delivery at both walk heads.
    #[test]
    fn modifier_block_free_rail_modifiers_events_match() {
        check_events(
            ee::s11::f_00353300__generated,
            modifier_block_free_rail_modifiers,
            150,
            &|c, seed| {
                // Free all (seed even) or an absent id over five links.
                let base = 15 + if seed % 2 == 0 { 0 } else { 2 };
                mod_block_free_rail_fixture(c, base);
                c.bus
                    .write32(crate::difftest::fakes::LOG_UNWIND_AT, u32::MAX);
                let mut r = Rng(0x3533_0001 ^ seed as u64);
                schedule(c, &mut r, seed, 0x0035_3300)
            },
        );
    }

    /// `ModifierBlock_TakeParticleList`: an empty list, or 1-3 particle
    /// nodes with vtable slot +0x18 methods (fakes); the block's own list
    /// starts empty or holds an entry.
    #[test]
    fn modifier_block_take_particle_list_matches() {
        use crate::difftest::fakes::covered;
        use crate::difftest::fakes::machine;
        use crate::difftest::fakes::vtable;
        const BLOCK: u32 = DATA + 0x100;
        const LIST: u32 = DATA + 0x200;
        const BLOCK_LIST: u32 = DATA + 0x300;
        const NODES: u32 = DATA + 0x1000;
        covered(
            ee::s11::f_00353398__generated,
            modifier_block_take_particle_list,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x353398, 4);
                c.pc = 0x0035_3398;
                c.bus.write32(BLOCK + 0x10, BLOCK_LIST);
                let n = if seed % 5 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 3
                };
                let mut prev = 0u32;
                for k in 0..n {
                    let node = NODES + 0x100 * k;
                    if k == 0 {
                        c.bus.write32(LIST, node);
                    } else {
                        c.bus.write32(prev, node);
                    }
                    c.bus.write32(node, 0);
                    let vt = vtable(c, &mut r, &[0x18]);
                    c.bus.write32(node + 8, vt);
                    prev = node;
                }
                c.bus.write32(LIST + 4, if n == 0 { 0 } else { NODES });
                c.g[A0][0] = sx(BLOCK);
                c.g[A1][0] = sx(LIST);
            },
        );
    }

    // ------------------------- ModifierBlock_SetTransformRadius

    #[test]
    fn modifier_block_set_transform_radius_matches() {
        covered(
            ee::s11::f_00352c38__generated,
            modifier_block_set_transform_radius,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x352c, 1);
                c.pc = 0x0035_2c38;
                let obj = DATA;
                let holder = DATA + 0x100;
                let vt = DATA + 0x200;
                let wired = seed % 4 != 0;
                c.bus.write32(obj, if wired { holder } else { 0 });
                c.bus.write32(holder, vt);
                c.bus
                    .write16(vt + 0x78, ((r.next_u32() as i16) & !3) as u16);
                c.bus.write32(vt + 0x7c, FAKE);
                c.g[A0][0] = sx(obj);
            },
        );
    }

    /// `ModifierBlock_ClearTransformFinished`: no transform modifier, or one
    /// whose method (a fake through a fake vtable) is called on the
    /// adjusted `this`.
    #[test]
    fn modifier_block_clear_transform_finished_matches() {
        covered(
            ee::s11::f_00352f08__generated,
            modifier_block_clear_transform_finished,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_2f08, 1);
                c.pc = 0x0035_2f08;
                let block = DATA + 0x5_0000;
                let modifier = DATA + 0x5_1000;
                let vt = DATA + 0x5_2000;
                let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                c.bus
                    .write16(vt + MODIFIER_BLOCK_CLEAR_FINISHED_SLOT, delta as u16);
                c.bus
                    .write32(vt + MODIFIER_BLOCK_CLEAR_FINISHED_SLOT + 4, FAKE);
                c.bus.write32(modifier, vt);
                c.bus
                    .write32(block, if seed % 4 == 0 { 0 } else { modifier });
                c.g[A0][0] = sx(block);
            },
        );
    }

    pub(crate) use crate::game::object::instanceman::tests::gate_on;
    pub(crate) use crate::game::object::instanceman::tests::SEEDS as SEEDS_7;

    pub(crate) use crate::game::object::instanceman::tests::install_fakes;
    pub(crate) use crate::game::object::instanceman::tests::LEAF;

    #[test]
    fn modifier_block_update_transform_matches() {
        check(
            ee::s11::f_00352e98__generated,
            modifier_block_update_transform,
            SEEDS_7,
            &|c, seed| {
                let mut r = Rng(0x587_2e98 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0035_2e98;
                c.g[A0][0] = sx(DATA + 0x100);
                c.bus
                    .write32(DATA + 0x100, if seed % 3 == 0 { 0 } else { DATA + 0x200 });
                c.bus.write32(DATA + 0x200, DATA + 0x300);
                c.bus.write16(DATA + 0x318, ((seed % 4) * 0x10) as u16);
                c.bus.write32(DATA + 0x31c, LEAF);
            },
        );
    }

    #[test]
    fn sky_box_dtor_matches() {
        install_fakes();
        check(
            ee::s11::f_00353ae8__generated,
            sky_box_dtor,
            SEEDS_7,
            &|c, seed| {
                let mut r = Rng(0x587_3ae8 ^ seed as u64);
                scramble(c, &mut r);
                gate_on(c);
                c.pc = 0x0035_3ae8;
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 2) * 0x40);
            },
        );
    }

    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::prev_cell;
    use std::ptr::addr_of_mut;

    /// Gate word for the callee fakes (a magic word in the machine's RAM).
    pub(crate) const OC664_CTRL: u32 = DATA + 0x3f00;

    pub(crate) const OC664_MAGIC: u32 = 0x6f63_3634;

    /// The objects under test.
    #[allow(dead_code)]
    pub(crate) const OC664_OBJ: u32 = DATA + 0x3000;

    #[allow(dead_code)]
    pub(crate) const OC664_NODE: u32 = DATA + 0x3a00;

    /// A vtable whose slot at +0x110 is a fake (the dead branch of the update).
    #[allow(dead_code)]
    pub(crate) const OC664_VT2: u32 = DATA + 0x3d00;

    /// A 52-byte modifier-block header at `at`: flags (the low nibble of the
    /// seed), transform type (0..8), kind (1 or 2), numA and numB, effect types.
    #[allow(dead_code)]
    pub(crate) fn oc664_header(c: &mut Cpu, at: u32, seed: u32, r: &mut Rng) {
        c.bus.write32(at, seed & 0xf);
        c.bus.write32(at + 4, (seed >> 4) % 9);
        c.bus.write32(at + 8, if seed % 5 == 0 { 2 } else { 1 });
        c.bus.write32(at + 0xc, r.next_u32() % 4);
        c.bus.write32(at + 0x10, r.next_u32() % 3);
        for i in 0..4 {
            c.bus.write8(at + 0x14 + i, (r.next_u32() % 4) as u8);
        }
    }

    /// Gated fakes for the callees that have an override slot: while the gate
    /// is set, a call is logged and returns; otherwise the body it replaced runs.
    macro_rules! oc664_callees {
        ($($fake:ident => $shard:ident :: $slot:ident, $gen:ident, $addr:expr;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC664_CTRL) != OC664_MAGIC {
                        return fall_back(c, prev_cell($fake), ee::$shard::$gen);
                    }
                    fake_call(c, $addr);
                }
            )*

            pub(crate) fn oc664_install_slots() {
                $(hook(addr_of_mut!(ee::$shard::$slot), $fake, prev_cell($fake));)*
            }
        };
    }

    oc664_callees! {
        oc664_fake_alloc => s10::OVERRIDE_00317D70, f_00317d70__generated, 0x0031_7d70;
        oc664_fake_spline => s11::OVERRIDE_003595D8, f_003595d8__generated, 0x0035_95d8;
        oc664_fake_multi_spline => s11::OVERRIDE_0035A118, f_0035a118__generated, 0x0035_a118;
        oc664_fake_parent => s11::OVERRIDE_00357090, f_00357090__generated, 0x0035_7090;
        oc664_fake_position => s11::OVERRIDE_00356F68, f_00356f68__generated, 0x0035_6f68;
        oc664_fake_magnet => s11::OVERRIDE_00357358, f_00357358__generated, 0x0035_7358;
        oc664_fake_ava => s11::OVERRIDE_00357798, f_00357798__generated, 0x0035_7798;
        oc664_fake_uv => s12::OVERRIDE_0035F788, f_0035f788__generated, 0x0035_f788;
        oc664_fake_tex_flip => s12::OVERRIDE_0035F1A8, f_0035f1a8__generated, 0x0035_f1a8;
        oc664_fake_boost => s11::OVERRIDE_00342808, f_00342808__generated, 0x0034_2808;
        oc664_fake_particle => s11::OVERRIDE_003459A8, f_003459a8__generated, 0x0034_59a8;
        oc664_fake_dyn_particle => s11::OVERRIDE_00345D80, f_00345d80__generated, 0x0034_5d80;
        oc664_fake_halo => s11::OVERRIDE_003461C0, f_003461c0__generated, 0x0034_61c0;
        oc664_fake_effect => s11::OVERRIDE_00345720, f_00345720__generated, 0x0034_5720;
        oc664_fake_rail_link => s11::OVERRIDE_0035B690, f_0035b690__generated, 0x0035_b690;
        oc664_fake_world => s11::OVERRIDE_0034FB90, f_0034fb90__generated, 0x0034_fb90;
        oc664_fake_setup_overlap => s11::OVERRIDE_00356780, f_00356780__generated, 0x0035_6780;
        oc664_fake_anim_ctor => s11::OVERRIDE_00341C80, f_00341c80__generated, 0x0034_1c80;
        oc664_fake_anim_v15 => s11::OVERRIDE_00341D48, f_00341d48__generated, 0x0034_1d48;
    }

    use crate::difftest::fakes;
    use crate::difftest::SP as TEST_SP;

    pub(crate) use crate::game::object::flexrailnode::tests::FIX;

    pub(crate) use crate::game::object::flexrailnode::tests::oc687_gate;
    pub(crate) use crate::game::object::flexrailnode::tests::oc687_install_fakes;

    pub(crate) use crate::game::object::animnode::tests::oc687_stream;

    /// Sets the 52-byte modifier header the stream's read method would fill
    /// (the fake read leaves it alone) at the callee's frame, `SP - 0xe0 -
    /// extra`.
    pub(crate) fn oc687_block_header(c: &mut Cpu, r: &mut Rng, seed: u32, extra: u32) {
        let h = TEST_SP - 0xe0 - extra;
        let flags = [0, 1, 3, 5, 9, 0xf, 2, 4][(seed % 8) as usize] | (r.next_u32() & 1);
        c.bus.write32(h, flags);
        c.bus.write32(h + 4, (seed / 8) % 9);
        c.bus.write32(h + 8, (seed / 3) % 3);
        c.bus.write32(h + 0xc, (seed / 5) % 4);
        c.bus.write32(h + 0x10, (seed / 7) % 3);
        for i in 0..4 {
            c.bus.write8(h + 0x14 + i, (r.next_u32() % 5) as u8);
        }
    }

    /// `ModifierBlock_Load`: the root transform type 1..7 (and none), the UV
    /// scroll, tex flip and boost flags, 0..3 effects of type 1..3 (or none),
    /// and 0..2 rails (the real `RailModifier_ctor_2` runs on a zero world).
    #[test]
    fn modifier_block_load_matches() {
        oc687_install_fakes();
        fakes::covered(
            ee::s11::f_003536d8__generated,
            modifier_block_load_3,
            400,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x35_36d8, 14);
                oc687_gate(c);
                c.pc = 0x0035_36d8;
                let (me, st) = (FIX, FIX + 0x1000);
                oc687_stream(c, &mut r, st);
                oc687_block_header(c, &mut r, seed, 0);
                c.g[A0][0] = me as u64;
                c.g[A1][0] = st as u64;
            },
        );
    }
}
