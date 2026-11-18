//! `src/seg/seg_1fbe38.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::main::ssxapp::oc429_at;
pub(crate) use crate::game::object::debouncenode::OC327_EA_MEM_ALLOC_NAMED;
pub(crate) use crate::game::ui::uiengine::oc391_at;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002f_ae38,
        world_obj_factory_attach_modifier,
        "WorldObjFactory_AttachModifier",
    ),
    (
        0x002f_c2c0,
        world_obj_factory_dead_node,
        "WorldObjFactory_DeadNode",
    ),
    (
        0x002f_b498,
        world_obj_factory_anim_teeter,
        "WorldObjFactory_AnimTeeter",
    ),
    (
        0x002f_c7d0,
        world_obj_factory_debounce,
        "WorldObjFactory_Debounce",
    ),
    (
        0x002f_cff0,
        world_obj_factory_mesh_anim,
        "WorldObjFactory_MeshAnim",
    ),
    (
        0x002f_ded0,
        wscript_fn_add_spline_modifier,
        "WScriptFn_AddSplineModifier",
    ),
    (0x002f_fb50, world_obj_factory_hide, "WorldObjFactory_Hide"),
    (
        0x0030_12f0,
        wscript_fn_mission_command,
        "WScriptFn_MissionCommand",
    ),
    (
        0x0030_3130,
        wscript_fn_object_exists,
        "WScriptFn_ObjectExists",
    ),
    (
        0x0030_61b0,
        wscript_fn_get_progress_flag_bit,
        "WScriptFn_GetProgressFlagBit",
    ),
    (
        0x002f_f1c8,
        wscript_fn_add_rail_modifier,
        "WScriptFn_AddRailModifier",
    ),
    (
        0x0030_57c0,
        wscript_fn_add_halo_modifier,
        "WScriptFn_AddHaloModifier",
    ),
    (
        0x002f_d420,
        world_obj_factory_particle_node,
        "WorldObjFactory_ParticleNode",
    ),
    (
        0x0030_1680,
        wscript_fn_get_object_slot30,
        "WScriptFn_GetObjectSlot30",
    ),
    (0x002f_c9c8, w_script_fn_create_flag, "WScriptFn_CreateFlag"),
    (
        0x0030_2778,
        w_script_fn_is_event_type,
        "WScriptFn_IsEventType",
    ),
    (
        0x0030_3df8,
        wscript_fn_event_type_matches,
        "WScriptFn_EventTypeMatches",
    ),
    (
        0x0030_5478,
        wscript_fn_add_magnet_modifier,
        "WScriptFn_AddMagnetModifier",
    ),
    (
        0x0030_6cb0,
        wscript_fn_multi_particle_control,
        "WScriptFn_MultiParticleControl",
    ),
    (
        0x002f_dc60,
        wscript_fn_add_parent_modifier_rev,
        "WScriptFn_AddParentModifierRev",
    ),
    (
        0x0030_0260,
        wscript_fn_stop_object_loop,
        "WScriptFn_StopObjectLoop",
    ),
    (0x0030_3598, wscript_fn_rand_range, "WScriptFn_RandRange"),
    (
        0x002f_f850,
        wscript_fn_rider_command,
        "WScriptFn_RiderCommand",
    ),
    (
        0x002f_f9a8,
        w_script_fn_object_slot36,
        "WScriptFn_ObjectSlot36",
    ),
    (
        0x0030_0f50,
        w_script_fn_add_collect_items,
        "WScriptFn_AddCollectItems",
    ),
];

// ---------------------------------------------------------------------------
// WorldObjFactory_DeadNode (0x2fc2c0)

/// The event's kind (+4): 0 kill (replace with a `DeadNode`), 1 hide
/// (flags only), 3 restore (replace with a `RestoreNode`).
pub(crate) const DEAD_NODE_EVENT_KIND: u32 = 0x4;

/// The world node: flags at +8, its current object at +0xc.
pub(crate) const DEAD_NODE_FLAGS: u32 = 0x8;

pub(crate) const DEAD_NODE_OBJECT: u32 = 0xc;

/// The object's vtable (+0xc) and type (`s16` at +0x10).
pub(crate) const DEAD_NODE_OBJ_VTABLE: u32 = 0xc;

pub(crate) const DEAD_NODE_OBJ_TYPE: u32 = 0x10;

/// Object types left alone: a dead node (6), a restore node (0x13).
pub(crate) const DEAD_NODE_TYPE_DEAD: u64 = 0x6;

pub(crate) const DEAD_NODE_TYPE_RESTORE: u64 = 0x13;

/// The allocation names of the two replacement nodes.
pub(crate) const DEAD_NODE_NAME: u64 = 0x0048_96b8;

pub(crate) const DEAD_NODE_RESTORE_NAME: u64 = 0x0048_96c8;

/// Delete the node's current object: `obj->vtable[8](obj + delta, 3)`.
pub(crate) fn dead_node_delete_object(c: &mut Cpu, obj: u64, ret: u32) -> crate::call::Flow {
    c.cycles += 6;
    let vt = Addr((obj as u32).wrapping_add(DEAD_NODE_OBJ_VTABLE)).lw(c);
    c.g[V0][0] = vt;
    c.g[A1][0] = 3;
    let slot = crate::call::VSlot::read(c, vt, 8);
    c.g[V1][0] = slot.func;
    c.g[A0][0] = sx((obj as u32).wrapping_add(slot.delta as u32));
    call_ptr(c, slot.func as u32, ret)
}

/// `EA_MemAllocNamed(0x1c, name, 0x20000000, 0)`, then `ctor(block, node)`.
pub(crate) fn dead_node_replace(
    c: &mut Cpu,
    name: u64,
    alloc_ret: u32,
    ctor: u32,
    ret: u32,
) -> crate::call::Flow {
    c.g[A1][0] = name;
    c.g[A0][0] = 0x1c;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    Callee {
        addr: 0x0031_7d70,
        f: ee::s10::EA_MemAllocNamed,
    }
    .call(c, alloc_ret)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, ctor, ret)
}

/// `void WorldObjFactory_DeadNode(WorldObjFactory* node, Event* ev)`
/// (0x2fc2c0): applies a dead-node script event to a world node. Kind 1
/// deletes the node's object (unless it already is a dead node) and folds
/// the flags (`f &= 0xffff0300; f |= f >> 16 (arith) | 2`); kind 0 deletes
/// it and installs a new `DeadNode`; kind 3 deletes it (unless it is a
/// dead or restore node) and installs a `RestoreNode`. Frame 0x20 (s0 =
/// node).
pub fn world_obj_factory_dead_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let kind = Addr(r32(c, A1).wrapping_add(DEAD_NODE_EVENT_KIND)).lw(c);
        c.g[V1][0] = kind;
        c.g[S0][0] = c.g[A0][0];
        let node = r32(c, S0);
        match kind {
            1 => {
                c.cycles += 3;
                let obj = Addr(node.wrapping_add(DEAD_NODE_OBJECT)).lw(c);
                c.g[A2][0] = obj;
                c.g[V0][0] = DEAD_NODE_TYPE_DEAD;
                if obj != 0 {
                    c.cycles += 3;
                    c.g[V1][0] = Addr((obj as u32).wrapping_add(DEAD_NODE_OBJ_TYPE)).lh(c);
                    if c.g[V1][0] == DEAD_NODE_TYPE_DEAD {
                        frame.restore(c, S0, 0x10);
                        return dead_node_leave(c, frame);
                    }
                    dead_node_delete_object(c, obj, 0x002f_c338)?;
                }
                c.cycles += 9;
                let node = r32(c, S0);
                let kept = Addr(node.wrapping_add(DEAD_NODE_FLAGS)).lw(c) & 0xffff_ffff_ffff_0300;
                c.g[V1][0] = 0xffff_ffff_ffff_0300;
                let high = sx(((kept as u32) as i32 >> 16) as u32);
                c.g[A0][0] = high;
                let flags = kept | high | 2;
                c.g[V0][0] = flags;
                Addr(node.wrapping_add(DEAD_NODE_FLAGS)).sw(c, flags);
                c.cycles += 1;
            }
            0 => {
                c.cycles += 3 + 2 + 2;
                let obj = Addr(node.wrapping_add(DEAD_NODE_OBJECT)).lw(c);
                c.g[A2][0] = obj;
                c.g[V0][0] = DEAD_NODE_TYPE_DEAD;
                if obj != 0 {
                    c.cycles += 3;
                    c.g[V1][0] = Addr((obj as u32).wrapping_add(DEAD_NODE_OBJ_TYPE)).lh(c);
                    if c.g[V1][0] == DEAD_NODE_TYPE_DEAD {
                        frame.restore(c, S0, 0x10);
                        return dead_node_leave(c, frame);
                    }
                    dead_node_delete_object(c, obj, 0x002f_c388)?;
                }
                c.cycles += 11;
                dead_node_replace(c, DEAD_NODE_NAME, 0x002f_c3a0, 0x0035_06d8, 0x002f_c3ac)?;
            }
            3 => {
                c.cycles += 3 + 2 + 2;
                let obj = Addr(node.wrapping_add(DEAD_NODE_OBJECT)).lw(c);
                c.g[A2][0] = obj;
                c.g[V0][0] = DEAD_NODE_TYPE_DEAD;
                if obj != 0 {
                    c.cycles += 3;
                    c.g[V1][0] = Addr((obj as u32).wrapping_add(DEAD_NODE_OBJ_TYPE)).lh(c);
                    c.g[V0][0] = DEAD_NODE_TYPE_RESTORE;
                    if c.g[V1][0] == DEAD_NODE_TYPE_DEAD {
                        c.cycles += 1;
                        frame.restore(c, S0, 0x10);
                        return dead_node_leave(c, frame);
                    }
                    c.cycles += 2;
                    if c.g[V1][0] == DEAD_NODE_TYPE_RESTORE {
                        frame.restore(c, S0, 0x10);
                        return dead_node_leave(c, frame);
                    }
                    dead_node_delete_object(c, obj, 0x002f_c3e8)?;
                }
                c.cycles += 9;
                dead_node_replace(
                    c,
                    DEAD_NODE_RESTORE_NAME,
                    0x002f_c400,
                    0x0035_0f60,
                    0x002f_c40c,
                )?;
                c.cycles += 1;
            }
            _ => {
                // Any other kind: nothing to do.
                c.cycles += 3 + 2 + 2;
                c.g[V0][0] = if (kind as i64) < 2 { 1 } else { 3 };
            }
        }
        frame.restore(c, S0, 0x10);
        dead_node_leave(c, frame)
    })
}

/// The shared epilogue (`ld ra; addiu sp, 0x20; jr ra`, 3 cycles).
pub(crate) fn dead_node_leave(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// 0x002fae38 WorldObjFactory_AttachModifier

/// `Node.vtable` (+0xc).
pub(crate) const WOF_ATTACH_VTABLE: u32 = 0xc;

/// `Node.object` (+0x18): the world object a modifier attaches to.
pub(crate) const WOF_ATTACH_OBJECT: u32 = 0x18;

/// Vtable slots (`{i16 delta, fn}` pairs): `IsType(type)` (+0x50),
/// `Refresh()` (+0x128), the deleting destructor (+0x8).
pub(crate) const WOF_ATTACH_VT_IS_TYPE: u32 = 0x50;

pub(crate) const WOF_ATTACH_VT_REFRESH: u32 = 0x128;

pub(crate) const WOF_ATTACH_VT_DELETE: u32 = 0x8;

/// `node->vtable->slot(node + delta, a1)` with the original's registers:
/// `rv` holds the vtable, `rf` the method.
pub(crate) fn wof_attach_vcall(
    c: &mut Cpu,
    rv: usize,
    rf: usize,
    slot: u32,
    ret: u32,
) -> crate::call::Flow {
    let node = c.g[S0][0] as u32;
    c.g[rv][0] = c.lw(node.wrapping_add(WOF_ATTACH_VTABLE));
    let vt = c.g[rv][0] as u32;
    c.g[A0][0] = c.lh(vt.wrapping_add(slot));
    c.g[rf][0] = c.lw(vt.wrapping_add(slot + 4));
    let target = c.g[rf][0] as u32;
    c.g[A0][0] = sx(node.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `int WorldObjFactory_AttachModifier(Node* node, int type, void* data)`
/// (0x2fae38): a node that already is a type 9, 6 or 22 modifier keeps
/// going (1); one that already is `type` is refreshed (1). Otherwise types
/// 13 (particle), 11 (mesh animation), 0 (debounce) and 22 (dead fade)
/// allocate their modifier on the node's object (whose +0xc word is
/// cleared), built from `data` (-1); any other type deletes the node (0).
pub fn world_obj_factory_attach_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0]; // node
        c.g[S2][0] = c.g[A1][0]; // type
        c.g[S3][0] = c.g[A2][0]; // data
        'done: {
            // Already a type 9, 6 or 22 modifier: nothing to do.
            for (i, (kind, ret)) in [
                (0x9u64, 0x002f_ae74u32),
                (0x6, 0x002f_ae94),
                (0x16, 0x002f_aeb4),
            ]
            .into_iter()
            .enumerate()
            {
                if i > 0 {
                    c.cycles += 8;
                }
                c.g[A1][0] = kind;
                wof_attach_vcall(c, V1, V0, WOF_ATTACH_VT_IS_TYPE, ret)?;
                let is = c.g[V0][0] != 0;
                c.g[V0][0] = 0x1;
                if is {
                    break 'done;
                }
            }
            // Already `type`: refresh it.
            c.cycles += 8;
            c.g[A1][0] = c.g[S2][0];
            wof_attach_vcall(c, V1, V0, WOF_ATTACH_VT_IS_TYPE, 0x002f_aed4)?;
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                wof_attach_vcall(c, V0, V1, WOF_ATTACH_VT_REFRESH, 0x002f_aef0)?;
                c.g[V0][0] = 0x1;
                break 'done;
            }
            c.g[V0][0] = 0xb;

            // The modifier to build: (alloc size, name, ret, ctor, ctor ret).
            c.cycles += 2;
            let ty = c.g[S2][0];
            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(WOF_ATTACH_OBJECT));
            let object = c.g[S1][0] as u32;
            let kind: Option<u8> = if ty == 0xb {
                Some(11)
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((ty as i64) < 12) as u64;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    if ty == 0 {
                        c.sw(object.wrapping_add(0xc), 0);
                        Some(0)
                    } else {
                        c.cycles += 2;
                        None
                    }
                } else {
                    c.g[V0][0] = 0xd;
                    c.cycles += 2;
                    if ty == 0xd {
                        c.sw(object.wrapping_add(0xc), 0);
                        Some(13)
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x16;
                        if ty == 0x16 {
                            c.sw(object.wrapping_add(0xc), 0);
                            Some(22)
                        } else {
                            c.cycles += 2;
                            None
                        }
                    }
                }
            };
            let Some(kind) = kind else {
                // Unknown type: delete the node.
                c.cycles += 2;
                c.g[A1][0] = 0x3;
                if c.g[S0][0] != 0 {
                    c.cycles += 5;
                    wof_attach_vcall(c, V0, V1, WOF_ATTACH_VT_DELETE, 0x002f_b040)?;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'done;
            };
            // (size, name, alloc ret, ctor, ctor ret)
            let (size, name, alloc_ret, ctor, ctor_ret) = match kind {
                13 => (
                    0x2c,
                    0x0049_0000 - 0x69e8,
                    0x002f_af50,
                    0x0035_78a8,
                    0x002f_af68,
                ),
                11 => (
                    0x6c0,
                    0x0049_0000 - 0x69d8,
                    0x002f_af8c,
                    0x0035_1b40,
                    0x002f_afac,
                ),
                0 => (
                    0x34,
                    0x0049_0000 - 0x69c8,
                    0x002f_afcc,
                    0x0034_2c08,
                    0x002f_afe4,
                ),
                _ => (
                    0x3c,
                    0x0049_0000 - 0x69b8,
                    0x002f_b004,
                    0x0035_0750,
                    0x002f_b01c,
                ),
            };
            if kind == 11 {
                c.cycles += 17;
                c.sw(object.wrapping_add(0xc), 0);
            } else {
                c.cycles += 14;
            }
            // EA_MemAllocNamed(size, name, 0x20000000, 0)
            c.g[A1][0] = sx(name);
            c.g[A0][0] = size;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            Callee {
                addr: 0x0031_7d70,
                f: ee::s10::EA_MemAllocNamed,
            }
            .call(c, alloc_ret)?;
            // ctor(mem, 1, object, data, [the world's +0x294 for meshes,] node)
            let node = c.g[S0][0];
            if kind == 11 {
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x294));
                c.g[T1][0] = node;
            } else {
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[T0][0] = node;
            }
            c.g[A1][0] = 0x1;
            call_ptr(c, ctor, ctor_ret)?;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObjFactory_Hide (0x2ffb50)

/// gp-relative globals of the factory (`gp+0x2560` = `D_004A5650`, the
/// "static seed ready" flag).
pub(crate) const WOF_INIT_FLAG: u32 = 0x2560;

/// The 64-bit static seed word at `D_004A5648`, and its trailing byte at
/// `D_004A564C` (`gp+0x255c`).
pub(crate) const WOF_SEED_WORD: u32 = 0x2558;

pub(crate) const WOF_SEED_BYTE: u32 = 0x255c;

/// `D_004A3C10`: the per-index type of the entries (2 = float value).
pub(crate) const WOF_KINDS: u32 = 0xb20;

/// `D_004A47B8`: the world's object reference table.
pub(crate) const WOF_OBJECTS: u32 = 0x16c8;

/// `D_004A3DD8`: the object whose `+0x290` is returned for id `-1`.
pub(crate) const WOF_SPECIAL: u32 = 0xce8;

/// The tag passed to the allocator: `0x4a0000 + 0x3c18` = `D_004A3C18`.
pub(crate) const WOF_ALLOC_TAG_BASE: u32 = 0x4a_0000;

pub(crate) const WOF_ALLOC_TAG: u32 = 0x3c18;

/// `s4` holds `0x4d0000`, the epilogue's copy source minus 0x6f68.
pub(crate) const WOF_RESULT_BASE: u32 = 0x4d_0000;

pub(crate) const WOF_RESULT_OFF: u32 = 0x6f68;

/// `EA_MemCopy` (0x3e6574).
pub(crate) const WOF_MEM_COPY: u32 = 0x003e_6574;

/// `EA_MemAllocNamed` (0x317d70).
pub(crate) const WOF_MEM_ALLOC: u32 = 0x0031_7d70;

/// `Hide_ctor` (0x350e90), called on the new 0x1c-byte block.
pub(crate) const WOF_HIDE_CTOR: u32 = 0x0035_0e90;

/// `V3 WorldObjFactory_Hide(int n, Ent* e)` (0x2ffb50): builds the 12-byte
/// world-object description `{id, value, value}` from `n` entries
/// (`{idx, val, pad, type}`), then hides the referenced object (kills it if it
/// is visible, else allocates a `Hide` node for it). `a0` is the hidden
/// return-buffer pointer; the result is copied there and returned in `v0`.
pub fn world_obj_factory_hide(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(r32(c, GP)).off(WOF_INIT_FLAG).lw(c);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !ready {
            // Guarded static init: seed word = -1, flag = 1, and the flag
            // byte goes through the stack (the copy-ctor artifact).
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, GP)).off(WOF_SEED_BYTE).lbu(c);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[V1][0] |= 0xffff;
            c.g[A0][0] = 0x1;
            c.sw(r32(c, GP).wrapping_add(WOF_SEED_WORD), c.g[V1][0]);
            c.sw(r32(c, GP).wrapping_add(WOF_INIT_FLAG), c.g[A0][0]);
            c.sb(r32(c, SP), c.g[V0][0]);
        }
        // buf64[0] = -1, then the 8-byte seed word is copied over it.
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x8;
        c.g[A0][0] = c.g[S0][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = sx(r32(c, GP).wrapping_add(WOF_SEED_WORD));
        c.g[S4][0] = sx(WOF_RESULT_BASE);
        call_ptr(c, WOF_MEM_COPY, 0x002f_fbc0)?;
        // `a2 = s2`: the entry pointer, in the branch delay slot.
        c.g[A2][0] = c.g[S2][0];
        // For each entry: buf[idx] = val, as a float when the kind table says
        // 2 and the entry's own type says otherwise.
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(r32(c, GP).wrapping_add(WOF_KINDS));
            c.g[A3][0] = 0x2;
            loop {
                c.cycles += 8;
                let entry = r32(c, A2);
                c.g[V0][0] = Addr(entry).lw(c);
                c.g[V1][0] = Addr(entry).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S0)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, T0)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                let mut store_int = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A3][0] {
                        c.cycles += 4;
                        c.lwc1(0, entry.wrapping_add(0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        store_int = false;
                    } else {
                        c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                    }
                }
                if store_int {
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_fbd8);
                }
            }
        }
        // Resolve the id: id -1 means the object the world points at, else
        // `table[id & 0xff] -> [id >> 8]`, a reference (byte offset) or 0.
        c.cycles += 4;
        c.g[A2][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = !(0u64 | c.g[A2][0]);
        let special = c.g[V0][0] == 0;
        c.g[A1][0] = Addr(r32(c, GP)).off(WOF_OBJECTS).lw(c);
        if special {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, GP)).off(WOF_SPECIAL).lw(c);
            c.g[S1][0] = Addr(r32(c, V0)).off(0x290).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A1][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0;
            }
            c.cycles += 1;
            c.g[S1][0] = c.g[A1][0];
        }
        // Hide it: nothing to do without an object; if it is still visible,
        // kill it; otherwise build a Hide node for it.
        c.cycles += 2;
        let mut hide = false;
        if c.g[S1][0] != 0 {
            hide = true;
            c.cycles += 3;
            c.g[S0][0] = Addr(r32(c, S1)).off(0xc).lw(c);
            c.g[A1][0] = sx(WOF_ALLOC_TAG_BASE);
            if c.g[S0][0] != 0 {
                // p->v10(16)?
                c.cycles += 8;
                c.g[V1][0] = Addr(r32(c, S0)).off(0xc).lw(c);
                c.g[A1][0] = 0x10;
                c.g[A0][0] = Addr(r32(c, V1)).off(0x50).lh(c);
                c.g[V0][0] = Addr(r32(c, V1)).off(0x54).lw(c);
                let query = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, query, 0x002f_fcb8)?;
                let hidden = c.g[V0][0] != 0;
                c.g[A1][0] = 0x6;
                if hidden {
                    hide = false;
                } else {
                    // p->v10(6)?
                    c.cycles += 7;
                    c.g[V1][0] = Addr(r32(c, S0)).off(0xc).lw(c);
                    c.g[A0][0] = Addr(r32(c, V1)).off(0x50).lh(c);
                    c.g[V0][0] = Addr(r32(c, V1)).off(0x54).lw(c);
                    let query = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    call_ptr(c, query, 0x002f_fcd4)?;
                    let hidden = c.g[V0][0] != 0;
                    c.g[A1][0] = 0x3;
                    if hidden {
                        hide = false;
                    } else {
                        // Still visible: p->v01(3) kills it.
                        c.cycles += 6;
                        c.g[V0][0] = Addr(r32(c, S0)).off(0xc).lw(c);
                        c.g[A0][0] = Addr(r32(c, V0)).off(0x8).lh(c);
                        c.g[V1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
                        let kill = r32(c, V1);
                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                        call_ptr(c, kill, 0x002f_fcf0)?;
                        c.g[A1][0] = sx(WOF_ALLOC_TAG_BASE);
                    }
                }
            }
        }
        if hide {
            c.cycles += 8;
            c.g[A0][0] = 0x1c;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(WOF_ALLOC_TAG));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, WOF_MEM_ALLOC, 0x002f_fd08)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, WOF_HIDE_CTOR, 0x002f_fd14)?;
        }
        // Return the 12-byte result description by value.
        c.cycles += 16;
        let src = r32(c, S4).wrapping_sub(WOF_RESULT_OFF);
        c.g[A1][0] = sx(src);
        c.ldl(2, src.wrapping_add(0x7));
        c.ldr(2, src);
        c.g[V1][0] = Addr(src).off(0x8).lw(c);
        let dst = r32(c, S3);
        c.sdl(2, dst.wrapping_add(0x7));
        c.sdr(2, dst);
        c.sw(dst.wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S3][0];
        for (reg, off) in [(S0, 0x70), (S1, 0x60), (S2, 0x50), (S3, 0x40), (S4, 0x30)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `WorldObjFactory_Debounce` globals: the static seed word at `0x4fb528`
/// (`a3 = 0x500000`, minus 0x4ad8), its ready flag at `gp+0x24fc`, and the
/// per-index kind table at `0x445fb0`.
pub(crate) const WOD_INIT_FLAG: u32 = 0x24fc;

pub(crate) const WOD_STATIC_BASE: u32 = 0x50_0000;

pub(crate) const WOD_STATIC_OFF: u32 = 0x4ad8;

pub(crate) const WOD_KINDS_BASE: u32 = 0x44_0000;

pub(crate) const WOD_KINDS_OFF: u32 = 0x5fb0;

/// The allocator tag: `0x490000 - 0x6918` = `D_004896E8`.
pub(crate) const WOD_ALLOC_TAG: u32 = 0x490000;

pub(crate) const WOD_ALLOC_TAG_OFF: u32 = 0x6918;

/// `WorldObjFactory_AttachModifier` (0x2fae38).
pub(crate) const WOD_CHECK_ACTIVE: u32 = 0x002f_ae38;

/// `Debounce_ctor_2` (0x342c08).
pub(crate) const WOD_DEBOUNCE_CTOR: u32 = 0x0034_2c08;

/// `V3 WorldObjFactory_Debounce(int n, Ent* e)` (0x2fc7d0): like
/// `WorldObjFactory_Hide`, but the node it builds is a `Debounce` (0x34
/// bytes) that ignores objects an active modifier already covers.
pub fn world_obj_factory_debounce(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(r32(c, GP)).off(WOD_INIT_FLAG).lw(c);
        frame.save(c, S0, 0x80);
        c.g[A3][0] = sx(WOD_STATIC_BASE);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S4, 0x40);
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0x30);
        if !ready {
            // Guarded static init of the 20-byte seed: -1, 1.0f, 1, -1 and
            // its trailing flag byte through the stack.
            c.cycles += 15;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(WOD_STATIC_OFF));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A2][0] = Addr(r32(c, V1)).off(0x10).lbu(c);
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A0][0] = !0u64;
            c.sw(r32(c, A3).wrapping_sub(WOD_STATIC_OFF), c.g[V0][0]);
            c.g[A1][0] = 0x1;
            c.sb(r32(c, V1).wrapping_add(0x10), c.g[A2][0]);
            c.swc1(0, r32(c, V1).wrapping_add(0x4));
            c.sw(r32(c, V1).wrapping_add(0xc), c.g[A0][0]);
            c.sw(r32(c, GP).wrapping_add(WOD_INIT_FLAG), c.g[A1][0]);
            c.sw(r32(c, V1).wrapping_add(0x8), c.g[A1][0]);
            c.sb(r32(c, SP), c.g[A2][0]);
        }
        // buf64[5] = -1, then the 20-byte static over it.
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x14;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(WOD_STATIC_OFF));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = sx(WOF_RESULT_BASE);
        call_ptr(c, WOF_MEM_COPY, 0x002f_c864)?;
        // `a2 = s3`: the entry pointer (the third argument), in the branch
        // delay slot.
        c.g[A2][0] = c.g[S3][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V0][0] = sx(WOD_KINDS_BASE);
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(WOD_KINDS_OFF));
            c.g[A3][0] = 0x2;
            loop {
                c.cycles += 8;
                let entry = r32(c, A2);
                c.g[V0][0] = Addr(entry).lw(c);
                c.g[V1][0] = Addr(entry).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S1)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, T0)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                let mut store_int = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A3][0] {
                        c.cycles += 4;
                        c.lwc1(0, entry.wrapping_add(0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        store_int = false;
                    } else {
                        c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                    }
                }
                if store_int {
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_c880);
                }
            }
        }
        // Resolve the id, exactly as Hide does.
        c.cycles += 4;
        c.g[A2][0] = Addr(r32(c, S1)).lw(c);
        c.g[V0][0] = !(0u64 | c.g[A2][0]);
        let special = c.g[V0][0] == 0;
        c.g[A1][0] = Addr(r32(c, GP)).off(WOF_OBJECTS).lw(c);
        if special {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, GP)).off(WOF_SPECIAL).lw(c);
            c.g[S0][0] = Addr(r32(c, V0)).off(0x290).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
        }
        // Skip objects that are already covered by an active modifier, else
        // build the Debounce node.
        c.cycles += 2;
        let mut insert = false;
        if c.g[S0][0] != 0 {
            insert = true;
            c.cycles += 3;
            c.g[A0][0] = Addr(r32(c, S0)).off(0xc).lw(c);
            c.g[A1][0] = 0;
            if c.g[A0][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, WOD_CHECK_ACTIVE, 0x002f_c950)?;
                if c.g[V0][0] != 0 {
                    insert = false;
                }
            }
        }
        if insert {
            c.cycles += 12;
            c.g[A1][0] = sx(WOD_ALLOC_TAG);
            c.g[A0][0] = 0x34;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(WOD_ALLOC_TAG_OFF));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, WOF_MEM_ALLOC, 0x002f_c970)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S1][0];
            c.g[T0][0] = 0;
            call_ptr(c, WOD_DEBOUNCE_CTOR, 0x002f_c988)?;
        }
        // Return the 12-byte result description by value.
        c.cycles += 16;
        let src = r32(c, S4).wrapping_sub(WOF_RESULT_OFF);
        c.g[A1][0] = sx(src);
        c.ldl(2, src.wrapping_add(0x7));
        c.ldr(2, src);
        c.g[V1][0] = Addr(src).off(0x8).lw(c);
        let dst = r32(c, S2);
        c.sdl(2, dst.wrapping_add(0x7));
        c.sdr(2, dst);
        c.sw(dst.wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        for (reg, off) in [(S0, 0x80), (S1, 0x70), (S2, 0x60), (S3, 0x50), (S4, 0x40)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptFn_AddSplineModifier` globals: the static spline seed at 0x4fb778
/// (`0x500000 - 0x4888`), its ready flag at `gp+0x2524`, the kind table at
/// 0x4461d8, and the 12-word stack buffer at `sp+0x10`.
pub(crate) const WSA_INIT_FLAG: u32 = 0x2524;

pub(crate) const WSA_STATIC_BASE: u32 = 0x50_0000;

pub(crate) const WSA_STATIC_OFF: u32 = 0x4888;

pub(crate) const WSA_KINDS_BASE: u32 = 0x44_0000;

pub(crate) const WSA_KINDS_OFF: u32 = 0x61d8;

/// `EA_MemCopy` (0x3e6574).
pub(crate) const WSA_MEM_COPY: u32 = 0x003e_6574;

/// `cMoveNode_addSpline1` (0x355ad0).
pub(crate) const WSA_ADD_SPLINE: u32 = 0x0035_5ad0;

pub(crate) const WSA_RESULT_BASE: u32 = 0x4d_0000;

pub(crate) const WSA_RESULT_OFF: u32 = 0x6f68;

/// `V3 WScriptFn_AddSplineModifier(int n, Ent* e)` (0x2fded0): the script
/// function behind "add spline": builds a 48-byte spline description from `n`
/// entries and, if its second word names a node, adds the spline to the move
/// node the first word resolves to.
pub fn wscript_fn_add_spline_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(r32(c, GP)).off(WSA_INIT_FLAG).lw(c);
        frame.save(c, S0, 0x80);
        c.g[A3][0] = sx(WSA_STATIC_BASE);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A0][0];
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0x40);
        if !ready {
            // Guarded static init: -1, -1, 1, 3, 20.0, 0, 0, 0, 1, 0, 0 and
            // the trailing tag byte through the stack.
            c.cycles += 23;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(WSA_STATIC_OFF));
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.mtc1(0, 0);
            c.g[V1][0] |= 0xffff;
            c.g[A2][0] = Addr(r32(c, V0)).off(0x2c).lbu(c);
            c.g[A1][0] = 0x3;
            c.g[AT][0] = 0x41a0_0000;
            c.mtc1(1, 1);
            c.g[A0][0] = 0x1;
            c.sw(r32(c, A3).wrapping_sub(WSA_STATIC_OFF), c.g[V1][0]);
            c.sb(r32(c, V0).wrapping_add(0x2c), c.g[A2][0]);
            c.sw(r32(c, V0).wrapping_add(0x4), c.g[V1][0]);
            c.sw(r32(c, V0).wrapping_add(0xc), c.g[A1][0]);
            c.swc1(1, r32(c, V0).wrapping_add(0x10));
            c.swc1(0, r32(c, V0).wrapping_add(0x28));
            c.sw(r32(c, GP).wrapping_add(WSA_INIT_FLAG), c.g[A0][0]);
            c.sw(r32(c, V0).wrapping_add(0x8), c.g[A0][0]);
            c.swc1(0, r32(c, V0).wrapping_add(0x14));
            c.swc1(0, r32(c, V0).wrapping_add(0x18));
            c.swc1(0, r32(c, V0).wrapping_add(0x1c));
            c.sw(r32(c, V0).wrapping_add(0x20), c.g[A0][0]);
            c.swc1(0, r32(c, V0).wrapping_add(0x24));
            c.sb(r32(c, SP), c.g[A2][0]);
        }
        // buf64[12]: -1, -1, then the 48-byte static spline over it.
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x30;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(WSA_STATIC_OFF));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        call_ptr(c, WSA_MEM_COPY, 0x002f_df80)?;
        // `a2 = s2`: the entry pointer, in the branch delay slot.
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = sx(WSA_KINDS_BASE);
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(WSA_KINDS_OFF));
            c.g[A3][0] = 0x2;
            loop {
                c.cycles += 8;
                let entry = r32(c, A2);
                c.g[V0][0] = Addr(entry).lw(c);
                c.g[V1][0] = Addr(entry).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S1)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, T0)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                let mut store_int = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A3][0] {
                        c.cycles += 4;
                        c.lwc1(0, entry.wrapping_add(0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        store_int = false;
                    } else {
                        c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                    }
                }
                if store_int {
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_df98);
                }
            }
        }
        // Resolve the node id from buf[0].
        c.cycles += 4;
        c.g[A2][0] = Addr(r32(c, S1)).lw(c);
        c.g[V0][0] = !(0u64 | c.g[A2][0]);
        let special = c.g[V0][0] == 0;
        c.g[A1][0] = Addr(r32(c, GP)).off(WOF_OBJECTS).lw(c);
        if special {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, GP)).off(WOF_SPECIAL).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x290).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            let missing = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            if !missing {
                c.cycles += 7;
                c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
        }
        // If buf[1] names something, add the spline to the node it hangs on.
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, S1)).off(0x4).lw(c);
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        if c.g[V0][0] == 0 {
            c.g[V1][0] = sx(WSA_RESULT_BASE);
        } else {
            c.cycles += 3;
            c.g[S0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
            let no_node = c.g[S0][0] == 0;
            c.g[V1][0] = sx(WSA_RESULT_BASE);
            if !no_node {
                c.cycles += 7;
                c.g[V1][0] = Addr(r32(c, S0)).off(0xc).lw(c);
                c.g[A0][0] = Addr(r32(c, V1)).off(0x80).lh(c);
                c.g[V0][0] = Addr(r32(c, V1)).off(0x84).lw(c);
                let spline = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, spline, 0x002f_e070)?;
                let unsupported = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !unsupported {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, WSA_ADD_SPLINE, 0x002f_e080)?;
                }
                c.cycles += 1;
                c.g[V1][0] = sx(WSA_RESULT_BASE);
            }
        }
        // Return the 12-byte result description by value.
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(WSA_RESULT_OFF));
        let src = r32(c, A3);
        c.ldl(4, src.wrapping_add(0x7));
        c.ldr(4, src);
        c.g[A1][0] = Addr(src).off(0x8).lw(c);
        let dst = r32(c, S3);
        c.sdl(4, dst.wrapping_add(0x7));
        c.sdr(4, dst);
        c.sw(dst.wrapping_add(0x8), c.g[A1][0]);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The `gp` register as an address.
#[inline]
pub(crate) fn gp_addr(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// `gp + off`.
#[inline]
pub(crate) fn gp_oc322(c: &Cpu, off: u32) -> u32 {
    gp_addr(c).wrapping_add(off)
}

// ---------------------------------------------------------------------------
// WScriptFn_ObjectExists (0x303130)
// ---------------------------------------------------------------------------

/// `D_004A57B4`: the one-time initialisation flag of the shared id buffer.
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_INIT: u32 = 0x26c4;

/// `D_004A3D18`: the script value-type table (indexed by the entry's slot).
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_TYPES: u32 = 0xc28;

/// `D_004A47B8`: the id table head.
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_TABLE: u32 = 0x16c8;

/// `D_004A28A8`: the world object the id `-1` maps to (`+0x290`).
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_WORLD: u32 = 0xce8;

/// `D_004FBC50` (`D_3130`): the shared id buffer (3 words), `0x500000-0x43b0`.
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_IDS: u32 = 0x004f_bc50;

/// The local copy of the ids (`buf`) and the 0x10-byte entry record
/// (`{int slot; int val; int pad; int type}`).
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_BUF: u32 = 0x10;

pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_ENTRY: u32 = 0x10;

/// `V3* WScriptFn_ObjectExists(V3* ret, int n, Ent* e)` (0x303130): the
/// script bridge's `ObjectExists`; copy the shared id buffer (`D_3130`),
/// overlay the `n` entries (`slot`, `value`, `type`; a float value when the
/// entry's type differs from the slot's script type and that type is 2), look
/// the first id up in the world's object table and report
/// `ret[2] = 1`, `*ret = object != 0 && object->field_0xc`, storing the flag
/// in the script object at `+8`.
pub fn wscript_fn_object_exists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 19, 0x30);
        frame.save(c, 17, 0x50);
        frame.save(c, 18, 0x40);
        frame.save(c, 16, 0x60);
        frame.save_ra(c, 0x20);
        c.g[V0][0] = c.lw(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_INIT));
        c.g[S3][0] = c.g[A2][0]; // the entry array
        c.g[S1][0] = c.g[A0][0]; // the script object
        c.g[S2][0] = c.g[A1][0]; // the entry count
        c.g[A2][0] = WSCRIPT_FN_OBJECT_EXISTS_IDS.wrapping_add(0x43b0) as u64;
        if c.g[V0][0] == 0 {
            // One-time init of the shared id buffer: `D_3130[0] = -1`,
            // `D_3130[1] = 1` and the tag byte copied onto itself.
            c.cycles += 10;
            c.g[V1][0] = sx(WSCRIPT_FN_OBJECT_EXISTS_IDS);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(8));
            c.g[V0][0] |= 0xffff;
            c.sw(WSCRIPT_FN_OBJECT_EXISTS_IDS, c.g[V0][0]);
            c.g[A0][0] = 1;
            c.sb((c.g[V1][0] as u32).wrapping_add(8), c.g[A1][0]);
            c.sw(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_INIT), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(4), c.g[A0][0]);
            c.sb(c.g[SP][0] as u32, c.g[A1][0]);
        }
        // `buf[0] = -1; memcpy(buf, D_3130, 12)` and the entry overlay.
        'overlay: {
            c.cycles += 10;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(WSCRIPT_FN_OBJECT_EXISTS_BUF));
            c.g[V0][0] |= 0xffff;
            c.g[A1][0] = sx(WSCRIPT_FN_OBJECT_EXISTS_IDS);
            c.g[A2][0] = 0xc;
            c.sw(
                (c.g[SP][0] as u32).wrapping_add(WSCRIPT_FN_OBJECT_EXISTS_BUF),
                c.g[V0][0],
            );
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e_6574, 0x0030_31a8)?;
            let no_entries = (c.g[S2][0] as i64) <= 0;
            c.g[A2][0] = c.g[S3][0];
            if no_entries {
                break 'overlay;
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[T0][0] = sx(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_TYPES));
            c.g[A3][0] = 0x2; // the script type of a float slot
            loop {
                'store: {
                    'value: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(c.g[A2][0] as u32); // the slot
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc)); // type
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V0][0] as u32); // the slot's type
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
                            break 'value;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
                            break 'value;
                        }
                        // A float value: `*(float*)d = (float)x->val`.
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        break 'store;
                    }
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(WSCRIPT_FN_OBJECT_EXISTS_ENTRY));
                if !more {
                    break 'overlay;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_31c0);
                }
            }
        }
        // `look_3130(buf[0])`: the id -> object lookup.
        'lookup: {
            {
                c.cycles += 4;
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let id_is_minus_one = c.g[V0][0] == 0;
                c.g[A1][0] = c.lw(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_TABLE));
                if id_is_minus_one {
                    // id -1: the world's own object.
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_WORLD));
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                    break 'lookup;
                }
                'table: {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let absent = c.g[V1][0] == 0;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                    if absent {
                        break 'table;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let found = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if found {
                        break 'lookup;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // Report: `*ret = obj && obj->field_0xc`, `ret[2] = 1`, and the flag
        // goes into the script object at `+8`.
        'report: {
            'nonnull: {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 1;
                    break 'nonnull;
                }
                'live: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let dead = c.g[V0][0] == 0;
                    c.g[S0][0] = 1;
                    if dead {
                        break 'live;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[S0][0]);
                    call_ptr(c, 0x0022_6610, 0x0030_3284)?;
                    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                    break 'report;
                }
                c.cycles += 1;
                c.g[V0][0] = 1;
                break 'nonnull;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[V0][0]);
            call_ptr(c, 0x0022_6610, 0x0030_329c)?;
            c.sw(c.g[V0][0] as u32, 0);
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `MEM_copy` (EA_MemCopy, 0x3e6574).
pub(crate) const OC324_MEM_COPY: u32 = 0x003e_6574;

/// `LunoVMRegister_AsInt` (0x226610): returns its argument.
pub(crate) const OC324_LUNO_REGISTER_AS_INT: u32 = 0x0022_6610;

/// The career-flag block a WScript BIND call unpacks from (`gp + 0x2790`: a
/// word, then the bind tag byte, then the "initialised" flag).
pub(crate) const OC324_CAREER_FLAGS: u32 = 0x2790;

/// The "initialised" word of that block (`gp + 0x2798`).
pub(crate) const OC324_CAREER_FLAGS_INIT: u32 = 0x2798;

/// The mission-command call's static argument description (`D_004FBEE0`).
#[cfg(test)]
pub(crate) const OC324_MISSION_DESC: u32 = 0x004f_bee0;

/// The "no change" result a script call copies into its slot
/// (`D_004C9098`, 12 bytes).
pub(crate) const OC324_NO_CHANGE: u32 = 0x004c_9098;

// ---------------------------------------------------------------------------
// 0x003061b0 WScriptFn_GetProgressFlagBit

/// `int* WScriptFn_GetProgressFlagBit(int *ret, int n, Ent *e)` (0x3061b0): a
/// Luno script call (the binder `BIND_UNPACK_ARGS` of the career flag block)
/// that answers one progress-flag bit. The first unpacked value picks the bit
/// of `D_005308d0` (0: bit 6, 1: bit 8, 2: bit 7) and the flag is its
/// complement (1 when the bit is clear); any other value answers 1. The
/// result goes to the slot `LunoVMRegister_AsInt(ret)` returns.
pub fn wscript_fn_get_progress_flag_bit(c: &mut Cpu) {
    /// The career flags word the selected bit is read from.
    const FLAGS: u32 = 0x0053_08d0;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(OC324_CAREER_FLAGS_INIT)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !initialised {
            // Guarded static-local init: zero the word, keep the tag byte.
            c.cycles += 5;
            c.g[V1][0] = Addr(gp.wrapping_add(OC324_CAREER_FLAGS + 4)).lbu(c);
            c.g[V0][0] = 1;
            Addr(gp.wrapping_add(OC324_CAREER_FLAGS_INIT)).sw(c, c.g[V0][0]);
            Addr(gp.wrapping_add(OC324_CAREER_FLAGS)).sw(c, 0);
            c.sb(frame.at(c, 0), c.g[V1][0]);
        }
        // Copy the 8-byte flag block into a stack buffer, then expand the
        // argument list into it.
        c.cycles += 7;
        c.g[S0][0] = sx(frame.at(c, 0x10));
        c.g[A2][0] = 8;
        c.g[A1][0] = sx(gp.wrapping_add(OC324_CAREER_FLAGS));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, OC324_MEM_COPY, 0x0030_6204)?;
        let count = c.g[S1][0] as i64;
        c.g[A2][0] = c.g[S2][0];
        if count > 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(gp.wrapping_add(0xcb8));
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                // A flag whose type is 2 is stored as a float, everything
                // else as the raw word.
                let mut float_store = false;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        float_store = true;
                    }
                }
                if !float_store {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_6218);
                }
            }
        }
        // The selected flag bit: the first unpacked value picks the bit.
        c.cycles += 4;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = 1;
        let key = c.g[A0][0];
        c.g[V1][0] = 0x0053_0000;
        if key == 1 {
            c.cycles += 4;
            c.g[S0][0] = 0;
            c.g[V0][0] = Addr(FLAGS).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 8) as u32);
            c.cycles += 2;
            c.g[V0][0] &= 1;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = c.g[A1][0];
            }
        } else if (key as i64) < 2 {
            c.cycles += 3;
            if key == 0 {
                c.cycles += 2;
                c.g[S0][0] = 0;
                c.cycles += 3;
                c.g[V0][0] = Addr(FLAGS).lw(c);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                c.cycles += 2;
                c.g[V0][0] &= 1;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = c.g[A1][0];
                }
            } else {
                // A negative value: no flag, answer 1. (The value test's
                // arm, then the switch's default arm.)
                c.cycles += 4;
                c.g[S0][0] = 1;
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 2;
            c.cycles += 2;
            if key == 2 {
                c.cycles += 3;
                c.g[S0][0] = 0;
                c.g[V0][0] = Addr(FLAGS).lw(c);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                c.cycles += 2;
                c.g[V0][0] &= 1;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = c.g[A1][0];
                }
            } else {
                c.cycles += 2;
                c.g[S0][0] = 1;
            }
        }
        c.cycles += 13;
        c.g[V0][0] = 1;
        c.g[A0][0] = c.g[S3][0];
        c.sw((c.g[S3][0] as u32).wrapping_add(8), c.g[V0][0]);
        call_ptr(c, OC324_LUNO_REGISTER_AS_INT, 0x0030_62dc)?;
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003012f0 WScriptFn_MissionCommand

/// `int* WScriptFn_MissionCommand(int *ret, int n, Ent *e)` (0x3012f0): the
/// Luno script call that issues a mission command. It expands the argument
/// block over the static description `D_004FBBE0` (three words, all -1) and,
/// unless the mission id is -1, runs `WScriptMan_MissionCommand` (forced, -1)
/// and returns its result; for id -1 it copies the "no change" result
/// `D_004C9098` (12 bytes) into `ret`.
pub fn wscript_fn_mission_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(0x25c8)).lw(c);
        frame.save(c, S3, 0x30);
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x0050_0000;
        if !initialised {
            // Guarded static-local init: both words -1, tag byte kept.
            c.cycles += 11;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4420));
            c.g[V0][0] = 0xffff_ffff_ffff_0000u64;
            c.g[A1][0] = Addr(c.g[V1][0] as u32).off(8).lbu(c);
            c.g[V0][0] |= 0xffff;
            c.sw((c.g[A2][0] as u32).wrapping_sub(0x4420), c.g[V0][0]);
            c.g[A0][0] = 0xffff_ffff_ffff_ffffu64;
            c.sb((c.g[V1][0] as u32).wrapping_add(8), c.g[A1][0]);
            c.g[V0][0] = 1;
            c.sw((c.g[V1][0] as u32).wrapping_add(4), c.g[A0][0]);
            c.sw(gp.wrapping_add(0x25c8), c.g[V0][0]);
            c.sb(frame.at(c, 0), c.g[A1][0]);
        }
        // Expand the argument block over a copy of the description.
        c.cycles += 7;
        c.g[S0][0] = sx(frame.at(c, 0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4420));
        c.g[A2][0] = 0xc;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, OC324_MEM_COPY, 0x0030_1360)?;
        let count = c.g[S2][0] as i64;
        c.g[A2][0] = c.g[S3][0];
        if count > 0 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[T0][0] = sx(gp.wrapping_add(0xb80));
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                let mut float_store = false;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        float_store = true;
                    }
                }
                if !float_store {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_1378);
                }
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0xffff_ffff_ffff_0000u64;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] |= 0xffff;
        let no_command = c.g[A1][0] == c.g[V0][0];
        c.g[A3][0] = 0xffff_ffff_ffff_ffffu64;
        if !no_command {
            // Issue the mission command; the script manager is the callee's
            // `this`, the mission id and value the first two words.
            c.cycles += 10;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
            c.g[A0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
            Callee {
                addr: 0x0030_a868,
                f: ee::s10::WScriptMan_MissionCommand,
            }
            .call(c, 0x0030_13e0)?;
            c.g[V1][0] = 1;
            c.g[S0][0] = c.g[V0][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[V1][0]);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, OC324_LUNO_REGISTER_AS_INT, 0x0030_13f4)?;
            c.sw(c.g[V0][0] as u32, c.g[S0][0]);
        } else {
            // No command: copy the "no change" result into the slot.
            c.cycles += 8;
            c.g[V0][0] = 0x004d_0000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6f68));
            c.ldl(V1, (c.g[A2][0] as u32).wrapping_add(7));
            c.ldr(V1, c.g[A2][0] as u32);
            c.g[A0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.sdl(V1, (c.g[S1][0] as u32).wrapping_add(7));
            c.sdr(V1, c.g[S1][0] as u32);
            c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[A0][0]);
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002fcff0 WorldObjFactory_MeshAnim

/// The mesh-animation modifier's static description (`D_004FB5E8`, 0x40
/// bytes: id, model handle, speeds, sizes, count, colour).
#[cfg(test)]
pub(crate) const OC324_MESH_ANIM_DESC: u32 = 0x004f_b5e8;

/// The mesh-animation modifier's init flag (`D_004A55FC`).
pub(crate) const OC324_MESH_ANIM_INIT: u32 = 0x250c;

/// `void* WorldObjFactory_MeshAnim(void *ret, int n, Ent *e)` (0x2fcff0): the
/// Luno call that attaches a mesh-animation modifier to the object named by
/// the argument block: expand the block over the static description, resolve
/// the object (id -1: the world's current object), skip when the object
/// already runs one (the factory's type-11 modifier is active), else
/// allocate 0x6c0 bytes and build the anim onto it. `ret` receives the
/// "no change" result (`D_004C9098`, 12 bytes).
pub fn world_obj_factory_mesh_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xb0);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(OC324_MESH_ANIM_INIT)).lw(c);
        frame.save(c, S0, 0xa0);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x80);
        c.g[S0][0] = c.g[A1][0]; // n
        frame.save(c, S4, 0x60);
        c.g[S2][0] = c.g[A0][0]; // ret
        frame.save(c, S1, 0x90);
        c.g[S4][0] = c.g[A2][0]; // e
        frame.save(c, S3, 0x70);
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x50);
        if !initialised {
            // Guarded static-local init of the 0x40-byte description.
            c.cycles += 38;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4a18));
            c.g[V1][0] = 0xffff_ffff_ffff_0000u64;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x3c).lbu(c);
            c.g[V1][0] |= 0xffff;
            c.mtc1(0, 0);
            c.g[A2][0] = 0xffff_ffff_ffff_ffffu64;
            c.g[AT][0] = 0x4416_0000;
            c.mtc1(AT, 7);
            c.g[A1][0] = 1;
            c.lwc1(1, gp.wrapping_sub(0x338c));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2);
            c.g[AT][0] = 0x4000_0000;
            c.mtc1(AT, 3);
            c.g[AT][0] = 0x4396_0000;
            c.mtc1(AT, 4);
            c.g[AT][0] = 0x43b4_0000;
            c.mtc1(AT, 5);
            c.g[AT][0] = 0xffff_ffff_bf80_0000u64;
            c.mtc1(AT, 6);
            c.sw((c.g[A3][0] as u32).wrapping_sub(0x4a18), c.g[V1][0]);
            c.sb((c.g[V0][0] as u32).wrapping_add(0x3c), c.g[A0][0]);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(8));
            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x18));
            c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x1c));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x28));
            c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x2c));
            c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x30));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x34), c.g[V1][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x38), c.g[A2][0]);
            c.sw(gp.wrapping_add(OC324_MESH_ANIM_INIT), c.g[A1][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(4), 0);
            c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x14));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x24));
            c.sb(frame.at(c, 0), c.g[A0][0]);
        }
        // Expand the argument block over a copy of the description.
        c.cycles += 12;
        c.g[V0][0] = 0xffff_ffff_ffff_0000u64;
        c.g[S1][0] = sx(frame.at(c, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x40;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4a18));
        c.sw(frame.at(c, 0x44), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw(frame.at(c, 0x10), c.g[V0][0]);
        call_ptr(c, OC324_MEM_COPY, 0x002f_d0e0)?;
        // `lui s3, 0x4d`: the page base `D_004C9098` is built off (kept for
        // the register state a callee's call log records).
        c.g[S3][0] = 0x004d_0000;
        let count = c.g[S0][0] as i64;
        c.g[A2][0] = c.g[S4][0];
        if count > 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6060));
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                let mut float_store = false;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        float_store = true;
                    }
                }
                if !float_store {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_d100);
                }
            }
        }
        // Resolve the object the block names (-1: the current one).
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !(0u64 | c.g[A2][0]);
        let named = c.g[V0][0] != 0;
        c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
        if named {
            // The packed id names an instance table entry.
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(8).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
                // conv(): a table index of 0 means "no object".
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
            c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x290).lw(c);
        }
        // No object to attach to: the "no change" result only. An object
        // whose type-11 modifier node is already active: the same. Both
        // return before the allocation.
        c.cycles += 2;
        let no_object = c.g[S0][0] == 0;
        let mut build_anim = false;
        if !no_object {
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
            let no_node = c.g[A0][0] == 0;
            c.g[A1][0] = 0xb;
            if no_node {
                build_anim = true;
            } else {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                Callee {
                    addr: 0x002f_ae38,
                    f: ee::s10::WorldObjFactory_AttachModifier,
                }
                .call(c, 0x002f_d1d0)?;
                // A node that answers "active" skips the allocation.
                build_anim = c.g[V0][0] == 0;
            }
        }
        if build_anim {
            c.cycles += 14;
            c.g[A1][0] = 0x0049_0000;
            c.g[A0][0] = 0x6c0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x68f8));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            Callee {
                addr: 0x0031_7d70,
                f: ee::s10::EA_MemAllocNamed,
            }
            .call(c, 0x002f_d1f0)?;
            c.g[V1][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = c.g[S0][0];
            c.g[T0][0] = Addr(c.g[V1][0] as u32).off(0x294).lw(c);
            c.g[A3][0] = c.g[S1][0];
            c.g[T1][0] = 0;
            call_ptr(c, 0x0035_1b40, 0x002f_d210)?;
        }
        // The result slot: the "no change" description D_004C9098
        // (0x4d0000 - 0x6f68).
        c.cycles += 16;
        c.g[A1][0] = sx(OC324_NO_CHANGE);
        c.ldl(V0, (c.g[A1][0] as u32).wrapping_add(7));
        c.ldr(V0, c.g[A1][0] as u32);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(8).lw(c);
        c.sdl(V0, (c.g[S2][0] as u32).wrapping_add(7));
        c.sdr(V0, c.g[S2][0] as u32);
        c.sw((c.g[S2][0] as u32).wrapping_add(8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `WorldObjFactory_AttachModifier` (0x2fae38, `checkActiveNode`).
pub(crate) const OC327_CHECK_ACTIVE_NODE: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// `MEM_copy` (0x3e6574; `call_indirect`).
pub(crate) const OC327_MEM_COPY: u32 = 0x003e_6574;

/// The generic animated-object constructor (0x3421a0; `call_indirect`).
pub(crate) const OC327_ANIM_OBJECT_CTOR_TEETER: u32 = 0x0034_21a0;

// ---------------------------------------------------------------------------
// World object factory.

/// `void WorldObjFactory_AnimTeeter(Vec3* out, s32 n, const AnimEntry* e)`
/// (0x2fb498): build a teeter modifier template (initialised once into
/// 0x4fb400), overlay the `n` entries (`{ s32 idx; s32 val; u8 pad[4]; s32
/// type; }`), resolve the object id in the template, skip when the object is
/// already flagged, else create the modifier through the object's ctor, and
/// return the fixed global (0x4c9098) into `out`.
pub fn world_obj_factory_anim_teeter(c: &mut Cpu) {
    guest(c, |c| {
        const TEMPLATE: u32 = 0x004f_b400;
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = Addr(gp.wrapping_add(0x24d8)).lw(c);
        frame.save(c, S3, 0x60);
        frame.save(c, S0, 0x90);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x80);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x50);
        c.g[A2][0] = 0x50_0000;
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x40);
        if !initialised {
            // First call: fill the global template.
            c.cycles += 26;
            c.g[V0][0] = sx(TEMPLATE);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[A0][0] = Addr(TEMPLATE.wrapping_add(0x28)).lbu(c);
            c.g[V1][0] |= 0xffff;
            c.lwc1(3, gp.wrapping_sub(0x339c));
            c.g[A1][0] = 1;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 4);
            c.g[AT][0] = 0x4270_0000;
            c.mtc1(1, 0);
            c.lwc1(1, gp.wrapping_sub(0x3398));
            c.g[AT][0] = 0xffff_ffff_c120_0000;
            c.mtc1(1, 2);
            Addr(TEMPLATE).sw(c, c.g[V1][0]);
            Addr(TEMPLATE.wrapping_add(0x28)).sb(c, c.g[A0][0]);
            c.swc1(0, TEMPLATE.wrapping_add(0x0c));
            c.swc1(1, TEMPLATE.wrapping_add(0x10));
            c.swc1(2, TEMPLATE.wrapping_add(0x14));
            c.swc1(3, TEMPLATE.wrapping_add(0x18));
            c.swc1(4, TEMPLATE.wrapping_add(0x24));
            Addr(gp.wrapping_add(0x24d8)).sw(c, c.g[A1][0]);
            c.swc1(3, TEMPLATE.wrapping_add(0x04));
            Addr(TEMPLATE.wrapping_add(0x08)).sw(c, 0);
            Addr(TEMPLATE.wrapping_add(0x1c)).sw(c, 0);
            c.swc1(4, TEMPLATE.wrapping_add(0x20));
            Addr(c.g[SP][0] as u32).sb(c, c.g[A0][0]);
        }
        // Copy the template into the stack buffer (`MEM_copy`).
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4c00));
        c.g[A2][0] = 0x2c;
        Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x002f_b558);
        c.g[S4][0] = 0x4d_0000;
        call_ptr(c, OC327_MEM_COPY, 0x002f_b558)?;
        let no_entries = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if !no_entries {
            // Overlay the entries onto the buffer.
            c.cycles += 4;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(0x0044_5ed0);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                let same = c.g[V1][0] == c.g[V0][0];
                if !same {
                    c.cycles += 2;
                }
                if same || c.g[V0][0] != c.g[A3][0] {
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
                    c.cycles += 1;
                    Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
                } else {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, c.g[A0][0] as u32);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_b570);
                }
            }
        }
        // Resolve the template id to its object.
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        let resolvable = c.g[V0][0] != 0;
        c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
        if !resolvable {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
            c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            // Charge of `s0 = obj`, after the `'b_2fb620` block: both the
            // resolved and the null-reference path pay it.
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
        }
        // Already active? Else create the modifier.
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
            let none = c.g[A0][0] == 0;
            c.g[A1][0] = 4;
            let mut skip = false;
            if !none {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002f_b640);
                c.g[A2][0] = c.g[S1][0];
                OC327_CHECK_ACTIVE_NODE.call(c, 0x002f_b640)?;
                skip = c.g[V0][0] != 0;
            }
            if !skip {
                c.cycles += 11;
                c.g[A1][0] = 0x49_0000;
                c.g[A0][0] = 0x80;
                c.g[A1][0] = sx(0x0048_9678);
                c.g[A2][0] = 0x2000_0000;
                c.g[RA][0] = sx(0x002f_b660);
                c.g[A3][0] = 0;
                call_ptr(c, OC327_EA_MEM_ALLOC_NAMED, 0x002f_b660)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x002f_b674);
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, OC327_ANIM_OBJECT_CTOR_TEETER, 0x002f_b674)?;
            }
        }
        // Return the fixed global vector into `out`.
        c.cycles += 16;
        c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6f68));
        c.ldl(2, (c.g[A1][0] as u32).wrapping_add(7));
        c.ldr(2, c.g[A1][0] as u32);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(8)).lw(c);
        c.sdl(2, (c.g[S2][0] as u32).wrapping_add(7));
        c.sdr(2, c.g[S2][0] as u32);
        Addr((c.g[S2][0] as u32).wrapping_add(8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// WScriptFn_AddHaloModifier (0x3057c0)

/// The halo-modifier parameter record built on every call (`0x4fbd68`), and
/// its static-init guard (`gp+0x2764`).
pub(crate) const HALO_TEMPLATE: u32 = 0x004f_bd68;

pub(crate) const HALO_TEMPLATE_GUARD: u32 = 0x2764;

/// The script-parameter type table the modifier loop reads: `0x440000`
/// plus this (`0x4466b0`).
pub(crate) const HALO_PARAM_TYPES_OFF: u32 = 0x66b0;

/// `cLunoVar WScriptFn_AddHaloModifier(cLunoVar* out, int n, WsParam* params)`
/// (0x3057c0): copy the halo-modifier record template (`0x4fbd68`, built on
/// the first call), apply the `n` script parameters to the copy (integers
/// copied, floats converted for type-2 entries), resolve the record's id to
/// an object, and when that object is a move node with a live halo interface
/// add the halo modifier. The record (12 bytes) is returned in `*out`.
pub fn wscript_fn_add_halo_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(HALO_TEMPLATE_GUARD)).lw(c);
        frame.save(c, S0, 0x80);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A0][0];
        let inited = c.g[V0][0] != 0;
        frame.save_ra(c, 0x40);
        if !inited {
            // First call: build the template in place. The trailing byte is
            // passed through unchanged (the copy constructor's byte).
            c.cycles += 22;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4298));
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[AT][0] = 0x3f80_0000; // 1.0
            c.mtc1(1, 0);
            c.g[V1][0] |= 0xffff;
            c.g[A2][0] = Addr(HALO_TEMPLATE.wrapping_add(0x24)).lbu(c);
            c.g[A0][0] = u64::MAX;
            c.g[AT][0] = 0x42c8_0000; // 100.0
            c.mtc1(1, 1);
            c.g[A1][0] = 1;
            Addr(HALO_TEMPLATE).sw(c, c.g[V1][0]);
            Addr(HALO_TEMPLATE.wrapping_add(0x24)).sb(c, c.g[A2][0]);
            Addr(HALO_TEMPLATE.wrapping_add(0x8)).sw(c, c.g[A0][0]);
            c.swc1(0, HALO_TEMPLATE.wrapping_add(0x18));
            c.swc1(1, HALO_TEMPLATE.wrapping_add(0x1c));
            let guard = r32(c, GP).wrapping_add(HALO_TEMPLATE_GUARD);
            Addr(guard).sw(c, c.g[A1][0]);
            Addr(HALO_TEMPLATE.wrapping_add(0x4)).sw(c, 0);
            c.swc1(0, HALO_TEMPLATE.wrapping_add(0xc));
            c.swc1(0, HALO_TEMPLATE.wrapping_add(0x10));
            c.swc1(0, HALO_TEMPLATE.wrapping_add(0x14));
            Addr(HALO_TEMPLATE.wrapping_add(0x20)).sw(c, 0);
            let spv = sp(c);
            c.sb(spv, c.g[A2][0]);
        }
        'b_3058c8: {
            // The stack copy of the record, id preset to -1.
            c.cycles += 10;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            let spv = sp(c);
            c.g[S1][0] = sx(spv.wrapping_add(0x10));
            c.g[V0][0] |= 0xffff;
            c.g[A2][0] = 0x28;
            c.g[A1][0] = sx(HALO_TEMPLATE);
            let buf = r32(c, S1);
            c.sw(buf, c.g[V0][0]);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003e_6574, 0x0030_5868)?; // EA_MemCopy
            let no_params = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if no_params {
                break 'b_3058c8;
            }
            c.cycles += 4;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(HALO_PARAM_TYPES_OFF));
            c.g[A3][0] = 2;
            loop {
                // One script parameter: `idx` selects the record word, the
                // type table decides whether a value is converted to float.
                'b_3058bc: {
                    'b_3058b8: {
                        c.cycles += 8;
                        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                        c.g[V1][0] = Addr(r32(c, A2)).off(0xc).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                            break 'b_3058b8;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                            break 'b_3058b8;
                        }
                        c.cycles += 4;
                        let val = r32(c, A2).wrapping_add(4);
                        c.lwc1(0, val);
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        break 'b_3058bc;
                    }
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                    break 'b_3058bc;
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break 'b_3058c8;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_5880);
                }
            }
        }
        // Resolve the record's id to an object (v0), -1 meaning "the
        // currently controlled object".
        'b_305924: {
            {
                c.cycles += 4;
                c.g[A2][0] = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let not_minus_one = c.g[V0][0] != 0;
                c.g[A1][0] = Addr(r32(c, GP).wrapping_add(0x16c8)).lw(c);
                if !not_minus_one {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(0xce8)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V0).wrapping_add(0x290)).lw(c);
                    break 'b_305924;
                }
            }
            {
                c.cycles += 8;
                c.g[V0][0] = c.g[A2][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = Addr(r32(c, A1)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x8)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                let no_slot = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                if !no_slot {
                    c.cycles += 7;
                    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let found = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if found {
                        break 'b_305924;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        // A live move node: call its halo-interface test and, when it says
        // yes, add the modifier.
        'b_305964: {
            'b_305960: {
                c.cycles += 2;
                let no_obj = c.g[V0][0] == 0;
                c.g[V1][0] = 0x004d_0000;
                if no_obj {
                    break 'b_305960;
                }
                c.cycles += 3;
                c.g[S0][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
                let no_node = c.g[S0][0] == 0;
                c.g[V0][0] = c.g[S3][0];
                if no_node {
                    break 'b_305964;
                }
                {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x80)).lh(c);
                    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0030_594c)?;
                    let no_interface = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !no_interface {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0035_5eb0, 0x0030_595c)?; // addHalo
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = 0x004d_0000;
                break 'b_305960;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
            break 'b_305964;
        }
        // Return the record by value: 12 bytes from the static result.
        c.cycles += 14;
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let res = r32(c, A3);
        c.ldl(4, res.wrapping_add(7));
        c.ldr(4, res);
        c.g[A1][0] = Addr(res).off(0x8).lw(c);
        let out = r32(c, S3);
        c.sdl(4, out.wrapping_add(7));
        c.sdr(4, out);
        Addr(out).off(0x8).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_AddRailModifier (0x2ff1c8)

/// The rail-modifier record template (`0x4fba28`), its static-init guard
/// (`gp+0x2540`) and its parameter type table (`0x446470`).
pub(crate) const RAIL_TEMPLATE: u32 = 0x004f_ba28;

pub(crate) const RAIL_TEMPLATE_GUARD: u32 = 0x2540;

pub(crate) const RAIL_PARAM_TYPES_OFF: u32 = 0x6470;

/// `cLunoVar WScriptFn_AddRailModifier(cLunoVar* out, int n, WsParam* params)`
/// (0x2ff1c8): same shape as `WScriptFn_AddHaloModifier` for the rail
/// modifier: copy the 16-byte template `0x4fba28` (built on the first call),
/// apply the parameters, resolve the object from the record's first word and
/// — only when the second word (the rail id) is not -1 — add the rail
/// modifier to the object's move node.
pub fn wscript_fn_add_rail_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(RAIL_TEMPLATE_GUARD)).lw(c);
        frame.save(c, S0, 0x60);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A0][0];
        let inited = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !inited {
            c.cycles += 12;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x45d8));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A2][0] = Addr(RAIL_TEMPLATE.wrapping_add(0xc)).lbu(c);
            c.g[V0][0] |= 0xffff;
            Addr(RAIL_TEMPLATE).sw(c, c.g[V0][0]);
            c.g[A0][0] = u64::MAX;
            Addr(RAIL_TEMPLATE.wrapping_add(0xc)).sb(c, c.g[A2][0]);
            c.g[A1][0] = 1;
            Addr(RAIL_TEMPLATE.wrapping_add(0x4)).sw(c, c.g[V0][0]);
            Addr(RAIL_TEMPLATE.wrapping_add(0x8)).sw(c, c.g[A0][0]);
            let guard = r32(c, GP).wrapping_add(RAIL_TEMPLATE_GUARD);
            Addr(guard).sw(c, c.g[A1][0]);
            let spv = sp(c);
            c.sb(spv, c.g[A2][0]);
        }
        'b_2ff2b0: {
            c.cycles += 11;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            let spv = sp(c);
            c.g[S1][0] = sx(spv.wrapping_add(0x10));
            c.g[V0][0] |= 0xffff;
            c.g[A2][0] = 0x10;
            c.g[A1][0] = sx(RAIL_TEMPLATE);
            c.sw(spv.wrapping_add(0x14), c.g[V0][0]);
            c.g[A0][0] = c.g[S1][0];
            c.sw(spv.wrapping_add(0x10), c.g[V0][0]);
            call_ptr(c, 0x003e_6574, 0x002f_f24c)?; // EA_MemCopy
            let no_params = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if no_params {
                break 'b_2ff2b0;
            }
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(RAIL_PARAM_TYPES_OFF));
            c.g[A3][0] = 2;
            loop {
                'b_2ff2a4: {
                    'b_2ff2a0: {
                        c.cycles += 8;
                        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                        c.g[V1][0] = Addr(r32(c, A2)).off(0xc).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                            break 'b_2ff2a0;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                            break 'b_2ff2a0;
                        }
                        c.cycles += 4;
                        let val = r32(c, A2).wrapping_add(4);
                        c.lwc1(0, val);
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        break 'b_2ff2a4;
                    }
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                    break 'b_2ff2a4;
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break 'b_2ff2b0;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_f268);
                }
            }
        }
        // Resolve the record's id (word 0) to an object; the result is v1.
        'b_2ff310: {
            {
                c.cycles += 4;
                c.g[A2][0] = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let not_minus_one = c.g[V0][0] != 0;
                c.g[A1][0] = Addr(r32(c, GP).wrapping_add(0x16c8)).lw(c);
                if !not_minus_one {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(0xce8)).lw(c);
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x290)).lw(c);
                    break 'b_2ff310;
                }
            }
            'b_2ff30c: {
                {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x8)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    let no_slot = c.g[V1][0] == 0;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                    if !no_slot {
                        c.cycles += 7;
                        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                        let found = c.g[V0][0] != 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if found {
                            break 'b_2ff30c;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
        }
        // The rail id (word 1) must be set, the node alive and its rail
        // interface present.
        'b_2ff354: {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S1)).off(0x4).lw(c);
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            if c.g[V0][0] == 0 {
                c.g[V1][0] = 0x004d_0000;
                break 'b_2ff354;
            }
            c.cycles += 3;
            c.g[S0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
            let no_node = c.g[S0][0] == 0;
            c.g[V1][0] = 0x004d_0000;
            if no_node {
                break 'b_2ff354;
            }
            {
                c.cycles += 7;
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x80)).lh(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x002f_f340)?;
                let no_interface = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !no_interface {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0035_5e38, 0x002f_f350)?; // addRail
                }
            }
            c.cycles += 1;
            c.g[V1][0] = 0x004d_0000;
            break 'b_2ff354;
        }
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let res = r32(c, A3);
        c.ldl(4, res.wrapping_add(7));
        c.ldr(4, res);
        c.g[A1][0] = Addr(res).off(0x8).lw(c);
        let out = r32(c, S3);
        c.sdl(4, out.wrapping_add(7));
        c.sdr(4, out);
        Addr(out).off(0x8).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObjFactory_ParticleNode (0x2fd420, worldobjfactory.cpp)

/// The particle-node defaults: the guard word (`gp + 0x2514` = 0x4a5604)
/// and the 0xe0-byte template (0x500000 - 0x49c0).
pub(crate) const WOFPN_INIT_GUARD: u32 = 0x2514;

pub(crate) const WOFPN_TEMPLATE: u32 = 0x004f_b640;

/// Parameter types by index (2: float).
pub(crate) const WOFPN_PARAM_TYPES: u32 = 0x0044_60b0;

/// The world object table pointer (`gp + 0x16c8`) and the default world
/// object holder (`gp + 0xce8`, object at +0x290).
pub(crate) const WOFPN_OBJECT_TABLE: u32 = 0x16c8;

pub(crate) const WOFPN_DEFAULT_HOLDER: u32 = 0xce8;

/// The factory's 12-byte "done" result (0x4d0000 - 0x6f68).
pub(crate) const WOFPN_RESULT: u32 = 0x004c_9098;

/// Node fields: the object id word (+0xd8, -1: the default object), the
/// lifetime (+0x1c) clamped to the max (+0x14).
pub(crate) const WOFPN_NODE_OBJECT: u32 = 0xd8;

pub(crate) const WOFPN_ALLOC: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};

pub(crate) const WOFPN_DELETE: Callee = Callee {
    addr: 0x0031_7e50,
    f: ee::s10::__builtin_delete,
};

pub(crate) const WOFPN_ATTACH_MODIFIER: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// `memcpy` and the particle-node ctor (through the table).
pub(crate) const WOFPN_MEMCPY: u32 = 0x003e_6574;

pub(crate) const WOFPN_NODE_CTOR: u32 = 0x0035_78a8;

/// The guarded static init of the particle-node template.
pub(crate) fn wofpn_init_template(c: &mut Cpu) {
    c.cycles += 72;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 1); // -1.0
    c.g[V0][0] = sx(WOFPN_TEMPLATE);
    c.g[AT][0] = 0x4080_0000;
    c.mtc1(AT, 2); // 4.0
    c.g[A0][0] = 1;
    c.mtc1(0, 0); // 0.0
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3); // 1.0
    c.g[A2][0] = 0x10;
    let t = WOFPN_TEMPLATE;
    c.swc1(1, t + 0x8);
    c.g[V1][0] |= 0xffff;
    c.swc1(2, t + 0x10);
    c.sw(t, c.g[A0][0]);
    c.sw(t + 0x4, 0);
    c.swc1(3, t + 0xc);
    for off in (0x14..=0x80).step_by(4) {
        c.swc1(0, t + off);
    }
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(AT, 1); // 20.0
    c.swc1(3, t + 0x84);
    c.g[A1][0] = c.lbu(t + 0xdc);
    c.swc1(3, t + 0xb4);
    c.sw(t + 0xc4, c.g[A2][0]);
    c.swc1(0, t + 0xcc);
    c.swc1(1, t + 0xd4);
    c.sw(t + 0xd8, c.g[V1][0]);
    let guard = r32(c, GP).wrapping_add(WOFPN_INIT_GUARD);
    c.sw(guard, c.g[A0][0]);
    for off in [0x88, 0x8c, 0x90] {
        c.swc1(0, t + off);
    }
    for off in [0x94, 0x98, 0x9c, 0xa0, 0xa4] {
        c.swc1(3, t + off);
    }
    for off in [0xa8, 0xac, 0xb0, 0xb8, 0xbc, 0xc0] {
        c.swc1(0, t + off);
    }
    c.sw(t + 0xc8, 0);
    c.sw(t + 0xd0, c.g[A0][0]);
    c.sb(sp(c), c.g[A1][0]);
}

/// `Result WorldObjFactory_ParticleNode(int n, Param* params)` (0x2fd420;
/// the 12-byte result through the hidden pointer in a0): allocates a
/// particle node from the default template, applies the `n` params
/// (16 bytes each: index, value, -, type; an int given for a float slot
/// is converted), looks up its world object (by id, or the default one),
/// and unless the object's modifier refuses it (AttachModifier: > 0 frees
/// the node, < 0 keeps it) clamps the lifetime and creates the particle
/// node on the object. A missing object frees the node.
pub fn world_obj_factory_particle_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        let guard = r32(c, GP).wrapping_add(WOFPN_INIT_GUARD);
        c.g[V0][0] = c.lw(guard);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = 0x500000;
        frame.save(c, S4, 0x30);
        c.g[S2][0] = c.g[A0][0]; // result
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A1][0]; // n
        frame.save(c, S0, 0x70);
        c.g[S5][0] = c.g[A2][0]; // params
        frame.save(c, S3, 0x40);
        frame.save_ra(c, 0x10);
        if c.g[V0][0] == 0 {
            wofpn_init_template(c);
        }

        // node = alloc(0xe0); node->object = -1; memcpy(node, template, 0xe0)
        c.cycles += 17;
        c.g[A0][0] = 0xe0;
        c.g[A1][0] = sx(0x0049_0000 - 0x68e8);
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[S3][0] = 0x4d0000;
        WOFPN_ALLOC.call(c, 0x002f_d594)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = 0xe0;
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = sx(WOFPN_TEMPLATE);
        c.g[A0][0] = c.g[S0][0];
        c.sw(r32(c, S0).wrapping_add(WOFPN_NODE_OBJECT), c.g[V0][0]);
        call_ptr(c, WOFPN_MEMCPY, 0x002f_d5b4)?;
        c.g[A2][0] = c.g[S5][0];
        if (c.g[S4][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x440000;
            c.g[A1][0] = c.g[S4][0];
            c.g[T0][0] = sx(WOFPN_PARAM_TYPES);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                let param = r32(c, A2);
                c.g[V0][0] = c.lw(param);
                c.g[V1][0] = c.lw(param.wrapping_add(0xc));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                c.g[V0][0] = c.lw(r32(c, V0));
                let convert = c.g[V1][0] != c.g[V0][0] && {
                    c.cycles += 2;
                    c.g[V0][0] == c.g[A3][0]
                };
                if convert {
                    c.cycles += 4;
                    c.lwc1(0, param.wrapping_add(4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, r32(c, A0));
                } else {
                    c.g[V0][0] = c.lw(param.wrapping_add(4));
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx(param.wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_d5d0);
                }
            }
        }

        // obj = id == -1 ? default object : table[id & 0xff] entry id >> 8
        c.cycles += 4;
        c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(WOFPN_NODE_OBJECT));
        c.g[V0][0] = !c.g[A2][0];
        let gp = r32(c, GP);
        c.g[A1][0] = c.lw(gp.wrapping_add(WOFPN_OBJECT_TABLE));
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(gp.wrapping_add(WOFPN_DEFAULT_HOLDER));
            c.g[S1][0] = c.lw(r32(c, V0).wrapping_add(0x290));
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = c.lw(r32(c, A1));
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(8));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = c.lw(r32(c, V0));
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                c.cycles += 7;
                c.g[V0][0] = sx(r32(c, A2) >> 8);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                c.g[V1][0] = c.lw(r32(c, V0));
                c.g[V0][0] = sx(r32(c, V1) >> 8);
                let some = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, V0) << 2);
                if !some {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            c.cycles += 1;
            c.g[S1][0] = c.g[A2][0];
        }

        'done: {
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                WOFPN_DELETE.call(c, 0x002f_d694)?;
                break 'done;
            }
            c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0xc));
            c.cycles += 2;
            c.g[A1][0] = 0xd;
            if c.g[A0][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                WOFPN_ATTACH_MODIFIER.call(c, 0x002f_d6ac)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    if (c.g[V0][0] as i64) > 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        WOFPN_DELETE.call(c, 0x002f_d6c4)?;
                    }
                    break 'done;
                }
            } else {
                c.cycles += 1;
            }
            // lifetime = min(lifetime, max)
            let node = r32(c, S0);
            c.lwc1(0, node.wrapping_add(0x1c));
            c.cycles += 5;
            c.lwc1(1, node.wrapping_add(0x14));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, node.wrapping_add(0x1c));
            }
            c.cycles += 12;
            c.g[A0][0] = 0x2c;
            c.g[A1][0] = sx(0x0049_0000 - 0x68d0);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            WOFPN_ALLOC.call(c, 0x002f_d6fc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[T0][0] = 0;
            call_ptr(c, WOFPN_NODE_CTOR, 0x002f_d714)?;
        }

        // *result = factory "done" result
        c.cycles += 17;
        c.g[A1][0] = sx(WOFPN_RESULT); // s3 (0x4d0000) - 0x6f68
        let (src, dst) = (r32(c, A1), r32(c, S2));
        c.ldl(V0, src.wrapping_add(7));
        c.ldr(V0, src);
        c.g[V1][0] = c.lw(src.wrapping_add(8));
        c.sdl(V0, dst.wrapping_add(7));
        c.sdr(V0, dst);
        c.sw(dst.wrapping_add(8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_GetObjectSlot30 (0x301680)

/// `gp+0x25e0`: the shared 8-byte id buffer (`D_1680`).
pub(crate) const WSCRIPT_GOS30_IDS: u32 = 0x25e0;

/// `gp+0x25e4`: its tag byte (copied to the stack on the first call).
pub(crate) const WSCRIPT_GOS30_TAG: u32 = 0x25e4;

/// `gp+0x25e8`: the one-time initialisation flag.
pub(crate) const WSCRIPT_GOS30_INIT: u32 = 0x25e8;

/// `gp+0xb98`: the script value type of each slot.
pub(crate) const WSCRIPT_GOS30_TYPES: u32 = 0xb98;

/// `gp+0x16c8`: the id table head; `gp+0xce8`: the world (`+0x290` is the
/// object of id -1).
pub(crate) const WSCRIPT_GOS30_TABLE: u32 = 0x16c8;

pub(crate) const WSCRIPT_GOS30_WORLD: u32 = 0xce8;

/// `cLunoVar* WScriptFn_GetObjectSlot30(cLunoVar* ret, int n, Ent* e)`
/// (0x301680): the script bridge's float query: copy the shared id buffer,
/// overlay the `n` argument entries (as `WScriptFn_ObjectExists`), look the
/// first id up and return (type 2, a float) the object's `+0xc` component's
/// vtable slot 0xf0 result, or -1 when there is no object or component.
pub fn wscript_fn_get_object_slot30(c: &mut Cpu) {
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(gp.wrapping_add(WSCRIPT_GOS30_INIT)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0]; // n
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0]; // ret
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A2][0]; // entries
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 20, 0x70);
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(gp.wrapping_add(WSCRIPT_GOS30_TAG)).lbu(c);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            Addr(gp.wrapping_add(WSCRIPT_GOS30_IDS)).sw(c, c.g[V1][0]);
            Addr(gp.wrapping_add(WSCRIPT_GOS30_INIT)).sw(c, 1);
            Addr(sp(c)).sb(c, c.g[V0][0]);
        }
        // `buf[0] = -1; memcpy(buf, D_1680, 8)` and the entry overlay.
        'overlay: {
            c.cycles += 10;
            c.g[V0][0] = u64::MAX;
            c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
            c.g[A2][0] = 8;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(gp.wrapping_add(WSCRIPT_GOS30_IDS));
            Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x003e_6574, 0x0030_16ec)?;
            let no_entries = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S3][0];
            if no_entries {
                break 'overlay;
            }
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(gp.wrapping_add(WSCRIPT_GOS30_TYPES));
            c.g[A3][0] = 2; // the script type of a float slot
            loop {
                'store: {
                    'value: {
                        c.cycles += 8;
                        c.g[V0][0] = Addr(r32(c, A2)).lw(c); // the slot
                        c.g[V1][0] = Addr(oc391_at(c, A2, 0xc)).lw(c); // type
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = Addr(r32(c, V0)).lw(c); // the slot's type
                        let raw = c.g[V1][0] == c.g[V0][0] || {
                            c.cycles += 2;
                            c.g[V0][0] != c.g[A3][0]
                        };
                        if raw {
                            c.g[V0][0] = Addr(oc391_at(c, A2, 4)).lw(c);
                            break 'value;
                        }
                        // A float value: `*(float*)d = (float)x->val`.
                        c.cycles += 4;
                        c.lwc1(0, oc391_at(c, A2, 4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        break 'store;
                    }
                    c.cycles += 1;
                    Addr(r32(c, A0)).sw(c, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if !more {
                    break 'overlay;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_1700);
                }
            }
        }
        // Look the first id up: -1 is the world's object, else the low byte
        // picks a descriptor and the next bits its entry.
        c.cycles += 4;
        c.g[A2][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        c.g[A1][0] = Addr(gp.wrapping_add(WSCRIPT_GOS30_TABLE)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp.wrapping_add(WSCRIPT_GOS30_WORLD)).lw(c);
            c.g[V0][0] = Addr(oc391_at(c, V0, 0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = sx(((c.g[A2][0] & 0xff) as u32) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(oc391_at(c, V1, 8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = sx(r32(c, A2) >> 8);
            let mut found = false;
            if c.g[V1][0] != 0 {
                c.cycles += 7;
                c.g[A0][0] = Addr(oc391_at(c, V1, 0x1c)).lw(c);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.g[V0][0] = sx(r32(c, V1) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, V0) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // The value: the component's slot 0xf0, or -1.
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 2;
            c.cycles += 4;
            c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
            c.mtc1(1, 20);
            Addr(oc391_at(c, S2, 8)).sw(c, c.g[V0][0]);
            c.g[A0][0] = c.g[S2][0];
        } else {
            c.cycles += 3;
            c.g[A1][0] = Addr(oc391_at(c, V0, 0xc)).lw(c);
            if c.g[A1][0] == 0 {
                c.cycles += 2;
                c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
                c.mtc1(1, 20);
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr(oc391_at(c, A1, 0xc)).lw(c);
                c.g[A0][0] = Addr(oc391_at(c, V0, 0xf0)).lh(c);
                c.g[V1][0] = Addr(oc391_at(c, V0, 0xf4)).lw(c);
                let func = r32(c, V1);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x0030_17cc)?;
                c.mov_s(20, 0);
            }
            c.cycles += 4;
            c.g[V0][0] = 2;
            c.g[A0][0] = c.g[S2][0];
            Addr(oc391_at(c, S2, 8)).sw(c, 2);
        }
        c.cycles += 12;
        call_ptr(c, 0x0022_6618, 0x0030_1804)?; // LunoVMRegister_AsFloat
        c.swc1(20, r32(c, V0));
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_CreateFlag (0x002fc9c8)

/// `WorldObjFactory_AttachModifier` (0x002fae38).
pub(crate) const WORLD_OBJ_FACTORY_ATTACH_MODIFIER_CALL: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// `EA_MemAllocNamed` (0x00317d70).
pub(crate) const EA_MEM_ALLOC_NAMED_CALL: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};

/// `WScriptFn::CreateFlag(entries, n)` (0x2fc9c8): build the 108-byte flag template (static, initialised once), apply the `n` (idx, type, value) overrides (floats converted when the slot is a float), resolve the target object (the world object of the id in the template, or the default one), and unless it already has an active node of kind 10, allocate a 0x1c-byte flag node on it. Returns a constant.
pub fn w_script_fn_create_flag(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xe0 };
        'b_2fca98: {
            c.cycles += 13;
            Frame::open(c, 0xe0);
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(0x2500));
            frame.save(c, S4, 0x90);
            frame.save(c, S0, 0xd0);
            c.g[S4][0] = c.g[A2][0];
            frame.save(c, S2, 0xb0);
            c.g[S0][0] = c.g[A1][0];
            frame.save(c, S1, 0xc0);
            c.g[S2][0] = c.g[A0][0];
            frame.save(c, S3, 0xa0);
            c.g[A2][0] = 0x500000;
            let t = c.g[V0][0] != 0u64;
            frame.save_ra(c, 0x80);
            if t {
                break 'b_2fca98;
            } else {
                c.cycles += 39;
                c.g[V0][0] = sx(r32(c, A2).wrapping_sub(0x4ac0));
                c.g[V1][0] = 0xffffffffffff0000;
                c.mtc1(0, 1);
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.g[A0][0] = 0x1;
                c.g[A1][0] = c.lbu(r32(c, V0).wrapping_add(0x68));
                c.g[AT][0] = 0x41c8_0000;
                c.mtc1(1, 2);
                c.sw(r32(c, A2).wrapping_sub(0x4ac0), c.g[V1][0]);
                c.sb(r32(c, V0).wrapping_add(0x68), c.g[A1][0]);
                c.swc1(2, r32(c, V0).wrapping_add(0x14));
                c.swc1(0, r32(c, V0).wrapping_add(0x3c));
                c.swc1(1, r32(c, V0).wrapping_add(0x60));
                c.sw(r32(c, GP).wrapping_add(0x2500), c.g[A0][0]);
                c.sw(r32(c, V0).wrapping_add(0x4), c.g[A0][0]);
                c.sw(r32(c, V0).wrapping_add(0x8), 0u64);
                c.sw(r32(c, V0).wrapping_add(0xc), 0u64);
                c.swc1(0, r32(c, V0).wrapping_add(0x10));
                c.swc1(0, r32(c, V0).wrapping_add(0x18));
                c.swc1(0, r32(c, V0).wrapping_add(0x1c));
                c.swc1(1, r32(c, V0).wrapping_add(0x20));
                c.swc1(0, r32(c, V0).wrapping_add(0x24));
                c.swc1(0, r32(c, V0).wrapping_add(0x28));
                c.swc1(1, r32(c, V0).wrapping_add(0x2c));
                c.swc1(0, r32(c, V0).wrapping_add(0x30));
                c.swc1(0, r32(c, V0).wrapping_add(0x34));
                c.swc1(1, r32(c, V0).wrapping_add(0x38));
                c.swc1(1, r32(c, V0).wrapping_add(0x40));
                c.swc1(1, r32(c, V0).wrapping_add(0x44));
                c.swc1(1, r32(c, V0).wrapping_add(0x48));
                c.swc1(1, r32(c, V0).wrapping_add(0x4c));
                c.swc1(1, r32(c, V0).wrapping_add(0x50));
                c.swc1(1, r32(c, V0).wrapping_add(0x54));
                c.swc1(1, r32(c, V0).wrapping_add(0x58));
                c.swc1(1, r32(c, V0).wrapping_add(0x5c));
                c.sw(r32(c, V0).wrapping_add(0x64), 0u64);
                c.sb(r32(c, SP), c.g[A1][0]);
                break 'b_2fca98;
            }
        }
        'b_2fcb20: {
            c.cycles += 11;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[S1][0] = sx(r32(c, SP).wrapping_add(0x10));
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.g[A1][0] = sx(r32(c, A2).wrapping_sub(0x4ac0));
            c.g[A2][0] = 0x6c;
            c.sw(r32(c, SP).wrapping_add(0x10), c.g[V0][0]);
            c.g[A0][0] = c.g[S1][0];
            c.g[S3][0] = 0x4d0000;
            call_ptr(c, 0x003e_6574, 0x002f_cabc)?;
            let t = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = c.g[S4][0];
            if t {
                break 'b_2fcb20;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = c.g[S0][0];
                c.g[T0][0] = sx(r32(c, V0).wrapping_add(0x5fc0));
                c.g[A3][0] = 0x2;
                'l_2fcad8: loop {
                    'b_2fcb14: {
                        'b_2fcb10: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(r32(c, A2));
                            c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0xc));
                            c.g[V0][0] = sx(r32(c, V0) << 2);
                            c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                            c.g[V0][0] = c.lw(r32(c, V0));
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x4));
                                break 'b_2fcb10;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x4));
                                    break 'b_2fcb10;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, r32(c, A2).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, r32(c, A0));
                                    if t {
                                        break 'b_2fcb14;
                                    } else {
                                        break 'b_2fcb10;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(r32(c, A0), c.g[V0][0]);
                        break 'b_2fcb14;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002f_cad8);
                        }
                        continue 'l_2fcad8;
                    } else {
                        break 'b_2fcb20;
                    }
                }
            }
        }
        'b_2fcb8c: {
            'b_2fcb3c: {
                c.cycles += 4;
                c.g[A2][0] = c.lw(r32(c, S1));
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = c.lw(r32(c, GP).wrapping_add(0x16c8));
                if t {
                    break 'b_2fcb3c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(0xce8));
                    let t = true;
                    c.g[S0][0] = c.lw(r32(c, V0).wrapping_add(0x290));
                    if t {
                        break 'b_2fcb8c;
                    } else {
                        break 'b_2fcb3c;
                    }
                }
            }
            'b_2fcb88: {
                'b_2fcb64: {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V1][0] = c.lw(r32(c, A1));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    if c.g[V1][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                        break 'b_2fcb64;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[A2][0] = 0u64;
                        if t {
                            break 'b_2fcb88;
                        } else {
                            break 'b_2fcb64;
                        }
                    }
                }
                'b_2fcb84: {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r32(c, A2) >> 8);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.g[V0][0] = sx(r32(c, V1) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if t {
                        break 'b_2fcb84;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0u64;
                        break 'b_2fcb84;
                    }
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
                break 'b_2fcb88;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
            break 'b_2fcb8c;
        }
        'b_2fcbdc: {
            c.cycles += 2;
            let t = c.g[S0][0] == 0u64;

            if t {
                break 'b_2fcbdc;
            } else {
                'b_2fcbb0: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0xc));
                    let t = c.g[A0][0] == 0u64;
                    c.g[A1][0] = 0xa;
                    if t {
                        break 'b_2fcbb0;
                    } else {
                        c.cycles += 4;
                        c.g[A2][0] = c.g[S1][0];
                        WORLD_OBJ_FACTORY_ATTACH_MODIFIER_CALL.call(c, 0x002f_cba8)?;
                        let t = c.g[V0][0] != 0u64;

                        if t {
                            break 'b_2fcbdc;
                        } else {
                            break 'b_2fcbb0;
                        }
                    }
                }
                c.cycles += 11;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = 0x1c;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x3c08));
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0u64;
                EA_MEM_ALLOC_NAMED_CALL.call(c, 0x002f_cbc8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0034_add8, 0x002f_cbdc)?;
                break 'b_2fcbdc;
            }
        }
        c.cycles += 16;
        c.g[A1][0] = sx(r32(c, S3).wrapping_sub(0x6f68));
        c.ldl(2, r32(c, A1).wrapping_add(0x7));
        c.ldr(2, r32(c, A1));
        c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x8));
        c.sdl(2, r32(c, S2).wrapping_add(0x7));
        c.sdr(2, r32(c, S2));
        c.sw(r32(c, S2).wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `Result* WScriptFn_IsEventType(Result* ret, int n, Arg* e)` (0x302778):
/// bind the script call's arguments into a 2-word record (defaults
/// `{-1, ...}` set once), convert the float ones, store `IsEventType(v[0])`
/// as the integer result and return `ret`.
pub fn w_script_fn_is_event_type(c: &mut Cpu) {
    const W_SCRIPT_FN_EVENT_TYPE_MATCHES: Callee = Callee {
        addr: 0x00303df8,
        f: ee::s10::f_00303df8,
    };
    guest(c, |c| {
        'b_3027bc: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2660));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A2][0];
            let t = c.g[V0][0] != 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            if t {
                break 'b_3027bc;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x265c));
                c.g[V1][0] = 0xffffffffffffffff;
                c.g[A0][0] = 0x1;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2658), c.g[V1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2660), c.g[A0][0]);
                c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                break 'b_3027bc;
            }
        }
        'b_302830: {
            c.cycles += 7;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A2][0] = 0x8;
            c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2658));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e6574, 0x003027d0)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S3][0];
            if t {
                break 'b_302830;
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(0xbe8));
                c.g[A3][0] = 0x2;
                'l_3027e8: loop {
                    'b_302824: {
                        'b_302820: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_302820;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_302820;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, c.g[A0][0] as u32);
                                    if t {
                                        break 'b_302824;
                                    } else {
                                        break 'b_302820;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        break 'b_302824;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003027e8);
                        }
                        continue 'l_3027e8;
                    } else {
                        break 'b_302830;
                    }
                }
            }
        }
        c.cycles += 16;
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        W_SCRIPT_FN_EVENT_TYPE_MATCHES.call(c, 0x00302838)?;
        c.g[V1][0] = 0x1;
        c.g[S0][0] = c.g[V0][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x00226610, 0x0030284c)?;
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
        c.g[V0][0] = c.g[S2][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Small register-relative accessors (`lw off(reg)` and friends). They read the
// base register themselves, so the call never borrows `c` twice.

/// `lw dst, off(reg)` (sign-extended).
#[inline]
pub(crate) fn oc417_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).lw(c)
}

/// `lh dst, off(reg)` (sign-extended).
#[inline]
pub(crate) fn oc417_lh(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).lh(c)
}

/// `lb dst, off(reg)` (sign-extended).
#[inline]
pub(crate) fn oc417_lb(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).lb(c)
}

/// `lbu dst, off(reg)`.
#[inline]
pub(crate) fn oc417_lbu(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).lbu(c)
}

/// `sw src, off(reg)`.
#[inline]
pub(crate) fn oc417_sw(c: &mut Cpu, reg: usize, off: u32, src: usize) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    let v = c.g[src][0];
    Addr(a).sw(c, v);
}

/// `sw zero, off(reg)`.
#[inline]
pub(crate) fn oc417_sw0(c: &mut Cpu, reg: usize, off: u32) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).sw(c, 0);
}

/// `addu` of two registers' low words, sign-extended.
#[inline]
pub(crate) fn oc417_add(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

// ---------------------------------------------------------------------------
// WScriptFn_EventTypeMatches (0x303df8)

/// `int WScriptFn_EventTypeMatches(unsigned i)` (0x303df8): is the world
/// script's event type `i` (below 8) equal to the current event byte
/// (`*0x535c10`)? Looks the BE interface up first (result unused), then
/// compares `table[i]` (`0x4465f8`) with the byte. 0 for `i >= 8`.
pub fn wscript_fn_event_type_matches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = (c.g[S0][0] < 8) as u64;
        let in_range = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if in_range {
            c.cycles += 14;
            call_ptr(c, 0x0014_dc80, 0x0030_3e20)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_dd58, 0x0030_3e2c)?;
            c.g[V1][0] = 0x44_0000;
            c.g[V0][0] = 0x53_0000;
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[V1][0] = oc417_add(c.g[V1][0], 0x65f8);
            c.g[A1][0] = oc417_lb(c, V0, 0x5c10);
            c.g[A0][0] = oc417_add(c.g[A0][0], c.g[V1][0]);
            c.g[V0][0] = oc417_lw(c, A0, 0);
            c.g[V0][0] ^= c.g[A1][0];
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_MultiParticleControl (0x306cb0) and WScriptFn_AddMagnetModifier
// (0x305478): the same world-script argument decoding.

/// The `MEM_copy` (0x3e6574) both functions use to copy the default record.
pub(crate) const WSCRIPT_MEM_COPY: u32 = 0x003e_6574;

/// `WScriptFn_MultiParticleControl(Rec* result, int n, Ent* entries)`
/// (0x306cb0): on first use initialise the static default record
/// (`0x4fc050`: object id -1 ...), copy it to a 16-byte stack record, apply
/// the `n` 16-byte `{index, value, type, ..}` argument entries to the
/// record's words (an int argument for a float parameter is converted with
/// `cvt.s.w`), resolve the object id (`rec[1]`, -1 = the current object at
/// `world+0x290`, else the world cache's slot table) and, when found, call
/// `MultiParticle` add (`rec[2] == 0`, `0x357ae8`) or remove (`0x357b10`)
/// with the world's particle manager (`gp-0xf38 slot`, `rec[0]`) and the
/// object's `+0x78`. Copies the 12-byte default result (`0x4c9098`) to
/// `*result` and returns `result`.
pub fn wscript_fn_multi_particle_control(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = oc417_lw(c, GP, 0x27c8);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x50_0000;
        if !initialised {
            c.cycles += 11;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3fb0));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = oc417_lbu(c, V1, 0xc);
            c.g[V0][0] |= 0xffff;
            Addr((c.g[A2][0] as u32).wrapping_sub(0x3fb0)).sw(c, 0);
            c.g[A0][0] = 1;
            let a = (c.g[V1][0] as u32).wrapping_add(0xc);
            let v = c.g[A1][0];
            Addr(a).sb(c, v);
            oc417_sw(c, V1, 4, V0);
            oc417_sw(c, GP, 0x27c8, A0);
            oc417_sw0(c, V1, 8);
            let a = sp(c);
            let v = c.g[A1][0];
            Addr(a).sb(c, v);
        }
        c.cycles += 10;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S0][0] = oc417_add(c.g[SP][0], 0x10);
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3fb0));
        c.g[A2][0] = 0x10;
        oc417_sw(c, SP, 0x14, V0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, WSCRIPT_MEM_COPY, 0x0030_6d2c)?;
        let no_entries = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !no_entries {
            c.cycles += 5;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = oc417_add(c.g[V0][0], 0x6970);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = oc417_lw(c, A2, 0);
                c.g[V1][0] = oc417_lw(c, A2, 0xc);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = oc417_add(c.g[V0][0], c.g[S0][0]);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[T0][0]);
                c.g[V0][0] = oc417_lw(c, V0, 0);
                // `entry.type != paramType && paramType == 2`: int -> float.
                let convert = if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = oc417_lw(c, A2, 4);
                    false
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = oc417_lw(c, A2, 4);
                        false
                    } else {
                        c.cycles += 4;
                        let a = (c.g[A2][0] as u32).wrapping_add(4);
                        c.lwc1(0, a);
                        c.cvt_s_w(0, 0);
                        let a = c.g[A0][0] as u32;
                        c.swc1(0, a);
                        true
                    }
                };
                if !convert {
                    c.cycles += 1;
                    oc417_sw(c, A0, 0, V0);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = oc417_add(c.g[A2][0], 0x10);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_6d48);
                    }
                    continue;
                }
                break;
            }
        }
        // Resolve the object (rec[1]).
        c.cycles += 4;
        c.g[A2][0] = oc417_lw(c, S0, 4);
        c.g[V0][0] = !(c.g[A2][0]);
        let looked_up = c.g[V0][0] != 0;
        c.g[A1][0] = oc417_lw(c, GP, 0x16c8);
        if !looked_up {
            c.cycles += 3;
            c.g[V0][0] = oc417_lw(c, GP, 0xce8);
            c.g[V1][0] = oc417_lw(c, V0, 0x290);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = oc417_lw(c, A1, 0);
            c.g[A0][0] = oc417_lw(c, V1, 8);
            c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A0][0]);
            c.g[V1][0] = oc417_lw(c, V0, 0);
            let no_slot = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            let mut found = false;
            if !no_slot {
                c.cycles += 7;
                c.g[A0][0] = oc417_lw(c, V1, 0x1c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = oc417_lw(c, V0, 0);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
        }
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.g[V1][0] = 0x4d_0000;
        } else {
            let remove_instead = {
                c.cycles += 3;
                c.g[V0][0] = oc417_lw(c, S0, 8);
                let r = c.g[V0][0] != 0;
                c.g[A1][0] = oc417_lw(c, S0, 0);
                r
            };
            if !remove_instead {
                c.cycles += 5;
                c.g[A2][0] = oc417_lw(c, V1, 0x78);
                c.g[A0][0] = oc417_lw(c, GP, 0xf38);
                call_ptr(c, 0x0035_7ae8, 0x0030_6e10)?;
                c.g[V1][0] = 0x4d_0000;
            } else {
                c.cycles += 4;
                c.g[A2][0] = oc417_lw(c, V1, 0x78);
                c.g[A0][0] = oc417_lw(c, GP, 0xf38);
                call_ptr(c, 0x0035_7b10, 0x0030_6e24)?;
                c.g[V1][0] = 0x4d_0000;
            }
        }
        // Copy the default result record to *result.
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let a = (c.g[A3][0] as u32).wrapping_add(7);
        c.ldl(4, a);
        let a = c.g[A3][0] as u32;
        c.ldr(4, a);
        c.g[A1][0] = oc417_lw(c, A3, 8);
        let a = (c.g[S3][0] as u32).wrapping_add(7);
        c.sdl(4, a);
        let a = c.g[S3][0] as u32;
        c.sdr(4, a);
        oc417_sw(c, S3, 8, A1);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptFn_AddMagnetModifier(Rec* result, int n, Ent* entries)`
/// (0x305478): like `WScriptFn_MultiParticleControl` but with the static
/// default record at `0x4fbd58` (object id -1, strength 400.0, radius 30.0)
/// and the parameter types at `0x4466a0`. When the resolved object has a
/// move node (`obj+0xc`) whose virtual `0x80` says it accepts magnets, adds
/// a magnet to it (`0x3559f8`: node, the node's virtual `0xc0` result,
/// strength, radius). Returns `result` with the default result record
/// (`0x4c9098`) copied in.
pub fn wscript_fn_add_magnet_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = oc417_lw(c, GP, 0x2750);
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x50_0000;
        if !initialised {
            c.cycles += 15;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42a8));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = oc417_lbu(c, V1, 0xc);
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x43c8_0000;
            c.mtc1(1, 1);
            c.g[A0][0] = 1;
            c.g[AT][0] = 0x41f0_0000;
            c.mtc1(1, 0);
            let a = (c.g[A2][0] as u32).wrapping_sub(0x42a8);
            let v = c.g[V0][0];
            Addr(a).sw(c, v);
            let a = (c.g[V1][0] as u32).wrapping_add(0xc);
            let v = c.g[A1][0];
            Addr(a).sb(c, v);
            let a = (c.g[V1][0] as u32).wrapping_add(4);
            c.swc1(1, a);
            let a = (c.g[V1][0] as u32).wrapping_add(8);
            c.swc1(0, a);
            oc417_sw(c, GP, 0x2750, A0);
            let a = sp(c);
            let v = c.g[A1][0];
            Addr(a).sb(c, v);
        }
        c.cycles += 10;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = oc417_add(c.g[SP][0], 0x10);
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42a8));
        c.g[A2][0] = 0x10;
        oc417_sw(c, SP, 0x10, V0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, WSCRIPT_MEM_COPY, 0x0030_5504)?;
        let no_entries = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !no_entries {
            c.cycles += 5;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = oc417_add(c.g[V0][0], 0x66a0);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = oc417_lw(c, A2, 0);
                c.g[V1][0] = oc417_lw(c, A2, 0xc);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = oc417_add(c.g[V0][0], c.g[S1][0]);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[T0][0]);
                c.g[V0][0] = oc417_lw(c, V0, 0);
                let convert = if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = oc417_lw(c, A2, 4);
                    false
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = oc417_lw(c, A2, 4);
                        false
                    } else {
                        c.cycles += 4;
                        let a = (c.g[A2][0] as u32).wrapping_add(4);
                        c.lwc1(0, a);
                        c.cvt_s_w(0, 0);
                        let a = c.g[A0][0] as u32;
                        c.swc1(0, a);
                        true
                    }
                };
                if !convert {
                    c.cycles += 1;
                    oc417_sw(c, A0, 0, V0);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = oc417_add(c.g[A2][0], 0x10);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_5520);
                    }
                    continue;
                }
                break;
            }
        }
        // Resolve the object (rec[0]).
        c.cycles += 4;
        c.g[A2][0] = oc417_lw(c, S1, 0);
        c.g[V0][0] = !(c.g[A2][0]);
        let looked_up = c.g[V0][0] != 0;
        c.g[A1][0] = oc417_lw(c, GP, 0x16c8);
        if !looked_up {
            c.cycles += 3;
            c.g[V0][0] = oc417_lw(c, GP, 0xce8);
            c.g[V0][0] = oc417_lw(c, V0, 0x290);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = oc417_lw(c, A1, 0);
            c.g[A0][0] = oc417_lw(c, V1, 8);
            c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A0][0]);
            c.g[V1][0] = oc417_lw(c, V0, 0);
            let no_slot = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            let mut found = false;
            if !no_slot {
                c.cycles += 7;
                c.g[A0][0] = oc417_lw(c, V1, 0x1c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = oc417_lw(c, V0, 0);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // 'b_305624: the move node and the magnet.
        'magnet: {
            'result_v0: {
                c.cycles += 2;
                let no_object = c.g[V0][0] == 0;
                c.g[V1][0] = 0x4d_0000;
                if no_object {
                    break 'result_v0;
                }
                c.cycles += 3;
                c.g[S0][0] = oc417_lw(c, V0, 0xc);
                let no_node = c.g[S0][0] == 0;
                c.g[V0][0] = c.g[S3][0];
                if no_node {
                    break 'magnet;
                }
                c.cycles += 7;
                c.g[V1][0] = oc417_lw(c, S0, 0xc);
                c.g[A0][0] = oc417_lh(c, V1, 0x80);
                c.g[V0][0] = oc417_lw(c, V1, 0x84);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = oc417_add(c.g[S0][0], c.g[A0][0]);
                call_ptr(c, target, 0x0030_55ec)?;
                let rejects = c.g[V0][0] == 0;
                c.g[V1][0] = 0x4d_0000;
                if rejects {
                    break 'result_v0;
                }
                c.cycles += 11;
                c.g[V1][0] = oc417_lw(c, S0, 0xc);
                c.g[A0][0] = oc417_lh(c, V1, 0xc0);
                c.g[V0][0] = oc417_lw(c, V1, 0xc4);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = oc417_add(c.g[S0][0], c.g[A0][0]);
                call_ptr(c, target, 0x0030_5608)?;
                let a = (c.g[S1][0] as u32).wrapping_add(8);
                c.lwc1(13, a);
                c.g[A0][0] = c.g[S0][0];
                let a = (c.g[S1][0] as u32).wrapping_add(4);
                c.lwc1(12, a);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0035_59f8, 0x0030_561c)?;
                c.g[V1][0] = 0x4d_0000;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 14;
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let a = (c.g[A3][0] as u32).wrapping_add(7);
        c.ldl(4, a);
        let a = c.g[A3][0] as u32;
        c.ldr(4, a);
        c.g[A1][0] = oc417_lw(c, A3, 8);
        let a = (c.g[S3][0] as u32).wrapping_add(7);
        c.sdl(4, a);
        let a = c.g[S3][0] as u32;
        c.sdr(4, a);
        oc417_sw(c, S3, 8, A1);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Shared pieces of the WScriptFn_* argument decoders.

/// The argument-copy loop of the `WScriptFn_*` decoders: `a1` entries of 16
/// bytes `{index, value, _, kind}` at `a2` are written into the word buffer
/// at `buf_reg` (`buf[index] = value`, converted to float when the entry's
/// kind differs from the kind table's (`t0`) and that one is 2). `a3` is 2.
pub(crate) fn oc420_param_loop(c: &mut Cpu, buf_reg: usize, head: u32) {
    loop {
        c.cycles += 8;
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[buf_reg][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let mut convert = false;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
            } else {
                convert = true;
            }
        }
        if convert {
            c.cycles += 4;
            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
            c.cvt_s_w(0, 0);
            c.swc1(0, c.g[A0][0] as u32);
        } else {
            c.cycles += 1;
            Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The tail the `WScriptFn_*` decoders share: copy the 12-byte default
/// result block at `0x4c9098` into the caller's return slot (`s3`, unaligned
/// `ldl/ldr/sdl/sdr` plus a word) and return it in `v0`. `v1` is 0x4d0000.
pub(crate) fn oc420_copy_result_block(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = c.g[S3][0];
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(7));
    c.ldr(4, c.g[A3][0] as u32);
    c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(8)).lw(c);
    c.sdl(4, (c.g[S3][0] as u32).wrapping_add(7));
    c.sdr(4, c.g[S3][0] as u32);
    Addr((c.g[S3][0] as u32).wrapping_add(8)).sw(c, c.g[A1][0]);
}

// ---------------------------------------------------------------------------
// WScriptFn_RandRange.

/// `float Rand_FloatRange(float min, float max)` (0x317830).
pub(crate) const OC420_RAND_FLOAT_RANGE: Callee = Callee {
    addr: 0x0031_7830,
    f: ee::s10::f_00317830,
};

/// `void* WScriptFn_RandRange(Ctx* self, int n, Arg* args)` (0x303598): a
/// script function filling a 3-float parameter block (`{min, max, ..}`,
/// from a lazily initialised static at 0x4fbc60: 0, 1.0, flag byte) with
/// `n` 16-byte `{index, value, _, kind}` arguments (the value is converted
/// to float when the argument kind differs from the parameter's table kind
/// and that kind is 2), then stores a random float in `[min, max)` into the
/// result register of `self` (`self[2] = 2`, `LunoVMRegister_AsFloat`).
/// Returns `self`.
pub fn wscript_fn_rand_range(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x26c8)).lw(c);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0x0050_0000;
        let initialised = c.g[V0][0] != 0;
        frame.save_fpr(c, 20, 0x70);
        if !initialised {
            // First use: `static Params p = {0, 1.0f, flag}` (the guard
            // and the trailing byte copy included).
            c.cycles += 10;
            c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0x43a0));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lbu(c);
            c.g[A0][0] = 1;
            Addr(0x0050_0000u32.wrapping_sub(0x43a0)).sw(c, 0);
            Addr((c.g[V0][0] as u32).wrapping_add(8)).sb(c, c.g[V1][0]);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(4));
            Addr((c.g[GP][0] as u32).wrapping_add(0x26c8)).sw(c, c.g[A0][0]);
            Addr(sp(c)).sb(c, c.g[V1][0]);
        }
        c.cycles += 7;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0xc;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x43a0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x0030_3608)?;
        c.g[A2][0] = c.g[S3][0];
        if (c.g[S2][0] as i64) > 0 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(0xc30));
            c.g[A3][0] = 2;
            oc420_param_loop(c, S0, 0x0030_3620);
        }
        c.cycles += 18;
        c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(4));
        c.lwc1(12, c.g[S0][0] as u32);
        OC420_RAND_FLOAT_RANGE.call(c, 0x0030_3674)?;
        c.g[V0][0] = 2;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S1][0];
        Addr((c.g[S1][0] as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0022_6618, 0x0030_3688)?;
        c.swc1(20, c.g[V0][0] as u32);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_StopObjectLoop.

/// `SSXAudioSystem_GetInstance` (0x28b180) and
/// `SSXAudioSystem_StopObjectLoop` (0x297eb8).
pub(crate) const OC420_AUDIO_GET_INSTANCE: u32 = 0x0028_b180;

pub(crate) const OC420_AUDIO_STOP_OBJECT_LOOP: u32 = 0x0029_7eb8;

/// `Result WScriptFn_StopObjectLoop(Result* out, int n, Arg* args)`
/// (0x300260): decodes `n` arguments into `{object id, sound, loop}`
/// (static defaults at 0x4fbb28: `-1` objects, lazily initialised), resolves
/// the object (id `-1` is the current object `*(gp[0xce8] + 0x290)`;
/// otherwise through the world cache) and, if there is one, stops the
/// sound's loop on it. Always copies the 12-byte default result into `*out`.
pub fn wscript_fn_stop_object_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x257c)).lw(c);
        frame.save(c, S1, 0x50);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        if c.g[V0][0] == 0 {
            // First use: `static Params p = {-1, -1, -1, flag}`.
            c.cycles += 12;
            c.g[V1][0] = sx(0x0050_0000u32.wrapping_sub(0x44d8));
            c.g[V0][0] = u64::MAX;
            c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lbu(c);
            Addr(0x0050_0000u32.wrapping_sub(0x44d8)).sw(c, c.g[V0][0]);
            c.g[A0][0] = u64::MAX;
            Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sb(c, c.g[A2][0]);
            c.g[A1][0] = 1;
            Addr((c.g[V1][0] as u32).wrapping_add(4)).sw(c, c.g[V0][0]);
            Addr((c.g[V1][0] as u32).wrapping_add(8)).sw(c, c.g[A0][0]);
            Addr((c.g[GP][0] as u32).wrapping_add(0x257c)).sw(c, c.g[A1][0]);
            Addr(sp(c)).sb(c, c.g[A2][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x10;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x44d8));
        Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x0030_02e0)?;
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(0x0044_0000u32.wrapping_add(0x6560));
            c.g[A3][0] = 2;
            oc420_param_loop(c, S0, 0x0030_02f8);
        }
        // The object id (buf[0]): -1 means the current object.
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        let by_id = c.g[V0][0] != 0;
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x16c8)).lw(c);
        if !by_id {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xce8)).lw(c);
            c.g[S2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let present = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !present {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A1][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0;
            }
            c.cycles += 1;
            c.g[S2][0] = c.g[A1][0];
        }
        c.cycles += 2;
        c.g[V1][0] = 0x004d_0000;
        if c.g[S2][0] != 0 {
            c.cycles += 9;
            c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
            c.g[S0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
            call_ptr(c, OC420_AUDIO_GET_INSTANCE, 0x0030_03c0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S1][0];
            call_ptr(c, OC420_AUDIO_STOP_OBJECT_LOOP, 0x0030_03d4)?;
            c.g[V1][0] = 0x004d_0000;
        }
        oc420_copy_result_block(c);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_AddParentModifierRev.

/// `cMoveNode_addParent` (0x355978).
pub(crate) const OC420_MOVE_NODE_ADD_PARENT: u32 = 0x0035_5978;

/// `Result WScriptFn_AddParentModifierRev(Result* out, int n, Arg* args)`
/// (0x2fdc60): decodes `n` arguments into `{child id, parent id, flags,
/// x, y, z}` (static defaults at 0x4fb758), resolves both objects (`-1` child
/// = the current object), and when the child's move node is not already of
/// kind 6 (`vtable+0x50`, `v10(6)`) and the parent's move node answers
/// `vtable+0x80`, adds the parent to its move node with the offset vector.
/// Copies the default result block into `*out`.
pub fn wscript_fn_add_parent_modifier_rev(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2520)).lw(c);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        c.g[A2][0] = 0x0050_0000;
        if c.g[V0][0] == 0 {
            // First use: `static Params p = {-1, -1, 0, 0.0, 0.0, 0.0, flag}`.
            c.cycles += 15;
            c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0x48a8));
            c.g[V1][0] = u64::MAX;
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lbu(c);
            c.mtc1(0, 0);
            c.g[A0][0] = 1;
            Addr(0x0050_0000u32.wrapping_sub(0x48a8)).sw(c, c.g[V1][0]);
            Addr((c.g[V0][0] as u32).wrapping_add(0x18)).sb(c, c.g[A1][0]);
            Addr((c.g[V0][0] as u32).wrapping_add(4)).sw(c, c.g[V1][0]);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
            Addr((c.g[GP][0] as u32).wrapping_add(0x2520)).sw(c, c.g[A0][0]);
            Addr((c.g[V0][0] as u32).wrapping_add(8)).sw(c, 0);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
            Addr(sp(c)).sb(c, c.g[A1][0]);
        }
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x48a8));
        c.g[A2][0] = 0x1c;
        Addr(sp(c).wrapping_add(0x14)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x003e_6574, 0x002f_dcf0)?;
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(0x0044_0000u32.wrapping_add(0x61c0));
            c.g[A3][0] = 2;
            oc420_param_loop(c, S1, 0x002f_dd08);
        }
        // The child object id (buf[0]): -1 means the current object.
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        let by_id = c.g[V0][0] != 0;
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x16c8)).lw(c);
        if !by_id {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xce8)).lw(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let record = c.g[V1][0];
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            if record != 0 {
                c.cycles += 7;
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let present = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !present {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
        }
        // The parent: only when buf[1] is not -1.
        c.cycles += 4;
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(4)).lw(c);
        c.g[V0][0] = !c.g[V0][0];
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0x004d_0000;
        } else {
            'done: {
                c.cycles += 3;
                c.g[S2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = 6;
                if c.g[S2][0] != 0 {
                    c.cycles += 7;
                    c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x50)).lh(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x54)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002f_dde0)?;
                    c.g[V1][0] = 0x004d_0000;
                    if c.g[V0][0] != 0 {
                        break 'done;
                    }
                    // Parent object through the world cache.
                    c.cycles += 10;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x16c8)).lw(c);
                    c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(4)).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                    c.g[V0][0] = c.g[A1][0] & 0xff;
                    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    let record = c.g[V1][0];
                    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                    if record != 0 {
                        c.cycles += 7;
                        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                        let present = c.g[V0][0] != 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if !present {
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                        }
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                    c.cycles += 2;
                    c.g[S0][0] = 0;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    }
                    c.cycles += 2;
                    c.g[V1][0] = 0x004d_0000;
                    if c.g[S0][0] == 0 {
                        break 'done;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002f_de58)?;
                    let answered = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if answered {
                        // Add the parent with the offset vector (x, y, z, 0).
                        c.cycles += 11;
                        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0xc));
                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[A3][0] = sx(sp(c).wrapping_add(0x30));
                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
                        c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x18)).lw(c);
                        c.swc1(2, sp(c).wrapping_add(0x30));
                        c.swc1(0, sp(c).wrapping_add(0x34));
                        c.swc1(1, sp(c).wrapping_add(0x38));
                        Addr(sp(c).wrapping_add(0x3c)).sw(c, 0);
                        c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(8)).lw(c);
                        call_ptr(c, OC420_MOVE_NODE_ADD_PARENT, 0x002f_de8c)?;
                    }
                }
                // (also the exit when the child has no move node, or the
                // parent has none / does not answer)
                c.cycles += 1;
                c.g[V1][0] = 0x004d_0000;
            }
        }
        oc420_copy_result_block(c);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptValue* WScriptFn_ObjectSlot36(ScriptValue* out, int n, Entry* e)`
/// (0x2ff9a8): script call "object slot 36". Builds an argument block of four
/// words (initialised to -1 on first use from the `0x4fbaf8` table, copied
/// with `EA_MemCopy`), fills `args[e.idx]` from the n entries (an int entry
/// whose slot type is 2 becomes a float), looks the object up by `args[0]`
/// (handle: low byte = slot, high = generation; through `gp[0x16c8]`) and
/// calls its method at vtable `+0x120` with `(args[1], args[2])`. Returns
/// `luno_gLunoNil` (copied into `out`).
#[allow(unreachable_code)]
pub fn w_script_fn_object_slot36(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ffa08: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2550));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = 0x500000;
            if t {
                break 'b_2ffa08;
            } else {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4508));
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.sw((c.g[A2][0] as u32).wrapping_sub(0x4508), c.g[V0][0]);
                c.g[A0][0] = 0xffffffffffffffff;
                c.sb((c.g[V1][0] as u32).wrapping_add(0xc), c.g[A1][0]);
                c.g[V0][0] = 0x1;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2550), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.sb(c.g[SP][0] as u32, c.g[A1][0]);
                break 'b_2ffa08;
            }
        }
        'b_2ffa88: {
            c.cycles += 10;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4508));
            c.g[A2][0] = 0x10;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e6574, 0x002ffa28)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if t {
                break 'b_2ffa88;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6540));
                c.g[A3][0] = 0x2;
                'l_2ffa40: loop {
                    'b_2ffa7c: {
                        'b_2ffa78: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_2ffa78;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_2ffa78;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, c.g[A0][0] as u32);
                                    if t {
                                        break 'b_2ffa7c;
                                    } else {
                                        break 'b_2ffa78;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        break 'b_2ffa7c;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002ffa40);
                        }
                        continue 'l_2ffa40;
                    } else {
                        break 'b_2ffa88;
                    }
                }
            }
        }
        'b_2ffae4: {
            'b_2ffaa4: {
                c.cycles += 4;
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                if t {
                    break 'b_2ffaa4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                    if t {
                        break 'b_2ffae4;
                    } else {
                        break 'b_2ffaa4;
                    }
                }
            }
            'b_2ffae0: {
                c.cycles += 8;
                c.g[V0][0] = c.g[A2][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                if t {
                    break 'b_2ffae0;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        break 'b_2ffae4;
                    } else {
                        break 'b_2ffae0;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_2ffae4;
        }
        'b_2ffb10: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let t = c.g[A2][0] == 0u64;
            c.g[V1][0] = 0x4d0000;
            if t {
                break 'b_2ffb10;
            } else {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x120));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002ffb0c)?;
                c.g[V1][0] = 0x4d0000;
                break 'b_2ffb10;
            }
        }
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S3][0] as u32);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `ScriptValue* WScriptFn_AddCollectItems(ScriptValue* out, int n, Entry*
/// e)` (0x300f50): script call "add collect items". Builds a 22-word
/// argument block initialised to -1 (first call: the template at 0x4fbb78,
/// flag `gp[0x25c0]`; copied with `EA_MemCopy`), fills `args[e.idx]` from the
/// n entries (an int entry whose slot type, table 0x446570, is 2 becomes a
/// float), and, when `args[0]` is set, calls `WScriptMan_AddCollectItems(
/// gWScriptMan, args[0], args + 1, 20)`. Returns `luno_gLunoNil` in `out`.
#[allow(unreachable_code)]
pub fn w_script_fn_add_collect_items(c: &mut Cpu) {
    guest(c, |c| {
        'b_300ff4: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x25c0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S2][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S3][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = 0x500000;
            if t {
                break 'b_300ff4;
            } else {
                c.cycles += 29;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4488));
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x54));
                c.sw((c.g[A2][0] as u32).wrapping_sub(0x4488), c.g[V0][0]);
                c.g[A0][0] = 0x1;
                c.sb((c.g[V1][0] as u32).wrapping_add(0x54), c.g[A1][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x25c0), c.g[A0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                c.sb(c.g[SP][0] as u32, c.g[A1][0]);
                break 'b_300ff4;
            }
        }
        'b_3010c0: {
            c.cycles += 29;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4488));
            c.g[A2][0] = 0x58;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x60), c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x50), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x54), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
            call_ptr(c, 0x003e6574, 0x00301060)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if t {
                break 'b_3010c0;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6570));
                c.g[A3][0] = 0x2;
                'l_301078: loop {
                    'b_3010b4: {
                        'b_3010b0: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_3010b0;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_3010b0;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, c.g[A0][0] as u32);
                                    if t {
                                        break 'b_3010b4;
                                    } else {
                                        break 'b_3010b0;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        break 'b_3010b4;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00301078);
                        }
                        continue 'l_301078;
                    } else {
                        break 'b_3010c0;
                    }
                }
            }
        }
        'b_3010e0: {
            c.cycles += 5;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            let t = c.g[A1][0] == c.g[V0][0];
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            if t {
                break 'b_3010e0;
            } else {
                c.cycles += 3;
                c.g[A2][0] = c.g[S0][0] | 0x4;
                c.g[A3][0] = 0x14;
                Callee {
                    addr: 0x0030bac8,
                    f: ee::s10::WScriptMan_AddCollectItems,
                }
                .call(c, 0x003010e0)?;
                break 'b_3010e0;
            }
        }
        c.cycles += 16;
        c.g[V1][0] = 0x4d0000;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S3][0] as u32);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        Ok(())
    })
}

/// `RiderCommand* WScriptFn_RiderCommand(RiderCommand* out, int n, Ent* e)`
/// (0x2ff850): script function "rider command". On first use initialises the
/// static default command (`0x4fbae8`: id -1, two zero words, flag byte).
/// Copies it to a 16-byte stack record, overlays the `n` (index, value, ...)
/// entries of `e` (0x10 bytes each: `+0` slot, `+4` value, `+0xc` kind; a
/// value is converted to float when the slot's type in the table at
/// `0x446530` is 2 and the entry's kind differs), defaults the rider id from
/// the global at `gp + 0xce8` and calls `cRider_flagEventWorld(rider, v1,
/// v2)`; returns `out` filled from the 12-byte record at `0x4c9098`.
pub fn wscript_fn_rider_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0x254c));
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x50_0000;
        if !initialised {
            c.cycles += 10;
            c.g[V0][0] = sx(oc429_at(c.g[A2][0], 0u32.wrapping_sub(0x4518)));
            c.g[V1][0] = u64::MAX;
            c.g[A1][0] = c.lbu(oc429_at(c.g[V0][0], 0xc));
            c.g[A0][0] = 1;
            c.sw(oc429_at(c.g[A2][0], 0u32.wrapping_sub(0x4518)), c.g[V1][0]);
            c.sb(oc429_at(c.g[V0][0], 0xc), c.g[A1][0]);
            c.sw(oc429_at(c.g[GP][0], 0x254c), c.g[A0][0]);
            c.sw(oc429_at(c.g[V0][0], 0x4), 0);
            c.sw(oc429_at(c.g[V0][0], 0x8), 0);
            c.sb(sp(c), c.g[A1][0]);
        }
        c.cycles += 7;
        c.g[S0][0] = sx(oc429_at(c.g[SP][0], 0x10));
        c.g[A1][0] = sx(oc429_at(c.g[A2][0], 0u32.wrapping_sub(0x4518)));
        c.g[A2][0] = 0x10;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x002f_f8bc)?; // EA_MemCopy
        let none = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !none {
            c.cycles += 5;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(oc429_at(c.g[V0][0], 0x6530));
            c.g[A3][0] = 2;
            loop {
                'stored: {
                    'int_store: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                        c.g[V1][0] = c.lw(oc429_at(c.g[A2][0], 0xc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx(oc429_at(c.g[V0][0], c.g[S0][0] as u32));
                        c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[T0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw(oc429_at(c.g[A2][0], 0x4));
                            break 'int_store;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = c.lw(oc429_at(c.g[A2][0], 0x4));
                            break 'int_store;
                        }
                        c.cycles += 4;
                        c.lwc1(0, oc429_at(c.g[A2][0], 0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        break 'stored;
                    }
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(1)));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx(oc429_at(c.g[A2][0], 0x10));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002f_f8d8);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = u64::MAX;
        let have_rider = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
        if !have_rider {
            c.cycles += 5;
            c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0xce8));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        }
        c.cycles += 24;
        c.g[A0][0] = c.lw(oc429_at(c.g[V0][0], 0x84));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A1][0] = c.lw(oc429_at(c.g[S0][0], 0x4));
        c.g[V0][0] = c.lw(oc429_at(c.g[A0][0], 0xc));
        c.lwc1(12, oc429_at(c.g[S0][0], 0x8));
        c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[V1][0] as u32));
        c.g[A0][0] = c.lw(oc429_at(c.g[V0][0], 0x28));
        call_ptr(c, 0x0010_f1c0, 0x002f_f964)?; // cRider_flagEventWorld
        c.g[V1][0] = 0x4d_0000;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(oc429_at(c.g[V1][0], 0u32.wrapping_sub(0x6f68)));
        c.ldl(A0, oc429_at(c.g[A3][0], 0x7));
        c.ldr(A0, oc429_at(c.g[A3][0], 0));
        c.g[A1][0] = c.lw(oc429_at(c.g[A3][0], 0x8));
        c.sdl(A0, oc429_at(c.g[S3][0], 0x7));
        c.sdr(A0, oc429_at(c.g[S3][0], 0));
        c.sw(oc429_at(c.g[S3][0], 0x8), c.g[A1][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::game::scripter::ssxscriptengine::tests::oc428_install_fakes;

    use super::*;
    use crate::game::replay::replaycache::tests::{
        oc420_hook_add_parent, oc420_hook_audio_instance, oc420_hook_audio_stop_loop,
        oc420_hook_luno_as_float, oc420_hook_rand_range,
    };

    use crate::difftest::fill;
    use crate::difftest::DATA;

    /// `WorldObjFactory_DeadNode`: every event kind (0, 1, 3, others on both
    /// sides of 2), no object or an object of type 6, 0x13 or other (its
    /// delete through a fake vtable slot), random node flags; the real
    /// `EA_MemAllocNamed` and node constructors as in
    /// `world_obj_replace_with_dead_node_matches`.
    #[test]
    fn world_obj_factory_dead_node_matches() {
        use crate::difftest::check;
        use crate::difftest::fakes::vtable;
        use crate::game::fe::ovtemplatepausemenu::tests::setup;
        const NODE: u32 = DATA + 0x800;
        const OBJ: u32 = DATA + 0xa00;
        const EVENT: u32 = DATA + 0xc00;
        check(
            ee::s10::f_002fc2c0__generated,
            world_obj_factory_dead_node,
            240,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2f_c2c0);
                c.pc = 0x002f_c2c0;
                let kind: u32 = match seed % 7 {
                    0 | 1 => 0,
                    2 | 3 => 1,
                    4 | 5 => 3,
                    _ => [2, 0xffff_ffff, 4, 0x80][(seed / 7 % 4) as usize],
                };
                c.bus.write32(EVENT + DEAD_NODE_EVENT_KIND, kind);
                fill(c, &mut r, NODE, 8);
                let obj = if seed / 7 % 4 == 0 { 0 } else { OBJ };
                c.bus.write32(NODE + DEAD_NODE_OBJECT, obj);
                fill(c, &mut r, OBJ, 8);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(OBJ + DEAD_NODE_OBJ_VTABLE, vt);
                let ty = match seed / 3 % 4 {
                    0 => 6,
                    1 => 0x13,
                    _ => r.next_u32() as u16 & 0x1f,
                };
                c.bus.write16(OBJ + DEAD_NODE_OBJ_TYPE, ty);
                c.g[A0][0] = sx(NODE);
                c.g[A1][0] = sx(EVENT);
            },
        );
    }

    /// `WorldObjFactory_AttachModifier`: a node that already is a type 9, 6
    /// or 22 modifier, or already `type` (refresh: a fake), and for other
    /// nodes types 13 (particle) and 0 (debounce: the real allocator and
    /// constructors on a test heap; the node's "which transform" method
    /// returns the object's own matrix, so ModObj_TakeOverObject deletes the
    /// node through a fake) and unknown types (deleted through a fake, some
    /// unwinding). `IsType` is the real leaf 0x2afa58 (`*field == type`).
    /// Not reached: types 11 (mesh animation) and 22 (dead fade), whose
    /// constructors walk model data.
    #[test]
    fn world_obj_factory_attach_modifier_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{elf_image, DATA, GAME_GP};
        const NODE: u32 = DATA + 0x1000;
        const NODE_KIND: u32 = NODE + 0x40;
        const VT: u32 = DATA + 0x2000;
        const OBJECT: u32 = DATA + 0x3000;
        const BUF: u32 = DATA + 0x4000;
        const BELIB: u32 = DATA + 0x5000;
        covered(
            ee::s10::f_002fae38__generated,
            world_obj_factory_attach_modifier,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_ae38, 2);
                elf_image(c);
                crate::game::bx::bxstring::tests::heap(c);
                for a in (NODE..BELIB + 0x100).step_by(4) {
                    c.bus.write32(a, 0);
                }
                c.bus.write32(GAME_GP - 0x1f04, BELIB);
                // The node: vtable, kind field, its object.
                c.bus.write32(NODE + WOF_ATTACH_VTABLE, VT);
                c.bus.write32(NODE + WOF_ATTACH_OBJECT, OBJECT);
                c.bus.write16(VT + WOF_ATTACH_VT_IS_TYPE, 0x40);
                c.bus.write32(VT + WOF_ATTACH_VT_IS_TYPE + 4, 0x002a_fa58);
                c.bus.write32(VT + WOF_ATTACH_VT_REFRESH + 4, FAKE);
                c.bus.write32(VT + WOF_ATTACH_VT_DELETE + 4, FAKE + 0x10);
                c.bus.write32(VT + 0x84, 0x0014_06c0); // no particle modifiers
                c.bus.write32(VT + 0xc4, FAKE + 0x20); // its transform
                c.bus.write32(LOG_RESULTS, OBJECT + 0x10);
                c.bus.write32(OBJECT + 0x8, 0x100);
                c.bus.write32(OBJECT + 0xc, r.next_u32());
                let ty: u32 = match seed % 8 {
                    0 | 1 => 13,
                    2 | 3 => 0,
                    4 => [1, 5, 12, 14, 30][(seed / 8 % 5) as usize],
                    5 => (r.next_u32() % 40).wrapping_neg(),
                    _ => match r.next_u32() % 24 {
                        // Not the mesh-animation / dead-fade constructors.
                        t @ (11 | 22) => t + 1,
                        t => t,
                    },
                };
                let kind = match seed % 11 {
                    0 => 9,
                    1 => 6,
                    2 => 22,
                    3 => ty,
                    _ => 100,
                };
                c.bus.write32(NODE_KIND, kind);
                c.pc = 0x002f_ae38;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.g[A0][0] = sx(NODE);
                c.g[A1][0] = sx(ty);
                c.g[A2][0] = sx(BUF);
            },
        );
    }

    use crate::difftest::check;
    use crate::difftest::fakes::covered;

    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::oc327_fake_gated;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::oc327_hook;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::OC327_CTRL;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::OC327_MAGIC;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::OC327_RET;
    pub(crate) use crate::game::object::debouncenode::tests::oc327_install_alloc;
    pub(crate) use crate::game::object::debouncenode::tests::RET_ALLOC;
    pub(crate) use crate::game::object::debouncenode::tests::RET_BASE_CTOR;
    use std::sync::atomic::AtomicUsize;

    /// `WorldObjFactory_Hide`: `n` entries at DATA+0x8000 (the first one, when
    /// present, carries the id `0x123`), the world's object reference table at
    /// DATA+0x5000, the `-1` object at DATA+0x5400, and in some seeds an
    /// object to hide at DATA+0x6000 whose vtable slot 0x50 is a fake (hidden
    /// object) or the real zero leaf (killed, then a `Hide` node allocated).
    pub(crate) fn world_obj_factory_hide_setup(c: &mut Cpu, seed: u32) {
        /// `func_001406C0`, a real leaf returning 0.
        const ZERO_LEAF: u32 = 0x0014_06c0;
        use crate::difftest::fakes::{machine, vtable, LOG_UNWIND_AT};
        use crate::difftest::{DATA, GAME_GP};
        use crate::game::fe::feasyncfile::tests::{
            be_library_5 as be_library, heap_init_5 as heap_init, world_3 as world,
        };
        use crate::game::worldpainter::quadtree::tests::pool_allocator;

        let mut r = machine(c, seed, 0x002f_fb50, 6);
        // Make sure the fake query/kill calls unwind in some seeds.
        if seed % 2 == 0 {
            c.bus.write32(LOG_UNWIND_AT, 0);
        }
        pool_allocator(c);
        heap_init(c);
        be_library(c);
        world(c);

        // The guarded static seed, and sometimes the uninitialized path.
        c.bus
            .write32(GAME_GP + WOF_INIT_FLAG, if seed % 4 == 0 { 0 } else { 1 });
        // The seed word is -1, as the guarded init leaves it.
        c.bus.write32(GAME_GP + WOF_SEED_WORD, 0xffff_ffff);
        c.bus
            .write32(GAME_GP + WOF_SEED_BYTE, 0x1122_3300 | (seed & 0xff));
        for (i, kind) in [2u32, 3, 2, 0].into_iter().enumerate() {
            c.bus.write32(GAME_GP + WOF_KINDS + 4 * i as u32, kind);
        }

        // Variant: what the entry list holds (0-2 leave the id at -1).
        let variant = seed % 6;
        let obj_kind = (seed / 6) % 5;
        let entries: &[(u32, u32, u32)] = match variant {
            0 | 1 => &[],
            2 => &[(1, 0x42, 2), (2, 0x10, 3)],
            _ => &[(0, 0x123, 2), (1, 0x42, 2), (2, 0x10, 3), (3, 0x7, 0)],
        };
        let n: u32 = match variant {
            0 | 1 => 0,
            2 => 2,
            _ => 4,
        };
        let ents = DATA + 0x8000;
        for (i, &(idx, val, typ)) in entries.iter().enumerate() {
            let p = ents + 0x10 * i as u32;
            c.bus.write32(p, idx);
            c.bus.write32(p + 4, val);
            c.bus.write32(p + 0xc, typ);
        }

        // What the id resolves to: 0 (missing entry), 0 (null reference),
        // an object with no instance, a hidden object, or a visible one.
        let obj = if obj_kind < 2 { 0 } else { DATA + 0x6000 };
        if variant >= 3 {
            let world_obj = DATA + 0x5000;
            let table = DATA + 0x5100;
            let table_slot = table + 4 * 0x23; // id & 0xff
            c.bus.write32(GAME_GP + WOF_OBJECTS, world_obj);
            c.bus.write32(world_obj + 8, table);
            if obj_kind == 0 {
                c.bus.write32(table_slot, 0);
            } else {
                let s = DATA + 0x5200;
                c.bus.write32(table_slot, s);
                c.bus.write32(s + 0x1c, DATA + 0x5300);
                // The reference: obj/4, read back shifted left by 8.
                let word = if obj_kind == 1 { 0 } else { (obj >> 2) << 8 };
                c.bus.write32(DATA + 0x5304, word); // [id >> 8] = [1]
            }
        } else {
            let d = DATA + 0x5400;
            c.bus.write32(GAME_GP + WOF_SPECIAL, d);
            c.bus.write32(d + 0x290, obj);
        }
        if obj != 0 {
            let p = DATA + 0x5900;
            match obj_kind {
                2 => c.bus.write32(obj + 0xc, 0),
                3 => {
                    let vt = vtable(c, &mut r, &[0x50]);
                    c.bus.write32(p + 0xc, vt);
                    c.bus.write32(obj + 0xc, p);
                }
                _ => {
                    let vt = DATA + 0x5800;
                    c.bus.write16(vt + 0x50, 0);
                    c.bus.write32(vt + 0x54, ZERO_LEAF);
                    c.bus.write32(p + 0xc, vt);
                    c.bus.write32(obj + 0xc, p);
                }
            }
        }

        c.g[A0][0] = sx(DATA + 0x700); // the 12-byte result buffer
        c.g[A1][0] = sx(n);
        c.g[A2][0] = sx(ents);
        c.pc = 0x002f_fb50;
    }

    #[test]
    fn world_obj_factory_hide_matches() {
        use crate::difftest::fakes::covered;
        covered(
            ee::s10::f_002ffb50__generated,
            world_obj_factory_hide,
            240,
            &|c, seed| world_obj_factory_hide_setup(c, seed),
        );
    }

    /// `WorldObjFactory_Debounce`: the same kind of entry list as Hide (five
    /// slots), the static seed at 0x4fb528, and an object whose +0xc node is
    /// absent (the node is allocated) or present with a fake query method
    /// (nonzero: the object counts as covered, no allocation).
    #[test]
    fn world_obj_factory_debounce_matches() {
        use crate::difftest::fakes::{machine, vtable, LOG_UNWIND_AT};
        use crate::difftest::{check, DATA, GAME_GP};
        use crate::game::fe::feasyncfile::tests::{
            be_library_5 as be_library, heap_init_5 as heap_init, world_3 as world,
        };
        use crate::game::worldpainter::quadtree::tests::pool_allocator;
        check(
            ee::s10::f_002fc7d0__generated,
            world_obj_factory_debounce,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002f_c7d0, 6);
                if seed % 2 == 0 {
                    c.bus.write32(LOG_UNWIND_AT, 0);
                }
                pool_allocator(c);
                heap_init(c);
                be_library(c);
                world(c);

                c.bus
                    .write32(GAME_GP + WOD_INIT_FLAG, if seed % 4 == 0 { 0 } else { 1 });
                // 0x4fb528: the static's post-init contents — id -1, 1.0f, 1,
                // -1 and the flag byte.
                let seed_word = WOF_RESULT_BASE - WOD_STATIC_OFF;
                c.bus.write32(seed_word, 0xffff_ffff);
                c.bus.write32(seed_word + 4, 0x3f80_0000);
                c.bus.write32(seed_word + 8, 1);
                c.bus.write32(seed_word + 12, 0xffff_ffff);
                c.bus.write32(seed_word + 16, 0x1122_3300 | (seed & 0xff));
                for (i, kind) in [2u32, 3, 2, 0].into_iter().enumerate() {
                    c.bus
                        .write32(WOD_KINDS_BASE + WOD_KINDS_OFF + 4 * i as u32, kind);
                }

                let variant = seed % 6;
                let obj_kind = (seed / 6) % 4;
                let entries: &[(u32, u32, u32)] = match variant {
                    0 | 1 => &[],
                    2 => &[(1, 0x42, 2), (2, 0x10, 3)],
                    _ => &[(0, 0x123, 2), (1, 0x42, 2), (2, 0x10, 3), (3, 0x7, 0)],
                };
                let n: u32 = match variant {
                    0 | 1 => 0,
                    2 => 2,
                    _ => 4,
                };
                let ents = DATA + 0x9000;
                for (i, &(idx, val, typ)) in entries.iter().enumerate() {
                    let p = ents + 0x10 * i as u32;
                    c.bus.write32(p, idx);
                    c.bus.write32(p + 4, val);
                    c.bus.write32(p + 0xc, typ);
                }

                let obj = if obj_kind < 2 { 0 } else { DATA + 0x6000 };
                if variant >= 3 {
                    let world_obj = DATA + 0x5000;
                    let table = DATA + 0x5100;
                    let table_slot = table + 4 * 0x23;
                    c.bus.write32(GAME_GP + WOF_OBJECTS, world_obj);
                    c.bus.write32(world_obj + 8, table);
                    if obj_kind == 0 {
                        c.bus.write32(table_slot, 0);
                    } else {
                        let s = DATA + 0x5200;
                        c.bus.write32(table_slot, s);
                        c.bus.write32(s + 0x1c, DATA + 0x5300);
                        let word = if obj_kind == 1 { 0 } else { (obj >> 2) << 8 };
                        c.bus.write32(DATA + 0x5304, word);
                    }
                } else {
                    let d = DATA + 0x5400;
                    c.bus.write32(GAME_GP + WOF_SPECIAL, d);
                    c.bus.write32(d + 0x290, obj);
                }
                if obj != 0 {
                    // No node yet, or a node whose virtual query is a fake
                    // (nonzero: the object counts as covered, no allocation).
                    let p = DATA + 0x5900;
                    if obj_kind == 2 {
                        c.bus.write32(obj + 0xc, 0);
                    } else {
                        let vt = vtable(c, &mut r, &[0x50]);
                        c.bus.write32(p + 0xc, vt);
                        c.bus.write32(obj + 0xc, p);
                    }
                }

                c.g[A0][0] = sx(DATA + 0x700);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(ents);
                c.pc = 0x002f_c7d0;
            },
        );
    }

    /// `WScriptFn_AddSplineModifier`: the 48-byte static seed at 0x4fb778,
    /// twelve entry slots, the id and the target-note word controlled by the
    /// entries, and a move node whose virtual "supports splines" slot 0x80 is
    /// a fake (adds the spline) or the real zero leaf (does not).
    #[test]
    fn wscript_fn_add_spline_modifier_matches() {
        /// `func_001406C0`, a real leaf returning 0.
        const ZERO_LEAF: u32 = 0x0014_06c0;
        use crate::difftest::fakes::{machine, vtable, LOG_UNWIND_AT};
        use crate::difftest::{check, DATA, GAME_GP};
        use crate::game::fe::feasyncfile::tests::{
            be_library_5 as be_library, heap_init_5 as heap_init, world_3 as world,
        };
        use crate::game::worldpainter::quadtree::tests::pool_allocator;
        check(
            ee::s10::f_002fded0__generated,
            wscript_fn_add_spline_modifier,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002f_ded0, 8);
                if seed % 2 == 0 {
                    c.bus.write32(LOG_UNWIND_AT, 0);
                }
                pool_allocator(c);
                heap_init(c);
                be_library(c);
                world(c);

                c.bus
                    .write32(GAME_GP + WSA_INIT_FLAG, if seed % 4 == 0 { 0 } else { 1 });
                // 0x4fb778: the static's post-init contents — -1, -1, 1, 3,
                // 20.0f, 0, 0, 0, 1, 0, 0 and the tag byte at +0x2c.
                let seed_word = WSA_STATIC_BASE - WSA_STATIC_OFF;
                for (off, word) in [
                    (0u32, 0xffff_ffffu32),
                    (4, 0xffff_ffff),
                    (8, 1),
                    (12, 3),
                    (16, 0x41a0_0000),
                    (20, 0),
                    (24, 0),
                    (28, 0),
                    (32, 1),
                    (36, 0),
                    (40, 0),
                ] {
                    c.bus.write32(seed_word + off, word);
                }
                c.bus.write32(seed_word + 44, 0x1122_3300 | (seed & 0xff));
                for (i, kind) in [2u32, 3, 2, 0].into_iter().enumerate() {
                    c.bus
                        .write32(WSA_KINDS_BASE + WSA_KINDS_OFF + 4 * i as u32, kind);
                }

                let variant = seed % 6;
                let obj_kind = (seed / 6) % 5;
                let entries: &[(u32, u32, u32)] = match variant {
                    0 | 1 => &[],
                    2 => &[(2, 0x10, 3), (3, 0x7, 0)],
                    4 => &[(0, 0x123, 2), (2, 0x10, 3), (3, 0x7, 0)],
                    _ => &[(0, 0x123, 2), (1, 0x42, 2), (2, 0x10, 3), (3, 0x7, 0)],
                };
                let n: u32 = match variant {
                    0 | 1 => 0,
                    2 => 2,
                    4 => 3,
                    _ => 4,
                };
                let ents = DATA + 0x8000;
                for (i, &(idx, val, typ)) in entries.iter().enumerate() {
                    let p = ents + 0x10 * i as u32;
                    c.bus.write32(p, idx);
                    c.bus.write32(p + 4, val);
                    c.bus.write32(p + 0xc, typ);
                }

                let obj = if obj_kind < 2 { 0 } else { DATA + 0x6000 };
                if variant >= 3 {
                    let world_obj = DATA + 0x5000;
                    let table = DATA + 0x5100;
                    let table_slot = table + 4 * 0x23;
                    c.bus.write32(GAME_GP + WOF_OBJECTS, world_obj);
                    c.bus.write32(world_obj + 8, table);
                    if obj_kind == 0 {
                        c.bus.write32(table_slot, 0);
                    } else {
                        let s = DATA + 0x5200;
                        c.bus.write32(table_slot, s);
                        c.bus.write32(s + 0x1c, DATA + 0x5300);
                        let word = if obj_kind == 1 { 0 } else { (obj >> 2) << 8 };
                        c.bus.write32(DATA + 0x5304, word);
                    }
                } else {
                    let d = DATA + 0x5400;
                    c.bus.write32(GAME_GP + WOF_SPECIAL, d);
                    c.bus.write32(d + 0x290, obj);
                }
                if obj != 0 {
                    let p = DATA + 0x5900;
                    match obj_kind {
                        2 => c.bus.write32(obj + 0xc, 0),
                        3 => {
                            let vt = vtable(c, &mut r, &[0x80]);
                            c.bus.write32(p + 0xc, vt);
                            c.bus.write32(obj + 0xc, p);
                        }
                        _ => {
                            let vt = DATA + 0x5800;
                            c.bus.write16(vt + 0x80, 0);
                            c.bus.write32(vt + 0x84, ZERO_LEAF);
                            c.bus.write32(p + 0xc, vt);
                            c.bus.write32(obj + 0xc, p);
                        }
                    }
                }

                c.g[A0][0] = sx(DATA + 0x700);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(ents);
                c.pc = 0x002f_ded0;
            },
        );
    }

    /// `WScriptFn_ObjectExists`: the one-time id-buffer init, the entry
    /// overlay (matching type, float slot, int slot), and every id lookup
    /// outcome (`-1`, absent low byte, absent high byte, found live/dead).
    #[test]
    fn wscript_fn_object_exists_matches() {
        check(
            ee::s10::f_00303130__generated,
            wscript_fn_object_exists,
            240,
            &|c, seed| {
                let mut r = Rng(0x3031_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                let obj = DATA + 0x2000;
                let entries = DATA + 0x2100;
                let vt = DATA + 0x2200;
                let x = DATA + 0x2300;
                let base = DATA + 0x2400;
                let high = DATA + 0x2800;
                let desc = DATA + 0x2c00;
                let target = DATA + 0x3000;
                c.g[A0][0] = obj as u64;
                let n = [-1i64, 0, 1, 2, 3][(seed % 5) as usize];
                c.g[A1][0] = n as u64;
                c.g[A2][0] = entries as u64;
                // The script value types of the three slots.
                for i in 0..4u32 {
                    c.bus.write32(GAME_GP + 0xc28 + 4 * i, i % 3);
                }
                c.bus.write32(GAME_GP + 0x26c4, seed as u32 % 2);
                // The shared id buffer: the all-ones id, or a real one whose
                // low/high bytes select the lookup outcome.
                let id = match seed % 5 {
                    0 => 0xffff_ffffu32, // the world object
                    1 => 0x0000_0101,    // lo=1, hi=1: found
                    2 => 0x0000_0103,    // lo=3: no such object
                    3 => 0x0000_0001,    // hi=0: no such entry
                    _ => 0x0000_0101,
                };
                c.bus.write32(WSCRIPT_FN_OBJECT_EXISTS_IDS, id);
                c.bus
                    .write32(WSCRIPT_FN_OBJECT_EXISTS_IDS + 4, r.next_u32());
                c.bus
                    .write32(WSCRIPT_FN_OBJECT_EXISTS_IDS + 8, r.next_u32());
                // The entries: slot, value, type (0..3).
                for i in 0..4u32 {
                    let e = entries + 16 * i;
                    c.bus.write32(e, (seed as u32 >> (2 * i)) % 3);
                    c.bus.write32(e + 4, r.next_u32());
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, (seed as u32 >> (4 + i)) % 4);
                }
                // The id -> object tables.
                c.bus.write32(GAME_GP + 0x16c8, x);
                c.bus.write32(x, x + 0x40);
                c.bus.write32(x + 0x48, base);
                c.bus.write32(GAME_GP + 0xce8, vt);
                c.bus
                    .write32(vt + 0x290, if seed % 3 == 0 { 0 } else { target });
                for lo in 0..4u32 {
                    c.bus
                        .write32(base + 4 * lo, if lo == 1 || lo == 2 { desc } else { 0 });
                }
                c.bus.write32(desc + 0x1c, high);
                for hi in 0..4u32 {
                    c.bus
                        .write32(high + 4 * hi, if hi == 1 { (target >> 2) << 8 } else { 0 });
                }
                c.bus
                    .write32(target + 0xc, if seed % 5 < 2 { 0 } else { 1 });
                c.bus.write32(target + 0x10, r.next_u32());
            },
        );
    }

    /// The career flag block (initialised or not, and a known or unknown
    /// first value), the per-flag types, the flags word and an argument list
    /// of -1..3 entries; both store forms (raw word and float).
    #[test]
    fn wscript_fn_get_progress_flag_bit_matches() {
        use crate::difftest::fakes::machine;
        const RET: u32 = DATA + 0x100;
        const ENTRIES: u32 = DATA + 0x200;
        check(
            ee::s10::f_003061b0__generated,
            wscript_fn_get_progress_flag_bit,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_61b0, 0);
                c.pc = 0x0030_61b0;
                let gp = GAME_GP;
                // An uninitialised block is zeroed by the function itself.
                c.bus.write32(
                    gp + OC324_CAREER_FLAGS_INIT,
                    if seed % 3 == 0 { 0 } else { 1 },
                );
                let key = match seed % 6 {
                    0 => 0u32,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 0xffff_ffff,
                    _ => r.next_u32() % 4,
                };
                c.bus.write32(gp + OC324_CAREER_FLAGS, key);
                c.bus.write32(gp + OC324_CAREER_FLAGS + 4, r.next_u32());
                // The career flags word the bit is read from.
                c.bus.write32(0x0053_08d0, r.next_u32());
                // The per-slot types the entries are compared against.
                for i in 0..4u32 {
                    let ty = if (i + seed as u32) % 3 == 0 {
                        2
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(gp + 0xcb8 + 4 * i, ty);
                }
                let n = (seed % 5) as i32 - 1;
                for i in 0..4u32 {
                    let e = ENTRIES + 0x10 * i;
                    c.bus.write32(e, (seed + i) % 4);
                    c.bus.write32(e + 4, r.next_u32() % 1000);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(RET);
                c.g[A1][0] = n as i64 as u64;
                c.g[A2][0] = sx(ENTRIES);
            },
        );
    }

    /// The mission description (a -1 id or a real one), the mission id and
    /// value words, and a script manager whose command answers -1 at once
    /// (the value is never 0xf / 0x13 and the game state is never 4), so
    /// both the "no command" copy and the command path run.
    #[test]
    fn wscript_fn_mission_command_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::{machine, FAKE};
        const RET: u32 = DATA + 0x100;
        const ENTRIES: u32 = DATA + 0x200;
        const MAN: u32 = DATA + 0x400;
        const BE_LIB: u32 = DATA + 0x1000;
        const BE_IFACE: u32 = DATA + 0x1100;
        const BE_VT: u32 = DATA + 0x1200;
        check(
            ee::s10::f_003012f0__generated,
            wscript_fn_mission_command,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_12f0, 0);
                elf_image(c);
                c.pc = 0x0030_12f0;
                let gp = GAME_GP;
                // The static description (id, value, reserved) and its flag.
                c.bus.write32(
                    OC324_MISSION_DESC,
                    if seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 64
                    },
                );
                c.bus.write32(OC324_MISSION_DESC + 4, 5 + seed as u32 % 7);
                c.bus.write32(OC324_MISSION_DESC + 8, r.next_u32());
                c.bus
                    .write32(gp + 0x25c8, if seed % 3 == 0 { 0 } else { 1 });
                // The per-slot types of the argument expansion.
                for i in 0..4u32 {
                    let ty = if (i + seed as u32) % 3 == 0 {
                        2
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(gp + 0xb80 + 4 * i, ty);
                }
                // The script manager and the back end.
                c.bus.write32(gp + 0xce8, MAN);
                c.bus.write32(gp - 0x1f04, BE_LIB);
                c.bus.write32(gp - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, BE_IFACE);
                c.bus.write32(BE_IFACE + 4, BE_VT);
                c.bus.write16(BE_VT + 8, 0);
                c.bus.write32(BE_VT + 0xc, FAKE + 0x80);
                c.bus.write8(0x0053_5c10, (r.next_u32() % 3) as u8);
                // Entries only touch slots 2 and 3: the id and value words
                // stay as described.
                let n = (seed % 4) as i32 - 1;
                for i in 0..3u32 {
                    let e = ENTRIES + 0x10 * i;
                    c.bus.write32(e, 2 + i % 2);
                    c.bus.write32(e + 4, r.next_u32() % 1000);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(RET);
                c.g[A1][0] = n as i64 as u64;
                c.g[A2][0] = sx(ENTRIES);
            },
        );
    }

    /// A description block with ids of -1 and of a table entry, an object
    /// with and without a modifier node, over the game image and a test
    /// heap (the constructor allocates).
    #[test]
    fn world_obj_factory_mesh_anim_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::machine;
        const RET: u32 = DATA + 0x100;
        const ENTRIES: u32 = DATA + 0x200;
        const INST: u32 = DATA + 0x8000;
        const INST_HDR: u32 = DATA + 0x8100;
        const TABLE: u32 = DATA + 0x8200;
        const ENTRY: u32 = DATA + 0x8300;
        const SLOTS: u32 = DATA + 0x8400;
        const OBJECT: u32 = DATA + 0x8800;
        check(
            ee::s10::f_002fcff0__generated,
            world_obj_factory_mesh_anim,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_cff0, 0);
                elf_image(c);
                crate::game::bx::bxstring::tests::heap(c);
                c.pc = 0x002f_cff0;
                let gp = GAME_GP;
                // The static description and its flag.
                c.bus.write32(OC324_MESH_ANIM_DESC, r.next_u32());
                c.bus.write32(OC324_MESH_ANIM_DESC + 4, seed as u32 % 8);
                for i in 2..14u32 {
                    c.bus.write32(OC324_MESH_ANIM_DESC + 4 * i, r.f32(10.0));
                }
                c.bus
                    .write32(gp + OC324_MESH_ANIM_INIT, if seed % 3 == 0 { 0 } else { 1 });
                // The per-slot types of the argument expansion.
                for i in 0..16u32 {
                    let ty = if (i + seed as u32) % 4 == 0 {
                        2
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(0x0044_6060 + 4 * i, ty);
                }
                // The instance tables the id resolves through.
                c.bus.write32(gp + 0x16c8, INST);
                c.bus.write32(INST, INST_HDR);
                c.bus.write32(INST_HDR + 8, TABLE);
                for i in 0..4u32 {
                    c.bus.write32(TABLE + 4 * i, ENTRY);
                }
                c.bus.write32(ENTRY + 0x1c, SLOTS);
                for i in 0..8u32 {
                    let word = match (seed + i) % 3 {
                        0 => 0,
                        _ => (OBJECT >> 2) << 8 | (r.next_u32() & 0xff),
                    };
                    c.bus.write32(SLOTS + 4 * i, word);
                }
                // The object the modifier attaches to: no node (skip the
                // active-node test) or a node that already is a type-9
                // modifier (the test answers 1, so it is skipped).
                let node = DATA + 0x8900;
                let node_vt = DATA + 0x8a00;
                c.bus
                    .write32(OBJECT + 0xc, if seed % 4 == 0 { 0 } else { node });
                c.bus.write32(OBJECT + 8, 0);
                c.bus.write32(node, 9);
                c.bus.write32(node + 0xc, node_vt);
                c.bus.write16(node_vt + 0x50, 0);
                c.bus.write32(node_vt + 0x54, 0x002a_fa58);
                c.bus.write32(gp + 0xce8, DATA + 0x8b00);
                // The argument block: id (or -1) and model handle.
                let n = (seed % 4) as i32 - 1;
                for i in 0..4u32 {
                    let e = ENTRIES + 0x10 * i;
                    c.bus.write32(e, 14 + i % 2);
                    c.bus.write32(e + 4, r.next_u32() % 100);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(RET);
                c.g[A1][0] = n as i64 as u64;
                c.g[A2][0] = sx(ENTRIES);
            },
        );
    }

    pub(crate) const RET_ATTACH: u32 = 3;

    oc327_hook!(
        oc327_fake_attach,
        OC327_PREV_ATTACH,
        0x002f_ae38,
        RET_ATTACH,
        ee::s10::f_002fae38__generated
    );

    oc327_hook!(
        oc327_fake_base_ctor,
        OC327_PREV_BASE_CTOR,
        0x0034_d9b0,
        RET_BASE_CTOR,
        ee::s11::f_0034d9b0__generated
    );

    /// `AnimTeeter`'s create path: the allocator, `checkActiveNode` and the
    /// animated object's base constructor are faked.
    pub(crate) fn oc327_install_teeter() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_002FAE38),
            oc327_fake_attach,
            &OC327_PREV_ATTACH,
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0034D9B0),
            oc327_fake_base_ctor,
            &OC327_PREV_BASE_CTOR,
        );
        oc327_install_alloc();
    }

    /// `WorldObjFactory_AnimTeeter`: the template init on the first call, the
    /// entry overlay (type match, type 2 as a float, plain words), both id
    /// paths, and the already-attached vs create branches (faked ctor).
    #[test]
    fn world_obj_factory_anim_teeter_matches() {
        oc327_install_teeter();
        covered(
            ee::s10::f_002fb498__generated,
            world_obj_factory_anim_teeter,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002f_b498, 4);
                c.pc = 0x002f_b498;
                c.bus.write32(OC327_CTRL, OC327_MAGIC);
                c.bus.write32(OC327_RET + 4 * RET_ATTACH, (seed % 2) as u32);
                c.bus.write32(OC327_RET + 4 * RET_ALLOC, DATA + 0x8_0000);
                c.bus.write32(OC327_RET + 4 * RET_BASE_CTOR, 0);
                let gp = GAME_GP;
                c.g[GP][0] = gp as u64;
                // The one-time template flag (skipped when set) and the two
                // `.lit4` constants the init copies.
                let fresh = seed % 3 == 0;
                c.bus.write32(gp + 0x24d8, if fresh { 0 } else { 1 });
                c.bus.write32(gp.wrapping_sub(0x339c), 0x3f19_999a); // 0.6f
                c.bus.write32(gp.wrapping_sub(0x3398), 0x3ca3_d70a); // 0.02f
                                                                     // The entry type table (0x445ed0).
                for i in 0..12u32 {
                    c.bus.write32(0x0044_5ed0 + i * 4, (i * 7 + seed) % 4);
                }
                let template = 0x004f_b400;
                let id = 0x21 + (seed % 7);
                let word = match seed % 3 {
                    0 => 0,
                    1 => 0x100,
                    _ => 0x1_0000,
                };
                if !fresh {
                    c.bus.write32(template, id);
                    for i in 1..12u32 {
                        c.bus.write32(template + i * 4, r.next_u32());
                    }
                }
                // id != -1: `(*(u32**)(s + 0x1c))[id >> 8] >> 8`, a reference.
                let holder = DATA + 0x1000;
                let slots = DATA + 0x1100;
                let array = DATA + 0x1200;
                let node = DATA + 0x1400;
                c.bus.write32(gp + 0x16c8, holder);
                c.bus.write32(holder, slots);
                c.bus.write32(slots + 8, array);
                c.bus.write32(array + (id & 0xff) * 4, node);
                c.bus.write32(node + 0x1c, DATA + 0x1500);
                c.bus.write32(DATA + 0x1500 + (id >> 8) * 4, word);
                // obj = (i << 2); its +0xc word is the node the factory probes.
                let obj = (word >> 8) << 2;
                c.bus
                    .write32(obj + 0xc, if seed % 4 == 0 { 0 } else { node });
                // id == -1: the world's "current object" at +0x290.
                c.bus.write32(gp + 0xce8, DATA + 0x1000);
                c.bus.write32(DATA + 0x1290, DATA + 0x1600);
                c.bus.write32(DATA + 0x160c, node);
                c.bus.write32(DATA + 0x1600 + 0x80, 0);
                // `0x3421a0` divides by these globals.
                c.bus.write32(gp + 0x2a74, DATA + 0x1800);
                c.bus.write32(DATA + 0x1810, 0x40_0000);
                for i in 0..4u32 {
                    // The intersect info the ctor reads (+8, +0x24, +4, +0xc..).
                    c.bus.write32(node + 8 + i * 4, r.next_u32());
                }
                // The entries: idx 0..11, val, type.
                let e = DATA + 0x3000;
                let n = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 3,
                    _ => (seed % 6) as u32,
                };
                for i in 0..12u32 {
                    let ent = e + i * 16;
                    c.bus.write32(ent, i % 12);
                    c.bus.write32(ent + 4, r.next_u32());
                    c.bus.write32(ent + 0xc, (seed + i) % 3);
                }
                let out = DATA + 0x7000;
                for i in 0..3u32 {
                    c.bus.write32(out + i * 4, r.next_u32());
                }
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(e);
                c.g[A3][0] = 0;
            },
        );
    }

    // -- WScriptFn_AddHaloModifier / AddRailModifier ----------------------

    /// The record template guard, the id tables at `gp+0x16c8` / `gp+0xce8`,
    /// the object → node → vtable chain, and the script parameters both
    /// modifier ports consume. `types` is the parameter type table.
    /// Modes: 0 the record keeps id -1, 1 the id resolves to nothing, 2 the
    /// object has no node, 3 the node's halo/rail interface answers `0`.
    pub(crate) fn modifier_fixture(c: &mut Cpu, seed: u32, types: u32) {
        let params = DATA + 0x1000;
        let node = DATA + 0x3000;
        let vt = DATA + 0x3100;
        let obj = DATA + 0x3200;
        let holder = DATA + 0x3400;
        let slots = DATA + 0x3500;
        let second = DATA + 0x3600;
        let id = (2u32 << 8) | 1;
        let mode = (seed / 2) % 4;
        let inited = (seed >> 1) & 1;
        let n = if mode == 0 { (seed / 8) % 2 } else { 3 };

        c.bus.write32(GAME_GP + 0x2764, inited);
        c.bus.write32(GAME_GP + 0x2540, inited);
        // Type table: index 0 integer, index 1 float conversion, index 2
        // matching integer.
        for (i, t) in [0u32, 2, 2, 0, 0, 0, 0, 0].iter().enumerate() {
            c.bus.write32(types + 4 * i as u32, *t);
        }
        if n > 0 {
            if mode == 0 {
                // A parameter that leaves the record's id alone.
                c.bus.write32(params, 3);
                c.bus.write32(params + 4, 0x4142_0000);
                c.bus.write32(params + 0xc, 0);
            } else {
                c.bus.write32(params, 0); // idx 0: the object id
                c.bus.write32(params + 4, id);
                c.bus.write32(params + 0xc, 3);
            }
        }
        if n > 1 {
            c.bus.write32(params + 0x10, 1); // idx 1: a float
            c.bus.write32(params + 0x14, 0x4120_0000);
            c.bus.write32(params + 0x1c, 3);
        }
        if n > 2 {
            c.bus.write32(params + 0x20, 2); // idx 2: an integer
            c.bus.write32(params + 0x24, 0x1122_3344);
            c.bus.write32(params + 0x2c, 2);
        }
        // The "current object" for id -1: *(*(gp+0xce8)+0x290).
        let p = DATA + 0x3700;
        c.bus.write32(GAME_GP + 0xce8, p);
        c.bus.write32(p + 0x290, DATA + 0x3780);
        // The id table: *D_004A47B8 -> holder -> slots -> slot -> second.
        c.bus.write32(GAME_GP + 0x16c8, holder);
        c.bus.write32(holder, 0);
        c.bus.write32(holder + 8, slots);
        c.bus.write32(slots, 0);
        c.bus
            .write32(slots + 4, if mode >= 2 { slots + 0x50 } else { 0 });
        c.bus.write32(slots + 0x50 + 0x1c, second);
        c.bus.write32(second + 2 * 4, (obj >> 2) << 8);
        c.bus.write32(obj + 0xc, if mode >= 3 { node } else { 0 });
        c.bus.write32(node + 0xc, vt);
        c.bus.write16(vt + 0x80, 0);
        // A real leaf that answers 0, so the add-modifier call is skipped.
        c.bus.write32(vt + 0x84, 0x0014_06c0);
        c.g[A0][0] = sx(DATA + 0x200);
        c.g[A1][0] = n as u64;
        c.g[A2][0] = sx(params);
    }

    #[test]
    fn wscript_fn_add_halo_modifier_matches() {
        check(
            ee::s10::f_003057c0__generated,
            wscript_fn_add_halo_modifier,
            240,
            &|c, seed| {
                let _r = machine(c, seed, 0x30_57c0, 0);
                c.pc = 0x0030_57c0;
                modifier_fixture(c, seed, 0x0044_0000 + HALO_PARAM_TYPES_OFF);
            },
        );
    }

    #[test]
    fn wscript_fn_add_rail_modifier_matches() {
        check(
            ee::s10::f_002ff1c8__generated,
            wscript_fn_add_rail_modifier,
            240,
            &|c, seed| {
                let _r = machine(c, seed, 0x2f_f1c8, 0);
                c.pc = 0x002f_f1c8;
                modifier_fixture(c, seed, 0x0044_0000 + RAIL_PARAM_TYPES_OFF);
            },
        );
    }

    /// WorldObjFactory_ParticleNode: template initialised or not; 0-3
    /// params (or a negative count) with matching / mismatching types
    /// (int to float conversions); the default object or one from the
    /// object table (missing slot, empty entry, found), with and without a
    /// modifier that accepts (0), frees (> 0) or keeps (< 0) the node.
    /// Gated fakes for alloc, delete, the particle-node ctor and
    /// AttachModifier (sign-extended result); real memcpy.
    #[test]
    fn world_obj_factory_particle_node_matches() {
        use crate::difftest::fakes::{fake, fall_back, hook};
        use crate::game::object::movenode::tests::{oc338_fake, OC338_CTRL, OC338_MAGIC};
        use std::ptr::addr_of_mut;
        use std::sync::atomic::AtomicUsize;
        const BASE: u32 = DATA + 0x20_0000;
        const PARAMS: u32 = BASE;
        const RESULT: u32 = BASE + 0x100;
        const ATTACH_RESULT: u32 = BASE + 0x120;
        const HOLDER: u32 = BASE + 0x400;
        const TABLE_PTR: u32 = BASE + 0x800;
        const TABLE: u32 = BASE + 0x810;
        const SLOTS: u32 = BASE + 0x900;
        const SLOT: u32 = BASE + 0xa00;
        const ENTRIES: u32 = BASE + 0xb00;
        const OBJ: u32 = BASE + 0x1_0000;
        oc338_fake!(
            oc374_fake_wofpn_alloc,
            OC374_PREV_WOFPN_ALLOC,
            0x0031_7d70,
            ee::s10::f_00317d70__generated
        );
        oc338_fake!(
            oc374_fake_wofpn_delete,
            OC374_PREV_WOFPN_DELETE,
            0x0031_7e50,
            ee::s10::f_00317e50__generated
        );
        oc338_fake!(
            oc374_fake_wofpn_ctor,
            OC374_PREV_WOFPN_CTOR,
            0x0035_78a8,
            ee::s11::f_003578a8__generated
        );
        static OC374_PREV_ATTACH: AtomicUsize = AtomicUsize::new(0);
        fn oc374_fake_attach_modifier(c: &mut Cpu) {
            if c.bus.read32(OC338_CTRL) != OC338_MAGIC {
                return fall_back(c, &OC374_PREV_ATTACH, ee::s10::f_002fae38__generated);
            }
            let ra = c.g[RA][0] as u32;
            fake(c, 0x002f_ae38, ra);
            if c.pc == ra {
                c.g[V0][0] = sx(c.bus.read32(ATTACH_RESULT));
            }
        }
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc374_fake_wofpn_alloc,
            &OC374_PREV_WOFPN_ALLOC,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc374_fake_wofpn_delete,
            &OC374_PREV_WOFPN_DELETE,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_003578A8),
            oc374_fake_wofpn_ctor,
            &OC374_PREV_WOFPN_CTOR,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_002FAE38),
            oc374_fake_attach_modifier,
            &OC374_PREV_ATTACH,
        );
        covered(
            ee::s10::f_002fd420__generated,
            world_obj_factory_particle_node,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_d420, 6);
                c.pc = 0x002f_d420;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                let gp = r32(c, GP);
                c.g[A0][0] = sx(RESULT);
                for i in 0..3 {
                    c.bus.write32(WOFPN_RESULT + 4 * i, r.next_u32());
                }
                let initialised = seed % 2;
                c.bus.write32(gp + WOFPN_INIT_GUARD, initialised);
                for off in (0..0xe0).step_by(4) {
                    let f = r.f32(100.0);
                    c.bus.write32(WOFPN_TEMPLATE + off, f);
                }
                for i in 0..56 {
                    c.bus.write32(WOFPN_PARAM_TYPES + 4 * i, r.next_u32() % 4);
                }
                // The object: default (-1) or table entry (id >> 8, slot).
                let which = seed / 2 % 5;
                let id: u32 = match which {
                    0 => u32::MAX,
                    1 => 3,          // slot 3 is empty
                    2 => 2 << 8 | 1, // entry 2 is empty
                    _ => 1 << 8 | 1,
                };
                if initialised == 1 {
                    c.bus.write32(WOFPN_TEMPLATE + WOFPN_NODE_OBJECT, id);
                }
                let n = [0u32, 1, 2, 3, 0xffff_ffff][(seed / 10 % 5) as usize];
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(PARAMS);
                for i in 0..3 {
                    let p = PARAMS + 0x10 * i;
                    let idx = if i == 0 && initialised == 0 {
                        54
                    } else {
                        r.next_u32() % 56
                    };
                    c.bus.write32(p, idx);
                    let value = if idx == 54 { id } else { r.next_u32() };
                    c.bus.write32(p + 4, value);
                    c.bus.write32(p + 8, r.next_u32());
                    let ty = if idx == 54 {
                        c.bus.read32(WOFPN_PARAM_TYPES + 4 * 54)
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(p + 0xc, ty);
                }
                let obj = if seed % 7 == 0 { 0 } else { OBJ };
                c.bus.write32(gp + WOFPN_DEFAULT_HOLDER, HOLDER);
                c.bus.write32(HOLDER + 0x290, obj);
                c.bus.write32(gp + WOFPN_OBJECT_TABLE, TABLE_PTR);
                c.bus.write32(TABLE_PTR, TABLE);
                c.bus.write32(TABLE + 8, SLOTS);
                for i in 0..4 {
                    c.bus.write32(SLOTS + 4 * i, if i == 3 { 0 } else { SLOT });
                }
                c.bus.write32(SLOT + 0x1c, ENTRIES);
                c.bus
                    .write32(ENTRIES + 4, (OBJ >> 2) << 8 | (r.next_u32() & 0xff));
                c.bus.write32(ENTRIES + 8, r.next_u32() & 0xff);
                let modifier = if seed % 3 == 0 { 0 } else { DATA + 0x30_0000 };
                c.bus.write32(OBJ + 0xc, modifier);
                let attach = [0u32, 0, 1, 0xffff_ffff][(seed / 3 % 4) as usize];
                c.bus.write32(ATTACH_RESULT, attach);
            },
        );
    }

    use crate::difftest::fakes::vtable;

    /// WScriptFn_GetObjectSlot30: initialised or not, -1..3 argument entries
    /// (raw and float stores), the world object (id -1), a found object with
    /// or without a component (fake slot 0xf0), an unknown descriptor or
    /// entry.
    #[test]
    fn wscript_fn_get_object_slot30_matches() {
        check(
            ee::s10::f_00301680__generated,
            wscript_fn_get_object_slot30,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x301680, 1);
                c.pc = 0x0030_1680;
                let gp = GAME_GP;
                let ret = DATA + 0x100;
                let entries = DATA + 0x200;
                let x = DATA + 0x2300;
                let base = DATA + 0x2400;
                let high = DATA + 0x2800;
                let desc = DATA + 0x2c00;
                let world = DATA + 0x2e00;
                let target = DATA + 0x3000;
                let comp = DATA + 0x3100;
                c.g[A0][0] = sx(ret);
                let n = [-1i64, 0, 1, 2, 3][(seed % 5) as usize];
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(entries);
                for i in 0..4u32 {
                    c.bus.write32(gp + WSCRIPT_GOS30_TYPES + 4 * i, i % 3);
                }
                c.bus.write32(gp + WSCRIPT_GOS30_INIT, (seed / 5) % 2);
                c.bus.write8(gp + WSCRIPT_GOS30_TAG, r.next_u32() as u8);
                let id = match (seed / 2) % 5 {
                    0 => 0xffff_ffffu32, // the world object
                    1 => 0x0000_0101,    // lo=1, hi=1: found
                    2 => 0x0000_0103,    // lo=3: no such descriptor
                    3 => 0x0000_0201,    // hi=2: an empty entry
                    _ => 0x0000_0101,
                };
                c.bus.write32(gp + WSCRIPT_GOS30_IDS, id);
                c.bus.write32(gp + WSCRIPT_GOS30_IDS + 4, r.next_u32());
                for i in 0..4u32 {
                    let e = entries + 16 * i;
                    c.bus.write32(e, (seed >> (2 * i)) % 2);
                    c.bus.write32(e + 4, r.next_u32());
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, (seed >> (3 + i)) % 4);
                }
                c.bus.write32(gp + WSCRIPT_GOS30_TABLE, x);
                c.bus.write32(x, x + 0x40);
                c.bus.write32(x + 0x48, base);
                c.bus.write32(gp + WSCRIPT_GOS30_WORLD, world);
                c.bus
                    .write32(world + 0x290, if seed % 7 == 0 { 0 } else { target });
                for lo in 0..4u32 {
                    c.bus
                        .write32(base + 4 * lo, if lo == 1 || lo == 2 { desc } else { 0 });
                }
                c.bus.write32(desc + 0x1c, high);
                for hi in 0..4u32 {
                    c.bus
                        .write32(high + 4 * hi, if hi == 1 { (target >> 2) << 8 } else { 0 });
                }
                c.bus
                    .write32(target + 0xc, if seed % 3 == 0 { 0 } else { comp });
                let vt = vtable(c, &mut r, &[0xf0]);
                c.bus.write32(comp + 0xc, vt);
            },
        );
    }

    use crate::difftest::fakes;

    pub(crate) use crate::game::ai::rider::tests::w32_2 as w32;
    pub(crate) use crate::game::world::worldview::tests::oc410_machine;
    pub(crate) use crate::game::world::worldview::tests::oc410_result;

    pub(crate) use crate::game::ai::ai::tests::oc414_env;

    /// `WScriptFn_CreateFlag`: static template init, the override loop (raw
    /// and float-converted), the object lookup and the node allocation.
    #[test]
    fn w_script_fn_create_flag_matches() {
        const ENTRIES: u32 = DATA + 0x1000;
        const WORLD_CELL: u32 = DATA + 0x2000;
        const WORLD_1: u32 = DATA + 0x2100;
        const WORLD_ARR: u32 = DATA + 0x2200;
        const SLOT_OBJ: u32 = DATA + 0x2300;
        const SLOT_TAB: u32 = DATA + 0x2400;
        const DEFAULT_CELL: u32 = DATA + 0x2500;
        const OBJ: u32 = DATA + 0x3000;
        const GP_FIRST_INIT: u32 = crate::difftest::GAME_GP + 0x2500;
        check(
            ee::s10::f_002fc9c8__generated,
            w_script_fn_create_flag,
            400,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x002f_c9c8, 5);
                c.pc = 0x002f_c9c8;
                c.g[A0][0] = (DATA + 0x500) as u64;
                let n = seed % 4;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = ENTRIES as u64;
                w32(c, GP_FIRST_INIT, (seed % 2 == 0) as u32);
                for idx in 0..9u32 {
                    w32(
                        c,
                        0x0044_5fc0 + 4 * idx,
                        if idx == 0 { 0 } else { r.next_u32() % 4 },
                    );
                }
                let (lo, hi) = (seed % 3, seed % 2);
                for i in 0..n {
                    let e = ENTRIES + 0x10 * i;
                    if i == 0 {
                        w32(c, e, 0);
                        w32(c, e + 4, (hi << 8) | lo);
                        w32(c, e + 0xc, 0);
                    } else {
                        w32(c, e, 1 + r.next_u32() % 8);
                        w32(c, e + 4, r.next_u32() % 1000);
                        w32(c, e + 0xc, r.next_u32() % 4);
                    }
                }
                w32(c, crate::difftest::GAME_GP + 0x16c8, WORLD_CELL);
                w32(c, WORLD_CELL, WORLD_1);
                w32(c, WORLD_1 + 8, WORLD_ARR);
                for i in 0..3 {
                    w32(
                        c,
                        WORLD_ARR + 4 * i,
                        if seed % 5 == 2 && i == lo {
                            0
                        } else {
                            SLOT_OBJ
                        },
                    );
                }
                w32(c, SLOT_OBJ + 0x1c, SLOT_TAB);
                for i in 0..2 {
                    let x = if seed % 9 == 4 {
                        0
                    } else {
                        ((OBJ >> 2) << 8) | (r.next_u32() & 0xff)
                    };
                    w32(c, SLOT_TAB + 4 * i, x);
                }
                w32(c, crate::difftest::GAME_GP + 0xce8, DEFAULT_CELL);
                w32(c, DEFAULT_CELL + 0x290, if seed % 6 == 5 { 0 } else { OBJ });
                w32(c, OBJ + 0xc, if seed % 4 == 3 { 0 } else { DATA + 0x3400 });
                w32(c, oc410_result(30), (seed % 3 == 0) as u32);
                w32(c, oc410_result(12), DATA + 0x3800);
                for k in 0..3 {
                    w32(c, 0x004c_9098 + 4 * k, r.next_u32());
                }
            },
        );
    }

    /// `WScriptFn_IsEventType`: first call (the record defaults are set) and
    /// later ones, 0..3 arguments whose type matches the table or not
    /// (matching: integer store; table type 2: float conversion).
    #[test]
    fn w_script_fn_is_event_type_matches() {
        check(
            ee::s10::f_00302778__generated,
            w_script_fn_is_event_type,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x302778, 0);
                c.pc = 0x0030_2778;
                oc414_env(c);
                let gp = GAME_GP;
                c.bus.write32(gp + 0x2660, (seed % 2) as u32);
                c.bus.write8(gp + 0x265c, r.next_u32() as u8);
                for i in 0..4u32 {
                    c.bus.write32(gp + 0xbe8 + 4 * i, 2 + r.next_u32() % 2);
                }
                let args = DATA + 0x200;
                let n = seed % 4;
                for i in 0..n {
                    let rec = args + 16 * i;
                    c.bus.write32(rec, r.next_u32() % 2);
                    c.bus.write32(rec + 4, r.next_u32() % 100);
                    c.bus.write32(rec + 0xc, 2 + r.next_u32() % 2);
                }
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(args);
            },
        );
    }

    /// Control block of the gated fakes: the gate word, the call counter, the
    /// call index that unwinds, forced results (`+0x10 + 4 * id`), forced
    /// `f0` results (`+0x80 + 4 * id`) and the call log (`+0x100`).
    pub(crate) const OC417_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC417_MAGIC: u32 = 0x0c41_7a17;

    /// A gated fake callee: records its arguments, returns the forced result
    /// of its id, and unwinds at the chosen call index.
    pub(crate) fn oc417_fake(c: &mut Cpu, id: u32) {
        let n = c.bus.read32(OC417_CTRL + 4);
        c.bus.write32(OC417_CTRL + 4, n.wrapping_add(1));
        let p = OC417_CTRL + 0x100 + (n & 0x3f) * 0x40;
        c.bus.write32(p, id);
        c.bus.write32(p + 4, c.pc);
        for i in 0..4usize {
            let v = c.g[A0 + i][0];
            c.bus.write64(p + 8 + 8 * i as u32, v);
        }
        let (f12, f13) = (c.f[12], c.f[13]);
        c.bus.write32(p + 0x28, f12);
        c.bus.write32(p + 0x2c, f13);
        let (ra, spv, cyc) = (c.g[RA][0] as u32, c.g[SP][0] as u32, c.cycles);
        c.bus.write32(p + 0x30, ra);
        c.bus.write32(p + 0x34, spv);
        c.bus.write64(p + 0x38, cyc);
        let forced = c.bus.read32(OC417_CTRL + 0x10 + 4 * id);
        c.g[V0][0] = sx(forced);
        c.g[V1][0] = 0x5151_0000 + n as u64;
        c.f[0] = c.bus.read32(OC417_CTRL + 0x80 + 4 * id);
        c.cycles += 5;
        if n == c.bus.read32(OC417_CTRL + 8) {
            c.pc = 0xdead_0000 | n << 4;
        } else {
            c.pc = c.g[RA][0] as u32;
        }
    }

    macro_rules! oc417_gated {
        ($name:ident, $prev:ident, $id:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC417_CTRL) == OC417_MAGIC {
                    return oc417_fake(c, $id);
                }
                fakes::fall_back(c, &$prev, $gen)
            }
        };
    }

    // Fake ids (index into the forced-result table).
    pub(crate) const FK_BE_INSTANCE: u32 = 0;

    pub(crate) const FK_BE_INTERFACE: u32 = 1;

    pub(crate) const FK_READ_U32: u32 = 2;

    pub(crate) const FK_NEW_NAMED: u32 = 3;

    pub(crate) const FK_BOLT_COUNT: u32 = 4;

    pub(crate) const FK_BOLTS: u32 = 5;

    pub(crate) const FK_HEAP_FREE: u32 = 6;

    pub(crate) const FK_QUEUE_SECTION: u32 = 7;

    pub(crate) const FK_ADD_MAGNET: u32 = 8;

    pub(crate) const FK_LUNO_DTOR: u32 = 9;

    pub(crate) const FK_LUNO_ASSIGN: u32 = 10;

    pub(crate) const FK_MEM_ALLOC: u32 = 11;

    pub(crate) const FK_BE_TABLE: u32 = 12;

    pub(crate) const FK_BE_DB_ENTRY: u32 = 13;

    pub(crate) const FK_GET_CHARACTER: u32 = 14;

    pub(crate) const FK_SET_SUB_STATE: u32 = 15;

    pub(crate) const FK_SET_MAIN_STATE: u32 = 16;

    pub(crate) const FK_RESET_FX: u32 = 17;

    pub(crate) const FK_BOOST_INPUT: u32 = 18;

    pub(crate) const FK_BUILD_MATRIX: u32 = 19;

    pub(crate) const FK_QUAT_EULER: u32 = 20;

    pub(crate) const FK_ANIM_LAYER_A: u32 = 21;

    pub(crate) const FK_ANIM_LAYER_B: u32 = 22;

    pub(crate) const FK_ANIM_RESET: u32 = 23;

    pub(crate) const FK_ANIM_PLAY: u32 = 24;

    pub(crate) const FK_SINCOS: u32 = 25;

    oc417_gated!(
        fake_be_instance,
        PREV_BE_INSTANCE,
        FK_BE_INSTANCE,
        ee::s01::f_0014dc80__generated
    );

    oc417_gated!(
        fake_be_interface,
        PREV_BE_INTERFACE,
        FK_BE_INTERFACE,
        ee::s01::f_0014dd58__generated
    );

    oc417_gated!(
        fake_read_u32,
        PREV_READ_U32,
        FK_READ_U32,
        ee::s09::f_002d2510__generated
    );

    oc417_gated!(
        fake_new_named,
        PREV_NEW_NAMED,
        FK_NEW_NAMED,
        ee::s10::f_00317e30__generated
    );

    oc417_gated!(
        fake_bolt_count,
        PREV_BOLT_COUNT,
        FK_BOLT_COUNT,
        ee::s01::f_0014d988__generated
    );

    oc417_gated!(
        fake_bolts,
        PREV_BOLTS,
        FK_BOLTS,
        ee::s01::f_0014d998__generated
    );

    oc417_gated!(
        fake_heap_free,
        PREV_HEAP_FREE,
        FK_HEAP_FREE,
        ee::s07::f_002723a8__generated
    );

    oc417_gated!(
        fake_queue_section,
        PREV_QUEUE_SECTION,
        FK_QUEUE_SECTION,
        ee::s05::f_0022e298__generated
    );

    oc417_gated!(
        fake_add_magnet,
        PREV_ADD_MAGNET,
        FK_ADD_MAGNET,
        ee::s11::f_003559f8__generated
    );

    oc417_gated!(
        fake_luno_dtor,
        PREV_LUNO_DTOR,
        FK_LUNO_DTOR,
        ee::s05::f_00224df0__generated
    );

    oc417_gated!(
        fake_luno_assign,
        PREV_LUNO_ASSIGN,
        FK_LUNO_ASSIGN,
        ee::s05::f_00225068__generated
    );

    oc417_gated!(
        fake_mem_alloc,
        PREV_MEM_ALLOC,
        FK_MEM_ALLOC,
        ee::s10::f_00317d70__generated
    );

    oc417_gated!(
        fake_be_table,
        PREV_BE_TABLE,
        FK_BE_TABLE,
        ee::s01::f_0014bdb8__generated
    );

    oc417_gated!(
        fake_be_db_entry,
        PREV_BE_DB_ENTRY,
        FK_BE_DB_ENTRY,
        ee::s01::f_0014ad50__generated
    );

    oc417_gated!(
        fake_get_character,
        PREV_GET_CHARACTER,
        FK_GET_CHARACTER,
        ee::s00::f_00123128__generated
    );

    oc417_gated!(
        fake_set_sub_state,
        PREV_SET_SUB_STATE,
        FK_SET_SUB_STATE,
        ee::s00::f_0011fec8__generated
    );

    oc417_gated!(
        fake_set_main_state,
        PREV_SET_MAIN_STATE,
        FK_SET_MAIN_STATE,
        ee::s00::f_0011fe78__generated
    );

    oc417_gated!(
        fake_reset_fx,
        PREV_RESET_FX,
        FK_RESET_FX,
        ee::s00::f_00111890__generated
    );

    oc417_gated!(
        fake_boost_input,
        PREV_BOOST_INPUT,
        FK_BOOST_INPUT,
        ee::s00::f_00114130__generated
    );

    oc417_gated!(
        fake_build_matrix,
        PREV_BUILD_MATRIX,
        FK_BUILD_MATRIX,
        ee::s00::f_0011e098__generated
    );

    oc417_gated!(
        fake_quat_euler,
        PREV_QUAT_EULER,
        FK_QUAT_EULER,
        ee::s00::f_001241c0__generated
    );

    oc417_gated!(
        fake_anim_layer_a,
        PREV_ANIM_LAYER_A,
        FK_ANIM_LAYER_A,
        ee::s10::f_0030e9e0__generated
    );

    oc417_gated!(
        fake_anim_layer_b,
        PREV_ANIM_LAYER_B,
        FK_ANIM_LAYER_B,
        ee::s10::f_0030ea80__generated
    );

    oc417_gated!(
        fake_anim_reset,
        PREV_ANIM_RESET,
        FK_ANIM_RESET,
        ee::s10::f_00311a50__generated
    );

    oc417_gated!(
        fake_anim_play,
        PREV_ANIM_PLAY,
        FK_ANIM_PLAY,
        ee::s10::f_003128e8__generated
    );

    oc417_gated!(
        fake_sincos,
        PREV_SINCOS,
        FK_SINCOS,
        ee::s10::f_0031be50__generated
    );

    /// Put every fake of this module in its override slot (once per
    /// process; with the gate word clear they run what the slot held).
    pub(crate) fn oc417_hook_fakes() {
        use std::ptr::addr_of_mut;
        let slots: [(*mut fn(&mut Cpu), fn(&mut Cpu), &AtomicUsize); 26] = [
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
                fake_be_instance,
                &PREV_BE_INSTANCE,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
                fake_be_interface,
                &PREV_BE_INTERFACE,
            ),
            (
                addr_of_mut!(ee::s09::OVERRIDE_002D2510),
                fake_read_u32,
                &PREV_READ_U32,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00317E30),
                fake_new_named,
                &PREV_NEW_NAMED,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014D988),
                fake_bolt_count,
                &PREV_BOLT_COUNT,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014D998),
                fake_bolts,
                &PREV_BOLTS,
            ),
            (
                addr_of_mut!(ee::s07::OVERRIDE_002723A8),
                fake_heap_free,
                &PREV_HEAP_FREE,
            ),
            (
                addr_of_mut!(ee::s05::OVERRIDE_0022E298),
                fake_queue_section,
                &PREV_QUEUE_SECTION,
            ),
            (
                addr_of_mut!(ee::s11::OVERRIDE_003559F8),
                fake_add_magnet,
                &PREV_ADD_MAGNET,
            ),
            (
                addr_of_mut!(ee::s05::OVERRIDE_00224DF0),
                fake_luno_dtor,
                &PREV_LUNO_DTOR,
            ),
            (
                addr_of_mut!(ee::s05::OVERRIDE_00225068),
                fake_luno_assign,
                &PREV_LUNO_ASSIGN,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00317D70),
                fake_mem_alloc,
                &PREV_MEM_ALLOC,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014BDB8),
                fake_be_table,
                &PREV_BE_TABLE,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014AD50),
                fake_be_db_entry,
                &PREV_BE_DB_ENTRY,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_00123128),
                fake_get_character,
                &PREV_GET_CHARACTER,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_0011FEC8),
                fake_set_sub_state,
                &PREV_SET_SUB_STATE,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_0011FE78),
                fake_set_main_state,
                &PREV_SET_MAIN_STATE,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_00111890),
                fake_reset_fx,
                &PREV_RESET_FX,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_00114130),
                fake_boost_input,
                &PREV_BOOST_INPUT,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_0011E098),
                fake_build_matrix,
                &PREV_BUILD_MATRIX,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_001241C0),
                fake_quat_euler,
                &PREV_QUAT_EULER,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_0030E9E0),
                fake_anim_layer_a,
                &PREV_ANIM_LAYER_A,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_0030EA80),
                fake_anim_layer_b,
                &PREV_ANIM_LAYER_B,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00311A50),
                fake_anim_reset,
                &PREV_ANIM_RESET,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_003128E8),
                fake_anim_play,
                &PREV_ANIM_PLAY,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_0031BE50),
                fake_sincos,
                &PREV_SINCOS,
            ),
        ];
        for (slot, fake, prev) in slots {
            fakes::hook(slot, fake, prev);
        }
    }

    /// Scrambled machine with a game gp, the fakes' gate open and (one seed
    /// in three, if `calls > 0`) a gated fake call that unwinds. The
    /// generic fakes (`fakes::machine`) take the vtable calls; their own
    /// unwinding is chosen with the same `calls`.
    pub(crate) fn oc417_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc417_hook_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC417_CTRL, OC417_MAGIC);
        let unwind = if calls > 0 && seed % 3 == 1 {
            r.next_u32() % calls
        } else {
            u32::MAX
        };
        c.bus.write32(OC417_CTRL + 8, unwind);
        for id in 0..32 {
            c.bus.write32(OC417_CTRL + 0x10 + 4 * id, 0);
        }
        r
    }

    // ----------------------------------------------------------------------
    // WScriptFn_EventTypeMatches

    /// Event type below and above 8; table entry equal to the current event
    /// byte or not; the BE lookups faked (and unwinding at one of them).
    #[test]
    fn wscript_fn_event_type_matches_matches() {
        check(
            ee::s10::f_00303df8__generated,
            wscript_fn_event_type_matches,
            200,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0002, 2);
                c.pc = 0x0030_3df8;
                let current = r.next_u32() % 4;
                c.bus.write8(0x0053_5c10, current as u8);
                for i in 0..8u32 {
                    let v = if r.next_u32() % 3 == 0 {
                        current
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(0x0044_65f8 + 4 * i, v);
                }
                c.g[A0][0] = (seed % 12) as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // WScriptFn_MultiParticleControl / WScriptFn_AddMagnetModifier

    pub(crate) const WS_WORLD: u32 = DATA + 0x2000;

    pub(crate) const WS_CACHE_PTR: u32 = DATA + 0x2800;

    pub(crate) const WS_CACHE: u32 = DATA + 0x2900;

    pub(crate) const WS_SLOTS: u32 = DATA + 0x2a00;

    pub(crate) const WS_SLOT: u32 = DATA + 0x2b00;

    pub(crate) const WS_ENTRIES: u32 = DATA + 0x2c00;

    pub(crate) const WS_OBJ: u32 = DATA + 0x4000;

    pub(crate) const WS_OBJ2: u32 = DATA + 0x4100;

    pub(crate) const WS_NODE: u32 = DATA + 0x4400;

    pub(crate) const WS_NODE_VT: u32 = DATA + 0x4600;

    pub(crate) const WS_MGR: u32 = DATA + 0x5000;

    pub(crate) const WS_ARGS: u32 = DATA + 0x1000;

    pub(crate) const WS_RESULT: u32 = DATA + 0x1800;

    /// The world, its object cache and two objects ids can name, written
    /// the way both functions look them up: `-1` is the current object
    /// (`world+0x290`), `slot | index << 8` goes through the cache's slot
    /// table.
    pub(crate) fn oc417_wscript_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(GAME_GP + 0xce8, WS_WORLD);
        c.bus
            .write32(WS_WORLD + 0x290, if seed % 5 == 0 { 0 } else { WS_OBJ2 });
        c.bus.write32(GAME_GP + 0x16c8, WS_CACHE_PTR);
        c.bus.write32(WS_CACHE_PTR, WS_CACHE);
        c.bus.write32(WS_CACHE + 8, WS_SLOTS);
        c.bus.write32(WS_SLOTS, WS_SLOT);
        c.bus.write32(WS_SLOTS + 5 * 4, WS_SLOT);
        c.bus.write32(WS_SLOT + 0x1c, WS_ENTRIES);
        c.bus.write32(WS_ENTRIES, (WS_OBJ >> 2) << 8);
        c.bus.write32(
            WS_ENTRIES + 3 * 4,
            if seed % 4 == 0 { 0 } else { (WS_OBJ >> 2) << 8 },
        );
        c.bus.write32(WS_OBJ + 0x78, r.next_u32());
        c.bus.write32(WS_OBJ2 + 0x78, r.next_u32());
        c.bus.write32(GAME_GP + 0xf38, WS_MGR);
        for i in 0..4u32 {
            c.bus.write32(WS_MGR + 4 * i, WS_MGR + 0x400);
        }
    }

    /// The 12-byte default result record both functions copy out.
    pub(crate) fn oc417_wscript_result(c: &mut Cpu, r: &mut Rng) {
        for i in 0..3u32 {
            c.bus.write32(0x004c_9098 + 4 * i, r.next_u32());
        }
    }

    /// `n` argument entries (`{index, value, ?, type}`): index 0-3 of the
    /// 4-word record, the id word taken from the ids above.
    pub(crate) fn oc417_wscript_args(
        c: &mut Cpu,
        r: &mut Rng,
        seed: u32,
        id_index: u32,
        types: u32,
    ) {
        let n = (seed / 2) % 4;
        for i in 0..n {
            let e = WS_ARGS + 0x10 * i;
            let idx = r.next_u32() % 4;
            c.bus.write32(e, idx);
            let value = if idx == id_index {
                [0xffff_ffffu32, 0x0000, 0x0305, 0x0107, 0x0900][(r.next_u32() % 5) as usize]
            } else if idx == 0 {
                r.next_u32() % 4
            } else {
                r.next_u32() % 3
            };
            c.bus.write32(e + 4, value);
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
        for i in 0..4u32 {
            c.bus.write32(types + 4 * i, r.next_u32() % 3);
        }
    }

    /// Both initialisation states of the static record, 0-3 entries (int to
    /// float conversions included), objects found through the current object,
    /// the slot table or not at all, add and remove.
    #[test]
    fn wscript_fn_multi_particle_control_matches() {
        check(
            ee::s10::f_00306cb0__generated,
            wscript_fn_multi_particle_control,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4170_0007, 0);
                c.pc = 0x0030_6cb0;
                oc417_wscript_world(c, &mut r, seed);
                oc417_wscript_result(c, &mut r);
                oc417_wscript_args(c, &mut r, seed, 1, 0x0044_6970);
                let initialised = seed % 3 != 0;
                c.bus.write32(GAME_GP + 0x27c8, initialised as u32);
                if initialised {
                    for i in 0..4u32 {
                        let w = if i == 1 {
                            0xffff_ffff
                        } else {
                            r.next_u32() % 3
                        };
                        c.bus.write32(0x004f_c050 + 4 * i, w);
                    }
                }
                c.g[A0][0] = WS_RESULT as u64;
                c.g[A1][0] = ((seed / 2) % 4) as u64;
                c.g[A2][0] = WS_ARGS as u64;
            },
        );
    }

    /// As above for the magnet modifier: the object's move node, its
    /// "accepts magnets" virtual (`0x27c070` reads the node, so the fixture
    /// chooses the answer) and the faked magnet call.
    #[test]
    fn wscript_fn_add_magnet_modifier_matches() {
        check(
            ee::s10::f_00305478__generated,
            wscript_fn_add_magnet_modifier,
            300,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0008, 4);
                c.pc = 0x0030_5478;
                oc417_wscript_world(c, &mut r, seed);
                oc417_wscript_result(c, &mut r);
                oc417_wscript_args(c, &mut r, seed, 0, 0x0044_66a0);
                for obj in [WS_OBJ, WS_OBJ2] {
                    c.bus
                        .write32(obj + 0xc, if seed % 7 == 0 { 0 } else { WS_NODE });
                }
                c.bus.write32(WS_NODE + 0xc, WS_NODE_VT);
                c.bus.write32(WS_NODE + 0x14, (seed / 3) % 2);
                c.bus.write16(WS_NODE_VT + 0x80, 0);
                c.bus.write32(WS_NODE_VT + 0x84, 0x0027_c070);
                c.bus
                    .write16(WS_NODE_VT + 0xc0, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(WS_NODE_VT + 0xc4, fakes::FAKE);
                c.bus.write32(GAME_GP + 0x2750, (seed % 3 != 0) as u32);
                if seed % 3 != 0 {
                    for i in 0..4u32 {
                        let w = if i == 0 {
                            0xffff_ffff
                        } else {
                            r.next_u32() % 5
                        };
                        c.bus.write32(0x004f_bd58 + 4 * i, w);
                    }
                }
                c.g[A0][0] = WS_RESULT as u64;
                c.g[A1][0] = ((seed / 2) % 4) as u64;
                c.g[A2][0] = WS_ARGS as u64;
            },
        );
    }

    use crate::difftest::fakes::FAKE;

    pub(crate) use crate::game::ai::ai::tests::OC420_CTRL;
    pub(crate) use crate::game::ai::ai::tests::OC420_MAGIC;
    pub(crate) use crate::game::ai::ai::tests::OC420_RET;

    // ------------------------------------------------------ WScriptFn_RandRange

    /// With and without the first-use static init, 0..3 arguments of
    /// matching and differing kinds (the int->float conversion included);
    /// the random call and the register accessor are fakes, `MEM_copy` is real.
    #[test]
    fn wscript_fn_rand_range_matches() {
        oc420_hook_rand_range();
        oc420_hook_luno_as_float();
        covered(
            ee::s10::f_00303598__generated,
            wscript_fn_rand_range,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_3598, 3);
                c.pc = 0x0030_3598;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                c.bus.write32(OC420_RET + 4 * 7, DATA + 0x5000);
                c.bus.write32(GAME_GP + 0x26c8, (seed % 2) as u32);
                // The static parameter block (already initialised half the time).
                c.bus.write32(0x004f_bc60, r.next_u32());
                c.bus.write32(0x004f_bc64, 0x3f80_0000);
                c.bus.write8(0x004f_bc68, r.next_u32() as u8);
                let kinds = [0u32, 1, 2];
                for k in 0..3 {
                    c.bus
                        .write32(GAME_GP + 0xc30 + 4 * k, kinds[(r.next_u32() % 3) as usize]);
                }
                let args = DATA + 0x2000;
                let n = (seed / 2) % 4;
                for i in 0..n {
                    let e = args + 0x10 * i;
                    let k = r.next_u32() % 3;
                    c.bus.write32(e, k);
                    c.bus.write32(e + 4, r.next_u32() % 1000);
                    c.bus.write32(e + 0xc, (r.next_u32() % 3) as u32);
                }
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = if seed % 9 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    n as u64
                };
                c.g[A2][0] = sx(args);
            },
        );
    }

    // ----------------------------------------- the WScriptFn_* decoder fixture

    /// The object ids a decoder resolves: `slot | index << 8` for the slots
    /// and indices of `oc420_world_objects`, and -1 (the current object).
    pub(crate) const OC420_IDS: [u32; 8] =
        [0xffff_ffff, 0x000, 0x100, 0x001, 0x101, 0x302, 0x005, 0x200];

    /// World cache, the object records and their move nodes. The cache is
    /// reached through `gp + 0x16c8`; slots 0..3 hold records (slot 2 empty),
    /// whose tables hold `value << 8` with `value << 2` an object address
    /// (zero for some entries); the current object is `OBJ0`. Each object's
    /// word at +0xc is its move node; node vtable slots +0x50/+0x54 and
    /// +0x80/+0x84 are fakes or the zero leaf.
    pub(crate) fn oc420_world_objects(c: &mut Cpu, r: &mut Rng) {
        let (cache_ptr, cache, slots) = (DATA + 0x1100, DATA + 0x1200, DATA + 0x1300);
        c.bus.write32(GAME_GP + 0x16c8, cache_ptr);
        c.bus.write32(cache_ptr, cache);
        c.bus.write32(cache + 8, slots);
        let vt = DATA + 0x2800;
        for s in 0..4u32 {
            let rec = DATA + 0x1400 + 0x40 * s;
            c.bus.write32(slots + 4 * s, if s == 2 { 0 } else { rec });
            let table = DATA + 0x1500 + 0x20 * s;
            c.bus.write32(rec + 0x1c, table);
            for k in 0..4u32 {
                let obj = DATA + 0x2000 + 0x100 * ((s + k) % 4);
                let entry = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    (obj >> 2) << 8
                };
                c.bus.write32(table + 4 * k, entry);
            }
        }
        c.bus.write32(GAME_GP + 0xce8, DATA + 0x1600);
        c.bus.write32(DATA + 0x1600 + 0x290, DATA + 0x2000);
        for j in 0..4u32 {
            let obj = DATA + 0x2000 + 0x100 * j;
            let node = DATA + 0x2400 + 0x100 * j;
            c.bus
                .write32(obj + 0xc, if r.next_u32() % 5 == 0 { 0 } else { node });
            c.bus.write32(node + 0xc, vt);
            c.bus.write32(node + 0x18, r.next_u32());
        }
        c.bus
            .write16(vt + 0x50, [0u16, 8][(r.next_u32() % 2) as usize]);
        c.bus.write32(
            vt + 0x54,
            [FAKE + 0x10, 0x0014_06c0][(r.next_u32() % 2) as usize],
        );
        c.bus
            .write16(vt + 0x80, [0u16, 4][(r.next_u32() % 2) as usize]);
        c.bus.write32(
            vt + 0x84,
            [FAKE + 0x20, 0x0014_06c0][(r.next_u32() % 2) as usize],
        );
    }

    /// `n` argument entries `{index, value, _, kind}` at `args`; the first
    /// one often sets word 0 (the object id) to one of `OC420_IDS`.
    pub(crate) fn oc420_args(c: &mut Cpu, r: &mut Rng, args: u32, n: u32, words: u32, table: u32) {
        for k in 0..words {
            c.bus
                .write32(table + 4 * k, [0u32, 1, 2][(r.next_u32() % 3) as usize]);
        }
        for i in 0..n {
            let e = args + 0x10 * i;
            let idx = if i == 0 && r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32() % words
            };
            c.bus.write32(e, idx);
            let val = if idx == 0 {
                OC420_IDS[(r.next_u32() % 8) as usize]
            } else {
                r.next_u32() % 1000
            };
            c.bus.write32(e + 4, val);
            let tv = c.bus.read32(table + 4 * idx);
            c.bus.write32(e + 0xc, tv);
            if r.next_u32() % 3 == 0 {
                c.bus.write32(e + 0xc, r.next_u32() % 3);
            }
        }
    }

    // ------------------------------------------------ WScriptFn_StopObjectLoop

    #[test]
    fn wscript_fn_stop_object_loop_matches() {
        oc420_hook_audio_instance();
        oc420_hook_audio_stop_loop();
        covered(
            ee::s10::f_00300260__generated,
            wscript_fn_stop_object_loop,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_0260, 3);
                c.pc = 0x0030_0260;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                c.bus.write32(OC420_RET + 4 * 8, DATA + 0x5000);
                c.bus.write32(GAME_GP + 0x257c, (seed % 2) as u32);
                oc420_world_objects(c, &mut r);
                // The static parameter block {-1, -1, -1, flag}.
                let d = 0x004f_bb28;
                c.bus.write32(d, OC420_IDS[(r.next_u32() % 8) as usize]);
                c.bus.write32(d + 4, r.next_u32());
                c.bus.write32(d + 8, r.next_u32());
                c.bus.write8(d + 0xc, r.next_u32() as u8);
                let args = DATA + 0x3000;
                let n = (seed / 2) % 4;
                oc420_args(c, &mut r, args, n, 4, 0x0044_6560);
                c.g[A0][0] = sx(DATA + 0x500);
                c.g[A1][0] = if seed % 11 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    n as u64
                };
                c.g[A2][0] = sx(args);
                for k in 0..3 {
                    c.bus.write32(DATA + 0x500 + 4 * k, r.next_u32());
                }
                c.bus.write32(0x004c_9098, r.next_u32());
                c.bus.write32(0x004c_909c, r.next_u32());
                c.bus.write32(0x004c_90a0, r.next_u32());
            },
        );
    }

    // -------------------------------------------- WScriptFn_AddParentModifierRev

    #[test]
    fn wscript_fn_add_parent_modifier_rev_matches() {
        oc420_hook_add_parent();
        covered(
            ee::s10::f_002fdc60__generated,
            wscript_fn_add_parent_modifier_rev,
            500,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_dc60, 5);
                c.pc = 0x002f_dc60;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                c.bus.write32(OC420_RET + 4 * 10, DATA + 0x5000);
                c.bus.write32(GAME_GP + 0x2520, (seed % 2) as u32);
                oc420_world_objects(c, &mut r);
                // The static parameter block {-1, -1, 0, 0.0 x3, flag}.
                let d = 0x004f_b758;
                c.bus.write32(d, OC420_IDS[(r.next_u32() % 8) as usize]);
                c.bus.write32(d + 4, OC420_IDS[(r.next_u32() % 8) as usize]);
                c.bus.write32(d + 8, r.next_u32());
                for k in 3..6 {
                    c.bus.write32(d + 4 * k, r.f32(10.0));
                }
                c.bus.write8(d + 0x18, r.next_u32() as u8);
                let args = DATA + 0x3000;
                let n = (seed / 2) % 4;
                oc420_args(c, &mut r, args, n, 6, 0x0044_61c0);
                c.g[A0][0] = sx(DATA + 0x500);
                c.g[A1][0] = if seed % 13 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    n as u64
                };
                c.g[A2][0] = sx(args);
                for k in 0..3 {
                    c.bus.write32(DATA + 0x500 + 4 * k, r.next_u32());
                }
                c.bus.write32(0x004c_9098, r.next_u32());
                c.bus.write32(0x004c_909c, r.next_u32());
                c.bus.write32(0x004c_90a0, r.next_u32());
            },
        );
    }

    pub(crate) use crate::game::main::ssxapp::tests::oc429_gp;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_machine;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_w;

    pub(crate) use crate::game::main::ssxapp::tests::OC429_FX;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc428_gate;

    /// `WScriptFn_ObjectSlot36` against the generated body: the first-call
    /// initialisation (flag at `gp + 0x2550`) or a table left in RAM, 0-4
    /// argument entries (int/float slot types), and the target object found
    /// by handle through the two lookup tables, through the global object,
    /// or missing (`EA_MemCopy` real; the object's method is a fake call).
    #[test]
    fn w_script_fn_object_slot36_matches() {
        oc428_install_fakes();
        fakes::covered(
            ee::s10::f_002ff9a8__generated,
            w_script_fn_object_slot36,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ff9a8, 1);
                oc428_gate(c);
                let (out, entries) = (DATA + 0x7000, DATA + 0x7100);
                let (ht, h, arr, ent, back) = (
                    DATA + 0x8000,
                    DATA + 0x8100,
                    DATA + 0x8200,
                    DATA + 0x8300,
                    DATA + 0x8400,
                );
                let (global, obj, p, vt) =
                    (DATA + 0x8500, DATA + 0x6000, DATA + 0x6100, DATA + 0x6200);
                let table = 0x004f_baf8;
                let flag = seed % 2;
                c.bus.write32(GAME_GP + 0x2550, flag);
                // The handle in the table: none, found, or a dead slot.
                let handle = [
                    0xffff_ffffu32,
                    0x0000_0102,
                    0x0000_0103,
                    0x0000_0001,
                    0x0000_0201,
                ][((seed / 2) % 5) as usize];
                fill(c, &mut r, table, 8);
                c.bus.write32(table, handle);
                // Slot types of the argument indices 0..3.
                for i in 0..4 {
                    c.bus.write32(0x0044_6540 + 4 * i, 1 + r.next_u32() % 3);
                }
                // Lookup tables: handle -> slot 2 -> generation 1 -> object.
                c.bus.write32(GAME_GP + 0x16c8, ht);
                c.bus.write32(ht, h);
                c.bus.write32(h + 8, arr);
                c.bus.write32(arr + 8, ent);
                c.bus.write32(arr + 4, ent);
                c.bus.write32(arr + 0xc, 0);
                c.bus.write32(ent + 0x1c, back);
                c.bus.write32(back + 4, ((obj >> 2) << 8) | 5);
                c.bus.write32(back + 8, 0x0000_0005);
                c.bus.write32(back + 12, 0);
                c.bus.write32(GAME_GP + 0xce8, global);
                c.bus.write32(global + 0x290, obj);
                c.bus.write32(obj + 0xc, if seed % 7 == 0 { 0 } else { p });
                c.bus.write32(p + 0xc, vt);
                c.bus.write16(vt + 0x120, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(vt + 0x124, FAKE + 0x50);
                // Entries: (index, value, _, type).
                let n = (seed / 3) % 5;
                for k in 0..4u32 {
                    let e = entries + 0x10 * k;
                    let idx = if seed % 5 == 0 && k == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 3
                    };
                    c.bus.write32(e, idx);
                    c.bus
                        .write32(e + 4, if idx == 0 { handle } else { r.next_u32() % 100 });
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, 1 + r.next_u32() % 3);
                }
                c.g[A0][0] = sx(out);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(entries);
            },
        );
    }

    /// `WScriptFn_AddCollectItems` against the generated body: first-call
    /// initialisation or not, 0-4 entries of int/float slot types (indices
    /// 0-20), with `args[0]` set (the real `EA_MemCopy`d table's first word,
    /// or an entry's value) or still -1; `WScriptMan_AddCollectItems` is a
    /// fake.
    #[test]
    fn w_script_fn_add_collect_items_matches() {
        oc428_install_fakes();
        fakes::covered(
            ee::s10::f_00300f50__generated,
            w_script_fn_add_collect_items,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x300f50, 1);
                oc428_gate(c);
                let (out, entries) = (DATA + 0x7000, DATA + 0x7100);
                c.bus.write32(GAME_GP + 0x25c0, seed % 2);
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x8000);
                let table = 0x004f_bb78;
                fill(c, &mut r, table, 0x16);
                if seed % 2 == 0 {
                    c.bus.write32(table, 0xffff_ffff);
                }
                for i in 0..21 {
                    c.bus.write32(0x0044_6570 + 4 * i, 1 + r.next_u32() % 3);
                }
                let n = (seed / 2) % 5;
                for k in 0..4u32 {
                    let e = entries + 0x10 * k;
                    let idx = if seed % 4 == 1 && k == 0 {
                        0
                    } else {
                        r.next_u32() % 21
                    };
                    c.bus.write32(e, idx);
                    c.bus.write32(e + 4, r.next_u32() % 1000);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, 1 + r.next_u32() % 3);
                }
                c.g[A0][0] = sx(out);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(entries);
            },
        );
    }

    /// `WScriptFn_RiderCommand`: first call (static defaults written) and
    /// later calls, 0..4 entries (int stores and the int-to-float conversion
    /// for slots of type 2), the rider id defaulted from the global or given.
    /// `EA_MemCopy` runs for real; the rider call is a fake.
    #[test]
    fn wscript_fn_rider_command_matches() {
        covered(
            ee::s10::f_002ff850__generated,
            wscript_fn_rider_command,
            160,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x2ff850, 1, 0x002f_f850);
                let world = OC429_FX + 0x300;
                let x = OC429_FX + 0x340;
                let riders = OC429_FX + 0x700;
                oc429_w(c, oc429_gp(-0x848), world);
                oc429_w(c, world + 0x84, x);
                oc429_w(c, x + 0xc, riders);
                // `riders[idx] + 0x28` is the rider pointer: idx 0..3 at +0x28...
                for i in 0..4u32 {
                    oc429_w(c, riders + 0x28 + 4 * i, OC429_FX + 0x1000 + 0x10 * i);
                }
                // The default record, its flag, the global rider id.
                oc429_w(c, oc429_gp(0x254c), (seed % 3 != 0) as u32);
                fill(c, &mut r, 0x004f_bae8, 4);
                oc429_w(
                    c,
                    0x004f_bae8,
                    if seed % 4 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 4
                    },
                );
                oc429_w(c, oc429_gp(0xce8), OC429_FX + 0x1200);
                oc429_w(c, OC429_FX + 0x1200, r.next_u32() % 4);
                // Slot types.
                // (Slot 0, the rider index, never holds a float: it indexes a table.)
                for k in 0..4u32 {
                    let ty = if k == 0 {
                        [0u32, 1, 3][(r.next_u32() % 3) as usize]
                    } else {
                        r.next_u32() % 4
                    };
                    oc429_w(c, 0x0044_6530 + 4 * k, ty);
                }
                fill(c, &mut r, 0x004c_9098, 3);
                let n = (seed / 3) % 5;
                let ents = OC429_FX + 0x1400;
                for i in 0..n.max(1) {
                    let e = ents + 0x10 * i;
                    oc429_w(c, e, r.next_u32() % 4);
                    oc429_w(c, e + 4, r.next_u32() % 4);
                    oc429_w(c, e + 8, r.next_u32());
                    oc429_w(c, e + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = (OC429_FX + 0x1800) as u64;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = ents as u64;
            },
        );
    }
}
