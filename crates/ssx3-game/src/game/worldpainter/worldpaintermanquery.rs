//! `src/worldpainter/worldpaintermanquery.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::events;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
pub(crate) use crate::game::visualfx::worldlightman::DELETE;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002c_1460,
        world_painter_query_base_dtor,
        "WorldPainterQuery_BaseDtor",
    ),
    (0x002c_16a8, quad_tree_node_set_leaf, "QuadTreeNode_SetLeaf"),
    (
        0x002c_16b8,
        quad_tree_node_set_children,
        "QuadTreeNode_SetChildren",
    ),
    (
        0x002c_17c0,
        quad_tree_init_free_list,
        "QuadTree_InitFreeList",
    ),
    (
        0x002c_18e8,
        quad_tree_free_children,
        "QuadTree_FreeChildren",
    ),
];

// ---------------------------------------------------------------------------
// Quad tree.

/// `void QuadTreeNode_SetLeaf(QuadTreeNode* n, uint a, uint b)` (0x2c16a8):
/// store `b` at +0 and `a` at +4.
pub fn quad_tree_node_set_leaf(c: &mut Cpu) {
    c.cycles += 3;
    let n = Addr(abi::arg(c, 0));
    n.sw(c, c.g[A2][0]);
    n.off(0x4).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void QuadTreeNode_SetChildren(QuadTreeNode* n, uint c0, uint c1, uint c2,
/// uint c3)` (0x2c16b8): update the node's low half-words from the child
/// indices (each doubled, low bit from the node).
pub fn quad_tree_node_set_children(c: &mut Cpu) {
    c.cycles += 23;
    let n = Addr(abi::arg(c, 0));
    c.g[T1][0] = n.off(0x2).lhu(c);
    c.g[A1][0] &= 0xffff;
    c.g[V1][0] = n.off(0x4).lhu(c);
    c.g[A2][0] &= 0xffff;
    c.g[V0][0] = n.off(0x6).lhu(c);
    c.g[A3][0] &= 0xffff;
    c.g[T0][0] &= 0xffff;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
    c.g[T1][0] &= 0x1;
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 1);
    c.g[V1][0] &= 0x1;
    c.g[T0][0] = sx((c.g[T0][0] as u32) << 1);
    c.g[V0][0] &= 0x1;
    c.g[A2][0] |= c.g[T1][0];
    c.g[A3][0] |= c.g[V1][0];
    c.g[T0][0] |= c.g[V0][0];
    c.g[A1][0] |= 0x1;
    n.off(0x6).sh(c, c.g[T0][0]);
    n.sh(c, c.g[A1][0]);
    n.off(0x2).sh(c, c.g[A2][0]);
    n.off(0x4).sh(c, c.g[A3][0]);
    abi::ret(c);
}

/// `void QuadTree_InitFreeList(QuadTree* q)` (0x2c17c0): point the free-list
/// head and tail at the embedded list node at +0x28.
pub fn quad_tree_init_free_list(c: &mut Cpu) {
    c.cycles += 4;
    let q = Addr(abi::arg(c, 0));
    c.g[V0][0] = sx(q.0.wrapping_add(0x28));
    q.off(0x24).sw(c, c.g[V0][0]);
    q.off(0x20).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void QuadTree_FreeChildren(QuadTree* q, uint i0, uint i1, uint i2, uint
/// i3)` (0x2c18e8): splice the free-list node at +0x20, unlink the four child
/// entries and decrement the count at +0xc.
pub fn quad_tree_free_children(c: &mut Cpu) {
    c.cycles += 25;
    let q = Addr(abi::arg(c, 0));
    c.g[T1][0] = q.off(0x20).lw(c);
    c.g[A1][0] &= 0xffff;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
    c.g[A2][0] &= 0xffff;
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A3][0] &= 0xffff;
    Addr(c.g[V0][0] as u32).sh(c, c.g[A2][0]);
    c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
    Addr(c.g[V1][0] as u32).sh(c, c.g[A3][0]);
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[T0][0] &= 0xffff;
    Addr(c.g[V0][0] as u32).sh(c, c.g[T0][0]);
    c.g[T0][0] = sx((c.g[T0][0] as u32) << 3);
    c.g[V0][0] = q.off(0x24).lw(c);
    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
    Addr(c.g[T1][0] as u32).sh(c, c.g[V1][0]);
    c.g[A2][0] = q.off(0x24).lw(c);
    Addr(c.g[A2][0] as u32).sh(c, c.g[A1][0]);
    c.g[V0][0] = q.off(0xc).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    q.off(0xc).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `WorldPainterQuery_BaseDtor` (0x2c1460): vtable at `+0x4`.
pub fn world_painter_query_base_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x4, 0x0048_5d30, 0x002c_1484);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::visualfx::worldlightman::tests::alloc_setup;
    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;
    pub(crate) use crate::game::visualfx::worldlightman::tests::MEM_MUTEX;
    use ssx3_ee as ee;

    #[test]
    fn quad_tree_node_set_leaf_matches() {
        check(
            ssx3_ee::s08::f_002c16a8,
            quad_tree_node_set_leaf,
            50,
            &|c, seed| {
                let mut r = Rng(0x16a8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn quad_tree_node_set_children_matches() {
        check(
            ssx3_ee::s08::f_002c16b8,
            quad_tree_node_set_children,
            200,
            &|c, seed| {
                let mut r = Rng(0x16b8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for (reg, v) in [
                    (A1, r.next_u32()),
                    (A2, r.next_u32()),
                    (A3, r.next_u32()),
                    (T0, r.next_u32()),
                ] {
                    c.g[reg][0] = ((v as u64) << 32) | v as u64;
                }
                for k in 0..4u32 {
                    c.bus.write16(DATA + 2 * k, r.next_u32() as u16);
                }
            },
        );
    }

    #[test]
    fn quad_tree_init_free_list_matches() {
        check(
            ssx3_ee::s08::f_002c17c0,
            quad_tree_init_free_list,
            50,
            &|c, seed| {
                let mut r = Rng(0x17c0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + (seed % 8)) as u64;
            },
        );
    }

    #[test]
    fn quad_tree_free_children_matches() {
        check(
            ssx3_ee::s08::f_002c18e8,
            quad_tree_free_children,
            200,
            &|c, seed| {
                let mut r = Rng(0x18e8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for reg in [A1, A2, A3, T0] {
                    c.g[reg][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                }
                c.bus.write32(DATA + 0x20, DATA + 0x400);
                c.bus.write32(DATA + 0x24, DATA + 0x600);
                c.bus.write32(DATA + 0xc, r.next_u32());
                c.bus.write32(DATA + 0x400, r.next_u32());
                c.bus.write32(DATA + 0x600, r.next_u32() % 8);
            },
        );
    }

    // ------------------------------------------------------- menu

    #[test]
    fn world_painter_query_base_dtor_matches() {
        dtor_case(
            ee::s08::f_002c1460__generated,
            world_painter_query_base_dtor,
            0x2c1460,
        );
    }
}
