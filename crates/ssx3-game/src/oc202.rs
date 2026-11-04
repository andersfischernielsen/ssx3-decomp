//! Staging wave 201 (oc202): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size),
//! one function per writer. Each reproduces the generated body's registers,
//! cycles, memory and call order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0033_2db8, overlap_list_gather_all, "OverlapList_GatherAll"),
    (0x001e_a930, c_ov_state_hud1p_update, "cOVStateHUD1P_Update"),
    (
        0x002e_db20,
        world_painter_sample_surface_colours,
        "WorldPainter_SampleSurfaceColours",
    ),
];

// ---------------------------------------------------------------------------
// OverlapList_GatherAll (0x00332db8).

/// Overlap list fields (`sOverlapList`): the world sphere tree's grid, a
/// stamp copied from `grid+0xa0`, and three result lists (count, then the
/// pointer array): nodes (cap 0x40), objects ("tris", cap 0x40) and the
/// all-list (cap 0x200).
mod gather_all_list {
    pub const GRID: u32 = 0x0;
    pub const STAMP: u32 = 0x4;
    pub const NODE_COUNT: u32 = 0x8;
    pub const NODES: u32 = 0xc;
    pub const OBJ_COUNT: u32 = 0x10c;
    pub const OBJS: u32 = 0x110;
    pub const ALL_COUNT: u32 = 0x210;
    pub const ALL: u32 = 0x214;
    /// `grid+0xa0`: copied to `STAMP`.
    pub const GRID_STAMP: u32 = 0xa0;
    /// The grid holds eight top-level cells of 0x14 bytes:
    /// `{ int level, x, y, z; TreeNode* root; }`.
    pub const CELL_SIZE: u32 = 0x14;
    pub const CELL_ROOT: u32 = 0x10;
}

/// Tree node fields (see `Cell_GatherAllRecursive`): eight children at
/// +0..+0x1c, then the node, object and all-lists.
mod gather_all_node {
    pub const OBJECTS: u32 = 0x24;
    pub const ALL: u32 = 0x28;
    /// An object's box: min at +0x158, max at +0x164.
    pub const OBJ_MIN: u32 = 0x158;
    pub const OBJ_MAX: u32 = 0x164;
}

/// `SpatialKey_ClassifyAABB(cell, box)` (0x00328360, through the function
/// table): 0 the box lies within the cell (gather the whole subtree), 1
/// disjoint (skip), 2 partly overlapping (recurse).
const GATHER_ALL_CLASSIFY: u32 = 0x0032_8360;

/// Where the eight inlined cell queries return to (each copy of the
/// inlined `queryCell` has its own call sites and loop heads).
struct GatherAllCell {
    classify_ret: u32,
    objects_ret: u32,
    objs_loop: u32,
    all_loop: u32,
    subtree_ret: [u32; 8],
    recursive_ret: u32,
    /// Cycles of the block that calls `Cell_GatherObjects`.
    objects_cycles: u64,
    /// Cycles before the object scan loop.
    scan_cycles: u64,
}

const GATHER_ALL_CELLS: [GatherAllCell; 8] = [
    GatherAllCell {
        classify_ret: 0x0033_2e80,
        objects_ret: 0x0033_2e98,
        objs_loop: 0x0033_2ea8,
        all_loop: 0x0033_2f70,
        subtree_ret: [
            0x0033_2fac, 0x0033_2fc0, 0x0033_2fd4, 0x0033_2fe8, 0x0033_2ffc, 0x0033_3010,
            0x0033_3024, 0x0033_3038,
        ],
        recursive_ret: 0x0033_3064,
        objects_cycles: 6,
        scan_cycles: 1,
    },
    GatherAllCell {
        classify_ret: 0x0033_30a8,
        objects_ret: 0x0033_30c0,
        objs_loop: 0x0033_30d8,
        all_loop: 0x0033_31a0,
        subtree_ret: [
            0x0033_31dc, 0x0033_31f0, 0x0033_3204, 0x0033_3218, 0x0033_322c, 0x0033_3240,
            0x0033_3254, 0x0033_3268,
        ],
        recursive_ret: 0x0033_3294,
        objects_cycles: 7,
        scan_cycles: 2,
    },
    GatherAllCell {
        classify_ret: 0x0033_32b0,
        objects_ret: 0x0033_32c8,
        objs_loop: 0x0033_32e0,
        all_loop: 0x0033_33a8,
        subtree_ret: [
            0x0033_33e4, 0x0033_33f8, 0x0033_340c, 0x0033_3420, 0x0033_3434, 0x0033_3448,
            0x0033_345c, 0x0033_3470,
        ],
        recursive_ret: 0x0033_349c,
        objects_cycles: 7,
        scan_cycles: 2,
    },
    GatherAllCell {
        classify_ret: 0x0033_34b8,
        objects_ret: 0x0033_34d0,
        objs_loop: 0x0033_34e8,
        all_loop: 0x0033_35b0,
        subtree_ret: [
            0x0033_35ec, 0x0033_3600, 0x0033_3614, 0x0033_3628, 0x0033_363c, 0x0033_3650,
            0x0033_3664, 0x0033_3678,
        ],
        recursive_ret: 0x0033_36a4,
        objects_cycles: 7,
        scan_cycles: 2,
    },
    GatherAllCell {
        classify_ret: 0x0033_36c0,
        objects_ret: 0x0033_36d8,
        objs_loop: 0x0033_36f0,
        all_loop: 0x0033_37b8,
        subtree_ret: [
            0x0033_37f4, 0x0033_3808, 0x0033_381c, 0x0033_3830, 0x0033_3844, 0x0033_3858,
            0x0033_386c, 0x0033_3880,
        ],
        recursive_ret: 0x0033_38ac,
        objects_cycles: 7,
        scan_cycles: 2,
    },
    GatherAllCell {
        classify_ret: 0x0033_38cc,
        objects_ret: 0x0033_38e4,
        objs_loop: 0x0033_38f8,
        all_loop: 0x0033_39c0,
        subtree_ret: [
            0x0033_39fc, 0x0033_3a10, 0x0033_3a24, 0x0033_3a38, 0x0033_3a4c, 0x0033_3a60,
            0x0033_3a74, 0x0033_3a88,
        ],
        recursive_ret: 0x0033_3ab4,
        objects_cycles: 7,
        scan_cycles: 1,
    },
    GatherAllCell {
        classify_ret: 0x0033_3ad4,
        objects_ret: 0x0033_3aec,
        objs_loop: 0x0033_3b00,
        all_loop: 0x0033_3bc8,
        subtree_ret: [
            0x0033_3c04, 0x0033_3c18, 0x0033_3c2c, 0x0033_3c40, 0x0033_3c54, 0x0033_3c68,
            0x0033_3c7c, 0x0033_3c90,
        ],
        recursive_ret: 0x0033_3cbc,
        objects_cycles: 7,
        scan_cycles: 1,
    },
    GatherAllCell {
        classify_ret: 0x0033_3cdc,
        objects_ret: 0x0033_3cf4,
        objs_loop: 0x0033_3d08,
        all_loop: 0x0033_3dd0,
        subtree_ret: [
            0x0033_3e0c, 0x0033_3e20, 0x0033_3e34, 0x0033_3e48, 0x0033_3e5c, 0x0033_3e70,
            0x0033_3e84, 0x0033_3e98,
        ],
        recursive_ret: 0x0033_3ec4,
        objects_cycles: 7,
        scan_cycles: 1,
    },
];

/// Where a cell query finds the cell pointer it passes to
/// `Cell_GatherAllRecursive`: a register, or a frame slot.
#[derive(Clone, Copy)]
enum GatherAllCellPtr {
    Reg(usize),
    Slot(u32),
}

/// `void OverlapList_GatherAll(sOverlapList* self, const float* box)`
/// (0x00332db8): rebuild the overlap list's three result lists (nodes,
/// objects, all) for the query box. A gather context on the stack holds
/// three `{ list, count*, cap, box }` sub-contexts (nodes at sp+0, objects
/// at sp+0x10, all at sp+0x20); the three counts are reset, then each of the
/// grid's eight top-level cells (the inlined `queryCell`) with a tree is
/// classified against the box: a cell the box lies within has its root's
/// nodes gathered (`Cell_GatherObjects`), its objects whose box touches the
/// query box and its whole all-list appended, and its eight children
/// gathered whole (`Cell_GatherAllSubtree`); a partly overlapping cell is
/// recursed into (`Cell_GatherAllRecursive`); a disjoint one is skipped.
pub fn overlap_list_gather_all(c: &mut Cpu) {
    use gather_all_list as list;
    guest(c, |c| {
        // Frame and the gather context.
        c.cycles += 46;
        let frame = Frame::open(c, 0xf0);
        frame.save(c, S0, 0xe0);
        frame.save(c, S2, 0xc0);
        frame.save(c, S4, 0xa0);
        frame.save(c, S5, 0x90);
        frame.save(c, S1, 0xd0);
        frame.save(c, S3, 0xb0);
        frame.save(c, S6, 0x80);
        frame.save(c, S7, 0x70);
        frame.save(c, FP, 0x60);
        frame.save_ra(c, 0x50);
        let this = r32(c, A0);
        let s = sp(c);
        c.g[A3][0] = 0x40;
        c.g[V1][0] = sx(this.wrapping_add(list::NODES));
        c.g[A2][0] = sx(this.wrapping_add(list::OBJS));
        c.g[S2][0] = sx(s.wrapping_add(0x10));
        c.g[T0][0] = sx(this.wrapping_add(list::ALL));
        c.g[S5][0] = c.g[A1][0];
        c.g[A1][0] = sx(this.wrapping_add(list::NODE_COUNT));
        c.g[S4][0] = sx(s.wrapping_add(0x20));
        c.g[V0][0] = c.lw(this.wrapping_add(list::GRID));
        c.g[T1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(list::GRID_STAMP));
        c.g[V0][0] = sx(this.wrapping_add(list::OBJ_COUNT));
        // nodes = { this->nodes, &this->node_count, 0x40, box }
        c.sw(s, c.g[V1][0]);
        c.sw(s.wrapping_add(0x4), c.g[A1][0]);
        c.g[V1][0] = 0x200;
        // objects = { this->objs, &this->obj_count, 0x40, box }
        c.sw(s.wrapping_add(0x10), c.g[A2][0]);
        c.g[A1][0] = sx(this.wrapping_add(list::ALL_COUNT));
        c.sw(s.wrapping_add(0x14), c.g[V0][0]);
        c.sw(s.wrapping_add(0x18), c.g[A3][0]);
        c.sw(s.wrapping_add(0x8), c.g[A3][0]);
        c.sw(s.wrapping_add(0xc), c.g[S5][0]);
        c.sw(s.wrapping_add(0x1c), c.g[S5][0]);
        c.sw(this.wrapping_add(list::STAMP), c.g[T1][0]);
        // node_count = 0; *objects.count = 0
        c.g[V0][0] = c.lw(s.wrapping_add(0x14));
        c.sw(this.wrapping_add(list::NODE_COUNT), 0);
        c.sw(c.g[V0][0] as u32, 0);
        // all = { this->all, &this->all_count, 0x200, box }; *all.count = 0
        c.sw(s.wrapping_add(0x20), c.g[T0][0]);
        c.sw(s.wrapping_add(0x24), c.g[A1][0]);
        c.sw(s.wrapping_add(0x28), c.g[V1][0]);
        c.sw(s.wrapping_add(0x2c), c.g[S5][0]);
        c.g[V0][0] = c.lw(s.wrapping_add(0x24));
        c.sw(c.g[V0][0] as u32, 0);
        c.g[S6][0] = c.lw(this.wrapping_add(list::GRID));
        let grid = r32(c, S6);
        c.g[S1][0] = c.lw(grid.wrapping_add(list::CELL_ROOT));
        c.g[S0][0] = c.g[SP][0];

        // Cell 0 (the grid pointer itself is the cell).
        if c.g[S1][0] != 0 {
            c.cycles += 7;
            c.g[S3][0] = sx(s.wrapping_add(0x30));
            c.g[A0][0] = c.g[S6][0];
            c.g[A1][0] = c.g[S5][0];
            call_ptr(c, GATHER_ALL_CLASSIFY, GATHER_ALL_CELLS[0].classify_ret)?;
            gather_all_query_cell(c, &GATHER_ALL_CELLS[0], S1, S3, GatherAllCellPtr::Reg(S6), true)?;
        } else {
            c.cycles += 1;
        }
        gather_all_next_root(c, 0);

        // Cell 1; cells 2..4 are kept in s4/s7/fp, 5..7 in frame slots.
        c.cycles += 11;
        let cell = |k: u32| grid.wrapping_add(list::CELL_SIZE * k);
        c.g[V1][0] = sx(cell(5));
        c.sw(s.wrapping_add(0x38), c.g[V1][0]);
        c.g[S1][0] = sx(cell(1));
        c.g[V1][0] = sx(cell(6));
        c.g[S4][0] = sx(cell(2));
        c.sw(s.wrapping_add(0x3c), c.g[V1][0]);
        c.g[S7][0] = sx(cell(3));
        c.g[V1][0] = sx(cell(7));
        c.g[FP][0] = sx(cell(4));
        c.sw(s.wrapping_add(0x40), c.g[V1][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            c.g[S2][0] = c.lw(r32(c, S1).wrapping_add(list::CELL_ROOT));
            c.g[S3][0] = sx(s.wrapping_add(0x31));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S5][0];
            call_ptr(c, GATHER_ALL_CLASSIFY, GATHER_ALL_CELLS[1].classify_ret)?;
            gather_all_query_cell(c, &GATHER_ALL_CELLS[1], S2, S3, GatherAllCellPtr::Reg(S1), false)?;
        } else {
            c.cycles += 1;
        }
        gather_all_next_root(c, 1);

        // Cells 2..4: the cell pointer is in a saved register.
        for (i, cell_reg) in [(2usize, S4), (3, S7), (4, FP)] {
            c.cycles += 2;
            let present = c.g[V0][0] != 0;
            c.g[S2][0] = sx(s.wrapping_add(0x30 + i as u32));
            if present {
                c.cycles += 7;
                c.g[S1][0] = c.lw(r32(c, cell_reg).wrapping_add(list::CELL_ROOT));
                c.g[A0][0] = c.g[cell_reg][0];
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, GATHER_ALL_CLASSIFY, GATHER_ALL_CELLS[i].classify_ret)?;
                let ptr = GatherAllCellPtr::Reg(cell_reg);
                gather_all_query_cell(c, &GATHER_ALL_CELLS[i], S1, S2, ptr, false)?;
            } else {
                c.cycles += 1;
            }
            gather_all_next_root(c, i);
        }

        // Cells 5..7: the cell pointer is reloaded from its frame slot (into
        // v0, v1, v0).
        for (i, slot, ptr_reg) in [(5usize, 0x38u32, V0), (6, 0x3c, V1), (7, 0x40, V0)] {
            c.cycles += 2;
            let present = c.g[V0][0] != 0;
            c.g[ptr_reg][0] = c.lw(s.wrapping_add(slot));
            if present {
                c.cycles += 8;
                c.g[S2][0] = sx(s.wrapping_add(0x30 + i as u32));
                c.g[A1][0] = c.g[S5][0];
                c.g[S1][0] = c.lw(r32(c, ptr_reg).wrapping_add(list::CELL_ROOT));
                c.g[A0][0] = c.g[ptr_reg][0];
                call_ptr(c, GATHER_ALL_CLASSIFY, GATHER_ALL_CELLS[i].classify_ret)?;
                let ptr = GatherAllCellPtr::Slot(slot);
                gather_all_query_cell(c, &GATHER_ALL_CELLS[i], S1, S2, ptr, false)?;
            } else {
                c.cycles += 1;
            }
            gather_all_next_root(c, i);
        }

        // Epilogue (s0 was reloaded by the last cell query).
        c.cycles += 11;
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// End of cell query `i`: load the next cell's tree root into v0 (after the
/// last cell, the epilogue's first restore of s0 sits here).
fn gather_all_next_root(c: &mut Cpu, i: usize) {
    if i < 7 {
        let next = gather_all_list::CELL_SIZE * (i as u32 + 1) + gather_all_list::CELL_ROOT;
        c.g[V0][0] = c.lw(r32(c, S6).wrapping_add(next));
    } else {
        c.lq(S0, sp(c).wrapping_add(0xe0));
    }
}

/// One inlined `queryCell` after `SpatialKey_ClassifyAABB` returned (v0):
/// `root` holds the cell's tree root, `tag` the scratch pointer passed as
/// `Cell_GatherAllRecursive`'s unused first argument. `first` is the copy
/// for cell 0, which keeps the object context in s2 and leaves a3 alone.
fn gather_all_query_cell(
    c: &mut Cpu,
    q: &GatherAllCell,
    root: usize,
    tag: usize,
    cell: GatherAllCellPtr,
    first: bool,
) -> crate::call::Flow {
    use crate::game::intersect::worldsphtree::gather_callee::{
        GATHER_ALL_RECURSIVE, GATHER_ALL_SUBTREE, GATHER_OBJECTS,
    };
    use gather_all_node as node;
    c.g[V1][0] = c.g[V0][0];
    let class = c.g[V1][0];
    c.g[V0][0] = 0x2;
    if class != 0 {
        // Partly overlapping: recurse; disjoint: skip.
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            return Ok(());
        }
        c.cycles += 7;
        c.g[T1][0] = match cell {
            GatherAllCellPtr::Reg(r) => c.g[r][0],
            GatherAllCellPtr::Slot(off) => c.lw(sp(c).wrapping_add(off)),
        };
        c.g[A0][0] = c.g[tag][0];
        c.g[T0][0] = c.g[root][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S5][0];
        GATHER_ALL_RECURSIVE.call(c, q.recursive_ret)?;
        c.cycles += 1;
        return Ok(());
    }

    // The box lies within the cell: gather the root's nodes...
    c.cycles += q.objects_cycles;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[root][0];
    GATHER_OBJECTS.call(c, q.objects_ret)?;
    let tree = r32(c, root);
    let ctx = r32(c, S0);
    let objs = ctx.wrapping_add(0x10);
    c.g[A2][0] = c.lw(tree.wrapping_add(node::OBJECTS));
    if !first {
        c.g[A3][0] = sx(objs);
    }
    c.g[T0][0] = sx(ctx.wrapping_add(0x20));

    // ...its objects whose box touches the query box...
    if c.g[A2][0] != 0 {
        c.cycles += q.scan_cycles;
        c.g[V0][0] = c.lw(objs.wrapping_add(0xc));
        loop {
            let obj = r32(c, A2);
            let bx = r32(c, V0);
            gather_all_object_overlaps(c, obj, bx);
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                // objects.list[(*objects.count)++] = obj
                c.cycles += 9;
                c.g[A1][0] = c.lw(objs.wrapping_add(4));
                c.g[A0][0] = c.lw(objs);
                c.g[V1][0] = c.lw(r32(c, A1));
                c.g[V0][0] = sx(r32(c, V1) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                c.sw(r32(c, V0), c.g[A2][0]);
                c.sw(r32(c, A1), c.g[V1][0]);
            }
            c.g[A2][0] = c.lw(obj);
            c.cycles += 2;
            if c.g[A2][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lw(objs.wrapping_add(0xc));
            if event_due(c) {
                deliver_event_here(c, q.objs_loop);
            }
        }
    }

    // ...its whole all-list...
    c.cycles += 3;
    c.g[A2][0] = c.lw(tree.wrapping_add(node::ALL));
    if c.g[A2][0] != 0 {
        let all = r32(c, T0);
        c.cycles += 2;
        c.g[A1][0] = c.lw(all.wrapping_add(4));
        loop {
            // all.list[(*all.count)++] = item
            c.cycles += 10;
            c.g[A0][0] = c.lw(all);
            c.g[V1][0] = c.lw(r32(c, A1));
            c.g[V0][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            c.sw(r32(c, V0), c.g[A2][0]);
            c.sw(r32(c, A1), c.g[V1][0]);
            c.g[A2][0] = c.lw(r32(c, A2));
            if c.g[A2][0] == 0 {
                c.cycles += 1;
                break;
            }
            c.g[A1][0] = c.lw(all.wrapping_add(4));
            if event_due(c) {
                deliver_event_here(c, q.all_loop);
            }
        }
    }
    c.g[A0][0] = c.lw(tree);

    // ...and its eight children whole.
    for k in 0..8u32 {
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += if k == 7 { 4 } else { 3 };
            c.g[A1][0] = c.g[S0][0];
            GATHER_ALL_SUBTREE.call(c, q.subtree_ret[k as usize])?;
        }
        if k < 7 {
            c.g[A0][0] = c.lw(tree.wrapping_add(4 * (k + 1)));
        }
    }
    Ok(())
}

/// The inlined box test of an object (`obj`) against the query box (`bx`,
/// min at +0, max at +0x10): `c.le.s` on each bound pair, leaving v1 = 1 on
/// overlap (0 otherwise) and the last pair in f0/f1.
#[inline(always)]
fn gather_all_object_overlaps(c: &mut Cpu, obj: u32, bx: u32) {
    use gather_all_node as node;
    c.cycles += 6;
    c.lwc1(1, obj.wrapping_add(node::OBJ_MAX));
    c.lwc1(0, bx);
    c.c_le_s(0, 1);
    c.g[V1][0] = 0;
    if !c.fcc() {
        return;
    }
    c.cycles += 5;
    c.lwc1(1, bx.wrapping_add(0x10));
    c.lwc1(0, obj.wrapping_add(node::OBJ_MIN));
    c.c_le_s(0, 1);
    if !c.fcc() {
        return;
    }
    c.cycles += 5;
    c.lwc1(1, bx.wrapping_add(0x4));
    c.lwc1(0, obj.wrapping_add(node::OBJ_MAX + 4));
    c.c_le_s(1, 0);
    if !c.fcc() {
        return;
    }
    c.cycles += 5;
    c.lwc1(1, bx.wrapping_add(0x14));
    c.lwc1(0, obj.wrapping_add(node::OBJ_MIN + 4));
    c.c_le_s(0, 1);
    if !c.fcc() {
        return;
    }
    c.cycles += 5;
    c.lwc1(1, bx.wrapping_add(0x8));
    c.lwc1(0, obj.wrapping_add(node::OBJ_MAX + 8));
    c.c_le_s(1, 0);
    if !c.fcc() {
        return;
    }
    c.cycles += 6;
    c.lwc1(1, bx.wrapping_add(0x18));
    c.lwc1(0, obj.wrapping_add(node::OBJ_MIN + 8));
    c.c_le_s(0, 1);
    if c.fcc() {
        c.g[V1][0] = 1;
    }
}

// ---------------------------------------------------------------------------
// cOVStateHUD1P_Update (0x001ea930).

/// What `cOVStateHUD1P_Update` calls (all through the function table, as
/// the original's cross-shard calls).
mod hud1p_update {
    use crate::call::Callee;
    use ssx3_ee as ee;

    /// `RiderMetrix_IsHUDMessageActive(metrix, id)`.
    pub const RIDER_METRIX_IS_HUD_MESSAGE_ACTIVE: u32 = 0x0011_7400;
    /// `Font_MeasureText(f12, f13, font, text, out, flags)`.
    pub const FONT_MEASURE_TEXT: u32 = 0x0039_1fb0;
    /// `Font_FindLineBreak(f12, f13 = width, font, text, line, max)`.
    pub const FONT_FIND_LINE_BREAK: u32 = 0x0039_2430;
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const STRCPY: u32 = 0x0041_66f4;
    pub const SPRINTF: u32 = 0x0041_7828;
    pub const GAME_MODE_MAN_HANDLER_V09: u32 = 0x0023_8590;
    pub const BE_NEW_PLAYER_INT_CHAR_PROFILE_CALL_523E8: u32 = 0x0014_5e68;
    pub const FIXUNSSFSI: u32 = 0x0041_39f8;
    pub const WSCRIPT_MAN_IS_MISSION_TYPE2_ACTIVE: u32 = 0x0030_b8c0;
    pub const MUSIC_PLAYER_GET_SONG_TITLE: u32 = 0x002b_40f0;
    pub const MUSIC_PLAYER_GET_SONG_ARTIST: u32 = 0x002b_4120;
    pub const MUSIC_PLAYER_GET_SONG_ALBUM: u32 = 0x002b_4150;
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;
    pub const SCRIPT_ENGINE_IS_HUD_ALLOWED: u32 = 0x0027_92e0;
    pub const AI_ALL_PLAYERS_FINISHED: u32 = 0x0012_a250;
    /// `cOVStateHUD1P_UpdateRaceProgress()`.
    pub const UPDATE_RACE_PROGRESS: u32 = 0x0020_ed20;
    /// `HUD_GetElemScale(def, out)` (same shard: a direct call).
    pub const HUD_GET_ELEM_SCALE: Callee = Callee {
        addr: 0x001e_9290,
        f: ee::s04::f_001e9290,
    };
    /// `HUD_SetIconText(icon, str, font, pos)` (same shard: a direct call).
    pub const HUD_SET_ICON_TEXT: Callee = Callee {
        addr: 0x001e_94e0,
        f: ee::s04::f_001e94e0,
    };
}

/// `void cOVStateHUD1P_Update(cOVStateHUD1P* self)` (0x001ea930): the 1P
/// HUD's per-frame update. It rebuilds the element mask `self+0x3cc` from
/// the game mode, the BE settings, the race and the script engine; steps the
/// HUD's timers and fades (blink phase, music ticker, trick-score readout,
/// element and message fades, the race/mission clock, meter pulses); then,
/// for each HUD player (`self+0x48`, 0x8c bytes each), turns the rider's
/// active HUD messages (`RiderMetrix_IsHUDMessageActive`) and metric records
/// into flags and fade timers, and finally publishes bit 6 of the mask
/// (`gp-0x994`) and updates the race progress bar.
///
/// The body keeps the original's block structure (every block's cycle
/// charge, the scratch registers each block leaves, the `.lit4` steps read
/// through `gp`), with the calls through the call protocol and the loop
/// back edges' event checks through [`event_due`]/[`deliver_event_here`].
pub fn c_ov_state_hud1p_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_1ea9f0: {
            // Per-mode element mask: self+0x3cc keeps its 0x70000 bits and takes the
            // mask of BE mode byte 0x535c12 (table 0x478078). sp+0x2d0 = the game mode
            // (world+0xc0), sp+0x2d4 = BE mode 0x535c10, sp+0x2d8 = 'score-attack'
            // (game mode 2 and BE mode 1..3).
            c.cycles += 42;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x3b0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x370));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x380));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x360));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x350));
            c.g[S2][0] = 0x2;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x340));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x330));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x320));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x310));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x300));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x2f0), c.g[RA][0]);
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x3a0));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x398));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x390));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2d8), 0u64);
            call_ptr(c, hud1p_update::BE_LIBRARY_GET_INSTANCE, 0x001ea978)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, hud1p_update::BE_LIBRARY_GET_INTERFACE, 0x001ea984)?;
            c.g[V1][0] = 0x530000;
            c.g[V0][0] = 0x480000;
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f88));
            c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4a));
            c.g[A1][0] = 0x70000;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = c.g[A0][0] & c.g[A1][0];
            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
            c.g[V1][0] = c.g[V0][0] | c.g[A0][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V1][0]);
            c.g[A2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc0));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2d0), c.g[A2][0]);
            c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x48));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2d4), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[A2][0] as u32));
            let t = c.g[V0][0] != c.g[S2][0];
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
            if t {
                break 'b_1ea9f0;
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d4));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x2d8), c.g[V0][0]);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
                break 'b_1ea9f0;
            }
        }
        'b_1eaa0c: {
            // Game mode +0x8c: the mask also gets 0x22 (minus 0x100).
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = c.g[V1][0] | 0x22;
            if t {
                break 'b_1eaa0c;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[V0][0] = c.g[V0][0] | 0xfeff;
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V1][0]);
                break 'b_1eaa0c;
            }
        }
        'b_1eaa94: {
            // Two or more racers (race+0x7c + race+0x80): position display (bit 0,
            // clear 0x30), and 0x8000000 unless BE byte 0x535c11 is 2 with 0x534b30
            // clear, or the BE mode is 0 or 5.
            c.cycles += 9;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x80));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x7c));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d4));
            if t {
                break 'b_1eaa94;
            } else {
                'b_1eaa74: {
                    c.cycles += 13;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                    c.g[V1][0] = 0xffffffffffff0000;
                    c.g[V1][0] = c.g[V1][0] | 0xffcf;
                    c.g[V0][0] = c.g[V0][0] | 0x1;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V0][0]);
                    call_ptr(c, hud1p_update::BE_LIBRARY_GET_INSTANCE, 0x001eaa4c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x7;
                    call_ptr(c, hud1p_update::BE_LIBRARY_GET_INTERFACE, 0x001eaa58)?;
                    c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x49));
                    let t = c.g[V1][0] != c.g[S2][0];
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d4));
                    if t {
                        break 'b_1eaa74;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = 0x530000;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4b30));
                        let t = c.g[V1][0] == 0u64;
                        c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d4));
                        if t {
                            break 'b_1eaa94;
                        } else {
                            break 'b_1eaa74;
                        }
                    }
                }
                'b_1eaa90: {
                    c.cycles += 2;
                    let t = c.g[A2][0] == 0u64;
                    c.g[V0][0] = 0x5;
                    if t {
                        break 'b_1eaa90;
                    } else {
                        c.cycles += 2;
                        let t = c.g[A2][0] == c.g[V0][0];
                        c.g[V1][0] = 0x8000000;
                        if t {
                            break 'b_1eaa90;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V0][0]);
                            break 'b_1eaa90;
                        }
                    }
                }
                c.cycles += 1;
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d4));
                break 'b_1eaa94;
            }
        }
        'b_1eaaec: {
            // Clock kind (sp+0x2dc) outside BE mode 4: race type 6 clears 0x30 (no
            // clock); otherwise 1 (count-down), and type 5 keeps the flags while other
            // types clear bit 0.
            c.cycles += 3;
            c.g[V0][0] = 0x4;
            let t = c.g[A3][0] == c.g[V0][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2dc), 0u64);
            if t {
                break 'b_1eaaec;
            } else {
                'b_1eaae4: {
                    'b_1eaad8: {
                        c.cycles += 7;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = 0x6;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        c.g[V1][0] = c.lw((c.g[V1][0] as u32));
                        let t = c.g[V1][0] == c.g[A1][0];
                        c.g[T0][0] = 0x1;
                        if t {
                            break 'b_1eaad8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x5;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x2dc), c.g[T0][0]);
                            if t {
                                break 'b_1eaaec;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                c.g[V0][0] = 0xffffffffffff0000;
                                let t = true;
                                c.g[V0][0] = c.g[V0][0] | 0xfffe;
                                if t {
                                    break 'b_1eaae4;
                                } else {
                                    break 'b_1eaad8;
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffcf;
                    break 'b_1eaae4;
                }
                c.cycles += 2;
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V1][0]);
                break 'b_1eaaec;
            }
        }
        'b_1eab88: {
            'b_1eab84: {
                'b_1eab80: {
                    'b_1eab78: {
                        // The HUD blink phase self+0xc94: advances by a .lit4 step and wraps to
                        // [0, 1] while the HUD is allowed (gp-0x6a0 bit 3 clear, script engine not
                        // in state 2, ScriptEngine_IsHUDAllowed), else -1.
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                        c.g[V0][0] = c.g[V0][0] & 0x8;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        if t {
                            break 'b_1eab78;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                            c.g[V0][0] = c.g[V0][0] ^ 0x2;
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_1eab78;
                            } else {
                                c.cycles += 4;
                                call_ptr(
                                    c,
                                    hud1p_update::SCRIPT_ENGINE_IS_HUD_ALLOWED,
                                    0x001eab14,
                                )?;
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    break 'b_1eab78;
                                } else {
                                    c.cycles += 6;
                                    c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0xc94));
                                    c.mtc1(0, 0);
                                    c.c_le_s(0, 2);
                                    let t = !c.fcc();
                                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc94));
                                    if t {
                                        break 'b_1eab80;
                                    } else {
                                        c.cycles += 8;
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x576c));
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 1);
                                        c.add_s(0, 2, 0);
                                        c.c_lt_s(1, 0);
                                        let t = !c.fcc();
                                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc94));
                                        if t {
                                            break 'b_1eab84;
                                        } else {
                                            c.cycles += 1;
                                            'l_1eab58: loop {
                                                c.cycles += 6;
                                                c.lwc1(0, (c.g[V0][0] as u32));
                                                c.sub_s(0, 0, 1);
                                                c.c_lt_s(1, 0);
                                                let t = c.fcc();
                                                c.swc1(0, (c.g[V0][0] as u32));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x001eab58);
                                                    }
                                                    continue 'l_1eab58;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x6a0)
                                                        );
                                                    if t {
                                                        break 'b_1eab88;
                                                    } else {
                                                        break 'b_1eab78;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 0);
                    break 'b_1eab80;
                }
                c.cycles += 1;
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc94));
                break 'b_1eab84;
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
            break 'b_1eab88;
        }
        'b_1ec1b8: {
            // gp-0x6a0 bit 0: the HUD is suspended, straight to the epilogue.
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let t = c.g[V0][0] != 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x380));
            if t {
                break 'b_1ec1b8;
            } else {
                'b_1eb164: {
                    'b_1eb160: {
                        // Music ticker (0x10000): the fade timer self+0x190 runs through fade-in,
                        // hold and fade-out (alphas at +0x194/+0x198); a negative timer lays out
                        // the 'now playing' lines (title wrapped, artist, album) at self+0x19c
                        // (0x68 bytes each) with their widths and heights.
                        c.cycles += 7;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                        c.g[V0][0] = 0x10000;
                        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x48));
                        c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                        let t = c.g[V0][0] == 0u64;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2e0), c.g[V1][0]);
                        if t {
                            break 'b_1eb160;
                        } else {
                            'b_1eb02c: {
                                c.cycles += 6;
                                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x190));
                                c.mtc1(0, 20);
                                c.c_lt_s(1, 20);
                                let t = !c.fcc();
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_1eb02c;
                                } else {
                                    'b_1ead24: {
                                        c.cycles += 87;
                                        c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x190));
                                        c.g[A0][0] = 0x470000;
                                        c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x194));
                                        c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x198));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7120));
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x3a4), 0u64);
                                        c.mov_s(21, 20);
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x3a8), 0u64);
                                        c.g[S0][0] = 0x68;
                                        c.swc1(20, (c.g[SP][0] as u32));
                                        c.mov_s(22, 21);
                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        hud1p_update::HUD_GET_ELEM_SCALE.call(c, 0x001eabfc)?;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x42c));
                                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x30));
                                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x34));
                                        c.mul_s(0, 0, 1);
                                        c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5768));
                                        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
                                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                                        c.mul_s(2, 2, 1);
                                        c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x3c));
                                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                                        c.mul_s(3, 3, 4);
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                                        c.mul_s(1, 1, 4);
                                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x30));
                                        c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x34));
                                        c.mul_s(0, 0, 3);
                                        c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x38));
                                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                                        c.mul_s(2, 2, 1);
                                        c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x3c));
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x42c));
                                        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                                        c.ldr(4, (c.g[SP][0] as u32));
                                        c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x2f));
                                        c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x28));
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                                        c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x7));
                                        c.ldr(8, (c.g[SP][0] as u32));
                                        c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x2f));
                                        c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x28));
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
                                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                                        c.cvt_s_w(0, 0);
                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x34));
                                        c.mul_s(0, 0, 1);
                                        c.mul_s(0, 0, 2);
                                        c.mul_s(20, 0, 4);
                                        call_ptr(
                                            c,
                                            hud1p_update::SSX_AUDIO_SYSTEM_GET_INSTANCE,
                                            0x001eaca8,
                                        )?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                        c.g[S2][0] = c.g[V0][0];
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xe74));
                                        c.mult(0, 4, 4, 16);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19c));
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, hud1p_update::STRCPY, 0x001eacc4)?;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                        c.g[A2][0] = 0u64;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x42c));
                                        c.g[A3][0] = 0u64;
                                        c.mult(0, 2, 5, 16);
                                        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                                        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                                        c.g[A1][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[S1][0] as u32)));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x19c));
                                        call_ptr(c, hud1p_update::FONT_MEASURE_TEXT, 0x001eacec)?;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1fc));
                                        c.g[S6][0] = c.g[A0][0];
                                        c.mult(0, 2, 2, 16);
                                        c.g[V0][0] =
                                            sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.swc1(0, (c.g[V0][0] as u32));
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                        c.mult(0, 3, 3, 16);
                                        c.g[A0][0] =
                                            sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.lwc1(0, (c.g[A0][0] as u32));
                                        c.c_lt_s(22, 0);
                                        if c.fcc() {
                                            c.mov_s(22, 0);
                                            break 'b_1ead24;
                                        } else {
                                            break 'b_1ead24;
                                        }
                                    }
                                    c.cycles += 17;
                                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x200));
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5764));
                                    c.g[S5][0] = c.g[V0][0];
                                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.mov_s(21, 0);
                                    c.swc1(0, (c.g[V0][0] as u32));
                                    c.g[FP][0] = c.g[A0][0];
                                    c.g[A1][0] = 0xffffffffffffffff;
                                    c.g[S4][0] = c.g[S5][0];
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                    c.g[S2][0] = 0x68;
                                    c.g[S3][0] = c.g[S6][0];
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x3a4), c.g[V0][0]);
                                    call_ptr(
                                        c,
                                        hud1p_update::MUSIC_PLAYER_GET_SONG_TITLE,
                                        0x001ead64,
                                    )?;
                                    c.g[S0][0] = c.g[V0][0];
                                    'l_1ead68: loop {
                                        'b_1eadf0: {
                                            c.cycles += 34;
                                            c.g[A2][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                                            c.g[A3][0] = 0x60;
                                            c.mult(0, 2, 6, 18);
                                            c.g[AT][0] = 0x43960000;
                                            c.mtc1(1, 13);
                                            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                                            c.g[A2][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[S1][0] as u32)));
                                            c.g[A2][0] =
                                                sx((c.g[A2][0] as u32).wrapping_add(0x19c));
                                            call_ptr(
                                                c,
                                                hud1p_update::FONT_FIND_LINE_BREAK,
                                                0x001ead94,
                                            )?;
                                            c.g[A1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                                            c.g[A2][0] = 0u64;
                                            c.mult(0, 2, 5, 18);
                                            c.g[A3][0] = 0u64;
                                            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                                            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                                            c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[S1][0] as u32)));
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x19c));
                                            call_ptr(
                                                c,
                                                hud1p_update::FONT_MEASURE_TEXT,
                                                0x001eadc0,
                                            )?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                            c.mult(0, 3, 2, 18);
                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[S3][0] as u32)));
                                            c.swc1(0, (c.g[V0][0] as u32));
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                            c.mult(0, 3, 3, 18);
                                            c.g[V0][0] = sx((c.g[S3][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            c.lwc1(0, (c.g[V0][0] as u32));
                                            c.c_lt_s(22, 0);
                                            if c.fcc() {
                                                c.mov_s(22, 0);
                                                break 'b_1eadf0;
                                            } else {
                                                break 'b_1eadf0;
                                            }
                                        }
                                        c.cycles += 10;
                                        c.g[V0][0] =
                                            sx((c.g[S4][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.swc1(20, (c.g[V0][0] as u32));
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                        c.mult(0, 3, 4, 18);
                                        c.g[V0][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[S4][0] as u32)));
                                        c.lwc1(0, (c.g[V0][0] as u32));
                                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x3a4), c.g[V1][0]);
                                        let t = c.g[S0][0] != 0u64;
                                        c.add_s(21, 21, 0);
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001ead68);
                                            }
                                            continue 'l_1ead68;
                                        } else {
                                            'b_1eaebc: {
                                                c.cycles += 41;
                                                c.g[S2][0] = 0x68;
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0x3a8),
                                                    c.g[A0][0],
                                                );
                                                c.mult(0, 3, 4, 18);
                                                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5760));
                                                c.g[A0][0] = c.g[FP][0];
                                                c.g[A1][0] = 0xffffffffffffffff;
                                                c.add_s(21, 21, 1);
                                                c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[S5][0] as u32)));
                                                c.lwc1(0, (c.g[V0][0] as u32));
                                                c.add_s(0, 0, 1);
                                                c.swc1(0, (c.g[V0][0] as u32));
                                                c.g[S0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                                c.mult(0, 16, 16, 18);
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x19c));
                                                c.g[S0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[S0][0] as u32)));
                                                call_ptr(
                                                    c,
                                                    hud1p_update::MUSIC_PLAYER_GET_SONG_ARTIST,
                                                    0x001eae58,
                                                )?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, hud1p_update::STRCPY, 0x001eae64)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                                c.g[A2][0] = 0u64;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                                                c.g[A3][0] = 0u64;
                                                c.mult(0, 2, 5, 18);
                                                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                                                c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                                                c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[S1][0] as u32)));
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x19c));
                                                call_ptr(
                                                    c,
                                                    hud1p_update::FONT_MEASURE_TEXT,
                                                    0x001eae8c,
                                                )?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                                c.mult(0, 3, 2, 18);
                                                c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[S6][0] as u32)));
                                                c.swc1(0, (c.g[V0][0] as u32));
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                                c.mult(0, 3, 3, 18);
                                                c.g[V0][0] = sx((c.g[S6][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.lwc1(0, (c.g[V0][0] as u32));
                                                c.c_lt_s(22, 0);
                                                if c.fcc() {
                                                    c.mov_s(22, 0);
                                                    break 'b_1eaebc;
                                                } else {
                                                    break 'b_1eaebc;
                                                }
                                            }
                                            'b_1eaf5c: {
                                                c.cycles += 40;
                                                c.g[V0][0] = sx((c.g[S5][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.g[A0][0] = c.g[FP][0];
                                                c.swc1(20, (c.g[V0][0] as u32));
                                                c.g[A1][0] = 0xffffffffffffffff;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                                c.mult(0, 6, 2, 18);
                                                c.g[V1][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                c.mult(0, 16, 3, 18);
                                                c.g[V0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add((c.g[S5][0] as u32)));
                                                c.lwc1(0, (c.g[V0][0] as u32));
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x19c));
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0x3a4),
                                                    c.g[V1][0],
                                                );
                                                c.g[S0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[S0][0] as u32)));
                                                c.add_s(21, 21, 0);
                                                call_ptr(
                                                    c,
                                                    hud1p_update::MUSIC_PLAYER_GET_SONG_ALBUM,
                                                    0x001eaef8,
                                                )?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, hud1p_update::STRCPY, 0x001eaf04)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                                c.g[A2][0] = 0u64;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                                                c.g[A3][0] = 0u64;
                                                c.mult(0, 2, 5, 18);
                                                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                                                c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                                                c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[S1][0] as u32)));
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x19c));
                                                call_ptr(
                                                    c,
                                                    hud1p_update::FONT_MEASURE_TEXT,
                                                    0x001eaf2c,
                                                )?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                                c.mult(0, 3, 2, 18);
                                                c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[S6][0] as u32)));
                                                c.swc1(0, (c.g[V0][0] as u32));
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                                c.mult(0, 3, 3, 18);
                                                c.g[V0][0] = sx((c.g[S6][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.lwc1(0, (c.g[V0][0] as u32));
                                                c.c_lt_s(22, 0);
                                                if c.fcc() {
                                                    c.mov_s(22, 0);
                                                    break 'b_1eaf5c;
                                                } else {
                                                    break 'b_1eaf5c;
                                                }
                                            }
                                            c.cycles += 52;
                                            c.g[V0][0] = sx((c.g[S5][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            c.g[AT][0] = 0x3f800000;
                                            c.mtc1(1, 4);
                                            c.swc1(20, (c.g[V0][0] as u32));
                                            c.g[AT][0] = 0x40000000;
                                            c.mtc1(1, 2);
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x3a4));
                                            c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x34));
                                            c.mult(0, 4, 2, 18);
                                            c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x24));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            c.g[AT][0] = 0x42200000;
                                            c.mtc1(1, 1);
                                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x575c));
                                            c.g[V1][0] = sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[S5][0] as u32)));
                                            c.add_s(1, 22, 1);
                                            c.lwc1(0, (c.g[V1][0] as u32));
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0x3a4),
                                                c.g[V0][0],
                                            );
                                            c.add_s(21, 21, 0);
                                            c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x3ac));
                                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            c.add_s(3, 21, 3);
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x42c));
                                            c.swc1(3, (c.g[S1][0] as u32).wrapping_add(0x3b0));
                                            c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x30));
                                            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x34));
                                            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x38));
                                            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                                            c.mul_s(0, 0, 1);
                                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x3c));
                                            c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x34));
                                            c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                                            c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x30));
                                            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x34));
                                            c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x38));
                                            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                                            c.mul_s(0, 0, 1);
                                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x3c));
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x42c));
                                            c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x27));
                                            c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x2f));
                                            c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x28));
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                                            c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x27));
                                            c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x2f));
                                            c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x28));
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                            if t {
                                                break 'b_1eb164;
                                            } else {
                                                break 'b_1eb02c;
                                            }
                                        }
                                    }
                                }
                            }
                            'b_1eb060: {
                                c.cycles += 7;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5758));
                                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5754));
                                c.add_s(1, 1, 0);
                                c.c_lt_s(1, 2);
                                let t = !c.fcc();
                                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x190));
                                if t {
                                    break 'b_1eb060;
                                } else {
                                    c.cycles += 6;
                                    c.g[AT][0] = 0x40200000;
                                    c.mtc1(1, 0);
                                    c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x198));
                                    c.mul_s(0, 1, 0);
                                    let t = true;
                                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x194));
                                    if t {
                                        break 'b_1eb160;
                                    } else {
                                        break 'b_1eb060;
                                    }
                                }
                            }
                            'b_1eb094: {
                                c.cycles += 4;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5750));
                                c.c_lt_s(1, 0);
                                let t = !c.fcc();

                                if t {
                                    break 'b_1eb094;
                                } else {
                                    c.cycles += 9;
                                    c.sub_s(0, 1, 2);
                                    c.g[AT][0] = 0x40200000;
                                    c.mtc1(1, 1);
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 2);
                                    c.mul_s(0, 0, 1);
                                    c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x194));
                                    let t = true;
                                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x198));
                                    if t {
                                        break 'b_1eb160;
                                    } else {
                                        break 'b_1eb094;
                                    }
                                }
                            }
                            'b_1eb0b8: {
                                c.cycles += 4;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x574c));
                                c.c_lt_s(1, 0);
                                let t = !c.fcc();

                                if t {
                                    break 'b_1eb0b8;
                                } else {
                                    c.cycles += 5;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 0);
                                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x198));
                                    let t = true;
                                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x194));
                                    if t {
                                        break 'b_1eb160;
                                    } else {
                                        break 'b_1eb0b8;
                                    }
                                }
                            }
                            'b_1eb0f0: {
                                c.cycles += 4;
                                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5748));
                                c.c_lt_s(1, 2);
                                let t = !c.fcc();

                                if t {
                                    break 'b_1eb0f0;
                                } else {
                                    c.cycles += 10;
                                    c.sub_s(0, 1, 0);
                                    c.g[AT][0] = 0x40200000;
                                    c.mtc1(1, 2);
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 1);
                                    c.mul_s(0, 0, 2);
                                    c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x194));
                                    c.sub_s(1, 1, 0);
                                    let t = true;
                                    c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x198));
                                    if t {
                                        break 'b_1eb160;
                                    } else {
                                        break 'b_1eb0f0;
                                    }
                                }
                            }
                            'b_1eb12c: {
                                c.cycles += 5;
                                c.g[AT][0] = 0x41000000;
                                c.mtc1(1, 0);
                                c.c_lt_s(1, 0);
                                let t = !c.fcc();

                                if t {
                                    break 'b_1eb12c;
                                } else {
                                    c.cycles += 10;
                                    c.sub_s(0, 1, 2);
                                    c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x198));
                                    c.g[AT][0] = 0x40200000;
                                    c.mtc1(1, 2);
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 1);
                                    c.mul_s(0, 0, 2);
                                    c.sub_s(1, 1, 0);
                                    let t = true;
                                    c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x194));
                                    if t {
                                        break 'b_1eb160;
                                    } else {
                                        break 'b_1eb12c;
                                    }
                                }
                            }
                            c.cycles += 13;
                            c.g[V0][0] = 0xfffffffffffe0000;
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 0);
                            c.g[V0][0] = c.g[V0][0] | 0xffff;
                            c.g[V1][0] = 0xfffffffff0000000;
                            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x3b4), c.g[V1][0]);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V0][0]);
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x190));
                            c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x198));
                            c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x194));
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x3a4), 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x3a8), 0u64);
                            break 'b_1eb160;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                    break 'b_1eb164;
                }
                'b_1eb328: {
                    // Trick-score readout (0x10): the next of the four score thresholds
                    // (tables 0x536708/0x536640) as text at self+0x3dc (0x18 each) with their
                    // right-aligned offsets.
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & 0x10;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_1eb328;
                    } else {
                        'b_1eb1bc: {
                            c.cycles += 8;
                            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                            c.g[FP][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x18));
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x790));
                            let t = c.g[A3][0] == 0u64;
                            c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x198));
                            if t {
                                break 'b_1eb1bc;
                            } else {
                                c.cycles += 11;
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x6c0));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001eb1a8)?;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A2][0] = 0u64;
                                call_ptr(c, hud1p_update::GAME_MODE_MAN_HANDLER_V09, 0x001eb1b8)?;
                                c.g[S3][0] =
                                    sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                break 'b_1eb1bc;
                            }
                        }
                        c.cycles += 23;
                        c.g[T0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3f0));
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6bec));
                        c.g[A1][0] = c.g[SP][0];
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2e4), c.g[T0][0]);
                        c.g[S5][0] = 0x1000000;
                        hud1p_update::HUD_GET_ELEM_SCALE.call(c, 0x001eb1d8)?;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3dc));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x42c));
                        c.g[A0][0] = 0x530000;
                        c.lwc1(0, (c.g[SP][0] as u32));
                        c.g[S6][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6708));
                        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x30));
                        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3ec));
                        c.g[S4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2e4));
                        c.mul_s(1, 1, 0);
                        c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x34));
                        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x38));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.mul_s(2, 2, 0);
                        c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x3c));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x424), 0u64);
                        'l_1eb218: loop {
                            'b_1eb298: {
                                'b_1eb27c: {
                                    c.cycles += 9;
                                    c.g[V0][0] = c.lb((c.g[S6][0] as u32));
                                    c.g[V1][0] = 0x530000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[A2][0] as i64)) as u64;
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    if t {
                                        break 'b_1eb27c;
                                    } else {
                                        c.cycles += 16;
                                        c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x18));
                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                        call_ptr(c, t, 0x001eb258)?;
                                        c.g[A3][0] = 0x4a0000;
                                        c.g[A2][0] = c.g[S3][0];
                                        c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2138));
                                        c.g[S3][0] = 0xffffffffffffffff;
                                        call_ptr(c, hud1p_update::SPRINTF, 0x001eb274)?;
                                        let t = true;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x42c));
                                        if t {
                                            break 'b_1eb298;
                                        } else {
                                            break 'b_1eb27c;
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.g[T0][0] = 0x4a0000;
                                c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x2138));
                                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                                call_ptr(c, hud1p_update::SPRINTF, 0x001eb294)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x42c));
                                break 'b_1eb298;
                            }
                            'b_1eb2cc: {
                                c.cycles += 13;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                                c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                                call_ptr(c, hud1p_update::FONT_MEASURE_TEXT, 0x001eb2b0)?;
                                c.mov_s(1, 0);
                                c.swc1(1, (c.g[S4][0] as u32));
                                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x424));
                                c.c_lt_s(0, 1);
                                if c.fcc() {
                                    c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x424));
                                    break 'b_1eb2cc;
                                } else {
                                    break 'b_1eb2cc;
                                }
                            }
                            c.cycles += 9;
                            c.g[V0][0] = c.g[S5][0];
                            c.g[V1][0] = 0x1000000;
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x18));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                            let t = c.g[V0][0] != 0u64;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001eb218);
                                }
                                continue 'l_1eb218;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2e4));
                                c.g[A0][0] = 0x1000000;
                                c.g[A1][0] = 0x1000000;
                                'l_1eb300: loop {
                                    c.cycles += 10;
                                    c.lwc1(1, (c.g[V1][0] as u32));
                                    c.g[V0][0] = c.g[A0][0];
                                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x424));
                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    c.sub_s(0, 0, 1);
                                    c.swc1(0, (c.g[V1][0] as u32));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x18));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001eb300);
                                        }
                                        continue 'l_1eb300;
                                    } else {
                                        break 'b_1eb328;
                                    }
                                }
                            }
                        }
                    }
                }
                'b_1eb3e8: {
                    // Mission timers (WScriptMan gp+0xce8, +0x2a0 != -1): flags 0x400000
                    // and up from the active mission's fields; a +0x1c timer makes the clock
                    // kind 2.
                    c.cycles += 6;
                    c.g[S2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x2a0));
                    if c.g[S0][0] == c.g[V0][0] {
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c4));
                        break 'b_1eb3e8;
                    } else {
                        'b_1eb3e4: {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(
                                c,
                                hud1p_update::WSCRIPT_MAN_IS_MISSION_TYPE2_ACTIVE,
                                0x001eb348,
                            )?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = 0x400000;
                            if t {
                                break 'b_1eb3e4;
                            } else {
                                'b_1eb388: {
                                    c.cycles += 10;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                    c.g[V0][0] = 0xffffffffffff0000;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x18c), c.g[S0][0]);
                                    c.g[V0][0] = c.g[V0][0] | 0xff7f;
                                    c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                                    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V1][0]);
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V1][0] = c.g[V1][0] | 0x400;
                                    if t {
                                        break 'b_1eb388;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = 0xffffffffffff0000;
                                        c.g[V0][0] = c.g[V0][0] | 0xfcfd;
                                        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V1][0]);
                                        break 'b_1eb388;
                                    }
                                }
                                'b_1eb3a4: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28));
                                        break 'b_1eb3a4;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                        c.g[V0][0] = c.g[V0][0] | 0x800;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V0][0]);
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28));
                                        break 'b_1eb3a4;
                                    }
                                }
                                'b_1eb3bc: {
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                        break 'b_1eb3bc;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                        c.g[V0][0] = c.g[V0][0] | 0x1000;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V0][0]);
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                        break 'b_1eb3bc;
                                    }
                                }
                                'b_1eb3d4: {
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                                        break 'b_1eb3d4;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                        c.g[V0][0] = c.g[V0][0] | 0x6000;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V0][0]);
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                                        break 'b_1eb3d4;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = 0x2;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2dc));
                                if c.g[V1][0] != 0 {
                                    c.g[A0][0] = c.g[V0][0];
                                }
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x2dc), c.g[A0][0]);
                                break 'b_1eb3e4;
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c4));
                        break 'b_1eb3e8;
                    }
                }
                'b_1eb458: {
                    // The element fade self+0x3c0 for element kind self+0x3c4 (0x50 = none).
                    c.cycles += 3;
                    c.g[V0][0] = 0x50;
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                        break 'b_1eb458;
                    } else {
                        'b_1eb440: {
                            'b_1eb420: {
                                c.cycles += 7;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 2);
                                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x3c0));
                                c.c_le_s(2, 1);
                                let t = !c.fcc();
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_1eb420;
                                } else {
                                    c.cycles += 4;
                                    c.g[AT][0] = 0xffffffffbf800000;
                                    c.mtc1(1, 0);
                                    let t = true;
                                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c0));
                                    if t {
                                        break 'b_1eb440;
                                    } else {
                                        break 'b_1eb420;
                                    }
                                }
                            }
                            'b_1eb43c: {
                                c.cycles += 6;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5744));
                                c.add_s(0, 1, 0);
                                c.c_lt_s(2, 0);
                                let t = !c.fcc();
                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3c0));
                                if t {
                                    break 'b_1eb43c;
                                } else {
                                    c.cycles += 1;
                                    c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x3c0));
                                    break 'b_1eb43c;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0x1;
                            break 'b_1eb440;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                            break 'b_1eb458;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = 0x50;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x3c0), 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x3c4), c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                            break 'b_1eb458;
                        }
                    }
                }
                'b_1eb4d8: {
                    // The wide-message fade self+0xa44 (0x20000); done clears the message.
                    c.cycles += 4;
                    c.g[V1][0] = 0x20000;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2dc));
                    if t {
                        break 'b_1eb4d8;
                    } else {
                        'b_1eb4b4: {
                            'b_1eb494: {
                                c.cycles += 7;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 2);
                                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0xa44));
                                c.c_le_s(2, 1);
                                let t = !c.fcc();
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_1eb494;
                                } else {
                                    c.cycles += 4;
                                    c.g[AT][0] = 0xffffffffbf800000;
                                    c.mtc1(1, 0);
                                    let t = true;
                                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xa44));
                                    if t {
                                        break 'b_1eb4b4;
                                    } else {
                                        break 'b_1eb494;
                                    }
                                }
                            }
                            'b_1eb4b0: {
                                c.cycles += 6;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5740));
                                c.add_s(0, 1, 0);
                                c.c_lt_s(2, 0);
                                let t = !c.fcc();
                                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xa44));
                                if t {
                                    break 'b_1eb4b0;
                                } else {
                                    c.cycles += 1;
                                    c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0xa44));
                                    break 'b_1eb4b0;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0x1;
                            break 'b_1eb4b4;
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2dc));
                        if t {
                            break 'b_1eb4d8;
                        } else {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                            c.g[V0][0] = 0xfffffffffffd0000;
                            c.g[V0][0] = c.g[V0][0] | 0xffff;
                            c.sh((c.g[S1][0] as u32).wrapping_add(0x944), 0u64);
                            c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V1][0]);
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2dc));
                            break 'b_1eb4d8;
                        }
                    }
                }
                'b_1eb6e8: {
                    // The clock: remaining time from the race (kind 1) or the mission
                    // (kind 2), split into h/m/s/hundredths at self+0x17c..0x188, and the
                    // last-ten-seconds pulse self+0x3d0.
                    'b_1eb6e4: {
                        c.cycles += 2;
                        let t = c.g[A1][0] == 0u64;
                        c.g[S5][0] = 0u64;
                        if t {
                            break 'b_1eb6e4;
                        } else {
                            'b_1eb598: {
                                'b_1eb584: {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                    c.g[V1][0] = 0x1;
                                    c.g[V0][0] = c.g[A0][0] | 0x4;
                                    let t = c.g[A1][0] != c.g[V1][0];
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V0][0]);
                                    if t {
                                        break 'b_1eb584;
                                    } else {
                                        'b_1eb52c: {
                                            c.cycles += 8;
                                            c.g[A1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[S2][0] = c.g[S5][0];
                                            c.g[A2][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
                                            c.g[V1][0] =
                                                c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x88));
                                            c.g[A0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[S0][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                            if t {
                                                break 'b_1eb52c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[V1][0] = 0x530000;
                                                c.g[S2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x2dc));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                                                c.g[V0][0] =
                                                    sx(((c.g[V0][0] as u32) as i32 >> 9) as u32);
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                if c.g[V0][0] != 0 {
                                                    c.g[S2][0] = 0u64;
                                                }
                                                break 'b_1eb52c;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[S2][0] == 0u64;

                                        if t {
                                            break 'b_1eb598;
                                        } else {
                                            'b_1eb568: {
                                                'b_1eb554: {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[A1][0] as u32).wrapping_add(0xc0));
                                                    c.g[V0][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                                    let t = (c.g[V0][0] as i64) < 0;
                                                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                                                    if t {
                                                        break 'b_1eb554;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.mtc1(2, 0);
                                                        c.cvt_s_w(0, 0);
                                                        let t = true;

                                                        if t {
                                                            break 'b_1eb568;
                                                        } else {
                                                            break 'b_1eb554;
                                                        }
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                                c.mtc1(2, 0);
                                                c.cvt_s_w(0, 0);
                                                c.add_s(0, 0, 0);
                                                break 'b_1eb568;
                                            }
                                            c.cycles += 7;
                                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x573c));
                                            c.mul_s(12, 0, 12);
                                            call_ptr(c, hud1p_update::FIXUNSSFSI, 0x001eb574)?;
                                            c.g[V1][0] = 0x3c;
                                            c.mult(0, 2, 2, 3);
                                            let t = true;
                                            c.g[S0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_sub((c.g[S0][0] as u32)));
                                            if t {
                                                break 'b_1eb598;
                                            } else {
                                                break 'b_1eb584;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                                c.g[V0][0] = c.g[A0][0] | 0xc;
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V0][0]);
                                c.g[S2][0] = sx((c.g[V1][0] as u32) >> 31);
                                break 'b_1eb598;
                            }
                            'b_1eb654: {
                                'b_1eb5b4: {
                                    c.cycles += 2;
                                    if (c.g[S0][0] as i64) > 0 {
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                        break 'b_1eb5b4;
                                    } else {
                                        c.cycles += 5;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x17c), 0u64);
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x180), 0u64);
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x184), 0u64);
                                        let t = true;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x188), 0u64);
                                        if t {
                                            break 'b_1eb654;
                                        } else {
                                            break 'b_1eb5b4;
                                        }
                                    }
                                }
                                'b_1eb618: {
                                    'b_1eb5f8: {
                                        c.cycles += 9;
                                        c.mtc1(16, 1);
                                        c.cvt_s_w(1, 1);
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5738));
                                        c.mul_s(2, 1, 0);
                                        c.g[V0][0] = c.g[V0][0] & 0x8;
                                        c.cvt_w_s(0, 2);
                                        c.mfc1(4, 0);
                                        let t = c.g[V0][0] != 0u64;

                                        if t {
                                            break 'b_1eb5f8;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[S2][0] == 0u64;
                                            c.sw((c.g[S1][0] as u32).wrapping_add(0x188), 0u64);
                                            if t {
                                                break 'b_1eb618;
                                            } else {
                                                c.cycles += 6;
                                                c.g[V0][0] = 0x3c;
                                                c.mult(0, 2, 4, 2);
                                                c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_sub((c.g[V0][0] as u32)));
                                                c.g[V0][0] =
                                                    ((0u64 as i64) < (c.g[V0][0] as i64)) as u64;
                                                let t = true;
                                                c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                if t {
                                                    break 'b_1eb618;
                                                } else {
                                                    break 'b_1eb5f8;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 8;
                                    c.mtc1(4, 0);
                                    c.cvt_s_w(0, 0);
                                    c.g[AT][0] = 0x42c80000;
                                    c.mtc1(1, 1);
                                    c.sub_s(0, 2, 0);
                                    c.mul_s(0, 0, 1);
                                    c.cvt_w_s(1, 0);
                                    c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x188));
                                    break 'b_1eb618;
                                }
                                'b_1eb62c: {
                                    c.cycles += 5;
                                    c.g[V0][0] = 0xe10;
                                    c.g[V1][0] = 0x3c;
                                    c.div(0, 4, 2);
                                    if c.g[V0][0] == 0u64 {
                                        break 'b_1eb62c;
                                    } else {
                                        break 'b_1eb62c;
                                    }
                                }
                                c.cycles += 10;
                                c.g[A1][0] = 0xe10;
                                c.g[A3][0] = 0x3c;
                                c.g[A2][0] = c.lo[0];
                                c.g[A0][0] = c.hi[0];
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x17c), c.g[A2][0]);
                                c.div(0, 4, 3);
                                c.g[V0][0] = c.lo[0];
                                c.g[T0][0] = c.hi[0];
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x180), c.g[V0][0]);
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x184), c.g[T0][0]);
                                break 'b_1eb654;
                            }
                            'b_1eb6e0: {
                                'b_1eb6d8: {
                                    c.cycles += 2;
                                    let t = c.g[S2][0] == 0u64;
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < 601i64) as u64;
                                    if t {
                                        break 'b_1eb6d8;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_1eb6d8;
                                        } else {
                                            'b_1eb6c8: {
                                                c.cycles += 2;
                                                let t = (c.g[S0][0] as i64) <= 0;

                                                if t {
                                                    break 'b_1eb6c8;
                                                } else {
                                                    c.cycles += 6;
                                                    c.lwc1(
                                                        2,
                                                        (c.g[S1][0] as u32).wrapping_add(0x3d0),
                                                    );
                                                    c.mtc1(0, 0);
                                                    c.c_lt_s(2, 0);
                                                    let t = c.fcc();
                                                    c.g[V0][0] =
                                                        sx((c.g[S1][0] as u32).wrapping_add(0x3d0));
                                                    if t {
                                                        break 'b_1eb6e0;
                                                    } else {
                                                        c.cycles += 8;
                                                        c.lwc1(
                                                            0,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x5734),
                                                        );
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 1);
                                                        c.add_s(0, 2, 0);
                                                        c.c_lt_s(1, 0);
                                                        let t = !c.fcc();
                                                        c.swc1(
                                                            0,
                                                            (c.g[S1][0] as u32).wrapping_add(0x3d0),
                                                        );
                                                        if t {
                                                            break 'b_1eb6e4;
                                                        } else {
                                                            c.cycles += 1;
                                                            'l_1eb6a8: loop {
                                                                c.cycles += 6;
                                                                c.lwc1(0, (c.g[V0][0] as u32));
                                                                c.sub_s(0, 0, 1);
                                                                c.c_lt_s(1, 0);
                                                                let t = c.fcc();
                                                                c.swc1(0, (c.g[V0][0] as u32));
                                                                if t {
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x001eb6a8,
                                                                        );
                                                                    }
                                                                    continue 'l_1eb6a8;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x160));
                                                                    if t {
                                                                        break 'b_1eb6e8;
                                                                    } else {
                                                                        break 'b_1eb6c8;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[AT][0] = 0x3f000000;
                                            c.mtc1(1, 0);
                                            let t = true;
                                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3d0));
                                            if t {
                                                break 'b_1eb6e4;
                                            } else {
                                                break 'b_1eb6d8;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[AT][0] = 0xffffffffbf800000;
                                c.mtc1(1, 0);
                                break 'b_1eb6e0;
                            }
                            c.cycles += 1;
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x3d0));
                            break 'b_1eb6e4;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x160));
                    break 'b_1eb6e8;
                }
                'b_1eb7cc: {
                    // The self+0x160 meter pulse (+0x164 phase, +0x168 timer, flag 0x80000).
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_1eb7cc;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3d4));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if t {
                            break 'b_1eb7cc;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3d8));
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            if t {
                                break 'b_1eb7cc;
                            } else {
                                'b_1eb754: {
                                    c.cycles += 13;
                                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x164));
                                    c.g[V1][0] = 0x80000;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5730));
                                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x164));
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 2);
                                    c.add_s(1, 1, 0);
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.c_lt_s(2, 1);
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V0][0]);
                                    let t = !c.fcc();
                                    c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x164));
                                    if t {
                                        break 'b_1eb754;
                                    } else {
                                        c.cycles += 1;
                                        c.sub_s(1, 1, 2);
                                        'l_1eb740: loop {
                                            c.cycles += 4;
                                            c.c_lt_s(2, 1);
                                            if c.fcc() {
                                                c.sub_s(1, 1, 2);
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x001eb740);
                                                }
                                                continue 'l_1eb740;
                                            } else {
                                                c.cycles += 1;
                                                c.swc1(1, (c.g[A0][0] as u32));
                                                break 'b_1eb754;
                                            }
                                        }
                                    }
                                }
                                'b_1eb7a0: {
                                    'b_1eb780: {
                                        c.cycles += 7;
                                        c.g[AT][0] = 0x40a00000;
                                        c.mtc1(1, 2);
                                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x168));
                                        c.c_le_s(2, 1);
                                        let t = !c.fcc();
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_1eb780;
                                        } else {
                                            c.cycles += 4;
                                            c.g[AT][0] = 0xffffffffbf800000;
                                            c.mtc1(1, 0);
                                            let t = true;
                                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x168));
                                            if t {
                                                break 'b_1eb7a0;
                                            } else {
                                                break 'b_1eb780;
                                            }
                                        }
                                    }
                                    'b_1eb79c: {
                                        c.cycles += 6;
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x572c));
                                        c.add_s(0, 1, 0);
                                        c.c_lt_s(2, 0);
                                        let t = !c.fcc();
                                        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x168));
                                        if t {
                                            break 'b_1eb79c;
                                        } else {
                                            c.cycles += 1;
                                            c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x168));
                                            break 'b_1eb79c;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = 0x1;
                                    break 'b_1eb7a0;
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_1eb7cc;
                                } else {
                                    c.cycles += 9;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                    c.g[V0][0] = 0xfffffffffff70000;
                                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x160), 0u64);
                                    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x164), 0u64);
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V1][0]);
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x168), 0u64);
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    break 'b_1eb7cc;
                                }
                            }
                        }
                    }
                }
                'b_1eb7f8: {
                    // More than one viewport (race manager +0x84 -> +0x10 > 1): clear 0x1000000.
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_1eb7f8;
                    } else {
                        c.cycles += 5;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                        c.g[V0][0] = 0xfffffffffeff0000;
                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V1][0]);
                        break 'b_1eb7f8;
                    }
                }
                'b_1eb8bc: {
                    // The BE interfaces, the settings block 0x535610 (0x288 bytes) copied to
                    // sp+0x40, and the 'race over' mask (first word's top bits 01).
                    'b_1eb890: {
                        c.cycles += 17;
                        c.g[FP][0] = 0u64;
                        call_ptr(c, hud1p_update::BE_LIBRARY_GET_INSTANCE, 0x001eb800)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0x1;
                        call_ptr(c, hud1p_update::BE_LIBRARY_GET_INTERFACE, 0x001eb80c)?;
                        c.g[S6][0] = c.g[V0][0];
                        call_ptr(c, hud1p_update::BE_LIBRARY_GET_INSTANCE, 0x001eb814)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0x4;
                        call_ptr(c, hud1p_update::BE_LIBRARY_GET_INTERFACE, 0x001eb820)?;
                        c.g[V0][0] = 0x530000;
                        c.g[V1][0] = c.g[S7][0];
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5610));
                        c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
                        c.g[V0][0] = c.g[V0][0] & 0x7;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x280));
                        if t {
                            break 'b_1eb890;
                        } else {
                            'l_1eb83c: loop {
                                c.cycles += 19;
                                c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                                c.ldr(5, (c.g[A0][0] as u32));
                                c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                                c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                                c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                                c.ldr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                                c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                                c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                                c.sdr(5, (c.g[V1][0] as u32));
                                c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                                c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                                c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                                c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                let t = c.g[A0][0] != c.g[V0][0];
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x001eb83c);
                                    }
                                    continue 'l_1eb83c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_1eb8bc;
                                    } else {
                                        break 'b_1eb890;
                                    }
                                }
                            }
                        }
                    }
                    'l_1eb890: loop {
                        c.cycles += 11;
                        c.g[A1][0] = c.ld((c.g[A0][0] as u32));
                        c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                        c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                        c.sd((c.g[V1][0] as u32), c.g[A1][0]);
                        c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                        c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                        c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                        let t = c.g[A0][0] != c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001eb890);
                            }
                            continue 'l_1eb890;
                        } else {
                            break 'b_1eb8bc;
                        }
                    }
                }
                'b_1eb918: {
                    c.cycles += 10;
                    c.ldl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.ldr(2, (c.g[A0][0] as u32));
                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.sdr(2, (c.g[V1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = 0xffffffffc0000000;
                    c.g[A0][0] = 0x40000000;
                    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_1eb918;
                    } else {
                        'b_1eb914: {
                            c.cycles += 8;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                            c.g[V0][0] = 0xfffffffff8ef0000;
                            c.g[V0][0] = c.g[V0][0] | 0x7fbf;
                            c.g[FP][0] = 0x1;
                            c.g[A0][0] = c.g[V1][0] & c.g[V0][0];
                            c.g[V1][0] = c.g[V1][0] & 0x2000;
                            let t = c.g[V1][0] != 0u64;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[A0][0]);
                            if t {
                                break 'b_1eb914;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = 0xffffffffffff0000;
                                c.g[V0][0] = c.g[V0][0] | 0xbfff;
                                c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V0][0]);
                                break 'b_1eb914;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        break 'b_1eb918;
                    }
                }
                'b_1eb938: {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    call_ptr(c, hud1p_update::AI_ALL_PLAYERS_FINISHED, 0x001eb924)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0x170000;
                    if t {
                        break 'b_1eb938;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V0][0]);
                        break 'b_1eb938;
                    }
                }
                'b_1ec17c: {
                    // Per HUD player (self+0x48, 0x8c each, at most two): the rider's
                    // message flags, boost/trick meters and the element fades.
                    c.cycles += 8;
                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2e0));
                    c.g[S4][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc8), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xcc), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc4), 0u64);
                    let t = (c.g[V0][0] as i64) < 0;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xd0), 0u64);
                    if t {
                        break 'b_1ec17c;
                    } else {
                        c.cycles += 4;
                        c.mtc1(0, 21);
                        c.g[AT][0] = 0xffffffffbf800000;
                        c.mtc1(1, 20);
                        'l_1eb968: loop {
                            'b_1eb9b8: {
                                c.cycles += 9;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d8));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                let t = c.g[T0][0] == 0u64;
                                c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                if t {
                                    break 'b_1eb9b8;
                                } else {
                                    c.cycles += 11;
                                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6c0));
                                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6c0));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001eb9a4)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A2][0] = 0u64;
                                    call_ptr(
                                        c,
                                        hud1p_update::GAME_MODE_MAN_HANDLER_V09,
                                        0x001eb9b4,
                                    )?;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                                    break 'b_1eb9b8;
                                }
                            }
                            'b_1ec150: {
                                'b_1ec14c: {
                                    'b_1eba10: {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d4));
                                        c.g[V0][0] = 0x4;
                                        if c.g[V1][0] == c.g[V0][0] {
                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                            break 'b_1eba10;
                                        } else {
                                            'b_1eb9e0: {
                                                c.cycles += 5;
                                                c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x470));
                                                c.c_le_s(21, 0);
                                                let t = c.fcc();
                                                c.g[V0][0] = 0x1;
                                                if t {
                                                    break 'b_1eb9e0;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = 0u64;
                                                    break 'b_1eb9e0;
                                                }
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                                break 'b_1eba10;
                                            } else {
                                                'b_1eb9f8: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x480)
                                                        );
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V0][0] = 0x2;
                                                    if t {
                                                        break 'b_1eb9f8;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = 0x1;
                                                        break 'b_1eb9f8;
                                                    }
                                                }
                                                c.cycles += 6;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x88),
                                                    c.g[V0][0],
                                                );
                                                c.g[V0][0] = 0xffffffffffef0000;
                                                c.g[S4][0] =
                                                    sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                                c.g[V0][0] = c.g[V0][0] | 0xffff;
                                                let t = true;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x80),
                                                    c.g[V0][0],
                                                );
                                                if t {
                                                    break 'b_1ec14c;
                                                } else {
                                                    break 'b_1eba10;
                                                }
                                            }
                                        }
                                    }
                                    'b_1eba50: {
                                        'b_1eba40: {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A2][0] = 0x1;
                                            call_ptr(c, hud1p_update::BE_NEW_PLAYER_INT_CHAR_PROFILE_CALL_523E8, 0x001eba1c)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                                                break 'b_1eba40;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                                c.g[A0][0] = c.g[S6][0];
                                                c.g[A2][0] = 0x2;
                                                call_ptr(c, hud1p_update::BE_NEW_PLAYER_INT_CHAR_PROFILE_CALL_523E8, 0x001eba34)?;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x3cc)
                                                        );
                                                    break 'b_1eba50;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x80));
                                                    break 'b_1eba40;
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[V1][0] = 0x1000000;
                                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x80), c.g[V0][0]);
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                        break 'b_1eba50;
                                    }
                                    'b_1ebb00: {
                                        c.cycles += 4;
                                        c.g[V0][0] = 0x100000;
                                        c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x790));
                                        if t {
                                            break 'b_1ebb00;
                                        } else {
                                            'b_1ebab8: {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.g[V1][0] & 0x4;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = c.g[S2][0];
                                                if t {
                                                    break 'b_1ebab8;
                                                } else {
                                                    'b_1ebaa8: {
                                                        c.cycles += 4;
                                                        c.g[A1][0] = 0x29;
                                                        call_ptr(c, hud1p_update::RIDER_METRIX_IS_HUD_MESSAGE_ACTIVE, 0x001eba74)?;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x80));
                                                            break 'b_1ebaa8;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[A0][0] = c.g[S2][0];
                                                            c.g[A1][0] = 0x2a;
                                                            call_ptr(c, hud1p_update::RIDER_METRIX_IS_HUD_MESSAGE_ACTIVE, 0x001eba88)?;
                                                            if c.g[V0][0] != 0u64 {
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x80));
                                                                break 'b_1ebaa8;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[A0][0] = c.g[S2][0];
                                                                c.g[A1][0] = 0x2b;
                                                                call_ptr(c, hud1p_update::RIDER_METRIX_IS_HUD_MESSAGE_ACTIVE, 0x001eba9c)?;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[A0][0] = c.g[S2][0];
                                                                if t {
                                                                    break 'b_1ebab8;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x80));
                                                                    break 'b_1ebaa8;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[V1][0] = 0x100000;
                                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x80),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_1ebb00;
                                                    } else {
                                                        break 'b_1ebab8;
                                                    }
                                                }
                                            }
                                            'b_1ebafc: {
                                                c.cycles += 4;
                                                c.g[A1][0] = 0x33;
                                                call_ptr(c, hud1p_update::RIDER_METRIX_IS_HUD_MESSAGE_ACTIVE, 0x001ebac0)?;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] = 0x4;
                                                if t {
                                                    break 'b_1ebafc;
                                                } else {
                                                    'b_1ebae8: {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x3cc));
                                                        c.g[V0][0] = c.g[V0][0] & 0x4000;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        if t {
                                                            break 'b_1ebae8;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[A1][0] = 0x32;
                                                            call_ptr(c, hud1p_update::RIDER_METRIX_IS_HUD_MESSAGE_ACTIVE, 0x001ebae0)?;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = 0x5;
                                                            if t {
                                                                break 'b_1ebafc;
                                                            } else {
                                                                break 'b_1ebae8;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    c.g[A1][0] = 0x2c;
                                                    call_ptr(c, hud1p_update::RIDER_METRIX_IS_HUD_MESSAGE_ACTIVE, 0x001ebaf4)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x6;
                                                    if t {
                                                        break 'b_1ebb00;
                                                    } else {
                                                        break 'b_1ebafc;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x88),
                                                c.g[V0][0],
                                            );
                                            break 'b_1ebb00;
                                        }
                                    }
                                    'b_1ebb50: {
                                        'b_1ebb4c: {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A1][0] = 0x31;
                                            call_ptr(
                                                c,
                                                hud1p_update::RIDER_METRIX_IS_HUD_MESSAGE_ACTIVE,
                                                0x001ebb0c,
                                            )?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] = 0x8400000;
                                            if t {
                                                break 'b_1ebb4c;
                                            } else {
                                                'b_1ebb44: {
                                                    c.cycles += 9;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x84));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x80));
                                                    c.g[V1][0] = c.g[V1][0] | 0x31;
                                                    c.g[V0][0] = c.g[V0][0] | 0x80;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x88));
                                                    c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x84),
                                                        c.g[V0][0],
                                                    );
                                                    let t = c.g[A1][0] == 0u64;
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x80),
                                                        c.g[A0][0],
                                                    );
                                                    if t {
                                                        break 'b_1ebb44;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            ((c.g[A1][0] as i64) < 4i64) as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        if t {
                                                            break 'b_1ebb50;
                                                        } else {
                                                            break 'b_1ebb44;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                c.g[V0][0] = 0x3;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x88),
                                                    c.g[V0][0],
                                                );
                                                break 'b_1ebb4c;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A0][0] = c.g[S2][0];
                                        break 'b_1ebb50;
                                    }
                                    'b_1ebb84: {
                                        'b_1ebb78: {
                                            c.cycles += 4;
                                            c.g[A1][0] = 0xb;
                                            call_ptr(
                                                c,
                                                hud1p_update::RIDER_METRIX_IS_HUD_MESSAGE_ACTIVE,
                                                0x001ebb58,
                                            )?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                                                break 'b_1ebb78;
                                            } else {
                                                c.cycles += 5;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[A1][0] = 0x21;
                                                call_ptr(c, hud1p_update::RIDER_METRIX_IS_HUD_MESSAGE_ACTIVE, 0x001ebb6c)?;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x3cc)
                                                        );
                                                    break 'b_1ebb84;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x80));
                                                    break 'b_1ebb78;
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = c.g[V0][0] | 0x4000;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x80), c.g[V0][0]);
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                        break 'b_1ebb84;
                                    }
                                    'b_1ebc98: {
                                        'b_1ebc94: {
                                            'b_1ebc90: {
                                                'b_1ebc7c: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                                    if c.g[V0][0] == 0u64 {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x44));
                                                        break 'b_1ebc7c;
                                                    } else {
                                                        'b_1ebc08: {
                                                            c.cycles += 4;
                                                            c.g[V0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0xec));
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x44));
                                                            let t = (c.g[V1][0] as i64) < 0;
                                                            c.g[A0][0] =
                                                                (c.g[V0][0] < 0x1u64) as u64;
                                                            if t {
                                                                break 'b_1ebc08;
                                                            } else {
                                                                'b_1ebbfc: {
                                                                    c.cycles += 4;
                                                                    c.lwc1(
                                                                        0,
                                                                        (c.g[S0][0] as u32)
                                                                            .wrapping_add(0x48),
                                                                    );
                                                                    c.c_le_s(21, 0);
                                                                    let t = !c.fcc();

                                                                    if t {
                                                                        break 'b_1ebbfc;
                                                                    } else {
                                                                        'b_1ebbc0: {
                                                                            c.cycles += 3;
                                                                            c.lwc1(
                                                                                2,
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0x5728,
                                                                                    ),
                                                                            );
                                                                            let t =
                                                                                c.g[A0][0] == 0u64;

                                                                            if t {
                                                                                break 'b_1ebbc0;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5724));
                                                                                break 'b_1ebbc0;
                                                                            }
                                                                        }
                                                                        'b_1ebbf4: {
                                                                            'b_1ebbe0: {
                                                                                c.cycles += 6;
                                                                                c.g[AT][0] =
                                                                                    0x3f800000;
                                                                                c.mtc1(1, 1);
                                                                                c.c_le_s(1, 0);
                                                                                if !c.fcc() {
                                                                                    c.add_s(
                                                                                        0, 0, 2,
                                                                                    );
                                                                                    break 'b_1ebbe0;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    let t = true;
                                                                                    c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x48));
                                                                                    if t {
                                                                                        break 'b_1ebbf4;
                                                                                    } else {
                                                                                        break 'b_1ebbe0;
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 4;
                                                                            c.c_lt_s(1, 0);
                                                                            let t = !c.fcc();
                                                                            c.swc1(
                                                                                0,
                                                                                (c.g[S0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x48,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_1ebbf4;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x48));
                                                                                break 'b_1ebbf4;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        c.g[V1][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x44));
                                                                        c.g[V0][0] = c.lw((c.g[S3]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xec));
                                                                        break 'b_1ebbfc;
                                                                    }
                                                                }
                                                                'b_1ebc04: {
                                                                    c.cycles += 2;
                                                                    if c.g[V1][0] != c.g[V0][0] {
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x48),
                                                                            0u64,
                                                                        );
                                                                        break 'b_1ebc04;
                                                                    } else {
                                                                        break 'b_1ebc04;
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0xec));
                                                                break 'b_1ebc08;
                                                            }
                                                        }
                                                        'b_1ebc74: {
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x44),
                                                                c.g[V0][0],
                                                            );
                                                            if t {
                                                                break 'b_1ebc74;
                                                            } else {
                                                                'b_1ebc68: {
                                                                    c.cycles += 5;
                                                                    c.lwc1(
                                                                        1,
                                                                        (c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4c),
                                                                    );
                                                                    c.c_le_s(21, 1);
                                                                    let t = !c.fcc();
                                                                    c.g[V0][0] = sx((c.g[S0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x4c));
                                                                    if t {
                                                                        break 'b_1ebc68;
                                                                    } else {
                                                                        c.cycles += 9;
                                                                        c.lwc1(
                                                                            0,
                                                                            (c.g[GP][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x5720,
                                                                                ),
                                                                        );
                                                                        c.g[S4][0] =
                                                                            sx((c.g[S4][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.g[AT][0] = 0x40000000;
                                                                        c.mtc1(1, 2);
                                                                        c.add_s(0, 1, 0);
                                                                        c.c_lt_s(2, 0);
                                                                        let t = !c.fcc();
                                                                        c.swc1(
                                                                            0,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x4c),
                                                                        );
                                                                        if t {
                                                                            break 'b_1ebc94;
                                                                        } else {
                                                                            'l_1ebc48: loop {
                                                                                c.cycles += 6;
                                                                                c.lwc1(
                                                                                    0,
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.sub_s(0, 0, 2);
                                                                                c.c_lt_s(2, 0);
                                                                                let t = c.fcc();
                                                                                c.swc1(
                                                                                    0,
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                );
                                                                                if t {
                                                                                    if event_due(c)
                                                                                    {
                                                                                        deliver_event_here(c, 0x001ebc48);
                                                                                    }
                                                                                    continue 'l_1ebc48;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    let t = true;
                                                                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b0));
                                                                                    if t {
                                                                                        break 'b_1ebc98;
                                                                                    } else {
                                                                                        break 'b_1ebc68;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 3;
                                                                c.g[S4][0] = sx((c.g[S4][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                let t = true;
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x4c),
                                                                    0u64,
                                                                );
                                                                if t {
                                                                    break 'b_1ebc94;
                                                                } else {
                                                                    break 'b_1ebc74;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[S4][0] =
                                                            sx((c.g[S4][0] as u32)
                                                                .wrapping_add(0x1));
                                                        if t {
                                                            break 'b_1ebc90;
                                                        } else {
                                                            break 'b_1ebc7c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = (c.g[V0][0] as i64) < 0;
                                                c.g[S4][0] =
                                                    sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_1ebc94;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0xffffffffffffffff;
                                                    c.swc1(
                                                        20,
                                                        (c.g[S0][0] as u32).wrapping_add(0x48),
                                                    );
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x44),
                                                        c.g[V0][0],
                                                    );
                                                    break 'b_1ebc90;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x4c));
                                            break 'b_1ebc94;
                                        }
                                        c.cycles += 1;
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b0));
                                        break 'b_1ebc98;
                                    }
                                    'b_1ebebc: {
                                        'b_1ebeb8: {
                                            'b_1ebeb4: {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x57c));
                                                if c.g[V1][0] == 0 {
                                                    c.g[V0][0] = 0u64;
                                                }
                                                c.g[V1][0] = c.g[V0][0];
                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                c.g[V0][0] = c.g[V0][0] ^ 0x34;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = c.g[V1][0];
                                                if t {
                                                    break 'b_1ebeb4;
                                                } else {
                                                    'b_1ebd8c: {
                                                        'b_1ebcf0: {
                                                            c.cycles += 4;
                                                            c.g[V1][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc));
                                                            c.g[V0][0] =
                                                                ((c.g[V1][0] as i64) < 5i64) as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V0][0] = 0x4d0000;
                                                            if t {
                                                                break 'b_1ebcf0;
                                                            } else {
                                                                c.cycles += 11;
                                                                c.g[T0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x7bd8));
                                                                c.ldl(
                                                                    5,
                                                                    (c.g[T0][0] as u32)
                                                                        .wrapping_add(0x7),
                                                                );
                                                                c.ldr(5, (c.g[T0][0] as u32));
                                                                c.ldl(
                                                                    6,
                                                                    (c.g[T0][0] as u32)
                                                                        .wrapping_add(0xf),
                                                                );
                                                                c.ldr(
                                                                    6,
                                                                    (c.g[T0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                );
                                                                c.sdl(
                                                                    5,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x5b),
                                                                );
                                                                c.sdr(
                                                                    5,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x54),
                                                                );
                                                                c.sdl(
                                                                    6,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x63),
                                                                );
                                                                c.sdr(
                                                                    6,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x5c),
                                                                );
                                                                let t = true;
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                if t {
                                                                    break 'b_1ebd8c;
                                                                } else {
                                                                    break 'b_1ebcf0;
                                                                }
                                                            }
                                                        }
                                                        'b_1ebd28: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = ((c.g[V1][0] as i64)
                                                                < 10i64)
                                                                as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V0][0] = 0x4d0000;
                                                            if t {
                                                                break 'b_1ebd28;
                                                            } else {
                                                                c.cycles += 11;
                                                                c.g[A3][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x7bb8));
                                                                c.ldl(
                                                                    3,
                                                                    (c.g[A3][0] as u32)
                                                                        .wrapping_add(0x7),
                                                                );
                                                                c.ldr(3, (c.g[A3][0] as u32));
                                                                c.ldl(
                                                                    5,
                                                                    (c.g[A3][0] as u32)
                                                                        .wrapping_add(0xf),
                                                                );
                                                                c.ldr(
                                                                    5,
                                                                    (c.g[A3][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                );
                                                                c.sdl(
                                                                    3,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x5b),
                                                                );
                                                                c.sdr(
                                                                    3,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x54),
                                                                );
                                                                c.sdl(
                                                                    5,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x63),
                                                                );
                                                                c.sdr(
                                                                    5,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x5c),
                                                                );
                                                                let t = true;
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                if t {
                                                                    break 'b_1ebd8c;
                                                                } else {
                                                                    break 'b_1ebd28;
                                                                }
                                                            }
                                                        }
                                                        'b_1ebd64: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = 0xa;
                                                            let t = c.g[V1][0] != c.g[V0][0];
                                                            c.g[V0][0] = 0x4d0000;
                                                            if t {
                                                                break 'b_1ebd64;
                                                            } else {
                                                                c.cycles += 12;
                                                                c.g[V0][0] = 0x4d0000;
                                                                c.g[A2][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x7b98));
                                                                c.ldl(
                                                                    8,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x7),
                                                                );
                                                                c.ldr(8, (c.g[A2][0] as u32));
                                                                c.ldl(
                                                                    3,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0xf),
                                                                );
                                                                c.ldr(
                                                                    3,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                );
                                                                c.sdl(
                                                                    8,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x5b),
                                                                );
                                                                c.sdr(
                                                                    8,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x54),
                                                                );
                                                                c.sdl(
                                                                    3,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x63),
                                                                );
                                                                c.sdr(
                                                                    3,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x5c),
                                                                );
                                                                let t = true;
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                if t {
                                                                    break 'b_1ebd8c;
                                                                } else {
                                                                    break 'b_1ebd64;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 10;
                                                        c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_sub(0x7b78));
                                                        c.ldl(
                                                            7,
                                                            (c.g[A1][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.ldr(7, (c.g[A1][0] as u32));
                                                        c.ldl(
                                                            8,
                                                            (c.g[A1][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.ldr(
                                                            8,
                                                            (c.g[A1][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sdl(
                                                            7,
                                                            (c.g[S0][0] as u32).wrapping_add(0x5b),
                                                        );
                                                        c.sdr(
                                                            7,
                                                            (c.g[S0][0] as u32).wrapping_add(0x54),
                                                        );
                                                        c.sdl(
                                                            8,
                                                            (c.g[S0][0] as u32).wrapping_add(0x63),
                                                        );
                                                        c.sdr(
                                                            8,
                                                            (c.g[S0][0] as u32).wrapping_add(0x5c),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[A0][0] as u32).wrapping_add(0x4),
                                                        );
                                                        break 'b_1ebd8c;
                                                    }
                                                    'b_1ebdf4: {
                                                        c.cycles += 8;
                                                        c.lwc1(
                                                            0,
                                                            (c.g[A0][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 2);
                                                        c.div_s(0, 0, 1);
                                                        c.c_eq_s(0, 2);
                                                        let t = !c.fcc();
                                                        c.g[A0][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x560));
                                                        if t {
                                                            break 'b_1ebdf4;
                                                        } else {
                                                            c.cycles += 18;
                                                            c.swc1(
                                                                20,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x64),
                                                            );
                                                            c.g[A1][0] = sx((c.g[GP][0] as u32)
                                                                .wrapping_sub(0xf20));
                                                            c.g[A3][0] = sx((c.g[GP][0] as u32)
                                                                .wrapping_add(0x1e60));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x55c));
                                                            c.g[A2][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x428));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] =
                                                                sx((c.g[V1][0] as u32) >> 31);
                                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[V0][0] = sx(((c.g[V0][0] as u32)
                                                                as i32
                                                                >> 1)
                                                                as u32);
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 1);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub((c.g[V0][0] as u32)));
                                                            c.g[T0][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x55c),
                                                                c.g[V1][0],
                                                            );
                                                            c.g[T0][0] = sx((c.g[T0][0] as u32)
                                                                .wrapping_add((c.g[A1][0] as u32)));
                                                            c.g[A1][0] = c.lw((c.g[T0][0] as u32));
                                                            hud1p_update::HUD_SET_ICON_TEXT
                                                                .call(c, 0x001ebdec)?;
                                                            let t = true;
                                                            c.g[A0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x1b0));
                                                            if t {
                                                                break 'b_1ebebc;
                                                            } else {
                                                                break 'b_1ebdf4;
                                                            }
                                                        }
                                                    }
                                                    'b_1ebe64: {
                                                        'b_1ebe10: {
                                                            c.cycles += 5;
                                                            c.lwc1(
                                                                4,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x64),
                                                            );
                                                            c.c_lt_s(4, 21);
                                                            let t = !c.fcc();
                                                            c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x64));
                                                            if t {
                                                                break 'b_1ebe10;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.swc1(
                                                                    2,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x64),
                                                                );
                                                                if t {
                                                                    break 'b_1ebe64;
                                                                } else {
                                                                    break 'b_1ebe10;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 14;
                                                        c.sub_s(0, 2, 0);
                                                        c.lwc1(
                                                            3,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x571c),
                                                        );
                                                        c.lwc1(
                                                            2,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x5718),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x5714),
                                                        );
                                                        c.mul_s(0, 0, 2);
                                                        c.g[AT][0] = 0x40000000;
                                                        c.mtc1(1, 2);
                                                        c.add_s(0, 0, 3);
                                                        c.div_s(1, 1, 0);
                                                        c.add_s(0, 4, 1);
                                                        c.c_lt_s(2, 0);
                                                        let t = !c.fcc();
                                                        c.swc1(
                                                            0,
                                                            (c.g[S0][0] as u32).wrapping_add(0x64),
                                                        );
                                                        if t {
                                                            break 'b_1ebe64;
                                                        } else {
                                                            c.cycles += 2;
                                                            c.sub_s(0, 0, 2);
                                                            'l_1ebe50: loop {
                                                                c.cycles += 4;
                                                                c.c_lt_s(2, 0);
                                                                if c.fcc() {
                                                                    c.sub_s(0, 0, 2);
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x001ebe50,
                                                                        );
                                                                    }
                                                                    continue 'l_1ebe50;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.swc1(0, (c.g[V0][0] as u32));
                                                                    break 'b_1ebe64;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    c.g[A1][0] = 0u64;
                                                    call_ptr(c, hud1p_update::RIDER_METRIX_IS_HUD_MESSAGE_ACTIVE, 0x001ebe70)?;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x1b0));
                                                        break 'b_1ebebc;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        c.g[A1][0] = 0x21;
                                                        call_ptr(c, hud1p_update::RIDER_METRIX_IS_HUD_MESSAGE_ACTIVE, 0x001ebe84)?;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x1b0));
                                                            break 'b_1ebebc;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[A0][0] = c.g[S2][0];
                                                            c.g[A1][0] = 0xb;
                                                            call_ptr(c, hud1p_update::RIDER_METRIX_IS_HUD_MESSAGE_ACTIVE, 0x001ebe98)?;
                                                            if c.g[V0][0] != 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x1b0));
                                                                break 'b_1ebebc;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x84));
                                                                c.g[V1][0] = 0x2000000;
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] | c.g[V1][0];
                                                                let t = true;
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x84),
                                                                    c.g[V0][0],
                                                                );
                                                                if t {
                                                                    break 'b_1ebeb8;
                                                                } else {
                                                                    break 'b_1ebeb4;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x64));
                                            break 'b_1ebeb8;
                                        }
                                        c.cycles += 1;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b0));
                                        break 'b_1ebebc;
                                    }
                                    'b_1ebf3c: {
                                        'b_1ebf38: {
                                            c.cycles += 6;
                                            c.g[V1][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x618));
                                            if c.g[A0][0] == 0 {
                                                c.g[V1][0] = 0u64;
                                            }
                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                            c.g[V0][0] = c.g[V0][0] ^ 0x34;
                                            if c.g[V0][0] == 0u64 {
                                                c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x68));
                                                break 'b_1ebf38;
                                            } else {
                                                'b_1ebef8: {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[S0][0] as u32).wrapping_add(0x68),
                                                    );
                                                    c.c_lt_s(1, 21);
                                                    let t = !c.fcc();
                                                    c.g[V0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x68));
                                                    if t {
                                                        break 'b_1ebef8;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[AT][0] = 0x3f000000;
                                                        c.mtc1(1, 0);
                                                        let t = true;
                                                        c.swc1(
                                                            0,
                                                            (c.g[S0][0] as u32).wrapping_add(0x68),
                                                        );
                                                        if t {
                                                            break 'b_1ebf38;
                                                        } else {
                                                            break 'b_1ebef8;
                                                        }
                                                    }
                                                }
                                                c.cycles += 8;
                                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5710));
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 2);
                                                c.add_s(0, 1, 0);
                                                c.c_lt_s(2, 0);
                                                let t = !c.fcc();
                                                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x68));
                                                if t {
                                                    break 'b_1ebf38;
                                                } else {
                                                    'l_1ebf18: loop {
                                                        c.cycles += 6;
                                                        c.lwc1(0, (c.g[V0][0] as u32));
                                                        c.sub_s(0, 0, 2);
                                                        c.c_lt_s(2, 0);
                                                        let t = c.fcc();
                                                        c.swc1(0, (c.g[V0][0] as u32));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x001ebf18);
                                                            }
                                                            continue 'l_1ebf18;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x1b0));
                                                            if t {
                                                                break 'b_1ebf3c;
                                                            } else {
                                                                break 'b_1ebf38;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b0));
                                        break 'b_1ebf3c;
                                    }
                                    'b_1ebfcc: {
                                        'b_1ebfc8: {
                                            'b_1ebfbc: {
                                                c.cycles += 6;
                                                c.g[V1][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x9c));
                                                if c.g[A0][0] == 0 {
                                                    c.g[V1][0] = 0u64;
                                                }
                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                c.g[V0][0] = c.g[V0][0] ^ 0x34;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                                    break 'b_1ebfbc;
                                                } else {
                                                    'b_1ebfa0: {
                                                        c.cycles += 5;
                                                        c.lwc1(
                                                            0,
                                                            (c.g[S0][0] as u32).wrapping_add(0x70),
                                                        );
                                                        c.c_le_s(21, 0);
                                                        let t = !c.fcc();
                                                        c.g[V0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc));
                                                        if t {
                                                            break 'b_1ebfa0;
                                                        } else {
                                                            'b_1ebf8c: {
                                                                c.cycles += 7;
                                                                c.g[AT][0] = 0x3f800000;
                                                                c.mtc1(1, 1);
                                                                c.lwc1(
                                                                    2,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x570c),
                                                                );
                                                                c.c_le_s(1, 0);
                                                                if !c.fcc() {
                                                                    c.add_s(0, 0, 2);
                                                                    break 'b_1ebf8c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.swc1(
                                                                        20,
                                                                        (c.g[S0][0] as u32)
                                                                            .wrapping_add(0x70),
                                                                    );
                                                                    if t {
                                                                        break 'b_1ebfa0;
                                                                    } else {
                                                                        break 'b_1ebf8c;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.c_lt_s(1, 0);
                                                            let t = !c.fcc();
                                                            c.swc1(
                                                                0,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x70),
                                                            );
                                                            if t {
                                                                break 'b_1ebfa0;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.swc1(
                                                                    1,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x70),
                                                                );
                                                                break 'b_1ebfa0;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 3;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                                    if (c.g[V1][0] as i64) < 0 {
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x6c),
                                                            c.g[V0][0],
                                                        );
                                                        break 'b_1ebfcc;
                                                    } else {
                                                        c.cycles += 2;
                                                        if c.g[V1][0] != c.g[V0][0] {
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x70),
                                                                0u64,
                                                            );
                                                            break 'b_1ebfc8;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x6c),
                                                                c.g[V0][0],
                                                            );
                                                            if t {
                                                                break 'b_1ebfcc;
                                                            } else {
                                                                break 'b_1ebfbc;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = (c.g[V0][0] as i64) < 0;
                                            c.g[V0][0] = 0xffffffffffffffff;
                                            if t {
                                                break 'b_1ebfcc;
                                            } else {
                                                c.cycles += 1;
                                                c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x70));
                                                break 'b_1ebfc8;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), c.g[V0][0]);
                                        break 'b_1ebfcc;
                                    }
                                    'b_1ec07c: {
                                        'b_1ec078: {
                                            'b_1ec05c: {
                                                c.cycles += 4;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                                c.g[V0][0] = c.g[V0][0] & 0x20;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
                                                if t {
                                                    break 'b_1ec05c;
                                                } else {
                                                    'b_1ec02c: {
                                                        c.cycles += 6;
                                                        c.lwc1(
                                                            0,
                                                            (c.g[S0][0] as u32).wrapping_add(0x78),
                                                        );
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x790));
                                                        c.c_le_s(21, 0);
                                                        c.g[A1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x198));
                                                        let t = !c.fcc();
                                                        c.g[V1][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x7c));
                                                        if t {
                                                            break 'b_1ec02c;
                                                        } else {
                                                            c.cycles += 7;
                                                            c.g[AT][0] = 0x3f800000;
                                                            c.mtc1(1, 1);
                                                            c.lwc1(
                                                                2,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x5708),
                                                            );
                                                            c.c_le_s(1, 0);
                                                            if c.fcc() {
                                                                c.swc1(
                                                                    20,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x78),
                                                                );
                                                                break 'b_1ec07c;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.add_s(0, 0, 2);
                                                                c.c_lt_s(1, 0);
                                                                let t = !c.fcc();
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x78),
                                                                );
                                                                if t {
                                                                    break 'b_1ec07c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.swc1(
                                                                        1,
                                                                        (c.g[S0][0] as u32)
                                                                            .wrapping_add(0x78),
                                                                    );
                                                                    if t {
                                                                        break 'b_1ec07c;
                                                                    } else {
                                                                        break 'b_1ec02c;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_1ec04c: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x74));
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[V0][0] = ((c.g[A1][0] as i64)
                                                            < (c.g[V1][0] as i64))
                                                            as u64;
                                                        if t {
                                                            break 'b_1ec04c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = 0x1;
                                                            if t {
                                                                break 'b_1ec07c;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x78),
                                                                    0u64,
                                                                );
                                                                let t = true;
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x74),
                                                                    c.g[V0][0],
                                                                );
                                                                if t {
                                                                    break 'b_1ec07c;
                                                                } else {
                                                                    break 'b_1ec04c;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;

                                                    if t {
                                                        break 'b_1ec07c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x74),
                                                            0u64,
                                                        );
                                                        if t {
                                                            break 'b_1ec078;
                                                        } else {
                                                            break 'b_1ec05c;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A3][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
                                            if t {
                                                break 'b_1ec07c;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x8c));
                                                let t = c.g[V0][0] != 0u64;

                                                if t {
                                                    break 'b_1ec07c;
                                                } else {
                                                    c.cycles += 1;
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x74),
                                                        0u64,
                                                    );
                                                    break 'b_1ec078;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x78));
                                        break 'b_1ec07c;
                                    }
                                    'b_1ec130: {
                                        c.cycles += 2;
                                        let t = c.g[S5][0] != 0u64;

                                        if t {
                                            break 'b_1ec130;
                                        } else {
                                            'b_1ec0d8: {
                                                'b_1ec0cc: {
                                                    c.cycles += 7;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x1b0)
                                                        );
                                                    c.g[V1][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0xfd8));
                                                    if c.g[A0][0] == 0 {
                                                        c.g[V1][0] = 0u64;
                                                    }
                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                    c.g[V0][0] = c.g[V0][0] ^ 0x34;
                                                    let t = c.g[V0][0] == 0u64;

                                                    if t {
                                                        break 'b_1ec0cc;
                                                    } else {
                                                        c.cycles += 7;
                                                        c.lwc1(
                                                            1,
                                                            (c.g[V1][0] as u32).wrapping_add(0x4),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[V1][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.div_s(0, 0, 1);
                                                        c.c_eq_s(0, 21);
                                                        let t = !c.fcc();
                                                        c.g[V0][0] = 0x8;
                                                        if t {
                                                            break 'b_1ec0d8;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x3c0),
                                                                0u64,
                                                            );
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x3c4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[S5][0] = 0x1;
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x3c8),
                                                                0u64,
                                                            );
                                                            break 'b_1ec0cc;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[S5][0] != 0u64;

                                                if t {
                                                    break 'b_1ec130;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x1b0)
                                                        );
                                                    break 'b_1ec0d8;
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[V1][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x1074));
                                            if c.g[A0][0] == 0 {
                                                c.g[V1][0] = 0u64;
                                            }
                                            c.g[A0][0] = c.g[V1][0];
                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                            c.g[V0][0] = c.g[V0][0] ^ 0x34;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] = 0x400000;
                                            if t {
                                                break 'b_1ec130;
                                            } else {
                                                c.cycles += 10;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0x3cc),
                                                    c.g[V0][0],
                                                );
                                                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                                                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x4));
                                                c.div_s(0, 0, 1);
                                                c.c_eq_s(0, 21);
                                                let t = !c.fcc();
                                                c.g[V0][0] = 0x3a;
                                                if t {
                                                    break 'b_1ec130;
                                                } else {
                                                    c.cycles += 5;
                                                    c.sw(
                                                        (c.g[S1][0] as u32).wrapping_add(0x3c0),
                                                        0u64,
                                                    );
                                                    c.sw(
                                                        (c.g[S1][0] as u32).wrapping_add(0x3c4),
                                                        c.g[V0][0],
                                                    );
                                                    c.g[S5][0] = 0x1;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[A0][0] as u32).wrapping_add(0x14));
                                                    c.sw(
                                                        (c.g[S1][0] as u32).wrapping_add(0x3c8),
                                                        c.g[V0][0],
                                                    );
                                                    break 'b_1ec130;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[FP][0] == 0u64;
                                    c.g[V0][0] = ((c.g[S4][0] as i64) < 2i64) as u64;
                                    if t {
                                        break 'b_1ec150;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                                        c.g[V0][0] = 0xfffffffff8ef0000;
                                        c.g[V0][0] = c.g[V0][0] | 0x7fbf;
                                        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x84), c.g[V1][0]);
                                        break 'b_1ec14c;
                                    }
                                }
                                c.cycles += 1;
                                c.g[V0][0] = ((c.g[S4][0] as i64) < 2i64) as u64;
                                break 'b_1ec150;
                            }
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x8c;
                            if t {
                                break 'b_1ec17c;
                            } else {
                                c.cycles += 9;
                                c.mult(0, 2, 20, 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[S0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x80), 0u64);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x84), 0u64);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0u64);
                                let t = (c.g[V1][0] as i64) >= 0;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x88), 0u64);
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x001eb968);
                                    }
                                    continue 'l_1eb968;
                                } else {
                                    break 'b_1ec17c;
                                }
                            }
                        }
                    }
                }
                'b_1ec1a4: {
                    // Clear 0x100000 while an element fades; publish bit 6 to gp-0x994; the
                    // race progress bar.
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c4));
                    c.g[V0][0] = 0x50;
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                        break 'b_1ec1a4;
                    } else {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                        c.g[V0][0] = 0xffffffffffef0000;
                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x3cc), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3cc));
                        break 'b_1ec1a4;
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x994), c.g[V0][0]);
                call_ptr(c, hud1p_update::UPDATE_RACE_PROGRESS, 0x001ec1b4)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x380));
                break 'b_1ec1b8;
            }
        }
        c.cycles += 14;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x370));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x360));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x350));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x340));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x330));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x320));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x310));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x300));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x2f0));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x3a0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x398));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x390));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3b0));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldPainter_SampleSurfaceColours (0x002edb20).

/// World painter fields (`this` in s0) read by the colour sampler.
mod world_painter {
    /// Layer kinds, three bits per layer (layer i at bit 3i).
    pub const LAYER_KINDS: u32 = 0xc;
    /// Rectangle sampled by kind-5 layers: origin x, y, size x, y.
    pub const RECT: u32 = 0x10;
    /// Quad corners (x, y) sampled by kind-1 layers: P0, P1, P2, P3.
    pub const QUAD: u32 = 0x20;
    /// Byte: index into the surface table (8-byte entries).
    pub const SURFACE_INDEX: u32 = 0x155;
    /// Short: index into the material table (0x18-byte entries); negative
    /// means none.
    pub const MATERIAL_INDEX: u32 = 0x156;
    /// One short sampler id per layer.
    pub const LAYER_IDS: u32 = 0x1a0;
    /// Layer kind: sample the rectangle at the clamped (u, v).
    pub const KIND_RECT: u64 = 5;
    /// Layer kind: sample the quad at (u, v), wrapped to the texel fraction.
    pub const KIND_QUAD: u64 = 1;
    /// Surface table entry: kind word (+0x24) that must be 6.
    pub const SURFACE_KIND: u32 = 0x24;
    pub const SURFACE_KIND_PAINTED: u64 = 6;
    /// Material table entry (relative to the table base): kind word that
    /// must be 3.
    pub const MATERIAL_KIND: u32 = 0x3f0;
    pub const MATERIAL_KIND_PAINTED: u64 = 3;
    /// `gp+0x16c8`: pointer to the painter tables (first word: the table base).
    pub const GP_TABLES: u32 = 0x16c8;
    /// `gp-0x854`: the sampler object; its interface at +0x10d8, method slot
    /// +0x1d0 (`{delta, fn}`) samples a colour.
    pub const GP_SAMPLER: i32 = -0x854;
    pub const SAMPLER_IFACE: u32 = 0x10d8;
    pub const SAMPLE_SLOT: u32 = 0x1d0;
    /// Per-channel colour weights (`0x500000 - 0x5500`).
    pub const WEIGHTS_HI: u32 = 0x0050_0000;
    pub const WEIGHTS: u32 = WEIGHTS_HI - 0x5500;
    /// Offset added before splitting the quad sample into texel and fraction.
    pub const TEXEL_BIAS: u32 = 0x447a_0000; // 1000.0
}

/// Unaligned 16-byte copy as gcc emits it (`ldl/ldr` into `ra`, `rb`, then
/// `sdl/sdr`); the registers keep the copied doublewords.
fn copy16_unaligned(c: &mut Cpu, src: u32, dst: u32, ra: usize, rb: usize) {
    c.ldl(ra, src.wrapping_add(7));
    c.ldr(ra, src);
    c.ldl(rb, src.wrapping_add(0xf));
    c.ldr(rb, src.wrapping_add(8));
    c.sdl(ra, dst.wrapping_add(7));
    c.sdr(ra, dst);
    c.sdl(rb, dst.wrapping_add(0xf));
    c.sdr(rb, dst.wrapping_add(8));
}

/// `void WorldPainter_SampleSurfaceColours(WorldPainter* this, Vec4* scale,
/// Vec4* ratio, float u /*f12*/, float v /*f13*/)` (0x002edb20).
///
/// When the painter's surface and material are both of the painted kind,
/// samples up to two colours through the sampler object's virtual method
/// (`gp-0x854`, interface +0x10d8, slot +0x1d0): colour A from the layer of
/// kind 5 (its rectangle at the clamped (u, v)), colour B from the layer of
/// kind 1 (its quad bilinearly at (u, v), passed as the fraction of
/// `x + 1000`). A layer of neither kind leaves the colour at zero; a later
/// layer of the same kind overrides an earlier one. Then writes
/// `scale = (B - A) * weights * 2 A.x` and `ratio = scale / B`, with the
/// x components of both set to 0.5.
pub fn world_painter_sample_surface_colours(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use world_painter as wp;
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0xe0);
        frame.save(c, S4, 0xa0);
        frame.save(c, S6, 0x80);
        frame.save_fpr(c, 21, 0xf8);
        frame.save_fpr(c, 20, 0xf0);
        frame.save(c, S1, 0xd0);
        frame.save(c, S2, 0xc0);
        frame.save(c, S3, 0xb0);
        frame.save(c, S5, 0x90);
        frame.save(c, S7, 0x70);
        frame.save(c, FP, 0x60);
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 22, 0x100);
        c.g[S0][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        c.g[S6][0] = c.g[A2][0];
        c.f[21] = c.f[12];
        c.f[20] = c.f[13];
        let u = FpuF32(c.f[21]);
        let v = FpuF32(c.f[20]);
        let this = r32(c, S0);
        let scale_out = r32(c, S4);
        let ratio_out = r32(c, S6);
        // Colour A at sp+0, colour B at sp+0x10; both start at zero.
        for k in 0..8u32 {
            Addr(frame.at(c, 4 * k)).sw(c, 0);
        }
        'body: {
            if c.g[S0][0] == 0 {
                break 'body;
            }

            // Painted surface and painted material?
            c.cycles += 5;
            let tables = Addr(r32(c, GP).wrapping_add(wp::GP_TABLES)).lw(c);
            let material = Addr(this.wrapping_add(wp::MATERIAL_INDEX)).lh(c);
            let base = Addr(tables as u32).lw(c);
            c.g[A1][0] = material;
            c.g[A0][0] = base;
            c.g[V0][0] = 0;
            if (material as i64) >= 0 {
                c.cycles += 7;
                let surface = Addr(this.wrapping_add(wp::SURFACE_INDEX)).lbu(c);
                let entry = sx((base as u32).wrapping_add((surface as u32) << 3));
                let kind = Addr((entry as u32).wrapping_add(wp::SURFACE_KIND)).lw(c);
                c.g[V1][0] = kind ^ wp::SURFACE_KIND_PAINTED;
                if c.g[V1][0] == 0 {
                    c.cycles += 6;
                    c.g[V0][0] = 0x18;
                    let off = crate::abi::mult(c, material as u32, 0x18);
                    c.g[V1][0] = sx(off);
                    let mat = sx(off.wrapping_add(base as u32));
                    let kind = Addr((mat as u32).wrapping_add(wp::MATERIAL_KIND)).lw(c);
                    c.g[V1][0] = kind ^ wp::MATERIAL_KIND_PAINTED;
                    c.g[V0][0] = (c.g[V1][0] < 1) as u64;
                } else {
                    c.g[V0][0] = 0;
                }
            } else {
                c.g[V0][0] = 0;
            }
            c.cycles += 2;
            c.g[FP][0] = wp::WEIGHTS_HI as u64;
            if c.g[V0][0] == 0 {
                break 'body;
            }

            c.cycles += 8;
            c.g[AT][0] = wp::TEXEL_BIAS as u64;
            c.f[22] = wp::TEXEL_BIAS;
            let bias = FpuF32(wp::TEXEL_BIAS);
            c.g[S7][0] = wp::KIND_RECT;
            c.g[S1][0] = sx(this.wrapping_add(wp::LAYER_IDS));
            c.g[S5][0] = 0;
            c.g[S3][0] = 0;
            c.g[S2][0] = 2;
            let rd = |c: &mut Cpu, off: u32| Addr(this.wrapping_add(off)).read_fpu(c);
            loop {
                let shift = r32(c, S3) & 31;
                let layer_id = Addr(r32(c, S1));

                // Kind 5: colour A from the rectangle at the clamped (u, v).
                c.cycles += 5;
                let kinds = Addr(this.wrapping_add(wp::LAYER_KINDS)).lw(c);
                c.g[V0][0] = sx(((kinds as u32) as i32 >> shift) as u32) & 7;
                if c.g[V0][0] == wp::KIND_RECT {
                    c.cycles += 7;
                    c.g[AT][0] = 0x3f80_0000;
                    let one = FpuF32::ONE;
                    let u_ok = FpuF32::ZERO.le(u);
                    crate::abi::set_fcc(c, u_ok);
                    let mut cu = FpuF32::ZERO;
                    if u_ok {
                        c.cycles += 1;
                        cu = u.min(one);
                    }
                    c.cycles += 6;
                    let size_x = rd(c, wp::RECT + 8);
                    let v_ok = FpuF32::ZERO.le(v);
                    crate::abi::set_fcc(c, v_ok);
                    let origin_x = rd(c, wp::RECT);
                    let x = origin_x + size_x * cu;
                    let mut cv = FpuF32::ZERO;
                    if v_ok {
                        c.cycles += 1;
                        cv = v.min(one);
                    }
                    c.cycles += 22;
                    let size_y = rd(c, wp::RECT + 0xc);
                    let origin_y = rd(c, wp::RECT + 4);
                    let y = origin_y + size_y * cv;
                    // Registers at the call.
                    c.f[0] = origin_y.0;
                    c.f[1] = origin_x.0;
                    c.f[2] = cv.0;
                    c.f[3] = one.0;
                    c.f[4] = cu.0;
                    c.f[12] = x.0;
                    c.f[13] = y.0;
                    let sampler = Addr(r32(c, GP).wrapping_add(wp::GP_SAMPLER as u32)).lw(c);
                    let iface = Addr((sampler as u32).wrapping_add(wp::SAMPLER_IFACE)).lw(c);
                    let slot = crate::call::VSlot::read(c, iface, wp::SAMPLE_SLOT);
                    c.g[A0][0] = sx(frame.at(c, 0x20));
                    c.g[T0][0] = sampler;
                    c.g[A3][0] = 1;
                    c.g[V1][0] = iface;
                    c.g[A2][0] = layer_id.lh(c);
                    c.g[V0][0] = slot.func;
                    c.g[A1][0] = slot.this(sampler);
                    call_ptr(c, slot.func as u32, 0x002e_dc84)?;
                    copy16_unaligned(c, frame.at(c, 0x20), frame.at(c, 0), A0, A1);
                }
                c.g[V0][0] = Addr(this.wrapping_add(wp::LAYER_KINDS)).lw(c);

                // Kind 1: colour B from the quad at (u, v).
                c.cycles += 5;
                c.g[V1][0] = 1;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> (r32(c, S5) & 31)) as u32) & 7;
                if c.g[V0][0] == wp::KIND_QUAD {
                    c.cycles += 56;
                    let p0x = rd(c, wp::QUAD);
                    let p0y = rd(c, wp::QUAD + 4);
                    let p1x = rd(c, wp::QUAD + 8);
                    let p1y = rd(c, wp::QUAD + 0xc);
                    let p2x = rd(c, wp::QUAD + 0x10);
                    let p2y = rd(c, wp::QUAD + 0x14);
                    let p3x = rd(c, wp::QUAD + 0x18);
                    let p3y = rd(c, wp::QUAD + 0x1c);
                    // x: P0->P2 and P1->P3 by u, then between them by v;
                    // y: P0->P1 and P2->P3 by v, then between them by u.
                    let dx02 = (p2x - p0x) * u;
                    let dy01 = (p1y - p0y) * v;
                    let dy23 = (p3y - p2y) * v;
                    let dx13 = (p3x - p1x) * u;
                    let x02 = p0x + dx02;
                    let x13 = p1x + dx13;
                    let y01 = p0y + dy01;
                    let y23 = p2y + dy23;
                    let dx = (x13 - x02) * v;
                    let dy = (y23 - y01) * u;
                    let x = x02 + dx + bias;
                    let y = y01 + dy + bias;
                    let tx = x.to_i32();
                    let ty = y.to_i32();
                    let fx = x - FpuF32::from_i32(tx);
                    let fy = y - FpuF32::from_i32(ty);
                    // Registers at the call.
                    c.f[0] = ty as u32;
                    c.f[1] = dx.0;
                    c.f[2] = y.0;
                    c.f[3] = x.0;
                    c.f[4] = dx02.0;
                    c.f[5] = dy01.0;
                    c.f[6] = dy23.0;
                    c.f[7] = dx13.0;
                    c.f[12] = fx.0;
                    c.f[13] = fy.0;
                    let sampler = Addr(r32(c, GP).wrapping_add(wp::GP_SAMPLER as u32)).lw(c);
                    let iface = Addr((sampler as u32).wrapping_add(wp::SAMPLER_IFACE)).lw(c);
                    let slot = crate::call::VSlot::read(c, iface, wp::SAMPLE_SLOT);
                    c.g[A0][0] = sx(frame.at(c, 0x20));
                    c.g[A3][0] = 1;
                    c.g[A2][0] = layer_id.lh(c);
                    c.g[A1][0] = slot.this(sampler);
                    c.g[T0][0] = slot.func;
                    c.g[V0][0] = sx(tx as u32);
                    c.g[V1][0] = sx(ty as u32);
                    call_ptr(c, slot.func as u32, 0x002e_dd78)?;
                    copy16_unaligned(c, frame.at(c, 0x20), frame.at(c, 0x10), V0, V1);
                }
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(2));

                c.cycles += 4;
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(3));
                c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(3));
                if (c.g[S2][0] as i64) < 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_dc00);
                }
            }

            // scale = (B - A) * weights * 2 A.x; ratio = scale / B.
            c.cycles += 70;
            let sp_rd = |c: &mut Cpu, off: u32| Addr(frame.at(c, off)).read_fpu(c);
            let sp_wr = |c: &mut Cpu, off: u32, v: FpuF32| Addr(frame.at(c, off)).write_fpu(c, v);
            let a = [sp_rd(c, 0), sp_rd(c, 4), sp_rd(c, 8), sp_rd(c, 0xc)];
            let b = [sp_rd(c, 0x10), sp_rd(c, 0x14), sp_rd(c, 0x18), sp_rd(c, 0x1c)];
            c.g[V0][0] = sx(wp::WEIGHTS);
            let weights = Addr(wp::WEIGHTS);
            let w = [
                weights.read_fpu(c),
                weights.off(4).read_fpu(c),
                weights.off(8).read_fpu(c),
                weights.off(0xc).read_fpu(c),
            ];
            let two_ax = a[0] + a[0];
            let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2], b[3] - a[3]];
            let dw = [d[0] * w[0], d[1] * w[1], d[2] * w[2], d[3] * w[3]];
            let scale = [dw[0] * two_ax, dw[1] * two_ax, dw[2] * two_ax, dw[3] * two_ax];
            for k in 0..4u32 {
                sp_wr(c, 0x40 + 4 * k, d[k as usize]);
            }
            sp_wr(c, 0x30, dw[0]);
            sp_wr(c, 0x24, scale[1]);
            sp_wr(c, 0x20, scale[0]);
            sp_wr(c, 0x34, dw[1]);
            sp_wr(c, 0x28, scale[2]);
            sp_wr(c, 0x2c, scale[3]);
            sp_wr(c, 0x38, dw[2]);
            sp_wr(c, 0x3c, dw[3]);
            c.g[AT][0] = 0x3f00_0000;
            let half = FpuF32(0x3f00_0000);
            copy16_unaligned(c, frame.at(c, 0x20), scale_out, T1, V0);
            let s = [
                Addr(scale_out).read_fpu(c),
                Addr(scale_out.wrapping_add(4)).read_fpu(c),
                Addr(scale_out.wrapping_add(8)).read_fpu(c),
                Addr(scale_out.wrapping_add(0xc)).read_fpu(c),
            ];
            let ratio = [s[0].div(b[0]), s[1].div(b[1]), s[2].div(b[2]), s[3].div(b[3])];
            for k in 0..4u32 {
                sp_wr(c, 0x20 + 4 * k, ratio[k as usize]);
            }
            copy16_unaligned(c, frame.at(c, 0x20), ratio_out, A1, T1);
            Addr(scale_out).write_fpu(c, half);
            Addr(ratio_out).write_fpu(c, half);
            // FPU registers as the original leaves them.
            c.f[0] = ratio[3].0;
            c.f[1] = ratio[0].0;
            c.f[2] = ratio[1].0;
            c.f[3] = ratio[2].0;
            c.f[4] = half.0;
            c.f[5] = d[3].0;
            c.f[6] = dw[2].0;
            c.f[7] = dw[3].0;
            c.f[8] = scale[2].0;
            c.f[9] = scale[0].0;
            c.f[10] = b[3].0;
            c.f[11] = b[0].0;
            c.f[12] = b[2].0;
            c.f[13] = b[1].0;
        }

        c.cycles += 15;
        for (reg, off) in [
            (S0, 0xe0),
            (S1, 0xd0),
            (S2, 0xc0),
            (S3, 0xb0),
            (S4, 0xa0),
            (S5, 0x90),
            (S6, 0x80),
            (S7, 0x70),
            (FP, 0x60),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 22, 0x100);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `OverlapList_GatherAll` against the generated body, with the real
    /// callees (`SpatialKey_ClassifyAABB`, `Cell_GatherObjects`,
    /// `Cell_GatherAllSubtree`, `Cell_GatherAllRecursive`) on short paths.
    /// Every seed gives each of the eight top-level cells a random fate:
    /// no tree, box inside the cell (gather the root's objects, all-list and
    /// children whole), partly overlapping (recurse) or disjoint (skip).
    #[test]
    fn overlap_list_gather_all_matches() {
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        let f = |x: f32| x.to_bits();
        check(ee::s11::f_00332db8__generated, overlap_list_gather_all, 200, &|c, seed| {
            let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ 0x2db8);
            scramble(c, &mut r);
            c.pc = 0x0033_2db8;
            c.g[GP][0] = GAME_GP as u64;
            let gp_at = |off: i32| (GAME_GP as i32 + off) as u32;
            // The BELibrary dummy and the world (callees' globals).
            let belib = DATA + 0x3_0000;
            let world = DATA + 0x3_1000;
            c.bus.write32(gp_at(-0x1f04), belib);
            c.bus.write32(gp_at(-0x848), world);
            c.bus.write32(world + 0x84, world + 0x100);
            c.bus.write32(world + 0x100 + 0x7c, 0);
            // SpatialKey_ClassifyAABB's cell margin: the cell [x, x+1]
            // widened by 0.25 on each side, times 2^level.
            c.bus.write32(gp_at(-0x2dfc), f(0.25));

            let this = DATA;
            let grid = DATA + 0x1000;
            let bx = DATA + 0x1200;
            let leaf = DATA + 0x3000; // all zero: no lists, no children
            // Stale counts (the function resets them) and the grid stamp.
            c.bus.write32(this + gather_all_list::NODE_COUNT, r.next_u32());
            c.bus.write32(this + gather_all_list::OBJ_COUNT, r.next_u32());
            c.bus.write32(this + gather_all_list::ALL_COUNT, r.next_u32());
            c.bus.write32(this + gather_all_list::GRID, grid);
            c.bus.write32(grid + gather_all_list::GRID_STAMP, r.next_u32());
            // The query box. With min = (1.25, 1.25, 0) and max = 0.5, a
            // level-0 cell at the origin (hi = 1.25) classifies 0 ("within"),
            // a level-1 cell at the origin 2 (partly) and a cell at x = 100
            // 1 (disjoint).
            for (k, v) in [1.25f32, 1.25, 0.0].into_iter().enumerate() {
                c.bus.write32(bx + 4 * k as u32, f(v));
                c.bus.write32(bx + 0x10 + 4 * k as u32, f(0.5));
            }
            for i in 0..8u32 {
                let cell = grid + gather_all_list::CELL_SIZE * i;
                let root = DATA + 0x2000 + 0x100 * i;
                let (level, x) = match r.next_u32() % 4 {
                    0 => (0, 0),
                    1 => (1, 0),
                    _ => (0, 100),
                };
                c.bus.write32(cell, level);
                c.bus.write32(cell + 4, x);
                c.bus.write32(cell + 8, 0);
                c.bus.write32(cell + 0xc, 0);
                let present = r.next_u32() % 5 != 0;
                c.bus.write32(cell + gather_all_list::CELL_ROOT, if present { root } else { 0 });
                // The root: no node list; 0..2 objects (hit or random box);
                // 0..2 all-list items; random leaf children.
                c.bus.write32(root + 0x20, 0);
                let mut next = 0;
                for j in 0..r.next_u32() % 3 {
                    let obj = DATA + 0x4000 + 0x200 * (2 * i + j);
                    c.bus.write32(obj, next);
                    let hit = r.next_u32() % 2 == 0;
                    for k in 0..3u32 {
                        let (lo, hi) = if hit {
                            (f(-10.0), f(10.0))
                        } else {
                            (r.f32(20.0), r.f32(20.0))
                        };
                        c.bus.write32(obj + gather_all_node::OBJ_MIN + 4 * k, lo);
                        c.bus.write32(obj + gather_all_node::OBJ_MAX + 4 * k, hi);
                    }
                    next = obj;
                }
                c.bus.write32(root + gather_all_node::OBJECTS, next);
                let mut next = 0;
                for j in 0..r.next_u32() % 3 {
                    let item = DATA + 0x7000 + 0x10 * (2 * i + j);
                    c.bus.write32(item, next);
                    next = item;
                }
                c.bus.write32(root + gather_all_node::ALL, next);
                for k in 0..8u32 {
                    let child = if r.next_u32() % 2 == 0 { leaf } else { 0 };
                    c.bus.write32(root + 4 * k, child);
                }
            }
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(bx);
        });
    }

    /// `cOVStateHUD1P_Update` against the generated body. The world, race
    /// manager, race, game mode, rider, rider metrix (with its metric
    /// record table), script engine, WScriptMan and BE settings are small
    /// dummies whose fields each seed picks so that every part of the update
    /// runs both ways: blink phase (HUD allowed or not), music ticker (fade
    /// phases and the 'now playing' layout), trick-score readout, mission
    /// flags, element/message fades, race and mission clocks, meter pulses
    /// and, for one or two HUD players, the message flags, meters and record
    /// fades. Real callees stay on short paths (empty song strings, zeroed
    /// fonts, clean metrix bitmasks, `gp-0x9a0 = 0` for the progress bar);
    /// the virtual calls (BELibrary interfaces, the rider's +0x6c0 object,
    /// the game-mode handler, the mission's vtable) land in the fakes, one of
    /// which unwinds in some seeds.
    #[test]
    fn c_ov_state_hud1p_update_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::{elf_image, Rng, DATA, GAME_GP};
        let f = |x: f32| x.to_bits();
        let gp_at = |off: i32| (GAME_GP as i32 + off) as u32;
        fn pick(r: &mut Rng, n: u32) -> u32 {
            r.next_u32() % n
        }
        // Dummy objects.
        const HUD: u32 = DATA + 0x6_0000;
        const WORLD: u32 = DATA + 0x7_0000;
        const RM: u32 = DATA + 0x7_0100;
        const RACE: u32 = DATA + 0x7_0200;
        const PLAYER: u32 = DATA + 0x7_0400;
        const RIDER: u32 = DATA + 0x7_1000;
        const RVT: u32 = DATA + 0x7_1800;
        const METRIX: u32 = DATA + 0x7_2000;
        const TABLE: u32 = DATA + 0x7_3000;
        const GM: u32 = DATA + 0x7_5000;
        const HOBJ: u32 = DATA + 0x7_5200;
        const HVT: u32 = DATA + 0x7_5300;
        const SE: u32 = DATA + 0x7_6000;
        const LIST: u32 = DATA + 0x7_6800;
        const WSM: u32 = DATA + 0x7_7000;
        const MN: u32 = DATA + 0x7_7800;
        const MVT: u32 = DATA + 0x7_7900;
        const FONT1: u32 = DATA + 0x7_8000;
        const FONT2: u32 = DATA + 0x7_8100;
        const AUDIO: u32 = DATA + 0x7_9000;
        const STR: u32 = DATA + 0x7_a000;
        const BEDUMMY: u32 = DATA + 0x7_b000;
        const BEOBJ: u32 = DATA + 0x7_b100;
        const BEVT: u32 = DATA + 0x7_b200;
        // Metric records the update reads from the metrix table (+0x1b0).
        const RECORDS: [u32; 5] = [0x9c, 0x57c, 0x618, 0xfd8, 0x1074];
        covered(ee::s04::f_001ea930__generated, c_ov_state_hud1p_update, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x001e_a930, 10);
            elf_image(c);
            let r = &mut r;
            // Globals.
            let flags = if seed % 16 == 15 { 1 } else { pick(r, 2) << 3 };
            c.bus.write32(gp_at(-0x6a0), flags);
            c.bus.write32(gp_at(-0x848), WORLD);
            c.bus.write32(gp_at(-0x84c), SE);
            c.bus.write32(gp_at(0xce8), WSM);
            c.bus.write32(gp_at(0x410), AUDIO);
            c.bus.write32(gp_at(-0x9a0), 0); // no race-progress riders
            // BELibrary: the instance, and interface objects whose method
            // (vtable +8/+0xc) is a fake.
            c.bus.write32(gp_at(-0x1f04), BEDUMMY);
            c.bus.write32(gp_at(-0x1efc), 0);
            c.bus.write32(0x0053_08a8, BEOBJ);
            c.bus.write32(BEOBJ + 4, BEVT);
            c.bus.write16(BEVT + 8, 0);
            c.bus.write32(BEVT + 0xc, FAKE);
            // BE settings bytes and words.
            c.bus.write8(0x0053_5c10, pick(r, 6) as u8); // BE mode
            c.bus.write8(0x0053_5c11, [2u8, 0][pick(r, 2) as usize]);
            c.bus.write8(0x0053_5c12, pick(r, 4) as u8); // mask index
            c.bus.write32(0x0053_4b30, pick(r, 2));
            c.bus.write32(0x0053_08d0, r.next_u32());
            c.bus.write32(0x0053_5610, r.next_u32()); // settings block word 0
            // Trick-score thresholds: index bytes and thresholds.
            for k in 0..4u32 {
                c.bus.write8(0x0053_6708 + 4 * k, k as u8);
                c.bus.write32(0x0053_6640 + 4 * k, pick(r, 3000));
            }
            // Strings: the ticker header and the two icon labels.
            for (at, s) in [
                (STR, &b"NOW PLAYING\0"[..]),
                (STR + 0x20, &b"A\0"[..]),
                (STR + 0x30, &b"BB\0"[..]),
            ] {
                for (i, b) in s.iter().enumerate() {
                    c.bus.write8(at + i as u32, *b);
                }
            }
            c.bus.write32(gp_at(-0xe74), STR);
            c.bus.write32(gp_at(-0xf20), STR + 0x20);
            c.bus.write32(gp_at(-0xf1c), STR + 0x30);
            // Audio system: no current song (the getters return NULL).
            c.bus.write32(AUDIO + 0x118 + 0x3f0, 0xffff_ffff);
            // World -> race manager -> race; the game mode.
            c.bus.write32(WORLD + 0xc0, GM);
            c.bus.write32(WORLD + 0x84, RM);
            c.bus.write32(RM + 0xc, RACE);
            c.bus.write32(RM + 0x84, RM);
            c.bus.write32(RM + 0x10, pick(r, 3)); // viewports
            c.bus.write32(RACE, [5u32, 6, 0, 1][pick(r, 4) as usize]); // race type
            c.bus.write32(RACE + 0xc, (pick(r, 4000) as i32 - 200) as u32); // time
            c.bus.write32(RACE + 0x28, RIDER);
            c.bus.write32(RACE + 0x2c, RIDER);
            c.bus.write32(RACE + 0x40, PLAYER);
            c.bus.write32(RACE + 0x44, PLAYER);
            c.bus.write32(RACE + 0x78, 0);
            c.bus.write32(RACE + 0x7c, pick(r, 3));
            c.bus.write32(RACE + 0x80, pick(r, 2));
            c.bus.write32(PLAYER + 0x18, RIDER);
            c.bus.write32(GM, [2u32, 0, 2][pick(r, 3) as usize]);
            c.bus.write32(GM + 4, 0);
            c.bus.write32(GM + 0x78, pick(r, 300));
            c.bus.write32(GM + 0x7c, pick(r, 3000));
            c.bus.write32(GM + 0x88, pick(r, 2));
            c.bus.write32(GM + 0x8c, pick(r, 2));
            // GameModeMan handler table (0x536668[GM+4]) -> fake method.
            c.bus.write32(0x0053_6668, HOBJ);
            c.bus.write32(HOBJ, HVT);
            c.bus.write16(HVT + 0x48, 0);
            c.bus.write32(HVT + 0x4c, FAKE + 0x20);
            // The rider: its +0x6c0 object (fake +0x3c method), metrix, the
            // trick-chain id, the finish time and the finished flag.
            c.bus.write32(RIDER + 0x6c0, RVT);
            c.bus.write16(RVT + 0x38, 0);
            c.bus.write32(RVT + 0x3c, FAKE + 0x10);
            c.bus.write32(RIDER + 0x790, METRIX);
            c.bus.write32(RIDER + 0xec, pick(r, 3));
            c.bus.write32(RIDER + 0x470, r.f32(1.0));
            c.bus.write32(RIDER + 0x480, pick(r, 2));
            // Metrix: score, record table (or none), active-message bits
            // (clean, so no rebuild).
            c.bus.write32(METRIX + 0x198, pick(r, 3000));
            c.bus.write32(METRIX + 0x1b0, if pick(r, 5) == 0 { 0 } else { TABLE });
            c.bus.write32(METRIX + 0x1b8, r.next_u32());
            c.bus.write32(METRIX + 0x1bc, r.next_u32());
            c.bus.write32(METRIX + 0x1c0, 0);
            for off in RECORDS {
                let rec = TABLE + off;
                let id = if pick(r, 3) == 0 { 0x34 } else { 1 };
                let total = [1.0f32, 2.0][pick(r, 2) as usize];
                let done = [0.0f32, 0.5 * total, total][pick(r, 3) as usize];
                c.bus.write32(rec, id);
                c.bus.write32(rec + 4, f(total));
                c.bus.write32(rec + 8, f(done));
                c.bus.write32(rec + 0xc, [0u32, 7, 10, 20][pick(r, 4) as usize]);
                c.bus.write32(rec + 0x14, r.next_u32());
            }
            // Script engine (gp-0x84c): state 0 or 2; HUD allowed through
            // a skippable, playing script list.
            c.bus.write32(SE + 0x550, if pick(r, 4) == 0 { 2 } else { 0 });
            c.bus.write32(SE + 0x554, pick(r, 2));
            c.bus.write32(SE + 0x54c, LIST);
            c.bus.write32(LIST + 0xa4, 3);
            c.bus.write32(LIST + 0x14, pick(r, 2));
            c.bus.write32(LIST + 0xa8, 0);
            c.bus.write32(LIST + 0xc4, 0);
            c.bus.write32(LIST + 0xc8, 1);
            c.bus.write32(LIST + 0xbc, r.next_u32());
            // WScriptMan: a mission (or none) and its timer fields.
            let mission = if pick(r, 2) == 0 { 0xffff_ffff } else { pick(r, 5) };
            c.bus.write32(WSM + 0x2a0, mission);
            c.bus.write32(WSM + 0x2b8, if pick(r, 3) == 0 { 0 } else { MN });
            c.bus.write32(MN + 0x5c, MVT);
            c.bus.write16(MVT + 0x18, 0);
            c.bus.write32(MVT + 0x1c, FAKE + 0x30);
            c.bus.write32(MN + 0x60, [2u32, 0][pick(r, 2) as usize]);
            c.bus.write32(MN + 0x18, 0);
            for off in [4u32, 0x10, 0x1c, 0x28, 0x34] {
                c.bus.write32(WSM + off, pick(r, 2));
            }
            c.bus.write32(WSM + 0x20, (pick(r, 4000) as i32 - 200) as u32);
            c.bus.write32(WSM + 0x24, r.next_u32());
            // Fonts: zeroed glyph tables, random scales.
            for font in [FONT1, FONT2] {
                c.bus.write32(font + 0x30, r.f32(2.0));
                c.bus.write32(font + 0x34, r.f32(2.0));
                c.bus.write32(font + 0x14, pick(r, 20));
            }
            // The HUD state.
            c.bus.write32(HUD + 0x428, FONT2);
            c.bus.write32(HUD + 0x42c, FONT1);
            c.bus.write32(HUD + 0x3cc, r.next_u32()); // element mask
            c.bus.write32(HUD + 0x3c4, [0x50u32, 8, 0x3a, 9][pick(r, 4) as usize]);
            for (off, range) in [
                (0xc94u32, 3.0f32),
                (0x190, 9.0),
                (0x3c0, 2.0),
                (0xa44, 2.0),
                (0x3d0, 2.0),
                (0x164, 3.0),
                (0x168, 7.0),
            ] {
                c.bus.write32(HUD + off, r.f32(range));
            }
            c.bus.write32(HUD + 0x160, pick(r, 2));
            c.bus.write32(HUD + 0x3d4, (pick(r, 4) == 0) as u32);
            c.bus.write32(HUD + 0x3d8, (pick(r, 4) == 0) as u32);
            c.bus.write32(HUD + 0x55c, pick(r, 2));
            // HUD players: slot indices (negative ends the list) and their
            // meter/fade fields.
            c.bus.write32(HUD + 0x48, [0u32, 1, 0, 0xffff_ffff][pick(r, 4) as usize]);
            c.bus.write32(HUD + 0x48 + 0x8c, [0u32, 1, 0xffff_ffff][pick(r, 3) as usize]);
            for k in 0..2u32 {
                let s = HUD + 0x48 + 0x8c * k;
                c.bus.write32(s + 0x44, [0xffff_ffffu32, 0, 1, 2][pick(r, 4) as usize]);
                for off in [0x48u32, 0x4c, 0x64, 0x68, 0x70, 0x78] {
                    c.bus.write32(s + off, r.f32(2.5));
                }
                c.bus.write32(s + 0x6c, [0xffff_ffffu32, 0, 3, 7][pick(r, 4) as usize]);
                c.bus.write32(s + 0x74, pick(r, 2));
                for off in [0x80u32, 0x84, 0x88] {
                    c.bus.write32(s + off, r.next_u32());
                }
            }
            c.g[A0][0] = sx(HUD);
            c.pc = 0x001e_a930;
        });
    }

    /// `WorldPainter_SampleSurfaceColours` against the generated body. The
    /// painter tables (`gp+0x16c8`) make the surface/material gate pass or
    /// fail (null painter, negative material, wrong surface or material
    /// kind); the three layer kinds are random among rect (5), quad (1) and
    /// neither, so each seed samples zero to three colours through the
    /// sampler's virtual method (`gp-0x854`, interface +0x10d8, slot +0x1d0,
    /// a fake that sometimes unwinds). (u, v) cover below 0, inside and
    /// above 1 (the rectangle clamps); the output vectors are sometimes only
    /// word-aligned (the `sdl/sdr` copies).
    #[test]
    fn world_painter_sample_surface_colours_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::{DATA, GAME_GP};
        let gp_at = |off: i32| (GAME_GP as i32 + off) as u32;
        covered(
            ee::s09::f_002edb20__generated,
            world_painter_sample_surface_colours,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2edb20, 6);
                c.pc = 0x002e_db20;
                let this = DATA + 0x1000;
                let tables = DATA + 0x2000;
                let base = DATA + 0x3000;
                let sampler = DATA + 0x6000;
                // The gate: surface entry kind 6, material entry kind 3.
                c.bus.write32(gp_at(world_painter::GP_TABLES as i32), tables);
                c.bus.write32(tables, base);
                let surface = r.next_u32() % 4;
                let material = r.next_u32() % 4;
                c.bus.write8(this + world_painter::SURFACE_INDEX, surface as u8);
                let gate = r.next_u32() % 8;
                let mat_index = if gate == 0 { 0x8000 | material } else { material };
                c.bus.write16(this + world_painter::MATERIAL_INDEX, mat_index as u16);
                for k in 0..4 {
                    c.bus.write32(base + 8 * k + world_painter::SURFACE_KIND, 6);
                    c.bus.write32(base + 0x18 * k + world_painter::MATERIAL_KIND, 3);
                }
                if gate == 1 {
                    c.bus.write32(base + 8 * surface + world_painter::SURFACE_KIND, 7);
                }
                if gate == 2 {
                    c.bus.write32(base + 0x18 * material + world_painter::MATERIAL_KIND, 2);
                }
                // Layer kinds (3 bits each; upper bits random) and ids.
                let mut kinds = r.next_u32() & !0x1ff;
                for i in 0..3 {
                    let k = [5u32, 1, 5, 1, 0, 7][(r.next_u32() % 6) as usize];
                    kinds |= k << (3 * i);
                }
                c.bus.write32(this + world_painter::LAYER_KINDS, kinds);
                for i in 0..3 {
                    c.bus.write16(this + world_painter::LAYER_IDS + 2 * i, r.next_u32() as u16);
                }
                // Rectangle and quad.
                for k in 0..12 {
                    c.bus.write32(this + world_painter::RECT + 4 * k, r.f32(64.0));
                }
                // The sampler object and its interface's slot.
                c.bus.write32(gp_at(world_painter::GP_SAMPLER), sampler);
                let iface = vtable(c, &mut r, &[world_painter::SAMPLE_SLOT]);
                c.bus.write32(sampler + world_painter::SAMPLER_IFACE, iface);
                // Colour weights.
                for k in 0..4 {
                    c.bus.write32(world_painter::WEIGHTS + 4 * k, r.f32(2.0));
                }
                // Sampled colours: whatever the stack held (random fill) or
                // the forced fake results do not reach memory, so seed the
                // result slot area with random floats (frame at SP-0x110).
                for k in 0..8 {
                    c.bus.write32(crate::difftest::SP - 0x110 + 0x20 + 4 * k, r.f32(4.0));
                }
                let scale = DATA + 0x8000 + 4 * (r.next_u32() % 3);
                let ratio = DATA + 0x8100 + 4 * (r.next_u32() % 3);
                c.g[A0][0] = if seed % 16 == 0 { 0 } else { sx(this) };
                c.g[A1][0] = sx(scale);
                c.g[A2][0] = sx(ratio);
                let uv = |r: &mut crate::difftest::Rng| match r.next_u32() % 4 {
                    0 => r.edge_f32(),
                    _ => r.f32(1.5),
                };
                c.f[12] = uv(&mut r);
                c.f[13] = uv(&mut r);
            },
        );
    }
}
