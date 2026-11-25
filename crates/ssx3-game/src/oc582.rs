//! Staging module oc582 (wave wave233): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc582`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0034_66a0, flex_bridge_build_mesh, "FlexBridge_BuildMesh")];

// Bridge node fields (offsets from `self`).
const NODE_DEF: u32 = 0x18; // the instance record the mesh is built from
const NODE_COUNT: u32 = 0x50; // segments; the grid has 2 rows of `count` vertices
const NODE_MESH: u32 = 0x78; // the grid mesh object (its vt[6] fills the vertices)
const NODE_POS: u32 = 0x7c; // pts[0]: 2 * count grid vertices (16 bytes each)
const NODE_NRM: u32 = 0x80; // pts[1]: 2 * count normals (16 bytes each)
const NODE_IDX: u32 = 0x84; // triangle index bytes, 3 per triangle
const NODE_BOX: u32 = 0x90; // tBoundBox: min at +0x90, max at +0xa0

// Instance record and world fields.
const DEF_MIN: u32 = 0x60; // def->min (x, y, z floats)
const DEF_MAX: u32 = 0x6c; // def->max (x, y, z floats)
const GFX_MAN_GP: u32 = 0x2a90; // gp-relative: the graphics manager
const GFX_MAN_WORLD: u32 = 0x10d8; // graphics manager -> world
const WORLD_GRID_MESH_DELTA: u32 = 0x260; // CreateGridMesh: this delta (halfword) ...
const WORLD_GRID_MESH_FN: u32 = 0x264; // ... and the function (word)
const WORLD_GET_QUAD_DELTA: u32 = 0x308; // GetMeshQuad: this delta (halfword) ...
const WORLD_GET_QUAD_FN: u32 = 0x30c; // ... and the function (word)
const MESH_VT_DELTA: u32 = 0x30; // mesh vtable slot 6: this delta (halfword) ...
const MESH_VT_FN: u32 = 0x34; // ... and the vertex-fill function (word)

// Callees (guest addresses).
const OPERATOR_NEW: u32 = 0x0031_7e30;
const OPERATOR_DELETE: u32 = 0x0031_7e98;
const WORLD_GET_NASH: u32 = 0x002d_1be0;
const SPATIAL_HASH_UPDATE: u32 = 0x0032_91e0;

/// Named blocks of the allocation names (`str_BridgeVerts`, ...): the
/// generated code passes them as `0x490000 - k`.
const NAME_BASE: u32 = 0x0049_0000;

/// `cFlexBridgeNode_setupGrid` (`FlexBridge_BuildMesh`, 0x3466a0): builds the
/// grid mesh of a flex bridge node. `count` segments give `2 * count`
/// vertices (two rows, one per side of the bridge) and `2 * (count - 1)`
/// triangles. The vertices and normals are interpolated from the two end
/// quads of the instance record (`GetMeshQuad`); the grid mesh fills its
/// vertices through vtable slot 6; the bounding box is the record's box
/// grown by 100 on every side; the spatial hash is updated with the box
/// before and after.
pub fn flex_bridge_build_mesh(c: &mut Cpu) {
    guest(c, build_mesh);
}

fn build_mesh(c: &mut Cpu) -> Flow {
    c.cycles += 42;
    let frame = Frame::open(c, 0x1b0);
    let stk = sp(c);
    let slot = |off: u32| stk.wrapping_add(off);
    c.g[A2][0] = 0x2;
    frame.save(c, S0, 0x190);
    c.g[A3][0] = 0x2000_0000;
    frame.save(c, S1, 0x180);
    frame.save(c, S2, 0x170);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S3, 0x160);
    frame.save(c, S5, 0x140);
    c.g[S3][0] = sx(r32(c, S1).wrapping_add(NODE_POS));
    frame.save(c, FP, 0x110);
    frame.save(c, S6, 0x130);
    frame.save(c, S7, 0x120);
    frame.save_ra(c, 0x100);
    frame.save_fpr(c, 21, 0x1a8);
    frame.save_fpr(c, 20, 0x1a0);
    frame.save(c, S4, 0x150);

    // Graphics manager -> world; n = count; nverts = 2n; ntris = 2(n-1).
    c.g[S5][0] = c.lw(r32(c, GP).wrapping_add(GFX_MAN_GP));
    c.g[S4][0] = c.lw(r32(c, S1).wrapping_add(NODE_COUNT));
    c.g[V1][0] = c.lw(r32(c, S5).wrapping_add(GFX_MAN_WORLD));
    c.g[V0][0] = sx(r32(c, S4).wrapping_sub(1));
    c.g[A1][0] = c.g[S4][0];
    c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(WORLD_GRID_MESH_DELTA));
    c.g[S2][0] = sx(r32(c, V0) << 1);
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(WORLD_GRID_MESH_FN));
    c.g[S0][0] = sx(r32(c, S4) << 1);
    // self->mesh = world->CreateGridMesh(this, n, 2, 0x20000000)
    let create = r32(c, V0);
    c.g[RA][0] = sx(0x0034_6710);
    c.g[A0][0] = sx(r32(c, S5).wrapping_add(r32(c, A0)));
    call_ptr(c, create, 0x0034_6710)?;
    c.g[A1][0] = NAME_BASE as u64;
    c.g[A2][0] = 0x2000_0000;
    c.sw(r32(c, S1).wrapping_add(NODE_MESH), c.g[V0][0]);
    c.g[A1][0] = sx(NAME_BASE.wrapping_sub(0x1888));
    c.g[A0][0] = sx(r32(c, S4) << 5);
    c.g[A3][0] = 0;
    // pts[0] = new sGridV4[2n]
    call_ptr(c, OPERATOR_NEW, 0x0034_672c)?;

    c.g[V1][0] = sx(slot(0x50));
    c.g[A2][0] = sx(slot(0x90));
    c.sw(slot(0xf4), c.g[V1][0]);
    c.g[FP][0] = sx(slot(0x30));
    c.sw(slot(0xfc), c.g[A2][0]);
    let s0_is_zero = c.g[S0][0] == 0;
    c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
    if !s0_is_zero {
        c.cycles += 2;
        c.g[A0][0] = u64::MAX;
    }

    // pts[0] stored in self; pts[1] = new sGridV4[2n]; idx = new u8[3 * ntris].
    c.cycles += 20;
    c.g[A0][0] = sx(r32(c, S0) << 4);
    c.g[A1][0] = NAME_BASE as u64;
    c.sw(r32(c, S3), c.g[V0][0]);
    c.g[S7][0] = c.g[A0][0];
    c.g[A1][0] = sx(NAME_BASE.wrapping_sub(0x1878));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[S6][0] = sx(r32(c, S0) << 3);
    call_ptr(c, OPERATOR_NEW, 0x0034_6774)?;
    c.g[S3][0] = sx(r32(c, S1).wrapping_add(NODE_NRM));
    c.g[V1][0] = c.g[V0][0];
    c.g[A0][0] = sx(r32(c, S0).wrapping_sub(1));
    c.g[T3][0] = sx(r32(c, S1).wrapping_add(NODE_BOX));
    c.g[T4][0] = sx(r32(c, S2).wrapping_sub(1));
    c.g[A1][0] = sx(r32(c, S2) << 1);
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = c.g[A0][0];
    c.sw(slot(0xf0), c.g[T3][0]);
    c.sw(slot(0xf8), c.g[T4][0]);

    c.cycles += 14;
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, S2)));
    c.sw(r32(c, S3), c.g[V1][0]);
    c.g[A1][0] = NAME_BASE as u64;
    c.g[A2][0] = 0x2000_0000;
    c.g[A1][0] = sx(NAME_BASE.wrapping_sub(0x1868));
    c.g[A3][0] = 0;
    call_ptr(c, OPERATOR_NEW, 0x0034_67bc)?;
    c.sw(r32(c, S1).wrapping_add(NODE_IDX), c.g[V0][0]);
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = 0x3;

    // for (v0 = 3; v0 != -1; v0--) { 7 cycles }
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let more = c.g[V0][0] != c.g[V1][0];
        c.g[A1][0] = NAME_BASE as u64;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0034_67d8);
        }
    }

    // tmp uv = new sGridV2[2n]
    c.cycles += 9;
    c.g[A0][0] = c.g[S6][0];
    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1858));
    c.g[A2][0] = 0x100;
    c.g[A3][0] = 0;
    call_ptr(c, OPERATOR_NEW, 0x0034_6808)?;
    c.g[S3][0] = c.g[V0][0];
    c.g[V0][0] = u64::MAX;
    c.g[V1][0] = c.g[S0][0];

    // tmp col = new sGridCol[2n]
    c.cycles += 11;
    c.g[A1][0] = NAME_BASE as u64;
    c.g[A0][0] = c.g[S7][0];
    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1848));
    c.g[A2][0] = 0x100;
    c.g[A3][0] = 0;
    call_ptr(c, OPERATOR_NEW, 0x0034_6830)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V1][0] = c.g[S0][0];
    c.g[V0][0] = u64::MAX;
    if c.g[V1][0] == c.g[V0][0] {
        c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(GFX_MAN_WORLD));
    } else {
        c.cycles += 2;
        c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(GFX_MAN_WORLD));
    }

    // world->GetMeshQuad(def, 0, pos, uvs, cols)
    c.cycles += 11;
    c.g[A3][0] = c.g[SP][0];
    c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(NODE_DEF));
    c.g[T0][0] = c.g[FP][0];
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(WORLD_GET_QUAD_DELTA));
    c.g[A2][0] = 0;
    c.g[T1][0] = c.lw(slot(0xf4));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(WORLD_GET_QUAD_FN));
    let get_quad = r32(c, V1);
    c.g[A0][0] = sx(r32(c, S5).wrapping_add(r32(c, A0)));
    call_ptr(c, get_quad, 0x0034_6870)?;

    let no_quads = (c.g[S4][0] as i64) <= 0;
    c.g[A3][0] = 0;
    if !no_quads {
        // Per segment i: the two rows of vertices, normals and colours, interpolated
        // between the end quads with t = i / (n - 1).
        c.cycles += 14;
        c.g[V0][0] = sx(r32(c, S4).wrapping_sub(1));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 21);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.g[V1][0] = sx(r32(c, S4) << 4);
        c.g[V0][0] = sx(r32(c, S4) << 3);
        c.g[T2][0] = c.g[V1][0];
        c.g[T1][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
        c.g[A2][0] = sx(r32(c, V1).wrapping_add(r32(c, S2)));
        c.div_s(20, 21, 0);
        c.g[A1][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S3][0];
        loop {
            c.cycles += 179;
            c.lwc1(9, slot(0x0));
            c.mtc1(7, 6);
            c.cvt_s_w(6, 6);
            c.lwc1(0, slot(0xc));
            c.g[V1][0] = sx(r32(c, A3) << 4);
            c.lwc1(8, slot(0x4));
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
            c.lwc1(7, slot(0x8));
            c.sub_s(0, 0, 9);
            c.lwc1(1, slot(0x10));
            c.g[A0][0] = ((c.g[A3][0] as i64) < (c.g[S4][0] as i64)) as u64;
            c.lwc1(2, slot(0x14));
            c.swc1(21, slot(0x9c));
            c.sub_s(1, 1, 8);
            c.sub_s(2, 2, 7);
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_POS));
            c.mul_s(3, 0, 6);
            c.swc1(0, slot(0xd0));
            c.mul_s(4, 1, 6);
            c.mul_s(0, 2, 6);
            c.swc1(1, slot(0xd4));
            c.swc1(2, slot(0xd8));
            c.mul_s(5, 3, 20);
            c.mul_s(1, 4, 20);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.mul_s(2, 0, 20);
            c.swc1(3, slot(0xc0));
            c.add_s(9, 9, 5);
            c.swc1(4, slot(0xc4));
            c.add_s(8, 8, 1);
            c.swc1(0, slot(0xc8));
            c.add_s(7, 7, 2);
            c.swc1(5, slot(0xb0));
            c.swc1(9, slot(0x90));
            c.swc1(8, slot(0x94));
            c.swc1(7, slot(0x98));
            c.swc1(1, slot(0xb4));
            c.lq(2, slot(0x90));
            c.swc1(2, slot(0xb8));
            c.swc1(9, slot(0xa0));
            c.swc1(8, slot(0xa4));
            c.swc1(7, slot(0xa8));
            c.sq(2, r32(c, V1));
            c.lwc1(5, slot(0x18));
            c.lwc1(7, slot(0x1c));
            c.lwc1(8, slot(0x20));
            c.lwc1(0, slot(0x24));
            c.lwc1(1, slot(0x28));
            c.lwc1(2, slot(0x2c));
            c.sub_s(0, 0, 5);
            c.sub_s(1, 1, 7);
            c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(NODE_POS));
            c.sub_s(2, 2, 8);
            c.mul_s(3, 0, 6);
            c.swc1(0, slot(0xe0));
            c.mul_s(4, 1, 6);
            c.swc1(1, slot(0xe4));
            c.mul_s(10, 2, 6);
            c.swc1(21, slot(0x9c));
            c.mul_s(0, 3, 20);
            c.swc1(3, slot(0xc0));
            c.mul_s(1, 4, 20);
            c.swc1(4, slot(0xc4));
            c.mul_s(9, 10, 20);
            c.g[V1][0] = sx(r32(c, T2).wrapping_add(r32(c, V1)));
            c.add_s(5, 5, 0);
            c.swc1(0, slot(0xb0));
            c.add_s(7, 7, 1);
            c.swc1(1, slot(0xb4));
            c.add_s(8, 8, 9);
            c.swc1(2, slot(0xe8));
            c.swc1(5, slot(0x90));
            c.g[T2][0] = sx(r32(c, T2).wrapping_add(0x10));
            c.swc1(7, slot(0x94));
            c.swc1(8, slot(0x98));
            c.swc1(5, slot(0xa0));
            c.lq(2, slot(0x90));
            c.swc1(7, slot(0xa4));
            c.swc1(10, slot(0xc8));
            c.swc1(9, slot(0xb8));
            c.swc1(8, slot(0xa8));
            c.sq(2, r32(c, V1));
            c.lwc1(5, slot(0x30));
            c.lwc1(7, slot(0x34));
            c.lwc1(0, slot(0x38));
            c.lwc1(1, slot(0x3c));
            c.sub_s(0, 0, 5);
            c.lwc1(15, slot(0x40));
            c.sub_s(1, 1, 7);
            c.lwc1(4, slot(0x48));
            c.lwc1(14, slot(0x44));
            c.mul_s(2, 0, 6);
            c.swc1(0, slot(0xc0));
            c.mul_s(3, 1, 6);
            c.swc1(1, slot(0xc4));
            c.sub_s(4, 4, 15);
            c.lwc1(10, slot(0x4c));
            c.mul_s(0, 2, 20);
            c.swc1(2, slot(0xb0));
            c.mul_s(1, 3, 20);
            c.swc1(3, slot(0xb4));
            c.sub_s(10, 10, 14);
            c.add_s(5, 5, 0);
            c.swc1(0, slot(0xa0));
            c.add_s(7, 7, 1);
            c.swc1(1, slot(0xa4));
            c.mul_s(16, 4, 6);
            c.swc1(5, slot(0x90));
            c.mul_s(17, 10, 6);
            c.swc1(7, slot(0x94));
            c.mul_s(18, 16, 20);
            c.ldl(2, slot(0x97));
            c.ldr(2, slot(0x90));
            c.sdl(2, r32(c, T0).wrapping_add(0x7));
            c.sdr(2, r32(c, T0));
            c.mul_s(19, 17, 20);
            c.swc1(4, slot(0xc0));
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x8));
            c.lwc1(11, slot(0x54));
            c.add_s(15, 15, 18);
            c.lwc1(12, slot(0x58));
            c.add_s(14, 14, 19);
            c.lwc1(13, slot(0x5c));
            c.lwc1(3, slot(0x64));
            c.lwc1(4, slot(0x68));
            c.lwc1(5, slot(0x6c));
            c.sub_s(3, 3, 11);
            c.lwc1(8, slot(0x74));
            c.sub_s(4, 4, 12);
            c.lwc1(9, slot(0x78));
            c.sub_s(5, 5, 13);
            c.lwc1(7, slot(0x7c));
            c.mul_s(3, 3, 6);
            c.lwc1(0, slot(0x84));
            c.mul_s(4, 4, 6);
            c.lwc1(1, slot(0x88));
            c.mul_s(5, 5, 6);
            c.lwc1(2, slot(0x8c));
            c.sub_s(0, 0, 8);
            c.sub_s(1, 1, 9);
            c.swc1(15, slot(0x90));
            c.sub_s(2, 2, 7);
            c.swc1(14, slot(0x94));
            c.mul_s(0, 0, 6);
            c.swc1(10, slot(0xc4));
            c.mul_s(1, 1, 6);
            c.swc1(16, slot(0xb0));
            c.mul_s(2, 2, 6);
            c.swc1(17, slot(0xb4));
            c.mul_s(3, 3, 20);
            c.swc1(18, slot(0xa0));
            c.mul_s(4, 4, 20);
            c.swc1(19, slot(0xa4));
            c.mul_s(5, 5, 20);
            c.swc1(21, r32(c, A1));
            c.mul_s(0, 0, 20);
            c.ldl(2, slot(0x97));
            c.ldr(2, slot(0x90));
            c.sdl(2, r32(c, T1).wrapping_add(0x7));
            c.sdr(2, r32(c, T1));
            c.mul_s(1, 1, 20);
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(0x8));
            c.mul_s(2, 2, 20);
            c.add_s(11, 11, 3);
            c.add_s(12, 12, 4);
            c.add_s(13, 13, 5);
            c.add_s(8, 8, 0);
            c.swc1(11, r32(c, A1).wrapping_add(0x4));
            c.add_s(9, 9, 1);
            c.swc1(12, r32(c, A1).wrapping_add(0x8));
            c.add_s(7, 7, 2);
            c.swc1(13, r32(c, A1).wrapping_add(0xc));
            c.swc1(8, r32(c, A2).wrapping_add(0x4));
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
            c.swc1(9, r32(c, A2).wrapping_add(0x8));
            c.swc1(7, r32(c, A2).wrapping_add(0xc));
            c.swc1(21, r32(c, A2));
            let more = c.g[A0][0] != 0;
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_68b0);
            }
        }
    }

    // world->vt[...]: the mesh fills its vertices (vt[6]), then the temporaries go.
    c.cycles += 11;
    c.g[T0][0] = c.lw(r32(c, S1).wrapping_add(NODE_MESH));
    c.g[A1][0] = c.g[SP][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.g[S3][0];
    c.g[V0][0] = c.lw(r32(c, T0).wrapping_add(0x4));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(MESH_VT_DELTA));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(MESH_VT_FN));
    let fill = r32(c, V1);
    c.g[RA][0] = sx(0x0034_6ba0);
    c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
    call_ptr(c, fill, 0x0034_6ba0)?;
    if c.g[S3][0] != 0 {
        c.cycles += 2;
        c.g[RA][0] = sx(0x0034_6bb0);
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, OPERATOR_DELETE, 0x0034_6bb0)?;
    }

    c.cycles += 2;
    if c.g[S2][0] == 0 {
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_DEF));
    } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x0034_6bc0);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, OPERATOR_DELETE, 0x0034_6bc0)?;
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_DEF));
    }

    // box = def's min/max (w = 1), grown by 100 on every side; `old` keeps the
    // unexpanded box for the spatial hash.
    c.cycles += 49;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.lwc1(1, r32(c, V0).wrapping_add(DEF_MIN));
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(1, 2);
    c.swc1(1, slot(0x90));
    c.lwc1(0, r32(c, V0).wrapping_add(DEF_MIN + 0x4));
    c.swc1(0, slot(0x94));
    c.lwc1(1, r32(c, V0).wrapping_add(DEF_MIN + 0x8));
    c.swc1(3, slot(0x9c));
    c.swc1(1, slot(0x98));
    c.lq(3, slot(0x90));
    c.sq(3, r32(c, S1).wrapping_add(NODE_BOX));
    c.lwc1(0, r32(c, V0).wrapping_add(DEF_MAX));
    c.swc1(0, slot(0x90));
    c.lwc1(1, r32(c, V0).wrapping_add(DEF_MAX + 0x4));
    c.swc1(1, slot(0x94));
    c.lwc1(0, r32(c, V0).wrapping_add(DEF_MAX + 0x8));
    c.swc1(3, slot(0x9c));
    c.swc1(0, slot(0x98));
    c.swc1(2, slot(0xb8));
    c.lq(2, slot(0x90));
    c.sq(3, slot(0x90));
    c.sq(2, slot(0xa0));
    c.swc1(2, slot(0xb0));
    c.swc1(2, slot(0xb4));
    c.sw(slot(0xbc), 0);
    c.sq(2, r32(c, S1).wrapping_add(NODE_BOX + 0x10));
    c.lqc2(3, r32(c, S1).wrapping_add(NODE_BOX));
    c.lqc2(4, slot(0xb0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, r32(c, S1).wrapping_add(NODE_BOX));
    c.lqc2(3, r32(c, S1).wrapping_add(NODE_BOX + 0x10));
    c.lqc2(4, slot(0xb0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, r32(c, S1).wrapping_add(NODE_BOX + 0x10));

    // The spatial hash: nash = world->GetNash(), update with (def, box, old).
    c.g[RA][0] = sx(0x0034_6c5c);
    c.g[S0][0] = c.lw(r32(c, S1).wrapping_add(NODE_DEF));
    call_ptr(c, WORLD_GET_NASH, 0x0034_6c5c)?;
    c.g[A3][0] = c.lw(slot(0xf0));
    c.g[A0][0] = c.g[V0][0];
    c.g[T0][0] = c.lw(slot(0xfc));
    c.g[A2][0] = c.g[S0][0];
    c.g[RA][0] = sx(0x0034_6c74);
    c.g[A1][0] = 0;
    call_ptr(c, SPATIAL_HASH_UPDATE, 0x0034_6c74)?;

    // Index buffer: two triangles per quad strip, indices (base, base+n, base+1)
    // and (base+n, base+n+1, base+1).
    c.g[V0][0] = c.lw(slot(0xf8));
    c.g[T2][0] = 0;
    c.g[A3][0] = 0;
    let no_tris = (c.g[V0][0] as i64) <= 0;
    c.g[T1][0] = 0;
    if !no_tris {
        c.cycles += 2;
        c.g[T0][0] = sx(r32(c, S4).wrapping_add(0x1));
        loop {
            c.cycles += 27;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_IDX));
            c.g[A1][0] = sx(r32(c, T2).wrapping_add(r32(c, S4)));
            c.g[V1][0] = c.lw(slot(0xf8));
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(0x2));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
            c.g[A0][0] = sx(r32(c, T2).wrapping_add(0x1));
            c.sb(r32(c, V0), c.g[T2][0]);
            c.g[A2][0] = ((c.g[T1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[T2][0] = c.g[A0][0];
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_IDX));
            c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            c.sb(r32(c, V0).wrapping_add(0x1), c.g[A1][0]);
            c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(NODE_IDX));
            c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
            c.sb(r32(c, V1).wrapping_add(0x2), c.g[A0][0]);
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_IDX));
            c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            c.sb(r32(c, V0).wrapping_add(0x3), c.g[A1][0]);
            c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(NODE_IDX));
            c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
            c.sb(r32(c, V1).wrapping_add(0x4), c.g[T0][0]);
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x1));
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(NODE_IDX));
            c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            c.sb(r32(c, V0).wrapping_add(0x5), c.g[A0][0]);
            let more = c.g[A2][0] != 0;
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x6));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_6c90);
            }
        }
    }

    // Epilogue.
    c.cycles += 14;
    frame.restore(c, S0, 0x190);
    frame.restore(c, S1, 0x180);
    frame.restore(c, S2, 0x170);
    frame.restore(c, S3, 0x160);
    frame.restore(c, S4, 0x150);
    frame.restore(c, S5, 0x140);
    frame.restore(c, S6, 0x130);
    frame.restore(c, S7, 0x120);
    frame.restore(c, FP, 0x110);
    frame.restore_ra(c, 0x100);
    frame.restore_fpr(c, 21, 0x1a8);
    frame.restore_fpr(c, 20, 0x1a0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, LOG_RESULTS};
    use crate::difftest::{DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Fixture layout (inside the test data area, clear of the fakes' log).
    const OC582_NODE: u32 = DATA + 0x5000;
    const OC582_DEF: u32 = DATA + 0x5400;
    const OC582_MESH: u32 = DATA + 0x5800;
    const OC582_MESH_VT: u32 = DATA + 0x5900;
    const OC582_WORLD: u32 = DATA + 0x5c00;
    const OC582_APP: u32 = DATA + 0x6000;
    const OC582_MODULE: u32 = DATA + 0x6200;
    const OC582_NASH: u32 = DATA + 0x6400;
    const OC582_GFX: u32 = DATA + 0x8000;
    /// Bump-allocated blocks for the fake `operator new`.
    const OC582_HEAP: u32 = DATA + 0x3_0000;
    /// Gate word and the bump pointer of the fakes.
    const OC582_CTRL: u32 = DATA + 0xf_f000;
    const OC582_MAGIC: u32 = 0x4f42_5245;

    static OC582_PREV_NEW: AtomicUsize = AtomicUsize::new(0);
    static OC582_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);
    static OC582_PREV_SPATIAL: AtomicUsize = AtomicUsize::new(0);

    /// Gated `operator new`: a bump allocation in the machine's RAM.
    fn oc582_fake_new(c: &mut Cpu) {
        if c.bus.read32(OC582_CTRL) != OC582_MAGIC {
            return fakes::fall_back(c, &OC582_PREV_NEW, ee::s10::f_00317e30__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, OPERATOR_NEW, ra);
        if c.pc == ra {
            let bump = c.bus.read32(OC582_CTRL + 4);
            c.bus.write32(OC582_CTRL + 4, bump + 0x400);
            c.g[V0][0] = (OC582_HEAP + bump) as u64;
        }
    }

    /// Gated `operator delete`: logged, nothing freed.
    fn oc582_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC582_CTRL) != OC582_MAGIC {
            return fakes::fall_back(c, &OC582_PREV_DELETE, ee::s10::f_00317e98__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, OPERATOR_DELETE, ra);
    }

    /// Gated `SpatialHash_Update`: logged, nothing updated.
    fn oc582_fake_spatial(c: &mut Cpu) {
        if c.bus.read32(OC582_CTRL) != OC582_MAGIC {
            return fakes::fall_back(c, &OC582_PREV_SPATIAL, ee::s10::f_003291e0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, SPATIAL_HASH_UPDATE, ra);
    }

    fn oc582_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc582_fake_new,
            &OC582_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc582_fake_delete,
            &OC582_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003291E0),
            oc582_fake_spatial,
            &OC582_PREV_SPATIAL,
        );
    }

    /// The bridge node with `seed % 7` segments (0 covers the empty path),
    /// its instance record, the graphics manager and world with fake
    /// CreateGridMesh/GetMeshQuad/vertex-fill callees, and the real
    /// `World_GetNash` reading the app's module.
    fn oc582_setup(c: &mut Cpu, seed: u32) {
        oc582_install();
        let mut r = fakes::machine(c, seed, 0x0582_f1ab, 10);
        c.bus.write32(OC582_CTRL, OC582_MAGIC);
        c.bus.write32(OC582_CTRL + 4, 0);

        // gp-relative graphics manager -> world; world's grid and quad functions are fakes.
        c.bus.write32(GAME_GP + GFX_MAN_GP, OC582_GFX);
        c.bus.write32(OC582_GFX + GFX_MAN_WORLD, OC582_WORLD);
        c.bus.write16(OC582_WORLD + WORLD_GRID_MESH_DELTA, 0);
        c.bus
            .write32(OC582_WORLD + WORLD_GRID_MESH_FN, fakes::FAKE + 0x40);
        c.bus.write16(OC582_WORLD + WORLD_GET_QUAD_DELTA, 0);
        c.bus
            .write32(OC582_WORLD + WORLD_GET_QUAD_FN, fakes::FAKE + 0x80);
        // CreateGridMesh (call 0) returns the mesh; its vertex fill is a fake.
        c.bus.write32(LOG_RESULTS, OC582_MESH);
        c.bus.write32(OC582_MESH + 4, OC582_MESH_VT);
        c.bus.write16(OC582_MESH_VT + MESH_VT_DELTA, 0);
        c.bus
            .write32(OC582_MESH_VT + MESH_VT_FN, fakes::FAKE + 0x100);

        // The app -> module -> nash chain read by World_GetNash.
        c.bus.write32(GAME_GP - 0x848, OC582_APP);
        c.bus.write32(OC582_APP + 0x84, OC582_MODULE);
        c.bus.write32(OC582_MODULE + 0x20, OC582_NASH);

        // The bridge node and its instance record (min/max floats).
        c.bus.write32(OC582_NODE + NODE_DEF, OC582_DEF);
        c.bus.write32(OC582_NODE + NODE_COUNT, seed % 7);
        for k in 0..6u32 {
            c.bus.write32(OC582_DEF + DEF_MIN + 4 * k, r.f32(50.0));
        }
        c.g[A0][0] = OC582_NODE as u64;
        c.pc = 0x0034_66a0;
    }

    #[test]
    fn flex_bridge_build_mesh_matches() {
        fakes::covered(
            ee::s11::f_003466a0__generated,
            flex_bridge_build_mesh,
            120,
            &|c, seed| oc582_setup(c, seed),
        );
    }
}
