//! Staging module oc583 (wave wave233): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc583`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0034_75d8,
    flexbridge_update_verts,
    "FlexBridge_UpdateVerts",
)];

/// `cFlexBridgeNode` fields the update touches.
pub(crate) const FB_WIDTH: u32 = 0x50; // Mesh.Width (spring node count)
pub(crate) const FB_NODES: u32 = 0x74; // Mesh.Nodes, tSpringNode[Width]
pub(crate) const FB_NODE_SIZE: u32 = 0x50;
pub(crate) const FB_NODE_P: u32 = 0x00; // node->P (vec4)
pub(crate) const FB_NODE_O: u32 = 0x20; // node->O (the sway angle)
pub(crate) const FB_INSTANCE: u32 = 0x18; // Nash object id
pub(crate) const FB_BOTTOM_LEFT: u32 = 0x20; // BottomLeftOffset (vec4)
pub(crate) const FB_BOTTOM_RIGHT: u32 = 0x30; // BottomRightOffset (vec4)
pub(crate) const FB_ROOT_VPTR: u32 = 0x0c; // vtable of getRootTransform
pub(crate) const FB_GRID_MESH: u32 = 0x78; // GridMesh object (vtable slot 3)
pub(crate) const FB_VERTS: u32 = 0x7c; // Verts, vec4[2 * Width]
pub(crate) const FB_FACE_NORMS: u32 = 0x80; // FaceNorms, vec4[2 * quads]
pub(crate) const FB_BOUND_MIN: u32 = 0x90; // BoundBox.MinPoint
pub(crate) const FB_BOUND_MAX: u32 = 0xa0; // BoundBox.MaxPoint
pub(crate) const FB_FRAME: u32 = 0x2d0; // stack frame size

/// `void FlexBridge_UpdateVerts(cFlexBridgeNode* self)` (0x3475d8),
/// `cFlexBridgeNode::updateGrid`. Places the bridge's two rows of vertices
/// from its spring nodes: each node's `BottomRight` / `BottomLeft` offset
/// is turned by the node's angle `O` (`cos`) and lifted by the up / side
/// reference (`sin`). Then it builds the two face normals of every quad,
/// takes the bounding box of the verts after the root transform, stores it
/// in `BoundBox`, moves the object in the Nash spatial hash
/// (`nash_movenode(World_GetNash(), 0, Instance, &BoundBox, &old)`) and
/// hands the vertex positions to the grid mesh (vtable slot 3).
///
/// The stack frame (0x2d0 bytes): the references at +0x0 / +0x10 / +0x20..,
/// verts scratch at +0x20..+0x70, the sin/cos pair at +0x1f0 / +0x1f4, and
/// `pos[]` (12-byte points) at +0x70. Every register and cycle count follows
/// the generated body (VU0 macro ops are issued as the original does).
pub fn flexbridge_update_verts(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: save ra, s0-s7, fp and f20-f25 (`sq` is 128-bit).
        c.cycles += 62;
        c.g[SP][0] = sx(sp(c).wrapping_sub(FB_FRAME));
        c.sq(17, sp(c).wrapping_add(0x280));
        c.sq(18, sp(c).wrapping_add(0x270));
        c.g[S1][0] = c.g[A0][0];
        c.sq(21, sp(c).wrapping_add(0x240));
        c.g[A0][0] = 0x500000;
        c.sq(22, sp(c).wrapping_add(0x230));
        c.g[S2][0] = 0;
        c.sq(23, sp(c).wrapping_add(0x220));
        c.sq(30, sp(c).wrapping_add(0x210));
        c.g[S7][0] = sx(sp(c).wrapping_add(0x20));
        c.sq(16, sp(c).wrapping_add(0x290));
        c.g[FP][0] = sx(sp(c).wrapping_add(0x70));
        c.sq(20, sp(c).wrapping_add(0x250));
        c.sd(sp(c).wrapping_add(0x200), c.g[RA][0]);
        c.swc1(25, sp(c).wrapping_add(0x2c8));
        c.swc1(24, sp(c).wrapping_add(0x2c0));
        c.swc1(23, sp(c).wrapping_add(0x2b8));
        c.swc1(22, sp(c).wrapping_add(0x2b0));
        c.swc1(21, sp(c).wrapping_add(0x2a8));
        c.swc1(20, sp(c).wrapping_add(0x2a0));
        c.sq(19, sp(c).wrapping_add(0x260));

        // n = Width. upRef = ZAxis * |BottomRight| (at sp+0), sideRef =
        // ZAxis * |BottomLeft| (at sp+0x10), both through VU0.
        c.g[S3][0] = c.lw(r32(c, S1).wrapping_add(FB_WIDTH));
        c.lqc2(3, r32(c, S1).wrapping_add(FB_BOTTOM_RIGHT));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 0);
        c.mfc1(3, 0);
        c.lqc2(4, r32(c, A0).wrapping_sub(0xea0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp(c).wrapping_add(0x10));
        c.lq(2, sp(c).wrapping_add(0x10));

        c.lqc2(3, r32(c, S1).wrapping_add(FB_BOTTOM_LEFT));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(5, 22);
        c.mtc1(5, 0);
        c.g[V1][0] = sx(r32(c, S3).wrapping_sub(1));
        c.g[S6][0] = sx(r32(c, S3) << 1);
        c.sq(2, sp(c));
        c.g[S5][0] = sx(r32(c, V1) << 1);
        c.mfc1(2, 0);
        c.lqc2(4, r32(c, A0).wrapping_sub(0xea0));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp(c).wrapping_add(0x20));
        c.lq(2, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.lw(r32(c, S1).wrapping_add(FB_NODES));
        let no_nodes = (c.g[S3][0] as i64) <= 0;
        c.sq(2, sp(c).wrapping_add(0x10));

        // Vertex rows: verts[i] = P + BR*cos + upRef*sin and
        // verts[n + i] = P + BL*cos - sideRef*sin, for each node i.
        if !no_nodes {
            c.cycles += 2;
            c.g[S4][0] = sx(r32(c, S3) << 4);
            loop {
                c.cycles += 72;
                c.lwc1(12, r32(c, S0).wrapping_add(FB_NODE_O));
                c.g[A0][0] = sx(sp(c).wrapping_add(0x1f0));
                c.g[RA][0] = sx(0x0034_76e8);
                c.g[A1][0] = sx(sp(c).wrapping_add(0x1f4));
                // fSinCos(&sin, &cos, node->O)
                call_ptr(c, 0x0031_be50, 0x0034_76e8)?;
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.g[A0][0] = sx(r32(c, S2) << 4);
                c.lwc1(0, sp(c).wrapping_add(0x1f4));
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x1));
                c.mfc1(2, 0);
                c.lqc2(4, r32(c, S1).wrapping_add(FB_BOTTOM_RIGHT));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                c.lq(2, sp(c).wrapping_add(0x50));
                c.g[A2][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                c.sq(2, sp(c).wrapping_add(0x40));
                c.lqc2(3, r32(c, S0).wrapping_add(FB_NODE_P));
                c.lqc2(4, sp(c).wrapping_add(0x40));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.lq(2, sp(c).wrapping_add(0x50));
                c.lwc1(0, sp(c).wrapping_add(0x1f0));
                c.sq(2, sp(c).wrapping_add(0x30));
                c.mfc1(3, 0);
                c.lqc2(4, sp(c));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x50));
                c.lqc2(3, sp(c).wrapping_add(0x30));
                c.lqc2(4, sp(c).wrapping_add(0x50));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x20));
                c.sq(2, r32(c, A0));
                c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.lwc1(0, sp(c).wrapping_add(0x1f4));
                c.mfc1(2, 0);
                c.lqc2(4, r32(c, S1).wrapping_add(FB_BOTTOM_LEFT));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.g[A1][0] = sx(r32(c, S4).wrapping_add(r32(c, A1)));
                c.lq(2, sp(c).wrapping_add(0x50));
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(0x10));
                c.sq(2, sp(c).wrapping_add(0x40));
                c.lqc2(3, r32(c, S0).wrapping_add(FB_NODE_P));
                c.lqc2(4, sp(c).wrapping_add(0x40));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.lq(2, sp(c).wrapping_add(0x50));
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(FB_NODE_SIZE));
                c.lwc1(0, sp(c).wrapping_add(0x1f0));
                c.sq(2, sp(c).wrapping_add(0x30));
                c.mfc1(3, 0);
                c.lqc2(4, sp(c).wrapping_add(0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x50));
                c.lqc2(3, sp(c).wrapping_add(0x30));
                c.lqc2(4, sp(c).wrapping_add(0x50));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x20));
                let more = c.g[A2][0] != 0;
                c.sq(2, r32(c, A1));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_76d8);
                }
            }
        }

        // Face normals: for each quad j (k = 2j, ntris = 2 * (n - 1)),
        // normals of the two triangles, normalised (rsqrt).
        c.cycles += 4;
        c.g[T3][0] = 0;
        c.g[T2][0] = 0;
        let no_tris = (c.g[S5][0] as i64) <= 0;
        c.g[S2][0] = sx(r32(c, S1).wrapping_add(FB_BOUND_MIN));
        if !no_tris {
            c.cycles += 2;
            c.g[T1][0] = sx(r32(c, S3) << 4);
            loop {
                c.cycles += 89;
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.g[A1][0] = sx(r32(c, T3) << 4);
                c.g[A2][0] = c.lw(r32(c, S1).wrapping_add(FB_FACE_NORMS));
                c.g[A3][0] = sx(r32(c, T2) << 4);
                c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.g[T2][0] = sx(r32(c, T2).wrapping_add(0x2));
                c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, V0)));
                c.g[A2][0] = sx(r32(c, A3).wrapping_add(r32(c, A2)));
                c.lqc2(3, r32(c, V0).wrapping_add(0x10));
                c.lqc2(4, r32(c, V1));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.g[T3][0] = sx(r32(c, T3).wrapping_add(0x1));
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.g[T0][0] = ((c.g[T2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                c.lq(2, sp(c).wrapping_add(0x50));
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                c.sq(2, sp(c).wrapping_add(0x40));
                c.g[V1][0] = sx(r32(c, T1).wrapping_add(r32(c, V1)));
                c.lqc2(3, r32(c, V1));
                c.lqc2(4, r32(c, A0));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x50));
                c.lqc2(4, sp(c).wrapping_add(0x40));
                c.lqc2(5, sp(c).wrapping_add(0x50));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x30));
                c.lqc2(3, sp(c).wrapping_add(0x30));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x20));
                c.sq(2, r32(c, A2));
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(FB_FACE_NORMS));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                c.g[V1][0] = c.g[V0][0];
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                c.lqc2(3, r32(c, V1).wrapping_add(0x10));
                c.lqc2(4, r32(c, V0));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x50));
                c.lq(2, sp(c).wrapping_add(0x50));
                c.sq(2, sp(c).wrapping_add(0x40));
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                c.g[V1][0] = sx(r32(c, T1).wrapping_add(r32(c, V1)));
                c.lqc2(3, r32(c, V1).wrapping_add(0x10));
                c.lqc2(4, r32(c, A1));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.g[T1][0] = sx(r32(c, T1).wrapping_add(0x10));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x50));
                c.lqc2(4, sp(c).wrapping_add(0x40));
                c.lqc2(5, sp(c).wrapping_add(0x50));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x30));
                c.lqc2(3, sp(c).wrapping_add(0x30));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, sp(c).wrapping_add(0x60));
                c.lq(2, sp(c).wrapping_add(0x60));
                c.sq(2, sp(c).wrapping_add(0x20));
                let more = c.g[T0][0] != 0;
                c.sq(2, r32(c, A3).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_7810);
                }
            }
        }

        // Bounds: the verts (3 floats each, into pos[] at sp+0x70) through
        // the root transform (`getRootTransform`, virtual at this+0xc);
        // min / max per axis in f22 / f20 / f23 (min) and f24 / f21 / f25 (max).
        c.cycles += 16;
        c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(FB_ROOT_VPTR));
        c.g[V0][0] = 0x1f;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = c.lh(r32(c, A1).wrapping_add(0xc0));
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0xc4));
        c.lwc1(20, r32(c, GP).wrapping_sub(0x2b3c));
        c.lwc1(21, r32(c, GP).wrapping_sub(0x2b38));
        let root_fn = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, root_fn, 0x0034_7998)?;
        c.g[A2][0] = c.g[V0][0];
        c.mov_s(22, 20);
        c.mov_s(23, 20);
        c.mov_s(24, 21);
        c.mov_s(25, 21);
        let no_verts = (c.g[S6][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !no_verts {
            c.cycles += 1;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x70));
            loop {
                // Copy vert i (x, y, z) to pos[i], transform it by the root
                // matrix (A2), and fold its x / y / z into the min / max.
                c.cycles += 27;
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(FB_VERTS));
                c.g[V1][0] = sx(r32(c, A1) << 4);
                c.mov_s(2, 22);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.lwc1(0, r32(c, V1));
                c.swc1(0, r32(c, A0));
                c.lwc1(1, r32(c, V1).wrapping_add(0x4));
                c.swc1(1, r32(c, A0).wrapping_add(0x4));
                c.lwc1(0, r32(c, V1).wrapping_add(0x8));
                c.swc1(0, r32(c, A0).wrapping_add(0x8));
                c.lqc2(8, r32(c, V1));
                c.lqc2(4, r32(c, A2));
                c.lqc2(5, r32(c, A2).wrapping_add(0x10));
                c.lqc2(6, r32(c, A2).wrapping_add(0x20));
                c.lqc2(7, r32(c, A2).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp(c).wrapping_add(0x30));
                c.lq(2, sp(c).wrapping_add(0x30));
                c.sq(2, sp(c).wrapping_add(0x20));
                // minX: w.x < minX ? w.x : minX (f2 holds the candidate)
                c.lwc1(0, sp(c).wrapping_add(0x20));
                c.c_lt_s(0, 22);
                if c.fcc() {
                    c.mov_s(2, 0);
                }
                // maxX: maxX < w.x ? w.x : maxX
                c.cycles += 4;
                c.c_lt_s(24, 0);
                c.mov_s(22, 2);
                let not_gt = !c.fcc();
                c.mov_s(1, 24);
                if !not_gt {
                    c.cycles += 1;
                    c.mov_s(1, 0);
                }
                // minY
                c.cycles += 7;
                c.lwc1(0, sp(c).wrapping_add(0x24));
                c.mov_s(24, 1);
                c.mov_s(2, 20);
                c.c_lt_s(0, 20);
                let not_lt = !c.fcc();
                c.mov_s(1, 0);
                if !not_lt {
                    c.cycles += 1;
                    c.mov_s(2, 1);
                }
                // maxY
                c.cycles += 4;
                c.c_lt_s(21, 0);
                c.mov_s(20, 2);
                let not_gt = !c.fcc();
                c.mov_s(1, 21);
                if !not_gt {
                    c.cycles += 1;
                    c.mov_s(1, 0);
                }
                // minZ
                c.cycles += 7;
                c.lwc1(0, sp(c).wrapping_add(0x28));
                c.mov_s(21, 1);
                c.mov_s(2, 23);
                c.c_lt_s(0, 23);
                let not_lt = !c.fcc();
                c.mov_s(1, 0);
                if !not_lt {
                    c.cycles += 1;
                    c.mov_s(2, 1);
                }
                // maxZ
                c.cycles += 4;
                c.c_lt_s(25, 0);
                c.mov_s(23, 2);
                let not_gt = !c.fcc();
                c.mov_s(1, 25);
                if !not_gt {
                    c.cycles += 1;
                    c.mov_s(1, 0);
                }
                c.cycles += 5;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1));
                c.mov_s(25, 1);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S6][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_79b8);
                }
            }
        }

        // Store the box: corner = (min, 1.0) into BoundBox.MinPoint, and
        // (max, 1.0) into MaxPoint; the old box is copied to the stack first.
        c.cycles += 50;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.swc1(22, sp(c).wrapping_add(0x40));
        c.swc1(20, sp(c).wrapping_add(0x44));
        c.swc1(23, sp(c).wrapping_add(0x48));
        c.swc1(0, sp(c).wrapping_add(0x4c));
        c.lq(4, r32(c, S1).wrapping_add(FB_BOUND_MIN));
        c.lq(5, sp(c).wrapping_add(0x40));
        c.swc1(24, sp(c).wrapping_add(0x40));
        c.swc1(21, sp(c).wrapping_add(0x44));
        c.swc1(25, sp(c).wrapping_add(0x48));
        c.lq(2, r32(c, S1).wrapping_add(FB_BOUND_MAX));
        c.sq(4, sp(c).wrapping_add(0x20));
        c.lq(3, sp(c).wrapping_add(0x40));
        c.sq(2, sp(c).wrapping_add(0x30));
        c.sq(3, r32(c, S1).wrapping_add(FB_BOUND_MAX));
        c.g[S0][0] = c.lw(r32(c, S1).wrapping_add(FB_INSTANCE));
        c.sq(5, r32(c, S1).wrapping_add(FB_BOUND_MIN));

        // nash_movenode(World_GetNash(), 0, Instance, &BoundBox, &old)
        call_ptr(c, 0x002d_1be0, 0x0034_7b00)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S7][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0032_91e0, 0x0034_7b18)?;

        // gridMesh->vtable[3](gridMesh + delta, pos)
        c.g[A2][0] = c.lw(r32(c, S1).wrapping_add(FB_GRID_MESH));
        c.g[A1][0] = c.g[FP][0];
        c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x4));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x18));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1c));
        let grid_fn = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, grid_fn, 0x0034_7b34)?;

        // Epilogue: restore s0-s7 / fp / f20-f25 and ra, pop the frame.
        c.lq(16, sp(c).wrapping_add(0x290));
        c.lq(17, sp(c).wrapping_add(0x280));
        c.lq(18, sp(c).wrapping_add(0x270));
        c.lq(19, sp(c).wrapping_add(0x260));
        c.lq(20, sp(c).wrapping_add(0x250));
        c.lq(21, sp(c).wrapping_add(0x240));
        c.lq(22, sp(c).wrapping_add(0x230));
        c.lq(23, sp(c).wrapping_add(0x220));
        c.lq(30, sp(c).wrapping_add(0x210));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x200));
        c.lwc1(25, sp(c).wrapping_add(0x2c8));
        c.lwc1(24, sp(c).wrapping_add(0x2c0));
        c.lwc1(23, sp(c).wrapping_add(0x2b8));
        c.lwc1(22, sp(c).wrapping_add(0x2b0));
        c.lwc1(21, sp(c).wrapping_add(0x2a8));
        c.lwc1(20, sp(c).wrapping_add(0x2a0));
        let t = r32(c, RA);
        c.g[SP][0] = sx(sp(c).wrapping_add(FB_FRAME));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{put_vec, put_words, Rng, DATA, GAME_GP};

    const FB_TEST_SEEDS: u32 = 200;
    /// The bridge object, its spring nodes, verts, face normals, the grid
    /// mesh and the two vtables (root transform at this+0xc, grid mesh at +4).
    pub(crate) const FB_TEST_SELF: u32 = DATA + 0x1000;
    pub(crate) const FB_TEST_NODES: u32 = DATA + 0x2000;
    pub(crate) const FB_TEST_VERTS: u32 = DATA + 0x4000;
    pub(crate) const FB_TEST_NORMS: u32 = DATA + 0x6000;
    pub(crate) const FB_TEST_GRID: u32 = DATA + 0x8000;
    pub(crate) const FB_TEST_GRID_VT: u32 = DATA + 0x9000;
    pub(crate) const FB_TEST_ROOT_VT: u32 = DATA + 0xa000;

    /// A random bridge with 0..=3 spring nodes (0 and 1 skip the loops that
    /// need two or more verts), fake virtual methods, and the Nash grid
    /// constants that SpatialKey_FromAABB needs.
    fn put_flexbridge(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let width = seed % 4;
        c.bus.write32(FB_TEST_SELF + FB_WIDTH, width);
        c.bus.write32(FB_TEST_SELF + FB_NODES, FB_TEST_NODES);
        c.bus
            .write32(FB_TEST_SELF + FB_INSTANCE, seed.wrapping_add(5));
        c.bus.write32(FB_TEST_SELF + FB_GRID_MESH, FB_TEST_GRID);
        c.bus.write32(FB_TEST_SELF + FB_VERTS, FB_TEST_VERTS);
        c.bus.write32(FB_TEST_SELF + FB_FACE_NORMS, FB_TEST_NORMS);
        c.bus.write32(FB_TEST_SELF + FB_ROOT_VPTR, FB_TEST_ROOT_VT);
        c.bus.write32(FB_TEST_GRID + 4, FB_TEST_GRID_VT);
        // Slot 3 of the grid mesh vtable: this delta and a fake method.
        let grid_delta = ((r.next_u32() % 64) & !3) as u16;
        c.bus.write16(FB_TEST_GRID_VT + 0x18, grid_delta);
        c.bus.write32(FB_TEST_GRID_VT + 0x1c, FAKE + 0x40);
        // getRootTransform: slot at +0xc0 (delta) / +0xc4 (fake method).
        let root_delta = ((r.next_u32() % 64) & !3) as u16;
        c.bus.write16(FB_TEST_ROOT_VT + 0xc0, root_delta);
        c.bus.write32(FB_TEST_ROOT_VT + 0xc4, FAKE + 0x80);
        put_vec(
            c,
            FB_TEST_SELF + FB_BOTTOM_RIGHT,
            [r.f32(4.0), r.f32(4.0), r.f32(4.0), 0x3f80_0000],
        );
        put_vec(
            c,
            FB_TEST_SELF + FB_BOTTOM_LEFT,
            [r.f32(4.0), r.f32(4.0), r.f32(4.0), 0x3f80_0000],
        );
        for i in 0..width {
            let node = FB_TEST_NODES + FB_NODE_SIZE * i;
            put_vec(
                c,
                node + FB_NODE_P,
                [r.f32(20.0), r.f32(20.0), r.f32(20.0), 0x3f80_0000],
            );
            c.bus.write32(node + FB_NODE_O, r.f32(3.0));
        }
        // The Nash grid constants (gp-0x2df8 / -0x2df4) for SpatialKey_FromAABB.
        put_words(c, GAME_GP - 0x2df8, &[0x3f36_db6e, 0x3e4c_cccd]);
        c.g[A0][0] = FB_TEST_SELF as u64;
    }

    #[test]
    fn flexbridge_update_verts_matches() {
        covered(
            ssx3_ee::s11::f_003475d8__generated,
            flexbridge_update_verts,
            FB_TEST_SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3475_d8, 6);
                put_flexbridge(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn flexbridge_update_verts_events() {
        check_events(
            ssx3_ee::s11::f_003475d8__generated,
            flexbridge_update_verts,
            FB_TEST_SEEDS,
            &|c, seed| -> Events {
                let mut r = machine(c, seed, 0x3475_d8 ^ 0xe7e7, 0);
                put_flexbridge(c, &mut r, seed);
                schedule(c, &mut r, seed, 0x0034_75d8)
            },
        );
    }
}
