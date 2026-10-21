//! Phase 2 wave 3 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for collision, input and script helpers, each with a
//! differential test against the generated body.
//!
//! - **World painter cell lookup** (`WorldPainterQuery_LookupCell`): the
//!   quad-tree cell a query key falls in, or null for "outside".
//! - **Input VM operands** (`InputVM_ReadOperand`): the floating-point value
//!   of one bytecode operand: a script variable, a builtin or a controller
//!   channel reading.
//! - **Segment/triangle** (`CollSegment_CollideTriangle`): the segment
//!   against a ray/triangle test in a sphere tree.
//! - **Focus octree** (`FocusMan_CollectOctreeNode`): the focus manager's
//!   recursive walk of the octree, collecting the nodes that overlap the
//!   query cell's volume.
//! - **Bone matrices** (`AnimObject_BuildMatrices`): one scaled matrix per
//!   skeleton bone, from its parent and its animation matrix.
//! - **Overlap list** (`OverlapList_CollideObjects`): the broad-phase box
//!   filter and per-object collision of an overlap query.
//! - `OverlapList_FindNearestRail` is ported at the end of the module but not
//!   listed in [`OVERRIDES`]: its test cannot reach the rail finders (they are
//!   dispatchers that need real rail data), so the call paths stay on the
//!   generated code until they can be tested on a real level.
//!
//! The VU0 code issues the original's COP2 macro operations in the same
//! order, so VF/ACC/Q and the VU0 flags end as the original leaves them.

use crate::call::{guest, Callee, Flow, Frame};
use crate::events;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002c_0a10,
        world_painter_query_lookup_cell,
        "WorldPainterQuery_LookupCell",
    ),
    (0x0032_5260, input_vm_read_operand, "InputVM_ReadOperand"),
    (
        0x0032_e4d0,
        coll_segment_collide_triangle,
        "CollSegment_CollideTriangle",
    ),
    (
        0x0034_dd18,
        anim_object_build_matrices,
        "AnimObject_BuildMatrices",
    ),
    (
        0x0033_4458,
        overlap_list_collide_objects,
        "OverlapList_CollideObjects",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const WP_DATA_FIND_CELL_INDEX: Callee = Callee {
        addr: 0x002b_af90,
        f: ee::s08::WPData_FindCellIndex,
    };
    pub const INPUT_VM_READ_BUILTIN: Callee = Callee {
        addr: 0x0032_5430,
        f: ee::s10::InputVM_ReadBuiltin,
    };
    pub const INPUT_VM_READ_PAD_CHANNEL: Callee = Callee {
        addr: 0x0032_52f8,
        f: ee::s10::InputVM_ReadPadChannel,
    };
    pub const FOCUS_MAN_COLLECT_OCTREE_NODE: Callee = Callee {
        addr: 0x0014_4368,
        f: ee::s01::FocusMan_CollectOctreeNode,
    };
    pub const ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX: Callee = Callee {
        addr: 0x0034_e320,
        f: ee::s11::AnimObject_GetNodeIntersectMatrix,
    };
    pub const COLLIDE_VOLUME_OBJECT: Callee = Callee {
        addr: 0x0033_4888,
        f: ee::s11::Collide_VolumeObject,
    };
    pub const RAIL_MODIFIER_FIND_CLOSEST_POINT: Callee = Callee {
        addr: 0x0035_c698,
        f: ee::s11::RailModifier_FindClosestPoint,
    };
    pub const FLEX_RAIL_FIND_CLOSEST_POINT: Callee = Callee {
        addr: 0x0034_8290,
        f: ee::s11::FlexRail_FindClosestPoint,
    };
    pub const RAIL_FIND_NEAREST_POINT: Callee = Callee {
        addr: 0x0033_5128,
        f: ee::s11::Rail_FindNearestPoint,
    };
}

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global address.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// The current stack pointer (the frame moves it, so read it afresh).
#[inline(always)]
fn sp(c: &Cpu) -> u32 {
    c.g[SP][0] as u32
}

// ---------------------------------------------------------------------------
// World painter query: the cell a key falls in.

/// `sItem* WorldPainterQuery_LookupCell(sQuery* self, sList* list)`
/// (0x002c0a10): look the key up in the list's quad tree (the tree search
/// returns an index; -1 is "no cell") and return the cell record at
/// `list->items[index]`, or null.
pub fn world_painter_query_lookup_cell(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0]; // the list
        callee::WP_DATA_FIND_CELL_INDEX.call(c, 0x002c_0a28)?;
        // The tree search leaves the cell index in v0; -1 means outside.
        let index = c.g[V0][0];
        c.g[A0][0] = index;
        c.g[V0][0] = u64::MAX;
        let outside = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
        if outside {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x8)).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Input VM: operand values.

/// `float InputVM_ReadOperand(sInputVM* self, int id)` (0x00325260): the
/// value of one operand. `id < 0x40` is a script variable (`self->v[id]`),
/// `0x40..0x60` a builtin, `0x60..0x3fc` controller channel
/// `(id - 0x60) / 6` reading `(id - 0x60) % 6`, and anything else 0.
pub fn input_vm_read_operand(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let id = c.g[A1][0];
        c.g[V0][0] = (id < 0x40) as u64;
        let not_variable = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !not_variable {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            let at = p(c.g[V0][0] as u32, 0x8);
            c.lwc1(0, at);
            c.cycles += 1;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        // 0x40..0x60: a builtin.
        c.cycles += 4;
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = (c.g[V1][0] < 0x20) as u64;
        let not_builtin = c.g[V0][0] == 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x60));
        if !not_builtin {
            c.cycles += 4;
            c.g[A1][0] = c.g[V1][0];
            callee::INPUT_VM_READ_BUILTIN.call(c, 0x0032_5298)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        // 0x60..0x3fc: controller channel `id / 6`, reading `id % 6`.
        c.cycles += 3;
        c.g[V0][0] = (c.g[A1][0] < 0x39c) as u64;
        let padded = c.g[V0][0] != 0;
        c.g[V0][0] = 0x6;
        if !padded {
            c.cycles += 3;
            c.mtc1(0, 0);
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V1][0] = 0x6;
        c.div(0, A1, V0);
        c.cycles += 4;
        c.g[A1][0] = c.lo[0];
        c.g[A2][0] = c.hi[0];
        callee::INPUT_VM_READ_PAD_CHANNEL.call(c, 0x0032_52d8)?;
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Focus manager octree.

/// `void FocusMan_CollectOctreeNode(sFocusMan* self, sOctNode* node,
/// sOctKey* key)` (0x00144368): walk the octree for the cell described by
/// `key` (level, x, y, z). The first of the manager's boxes that overlaps the
/// cell's volume (scaled by `2^level`, 0.2 margin) picks the node: the node
/// is reported to the focus manager and, unless it is a leaf or a level-11
/// node, all eight children are visited with the doubled, halved-level key.
pub fn focus_man_collect_octree_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0]; // key
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0]; // self
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A1][0]; // node
        frame.save_ra(c, 0x10);
        c.g[T1][0] = Addr(p(c.g[S2][0] as u32, 8)).lw(c); // box count
                                                          // Box loop: pick the first box overlapping the cell volume.
        let skip_cycle = loop {
            let i = c.g[A3][0] as u32;
            c.cycles += 2;
            let done = c.g[A3][0] == c.g[T1][0];
            c.g[A0][0] = sx(i << 5);
            if done {
                break false;
            }
            c.cycles += 18;
            c.g[A2][0] = Addr(p(c.g[S1][0] as u32, 4)).lw(c); // key->x
            c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0)).lw(c); // key->level
            c.mtc1(6, 1);
            c.cvt_s_w(1, 1); // f1 = (float)level ...
            c.lwc1(3, gp(c, -0x6e1c)); // 0.2 margin
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = Addr(p(c.g[S2][0] as u32, 4)).lw(c); // boxes
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7f));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
            c.sub_s(1, 1, 3);
            c.mtc1(2, 2); // f2 = 2^(level-127) from the exponent bits
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.lwc1(0, p(c.g[A0][0] as u32, 0x10));
            c.mul_s(1, 1, 2);
            c.c_le_s(1, 0);
            c.g[T0][0] = 0;
            if c.fcc() {
                c.cycles += 9;
                c.g[V1][0] = Addr(p(c.g[S1][0] as u32, 8)).lw(c); // key->y
                c.lwc1(1, p(c.g[A0][0] as u32, 0x14));
                c.mtc1(3, 0);
                c.cvt_s_w(0, 0);
                c.sub_s(0, 0, 3);
                c.mul_s(0, 0, 2);
                c.c_le_s(0, 1);
                if c.fcc() {
                    c.cycles += 10;
                    c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0xc)).lw(c); // key->z
                    c.lwc1(1, p(c.g[A0][0] as u32, 0x18));
                    c.mtc1(5, 0);
                    c.cvt_s_w(0, 0);
                    c.sub_s(0, 0, 3);
                    c.mul_s(0, 0, 2);
                    c.c_le_s(0, 1);
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    if c.fcc() {
                        c.cycles += 9;
                        c.lwc1(1, c.g[A0][0] as u32);
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.add_s(0, 0, 3);
                        c.mul_s(0, 0, 2);
                        c.c_le_s(1, 0);
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                        if c.fcc() {
                            c.cycles += 9;
                            c.lwc1(1, p(c.g[A0][0] as u32, 4));
                            c.mtc1(2, 0);
                            c.cvt_s_w(0, 0);
                            c.add_s(0, 0, 3);
                            c.mul_s(0, 0, 2);
                            c.c_le_s(1, 0);
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                            if c.fcc() {
                                c.cycles += 9;
                                c.lwc1(1, p(c.g[A0][0] as u32, 8));
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 0, 3);
                                c.mul_s(0, 0, 2);
                                c.c_le_s(1, 0);
                                if c.fcc() {
                                    c.g[T0][0] = 1;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            let no_match = c.g[T0][0] == 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            if no_match {
                if events::event_due(c) {
                    events::deliver_event_here(c, 0x0014_4398);
                }
                continue;
            }
            // The node overlaps: report it, and descend unless it is a leaf.
            c.cycles += 7;
            c.g[S0][0] = ((c.g[S0][0] as i64) < 14i64) as u64;
            c.g[A0][0] = Addr(p(c.g[S2][0] as u32, 0)).lw(c);
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S0][0];
            crate::call::call_ptr(c, 0x0010_1a28, 0x0014_44bc)?;
            if c.g[S0][0] != 0 {
                break true;
            }
            c.cycles += 4;
            c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0)).lw(c);
            c.g[V0][0] = 0xb;
            let level_11 = c.g[A1][0] == c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            if level_11 {
                break false;
            }
            // Child keys: level-1, doubled x/y/z; each child shifts one
            // coordinate by one after its visit.
            c.cycles += 12;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 4)).lw(c);
            c.g[V1][0] = Addr(p(c.g[S1][0] as u32, 8)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[A2][0] = Addr(p(c.g[S3][0] as u32, 0)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
            c.sw(sp(c), c.g[A1][0]);
            c.sw(p(sp(c), 4), c.g[V0][0]);
            c.sw(p(sp(c), 8), c.g[V1][0]);
            c.sw(p(sp(c), 0xc), c.g[A0][0]);
            if c.g[A2][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.g[A2][0];
                descend(c, 0x0014_4514)?;
            }
            // child[4] (+0x10), key.x + 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp(c), 4)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0x10)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp(c), 4), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_4534)?;
            }
            // child[6] (+0x18), key.y + 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp(c), 8)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0x18)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp(c), 8), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_4554)?;
            }
            // child[2] (+0x8), key.x - 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp(c), 4)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0x8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp(c), 4), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_4574)?;
            }
            // child[3] (+0xc), key.z + 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp(c), 0xc)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0xc)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp(c), 0xc), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_4594)?;
            }
            // child[7] (+0x1c), key.x + 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp(c), 4)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0x1c)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp(c), 4), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_45b4)?;
            }
            // child[5] (+0x14), key.y - 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp(c), 8)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0x14)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp(c), 8), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_45d4)?;
            }
            // child[1] (+0x4), key.x - 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp(c), 4)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 4)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp(c), 4), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_45f4)?;
            }
            break false;
        };
        if !skip_cycle {
            c.cycles += 1;
        }
        frame.restore(c, 16, 0x50);
        c.cycles += 6;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The recursive child visit of [`focus_man_collect_octree_node`]:
/// `FocusMan_CollectOctreeNode(self, child, key)`, returning to `ret`.
#[inline(always)]
fn descend(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = sx(sp(c));
    callee::FOCUS_MAN_COLLECT_OCTREE_NODE.call(c, ret)
}

// ---------------------------------------------------------------------------
// Animation object: bone matrices.

/// `out = a * b` with `mulScaledFS`'s layout: the four rows of `a` times the
/// broadcast columns of `b`, transposed twice through the two scratch quads
/// (`s1`, `s2`) and stored to `dst`.
fn mul_scaled(c: &mut Cpu, a: u32, b: u32, dst: u32, s1: u32, s2: u32) {
    c.lqc2(4, a);
    c.lqc2(5, p(a, 0x10));
    c.lqc2(6, p(a, 0x20));
    c.lqc2(7, p(a, 0x30));
    c.lqc2(8, b);
    c.lqc2(9, p(b, 0x10));
    c.lqc2(10, p(b, 0x20));
    c.lqc2(11, p(b, 0x30));
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
    c.sqc2(12, s1);
    c.sqc2(13, p(s1, 0x10));
    c.sqc2(14, p(s1, 0x20));
    c.sqc2(15, p(s1, 0x30));
    c.lqc2(1, s1);
    c.lqc2(2, p(s1, 0x10));
    c.lqc2(3, p(s1, 0x20));
    c.lqc2(4, p(s1, 0x30));
    c.sqc2(1, s2);
    c.sqc2(2, p(s2, 0x10));
    c.sqc2(3, p(s2, 0x20));
    c.sqc2(4, p(s2, 0x30));
    c.lqc2(1, s2);
    c.lqc2(2, p(s2, 0x10));
    c.lqc2(3, p(s2, 0x20));
    c.lqc2(4, p(s2, 0x30));
    c.sqc2(1, dst);
    c.sqc2(2, p(dst, 0x10));
    c.sqc2(3, p(dst, 0x20));
    c.sqc2(4, p(dst, 0x30));
}

/// The scaled second operand of [`mul_scaled`]: the frame's scratch matrix
/// (`+0x50`, rows 0..2) from `mtx`, with row 3 replaced by the translation
/// row times the scale quad at `sp+0..0xc`. `fx`/`fy` are the FPU registers
/// the original keeps the x/y scale in (6 and 7, swapped per branch);
/// `r01`/`r2`/`r3` are the GPRs the original loads each row through (they are
/// left holding whole quads, so their upper halves must match too).
fn scaled_rows(
    c: &mut Cpu,
    mtx: u32,
    sp: u32,
    fx: usize,
    fy: usize,
    r01: usize,
    r2: usize,
    r3: usize,
) {
    c.lq(r01, mtx);
    c.sq(r01, p(sp, 0x90));
    c.sq(r01, p(sp, 0x50));
    c.lwc1(fx, p(sp, 0));
    c.lq(r01, p(mtx, 0x10));
    c.lwc1(fy, p(sp, 4));
    c.sq(r01, p(sp, 0x90));
    c.sq(r01, p(sp, 0x60));
    c.lwc1(5, p(sp, 8));
    c.lq(r2, p(mtx, 0x20));
    c.lwc1(4, p(sp, 0xc));
    c.sq(r2, p(sp, 0x90));
    c.sq(r2, p(sp, 0x70));
    c.lq(r3, p(mtx, 0x30));
    c.sq(r3, p(sp, 0xa0));
    c.lwc1(1, p(sp, 0xa0));
    c.lwc1(2, p(sp, 0xa4));
    c.lwc1(3, p(sp, 0xa8));
    c.mul_s(1, 1, fx);
    c.lwc1(0, p(sp, 0xac));
    c.mul_s(2, 2, fy);
    c.mul_s(3, 3, 5);
    c.mul_s(0, 0, 4);
    c.swc1(1, p(sp, 0x90));
    c.swc1(2, p(sp, 0x94));
    c.swc1(3, p(sp, 0x98));
    c.swc1(0, p(sp, 0x9c));
    c.lq(2, p(sp, 0x90));
    c.sq(2, p(sp, 0x80));
}

/// `void AnimObject_BuildMatrices(AnimObject* self, const sFsMtx* root,
/// sFsMtx* out)` (0x0034dd18): one matrix per skeleton bone, `out[i]`, from
/// the bone's parent (`out[parent]`, or `root` when it has none) and its
/// animation matrix (from `AnimObject_GetNodeIntersectMatrix` for animated
/// bones, the bone's own matrix otherwise), with the model scale applied to
/// the translation row.
pub fn anim_object_build_matrices(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x1b0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        frame.save(c, S7, 0x130);
        frame.save(c, S0, 0x1a0);
        frame.save(c, S1, 0x190);
        frame.save(c, S2, 0x180);
        frame.save(c, S3, 0x170);
        frame.save(c, S4, 0x160);
        frame.save(c, S5, 0x150);
        frame.save(c, S6, 0x140);
        frame.save(c, FP, 0x120);
        frame.save_ra(c, 0x110);
        c.sw(p(sp(c), 0xf0), c.g[A0][0]);
        let model = Addr(p(c.g[A0][0] as u32, 0x2c)).lw(c);
        c.g[V0][0] = model;
        c.sw(p(sp(c), 0xf4), c.g[A2][0]);
        c.sw(p(sp(c), 0xf8), c.g[A1][0]);
        let skel = Addr(p(c.g[V0][0] as u32, 0x80)).lw(c);
        c.g[V1][0] = skel;
        c.sw(p(sp(c), 0x100), 0);
        c.sw(p(sp(c), 0xfc), c.g[V1][0]);
        c.lwc1(0, p(c.g[V0][0] as u32, 0x84)); // model scale
        c.g[S3][0] = Addr(p(c.g[V1][0] as u32, 8)).lw(c); // bones
        c.swc1(0, p(sp(c), 0x8));
        c.swc1(1, p(sp(c), 0xc));
        c.swc1(0, p(sp(c), 0));
        c.swc1(0, p(sp(c), 0x4));
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 4)).lw(c); // bone count
        let empty = c.g[V0][0] as i64 <= 0;
        c.g[S7][0] = 0;
        let spc = sp(c);
        if !empty {
            c.cycles += 7;
            c.g[S5][0] = c.g[A2][0];
            c.g[S2][0] = sx(p(spc, 0x10));
            c.g[S6][0] = c.g[S5][0];
            c.sw(p(spc, 0x104), c.g[S5][0]);
            c.g[S1][0] = sx(p(spc, 0xb0));
            c.g[S4][0] = sx(p(spc, 0x50));
            c.g[FP][0] = c.g[S6][0];
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(c.g[S3][0] as u32, 8)).lw(c); // animated
                let still = c.g[V0][0] == 0;
                c.g[A0][0] = u64::MAX;
                if still {
                    // Not animated: use the bone's own matrix.
                    c.cycles += 4;
                    c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0)).lw(c); // parent
                    c.g[V1][0] = u64::MAX;
                    let root_bone = c.g[A1][0] == c.g[V1][0];
                    c.g[V0][0] = Addr(p(spc, 0xf4)).lw(c); // out
                    if root_bone {
                        // out[i] = mul(root, bone->mtx * scale)
                        c.cycles += 77;
                        let mtx = Addr(p(c.g[S3][0] as u32, 0xc)).lw(c) as u32;
                        scaled_rows(c, mtx, spc, 6, 7, 2, 3, 2);
                        c.g[V1][0] = Addr(p(spc, 0xf8)).lw(c); // root
                        let dst = Addr(p(spc, 0x104)).lw(c);
                        c.g[A0][0] = dst;
                        mul_scaled(
                            c,
                            c.g[V1][0] as u32,
                            p(spc, 0x50),
                            c.g[A0][0] as u32,
                            p(spc, 0xb0),
                            p(spc, 0x10),
                        );
                        c.g[V1][0] = Addr(p(spc, 0xfc)).lw(c);
                    } else {
                        // out[i] = mul(out[parent], bone->mtx * scale)
                        c.cycles += 78;
                        c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
                        let mtx = Addr(p(c.g[S3][0] as u32, 0xc)).lw(c) as u32;
                        c.g[A0][0] = sx(mtx);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        scaled_rows(c, mtx, spc, 7, 6, 2, 3, 2);
                        mul_scaled(
                            c,
                            c.g[A1][0] as u32,
                            p(spc, 0x50),
                            c.g[S5][0] as u32,
                            p(spc, 0xb0),
                            p(spc, 0x10),
                        );
                        c.g[V1][0] = Addr(p(spc, 0xfc)).lw(c);
                    }
                } else {
                    // Animated: the node's intersect matrix, k counts them.
                    c.cycles += 3;
                    c.g[S0][0] = Addr(p(c.g[S3][0] as u32, 0)).lw(c); // parent
                    let root_bone = c.g[S0][0] == c.g[A0][0];
                    c.g[A0][0] = Addr(p(spc, 0xf0)).lw(c); // self
                    if root_bone {
                        // out[i] = mul(root, anim_mtx * scale)
                        c.cycles += 79;
                        c.g[A1][0] = c.g[S7][0];
                        c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(1));
                        callee::ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX.call(c, 0x0034_df10)?;
                        let mtx = c.g[V0][0] as u32;
                        scaled_rows(c, mtx, spc, 6, 7, 3, 4, 3);
                        c.g[V0][0] = Addr(p(spc, 0xf8)).lw(c); // root
                        mul_scaled(
                            c,
                            c.g[V0][0] as u32,
                            p(spc, 0x50),
                            c.g[FP][0] as u32,
                            p(spc, 0xb0),
                            p(spc, 0x10),
                        );
                        c.g[V1][0] = Addr(p(spc, 0xfc)).lw(c);
                    } else {
                        // out[i] = mul(out[parent], anim_mtx * scale)
                        c.cycles += 81;
                        c.g[A1][0] = c.g[S7][0];
                        c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(1));
                        callee::ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX.call(c, 0x0034_ddcc)?;
                        let mtx = c.g[V0][0] as u32;
                        c.g[A1][0] = sx((c.g[S0][0] as u32) << 6);
                        c.g[A0][0] = Addr(p(spc, 0xf4)).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        scaled_rows(c, mtx, spc, 7, 6, 3, 4, 3);
                        mul_scaled(
                            c,
                            c.g[A1][0] as u32,
                            p(spc, 0x50),
                            c.g[S6][0] as u32,
                            p(spc, 0xb0),
                            p(spc, 0x10),
                        );
                        c.g[V1][0] = Addr(p(spc, 0xfc)).lw(c);
                    }
                }
                // Next bone: advance the bone, both outputs and the counter.
                c.cycles += 13;
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = Addr(p(spc, 0x100)).lw(c);
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 4)).lw(c); // bone count
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = Addr(p(spc, 0x104)).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.sw(p(spc, 0x100), c.g[A0][0]);
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                let more = c.g[V0][0] != 0;
                c.sw(p(spc, 0x104), c.g[V1][0]);
                if more {
                    if events::event_due(c) {
                        events::deliver_event_here(c, 0x0034_dda8);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 12;
        frame.restore(c, 16, 0x1a0);
        frame.restore(c, 17, 0x190);
        frame.restore(c, 18, 0x180);
        frame.restore(c, 19, 0x170);
        frame.restore(c, 20, 0x160);
        frame.restore(c, 21, 0x150);
        frame.restore(c, 22, 0x140);
        frame.restore(c, 23, 0x130);
        frame.restore(c, FP, 0x120);
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    })
}
// ---------------------------------------------------------------------------
// Segment against a triangle (inlined VU0 dot/add/scale sequences).

/// `int CollSegment_CollideTriangle(sSegTree* self, sVec4* p,
/// void* c, sVec4* n, sVec4* hit, sVec4* outN, float* outT)` (0x0032e4d0):
/// the tree's segment `{origin +0x60, dir +0x70}` against the plane through
/// `p` with normal `n`. Fills the hit point and the hit normal and returns
/// the triangle test at 0x32b6a8; 0 when the segment is parallel, misses
/// behind the origin or goes past its end (`t` is still stored).
pub fn coll_segment_collide_triangle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        c.g[T4][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[A0][0] = c.g[A1][0];
        // denom = dot(dir, n) (the unit's VU0 dot sequence)
        let tree = c.g[T4][0] as u32;
        c.lqc2(3, p(tree, 0x70));
        c.lqc2(5, c.g[T0][0] as u32);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 2);
        c.lwc1(1, gp(c, -0x2d7c));
        c.abs_s(0, 2);
        c.g[A1][0] = c.g[A2][0];
        c.c_lt_s(0, 1);
        let parallel = c.fcc();
        c.g[A2][0] = c.g[A3][0];
        let mut miss = parallel;
        if !parallel {
            c.cycles += 21;
            // t = dot(p - origin, n) / denom
            c.lqc2(3, c.g[A0][0] as u32);
            c.lqc2(4, p(tree, 0x60));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, p(sp(c), 0x10));
            c.mtc1(0, 1);
            c.lq(2, p(sp(c), 0x10));
            c.sq(2, sp(c));
            c.lqc2(3, sp(c));
            c.lqc2(5, c.g[T0][0] as u32);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.div_s(2, 0, 2);
            c.c_lt_s(2, 1);
            let behind = c.fcc();
            c.swc1(2, c.g[T3][0] as u32);
            if behind {
                miss = true;
            } else {
                c.cycles += 5;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.c_lt_s(0, 2);
                let past_end = !c.fcc();
                if past_end {
                    // On the segment: hit = origin + dir * t, outN = n.
                    c.cycles += 19;
                    c.mfc1(2, 2);
                    c.lqc2(4, p(tree, 0x70));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp(c), 0x20));
                    c.lwc1(12, gp(c, -0x2d78));
                    c.lq(2, p(sp(c), 0x20));
                    c.g[A3][0] = c.g[T1][0];
                    c.sq(2, p(sp(c), 0x10));
                    c.lqc2(3, p(tree, 0x60));
                    c.lqc2(4, p(sp(c), 0x10));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, p(sp(c), 0x20));
                    let hit = c.g[T1][0] as u32;
                    c.lq(3, p(sp(c), 0x20));
                    c.sq(3, hit);
                    c.sq(3, sp(c));
                    c.lq(2, c.g[T0][0] as u32);
                    c.sq(2, c.g[T2][0] as u32); // *outN = *n
                    crate::call::call_ptr(c, 0x0032_b6a8, 0x0032_e5dc)?;
                } else {
                    miss = true;
                }
            }
        }
        if miss {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Overlap list: collide objects.

/// One inlined AABB comparison of the overlap query (`x`/`y` are the two
/// floats' addresses; `first_lt` picks `x < y` or `y < x`).
#[inline(always)]
fn box_pair(c: &mut Cpu, x: u32, y: u32, first_lt: bool) -> bool {
    c.lwc1(1, x);
    c.lwc1(0, y);
    if first_lt {
        c.c_lt_s(0, 1);
    } else {
        c.c_lt_s(1, 0);
    }
    c.fcc()
}

/// `int OverlapList_CollideObjects(sOverlapList* self, sQuery* q, char* buf,
/// int cap, int full)` (0x00334458): collide every object of the list whose
/// broad-phase box overlaps `q`'s box, writing up to `cap` contact records
/// (0x80 bytes each) to `buf`; returns how many were written. Objects with
/// flag 0x20 use their node box, objects with flag 0x40 ask their object
/// (`vt[44]` "should test", `vt[45]` box).
pub fn overlap_list_collide_objects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x2;
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A1][0]; // q
        frame.save(c, S3, 0x50);
        c.g[S2][0] = 0; // written
        frame.save(c, S4, 0x40);
        c.g[S3][0] = 0; // i
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A0][0]; // self
        frame.save(c, S6, 0x20);
        c.g[S5][0] = c.g[A3][0]; // cap
        frame.save(c, S7, 0x10);
        c.g[S6][0] = c.g[A2][0]; // buf
        frame.save(c, S0, 0x80);
        c.g[S7][0] = 0x1; // mode
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(c.g[S4][0] as u32, 8)).lw(c); // count
        let empty = c.g[V1][0] == 0;
        if c.g[T0][0] != 0 {
            c.g[S7][0] = c.g[V0][0];
        }
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
            loop {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[S0][0] = Addr(p(c.g[V0][0] as u32, 0xc)).lw(c); // node
                c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c); // flags
                c.g[V0][0] = c.g[V1][0] & 0x20;
                let no_solid = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0] & 0x40;
                let q = c.g[S1][0] as u32;
                let node = c.g[S0][0] as u32;
                let mut collide = false;
                let mut short = false;
                if !no_solid {
                    // The node's own box (+0x60..+0x74) against the query.
                    let mut overlap = 0u64;
                    c.cycles += 6;
                    if box_pair(c, p(q, 0x20), p(node, 0x60), true) {
                        c.cycles += 5;
                        if box_pair(c, p(q, 0x30), p(node, 0x6c), false) {
                            c.cycles += 5;
                            if box_pair(c, p(q, 0x24), p(node, 0x64), true) {
                                c.cycles += 5;
                                if box_pair(c, p(q, 0x34), p(node, 0x70), false) {
                                    c.cycles += 5;
                                    if box_pair(c, p(q, 0x28), p(node, 0x68), true) {
                                        c.cycles += 6;
                                        if box_pair(c, p(q, 0x38), p(node, 0x74), false) {
                                            overlap = 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.g[V0][0] = overlap;
                    c.cycles += 2;
                    let apart = c.g[V0][0] == 0;
                    c.g[A2][0] = sx((c.g[S2][0] as u32) << 7);
                    if !apart {
                        c.cycles += 2;
                        c.g[A3][0] = sx((c.g[S5][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
                        collide = true;
                    }
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        // Neither flag: nothing to do this node.
                        c.g[V0][0] = Addr(p(c.g[S4][0] as u32, 8)).lw(c);
                        short = true;
                    } else {
                        // Ask the object whether it should be tested.
                        c.cycles += 8;
                        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xc)).lw(c);
                        c.g[V1][0] = Addr(p(c.g[A1][0] as u32, 0xc)).lw(c);
                        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x160)).lh(c);
                        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x164)).lw(c);
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        crate::call::call_ptr(c, target, 0x0033_4578)?;
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = Addr(p(c.g[S4][0] as u32, 8)).lw(c);
                            short = true;
                        } else {
                            // Its box from vt[45], then the same comparisons.
                            c.cycles += 12;
                            c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xc)).lw(c);
                            c.g[V1][0] = Addr(p(c.g[A1][0] as u32, 0xc)).lw(c);
                            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x168)).lh(c);
                            c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x16c)).lw(c);
                            let target = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            crate::call::call_ptr(c, target, 0x0033_4598)?;
                            let bx = c.g[V0][0] as u32;
                            let mut overlap = 0u64;
                            if box_pair(c, p(q, 0x20), bx, true) {
                                c.cycles += 5;
                                if box_pair(c, p(q, 0x30), p(bx, 0x10), false) {
                                    c.cycles += 5;
                                    if box_pair(c, p(q, 0x24), p(bx, 4), true) {
                                        c.cycles += 5;
                                        if box_pair(c, p(q, 0x34), p(bx, 0x14), false) {
                                            c.cycles += 5;
                                            if box_pair(c, p(q, 0x28), p(bx, 8), true) {
                                                c.cycles += 6;
                                                if box_pair(c, p(bx, 0x18), p(q, 0x38), true) {
                                                    overlap = 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.g[V1][0] = overlap;
                            c.cycles += 2;
                            let apart = c.g[V1][0] == 0;
                            c.g[A2][0] = sx((c.g[S2][0] as u32) << 7);
                            if !apart {
                                c.cycles += 1;
                                c.g[A3][0] =
                                    sx((c.g[S5][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
                                collide = true;
                            }
                        }
                    }
                }
                if collide {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[T0][0] = c.g[S7][0];
                    callee::COLLIDE_VOLUME_OBJECT.call(c, 0x0033_4638)?;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                }
                if !short {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(c.g[S4][0] as u32, 8)).lw(c);
                }
                c.cycles += 4;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                if more {
                    if events::event_due(c) {
                        events::deliver_event_here(c, 0x0033_44b0);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 12;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore(c, 20, 0x40);
        frame.restore(c, 21, 0x30);
        frame.restore(c, 22, 0x20);
        frame.restore(c, 23, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Overlap list: nearest rail.

/// `int OverlapList_FindNearestRail(sOverlapList* self, const sVec4* pos,
/// void* a2, int mask, float r)` (0x00334680): for every rail object of the
/// list whose type passes the mask, find the point closest to `pos` (rails
/// via `RailModifier_FindClosestPoint`/`FlexRail_FindClosestPoint`, plain
/// rails via `Rail_FindNearestPoint`) and keep the nearest contact; returns
/// the number of contacts found. The sphere `r` around `pos` is the query
/// volume (`box.min = pos - r`, `box.max = pos + r`).
pub fn overlap_list_find_nearest_rail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0xe0);
        c.mtc1(0, 0); // f0 = 0
        frame.save(c, S0, 0xd0);
        frame.save(c, S2, 0xb0);
        c.g[S0][0] = c.g[A1][0]; // pos
        frame.save(c, S3, 0xa0);
        c.g[S2][0] = 0; // i
        frame.save(c, S4, 0x90);
        c.g[S3][0] = c.g[A3][0]; // mask
        frame.save(c, S5, 0x80);
        c.g[S4][0] = c.g[A2][0]; // a2
        frame.save(c, S6, 0x70);
        c.g[S5][0] = c.g[A0][0]; // self
        frame.save(c, S7, 0x60);
        frame.save(c, S1, 0xc0);
        frame.save(c, FP, 0x50);
        frame.save_ra(c, 0x40);
        c.lq(2, c.g[S0][0] as u32);
        c.swc1(12, p(sp(c), 0x28));
        c.sq(2, p(sp(c), 0x10));
        c.swc1(0, p(sp(c), 0x2c));
        c.sw(p(sp(c), 0x30), 0); // cnt
        c.swc1(0, p(sp(c), 0x34)); // dist
        c.sq(2, sp(c));
        c.swc1(12, p(sp(c), 0x20));
        c.swc1(12, p(sp(c), 0x24));
        c.lqc2(3, sp(c));
        c.lqc2(4, p(sp(c), 0x20));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c));
        c.lqc2(3, p(sp(c), 0x10));
        c.lqc2(4, p(sp(c), 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(sp(c), 0x10));
        c.g[V0][0] = Addr(p(c.g[S5][0] as u32, 0x210)).lw(c); // count
        let empty = c.g[V0][0] == 0;
        c.g[FP][0] = 0x2; // FP holds the "rail modifier" type
        if !empty {
            c.cycles += 3;
            c.g[S7][0] = 0x3; // "flex rail" type
            c.g[S6][0] = 0x1; // "rail" type
            c.g[S1][0] = sx(p(c.g[S5][0] as u32, 0x214)); // &items[0]
            loop {
                c.cycles += 4;
                c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0)).lw(c); // object
                c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 8)).lw(c); // type
                let is_modifier = c.g[V0][0] == c.g[FP][0];
                c.g[A1][0] = c.g[SP][0];
                let mut finished = false;
                if is_modifier {
                    c.cycles += 10;
                    c.g[A0][0] = Addr(p(c.g[A0][0] as u32, 0xc)).lw(c);
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = c.g[S3][0];
                    c.g[T1][0] = sx(p(sp(c), 0x30));
                    c.g[T2][0] = sx(p(sp(c), 0x34));
                    c.g[T3][0] = c.g[S4][0];
                    callee::RAIL_MODIFIER_FIND_CLOSEST_POINT.call(c, 0x0033_4750)?;
                    c.g[V0][0] = Addr(p(c.g[S5][0] as u32, 0x210)).lw(c);
                    finished = true;
                }
                if !finished {
                    c.cycles += 2;
                    let is_flex = c.g[V0][0] == c.g[S7][0];
                    c.g[A2][0] = c.g[SP][0];
                    if is_flex {
                        c.cycles += 9;
                        c.g[A0][0] = Addr(p(c.g[A0][0] as u32, 0xc)).lw(c);
                        c.g[A3][0] = c.g[S0][0];
                        c.g[T0][0] = c.g[S3][0];
                        c.g[T1][0] = sx(p(sp(c), 0x30));
                        c.g[T2][0] = sx(p(sp(c), 0x34));
                        c.g[T3][0] = c.g[S4][0];
                        callee::FLEX_RAIL_FIND_CLOSEST_POINT.call(c, 0x0033_477c)?;
                        c.g[V0][0] = Addr(p(c.g[S5][0] as u32, 0x210)).lw(c);
                        finished = true;
                    }
                }
                if !finished {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S6][0] {
                        c.g[V0][0] = Addr(p(c.g[S5][0] as u32, 0x210)).lw(c);
                    } else {
                        // A plain rail: only when the object's flags pass.
                        c.cycles += 5;
                        c.g[V1][0] = Addr(p(c.g[A0][0] as u32, 0x68)).lw(c);
                        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x1c)).lw(c);
                        c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                        let masked_out = c.g[V0][0] == 0;
                        c.g[A1][0] = c.g[SP][0];
                        if !masked_out {
                            c.cycles += 5;
                            c.g[A2][0] = c.g[S0][0];
                            c.g[A3][0] = sx(p(sp(c), 0x30));
                            c.g[T0][0] = sx(p(sp(c), 0x34));
                            c.g[T1][0] = c.g[S4][0];
                            callee::RAIL_FIND_NEAREST_POINT.call(c, 0x0033_47b4)?;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = Addr(p(c.g[S5][0] as u32, 0x210)).lw(c);
                    }
                }
                c.cycles += 4;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                if more {
                    if events::event_due(c) {
                        events::deliver_event_here(c, 0x0033_4720);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 13;
        c.g[V0][0] = Addr(p(sp(c), 0x30)).lw(c); // contacts found
        frame.restore(c, 16, 0xd0);
        frame.restore(c, 17, 0xc0);
        frame.restore(c, 18, 0xb0);
        frame.restore(c, 19, 0xa0);
        frame.restore(c, 20, 0x90);
        frame.restore(c, 21, 0x80);
        frame.restore(c, 22, 0x70);
        frame.restore(c, 23, 0x60);
        frame.restore(c, FP, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fake_call, machine, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 200;

    /// The bits of a float.
    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    fn put_v(c: &mut Cpu, at: u32, v: [f32; 4]) {
        for (k, x) in v.iter().enumerate() {
            c.bus.write32(at + 4 * k as u32, f(*x));
        }
    }

    #[test]
    fn world_painter_query_lookup_cell_matches() {
        const LIST: u32 = DATA + 0x1000;
        check(
            ssx3_ee::s08::f_002c0a10__generated,
            world_painter_query_lookup_cell,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x51ed_0001 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let node = LIST + 0x100;
                let cell = LIST + 0x200;
                c.g[A0][0] = sx(LIST + 0x800);
                c.g[A1][0] = sx(LIST);
                c.bus.write32(LIST, node); // the tree root
                c.bus.write32(LIST + 8, LIST + 0x300); // items
                for k in 0..4 {
                    c.bus.write32(node + 4 * k, 0);
                }
                c.bus.write16(node + 0x14, 0); // no children
                c.bus.write32(node + 0x20, cell); // leaf data
                c.bus.write16(cell, 0); // a leaf
                c.bus.write32(
                    cell + 4,
                    if seed % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 4
                    },
                );
                c.f[12] = 0;
                c.f[13] = 0;
            },
        );
    }

    #[test]
    fn input_vm_read_operand_matches() {
        check(
            ssx3_ee::s10::f_00325260__generated,
            input_vm_read_operand,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4002, 0);
                c.g[A0][0] = sx(DATA);
                let id = match seed % 4 {
                    0 => r.next_u32() % 0x40,          // a script variable
                    1 => 0x40 + r.next_u32() % 0x20,   // a builtin
                    2 => 0x60 + r.next_u32() % 0x39c,  // a controller channel
                    _ => 0x3fc + r.next_u32() % 0x100, // past the end
                };
                c.g[A1][0] = sx(id);
                c.bus.write32(DATA + 4, DATA + 0x400); // pad state
                c.bus.write32(DATA + 0x400, 8); // channel count
                for k in 0..0x90 {
                    c.bus.write32(DATA + 8 + 4 * k, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn coll_segment_collide_triangle_matches() {
        const TREE: u32 = DATA + 0x2000;
        const N: u32 = DATA + 0x100;
        const P: u32 = DATA + 0x110;
        const B: u32 = DATA + 0x120;
        const CC: u32 = DATA + 0x130;
        const HIT: u32 = DATA + 0x140;
        const OUT_N: u32 = DATA + 0x150;
        const OUT_T: u32 = DATA + 0x160;
        check(
            ssx3_ee::s11::f_0032e4d0__generated,
            coll_segment_collide_triangle,
            SEEDS,
            &|c, seed| {
                let _r = machine(c, seed, 0x4003, 0);
                // The unit's epsilon and tolerance constants.
                c.bus.write32(GAME_GP.wrapping_sub(0x2d7c), 0x2edb_e6ff);
                c.bus.write32(GAME_GP.wrapping_sub(0x2d78), 0x38d1_b717);
                let (dx, nx) = match seed % 20 {
                    0 => (0.0, 0.0),      // the segment is parallel to the plane
                    1..=6 => (-1.0, 1.0), // the hit is behind the origin
                    7..=11 => (2.0, 1.0), // the hit is past the end
                    _ => (0.5, 1.0),      // on the segment
                };
                put_v(c, TREE + 0x60, [0.0, 0.0, 0.0, 0.0]); // origin
                put_v(c, TREE + 0x70, [1.0, 0.0, 0.0, 0.0]); // dir
                put_v(c, N, [nx, 0.0, 0.0, 0.0]);
                put_v(c, P, [dx, 0.0, 0.0, 0.0]);
                put_v(c, B, [0.25, 0.5, 0.75, 1.0]);
                put_v(c, CC, [-0.25, -0.5, -0.75, 1.0]);
                put_v(c, HIT, [0.0; 4]);
                put_v(c, OUT_N, [0.0; 4]);
                c.bus.write32(OUT_T, 0);
                c.g[A0][0] = sx(TREE);
                c.g[A1][0] = sx(P);
                c.g[A2][0] = sx(B);
                c.g[A3][0] = sx(CC);
                c.g[T0][0] = sx(N);
                c.g[T1][0] = sx(HIT);
                c.g[T2][0] = sx(OUT_N);
                c.g[T3][0] = sx(OUT_T);
            },
        );
    }
    

    #[test]
    fn anim_object_build_matrices_matches() {
        const SELF: u32 = DATA + 0x1000;
        const MODEL: u32 = DATA + 0x1100;
        const SKEL: u32 = DATA + 0x1200;
        const BONES: u32 = DATA + 0x1400;
        const OUT: u32 = DATA + 0x2000;
        const ROOT: u32 = DATA + 0x3000;
        const NODES: u32 = DATA + 0x4000;
        const OWN: u32 = DATA + 0x6000;
        check(
            ssx3_ee::s11::f_0034dd18__generated,
            anim_object_build_matrices,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4005, 0);
                let n = 1 + seed % 4;
                c.bus.write32(SELF + 0x2c, MODEL);
                c.bus.write32(SELF + 0x48, NODES);
                c.bus.write32(MODEL + 0x80, SKEL);
                c.bus.write32(MODEL + 0x84, f(1.0));
                c.bus.write32(SKEL + 4, n);
                c.bus.write32(SKEL + 8, BONES);
                for i in 0..n {
                    let b = BONES + 0x10 * i;
                    let parent = if i == 0 || seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        i - 1
                    };
                    c.bus.write32(b, parent);
                    c.bus.write32(b + 8, (seed + i) % 2);
                    c.bus.write32(b + 0xc, OWN + 0x40 * i);
                    for k in 0..16 {
                        c.bus.write32(OWN + 0x40 * i + 4 * k, r.f32(2.0));
                    }
                }
                for k in 0..n {
                    let m = NODES + 0x90 + 0xd0 * k;
                    for j in 0..16 {
                        c.bus.write32(m + 4 * j, r.f32(1.0));
                    }
                }
                for k in 0..16 {
                    c.bus.write32(ROOT + 4 * k, r.f32(1.0));
                }
                for k in 0..(16 * n) {
                    c.bus.write32(OUT + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(ROOT);
                c.g[A2][0] = sx(OUT);
            },
        );
    }

    #[test]
    fn overlap_list_collide_objects_matches() {
        const LIST: u32 = DATA + 0x1000;
        const Q: u32 = DATA + 0x1200;
        const BUF: u32 = DATA + 0x1400;
        const OBJ: u32 = DATA + 0x1c00;
        const VT: u32 = DATA + 0x1800;
        const BOX: u32 = DATA + 0x1a00;
        const NODES: u32 = DATA + 0x2000;
        check(
            ssx3_ee::s11::f_00334458__generated,
            overlap_list_collide_objects,
            SEEDS,
            &|c, seed| {
                let _r = machine(c, seed, 0x4006, 0);
                // The object's methods: "should test" (0x160), box (0x168), and
                // the collider's own first query (0x130, answered "no").
                c.hooks.miss = |c: &mut Cpu, target: u32| {
                    fake_call(c, target);
                    match target.wrapping_sub(FAKE) {
                        0x20 => c.g[V0][0] = BOX as u64,
                        0x30 => c.g[V0][0] = 0,
                        _ => {}
                    }
                };
                c.bus.write16(VT + 0x130, 0);
                c.bus.write32(VT + 0x134, FAKE + 0x30);
                c.bus.write16(VT + 0x160, 0);
                // Every third seed has the object answer "do not test it".
                c.bus
                    .write32(VT + 0x164, if seed % 3 == 0 { FAKE + 0x30 } else { FAKE });
                c.bus.write16(VT + 0x168, 0);
                c.bus.write32(VT + 0x16c, FAKE + 0x20);
                c.bus.write32(OBJ + 0xc, VT);
                // The query box and the object box that overlaps it.
                for k in 0..3 {
                    c.bus.write32(Q + 0x20 + 4 * k, f(1.0));
                    c.bus.write32(Q + 0x30 + 4 * k, f(-1.0));
                    c.bus.write32(BOX + 4 * k, f(0.0));
                    c.bus.write32(BOX + 0x10 + 4 * k, f(0.0));
                }
                let count = seed % 5;
                c.bus.write32(LIST + 8, count);
                for i in 0..count {
                    let node = NODES + 0x100 * i;
                    let flags = match i % 4 {
                        0 => 0x20, // the node's own box, overlapping
                        1 => 0x20, // the node's own box, apart
                        2 => 0x40, // ask the object
                        _ => 0,
                    };
                    c.bus.write32(node + 8, flags);
                    c.bus
                        .write32(node + 0xc, if flags == 0x40 { OBJ } else { 0 });
                    for k in 0..3 {
                        let min = if i % 4 == 1 { 2.0 } else { 0.0 };
                        c.bus.write32(node + 0x60 + 4 * k, f(min));
                        c.bus.write32(node + 0x6c + 4 * k, f(0.0));
                    }
                    c.bus.write32(LIST + 0xc + 4 * i, node);
                }
                c.g[A0][0] = sx(LIST);
                c.g[A1][0] = sx(Q);
                c.g[A2][0] = sx(BUF);
                c.g[A3][0] = sx(8);
                c.g[T0][0] = if seed % 2 == 0 { 1 } else { 0 };
            },
        );
    }

    /// Not listed in `OVERRIDES`: the rail finders it calls
    /// (`RailModifier_FindClosestPoint`, `FlexRail_FindClosestPoint`,
    /// `Rail_FindNearestPoint`) are dispatchers that need real rail data, so
    /// the test only reaches the loop and the masked-out rail object. The
    /// port stays here for the call paths to be tested on real data.
    #[test]
    fn overlap_list_find_nearest_rail_matches() {
        const LIST: u32 = DATA + 0x1000;
        const POS: u32 = DATA + 0x2000;
        const POS2: u32 = DATA + 0x2100;
        const NODES: u32 = DATA + 0x4000;
        check(
            ssx3_ee::s11::f_00334680,
            overlap_list_find_nearest_rail,
            SEEDS,
            &|c, seed| {
                let _r = machine(c, seed, 0x4007, 0);
                let count = if seed % 5 == 0 { 0 } else { 1 + seed % 3 };
                c.bus.write32(LIST + 0x210, count);
                for i in 0..count {
                    let node = NODES + 0x100 * i;
                    // Plain rails (type 1) whose flags fail the mask, and
                    // objects of no rail type at all.
                    c.bus.write32(node + 8, if seed % 3 == 0 { 1 } else { 0 });
                    c.bus.write32(node + 0x68, node + 0x90);
                    c.bus.write32(node + 0x90 + 0x1c, 0);
                    c.bus.write32(LIST + 0x214 + 4 * i, node);
                }
                put_v(c, POS, [4.0, 3.0, 2.0, 0.0]);
                c.g[A0][0] = sx(LIST);
                c.g[A1][0] = sx(POS);
                c.g[A2][0] = sx(POS2);
                c.g[A3][0] = sx(0xffff_ffff);
                c.f[12] = f(1.0);
            },
        );
    }
}
