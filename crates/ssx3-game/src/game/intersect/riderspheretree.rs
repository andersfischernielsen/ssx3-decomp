//! `src/intersect/riderspheretree.cpp`: hand-written replacements (moved from the
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
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::fr_2 as fr;
pub(crate) use crate::game::ai::computer::gp_2 as gp_3;
pub(crate) use crate::game::ai::computer::ldf_2 as ldf;
pub(crate) use crate::game::ai::computer::lt_2 as lt;
pub(crate) use crate::game::ai::computer::lw;
pub(crate) use crate::game::ai::control::handplantcontrol::gp_2;
pub(crate) use crate::game::ai::rider::at;
pub(crate) use crate::game::ai::rider::epilogue;
pub(crate) use crate::game::ai::rider::gp_2 as gp;
pub(crate) use crate::game::ai::rider::p;
pub(crate) use crate::game::ai::rider::prologue;
pub(crate) use crate::game::ai::rider::r;
pub(crate) use crate::game::ai::rider::slt;
pub(crate) use crate::game::ai::rider::sp_2;
pub(crate) use crate::game::ai::rider::vcall;
pub(crate) use crate::game::ai::rider::Ops;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::fe::ovstates::at as at_4;
pub(crate) use crate::game::fe::ovstates::open;
pub(crate) use crate::game::intersect::worldsphtree::sp_2 as sp_3;
pub(crate) use crate::game::object::flexrailnode::at_3 as at_5;
pub(crate) use crate::game::object::flexrailnode::r32v;
pub(crate) use crate::game::object::flexrailnode::spa;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p_3;
pub(crate) use crate::game::world::worldview::p_2 as p_4;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

use crate::abi::addu;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_reg;
use crate::call;
/// Replacements in this unit: (address, function, name).
pub(crate) use crate::game::fe::ovstateprofile::ov_screen_set_widget_visibility;
pub(crate) use crate::game::fe::splashmanager::oc230_gp;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_9970, rider_sphere_tree_dtor, "RiderSphereTree_dtor"),
    (0x0032_99c8, rider_sphere_tree_init, "RiderSphereTree_Init"),
    (
        0x0032_9a90,
        rider_sphere_tree_set_spheres,
        "RiderSphereTree_SetSpheres",
    ),
    (
        0x0032_9dc8,
        rider_sphere_tree_get_bounds,
        "RiderSphereTree_GetBounds",
    ),
    (
        0x0032_9f98,
        rider_sphere_tree_intersect,
        "RiderSphereTree_Intersect",
    ),
    (0x0032_b6a8, vu0_triangle_intersect, "VU0_TriangleIntersect"),
    (
        0x0032_b6e0,
        ai_fwd_diff_cache_collide_entry,
        "AIFwdDiffCache_CollideEntry",
    ),
    (0x0032_c508, sphere_tree_ctor, "SphereTree_ctor"),
    (
        0x0032_c540,
        sphere_tree_reset_transform,
        "SphereTree_ResetTransform",
    ),
    (
        0x0032_c5a8,
        sphere_tree_get_half_leaf_radius,
        "SphereTree_GetHalfLeafRadius",
    ),
    (
        0x0032_c898,
        sphere_tree_collide_sphere,
        "SphereTree_CollideSphere",
    ),
    (
        0x0032_c928,
        sphere_tree_collide_aabb,
        "SphereTree_CollideAABB",
    ),
    (
        0x0032_ca78,
        spheretree_collide_triangle,
        "SphereTree_CollideTriangle",
    ),
    (
        0x0032_cb58,
        sphere_tree_collide_sphere_tree,
        "SphereTree_CollideSphereTree",
    ),
    (0x0032_da40, shell_sort_desc_key_val, "ShellSortDesc_KeyVal"),
    (0x0032_e100, coll_segment_ctor, "CollSegment_ctor"),
    (
        0x0032_e288,
        coll_segment_overlaps_aabb,
        "CollSegment_OverlapsAABB",
    ),
    (
        0x0032_e4b8,
        coll_segment_reset_transform,
        "CollSegment_ResetTransform",
    ),
    (
        0x0032_e4d0,
        coll_segment_collide_triangle,
        "CollSegment_CollideTriangle",
    ),
    (
        0x0032_e690,
        coll_segment_collide_aabb,
        "CollSegment_CollideAABB",
    ),
    (0x0032_f650, coll_hull_ctor, "CollHull_ctor"),
    (0x0032_f708, coll_hull_dtor, "CollHull_dtor"),
    (0x0032_f8c0, coll_hull_get_diameter, "CollHull_GetDiameter"),
    (
        0x0032_f990,
        coll_hull_collide_triangle2_sided,
        "CollHull_CollideTriangle2Sided",
    ),
    (
        0x0032_fa30,
        coll_hull_collide_sphere_tree,
        "CollHull_CollideSphereTree",
    ),
    (0x0032_fb48, coll_sphere_ctor, "CollSphere_ctor"),
    (
        0x0032_fc08,
        coll_sphere_overlaps_aabb,
        "CollSphere_OverlapsAABB",
    ),
    (0x0032_fd40, coll_sphere_transform, "CollSphere_Transform"),
    (
        0x0032_fd98,
        coll_sphere_reset_transform,
        "CollSphere_ResetTransform",
    ),
    (
        0x0032_fdb0,
        coll_sphere_collide_sphere_tree,
        "CollSphere_CollideSphereTree",
    ),
    (
        0x0032_fe40,
        coll_sphere_collide_aabb,
        "CollSphere_CollideAABB",
    ),
    (
        0x0033_0250,
        coll_sphere_hits_triangle_edges,
        "CollSphere_HitsTriangleEdges",
    ),
    (
        0x0033_0360,
        coll_sphere_plane_distance,
        "CollSphere_PlaneDistance",
    ),
    (
        0x0032_a1c0,
        rider_sphere_tree_collide_triangle,
        "RiderSphereTree_CollideTriangle",
    ),
    (
        0x0032_aa28,
        rider_sphere_tree_collide_triangle2,
        "RiderSphereTree_CollideTriangle2",
    ),
    (
        0x0032_9b90,
        rider_sphere_tree_scale,
        "RiderSphereTree_Scale",
    ),
    (
        0x0032_b2b8,
        rider_sphere_tree_collide_box,
        "RiderSphereTree_CollideBox",
    ),
    (
        0x0032_c0f8,
        collide_mesh_leaf_triangles,
        "Collide_MeshLeafTriangles",
    ),
    (
        0x0032_f8f0,
        coll_hull_collide_triangle,
        "CollHull_CollideTriangle",
    ),
    (
        0x0032_9a28,
        rider_sphere_tree_assign,
        "RiderSphereTree_Assign",
    ),
    (
        0x0032_f760,
        coll_hull_overlaps_aabb,
        "CollHull_OverlapsAABB",
    ),
    (0x0032_f840, coll_hull_transform, "CollHull_Transform"),
    (0x0032_fac0, coll_hull_collide_aabb, "CollHull_CollideAABB"),
    (
        0x0032_9b40,
        rider_sphere_tree_translate,
        "RiderSphereTree_Translate",
    ),
    (
        0x0032_dc10,
        world_init_sphere_tree_cache,
        "World_InitSphereTreeCache",
    ),
    (
        0x0032_9938,
        rider_sphere_tree_ctor_copy,
        "RiderSphereTree_ctor_copy",
    ),
    (0x0032_b468, rle_encode, "RLE_Encode"),
    (0x0032_b620, rle_decode, "RLE_Decode"),
    (
        0x0032_c648,
        sphere_tree_set_transform,
        "SphereTree_SetTransform",
    ),
    (
        0x0032_c770,
        sphere_tree_apply_transform,
        "SphereTree_ApplyTransform",
    ),
    (
        0x0032_cdb0,
        sphere_tree_collide_sphere_node,
        "SphereTree_CollideSphereNode",
    ),
    (
        0x0032_d028,
        sphere_tree_collide_aabb_node,
        "SphereTree_CollideAABBNode",
    ),
    (
        0x0032_d208,
        sphere_tree_collide_plane_node,
        "SphereTree_CollidePlaneNode",
    ),
    (
        0x0032_d440,
        sphere_tree_point_in_triangle,
        "SphereTree_PointInTriangle",
    ),
    (
        0x0032_d470,
        sphere_tree_collide_triangle_node,
        "SphereTree_CollideTriangleNode",
    ),
    (
        0x0032_de20,
        world_free_sphere_tree_cache,
        "World_FreeSphereTreeCache",
    ),
    (
        0x0032_df28,
        sphere_tree_cache_get_masks,
        "SphereTreeCache_GetMasks",
    ),
    (
        0x0032_e5e8,
        coll_segment_collide_triangle_2sided,
        "CollSegment_CollideTriangle2Sided",
    ),
    (
        0x0032_fe78,
        coll_sphere_collide_triangle,
        "CollSphere_CollideTriangle",
    ),
    (
        0x0032_ffa0,
        coll_sphere_collide_triangle_2sided,
        "CollSphere_CollideTriangle2Sided",
    ),
    (0x0033_00f8, vec3_neg_dot, "Vec3_NegDot"),
    (
        0x0033_0128,
        closest_point_on_segment,
        "ClosestPointOnSegment",
    ),
    (
        0x0032_cbf8,
        sphere_tree_collide_tree_node,
        "SphereTree_CollideTreeNode",
    ),
    (
        0x0032_db40,
        sphere_tree_sort_child_order_2,
        "SphereTree_SortChildOrder",
    ),
];

// ---------------------------------------------------------------------------
// Sphere tree (`sSphereBox_9DC8`, 0x20-byte nodes).

pub(crate) mod sphere_tree {

    pub const CENTER: u32 = 0x10;

    pub const RADIUS: u32 = 0x20;

    pub const COUNT: u32 = 0x2c;

    pub const CHILD_CENTER: u32 = 0x30;

    pub const CHILD_RADIUS: u32 = 0x40;

    pub const CHILD_SIZE: u32 = 0x20;

    /// Bit `i` selects child `i`; 0 means a leaf.
    pub const MASK: u32 = 0x28;

    /// Child `i`'s fields are read from the slot base `tree + i * 0x20`: the
    /// position quadword at `+0x30`, the radius one quadword further at
    /// `+0x40`.
    pub const KID_POS: u32 = 0x30;

    pub const KID_RADIUS: u32 = 0x40;

    pub const KID_STRIDE: u32 = 0x20;
}

/// A cache cell (0x40 bytes): two triangle normals (`+0`, `+0x10`; x =
/// the "not computed" sentinel `gp-0x2d94`, or the "degenerate" value
/// `gp-0x2d90`) and the cell's box (`+0x20` min, `+0x30` max).
pub(crate) mod cell {

    pub const NORMAL1: u32 = 0x10;

    pub const BOX_MIN: u32 = 0x20;

    pub const BOX_MAX: u32 = 0x30;

    pub const SIZE: u32 = 0x40;
}

/// The hint (`t3`): `{key, u16 row, u16 col, first_triangle, kind}`.
pub(crate) mod hint {

    pub const KEY: u32 = 0x0;

    pub const ROW: u32 = 0x4;

    pub const COL: u32 = 0x6;

    /// Non-zero: the cell's first triangle (else the second).
    pub const FIRST: u32 = 0x8;

    pub const KIND: u32 = 0xc;
}

/// The volume (`a0`): kind (`+0`), facing test (`+8`) with its direction
/// (`+0x10`), box (`+0x20` max, `+0x30` min... compared as the original),
/// vtable (`+0x50`: 0x20 triangle test, 0x40 UV refine).
pub(crate) mod volume {

    pub const KIND: u32 = 0x0;

    pub const FACING: u32 = 0x8;

    pub const DIR: u32 = 0x10;

    pub const VTABLE: u32 = 0x50;
}

/// Direct-call targets (dossier callees, found by address in `generated/ee`).
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::bedbprofile::callee::CLOSEST_POINT_ON_SEGMENT;

    pub(crate) use crate::game::camera::camera::callee::SPHERE_TREE_RESET_TRANSFORM;

    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_TREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;

    pub(crate) use crate::game::render::bezierman::callee::RIDER_SPHERE_TREE_CTOR;
    pub(crate) use crate::game::render::bezierman::callee::RIDER_SPHERE_TREE_GET_BOUNDS;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;

    /// `__builtin_delete` (operator delete): frees through the heap manager.
    pub const DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::f_00317e50,
    };

    /// `Dma_FromScratchpad` (0x371dd8): programs a scratchpad → RAM transfer.
    pub const DMA_FROM_SCRATCHPAD: Callee = Callee {
        addr: 0x0037_1dd8,
        f: ee::s12::f_00371dd8,
    };

    /// `ModObj_Update` (0x356198): the base object update.
    pub const MOD_OBJ_UPDATE: Callee = Callee {
        addr: 0x0035_6198,
        f: ee::s11::f_00356198,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE2;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_CALC;
}

// ---------------------------------------------------------------------------
// Sphere tree.

/// `float SphereTree_GetHalfLeafRadius(SphereTree *tree)` (0x32c5a8): half the
/// radius read through `tree->leaves[?]`, scaled by the value at `+0x90`.
pub fn sphere_tree_get_half_leaf_radius(c: &mut Cpu) {
    c.cycles += 13;
    let tree = Addr(abi::arg(c, 0));
    let leaf = tree.off(0x98).lw(c);
    c.g[V1][0] = leaf;
    c.g[A1][0] = 0xc;
    c.f[2] = tree.off(0x90).read_fpu(c).0;
    let a = Addr(leaf as u32).off(0xc).lw(c);
    let b = Addr(leaf as u32).off(0x20).lw(c);
    let scaled = abi::mult(c, a as u32, 0xc);
    c.g[V0][0] = sx(scaled);
    c.g[A0][0] = b;
    c.g[AT][0] = 0x3f00_0000;
    c.f[1] = 0x3f00_0000; // 0.5, as `lui 0x3f00` + `mtc1 $at, $f1`
    let entry = sx(scaled.wrapping_add(b as u32));
    c.g[V0][0] = entry;
    c.f[0] = Addr(entry as u32).read_fpu(c).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[1])).0;
    abi::ret(c);
}

pub(crate) const ST_ENTRY: u32 = 0x0032_9f98;

/// The hit block (reached from the inner loop's back edge): resume entry.
pub(crate) const ST_HIT: u32 = 0x0032_a004;

/// Outer loop head (children of tree A against B's root): resume entry.
pub(crate) const ST_OUTER: u32 = 0x0032_a0b8;

/// Inner loop head (children of B against A's child): resume entry.
pub(crate) const ST_INNER: u32 = 0x0032_a130;

pub(crate) const ST_END: u32 = 0x0032_a1c0;

#[derive(Clone, Copy)]
pub(crate) enum SphereStage {
    Children,
    Outer,
    OuterReload,
    OuterNext,
    Inner,
    Hit,
    Miss,
    Done,
}

/// The inlined sphere-overlap test: `f1 = rb`, `f0 = ra`, `d = cb - ca`
/// (stored at `sp_2`, `sp_2+0x10`, `sp_2+0x20`), `f2 = rb + ra`, `f1 = f2^2`,
/// `f0 = |d|^2`, FCC = `f1 < f0`. True when the spheres are apart.
pub(crate) fn spheres_apart(
    c: &mut Cpu,
    b_center: u32,
    b_radius: u32,
    a_center: u32,
    a_radius: u32,
) -> bool {
    let s = sp_2(c);
    c.lwc1(1, b_radius);
    c.lwc1(0, a_radius);
    c.vsub(b_center, a_center);
    c.sqc2(5, p(s, 0x20));
    c.add_s(2, 1, 0);
    c.lq(V0, p(s, 0x20));
    c.sq(V0, s);
    c.mul_s(1, 2, 2);
    c.sq(V0, p(s, 0x10));
    c.lqc2(3, s);
    c.lqc2(5, s);
    c.vdot_regs(5);
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    c.c_lt_s(1, 0);
    c.fcc()
}

/// `bool RiderSphereTree_Intersect(SphereTree* a, SphereTree* b, Vec4*
/// push, Vec4* point)` (0x329f98): when the root spheres overlap, tests
/// every child sphere of `a` against `b`'s root and then against every
/// child of `b`; at the first overlapping pair writes the push-out vector
/// (`normal * penetration`) to `push` and the contact point (on `a`'s
/// child, `t2`) to `point` and returns true.
///
/// Dispatcher-shaped in the original: the loop heads and the hit block are
/// resume entries, all state lives in registers (t0-t4, f0/f2, the stack
/// scratch).
pub fn rider_sphere_tree_intersect(c: &mut Cpu) {
    use sphere_tree as st;
    let mut stage = match c.pc {
        ST_ENTRY => {
            c.cycles += 25;
            c.set_r(SP, sp_2(c).wrapping_sub(0x50));
            let (a, b) = (r(c, A0), r(c, A1));
            let apart = spheres_apart(
                c,
                p(b, st::CENTER),
                p(b, st::RADIUS),
                p(a, st::CENTER),
                p(a, st::RADIUS),
            );
            c.g[V0][0] = 0;
            if apart {
                c.cycles += 2;
                SphereStage::Done
            } else {
                SphereStage::Children
            }
        }
        ST_HIT => SphereStage::Hit,
        ST_OUTER => SphereStage::Outer,
        ST_INNER => SphereStage::Inner,
        pc => {
            c.jump_out(pc, ST_ENTRY, ST_END);
            return;
        }
    };
    loop {
        stage = match stage {
            SphereStage::Children => {
                c.cycles += 3;
                c.lw_r(V0, at(c, A0, st::COUNT));
                c.g[T4][0] = 0;
                if (c.g[V0][0] as i64) <= 0 {
                    SphereStage::Miss
                } else {
                    c.cycles += 2;
                    c.g[T3][0] = c.g[A0][0];
                    SphereStage::Outer
                }
            }
            // A's child t3 against B's root.
            SphereStage::Outer => {
                c.cycles += 24;
                let (b, child) = (r(c, A1), r(c, T3));
                let apart = spheres_apart(
                    c,
                    p(b, st::CENTER),
                    p(b, st::RADIUS),
                    p(child, st::CHILD_CENTER),
                    p(child, st::CHILD_RADIUS),
                );
                if apart {
                    c.lw_r(V0, at(c, A0, st::COUNT));
                    SphereStage::OuterNext
                } else {
                    c.cycles += 3;
                    c.lw_r(V0, at(c, A1, st::COUNT));
                    c.g[T1][0] = 0;
                    if (c.g[V0][0] as i64) <= 0 {
                        SphereStage::OuterReload
                    } else {
                        c.cycles += 3;
                        c.g[T2][0] = c.g[T3][0];
                        c.g[T0][0] = c.g[A1][0];
                        SphereStage::Inner
                    }
                }
            }
            // B's child t0 against A's child t2.
            SphereStage::Inner => {
                c.cycles += 24;
                let (bc, ac) = (r(c, T0), r(c, T2));
                let apart = spheres_apart(
                    c,
                    p(bc, st::CHILD_CENTER),
                    p(bc, st::CHILD_RADIUS),
                    p(ac, st::CHILD_CENTER),
                    p(ac, st::CHILD_RADIUS),
                );
                c.inc_r(T1, 1);
                if !apart {
                    if back_edge_unwinds(c, ST_HIT) {
                        return;
                    }
                    SphereStage::Hit
                } else {
                    c.cycles += 4;
                    c.lw_r(V0, at(c, A1, st::COUNT));
                    c.g[V0][0] = slt(c, T1, V0);
                    c.inc_r(T0, st::CHILD_SIZE);
                    if c.g[V0][0] != 0 {
                        if back_edge_unwinds(c, ST_INNER) {
                            return;
                        }
                        SphereStage::Inner
                    } else {
                        SphereStage::OuterReload
                    }
                }
            }
            SphereStage::OuterReload => {
                c.cycles += 1;
                c.lw_r(V0, at(c, A0, st::COUNT));
                SphereStage::OuterNext
            }
            SphereStage::OuterNext => {
                c.cycles += 4;
                c.inc_r(T4, 1);
                c.g[V0][0] = slt(c, T4, V0);
                c.inc_r(T3, st::CHILD_SIZE);
                if c.g[V0][0] != 0 {
                    if back_edge_unwinds(c, ST_OUTER) {
                        return;
                    }
                    SphereStage::Outer
                } else {
                    SphereStage::Miss
                }
            }
            SphereStage::Hit => {
                sphere_tree_hit(c);
                SphereStage::Done
            }
            SphereStage::Miss => {
                c.cycles += 1;
                c.g[V0][0] = 0;
                SphereStage::Done
            }
            SphereStage::Done => {
                c.cycles += 2;
                let ra = r(c, RA);
                c.set_r(SP, sp_2(c).wrapping_add(0x50));
                c.pc = ra;
                return;
            }
        };
    }
}

/// The hit: `n = d / |d|` (d at `sp_2`), `pen = (rb + ra) - |d|` (f2 - f1),
/// `*push = n * pen`, `*point = ca - n * (ra - pen)` with `a`'s child t2.
pub(crate) fn sphere_tree_hit(c: &mut Cpu) {
    c.cycles += 40;
    let s = sp_2(c);
    c.vlen2(s);
    c.vnorm();
    c.sqc2(5, p(s, 0x20));
    c.sqrt_s(1, 0);
    c.lq(V1, p(s, 0x20));
    c.sub_s(1, 2, 1);
    c.g[V0][0] = 1;
    c.sq(V1, p(s, 0x10));
    c.mfc1(A0, 1);
    c.vscale(p(s, 0x10), A0);
    let push = r(c, A2);
    c.vstore(5, p(s, 0x30), V1, &[push, p(s, 0x20)]);
    let child = r(c, T2);
    c.lwc1(0, p(child, sphere_tree::CHILD_RADIUS));
    c.sub_s(0, 0, 1);
    c.mfc1(A0, 0);
    c.vscale(p(s, 0x10), A0);
    c.vstore(5, p(s, 0x40), V1, &[p(s, 0x30)]);
    c.vsub(p(child, sphere_tree::CHILD_CENTER), p(s, 0x30));
    let point = r(c, A3);
    c.vstore(5, p(s, 0x40), V1, &[point, p(s, 0x20)]);
}

// ---------------------------------------------------------------------------
// Forward-difference cache.

pub(crate) const FD_ROW: u32 = 0x0032_bbf0;

pub(crate) const FD_CELL: u32 = 0x0032_bc28;

/// The triangle normal of `(a, b, d)` on VU0: `vf5 = normalize((a - b) x
/// (b - d))`, `vf10.w = 1 / |n|`, `v0 = vf10` (`qmfc2`).
pub(crate) fn triangle_normal_regs(c: &mut Cpu, a: u32, b: u32, d: u32) {
    c.lqc2(2, a);
    c.lqc2(3, b);
    c.lqc2(4, d);
    let v = c.vu0();
    v.sub(0b1111, 5, 2, Src::Vec(3));
    v.sub(0b1111, 6, 3, Src::Vec(4));
    v.opmula(5, 6);
    v.opmsub(7, 6, 5);
    v.sub(0b0001, 7, 7, Src::Vec(7));
    v.add(0b1000, 8, 0, Src::Bc(0, W));
    v.mul(0b1111, 9, 7, Src::Vec(7));
    v.adda(0b1000, 9, Src::Bc(9, Y));
    v.madda(0b1000, 8, Src::Bc(9, Z));
    v.madd(0b1000, 9, 8, Src::Bc(9, W));
    v.rsqrt(0, 3, 9, 0);
    v.waitq();
    v.mul(0b1111, 5, 7, Src::Q);
    v.add(0b1000, 10, 0, Src::Bc(0, W));
    v.mul(0b1111, 10, 10, Src::Q);
    c.qmfc2(V0, 10);
}

/// The box of the quad `(a, b, d, e)` into `cell+0x20` (min) / `+0x30`
/// (max).
pub(crate) fn quad_box(c: &mut Cpu, a: u32, b: u32, d: u32, e: u32, cell_at: u32) {
    c.lqc2(2, a);
    c.lqc2(3, b);
    c.lqc2(4, d);
    c.lqc2(5, e);
    let v = c.vu0();
    v.max(0b1111, 6, 2, Src::Vec(3));
    v.mini(0b1111, 7, 2, Src::Vec(3));
    v.max(0b1111, 8, 4, Src::Vec(5));
    v.mini(0b1111, 9, 4, Src::Vec(5));
    v.max(0b1111, 6, 6, Src::Vec(8));
    v.mini(0b1111, 7, 7, Src::Vec(9));
    c.sqc2(6, p(cell_at, cell::BOX_MAX));
    c.sqc2(7, p(cell_at, cell::BOX_MIN));
}

/// The volume's UV refine (virtual 0x40) for kind-2 volumes: `vt[0x40](
/// vol, key, &u, &v, a3', t0', t1')` with the saved t5-t7.
pub(crate) fn uv_refine(c: &mut Cpu, uv: u32, ret: u32) -> Flow {
    c.cycles += 11;
    c.lw_r(V0, at(c, S3, volume::VTABLE));
    c.g[T0][0] = c.g[T5][0];
    let s = sp_2(c);
    c.lw_r(A1, p(s, 0x30));
    c.g[T1][0] = c.g[T6][0];
    c.g[T2][0] = c.g[T7][0];
    c.set_r(A2, p(s, uv));
    c.set_r(A3, p(s, uv + 4));
    vcall(c, S3, V0, 0x40, V1, ret)
}

/// The volume's triangle test (virtual 0x20) with t4-t7 (`t4` only in the
/// grid scan) saved around it; v0 = hit.
pub(crate) fn triangle_test(c: &mut Cpu, save_t4: bool, ret: u32) -> Flow {
    let s = sp_2(c);
    if save_t4 {
        c.sq(T4, p(s, 0x50));
    }
    c.sq(T5, p(s, 0x60));
    c.sq(T6, p(s, 0x70));
    c.sq(T7, p(s, 0x80));
    vcall(c, S3, V1, 0x20, V0, ret)?;
    let s = sp_2(c);
    if save_t4 {
        c.lq(T4, p(s, 0x50));
    }
    c.lq(T5, p(s, 0x60));
    c.lq(T6, p(s, 0x70));
    c.lq(T7, p(s, 0x80));
    Ok(())
}

/// Facing test of a triangle normal at `n` against the volume direction:
/// when the volume asks for it (`+8`), `f{fr} = dot(n, dir)` and `c.le.s
/// fr, limit` (`limit` loaded from `gp + limit_at` first when given). True
/// when the triangle is to be tested; leaves v1 = the volume vtable then.
pub(crate) fn faces_volume(
    c: &mut Cpu,
    n: u32,
    fr: usize,
    limit: usize,
    limit_at: Option<i32>,
    cycles: u64,
) -> bool {
    c.cycles += 3;
    c.lw_r(V0, at(c, S3, volume::FACING));
    if c.g[V0][0] != 0 {
        c.cycles += cycles;
        if let Some(off) = limit_at {
            c.lwc1(limit, gp(c, off));
        }
        let dir = at(c, S3, volume::DIR);
        c.lqc2(3, n);
        c.lqc2(5, dir);
        c.vdot_regs(5);
        c.qmfc2(V0, 4);
        c.mtc1(V0, fr);
        c.c_le_s(fr, limit);
        if !c.fcc() {
            return false;
        }
        c.cycles += 1;
    }
    c.lw_r(V1, at(c, S3, volume::VTABLE));
    true
}

/// `bool AIFwdDiffCache_CollideEntry(Volume* vol, int key, Entry* e, a3,
/// t0, t1, float uv[2], Hint* hint)` (0x32b6e0): the forward-difference
/// grid of a cache entry (fine 3x3 cells of 0x15 vertices, or coarse 9x9
/// of 1) against a volume. The hint's cell (same key and kind) is tried
/// first; otherwise every cell whose box overlaps the volume's box has its
/// two triangle normals computed on first use (degenerate ones marked) and
/// its triangles, when facing, handed to the volume's triangle test. A hit
/// writes the cell's UV (refined by kind-2 volumes) and the hint, and
/// returns true.
pub fn ai_fwd_diff_cache_collide_entry(c: &mut Cpu) {
    const SAVED: &[(usize, u32)] = &[
        (S1, 0x110),
        (S3, 0xf0),
        (S6, 0xc0),
        (S0, 0x120),
        (S2, 0x100),
        (S4, 0xe0),
        (S5, 0xd0),
        (S7, 0xb0),
        (FP, 0xa0),
    ];
    const FSAVED: &[(usize, u32)] = &[
        (25, 0x158),
        (24, 0x150),
        (23, 0x148),
        (22, 0x140),
        (21, 0x138),
        (20, 0x130),
    ];
    guest(c, |c| {
        c.cycles += 27;
        prologue(c, 0x160, SAVED, 0x90, FSAVED);
        c.g[T5][0] = c.g[A3][0];
        c.g[T6][0] = c.g[T0][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[T7][0] = c.g[T1][0];
        c.g[S6][0] = c.g[T3][0];
        let s = sp_2(c);
        c.sw_r(p(s, 0x30), A1);
        c.lw_r(V0, at(c, S3, volume::KIND));
        c.sw_r(p(s, 0x34), T2);
        // The grid: sp_2+0x38 cells per side, sp_2+0x3c vertex rows per cell
        // row, s2 vertex step, s0 the cells, f22/f23 the UV scale/offset.
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            c.g[A2][0] = 3;
            c.g[V0][0] = 0x15;
            c.g[S2][0] = 3;
            c.sw_r(p(s, 0x3c), V0);
            c.sw_r(p(s, 0x38), A2);
            c.lwc1(22, gp(c, -0x2dc0));
            c.lwc1(23, gp(c, -0x2dbc));
            c.set_r(S0, at(c, S1, 0x1a90));
        } else {
            c.cycles += 8;
            c.g[A2][0] = 1;
            c.g[V1][0] = 9;
            c.lwc1(22, gp(c, -0x2db8));
            c.g[S2][0] = 1;
            c.lwc1(23, gp(c, -0x2db4));
            c.set_r(S0, at(c, S1, 0x650));
            c.sw_r(p(s, 0x3c), A2);
            c.sw_r(p(s, 0x38), V1);
        }
        let hit = fd_try_hint(c)?;
        if !hit {
            fd_scan(c)?;
        }
        c.cycles += 18;
        epilogue(
            c,
            &[
                (S0, 0x120),
                (S1, 0x110),
                (S2, 0x100),
                (S3, 0xf0),
                (S4, 0xe0),
                (S5, 0xd0),
                (S6, 0xc0),
                (S7, 0xb0),
                (FP, 0xa0),
            ],
            &[
                (25, 0x158),
                (24, 0x150),
                (23, 0x148),
                (22, 0x140),
                (21, 0x138),
                (20, 0x130),
            ],
            0x90,
            0x160,
        );
        Ok(())
    })
}

/// The hint's cell first. True (v0 = 1) on a hit; otherwise the hint is
/// cleared (when it was this entry's) and v0 = cells per side.
pub(crate) fn fd_try_hint(c: &mut Cpu) -> Result<bool, Unwound> {
    let s = sp_2(c);
    c.cycles += 2;
    let no_hint = c.g[S6][0] == 0;
    c.lw_r(V1, p(s, 0x30));
    if no_hint {
        c.cycles += 1;
        c.lw_r(V0, p(s, 0x38));
        return Ok(false);
    }
    c.cycles += 3;
    c.lw_r(V0, at(c, S6, hint::KEY));
    let other_key = c.g[V0][0] != c.g[V1][0];
    c.lw_r(V0, p(s, 0x38));
    if other_key {
        return Ok(false);
    }
    c.cycles += 4;
    c.lw_r(V1, at(c, S6, hint::KIND));
    c.lw_r(V0, at(c, S3, volume::KIND));
    if c.g[V1][0] != c.g[V0][0] {
        c.lw_r(V0, p(s, 0x38));
        return Ok(false);
    }
    // The cell (t1) and its quad's vertices a3, a2, a0, a1.
    c.cycles += 2;
    let coarse = c.g[V1][0] != 0;
    c.g[V1][0] = c.lhu(at(c, S6, hint::ROW));
    if !coarse {
        c.cycles += 6;
        c.g[V0][0] = 0xc0;
        c.g[A0][0] = c.lhu(at(c, S6, hint::COL));
        c.mult(0, V0, V1, V0);
        c.set_r(A1, r(c, A0) << 6);
        c.inc_r(V0, 0x1a90);
    } else {
        c.cycles += 5;
        c.g[V0][0] = 0x240;
        c.g[A0][0] = c.lhu(at(c, S6, hint::COL));
        c.mult(0, V0, V1, V0);
        c.set_r(A1, r(c, A0) << 6);
        c.inc_r(V0, 0x650);
    }
    c.cycles += 19;
    c.set_r(V0, r(c, S1).wrapping_add(r(c, V0)));
    c.set_r(T1, r(c, V0).wrapping_add(r(c, A1)));
    c.mult(0, V0, V1, A2);
    c.lwc1(0, at(c, T1, cell::BOX_MIN));
    c.mult(0, V1, A0, S2);
    c.lwc1(1, gp(c, -0x2db0));
    c.g[A0][0] = 0xa0;
    c.set_r(A1, r(c, S2) << 4);
    c.mult(0, A2, S2, A0);
    c.c_eq_s(0, 1);
    c.mult(0, V0, V0, A0);
    c.set_r(V1, r(c, V1) << 4);
    c.inc_r(V0, 0x10);
    c.set_r(V0, r(c, S1).wrapping_add(r(c, V0)));
    c.set_r(A3, r(c, V0).wrapping_add(r(c, V1)));
    c.set_r(A2, r(c, A3).wrapping_add(r(c, A2)));
    c.set_r(A0, r(c, A3).wrapping_add(r(c, A1)));
    let no_box = c.fcc();
    c.set_r(A1, r(c, A2).wrapping_add(r(c, A1)));
    if no_box {
        c.cycles += 12;
        let (a3, a2, a0, a1, cl) = (r(c, A3), r(c, A2), r(c, A0), r(c, A1), r(c, T1));
        quad_box(c, a3, a2, a0, a1, cl);
    }
    // The first triangle when the hint says so, else the second.
    c.cycles += 3;
    c.lw_r(V0, at(c, S6, hint::FIRST));
    let cl = r(c, T1);
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.lwc1(0, cl);
        c.c_eq_s(0, 1);
        if c.fcc() {
            c.cycles += 27;
            let (a1, a2, a0) = (r(c, A1), r(c, A2), r(c, A0));
            triangle_normal_regs(c, a1, a2, a0);
            c.sqc2(5, cl);
            c.g[AT][0] = 0x4120_0000; // 10.0
            c.mtc1(AT, 0);
            c.mtc1(V0, 1);
            c.c_lt_s(0, 1);
            let tiny = c.fcc();
            let s = sp_2(c);
            c.sw_r(s, V0);
            if tiny {
                c.cycles += 2;
                c.lwc1(0, gp(c, -0x2dac));
                c.swc1(0, cl);
            }
            c.cycles += 1;
        }
        c.lwc1(1, cl);
        c.cycles += 5;
        c.lwc1(0, gp(c, -0x2da8));
        c.c_eq_s(1, 0);
        if c.fcc() {
            c.sw(r(c, S6), 0);
            fd_clear_hint(c);
            return Ok(false);
        }
        if !faces_volume(c, cl, 1, 0, Some(-0x2da4), 14) {
            c.sw(r(c, S6), 0);
            fd_clear_hint(c);
            return Ok(false);
        }
        c.cycles += 16;
        c.g[A3][0] = c.g[A0][0];
        c.g[T0][0] = c.g[T1][0];
        c.g[T2][0] = c.g[T6][0];
        c.g[T1][0] = c.g[T5][0];
        c.g[T3][0] = c.g[T7][0];
        triangle_test(c, false, 0x0032_b988)?;
        if c.g[V0][0] == 0 {
            fd_drop_hint(c);
            return Ok(false);
        }
        fd_hint_uv(c, 0x10, 0x0032_ba00, 15, 4)?;
        return Ok(true);
    }
    c.lwc1(0, p(cl, cell::NORMAL1));
    c.cycles += 4;
    c.c_eq_s(0, 1);
    if c.fcc() {
        c.cycles += 27;
        let (a0, a2, a3) = (r(c, A0), r(c, A2), r(c, A3));
        triangle_normal_regs(c, a0, a2, a3);
        c.sqc2(5, p(cl, cell::NORMAL1));
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(AT, 0);
        c.mtc1(V0, 1);
        c.c_lt_s(0, 1);
        let tiny = c.fcc();
        let s = sp_2(c);
        c.sw_r(s, V0);
        if tiny {
            c.cycles += 2;
            c.lwc1(0, gp(c, -0x2da0));
            c.swc1(0, p(cl, cell::NORMAL1));
        }
        c.cycles += 1;
    }
    c.lwc1(1, p(cl, cell::NORMAL1));
    c.cycles += 5;
    c.lwc1(0, gp(c, -0x2d9c));
    c.c_eq_s(1, 0);
    let degenerate = c.fcc();
    c.set_r(T0, p(cl, cell::NORMAL1));
    if degenerate {
        fd_drop_hint(c);
        return Ok(false);
    }
    if !faces_volume(c, p(cl, cell::NORMAL1), 1, 0, Some(-0x2d98), 14) {
        c.sw(r(c, S6), 0);
        fd_clear_hint(c);
        return Ok(false);
    }
    c.cycles += 15;
    c.g[A1][0] = c.g[A0][0];
    c.g[T1][0] = c.g[T5][0];
    c.g[T2][0] = c.g[T6][0];
    c.g[T3][0] = c.g[T7][0];
    triangle_test(c, false, 0x0032_bb20)?;
    if c.g[V0][0] == 0 {
        fd_drop_hint(c);
        return Ok(false);
    }
    fd_hint_uv(c, 0x18, 0x0032_bb98, 15, 3)?;
    Ok(true)
}

/// The hint cell's UV (`sp_2+uv`), refined for kind-2 volumes, written to
/// the caller's `uv`; v0 = 1.
pub(crate) fn fd_hint_uv(c: &mut Cpu, uv: u32, ret: u32, cycles: u64, tail: u64) -> Flow {
    c.cycles += cycles;
    let s = sp_2(c);
    c.g[V0][0] = c.lhu(at(c, S6, hint::COL));
    c.g[A1][0] = 2;
    c.g[V1][0] = c.lhu(at(c, S6, hint::ROW));
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    c.lw_r(A0, at(c, S3, volume::KIND));
    c.mtc1(V1, 0);
    c.cvt_s_w(0, 0);
    c.mul_s(1, 1, 22);
    c.mul_s(0, 0, 22);
    c.add_s(1, 1, 23);
    c.add_s(0, 0, 23);
    c.swc1(1, p(s, uv + 4));
    let refine = c.g[A0][0] == c.g[A1][0];
    c.swc1(0, p(s, uv));
    if refine {
        uv_refine(c, uv, ret)?;
    }
    c.cycles += tail;
    let s = sp_2(c);
    c.lwc1(1, p(s, uv));
    c.g[V0][0] = 1;
    c.lwc1(0, p(s, uv + 4));
    c.cycles += 4;
    c.lw_r(V1, p(s, 0x34));
    let out = r(c, V1);
    c.swc1(1, out);
    c.swc1(0, p(out, 4));
    Ok(())
}

/// `hint->key = 0`, then [`fd_clear_hint`].
pub(crate) fn fd_drop_hint(c: &mut Cpu) {
    c.cycles += 1;
    c.sw(r(c, S6), 0);
    fd_clear_hint(c);
}

/// Clear the rest of the hint; v0 = cells per side.
pub(crate) fn fd_clear_hint(c: &mut Cpu) {
    c.cycles += 4;
    let h = r(c, S6);
    c.sh(p(h, hint::ROW), 0);
    c.sh(p(h, hint::COL), 0);
    c.sw(p(h, hint::FIRST), 0);
    c.sw(p(h, hint::KIND), 0);
    c.cycles += 1;
    let s = sp_2(c);
    c.lw_r(V0, p(s, 0x38));
}

/// Scan every cell (v0 = cells per side on entry); v0 = 1 on a hit, else 0.
pub(crate) fn fd_scan(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.set_r(S4, at(c, S1, 0x10));
    let empty = c.g[V0][0] == 0;
    c.g[FP][0] = 0;
    if empty {
        c.cycles += 1;
        c.g[V0][0] = 0;
        return Ok(());
    }
    c.cycles += 6;
    let s = sp_2(c);
    c.set_r(V1, r(c, S2) << 4);
    c.g[V0][0] = 0xa0;
    c.sw_r(p(s, 0x44), V1);
    c.mult(0, V1, S2, V0);
    c.sw_r(p(s, 0x40), V1);
    c.lw_r(V0, p(s, 0x40));
    loop {
        // Row fp: the quad corners s4, s1 (next row), s2, s5.
        c.cycles += 7;
        c.g[S7][0] = 0;
        c.lw_r(V1, p(s, 0x44));
        c.set_r(S1, r(c, S4).wrapping_add(r(c, V0)));
        c.lw_r(V0, p(s, 0x38));
        c.set_r(S2, r(c, S4).wrapping_add(r(c, V1)));
        let no_cells = c.g[V0][0] == 0;
        c.set_r(S5, r(c, S1).wrapping_add(r(c, V1)));
        if !no_cells {
            c.cycles += 7;
            c.lwc1(21, gp(c, -0x2d94));
            c.g[T4][0] = c.g[V1][0];
            c.g[AT][0] = 0x4120_0000; // 10.0
            c.mtc1(AT, 25);
            c.lwc1(20, gp(c, -0x2d90));
            c.lwc1(24, gp(c, -0x2d8c));
            loop {
                if fd_cell(c)? {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    return Ok(());
                }
                c.cycles += 8;
                c.inc_r(S7, 1);
                let step = r(c, T4);
                c.inc_r(S4, step);
                c.inc_r(S1, step);
                c.inc_r(S2, step);
                c.inc_r(S5, step);
                c.g[V0][0] = slt(c, S7, V1);
                let more = c.g[V0][0] != 0;
                c.inc_r(S0, cell::SIZE);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, FD_CELL);
                }
            }
        }
        c.cycles += 8;
        c.lw_r(V0, p(s, 0x3c));
        c.inc_r(FP, 1);
        c.set_r(V1, r(c, V0) << 4);
        c.set_r(S4, r(c, S4).wrapping_add(r(c, V1)));
        c.lw_r(V1, p(s, 0x38));
        c.g[V0][0] = slt(c, FP, V1);
        let more = c.g[V0][0] != 0;
        c.lw_r(V0, p(s, 0x40));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, FD_ROW);
        }
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
    Ok(())
}

/// One cell (s0) of the scan: box, overlap with the volume, both
/// triangles. True on a hit (UV and hint written); otherwise v1 = cells
/// per side.
pub(crate) fn fd_cell(c: &mut Cpu) -> Result<bool, Unwound> {
    let cl = r(c, S0);
    let vol = r(c, S3);
    c.cycles += 5;
    c.lwc1(0, p(cl, cell::BOX_MIN));
    c.c_eq_s(0, 21);
    if c.fcc() {
        c.cycles += 14;
        let (s4, s1, s2, s5) = (r(c, S4), r(c, S1), r(c, S2), r(c, S5));
        quad_box(c, s4, s1, s2, s5, cl);
        c.lwc1(1, p(vol, 0x20));
        c.lwc1(0, p(cl, cell::BOX_MIN));
    } else {
        c.lwc1(1, p(vol, 0x20));
    }
    // Box overlap: cell min < vol max and vol min < cell max on each axis.
    let mut overlap = false;
    c.cycles += 4;
    c.c_lt_s(0, 1);
    c.g[V0][0] = 0;
    if c.fcc() {
        let tests: [(u32, u32, bool, u64); 5] = [
            (0x30, cell::BOX_MAX, true, 5),
            (0x24, cell::BOX_MIN + 4, false, 5),
            (0x34, cell::BOX_MAX + 4, true, 5),
            (0x28, cell::BOX_MIN + 8, false, 5),
            (0x38, cell::BOX_MAX + 8, true, 6),
        ];
        overlap = true;
        for (i, &(voff, coff, rev, cyc)) in tests.iter().enumerate() {
            c.cycles += cyc;
            c.lwc1(1, p(vol, voff));
            c.lwc1(0, p(cl, coff));
            if rev {
                c.c_lt_s(1, 0);
            } else {
                c.c_lt_s(0, 1);
            }
            if !c.fcc() {
                overlap = false;
                break;
            }
            if i == 4 {
                c.g[V0][0] = 1;
            }
        }
    }
    c.cycles += 2;
    let s = sp_2(c);
    c.lw_r(V1, p(s, 0x38));
    if !overlap {
        return Ok(false);
    }
    // First triangle (s5, s1, s2).
    c.cycles += 4;
    c.lwc1(0, cl);
    c.c_eq_s(0, 21);
    if c.fcc() {
        c.cycles += 25;
        let (s5, s1, s2) = (r(c, S5), r(c, S1), r(c, S2));
        triangle_normal_regs(c, s5, s1, s2);
        c.sqc2(5, cl);
        c.mtc1(V0, 0);
        c.c_lt_s(25, 0);
        let tiny = c.fcc();
        c.sw_r(s, V0);
        if tiny {
            c.cycles += 1;
            c.swc1(20, cl);
        }
        c.cycles += 1;
        c.lwc1(0, cl);
    }
    c.cycles += 4;
    c.c_eq_s(0, 20);
    let mut tested = false;
    if !c.fcc() && faces_volume(c, cl, 0, 24, None, 13) {
        c.cycles += 20;
        c.g[T1][0] = c.g[T5][0];
        c.g[T2][0] = c.g[T6][0];
        c.g[T3][0] = c.g[T7][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S0][0];
        triangle_test(c, true, 0x0032_be00)?;
        tested = true;
        if c.g[V0][0] != 0 {
            fd_cell_hit(c, 0x20, 0x0032_be74, true)?;
            return Ok(true);
        }
    }
    if tested {
        c.cycles += 1;
    }
    c.lwc1(0, p(cl, cell::NORMAL1));
    // Second triangle (s2, s1, s4).
    c.cycles += 4;
    c.c_eq_s(0, 21);
    if c.fcc() {
        c.cycles += 25;
        let (s2, s1, s4) = (r(c, S2), r(c, S1), r(c, S4));
        triangle_normal_regs(c, s2, s1, s4);
        c.sqc2(5, p(cl, cell::NORMAL1));
        c.mtc1(V0, 0);
        c.c_lt_s(25, 0);
        let tiny = c.fcc();
        c.sw_r(s, V0);
        if tiny {
            c.cycles += 1;
            c.swc1(20, p(cl, cell::NORMAL1));
        }
        c.cycles += 1;
        c.lwc1(0, p(cl, cell::NORMAL1));
    } else {
        c.lwc1(0, p(cl, cell::NORMAL1));
    }
    c.cycles += 4;
    c.c_eq_s(0, 20);
    let degenerate = c.fcc();
    c.set_r(T0, p(cl, cell::NORMAL1));
    if degenerate {
        c.cycles += 1;
        c.lw_r(V1, p(s, 0x38));
        return Ok(false);
    }
    // Facing (the original reloads v1 = cells per side when not facing).
    c.cycles += 3;
    c.lw_r(V0, p(vol, volume::FACING));
    if c.g[V0][0] != 0 {
        c.cycles += 13;
        c.lqc2(3, p(cl, cell::NORMAL1));
        c.lqc2(5, p(vol, volume::DIR));
        c.vdot_regs(5);
        c.qmfc2(V0, 4);
        c.mtc1(V0, 0);
        c.c_le_s(0, 24);
        let facing = c.fcc();
        c.lw_r(V1, p(s, 0x38));
        if !facing {
            return Ok(false);
        }
        c.cycles += 1;
    }
    c.lw_r(V1, p(vol, volume::VTABLE));
    c.cycles += 19;
    c.g[T1][0] = c.g[T5][0];
    c.g[T2][0] = c.g[T6][0];
    c.g[T3][0] = c.g[T7][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S4][0];
    triangle_test(c, true, 0x0032_bfb4)?;
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.lw_r(V1, p(s, 0x38));
        return Ok(false);
    }
    fd_cell_hit(c, 0x28, 0x0032_c028, false)?;
    Ok(true)
}

/// A hit in the scan: the cell's UV at `sp_2+uv` (refined for kind-2
/// volumes) to the caller's `uv`, the hint (`first`: the cell's first
/// triangle) with the volume kind.
pub(crate) fn fd_cell_hit(c: &mut Cpu, uv: u32, ret: u32, first: bool) -> Flow {
    c.cycles += 13;
    let s = sp_2(c);
    c.mtc1(FP, 0);
    c.cvt_s_w(0, 0);
    c.lw_r(V0, at(c, S3, volume::KIND));
    c.mtc1(S7, 1);
    c.cvt_s_w(1, 1);
    c.g[V1][0] = 2;
    c.mul_s(0, 0, 22);
    c.mul_s(1, 1, 22);
    c.add_s(0, 0, 23);
    c.add_s(1, 1, 23);
    c.swc1(0, p(s, uv));
    let refine = c.g[V0][0] == c.g[V1][0];
    c.swc1(1, p(s, uv + 4));
    if refine {
        uv_refine(c, uv, ret)?;
    }
    c.cycles += 6;
    let s = sp_2(c);
    c.lwc1(0, p(s, uv));
    c.lwc1(1, p(s, uv + 4));
    c.lw_r(V0, p(s, 0x34));
    let out = r(c, V0);
    c.swc1(0, out);
    let has_hint = c.g[S6][0] != 0;
    c.swc1(1, p(out, 4));
    if has_hint {
        let h = r(c, S6);
        if first {
            c.cycles += 7;
            c.lw_r(V0, p(s, 0x30));
            c.g[V1][0] = 1;
            c.sh(p(h, hint::ROW), c.g[FP][0]);
            c.sw_r(h, V0);
            c.sh(p(h, hint::COL), c.g[S7][0]);
            c.sw_r(p(h, hint::FIRST), V1);
        } else {
            c.cycles += 5;
            c.lw_r(V1, p(s, 0x30));
            c.sh(p(h, hint::ROW), c.g[FP][0]);
            c.sw_r(h, V1);
            c.sh(p(h, hint::COL), c.g[S7][0]);
            c.sw(p(h, hint::FIRST), 0);
        }
        c.cycles += 2;
        c.lw_r(V0, at(c, S3, volume::KIND));
        c.sw_r(p(h, hint::KIND), V0);
    }
    Ok(())
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
        c.lwc1(1, gp_2(c, -0x2d7c));
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
            c.sqc2(5, p(sp_3(c), 0x10));
            c.mtc1(0, 1);
            c.lq(2, p(sp_3(c), 0x10));
            c.sq(2, sp_3(c));
            c.lqc2(3, sp_3(c));
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
                    c.sqc2(5, p(sp_3(c), 0x20));
                    c.lwc1(12, gp_2(c, -0x2d78));
                    c.lq(2, p(sp_3(c), 0x20));
                    c.g[A3][0] = c.g[T1][0];
                    c.sq(2, p(sp_3(c), 0x10));
                    c.lqc2(3, p(tree, 0x60));
                    c.lqc2(4, p(sp_3(c), 0x10));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, p(sp_3(c), 0x20));
                    let hit = c.g[T1][0] as u32;
                    c.lq(3, p(sp_3(c), 0x20));
                    c.sq(3, hit);
                    c.sq(3, sp_3(c));
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

/// `swc1 fN, a`.
#[inline(always)]
pub(crate) fn stf(c: &mut Cpu, n: usize, a: u32) {
    let v = fr(c, n);
    Addr(a).write_fpu(c, v);
}

/// The generated loop back-edge event check (`if cycles >= next_event`).
#[inline(always)]
pub(crate) fn event_check(c: &mut Cpu, head: u32) {
    if c.cycles >= c.next_event {
        c.pc = head;
        crate::events::deliver_event_here(c, head);
    }
}

/// `void RiderSphereTree_GetBounds(tree, Vec4* mn, Vec4* mx)` (0x329dc8):
/// `*mn`/`*mx` start as ±1e10 (w = 1). A leaf node (mask 0) yields its own
/// sphere: centre ∓/± radius with w = centre.w. Otherwise the box grows over
/// the children its mask selects, one float at a time. The VU0 difference and
/// sum go through `vf3`/`vf4` into `vf5`; every test uses the FPU condition
/// bit.
pub fn rider_sphere_tree_get_bounds(c: &mut Cpu) {
    use sphere_tree::*;
    c.cycles += 19;
    let frame = Frame::open(c, 0x40);
    let tree = r32(c, A0);
    let mn = r32(c, A1);
    let mx = r32(c, A2);
    let sp0 = sp(c);
    // mn = (1e10, 1e10, 1e10, 1.0)
    ldf(c, 0, gp_3(c, -0x2de8));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    stf(c, 0, p(sp0, 8));
    stf(c, 0, sp0);
    stf(c, 0, p(sp0, 4));
    stf(c, 1, p(sp0, 0xc));
    // mx = (-1e10, -1e10, -1e10, 1.0)
    ldf(c, 0, gp_3(c, -0x2de4));
    c.lq(3, sp0);
    stf(c, 0, p(sp0, 8));
    stf(c, 0, sp0);
    stf(c, 0, p(sp0, 4));
    c.sq(3, mn);
    c.lq(2, sp0);
    c.sq(2, mx);
    c.g[V1][0] = lw(c, p(tree, MASK));
    if c.g[V1][0] != 0 {
        c.g[V0][0] = lw(c, p(tree, COUNT));
    } else {
        c.cycles += 25;
        // mn = centre - (r, r, r, 0)
        ldf(c, 0, p(tree, RADIUS));
        c.sw(p(sp0, 0x1c), 0);
        stf(c, 0, p(sp0, 0x18));
        stf(c, 0, p(sp0, 0x10));
        stf(c, 0, p(sp0, 0x14));
        c.lqc2(3, p(tree, CENTER));
        c.lqc2(4, p(sp0, 0x10));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(sp0, 0x20));
        c.lq(2, p(sp0, 0x20));
        c.sw(p(sp0, 0x1c), 0);
        c.sq(2, mn);
        c.sq(2, sp0);
        // mx = centre + (r, r, r, 0)
        ldf(c, 0, p(tree, RADIUS));
        stf(c, 0, p(sp0, 0x18));
        stf(c, 0, p(sp0, 0x10));
        stf(c, 0, p(sp0, 0x14));
        c.lqc2(3, p(tree, CENTER));
        c.lqc2(4, p(sp0, 0x10));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(sp0, 0x20));
        c.lq(2, p(sp0, 0x20));
        c.sq(2, mx);
        c.sq(2, sp0); // also mirrored into the frame's low quadword
                      // A leaf returns straight to the epilogue: the count test and the
                      // child loop belong to the masked-node path only.
        c.cycles += 2;
        frame.close(c);
        return;
    }
    c.cycles += 2;
    c.g[T0][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 4;
        c.mov_s(2, 1); // f2 = 1.0 (the children's w)
        c.g[T1][0] = 1;
        c.g[A3][0] = c.g[A0][0];
        loop {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(tree, MASK));
            c.g[V1][0] = sx((c.g[T1][0] as u32) << ((c.g[T0][0] as u32) & 31));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            if c.g[V0][0] != 0 {
                accumulate_kid(c, r32(c, A3), mn, mx);
                c.cycles += 1;
                c.g[V0][0] = lw(c, p(tree, COUNT));
            } else {
                c.g[V0][0] = lw(c, p(tree, COUNT));
            }
            c.cycles += 4;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(KID_STRIDE));
            if !more {
                break;
            }
            event_check(c, 0x0032_9e90);
        }
    }
    c.cycles += 2;
    frame.close(c);
}

/// Grow `mn`/`mx` by child `kid`'s sphere: `pos - r` against the minimum
/// (17 cycles, each further axis test 6) and `pos + r` against the maximum
/// (13, each further axis test 6). The radius quadword `(r, r, r, 1)` sits at
/// the frame's `sp + 0` throughout.
pub(crate) fn accumulate_kid(c: &mut Cpu, kid: u32, mn: u32, mx: u32) {
    use sphere_tree::*;
    let sp0 = sp(c);
    c.cycles += 17;
    ldf(c, 0, p(kid, KID_RADIUS));
    stf(c, 2, p(sp0, 0xc));
    stf(c, 0, p(sp0, 8));
    stf(c, 0, sp0);
    stf(c, 0, p(sp0, 4));
    c.lqc2(3, p(kid, KID_POS));
    c.lqc2(4, sp0);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp0, 0x20));
    c.lq(2, p(sp0, 0x20));
    ldf(c, 0, mn); // f0 = mn.x
    c.sq(2, p(sp0, 0x10));
    ldf(c, 1, p(sp0, 0x10)); // f1 = t.x
    if lt(c, 1, 0) {
        stf(c, 1, mn);
    }
    c.cycles += 6;
    ldf(c, 1, p(sp0, 0x14));
    ldf(c, 0, p(mn, 4));
    if lt(c, 1, 0) {
        stf(c, 1, p(mn, 4));
    }
    c.cycles += 6;
    ldf(c, 1, p(sp0, 0x18));
    ldf(c, 0, p(mn, 8));
    if lt(c, 1, 0) {
        stf(c, 1, p(mn, 8));
    }
    c.cycles += 13;
    c.lqc2(3, p(kid, KID_POS));
    c.lqc2(4, sp0);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp0, 0x30));
    c.lq(2, p(sp0, 0x30));
    ldf(c, 0, mx); // f0 = mx.x
    c.sq(2, p(sp0, 0x10));
    c.sq(2, p(sp0, 0x20));
    ldf(c, 1, p(sp0, 0x10)); // f1 = t.x
    if lt(c, 0, 1) {
        stf(c, 1, mx);
    }
    c.cycles += 6;
    ldf(c, 1, p(sp0, 0x14));
    ldf(c, 0, p(mx, 4));
    if lt(c, 0, 1) {
        stf(c, 1, p(mx, 4));
    }
    c.cycles += 6;
    ldf(c, 1, p(sp0, 0x18));
    ldf(c, 0, p(mx, 8));
    if lt(c, 0, 1) {
        stf(c, 1, p(mx, 8));
    }
}

// ---------------------------------------------------------------------------
// CollSegment_ctor.

/// `CollSegment* CollSegment_ctor(CollSegment* seg, const Vec4* p0,
/// const Vec4* p1, int state, float r)` (0x32e100): a capsule: the state and
/// vtable (`0x48e6a8`), the start point `p0` (raw and as the base), the
/// direction `p1 - p0`, the radius, the AABB of the two points and its
/// centre.
pub fn coll_segment_ctor(c: &mut Cpu) {
    c.cycles += 26;
    let seg = r32(c, A0);
    c.g[T0][0] = c.g[A0][0];
    c.g[V0][0] = sx(0x490000u32.wrapping_sub(0x1958));
    c.sw(seg, c.g[A3][0]); // state
    c.sw(seg.wrapping_add(0x50), c.g[V0][0]); // vtable
    let frame = Frame::open(c, 0x30);
    let sp = sp(c);
    c.sw(seg.wrapping_add(8), 0);
    c.sw(seg.wrapping_add(4), 0);
    let p0 = r32(c, A1);
    let p1 = r32(c, A2);
    c.lq(2, p0);
    c.sq(2, seg.wrapping_add(0x60)); // p0b
    c.sq(2, seg.wrapping_add(0x80)); // p0
    c.lqc2(3, p1);
    c.lqc2(4, p0);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp.wrapping_add(0x10));
    c.lq(2, sp.wrapping_add(0x10));
    c.swc1(12, seg.wrapping_add(0xa0)); // radius
    c.sq(2, seg.wrapping_add(0x70)); // dirb
    c.sq(2, seg.wrapping_add(0x90)); // dir
    c.sq(2, sp);

    // max = component-wise greater of p0 and p1, w = 1.
    c.lwc1(0, p1);
    c.lwc1(1, p0);
    c.c_lt_s(0, 1);
    let lt = c.fcc();
    c.mov_s(4, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(4, 1);
    }
    c.cycles += 6;
    c.lwc1(0, p1.wrapping_add(4));
    c.lwc1(1, p0.wrapping_add(4));
    c.c_lt_s(0, 1);
    let lt = c.fcc();
    c.mov_s(3, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(3, 1);
    }
    c.cycles += 6;
    c.lwc1(0, p1.wrapping_add(8));
    c.lwc1(1, p0.wrapping_add(8));
    c.c_lt_s(0, 1);
    let lt = c.fcc();
    c.mov_s(2, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(2, 1);
    }
    c.cycles += 14;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 5);
    c.swc1(4, sp);
    c.swc1(3, sp.wrapping_add(4));
    c.swc1(2, sp.wrapping_add(8));
    c.swc1(5, sp.wrapping_add(0xc));
    c.lq(2, sp);
    c.sq(2, seg.wrapping_add(0x20)); // max

    // min = component-wise lesser of p0 and p1, w = 1.
    c.lwc1(0, p1);
    c.lwc1(1, p0);
    c.c_lt_s(1, 0);
    let lt = c.fcc();
    c.mov_s(4, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(4, 1);
    }
    c.cycles += 6;
    c.lwc1(0, p1.wrapping_add(4));
    c.lwc1(1, p0.wrapping_add(4));
    c.c_lt_s(1, 0);
    let lt = c.fcc();
    c.mov_s(3, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(3, 1);
    }
    c.cycles += 6;
    c.lwc1(0, p1.wrapping_add(8));
    c.lwc1(1, p0.wrapping_add(8));
    c.c_lt_s(1, 0);
    let lt = c.fcc();
    c.mov_s(2, 0);
    if lt {
        c.cycles += 1;
        c.mov_s(2, 1);
    }

    // center = 0.5 * (min + max).
    c.cycles += 27;
    c.swc1(4, sp);
    c.g[V0][0] = c.g[T0][0];
    c.swc1(3, sp.wrapping_add(4));
    c.swc1(2, sp.wrapping_add(8));
    c.swc1(5, sp.wrapping_add(0xc));
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(1, 0);
    c.lq(3, sp);
    c.sq(3, seg.wrapping_add(0x30)); // min
    c.lqc2(3, seg.wrapping_add(0x30));
    c.lqc2(4, seg.wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp.wrapping_add(0x20));
    c.lq(3, sp.wrapping_add(0x20));
    c.sq(3, sp.wrapping_add(0x10));
    c.mfc1(4, 0);
    c.qmtc2(4, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, sp.wrapping_add(0x10));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, sp.wrapping_add(0x20));
    c.lq(3, sp.wrapping_add(0x20));
    c.sq(3, seg.wrapping_add(0x40)); // center
    c.sq(3, sp);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// CollHull_ctor.

/// `CollHull* CollHull_ctor(CollHull* hull, RiderSphereTree* tree, int flag)`
/// (0x32f650): the base at +0x50 (vtable `0x48e590`), the embedded sphere
/// tree node at +0x70, the tree at +0x60/+0x64, the state `flag == 0`, the
/// tree's bounds at +0x20/+0x30 and their centre at +0x40.
pub fn coll_hull_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x490000;
        frame.save(c, S0, 0x60);
        c.g[V0][0] = sx(0x490000u32.wrapping_sub(0x1a70));
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        let hull = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.sw(hull.wrapping_add(8), 0);
        c.g[A0][0] = sx(hull.wrapping_add(0x70));
        c.sw(hull.wrapping_add(4), 0);
        c.g[S1][0] = c.g[A1][0];
        c.sw(hull.wrapping_add(0x50), c.g[V0][0]); // vtable
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, callee::RIDER_SPHERE_TREE_CTOR, 0x0032_f68c)?;
        c.sw(hull.wrapping_add(0x64), c.g[S1][0]); // tree2
        c.g[S2][0] = (c.g[S2][0] < 1) as u64;
        c.sw(hull.wrapping_add(0x60), c.g[S1][0]); // tree
        c.g[A0][0] = c.g[S1][0];
        c.sw(hull, c.g[S2][0]); // state
        c.g[A1][0] = sx(hull.wrapping_add(0x30));
        c.g[A2][0] = sx(hull.wrapping_add(0x20));
        call_ptr(c, callee::RIDER_SPHERE_TREE_GET_BOUNDS, 0x0032_f6ac)?;

        // center = 0.5 * (min + max).
        c.lqc2(3, hull.wrapping_add(0x30));
        c.lqc2(4, hull.wrapping_add(0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        let sp = sp(c);
        c.sqc2(5, sp.wrapping_add(0x20));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 0); // f0 = 0.5
        c.lq(3, sp.wrapping_add(0x20));
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x50);
        c.sq(3, sp.wrapping_add(0x10));
        c.mfc1(4, 0);
        c.lqc2(4, sp.wrapping_add(0x10));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp.wrapping_add(0x20));
        frame.restore(c, S2, 0x40);
        c.lq(3, sp.wrapping_add(0x20));
        frame.restore_ra(c, 0x30);
        c.sq(3, hull.wrapping_add(0x40)); // center
        frame.restore(c, S0, 0x60);
        c.sq(3, sp);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// VU0_TriangleIntersect.

/// `bool VU0_TriangleIntersect(const Vec4* a, const Vec4* b, const Vec4* c,
/// const Vec4* d)` (0x32b6a8): loads the four points into VF10..VF13 and runs
/// the microprogram at VU address 0x6e0 (`vcallmsr` through VI27); VI2 comes
/// back non-zero when the triangle intersects.
pub fn vu0_triangle_intersect(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V0][0] = 0x0;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6e0));
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
    c.lqc2(10, c.g[A0][0] as u32);
    c.lqc2(11, c.g[A1][0] as u32);
    c.lqc2(12, c.g[A2][0] as u32);
    c.lqc2(13, c.g[A3][0] as u32);
    c.ctc2(V0, 27);
    c.vu0().vnop();
    c.vu0().vnop();
    c.vcallmsr();
    c.cfc2(V0, 2);
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    c.pc = t;
}

// ---------------------------------------------------------------------------
// RiderSphereTree_dtor / CollHull_dtor.

/// `void RiderSphereTree_dtor(RiderSphereTree* t, int flags)` (0x329970): free
/// the object when `flags & 1` (its members are plain data).
pub fn rider_sphere_tree_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let free = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if free {
            c.cycles += 2;
            callee::DELETE.call(c, 0x0032_9988)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CollHull_dtor(CollHull* h, int flags)` (0x32f708): restore the base
/// vtable (`+0x50`), destroy the sphere tree at `+0x70`, then free the hull
/// when `flags & 1`.
pub fn coll_hull_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1a70));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let vtable = c.g[V0][0];
        let hull = r32(c, S1);
        Addr(hull.wrapping_add(0x50)).sw(c, vtable);
        c.g[A0][0] = sx(hull.wrapping_add(0x70));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0032_9970, 0x0032_f738)?;
        c.g[S0][0] = c.g[S0][0] & 0x1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0032_f74c)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SphereTree_SweepSphereAABB` (0x329590).
pub(crate) const SPHERE_TREE_SWEEP_AABB: u32 = 0x0032_9590;

// ---------------------------------------------------------------------------
// Sphere tree helpers.

/// `SphereTree* SphereTree_ctor(SphereTree* self)` (0x32c508): clear the
/// count at `+0x98` and reset the root transform; returns `self`.
pub fn sphere_tree_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 7;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.sw((this as u32).wrapping_add(0x98), 0);
        c.g[A0][0] = c.g[S0][0];
        callee::SPHERE_TREE_RESET_TRANSFORM.call(c, 0x0032_c52c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int CollSphere_CollideAABB(coll, a, b, box)` (0x32fe40): test the sphere
/// at `self + 0x60` (radius at `+0x70`) against the AABB in `box` (`+0x0`,
/// `+0x10`, `+0x40`); the callee's result is returned as a bool.
pub fn coll_sphere_collide_aabb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
        c.lwc1(12, (c.g[V0][0] as u32).wrapping_add(0x70));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x60));
        c.g[T1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
        call_ptr(c, SPHERE_TREE_SWEEP_AABB, 0x0032_fe68)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `base + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn p_2(base: u32, off: u32) -> u32 {
    base.wrapping_add(off)
}

/// `void RiderSphereTree_SetSpheres(RiderSphereTree* self, int n,
/// const Sphere* spheres, const float* radii, const Vec4* center, ...)`
/// (0x329a90): copy up to `n` child sphere bounds, then the root sphere.
pub fn rider_sphere_tree_set_spheres(c: &mut Cpu) {
    c.cycles += 3;
    let this = r32(c, A0);
    Addr(this).off(0x2c).sw(c, c.g[A1][0]);
    let none = (c.g[A1][0] as i64) <= 0;
    c.g[T1][0] = 0;
    if !none {
        c.cycles += 1;
        c.g[V1][0] = c.g[A0][0];
        loop {
            c.cycles += 11;
            c.lq(2, r32(c, A2));
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
            c.sq(2, p_2(r32(c, V1), 0x30));
            c.lwc1(0, r32(c, A3));
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
            c.swc1(0, p_2(r32(c, V1), 0x40));
            c.g[V0][0] = Addr(this).off(0x2c).lw(c);
            c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_9aa0);
            }
        }
    }
    c.cycles += 4;
    c.lq(2, r32(c, T0));
    c.swc1(12, p_2(this, 0x20));
    c.sq(2, p_2(this, 0x10));
    abi::ret(c);
}

/// `void SphereTree_ResetTransform(SphereTree* self)` (0x32c540): copy the 8
/// unit sphere nodes into the tree and install the identity transform and a
/// 1.0 radius.
pub fn sphere_tree_reset_transform(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.g[V0][0] = 0x500000;
    c.g[V1][0] = c.g[A0][0];
    c.g[A2][0] = sx(0x004f_f640);
    c.g[A1][0] = 7;
    loop {
        c.cycles += 7;
        c.lq(2, r32(c, A2));
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
        c.sq(2, r32(c, V1));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x10));
        if !more {
            c.cycles += 8;
            c.g[V0][0] = 0x500000;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[V0][0] = sx(0x004f_f130);
            c.lq(3, r32(c, V0));
            c.swc1(0, p_2(this, 0x90));
            c.sq(3, p_2(this, 0x80));
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_c550);
        }
    }
}

// ---------------------------------------------------------------------------
// Collision.

/// `void CollSphere_Transform(CollSphere* s, const Matrix* m, float scale)`
/// (0x32fd40): transform the sphere centre at `+0x80` by `m` with the VU0
/// macro pipeline, store it at `+0x60`, set `+0x4 = 1` and scale the radius
/// (`+0x90` into `+0x70`).
pub fn coll_sphere_transform(c: &mut Cpu) {
    c.cycles += 21;
    let frame = Frame::open(c, 0x20);
    c.g[V1][0] = 1;
    c.lqc2(8, p_3(c, A0, 0x80));
    c.lqc2(4, abi::r32(c, A1));
    c.lqc2(5, p_3(c, A1, 0x10));
    c.lqc2(6, p_3(c, A1, 0x20));
    c.lqc2(7, p_3(c, A1, 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, frame.at(c, 0x10));
    c.lwc1(0, p_3(c, A0, 0x90));
    c.lq(2, frame.at(c, 0x10));
    c.mul_s(12, 12, 0);
    c.sw(p_3(c, A0, 4), c.g[V1][0]);
    c.sq(2, p_3(c, A0, 0x60));
    c.sq(2, frame.at(c, 0));
    c.swc1(12, p_3(c, A0, 0x70));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(abi::r32(c, SP).wrapping_add(0x20));
    c.pc = ra;
}

/// `void CollSegment_ResetTransform(CollSegment* self)` (0x32e4b8): copy the
/// transformed endpoints (`+0x80`, `+0x90`) back over the local frame
/// (`+0x60`, `+0x70`) and clear the flags at `+0x4`.
pub fn coll_segment_reset_transform(c: &mut Cpu) {
    c.cycles += 6;
    let this = r32(c, A0);
    c.lq(2, this.wrapping_add(0x80));
    c.lq(3, this.wrapping_add(0x90));
    c.sq(2, this.wrapping_add(0x60));
    c.sq(3, this.wrapping_add(0x70));
    c.sw(this.wrapping_add(0x4), 0);
    abi::ret(c);
}

/// `int CollSegment_CollideAABB(CollSegment* self, float* bmin, float* bmax,
/// CollHit* hits)` (0x32e690): slab test of the segment (`origin` at `+0x60`,
/// `dir` at `+0x70`) against the box `[bmin, bmax]`, writing up to two hit
/// records (0x80 bytes each: normal quad at `+0x10`, position at `+0x0`,
/// distance at `+0x40`, flags/indices around `+0x44..+0x70`).
pub fn coll_segment_collide_aabb(c: &mut Cpu) {
    // Prologue: constants and loop state.
    c.cycles += 16;
    let this = r32(c, A0);
    c.g[T1][0] = c.g[A0][0]; // the segment
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 4); // f4 = -1.0 (tmin)
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(1, 5); // f5 = 2.0 (tmax)
    let sp = r32(c, SP).wrapping_sub(0x30);
    c.g[SP][0] = sx(sp);
    c.lwc1(8, gpa(c, -0x2d74)); // f8 = epsilon
    c.g[T0][0] = sx(this.wrapping_add(0x70)); // pd = &dir
    c.mtc1(0, 7); // f7 = 0.0
    c.g[A0][0] = sx(this.wrapping_add(0x60)); // po = &origin
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 6); // f6 = 1.0
    c.g[T2][0] = u64::MAX; // fmin = -1
    c.g[T3][0] = u64::MAX; // fmax = -1
    c.g[V1][0] = 0;
    'outer: loop {
        'axis: {
            c.cycles += 6;
            c.lwc1(3, r32(c, T0));
            c.abs_s(0, 3);
            c.c_lt_s(8, 0); // epsilon < |d|
            if !c.fcc() {
                // Parallel to this axis: skip it.
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                break 'axis;
            }
            {
                c.cycles += 11;
                let po = r32(c, A0);
                c.lwc1(0, po);
                c.lwc1(1, r32(c, A1));
                c.lwc1(2, r32(c, A2));
                c.sub_s(1, 1, 0);
                c.sub_s(2, 2, 0);
                c.div_s(1, 1, 3);
                c.div_s(2, 2, 3);
                c.c_lt_s(2, 1);
                let ordered = !c.fcc();
                c.g[V0][0] = sx(r32(c, V1) << 1); // face = i * 2
                if !ordered {
                    c.cycles += 4;
                    c.mov_s(0, 1);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.mov_s(1, 2);
                    c.mov_s(2, 0);
                }
            }
            c.cycles += 3;
            c.c_lt_s(4, 1); // tmin < t1
            if c.fcc() {
                c.cycles += 2;
                c.mov_s(4, 1);
                c.g[T2][0] = c.g[V0][0];
            }
            c.cycles += 3;
            c.c_lt_s(2, 5); // t2 < tmax
            if c.fcc() {
                c.cycles += 2;
                c.mov_s(5, 2);
                c.g[T3][0] = c.g[V0][0] ^ 0x1;
            }
            c.cycles += 4;
            c.c_lt_s(5, 4); // tmax < tmin
            let empty = c.fcc();
            c.g[V0][0] = 0;
            if empty {
                break 'outer; // miss
            }
            c.cycles += 3;
            c.c_lt_s(5, 7); // tmax < 0
            if c.fcc() {
                break 'outer; // behind
            }
            c.cycles += 4;
            c.c_lt_s(6, 4); // 1 < tmin
            let behind = c.fcc();
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if behind {
                break 'outer; // past the end
            }
        }
        c.cycles += 6;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x4));
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x4));
        c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x4));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
        let next_axis = c.g[V0][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x4));
        if next_axis {
            if event_due(c) {
                deliver_event_here(c, 0x0032_e6d0);
            }
            continue 'outer;
        }
        // All three axes passed: build the hit records.
        // Lazy initialisation of the six box normals.
        'init: {
            c.cycles += 3;
            c.g[V0][0] = Addr(gpa(c, 0x2888)).lw(c);
            let ready = c.g[V0][0] != 0;
            c.g[V1][0] = 0x500000;
            if ready {
                break 'init;
            }
            c.cycles += 31;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 0); // f0 = -1.0
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1); // f1 = 1.0
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xda0)); // &normals[0]
            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0xda0));
            c.g[A0][0] = 0x1;
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x48));
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x58));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x5c), 0);
            c.sw(gpa(c, 0x2888), c.g[A0][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x20), 0);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x24));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x28), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x2c), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x30), 0);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x34));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x38), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x40), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x44), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x50), 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x54), 0);
        }
        // Entry hit (tmin in [0, 1]).
        'enter: {
            c.cycles += 5;
            c.mtc1(0, 1); // f1 = 0.0
            c.c_le_s(1, 4); // 0 <= tmin
            let miss = !c.fcc();
            c.g[T0][0] = 0;
            if miss {
                break 'enter;
            }
            c.cycles += 41;
            c.g[V0][0] = 0x2;
            c.g[V1][0] = u64::MAX;
            c.sw((c.g[A3][0] as u32).wrapping_add(0x44), c.g[V0][0]);
            c.g[A0][0] = 0x500000;
            c.g[V0][0] = 0x500000;
            c.sw((c.g[A3][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
            c.sw((c.g[A3][0] as u32).wrapping_add(0x54), 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xda0));
            c.sw((c.g[A3][0] as u32).wrapping_add(0x50), 0);
            c.g[A1][0] = sx((c.g[T2][0] as u32) << 4);
            c.sw((c.g[A3][0] as u32).wrapping_add(0x58), 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xee0));
            c.sw((c.g[A3][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[A3][0] as u32).wrapping_add(0x60), c.g[V1][0]);
            c.g[T0][0] = 0x1;
            c.sw((c.g[A3][0] as u32).wrapping_add(0x64), c.g[V1][0]);
            c.lq(2, c.g[A0][0] as u32);
            c.sq(2, (c.g[A3][0] as u32).wrapping_add(0x20));
            c.lq(3, c.g[A0][0] as u32);
            c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0x68));
            c.sq(3, (c.g[A3][0] as u32).wrapping_add(0x30));
            c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0x6c));
            c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0x70));
            c.lq(2, c.g[A1][0] as u32);
            c.sq(2, (c.g[A3][0] as u32).wrapping_add(0x10));
            c.mfc1(3, 4); // v1 = tmin bits
            c.lqc2(4, (c.g[T1][0] as u32).wrapping_add(0x70));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp.wrapping_add(0x20));
            c.lq(2, sp.wrapping_add(0x20));
            c.sq(2, sp.wrapping_add(0x10));
            c.lqc2(3, (c.g[T1][0] as u32).wrapping_add(0x60));
            c.lqc2(4, sp.wrapping_add(0x10));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp.wrapping_add(0x20));
            c.lq(2, sp.wrapping_add(0x20));
            c.swc1(4, (c.g[A3][0] as u32).wrapping_add(0x40));
            c.sq(2, c.g[A3][0] as u32);
            c.sq(2, sp);
        }
        // Exit hit (tmax < 1).
        'leave: {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0); // f0 = 1.0
            c.c_lt_s(5, 0); // tmax < 1
            let exit_hit = c.fcc();
            c.g[V1][0] = sx((c.g[T0][0] as u32) << 7);
            if !exit_hit {
                break 'leave;
            }
            c.cycles += 42;
            c.g[V0][0] = 0x2;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[A0][0] = u64::MAX;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
            c.g[A1][0] = 0x500000;
            c.g[V0][0] = 0x500000;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x54), 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xda0));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x50), 0);
            c.g[A2][0] = sx((c.g[T3][0] as u32) << 4);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x58), 0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xee0));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x5c), c.g[A0][0]);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x60), c.g[A0][0]);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[A0][0]);
            c.lq(2, c.g[A1][0] as u32);
            c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x20));
            c.lq(4, c.g[A1][0] as u32);
            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x70));
            c.sq(4, (c.g[V1][0] as u32).wrapping_add(0x30));
            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x68));
            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x6c));
            c.lq(2, c.g[A2][0] as u32);
            c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.mfc1(4, 5); // a0 = tmax bits
            c.lqc2(4, (c.g[T1][0] as u32).wrapping_add(0x70));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp.wrapping_add(0x20));
            c.lq(2, sp.wrapping_add(0x20));
            c.sq(2, sp.wrapping_add(0x10));
            c.lqc2(3, (c.g[T1][0] as u32).wrapping_add(0x60));
            c.lqc2(4, sp.wrapping_add(0x10));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp.wrapping_add(0x20));
            c.lq(2, sp.wrapping_add(0x20));
            c.sq(2, sp);
            c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x40));
            c.sq(2, c.g[V1][0] as u32);
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[T0][0];
        break 'outer;
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp.wrapping_add(0x30));
    abi::ret(c);
}

/// `float CollHull_GetDiameter(CollHull* hull)` (0x0032f8c0): -1 unless the
/// hull has a shape, else twice its bounding sphere's radius.
pub fn coll_hull_get_diameter(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(abi::arg(c, 0)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
        c.mtc1(1, 0);
    } else {
        c.cycles += 4;
        c.g[V0][0] = c.lw(abi::arg(c, 0).wrapping_add(0x60)); // the sphere
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
        c.add_s(0, 0, 0);
    }
    abi::ret(c);
}

/// `void CollSphere_ResetTransform(CollSphere* s)` (0x0032fd98): the
/// transform at +0x80/+0x90 becomes the current one at +0x60/+0x70.
pub fn coll_sphere_reset_transform(c: &mut Cpu) {
    c.cycles += 6;
    let s = abi::arg(c, 0);
    c.lq(2, p_4(s, 0x80));
    c.lwc1(0, p_4(s, 0x90));
    c.sq(2, p_4(s, 0x60));
    c.swc1(0, p_4(s, 0x70));
    c.sw(p_4(s, 0x4), 0);
    abi::ret(c);
}

/// `void RiderSphereTree_Init(RiderSphereTree* self, int type, int count,
/// int* ids, float* radii, int user, int radius)` (0x3299C8): fill `count`
/// (id, radius) entries at `+0x40`/`+0x44`, 0x20 bytes apart.
pub fn rider_sphere_tree_init(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.sw(ea(c.g[A0][0], 0x2c), c.g[A2][0]);
    c.g[V0][0] |= 0xffff;
    c.g[T2][0] = 0;
    let empty = (c.g[A2][0] as i64) <= 0;
    c.sw(ea(c.g[A0][0], 0x28), c.g[V0][0]);
    if !empty {
        c.cycles += 2;
        c.g[V1][0] = c.g[A0][0];
        loop {
            c.cycles += 11;
            c.g[V0][0] = c.lw(c.g[A3][0] as u32);
            c.g[T2][0] = sx(ea(c.g[T2][0], 1));
            c.g[A3][0] = sx(ea(c.g[A3][0], 4));
            c.sw(ea(c.g[V1][0], 0x44), c.g[V0][0]);
            c.lwc1(0, c.g[T0][0] as u32);
            c.g[T0][0] = sx(ea(c.g[T0][0], 4));
            c.swc1(0, ea(c.g[V1][0], 0x40));
            c.g[V0][0] = c.lw(ea(c.g[A0][0], 0x2c));
            c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(ea(c.g[V1][0], 0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_99e8);
            }
        }
    }
    c.cycles += 4;
    c.sw(ea(c.g[A0][0], 0x24), c.g[T1][0]);
    c.sw(ea(c.g[A0][0], 0), c.g[A1][0]);
    c.swc1(12, ea(c.g[A0][0], 0x20));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Collision.

/// `int CollHull_CollideSphereTree(CollHull* self, CollSphere* sphere,
/// Hit* hit)` (0x32fa30): collide the sphere with the hull's sphere tree and,
/// on a hit, store the hit direction length and normalise the direction.
pub fn coll_hull_collide_sphere_tree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A2][0] = sx(p(r32(c, S0), 0x10));
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = Addr(p(r32(c, V0), 0x60)).lw(c);
        call_ptr(c, 0x0032_7f18, 0x0032_fa58)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 19;
            c.lqc2(3, p(r32(c, S0), 0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 0);
            c.swc1(0, p(r32(c, S0), 0x40));
            c.g[V0][0] = 1;
            c.mfc1(3, 0);
            c.qmtc2(3, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, p(r32(c, S0), 0x10));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            c.sqc2(5, p(r32(c, S0), 0x10));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int CollSphere_PlaneDistance(CollSphere* self, Vec4* plane, Vec4* p,
/// float* out)` (0x330360): the signed distance of the sphere to the plane,
/// written to `out`, and whether its magnitude is inside the radius.
pub fn coll_sphere_plane_distance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S0][0];
        callee::VEC3_NEG_DOT.call(c, 0x0033_038c)?;
        c.lqc2(3, r32(c, S0));
        c.lqc2(5, p(r32(c, S1), 0x60));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 1);
        c.add_s(1, 1, 0);
        c.g[V0][0] = 1;
        c.abs_s(2, 1);
        c.swc1(1, r32(c, S2));
        c.lwc1(0, p(r32(c, S1), 0x70));
        c.c_lt_s(2, 0);
        if !c.fcc() {
            c.g[V0][0] = 0;
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

// ---------------------------------------------------------------------------
// Sphere tree collision.

/// `int SphereTree_CollideSphere(self, const Sphere* s, float f, void* a2,
/// void* a3)` (0x32c898): refresh the tree's node offset, then test a sphere
/// against the node at_2 `self + 0x80`.
pub fn sphere_tree_collide_sphere(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        let self_ = abi::arg(c, 0);
        let a1 = abi::arg64(c, 1);
        let a2 = abi::arg64(c, 2);
        let a3 = abi::arg64(c, 3);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx(self_);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = a2;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = a3;
        frame.save_fpr(c, 20, 0x50);
        c.g[S3][0] = a1;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at_2(c, S0, 0x98));
        c.g[V0][0] = c.lw(at_2(c, A0, 8));
        let no_offset = c.g[V0][0] == 0;
        c.mov_s(20, 12);
        if !no_offset {
            c.cycles += 4;
            call_ptr(c, callee::SPHERE_TREE_COMPUTE_OFFSET, 0x0032_c8dc)?;
            c.g[V1][0] = c.lw(at_2(c, S0, 0x98));
            c.sw(at_2(c, V1, 0x28), c.g[V0][0]);
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S3][0];
        c.mov_s(12, 20);
        c.g[T1][0] = c.g[S1][0];
        c.g[T2][0] = c.g[S2][0];
        c.g[A2][0] = sx(p(r32(c, A0), 0x80));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, callee::SPHERE_TREE_COLLIDE_SPHERE_NODE, 0x0032_c908)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `int SphereTree_CollideAABB(self, const Box* b, void* a2, void* a3,
/// void* a4)` (0x32c928): refresh the tree's node offset, then test an AABB
/// against the node at_2 `self + 0x80`.
pub fn sphere_tree_collide_aabb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        let self_ = abi::arg(c, 0);
        let a1 = abi::arg64(c, 1);
        let a2 = abi::arg64(c, 2);
        let a3 = abi::arg64(c, 3);
        let a4 = abi::arg64(c, 4);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(self_);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = a2;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = a3;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = a4;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at_2(c, S0, 0x98));
        c.g[V0][0] = c.lw(at_2(c, A0, 8));
        let no_offset = c.g[V0][0] == 0;
        c.g[S4][0] = a1;
        if !no_offset {
            c.cycles += 4;
            call_ptr(c, callee::SPHERE_TREE_COMPUTE_OFFSET, 0x0032_c96c)?;
            c.g[V1][0] = c.lw(at_2(c, S0, 0x98));
            c.sw(at_2(c, V1, 0x28), c.g[V0][0]);
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[T2][0] = c.g[S2][0];
        c.g[T3][0] = c.g[S3][0];
        c.g[A3][0] = sx(p(r32(c, A0), 0x80));
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        call_ptr(c, callee::SPHERE_TREE_COLLIDE_AABB_NODE, 0x0032_c998)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int CollSphere_CollideSphereTree(self, SphereTree* tree, Hit* hit)`
/// (0x32fdb0): test the collider's sphere against the tree and, on a hit,
/// normalise the hit normal and return its length.
pub fn coll_sphere_collide_sphere_tree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let tree = abi::arg64(c, 1);
        let hit = abi::arg64(c, 2);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[A0][0] = tree;
        frame.save_ra(c, 0);
        c.g[S0][0] = hit;
        c.g[A1][0] = sx(p(self_, 0x60));
        c.g[A2][0] = sx(p(r32(c, S0), 0x10));
        c.lwc1(12, at_2(c, V0, 0x70));
        c.g[A3][0] = c.g[S0][0];
        call_ptr(c, callee::SPHERE_TREE_COLLIDE_SPHERE, 0x0032_fddc)?;
        let miss = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !miss {
            c.cycles += 19;
            c.lqc2(3, at_2(c, S0, 0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 0);
            c.swc1(0, at_2(c, S0, 0x40));
            c.g[V0][0] = 1;
            c.mfc1(3, 0);
            c.qmtc2(3, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, at_2(c, S0, 0x10));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            c.sqc2(5, at_2(c, S0, 0x10));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub fn ov_screen_save_records_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        ov_screen_set_widget_visibility(c, c.g[A2][0])
    })
}

// ---------------------------------------------------------------------------
// Collision queries (VU0: the original's COP2 ops are issued in order).

/// The length/normalise sequence shared by the sphere/hull triangle tests:
/// `v` at_3 `sp` is the collision point, `out_len` (s0) receives |v| and
/// `out_dir` (s1) the normalised v.
pub(crate) fn collision_direction(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 22;
    c.lqc2(3, c.g[SP][0] as u32);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(2, 22);
    c.mtc1(2, 0);
    c.swc1(0, c.g[S0][0] as u32);
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(2, c.g[S1][0] as u32);
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    frame_restore_s0(c);
    Ok(())
}

#[inline(always)]
pub(crate) fn frame_restore_s0(c: &mut Cpu) {
    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
}

pub(crate) fn coll_sphere_tree_common(c: &mut Cpu, collide: u32, ret: u32) -> crate::call::Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    c.g[S0][0] = c.g[T3][0];
    frame.save_ra(c, 0x30);
    c.g[S1][0] = c.g[T2][0];
    c.g[T2][0] = c.g[T1][0];
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x60).lw(c);
    c.g[T1][0] = c.g[SP][0];
    call_ptr(c, collide, ret)?;
    c.g[A0][0] = c.g[V0][0];
    if c.g[A0][0] != 0 {
        collision_direction(c)?;
    } else {
        frame_restore_s0(c);
    }
    c.cycles += 5;
    c.g[V0][0] = c.g[A0][0];
    frame.restore(c, S1, 0x40);
    c.g[RA][0] = Addr(c.g[SP][0] as u32).off(0x30).ld(c);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = c.g[RA][0] as u32;
    Ok(())
}

/// `int CollHull_CollideTriangle2Sided(...)` (0x32f990): the hull-variant
/// collision with the contact direction.
pub fn coll_hull_collide_triangle2_sided(c: &mut Cpu) {
    guest(c, |c| {
        coll_sphere_tree_common(c, table::COLL_HULL_TRIANGLE2, 0x0032_f9b8)
    })
}

/// `void SphereTree_CollideSphereTree(self, other, ..., a2, a3)` (0x32cb58):
/// refresh both trees' cached bounds and descend.
pub fn sphere_tree_collide_sphere_tree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        c.g[A0][0] = lw_2(c, S0, 0x98);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        let skip = c.g[V0][0] == 0;
        if !skip {
            c.cycles += 4;
            call_ptr(c, table::COLL_SPHERE_CALC, 0x0032_cb94)?;
            c.g[V1][0] = lw_2(c, S0, 0x98);
            Addr(c.g[V1][0] as u32).off(0x28).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[A0][0] = lw_2(c, S1, 0x98);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        if c.g[V0][0] == 0 {
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 5;
            call_ptr(c, table::COLL_SPHERE_CALC, 0x0032_cbb4)?;
            c.g[V1][0] = lw_2(c, S1, 0x98);
            Addr(c.g[V1][0] as u32).off(0x28).sw(c, c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 14;
        c.g[A1][0] = c.g[S1][0];
        c.g[T1][0] = c.g[S2][0];
        c.g[T2][0] = c.g[S3][0];
        c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x80));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        callee::SPHERE_TREE_COLLIDE_TREE_NODE.call(c, 0x0032_cbdc)?;
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
// Collision objects.

/// `CollSphere* CollSphere_ctor(CollSphere* self, const Vec3* pos, int flags,
/// float radius)` (0x32fb48): set the vtable and `state`, copy the centre and
/// radius, and build the `pos ± r` AABBs (`w = 1`).
pub fn coll_sphere_ctor(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x490000;
    c.g[A3][0] = c.g[A0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x19b0));
    open(c, 0x10);
    Addr(c.g[A3][0] as u32).off(0x50).sw(c, c.g[V0][0]);
    Addr(c.g[A3][0] as u32).off(8).sw(c, 0);
    let no_flag = c.g[A2][0] == 0;
    Addr(c.g[A3][0] as u32).off(4).sw(c, 0);
    if no_flag {
        c.cycles += 2;
        c.g[V0][0] = 1;
        Addr(c.g[A3][0] as u32).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 2;
        Addr(c.g[A3][0] as u32).sw(c, 0);
    }
    c.cycles += 35;
    c.lq(3, c.g[A1][0] as u32);
    c.g[V0][0] = c.g[A3][0];
    c.swc1(12, at_4(c.g[A3][0] as u32, 0x70));
    c.sq(3, at_4(c.g[A3][0] as u32, 0x60));
    c.sq(3, at_4(c.g[A3][0] as u32, 0x80));
    c.swc1(12, at_4(c.g[A3][0] as u32, 0x90));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.lwc1(1, at_4(c.g[A1][0] as u32, 4));
    c.lwc1(2, at_4(c.g[A1][0] as u32, 8));
    c.lwc1(0, c.g[A1][0] as u32);
    c.add_s(1, 1, 12);
    c.add_s(2, 2, 12);
    c.swc1(3, at_4(c.g[SP][0] as u32, 0xc));
    c.add_s(0, 0, 12);
    c.swc1(1, at_4(c.g[SP][0] as u32, 4));
    c.swc1(2, at_4(c.g[SP][0] as u32, 8));
    c.swc1(0, c.g[SP][0] as u32);
    c.lq(3, c.g[SP][0] as u32);
    c.sq(3, at_4(c.g[A3][0] as u32, 0x20));
    c.lwc1(1, at_4(c.g[A1][0] as u32, 4));
    c.lwc1(2, at_4(c.g[A1][0] as u32, 8));
    c.lwc1(0, c.g[A1][0] as u32);
    c.sub_s(1, 1, 12);
    c.sub_s(2, 2, 12);
    c.sub_s(0, 0, 12);
    c.swc1(1, at_4(c.g[SP][0] as u32, 4));
    c.swc1(2, at_4(c.g[SP][0] as u32, 8));
    c.swc1(0, c.g[SP][0] as u32);
    c.lq(3, c.g[SP][0] as u32);
    c.sq(3, at_4(c.g[A3][0] as u32, 0x30));
    c.lq(4, c.g[A1][0] as u32);
    c.sq(4, at_4(c.g[A3][0] as u32, 0x40));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Sphere tree.

/// `int SphereTree_CollideTriangle(self, a1, a2, a3, t0, t1, t2)` (0x32ca78):
/// refresh the tree's cached bound, collide the triangle and report whether
/// the closest hit is in front.
pub fn spheretree_collide_triangle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[T1][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[T2][0];
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S6, 0x20);
        c.g[S5][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S6][0] = c.g[A3][0];
        c.g[A0][0] = Addr(p(r32(c, S0), 0x98)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, A0), 8)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, 0x0032_df28, 0x0032_cacc)?;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x98)).lw(c);
            Addr(p(r32(c, V1), 0x28)).sw(c, c.g[V0][0]);
        }
        c.cycles += 21;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0032_db40, 0x0032_cae0)?;
        Addr(p(r32(c, S0), 0x94)).sw(c, 0);
        c.g[A1][0] = c.g[S4][0];
        c.sw(r32(c, SP), c.g[S2][0]);
        c.g[A2][0] = c.g[S5][0];
        c.sw(p(r32(c, SP), 8), c.g[S3][0]);
        c.g[A3][0] = c.g[S6][0];
        c.g[T0][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[T1][0] = sx(p(r32(c, S0), 0x80));
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        call_ptr(c, 0x0032_d470, 0x0032_cb10)?;
        c.lwc1(1, p(r32(c, S0), 0x94));
        c.mtc1(0, 0);
        c.c_lt_s(1, 0);
        let hit = c.fcc();
        c.g[V0][0] = 1;
        if !hit {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void func_0032DA40(int n, int* keys, int* vals)` (0x32da40): shell sort
/// (descending on `keys`, `vals` carried along) using the gap table at
/// `D_0044AF98`, starting with the first gap below 2.
pub fn shell_sort_desc_key_val(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x45_0000;
    c.g[T5][0] = c.g[A0][0]; // n
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5068)); // D_0044AF98
    c.g[T2][0] = c.g[A1][0]; // keys
    c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T5][0] as i64)) as u64;
    let done = c.g[V0][0] == 0;
    c.g[V1][0] = 0x1;
    if !done {
        // k = 1; while (D[k] < n) k++
        c.cycles += 2;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T5][0] as i64)) as u64;
            let small = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if !small {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_da68);
            }
        }
    }
    c.cycles += 5;
    c.g[V0][0] = 0x45_0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5068));
    c.g[T6][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_sub(4));
    loop {
        // gap = D[--k]; the insertion pass only runs for gap < count
        c.cycles += 4;
        c.g[T0][0] = c.lw(c.g[T6][0] as u32);
        c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T5][0] as i64)) as u64;
        let no_pass = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[T0][0];
        if !no_pass {
            loop {
                c.cycles += 8;
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[T3][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                c.g[T4][0] = c.lw(c.g[V1][0] as u32); // vals[i]
                c.g[T1][0] = c.lw(c.g[V0][0] as u32); // key = keys[i]
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = sx((c.g[A3][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    let ordered = c.g[V1][0] == 0;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    if ordered {
                        break;
                    }
                    c.cycles += 9;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]); // keys[j+gap] = keys[j]
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    let more = (c.g[A3][0] as i64) >= 0;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]); // vals[j+gap] = vals[j]
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_dac8);
                    }
                }
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[A1][0] = c.g[T3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[T5][0] as i64)) as u64;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.sw(c.g[V1][0] as u32, c.g[T1][0]); // keys[j+gap] = key
                let next_i = c.g[A0][0] != 0;
                c.sw(c.g[V0][0] as u32, c.g[T4][0]); // vals[j+gap] = val
                if !next_i {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_daa8);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T0][0] as i64) < 2) as u64;
        let next_gap = c.g[V0][0] == 0;
        c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_sub(4));
        if !next_gap {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_da98);
        }
    }
}

// ---------------------------------------------------------------------------
// Collision.

/// `bool CollSegment_OverlapsAABB(Segment* seg, Vec3* boxMin, Vec3* boxMax)`
/// (0x32e288): the segment's centre (`+0x60`) plus its extent (`+0x70`) is its
/// AABB; the test rejects the pair on each axis as the original does.
pub fn coll_segment_overlaps_aabb(c: &mut Cpu) {
    c.cycles += 12;
    let seg = r32v(c, A0);
    let lo = r32v(c, A1);
    let hi = r32v(c, A2);
    c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0x20));
    c.lqc2(3, at_5(seg, 0x60));
    c.lqc2(4, at_5(seg, 0x70));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, spa(c, 0x10));
    // First: the segment min against the box min.
    c.lwc1(2, at_5(seg, 0x60));
    c.lwc1(1, lo);
    c.lq(2, spa(c, 0x10));
    c.c_lt_s(2, 1);
    let mut separated = false;
    let skips = !c.fcc();
    c.sq(2, spa(c, 0));
    if !skips {
        c.cycles += 5;
        c.lwc1(0, spa(c, 0));
        c.c_lt_s(0, 1);
        separated = c.fcc();
    }
    // Then the box max against the segment min/max.
    if !separated {
        c.cycles += 5;
        c.lwc1(1, hi);
        c.c_lt_s(1, 2);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 0));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(1, 0);
            separated = c.fcc();
        }
    }
    // y.
    if !separated {
        c.cycles += 6;
        c.lwc1(2, at_5(seg, 0x64));
        c.lwc1(1, at_5(lo, 4));
        c.c_lt_s(2, 1);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 4));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(0, 1);
            separated = c.fcc();
        }
    }
    if !separated {
        c.cycles += 5;
        c.lwc1(1, at_5(hi, 4));
        c.c_lt_s(1, 2);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 4));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(1, 0);
            separated = c.fcc();
        }
    }
    // z.
    if !separated {
        c.cycles += 6;
        c.lwc1(2, at_5(seg, 0x68));
        c.lwc1(1, at_5(lo, 8));
        c.c_lt_s(2, 1);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 8));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(0, 1);
            separated = c.fcc();
        }
    }
    if !separated {
        c.cycles += 5;
        c.lwc1(1, at_5(hi, 8));
        c.c_lt_s(1, 2);
        let skips = !c.fcc();
        c.lwc1(0, spa(c, 8));
        if !skips {
            c.cycles += 4;
            c.c_lt_s(1, 0);
            separated = c.fcc();
        }
    }
    if separated {
        c.g[V0][0] = 0;
    } else {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0x20));
    abi::ret(c);
}

/// `bool CollSphere_HitsTriangleEdges(Sphere*, Vec3 a, Vec3 b, Vec3 c)`
/// (0x330250): for each of the three triangle edges, clamp the sphere centre
/// onto the edge and compare the distance with the sphere radius.
pub fn coll_sphere_hits_triangle_edges(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let sphere = r32v(c, A0);
        c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0xc0));
        c.sq(17, spa(c, 0xa0));
        c.sq(18, spa(c, 0x90));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, spa(c, 0x80));
        c.sq(20, spa(c, 0x70));
        c.sq(16, spa(c, 0xb0));
        c.sd(spa(c, 0x60), c.g[RA][0]);
        // The three vertices are passed by value; spill them to the frame.
        c.lq(2, r32v(c, A1));
        c.g[A1][0] = 0;
        c.sq(2, spa(c, 0x10));
        c.lq(2, r32v(c, A2));
        c.sq(2, spa(c, 0x20));
        c.lq(3, r32v(c, A3));
        c.sq(3, spa(c, 0x30));
        c.g[S2][0] = sx(spa(c, 0x10));
        c.g[S3][0] = 3;
        c.g[S4][0] = sx(spa(c, 0x40));
        // s0 walks the edges (0, 1, 2), a1 the start vertex, s3 the vertex
        // count for the div below.
        c.g[S0][0] = 1;
        loop {
            c.cycles += 2;
            c.cycles += 32;
            c.div(0, 16, 19);
            c.g[A1][0] = sx(r32v(c, A1) << 4);
            c.g[A1][0] = sx(r32v(c, S2).wrapping_add(r32v(c, A1)));
            c.g[A0][0] = c.g[S4][0];
            c.g[A3][0] = sx(at_5(sphere, 0x60));
            c.g[A2][0] = c.hi[0];
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 4);
            c.g[A2][0] = sx(r32v(c, S2).wrapping_add(r32v(c, A2)));
            callee::CLOSEST_POINT_ON_SEGMENT.call(c, 0x0033_02cc)?;
            // d = closest - centre; |d|^2 against r^2 (VU0 dot product).
            c.lq(2, spa(c, 0x40));
            c.sq(2, spa(c, 0));
            c.lqc2(3, at_5(sphere, 0x60));
            c.lqc2(4, spa(c, 0));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, spa(c, 0x50));
            c.lq(2, spa(c, 0x50));
            c.lwc1(1, at_5(sphere, 0x70));
            c.sq(2, spa(c, 0x40));
            c.lqc2(3, spa(c, 0x40));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 0);
            c.c_lt_s(0, 1);
            let hit = c.fcc();
            c.g[V0][0] = 1;
            if hit {
                break;
            }
            c.cycles += 4;
            let i = c.g[S0][0];
            c.g[A1][0] = i;
            c.g[V0][0] = ((i as i64) < 3i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((i as u32).wrapping_add(1));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0033_02a0);
                }
                continue;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
            break;
        }
        c.cycles += 8;
        c.lq(16, spa(c, 0xb0));
        c.lq(17, spa(c, 0xa0));
        c.lq(18, spa(c, 0x90));
        c.lq(19, spa(c, 0x80));
        c.lq(20, spa(c, 0x70));
        c.g[RA][0] = c.ld(spa(c, 0x60));
        c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0xc0));
        abi::ret(c);
        Ok(())
    })
}

/// `bool CollSphere_OverlapsAABB(const CollSphere* s, const Vec4* mn,
/// const Vec4* mx)` (0x32fc08): the sphere's capsule (`+0x60` centre,
/// `+0x70` radius) grown by the radius on every axis is tested against the
/// box; true when they overlap.
pub fn coll_sphere_overlaps_aabb(c: &mut Cpu) {
    'b_32fd38: {
        'b_32fc78: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.stw_2(SP, 0xc, 0u64);
            c.lwc1(0, c.pa(A0, 0x70));
            c.swc1(0, c.pa(SP, 0x8));
            c.swc1(0, c.pa(SP, 0));
            c.swc1(0, c.pa(SP, 0x4));
            c.lqc2(3, c.pa(A0, 0x60));
            c.lqc2(4, c.pa(SP, 0));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, c.pa(SP, 0x20));
            c.lq(2, c.pa(SP, 0x20));
            c.sq(2, c.pa(SP, 0x10));
            c.lqc2(3, c.pa(A0, 0x60));
            c.lqc2(4, c.pa(SP, 0));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, c.pa(SP, 0x30));
            c.lwc1(1, c.pa(A1, 0));
            c.lwc1(2, c.pa(SP, 0x10));
            c.lq(2, c.pa(SP, 0x30));
            c.c_lt_s(2, 1);
            let t = !c.fcc();
            c.sq(2, c.pa(SP, 0x20));
            if t {
                break 'b_32fc78;
            } else {
                c.cycles += 5;
                c.lwc1(0, c.pa(SP, 0x20));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fc78;
                }
            }
        }
        'b_32fc9c: {
            c.cycles += 5;
            c.lwc1(1, c.pa(A2, 0));
            c.c_lt_s(1, 2);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x20));
            if t {
                break 'b_32fc9c;
            } else {
                c.cycles += 4;
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fc9c;
                }
            }
        }
        'b_32fcc4: {
            c.cycles += 6;
            c.lwc1(1, c.pa(A1, 0x4));
            c.lwc1(2, c.pa(SP, 0x14));
            c.c_lt_s(2, 1);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x24));
            if t {
                break 'b_32fcc4;
            } else {
                c.cycles += 4;
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fcc4;
                }
            }
        }
        'b_32fce8: {
            c.cycles += 5;
            c.lwc1(1, c.pa(A2, 0x4));
            c.c_lt_s(1, 2);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x24));
            if t {
                break 'b_32fce8;
            } else {
                c.cycles += 4;
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fce8;
                }
            }
        }
        'b_32fd10: {
            c.cycles += 6;
            c.lwc1(1, c.pa(A1, 0x8));
            c.lwc1(2, c.pa(SP, 0x18));
            c.c_lt_s(2, 1);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x28));
            if t {
                break 'b_32fd10;
            } else {
                c.cycles += 4;
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fd10;
                }
            }
        }
        'b_32fd34: {
            c.cycles += 5;
            c.lwc1(1, c.pa(A2, 0x8));
            c.c_lt_s(1, 2);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x28));
            if t {
                break 'b_32fd34;
            } else {
                c.cycles += 4;
                c.c_lt_s(1, 0);
                let t = c.fcc();
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_32fd38;
                } else {
                    break 'b_32fd34;
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x1;
        break 'b_32fd38;
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    return;
}

pub(crate) mod stree {

    pub const CENTRE: u32 = 0x10;

    pub const RADIUS: u32 = 0x20;

    pub const KID_MASK: u32 = 0x28;

    pub const KID_COUNT: u32 = 0x2c;

    /// Kids: 0x20 bytes each, `{ Vec4 pos; float radius; }`.
    pub const KIDS: u32 = 0x30;
}

// ---------------------------------------------------------------------------
// RiderSphereTree_CollideTriangle

/// `bool VU0_TriangleIntersect(a, b, c, point)` (0x32b6a8).
pub(crate) const TRI_INTERSECT: Callee = Callee {
    addr: 0x0032_b6a8,
    f: ee::s10::VU0_TriangleIntersect,
};

/// `a - b` on VU0 (vf5 = vf3 - vf4), spilled to `tmp` and copied through v0
/// to `dst`.
pub(crate) fn vsub_spill(c: &mut Cpu, a: u32, b: u32, tmp: u32, dst: u32) {
    use ssx3_ee_rt::vu0::Src;
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, tmp);
    c.lq(V0, tmp);
    c.sq(V0, dst);
}

/// The VU0 dot product of the vectors at `a` and `b` (xyz) into GPR `dst`
/// (`qmfc2` of vf4).
pub(crate) fn vdot(c: &mut Cpu, a: u32, b: u32, dst: usize) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(dst, 4);
}

/// The VU0 length of the vector at `a` (`sqrt` into Q) into v1.
pub(crate) fn vlen(c: &mut Cpu, a: u32) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.lqc2(3, a);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V1, 22);
}

/// The FPU registers of a sphere-vs-segment test: projection `t`, segment
/// length, squared centre distance, squared radius and a zero.
pub(crate) struct SegRegs {
    t: usize,
    len: usize,
    d2: usize,
    r2: usize,
    zero: usize,
}

pub(crate) const SEG_MAIN: SegRegs = SegRegs {
    t: 2,
    len: 4,
    d2: 5,
    r2: 3,
    zero: 1,
};

pub(crate) const SEG_KID: SegRegs = SegRegs {
    t: 1,
    len: 3,
    d2: 4,
    r2: 2,
    zero: 22,
};

/// The tail of a sphere-vs-segment test (f0 = dot(centre - start, edge)):
/// `t = f0 / len`; v0 = 1 when `t >= 0`, the centre is within the radius of
/// the line (`d2 - t^2 <= r2`) and the entry point `t - sqrt(r2 - (d2 -
/// t^2))` is not past the end.
pub(crate) fn seg_tail(c: &mut Cpu, r: &SegRegs) {
    c.div_s(r.t, 0, r.len);
    c.c_lt_s(r.t, r.zero);
    let behind = c.fcc();
    c.g[V0][0] = 0;
    if behind {
        return;
    }
    c.cycles += 5;
    c.mul_s(0, r.t, r.t);
    c.sub_s(0, r.d2, 0);
    c.c_lt_s(r.r2, 0);
    if c.fcc() {
        return;
    }
    c.cycles += 8;
    c.sub_s(0, r.r2, 0);
    c.g[V0][0] = 1;
    c.sqrt_s(0, 0);
    c.sub_s(0, r.t, 0);
    c.c_lt_s(r.len, 0);
    if c.fcc() {
        c.g[V0][0] = 0;
    }
}

/// Sphere vs the edge from vertex a (s2) to `end`; `cd` = centre - a in the
/// frame, the edge is spilled at `e_tmp`/`e`.
pub(crate) fn seg_from_a(
    c: &mut Cpu,
    end: u32,
    e_tmp: u32,
    e: u32,
    cd: u32,
    r: &SegRegs,
    main: bool,
) {
    c.lqc2(3, end);
    c.lqc2(4, r32(c, S2));
    c.vu0().sub(0b1111, 5, 3, ssx3_ee_rt::vu0::Src::Vec(4));
    c.sqc2(5, e_tmp);
    if main {
        c.mtc1(0, 1);
    }
    c.lq(V0, e_tmp);
    c.sq(V0, e);
    vlen(c, e);
    c.mtc1(V1, r.len);
    vdot(c, cd, e, V0);
    c.mtc1(V0, 0);
}

/// The hit: push = n * (r - dist) to `*out_push`, the contact point (at
/// sp+0) to `*out_point`, also kept at sp+0x10. `kid` selects the
/// registers the kid loop's copy uses.
pub(crate) fn sphere_tri_hit(c: &mut Cpu, kid: bool) {
    use ssx3_ee_rt::vu0::{Src, X};
    let s = sp(c);
    let (push, point, scale, ptr0) = if kid {
        (V1, V0, V0, A0)
    } else {
        (A0, V1, V1, A1)
    };
    c.sub_s(0, 21, 20);
    if kid {
        c.mov_s(23, 20);
    } else {
        c.g[V0][0] = 1;
    }
    c.mfc1(scale, 0);
    c.lqc2(4, r32(c, S1));
    c.qmtc2(scale, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x20));
    c.lq(push, s.wrapping_add(0x20));
    c.g[ptr0][0] = c.lw(s.wrapping_add(0x70));
    c.lq(point, s);
    c.sq(push, r32(c, ptr0));
    c.g[A1][0] = c.lw(s.wrapping_add(0x74));
    c.sq(point, r32(c, A1));
    c.sq(push, s.wrapping_add(0x10));
}

/// One kid of the sphere tree against the triangle (s0 = the kid, fp = the
/// tree + 0x20 * index): when its plane distance is within its radius and
/// below the best so far and it touches the triangle (projected centre
/// inside, near vertex a, or near one of the three edges), it becomes the
/// hit.
pub(crate) fn sphere_tree_kid(c: &mut Cpu) -> Flow {
    use ssx3_ee_rt::vu0::{Src, X};
    let s = sp(c);
    let kid = r32(c, S0);
    c.cycles += 21;
    vsub_spill(
        c,
        kid,
        r32(c, S2),
        s.wrapping_add(0x20),
        s.wrapping_add(0x10),
    );
    c.lwc1(21, kid.wrapping_add(0x10));
    vdot(c, r32(c, S1), s.wrapping_add(0x10), V1);
    c.mtc1(V1, 20);
    c.abs_s(0, 20);
    c.c_lt_s(0, 21);
    if !c.fcc() {
        return Ok(());
    }
    c.cycles += 4;
    c.c_lt_s(20, 23);
    if !c.fcc() {
        return Ok(());
    }

    // The projected centre inside the triangle?
    c.cycles += 24;
    c.mfc1(V0, 20);
    c.lqc2(4, r32(c, S1));
    c.qmtc2(V0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x30));
    c.lwc1(12, r32(c, GP).wrapping_sub(0x2dd8));
    c.lq(V0, s.wrapping_add(0x30));
    c.g[S6][0] = c.g[S0][0];
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S5][0];
    c.sq(V0, s.wrapping_add(0x20));
    c.g[A2][0] = c.g[S7][0];
    c.lqc2(3, kid);
    c.lqc2(4, s.wrapping_add(0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x30));
    c.g[A3][0] = c.g[SP][0];
    c.lq(V0, s.wrapping_add(0x30));
    c.sq(T0, s.wrapping_add(0x80));
    c.sq(V0, s);
    c.sq(V0, s.wrapping_add(0x10));
    TRI_INTERSECT.call(c, 0x0032_a6b8)?;
    let inside = c.g[V0][0] != 0;
    c.lq(T0, s.wrapping_add(0x80));
    'touch: {
        if inside {
            c.cycles += 2;
            c.g[V1][0] = 1;
            break 'touch;
        }
        // Near vertex a?
        c.cycles += 20;
        c.lqc2(3, r32(c, FP).wrapping_add(0x30));
        c.lqc2(4, r32(c, S2));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, s.wrapping_add(0x20));
        c.mul_s(2, 21, 21);
        c.lq(V0, s.wrapping_add(0x20));
        c.sq(V0, s.wrapping_add(0x10));
        vdot(c, s.wrapping_add(0x10), s.wrapping_add(0x10), V1);
        c.mtc1(V1, 4);
        c.c_lt_s(4, 2);
        c.g[V1][0] = 1;
        if c.fcc() {
            break 'touch;
        }
        // Near edge a-b, a-c?
        for end in [S5, S7] {
            c.cycles += 30;
            let e = r32(c, end);
            seg_from_a(
                c,
                e,
                s.wrapping_add(0x30),
                s.wrapping_add(0x20),
                s.wrapping_add(0x10),
                &SEG_KID,
                false,
            );
            seg_tail(c, &SEG_KID);
            c.cycles += 2;
            let hit = c.g[V0][0] != 0;
            if end == S5 {
                c.g[V1][0] = 1;
            }
            if hit {
                if end == S7 {
                    c.cycles += 2;
                    c.g[V1][0] = 1;
                }
                break 'touch;
            }
        }
        // Near edge b-c?
        let r = &SEG_KID;
        c.cycles += 45;
        vsub_spill(
            c,
            r32(c, S6),
            r32(c, S5),
            s.wrapping_add(0x30),
            s.wrapping_add(0x20),
        );
        vdot(c, s.wrapping_add(0x20), s.wrapping_add(0x20), V1);
        c.mtc1(V1, 4);
        vsub_spill(
            c,
            r32(c, S7),
            r32(c, S5),
            s.wrapping_add(0x40),
            s.wrapping_add(0x30),
        );
        vlen(c, s.wrapping_add(0x30));
        c.mtc1(V1, 3);
        vdot(c, s.wrapping_add(0x20), s.wrapping_add(0x30), V0);
        c.mtc1(V0, 0);
        c.div_s(r.t, 0, r.len);
        c.c_lt_s(r.t, r.zero);
        let behind = c.fcc();
        c.g[V0][0] = 0;
        if !behind {
            c.cycles += 6;
            c.mul_s(0, 1, 1);
            c.sub_s(0, 4, 0);
            c.c_lt_s(2, 0);
            let off_line = c.fcc();
            c.g[V1][0] = c.g[T0][0];
            if off_line {
                c.cycles += 1;
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = 0;
                }
                break 'touch;
            }
            c.cycles += 8;
            c.sub_s(0, 2, 0);
            c.g[V0][0] = 1;
            c.sqrt_s(0, 0);
            c.sub_s(0, 1, 0);
            c.c_lt_s(3, 0);
            if !c.fcc() {
                break 'touch;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        // Missed: v1 = 1, cleared again since v0 is 0.
        c.cycles += 1;
        c.g[V1][0] = c.g[T0][0];
        c.cycles += 1;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0;
        }
    }
    c.cycles += 2;
    if c.g[V1][0] != 0 {
        c.cycles += 15;
        sphere_tri_hit(c, true);
    }
    Ok(())
}

/// `bool RiderSphereTree_CollideTriangle(SphereTree* tree, Vec4* a, Vec4* b,
/// Vec4* c, Vec4* n, Vec4* out_push, Vec4* out_point)` (0x32a1c0): the
/// rider's sphere tree against a triangle with normal n. The root sphere
/// must be within its radius of the triangle's plane and touch the
/// triangle (projected centre inside, near vertex a, or near an edge). With
/// no kids the root is the hit; otherwise every kid in the mask is tried
/// and the nearest touching one is the hit. On a hit the push-out
/// `n * (r - dist)` and the contact point are written.
pub fn rider_sphere_tree_collide_triangle(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x150);
        for (reg, off) in [
            (S2, 0x100),
            (S4, 0xe0),
            (S1, 0x110),
            (S5, 0xd0),
            (S7, 0xb0),
            (S0, 0x120),
            (S3, 0xf0),
            (S6, 0xc0),
            (FP, 0xa0),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x90);
        for (f, off) in [(23, 0x148), (22, 0x140), (21, 0x138), (20, 0x130)] {
            frame.save_fpr(c, f, off);
        }
        c.g[S2][0] = c.g[A1][0];
        c.g[S4][0] = c.g[A0][0];
        c.g[S1][0] = c.g[T0][0];
        c.g[S5][0] = c.g[A2][0];
        c.g[S7][0] = c.g[A3][0];
        let tree = r32(c, S4);
        let s = sp(c);
        c.g[V1][0] = sx(tree.wrapping_add(stree::CENTRE));
        // dist = n . (centre - a), against the root radius (f21).
        vsub_spill(
            c,
            tree.wrapping_add(stree::CENTRE),
            r32(c, S2),
            s.wrapping_add(0x10),
            s,
        );
        c.lwc1(21, tree.wrapping_add(stree::RADIUS));
        vdot(c, r32(c, S1), s, V0);
        c.mtc1(V0, 20);
        c.mov_s(1, 20);
        c.sw(s.wrapping_add(0x70), c.g[T1][0]);
        c.abs_s(0, 1);
        c.c_lt_s(21, 0);
        let off_plane = c.fcc();
        c.sw(s.wrapping_add(0x74), c.g[T2][0]);
        let mut touch = false;
        if !off_plane {
            // The centre projected onto the plane (at sp+0) inside the
            // triangle?
            c.cycles += 22;
            c.mfc1(V0, 1);
            c.lqc2(4, r32(c, S1));
            c.qmtc2(V0, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, s.wrapping_add(0x20));
            c.lwc1(12, r32(c, GP).wrapping_sub(0x2de0));
            c.lq(V0, s.wrapping_add(0x20));
            c.g[S0][0] = c.g[V1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S5][0];
            c.sq(V0, s.wrapping_add(0x10));
            c.g[A2][0] = c.g[S7][0];
            c.lqc2(3, tree.wrapping_add(stree::CENTRE));
            c.lqc2(4, s.wrapping_add(0x10));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s.wrapping_add(0x20));
            c.g[A3][0] = c.g[SP][0];
            c.lq(V0, s.wrapping_add(0x20));
            c.sq(V0, s);
            TRI_INTERSECT.call(c, 0x0032_a2c0)?;
            'test: {
                let inside = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if inside {
                    break 'test;
                }
                // Near vertex a?
                c.cycles += 20;
                c.lqc2(3, tree.wrapping_add(stree::CENTRE));
                c.lqc2(4, r32(c, S2));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s.wrapping_add(0x40));
                c.mul_s(3, 21, 21);
                c.lq(V0, s.wrapping_add(0x40));
                c.sq(V0, s.wrapping_add(0x30));
                vdot(c, s.wrapping_add(0x30), s.wrapping_add(0x30), V1);
                c.mtc1(V1, 5);
                c.c_lt_s(5, 3);
                let near = c.fcc();
                c.g[V0][0] = 1;
                if near {
                    break 'test;
                }
                // Near edge a-b, a-c?
                for end in [S5, S7] {
                    c.cycles += 31;
                    let e = r32(c, end);
                    seg_from_a(
                        c,
                        e,
                        s.wrapping_add(0x50),
                        s.wrapping_add(0x40),
                        s.wrapping_add(0x30),
                        &SEG_MAIN,
                        true,
                    );
                    seg_tail(c, &SEG_MAIN);
                    c.cycles += 2;
                    let hit = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if hit {
                        break 'test;
                    }
                }
                // Near edge b-c?
                c.cycles += 46;
                c.lqc2(3, r32(c, S0));
                c.lqc2(4, r32(c, S5));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s.wrapping_add(0x50));
                c.mtc1(0, 1);
                c.lq(V0, s.wrapping_add(0x50));
                c.sq(V0, s.wrapping_add(0x40));
                vdot(c, s.wrapping_add(0x40), s.wrapping_add(0x40), V1);
                c.mtc1(V1, 5);
                vsub_spill(
                    c,
                    r32(c, S7),
                    r32(c, S5),
                    s.wrapping_add(0x60),
                    s.wrapping_add(0x50),
                );
                vlen(c, s.wrapping_add(0x50));
                c.mtc1(V1, 4);
                vdot(c, s.wrapping_add(0x40), s.wrapping_add(0x50), V0);
                c.mtc1(V0, 0);
                seg_tail(c, &SEG_MAIN);
            }
            c.cycles += 2;
            touch = c.g[V0][0] != 0;
        }
        if !touch {
            c.cycles += 2;
            c.g[V0][0] = 0;
            return sphere_tri_epilogue(c, frame);
        }

        c.g[V0][0] = c.lw(tree.wrapping_add(stree::KID_MASK));
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            // No kids: the root is the hit.
            c.cycles += 15;
            sphere_tri_hit(c, false);
            return sphere_tri_epilogue(c, frame);
        }
        c.g[V0][0] = c.lw(tree.wrapping_add(stree::KID_COUNT));

        // The kids: f23 = the best plane distance so far.
        c.cycles += 3;
        c.lwc1(23, r32(c, GP).wrapping_sub(0x2ddc));
        c.g[S3][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 5;
            c.mtc1(0, 22);
            c.g[T0][0] = 1;
            c.g[FP][0] = c.g[S4][0];
            c.g[S0][0] = sx(tree.wrapping_add(stree::KIDS));
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lw(tree.wrapping_add(stree::KID_MASK));
                c.g[V1][0] = sx((c.g[T0][0] as u32) << ((c.g[S3][0] as u32) & 31));
                c.g[V0][0] &= c.g[V1][0];
                if c.g[V0][0] != 0 {
                    sphere_tree_kid(c)?;
                }
                c.g[V0][0] = c.lw(tree.wrapping_add(stree::KID_COUNT));
                c.cycles += 5;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_a5e8);
                }
            }
        }
        // Hit when the best distance moved off its start value.
        c.cycles += 5;
        c.lwc1(0, r32(c, GP).wrapping_sub(0x2dd4));
        c.c_eq_s(23, 0);
        let none = c.fcc();
        c.g[V0][0] = 1;
        if none {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        sphere_tri_epilogue(c, frame)
    })
}

pub(crate) fn sphere_tri_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 16;
    for (reg, off) in [
        (S0, 0x120),
        (S1, 0x110),
        (S2, 0x100),
        (S3, 0xf0),
        (S4, 0xe0),
        (S5, 0xd0),
        (S6, 0xc0),
        (S7, 0xb0),
        (FP, 0xa0),
    ] {
        frame.restore(c, reg, off);
    }
    frame.restore_ra(c, 0x90);
    for (f, off) in [(23, 0x148), (22, 0x140), (21, 0x138), (20, 0x130)] {
        frame.restore_fpr(c, f, off);
    }
    frame.close(c);
    Ok(())
}

pub(crate) mod oc230_tree {

    /// Root sphere centre (Vec4) and radius.
    pub const CENTER: u32 = 0x10;

    pub const RADIUS: u32 = 0x20;

    /// Bit i: kid i takes part.
    pub const MASK: u32 = 0x28;

    pub const COUNT: u32 = 0x2c;

    /// Kids: centre (Vec4) and radius at +0x10, 0x20 apart.
    pub const KIDS: u32 = 0x30;
}

/// The tail of a sphere-vs-segment test (f0 = dot(centre - start, edge)) in
/// the FPU registers given: `t = f0 / len`; v0 = 1 when `t >= 0`, the centre
/// is within the radius of the line (`d2 - t^2 <= r2`) and the entry point
/// `t - sqrt(r2 - (d2 - t^2))` is not past the end.
pub(crate) fn oc230_seg_tail(c: &mut Cpu, t: usize, len: usize, d2: usize, r2: usize, zero: usize) {
    c.div_s(t, 0, len);
    c.c_lt_s(t, zero);
    let behind = c.fcc();
    c.g[V0][0] = 0;
    if behind {
        return;
    }
    c.cycles += 5;
    c.mul_s(0, t, t);
    c.sub_s(0, d2, 0);
    c.c_lt_s(r2, 0);
    if c.fcc() {
        return;
    }
    c.cycles += 8;
    c.sub_s(0, r2, 0);
    c.g[V0][0] = 1;
    c.sqrt_s(0, 0);
    c.sub_s(0, t, 0);
    c.c_lt_s(len, 0);
    if c.fcc() {
        c.g[V0][0] = 0;
    }
}

/// The hit of `RiderSphereTree_CollideTriangle2`: push = n * (r - dist)
/// to `*out_push` (sp+0x80), the contact point (sp+0x10) to `*out_point`
/// (sp+0x84), push kept at sp+0x20. `kid` selects the kid loop's
/// registers (and its `best = dist`).
pub(crate) fn oc230_ct2_hit(c: &mut Cpu, kid: bool) {
    use ssx3_ee_rt::vu0::{Src, X};
    let s = sp(c);
    let (scale, push, point, ptr0) = if kid {
        (V0, V1, V0, A0)
    } else {
        (V1, A0, V1, A1)
    };
    c.sub_s(0, 21, 20);
    if kid {
        c.mov_s(23, 20);
    } else {
        c.g[V0][0] = 1;
    }
    c.mfc1(scale, 0);
    c.lqc2(4, s);
    c.qmtc2(scale, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x30));
    c.lq(push, s.wrapping_add(0x30));
    c.g[ptr0][0] = c.lw(s.wrapping_add(0x80));
    c.lq(point, s.wrapping_add(0x10));
    c.sq(push, r32(c, ptr0));
    c.g[A1][0] = c.lw(s.wrapping_add(0x84));
    c.sq(point, r32(c, A1));
    c.sq(push, s.wrapping_add(0x20));
}

/// Does the root sphere touch the triangle (p, a, b)? v0 = 1/0. Its plane
/// distance is in f20, radius in f21, n at sp+0. Leaves s0 = the centre.
pub(crate) fn oc230_ct2_root_touch(c: &mut Cpu) -> crate::call::Flow {
    use crate::game::intersect::riderspheretree as rst;
    use ssx3_ee_rt::vu0::{Src, X};
    let s = sp(c);
    let center = r32(c, S3).wrapping_add(oc230_tree::CENTER);
    // Projected centre d = centre - n * dist (sp+0x10) inside?
    c.cycles += 22;
    c.mfc1(V0, 20);
    c.lqc2(4, s);
    c.qmtc2(V0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(0x30));
    c.g[S0][0] = c.g[A0][0];
    c.lq(V0, s.wrapping_add(0x30));
    c.g[A0][0] = c.g[S1][0];
    c.lwc1(12, oc230_gp(c, 0u32.wrapping_sub(0x2dd0)));
    c.g[A1][0] = c.g[S4][0];
    c.sq(V0, s.wrapping_add(0x20));
    c.g[A2][0] = c.g[S6][0];
    rst::vsub_spill(
        c,
        center,
        s.wrapping_add(0x20),
        s.wrapping_add(0x30),
        s.wrapping_add(0x10),
    );
    c.g[A3][0] = sx(s.wrapping_add(0x10));
    rst::TRI_INTERSECT.call(c, 0x0032_ab68)?;
    let inside = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if inside {
        return Ok(());
    }
    // Near p? (v = centre - p at sp+0x40, d2 = |v|^2 in f5, r^2 in f3)
    c.cycles += 20;
    let p = r32(c, S1);
    rst::vsub_spill(c, center, p, s.wrapping_add(0x50), s.wrapping_add(0x40));
    c.mul_s(3, 21, 21);
    rst::vdot(c, s.wrapping_add(0x40), s.wrapping_add(0x40), V1);
    c.mtc1(V1, 5);
    c.c_lt_s(5, 3);
    c.g[V0][0] = 1;
    if c.fcc() {
        return Ok(());
    }
    // Near edge p-a?
    c.cycles += 30;
    let a = r32(c, S4);
    rst::vsub_spill(c, a, p, s.wrapping_add(0x60), s.wrapping_add(0x50));
    rst::vlen(c, s.wrapping_add(0x50));
    c.mtc1(V1, 2);
    rst::vdot(c, s.wrapping_add(0x40), s.wrapping_add(0x50), V0);
    c.mtc1(V0, 0);
    oc230_seg_tail(c, 1, 2, 5, 3, 22);
    c.cycles += 2;
    let hit = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if hit {
        return Ok(());
    }
    // Near edge p-b?
    c.cycles += 31;
    let b = r32(c, S6);
    rst::vsub_spill(c, b, p, s.wrapping_add(0x60), s.wrapping_add(0x50));
    c.mtc1(0, 1);
    rst::vlen(c, s.wrapping_add(0x50));
    c.mtc1(V1, 4);
    rst::vdot(c, s.wrapping_add(0x40), s.wrapping_add(0x50), V0);
    c.mtc1(V0, 0);
    oc230_seg_tail(c, 2, 4, 5, 3, 1);
    c.cycles += 2;
    let hit = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if hit {
        return Ok(());
    }
    // Near edge a-b? (v2 = centre - a, d2 = |v2|^2)
    c.cycles += 46;
    let center = r32(c, S0);
    rst::vsub_spill(c, center, a, s.wrapping_add(0x60), s.wrapping_add(0x50));
    c.mtc1(0, 1);
    rst::vdot(c, s.wrapping_add(0x50), s.wrapping_add(0x50), V1);
    c.mtc1(V1, 5);
    rst::vsub_spill(c, b, a, s.wrapping_add(0x70), s.wrapping_add(0x60));
    rst::vlen(c, s.wrapping_add(0x60));
    c.mtc1(V1, 4);
    rst::vdot(c, s.wrapping_add(0x50), s.wrapping_add(0x60), V0);
    c.mtc1(V0, 0);
    oc230_seg_tail(c, 2, 4, 5, 3, 1);
    Ok(())
}

/// One kid of the tree (s0 = its centre, s7 = tree + 0x20 * index, s2 =
/// index, fp = 1): when it is in the mask, its plane distance is below its
/// radius and below the best so far (f23), and it touches the triangle, it
/// becomes the hit. Ends with v0 = the kid count.
pub(crate) fn oc230_ct2_kid(c: &mut Cpu) -> crate::call::Flow {
    use crate::game::intersect::riderspheretree as rst;
    use oc230_tree as tr;
    use ssx3_ee_rt::vu0::{Src, X};
    let s = sp(c);
    let tree = r32(c, S3);
    c.cycles += 5;
    c.g[V0][0] = c.lw(tree.wrapping_add(tr::MASK));
    c.g[V1][0] = sx(r32(c, FP) << (r32(c, S2) & 31));
    c.g[V0][0] &= c.g[V1][0];
    if c.g[V0][0] == 0 {
        c.g[V0][0] = c.lw(tree.wrapping_add(tr::COUNT));
        return Ok(());
    }
    c.cycles += 20;
    let kid = r32(c, S0);
    let p = r32(c, S1);
    rst::vsub_spill(c, kid, p, s.wrapping_add(0x30), s.wrapping_add(0x20));
    c.lwc1(21, kid.wrapping_add(0x10));
    rst::vdot(c, s, s.wrapping_add(0x20), V1);
    c.mtc1(V1, 20);
    c.c_lt_s(20, 21);
    if !c.fcc() {
        c.g[V0][0] = c.lw(tree.wrapping_add(tr::COUNT));
        return Ok(());
    }
    c.cycles += 4;
    c.c_lt_s(20, 23);
    if !c.fcc() {
        c.g[V0][0] = c.lw(tree.wrapping_add(tr::COUNT));
        return Ok(());
    }
    'touch: {
        // Projected centre inside?
        c.cycles += 23;
        c.mfc1(V0, 20);
        c.lqc2(4, s);
        c.qmtc2(V0, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s.wrapping_add(0x40));
        c.lwc1(12, oc230_gp(c, 0u32.wrapping_sub(0x2dc8)));
        c.lq(V0, s.wrapping_add(0x40));
        c.g[S5][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S4][0];
        c.sq(V0, s.wrapping_add(0x30));
        c.g[A2][0] = c.g[S6][0];
        rst::vsub_spill(
            c,
            kid,
            s.wrapping_add(0x30),
            s.wrapping_add(0x40),
            s.wrapping_add(0x10),
        );
        c.g[A3][0] = sx(s.wrapping_add(0x10));
        c.sq(V0, s.wrapping_add(0x20));
        rst::TRI_INTERSECT.call(c, 0x0032_af50)?;
        let inside = c.g[V0][0] != 0;
        c.g[V1][0] = 1;
        if inside {
            break 'touch;
        }
        // Near p?
        c.cycles += 20;
        let kid_at = r32(c, S7).wrapping_add(tr::KIDS);
        rst::vsub_spill(c, kid_at, p, s.wrapping_add(0x30), s.wrapping_add(0x20));
        c.mul_s(2, 21, 21);
        rst::vdot(c, s.wrapping_add(0x20), s.wrapping_add(0x20), V1);
        c.mtc1(V1, 4);
        c.c_lt_s(4, 2);
        c.g[V1][0] = 1;
        if c.fcc() {
            break 'touch;
        }
        // Near edge p-a, p-b?
        for end in [S4, S6] {
            c.cycles += 30;
            let e = r32(c, end);
            rst::vsub_spill(c, e, p, s.wrapping_add(0x40), s.wrapping_add(0x30));
            rst::vlen(c, s.wrapping_add(0x30));
            c.mtc1(V1, 3);
            rst::vdot(c, s.wrapping_add(0x20), s.wrapping_add(0x30), V0);
            c.mtc1(V0, 0);
            oc230_seg_tail(c, 1, 3, 4, 2, 22);
            c.cycles += 2;
            let hit = c.g[V0][0] != 0;
            c.g[V1][0] = 1;
            if hit {
                break 'touch;
            }
        }
        // Near edge a-b?
        c.cycles += 45;
        let (a, b) = (r32(c, S4), r32(c, S6));
        rst::vsub_spill(c, r32(c, S5), a, s.wrapping_add(0x40), s.wrapping_add(0x30));
        rst::vdot(c, s.wrapping_add(0x30), s.wrapping_add(0x30), V1);
        c.mtc1(V1, 4);
        rst::vsub_spill(c, b, a, s.wrapping_add(0x50), s.wrapping_add(0x40));
        rst::vlen(c, s.wrapping_add(0x40));
        c.mtc1(V1, 3);
        rst::vdot(c, s.wrapping_add(0x30), s.wrapping_add(0x40), V0);
        c.mtc1(V0, 0);
        c.div_s(1, 0, 3);
        c.c_lt_s(1, 22);
        let behind = c.fcc();
        c.g[V0][0] = 0;
        if !behind {
            c.cycles += 6;
            c.mul_s(0, 1, 1);
            c.sub_s(0, 4, 0);
            c.c_lt_s(2, 0);
            let off_line = c.fcc();
            c.g[V1][0] = c.g[FP][0];
            if off_line {
                c.cycles += 1;
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = 0;
                }
                break 'touch;
            }
            c.cycles += 8;
            c.sub_s(0, 2, 0);
            c.g[V0][0] = 1;
            c.sqrt_s(0, 0);
            c.sub_s(0, 1, 0);
            c.c_lt_s(3, 0);
            if !c.fcc() {
                break 'touch;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 1;
        c.g[V1][0] = c.g[FP][0];
        c.cycles += 1;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0;
        }
    }
    c.cycles += 2;
    if c.g[V1][0] != 0 {
        c.cycles += 15;
        oc230_ct2_hit(c, true);
    }
    c.g[V0][0] = c.lw(tree.wrapping_add(tr::COUNT));
    Ok(())
}

/// `bool RiderSphereTree_CollideTriangle2(SphereTree* tree, Vec4* p, Vec4*
/// a, Vec4* b, Vec4* n, Vec4* out_push, Vec4* out_point)` (0x32aa28): the
/// rider's sphere tree against the triangle (p, a, b) with normal n (the
/// second triangle test, n flipped toward the sphere). The root sphere must
/// be within its radius of the plane and touch the triangle (projected
/// centre inside, near p, or near an edge). With no kids in the mask the
/// root is the hit; otherwise every kid in the mask is tried and the
/// nearest touching one is the hit. On a hit the push-out `n * (r - dist)`
/// and the contact point are written.
pub fn rider_sphere_tree_collide_triangle2(c: &mut Cpu) {
    use crate::game::intersect::riderspheretree as rst;
    use oc230_tree as tr;
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x150);
        let s = sp(c);
        c.lq(V0, r32(c, T0));
        frame.save(c, S1, 0x110);
        frame.save(c, S3, 0xf0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S4, 0xe0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S6, 0xc0);
        c.g[A0][0] = sx(r32(c, S3).wrapping_add(tr::CENTER));
        c.sq(V0, s); // n
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S0, 0x120);
        c.g[S6][0] = c.g[A3][0];
        frame.save(c, S2, 0x100);
        frame.save(c, S5, 0xd0);
        frame.save(c, S7, 0xb0);
        frame.save(c, FP, 0xa0);
        frame.save_ra(c, 0x90);
        frame.save_fpr(c, 23, 0x148);
        frame.save_fpr(c, 22, 0x140);
        frame.save_fpr(c, 21, 0x138);
        frame.save_fpr(c, 20, 0x130);
        // dist = n . (centre - p) in f20, radius in f21.
        let tree = r32(c, S3);
        rst::vsub_spill(
            c,
            tree.wrapping_add(tr::CENTER),
            r32(c, S1),
            s.wrapping_add(0x20),
            s.wrapping_add(0x10),
        );
        let (push, point) = (c.g[T1][0], c.g[T2][0]);
        c.sw(s.wrapping_add(0x80), push);
        c.lwc1(21, tree.wrapping_add(tr::RADIUS));
        c.sw(s.wrapping_add(0x84), point);
        rst::vdot(c, s, s.wrapping_add(0x10), V1);
        c.mtc1(V1, 20);
        c.abs_s(0, 20);
        c.c_lt_s(21, 0);
        let far = c.fcc();
        c.g[V0][0] = 0;
        'done: {
            if far {
                break 'done;
            }
            // Below the plane: dist = -dist, n = -n.
            c.cycles += 4;
            c.mtc1(0, 22);
            c.c_lt_s(20, 22);
            if c.fcc() {
                c.cycles += 11;
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.mtc1(AT, 0);
                c.neg_s(20, 20);
                c.mfc1(V1, 0);
                c.lqc2(4, s);
                c.qmtc2(V1, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, s.wrapping_add(0x20));
                c.lq(V0, s.wrapping_add(0x20));
                c.sq(V0, s);
                c.sq(V0, s.wrapping_add(0x10));
            }
            oc230_ct2_root_touch(c)?;
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'done;
            }
            c.g[V0][0] = c.lw(tree.wrapping_add(tr::MASK));
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                // No kids: the root is the hit.
                c.cycles += 15;
                oc230_ct2_hit(c, false);
                break 'done;
            }
            c.g[V0][0] = c.lw(tree.wrapping_add(tr::COUNT));
            // The nearest touching kid (best in f23).
            c.cycles += 3;
            c.lwc1(23, oc230_gp(c, 0u32.wrapping_sub(0x2dcc)));
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = 0;
            if !none {
                c.cycles += 4;
                c.mtc1(0, 22);
                c.g[FP][0] = 1;
                c.g[S7][0] = c.g[S3][0];
                c.g[S0][0] = sx(tree.wrapping_add(tr::KIDS));
                loop {
                    oc230_ct2_kid(c)?;
                    c.cycles += 5;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                    c.g[S7][0] = sx(r32(c, S7).wrapping_add(0x20));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x20));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_ae88);
                    }
                }
            }
            // Hit when best moved off its start value.
            c.cycles += 5;
            c.lwc1(0, oc230_gp(c, 0u32.wrapping_sub(0x2dc4)));
            c.c_eq_s(23, 0);
            let untouched = c.fcc();
            c.g[V0][0] = 1;
            if untouched {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 16;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore(c, S2, 0x100);
        frame.restore(c, S3, 0xf0);
        frame.restore(c, S4, 0xe0);
        frame.restore(c, S5, 0xd0);
        frame.restore(c, S6, 0xc0);
        frame.restore(c, S7, 0xb0);
        frame.restore(c, FP, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 23, 0x148);
        frame.restore_fpr(c, 22, 0x140);
        frame.restore_fpr(c, 21, 0x138);
        frame.restore_fpr(c, 20, 0x130);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod vu {
    use ssx3_ee_rt::vu0::Src;
    use ssx3_ee_rt::vu0::W;
    use ssx3_ee_rt::vu0::X;
    use ssx3_ee_rt::vu0::Y;
    use ssx3_ee_rt::vu0::Z;
    use ssx3_ee_rt::Cpu;

    /// `a · b` (xyz) into GPR `g` and FPR `f`: vf3 = a, vf5 = b, w lane of vf4.
    pub fn dot(c: &mut Cpu, a: u32, b: u32, g: usize, f: usize) {
        c.lqc2(3, a);
        c.lqc2(5, b);
        dot_vf3_vf5(c, g, f);
    }

    /// The dot product of vf3 and vf5 (already loaded).
    pub fn dot_vf3_vf5(c: &mut Cpu, g: usize, f: usize) {
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(g, 4);
        c.mtc1(g, f);
    }

    /// `dst = src / |src|` (vf3 = src, Q = rsqrt).
    pub fn normalize(c: &mut Cpu, src: u32, dst: u32) {
        c.lqc2(3, src);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, dst);
    }

    /// `dst = src * f` (through GPR `g`: mfc1, qmtc2 into vf3).
    pub fn scale(c: &mut Cpu, f: usize, src: u32, dst: u32, g: usize) {
        c.mfc1(g, f);
        c.lqc2(4, src);
        c.qmtc2(g, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, dst);
    }

    /// `dst = a - b` (vf3 = a, vf4 = b).
    pub fn sub(c: &mut Cpu, a: u32, b: u32, dst: u32) {
        c.lqc2(3, a);
        c.lqc2(4, b);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, dst);
    }

    /// `dst = a + b` (vf3 = a, vf4 = b).
    pub fn add(c: &mut Cpu, a: u32, b: u32, dst: u32) {
        c.lqc2(3, a);
        c.lqc2(4, b);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, dst);
    }

    /// A 128-bit copy through GPR `g` (`lq`, `sq`).
    pub fn copy(c: &mut Cpu, src: u32, dst: u32, g: usize) {
        c.lq(g, src);
        c.sq(g, dst);
    }
}

pub(crate) mod leaf {

    /// Collider (`self`, s2): skip back-facing triangles, the sweep direction,
    /// the vtable.
    pub const USE_NORMAL: u32 = 0x8;

    pub const DIR: u32 = 0x10;

    pub const VTABLE: u32 = 0x50;

    /// Virtual `bool TestTri(p0, p1, p2, nrm, &pos, &hitNrm, &t)` (+0x20/+0x24),
    /// its swept form (+0x28/+0x2c), `bool TestBox(lo, hi)` (+0x48/+0x4c).
    pub const VT_TRI: u32 = 0x20;

    pub const VT_TRI_SWEPT: u32 = 0x28;

    pub const VT_BOX: u32 = 0x48;

    /// Leaf mesh (s3): triangle count (u16), indices (3 bytes per triangle),
    /// boxes (0x18 each: lo +0, hi +0xc, ten triangles per box), vertices,
    /// face normals (16 bytes each).
    pub const NUM_TRIS: u32 = 0x0;

    pub const INDICES: u32 = 0x4;

    pub const BOXES: u32 = 0x8;

    pub const VERTS: u32 = 0xc;

    pub const NORMALS: u32 = 0x10;

    pub const BOX_SIZE: u64 = 0x18;

    pub const TRIS_PER_BOX: u64 = 10;

    /// Hit records (0x80 bytes each).
    pub const HIT_SIZE: u32 = 0x80;

    /// The zero vector (0x4ff120).
    pub const ZERO_VEC: u32 = 0x0050_0000 - 0xee0;

    /// The back-face limits (gp-0x2d88: small meshes, gp-0x2d84: boxed).
    pub const GP_FACING_SMALL: u32 = 0x2d88;

    pub const GP_FACING_BOXED: u32 = 0x2d84;

    /// Stack: the swept flag, the output, the hit count, the box count; the
    /// test's position (sp), normal (sp+0x10) and fraction (sp+0x20).
    pub const SP_SWEPT: u32 = 0x24;

    pub const SP_OUT: u32 = 0x28;

    pub const SP_HITS: u32 = 0x2c;

    pub const SP_BOXES: u32 = 0x30;
}

// ------------------------------------------------- RiderSphereTree_CollideBox

/// Collision sphere: centre (+0x10), radius (+0x20), sub-sphere mask
/// (+0x28) and count (+0x2c).
pub(crate) const RST_CENTRE: u32 = 0x10;

pub(crate) const RST_RADIUS: u32 = 0x20;

pub(crate) const RST_MASK: u32 = 0x28;

pub(crate) const RST_COUNT: u32 = 0x2c;

/// `Collide_SphereVsBoxEdges` (0x329590).
pub(crate) const RST_SPHERE_VS_BOX: Callee = Callee {
    addr: 0x0032_9590,
    f: ee::s10::f_00329590,
};

/// `-1.0f`, the "no hit yet" distance.
pub(crate) const RST_NO_HIT: u64 = 0xffff_ffff_bf80_0000;

/// `bool RiderSphereTree_CollideBox(Sphere* self, Vec4* boxMin,
/// Vec4* boxMax, Vec4* outPos, Vec4* outNormal)` (0x32b2b8): sweep the
/// sphere against the box (`Collide_SphereVsBoxEdges(min, max, &centre,
/// &normal, &dir, &t, radius)`, results on the stack: normal at sp+0,
/// dir at sp+0x10, t at sp+0x40). Without a sub-sphere mask the first hit
/// is the answer (`outPos = dir * t`, `outNormal = normal`). Otherwise the
/// sweep is repeated once per set mask bit below the count and the hit with
/// the largest t is kept (f20 = best, from -1); true if any. The scaling is
/// a VU0 `vmul.xyzw` by the broadcast t. Frame 0x100 (s0-s7, fp, f20).
pub fn rider_sphere_tree_collide_box(c: &mut Cpu) {
    /// `outPos = dir * t; outNormal = normal` (also dir * t at sp+0x20 when
    /// `copy`), through `t_reg` (mfc1) and the given GPR pair.
    fn store_hit(c: &mut Cpu, t_reg: usize, pos_reg: usize, nrm_reg: usize, copy: bool) {
        let s = sp(c);
        c.mfc1(t_reg, 0);
        c.lqc2(4, s.wrapping_add(0x10));
        c.qmtc2(t_reg, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s.wrapping_add(0x30));
        c.lq(pos_reg, s.wrapping_add(0x30));
        c.lq(nrm_reg, s);
        let out_pos = r32(c, S4);
        let out_nrm = r32(c, S5);
        c.sq(pos_reg, out_pos);
        c.sq(nrm_reg, out_nrm);
        if copy {
            c.sq(pos_reg, s.wrapping_add(0x20));
        }
    }

    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x100);
        let s = sp(c);
        frame.save(c, S1, 0xd0);
        c.g[T1][0] = sx(s.wrapping_add(0x40));
        frame.save(c, S2, 0xc0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0xb0);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0xa0);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S5, 0x90);
        c.g[S4][0] = c.g[A3][0];
        frame.save(c, S7, 0x70);
        c.g[S5][0] = c.g[T0][0];
        frame.save(c, FP, 0x60);
        c.g[T0][0] = sx(s.wrapping_add(0x10));
        frame.save(c, S0, 0xe0);
        c.g[FP][0] = c.g[T0][0];
        frame.save(c, S6, 0x80);
        c.g[S7][0] = c.g[T1][0];
        frame.save_ra(c, 0x50);
        c.g[A0][0] = c.g[S3][0];
        frame.save_fpr(c, 20, 0xf0);
        c.g[A1][0] = c.g[S2][0];
        let this = r32(c, S1);
        c.g[A2][0] = sx(this.wrapping_add(RST_CENTRE));
        c.g[A3][0] = sx(s);
        c.lwc1(12, this.wrapping_add(RST_RADIUS));
        RST_SPHERE_VS_BOX.call(c, 0x0032_b324)?;

        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            let this = r32(c, S1);
            let mask = Addr(this.wrapping_add(RST_MASK)).lw(c);
            c.g[V0][0] = mask;
            c.cycles += 2;
            if mask == 0 {
                // The single sphere: its hit.
                c.cycles += 13;
                c.lwc1(0, sp(c).wrapping_add(0x40));
                c.g[V0][0] = 1;
                store_hit(c, V1, A0, V1, true);
            } else {
                let count = Addr(this.wrapping_add(RST_COUNT)).lw(c);
                c.g[V0][0] = count;
                c.cycles += 4;
                c.g[AT][0] = RST_NO_HIT;
                c.mtc1(AT, 20);
                c.g[S0][0] = 0;
                if (count as i64) > 0 {
                    c.cycles += 2;
                    c.g[S6][0] = 1;
                    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(RST_MASK)).lw(c);
                    loop {
                        c.cycles += 4;
                        let bit = sx(r32(c, S6) << (r32(c, S0) & 31));
                        c.g[V1][0] = bit;
                        c.g[V0][0] &= bit;
                        c.g[A0][0] = c.g[S3][0];
                        // Whether the iteration ends through the 1-cycle
                        // `v0 = count` block.
                        let mut tail = true;
                        if c.g[V0][0] != 0 {
                            c.cycles += 9;
                            let this = r32(c, S1);
                            c.lwc1(12, this.wrapping_add(RST_RADIUS));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = sx(this.wrapping_add(RST_CENTRE));
                            c.g[A3][0] = c.g[SP][0];
                            c.g[T0][0] = c.g[FP][0];
                            c.g[T1][0] = c.g[S7][0];
                            RST_SPHERE_VS_BOX.call(c, 0x0032_b3b4)?;
                            let hit = c.g[V0][0] != 0;
                            c.lwc1(0, sp(c).wrapping_add(0x40));
                            if hit {
                                c.cycles += 4;
                                c.c_lt_s(20, 0);
                                if c.fcc() {
                                    // Farther than the best so far.
                                    c.cycles += 11;
                                    store_hit(c, V0, V1, V0, true);
                                    c.lwc1(20, sp(c).wrapping_add(0x40));
                                } else {
                                    tail = false;
                                }
                            }
                        }
                        if tail {
                            c.cycles += 1;
                        }
                        let count = Addr(r32(c, S1).wrapping_add(RST_COUNT)).lw(c);
                        c.cycles += 4;
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        let more = (c.g[S0][0] as i64) < (count as i64);
                        c.g[V0][0] = more as u64;
                        if !more {
                            break;
                        }
                        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(RST_MASK)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0032_b388);
                        }
                    }
                }
                // Any hit: best != -1.
                c.cycles += 6;
                c.g[AT][0] = RST_NO_HIT;
                c.mtc1(AT, 0);
                c.c_eq_s(20, 0);
                c.g[V0][0] = 1;
                if c.fcc() {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
            }
        }

        c.cycles += 13;
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderSphereTree_Scale (0x329b90)

/// A rider sphere tree: the root sphere's centre (+0x10) and radius
/// (+0x20), the child count (+0x2c), then the children (0x20 bytes each from
/// +0x30: centre at +0, radius at +0x10).
pub(crate) const TREE_ROOT_CENTER: u32 = 0x10;

pub(crate) const TREE_ROOT_RADIUS: u32 = 0x20;

pub(crate) const TREE_CHILD_COUNT: u32 = 0x2c;

pub(crate) const TREE_CHILDREN: u32 = 0x30;

pub(crate) const TREE_CHILD_RADIUS: u32 = 0x10;

pub(crate) const TREE_CHILD_SIZE: u32 = 0x20;

/// `vf12 = m * vf8` (VU0 `mula`/`madda`/`madd` by the four columns of the
/// matrix at `m`, loaded into vf4..vf7), with vf8 loaded from `center`.
pub(crate) fn sphere_center_times_matrix(c: &mut Cpu, center: u32, m: u32) {
    c.lqc2(8, center);
    c.lqc2(4, m);
    c.lqc2(5, m.wrapping_add(0x10));
    c.lqc2(6, m.wrapping_add(0x20));
    c.lqc2(7, m.wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
}

/// `void RiderSphereTree_Scale(SphereTree* tree /*a0*/, Matrix* m /*a1*/,
/// float scale /*f12*/)` (0x329b90): move every sphere of the tree (root,
/// then each child) through the matrix and scale it: the centre becomes
/// `(m * c) * (scale, scale, scale, 1)`, the radius `radius * scale`. Leaf,
/// frame 0x40 (the scale vector at 0, the scaled centre at 0x10, the
/// transformed one at 0x20 and 0x30).
pub fn rider_sphere_tree_scale(c: &mut Cpu) {
    c.cycles += 43;
    let tree = r32(c, A0);
    let m = r32(c, A1);
    c.g[AT][0] = 0x3f80_0000; // 1.0
    c.mtc1(1, 0);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
    let frame = sp(c);
    // The scale vector (scale, scale, scale, 1).
    c.swc1(12, frame);
    c.g[A2][0] = 0;
    c.swc1(0, frame.wrapping_add(0xc));
    c.swc1(12, frame.wrapping_add(0x4));
    c.swc1(12, frame.wrapping_add(0x8));
    // The root.
    sphere_center_times_matrix(c, tree.wrapping_add(TREE_ROOT_CENTER), m);
    c.sqc2(12, frame.wrapping_add(0x30));
    c.lq(2, frame.wrapping_add(0x30));
    c.lwc1(0, frame);
    c.sq(2, frame.wrapping_add(0x20));
    c.lwc1(1, frame.wrapping_add(0x4));
    c.lwc1(2, frame.wrapping_add(0x20));
    c.lwc1(3, frame.wrapping_add(0x24));
    c.mul_s(2, 2, 0);
    c.lwc1(4, frame.wrapping_add(0x28));
    c.lwc1(6, frame.wrapping_add(0x8));
    c.mul_s(3, 3, 1);
    c.lwc1(0, frame.wrapping_add(0x2c));
    c.lwc1(5, frame.wrapping_add(0xc));
    c.mul_s(4, 4, 6);
    c.lwc1(1, tree.wrapping_add(TREE_ROOT_RADIUS));
    c.mul_s(0, 0, 5);
    c.swc1(2, frame.wrapping_add(0x10));
    c.swc1(3, frame.wrapping_add(0x14));
    c.mul_s(1, 1, 12);
    c.swc1(4, frame.wrapping_add(0x18));
    c.swc1(0, frame.wrapping_add(0x1c));
    c.lq(2, frame.wrapping_add(0x10));
    c.g[V1][0] = c.lw(tree.wrapping_add(TREE_CHILD_COUNT));
    c.swc1(1, tree.wrapping_add(TREE_ROOT_RADIUS));
    c.sq(2, tree.wrapping_add(TREE_ROOT_CENTER));
    if (c.g[V1][0] as i64) > 0 {
        // The children (v1: the child minus 0x30, a2: its index).
        c.cycles += 1;
        c.g[V1][0] = c.g[A0][0];
        loop {
            c.cycles += 38;
            let child = r32(c, V1).wrapping_add(TREE_CHILDREN);
            sphere_center_times_matrix(c, child, m);
            c.sqc2(12, frame.wrapping_add(0x30));
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
            c.lq(2, frame.wrapping_add(0x30));
            c.lwc1(2, frame.wrapping_add(0x4));
            c.sq(2, frame.wrapping_add(0x20));
            c.lwc1(0, frame);
            c.lwc1(4, frame.wrapping_add(0x24));
            c.lwc1(3, frame.wrapping_add(0x20));
            c.mul_s(4, 4, 2);
            c.lwc1(5, frame.wrapping_add(0x28));
            c.lwc1(6, frame.wrapping_add(0x8));
            c.mul_s(3, 3, 0);
            c.lwc1(1, frame.wrapping_add(0x2c));
            c.lwc1(2, frame.wrapping_add(0xc));
            c.mul_s(5, 5, 6);
            c.lwc1(0, child.wrapping_add(TREE_CHILD_RADIUS));
            c.mul_s(1, 1, 2);
            c.swc1(3, frame.wrapping_add(0x10));
            c.mul_s(0, 0, 12);
            c.swc1(4, frame.wrapping_add(0x14));
            c.swc1(5, frame.wrapping_add(0x18));
            c.swc1(1, frame.wrapping_add(0x1c));
            c.swc1(0, child.wrapping_add(TREE_CHILD_RADIUS));
            c.lq(2, frame.wrapping_add(0x10));
            c.sq(2, child);
            c.g[V0][0] = c.lw(tree.wrapping_add(TREE_CHILD_COUNT));
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(TREE_CHILD_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_9c40);
            }
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
    ret(c);
}

/// `sw reg, addr`.
#[inline]
pub(crate) fn sw_reg(c: &mut Cpu, addr: u32, reg: usize) {
    let v = c.g[reg][0];
    Addr(addr).sw(c, v);
}

/// `c.lt.s fs, ft` and the condition bit.
#[inline]
pub(crate) fn lt_s(c: &mut Cpu, fs: usize, ft: usize) -> bool {
    c.c_lt_s(fs, ft);
    c.fcc()
}

/// The back-face test of triangle s1 (`useNormal && nrm · dir > limit`):
/// true when it is skipped. Leaves t0 = the normal's address.
pub(crate) fn leaf_faces_away(c: &mut Cpu, limit: u32) -> bool {
    use leaf::*;
    c.cycles += 5;
    let mesh = r32(c, S3);
    let this = r32(c, S2);
    c.g[V0][0] = Addr(mesh + NORMALS).lw(c);
    c.g[V1][0] = sx(r32(c, S1) << 4);
    c.g[A0][0] = Addr(this + USE_NORMAL).lw(c);
    c.g[T0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    if c.g[A0][0] == 0 {
        return false;
    }
    c.cycles += 14;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(limit));
    let normal = r32(c, T0);
    vu::dot(c, normal, this + DIR, V0, 1);
    lt_s(c, 0, 1)
}

/// Test triangle s1 (index offset s5) with the collider's virtual triangle
/// test (swept or not); v0 = hit.
pub(crate) fn leaf_test_tri(c: &mut Cpu, ret_swept: u32, ret_plain: u32) -> call::Flow {
    use leaf::*;
    c.cycles += 14;
    let mesh = r32(c, S3);
    c.g[V0][0] = Addr(mesh + INDICES).lw(c);
    c.g[A2][0] = Addr(mesh + VERTS).lw(c);
    c.g[V0][0] = sx(r32(c, S5).wrapping_add(r32(c, V0)));
    let idx = r32(c, V0);
    c.g[V1][0] = Addr(idx + 2).lbu(c);
    c.g[A0][0] = Addr(idx).lbu(c);
    c.g[A1][0] = Addr(idx + 1).lbu(c);
    c.g[V1][0] = sx(r32(c, V1) << 4);
    c.g[A3][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
    c.g[A0][0] = sx(r32(c, A0) << 4);
    c.g[A1][0] = sx(r32(c, A1) << 4);
    c.g[V0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    let s = sp(c);
    c.g[V1][0] = Addr(s + SP_SWEPT).lw(c);
    let swept = c.g[V1][0] != 0;
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
    let (slot, ret) = if swept {
        c.cycles += 11;
        (VT_TRI_SWEPT, ret_swept)
    } else {
        c.cycles += 9;
        (VT_TRI, ret_plain)
    };
    let this = r32(c, S2);
    c.g[V1][0] = Addr(this + VTABLE).lw(c);
    let vt = r32(c, V1);
    c.g[A1][0] = c.g[V0][0];
    c.g[T1][0] = c.g[SP][0];
    c.g[T2][0] = sx(s + 0x10);
    c.g[A0][0] = Addr(vt + slot).lh(c);
    c.g[T3][0] = sx(s + 0x20);
    c.g[V0][0] = Addr(vt + slot + 4).lw(c);
    let f = r32(c, V0);
    c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
    call_ptr(c, f, ret)
}

/// Count a hit in the stack's hit counter (a0 = the new count).
pub(crate) fn leaf_count_hit(c: &mut Cpu) {
    use leaf::*;
    let s = sp(c);
    c.g[A0][0] = Addr(s + SP_HITS).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
    sw_reg(c, s + SP_HITS, A0);
}

/// `int Collide_MeshLeafTriangles(Collider *self, LeafMesh *mesh, int swept,
/// TriHit *out)` (0x32c0f8): test the leaf's triangles against the collider
/// (its virtual triangle test, swept or not), skipping back-facing ones when
/// asked, and write a hit record (type 2, position, normal, fraction,
/// triangle index) per hit. Meshes of 11 triangles or more are tested in
/// boxes of ten, each box first against the collider's virtual box test.
/// Returns the number of hits.
pub fn collide_mesh_leaf_triangles(c: &mut Cpu) {
    use leaf::*;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0xd0);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0xc0);
        frame.save(c, S4, 0x90);
        frame.save(c, S5, 0x80);
        frame.save(c, S6, 0x70);
        frame.save(c, S7, 0x60);
        frame.save(c, FP, 0x50);
        frame.save_ra(c, 0x40);
        let mesh = r32(c, S3);
        c.g[V0][0] = Addr(mesh + NUM_TRIS).lhu(c);
        let s = sp(c);
        sw_reg(c, s + SP_SWEPT, A2);
        sw_reg(c, s + SP_OUT, A3);
        c.g[V0][0] = (c.g[V0][0] < 11) as u64;
        Addr(s + SP_HITS).sw(c, 0);
        if c.g[V0][0] != 0 {
            leaf_small(c)?;
        } else {
            leaf_boxed(c)?;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore(c, S7, 0x60);
        frame.restore(c, FP, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Fewer than 11 triangles: test each (v0 = the hit count at the end).
pub(crate) fn leaf_small(c: &mut Cpu) -> call::Flow {
    use leaf::*;
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, S3) + NUM_TRIS).lhu(c);
    c.g[S1][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.g[V0][0] = Addr(sp(c) + SP_HITS).lw(c);
        return Ok(());
    }
    c.cycles += 6;
    c.g[V0][0] = 0x0050_0000;
    c.g[S0][0] = c.g[A3][0];
    c.g[S6][0] = sx(ZERO_VEC);
    c.g[S4][0] = u64::MAX;
    c.g[S5][0] = 0;
    loop {
        if leaf_faces_away(c, GP_FACING_SMALL) {
            c.g[V0][0] = Addr(r32(c, S3) + NUM_TRIS).lhu(c);
        } else {
            leaf_test_tri(c, 0x0032_c210, 0x0032_c23c)?;
            c.cycles += 2;
            let hit = c.g[V0][0] != 0;
            c.g[V0][0] = 2;
            if hit {
                c.cycles += 27;
                let out = r32(c, S0);
                let none = c.g[S4][0];
                Addr(out + 0x54).sw(c, 0);
                sw_reg(c, out + 0x44, V0);
                leaf_count_hit(c);
                Addr(out + 0x60).sw(c, none);
                Addr(out + 0x50).sw(c, 0);
                Addr(out + 0x58).sw(c, 0);
                Addr(out + 0x5c).sw(c, none);
                Addr(out + 0x64).sw(c, none);
                Addr(out + 0x4c).sw(c, none);
                let s = sp(c);
                let zero = r32(c, S6);
                c.lq(V0, zero);
                c.lq(V1, s);
                c.lq(A0, s + 0x10);
                c.sq(V0, out + 0x20);
                c.lwc1(0, s + 0x20);
                c.lq(V0, zero);
                c.sq(V1, out);
                c.sq(V0, out + 0x30);
                c.sq(A0, out + 0x10);
                c.swc1(0, out + 0x40);
                Addr(out + 0x68).sw(c, 0);
                Addr(out + 0x6c).sw(c, 0);
                Addr(out + 0x70).sw(c, 0);
                sw_reg(c, out + 0x60, S1);
                Addr(out + 0x54).sw(c, 0);
                c.g[S0][0] = sx(out.wrapping_add(HIT_SIZE));
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, S3) + NUM_TRIS).lhu(c);
        }
        c.cycles += 4;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
        let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
        c.g[V0][0] = more as u64;
        c.g[S5][0] = sx(r32(c, S5).wrapping_add(3));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_c168);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = Addr(sp(c) + SP_HITS).lw(c);
    Ok(())
}

/// 11 triangles or more: per box of ten that passes the box test, test its
/// triangles (v0 = the hit count at the end).
pub(crate) fn leaf_boxed(c: &mut Cpu) -> call::Flow {
    use leaf::*;
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, S3) + NUM_TRIS).lhu(c);
    c.g[V1][0] = TRIS_PER_BOX;
    c.cycles += 6;
    c.g[S0][0] = 0;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(9));
    c.div(0, V0, V1);
    c.g[A0][0] = c.lo[0];
    let s = sp(c);
    sw_reg(c, s + SP_BOXES, A0);
    if c.g[A0][0] != 0 {
        c.cycles += 1;
        c.g[V1][0] = BOX_SIZE;
        loop {
            // s0: the box index (fp: the next one).
            c.cycles += 12;
            let this = r32(c, S2);
            c.g[A2][0] = Addr(this + VTABLE).lw(c);
            c.mult(0, V1, S0, V1);
            c.g[A1][0] = Addr(r32(c, S3) + BOXES).lw(c);
            let vt = r32(c, A2);
            c.g[A0][0] = Addr(vt + VT_BOX).lh(c);
            c.g[FP][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = Addr(vt + VT_BOX + 4).lw(c);
            c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
            let f = r32(c, V0);
            c.g[A2][0] = sx(r32(c, A1).wrapping_add(0xc));
            call_ptr(c, f, 0x0032_c320)?;
            let inside = c.g[V0][0] != 0;
            c.g[V0][0] = TRIS_PER_BOX;
            if inside {
                leaf_box_tris(c)?;
            }
            c.cycles += 5;
            let s = sp(c);
            c.g[V1][0] = Addr(s + SP_BOXES).lw(c);
            c.g[S0][0] = c.g[FP][0];
            let more = (c.g[S0][0] as i64) < (c.g[V1][0] as i64);
            c.g[V0][0] = more as u64;
            c.g[V1][0] = BOX_SIZE;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_c2f8);
            }
        }
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(sp(c) + SP_HITS).lw(c);
    Ok(())
}

/// The triangles of box s0 (from s0*10 to min(fp*10, count)).
pub(crate) fn leaf_box_tris(c: &mut Cpu) -> call::Flow {
    use leaf::*;
    c.cycles += 8;
    c.g[S4][0] = Addr(r32(c, S3) + NUM_TRIS).lhu(c);
    c.mult(0, A0, FP, V0);
    c.mult(0, S1, S0, V0);
    if (c.g[A0][0] as i64) < (c.g[S4][0] as i64) {
        c.g[V0][0] = 1;
        c.g[S4][0] = c.g[A0][0];
    } else {
        c.g[V0][0] = 0;
    }
    let any = (c.g[S1][0] as i64) < (c.g[S4][0] as i64);
    c.g[V1][0] = any as u64;
    c.g[V0][0] = 0x0050_0000;
    if !any {
        return Ok(());
    }
    c.cycles += 8;
    c.g[A0][0] = sx(r32(c, S1) << 1);
    c.g[S7][0] = sx(ZERO_VEC);
    c.g[S5][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
    let s = sp(c);
    c.g[V0][0] = Addr(s + SP_HITS).lw(c);
    c.g[S6][0] = u64::MAX;
    c.g[A0][0] = Addr(s + SP_OUT).lw(c);
    c.g[V1][0] = sx(r32(c, V0) << 7);
    c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    loop {
        if leaf_faces_away(c, GP_FACING_BOXED) {
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
        } else {
            leaf_test_tri(c, 0x0032_c410, 0x0032_c43c)?;
            c.cycles += 2;
            let hit = c.g[V0][0] != 0;
            c.g[V0][0] = 2;
            if hit {
                c.cycles += 27;
                let out = r32(c, S0);
                let none = c.g[S6][0];
                let zero = r32(c, S7);
                Addr(out + 0x54).sw(c, 0);
                sw_reg(c, out + 0x44, V0);
                leaf_count_hit(c);
                Addr(out + 0x60).sw(c, none);
                Addr(out + 0x50).sw(c, 0);
                Addr(out + 0x58).sw(c, 0);
                Addr(out + 0x5c).sw(c, none);
                Addr(out + 0x64).sw(c, none);
                Addr(out + 0x4c).sw(c, none);
                c.lq(V0, zero);
                c.sq(V0, out + 0x20);
                c.lq(V1, zero);
                Addr(out + 0x68).sw(c, 0);
                c.sq(V1, out + 0x30);
                Addr(out + 0x6c).sw(c, 0);
                Addr(out + 0x70).sw(c, 0);
                let s = sp(c);
                c.lq(V0, s + 0x10);
                c.lq(V1, s);
                c.sq(V0, out + 0x10);
                c.sq(V1, out);
                c.lwc1(0, s + 0x20);
                sw_reg(c, out + 0x60, S1);
                c.swc1(0, out + 0x40);
                Addr(out + 0x54).sw(c, 0);
                c.g[S0][0] = sx(out.wrapping_add(HIT_SIZE));
            }
            c.cycles += 1;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
        }
        c.cycles += 3;
        let more = (c.g[S1][0] as i64) < (c.g[S4][0] as i64);
        c.g[V0][0] = more as u64;
        c.g[S5][0] = sx(r32(c, S5).wrapping_add(3));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_c368);
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// CollHull_CollideTriangle (0x32f8f0)

/// The hull's sphere tree versus a triangle (`0x32a1c0`, through the table).
pub(crate) const SPHERE_TREE_COLLIDE_TRIANGLE: u32 = 0x0032_a1c0;

/// `int CollHull_CollideTriangle(CollHull* hull, const Vec4* p, const Vec4* a,
/// const Vec4* b, const Vec4* n, Vec4* point, float* out_len, Vec4* out_dir)`
/// (0x32f8f0): collide the hull's sphere tree (`+0x60`) with the triangle;
/// on contact store the push vector's length in `out_len` and its
/// direction in `out_dir` (VU0 sqrt/div). Returns the contact flag. The
/// same shape as `CollHull_CollideTriangle2Sided`, with the one-sided test.
pub fn coll_hull_collide_triangle(c: &mut Cpu) {
    guest(c, |c| {
        crate::game::intersect::riderspheretree::coll_sphere_tree_common(
            c,
            SPHERE_TREE_COLLIDE_TRIANGLE,
            0x0032_f918,
        )
    })
}

// ---------------------------------------------------------------------------
// CollHull_Transform (0x32f840).

/// `CollHull` field: the source sphere tree (`sSphereTreeNode*`).
pub(crate) const HULL_SOURCE_TREE: u32 = 0x60;

/// `CollHull` field: the transformed sphere tree (inline).
pub(crate) const HULL_TREE: u32 = 0x70;

/// `CollHull` field: "transformed" flag.
pub(crate) const HULL_TRANSFORMED: u32 = 0x4;

/// `RiderSphereTree_Assign(dst, src)`.
pub(crate) const HULL_SPHERE_TREE_ASSIGN: u32 = 0x0032_9a28;

/// `RiderSphereTree_Scale(tree, matrix, scale)`.
pub(crate) const HULL_SPHERE_TREE_SCALE: u32 = 0x0032_9b90;

/// `void CollHull_Transform(CollHull* hull, sSphereMtx* m, float scale)`
/// (0x32f840): copy the hull's source sphere tree into its own tree,
/// point the source at the copy, transform the copy by `m` and `scale`,
/// and mark the hull transformed.
///
/// Frame 0x50 (s0 +0x30, s1 +0x20, s2 +0x10, ra +0, f20 +0x40).
pub fn coll_hull_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        let hull = reg(c, A0);
        set_reg(c, S1, hull);
        frame.save_ra(c, 0);
        let matrix = reg(c, A1);
        set_reg(c, S2, matrix);
        frame.save_fpr(c, 20, 0x40);
        let tree = addu(hull, HULL_TREE as u64);
        set_reg(c, S0, tree);
        c.f[20] = c.f[12];
        set_reg(c, A0, tree);
        let source = Addr(hull as u32).off(HULL_SOURCE_TREE).lw(c);
        set_reg(c, A1, source);
        call_ptr(c, HULL_SPHERE_TREE_ASSIGN, 0x0032_f874)?;
        let hull = reg(c, S1);
        let tree = reg(c, S0);
        Addr(hull as u32).off(HULL_SOURCE_TREE).sw(c, tree);
        set_reg(c, A0, tree);
        let matrix = reg(c, S2);
        set_reg(c, A1, matrix);
        c.f[12] = c.f[20];
        call_ptr(c, HULL_SPHERE_TREE_SCALE, 0x0032_f888)?;
        set_reg(c, V0, 1);
        frame.restore(c, S0, 0x30);
        let hull = reg(c, S1);
        Addr(hull as u32).off(HULL_TRANSFORMED).sw(c, 1);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderSphereTree_Assign (0x329a28, riderspheretree.cpp)

/// Sphere tree node fields: type, center (quad), radius, user word, parent,
/// child count, child spheres (0x20 bytes each).
pub(crate) const OC264_RST_TYPE: u32 = 0x0;

pub(crate) const OC264_RST_CENTER: u32 = 0x10;

pub(crate) const OC264_RST_RADIUS: u32 = 0x20;

pub(crate) const OC264_RST_USER: u32 = 0x24;

pub(crate) const OC264_RST_PARENT: u32 = 0x28;

pub(crate) const OC264_RST_COUNT: u32 = 0x2c;

pub(crate) const OC264_RST_CHILDREN: u32 = 0x30;

/// `EA_MemCopy` (reached through the function table).
pub(crate) const OC264_EA_MEM_COPY: u32 = 0x003e_6574;

/// `RiderSphereTree* RiderSphereTree_Assign(RiderSphereTree* self, const
/// RiderSphereTree* src)` (0x329a28): copy assignment: the header fields,
/// then `count` child spheres (`EA_MemCopy(self->children, src->children,
/// count * 0x20)`); returns `self`.
pub fn rider_sphere_tree_assign(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        let src = r32(c, A3);
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(src.wrapping_add(OC264_RST_CHILDREN));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A0][0] = sx(this.wrapping_add(OC264_RST_CHILDREN));
        let kind = Addr(src.wrapping_add(OC264_RST_TYPE)).lw(c);
        c.g[V0][0] = kind;
        Addr(this.wrapping_add(OC264_RST_TYPE)).sw(c, kind);
        c.lwc1(0, src.wrapping_add(OC264_RST_RADIUS));
        c.swc1(0, this.wrapping_add(OC264_RST_RADIUS));
        c.lq(V0, src.wrapping_add(OC264_RST_CENTER));
        c.sq(V0, this.wrapping_add(OC264_RST_CENTER));
        let user = Addr(src.wrapping_add(OC264_RST_USER)).lw(c);
        c.g[V1][0] = user;
        Addr(this.wrapping_add(OC264_RST_USER)).sw(c, user);
        let count = Addr(src.wrapping_add(OC264_RST_COUNT)).lw(c);
        Addr(this.wrapping_add(OC264_RST_COUNT)).sw(c, count);
        c.g[A2][0] = sx((count as u32) << 5);
        let parent = Addr(src.wrapping_add(OC264_RST_PARENT)).lw(c);
        c.g[V0][0] = parent;
        Addr(this.wrapping_add(OC264_RST_PARENT)).sw(c, parent);
        call_ptr(c, OC264_EA_MEM_COPY, 0x0032_9a7c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CollHull_OverlapsAABB (0x32f760)

/// Collision hull -> its bounding sphere (`{center: vec4 @0x10, radius @0x20}`).
pub(crate) const HULL_SPHERE: u32 = 0x60;

pub(crate) const HULL_SPHERE_CENTER: u32 = 0x10;

pub(crate) const HULL_SPHERE_RADIUS: u32 = 0x20;

/// `c.le.s a, b` with the condition bit latched.
pub(crate) fn hull_le(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let le = a.le(b);
    abi::set_fcc(c, le);
    le
}

/// `bool CollHull_OverlapsAABB(CollHull *hull, const vec4 *min, const vec4 *max)`
/// (0x32f760): whether the box around the hull's bounding sphere
/// (`center -+ radius` on x, y, z) overlaps the box `[min, max]`. The box
/// corners are built on the stack with VU0 (`vsub`/`vadd`), then x, y and z
/// are compared in turn, stopping at the first separated axis.
pub fn coll_hull_overlaps_aabb(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::Src;
    c.cycles += 26;
    let hull = Addr(abi::arg(c, 0));
    let min = Addr(abi::arg(c, 1));
    let max = Addr(abi::arg(c, 2));
    let sphere = hull.off(HULL_SPHERE).lw(c);
    c.g[V1][0] = sphere;
    let frame = Frame::open(c, 0x40);
    c.g[A3][0] = 0;
    let radius = Addr(sphere as u32).off(HULL_SPHERE_RADIUS).read_fpu(c);
    c.f[0] = radius.0;
    // (r, r, r, 0) at sp.
    let extent = frame.at(c, 0);
    Addr(extent).off(0xc).sw(c, 0);
    Addr(extent).off(8).write_fpu(c, radius);
    Addr(extent).write_fpu(c, radius);
    Addr(extent).off(4).write_fpu(c, radius);
    // Low corner: center - extent (to sp+0x20, copied to sp+0x10).
    let lo = frame.at(c, 0x10);
    let hi = frame.at(c, 0x20);
    let tmp_hi = frame.at(c, 0x30);
    c.lqc2(3, (sphere as u32).wrapping_add(HULL_SPHERE_CENTER));
    c.lqc2(4, extent);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, hi);
    c.lq(V0, hi);
    let sphere = hull.off(HULL_SPHERE).lw(c);
    c.g[V1][0] = sphere;
    c.sq(V0, lo);
    // High corner: center + extent (to sp+0x30, copied to sp+0x20).
    c.lqc2(3, (sphere as u32).wrapping_add(HULL_SPHERE_CENTER));
    c.lqc2(4, extent);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, tmp_hi);
    let max_x = max.read_fpu(c);
    c.f[1] = max_x.0;
    let lo_x = Addr(lo).read_fpu(c);
    c.f[0] = lo_x.0;
    c.lq(V0, tmp_hi);
    let mut overlap = hull_le(c, lo_x, max_x);
    c.sq(V0, hi);

    // Per axis: lo <= max, then min <= hi (the next value loads in the
    // branch delay slot, also when the test fails).
    if overlap {
        for axis in 0..3u32 {
            if axis > 0 {
                c.cycles += 5;
                let lo_a = FpuF32(c.f[0]);
                let max_a = max.off(4 * axis).read_fpu(c);
                c.f[1] = max_a.0;
                overlap = hull_le(c, lo_a, max_a);
                let hi_a = Addr(hi).off(4 * axis).read_fpu(c);
                c.f[0] = hi_a.0;
                if !overlap {
                    break;
                }
            } else {
                c.cycles += 1;
            }
            c.cycles += 5;
            let min_a = min.off(4 * axis).read_fpu(c);
            c.f[1] = min_a.0;
            let hi_a = if axis == 0 {
                let h = Addr(hi).read_fpu(c);
                c.f[0] = h.0;
                h
            } else {
                FpuF32(c.f[0])
            };
            overlap = hull_le(c, min_a, hi_a);
            if axis < 2 {
                let lo_next = Addr(lo).off(4 * (axis + 1)).read_fpu(c);
                c.f[0] = lo_next.0;
            }
            if !overlap {
                break;
            }
        }
    }
    if overlap {
        c.g[A3][0] = 1;
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[A3][0];
    frame.close(c);
}

// ---------------------------------------------------------------------------
// CollHull_CollideAABB (0x32fac0)

/// The hull's sphere tree.
pub(crate) const COLL_HULL_SPHERE: u32 = 0x60;

/// Hit record: normal at +0, push direction at +0x10, its length at +0x40.
pub(crate) const COLL_HULL_HIT_DIR: u32 = 0x10;

pub(crate) const COLL_HULL_HIT_LEN: u32 = 0x40;

pub(crate) const COLL_HULL_COLLIDE_BOX: u32 = 0x0032_b2b8;

/// `bool CollHull_CollideAABB(CollHull* self, Vec4* boxMin, Vec4* boxMax,
/// Hit* hit)` (0x32fac0): collide the hull's sphere tree with the box
/// (`RiderSphereTree_CollideBox(sphere, min, max, &hit->dir, hit)`); on a
/// hit store the push direction's length at `+0x40` and normalise it (VU0
/// `vsqrt`, then `vdiv` by the length). Returns whether it hit.
pub fn coll_hull_collide_aabb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0];
        c.g[A3][0] = sx(r32(c, S0).wrapping_add(COLL_HULL_HIT_DIR));
        c.g[T0][0] = c.g[S0][0];
        c.g[A0][0] = Addr(r32(c, A0).wrapping_add(COLL_HULL_SPHERE)).lw(c);
        call_ptr(c, COLL_HULL_COLLIDE_BOX, 0x0032_fae0)?;
        let hit = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if hit {
            c.cycles += 19;
            let dir = r32(c, S0).wrapping_add(COLL_HULL_HIT_DIR);
            // len = |dir.xyz|
            c.lqc2(3, dir);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(V0, 22);
            c.mtc1(V0, 0);
            c.swc1(0, r32(c, S0).wrapping_add(COLL_HULL_HIT_LEN));
            c.g[V0][0] = 0x1;
            // dir /= len
            c.mfc1(V1, 0);
            c.qmtc2(V1, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, dir);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            c.sqc2(5, dir);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod sphere_tree_f {

    /// Centre of the node's bounding sphere (quadword).
    pub const CENTER: u32 = 0x10;

    /// Number of child spheres.
    pub const COUNT: u32 = 0x2c;

    /// First child (0x20 bytes each, position quadword first).
    pub const CHILDREN: u32 = 0x30;

    pub const CHILD_SIZE: u32 = 0x20;
}

/// `void RiderSphereTree_Translate(sSphereTreeNode* s, sSphereVec4* v)`
/// (0x329b40): moves a rider's collision sphere tree by `v`: adds it to the
/// node centre and then to each child's position (VU0 `vadd.xyzw`).
pub fn rider_sphere_tree_translate(c: &mut Cpu) {
    use sphere_tree_f::*;
    let node = r32(c, A0);
    c.cycles += 7;
    c.lqc2(3, node.wrapping_add(CENTER));
    c.lqc2(4, r32(c, A1));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, node.wrapping_add(CENTER));
    c.g[V0][0] = Addr(node.wrapping_add(COUNT)).lw(c);
    c.g[A2][0] = 0;
    if c.g[V0][0] as i64 > 0 {
        c.cycles += 1;
        c.g[V1][0] = sx(node.wrapping_add(CHILDREN));
        loop {
            // (Registers re-read every pass: an event may run in between.)
            let child = r32(c, V1);
            let node = r32(c, A0);
            c.cycles += 9;
            c.lqc2(3, child);
            c.lqc2(4, r32(c, A1));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, child);
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
            let count = Addr(node.wrapping_add(COUNT)).lw(c);
            let more = (c.g[A2][0] as i64) < (count as i64);
            c.g[V0][0] = more as u64;
            c.g[V1][0] = sx(child.wrapping_add(CHILD_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_9b60);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `char* InputPreproc_Run(InputPreproc* self, const char* path, int* size)`
/// (0x325f48, `cInputPreProcessor::loadFileSize`): resets the preprocessor
/// (size 0, capacity 0x800, a 0x800-byte output buffer), splits `path` at its
/// last `/` into the directory (kept at `sp+0`, empty when there is no `/`)
/// and the file name (`sp+0x80`), runs `InputPreproc_ProcessFile` over it, copies
/// the output to a fresh buffer of exactly its length, frees the work buffer,
/// stores the length in `*size` and returns the copy.
///
/// Resume entries: when registering, add the `c.pc != entry` fallback to the
/// generated twin (`__entries`, `__generated` in the unwind form); the generated
/// body has resume points, this port does not handle them.
///
/// NOT registered: `InputPreproc_ProcessFile` (not in the override list) runs the
/// whole file reader, so no bounded differential fixture exists.
pub fn input_preproc_run(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x110);
        c.g[V0][0] = 0x800;
        frame.save(c, S3, 0xd0);
        c.g[A3][0] = 0;
        frame.save(c, S4, 0xc0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0xe0);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S5, 0xb0);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S0, 0x100);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S1, 0xf0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1c48));
        frame.save_ra(c, 0xa0);
        c.g[A0][0] = 0x800;
        c.sw(c.g[S3][0] as u32, 0);
        c.g[A2][0] = 0;
        c.sw((c.g[S3][0] as u32).wrapping_add(4), c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x0032_5f98)?; // operator_new_named
        c.sw((c.g[S3][0] as u32).wrapping_add(8), c.g[V0][0]);
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 0x2f;
        call_ptr(c, 0x0041_acc0, 0x0032_5fa8)?; // strrchr
        c.g[S2][0] = c.g[V0][0];
        let no_slash = c.g[S2][0] == 0;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
        if !no_slash {
            c.cycles += 13;
            c.g[A0][0] = c.g[SP][0];
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[S4][0] as u32));
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0041_6ce0, 0x0032_5fc8)?; // strncpy (directory)
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.sb(c.g[V1][0] as u32, 0);
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0041_66f4, 0x0032_5fe0)?; // strcpy (file name)
            c.g[A2][0] = c.g[S1][0];
        } else {
            c.cycles += 6;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            c.sb(c.g[SP][0] as u32, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S4][0];
            call_ptr(c, 0x0041_66f4, 0x0032_5ffc)?; // strcpy (file name)
            c.g[A2][0] = c.g[S0][0];
        }
        c.cycles += 16;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0032_6078, 0x0032_600c)?; // InputPreproc_ProcessFile
        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0032_6020)?; // operator_new_named
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(8));
        c.g[A2][0] = c.lw(c.g[S3][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_605c, 0x0032_6034)?; // memcpy
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(8));
        if c.g[A0][0] == 0 {
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0032_6048)?; // EA_MemFree
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x100);
        c.sw(c.g[S5][0] as u32, c.g[V1][0]);
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// One of the allocation calls of `World_InitSphereTreeCache`:
/// `operator_new_named(size, tag, 0, 0)` returning to `ret`.
pub(crate) fn world_sphere_tree_new(c: &mut Cpu, size: u64, tag: u64, ret: u32) -> Flow {
    c.g[A0][0] = size;
    c.g[A1][0] = tag;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, ret)
}

/// `void World_InitSphereTreeCache(int nSmall, int smallDepth, int nBig, int
/// bigDepth)` (0x32dc10): sets up the sphere-tree buffer cache. A tree of depth
/// `d` has `1 + 8 + ... + 8^d` nodes; the cache gets `nSmall` buffers of the
/// `smallDepth` tree size and `nBig` of the `bigDepth` size (`bigDepth` is also
/// kept at `gp + 0xef8`), each with an id slot set to -1, in the pointer/id
/// arrays at `gp + 0xed8/0xedc` (small) and `gp + 0xee8/0xeec` (big). Counts
/// go to `gp + 0xee0/0xef0`, the cursors (`+0xee4`, `+0xef4`) to 0.
pub fn world_init_sphere_tree_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        let gp = c.g[GP][0] as u32;
        c.sw(gp.wrapping_add(0xef8), c.g[A3][0]);
        frame.save(c, S3, 0x60);
        c.g[V0][0] = 1;
        frame.save(c, S4, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, FP, 0x10);
        c.g[S5][0] = 0;
        frame.save(c, S0, 0x90);
        c.g[FP][0] = 0;
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        let negative = (c.g[A1][0] as i64) < 0;
        frame.save_ra(c, 0);
        // smallSize = 1 + 8 + ... (smallDepth + 1 terms), in s5
        if !negative {
            c.cycles += 2;
            c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            loop {
                c.cycles += 7;
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                let more = c.g[S2][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_dc60);
                    }
                    continue;
                }
                break;
            }
        }
        // bigSize, in fp
        c.cycles += 3;
        c.g[V1][0] = c.lw(gp.wrapping_add(0xef8));
        let negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 1;
        if !negative {
            c.cycles += 2;
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            loop {
                c.cycles += 7;
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                let more = c.g[S2][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_dc90);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 5;
        for off in [0xed8u32, 0xedc, 0xee8] {
            c.sw(gp.wrapping_add(off), 0);
        }
        let no_small = c.g[S4][0] == 0;
        c.sw(gp.wrapping_add(0xeec), 0);
        if !no_small {
            c.cycles += 15;
            c.g[S0][0] = 0x49_0000;
            c.g[S1][0] = sx((c.g[S4][0] as u32) << 2);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1ac0));
            let (size, tag) = (c.g[S1][0], c.g[S0][0]);
            world_sphere_tree_new(c, size, tag, 0x0032_dce0)?; // pointers
            c.sw(gp.wrapping_add(0xed8), c.g[V0][0]);
            let (size, tag) = (c.g[S1][0], c.g[S0][0]);
            world_sphere_tree_new(c, size, tag, 0x0032_dcf8)?; // ids
            c.sw(gp.wrapping_add(0xedc), c.g[V0][0]);
        }
        c.cycles += 2;
        let no_big = c.g[S3][0] == 0;
        c.g[S0][0] = 0x49_0000;
        if !no_big {
            c.cycles += 14;
            c.g[S1][0] = sx((c.g[S3][0] as u32) << 2);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1ac0));
            let (size, tag) = (c.g[S1][0], c.g[S0][0]);
            world_sphere_tree_new(c, size, tag, 0x0032_dd20)?; // pointers
            c.sw(gp.wrapping_add(0xee8), c.g[V0][0]);
            let (size, tag) = (c.g[S1][0], c.g[S0][0]);
            world_sphere_tree_new(c, size, tag, 0x0032_dd38)?; // ids
            c.sw(gp.wrapping_add(0xeec), c.g[V0][0]);
        }
        c.cycles += 2;
        let none = (c.g[S4][0] as i64) <= 0;
        c.g[S2][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[S7][0] = 0x49_0000;
            c.g[S6][0] = 0xffff_ffff_ffff_ffff;
            loop {
                c.cycles += 15;
                c.g[S0][0] = c.lw(gp.wrapping_add(0xed8));
                c.g[S1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                let size = c.g[S5][0];
                let tag = sx((c.g[S7][0] as u32).wrapping_sub(0x1ac0));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                world_sphere_tree_new(c, size, tag, 0x0032_dd74)?; // small buffer
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
                c.g[V0][0] = c.lw(gp.wrapping_add(0xedc));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let more = c.g[V1][0] != 0;
                c.sw(c.g[S1][0] as u32, c.g[S6][0]);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_dd50);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 2;
        let none = (c.g[S3][0] as i64) <= 0;
        c.g[S2][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[S6][0] = 0x49_0000;
            c.g[S5][0] = 0xffff_ffff_ffff_ffff;
            loop {
                c.cycles += 15;
                c.g[S0][0] = c.lw(gp.wrapping_add(0xee8));
                c.g[S1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                let size = c.g[FP][0];
                let tag = sx((c.g[S6][0] as u32).wrapping_sub(0x1ac0));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                world_sphere_tree_new(c, size, tag, 0x0032_ddc4)?; // big buffer
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                c.g[V0][0] = c.lw(gp.wrapping_add(0xeec));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let more = c.g[V1][0] != 0;
                c.sw(c.g[S1][0] as u32, c.g[S5][0]);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_dda0);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 16;
        c.sw(gp.wrapping_add(0xee0), c.g[S4][0]);
        c.sw(gp.wrapping_add(0xef0), c.g[S3][0]);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        c.sw(gp.wrapping_add(0xee4), 0);
        c.sw(gp.wrapping_add(0xef4), 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod collide_plane_node {

    /// `float scale`: multiplies a level's radius.
    pub(crate) const SCALE: u32 = 0x90;

    /// `float worstCase`: the closest penetration so far, the distance at
    /// which a leaf last updated the outputs.
    pub(crate) const WORST_CASE: u32 = 0x94;

    /// `SphereTreeInfo* treeInfo`.
    pub(crate) const TREE_INFO: u32 = 0x98;

    /// `int sortAxis[8]`: the order the children are visited in.
    pub(crate) const SORT_AXIS: u32 = 0x9c;

    /// `SphereTreeInfo`: `int depth`.
    pub(crate) const INFO_DEPTH: u32 = 0x0c;

    /// `SphereTreeInfo`: `levels` (12-byte records).
    pub(crate) const INFO_LEVELS: u32 = 0x20;

    /// `SphereTreeInfo`: `unsigned char* bitTree` (one mask per node).
    pub(crate) const INFO_BIT_TREE: u32 = 0x28;

    /// A level record: `float radius` +0, `float cenOffset` +4, `int
    /// indexOffset` +8.
    pub(crate) const LEVEL_SIZE: u32 = 12;

    pub(crate) const LEVEL_RADIUS: u32 = 0;

    pub(crate) const LEVEL_CEN_OFFSET: u32 = 4;

    pub(crate) const LEVEL_INDEX_OFFSET: u32 = 8;

    /// The stack frame size.
    pub(crate) const FRAME: u32 = 0x100;

    /// Where a child call returns to.
    pub(crate) const RET_CHILD: u32 = 0x0032_d368;

    /// The child-loop head (the event check resumes there).
    pub(crate) const LOOP_HEAD: u32 = 0x0032_d2e0;

    /// This function's entry.
    pub(crate) const ENTRY: u32 = 0x0032_d208;
}

/// `RiderSphereTree_Assign(tree)` (0x329a28).
pub(crate) const RIDER_SPHERE_TREE_ASSIGN: Callee = Callee {
    addr: 0x0032_9a28,
    f: ee::s10::f_00329a28,
};

/// `RiderSphereTree* RiderSphereTree_ctor_copy(RiderSphereTree* t)`
/// (0x329938): sets the tree up by `RiderSphereTree_Assign` and returns it.
pub fn rider_sphere_tree_ctor_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x13;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = u64::MAX;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[S0][0];
        RIDER_SPHERE_TREE_ASSIGN.call(c, 0x0032_9958)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `int RLE_Decode(u8* dst, const s8* src)` (0x32b620): decodes the
/// signed-header run-length stream at `src` into `dst`. A header `h >= 0`
/// repeats the next byte `h + 1` times; `h < 0` copies the next `-h`
/// bytes; a zero header ends the stream. Returns the bytes written.
pub fn rle_decode(c: &mut Cpu) {
    c.cycles += 2;
    let has_src = c.g[A1][0] != 0;
    c.g[A2][0] = c.g[A0][0];
    if !has_src {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    c.cycles += 2;
    c.g[A3][0] = Addr(c.g[A1][0] as u32).lb(c);
    loop {
        c.cycles += 2;
        let repeat = (c.g[A3][0] as i64) >= 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        if repeat {
            c.cycles += 2;
            let end = c.g[A3][0] == 0;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            if end {
                break;
            }
            c.cycles += 1;
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lbu(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                let value = c.g[V0][0];
                Addr(c.g[A2][0] as u32).sb(c, value);
                let more = c.g[V1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_b678);
                    }
                    continue;
                }
                c.cycles += 1;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                break;
            }
        } else {
            c.cycles += 2;
            c.g[V1][0] = sx(0u32.wrapping_sub(c.g[A3][0] as u32));
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lbu(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                let value = c.g[V0][0];
                Addr(c.g[A2][0] as u32).sb(c, value);
                let more = c.g[V1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_b648);
                    }
                    continue;
                }
                c.cycles += 2;
                break;
            }
        }
        c.cycles += 2;
        if c.g[A3][0] != 0 {
            c.g[A3][0] = Addr(c.g[A1][0] as u32).lb(c);
            if event_due(c) {
                deliver_event_here(c, 0x0032_b638);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    ret(c);
}

// Field offsets of the sphere tree node (`self`) and of its tree info.
/// `t4Vector axis[8]`: one 16-byte axis per child, indexed by child number.
pub(crate) const SCT_AXIS: u32 = 0x00;

/// `float scale`.
pub(crate) const SCT_SCALE: u32 = 0x90;

/// `float worstCase`: the closest plane offset found so far.
pub(crate) const SCT_WORST_CASE: u32 = 0x94;

/// `SphereTreeInfo* treeInfo`.
pub(crate) const SCT_TREE_INFO: u32 = 0x98;

/// `int sortAxis[8]`: the child order.
pub(crate) const SCT_SORT_AXIS: u32 = 0x9c;

/// `int depth` of the tree.
pub(crate) const SCT_INFO_DEPTH: u32 = 0x0c;

/// `Level* levels`, 12 bytes each: radius, cenOffset, indexOffset.
pub(crate) const SCT_INFO_LEVELS: u32 = 0x20;

/// `unsigned char* bitTree`: one byte per node, bit k set when child k exists.
pub(crate) const SCT_INFO_BIT_TREE: u32 = 0x28;

pub(crate) const SCT_LEVEL_STRIDE: u32 = 12;

pub(crate) const SCT_LEVEL_CEN_OFFSET: u32 = 4;

pub(crate) const SCT_LEVEL_INDEX_OFFSET: u32 = 8;

/// The node function itself: the recursion into a child.
pub(crate) const SCT_RECURSE: Callee = Callee {
    addr: 0x0032_d470,
    f: ee::s11::f_0032d470,
};

/// `PointInTriangle` (0x32d440): is the plane point inside the triangle.
pub(crate) const SCT_POINT_IN_TRIANGLE: Callee = Callee {
    addr: 0x0032_d440,
    f: ee::s11::f_0032d440,
};

/// `base + off` for a register: the address `off(reg)` as the compiled code
/// forms it (32-bit wrap).
#[inline]
pub(crate) fn sct_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `void SphereTree_CollideTriangleNode(SphereTree* self, t4Vector* p1,
/// t4Vector* p2, t4Vector* p3, t4Vector* uNorm, t4Vector* pos, int depth,
/// int base, t4Vector* outA, t4Vector* outB)` (0x32d470): tests one node of
/// the sphere tree against a triangle. The plane distance of `pos` is
/// compared with the node's sphere (level radius times scale) and with
/// `worstCase`; the sphere's plane point is tested against the triangle, and
/// a leaf (or a node the triangle misses) that is the closest so far stores
/// its normal and plane point into `outA`/`outB` and raises `worstCase`. An
/// inner node recurses into its children, in `sortAxis` order, for the bits
/// set in its byte.
pub fn sphere_tree_collide_triangle_node(c: &mut Cpu) {
    guest(c, sct_body);
}

pub(crate) fn sct_body(c: &mut Cpu) -> Flow {
    'b_32da0c: {
        'b_32d53c: {
            c.cycles += 46;
            c.g[SP][0] = sx(sp(c).wrapping_sub(0x140));
            c.g[V1][0] = 0xc;
            c.sq(21, sct_at(c, SP, 0xd0));
            c.sq(30, sct_at(c, SP, 0xa0));
            c.g[S5][0] = c.g[A1][0];
            c.sq(17, sct_at(c, SP, 0x110));
            c.g[FP][0] = c.g[T1][0];
            c.sq(19, sct_at(c, SP, 0xf0));
            c.g[S1][0] = c.g[A0][0];
            c.sq(20, sct_at(c, SP, 0xe0));
            c.g[S3][0] = c.g[T0][0];
            c.sq(23, sct_at(c, SP, 0xb0));
            c.g[S4][0] = c.g[T2][0];
            c.swc1(20, sct_at(c, SP, 0x130));
            // depth * 12: the level's offset in `levels`.
            c.mult(0, 3, 20, 3);
            c.sq(16, sct_at(c, SP, 0x120));
            c.g[S7][0] = c.g[A2][0];
            c.sq(18, sct_at(c, SP, 0x100));
            c.sq(22, sct_at(c, SP, 0xc0));
            c.sd(sct_at(c, SP, 0x90), c.g[RA][0]);
            c.swc1(21, sct_at(c, SP, 0x138));
            c.lqc2(3, c.g[FP][0] as u32);
            c.lqc2(4, c.g[S5][0] as u32);
            // planePoint offset: pos - point1.
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sct_at(c, SP, 0x20));
            c.lq(2, sct_at(c, SP, 0x20));
            c.g[A0][0] = c.lw(sct_at(c, S1, SCT_TREE_INFO));
            c.lwc1(1, sct_at(c, S1, SCT_SCALE));
            c.sq(2, sct_at(c, SP, 0x10));
            c.sw(sct_at(c, SP, 0x80), c.g[A3][0]);
            c.lqc2(3, c.g[S3][0] as u32);
            c.lqc2(5, sct_at(c, SP, 0x10));
            // planeDist = Dot(uNorm, pos - point1).
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 21);
            c.sw(sct_at(c, SP, 0x84), c.g[T3][0]);
            c.mov_s(2, 21);
            c.g[V0][0] = c.lw(sct_at(c, A0, SCT_INFO_LEVELS));
            c.g[V1][0] = sx(sct_at(c, V1, 0).wrapping_add(c.g[V0][0] as u32));
            // levelRadius = levels[depth].radius * self->scale.
            c.lwc1(0, c.g[V1][0] as u32);
            let t = c.g[S4][0] != 0u64;
            c.mul_s(20, 0, 1);
            if t {
                break 'b_32d53c;
            } else {
                // depth 0: a triangle behind the plane is not hit.
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_lt_s(21, 0);
                let t = c.fcc();
                c.lq(16, sct_at(c, SP, 0x120));
                if t {
                    break 'b_32da0c;
                } else {
                    break 'b_32d53c;
                }
            }
        }
        'b_32da08: {
            // if (!(planeDist < levelRadius + worstCase)) return;
            c.cycles += 6;
            c.lwc1(0, sct_at(c, S1, SCT_WORST_CASE));
            c.add_s(0, 20, 0);
            c.c_lt_s(21, 0);
            let t = !c.fcc();
            c.g[S0][0] = 0u64;
            if t {
                break 'b_32da08;
            } else {
                // leaf (S0 = 1) unless the node has children below depth 3.
                'b_32d58c: {
                    'b_32d588: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(sct_at(c, A0, SCT_INFO_BIT_TREE));
                        c.g[V1][0] = c.lw(sct_at(c, A0, SCT_INFO_DEPTH));
                        c.g[A0][0] = c.lw(sct_at(c, SP, 0x84));
                        c.g[V1][0] = ((c.g[S4][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                        let t = c.g[V1][0] == 0u64;
                        c.sw(sct_at(c, SP, 0x88), c.g[V0][0]);
                        if t {
                            break 'b_32d588;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S4][0] as i64) < 3i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw(sct_at(c, SP, 0x88));
                            if t {
                                break 'b_32d588;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_32d58c;
                                } else {
                                    break 'b_32d588;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[S0][0] = 0x1;
                    break 'b_32d58c;
                }
                // The triangle test of the sphere (PointInTriangle on the
                // plane point), then the edge tests when it misses.
                'b_32d8dc: {
                    c.cycles += 22;
                    c.mfc1(2, 2);
                    c.lqc2(4, c.g[S3][0] as u32);
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, sct_at(c, SP, 0x30));
                    c.g[A0][0] = c.g[S1][0];
                    c.lq(2, sct_at(c, SP, 0x30));
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A3][0] = c.lw(sct_at(c, SP, 0x80));
                    c.g[A2][0] = c.g[S7][0];
                    c.sq(2, sct_at(c, SP, 0x20));
                    c.g[T0][0] = sx(sct_at(c, SP, 0x10));
                    c.lqc2(3, c.g[FP][0] as u32);
                    c.lqc2(4, sct_at(c, SP, 0x20));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sct_at(c, SP, 0x30));
                    c.mov_s(12, 20);
                    c.lq(2, sct_at(c, SP, 0x30));
                    c.sq(2, sct_at(c, SP, 0x10));
                    SCT_POINT_IN_TRIANGLE.call(c, 0x0032_d5dc)?;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_32d8dc;
                    } else {
                        c.cycles += 2;
                        if c.g[S0][0] != 0u64 {
                            c.lq(16, sct_at(c, SP, 0x120));
                            break 'b_32da0c;
                        } else {
                            c.cycles += 6;
                            c.g[AT][0] = 0x41a00000;
                            c.mtc1(1, 0);
                            c.c_lt_s(20, 0);
                            if c.fcc() {
                                c.lq(16, sct_at(c, SP, 0x120));
                                break 'b_32da0c;
                            } else {
                                c.cycles += 19;
                                c.lqc2(3, c.g[FP][0] as u32);
                                c.lqc2(4, c.g[S5][0] as u32);
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, sct_at(c, SP, 0x30));
                                c.mul_s(3, 20, 20);
                                c.lq(2, sct_at(c, SP, 0x30));
                                c.sq(2, sct_at(c, SP, 0x20));
                                c.lqc2(3, sct_at(c, SP, 0x20));
                                c.lqc2(5, sct_at(c, SP, 0x20));
                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                c.qmfc2(3, 4);
                                c.mtc1(3, 5);
                                c.c_lt_s(3, 5);
                                let t = !c.fcc();
                                if t {
                                    break 'b_32d8dc;
                                } else {
                                    'b_32d70c: {
                                        'b_32d708: {
                                            c.cycles += 31;
                                            c.lqc2(3, c.g[S7][0] as u32);
                                            c.lqc2(4, c.g[S5][0] as u32);
                                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                            c.sqc2(5, sct_at(c, SP, 0x40));
                                            c.mtc1(0, 1);
                                            c.lq(2, sct_at(c, SP, 0x40));
                                            c.sq(2, sct_at(c, SP, 0x30));
                                            c.lqc2(3, sct_at(c, SP, 0x30));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.vu0().sqrt(4, 0);
                                            c.vu0().waitq();
                                            c.cfc2(3, 22);
                                            c.mtc1(3, 4);
                                            c.lqc2(3, sct_at(c, SP, 0x20));
                                            c.lqc2(5, sct_at(c, SP, 0x30));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(2, 4);
                                            c.mtc1(2, 0);
                                            c.div_s(2, 0, 4);
                                            c.c_lt_s(2, 1);
                                            let t = c.fcc();
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_32d708;
                                            } else {
                                                c.cycles += 6;
                                                c.mul_s(0, 2, 2);
                                                c.sub_s(0, 5, 0);
                                                c.c_lt_s(3, 0);
                                                let t = c.fcc();
                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                if t {
                                                    break 'b_32d70c;
                                                } else {
                                                    c.cycles += 8;
                                                    c.sub_s(0, 3, 0);
                                                    c.g[V0][0] = 0x1;
                                                    c.sqrt_s(0, 0);
                                                    c.sub_s(0, 2, 0);
                                                    c.c_lt_s(4, 0);
                                                    let t = !c.fcc();
                                                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                    if t {
                                                        break 'b_32d70c;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = 0u64;
                                                        break 'b_32d708;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                        break 'b_32d70c;
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V1][0] = c.lw(sct_at(c, SP, 0x80));
                                    if t {
                                        break 'b_32d8dc;
                                    } else {
                                        'b_32d7d0: {
                                            'b_32d7cc: {
                                                c.cycles += 31;
                                                c.lqc2(3, c.g[V1][0] as u32);
                                                c.lqc2(4, c.g[S5][0] as u32);
                                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                                c.sqc2(5, sct_at(c, SP, 0x50));
                                                c.mtc1(0, 1);
                                                c.lq(2, sct_at(c, SP, 0x50));
                                                c.sq(2, sct_at(c, SP, 0x40));
                                                c.lqc2(3, sct_at(c, SP, 0x40));
                                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                c.vu0().sqrt(4, 0);
                                                c.vu0().waitq();
                                                c.cfc2(3, 22);
                                                c.mtc1(3, 4);
                                                c.lqc2(3, sct_at(c, SP, 0x20));
                                                c.lqc2(5, sct_at(c, SP, 0x40));
                                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                c.qmfc2(2, 4);
                                                c.mtc1(2, 0);
                                                c.div_s(2, 0, 4);
                                                c.c_lt_s(2, 1);
                                                let t = c.fcc();
                                                c.g[V0][0] = 0u64;
                                                if t {
                                                    break 'b_32d7cc;
                                                } else {
                                                    c.cycles += 6;
                                                    c.mul_s(0, 2, 2);
                                                    c.sub_s(0, 5, 0);
                                                    c.c_lt_s(3, 0);
                                                    let t = c.fcc();
                                                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                    if t {
                                                        break 'b_32d7d0;
                                                    } else {
                                                        c.cycles += 8;
                                                        c.sub_s(0, 3, 0);
                                                        c.g[V0][0] = 0x1;
                                                        c.sqrt_s(0, 0);
                                                        c.sub_s(0, 2, 0);
                                                        c.c_lt_s(4, 0);
                                                        let t = !c.fcc();
                                                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                        if t {
                                                            break 'b_32d7d0;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.g[V0][0] = 0u64;
                                                            break 'b_32d7cc;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                            break 'b_32d7d0;
                                        }
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_32d8dc;
                                        } else {
                                            'b_32d8d4: {
                                                'b_32d8d0: {
                                                    c.cycles += 47;
                                                    c.lqc2(3, c.g[FP][0] as u32);
                                                    c.lqc2(4, c.g[S7][0] as u32);
                                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                                    c.sqc2(5, sct_at(c, SP, 0x60));
                                                    c.mtc1(0, 1);
                                                    c.lq(2, sct_at(c, SP, 0x60));
                                                    c.g[A0][0] = c.lw(sct_at(c, SP, 0x80));
                                                    c.sq(2, sct_at(c, SP, 0x50));
                                                    c.lqc2(3, sct_at(c, SP, 0x50));
                                                    c.lqc2(5, sct_at(c, SP, 0x50));
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.qmfc2(3, 4);
                                                    c.mtc1(3, 5);
                                                    c.lqc2(3, c.g[A0][0] as u32);
                                                    c.lqc2(4, c.g[S7][0] as u32);
                                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                                    c.sqc2(5, sct_at(c, SP, 0x70));
                                                    c.lq(2, sct_at(c, SP, 0x70));
                                                    c.sq(2, sct_at(c, SP, 0x60));
                                                    c.lqc2(3, sct_at(c, SP, 0x60));
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.vu0().sqrt(4, 0);
                                                    c.vu0().waitq();
                                                    c.cfc2(3, 22);
                                                    c.mtc1(3, 4);
                                                    c.lqc2(3, sct_at(c, SP, 0x50));
                                                    c.lqc2(5, sct_at(c, SP, 0x60));
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.qmfc2(2, 4);
                                                    c.mtc1(2, 0);
                                                    c.div_s(2, 0, 4);
                                                    c.c_lt_s(2, 1);
                                                    let t = c.fcc();
                                                    c.g[V0][0] = 0u64;
                                                    if t {
                                                        break 'b_32d8d0;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.mul_s(0, 2, 2);
                                                        c.sub_s(0, 5, 0);
                                                        c.c_lt_s(3, 0);
                                                        let t = c.fcc();
                                                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                        if t {
                                                            break 'b_32d8d4;
                                                        } else {
                                                            c.cycles += 8;
                                                            c.sub_s(0, 3, 0);
                                                            c.g[V0][0] = 0x1;
                                                            c.sqrt_s(0, 0);
                                                            c.sub_s(0, 2, 0);
                                                            c.c_lt_s(4, 0);
                                                            let t = !c.fcc();
                                                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                            if t {
                                                                break 'b_32d8d4;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.g[V0][0] = 0u64;
                                                                break 'b_32d8d0;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                break 'b_32d8d4;
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] != 0u64 {
                                                c.lq(16, sct_at(c, SP, 0x120));
                                                break 'b_32da0c;
                                            } else {
                                                break 'b_32d8dc;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // Inner nodes recurse into the children (sortAxis order, the
                // child bit set in the node's byte); a leaf takes the plane.
                'b_32d9bc: {
                    c.cycles += 2;
                    if c.g[S0][0] != 0u64 {
                        c.sub_s(12, 21, 20);
                        break 'b_32d9bc;
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = 0xc;
                        c.g[V1][0] = c.lw(sct_at(c, S1, SCT_TREE_INFO));
                        c.mult(0, 2, 20, 2);
                        c.g[S0][0] = sx(sct_at(c, S1, SCT_SORT_AXIS));
                        c.g[A0][0] = c.lw(sct_at(c, V1, SCT_INFO_LEVELS));
                        c.g[S2][0] = 0x7;
                        c.g[S6][0] = c.g[V0][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.lwc1(20, sct_at(c, V0, SCT_LEVEL_STRIDE + SCT_LEVEL_CEN_OFFSET));
                        'l_32d908: loop {
                            'b_32d9a8: {
                                c.cycles += 6;
                                c.g[T4][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V1][0] = c.lw(sct_at(c, SP, 0x88));
                                c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32
                                    >> ((c.g[T4][0] as u32) & 31))
                                    as u32);
                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = sx((c.g[T4][0] as u32) << 4);
                                if t {
                                    break 'b_32d9a8;
                                } else {
                                    c.cycles += 34;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[V0][0] =
                                        sx(sct_at(c, V0, SCT_AXIS).wrapping_add(c.g[S1][0] as u32));
                                    c.g[T4][0] = sx(sct_at(c, T4, 1));
                                    c.mfc1(3, 20);
                                    c.lqc2(4, c.g[V0][0] as u32);
                                    c.qmtc2(3, 3);
                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                    c.sqc2(5, sct_at(c, SP, 0x40));
                                    c.g[A1][0] = c.g[S5][0];
                                    c.lq(2, sct_at(c, SP, 0x40));
                                    c.g[A2][0] = c.g[S7][0];
                                    c.g[A3][0] = c.lw(sct_at(c, SP, 0x80));
                                    c.g[T0][0] = c.g[S3][0];
                                    c.sq(2, sct_at(c, SP, 0x30));
                                    c.g[T1][0] = sx(sct_at(c, SP, 0x20));
                                    c.lqc2(3, c.g[FP][0] as u32);
                                    c.lqc2(4, sct_at(c, SP, 0x30));
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, sct_at(c, SP, 0x40));
                                    c.g[T2][0] = sx(sct_at(c, S4, 1));
                                    c.lq(3, sct_at(c, SP, 0x40));
                                    c.g[T3][0] = c.lw(sct_at(c, S1, SCT_TREE_INFO));
                                    c.sq(3, sct_at(c, SP, 0x20));
                                    c.g[V1][0] = c.lw(sct_at(c, SP, 0x148));
                                    c.g[V0][0] = c.lw(sct_at(c, T3, SCT_INFO_LEVELS));
                                    c.g[V0][0] =
                                        sx(sct_at(c, S6, 0).wrapping_add(c.g[V0][0] as u32));
                                    c.g[T3][0] = c.lw(sct_at(c, V0, SCT_LEVEL_INDEX_OFFSET));
                                    c.g[V0][0] = c.lw(sct_at(c, SP, 0x140));
                                    c.sw(sct_at(c, SP, 0x8), c.g[V1][0]);
                                    c.sw(sct_at(c, SP, 0), c.g[V0][0]);
                                    c.g[V0][0] = c.lw(sct_at(c, SP, 0x84));
                                    c.lo[0] = c.g[V0][0];
                                    // child base = (k + 1) * indexOffset + base
                                    c.madd(0, 11, 12, 11);
                                    SCT_RECURSE.call(c, 0x0032_d9a8)?;
                                    break 'b_32d9a8;
                                }
                            }
                            c.cycles += 3;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[S2][0] as i64) >= 0;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0032_d908);
                                }
                                continue 'l_32d908;
                            } else {
                                c.cycles += 2;
                                c.lq(16, sct_at(c, SP, 0x120));
                                break 'b_32da0c;
                            }
                        }
                    }
                }
                // The leaf (or the last child): the plane offset becomes the
                // new closest distance when it is smaller than worstCase.
                c.cycles += 5;
                c.lwc1(0, sct_at(c, S1, SCT_WORST_CASE));
                c.c_lt_s(12, 0);
                let t = !c.fcc();
                c.lq(16, sct_at(c, SP, 0x120));
                if t {
                    break 'b_32da0c;
                } else {
                    c.cycles += 14;
                    c.neg_s(0, 12);
                    c.mfc1(2, 0);
                    c.lqc2(4, c.g[S3][0] as u32);
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, sct_at(c, SP, 0x30));
                    c.lq(3, sct_at(c, SP, 0x30));
                    c.g[A0][0] = c.lw(sct_at(c, SP, 0x140));
                    c.lq(2, sct_at(c, SP, 0x10));
                    c.sq(3, c.g[A0][0] as u32);
                    c.g[A0][0] = c.lw(sct_at(c, SP, 0x148));
                    c.sq(2, c.g[A0][0] as u32);
                    c.swc1(12, sct_at(c, S1, SCT_WORST_CASE));
                    c.sq(3, sct_at(c, SP, 0x20));
                    break 'b_32da08;
                }
            }
        }
        c.cycles += 1;
        c.lq(16, sct_at(c, SP, 0x120));
        break 'b_32da0c;
    }
    c.cycles += 13;
    c.lq(17, sct_at(c, SP, 0x110));
    c.lq(18, sct_at(c, SP, 0x100));
    c.lq(19, sct_at(c, SP, 0xf0));
    c.lq(20, sct_at(c, SP, 0xe0));
    c.lq(21, sct_at(c, SP, 0xd0));
    c.lq(22, sct_at(c, SP, 0xc0));
    c.lq(23, sct_at(c, SP, 0xb0));
    c.lq(30, sct_at(c, SP, 0xa0));
    c.g[RA][0] = c.ld(sct_at(c, SP, 0x90));
    c.lwc1(21, sct_at(c, SP, 0x138));
    c.lwc1(20, sct_at(c, SP, 0x130));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sct_at(c, SP, 0x140));
    c.pc = t;
    Ok(())
}

/// `float Vec3_NegDot(const Vec3* a, const Vec3* b)` (0x3300f8): the negated
/// dot product of the vectors at `a0` and `a1`, computed on VU0 and returned
/// in f0.
pub fn vec3_neg_dot(c: &mut Cpu) {
    c.cycles += 11;
    c.lqc2(3, c.g[A0][0] as u32);
    c.lqc2(5, c.g[A1][0] as u32);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 0);
    c.neg_s(0, 0);
    ret(c);
}

/// `int CollSegment_CollideTriangle2Sided(seg, ..., t0 = triangle vector)`
/// (0x32e5e8). The segment's vector at `seg+0x70` is dotted with the vector
/// at `t0` on VU0. When that dot is below zero, the vector at `t0` is
/// negated (copied to the stack) and passed on; the callback at `+0x24` of
/// the object at `seg+0x50` is then called with `a0 = seg + lh(obj+0x20)`.
pub fn coll_segment_collide_triangle_2sided(c: &mut Cpu) {
    let frame = Frame::open(c, 0x30);
    guest(c, |c| coll_segment_body(c, frame));
}

pub(crate) fn coll_segment_body(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 17;
    c.g[T4][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);
    c.mtc1(0, 0);
    c.lqc2(3, (c.g[T4][0] as u32).wrapping_add(0x70));
    c.lqc2(5, c.g[T0][0] as u32);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 1);
    c.c_lt_s(1, 0);
    let negative = c.fcc();
    c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x50));
    if negative {
        c.cycles += 13;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.mfc1(2, 0);
        c.lqc2(4, c.g[T0][0] as u32);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        let spill = frame.at(c, 0x10);
        c.sqc2(5, spill);
        c.lq(2, spill);
        c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[T0][0] = c.g[SP][0];
        let copy = frame.at(c, 0);
        c.sq(2, copy);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[A0][0] as u32));
    }
    c.cycles += 5;
    let target = c.g[V0][0] as u32;
    call_ptr(c, target, 0x0032_e678)?;
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `void SphereTree_SortChildOrder(SphereTree* node, Vec4* dir)` (0x32db40).
/// Four child centres at `node+0..0x40` are projected on `dir` (scaled by
/// -1000 on VU0) and truncated to int keys; the keys are stored at
/// `node+0x9c` (index order) and `node+0xb8` (descending), then sorted
/// through `ShellSortDesc_KeyVal` (0x32da40) on the stack.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn sphere_tree_sort_child_order(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    guest(c, |c| sort_child_order_body(c, frame));
}

pub(crate) fn sort_child_order_body(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 20;
    c.g[AT][0] = 0xffff_ffff_c47a_0000;
    c.mtc1(1, 0);
    frame.save_ra(c, 0x40);
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9c));
    c.mfc1(2, 0);
    c.lqc2(4, c.g[A1][0] as u32);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let spill = frame.at(c, 0x30);
    c.sqc2(5, spill);
    c.g[T2][0] = 0;
    c.lq(2, spill);
    c.g[A1][0] = c.g[A0][0];
    c.g[T4][0] = 7;
    c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb8));
    let scaled = frame.at(c, 0x20);
    c.sq(2, scaled);
    c.g[T1][0] = c.g[A2][0];
    c.g[T0][0] = sx(frame.at(c, 0x1c));
    c.g[A3][0] = sx(frame.at(c, 0));
    loop {
        c.cycles += 25;
        c.lqc2(3, c.g[A1][0] as u32);
        c.lqc2(5, scaled);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.g[V1][0] = sx((c.g[T4][0] as u32).wrapping_sub(c.g[T2][0] as u32));
        c.cvt_w_s(1, 0);
        c.mfc1(4, 1);
        c.sw(c.g[T1][0] as u32, c.g[T2][0]);
        c.sw(c.g[T3][0] as u32, c.g[V1][0]);
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x4));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
        c.sw(c.g[A3][0] as u32, c.g[A0][0]);
        c.sw(c.g[T0][0] as u32, c.g[V0][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4));
        c.g[V0][0] = ((c.g[T2][0] as i64) < 4i64) as u64;
        let more = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_db90);
        }
    }
    c.cycles += 6;
    c.g[A0][0] = 0x8;
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, 0x0032_da40, 0x0032_dc00)?;
    frame.restore_ra(c, 0x40);
    frame.close(c);
    Ok(())
}

/// `base + off` as a 32-bit guest address (the `off(reg)` of the generated code).
pub(crate) fn sphere_addr(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `int RLE_Encode(out, in, len, minRun)` (0x32b468, `sphTreeRLE`): run-length
/// encodes `len` bytes of `in` into `out`. A run of at least `minRun` bytes (2
/// at least, at most 127) is written as `count, byte`; other bytes go out as a
/// literal stretch `-n, bytes...` (at most 127). A zero byte ends the output,
/// and the result is the output length.
pub fn rle_encode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x1;
        frame.save(c, S3, 0x40);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
        frame.save(c, S5, 0x20);
        c.g[S3][0] = c.g[A1][0]; // in
        frame.save(c, S4, 0x30);
        c.g[S5][0] = 0x2; // min run
        frame.save(c, S6, 0x10);
        c.g[S4][0] = sx(sphere_addr(c.g[S3][0], c.g[A2][0] as u32)); // end = in + len
        frame.save(c, S1, 0x60);
        c.g[S6][0] = c.g[A0][0]; // out (start)
        if c.g[V0][0] != 0 {
            c.g[S5][0] = c.g[A3][0];
        }
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[S6][0]; // out (cursor)
        frame.save_ra(c, 0);

        'outer: loop {
            // while (in < end)
            c.cycles += 3;
            c.g[V0][0] = (c.g[S3][0] < c.g[S4][0]) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = c.g[S3][0];
            if t {
                break 'outer;
            }
            // A run starting at `in`, or a literal stretch: S0 = run, S2 = literal length.
            'run_or_lit: {
                c.cycles += 3;
                c.g[S0][0] = 0;
                let t = c.g[V0][0] == 0u64;
                c.g[S2][0] = 0;
                if t {
                    break 'run_or_lit;
                }
                c.cycles += 1;
                c.g[A0][0] = c.lbu(sphere_addr(c.g[A1][0], 0));
                'scan: loop {
                    // Measure the run at the cursor.
                    'run_len: {
                        c.cycles += 4;
                        c.g[S0][0] = 0x1;
                        c.g[V0][0] = c.lbu(sphere_addr(c.g[A1][0], 0x1));
                        let t = c.g[A0][0] != c.g[V0][0];
                        c.g[V1][0] = sx(sphere_addr(c.g[A1][0], 0x1));
                        if t {
                            break 'run_len;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[V1][0] < c.g[S4][0]) as u64;
                        let t = c.g[V0][0] == 0u64;
                        if t {
                            break 'run_len;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = sx(sphere_addr(c.g[V1][0], 0x1));
                        'extend: loop {
                            c.cycles += 3;
                            c.g[V0][0] = c.lbu(sphere_addr(c.g[V1][0], 0));
                            let t = c.g[A0][0] != c.g[V0][0];
                            c.g[S0][0] = sx(sphere_addr(c.g[S0][0], 0x1));
                            if t {
                                break 'run_len;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V1][0] < c.g[S4][0]) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = ((c.g[S0][0] as i64) < 127i64) as u64;
                            if t {
                                break 'run_len;
                            }
                            c.cycles += 2;
                            if c.g[V0][0] != 0u64 {
                                c.g[V1][0] = sx(sphere_addr(c.g[V1][0], 0x1));
                                if event_due(c) {
                                    deliver_event_here(c, 0x0032_b4e8);
                                }
                                continue 'extend;
                            } else {
                                break 'run_len;
                            }
                        }
                    }
                    // Clamp the run to the input, and drop a run of one byte at the end.
                    'run_clip: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[S4][0];
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                        if t {
                            break 'run_clip;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                        if t {
                            break 'run_clip;
                        }
                        c.cycles += 3;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    }
                    // A run that is long enough ends the literal stretch.
                    'run_end: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        if t {
                            break 'run_end;
                        }
                        c.cycles += 3;
                        c.g[S2][0] = sx(sphere_addr(c.g[S2][0], c.g[S0][0] as u32));
                        c.g[A1][0] = c.g[V1][0];
                        c.g[S0][0] = 0u64;
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 127i64) as u64;
                    if t {
                        break 'run_or_lit;
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = (c.g[A1][0] < c.g[S4][0]) as u64;
                    if t {
                        break 'run_or_lit;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lbu(sphere_addr(c.g[A1][0], 0));
                        if event_due(c) {
                            deliver_event_here(c, 0x0032_b4c8);
                        }
                        continue 'scan;
                    } else {
                        break 'run_or_lit;
                    }
                }
            }
            // Emit: a literal stretch (-n, bytes) when S2 is set, then a run (count, byte).
            'emit: {
                c.cycles += 5;
                c.g[V1][0] = ((c.g[S2][0] as i64) < 128i64) as u64;
                c.g[V0][0] = 0x7f;
                if c.g[V1][0] == 0 {
                    c.g[S2][0] = c.g[V0][0];
                }
                let t = c.g[S2][0] == 0u64;
                c.g[A0][0] = c.g[S1][0];
                if t {
                    break 'emit;
                }
                c.cycles += 8;
                c.g[A1][0] = sx(0u32.wrapping_sub(c.g[S2][0] as u32));
                c.g[A2][0] = 0x1;
                c.g[S1][0] = sx(sphere_addr(c.g[S1][0], 0x1));
                call_ptr(c, 0x003e_6448, 0x0032_b578)?;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                let t = c.g[S2][0] == c.g[V0][0];
                if t {
                    break 'emit;
                }
                c.cycles += 2;
                c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                'copy: loop {
                    c.cycles += 7;
                    c.g[V0][0] = c.lbu(sphere_addr(c.g[S3][0], 0));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[S3][0] = sx(sphere_addr(c.g[S3][0], 0x1));
                    c.sb(sphere_addr(c.g[S1][0], 0), c.g[V0][0]);
                    let t = c.g[S2][0] != c.g[V1][0];
                    c.g[S1][0] = sx(sphere_addr(c.g[S1][0], 0x1));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0032_b590);
                        }
                        continue 'copy;
                    } else {
                        break 'emit;
                    }
                }
            }
            c.cycles += 2;
            let t = c.g[S0][0] == 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0032_b4ac);
                }
                continue 'outer;
            }
            c.cycles += 11;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x1;
            call_ptr(c, 0x003e_6448, 0x0032_b5c4)?;
            c.g[S1][0] = sx(sphere_addr(c.g[S1][0], 0x1));
            c.g[V0][0] = c.lbu(sphere_addr(c.g[S3][0], 0));
            c.g[S3][0] = sx(sphere_addr(c.g[S3][0], 0x1));
            c.sb(sphere_addr(c.g[S1][0], 0), c.g[V0][0]);
            c.g[S3][0] = sx(sphere_addr(c.g[S3][0], c.g[S0][0] as u32));
            c.g[S1][0] = sx(sphere_addr(c.g[S1][0], 0x1));
            if event_due(c) {
                deliver_event_here(c, 0x0032_b4ac);
            }
            continue 'outer;
        }
        // The terminating zero byte, then out - start.
        c.cycles += 16;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x1;
        c.g[S1][0] = sx(sphere_addr(c.g[S1][0], 0x1));
        call_ptr(c, 0x003e_6448, 0x0032_b5f4)?;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[S6][0] as u32));
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SphereTree_SetTransform(self, mat, scale)` (0x32c648, dev-build
/// `TransformAbs`): the four axes are the align axes (`gSphTreeAlignAxis`,
/// 0x4ff640) rotated by `mat` and scaled, their negations fill axes 4..7; the
/// centre is the rotated root centre times `scale`, and `self->scale = scale`.
pub fn sphere_tree_set_transform(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x0050_0000;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 0); // f0 = -1.0
    let frame = Frame::open(c, 0x40);
    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x9c0));
    c.g[A3][0] = sx(sphere_addr(c.g[A0][0], 0x70));
    c.g[A2][0] = c.g[A0][0];
    c.g[T1][0] = 0x3;
    loop {
        c.cycles += 33;
        c.mfc1(2, 12);
        c.lqc2(4, c.g[T0][0] as u32);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sphere_addr(c.g[SP][0], 0x20));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
        c.lq(2, sphere_addr(c.g[SP][0], 0x20));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
        c.sq(2, sphere_addr(c.g[SP][0], 0x10));
        c.lqc2(8, sphere_addr(c.g[SP][0], 0x10));
        c.lqc2(4, sphere_addr(c.g[A1][0], 0));
        c.lqc2(5, sphere_addr(c.g[A1][0], 0x10));
        c.lqc2(6, sphere_addr(c.g[A1][0], 0x20));
        c.lqc2(7, sphere_addr(c.g[A1][0], 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, sphere_addr(c.g[SP][0], 0x20));
        c.lq(2, sphere_addr(c.g[SP][0], 0x20));
        c.sq(2, sphere_addr(c.g[A2][0], 0));
        c.sq(2, sphere_addr(c.g[SP][0], 0));
        c.mfc1(3, 0);
        c.lqc2(4, sphere_addr(c.g[A2][0], 0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sphere_addr(c.g[SP][0], 0x10));
        c.lq(2, sphere_addr(c.g[SP][0], 0x10));
        c.g[A2][0] = sx(sphere_addr(c.g[A2][0], 0x10));
        c.sq(2, sphere_addr(c.g[A3][0], 0));
        c.sq(2, sphere_addr(c.g[SP][0], 0));
        let t = (c.g[T1][0] as i64) >= 0;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x10));
        if !t {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_c668);
        }
    }
    c.cycles += 32;
    c.g[V0][0] = c.lw(sphere_addr(c.g[A0][0], 0x98));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3); // f3 = 1.0
    c.lwc1(2, sphere_addr(c.g[V0][0], 0x1c));
    c.lwc1(0, sphere_addr(c.g[V0][0], 0x14));
    c.lwc1(1, sphere_addr(c.g[V0][0], 0x18));
    c.mul_s(2, 2, 12);
    c.mul_s(0, 0, 12);
    c.swc1(3, sphere_addr(c.g[SP][0], 0x1c));
    c.mul_s(1, 1, 12);
    c.swc1(2, sphere_addr(c.g[SP][0], 0x18));
    c.swc1(0, sphere_addr(c.g[SP][0], 0x10));
    c.swc1(1, sphere_addr(c.g[SP][0], 0x14));
    c.swc1(0, sphere_addr(c.g[SP][0], 0x20));
    c.swc1(1, sphere_addr(c.g[SP][0], 0x24));
    c.swc1(2, sphere_addr(c.g[SP][0], 0x28));
    c.lqc2(8, sphere_addr(c.g[SP][0], 0x10));
    c.lqc2(4, sphere_addr(c.g[A1][0], 0));
    c.lqc2(5, sphere_addr(c.g[A1][0], 0x10));
    c.lqc2(6, sphere_addr(c.g[A1][0], 0x20));
    c.lqc2(7, sphere_addr(c.g[A1][0], 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, sphere_addr(c.g[SP][0], 0x30));
    c.lq(2, sphere_addr(c.g[SP][0], 0x30));
    c.swc1(12, sphere_addr(c.g[A0][0], 0x90));
    c.sq(2, sphere_addr(c.g[A0][0], 0x80));
    c.sq(2, sphere_addr(c.g[SP][0], 0));
    frame.close(c);
}

/// `void SphereTree_ApplyTransform(self, mat, scale)` (0x32c770, dev-build
/// `TransformRel`): the axes and the centre are transformed again by `mat`
/// and scaled by `scale` (f12) on top of the current ones; `self->scale`
/// grows by the same factor. The axes' negations are rebuilt as in
/// [`sphere_tree_set_transform`].
pub fn sphere_tree_apply_transform(c: &mut Cpu) {
    c.cycles += 6;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 0); // f0 = -1.0
    let frame = Frame::open(c, 0x30);
    c.g[A3][0] = sx(sphere_addr(c.g[A0][0], 0x70));
    c.g[A2][0] = c.g[A0][0];
    c.g[T0][0] = 0x3;
    loop {
        c.cycles += 32;
        c.lqc2(8, c.g[A2][0] as u32);
        c.lqc2(4, c.g[A1][0] as u32);
        c.lqc2(5, sphere_addr(c.g[A1][0], 0x10));
        c.lqc2(6, sphere_addr(c.g[A1][0], 0x20));
        c.lqc2(7, sphere_addr(c.g[A1][0], 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, sphere_addr(c.g[SP][0], 0x20));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
        c.lq(2, sphere_addr(c.g[SP][0], 0x20));
        c.sq(2, sphere_addr(c.g[SP][0], 0x10));
        c.mfc1(3, 12);
        c.lqc2(4, sphere_addr(c.g[SP][0], 0x10));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sphere_addr(c.g[SP][0], 0x20));
        c.lq(2, sphere_addr(c.g[SP][0], 0x20));
        c.sq(2, sphere_addr(c.g[A2][0], 0));
        c.sq(2, sphere_addr(c.g[SP][0], 0));
        c.mfc1(3, 0);
        c.lqc2(4, sphere_addr(c.g[A2][0], 0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sphere_addr(c.g[SP][0], 0x10));
        c.lq(2, sphere_addr(c.g[SP][0], 0x10));
        c.g[A2][0] = sx(sphere_addr(c.g[A2][0], 0x10));
        c.sq(2, sphere_addr(c.g[A3][0], 0));
        c.sq(2, sphere_addr(c.g[SP][0], 0));
        let t = (c.g[T0][0] as i64) >= 0;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x10));
        if !t {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_c788);
        }
    }
    c.cycles += 36;
    c.lqc2(8, sphere_addr(c.g[A0][0], 0x80));
    c.lqc2(4, c.g[A1][0] as u32);
    c.lqc2(5, sphere_addr(c.g[A1][0], 0x10));
    c.lqc2(6, sphere_addr(c.g[A1][0], 0x20));
    c.lqc2(7, sphere_addr(c.g[A1][0], 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, sphere_addr(c.g[SP][0], 0x20));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1); // f1 = 1.0
    c.lq(2, sphere_addr(c.g[SP][0], 0x20));
    c.lwc1(3, sphere_addr(c.g[A0][0], 0x90));
    c.sq(2, sphere_addr(c.g[SP][0], 0x10));
    c.swc1(1, sphere_addr(c.g[SP][0], 0x2c));
    c.mul_s(3, 3, 12);
    c.lwc1(0, sphere_addr(c.g[SP][0], 0x10));
    c.lwc1(1, sphere_addr(c.g[SP][0], 0x14));
    c.lwc1(2, sphere_addr(c.g[SP][0], 0x18));
    c.mul_s(0, 0, 12);
    c.mul_s(1, 1, 12);
    c.lwc1(4, sphere_addr(c.g[SP][0], 0x1c));
    c.mul_s(2, 2, 12);
    c.swc1(3, sphere_addr(c.g[A0][0], 0x90));
    c.swc1(0, sphere_addr(c.g[SP][0], 0));
    c.swc1(1, sphere_addr(c.g[SP][0], 0x4));
    c.swc1(2, sphere_addr(c.g[SP][0], 0x8));
    c.swc1(4, sphere_addr(c.g[SP][0], 0xc));
    c.swc1(12, sphere_addr(c.g[SP][0], 0x20));
    c.lq(2, sphere_addr(c.g[SP][0], 0));
    c.swc1(12, sphere_addr(c.g[SP][0], 0x24));
    c.sq(2, sphere_addr(c.g[A0][0], 0x80));
    c.swc1(12, sphere_addr(c.g[SP][0], 0x28));
    frame.close(c);
}

/// `int SphereTree_CollidePlane(self, plane, outPush, outContact)` (0x32c9b8):
/// builds the tree's buffer when it is compressed, stores the plane's
/// zero-w copy in the frame for the tree's sort axes, resets `worstCase`
/// and walks the tree with `SphereTree_checkPlaneCollision`. Returns 1 when
/// the plane is hit (`worstCase < 0`), else 0.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn sphere_tree_collide_plane(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = c.lw(sphere_addr(c.g[S0][0], 0x98)); // treeInfo
        c.g[V0][0] = c.lw(sphere_addr(c.g[A0][0], 0x8)); // isCompressed
        c.g[S3][0] = c.g[A3][0];
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, 0x0032_df28, 0x0032_c9f4)?; // SphereTree_AllocBuf(treeInfo)
            c.g[V1][0] = c.lw(sphere_addr(c.g[S0][0], 0x98));
            c.sw(sphere_addr(c.g[V1][0], 0x28), c.g[V0][0]); // bitTree
        }
        c.cycles += 23;
        // The plane's copy with w cleared, as the sort axes' argument.
        c.lq(2, sphere_addr(c.g[S1][0], 0));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        c.sq(2, sphere_addr(c.g[SP][0], 0x10));
        c.sw(sphere_addr(c.g[SP][0], 0x1c), 0);
        c.lq(2, sphere_addr(c.g[SP][0], 0x10));
        c.sq(2, sphere_addr(c.g[SP][0], 0));
        call_ptr(c, 0x0032_db40, 0x0032_ca1c)?; // SphereTree_populateSortAxis_v
        c.sw(sphere_addr(c.g[S0][0], 0x94), 0); // worstCase = 0
        c.g[A1][0] = c.g[S1][0];
        c.g[T1][0] = c.g[S2][0];
        c.g[T2][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx(sphere_addr(c.g[S0][0], 0x80));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x0032_d208, 0x0032_ca40)?; // SphereTree_checkPlaneCollision
        c.lwc1(1, sphere_addr(c.g[S0][0], 0x94));
        c.mtc1(0, 0);
        c.c_lt_s(1, 0);
        let t = c.fcc();
        c.g[V0][0] = 0x1;
        if !t {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `VU0_TriangleIntersect` (0x32b6a8), reached through the function table.
pub(crate) const TRI_INTERSECT_ADDR: u32 = 0x0032_b6a8;

/// `EA_MemFree` (0x317e98), through the function table.
pub(crate) const EA_FREE_ADDR: u32 = 0x0031_7e98;

/// `CollSphere_PlaneDistance` (0x330360), a direct call.
pub(crate) const COLL_PLANE_DIST_CALLEE: Callee = Callee {
    addr: 0x0033_0360,
    f: ee::s11::f_00330360,
};

/// `CollSphere_HitsTriangleEdges` (0x330250), a direct call.
pub(crate) const COLL_HITS_EDGES_CALLEE: Callee = Callee {
    addr: 0x0033_0250,
    f: ee::s11::f_00330250,
};

/// The sphere-tree node itself (0x32cdb0): the recursive child call.
pub(crate) const SPHERE_NODE_CALLEE: Callee = Callee {
    addr: 0x0032_cdb0,
    f: ee::s10::f_0032cdb0,
};

/// `gp + off` for the game's gp-relative globals (`off` may be negative).
pub(crate) fn gp_word_addr(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `int SphereTree_PointInTriangle(a0, a1, a2, a3, t0)` (0x32d440): a tail
/// wrapper around `VU0_TriangleIntersect`, the tolerance (`gp-0x2d80`) passed
/// in f12. The callee's result is returned unchanged.
pub fn sphere_tree_point_in_triangle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[A0][0] = c.g[A1][0];
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A2][0];
        let tol = gp_word_addr(c, -0x2d80);
        c.lwc1(12, tol);
        c.g[A2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[T0][0];
        call_ptr(c, TRI_INTERSECT_ADDR, 0x0032_d460)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void World_FreeSphereTreeCache(void)` (0x32de20): frees the cached
/// sphere-tree node arrays and single entries through `EA_MemFree`, clearing
/// each pointer. The first array is at `gp+0xed8` (count `gp+0xee0`), the
/// single at `gp+0xedc`, the second array at `gp+0xee8` (count `gp+0xef0`),
/// and the last single at `gp+0xeec`.
pub fn world_free_sphere_tree_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        let a = gp_word_addr(c, 0xed8);
        c.g[A0][0] = c.lw(a);
        frame.save(c, S0, 0x10);
        let list_empty = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !list_empty {
            // First array: elements 0 .. count-1 at gp+0xed8.
            c.cycles += 3;
            let a = gp_word_addr(c, 0xee0);
            c.g[V0][0] = c.lw(a);
            let counted = (c.g[V0][0] as i64) > 0;
            c.g[S0][0] = 0;
            if counted {
                loop {
                    c.cycles += 6;
                    let a = gp_word_addr(c, 0xed8);
                    c.g[V1][0] = c.lw(a);
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let slot = c.g[V0][0] as u32;
                    c.g[A0][0] = c.lw(slot);
                    let empty = c.g[A0][0] == 0;
                    let a = gp_word_addr(c, 0xee0);
                    c.g[V0][0] = c.lw(a);
                    if !empty {
                        c.cycles += 3;
                        call_ptr(c, EA_FREE_ADDR, 0x0032_de60)?;
                        let a = gp_word_addr(c, 0xee0);
                        c.g[V0][0] = c.lw(a);
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    let a = gp_word_addr(c, 0xed8);
                    c.g[A0][0] = c.lw(a);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_de40);
                    }
                }
            }
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                let a = gp_word_addr(c, 0xed8);
                c.sw(a, 0u64);
            } else {
                c.cycles += 3;
                call_ptr(c, EA_FREE_ADDR, 0x0032_de84)?;
                let a = gp_word_addr(c, 0xed8);
                c.sw(a, 0u64);
            }
        }
        // The single at gp+0xedc, then the second array's head at gp+0xee8.
        c.cycles += 3;
        let a = gp_word_addr(c, 0xedc);
        c.g[A0][0] = c.lw(a);
        if c.g[A0][0] == 0 {
            let a = gp_word_addr(c, 0xee8);
            c.g[A0][0] = c.lw(a);
        } else {
            c.cycles += 4;
            call_ptr(c, EA_FREE_ADDR, 0x0032_de9c)?;
            let a = gp_word_addr(c, 0xedc);
            c.sw(a, 0u64);
            let a = gp_word_addr(c, 0xee8);
            c.g[A0][0] = c.lw(a);
        }
        // Second array: elements at gp+0xee8 (count gp+0xef0).
        c.cycles += 2;
        let second_empty = c.g[A0][0] == 0;
        let a = gp_word_addr(c, 0xef0);
        c.g[V0][0] = c.lw(a);
        if !second_empty {
            c.cycles += 2;
            let counted = (c.g[V0][0] as i64) > 0;
            c.g[S0][0] = 0;
            if counted {
                c.cycles += 1;
                loop {
                    c.cycles += 6;
                    let a = gp_word_addr(c, 0xee8);
                    c.g[V1][0] = c.lw(a);
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let slot = c.g[V0][0] as u32;
                    c.g[A0][0] = c.lw(slot);
                    let empty = c.g[A0][0] == 0;
                    let a = gp_word_addr(c, 0xef0);
                    c.g[V0][0] = c.lw(a);
                    if !empty {
                        c.cycles += 3;
                        call_ptr(c, EA_FREE_ADDR, 0x0032_ded8)?;
                        let a = gp_word_addr(c, 0xef0);
                        c.g[V0][0] = c.lw(a);
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    let a = gp_word_addr(c, 0xee8);
                    c.g[A0][0] = c.lw(a);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_deb8);
                    }
                }
            }
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                let a = gp_word_addr(c, 0xee8);
                c.sw(a, 0u64);
            } else {
                c.cycles += 3;
                call_ptr(c, EA_FREE_ADDR, 0x0032_defc)?;
                let a = gp_word_addr(c, 0xee8);
                c.sw(a, 0u64);
            }
        }
        // The single at gp+0xeec, then the epilogue.
        c.cycles += 3;
        let a = gp_word_addr(c, 0xeec);
        c.g[A0][0] = c.lw(a);
        let single_empty = c.g[A0][0] == 0;
        frame.restore(c, S0, 0x10);
        if !single_empty {
            c.cycles += 4;
            call_ptr(c, EA_FREE_ADDR, 0x0032_df14)?;
            let a = gp_word_addr(c, 0xeec);
            c.sw(a, 0u64);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The one-sided test of a sphere against a triangle that the centre is in
/// front of ('b_32ff50' of 0x32fe78): the centre projected into the plane
/// (`VU0_TriangleIntersect`, tolerance `gp-0x2d70`), then the triangle edges
/// (`CollSphere_HitsTriangleEdges`) when it is outside. True on a hit. On a
/// miss it leaves v0 = 0 and returns false.
pub(crate) fn collide_triangle_front_one_sided(
    c: &mut Cpu,
    frame: &Frame,
) -> Result<bool, Unwound> {
    c.cycles += 22;
    c.g[A3][0] = c.g[S3][0];
    c.mfc1(2, 0);
    let s0 = c.g[S0][0] as u32;
    c.lqc2(4, s0);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let t20 = frame.at(c, 0x20);
    c.sqc2(5, t20);
    let tol = gp_word_addr(c, -0x2d70);
    c.lwc1(12, tol);
    c.lq(2, t20);
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S4][0];
    c.g[A2][0] = c.g[S5][0];
    let t10 = frame.at(c, 0x10);
    c.sq(2, t10);
    let s1_60 = (c.g[S1][0] as u32).wrapping_add(0x60);
    c.lqc2(3, s1_60);
    c.lqc2(4, t10);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, t20);
    c.lq(2, t20);
    let s3 = c.g[S3][0] as u32;
    c.sq(2, s3);
    let sp0 = frame.at(c, 0);
    c.sq(2, sp0);
    call_ptr(c, TRI_INTERSECT_ADDR, 0x0032_ff28)?;
    if c.g[V0][0] != 0 {
        let s0 = c.g[S0][0] as u32;
        c.lq(3, s0);
        return Ok(true);
    }
    c.cycles += 7;
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S4][0];
    c.g[A3][0] = c.g[S5][0];
    c.g[A0][0] = c.g[S1][0];
    COLL_HITS_EDGES_CALLEE.call(c, 0x0032_ff44)?;
    let missed = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if missed {
        return Ok(false);
    }
    c.cycles += 1;
    let s0 = c.g[S0][0] as u32;
    c.lq(3, s0);
    Ok(true)
}

/// `int CollSphere_CollideTriangle(self, a1, a2, a3, t0, t1, t2, t3)`
/// (0x32fe78): one-sided sphere-triangle collision. Tests the sphere centre
/// against the triangle's plane (`CollSphere_PlaneDistance`, distance at
/// sp+0x30); on the front side it tests the centre inside the triangle and
/// then the edges. A hit writes the penetration-scaled contact normal to `t2`
/// and the corrected centre to `t3`, and returns 1.
pub fn coll_sphere_collide_triangle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S0][0] = c.g[T0][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[S4][0] = c.g[A2][0];
        c.g[S5][0] = c.g[A3][0];
        c.g[S3][0] = c.g[T1][0];
        c.g[S6][0] = c.g[T2][0];
        c.g[S7][0] = c.g[T3][0];

        // Plane distance of the centre (written to sp+0x30).
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S2][0];
        let dist = frame.at(c, 0x30);
        c.g[A3][0] = sx(dist);
        COLL_PLANE_DIST_CALLEE.call(c, 0x0032_fed0)?;
        let dist = frame.at(c, 0x30);
        let plane_front = c.g[V0][0] != 0;
        c.lwc1(0, dist);
        if !plane_front {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else if collide_triangle_front_one_sided(c, &frame)? {
            c.cycles += 7;
            c.g[V0][0] = 1;
            let dist = frame.at(c, 0x30);
            c.lwc1(1, dist);
            let s6 = c.g[S6][0] as u32;
            c.sq(3, s6);
            let s1_70 = (c.g[S1][0] as u32).wrapping_add(0x70);
            c.lwc1(0, s1_70);
            c.sub_s(0, 0, 1);
            let s7 = c.g[S7][0] as u32;
            c.swc1(0, s7);
        }

        // Epilogue.
        c.cycles += 11;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The two-sided test of a sphere against a triangle ('b_3300b0' of
/// 0x32ffa0): the centre projected into the plane (`VU0_TriangleIntersect`,
/// tolerance `gp-0x2d6c`), then the triangle edges when it is outside. True on
/// a hit; on a miss the caller sets v0 = 0 and charges one cycle.
pub(crate) fn collide_triangle_front_two_sided(
    c: &mut Cpu,
    frame: &Frame,
) -> Result<bool, Unwound> {
    c.cycles += 22;
    c.g[A3][0] = c.g[S2][0];
    c.mfc1(2, 0);
    let s = frame.at(c, 0);
    c.lqc2(4, s);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let t30 = frame.at(c, 0x30);
    c.sqc2(5, t30);
    let tol = gp_word_addr(c, -0x2d6c);
    c.lwc1(12, tol);
    c.lq(2, t30);
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S4][0];
    let t20 = frame.at(c, 0x20);
    c.sq(2, t20);
    let s0_60 = (c.g[S0][0] as u32).wrapping_add(0x60);
    c.lqc2(3, s0_60);
    c.lqc2(4, t20);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, t30);
    c.lq(2, t30);
    let s2 = c.g[S2][0] as u32;
    c.sq(2, s2);
    let t10 = frame.at(c, 0x10);
    c.sq(2, t10);
    call_ptr(c, TRI_INTERSECT_ADDR, 0x0033_008c)?;
    let inside = c.g[V0][0] != 0;
    let s = frame.at(c, 0);
    c.lq(3, s);
    if inside {
        return Ok(true);
    }
    c.cycles += 7;
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[A3][0] = c.g[S4][0];
    c.g[A0][0] = c.g[S0][0];
    COLL_HITS_EDGES_CALLEE.call(c, 0x0033_00a8)?;
    let missed = c.g[V0][0] == 0;
    let s = frame.at(c, 0);
    c.lq(3, s);
    Ok(!missed)
}

/// `int CollSphere_CollideTriangle2Sided(self, a1, a2, a3, t0, t1, t2, t3)`
/// (0x32ffa0): as the one-sided test, but the plane's normal is flipped when
/// the centre is behind it, so both faces collide. A hit writes the contact
/// normal to `t2` and the corrected centre to `t3`, and returns 1.
pub fn coll_sphere_collide_triangle_2sided(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0xd0);
        let t0 = c.g[T0][0] as u32;
        c.lq(2, t0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save_ra(c, 0x50);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A2][0];
        c.g[S4][0] = c.g[A3][0];
        c.g[S2][0] = c.g[T1][0];
        c.g[S5][0] = c.g[T2][0];
        c.g[S6][0] = c.g[T3][0];
        let s = frame.at(c, 0);
        c.sq(2, s);

        // Plane distance of the centre (written to sp+0x40).
        c.g[A1][0] = c.g[T0][0];
        c.g[A2][0] = c.g[S1][0];
        let dist = frame.at(c, 0x40);
        c.g[A3][0] = sx(dist);
        COLL_PLANE_DIST_CALLEE.call(c, 0x0032_fff8)?;
        let plane_front = c.g[V0][0] != 0;
        let dist = frame.at(c, 0x40);
        c.lwc1(1, dist);
        let mut hit = false;
        if plane_front {
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_lt_s(1, 0);
            let below = c.fcc();
            let dist = frame.at(c, 0x40);
            c.lwc1(0, dist);
            if below {
                // Behind the plane: flip the normal (and the distance) to -1.
                c.cycles += 10;
                c.neg_s(0, 1);
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.mtc1(1, 1);
                c.swc1(0, dist);
                c.mfc1(2, 1);
                let s = frame.at(c, 0);
                c.lqc2(4, s);
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, s);
                c.lwc1(0, dist);
            }
            hit = collide_triangle_front_two_sided(c, &frame)?;
        }
        if hit {
            c.cycles += 7;
            c.g[V0][0] = 1;
            let dist = frame.at(c, 0x40);
            c.lwc1(1, dist);
            let s5 = c.g[S5][0] as u32;
            c.sq(3, s5);
            let s0_70 = (c.g[S0][0] as u32).wrapping_add(0x70);
            c.lwc1(0, s0_70);
            c.sub_s(0, 0, 1);
            let s6 = c.g[S6][0] as u32;
            c.swc1(0, s6);
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }

        // Epilogue.
        c.cycles += 10;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `Vector ClosestPointOnSegment(out, a, b, p)` (0x330128): the point on the
/// segment `a`..`b` nearest `p`, written to `out`, and `out` returned. The
/// projection parameter is clamped to the ends: `a` when the parameter is
/// <= 0 and `b` when the far end is <= the parameter.
pub fn closest_point_on_segment(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x60);
        c.mtc1(0, 0);
        // p - a (sp+0x10 and sp+0), then b - a (sp+0x10 and sp+0x20).
        let p = c.g[A3][0] as u32;
        c.lqc2(3, p);
        let a = c.g[A1][0] as u32;
        c.lqc2(4, a);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        let t10 = frame.at(c, 0x10);
        c.sqc2(5, t10);
        c.lq(2, t10);
        let s0 = frame.at(c, 0);
        c.sq(2, s0);
        let b = c.g[A2][0] as u32;
        c.lqc2(3, b);
        let a = c.g[A1][0] as u32;
        c.lqc2(4, a);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        let t20 = frame.at(c, 0x20);
        c.sqc2(5, t20);
        c.lq(2, t20);
        c.sq(2, t10);
        // The unit direction (b - a) / |b - a| (sp+0x20).
        c.lqc2(3, t10);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        let t30 = frame.at(c, 0x30);
        c.sqc2(5, t30);
        c.lq(2, t30);
        c.sq(2, t20);
        // t = unit direction . (p - a) (f1).
        c.lqc2(3, t20);
        c.lqc2(5, s0);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 1);
        // |b - a| (f2) against the parameter.
        c.lqc2(3, t10);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 2);
        c.c_le_s(1, 0);
        if c.fcc() {
            // t <= 0: the result is a.
            c.cycles += 3;
            let a = c.g[A1][0] as u32;
            c.lq(2, a);
            let out = c.g[A0][0] as u32;
            c.sq(2, out);
        } else {
            c.cycles += 3;
            c.c_le_s(2, 1);
            if c.fcc() {
                // |b - a| <= t: the result is b.
                c.cycles += 3;
                let b = c.g[A2][0] as u32;
                c.lq(2, b);
                let out = c.g[A0][0] as u32;
                c.sq(2, out);
            } else {
                // a + t * unit direction.
                c.cycles += 14;
                c.mfc1(2, 1);
                c.lqc2(4, t20);
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                let t50 = frame.at(c, 0x50);
                c.sqc2(5, t50);
                c.lq(3, t50);
                let t40 = frame.at(c, 0x40);
                c.sq(3, t40);
                let a = c.g[A1][0] as u32;
                c.lqc2(3, a);
                c.lqc2(4, t40);
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, t50);
                c.lq(2, t50);
                let out = c.g[A0][0] as u32;
                c.sq(2, out);
                let t30 = frame.at(c, 0x30);
                c.sq(2, t30);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A0][0];
        frame.close(c);
        Ok(())
    })
}

/// `int SphereTree_CollideSphereNode(self, spherePos, radius, pos, depth,
/// base, outA, outB)` (0x32cdb0): the recursive sphere-against-tree test.
/// Returns 0 when the sphere misses this node's bound; at a leaf (or a node
/// with no children) it writes the contact normal to `outA` and the corrected
/// centre to `outB` and returns 1; otherwise it recurses into the children
/// whose bit is set in the node mask.
pub fn sphere_tree_collide_sphere_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0x110);
        c.g[V0][0] = 0xc;
        frame.save(c, S1, 0xe0);
        frame.save(c, S3, 0xc0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S4, 0xb0);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S5, 0xa0);
        // A3 = depth * 12 (the level stride).
        c.mult(0, A3, S3, V0);
        frame.save(c, S6, 0x90);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S7, 0x80);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, FP, 0x70);
        c.g[S4][0] = c.g[T0][0];
        frame.save_fpr(c, 21, 0x108);
        c.g[S7][0] = c.g[T1][0];
        frame.save(c, S0, 0xf0);
        c.mov_s(21, 12);
        frame.save(c, S2, 0xd0);
        c.g[FP][0] = c.g[T2][0];
        frame.save_ra(c, 0x60);
        frame.save_fpr(c, 20, 0x100);

        // Bound test: the centre distance against radius + the node radius.
        let info = (c.g[S1][0] as u32).wrapping_add(0x98);
        c.g[V1][0] = c.lw(info);
        let scale = (c.g[S1][0] as u32).wrapping_add(0x90);
        c.lwc1(1, scale);
        let levels_ptr = (c.g[V1][0] as u32).wrapping_add(0x20);
        c.g[V0][0] = c.lw(levels_ptr);
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let level = c.g[V0][0] as u32;
        c.lwc1(0, level);
        let s6 = c.g[S6][0] as u32;
        c.lqc2(3, s6);
        let s5 = c.g[S5][0] as u32;
        c.lqc2(4, s5);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        let t10 = frame.at(c, 0x10);
        c.sqc2(5, t10);
        c.mul_s(0, 0, 1);
        c.lq(2, t10);
        let s0 = frame.at(c, 0);
        c.sq(2, s0);
        c.add_s(1, 21, 0);
        c.lqc2(3, s0);
        c.lqc2(5, t10);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 2);
        c.mul_s(0, 1, 1);
        c.c_lt_s(2, 0);
        let near = c.fcc();
        c.g[V0][0] = 0;
        if near {
            let leaf = sphere_node_walk(c, &frame)?;
            if leaf {
                sphere_node_leaf(c, &frame);
            }
        }

        // Epilogue.
        c.cycles += 14;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 21, 0x108);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// The child walk of the sphere-tree node ('b_32cf54' and its loop). Returns
/// `Ok(true)` when the node is a leaf (the caller writes the contact); `Ok(false)`
/// when the result is already in v0 (a child hit, or no child hit).
pub(crate) fn sphere_node_walk(c: &mut Cpu, frame: &Frame) -> Result<bool, Unwound> {
    c.cycles += 7;
    let info = (c.g[S1][0] as u32).wrapping_add(0x98);
    c.g[A0][0] = c.lw(info);
    let masks_ptr = (c.g[A0][0] as u32).wrapping_add(0x28);
    c.g[V0][0] = c.lw(masks_ptr);
    let depth_ptr = (c.g[A0][0] as u32).wrapping_add(0xc);
    c.g[V1][0] = c.lw(depth_ptr);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
    c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let below_depth = c.g[V1][0] != 0;
    let mask_addr = c.g[V0][0] as u32;
    c.g[S2][0] = c.lbu(mask_addr);
    if !below_depth {
        return Ok(true);
    }
    c.cycles += 2;
    let no_children = c.g[S2][0] == 0;
    c.g[V1][0] = 0;
    if no_children {
        return Ok(true);
    }
    c.cycles += 5;
    let levels_ptr = (c.g[A0][0] as u32).wrapping_add(0x20);
    c.g[V0][0] = c.lw(levels_ptr);
    let a3_save = frame.at(c, 0x50);
    let depth_stride = c.g[A3][0];
    c.sw(a3_save, depth_stride);
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    // distance = levels[depth + 1].offset (f20).
    let next_level = (c.g[V0][0] as u32).wrapping_add(0x10);
    c.lwc1(20, next_level);
    c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> ((c.g[V1][0] as u32) & 31)) as u32);

    loop {
        // 'b_32cf3c: child i is present when bit i of the mask is set.
        c.cycles += 3;
        c.g[V0][0] &= 0x1;
        let absent = c.g[V0][0] == 0;
        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if !absent {
            c.cycles += 32;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A1][0] = c.g[S6][0];
            c.mfc1(3, 20);
            let axis = c.g[V0][0] as u32;
            c.lqc2(4, axis);
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            let t30 = frame.at(c, 0x30);
            c.sqc2(5, t30);
            c.mov_s(12, 21);
            c.lq(2, t30);
            let child_pos = frame.at(c, 0x10);
            c.g[A2][0] = sx(child_pos);
            c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            c.g[T1][0] = c.g[S7][0];
            let t20 = frame.at(c, 0x20);
            c.sq(2, t20);
            c.g[T2][0] = c.g[FP][0];
            let s5 = c.g[S5][0] as u32;
            c.lqc2(3, s5);
            c.lqc2(4, t20);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, t30);
            c.lq(3, t30);
            let info = (c.g[S1][0] as u32).wrapping_add(0x98);
            c.g[T0][0] = c.lw(info);
            c.sq(3, child_pos);
            let stride_slot = frame.at(c, 0x50);
            c.g[V1][0] = c.lw(stride_slot);
            let levels_ptr = (c.g[T0][0] as u32).wrapping_add(0x20);
            c.g[V0][0] = c.lw(levels_ptr);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let stride_field = (c.g[V0][0] as u32).wrapping_add(0x8);
            c.g[T0][0] = c.lw(stride_field);
            c.mult(0, 2, 16, 8);
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
            SPHERE_NODE_CALLEE.call(c, 0x0032_cf34)?;
            let child_hit = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if child_hit {
                return Ok(false);
            }
        }
        // Next child: v1 = i + 1 while i + 1 < 8.
        c.cycles += 4;
        c.g[V1][0] = c.g[S0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 8) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> ((c.g[V1][0] as u32) & 31)) as u32);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0032_ceb0);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            return Ok(false);
        }
    }
}

/// The leaf of the sphere-tree node ('cycles += 38' of 0x32cdb0): the contact
/// normal (the centre offset, normalised) scaled by the penetration goes to
/// `outA`, and the corrected centre to `outB`. Leaves v0 = 1.
pub(crate) fn sphere_node_leaf(c: &mut Cpu, frame: &Frame) {
    c.cycles += 38;
    let delta = frame.at(c, 0);
    c.lqc2(3, delta);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    let normal = frame.at(c, 0x20);
    c.sqc2(5, normal);
    c.sqrt_s(0, 2);
    c.lq(3, normal);
    // distance = sum - |delta| (f20); v0 = 1.
    c.sub_s(20, 1, 0);
    c.g[V0][0] = 1;
    let n_slot = frame.at(c, 0x10);
    c.sq(3, n_slot);
    c.mfc1(4, 20);
    c.lqc2(4, n_slot);
    c.qmtc2(4, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let scaled = frame.at(c, 0x30);
    c.sqc2(5, scaled);
    // radius - distance (f0): the normal's length left for the centre.
    c.sub_s(0, 21, 20);
    c.lq(3, scaled);
    let out_a = c.g[S7][0] as u32;
    c.sq(3, out_a);
    c.sq(3, normal);
    c.mfc1(4, 0);
    c.lqc2(4, n_slot);
    c.qmtc2(4, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let pushed = frame.at(c, 0x40);
    c.sqc2(5, pushed);
    c.lq(3, pushed);
    c.sq(3, scaled);
    let s6 = c.g[S6][0] as u32;
    c.lqc2(3, s6);
    c.lqc2(4, scaled);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, pushed);
    c.lq(3, pushed);
    let out_b = c.g[FP][0] as u32;
    c.sq(3, out_b);
    c.sq(3, normal);
}

// SphereTree (`self`): the sphere centres of the eight children at +0x00
// (t4Vector each), the scale at +0x90, the tree info pointer at +0x98.
#[cfg(test)]
pub(crate) const SELF_AXIS: u32 = 0x00;

pub(crate) const SELF_SCALE: u32 = 0x90;

pub(crate) const SELF_TREE_INFO: u32 = 0x98;

// SphereTreeInfo: depth at +0x0c, level table at +0x20, bit tree at +0x28.
pub(crate) const TREE_DEPTH: u32 = 0x0c;

pub(crate) const TREE_LEVELS: u32 = 0x20;

pub(crate) const TREE_BIT_TREE: u32 = 0x28;

// Level entries are 12 bytes: radius (f32) at +0, cenOffset (f32) at +4,
// indexOffset (int) at +8. The loaded cenOffset is `levels[depth + 1]`.
pub(crate) const LEVEL_SIZE: u32 = 0x0c;

pub(crate) const LEVEL_INDEX_OFFSET: u32 = 0x08;

pub(crate) const LEVEL_NEXT_CEN_OFFSET: u32 = LEVEL_SIZE + 0x04;

// Stack slots of the frame (0x110 bytes).
pub(crate) const SLOT_NORMAL: u32 = 0x00;

pub(crate) const SLOT_DIST: u32 = 0x50;

pub(crate) const SLOT_DIR: u32 = 0x10;

pub(crate) const SLOT_CHILD_POS: u32 = 0x20;

pub(crate) const SLOT_SCALED: u32 = 0x30;

pub(crate) const SLOT_TEMP: u32 = 0x40;

pub(crate) const SLOT_OUT_POS: u32 = 0x54;

pub(crate) const SLOT_SAVED_IDX: u32 = 0x58;

pub(crate) const SLOT_RA: u32 = 0x60;

pub(crate) const SLOT_FRAME: u32 = 0x110;

// Calls and return addresses (from the generated body).
pub(crate) const GIS_BOX_SPHERE_INT: u32 = 0x0032_9590;

pub(crate) const RET_AFTER_GIS: u32 = 0x0032_d0b4;

pub(crate) const RET_AFTER_CHILD: u32 = 0x0032_d17c;

pub(crate) const LOOP_HEAD: u32 = 0x0032_d0f8;

pub(crate) const SELF_ADDR: u32 = 0x0032_d028;

/// Address `off(reg)`, read from the register's low word.
#[inline(always)]
pub(crate) fn sphere_reg_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// Epilogue: `c.cycles += 13`, restore the callee-saved registers and `ra`
/// and f20, then `addiu sp, sp, 0x110; jr ra`.
pub(crate) fn sphere_finish(c: &mut Cpu, frame: Frame) {
    c.cycles += 13;
    frame.restore(c, S0, 0xf0);
    frame.restore(c, S1, 0xe0);
    frame.restore(c, S2, 0xd0);
    frame.restore(c, S3, 0xc0);
    frame.restore(c, S4, 0xb0);
    frame.restore(c, S5, 0xa0);
    frame.restore(c, S6, 0x90);
    frame.restore(c, S7, 0x80);
    frame.restore(c, FP, 0x70);
    frame.restore_ra(c, SLOT_RA);
    frame.restore_fpr(c, 20, 0x100);
    frame.close(c);
}

/// The part of the body after the bounding-box test passed: either the
/// recursion into the children (returns 1 at the first child that hits, or
/// 0), or the leaf, which writes the normal and the scaled direction. Every
/// exit then goes to the epilogue (`sphere_finish`).
pub(crate) fn sphere_children_or_leaf(c: &mut Cpu, frame: Frame) -> Flow {
    'leaf_test: {
        c.cycles += 7;
        c.g[A0][0] = Addr(sphere_reg_at(c, S1, SELF_TREE_INFO)).lw(c);
        let tree = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(tree.wrapping_add(TREE_BIT_TREE)).lw(c);
        c.g[V1][0] = Addr(tree.wrapping_add(TREE_DEPTH)).lw(c);
        let bits = c.g[V0][0] as u32;
        c.g[V0][0] = sx(bits.wrapping_add(c.g[S4][0] as u32));
        c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let inside_depth = c.g[V1][0] == 0;
        c.g[S2][0] = Addr(c.g[V0][0] as u32).lbu(c);
        if inside_depth {
            break 'leaf_test;
        }
        c.cycles += 2;
        let no_mask = c.g[S2][0] == 0;
        c.g[V1][0] = 0;
        if no_mask {
            break 'leaf_test;
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(tree.wrapping_add(TREE_LEVELS)).lw(c);
        let saved_stride = c.g[S0][0];
        Addr(sp(c).wrapping_add(SLOT_SAVED_IDX)).sw(c, saved_stride);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let level_next = (c.g[V0][0] as u32).wrapping_add(LEVEL_NEXT_CEN_OFFSET);
        c.lwc1(20, level_next);
        c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> ((c.g[V1][0] as u32) & 31)) as u32);
        // The child loop: i = V1 walks the eight bits of the mask.
        'scan: loop {
            'child: {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let bit_clear = c.g[V0][0] == 0;
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                if bit_clear {
                    break 'child;
                }
                c.cycles += 32;
                // childPos = pos + axis[i] * off, with off = levels[depth+1].cenOffset.
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[A0][0] = c.g[S1][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[A1][0] = c.g[FP][0];
                c.mfc1(3, 20);
                let axis = c.g[V0][0] as u32;
                c.lqc2(4, axis);
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                let temp = sp(c).wrapping_add(SLOT_TEMP);
                c.sqc2(5, temp);
                c.g[A2][0] = c.g[S7][0];
                c.lq(2, temp);
                c.g[A3][0] = sx(sp(c).wrapping_add(SLOT_CHILD_POS));
                c.g[T2][0] = Addr(sp(c).wrapping_add(SLOT_OUT_POS)).lw(c);
                c.g[T0][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                let scaled = sp(c).wrapping_add(SLOT_SCALED);
                c.sq(2, scaled);
                c.g[T3][0] = c.g[S6][0];
                let pos = c.g[S5][0] as u32;
                c.lqc2(3, pos);
                c.lqc2(4, scaled);
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, temp);
                c.lq(3, temp);
                c.g[T1][0] = Addr(sphere_reg_at(c, S1, SELF_TREE_INFO)).lw(c);
                c.sq(3, sp(c).wrapping_add(SLOT_CHILD_POS));
                c.g[V1][0] = Addr(sp(c).wrapping_add(SLOT_SAVED_IDX)).lw(c);
                let levels = c.g[T1][0] as u32;
                c.g[V0][0] = Addr(levels.wrapping_add(TREE_LEVELS)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let level = c.g[V0][0] as u32;
                c.g[T1][0] = Addr(level.wrapping_add(LEVEL_INDEX_OFFSET)).lw(c);
                // child base = (i + 1) * indexOffset + base
                c.mult(0, V0, S0, T1);
                c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                let child = Callee {
                    addr: SELF_ADDR,
                    f: ee::s11::f_0032d028__generated,
                };
                child.call(c, RET_AFTER_CHILD)?;
                let child_hit = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if child_hit {
                    return Ok(());
                }
                break 'child;
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[S0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 8) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> ((c.g[V1][0] as u32) & 31)) as u32);
            if more {
                if event_due(c) {
                    deliver_event_here(c, LOOP_HEAD);
                }
                continue 'scan;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            return Ok(());
        }
    }
    // Leaf: *outNormal = normal, *outPos = dir * dist.
    c.cycles += 13;
    frame.restore(c, V1, SLOT_NORMAL);
    c.g[V0][0] = 0x1;
    c.lwc1(0, sp(c).wrapping_add(SLOT_DIST));
    let out_normal = c.g[S6][0] as u32;
    c.sq(3, out_normal);
    c.mfc1(4, 0);
    c.lqc2(4, sp(c).wrapping_add(SLOT_DIR));
    c.qmtc2(4, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let scaled = sp(c).wrapping_add(SLOT_SCALED);
    c.sqc2(5, scaled);
    c.lq(3, scaled);
    c.g[A0][0] = Addr(sp(c).wrapping_add(SLOT_OUT_POS)).lw(c);
    let out_pos = c.g[A0][0] as u32;
    c.sq(3, out_pos);
    c.sq(3, sp(c).wrapping_add(SLOT_CHILD_POS));
    Ok(())
}

/// Body after the prologue: the box test, then the children or the leaf.
pub(crate) fn sphere_collide_body(c: &mut Cpu) -> Flow {
    c.cycles += 37;
    let frame = Frame::open(c, SLOT_FRAME);
    c.g[V0][0] = 0xc;
    frame.save(c, S0, 0xf0);
    frame.save(c, S1, 0xe0);
    frame.save(c, S3, 0xc0);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S4, 0xb0);
    c.g[S3][0] = c.g[T0][0];
    frame.save(c, S5, 0xa0);
    // s0 = depth * 12 (the level stride; v0 = 12).
    c.mult(0, S0, S3, V0);
    frame.save(c, S6, 0x90);
    c.g[S5][0] = c.g[A3][0];
    frame.save(c, S7, 0x80);
    c.g[S4][0] = c.g[T1][0];
    frame.save(c, FP, 0x70);
    c.g[S7][0] = c.g[A2][0];
    frame.save(c, S2, 0xd0);
    c.g[FP][0] = c.g[A1][0];
    frame.save_ra(c, SLOT_RA);
    c.g[S6][0] = c.g[T3][0];
    frame.save_fpr(c, 20, 0x100);
    // gIsBoxSphereInt(boxMin, boxMax, pos, &normal, &dir, &dist, radius * scale)
    c.g[A0][0] = c.g[FP][0];
    c.g[A1][0] = c.g[S7][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[V1][0] = Addr(sphere_reg_at(c, S1, SELF_TREE_INFO)).lw(c);
    c.g[A3][0] = c.g[SP][0];
    c.lwc1(0, sphere_reg_at(c, S1, SELF_SCALE));
    c.g[T0][0] = sx(sp(c).wrapping_add(SLOT_DIR));
    let tree = c.g[V1][0] as u32;
    c.g[V0][0] = Addr(tree.wrapping_add(TREE_LEVELS)).lw(c);
    c.g[T1][0] = sx(sp(c).wrapping_add(SLOT_DIST));
    let out_pos_slot = sp(c).wrapping_add(SLOT_OUT_POS);
    let out_pos_value = c.g[T2][0];
    Addr(out_pos_slot).sw(c, out_pos_value);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let radius_at = c.g[V0][0] as u32;
    c.lwc1(12, radius_at);
    c.mul_s(12, 12, 0);
    call_ptr(c, GIS_BOX_SPHERE_INT, RET_AFTER_GIS)?;
    let missed = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if missed {
        sphere_finish(c, frame);
        return Ok(());
    }
    sphere_children_or_leaf(c, frame)?;
    sphere_finish(c, frame);
    Ok(())
}

/// `int SphereTree_CollideAABBNode(SphereTree* self, t4Vector* boxMin,
/// t4Vector* boxMax, t4Vector* pos, int depth, int base, t4Vector* outPos,
/// t4Vector* outNormal)` (0x32d028): a sphere-box test of the node at
/// `depth`. On a miss it returns 0. On a hit at a leaf (or a node with no
/// children below the tree depth) it stores the normal and the scaled
/// direction and returns 1. Otherwise it recurses into the children whose
/// bit is set in the node's mask and returns 1 at the first child that hits.
pub fn sphere_tree_collide_aabb_node(c: &mut Cpu) {
    guest(c, sphere_collide_body)
}

/// How the interior walk ended.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum CollideWalk {
    /// The node is a leaf (no children, or past the tree's depth): run the
    /// leaf test.
    Leaf,
    /// The children were visited and the loop ended: the epilogue follows
    /// (`s0` already restored).
    Returned,
}

/// How the body ends before the epilogue.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum CollideExit {
    /// `s0` already restored: straight to the epilogue.
    Done,
    /// The early and the update exits: one more cycle, then `s0` restored.
    Early,
}

/// `void SphereTree_CollidePlaneNode(SphereTree* self, void* plane,
/// t4Vector* pos, int depth, int base, t4Vector* outA, t4Vector* outB)`
/// (0x0032d208): pushes a plane down the sphere tree. A node whose
/// `plane . pos` lies beyond its radius plus `worstCase` is skipped. An
/// interior node (`depth` above the tree's depth and a non-zero mask) hands
/// the plane to the children its mask selects, nearest-first in `sortAxis`
/// order, each moved by its axis times `cenOffset`. A leaf that lies closer
/// than `worstCase` stores its normal in `outA`, the point on the plane in
/// `outB` and its distance as the new `worstCase`.
pub fn sphere_tree_collide_plane_node(c: &mut Cpu) {
    guest(c, collide_plane_node_body);
}

pub(crate) fn collide_plane_node_body(c: &mut Cpu) -> Flow {
    use collide_plane_node::*;
    c.cycles += 42;
    let frame = Frame::open(c, FRAME);
    c.g[V0][0] = 0xc;
    frame.save(c, 16, 0xe0);
    frame.save(c, 20, 0xa0);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, 21, 0x90);
    c.g[S4][0] = c.g[A1][0];
    frame.save(c, 22, 0x80);
    c.g[S5][0] = c.g[A2][0];
    frame.save(c, 23, 0x70);
    c.g[S6][0] = c.g[A3][0];
    frame.save(c, 30, 0x60);
    // a3 = depth * 12: the byte offset of levels[depth].
    c.mult(0, 7, 22, 2);
    frame.save(c, 17, 0xd0);
    c.g[S7][0] = c.g[T0][0];
    frame.save(c, 18, 0xc0);
    c.g[FP][0] = c.g[T1][0];
    frame.save(c, 19, 0xb0);
    frame.save_ra(c, 0x50);
    frame.save_fpr(c, 20, 0xf0);

    // planeDist = dot(plane, pos) (the unit's VU0 dot sequence).
    c.lqc2(3, c.g[S4][0] as u32);
    c.lqc2(5, c.g[S5][0] as u32);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(3, 4);
    c.mtc1(3, 4);
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(TREE_INFO));
    c.mov_s(3, 4);
    c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(SCALE));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(INFO_LEVELS));
    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(WORST_CASE));
    // &levels[depth]; nodeRadius = radius * scale + worstCase.
    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(LEVEL_RADIUS));
    c.mul_s(1, 1, 2);
    c.add_s(0, 1, 0);
    // planeDist < nodeRadius is the test that keeps the node.
    c.c_lt_s(3, 0);
    let inside = c.fcc();
    let outb_slot = frame.at(c, 0x40);
    let outb = c.g[T2][0];
    c.sw(outb_slot, outb);

    let exit = if !inside {
        CollideExit::Early
    } else {
        match collide_walk(c, frame)? {
            CollideWalk::Returned => CollideExit::Done,
            CollideWalk::Leaf => collide_leaf(c, frame),
        }
    };
    if exit == CollideExit::Early {
        c.cycles += 1;
        frame.restore(c, 16, 0xe0);
    }

    // Epilogue.
    c.cycles += 12;
    frame.restore(c, 17, 0xd0);
    frame.restore(c, 18, 0xc0);
    frame.restore(c, 19, 0xb0);
    frame.restore(c, 20, 0xa0);
    frame.restore(c, 21, 0x90);
    frame.restore(c, 22, 0x80);
    frame.restore(c, 23, 0x70);
    frame.restore(c, 30, 0x60);
    frame.restore_ra(c, 0x50);
    frame.restore_fpr(c, 20, 0xf0);
    frame.close(c);
    Ok(())
}

/// The interior node: visits the children its mask selects (in `sortAxis`
/// order), or reports a leaf.
pub(crate) fn collide_walk(c: &mut Cpu, frame: Frame) -> Result<CollideWalk, Unwound> {
    use collide_plane_node::*;
    c.cycles += 6;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(INFO_BIT_TREE));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(INFO_DEPTH));
    // The node's mask: bitTree[base].
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
    c.g[V1][0] = ((c.g[S6][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let below_depth = c.g[V1][0] != 0;
    c.g[S3][0] = c.lbu(c.g[V0][0] as u32);
    if !below_depth {
        return Ok(CollideWalk::Leaf);
    }
    c.cycles += 2;
    let has_children = c.g[S3][0] != 0;
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(SORT_AXIS));
    if !has_children {
        return Ok(CollideWalk::Leaf);
    }
    c.cycles += 4;
    // distance = levels[depth + 1].cenOffset (the children's offset).
    c.lwc1(
        20,
        (c.g[A1][0] as u32).wrapping_add(LEVEL_SIZE + LEVEL_CEN_OFFSET),
    );
    let depth12_slot = frame.at(c, 0x44);
    let depth12 = c.g[A3][0];
    c.sw(depth12_slot, depth12);
    c.g[S2][0] = 0x7;
    loop {
        c.cycles += 5;
        c.g[T3][0] = c.lw(c.g[S1][0] as u32); // k = sortAxis[i]
        c.g[V0][0] = sx(((c.g[S3][0] as u32) as i32 >> ((c.g[T3][0] as u32) & 31)) as u32);
        c.g[V0][0] &= 0x1;
        let child_set = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[T3][0] as u32) << 4);
        if child_set {
            c.cycles += 29;
            c.g[A0][0] = c.g[S0][0];
            // &self->axis[k]
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
            c.mfc1(3, 20);
            c.lqc2(4, c.g[V0][0] as u32);
            c.qmtc2(3, 3);
            // childOffset = axis[k] * distance
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            let scaled = frame.at(c, 0x20);
            c.sqc2(5, scaled);
            c.g[A1][0] = c.g[S4][0];
            c.lq(2, frame.at(c, 0x20));
            c.g[A2][0] = c.g[SP][0];
            c.g[T2][0] = c.lw(frame.at(c, 0x40));
            c.g[A3][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
            c.sq(2, frame.at(c, 0x10));
            c.g[T1][0] = c.g[FP][0];
            c.lqc2(3, c.g[S5][0] as u32);
            c.lqc2(4, frame.at(c, 0x10));
            // childPos = pos + childOffset, kept at sp for the child.
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            let child_pos = frame.at(c, 0x20);
            c.sqc2(5, child_pos);
            c.lq(3, frame.at(c, 0x20));
            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(TREE_INFO));
            c.sq(3, sp(c));
            c.g[V1][0] = c.lw(frame.at(c, 0x44));
            c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(INFO_LEVELS));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(LEVEL_INDEX_OFFSET));
            // childBase = (k + 1) * indexOffset + base
            c.mult(0, 2, 11, 8);
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
            Callee {
                addr: ENTRY,
                f: ee::s11::SphereTree_CollidePlaneNode,
            }
            .call(c, RET_CHILD)?;
        }
        c.cycles += 3;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
        let more = (c.g[S2][0] as i64) >= 0;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        if !more {
            c.cycles += 2;
            frame.restore(c, 16, 0xe0);
            return Ok(CollideWalk::Returned);
        }
        // Loop back edge: the event is delivered here; the result is not
        // acted on (the generated code continues the loop either way).
        if event_due(c) {
            deliver_event_here(c, LOOP_HEAD);
        }
    }
}

/// The leaf test: the distance `planeDist - nodeRadius`, and when it is
/// closer than `worstCase`, the outputs and the new `worstCase`.
pub(crate) fn collide_leaf(c: &mut Cpu, frame: Frame) -> CollideExit {
    use collide_plane_node::*;
    c.cycles += 6;
    c.sub_s(20, 4, 1); // distance = planeDist - nodeRadius
    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(WORST_CASE));
    c.c_lt_s(20, 0);
    if !c.fcc() {
        frame.restore(c, 16, 0xe0);
        return CollideExit::Done;
    }
    c.cycles += 29;
    // n = plane with w cleared (the word at sp+0x1c is zeroed).
    c.lq(2, c.g[S4][0] as u32);
    c.sq(2, frame.at(c, 0x10));
    let w_slot = frame.at(c, 0x1c);
    c.sw(w_slot, 0u64);
    c.lq(2, frame.at(c, 0x10));
    c.sq(2, frame.at(c, 0x0));
    // *outA = n * distance
    c.mfc1(3, 20);
    c.lqc2(4, frame.at(c, 0x0));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0x20));
    c.lq(2, frame.at(c, 0x20));
    c.sq(2, c.g[FP][0] as u32);
    c.sq(2, frame.at(c, 0x10));
    // *outB = pos - n * planeDist
    c.mfc1(3, 3);
    c.lqc2(4, frame.at(c, 0x0));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0x30));
    c.lq(2, frame.at(c, 0x30));
    c.sq(2, frame.at(c, 0x20));
    c.lqc2(3, c.g[S5][0] as u32);
    c.lqc2(4, frame.at(c, 0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x30));
    c.lq(2, frame.at(c, 0x30));
    c.g[V1][0] = c.lw(frame.at(c, 0x40));
    c.sq(2, c.g[V1][0] as u32);
    c.swc1(20, (c.g[S0][0] as u32).wrapping_add(WORST_CASE));
    c.sq(2, frame.at(c, 0x10));
    CollideExit::Early
}

/// `SphereTreeCache_GetMasks` (0x0032df28): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sphere_tree_cache_get_masks(c: &mut Cpu) {
    if c.pc != 0x0032df28 {
        return ee::s11::f_0032df28__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0032df28;
        loop {
            match pc {
                0x0032df28 => {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef8));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = c.lw(c.g[A0][0] as u32);
                    if t {
                        pc = 0x0032e008;
                        continue;
                    }
                    pc = 0x0032df50;
                    continue;
                }
                0x0032df50 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xedc));
                    c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[V0][0] != c.g[S0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee0));
                    if t {
                        pc = 0x0032df78;
                        continue;
                    }
                    pc = 0x0032df6c;
                    continue;
                }
                0x0032df6c => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xed8));
                    let t = true;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if t {
                        pc = 0x0032e0e8;
                        continue;
                    }
                    pc = 0x0032df78;
                    continue;
                }
                0x0032df78 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V1][0] = 0u64;
                    if t {
                        pc = 0x0032dfa4;
                        continue;
                    }
                    pc = 0x0032df80;
                    continue;
                }
                0x0032df80 => {
                    c.cycles += 2;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[A2][0];
                    pc = 0x0032df88;
                    continue;
                }
                0x0032df88 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    let t = c.g[V0][0] == c.g[S0][0];
                    c.g[A2][0] = sx((c.g[V1][0] as u32) << 2);
                    if t {
                        pc = 0x0032e030;
                        continue;
                    }
                    pc = 0x0032df94;
                    continue;
                }
                0x0032df94 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0032df88) {
                            return Ok(());
                        }
                        pc = 0x0032df88;
                        continue;
                    }
                    pc = 0x0032dfa4;
                    continue;
                }
                0x0032dfa4 => {
                    c.cycles += 21;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee4));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xed8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[RA][0] = sx(0x0032dfc0);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    call_ptr(c, 0x0032b620, 0x0032dfc0)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee4));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xedc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[S0][0]);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee4));
                    c.sw(c.g[S1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee4));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.div(0, 2, 3);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0xee4), c.g[V0][0]);

                    if c.g[V1][0] == 0u64 {
                        pc = 0x0032dff8;
                        continue;
                    }
                    pc = 0x0032dff8;
                    continue;
                }
                0x0032dff8 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xed8));
                    c.g[A0][0] = c.hi[0];
                    let t = true;
                    c.sw((c.g[GP][0] as u32).wrapping_add(0xee4), c.g[A0][0]);
                    if t {
                        pc = 0x0032e0dc;
                        continue;
                    }
                    pc = 0x0032e008;
                    continue;
                }
                0x0032e008 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xeec));
                    c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[V0][0] != c.g[S0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef0));
                    if t {
                        pc = 0x0032e050;
                        continue;
                    }
                    pc = 0x0032e024;
                    continue;
                }
                0x0032e024 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee8));
                    let t = true;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if t {
                        pc = 0x0032e0e8;
                        continue;
                    }
                    pc = 0x0032e030;
                    continue;
                }
                0x0032e030 => {
                    c.cycles += 4;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xed8));
                    let t = true;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if t {
                        pc = 0x0032e0e8;
                        continue;
                    }
                    pc = 0x0032e040;
                    continue;
                }
                0x0032e040 => {
                    c.cycles += 4;
                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee8));
                    let t = true;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if t {
                        pc = 0x0032e0e8;
                        continue;
                    }
                    pc = 0x0032e050;
                    continue;
                }
                0x0032e050 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V1][0] = 0u64;
                    if t {
                        pc = 0x0032e07c;
                        continue;
                    }
                    pc = 0x0032e058;
                    continue;
                }
                0x0032e058 => {
                    c.cycles += 2;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[A2][0];
                    pc = 0x0032e060;
                    continue;
                }
                0x0032e060 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    let t = c.g[V0][0] == c.g[S0][0];
                    c.g[A2][0] = sx((c.g[V1][0] as u32) << 2);
                    if t {
                        if back_edge_unwinds(c, 0x0032e040) {
                            return Ok(());
                        }
                        pc = 0x0032e040;
                        continue;
                    }
                    pc = 0x0032e06c;
                    continue;
                }
                0x0032e06c => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0032e060) {
                            return Ok(());
                        }
                        pc = 0x0032e060;
                        continue;
                    }
                    pc = 0x0032e07c;
                    continue;
                }
                0x0032e07c => {
                    c.cycles += 21;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef4));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[RA][0] = sx(0x0032e098);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    call_ptr(c, 0x0032b620, 0x0032e098)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef4));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xeec));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[S0][0]);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef4));
                    c.sw(c.g[S1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef4));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.div(0, 2, 3);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0xef4), c.g[V0][0]);

                    if c.g[V1][0] == 0u64 {
                        pc = 0x0032e0d0;
                        continue;
                    }
                    pc = 0x0032e0d0;
                    continue;
                }
                0x0032e0d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee8));
                    c.g[A0][0] = c.hi[0];
                    c.sw((c.g[GP][0] as u32).wrapping_add(0xef4), c.g[A0][0]);
                    pc = 0x0032e0dc;
                    continue;
                }
                0x0032e0dc => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    pc = 0x0032e0e8;
                    continue;
                }
                0x0032e0e8 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0032df28, 0x0032e100);
                    return Ok(());
                }
            }
        }
    });
}

/// `int SphereTree_CollideTreeNode(SphereTree* tree, Vec4* pos, Vec4* centre,
/// int level, int maskOff, ...)` (0x32cbf8). Collides the node's level: when
/// `level` is inside the tree and its mask byte is set, every child slot whose
/// bit is set is tested with `SphereTree_CollideSphereNode`; a hit there sends
/// the walk one level deeper through this function itself. Returns 1 on a hit,
/// else 0 (1 also for the early exits that leave the result at 1).
pub fn sphere_tree_collide_tree_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x100);
        let stack = sp(c);
        frame.save(c, S0, 0xe0);
        frame.save(c, S1, 0xd0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0xa0);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S5, 0x90);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S6, 0x80);
        c.g[S5][0] = c.g[T2][0];
        frame.save(c, FP, 0x60);
        c.g[S6][0] = c.g[T1][0];
        frame.save(c, S2, 0xc0);
        c.g[FP][0] = c.g[T0][0];
        frame.save(c, S3, 0xb0);
        frame.save(c, S7, 0x70);
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 20, 0xf0);
        c.sw(stack + 0x40, c.g[A2][0]);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[FP][0] as u32));
        c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let outside = c.g[V1][0] == 0;
        c.g[S7][0] = c.lbu(c.g[V0][0] as u32);
        // Exits: `plus_one` is the early exit that adds a cycle and leaves
        // v0 = 1; the other breaks leave v0 as set.
        let mut plus_one = false;
        'body: {
            if outside {
                plus_one = true;
                break 'body;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
            let below_four = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if !below_four {
                break 'body;
            }
            c.cycles += 2;
            let no_mask = c.g[S7][0] == 0;
            c.g[V0][0] = 0xc;
            if no_mask {
                plus_one = true;
                break 'body;
            }
            c.cycles += 7;
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
            c.mult(0, V0, S1, V0);
            c.g[A0][0] = 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.lwc1(20, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx(((c.g[S7][0] as u32) as i32 >> ((c.g[A0][0] as u32) & 31)) as u32);
            loop {
                'slot: {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let bit_set = c.g[V0][0] != 0;
                    c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if !bit_set {
                        break 'slot;
                    }
                    c.cycles += 35;
                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 4);
                    c.g[V1][0] = 0xc;
                    c.mult(0, S2, S1, V1);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.mfc1(3, 20);
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, stack + 0x30);
                    c.g[A0][0] = c.g[S4][0];
                    c.lq(2, stack + 0x30);
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x80));
                    c.g[A3][0] = 0;
                    c.sq(2, stack + 0x20);
                    c.g[T0][0] = 0;
                    c.g[V0][0] = c.lw(stack + 0x40);
                    c.g[T1][0] = c.g[S6][0];
                    c.lqc2(3, c.g[V0][0] as u32);
                    c.lqc2(4, stack + 0x20);
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, stack + 0x30);
                    c.g[T2][0] = c.g[S5][0];
                    c.lq(2, stack + 0x30);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                    c.sq(2, stack);
                    c.sq(2, stack + 0x10);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.lwc1(12, c.g[V0][0] as u32);
                    c.mul_s(12, 12, 0);
                    Callee {
                        addr: 0x0032_cdb0,
                        f: ee::s10::SphereTree_CollideSphereNode,
                    }
                    .call(c, 0x0032_cd20)?;
                    let miss = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if miss {
                        break 'slot;
                    }
                    c.cycles += 14;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[T1][0] = c.g[S6][0];
                    c.g[T2][0] = c.g[S5][0];
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.mult(0, V0, S3, T0);
                    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    Callee {
                        addr: 0x0032_cbf8,
                        f: ee::s10::SphereTree_CollideTreeNode,
                    }
                    .call(c, 0x0032_cd58)?;
                    let hit = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if hit {
                        break 'body;
                    }
                    break 'slot;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                c.g[V0][0] = ((c.g[A0][0] as i64) < 8) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx(((c.g[S7][0] as u32) as i32 >> ((c.g[A0][0] as u32) & 31)) as u32);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_cc90);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'body;
            }
        }
        if plus_one {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 13;
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// `SphereTree_SortChildOrder` (0x32db40): builds the four child keys with
/// VU0 (`-1000` scaled by the sphere's centre), fills the order and distance
/// arrays for four children, then sorts the key/value pairs descending with
/// `ShellSortDesc_KeyVal` (0x32da40).
pub fn sphere_tree_sort_child_order_2(c: &mut Cpu) {
    guest(c, sort_child_order_body_2)
}

pub(crate) fn sort_child_order_body_2(c: &mut Cpu) -> Flow {
    c.cycles += 20;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.g[AT][0] = 0xffff_ffff_c47a_0000;
    c.mtc1(1, 0);
    c.sd(sp(c).wrapping_add(0x40), c.g[RA][0]);
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9c));
    c.mfc1(2, 0);
    c.lqc2(4, c.g[A1][0] as u32);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp(c).wrapping_add(0x30));
    c.g[T2][0] = 0;
    c.lq(2, sp(c).wrapping_add(0x30));
    c.g[A1][0] = c.g[A0][0];
    c.g[T4][0] = 0x7;
    c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb8));
    c.sq(2, sp(c).wrapping_add(0x20));
    c.g[T1][0] = c.g[A2][0];
    c.g[T0][0] = sx(sp(c).wrapping_add(0x1c));
    c.g[A3][0] = c.g[SP][0];
    loop {
        c.cycles += 25;
        c.lqc2(3, c.g[A1][0] as u32);
        c.lqc2(5, sp(c).wrapping_add(0x20));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.g[V1][0] = sx((c.g[T4][0] as u32).wrapping_sub(c.g[T2][0] as u32));
        c.cvt_w_s(1, 0);
        c.mfc1(4, 1);
        c.sw(c.g[T1][0] as u32, c.g[T2][0]);
        c.sw(c.g[T3][0] as u32, c.g[V1][0]);
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x4));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
        c.sw(c.g[A3][0] as u32, c.g[A0][0]);
        c.sw(c.g[T0][0] as u32, c.g[V0][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4));
        c.g[V0][0] = ((c.g[T2][0] as i64) < 4i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_db90);
        }
    }
    c.cycles += 6;
    c.g[A0][0] = 0x8;
    c.g[A1][0] = c.g[SP][0];
    Callee {
        addr: 0x0032_da40,
        f: ee::s11::f_0032da40,
    }
    .call(c, 0x0032_dc00)?;
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x40));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
    ret(c);
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;

    use crate::difftest::fakes::covered;

    use crate::difftest::fakes::machine;

    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::computer::tests::literals;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::ai::rider::tests::f;

    pub(crate) use crate::game::ai::rider::tests::gp_f32;

    pub(crate) use crate::game::ai::rider::tests::put_sphere_tree;
    pub(crate) use crate::game::ai::rider::tests::put_v;
    pub(crate) use crate::game::ai::rider::tests::rand_v;
    pub(crate) use crate::game::ai::rider::tests::setup_machine;
    pub(crate) use crate::game::ai::rider::tests::uf_2 as uf;

    pub(crate) use crate::game::ai::rider::tests::SEEDS;

    pub(crate) use crate::game::ai::rider::tests::VT_VOL;
    pub(crate) use crate::game::be::beintplayer::tests::base_10 as base_5;
    pub(crate) use crate::game::be::beintplayer::tests::fillb;

    pub(crate) use crate::game::be::beintplayer::tests::strict;
    pub(crate) use crate::game::be::beintplayer::tests::SEEDS_7 as SEEDS_6;
    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::bx::bxstringctor::tests::fbits;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_5;

    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::ovstates::tests::base_3;
    pub(crate) use crate::game::intersect::worldsphtree::tests::SEEDS_2;
    pub(crate) use crate::game::object::flexrailnode::tests::base_3 as base_6;
    pub(crate) use crate::game::object::flexrailnode::tests::check_all;
    pub(crate) use crate::game::object::flexrailnode::tests::floats;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_4;

    pub(crate) use crate::game::sound::soundsys::tests::wf;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_4;
    use ssx3_ee as ee;

    #[test]
    fn sphere_tree_get_half_leaf_radius_matches() {
        check_lohi(
            ssx3_ee::s10::f_0032c5a8,
            sphere_tree_get_half_leaf_radius,
            200,
            &|c, seed| {
                let mut r = Rng(0x100f ^ seed as u64);
                scramble(c, &mut r);
                let leaf = DATA + 0x200;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x98, leaf);
                let idx = seed % 8;
                c.bus.write32(leaf + 0xc, idx);
                c.bus.write32(leaf + 0x20, DATA);
                c.bus.write32(DATA + 0x90, r.edge_f32());
                c.bus.write32(DATA + idx * 0xc, r.edge_f32());
            },
        );
    }

    pub(crate) fn sphere_setup(c: &mut Cpu, r: &mut Rng, na: u32, nb: u32, spread: f32) {
        let (a, b) = (DATA + 0x1000, DATA + 0x2000);
        let ca = rand_v(r, -2.0, 2.0);
        let off = rand_v(r, -spread, spread);
        let cb = [ca[0] + off[0], ca[1] + off[1], ca[2] + off[2], 0.0];
        let (ra, rb) = (uf(r, 1.0, 3.0), uf(r, 1.0, 3.0));
        put_sphere_tree(c, r, a, ca, ra, na);
        put_sphere_tree(c, r, b, cb, rb, nb);
        c.g[A0][0] = sx(a);
        c.g[A1][0] = sx(b);
        c.g[A2][0] = sx(DATA + 0x3000);
        c.g[A3][0] = sx(DATA + 0x3010);
        c.pc = ST_ENTRY;
    }

    driven!(gen_sphere_tree, ee::s10::f_00329f98__generated);

    driven!(hand_sphere_tree, rider_sphere_tree_intersect);

    #[test]
    fn rider_sphere_tree_intersect_matches() {
        check(
            ee::s10::f_00329f98__generated,
            rider_sphere_tree_intersect,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x9f98, 0);
                let spread = if seed % 2 == 0 { 1.0 } else { 5.0 };
                sphere_setup(c, &mut r, seed % 5, (seed / 5) % 5, spread);
            },
        );
        // Events at the loop heads and the hit block, with re-entry.
        check_events(gen_sphere_tree, hand_sphere_tree, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x9f99, 0);
            sphere_setup(c, &mut r, 2 + seed % 4, 1 + (seed / 4) % 4, 0.5);
            schedule(c, &mut r, seed, ST_ENTRY)
        });
    }

    // -----------------------------------------------------------------
    // Forward-difference cache.

    pub(crate) const SENTINEL: f32 = -99.0;

    pub(crate) const DEGENERATE: f32 = -98.0;

    /// An entry at `e`: 10x10 vertices from +0x10 (0x10 apart, rows 0xa0),
    /// coarse cells at +0x650, fine cells at +0x1a90; cells either
    /// uncomputed (sentinel) or with precomputed normals and boxes.
    pub(crate) fn put_entry(c: &mut Cpu, r: &mut Rng, e: u32) {
        for i in 0..100u32 {
            let (row, col) = ((i / 10) as f32, (i % 10) as f32);
            let y = if r.next_u32() % 8 == 0 {
                0.0
            } else {
                uf(r, -1.0, 1.0)
            };
            put_v(c, e + 0x10 + 0x10 * i, [col - 4.5, y, row - 4.5, 1.0]);
        }
        for k in 0..(81 + 9) {
            let cl = e + 0x650 + 0x40 * k;
            let fresh = r.next_u32() % 3 != 0;
            if fresh {
                c.bus.write32(cl, f(SENTINEL));
                c.bus.write32(cl + 0x10, f(SENTINEL));
                c.bus.write32(cl + 0x20, f(SENTINEL));
            } else {
                let n0 = if r.next_u32() % 4 == 0 {
                    DEGENERATE
                } else {
                    uf(r, -1.0, 1.0)
                };
                let n1 = if r.next_u32() % 4 == 0 {
                    DEGENERATE
                } else {
                    uf(r, -1.0, 1.0)
                };
                put_v(c, cl, [n0, uf(r, 0.0, 1.0), uf(r, -1.0, 1.0), 0.0]);
                put_v(c, cl + 0x10, [n1, uf(r, 0.0, 1.0), uf(r, -1.0, 1.0), 0.0]);
                let lo = rand_v(r, -5.0, 5.0);
                put_v(c, cl + 0x20, lo);
                put_v(
                    c,
                    cl + 0x30,
                    [
                        lo[0] + uf(r, 0.0, 2.0),
                        lo[1] + uf(r, 0.0, 2.0),
                        lo[2] + uf(r, 0.0, 2.0),
                        0.0,
                    ],
                );
            }
        }
    }

    #[test]
    fn ai_fwd_diff_cache_collide_entry_matches() {
        covered(
            ee::s10::f_0032b6e0__generated,
            ai_fwd_diff_cache_collide_entry,
            SEEDS,
            &|c, seed| {
                let mut r = setup_machine(c, seed, 0xb6e0, 6);
                let (vol, entry, hint, uv) =
                    (DATA + 0x100, DATA + 0x4000, DATA + 0x200, DATA + 0x300);
                put_entry(c, &mut r, entry);
                for (off, x) in [
                    (-0x2dc0, 0.1f32),
                    (-0x2dbc, 0.05),
                    (-0x2db8, 0.3),
                    (-0x2db4, 0.15),
                ] {
                    gp_f32(c, off, x);
                }
                for off in [-0x2db0, -0x2d94] {
                    gp_f32(c, off, SENTINEL);
                }
                for off in [-0x2dac, -0x2da8, -0x2da0, -0x2d9c, -0x2d90] {
                    gp_f32(c, off, DEGENERATE);
                }
                for off in [-0x2da4, -0x2d98, -0x2d8c] {
                    gp_f32(c, off, uf(&mut r, -0.5, 0.5));
                }
                let kind = seed % 3;
                c.bus.write32(vol, kind);
                c.bus.write32(vol + 8, (r.next_u32() % 2 == 0) as u32);
                put_v(
                    c,
                    vol + 0x10,
                    [uf(&mut r, -0.5, 0.5), 1.0, uf(&mut r, -0.5, 0.5), 0.0],
                );
                let lo = rand_v(&mut r, -5.0, 3.0);
                let ext = uf(&mut r, 0.5, 4.0);
                put_v(c, vol + 0x20, [lo[0] + ext, lo[1] + ext, lo[2] + ext, 0.0]);
                put_v(c, vol + 0x30, lo);
                c.bus.write32(vol + 0x50, VT_VOL);
                let key = 7;
                let side = if kind == 0 { 3 } else { 9 };
                c.bus
                    .write32(hint, if seed % 4 == 0 { key + 1 } else { key });
                c.bus.write16(hint + 4, (r.next_u32() % side) as u16);
                c.bus.write16(hint + 6, (r.next_u32() % side) as u16);
                c.bus.write32(hint + 8, r.next_u32() % 2);
                c.bus.write32(
                    hint + 0xc,
                    if seed % 5 == 0 { (kind + 1) % 3 } else { kind },
                );
                c.g[A0][0] = sx(vol);
                c.g[A1][0] = sx(key);
                c.g[A2][0] = sx(entry);
                c.g[T2][0] = sx(uv);
                c.g[T3][0] = if seed % 7 == 0 { 0 } else { sx(hint) };
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
            SEEDS_2,
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
    fn rider_sphere_tree_get_bounds_matches() {
        check(
            ssx3_ee::s10::f_00329dc8__generated,
            rider_sphere_tree_get_bounds,
            300,
            &|c, seed| {
                let mut r = Rng(0x51f3_9a2b ^ seed as u64);
                scramble(c, &mut r);
                literals(c);
                c.g[GP][0] = GAME_GP as u64;
                let tree = DATA;
                c.g[A0][0] = tree as u64;
                c.g[A1][0] = (tree + 0x100) as u64;
                c.g[A2][0] = (tree + 0x110) as u64;
                // A leaf every fourth seed; otherwise a mask that selects some (or
                // none) of the children, with 0..2 of them counted.
                let leaf = seed % 4 == 0;
                let mask = if leaf { 0 } else { (seed / 4) % 4 + 1 };
                let count = (seed / 16) % 3;
                c.bus.write32(tree + 0x28, mask);
                c.bus.write32(tree + 0x2c, count);
                put_vec(
                    c,
                    tree + 0x10,
                    [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.edge_f32()],
                );
                c.bus.write32(tree + 0x20, r.f32(50.0));
                for i in 0..2 {
                    let kid = tree + 0x30 + i * 0x20;
                    put_vec(
                        c,
                        kid,
                        [r.f32(200.0), r.f32(200.0), r.f32(200.0), r.edge_f32()],
                    );
                    c.bus.write32(kid + 0x10, r.f32(30.0));
                }
            },
        );
    }

    // CollSegment_ctor -----------------------------------------------------

    pub(crate) const SEGMENT: u32 = DATA;

    pub(crate) const SEG_P0: u32 = DATA + 0x100;

    pub(crate) const SEG_P1: u32 = DATA + 0x200;

    #[test]
    fn coll_segment_ctor_matches() {
        check(
            ssx3_ee::s11::f_0032e100__generated,
            coll_segment_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x0032_e100 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = SEGMENT as u64;
                c.g[A1][0] = SEG_P0 as u64;
                c.g[A2][0] = SEG_P1 as u64;
                c.g[A3][0] = (seed % 3) as u64;
                put_vec(
                    c,
                    SEG_P0,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    SEG_P1,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                c.f[12] = r.f32(3.0);
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    // CollHull_ctor --------------------------------------------------------

    pub(crate) const HULL: u32 = DATA;

    pub(crate) const HULL_TREE: u32 = DATA + 0x1000;

    #[test]
    fn coll_hull_ctor_matches() {
        check(
            ssx3_ee::s11::f_0032f650__generated,
            coll_hull_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x0032_f650 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = HULL as u64;
                c.g[A1][0] = HULL_TREE as u64;
                c.g[A2][0] = (seed % 2) as u64;
                // An empty sphere tree: the bounds come from the empty-box
                // globals and there is no per-sphere loop.
                c.bus.write32(HULL_TREE + 0x28, 0);
                c.bus.write32(GAME_GP - 0x2de8, f(1.5));
                c.bus.write32(GAME_GP - 0x2de4, f(-2.5));
            },
        );
    }

    #[test]
    fn rider_sphere_tree_dtor_matches() {
        check(
            ssx3_ee::s10::f_00329970__generated,
            rider_sphere_tree_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3299_70, 0);
                // Freeing a live object would walk the (uninitialised) heap manager,
                // so the delete branch is driven with a null object and the keep
                // branch with a real one: both reach the same code.
                if seed % 2 == 1 {
                    c.g[A0][0] = 0;
                    c.g[A1][0] = 1;
                } else {
                    c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                    c.g[A1][0] = sx(r.next_u32() & !1);
                }
            },
        );
    }

    #[test]
    fn coll_hull_dtor_matches() {
        check(
            ssx3_ee::s11::f_0032f708__generated,
            coll_hull_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32f7_08, 0);
                let flags = (r.next_u32() & !1) | (seed % 2) as u32;
                c.g[A1][0] = sx(flags);
                c.g[A0][0] = if flags & 1 != 0 {
                    0 // delete(null): a live hull would go to the heap manager
                } else {
                    sx(DATA + (r.next_u32() & 0x1ff0))
                };
            },
        );
    }

    #[test]
    fn vu0_triangle_intersect_matches() {
        check(
            ssx3_ee::s10::f_0032b6a8__generated,
            vu0_triangle_intersect,
            100,
            &|c, seed| {
                let mut r = Rng(0x32b6_a801 ^ seed as u64);
                scramble(c, &mut r);
                for k in 0..4 {
                    let at = DATA + 0x100 + 0x10 * k as u32;
                    c.g[A0 + k][0] = sx(at);
                    put_vec(
                        c,
                        at,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    );
                }
            },
        );
    }

    // -----------------------------------------------------------------------
    // Sphere tree helpers.

    #[test]
    fn sphere_tree_ctor_matches() {
        check(
            ssx3_ee::s10::f_0032c508__generated,
            sphere_tree_ctor,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32c5_08, 0);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x98, r.next_u32());
                for i in 0..8u32 {
                    c.bus.write32(DATA + i * 16, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn coll_sphere_collide_aabb_matches() {
        check(
            ssx3_ee::s11::f_0032fe40__generated,
            coll_sphere_collide_aabb,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32fe_40, 0);
                let this = DATA;
                let box_ = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                c.g[A3][0] = box_ as u64;
                for i in 0..0x60u32 / 4 {
                    c.bus.write32(this + i * 4, r.next_u32());
                    c.bus.write32(box_ + i * 4, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn rider_sphere_tree_set_spheres_matches() {
        check(
            ee::s10::f_00329a90__generated,
            rider_sphere_tree_set_spheres,
            SEEDS_3,
            &|c, seed| {
                let mut r = base(c, seed, 0x329a_90);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let spheres = DATA + 0x1000;
                let radii = DATA + 0x2000;
                for i in 0..6u32 {
                    for k in 0..4 {
                        c.bus.write32(spheres + 0x10 * i + 4 * k, r.next_u32());
                        c.bus.write32(radii + 4 * (4 * i + k), r.edge_f32());
                    }
                }
                let center = DATA + 0x3000;
                for k in 0..4 {
                    c.bus.write32(center + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = [-1i64 as u64, 0, 1, 3, 6][(seed % 5) as usize];
                c.g[A2][0] = sx(spheres);
                c.g[A3][0] = sx(radii);
                c.g[T0][0] = sx(center);
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn sphere_tree_reset_transform_matches() {
        check(
            ee::s10::f_0032c540__generated,
            sphere_tree_reset_transform,
            SEEDS_3,
            &|c, seed| {
                let mut r = base(c, seed, 0x32c5_40);
                let this = DATA + 0x100;
                clear(c, this, 0x100);
                let table = 0x004f_f640u32;
                for i in 0..8u32 {
                    for k in 0..4 {
                        c.bus.write32(table + 0x10 * i + 4 * k, r.next_u32());
                    }
                }
                for k in 0..4 {
                    c.bus.write32(0x004f_f130 + 4 * k, r.next_u32());
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ---------------------------------------------------------- collision

    #[test]
    fn coll_sphere_transform_matches() {
        check(
            ssx3_ee::s11::f_0032fd40__generated,
            coll_sphere_transform,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0xfd40, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                fill(c, &mut r, DATA, 0x40);
                for i in 0..16 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.f32(4.0));
                }
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn coll_segment_reset_transform_matches() {
        check(
            ee::s11::f_0032e4b8__generated,
            coll_segment_reset_transform,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 47);
                c.g[A0][0] = AUX as u64;
                fillb(c, &mut r, AUX + 0x60, 0x40);
                put(c, AUX + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn coll_segment_collide_aabb_matches() {
        check(
            ee::s11::f_0032e690__generated,
            coll_segment_collide_aabb,
            250,
            &|c, seed| {
                let mut r = setup(c, seed, 48);
                gpf(c, 0x2888, if seed % 2 == 0 { 0 } else { 1 });
                gpf(c, -0x2d74, fbits(1e-6));
                let a = |x: f32| fbits(x);
                let (o, d, lo, hi): ([u32; 3], [u32; 3], [u32; 3], [u32; 3]) = match seed % 8 {
                    // Through the box: an entry and an exit hit.
                    0 => ([0, 0, 0], [a(1.0); 3], [a(-0.5); 3], [a(0.5); 3]),
                    // Origin inside, exit ahead only: one hit.
                    1 => ([0, 0, 0], [a(1.0); 3], [a(-2.0); 3], [a(0.5); 3]),
                    // Box behind the origin: miss (tmax < 0).
                    2 => ([0, 0, 0], [a(1.0); 3], [a(-1.0); 3], [a(-0.5); 3]),
                    // Box beyond the segment: miss (1 < tmin).
                    3 => ([0, 0, 0], [a(1.0); 3], [a(2.0); 3], [a(3.0); 3]),
                    // Parallel to x.
                    4 => ([0, 0, 0], [0, a(1.0), a(1.0)], [a(-0.5); 3], [a(0.5); 3]),
                    // Box to the side: miss (tmax < tmin).
                    5 => (
                        [0, 0, 0],
                        [a(1.0); 3],
                        [a(5.0), a(-1.0), a(-1.0)],
                        [a(6.0), a(1.0), a(1.0)],
                    ),
                    // Entering from the far corner.
                    6 => ([a(1.0); 3], [a(-1.0); 3], [a(0.2); 3], [a(0.8); 3]),
                    // Arbitrary.
                    _ => {
                        let mut v = [0u32; 3];
                        for slot in v.iter_mut() {
                            *slot = r.f32(2.0);
                        }
                        let mut mn = [0u32; 3];
                        let mut mx = [0u32; 3];
                        for k in 0..3usize {
                            let x = f32::from_bits(r.f32(2.0));
                            let y = f32::from_bits(r.f32(2.0));
                            mn[k] = fbits(x.min(y));
                            mx[k] = fbits(x.max(y));
                        }
                        (v, [r.f32(2.0), r.f32(2.0), r.f32(2.0)], mn, mx)
                    }
                };
                c.g[A0][0] = AUX as u64;
                for k in 0..3usize {
                    let off = 4 * k as u32;
                    put(c, AUX + 0x60 + off, o[k]);
                    put(c, AUX + 0x70 + off, d[k]);
                    put(c, AUX + 0x200 + off, lo[k]);
                    put(c, AUX + 0x210 + off, hi[k]);
                }
                c.g[A1][0] = (AUX + 0x200) as u64;
                c.g[A2][0] = (AUX + 0x210) as u64;
                c.g[A3][0] = (AUX + 0x400) as u64;
                fillb(c, &mut r, AUX + 0x400, 0x120);
            },
        );
    }

    // ----------------------------------------------------------------- front end

    #[test]
    fn coll_hull_get_diameter_matches() {
        check(
            ssx3_ee::s11::f_0032f8c0__generated,
            coll_hull_get_diameter,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32f8c0, 0);
                c.g[A0][0] = sx(DATA);
                if seed % 4 == 0 {
                    word(c, DATA, 0);
                } else {
                    word(c, DATA, DATA + 0x100);
                    word(c, DATA + 0x160, DATA + 0x200);
                    word(c, DATA + 0x220, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn coll_sphere_reset_transform_matches() {
        check(
            ssx3_ee::s11::f_0032fd98__generated,
            coll_sphere_reset_transform,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32fd98, 0);
                c.g[A0][0] = sx(DATA);
                for i in 0..5 {
                    word(c, DATA + 0x80 + 4 * i, r.next_u32());
                }
                word(c, DATA + 4, r.next_u32());
                word(c, DATA + 0x60, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_sphere_tree_init_matches() {
        check(
            ee::s10::f_003299c8__generated,
            rider_sphere_tree_init,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_99c8, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = (DATA + 0x400) as u64;
                c.g[T0][0] = (DATA + 0x500) as u64;
                c.g[T1][0] = (seed % 9) as u64;
                c.f[12] = r.edge_f32();
                for i in 0..5u32 {
                    w(c, DATA + 0x400 + 4 * i, r.next_u32());
                    w(c, DATA + 0x500 + 4 * i, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn coll_hull_collide_sphere_tree_matches() {
        check(
            ee::s11::f_0032fa30__generated,
            coll_hull_collide_sphere_tree,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x32_fa30);
                globals(c);
                // A zeroed tree node: the collide call finds nothing.
                const NODE: u32 = DATA + 0x2000;
                w(c, NODE + 0x98, NODE + 0x100);
                w(c, NODE + 0x100 + 0x20, 0);
                w(c, DATA + 0x60, NODE);
                w(c, DATA + 0x80, NODE + 0x100);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = sx(DATA + 0x4000);
            },
        );
    }

    #[test]
    fn coll_sphere_plane_distance_matches() {
        check(
            ee::s11::f_00330360__generated,
            coll_sphere_plane_distance,
            150,
            &|c, seed| {
                let mut r = base(c, seed, 0x33_0360);
                globals(c);
                const PLANE: u32 = DATA + 0x100;
                const PN: u32 = DATA + 0x200;
                const OUT: u32 = DATA + 0x300;
                for i in 0..4u32 {
                    c.bus.write32(PLANE + 4 * i, r.edge_f32());
                    c.bus.write32(PN + 4 * i, r.edge_f32());
                    c.bus.write32(DATA + 0x60 + 4 * i, r.edge_f32());
                }
                c.bus.write32(DATA + 0x70, r.f32(0.01));
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(PLANE);
                c.g[A2][0] = sx(PN);
                c.g[A3][0] = sx(OUT);
            },
        );
    }

    #[test]
    fn sphere_tree_collide_sphere_matches() {
        check(
            ee::s10::f_0032c898__generated,
            sphere_tree_collide_sphere,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32c8, 0);
                env(c);
                let this = DATA + 0x400;
                let tree = DATA + 0x1000;
                w32(c, this + 0x98, tree);
                w32(c, tree + 8, (seed % 3 != 0) as u32);
                for k in 0..4u32 {
                    w32(c, DATA + 0x2000 + 4 * k, r.edge_f32());
                    w32(c, DATA + 0x2100 + 4 * k, r.edge_f32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x2100);
                c.g[A3][0] = sx(DATA + 0x2200);
                c.f[12] = r.f32(8.0);
            },
        );
    }

    #[test]
    fn sphere_tree_collide_aabb_matches() {
        check(
            ee::s10::f_0032c928__generated,
            sphere_tree_collide_aabb,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32c9, 0);
                env(c);
                let this = DATA + 0x400;
                let tree = DATA + 0x1000;
                w32(c, this + 0x98, tree);
                w32(c, tree + 8, (seed % 3 != 0) as u32);
                for k in 0..8u32 {
                    w32(c, DATA + 0x2000 + 4 * k, r.edge_f32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x2100);
                c.g[A3][0] = sx(DATA + 0x2200);
                c.g[T0][0] = sx(DATA + 0x2300);
            },
        );
    }

    #[test]
    fn coll_sphere_collide_sphere_tree_matches() {
        check(
            ee::s11::f_0032fdb0__generated,
            coll_sphere_collide_sphere_tree,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32fd, 0);
                env(c);
                let this = DATA + 0x400;
                let tree = DATA + 0x1000;
                let hit = DATA + 0x2000;
                for k in 0..4u32 {
                    w32(c, this + 0x60 + 4 * k, r.edge_f32());
                    w32(c, hit + 0x10 + 4 * k, r.f32(2.0));
                }
                w32(c, this + 0x70, r.f32(4.0));
                w32(c, tree + 0x98, DATA + 0x3000);
                w32(c, DATA + 0x3000 + 8, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(tree);
                c.g[A2][0] = sx(hit);
            },
        );
    }

    // ------------------------------------------------------------------
    // Collision.

    pub(crate) fn coll_args(c: &mut Cpu, seed: u32, salt: u64, addr: u32, f: GuestFn) {
        let _r = base_2(c, seed, salt, 1);
        let this = DATA + 0x1000;
        c.g[A0][0] = this as u64;
        c.g[A1][0] = (DATA + 0x2000) as u64;
        c.g[A2][0] = (DATA + 0x2100) as u64;
        c.g[A3][0] = (DATA + 0x2200) as u64;
        c.g[T0][0] = (DATA + 0x2300) as u64;
        c.g[T1][0] = (DATA + 0x2400) as u64;
        c.g[T2][0] = (DATA + 0x5000) as u64;
        c.g[T3][0] = (DATA + 0x6000) as u64;
        w_2(c, this + 0x60, addr);
        put_vec(
            c,
            DATA + 0x2400,
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 0],
        );
        w_2(c, addr, 0);
        let _ = f;
    }

    #[test]
    fn coll_hull_collide_triangle2_sided_matches() {
        check(
            ssx3_ee::s11::f_0032f990__generated,
            coll_hull_collide_triangle2_sided,
            20,
            &|c, seed| {
                coll_args(
                    c,
                    seed,
                    0x32f990,
                    DATA + 0x3000,
                    ssx3_ee::s11::f_0032f990__generated,
                );
            },
        );
    }

    #[test]
    fn sphere_tree_collide_sphere_tree_matches() {
        check(
            ssx3_ee::s10::f_0032cb58__generated,
            sphere_tree_collide_sphere_tree,
            20,
            &|c, seed| {
                let _r = base_2(c, seed, 0x32cb58, 2);
                let a = DATA + 0x1000;
                let b = DATA + 0x2000;
                c.g[A0][0] = a as u64;
                c.g[A1][0] = b as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                c.g[A3][0] = (DATA + 0x4000) as u64;
                w_2(c, a + 0x98, DATA + 0x5000);
                w_2(c, b + 0x98, DATA + 0x6000);
                w_2(c, DATA + 0x5000 + 8, (seed & 1) as u32);
                w_2(c, DATA + 0x6000 + 8, ((seed >> 1) & 1) as u32);
                w_2(c, a + 0x80, 0);
            },
        );
    }

    // ------------------------------------------------------------ Collision.

    #[test]
    fn coll_sphere_ctor_matches() {
        check(
            ee::s11::f_0032fb48__generated,
            coll_sphere_ctor,
            200,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x32_fb48);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx((seed % 3) as u32);
                c.f[12] = r.edge_f32();
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x1000 + i * 4, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn spheretree_collide_triangle_matches() {
        check(
            ee::s10::f_0032ca78__generated,
            spheretree_collide_triangle,
            8,
            &|c, seed| {
                let _ = base_4(c, seed, 0x32ca78, 2);
                let this = DATA;
                clear_2(c, this, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (DATA + 0x300) as u64;
                c.g[A3][0] = 0;
                c.g[T0][0] = (seed % 2) as u64;
                c.g[T1][0] = (seed % 3) as u64;
                c.g[T2][0] = (seed % 5) as u64;
                w(c, this + 0x98, if seed % 3 == 0 { 0 } else { DATA + 0x500 });
                clear_2(c, DATA + 0x500, 0x100);
                wf(c, this + 0x94, 0.0);
            },
        );
    }

    #[test]
    fn shell_sort_desc_key_val_matches() {
        strict(
            ee::s11::f_0032da40__generated,
            shell_sort_desc_key_val,
            SEEDS_6,
            &|c, seed| {
                let mut r = base_5(c, seed, 0xda40, 0);
                let keys = DATA + 0x1000;
                let vals = DATA + 0x2000;
                let n = (seed % 7) as u32;
                // the gap table the sort walks down
                for (i, g) in [1u32, 4, 13, 40, 121, 364].iter().enumerate() {
                    w(c, 0x0044_af98 + 4 * i as u32, *g);
                }
                for i in 0..n {
                    let k = if i > 0 && seed % 3 == 0 {
                        i - 1
                    } else {
                        r.next_u32() % 100
                    };
                    w(c, keys + 4 * i, k);
                    w(c, vals + 4 * i, r.next_u32());
                }
                c.g[A0][0] = n as u64;
                c.g[A1][0] = sx(keys);
                c.g[A2][0] = sx(vals);
            },
        );
    }

    pub(crate) fn put3(c: &mut Cpu, addr: u32, v: [u32; 3]) {
        for (i, x) in v.into_iter().enumerate() {
            c.bus.write32(addr + 4 * i as u32, x);
        }
    }

    #[test]
    fn coll_segment_overlaps_aabb_matches() {
        check_all(
            ee::s11::f_0032e288__generated,
            coll_segment_overlaps_aabb,
            300,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x32_e288);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x120);
                match seed % 3 {
                    0 => {
                        // Overlapping: centre 10, half extents 1, box [0, 20].
                        put3(
                            c,
                            DATA + 0x60,
                            [10.0f32.to_bits(), 10.0f32.to_bits(), 10.0f32.to_bits()],
                        );
                        put3(
                            c,
                            DATA + 0x70,
                            [1.0f32.to_bits(), 1.0f32.to_bits(), 1.0f32.to_bits()],
                        );
                        put3(c, DATA + 0x100, [0; 3]);
                        put3(c, DATA + 0x120, [20.0f32.to_bits(); 3]);
                    }
                    1 => {
                        // Separated on the first axis.
                        put3(c, DATA + 0x60, [10.0f32.to_bits(); 3]);
                        put3(c, DATA + 0x70, [1.0f32.to_bits(); 3]);
                        put3(c, DATA + 0x100, [100.0f32.to_bits(); 3]);
                        put3(c, DATA + 0x120, [200.0f32.to_bits(); 3]);
                    }
                    _ => {
                        floats(c, &mut r, DATA + 0x60, 8);
                        floats(c, &mut r, DATA + 0x100, 3);
                        floats(c, &mut r, DATA + 0x120, 3);
                    }
                }
                // The VU0 sum is stored to the frame first.
                floats(c, &mut r, DATA + 0x80, 8);
            },
        );
    }

    #[test]
    fn coll_sphere_hits_triangle_edges_matches() {
        check_all(
            ee::s11::f_00330250__generated,
            coll_sphere_hits_triangle_edges,
            200,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x33_0250);
                c.g[A0][0] = sx(DATA); // sphere: centre +0x60, radius +0x70
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x120);
                c.g[A3][0] = sx(DATA + 0x140);
                match seed % 3 {
                    0 => {
                        // Tiny radius: every edge is tested.
                        put3(c, DATA + 0x60, [0; 3]);
                        c.bus.write32(DATA + 0x70, 0.001f32.to_bits());
                    }
                    1 => {
                        // Huge radius: the first edge already hits.
                        put3(c, DATA + 0x60, [0; 3]);
                        c.bus.write32(DATA + 0x70, 1.0e6f32.to_bits());
                    }
                    _ => {
                        floats(c, &mut r, DATA + 0x60, 4);
                        c.bus.write32(DATA + 0x70, r.edge_f32());
                    }
                }
                for base_off in [0x100u32, 0x120, 0x140] {
                    match seed % 3 {
                        0 => put3(
                            c,
                            DATA + base_off,
                            [
                                (base_off as f32).to_bits(),
                                2.0f32.to_bits(),
                                3.0f32.to_bits(),
                            ],
                        ),
                        _ => floats(c, &mut r, DATA + base_off, 4),
                    }
                }
            },
        );
    }

    #[test]
    fn coll_sphere_overlaps_aabb_matches() {
        same(
            ssx3_ee::s11::f_0032fc08__generated,
            coll_sphere_overlaps_aabb,
            200,
            &|c, seed| {
                let mut r = Rng(0x32fc08 ^ seed as u64);
                scramble(c, &mut r);
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (DATA + 0x240) as u64;
                put_vec(
                    c,
                    s + 0x60,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                c.bus.write32(s + 0x70, r.edge_f32());
                for i in 0..3 {
                    c.bus.write32(DATA + 0x200 + 4 * i, r.edge_f32());
                    c.bus.write32(DATA + 0x240 + 4 * i, r.edge_f32());
                }
            },
        );
    }

    /// RiderSphereTree_CollideTriangle: the triangle (0,0,0), (4,0,0),
    /// (0,4,0) with normal +z; the root sphere and up to four kids at random
    /// places around it (above/below the plane, near the vertices and
    /// edges, or far), random masks. `VU0_TriangleIntersect` runs a stub
    /// microprogram at 0x6e0 (`vi2 = vi14`), so its answer is chosen per
    /// seed through vi14.
    #[test]
    fn rider_sphere_tree_collide_triangle_matches() {
        check(
            ee::s10::f_0032a1c0__generated,
            rider_sphere_tree_collide_triangle,
            300,
            &|c, seed| {
                let mut r = Rng(0x32a1_c0 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                c.pc = 0x0032_a1c0;
                c.g[GP][0] = GAME_GP as u64;
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .hw
                    .vu0
                    .fallback = Some(ssx3_ps2::vu::reference::run_program);
                c.bus.write32(0x1100_0000 + 0x6e0, 0x8000_70b0); // IADD vi2, vi14, vi0
                c.bus.write32(0x1100_0000 + 0x6e4, 0x4000_02ff); // NOP, E
                c.g[14][0] = if seed % 4 == 0 { 1 } else { 0 };
                c.ctc2(14, 14);
                c.bus.write32(GAME_GP - 0x2de0, 0.001f32.to_bits());
                c.bus.write32(GAME_GP - 0x2dd8, 0.001f32.to_bits());
                c.bus.write32(GAME_GP - 0x2ddc, 1.0e10f32.to_bits());
                c.bus.write32(GAME_GP - 0x2dd4, 1.0e10f32.to_bits());
                let tree = DATA;
                let (a, b, cc, n) = (DATA + 0x400, DATA + 0x410, DATA + 0x420, DATA + 0x430);
                put_vec(c, a, [f(0.0), f(0.0), f(0.0), 0]);
                put_vec(c, b, [f(4.0), f(0.0), f(0.0), 0]);
                put_vec(c, cc, [f(0.0), f(4.0), f(0.0), 0]);
                put_vec(c, n, [f(0.0), f(0.0), f(1.0), 0]);
                let place = |c: &mut Cpu, r: &mut Rng, at: u32| {
                    let p = match r.next_u32() % 6 {
                        0 => [-0.5, -0.5],
                        1 => [2.0, -0.6],
                        2 => [-0.6, 2.0],
                        3 => [2.3, 2.3],
                        4 => [1.0, 1.0],
                        _ => [9.0, 9.0],
                    };
                    let jitter = |r: &mut Rng| (r.next_u32() % 100) as f32 * 0.004 - 0.2;
                    let z = (r.next_u32() % 100) as f32 * 0.03 - 1.5;
                    put_vec(c, at, [f(p[0] + jitter(r)), f(p[1] + jitter(r)), f(z), 0]);
                    c.bus
                        .write32(at + 0x10, f(0.3 + (r.next_u32() % 100) as f32 * 0.012));
                };
                place(c, &mut r, tree + 0x10);
                let count = r.next_u32() % 5;
                for k in 0..count {
                    place(c, &mut r, tree + 0x30 + 0x20 * k);
                }
                c.bus.write32(
                    tree + 0x28,
                    if seed % 3 == 0 { 0 } else { r.next_u32() & 0xf },
                );
                c.bus.write32(tree + 0x2c, count);
                c.g[A0][0] = sx(tree);
                c.g[A1][0] = sx(a);
                c.g[A2][0] = sx(b);
                c.g[A3][0] = sx(cc);
                c.g[T0][0] = sx(n);
                c.g[T1][0] = sx(DATA + 0x500);
                c.g[T2][0] = sx(DATA + 0x510);
            },
        );
    }

    /// RiderSphereTree_CollideTriangle2: the triangle p (0,0,0), a (4,0,0),
    /// b (0,4,0) with normal +z or -z; the root sphere and up to four kids
    /// at random places around it (above/below the plane, near the vertices
    /// and edges, or far), random masks. `VU0_TriangleIntersect` runs a stub
    /// microprogram at 0x6e0 (`vi2 = vi14`), so its answer is chosen per
    /// seed through vi14.
    #[test]
    fn rider_sphere_tree_collide_triangle2_matches() {
        use crate::difftest::put_vec;
        let f = |x: f32| x.to_bits();
        check(
            ee::s10::f_0032aa28__generated,
            rider_sphere_tree_collide_triangle2,
            300,
            &|c, seed| {
                let mut r = Rng(0x32_aa28 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                c.pc = 0x0032_aa28;
                c.g[GP][0] = GAME_GP as u64;
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .hw
                    .vu0
                    .fallback = Some(ssx3_ps2::vu::reference::run_program);
                c.bus.write32(0x1100_0000 + 0x6e0, 0x8000_70b0); // IADD vi2, vi14, vi0
                c.bus.write32(0x1100_0000 + 0x6e4, 0x4000_02ff); // NOP, E
                c.g[14][0] = if seed % 4 == 0 { 1 } else { 0 };
                c.ctc2(14, 14);
                c.bus.write32(GAME_GP - 0x2dd0, f(0.0001));
                c.bus.write32(GAME_GP - 0x2dc8, f(0.0001));
                c.bus.write32(GAME_GP - 0x2dcc, f(1.0e10));
                c.bus.write32(GAME_GP - 0x2dc4, f(1.0e10));
                let tree = DATA;
                let (p, a, b, n) = (DATA + 0x400, DATA + 0x410, DATA + 0x420, DATA + 0x430);
                put_vec(c, p, [f(0.0), f(0.0), f(0.0), 0]);
                put_vec(c, a, [f(4.0), f(0.0), f(0.0), 0]);
                put_vec(c, b, [f(0.0), f(4.0), f(0.0), 0]);
                let nz = if seed % 5 == 1 { -1.0 } else { 1.0 };
                put_vec(c, n, [f(0.0), f(0.0), f(nz), 0]);
                let place = |c: &mut Cpu, r: &mut Rng, at: u32| {
                    let q = match r.next_u32() % 7 {
                        0 => [-0.5, -0.5],
                        1 => [2.0, -0.6],
                        2 => [-0.6, 2.0],
                        3 => [2.3, 2.3],
                        4 => [1.0, 1.0],
                        5 => [4.4, 0.2],
                        _ => [9.0, 9.0],
                    };
                    let jitter = |r: &mut Rng| (r.next_u32() % 100) as f32 * 0.004 - 0.2;
                    let z = (r.next_u32() % 100) as f32 * 0.03 - 1.5;
                    put_vec(c, at, [f(q[0] + jitter(r)), f(q[1] + jitter(r)), f(z), 0]);
                    c.bus
                        .write32(at + 0x10, f(0.3 + (r.next_u32() % 100) as f32 * 0.012));
                };
                place(c, &mut r, tree + 0x10);
                let count = r.next_u32() % 5;
                for k in 0..count {
                    place(c, &mut r, tree + 0x30 + 0x20 * k);
                }
                let mask = if seed % 3 == 0 { 0 } else { r.next_u32() & 0xf };
                c.bus.write32(tree + 0x28, mask);
                c.bus.write32(tree + 0x2c, count);
                c.g[A0][0] = sx(tree);
                c.g[A1][0] = sx(p);
                c.g[A2][0] = sx(a);
                c.g[A3][0] = sx(b);
                c.g[T0][0] = sx(n);
                c.g[T1][0] = sx(DATA + 0x500);
                c.g[T2][0] = sx(DATA + 0x510);
            },
        );
    }

    /// RiderSphereTree_CollideBox: boxes around the origin, sphere centres
    /// mostly inside (hits) and sometimes outside (misses), radii 0..5; no
    /// sub-sphere mask (first hit), or masks with counts -1..6 (the best of
    /// the repeated sweeps, or none); the real Collide_SphereVsBoxEdges with
    /// its axis vectors (.bss) and 1e30 limits (.lit4).
    #[test]
    fn rider_sphere_tree_collide_box_matches() {
        const BOX: u32 = DATA + 0x2000;
        const SPHERE: u32 = DATA + 0x3000;
        const OUT_POS: u32 = DATA + 0x4000;
        const OUT_NORMAL: u32 = DATA + 0x4100;
        check(
            ee::s10::f_0032b2b8__generated,
            rider_sphere_tree_collide_box,
            300,
            &|c, seed| {
                let mut r = Rng(0x32_b2b8 ^ (seed as u64) << 24);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x0032_b2b8;
                put_words(c, GAME_GP - 0x2df0, &[0x7149_f2ca, 0x7149_f2ca]);
                let one = 1.0f32.to_bits();
                put_vec(c, 0x004f_f140, [one, 0, 0, 0]);
                put_vec(c, 0x004f_f150, [0, one, 0, 0]);
                put_vec(c, 0x004f_f160, [0, 0, one, 0]);
                let coord = |r: &mut Rng, lo: f32, hi: f32| {
                    (lo + (r.next_u32() % 1000) as f32 * (hi - lo) / 1000.0).to_bits()
                };
                let outside = seed % 7 == 0;
                for i in 0..3 {
                    c.bus.write32(BOX + 4 * i, coord(&mut r, -10.0, 0.0));
                    c.bus.write32(BOX + 0x10 + 4 * i, coord(&mut r, 0.0, 10.0));
                    let centre = if outside {
                        coord(&mut r, 20.0, 30.0)
                    } else {
                        coord(&mut r, -9.0, 9.0)
                    };
                    c.bus.write32(SPHERE + RST_CENTRE + 4 * i, centre);
                }
                c.bus.write32(SPHERE + RST_CENTRE + 0xc, one);
                c.bus.write32(SPHERE + RST_RADIUS, coord(&mut r, 0.0, 5.0));
                let (mask, count) = if seed % 3 == 0 {
                    (0, r.next_u32())
                } else {
                    let count = (r.next_u32() % 8).wrapping_sub(1);
                    (r.next_u32() & 0x7f, count)
                };
                c.bus.write32(SPHERE + RST_MASK, mask);
                c.bus.write32(SPHERE + RST_COUNT, count);
                fill(c, &mut r, OUT_POS, 4);
                fill(c, &mut r, OUT_NORMAL, 4);
                c.g[A0][0] = sx(SPHERE);
                c.g[A1][0] = sx(BOX);
                c.g[A2][0] = sx(BOX + 0x10);
                c.g[A3][0] = sx(OUT_POS);
                c.g[T0][0] = sx(OUT_NORMAL);
            },
        );
    }

    // --------------------------------------------------- RiderSphereTree_Scale

    /// `RiderSphereTree_Scale`: random matrices, scales (edge cases too) and
    /// trees with no, a negative count of, and up to six children.
    #[test]
    fn rider_sphere_tree_scale_matches() {
        use crate::difftest::{check, fill, scramble, Rng, DATA};
        check(
            ee::s10::f_00329b90__generated,
            rider_sphere_tree_scale,
            300,
            &|c, seed| {
                let mut r = Rng(0x0032_9b90 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0032_9b90;
                let tree = DATA;
                let m = DATA + 0x1000;
                c.g[A0][0] = sx(tree);
                c.g[A1][0] = sx(m);
                c.f[12] = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(4.0)
                };
                for i in 0..16 {
                    c.bus.write32(
                        m + 4 * i,
                        if seed % 9 == 0 {
                            r.edge_f32()
                        } else {
                            r.f32(100.0)
                        },
                    );
                }
                fill(c, &mut r, tree, 0x30 + 8 * 8);
                for i in 0..4 + 8 * 4 {
                    let a = tree + TREE_ROOT_CENTER + 4 * i;
                    if a != tree + TREE_CHILD_COUNT {
                        c.bus.write32(a, r.f32(500.0));
                    }
                }
                let count = match seed % 6 {
                    0 => 0,
                    1 => (-(r.next_u32() as i32 & 0xff) - 1) as u32,
                    _ => 1 + r.next_u32() % 6,
                };
                c.bus.write32(tree + TREE_CHILD_COUNT, count);
            },
        );
    }

    /// A collider and a leaf mesh of 0..25 triangles (small meshes and boxed
    /// ones, with a partial last box). The triangle and box tests are fakes
    /// (hits; some seeds unwind) or a real leaf returning 0 (misses), chosen
    /// per slot; back-face skipping on or off with random normals.
    #[test]
    fn collide_mesh_leaf_triangles_matches() {
        use crate::difftest::fakes::{covered, FAKE};
        use leaf::*;
        const MISS: u32 = 0x0014_06c0;
        covered(
            ssx3_ee::s10::f_0032c0f8__generated,
            collide_mesh_leaf_triangles,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32c0f8, 30);
                c.pc = 0x0032_c0f8;
                let this = DATA;
                let vt = DATA + 0x100;
                let mesh = DATA + 0x200;
                let indices = DATA + 0x400;
                let boxes = DATA + 0x600;
                let verts = DATA + 0x800;
                let normals = DATA + 0xa00;
                let out = DATA + 0x1000;
                c.bus.write32(this + USE_NORMAL, (seed % 3 != 0) as u32);
                put_vec(c, this + DIR, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                c.bus.write32(this + VTABLE, vt);
                for (i, slot) in [VT_TRI, VT_TRI_SWEPT, VT_BOX].into_iter().enumerate() {
                    c.bus
                        .write16(vt + slot, ((r.next_u32() as i16) & !3) as u16);
                    let f = if r.next_u32() % 4 == 0 {
                        MISS
                    } else {
                        FAKE + 0x10 * i as u32
                    };
                    c.bus.write32(vt + slot + 4, f);
                }
                let tris = match seed % 4 {
                    0 => r.next_u32() % 11,
                    1 => 11 + r.next_u32() % 15,
                    2 => 20,
                    _ => r.next_u32() % 26,
                };
                c.bus.write16(mesh + NUM_TRIS, tris as u16);
                c.bus.write32(mesh + INDICES, indices);
                c.bus.write32(mesh + BOXES, boxes);
                c.bus.write32(mesh + VERTS, verts);
                c.bus.write32(mesh + NORMALS, normals);
                for k in 0..3 * 25 {
                    c.bus.write8(indices + k, (r.next_u32() % 16) as u8);
                }
                for k in 0..25 {
                    put_vec(c, normals + 16 * k, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                }
                c.bus.write32(GAME_GP - GP_FACING_SMALL, r.f32(1.0));
                c.bus.write32(GAME_GP - GP_FACING_BOXED, r.f32(1.0));
                c.g[A0][0] = this as u64;
                c.g[A1][0] = mesh as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A3][0] = out as u64;
            },
        );
    }

    /// `CollHull_CollideTriangle`: a single sphere (no child mask) over a
    /// triangle in the y = 0 plane, above/inside it (contact: push length
    /// and direction written) or too far or outside (no contact); the real
    /// sphere-tree test runs with the game's constants.
    #[test]
    fn coll_hull_collide_triangle_matches() {
        use crate::difftest::{elf_image, put_vec};
        const HULL: u32 = DATA;
        const TREE: u32 = DATA + 0x100;
        const P: u32 = DATA + 0x200;
        const A: u32 = DATA + 0x210;
        const B: u32 = DATA + 0x220;
        const N: u32 = DATA + 0x230;
        const POINT: u32 = DATA + 0x300;
        const LEN: u32 = DATA + 0x400;
        const DIR: u32 = DATA + 0x500;
        let f = |x: f32| x.to_bits();
        check(
            ee::s11::f_0032f8f0__generated,
            coll_hull_collide_triangle,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_f8f0, 0);
                elf_image(c);
                c.pc = 0x0032_f8f0;
                for i in 0..0x40 {
                    c.bus.write32(TREE + 4 * i, 0);
                }
                c.bus.write32(HULL + 0x60, TREE);
                let (x, z) = match seed % 4 {
                    3 => (8.0, 8.0), // outside the triangle
                    _ => (
                        1.0 + (r.next_u32() % 4) as f32,
                        1.0 + (r.next_u32() % 4) as f32,
                    ),
                };
                let y = match seed % 5 {
                    0 => 3.0, // above the sphere's reach
                    1 => -0.5,
                    _ => (r.next_u32() % 100) as f32 / 120.0,
                };
                put_vec(c, TREE + 0x10, [f(x), f(y), f(z), f(1.0)]);
                c.bus.write32(TREE + 0x20, f(1.0));
                put_vec(c, P, [0, 0, 0, f(1.0)]);
                put_vec(c, A, [f(10.0), 0, 0, f(1.0)]);
                put_vec(c, B, [0, 0, f(10.0), f(1.0)]);
                put_vec(c, N, [0, f(1.0), 0, 0]);
                c.g[A0][0] = sx(HULL);
                c.g[A1][0] = sx(P);
                c.g[A2][0] = sx(A);
                c.g[A3][0] = sx(B);
                c.g[T0][0] = sx(N);
                c.g[T1][0] = sx(POINT);
                c.g[T2][0] = sx(DIR);
                c.g[T3][0] = sx(LEN);
            },
        );
    }

    // (fakes imported above)

    /// `CollHull_Transform` with the real `RiderSphereTree_Assign` (and its
    /// memcpy) and `RiderSphereTree_Scale`: source trees of 0..5 child
    /// spheres, random matrices and scales.
    #[test]
    fn coll_hull_transform_matches() {
        use crate::difftest::fill;
        check(
            ee::s11::f_0032f840__generated,
            coll_hull_transform,
            200,
            &|c, seed| {
                let mut r = Rng(0x0032_f840 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let hull = DATA;
                let matrix = DATA + 0x1000;
                let source = DATA + 0x2000;
                c.g[A0][0] = sx(hull);
                c.g[A1][0] = sx(matrix);
                c.f[12] = if seed % 7 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(4.0)
                };
                for i in 0..16 {
                    c.bus.write32(matrix + 4 * i, r.f32(100.0));
                }
                fill(c, &mut r, hull, 0x70 + 0x30 + 6 * 0x20);
                c.bus.write32(hull + HULL_SOURCE_TREE, source);
                fill(c, &mut r, source, 0x30);
                // Centre, radius and children: floats; the child count small.
                for i in 0..4 + 6 * 8 {
                    let a = source + 0x10 + 4 * i;
                    if a != source + 0x2c {
                        c.bus.write32(a, r.f32(500.0));
                    }
                }
                c.bus.write32(source + 0x2c, seed % 6);
                c.pc = 0x0032_f840;
            },
        );
    }

    /// RiderSphereTree_Assign: random headers with 0..5 children, copied
    /// by the real EA_MemCopy.
    #[test]
    fn rider_sphere_tree_assign_matches() {
        check(
            ee::s10::f_00329a28__generated,
            rider_sphere_tree_assign,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x329a_28, 0);
                c.pc = 0x0032_9a28;
                let this = DATA + 0x1000;
                let src = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(src);
                fill(c, &mut r, this, 0xd0 / 4);
                fill(c, &mut r, src, 0xd0 / 4);
                c.bus.write32(src + OC264_RST_COUNT, seed % 6);
            },
        );
    }

    // ------------------------------------------------------- CollHull_OverlapsAABB

    /// A hull's bounding sphere against a box that overlaps it, with one of
    /// the six axis tests made to fail (`seed % 7`; 6 = overlap), plus some
    /// edge-case floats.
    #[test]
    fn coll_hull_overlaps_aabb_matches() {
        check(
            ee::s11::f_0032f760__generated,
            coll_hull_overlaps_aabb,
            280,
            &|c, seed| {
                let mut r = Rng(0x0032_f760 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.pc = 0x0032_f760;
                let hull = DATA;
                let sphere = DATA + 0x100;
                let min = DATA + 0x200;
                let max = DATA + 0x300;
                fill(c, &mut r, hull, 0x20);
                c.bus.write32(hull + HULL_SPHERE, sphere);
                let center = [
                    (r.next_u32() % 16) as f32 - 8.0,
                    (r.next_u32() % 16) as f32 - 8.0,
                    (r.next_u32() % 16) as f32 - 8.0,
                    f32::from_bits(r.f32(4.0)),
                ];
                let rad = (r.next_u32() % 4) as f32 + 0.5;
                put_words(c, sphere + HULL_SPHERE_CENTER, &center.map(f32::to_bits));
                c.bus.write32(sphere + HULL_SPHERE_RADIUS, rad.to_bits());
                let mut lo = [0f32; 3];
                let mut hi = [0f32; 3];
                let mut mn = [0f32; 3];
                let mut mx = [0f32; 3];
                for a in 0..3 {
                    lo[a] = center[a] - rad;
                    hi[a] = center[a] + rad;
                    // Touching exactly on some seeds (the `<=`).
                    mn[a] = if seed % 5 == 0 {
                        hi[a]
                    } else {
                        center[a] - 0.25
                    };
                    mx[a] = if seed % 5 == 1 {
                        lo[a]
                    } else {
                        center[a] + 0.25
                    };
                }
                let fail = seed % 7;
                if fail < 6 {
                    let a = (fail / 2) as usize;
                    if fail % 2 == 0 {
                        mx[a] = lo[a] - 1.0;
                    } else {
                        mn[a] = hi[a] + 1.0;
                    }
                }
                let mut put = |c: &mut Cpu, at: u32, v: [f32; 3]| {
                    put_words(c, at, &v.map(f32::to_bits));
                    c.bus.write32(at + 12, r.next_u32());
                };
                put(c, min, mn);
                put(c, max, mx);
                if seed % 11 == 10 {
                    let mut r2 = Rng(seed as u64 * 77 + 1);
                    for i in 0..3 {
                        c.bus.write32(min + 4 * i, r2.edge_f32());
                        c.bus.write32(max + 4 * i, r2.edge_f32());
                    }
                    c.bus.write32(sphere + HULL_SPHERE_RADIUS, r2.edge_f32());
                }
                c.g[A0][0] = sx(hull);
                c.g[A1][0] = sx(min);
                c.g[A2][0] = sx(max);
            },
        );
    }

    /// CollHull_CollideAABB with the real RiderSphereTree_CollideBox (the
    /// fixture of its own test): boxes around the origin, the hull's sphere
    /// inside (hit: length and normalised direction) or outside (miss),
    /// with or without a sub-sphere mask.
    #[test]
    fn coll_hull_collide_aabb_matches() {
        use crate::difftest::{fill, put_words, scramble, Rng};
        const HULL: u32 = DATA + 0x1000;
        const BOX: u32 = DATA + 0x2000;
        const SPHERE: u32 = DATA + 0x3000;
        const HIT: u32 = DATA + 0x4000;
        crate::difftest::check(
            ee::s11::f_0032fac0__generated,
            coll_hull_collide_aabb,
            300,
            &|c, seed| {
                let mut r = Rng(0x32_fac0 ^ (seed as u64) << 24);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x0032_fac0;
                put_words(c, GAME_GP - 0x2df0, &[0x7149_f2ca, 0x7149_f2ca]);
                let one = 1.0f32.to_bits();
                put_vec(c, 0x004f_f140, [one, 0, 0, 0]);
                put_vec(c, 0x004f_f150, [0, one, 0, 0]);
                put_vec(c, 0x004f_f160, [0, 0, one, 0]);
                let coord = |r: &mut Rng, lo: f32, hi: f32| {
                    (lo + (r.next_u32() % 1000) as f32 * (hi - lo) / 1000.0).to_bits()
                };
                let outside = seed % 5 == 0;
                for i in 0..3 {
                    c.bus.write32(BOX + 4 * i, coord(&mut r, -10.0, 0.0));
                    c.bus.write32(BOX + 0x10 + 4 * i, coord(&mut r, 0.0, 10.0));
                    let centre = if outside {
                        coord(&mut r, 20.0, 30.0)
                    } else {
                        coord(&mut r, -9.0, 9.0)
                    };
                    c.bus.write32(SPHERE + 0x10 + 4 * i, centre);
                }
                c.bus.write32(SPHERE + 0x1c, one);
                c.bus.write32(SPHERE + 0x20, coord(&mut r, 0.0, 5.0));
                let (mask, count) = if seed % 3 == 0 {
                    (0, r.next_u32())
                } else {
                    ((r.next_u32() & 0x7f), (r.next_u32() % 8).wrapping_sub(1))
                };
                c.bus.write32(SPHERE + 0x28, mask);
                c.bus.write32(SPHERE + 0x2c, count);
                fill(c, &mut r, HIT, 0x14);
                c.bus.write32(HULL + COLL_HULL_SPHERE, SPHERE);
                c.g[A0][0] = sx(HULL);
                c.g[A1][0] = sx(BOX);
                c.g[A2][0] = sx(BOX + 0x10);
                c.g[A3][0] = sx(HIT);
            },
        );
    }

    /// A node with 0..6 children (sometimes a negative count) translated by
    /// a random vector.
    #[test]
    fn rider_sphere_tree_translate_matches() {
        check(
            ee::s10::f_00329b40__generated,
            rider_sphere_tree_translate,
            300,
            &|c, seed| {
                let mut r = Rng(0x329b_40a5 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0032_9b40;
                let node = DATA;
                let v = DATA + 0x8000;
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(v);
                fill(c, &mut r, node, 0x30 / 4 + 6 * 8);
                let count = match seed % 8 {
                    0 => 0,
                    1 => (r.next_u32() | 0x8000_0000) as i32,
                    k => (k - 2) as i32,
                };
                c.bus.write32(node + sphere_tree_f::COUNT, count as u32);
                put_vec(
                    c,
                    node + sphere_tree_f::CENTER,
                    [r.f32(1e4), r.f32(1e4), r.f32(1e4), r.edge_f32()],
                );
                for i in 0..6 {
                    let at = node + sphere_tree_f::CHILDREN + i * sphere_tree_f::CHILD_SIZE;
                    put_vec(c, at, [r.f32(1e4), r.f32(1e4), r.f32(1e4), r.edge_f32()]);
                }
                put_vec(
                    c,
                    v,
                    [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.edge_f32()],
                );
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::RA as SENTINEL_m2;
    pub(crate) use crate::game::be::beintaggression::be_aggression_step;
    use std::ptr::addr_of_mut;

    /// Fakes are live only in machines that hold `OC430_MAGIC` at `OC430_CTRL`.
    pub(crate) const OC430_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC430_MAGIC: u32 = 0x4f43_4330;

    /// Result a fake callee returns instead of the hashed pointer, where the
    /// function under test needs a usable value (call index `n`, `a0` as set by
    /// the caller).
    pub(crate) fn oc430_result(addr: u32, a0: u64, n: u32) -> Option<u64> {
        match addr {
            0x0031_7d70 => Some((DATA + 0x1_0000 + 0x1000 * (n & 15)) as u64),
            0x0031_7e30 => Some((DATA + 0x6000 + 0x800 * (n & 7)) as u64),
            0x0039_7b08 => Some(a0),
            0x0014_a0e0 => Some(a0 % 3),
            0x0014_a080 => Some((a0 * 3 + 1) % 10),
            _ => None,
        }
    }

    /// One logged fake call (`fakes::fake`), with `oc430_result` applied.
    pub(crate) fn oc430_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let a0 = c.g[A0][0];
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake(c, addr, ra);
        if c.pc == ra {
            if let Some(v) = oc430_result(addr, a0, n) {
                c.g[V0][0] = v;
            }
        }
    }

    macro_rules! oc430_fakes {
        ($(($name:ident, $addr:expr, $shard:ident, $slot:ident, $gen:ident)),* $(,)?) => {
            $(
                pub(crate) fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC430_CTRL) != OC430_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), ee::$shard::$gen);
                    }
                    oc430_fake_call(c, $addr);
                }
            )*

            /// Hooks the gated fakes into the callees' override slots (once per
            /// process; with the gate off they run what the slots held).
            pub(crate) fn oc430_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc430_fakes! {
        (oc430_fake_317d70, 0x0031_7d70, s10, OVERRIDE_00317D70, f_00317d70__generated),
        (oc430_fake_3554b0, 0x0035_54b0, s11, OVERRIDE_003554B0, f_003554b0__generated),
        (oc430_fake_359f88, 0x0035_9f88, s11, OVERRIDE_00359F88, f_00359f88__generated),
        (oc430_fake_31d790, 0x0031_d790, s10, OVERRIDE_0031D790, f_0031d790__generated),
        (oc430_fake_317e50, 0x0031_7e50, s10, OVERRIDE_00317E50, f_00317e50__generated),
        (oc430_fake_317e98, 0x0031_7e98, s10, OVERRIDE_00317E98, f_00317e98__generated),
        (oc430_fake_274918, 0x0027_4918, s07, OVERRIDE_00274918, f_00274918__generated),
        (oc430_fake_273a00, 0x0027_3a00, s07, OVERRIDE_00273A00, f_00273a00__generated),
        (oc430_fake_273970, 0x0027_3970, s07, OVERRIDE_00273970, f_00273970__generated),
        (oc430_fake_2724e8, 0x0027_24e8, s07, OVERRIDE_002724E8, f_002724e8__generated),
        (oc430_fake_152ba8, 0x0015_2ba8, s01, OVERRIDE_00152BA8, f_00152ba8__generated),
        (oc430_fake_416ce0, 0x0041_6ce0, s15, OVERRIDE_00416CE0, f_00416ce0__generated),
        (oc430_fake_145970, 0x0014_5970, s01, OVERRIDE_00145970, f_00145970__generated),
        (oc430_fake_147f78, 0x0014_7f78, s01, OVERRIDE_00147F78, f_00147f78__generated),
        (oc430_fake_14f900, 0x0014_f900, s01, OVERRIDE_0014F900, f_0014f900__generated),
        (oc430_fake_1515b8, 0x0015_15b8, s01, OVERRIDE_001515B8, f_001515b8__generated),
        (oc430_fake_14a0e0, 0x0014_a0e0, s01, OVERRIDE_0014A0E0, f_0014a0e0__generated),
        (oc430_fake_14a080, 0x0014_a080, s01, OVERRIDE_0014A080, f_0014a080__generated),
        (oc430_fake_317e30, 0x0031_7e30, s10, OVERRIDE_00317E30, f_00317e30__generated),
        (oc430_fake_283298, 0x0028_3298, s07, OVERRIDE_00283298, f_00283298__generated),
        (oc430_fake_275d10, 0x0027_5d10, s07, OVERRIDE_00275D10, f_00275d10__generated),
        (oc430_fake_120d58, 0x0012_0d58, s00, OVERRIDE_00120D58, f_00120d58__generated),
        (oc430_fake_125958, 0x0012_5958, s00, OVERRIDE_00125958, f_00125958__generated),
        (oc430_fake_14f7a8, 0x0014_f7a8, s01, OVERRIDE_0014F7A8, f_0014f7a8__generated),
        (oc430_fake_14dd58, 0x0014_dd58, s01, OVERRIDE_0014DD58, f_0014dd58__generated),
        (oc430_fake_117520, 0x0011_7520, s00, OVERRIDE_00117520, f_00117520__generated),
        (oc430_fake_397b08, 0x0039_7b08, s13, OVERRIDE_00397B08, f_00397b08__generated),
        (oc430_fake_397c40, 0x0039_7c40, s13, OVERRIDE_00397C40, f_00397c40__generated),
        (oc430_fake_39e2a0, 0x0039_e2a0, s13, OVERRIDE_0039E2A0, f_0039e2a0__generated),
        (oc430_fake_39f400, 0x0039_f400, s13, OVERRIDE_0039F400, f_0039f400__generated),
        (oc430_fake_14dc80, 0x0014_dc80, s01, OVERRIDE_0014DC80, f_0014dc80__generated),
        (oc430_fake_231cd0, 0x0023_1cd0, s06, OVERRIDE_00231CD0, f_00231cd0__generated),
        (oc430_fake_233c10, 0x0023_3c10, s06, OVERRIDE_00233C10, f_00233c10__generated),
        (oc430_fake_234188, 0x0023_4188, s06, OVERRIDE_00234188, f_00234188__generated),
        (oc430_fake_234ef8, 0x0023_4ef8, s06, OVERRIDE_00234EF8, f_00234ef8__generated),
        (oc430_fake_236208, 0x0023_6208, s06, OVERRIDE_00236208, f_00236208__generated),
        (oc430_fake_236868, 0x0023_6868, s06, OVERRIDE_00236868, f_00236868__generated),
        (oc430_fake_244240, 0x0024_4240, s06, OVERRIDE_00244240, f_00244240__generated),
        (oc430_fake_10fcd8, 0x0010_fcd8, s00, OVERRIDE_0010FCD8, f_0010fcd8__generated),
        (oc430_fake_11b698, 0x0011_b698, s00, OVERRIDE_0011B698, f_0011b698__generated),
        (oc430_fake_270070, 0x0027_0070, s07, OVERRIDE_00270070, f_00270070__generated),
        (oc430_fake_26afb8, 0x0026_afb8, s07, OVERRIDE_0026AFB8, f_0026afb8__generated),
        (oc430_fake_26a428, 0x0026_a428, s07, OVERRIDE_0026A428, f_0026a428__generated),
        (oc430_fake_26ac48, 0x0026_ac48, s07, OVERRIDE_0026AC48, f_0026ac48__generated),
        (oc430_fake_269f18, 0x0026_9f18, s07, OVERRIDE_00269F18, f_00269f18__generated),
        (oc430_fake_26a638, 0x0026_a638, s07, OVERRIDE_0026A638, f_0026a638__generated),
    }

    /// The aggression state machine against a plain restatement of the
    /// C++ switch (state / level / points transitions).
    #[test]
    fn be_aggression_step_matches_switch() {
        for st in -2..6i64 {
            for lvl in 0..8i64 {
                let (_, s, l, p) = be_aggression_step(st, lvl, 7);
                let (mut es, mut el, mut ep) = (st, lvl, 7u64);
                match st {
                    0 if lvl >= 3 => (el, ep) = (2, 10),
                    1 if lvl >= 4 => (el, ep, es) = (2, 10, 3),
                    2 if lvl <= 0 => el = 1,
                    2 if lvl >= 4 => (el, ep) = (3, 15),
                    3 if lvl == 0 => (es, el, ep) = (1, 3, 15),
                    3 if lvl >= 5 => (el, ep) = (4, 20),
                    _ => {}
                }
                assert_eq!((s, l, p), (es, el, ep), "st {st} lvl {lvl}");
            }
        }
    }

    /// `World_InitSphereTreeCache`: 0..3 small and big buffers, depths -1..3
    /// (a negative depth leaves the size 0); the allocator is a gated fake.
    #[test]
    fn world_init_sphere_tree_cache_matches() {
        oc430_install_fakes();
        covered(
            ee::s11::f_0032dc10__generated,
            world_init_sphere_tree_cache,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_dc10, 10);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0032_dc10;
                c.g[A0][0] = sx(r.next_u32() % 4);
                c.g[A1][0] = sx((r.next_u32() % 5).wrapping_sub(1));
                c.g[A2][0] = sx(r.next_u32() % 4);
                c.g[A3][0] = sx((r.next_u32() % 5).wrapping_sub(1));
            },
        );
    }

    /// The same with event delivery at the four loop back edges.
    #[test]
    fn world_init_sphere_tree_cache_events_match() {
        oc430_install_fakes();
        check_events(
            ee::s11::f_0032dc10__generated,
            world_init_sphere_tree_cache,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_dc11, 0);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                let args = [
                    r.next_u32() % 4,
                    (r.next_u32() % 5).wrapping_sub(1),
                    r.next_u32() % 4,
                    (r.next_u32() % 5).wrapping_sub(1),
                ];
                let mode = schedule(c, &mut r, seed, 0x0032_dc10);
                for (i, a) in args.into_iter().enumerate() {
                    c.g[A0 + i][0] = sx(a);
                }
                c.g[RA][0] = SENTINEL_m2 as u64;
                mode
            },
        );
    }

    use crate::difftest as dt;

    use std::sync::atomic::AtomicUsize;

    pub(crate) use crate::game::input::inputmap::tests::oc561_setup;

    pub(crate) use crate::game::input::inputmap::tests::oc561_install_fakes;

    #[test]
    fn rider_sphere_tree_ctor_copy_matches() {
        oc561_install_fakes();
        check(
            ee::s10::f_00329938__generated,
            rider_sphere_tree_ctor_copy,
            16,
            &|c, seed| {
                let mut r = Rng(0x0032_9938 ^ seed as u64);
                oc561_setup(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x40 * seed);
            },
        );
    }

    #[test]
    fn rle_decode_matches() {
        check(
            ee::s10::f_0032b620__generated,
            rle_decode,
            200,
            &|c, seed| {
                let mut r = Rng(0x0032_b620 ^ seed as u64);
                scramble(c, &mut r);
                let src = DATA + 0x1000;
                let mut at = src;
                if seed % 6 == 0 {
                    c.g[A1][0] = 0;
                } else {
                    c.g[A1][0] = sx(src);
                    let segments = 1 + r.next_u32() % 6;
                    for _ in 0..segments {
                        if r.next_u32() & 1 == 0 {
                            // Repeat run: header 0..127, then the byte to repeat.
                            c.bus.write8(at, (1 + r.next_u32() % 40) as u8);
                            c.bus.write8(at + 1, r.next_u32() as u8);
                            at += 2;
                        } else {
                            // Literal run: header -n, then n bytes.
                            let n = 1 + r.next_u32() % 20;
                            c.bus.write8(at, (256 - n) as u8);
                            at += 1;
                            for _ in 0..n {
                                c.bus.write8(at, r.next_u32() as u8);
                                at += 1;
                            }
                        }
                    }
                    c.bus.write8(at, 0);
                }
                c.g[A0][0] = sx(DATA + 0x20000);
            },
        );
    }

    /// Fixture layout (all inside `dt::DATA..dt::DATA + 0xfffff`).
    pub(crate) const SELF: u32 = dt::DATA;

    pub(crate) const TREE: u32 = dt::DATA + 0x200;

    pub(crate) const LEVELS: u32 = dt::DATA + 0x400;

    pub(crate) const BITS: u32 = dt::DATA + 0x600;

    pub(crate) const UNORM: u32 = dt::DATA + 0x800;

    pub(crate) const POS: u32 = dt::DATA + 0x820;

    pub(crate) const P1: u32 = dt::DATA + 0x840;

    pub(crate) const P2: u32 = dt::DATA + 0x860;

    pub(crate) const P3: u32 = dt::DATA + 0x880;

    pub(crate) const OUT_A: u32 = dt::DATA + 0x900;

    pub(crate) const OUT_B: u32 = dt::DATA + 0x920;

    pub(crate) fn set_vec(c: &mut Cpu, addr: u32, r: &mut Rng) {
        for i in 0..4 {
            c.bus.write32(addr + 4 * i, r.f32(8.0));
        }
    }

    #[test]
    fn sphere_tree_collide_triangle_node_matches() {
        check(
            ee::s11::f_0032d470__generated,
            sphere_tree_collide_triangle_node,
            200,
            &|c, seed| {
                let mut r = Rng(0x5eed_0562 ^ seed as u64);
                scramble(c, &mut r);
                // The node: a unit scale, a worst case that sometimes lets
                // every plane offset through, child axes and order.
                c.bus.write32(
                    SELF + SCT_SCALE,
                    if seed % 2 == 0 {
                        0x3f80_0000
                    } else {
                        r.f32(2.0)
                    },
                );
                c.bus.write32(
                    SELF + SCT_WORST_CASE,
                    if seed % 3 == 0 {
                        0x7f7f_ffff
                    } else {
                        r.f32(4.0)
                    },
                );
                c.bus.write32(SELF + SCT_TREE_INFO, TREE);
                for k in 0..8u32 {
                    set_vec(c, SELF + SCT_AXIS + 16 * k, &mut r);
                    c.bus.write32(SELF + SCT_SORT_AXIS + 4 * k, (7 - k) % 8);
                }
                // The tree: depth 0..2, levels with radius and offsets.
                c.bus.write32(TREE + SCT_INFO_DEPTH, (seed % 3) + 1);
                c.bus.write32(TREE + SCT_INFO_LEVELS, LEVELS);
                c.bus.write32(TREE + SCT_INFO_BIT_TREE, BITS);
                for l in 0..4u32 {
                    let base = LEVELS + SCT_LEVEL_STRIDE * l;
                    c.bus.write32(base, r.f32(30.0) & 0x7fff_ffff);
                    c.bus.write32(base + SCT_LEVEL_CEN_OFFSET, r.f32(10.0));
                    c.bus.write32(base + SCT_LEVEL_INDEX_OFFSET, 1 + l);
                }
                for i in 0..4u32 {
                    c.bus
                        .write8(BITS + i, if seed % 5 == 0 { 0 } else { r.next_u32() as u8 });
                }
                set_vec(c, UNORM, &mut r);
                set_vec(c, POS, &mut r);
                set_vec(c, P1, &mut r);
                set_vec(c, P2, &mut r);
                set_vec(c, P3, &mut r);
                set_vec(c, OUT_A, &mut r);
                set_vec(c, OUT_B, &mut r);
                // Stack arguments 9 and 10.
                c.bus.write32(dt::SP + 0x140, OUT_A);
                c.bus.write32(dt::SP + 0x148, OUT_B);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(P1);
                c.g[A2][0] = sx(P2);
                c.g[A3][0] = sx(P3);
                c.g[T0][0] = sx(UNORM);
                c.g[T1][0] = sx(POS);
                c.g[T2][0] = sx(seed % 3);
                c.g[T3][0] = sx(seed % 2);
                c.g[SP][0] = sx(dt::SP);
                c.g[RA][0] = sx(dt::RA);
                c.pc = 0x0032_d470;
            },
        );
    }

    #[test]
    fn vec3_neg_dot_matches() {
        check(
            ssx3_ee::s11::f_003300f8__generated,
            vec3_neg_dot,
            200,
            &|c, seed| {
                let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0033_00f8;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x20);
                for k in 0..4u32 {
                    c.bus.write32(DATA + 4 * k, r.f32(100.0));
                    c.bus.write32(DATA + 0x20 + 4 * k, r.f32(100.0));
                }
            },
        );
    }

    /// The callback the segment test calls: a real leaf that returns 0.
    pub(crate) const LEAF_RET0: u32 = 0x0014_06c0;

    #[test]
    fn coll_segment_collide_triangle_2sided_matches() {
        check(
            ssx3_ee::s11::f_0032e5e8__generated,
            coll_segment_collide_triangle_2sided,
            200,
            &|c, seed| {
                let mut r = Rng(0x2c6b_9d31_5a7e_1f03 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0032_e5e8;
                let seg = DATA + 0x100;
                let tri = DATA + 0x200;
                let obj = DATA + 0x300;
                c.g[A0][0] = sx(seg);
                c.g[T0][0] = sx(tri);
                for k in 0..4u32 {
                    c.bus.write32(seg + 0x70 + 4 * k, r.f32(10.0));
                    c.bus.write32(tri + 4 * k, r.f32(10.0));
                }
                c.bus.write32(seg + 0x50, obj);
                c.bus.write16(obj + 0x20, r.next_u32() as u16);
                c.bus.write32(obj + 0x24, LEAF_RET0);
            },
        );
    }

    /// The RLE test's input and output.
    pub(crate) const RLE_IN: u32 = DATA + 0x800;

    pub(crate) const RLE_OUT: u32 = DATA + 0x1000;

    /// The gSphTreeAlignAxis table the transform reads (0x4ff640 .. +0x40).
    pub(crate) const ALIGN_AXIS: u32 = 0x004f_f640;

    pub(crate) fn rle_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x7e1c_0de5_0000_0001 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        let len = r.next_u32() % 49;
        let min_run = r.next_u32() % 6;
        c.g[A0][0] = sx(RLE_OUT);
        c.g[A1][0] = sx(RLE_IN);
        c.g[A2][0] = len as u64;
        c.g[A3][0] = min_run as u64;
        // Bytes in runs of a few values, so both kinds of output occur.
        let mut byte = 0u8;
        for i in 0..len {
            if r.next_u32() % 3 == 0 {
                byte = (r.next_u32() % 3) as u8;
            }
            c.bus.write8(RLE_IN + i, byte);
        }
    }

    #[test]
    fn rle_encode_matches() {
        check(
            ee::s10::f_0032b468__generated,
            rle_encode,
            300,
            &|c, seed| rle_setup(c, seed),
        );
    }

    /// The transform's self (axes, centre, scale), its tree info (root centre)
    /// and the matrix at DATA+0x100.
    pub(crate) fn set_transform_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x5e7_0c64_0000_0001 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x100);
        c.f[12] = r.edge_f32();
        for i in 0..16u32 {
            c.bus.write32(ALIGN_AXIS + 4 * i, r.f32(2.0));
        }
        for i in 0..16u32 {
            c.bus.write32(DATA + 0x100 + 4 * i, r.f32(4.0));
        }
        let info = DATA + 0x200;
        c.bus.write32(DATA + 0x98, info);
        for i in 0..3u32 {
            c.bus.write32(info + 0x14 + 4 * i, r.f32(100.0));
        }
    }

    #[test]
    fn sphere_tree_set_transform_matches() {
        check(
            ee::s10::f_0032c648__generated,
            sphere_tree_set_transform,
            300,
            &|c, seed| set_transform_setup(c, seed),
        );
    }

    /// The transform's self (axes, centre at +0x80, scale at +0x90) and the matrix.
    pub(crate) fn apply_transform_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0a99_17e5_0000_0001 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x100);
        c.f[12] = r.edge_f32();
        for i in 0..0x26u32 {
            c.bus.write32(DATA + 4 * i, r.f32(4.0));
        }
        c.bus.write32(DATA + 0x90, r.f32(2.0));
        for i in 0..16u32 {
            c.bus.write32(DATA + 0x100 + 4 * i, r.f32(4.0));
        }
    }

    #[test]
    fn sphere_tree_apply_transform_matches() {
        check(
            ee::s10::f_0032c770__generated,
            sphere_tree_apply_transform,
            300,
            &|c, seed| apply_transform_setup(c, seed),
        );
    }

    /// The fakes below run only when the machine holds `MAGIC` at `CTRL`.
    pub(crate) const CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const MAGIC: u32 = 0x0569_f00d;

    pub(crate) static FREE_PREV: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static ALLOC_PREV: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static CTOR_PREV: AtomicUsize = AtomicUsize::new(0);

    /// `EA_MemFree`: a leaf that returns (gate on).
    pub(crate) fn oc569_fake_mem_free(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &FREE_PREV, ee::s10::f_00317e98__generated);
        }
        abi::ret(c);
    }

    /// `EA_MemAllocNamed`: returns a fixed block in the scratch area (gate on).
    pub(crate) fn oc569_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &ALLOC_PREV, ee::s10::f_00317d70__generated);
        }
        c.g[V0][0] = (DATA + 0x30_0000) as u64;
        abi::ret(c);
    }

    /// `SphereTree_ctor`: a leaf that returns (gate on).
    pub(crate) fn oc569_fake_tree_ctor(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &CTOR_PREV, ee::s10::f_0032c508__generated);
        }
        abi::ret(c);
    }

    pub(crate) fn hook_fakes() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc569_fake_mem_free,
            &FREE_PREV,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc569_fake_alloc,
            &ALLOC_PREV,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_0032C508),
            oc569_fake_tree_ctor,
            &CTOR_PREV,
        );
    }

    /// Scrambled registers and the game gp.
    pub(crate) fn base_7(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x5690_0000_0000_0001 ^ ((seed as u64) << 8) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// `words` random float bit patterns at `addr`.
    pub(crate) fn floats_2(c: &mut Cpu, r: &mut Rng, addr: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(addr + 4 * i, r.edge_f32());
        }
    }

    /// Self at DATA+0x1000, vectors at DATA+0x2000..; pointers in the argument
    /// registers of the triangle tests.
    pub(crate) fn triangle_fixture(c: &mut Cpu, r: &mut Rng) {
        floats_2(c, r, DATA + 0x1000, 0x180);
        floats_2(c, r, DATA + 0x2000, 0x180);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = sx(DATA + 0x2000);
        c.g[A2][0] = sx(DATA + 0x2100);
        c.g[A3][0] = sx(DATA + 0x2200);
        c.g[T0][0] = sx(DATA + 0x2300);
        c.g[T1][0] = sx(DATA + 0x2400);
        c.g[T2][0] = sx(DATA + 0x2500);
        c.g[T3][0] = sx(DATA + 0x2600);
    }

    #[test]
    fn sphere_tree_point_in_triangle_matches() {
        check(
            ee::s11::f_0032d440__generated,
            sphere_tree_point_in_triangle,
            200,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x32d4);
                c.pc = 0x0032_d440;
                floats_2(c, &mut r, DATA + 0x100, 0x40);
                c.bus.write32(GAME_GP - 0x2d80, r.f32(1.0e-4));
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x200);
                c.g[A2][0] = sx(DATA + 0x300);
                c.g[A3][0] = sx(DATA + 0x400);
                c.g[T0][0] = sx(DATA + 0x500);
            },
        );
    }

    #[test]
    fn world_free_sphere_tree_cache_matches() {
        hook_fakes();
        check(
            ee::s11::f_0032de20__generated,
            world_free_sphere_tree_cache,
            200,
            &|c, seed| {
                let _r = base_7(c, seed, 0x32de);
                c.pc = 0x0032_de20;
                c.bus.write32(CTRL, MAGIC);
                let arr1 = DATA + 0x2000;
                let arr2 = DATA + 0x2100;
                c.bus.write32(GAME_GP + 0xed8, arr1);
                c.bus.write32(GAME_GP + 0xee0, seed % 4);
                c.bus.write32(GAME_GP + 0xee8, arr2);
                c.bus.write32(GAME_GP + 0xef0, (seed / 4) % 4);
                let single = if seed % 3 == 0 { 0 } else { DATA + 0x2200 };
                c.bus.write32(GAME_GP + 0xedc, single);
                let single2 = if seed % 5 == 0 { 0 } else { DATA + 0x2300 };
                c.bus.write32(GAME_GP + 0xeec, single2);
                for i in 0..4u32 {
                    let e1 = if (seed + i) % 3 == 0 {
                        0
                    } else {
                        DATA + 0x2400 + 0x10 * i
                    };
                    let e2 = if (seed + i) % 4 == 0 {
                        0
                    } else {
                        DATA + 0x2500 + 0x10 * i
                    };
                    c.bus.write32(arr1 + 4 * i, e1);
                    c.bus.write32(arr2 + 4 * i, e2);
                }
            },
        );
    }

    #[test]
    fn coll_sphere_collide_triangle_matches() {
        check(
            ee::s11::f_0032fe78__generated,
            coll_sphere_collide_triangle,
            200,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x32fe);
                c.pc = 0x0032_fe78;
                triangle_fixture(c, &mut r);
            },
        );
    }

    #[test]
    fn coll_sphere_collide_triangle_2sided_matches() {
        check(
            ee::s11::f_0032ffa0__generated,
            coll_sphere_collide_triangle_2sided,
            200,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x32ff);
                c.pc = 0x0032_ffa0;
                triangle_fixture(c, &mut r);
            },
        );
    }

    #[test]
    fn closest_point_on_segment_matches() {
        check(
            ee::s11::f_00330128__generated,
            closest_point_on_segment,
            200,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x3301);
                c.pc = 0x0033_0128;
                floats_2(c, &mut r, DATA + 0x3100, 0x40);
                c.g[A0][0] = sx(DATA + 0x3000);
                c.g[A1][0] = sx(DATA + 0x3100);
                c.g[A2][0] = sx(DATA + 0x3200);
                c.g[A3][0] = sx(DATA + 0x3300);
            },
        );
    }

    #[test]
    fn sphere_tree_collide_sphere_node_matches() {
        check(
            ee::s10::f_0032cdb0__generated,
            sphere_tree_collide_sphere_node,
            200,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x32cd);
                c.pc = 0x0032_cdb0;
                let this = DATA + 0x1000;
                let info = DATA + 0x1100;
                let levels = DATA + 0x1200;
                let masks = DATA + 0x1300;
                // Axes (16 bytes each at self+0), scale at +0x90, tree at +0x98.
                floats_2(c, &mut r, this, 0x10);
                c.bus.write32(this + 0x90, r.f32(2.0));
                c.bus.write32(this + 0x98, info);
                c.bus.write32(info + 0xc, 2);
                c.bus.write32(info + 0x20, levels);
                c.bus.write32(info + 0x28, masks);
                floats_2(c, &mut r, levels, 9);
                for k in 0..3u32 {
                    c.bus.write32(levels + 12 * k + 8, 0x40 * (k + 1));
                }
                c.bus.write32(masks, r.next_u32());
                let pos = DATA + 0x1400;
                let centre = DATA + 0x1420;
                floats_2(c, &mut r, pos, 4);
                floats_2(c, &mut r, centre, 4);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pos);
                c.g[A2][0] = sx(centre);
                c.g[A3][0] = sx(seed % 3);
                c.g[T0][0] = 0;
                c.g[T1][0] = sx(DATA + 0x1440);
                c.g[T2][0] = sx(DATA + 0x1460);
                c.f[12] = r.f32(8.0);
            },
        );
    }

    #[test]
    fn sphere_tree_collide_aabb_node_matches() {
        check(
            ssx3_ee::s11::f_0032d028__generated,
            sphere_tree_collide_aabb_node,
            200,
            &|c, seed| {
                let mut r = Rng(0x0032_d028 ^ ((seed as u64 + 1) << 24) | 1);
                scramble(c, &mut r);
                let me = DATA;
                let tree = DATA + 0x400;
                let levels = DATA + 0x500;
                let bits = DATA + 0x600;
                let out_pos = DATA + 0x700;
                let out_normal = DATA + 0x720;
                let pos_at = DATA + 0x740;
                let box_min_at = DATA + 0x760;
                let box_max_at = DATA + 0x780;
                // Tree depth 0..2; the incoming depth below or at it (both
                // the leaf and the recursion paths get taken).
                let tree_depth = seed % 3;
                let depth = (seed / 3) % 3;
                let base = (seed / 9) % 16;
                c.bus.write32(me + SELF_TREE_INFO, tree);
                c.bus.write32(tree + TREE_DEPTH, tree_depth);
                c.bus.write32(tree + TREE_LEVELS, levels);
                c.bus.write32(tree + TREE_BIT_TREE, bits);
                c.bus.write32(me + SELF_SCALE, r.f32(2.0));
                for i in 0..8 {
                    put_vec(
                        c,
                        me + SELF_AXIS + i * 0x10,
                        [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0],
                    );
                }
                for k in 0..4 {
                    c.bus.write32(levels + k * LEVEL_SIZE, r.f32(1.0));
                    c.bus.write32(levels + k * LEVEL_SIZE + 4, r.f32(0.5));
                    c.bus.write32(
                        levels + k * LEVEL_SIZE + LEVEL_INDEX_OFFSET,
                        1 + r.next_u32() % 3,
                    );
                }
                fill(c, &mut r, bits, 64);
                let p = [r.f32(4.0), r.f32(4.0), r.f32(4.0), 0];
                put_vec(c, pos_at, p);
                // Most seeds put the box around the centre (a hit); every
                // fourth moves it far away (a miss).
                let far = if seed % 4 == 0 { 100.0 } else { 0.0 };
                let f = |u: u32| f32::from_bits(u);
                let lo = [
                    f(p[0]) - 1.5 + far,
                    f(p[1]) - 1.5 + far,
                    f(p[2]) - 1.5 + far,
                    0.0,
                ];
                let hi = [
                    f(p[0]) + 1.5 + far,
                    f(p[1]) + 1.5 + far,
                    f(p[2]) + 1.5 + far,
                    0.0,
                ];
                put_vec(c, box_min_at, lo.map(f32::to_bits));
                put_vec(c, box_max_at, hi.map(f32::to_bits));
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(box_min_at);
                c.g[A2][0] = sx(box_max_at);
                c.g[A3][0] = sx(pos_at);
                c.g[T0][0] = depth as u64;
                c.g[T1][0] = base as u64;
                c.g[T2][0] = sx(out_pos);
                c.g[T3][0] = sx(out_normal);
            },
        );
    }

    pub(crate) const SEEDS_7: u32 = 200;

    pub(crate) const SELF_2: u32 = DATA + 0x1000;

    pub(crate) const INFO: u32 = DATA + 0x2000;

    pub(crate) const BITS_2: u32 = DATA + 0x3000;

    pub(crate) const LEVELS_2: u32 = DATA + 0x4000;

    pub(crate) const PLANE: u32 = DATA + 0x5000;

    pub(crate) const POS_2: u32 = DATA + 0x5010;

    pub(crate) const OUT_A_2: u32 = DATA + 0x5100;

    pub(crate) const OUT_B_2: u32 = DATA + 0x5110;

    pub(crate) const STACK: u32 = DATA + 0x8000;

    /// The tree in memory: a depth 1..3 tree, the axes, the sort order, the
    /// scale and worstCase (seed-selected: early exits, always-update leaves,
    /// random ones), the masks (some seeds all zero) and the plane and point.
    pub(crate) fn fixture_data(c: &mut Cpu, seed: u32, r: &mut Rng) {
        use collide_plane_node::*;
        let depth = 1 + seed % 3;
        c.bus.write32(INFO + INFO_DEPTH, depth);
        c.bus.write32(INFO + INFO_LEVELS, LEVELS_2);
        c.bus.write32(INFO + INFO_BIT_TREE, BITS_2);
        for i in 0..=depth {
            let at = LEVELS_2 + i * LEVEL_SIZE;
            let radius = 0.5 + (r.next_u32() % 100) as f32 / 50.0;
            c.bus.write32(at + LEVEL_RADIUS, radius.to_bits());
            c.bus.write32(at + LEVEL_CEN_OFFSET, r.f32(1.0));
            c.bus.write32(at + LEVEL_INDEX_OFFSET, 1 + r.next_u32() % 3);
        }
        for k in 0..8 {
            let v = [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)];
            put_vec(c, SELF_2 + 16 * k, v);
            c.bus.write32(SELF_2 + SORT_AXIS + 4 * k, (k + seed) % 8);
        }
        c.bus
            .write32(SELF_2 + SCALE, (0.5 + (seed % 3) as f32 * 0.25).to_bits());
        let worst = match seed % 7 {
            0 => (-1.0e6f32).to_bits(),  // never inside: the early exit
            1..=4 => 1.0e6f32.to_bits(), // always inside, leaves update
            _ => r.f32(2.0),
        };
        c.bus.write32(SELF_2 + WORST_CASE, worst);
        c.bus.write32(SELF_2 + TREE_INFO, INFO);
        let plane = [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(2.0)];
        put_vec(c, PLANE, plane);
        let pos = [r.f32(2.0), r.f32(2.0), r.f32(2.0), 1.0f32.to_bits()];
        put_vec(c, POS_2, pos);
        put_vec(c, OUT_A_2, [0; 4]);
        put_vec(c, OUT_B_2, [0; 4]);
        for i in 0..0x100 {
            let mask = if seed % 5 == 2 { 0 } else { r.next_u32() as u8 };
            c.bus.write8(BITS_2 + i, mask);
        }
    }

    /// The argument registers. `depth` is below the tree's depth on most
    /// seeds (interior nodes), at it on the rest (leaves).
    pub(crate) fn fixture_args(c: &mut Cpu, seed: u32) {
        let depth = 1 + seed % 3;
        let level = if seed % 4 == 3 { depth } else { seed % depth };
        c.pc = collide_plane_node::ENTRY;
        c.g[A0][0] = sx(SELF_2);
        c.g[A1][0] = sx(PLANE);
        c.g[A2][0] = sx(POS_2);
        c.g[A3][0] = level as u64;
        c.g[T0][0] = (seed % 2) as u64;
        c.g[T1][0] = sx(OUT_A_2);
        c.g[T2][0] = sx(OUT_B_2);
    }

    #[test]
    fn sphere_tree_collide_plane_node_matches() {
        check(
            ee::s11::f_0032d208__generated,
            sphere_tree_collide_plane_node,
            SEEDS_7,
            &|c, seed| {
                let mut r = Rng(0x7a3c_51d9 ^ seed as u64);
                scramble(c, &mut r);
                fixture_data(c, seed, &mut r);
                c.g[SP][0] = sx(STACK);
                fixture_args(c, seed);
            },
        );
    }

    #[test]
    fn sphere_tree_collide_plane_node_events_match() {
        check_events(
            ee::s11::f_0032d208__generated,
            sphere_tree_collide_plane_node,
            SEEDS_7,
            &|c, seed| {
                let mut r = Rng(0x3e9b_0d17 ^ seed as u64);
                scramble(c, &mut r);
                fixture_data(c, seed, &mut r);
                let mode = schedule(c, &mut r, seed, collide_plane_node::ENTRY);
                fixture_args(c, seed);
                mode
            },
        );
    }

    #[test]
    fn sphere_tree_cache_get_masks_matches() {
        check(
            ee::s11::f_0032df28__generated,
            sphere_tree_cache_get_masks,
            200,
            &|c, seed| {
                machine(c, seed, 0xdf28, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0032df28;
            },
        );
    }

    pub(crate) fn tree_node_collide_base(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = Rng(0x32cb_f800_0000_0001 ^ ((seed as u64) << 8));
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.g[SP][0] = sx(crate::difftest::SP);
        c.g[RA][0] = sx(crate::difftest::RA);
        r
    }

    /// The sphere walk's tree layout (as `sphere_tree_collide_sphere_node`'s
    /// fixture): two levels deep, mask bytes per node, child centres at self.
    pub(crate) fn tree_node_collide_fixture(c: &mut Cpu, seed: u32, r: &mut Rng) {
        let this = DATA + 0x1000;
        let info = DATA + 0x1100;
        let levels = DATA + 0x1200;
        let masks = DATA + 0x1300;
        let pos = DATA + 0x1400;
        let centre = DATA + 0x1420;
        for i in 0..0x24u32 {
            c.bus.write32(this + 4 * i, r.f32(2.0));
        }
        c.bus.write32(this + 0x98, info);
        c.bus.write32(info + 0xc, 2);
        c.bus.write32(info + 0x20, levels);
        c.bus.write32(info + 0x28, masks);
        for k in 0..3u32 {
            let at = levels + 12 * k;
            c.bus.write32(at, r.f32(2.0));
            c.bus.write32(at + 4, r.f32(1.0));
            c.bus.write32(at + 8, 0x40 * (k + 1));
        }
        for i in 0..0x200u32 {
            let byte = if seed % 4 == 0 { 0 } else { r.next_u32() as u8 };
            c.bus.write8(masks + i, byte);
        }
        for i in 0..4u32 {
            c.bus.write32(pos + 4 * i, r.f32(2.0));
            c.bus.write32(centre + 4 * i, r.f32(2.0));
        }
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(pos);
        c.g[A2][0] = sx(centre);
        c.g[A3][0] = sx(seed % 3);
        c.g[T0][0] = sx((seed % 2) * 0x10);
        c.g[T1][0] = sx(DATA + 0x1440);
        c.g[T2][0] = sx(DATA + 0x1460);
        c.f[12] = r.f32(8.0);
        c.pc = 0x0032_cbf8;
    }

    #[test]
    fn sphere_tree_collide_tree_node_matches() {
        check(
            ee::s10::f_0032cbf8__generated,
            sphere_tree_collide_tree_node,
            200,
            &|c, seed| {
                let mut r = tree_node_collide_base(c, seed);
                tree_node_collide_fixture(c, seed, &mut r);
            },
        );
    }

    pub(crate) const FX_SORT: u32 = DATA + 0x30_0000;

    pub(crate) const FX_SORT_GATE: u32 = DATA + 0x31_0000;

    pub(crate) const FX_SORT_MAGIC: u32 = 0x5027_0706;

    // ShellSortDesc_KeyVal is bounded only by its input: the gated fake logs the
    // call and returns, the generated body runs otherwise.
    pub(crate) fn oc706_sort_fake(c: &mut Cpu) {
        if c.bus.read32(FX_SORT_GATE) != FX_SORT_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc706_sort_fake),
                ee::s11::f_0032da40__generated,
            );
        }
        fakes::fake_call(c, 0x0032_da40);
    }

    pub(crate) fn oc706_sort_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0032DA40),
            oc706_sort_fake,
            fakes::prev_cell(oc706_sort_fake),
        );
    }

    #[test]
    fn sphere_tree_sort_child_order_matches() {
        check(
            ee::s11::f_0032db40__generated,
            sphere_tree_sort_child_order_2,
            200,
            &|c, seed| {
                oc706_sort_install();
                c.bus.write32(FX_SORT_GATE, FX_SORT_MAGIC);
                let mut r = Rng(0x0032_db40 ^ seed as u64);
                for i in 0..0x60u32 {
                    c.bus.write32(FX_SORT + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(FX_SORT);
                c.g[A1][0] = sx(FX_SORT + 0x40);
                c.pc = 0x0032_db40;
            },
        );
    }
}
