//! Staging module oc597 (wave l586c.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc597`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0034_a8c8, spring_rail_node_update_nodes, "SpringRail_v47")];

/// cSpringRailNode field offsets (the node array is 0x60 bytes per node).
const SR_NUM_NODES: u32 = 0x20;
const SR_MATS: u32 = 0x50;
const SR_SPRING: u32 = 0x60;
const SR_DAMP: u32 = 0x64;
const SR_NODES: u32 = 0x70;

/// The VU0 matrix constant the segment transform multiplies by (0x500000 - 0x540).
const SR_SEG_XFORM: u32 = 0x004f_ffc0;

/// One VU0 pass of the segment build: loads the vector at `vec` into VF8,
/// transforms it by the matrix at `SR_SEG_XFORM` (VF4..VF7) and stores the
/// resulting quad to `dst` (VF12). The generated body spills the vector and the
/// result to the stack; the caller keeps those stores.
fn spring_rail_seg_xform(c: &mut Cpu, vec: u32, dst: u32) {
    c.lqc2(8, vec);
    c.lqc2(4, SR_SEG_XFORM);
    c.lqc2(5, SR_SEG_XFORM.wrapping_add(0x10));
    c.lqc2(6, SR_SEG_XFORM.wrapping_add(0x20));
    c.lqc2(7, SR_SEG_XFORM.wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, dst);
}

/// `void cSpringRailNode_updateMats(cSpringRailNode* self)` (0x34a0b0),
/// `SpringRail_v48`: rebuilds the per-segment matrices (Mats, 0x40 bytes each,
/// at self+0x50) from the node array (Nodes, 0x60 bytes each, at self+0x70):
/// the first segment, then the middle segments while `i < NumNodes - 2`, then
/// the last one (which ends the loop). Each segment is a VU0 transform of the
/// node's offset vectors, and its intermediate quads are spilled to the 0x50
/// byte frame in the generated order, because those bytes are observable.
/// Leaf function: no calls, no saved RA.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn spring_rail_node_update_mats(c: &mut Cpu) {
    c.cycles += 99;
    c.g[T5][0] = c.g[A0][0];
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
    let st = sp(c);
    c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(SR_NODES));
    c.g[A1][0] = 0x500000;
    c.g[A0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(SR_MATS));
    c.g[T7][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x540));
    c.lwc1(2, c.g[V1][0] as u32);
    c.g[T6][0] = 0x1;
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(SR_SPRING));
    c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0xc0));
    c.swc1(2, st.wrapping_add(0x14));
    c.swc1(0, st.wrapping_add(0x18));
    c.swc1(1, st.wrapping_add(0x1c));
    c.swc1(2, st.wrapping_add(0x10));
    spring_rail_seg_xform(c, st.wrapping_add(0x10), st.wrapping_add(0x20));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 3);
    c.lq(2, st.wrapping_add(0x20));
    c.sq(2, st);
    c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x64));
    c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0xc4));
    c.swc1(2, st.wrapping_add(0x24));
    c.swc1(0, st.wrapping_add(0x28));
    c.swc1(1, st.wrapping_add(0x2c));
    c.swc1(2, st.wrapping_add(0x20));
    spring_rail_seg_xform(c, st.wrapping_add(0x20), st.wrapping_add(0x30));
    c.lq(2, st.wrapping_add(0x30));
    c.sq(2, st.wrapping_add(0x10));
    c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0xc8));
    c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x68));
    c.swc1(1, st.wrapping_add(0x34));
    c.swc1(0, st.wrapping_add(0x38));
    c.swc1(2, st.wrapping_add(0x3c));
    c.swc1(1, st.wrapping_add(0x30));
    spring_rail_seg_xform(c, st.wrapping_add(0x30), st.wrapping_add(0x40));
    c.lq(2, st.wrapping_add(0x40));
    c.lq(3, st);
    c.sq(2, st.wrapping_add(0x20));
    c.sq(3, c.g[A0][0] as u32);
    c.lq(2, st.wrapping_add(0x10));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lq(3, st.wrapping_add(0x20));
    c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.sw(st.wrapping_add(0x30), 0u64);
    c.sw(st.wrapping_add(0x34), 0u64);
    c.sw(st.wrapping_add(0x38), 0u64);
    c.swc1(3, st.wrapping_add(0x3c));
    c.lq(2, st.wrapping_add(0x30));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.lq(12, c.g[A0][0] as u32);
    c.lq(11, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lq(10, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.lq(9, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.pextlw(8, 11, 12);
    c.pextuw(7, 11, 12);
    c.pextlw(6, 9, 10);
    c.pextuw(3, 9, 10);
    c.pcpyld(12, 6, 8);
    c.pcpyud(11, 8, 6);
    c.pcpyld(10, 3, 7);
    c.pcpyud(9, 7, 3);
    c.sq(12, c.g[A0][0] as u32);
    c.sq(11, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.sq(10, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.sq(9, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(SR_NUM_NODES));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
    c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[T8][0] = c.g[A1][0];
    'b_34a3dc: {
        if t {
            break 'b_34a3dc;
        }
        c.cycles += 3;
        c.mov_s(4, 3);
        c.g[T4][0] = 0x60;
        'l_34a248: loop {
            c.cycles += 101;
            c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(SR_NODES));
            c.g[T3][0] = sx((c.g[T6][0] as u32) << 6);
            c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(SR_MATS));
            c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V1][0] = c.g[A0][0];
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x60));
            c.lwc1(3, c.g[V1][0] as u32);
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x60));
            c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0xc0));
            c.lwc1(0, c.g[A0][0] as u32);
            c.swc1(3, st.wrapping_add(0x14));
            c.swc1(0, st.wrapping_add(0x10));
            c.swc1(1, st.wrapping_add(0x18));
            c.swc1(2, st.wrapping_add(0x1c));
            spring_rail_seg_xform(c, st.wrapping_add(0x10), st.wrapping_add(0x20));
            c.lq(2, st.wrapping_add(0x20));
            c.sq(2, st);
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x64));
            c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0xc4));
            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.swc1(1, st.wrapping_add(0x24));
            c.swc1(0, st.wrapping_add(0x20));
            c.swc1(2, st.wrapping_add(0x28));
            c.swc1(3, st.wrapping_add(0x2c));
            spring_rail_seg_xform(c, st.wrapping_add(0x20), st.wrapping_add(0x30));
            c.lq(2, st.wrapping_add(0x30));
            c.sq(2, st.wrapping_add(0x10));
            c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0xc8));
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x68));
            c.swc1(1, st.wrapping_add(0x34));
            c.swc1(0, st.wrapping_add(0x38));
            c.swc1(2, st.wrapping_add(0x3c));
            c.swc1(1, st.wrapping_add(0x30));
            spring_rail_seg_xform(c, st.wrapping_add(0x30), st.wrapping_add(0x40));
            c.lq(2, st.wrapping_add(0x40));
            c.lq(3, st);
            c.sq(2, st.wrapping_add(0x20));
            c.sq(3, c.g[T3][0] as u32);
            c.lq(2, st.wrapping_add(0x10));
            c.sq(2, (c.g[T3][0] as u32).wrapping_add(0x10));
            c.lq(3, st.wrapping_add(0x20));
            c.sq(3, (c.g[T3][0] as u32).wrapping_add(0x20));
            c.sw(st.wrapping_add(0x30), 0u64);
            c.sw(st.wrapping_add(0x34), 0u64);
            c.sw(st.wrapping_add(0x38), 0u64);
            c.swc1(4, st.wrapping_add(0x3c));
            c.lq(2, st.wrapping_add(0x30));
            c.sq(2, (c.g[T3][0] as u32).wrapping_add(0x30));
            c.lq(10, c.g[T3][0] as u32);
            c.lq(9, (c.g[T3][0] as u32).wrapping_add(0x10));
            c.lq(8, (c.g[T3][0] as u32).wrapping_add(0x20));
            c.lq(7, (c.g[T3][0] as u32).wrapping_add(0x30));
            c.pextlw(6, 9, 10);
            c.pextuw(5, 9, 10);
            c.pextlw(4, 7, 8);
            c.pextuw(3, 7, 8);
            c.pcpyld(10, 4, 6);
            c.pcpyud(9, 6, 4);
            c.pcpyld(8, 3, 5);
            c.pcpyud(7, 5, 3);
            c.sq(10, c.g[T3][0] as u32);
            c.sq(9, (c.g[T3][0] as u32).wrapping_add(0x10));
            c.sq(8, (c.g[T3][0] as u32).wrapping_add(0x20));
            c.sq(7, (c.g[T3][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(SR_NUM_NODES));
            c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
            c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x60));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_a248);
                }
                continue 'l_34a248;
            } else {
                break 'b_34a3dc;
            }
        }
    }
    c.cycles += 99;
    c.g[V1][0] = 0x60;
    c.g[A2][0] = c.lw((c.g[T5][0] as u32).wrapping_add(SR_NODES));
    c.mult(0, 3, 14, 3);
    c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(SR_MATS));
    c.g[A3][0] = sx((c.g[T8][0] as u32).wrapping_sub(0x540));
    c.g[A0][0] = sx((c.g[T6][0] as u32) << 6);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 3);
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = c.g[A1][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x60));
    c.lwc1(2, c.g[V1][0] as u32);
    c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x60));
    c.lwc1(0, c.g[A1][0] as u32);
    c.swc1(2, st.wrapping_add(0x14));
    c.swc1(0, st.wrapping_add(0x10));
    c.swc1(1, st.wrapping_add(0x1c));
    c.swc1(1, st.wrapping_add(0x18));
    spring_rail_seg_xform(c, st.wrapping_add(0x10), st.wrapping_add(0x20));
    c.lq(2, st.wrapping_add(0x20));
    c.sq(2, st);
    c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x64));
    c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
    c.swc1(2, st.wrapping_add(0x24));
    c.swc1(1, st.wrapping_add(0x20));
    c.swc1(0, st.wrapping_add(0x2c));
    c.swc1(0, st.wrapping_add(0x28));
    spring_rail_seg_xform(c, st.wrapping_add(0x20), st.wrapping_add(0x30));
    c.lq(2, st.wrapping_add(0x30));
    c.sq(2, st.wrapping_add(0x10));
    c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x68));
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x8));
    c.swc1(0, st.wrapping_add(0x34));
    c.swc1(1, st.wrapping_add(0x30));
    c.swc1(2, st.wrapping_add(0x3c));
    c.swc1(2, st.wrapping_add(0x38));
    spring_rail_seg_xform(c, st.wrapping_add(0x30), st.wrapping_add(0x40));
    c.lq(2, st.wrapping_add(0x40));
    c.lq(3, st);
    c.sq(2, st.wrapping_add(0x20));
    c.sq(3, c.g[A0][0] as u32);
    c.lq(2, st.wrapping_add(0x10));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lq(3, st.wrapping_add(0x20));
    c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.swc1(3, st.wrapping_add(0x3c));
    c.sw(st.wrapping_add(0x30), 0u64);
    c.sw(st.wrapping_add(0x34), 0u64);
    c.sw(st.wrapping_add(0x38), 0u64);
    c.lq(2, st.wrapping_add(0x30));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.lq(11, c.g[A0][0] as u32);
    c.lq(10, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lq(9, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.lq(8, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.pextlw(7, 10, 11);
    c.pextuw(6, 10, 11);
    c.pextlw(5, 8, 9);
    c.pextuw(3, 8, 9);
    c.pcpyld(11, 5, 7);
    c.pcpyud(10, 7, 5);
    c.pcpyld(9, 3, 6);
    c.pcpyud(8, 6, 3);
    c.sq(11, c.g[A0][0] as u32);
    c.sq(10, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.sq(9, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.sq(8, (c.g[A0][0] as u32).wrapping_add(0x30));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
    c.pc = t;
}

/// `void cSpringRailNode_updateNodes(cSpringRailNode* self)` (0x34a8c8),
/// `SpringRail_v47`: one physics step of the rail's nodes. It reads the update
/// time (`cObjectInterface_getUpdateTime`, a real callee), then, for the
/// NumNodes-1 segments: scales the spring and damping torques into a per-segment
/// torque (0x90-byte scratch), integrates each node's orientation and angular
/// velocity, accumulates the offset and places the next node's position from
/// the segment vector (a normalised cross term); finally clears the node
/// tangents. Frame: 0x100 bytes, RA at +0xe0, S0 at +0xf0.
pub fn spring_rail_node_update_nodes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x100));
        c.sq(16, sp(c).wrapping_add(0xf0));
        c.sd(sp(c).wrapping_add(0xe0), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002d_1c70, 0x0034_a8dc)?;
        let st = sp(c);
        c.mov_s(1, 0);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = 0x7;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0u64;
        'b_34a988: {
            if t {
                break 'b_34a988;
            }
            c.cycles += 2;
            c.g[A2][0] = 0u64;
            c.g[A1][0] = c.g[SP][0];
            'l_34a900: loop {
                c.cycles += 34;
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(SR_SPRING));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.neg_s(0, 0);
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.mfc1(4, 0);
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, st.wrapping_add(0x90));
                c.lq(3, st.wrapping_add(0x90));
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(SR_DAMP));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.sq(3, st.wrapping_add(0x80));
                c.neg_s(0, 0);
                c.sq(3, c.g[A1][0] as u32);
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.mfc1(3, 0);
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, st.wrapping_add(0x90));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60));
                c.lq(2, st.wrapping_add(0x90));
                c.sq(2, st.wrapping_add(0x80));
                c.lqc2(3, c.g[A1][0] as u32);
                c.lqc2(4, st.wrapping_add(0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, c.g[A1][0] as u32);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_a900);
                    }
                    continue 'l_34a900;
                } else {
                    break 'b_34a988;
                }
            }
        }
        'b_34a9d4: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_34a9d4;
            }
            c.cycles += 2;
            c.g[V1][0] = c.g[SP][0];
            c.g[A1][0] = 0u64;
            'l_34a9a0: loop {
                c.cycles += 13;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lqc2(3, c.g[V1][0] as u32);
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x40));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, c.g[V1][0] as u32);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_a9a0);
                    }
                    continue 'l_34a9a0;
                } else {
                    break 'b_34a9d4;
                }
            }
        }
        'b_34aa80: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A3][0] = 0u64;
            if t {
                break 'b_34aa80;
            }
            c.cycles += 3;
            c.g[A2][0] = 0u64;
            c.g[A1][0] = c.g[SP][0];
            'l_34a9f0: loop {
                c.cycles += 36;
                c.mfc1(2, 1);
                c.lqc2(4, c.g[A1][0] as u32);
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, st.wrapping_add(0xa0));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                c.lq(2, st.wrapping_add(0xa0));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sq(2, st.wrapping_add(0x90));
                c.sq(2, st.wrapping_add(0x80));
                c.mfc1(4, 1);
                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, st.wrapping_add(0xa0));
                c.lq(2, st.wrapping_add(0xa0));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.sq(2, st.wrapping_add(0x90));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, st.wrapping_add(0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.lqc2(4, st.wrapping_add(0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_a9f0);
                    }
                    continue 'l_34a9f0;
                } else {
                    break 'b_34aa80;
                }
            }
        }
        'b_34abbc: {
            c.cycles += 8;
            c.g[V0][0] = 0x500000;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
            c.g[A2][0] = 0u64;
            c.lq(4, c.g[V0][0] as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V1][0] as i64) <= 0;
            c.sq(4, st.wrapping_add(0x80));
            if t {
                break 'b_34abbc;
            }
            c.cycles += 2;
            c.g[A1][0] = 0u64;
            'l_34aaa8: loop {
                c.cycles += 69;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lqc2(3, st.wrapping_add(0x80));
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, st.wrapping_add(0x80));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(4, 22);
                c.mtc1(4, 0);
                c.g[V1][0] = c.g[V0][0];
                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.lqc2(5, st.wrapping_add(0x80));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, st.wrapping_add(0xd0));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.lq(2, st.wrapping_add(0xd0));
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sq(2, st.wrapping_add(0xc0));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.lqc2(4, st.wrapping_add(0xc0));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, st.wrapping_add(0xd0));
                c.lq(2, st.wrapping_add(0xd0));
                c.sq(2, st.wrapping_add(0xb0));
                c.lqc2(3, st.wrapping_add(0xb0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, st.wrapping_add(0xd0));
                c.lq(2, st.wrapping_add(0xd0));
                c.sq(2, st.wrapping_add(0xa0));
                c.mfc1(3, 0);
                c.lqc2(4, st.wrapping_add(0xa0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, st.wrapping_add(0xd0));
                c.lq(2, st.wrapping_add(0xd0));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.sq(2, st.wrapping_add(0x90));
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.lqc2(3, c.g[A0][0] as u32);
                c.lqc2(4, st.wrapping_add(0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, st.wrapping_add(0xb0));
                c.lq(2, st.wrapping_add(0xb0));
                c.sq(2, st.wrapping_add(0xa0));
                c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x60));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V1][0] != 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_aaa8);
                    }
                    continue 'l_34aaa8;
                } else {
                    break 'b_34abbc;
                }
            }
        }
        'b_34abfc: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_34abfc;
            }
            c.cycles += 4;
            c.g[V0][0] = 0x500000;
            c.g[A1][0] = 0u64;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
            'l_34abd8: loop {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NODES));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.lq(2, c.g[A2][0] as u32);
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SR_NUM_NODES));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_abd8);
                    }
                    continue 'l_34abd8;
                } else {
                    break 'b_34abfc;
                }
            }
        }
        c.cycles += 4;
        c.lq(16, st.wrapping_add(0xf0));
        c.g[RA][0] = c.ld(st.wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x100));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{self, check, fill, scramble, Rng, DATA, GAME_GP};

    /// Fixture for both functions: a rail of `1..=5` nodes (the loops take every
    /// branch), random node and matrix data, and the self pointer at `DATA`.
    fn spring_rail_fixture(c: &mut Cpu, seed: u32, r: &mut Rng, entry: u32) {
        scramble(c, r);
        c.pc = entry;
        c.g[RA][0] = sx(difftest::RA);
        c.g[SP][0] = sx(difftest::SP);
        c.g[GP][0] = sx(GAME_GP);
        c.g[A0][0] = sx(DATA);
        c.bus.write32(DATA + SR_NUM_NODES, seed % 5 + 1);
        c.bus.write32(DATA + SR_MATS, DATA + 0x1000);
        c.bus.write32(DATA + SR_NODES, DATA + 0x2000);
        let edge = seed % 5 == 0;
        // The two spring/damp floats; +0x70 holds the Nodes pointer, so stop at +0x64.
        for k in 0..2 {
            let v = if edge { r.edge_f32() } else { r.f32(2.0) };
            c.bus.write32(DATA + SR_SPRING + 4 * k, v);
        }
        fill(c, r, DATA + 0x1000, 0x100);
        fill(c, r, DATA + 0x2000, 0x180);
        fill(c, r, SR_SEG_XFORM, 16);
        fill(c, r, 0x004f_f120, 16);
    }

    #[test]
    fn spring_rail_node_update_nodes_matches() {
        check(
            ee::s11::f_0034a8c8__generated,
            spring_rail_node_update_nodes,
            300,
            &|c, seed| {
                let mut r = Rng(0x34a8_c8aa ^ seed as u64);
                spring_rail_fixture(c, seed, &mut r, 0x0034_a8c8);
                // The update-time callee reads world->+0x14 through GP-0x848.
                c.bus.write32(GAME_GP - 0x848, DATA + 0x4000);
                c.bus.write32(DATA + 0x4014, r.f32(2.0));
            },
        );
    }
}
