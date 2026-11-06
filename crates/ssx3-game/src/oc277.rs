//! Staging wave 210 (oc277): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0033_1450,
    overlap_list_gather_type2,
    "OverlapList_GatherType2",
)];

/// Field offsets of `OverlapList_GatherType2`'s overlap list and grid.
pub(crate) mod gather_type2_list {
    /// `self+0`: the spatial grid.
    pub const GRID: u32 = 0x0;
    /// `self+4`: copy of the grid's stamp (`grid+0xa0`).
    pub const STAMP: u32 = 0x4;
    /// `self+8` and `self+0x10c`: cached counts, invalidated (-1) at the end.
    pub const NTRIS: u32 = 0x8;
    pub const COUNT: u32 = 0x10c;
    /// `self+0x210`: number of gathered type-2 items; the list follows.
    pub const ITEM_COUNT: u32 = 0x210;
    pub const ITEMS: u32 = 0x214;
    /// Capacity stored in the gather context (unused by this gather).
    pub const CAP: u64 = 0x200;

    /// `grid+0xa0`: copied to `STAMP`.
    pub const GRID_STAMP: u32 = 0xa0;
    /// Eight top-level cells of 0x14 bytes `{ level, x, y, z, root }`.
    pub const CELL_SIZE: u32 = 0x14;
    pub const CELL_ROOT: u32 = 0x10;

    /// A tree node: eight children at +0..+0x1c, its type-2 list at +0x28.
    pub const NODE_TYPE2: u32 = 0x28;
}

/// `SpatialKey_ClassifyAABB(cell, box)` (0x00328360, through the function
/// table): 0 the box lies within the cell, 1 disjoint, 2 partly overlapping.
const TYPE2_CLASSIFY: u32 = 0x0032_8360;

/// `Cell_GatherType2Subtree(tree, ctx)` (0x00340b18): append a whole
/// subtree's type-2 lists.
const TYPE2_SUBTREE: Callee = Callee {
    addr: 0x0034_0b18,
    f: ee::s11::Cell_GatherType2Subtree,
};

/// `Cell_GatherType2Recursive(_, ctx, ctx, box, tree, cell)` (0x00339598).
const TYPE2_RECURSIVE: Callee = Callee {
    addr: 0x0033_9598,
    f: ee::s11::Cell_GatherType2Recursive,
};

/// Per inlined `queryCell` copy: its call sites and loop head.
struct Type2Cell {
    classify_ret: u32,
    /// Head of the type-2 list loop; the eight subtree calls return to
    /// `list_loop + 0x3c + 0x14 * k`.
    list_loop: u32,
    recursive_ret: u32,
    /// Cycles of the block before the list loop.
    list_cycles: u64,
}

const TYPE2_CELLS: [Type2Cell; 8] = [
    Type2Cell {
        classify_ret: 0x0033_14cc,
        list_loop: 0x0033_14e8,
        recursive_ret: 0x0033_15dc,
        list_cycles: 1,
    },
    Type2Cell {
        classify_ret: 0x0033_1624,
        list_loop: 0x0033_1640,
        recursive_ret: 0x0033_1734,
        list_cycles: 1,
    },
    Type2Cell {
        classify_ret: 0x0033_1750,
        list_loop: 0x0033_1770,
        recursive_ret: 0x0033_1864,
        list_cycles: 2,
    },
    Type2Cell {
        classify_ret: 0x0033_1880,
        list_loop: 0x0033_18a0,
        recursive_ret: 0x0033_1994,
        list_cycles: 2,
    },
    Type2Cell {
        classify_ret: 0x0033_19b4,
        list_loop: 0x0033_19d0,
        recursive_ret: 0x0033_1ac4,
        list_cycles: 1,
    },
    Type2Cell {
        classify_ret: 0x0033_1ae4,
        list_loop: 0x0033_1b00,
        recursive_ret: 0x0033_1bf4,
        list_cycles: 1,
    },
    Type2Cell {
        classify_ret: 0x0033_1c14,
        list_loop: 0x0033_1c30,
        recursive_ret: 0x0033_1d24,
        list_cycles: 1,
    },
    Type2Cell {
        classify_ret: 0x0033_1d44,
        list_loop: 0x0033_1d60,
        recursive_ret: 0x0033_1e50,
        list_cycles: 1,
    },
];

/// Where a cell query finds the cell pointer it passes to
/// `Cell_GatherType2Recursive`: a register, or a frame slot.
#[derive(Clone, Copy)]
enum Type2CellPtr {
    Reg(usize),
    Slot(u32),
}

/// `void OverlapList_GatherType2(sOverlapList* self, const float* box)`
/// (0x00331450): rebuild the overlap list's type-2 item list for the query
/// box. A gather context `{ list, count*, cap, box }` on the stack points
/// at `self->items`; the count is reset, the grid stamp copied, then each
/// of the grid's eight top-level cells with a tree (the inlined
/// `queryCell`) is classified against the box: a cell the box lies within
/// has its root's type-2 list appended and its eight children gathered
/// whole (`Cell_GatherType2Subtree`); a partly overlapping cell is recursed
/// into (`Cell_GatherType2Recursive`); a disjoint one is skipped. Finally
/// the two cached counts are invalidated (-1).
pub fn overlap_list_gather_type2(c: &mut Cpu) {
    use gather_type2_list as list;
    guest(c, |c| {
        // Frame and the gather context at sp+0.
        c.cycles += 27;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S5, 0x70);
        frame.save(c, FP, 0x40);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S1, 0xb0);
        c.g[FP][0] = c.g[A0][0];
        frame.save(c, S2, 0xa0);
        let this = r32(c, FP);
        c.g[V1][0] = sx(this.wrapping_add(list::ITEMS));
        frame.save(c, S3, 0x90);
        c.g[A0][0] = sx(this.wrapping_add(list::ITEM_COUNT));
        frame.save(c, S6, 0x60);
        c.g[A1][0] = list::CAP;
        frame.save(c, S7, 0x50);
        frame.save_ra(c, 0x30);
        frame.save(c, S4, 0x80);
        let s = sp(c);
        c.g[S4][0] = c.lw(this.wrapping_add(list::GRID));
        let grid = r32(c, S4);
        c.g[V0][0] = c.lw(grid.wrapping_add(list::GRID_STAMP));
        // ctx = { this->items, &this->item_count, 0x200, box }
        c.sw(s, c.g[V1][0]);
        c.sw(this.wrapping_add(list::STAMP), c.g[V0][0]);
        c.sw(s.wrapping_add(0x4), c.g[A0][0]);
        c.sw(s.wrapping_add(0x8), c.g[A1][0]);
        c.sw(s.wrapping_add(0xc), c.g[S5][0]);
        c.sw(this.wrapping_add(list::ITEM_COUNT), 0);
        c.g[S1][0] = c.lw(grid.wrapping_add(list::CELL_ROOT));
        c.g[S0][0] = c.g[SP][0];

        // Cell 0 (the grid pointer itself is the cell).
        if c.g[S1][0] != 0 {
            c.cycles += 7;
            c.g[S2][0] = sx(s.wrapping_add(0x10));
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S5][0];
            call_ptr(c, TYPE2_CLASSIFY, TYPE2_CELLS[0].classify_ret)?;
            gather_type2_query_cell(c, 0, S2, Type2CellPtr::Reg(S4))?;
        } else {
            c.cycles += 1;
            gather_type2_next_root(c, 0);
        }

        // Cell 1 in s2; cells 2, 3 in s6, s7; 4..7 in frame slots.
        c.cycles += 12;
        let grid = r32(c, S4);
        let cell = |k: u32| grid.wrapping_add(list::CELL_SIZE * k);
        c.g[V1][0] = sx(cell(4));
        c.sw(s.wrapping_add(0x18), c.g[V1][0]);
        c.g[S2][0] = sx(cell(1));
        c.g[V1][0] = sx(cell(5));
        c.g[S6][0] = sx(cell(2));
        c.sw(s.wrapping_add(0x1c), c.g[V1][0]);
        c.g[S7][0] = sx(cell(3));
        c.g[V1][0] = sx(cell(6));
        c.sw(s.wrapping_add(0x20), c.g[V1][0]);
        c.g[V1][0] = sx(cell(7));
        c.sw(s.wrapping_add(0x24), c.g[V1][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            c.g[S1][0] = c.lw(r32(c, S2).wrapping_add(list::CELL_ROOT));
            c.g[S3][0] = sx(s.wrapping_add(0x11));
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S5][0];
            call_ptr(c, TYPE2_CLASSIFY, TYPE2_CELLS[1].classify_ret)?;
            gather_type2_query_cell(c, 1, S3, Type2CellPtr::Reg(S2))?;
        } else {
            c.cycles += 1;
            gather_type2_next_root(c, 1);
        }

        // Cells 2, 3: the cell pointer is in a saved register.
        for (i, cell_reg) in [(2usize, S6), (3, S7)] {
            c.cycles += 2;
            let present = c.g[V0][0] != 0;
            c.g[S2][0] = sx(s.wrapping_add(0x10 + i as u32));
            if present {
                c.cycles += 7;
                c.g[S1][0] = c.lw(r32(c, cell_reg).wrapping_add(list::CELL_ROOT));
                c.g[A0][0] = c.g[cell_reg][0];
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, TYPE2_CLASSIFY, TYPE2_CELLS[i].classify_ret)?;
                gather_type2_query_cell(c, i, S2, Type2CellPtr::Reg(cell_reg))?;
            } else {
                c.cycles += 1;
                gather_type2_next_root(c, i);
            }
        }

        // Cells 4..7: the cell pointer is reloaded from its frame slot (into
        // v0, v1, v0, v1).
        for (i, slot, ptr_reg) in [
            (4usize, 0x18u32, V0),
            (5, 0x1c, V1),
            (6, 0x20, V0),
            (7, 0x24, V1),
        ] {
            c.cycles += 2;
            let present = c.g[V0][0] != 0;
            c.g[ptr_reg][0] = c.lw(s.wrapping_add(slot));
            if present {
                c.cycles += 8;
                c.g[S2][0] = sx(s.wrapping_add(0x10 + i as u32));
                c.g[A1][0] = c.g[S5][0];
                c.g[S1][0] = c.lw(r32(c, ptr_reg).wrapping_add(list::CELL_ROOT));
                c.g[A0][0] = c.g[ptr_reg][0];
                call_ptr(c, TYPE2_CLASSIFY, TYPE2_CELLS[i].classify_ret)?;
                gather_type2_query_cell(c, i, S2, Type2CellPtr::Slot(slot))?;
            } else if i < 7 {
                c.cycles += 1;
                gather_type2_next_root(c, i);
            }
        }

        // Invalidate the cached counts; epilogue.
        c.cycles += 16;
        c.g[V0][0] = u64::MAX;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        let this = r32(c, FP);
        c.sw(this.wrapping_add(list::NTRIS), c.g[V0][0]);
        c.sw(this.wrapping_add(list::COUNT), c.g[V0][0]);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore(c, FP, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// End of cell query `i < 7`: load the next cell's tree root into v0.
fn gather_type2_next_root(c: &mut Cpu, i: usize) {
    use gather_type2_list as list;
    let next = list::CELL_SIZE * (i as u32 + 1) + list::CELL_ROOT;
    c.g[V0][0] = c.lw(r32(c, S4).wrapping_add(next));
}

/// Inlined `queryCell` `i` after `SpatialKey_ClassifyAABB` returned (v0):
/// s1 holds the cell's tree root, `tag` the scratch pointer passed as
/// `Cell_GatherType2Recursive`'s unused first argument, `cell` the cell.
/// Every copy but the last ends with the next cell's root in v0.
fn gather_type2_query_cell(
    c: &mut Cpu,
    i: usize,
    tag: usize,
    cell: Type2CellPtr,
) -> crate::call::Flow {
    use gather_type2_list as list;
    let q = &TYPE2_CELLS[i];
    let last = i == 7;
    c.g[V1][0] = c.g[V0][0];
    let class = c.g[V1][0];
    c.g[V0][0] = 0x2;
    if class != 0 {
        // Partly overlapping: recurse; disjoint: skip.
        c.cycles += 2;
        if last {
            c.g[A1][0] = c.g[S0][0];
        }
        if c.g[V1][0] != c.g[V0][0] {
            if !last {
                gather_type2_next_root(c, i);
            }
            return Ok(());
        }
        c.cycles += if last { 6 } else { 7 };
        c.g[T1][0] = match cell {
            Type2CellPtr::Reg(r) => c.g[r][0],
            Type2CellPtr::Slot(off) => c.lw(sp(c).wrapping_add(off)),
        };
        c.g[A0][0] = c.g[tag][0];
        c.g[T0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S5][0];
        TYPE2_RECURSIVE.call(c, q.recursive_ret)?;
        if !last {
            c.cycles += 1;
            gather_type2_next_root(c, i);
        }
        return Ok(());
    }

    // The box lies within the cell: append the root's type-2 list...
    c.cycles += 3;
    let tree = r32(c, S1);
    let ctx = r32(c, S0);
    c.g[A2][0] = c.lw(tree.wrapping_add(list::NODE_TYPE2));
    if c.g[A2][0] != 0 {
        c.cycles += q.list_cycles;
        c.g[A1][0] = c.lw(ctx.wrapping_add(4));
        loop {
            // ctx.list[(*ctx.count)++] = item
            c.cycles += 10;
            c.g[A0][0] = c.lw(ctx);
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
            c.g[A1][0] = c.lw(ctx.wrapping_add(4));
            if event_due(c) {
                deliver_event_here(c, q.list_loop);
            }
        }
    }
    c.g[A0][0] = c.lw(tree);

    // ...and gather its eight children whole.
    for k in 0..8u32 {
        let tree = r32(c, S1);
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += if k == 7 { 4 } else { 3 };
            c.g[A1][0] = c.g[S0][0];
            TYPE2_SUBTREE.call(c, q.list_loop + 0x3c + 0x14 * k)?;
        }
        if k < 7 {
            c.g[A0][0] = c.lw(tree.wrapping_add(4 * (k + 1)));
        } else if !last {
            gather_type2_next_root(c, i);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `OverlapList_GatherType2` against the generated body, with the real
    /// callees (`SpatialKey_ClassifyAABB`, `Cell_GatherType2Subtree`,
    /// `Cell_GatherType2Recursive`) on short paths. Every seed gives each of
    /// the eight top-level cells a random fate: no tree, box inside the cell
    /// (append the root's type-2 list, gather its children whole), partly
    /// overlapping (recurse) or disjoint (skip).
    #[test]
    fn overlap_list_gather_type2_matches() {
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        use gather_type2_list as list;
        let fbits = |x: f32| x.to_bits();
        check(
            ee::s11::f_00331450__generated,
            overlap_list_gather_type2,
            200,
            &|c, seed| {
                let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ 0x1450);
                scramble(c, &mut r);
                c.pc = 0x0033_1450;
                c.g[GP][0] = GAME_GP as u64;
                let gp_at = |off: i32| (GAME_GP as i32 + off) as u32;
                // The BELibrary dummy and the world (callees' globals).
                let belib = DATA + 0x3_0000;
                let world = DATA + 0x3_1000;
                c.bus.write32(gp_at(-0x1f04), belib);
                c.bus.write32(gp_at(-0x848), world);
                c.bus.write32(world + 0x84, world + 0x100);
                c.bus.write32(world + 0x100 + 0x7c, 0);
                // SpatialKey_ClassifyAABB's cell margin.
                c.bus.write32(gp_at(-0x2dfc), fbits(0.25));

                let this = DATA;
                let grid = DATA + 0x1000;
                let bx = DATA + 0x1200;
                // Leaf children: one with no list, one with a one-item list.
                let leaf = DATA + 0x3000;
                let leaf_item = DATA + 0x3100;
                let leaf_list = DATA + 0x3200;
                c.bus.write32(leaf_item + list::NODE_TYPE2, leaf_list);
                c.bus.write32(leaf_list, 0);
                // Stale count and cached counts (reset / invalidated).
                c.bus.write32(this + list::ITEM_COUNT, r.next_u32());
                c.bus.write32(this + list::NTRIS, r.next_u32());
                c.bus.write32(this + list::COUNT, r.next_u32());
                c.bus.write32(this + list::GRID, grid);
                c.bus.write32(grid + list::GRID_STAMP, r.next_u32());
                // The query box (as in `overlap_list_gather_all_matches`): a
                // level-0 cell at the origin classifies 0 ("within"), a
                // level-1 cell at the origin 2 (partly), one at x = 100 1.
                for (k, v) in [1.25f32, 1.25, 0.0].into_iter().enumerate() {
                    c.bus.write32(bx + 4 * k as u32, fbits(v));
                    c.bus.write32(bx + 0x10 + 4 * k as u32, fbits(0.5));
                }
                for i in 0..8u32 {
                    let cell = grid + list::CELL_SIZE * i;
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
                    c.bus
                        .write32(cell + list::CELL_ROOT, if present { root } else { 0 });
                    // The root: 0..3 type-2 items; random leaf children.
                    for off in [0x20u32, 0x24] {
                        c.bus.write32(root + off, 0);
                    }
                    let mut next = 0;
                    for j in 0..r.next_u32() % 4 {
                        let item = DATA + 0x7000 + 0x10 * (4 * i + j);
                        c.bus.write32(item, next);
                        next = item;
                    }
                    c.bus.write32(root + list::NODE_TYPE2, next);
                    for k in 0..8u32 {
                        let child = match r.next_u32() % 4 {
                            0 => leaf,
                            1 => leaf_item,
                            _ => 0,
                        };
                        c.bus.write32(root + 4 * k, child);
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(bx);
            },
        );
    }
}
