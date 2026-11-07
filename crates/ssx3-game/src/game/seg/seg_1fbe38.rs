//! `src/seg/seg_1fbe38.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
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
}
