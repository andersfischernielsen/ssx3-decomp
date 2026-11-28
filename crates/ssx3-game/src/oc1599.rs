//! Staging module oc1599 (wave wave280): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1599`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002c_1a00,
    quad_tree_insert_recursive_oc1599,
    "QuadTree_InsertRecursive",
)];

const OC1599_ALLOC_NODE: Callee = Callee {
    addr: 0x002c_17d0,
    f: ee::s08::QuadTree_AllocNode,
};
const OC1599_SET_CHILDREN: Callee = Callee {
    addr: 0x002c_16b8,
    f: ee::s08::QuadTreeNode_SetChildren,
};
const OC1599_SET_LEAF: Callee = Callee {
    addr: 0x002c_16a8,
    f: ee::s08::QuadTreeNode_SetLeaf,
};
const OC1599_FREE_CHILDREN: Callee = Callee {
    addr: 0x002c_18e8,
    f: ee::s08::QuadTree_FreeChildren,
};
/// The recursion goes through the generated alias (the override table).
const OC1599_INSERT_RECURSIVE: Callee = Callee {
    addr: 0x002c_1a00,
    f: ee::s08::QuadTree_InsertRecursive,
};

/// `void QuadTree_InsertRecursive(QuadTree* tree, QuadTreeNode* node, u16 x, u16 z,
/// int size, a, b)` (0x2c1a00): set the cell `(x, z)` of the quad tree, below `node`
/// and `size` cells wide, to the leaf value `(a, b)`. A leaf node that is larger than
/// one cell is first split into four children (allocated with `QuadTree_AllocNode`,
/// all holding the old leaf value), the insert recurses into the child that holds
/// the point, and afterwards four equal children are merged back into a leaf
/// (`QuadTree_FreeChildren`, `QuadTreeNode_SetLeaf`). At size < 2 the cell is set
/// directly (freeing its children first if it had any) and the merge check follows.
/// The node's low half-word bit 0 marks an internal node; children are 8-byte
/// node indices in the half-words (`>> 1`) relative to the array at `tree+0x20`.
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut)]
pub fn quad_tree_insert_recursive_oc1599(c: &mut Cpu) {
    guest(c, |c| {
'b_2c1bc0: {
'b_2c1b80: {
c.cycles += 23;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
c.g[A2][0] = c.g[A2][0] & 0xffff;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[A3][0] = c.g[A3][0] & 0xffff;
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S1][0] = c.g[A1][0];
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S2][0] = c.g[A0][0];
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S6][0] = c.g[T1][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[S7][0] = c.g[T2][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[V0][0] = ((c.g[T0][0] as i64) < 2i64) as u64;
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32) >> 3);
let t = c.g[V0][0] != 0u64;
c.g[V1][0] = c.g[V1][0] & 0xffff;
if t {
break 'b_2c1b80;
} else {
'b_2c1b24: {
c.cycles += 14;
c.g[FP][0] = sx((c.g[V1][0] as u32) << 3);
c.g[V0][0] = c.lhu((c.g[S1][0] as u32));
c.g[V1][0] = sx((c.g[A2][0] as u32) << 1);
c.g[A0][0] = sx((c.g[A3][0] as u32) << 1);
c.g[T0][0] = sx(((c.g[T0][0] as u32) as i32 >> 1) as u32);
c.g[A2][0] = sx((c.g[A2][0] as u32) >> 15);
c.g[A3][0] = sx((c.g[A3][0] as u32) >> 15);
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[A0][0]);
c.g[V0][0] = c.g[V0][0] & 0x1;
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[T0][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
let t = c.g[V0][0] != 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A3][0]);
if t {
break 'b_2c1b24;
} else {
c.cycles += 25;
c.g[S5][0] = c.lw((c.g[S1][0] as u32));
c.g[A0][0] = c.g[S2][0];
c.g[S4][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[RA][0] = sx(0x002c1aa8);
c.g[S3][0] = 0x3;
OC1599_ALLOC_NODE.call(c, 0x002c1aa8)?;
c.sh((c.g[SP][0] as u32), c.g[V0][0]);
c.g[RA][0] = sx(0x002c1ab4);
c.g[A0][0] = c.g[S2][0];
OC1599_ALLOC_NODE.call(c, 0x002c1ab4)?;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x002c1ac0);
c.sh((c.g[SP][0] as u32).wrapping_add(0x2), c.g[V0][0]);
OC1599_ALLOC_NODE.call(c, 0x002c1ac0)?;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x002c1acc);
c.sh((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
OC1599_ALLOC_NODE.call(c, 0x002c1acc)?;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
c.sh((c.g[SP][0] as u32).wrapping_add(0x6), c.g[V0][0]);
c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[FP][0] as u32)));
c.g[A2][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x2));
c.g[A3][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
c.g[A1][0] = c.lhu((c.g[SP][0] as u32));
c.g[T0][0] = c.g[V0][0] & 0xffff;
c.g[RA][0] = sx(0x002c1af4);
c.g[S0][0] = c.g[S1][0];
OC1599_SET_CHILDREN.call(c, 0x002c1af4)?;
c.g[A0][0] = c.lhu((c.g[S0][0] as u32));
'l_2c1af8: loop {
c.cycles += 11;
c.g[A1][0] = c.g[S4][0];
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
c.g[A2][0] = c.g[S5][0];
c.g[A0][0] = sx((c.g[A0][0] as u32) >> 1);
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
c.g[RA][0] = sx(0x002c1b1c);
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
OC1599_SET_LEAF.call(c, 0x002c1b1c)?;
if (c.g[S3][0] as i64) >= 0 {
c.g[A0][0] = c.lhu((c.g[S0][0] as u32));
if event_due(c) { deliver_event_here(c, 0x002c1af8); } continue 'l_2c1af8;
} else {
break 'b_2c1b24;
}
}
}
}
c.cycles += 23;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
c.g[T1][0] = c.g[S6][0];
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[T2][0] = c.g[S7][0];
c.g[V0][0] = sx((c.g[V1][0] as u32) << 1);
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A1][0] = c.lhu((c.g[V0][0] as u32));
c.g[A2][0] = c.g[A0][0] & 0xfffe;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
c.g[A1][0] = sx((c.g[A1][0] as u32) >> 1);
c.g[A3][0] = c.g[A0][0] & 0xfffe;
c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x002c1b74);
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
OC1599_INSERT_RECURSIVE.call(c, 0x002c1b74)?;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
let t = true;
c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[FP][0] as u32)));
if t {
break 'b_2c1bc0;
} else {
break 'b_2c1b80;
}
}
}
'b_2c1bb0: {
c.cycles += 4;
c.g[A1][0] = c.lhu((c.g[S1][0] as u32));
c.g[V0][0] = c.g[A1][0] & 0x1;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = sx((c.g[A1][0] as u32) >> 1);
if t {
break 'b_2c1bb0;
} else {
c.cycles += 8;
c.g[A2][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x2));
c.g[A3][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S2][0];
c.g[T0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x6));
c.g[A2][0] = sx((c.g[A2][0] as u32) >> 1);
c.g[A3][0] = sx((c.g[A3][0] as u32) >> 1);
c.g[RA][0] = sx(0x002c1bb0);
c.g[T0][0] = sx((c.g[T0][0] as u32) >> 1);
OC1599_FREE_CHILDREN.call(c, 0x002c1bb0)?;
break 'b_2c1bb0;
}
}
c.cycles += 4;
c.g[A0][0] = c.g[S1][0];
c.g[A1][0] = c.g[S6][0];
c.g[RA][0] = sx(0x002c1bc0);
c.g[A2][0] = c.g[S7][0];
OC1599_SET_LEAF.call(c, 0x002c1bc0)?;
break 'b_2c1bc0;
}
'b_2c1ca8: {
'b_2c1ca4: {
c.cycles += 4;
c.g[V1][0] = c.lhu((c.g[S1][0] as u32));
c.g[V0][0] = c.g[V1][0] & 0x1;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = sx((c.g[V1][0] as u32) >> 1);
if t {
break 'b_2c1ca4;
} else {
'b_2c1c0c: {
c.cycles += 11;
c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x2));
c.g[T1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
c.g[A2][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = sx((c.g[A2][0] as u32) << 3);
c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
let t = c.g[V1][0] != c.g[V0][0];
c.g[T0][0] = 0u64;
if t {
break 'b_2c1c0c;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V1][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
c.g[T0][0] = (c.g[V0][0] < 0x1u64) as u64;
break 'b_2c1c0c;
}
}
c.cycles += 2;
let t = c.g[T0][0] == 0u64;
c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
if t {
break 'b_2c1ca4;
} else {
'b_2c1c48: {
c.cycles += 9;
c.g[V1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[A3][0] = sx((c.g[V1][0] as u32) >> 1);
c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
let t = c.g[A0][0] != c.g[V1][0];
c.g[T3][0] = 0u64;
if t {
break 'b_2c1c48;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[T2][0] as u32));
c.g[V1][0] = c.lw((c.g[T0][0] as u32));
c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
c.g[T3][0] = (c.g[V0][0] < 0x1u64) as u64;
break 'b_2c1c48;
}
}
c.cycles += 2;
let t = c.g[T3][0] == 0u64;
c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
if t {
break 'b_2c1ca4;
} else {
'b_2c1c84: {
c.cycles += 9;
c.g[V1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x6));
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[T0][0] = sx((c.g[V1][0] as u32) >> 1);
c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[T0][0] as u32) << 3);
c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
let t = c.g[A0][0] != c.g[V1][0];
c.g[T3][0] = 0u64;
if t {
break 'b_2c1c84;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[T2][0] as u32));
c.g[V1][0] = c.lw((c.g[T1][0] as u32));
c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
c.g[T3][0] = (c.g[V0][0] < 0x1u64) as u64;
break 'b_2c1c84;
}
}
c.cycles += 2;
let t = c.g[T3][0] == 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
if t {
break 'b_2c1ca8;
} else {
c.cycles += 6;
c.g[RA][0] = sx(0x002c1c94);
c.g[A0][0] = c.g[S2][0];
OC1599_FREE_CHILDREN.call(c, 0x002c1c94)?;
c.g[A0][0] = c.g[S1][0];
c.g[A1][0] = c.g[S6][0];
c.g[RA][0] = sx(0x002c1ca4);
c.g[A2][0] = c.g[S7][0];
OC1599_SET_LEAF.call(c, 0x002c1ca4)?;
break 'b_2c1ca4;
}
}
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
break 'b_2c1ca8;
}
c.cycles += 11;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
c.pc = t; return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check_paths, DATA};

    /// Quad tree at `DATA+0x1000`; its node array (8-byte nodes) at `DATA+0x10000`
    /// with a free list of 192 nodes from index 0x40 (so the real `AllocNode`,
    /// `SetChildren`, `SetLeaf` and `FreeChildren` all take their short paths).
    /// Root node index 1: a leaf, or an internal node with four leaf children
    /// whose values are all equal (merge) or drawn from two values.
    #[test]
    fn quad_tree_insert_recursive_matches() {
        const TREE: u32 = DATA + 0x1000;
        const BASE: u32 = DATA + 0x1_0000;
        check_paths(
            ee::s08::f_002c1a00__generated,
            quad_tree_insert_recursive_oc1599,
            400,
            &[],
            &|c, seed| {
                let mut r = machine(c, seed, 0x1599_0001, 0);
                c.bus.write32(TREE + 0x20, BASE);
                c.bus.write32(TREE + 0x24, BASE + 0x40 * 8);
                c.bus.write32(TREE + 0xc, seed % 9);
                c.bus.write32(TREE + 0x10, 0x400);
                for i in 0x40..0x100u32 {
                    c.bus.write16(BASE + i * 8, (i + 1) as u16);
                }
                let node = |i: u32| BASE + i * 8;
                let x_old = r.next_u32() & !1;
                let y_old = r.next_u32();
                let same = seed % 3 == 0;
                match seed % 4 {
                    // Leaf root.
                    0 | 2 => {
                        c.bus.write32(node(1), x_old);
                        c.bus.write32(node(1) + 4, y_old);
                    }
                    // Internal root, four leaf children.
                    _ => {
                        c.bus.write16(node(1), (2 << 1) | 1);
                        c.bus.write16(node(1) + 2, 3 << 1);
                        c.bus.write16(node(1) + 4, 4 << 1);
                        c.bus.write16(node(1) + 6, 5 << 1);
                        for k in 0..4u32 {
                            let alt = !same && (seed / 4 >> k) & 1 == 1;
                            c.bus.write32(node(2 + k), if alt { x_old ^ 0x10 } else { x_old });
                            c.bus.write32(node(2 + k) + 4, y_old);
                        }
                    }
                }
                // New value: sometimes the old one (merges), sometimes odd (an
                // "internal" marker over garbage children).
                let (mut new_b, mut new_a) = match seed % 5 {
                    0 | 1 => (x_old, y_old),
                    _ => (r.next_u32() & !1, r.next_u32()),
                };
                if seed % 13 == 0 {
                    new_b = (new_b & 0xffff) | 1;
                    new_a &= 0xffff;
                }
                let sizes = [0u32, 1, 2, 3, 4, 8, 16];
                c.g[A0][0] = sx(TREE);
                c.g[A1][0] = sx(node(1));
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = sx(sizes[(seed as usize / 2) % 7]);
                c.g[T1][0] = sx(new_a);
                c.g[T2][0] = sx(new_b);
            },
        );
    }
}
