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
pub(crate) use crate::game::object::debouncenode::OC327_EA_MEM_ALLOC_NAMED;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

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
}
