//! Staging module oc567 (wave wave232): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc567`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0034_1368, collision_global_ctors, "Collision_GlobalCtors"),
    (
        0x0034_10c0,
        cell_collide_objects_subtree,
        "Cell_CollideObjectsSubtree",
    ),
    (0x0034_11b8, cell_find_rail_subtree, "Cell_FindRailSubtree"),
    (0x0034_2e78, debounce_v36, "Debounce_v36"),
    (0x0034_2a88, boost_modifier_v07, "BoostModifier_v07"),
];

/// The static-init entry the global constructor thunk calls.
const COLLISION_STATIC_INIT: Callee = Callee {
    addr: 0x0034_00d8,
    f: ee::s11::Collision_StaticInit,
};

/// `nash_Cell` (spatial tree node) layout: eight child pointers
/// (`children[a][b][c]` at index `4a + 2b + c`), the head of the static
/// object list (`nodesinside[0]`), and the head of the typed-node list
/// (`nodesinside[2]`).
const CELL_CHILDREN: u32 = 0x00;
const CELL_OBJECTS: u32 = 0x20;
const CELL_TYPED: u32 = 0x28;

/// Static object record: `next` at +0 (flags at +8, read by the callback).
const OBJECT_NEXT: u32 = 0x00;

/// Typed space node record: `next` at +0, `type` at +8, the rail / flex
/// node / spline segment fields at +0xc / +0x68 (parent).
const TYPED_NEXT: u32 = 0x00;
const TYPED_TYPE: u32 = 0x08;
const TYPED_RAIL: u32 = 0x0c;
const TYPED_PARENT: u32 = 0x68;

/// The spline query record that the tree walk reads (box, position, mask,
/// hit count, distance, info).
const QUERY_BOX_MIN: u32 = 0x00;
const QUERY_BOX_MAX: u32 = 0x04;
const QUERY_MASK: u32 = 0x08;
const QUERY_POS: u32 = 0x0c;
const QUERY_HIT: u32 = 0x10;
const QUERY_DIST: u32 = 0x14;

/// Call sites of `Cell_CollideObjectsSubtree` (the 0x3410c0 child walk) and
/// the static-object callback, in generated order.
const COLLIDE_OBJECT: Callee = Callee {
    addr: 0x0033_6d40,
    f: ee::s11::Cell_CollideObjectEx,
};
const COLLIDE_OBJECTS_SUBTREE: Callee = Callee {
    addr: 0x0034_10c0,
    f: ee::s11::Cell_CollideObjectsSubtree,
};
const COLLIDE_CHILD_RETURNS: [u32; 8] = [
    0x0034_1110,
    0x0034_1124,
    0x0034_1138,
    0x0034_114c,
    0x0034_1160,
    0x0034_1174,
    0x0034_1188,
    0x0034_119c,
];
const COLLIDE_OBJECT_RETURN: u32 = 0x0034_10f0;
const COLLIDE_OBJECT_HEAD: u32 = 0x0034_10e8;

/// Rail, flex-rail and spline-segment hit tests of the typed list, and the
/// subtree walk itself.
const RAIL_MODIFIER_SPLINE: Callee = Callee {
    addr: 0x0035_c698,
    f: ee::s11::RailModifier_FindClosestPoint,
};
const FLEX_RAIL_SPLINE: Callee = Callee {
    addr: 0x0034_8290,
    f: ee::s11::FlexRail_FindClosestPoint,
};
const SPLINE_SEGMENT_SPLINE: Callee = Callee {
    addr: 0x0033_5128,
    f: ee::s11::Rail_FindNearestPoint,
};
const FIND_RAIL_SUBTREE: Callee = Callee {
    addr: 0x0034_11b8,
    f: ee::s11::Cell_FindRailSubtree,
};
const FIND_RAIL_CHILD_RETURNS: [u32; 8] = [
    0x0034_12b8,
    0x0034_12cc,
    0x0034_12e0,
    0x0034_12f4,
    0x0034_1308,
    0x0034_131c,
    0x0034_1330,
    0x0034_1344,
];
const RAIL_RETURN: u32 = 0x0034_1224;
const FLEX_RAIL_RETURN: u32 = 0x0034_1258;
const SEGMENT_RETURN: u32 = 0x0034_1298;
const FIND_RAIL_LOOP_HEAD: u32 = 0x0034_11f8;

/// `void Collision_GlobalCtors()` (0x341368): the static constructor thunk.
/// Calls `Collision_StaticInit(1, 0xffff)` and returns its result.
pub fn collision_global_ctors(c: &mut Cpu) {
    guest(c, collision_global_ctors_body)
}

fn collision_global_ctors_body(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    c.g[A0][0] = 1;
    frame.save_ra(c, 0);
    c.g[A1][0] = 0xffff;
    COLLISION_STATIC_INIT.call(c, 0x0034_137c)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void Cell_CollideObjectsSubtree(nash_Cell* cell, void* ctx)` (0x3410c0,
/// `nash_Cell_descend5`): runs the static-object callback over the cell's
/// object list, then walks the eight children in order (each non-null child
/// recursively). The object loop checks events at its back edge (head
/// 0x3410e8).
pub fn cell_collide_objects_subtree(c: &mut Cpu) {
    guest(c, cell_collide_objects_subtree_body)
}

fn cell_collide_objects_subtree_body(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    frame.save_ra(c, 0);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    c.g[S0][0] = Addr((c.g[S2][0] as u32).wrapping_add(CELL_OBJECTS)).lw(c);
    let no_objects = c.g[S0][0] == 0;
    c.g[S1][0] = c.g[A1][0];
    if !no_objects {
        c.cycles += 1;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            COLLIDE_OBJECT.call(c, COLLIDE_OBJECT_RETURN)?;
            c.g[S0][0] = Addr(c.g[S0][0] as u32).off(OBJECT_NEXT).lw(c);
            let more = c.g[S0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, COLLIDE_OBJECT_HEAD);
            }
        }
    }
    // The eight children. Each block tests the child loaded by the block
    // before it; the first block charges 3 cycles, the others 2.
    c.cycles += 3;
    c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
    for i in 0..8usize {
        if i > 0 {
            c.cycles += 2;
        }
        // The last block restores s0 before and after its call.
        if i == 7 {
            frame.restore(c, S0, 0x30);
        }
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            COLLIDE_OBJECTS_SUBTREE.call(c, COLLIDE_CHILD_RETURNS[i])?;
            if i == 7 {
                frame.restore(c, S0, 0x30);
            }
        }
        if i < 7 {
            c.g[A0][0] =
                Addr((c.g[S2][0] as u32).wrapping_add(CELL_CHILDREN + 4 * (i as u32 + 1))).lw(c);
        }
    }
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void Cell_FindRailSubtree(nash_Cell* cell, SplineQuery* q)` (0x3411b8,
/// `nash_Cell_descend6`): walks the cell's typed list and tests each node
/// against the spline query. Rails (type 2), flex rails (3) and spline
/// segments (1, only when the parent's flags pass the query mask) call their
/// intersection routine. Then the eight children, recursively.
pub fn cell_find_rail_subtree(c: &mut Cpu) {
    guest(c, cell_find_rail_subtree_body)
}

fn cell_find_rail_subtree_body(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S0, 0x60);
    frame.save(c, S2, 0x40);
    frame.save(c, S3, 0x30);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S4, 0x20);
    frame.save(c, S5, 0x10);
    frame.save_ra(c, 0);
    frame.save(c, S1, 0x50);
    c.g[S1][0] = Addr((c.g[S2][0] as u32).wrapping_add(CELL_TYPED)).lw(c);
    let no_typed = c.g[S1][0] == 0;
    c.g[S0][0] = c.g[A1][0];
    if !no_typed {
        c.cycles += 4;
        c.g[S5][0] = 2;
        c.g[S4][0] = 3;
        c.g[S3][0] = 1;
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(TYPED_TYPE)).lw(c);
        loop {
            c.cycles += 2;
            if c.g[V0][0] == c.g[S5][0] {
                // Rail: RailModifier_FindClosestPoint(rail, box, box, pos, mask, hit, dist, info).
                c.cycles += 11;
                let q = c.g[S0][0] as u32;
                c.g[A1][0] = Addr(q).off(QUERY_DIST).lw(c);
                c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(TYPED_RAIL)).lw(c);
                c.g[A3][0] = Addr(q).off(QUERY_BOX_MIN).lw(c);
                c.g[A2][0] = c.g[A1][0];
                c.g[T0][0] = Addr(q).off(QUERY_MASK).lw(c);
                c.g[T1][0] = Addr(q).off(QUERY_POS).lw(c);
                c.g[T2][0] = Addr(q).off(QUERY_HIT).lw(c);
                c.g[T3][0] = Addr(q).off(QUERY_BOX_MAX).lw(c);
                RAIL_MODIFIER_SPLINE.call(c, RAIL_RETURN)?;
                c.g[S1][0] = Addr(c.g[S1][0] as u32).off(TYPED_NEXT).lw(c);
            } else {
                c.cycles += 2;
                if c.g[V0][0] == c.g[S4][0] {
                    // Flex rail: cFlexRailNode_splineIntersect.
                    c.cycles += 11;
                    let q = c.g[S0][0] as u32;
                    c.g[A1][0] = Addr(q).off(QUERY_DIST).lw(c);
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(TYPED_RAIL)).lw(c);
                    c.g[A3][0] = Addr(q).off(QUERY_BOX_MIN).lw(c);
                    c.g[A2][0] = c.g[A1][0];
                    c.g[T0][0] = Addr(q).off(QUERY_MASK).lw(c);
                    c.g[T1][0] = Addr(q).off(QUERY_POS).lw(c);
                    c.g[T2][0] = Addr(q).off(QUERY_HIT).lw(c);
                    c.g[T3][0] = Addr(q).off(QUERY_BOX_MAX).lw(c);
                    FLEX_RAIL_SPLINE.call(c, FLEX_RAIL_RETURN)?;
                    c.g[S1][0] = Addr(c.g[S1][0] as u32).off(TYPED_NEXT).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[S3][0] {
                        // Spline segment, if its parent's flags pass the mask.
                        c.cycles += 6;
                        let q = c.g[S0][0] as u32;
                        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(TYPED_PARENT)).lw(c);
                        c.g[A0][0] = Addr(q).off(QUERY_MASK).lw(c);
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1c)).lw(c);
                        c.g[V1][0] &= c.g[A0][0];
                        let filtered_out = c.g[V1][0] == 0;
                        c.g[A0][0] = c.g[S1][0];
                        if !filtered_out {
                            c.cycles += 6;
                            c.g[A1][0] = Addr(q).off(QUERY_DIST).lw(c);
                            c.g[A2][0] = Addr(q).off(QUERY_BOX_MIN).lw(c);
                            c.g[A3][0] = Addr(q).off(QUERY_POS).lw(c);
                            c.g[T0][0] = Addr(q).off(QUERY_HIT).lw(c);
                            c.g[T1][0] = Addr(q).off(QUERY_BOX_MAX).lw(c);
                            SPLINE_SEGMENT_SPLINE.call(c, SEGMENT_RETURN)?;
                        }
                        c.cycles += 1;
                        c.g[S1][0] = Addr(c.g[S1][0] as u32).off(TYPED_NEXT).lw(c);
                    } else {
                        c.g[S1][0] = Addr(c.g[S1][0] as u32).off(TYPED_NEXT).lw(c);
                    }
                }
            }
            // Back edge of the typed list: the next node's type is loaded
            // here, and a due event is delivered with the list pointer set.
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(TYPED_TYPE)).lw(c);
            if event_due(c) {
                deliver_event_here(c, FIND_RAIL_LOOP_HEAD);
            }
        }
    }
    // The eight children, as in Cell_CollideObjectsSubtree.
    c.cycles += 3;
    c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
    for i in 0..8usize {
        if i > 0 {
            c.cycles += 2;
        }
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            FIND_RAIL_SUBTREE.call(c, FIND_RAIL_CHILD_RETURNS[i])?;
        }
        if i < 7 {
            c.g[A0][0] =
                Addr((c.g[S2][0] as u32).wrapping_add(CELL_CHILDREN + 4 * (i as u32 + 1))).lw(c);
        } else {
            frame.restore(c, S0, 0x60);
        }
    }
    c.cycles += 8;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void Debounce_v36(void* obj)` (0x342e78): forwards to `ModObj_v36`
/// (the move node's signal) with the same argument.
pub fn debounce_v36(c: &mut Cpu) {
    guest(c, debounce_v36_body)
}

fn debounce_v36_body(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    MOD_OBJ_V36.call(c, 0x0034_2e88)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

const MOD_OBJ_V36: Callee = Callee {
    addr: 0x0035_6b08,
    f: ee::s11::ModObj_v36,
};

/// `void BoostModifier_v07(void* self, void* obj)` (0x342a88): calls the
/// object's vtable slot at 0x14 (its `this`-relative argument is `self +
/// delta`, the delta a halfword at 0x10), then, when the vector scaled by
/// the frame time (`cObjectInterface_getUpdateTime`) is below the threshold
/// at self+0x20, calls the second slot (0x24) on the same object.
pub fn boost_modifier_v07(c: &mut Cpu) {
    guest(c, boost_modifier_v07_body)
}

fn boost_modifier_v07_body(c: &mut Cpu) -> Flow {
    c.cycles += 27;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S0, 0x60);
    frame.save(c, S1, 0x50);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0x40);
    c.g[S1][0] = c.g[A1][0];
    c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
    let slot = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, slot, 0x0034_2ab4)?;
    c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x20));
    c.lqc2(3, c.g[V0][0] as u32);
    c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x10));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(3, 4);
    c.mtc1(3, 0);
    c.mtc1(0, 1);
    c.sub_s(2, 2, 0);
    c.c_lt_s(1, 2);
    let below = c.fcc();
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
    if below {
        c.cycles += 29;
        c.mfc1(3, 2);
        c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x10));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        let spill = frame.at(c, 0x30);
        c.sqc2(5, spill);
        c.lq(2, spill);
        let spill20 = frame.at(c, 0x20);
        c.sq(2, spill20);
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x14));
        c.mfc1(3, 0);
        c.lqc2(4, spill20);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, spill);
        c.lq(2, spill);
        let spill10 = frame.at(c, 0x10);
        c.sq(2, spill10);
        call_ptr(c, 0x002d_1c70, 0x0034_2b34)?;
        c.mfc1(2, 0);
        c.lqc2(4, spill10);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, spill);
        c.g[A1][0] = c.g[SP][0];
        c.lq(2, spill);
        c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
        let sp_slot = frame.at(c, 0x0);
        c.sq(2, sp_slot);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
        let slot2 = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, slot2, 0x0034_2b68)?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore_ra(c, 0x40);
    frame.close(c);
    Ok(())
}

/// `void Conveyor_ctor_2(void* self, ..., void* stream)` (0x3430d0): runs the
/// base constructor (`WorldObj_ctor_2`), sets the conveyor's vptr-relative
/// fields (0x490000 + 0x570 at +0xc) and calls the stream's vtable slot 0x14
/// with `stream + delta`, the delta a halfword at 0x10. Resumes (a return
/// from a callee that unwound) go to the generated `__entries`.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn conveyor_ctor_2(c: &mut Cpu) {
    if c.pc != 0x0034_30d0 {
        return ee::s11::f_003430d0__entries(c);
    }
    guest(c, conveyor_ctor_2_body)
}

fn conveyor_ctor_2_body(c: &mut Cpu) -> Flow {
    c.cycles += 23;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    WORLD_OBJ_CTOR_2.call(c, 0x0034_30ec)?;
    c.g[V1][0] = 0x49_0000;
    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x570));
    c.g[A2][0] = 0x2c;
    let vptr_word = c.g[V1][0];
    Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sw(c, vptr_word);
    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x14).lw(c);
    let slot = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, slot, 0x0034_3114)?;
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S1, 0x10);
    frame.restore(c, S0, 0x20);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

const WORLD_OBJ_CTOR_2: Callee = Callee {
    addr: 0x0034_fb90,
    f: ee::s11::WorldObj_ctor_2,
};

/// `void AnimTeeter_v43(void* self, void* anim, void* out)` (0x342538): when
/// the anim's signed index at +0x5c is non-negative and the table entry it
/// selects is set, calls the controller's vtable slot 0xec (delta at 0xe8),
/// then, when the VU0 test passes, blends the teeter vector at `self` with
/// the anim vector and writes it back. Hand-written with the original's
/// VU0 macro ops; the early exits are the generated `break`s.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn anim_teeter_v43(c: &mut Cpu) {
    guest(c, anim_teeter_v43_body)
}

fn anim_teeter_v43_body(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x80);
    frame.save(c, S0, 0x70);
    frame.save(c, S1, 0x60);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S2, 0x50);
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0x40);
    c.g[A3][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x5c)).lw(c);
    let negative = (c.g[A3][0] as i64) < 0;
    c.g[S2][0] = c.g[A2][0];
    'exit: {
        if negative {
            break 'exit;
        }
        c.cycles += 9;
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x44));
        c.g[A1][0] = sx((c.g[A3][0] as u32) << 4);
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x80)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lw(c);
        let no_entry = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[A3][0];
        if no_entry {
            break 'exit;
        }
        c.cycles += 22;
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x50)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xe8)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xec)).lw(c);
        let slot = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, slot, 0x0034_2598)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(1, 1);
        c.lqc2(3, c.g[S1][0] as u32);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 0);
        c.c_lt_s(1, 0);
        let passed = c.fcc();
        c.g[A0][0] = c.g[V1][0];
        if !passed {
            break 'exit;
        }
        'blend: {
            c.cycles += 13;
            c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x30));
            c.mtc1(0, 1);
            let spill10 = frame.at(c, 0x10);
            c.sq(2, spill10);
            c.lqc2(3, c.g[S1][0] as u32);
            c.lqc2(4, spill10);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            let spill20 = frame.at(c, 0x20);
            c.sqc2(5, spill20);
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
            c.lq(2, spill20);
            c.c_lt_s(0, 1);
            let below = c.fcc();
            let spill0 = frame.at(c, 0x0);
            c.sq(2, spill0);
            if !below {
                break 'blend;
            }
            c.cycles += 10;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 0);
            c.mfc1(3, 0);
            let spill0_q = frame.at(c, 0x0);
            c.lqc2(4, spill0_q);
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, spill20);
            c.lq(2, spill20);
            c.sq(2, spill0_q);
            c.sq(2, spill10);
        }
        'scale: {
            c.cycles += 6;
            c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x5c)).lw(c);
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x2c)).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[V1][0] as u32) & 31)) as u32);
            c.g[V0][0] &= 0x1;
            let clear = c.g[V0][0] == 0;
            c.g[V0][0] = 0x50_0000;
            if clear {
                break 'scale;
            }
            c.cycles += 11;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 0);
            c.mfc1(3, 0);
            let base = frame.at(c, 0x0);
            c.lqc2(4, base);
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            let spill20 = frame.at(c, 0x20);
            c.sqc2(5, spill20);
            c.lq(2, spill20);
            let base2 = frame.at(c, 0x0);
            c.sq(2, base2);
            let spill10 = frame.at(c, 0x10);
            c.sq(2, spill10);
            c.g[V0][0] = 0x50_0000;
        }
        c.cycles += 33;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xeb0));
        c.lqc2(8, c.g[V0][0] as u32);
        c.lqc2(4, c.g[A0][0] as u32);
        c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        let spill20 = frame.at(c, 0x20);
        c.sqc2(12, spill20);
        c.lq(2, spill20);
        let spill10 = frame.at(c, 0x10);
        c.sq(2, spill10);
        let base = frame.at(c, 0x0);
        c.lqc2(4, base);
        c.lqc2(5, c.g[S2][0] as u32);
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
        let spill30 = frame.at(c, 0x30);
        c.sqc2(6, spill30);
        c.lq(2, spill30);
        c.lwc1(0, c.g[S0][0] as u32);
        c.sq(2, spill20);
        c.lqc2(3, spill10);
        c.lqc2(5, spill20);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 1);
        c.add_s(0, 0, 1);
        c.swc1(0, c.g[S0][0] as u32);
    }
    c.cycles += 6;
    frame.restore(c, S0, 0x70);
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore_ra(c, 0x40);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::RA as SENTINEL;

    /// Scratch regions of the fixtures, all inside the test data area and
    /// clear of the fakes' log and the pool.
    const TREE_CELLS: u32 = DATA + 0x2000;
    const TREE_OBJECTS: u32 = DATA + 0x8000;
    const CTX: u32 = DATA + 0x4000;
    const TYPED_CELLS: u32 = DATA + 0x3000;
    const TYPED_NODES: u32 = DATA + 0x9000;
    const TYPED_PARENTS: u32 = DATA + 0x6000;
    const QUERY: u32 = DATA + 0x5000;

    /// An acyclic tree of cells (children only point at later cells) with
    /// random static object lists. Object flags avoid 0x20 and 0x40, so the
    /// object callback returns at once.
    fn collide_tree_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        let n = 1 + seed % 9;
        let mut next_object = 0u32;
        for k in 0..n {
            let cell = TREE_CELLS + 0x40 * k;
            for s in 0..8u32 {
                let j = k + 1 + r.next_u32() % 4;
                let child = if j < n && r.next_u32() % 3 != 0 {
                    TREE_CELLS + 0x40 * j
                } else {
                    0
                };
                c.bus.write32(cell + CELL_CHILDREN + 4 * s, child);
            }
            let mut head = 0;
            for _ in 0..r.next_u32() % 3 {
                let obj = TREE_OBJECTS + 0x20 * next_object;
                next_object += 1;
                c.bus.write32(obj + OBJECT_NEXT, head);
                c.bus.write32(obj + 8, r.next_u32() & !0x60);
                head = obj;
            }
            c.bus.write32(cell + CELL_OBJECTS, head);
        }
        TREE_CELLS
    }

    /// Like [`collide_tree_fixture`], with typed lists instead of object
    /// lists. Node types are 0, 1, 4 or 5: type 1 nodes have a zero-flag
    /// parent, so their spline test is filtered out and nothing is called.
    /// (Types 2 and 3 call the rail intersection routines, which have no
    /// short path on zeroed fixture data; they are not generated here.)
    fn find_rail_tree_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        let n = 1 + seed % 9;
        let mut next_node = 0u32;
        for k in 0..n {
            let cell = TYPED_CELLS + 0x40 * k;
            for s in 0..8u32 {
                let j = k + 1 + r.next_u32() % 4;
                let child = if j < n && r.next_u32() % 3 != 0 {
                    TYPED_CELLS + 0x40 * j
                } else {
                    0
                };
                c.bus.write32(cell + CELL_CHILDREN + 4 * s, child);
            }
            let mut head = 0;
            for _ in 0..r.next_u32() % 3 {
                let node = TYPED_NODES + 0x80 * next_node;
                let parent = TYPED_PARENTS + 0x40 * next_node;
                next_node += 1;
                let kind = [0u32, 1, 4, 5][(r.next_u32() % 4) as usize];
                c.bus.write32(node + TYPED_NEXT, head);
                c.bus.write32(node + TYPED_TYPE, kind);
                c.bus.write32(node + TYPED_PARENT, parent);
                c.bus.write32(parent + 0x1c, 0);
                head = node;
            }
            c.bus.write32(cell + CELL_TYPED, head);
        }
        for i in 0..8u32 {
            c.bus.write32(QUERY + 4 * i, r.next_u32());
        }
        TYPED_CELLS
    }

    #[test]
    fn collision_global_ctors_matches() {
        check(
            ssx3_ee::s11::f_00341368__generated,
            collision_global_ctors,
            40,
            &|c, seed| {
                let mut r = Rng(0x3413_6800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0034_1368;
                c.g[RA][0] = sx(SENTINEL);
            },
        );
    }

    #[test]
    fn cell_collide_objects_subtree_matches() {
        check(
            ssx3_ee::s11::f_003410c0__generated,
            cell_collide_objects_subtree,
            200,
            &|c, seed| {
                let mut r = Rng(0x3410_c000 ^ seed as u64);
                scramble(c, &mut r);
                let root = collide_tree_fixture(c, &mut r, seed);
                c.pc = 0x0034_10c0;
                c.g[A0][0] = sx(root);
                c.g[A1][0] = sx(CTX);
                c.g[RA][0] = sx(SENTINEL);
            },
        );
    }

    #[test]
    fn cell_find_rail_subtree_matches() {
        check(
            ssx3_ee::s11::f_003411b8__generated,
            cell_find_rail_subtree,
            200,
            &|c, seed| {
                let mut r = Rng(0x3411_b800 ^ seed as u64);
                scramble(c, &mut r);
                let root = find_rail_tree_fixture(c, &mut r, seed);
                c.pc = 0x0034_11b8;
                c.g[A0][0] = sx(root);
                c.g[A1][0] = sx(QUERY);
                c.g[RA][0] = sx(SENTINEL);
            },
        );
    }

    #[test]
    fn debounce_v36_matches() {
        check(
            ssx3_ee::s11::f_00342e78__generated,
            debounce_v36,
            100,
            &|c, seed| {
                let mut r = Rng(0x0034_2e78 ^ seed as u64);
                scramble(c, &mut r);
                let obj = DATA + 0x1000 + (r.next_u32() & 0xff0);
                // ModifierBlock empty: the move node's signal returns at once.
                c.bus.write32(obj + 0x1c, 0);
                c.pc = 0x0034_2e78;
                c.g[A0][0] = sx(obj);
                c.g[RA][0] = sx(SENTINEL);
            },
        );
    }

    #[test]
    fn boost_modifier_v07_matches() {
        check(
            ssx3_ee::s11::f_00342a88__generated,
            boost_modifier_v07,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_2a88, 0);
                c.pc = 0x0034_2a88;
                // obj: first word is the vtable; slots 0x14 and 0x24 are
                // fakes with the halfword deltas at 0x10 and 0x20.
                let vt = vtable(c, &mut r, &[0x10, 0x20]);
                let obj = ptr(&mut r);
                c.bus.write32(obj, vt);
                // self: the vector at +0x10 and the float at +0x20.
                let this = DATA + 0x3000 + (r.next_u32() & 0xff0);
                for i in 0..8u32 {
                    c.bus.write32(this + 0x10 + 4 * i, r.f32(4.0));
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(obj);
                c.g[RA][0] = sx(SENTINEL);
            },
        );
    }
}
