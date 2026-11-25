//! Staging module oc604 (wave l587a.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc604`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0034_f120,
        floating_node_build_matrices,
        "Floating_BuildMatrices",
    ),
    (
        0x0034_f600,
        floating_node_apply_force,
        "cFloatingNode_applyForce",
    ),
    (
        0x0034_f790,
        floating_node_update_intersect_info,
        "cFloatingNode_updateIntersectInfo",
    ),
    (0x0035_0cb8, dead_fade_render, "DeadFade_Render"),
];

/// `base + off` as a guest address (the 32-bit wrap of a 64-bit register).
#[inline]
fn oc604_ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `lqc2 vf..vf+3` from the address in GPR `reg`, `+0x10`, `+0x20`, `+0x30`.
fn oc604_quad_load(c: &mut Cpu, vf: usize, reg: usize) {
    let base = c.g[reg][0] as u32;
    for k in 0..4 {
        c.lqc2(vf + k, base.wrapping_add(0x10 * k as u32));
    }
}

/// `sqc2 vf..vf+3` to the address in GPR `reg`, `+0x10`, `+0x20`, `+0x30`.
fn oc604_quad_store(c: &mut Cpu, vf: usize, reg: usize) {
    let base = c.g[reg][0] as u32;
    for k in 0..4 {
        c.sqc2(vf + k, base.wrapping_add(0x10 * k as u32));
    }
}

/// One matrix-times-column step on VU0: rows vf4..vf7 times the broadcast
/// components of `col`, into `dst` (the generated `mula`/`madda`/`madd` quartet).
fn oc604_mul4(c: &mut Cpu, col: u8, dst: u8) {
    c.vu0().mula(0b1111, 4, Src::Bc(col, X));
    c.vu0().madda(0b1111, 5, Src::Bc(col, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(col, Z));
    c.vu0().madd(0b1111, dst, 7, Src::Bc(col, W));
}

/// The four column products of a 4x4 transform: rows vf4..vf7, columns
/// vf8..vf11, results vf12..vf15.
fn oc604_mat4_columns(c: &mut Cpu) {
    for k in 0..4u8 {
        oc604_mul4(c, 8 + k, 12 + k);
    }
}

/// The `add`/`mul`/`adda`/`madda`/`madd` pass that combines the two
/// vectors loaded in vf3 and vf5 (the generated axis-product sequence).
fn oc604_axis_pass(c: &mut Cpu) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

// ---------------------------------------------------------------------------
// cFloatingNode::updateOverlapSystem (0x34ef40), `Floating_v50`
// ---------------------------------------------------------------------------

/// `void cFloatingNode::updateOverlapSystem(void)` (0x34ef40): with an
/// `Info` block, the node's bounding box is its position (`nodePos`, from the
/// transform at +0xc through the vtable slot at +0xc0/+0xc4) plus and minus
/// `Info->Radius` (+0x20) on all three axes; the node is then moved in the
/// overlap system through `Nash::movenode` with the old and new boxes. Without
/// `Info` the base class does it (`cMoveNode::updateOverlapSystem`, 0x3568b0).
///
/// `self` (s0): `Info` at +0x78, `Instance` at +0x18.
fn floating_v50_body(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0xa0));
    c.sq(16, sp(c).wrapping_add(0x90));
    c.sd(sp(c).wrapping_add(0x80), c.g[RA][0]);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = c.lw(oc604_ea(c.g[S0][0], 0x78));
    if c.g[A0][0] != 0 {
        c.cycles += 52;
        c.lq(3, c.g[A0][0] as u32);
        c.g[A1][0] = c.lw(oc604_ea(c.g[S0][0], 0xc));
        c.sq(3, sp(c).wrapping_add(0x20));
        c.lq(2, oc604_ea(c.g[A0][0], 0x10));
        c.sq(2, sp(c).wrapping_add(0x30));
        c.lwc1(0, oc604_ea(c.g[A0][0], 0x20));
        c.sw(sp(c).wrapping_add(0x4c), 0u64);
        c.swc1(0, sp(c).wrapping_add(0x48));
        c.swc1(0, sp(c).wrapping_add(0x40));
        c.swc1(0, sp(c).wrapping_add(0x44));
        c.g[A0][0] = c.lh(oc604_ea(c.g[A1][0], 0xc0));
        c.g[V0][0] = c.lw(oc604_ea(c.g[A1][0], 0xc4));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0034_ef94)?;
        c.lq(3, oc604_ea(c.g[V0][0], 0x30));
        c.sq(3, sp(c).wrapping_add(0x60));
        c.lqc2(3, sp(c).wrapping_add(0x60));
        c.lqc2(4, sp(c).wrapping_add(0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x70));
        c.lq(2, sp(c).wrapping_add(0x70));
        c.g[V1][0] = c.lw(oc604_ea(c.g[S0][0], 0xc));
        c.sq(2, sp(c));
        c.sq(2, sp(c).wrapping_add(0x50));
        c.g[A0][0] = c.lh(oc604_ea(c.g[V1][0], 0xc0));
        c.g[V0][0] = c.lw(oc604_ea(c.g[V1][0], 0xc4));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0034_efcc)?;
        c.lq(3, oc604_ea(c.g[V0][0], 0x30));
        c.sq(3, sp(c).wrapping_add(0x60));
        c.lqc2(3, sp(c).wrapping_add(0x60));
        c.lqc2(4, sp(c).wrapping_add(0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x70));
        c.lq(2, sp(c).wrapping_add(0x70));
        c.g[A0][0] = c.lw(oc604_ea(c.g[S0][0], 0x78));
        c.lq(3, sp(c));
        c.sq(2, sp(c).wrapping_add(0x10));
        c.sq(2, sp(c).wrapping_add(0x50));
        c.sq(3, c.g[A0][0] as u32);
        c.lq(2, sp(c).wrapping_add(0x10));
        c.sq(2, oc604_ea(c.g[A0][0], 0x10));
        c.g[S0][0] = c.lw(oc604_ea(c.g[S0][0], 0x18));
        call_ptr(c, 0x002d_1be0, 0x0034_f00c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A3][0] = c.g[SP][0];
        c.g[T0][0] = sx(sp(c).wrapping_add(0x20));
        call_ptr(c, 0x0032_91e0, 0x0034_f024)?;
        c.lq(16, sp(c).wrapping_add(0x90));
    } else {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0035_68b0, 0x0034_f034)?;
        c.lq(16, sp(c).wrapping_add(0x90));
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x80));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0xa0));
    c.pc = t;
    Ok(())
}

/// `cFloatingNode::updateOverlapSystem` (0x34ef40), see [`floating_v50_body`].
/// Not installed: its test runs the real nash move away (see the wave notes).
#[allow(dead_code)]
pub fn floating_node_update_overlap_system(c: &mut Cpu) {
    guest(c, floating_v50_body)
}

// ---------------------------------------------------------------------------
// Floating_BuildMatrices (0x34f120)
// ---------------------------------------------------------------------------

/// `Floating_BuildMatrices` (0x34f120): for each segment of the node's
/// spline (`count` at `[t6+4]`, the entries `t6`, `t6+0x40`... of 0x40 bytes
/// in the sample table at `self->Instance+0x80`), it evaluates the four
/// cubic basis rows on VU0 (`0x31be50` gives the segment weights) and writes the
/// per-sample matrices to the three output buffers `T5`, `T4`, `T3`, `T1`,
/// `T2` (generated register roles kept as named). The `-1` index entries are
/// skipped (the second, shorter path).
fn floating_build_matrices_body(c: &mut Cpu) -> Flow {
    c.cycles += 93;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x270));
    c.sq(16, sp(c).wrapping_add(0x260));
    c.sq(17, sp(c).wrapping_add(0x250));
    c.g[S0][0] = c.g[A0][0];
    c.sq(18, sp(c).wrapping_add(0x240));
    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c));
    c.sq(19, sp(c).wrapping_add(0x230));
    c.g[S2][0] = c.g[A1][0];
    c.sd(sp(c).wrapping_add(0x220), c.g[RA][0]);
    c.g[S1][0] = c.g[A2][0];
    c.g[A0][0] = sx(sp(c).wrapping_add(0x200));
    c.g[A1][0] = sx(sp(c).wrapping_add(0x204));
    c.lq(2, oc604_ea(c.g[S2][0], 0x30));
    c.lwc1(1, oc604_ea(c.g[S0][0], 0x50));
    c.sq(2, sp(c).wrapping_add(0x40));
    c.lwc1(0, sp(c).wrapping_add(0x48));
    c.lwc1(2, oc604_ea(c.g[V1][0], 0x28));
    c.sub_s(1, 1, 0);
    c.lwc1(3, oc604_ea(c.g[V1][0], 0x38));
    c.mov_s(12, 2);
    c.swc1(2, sp(c).wrapping_add(0x60));
    c.swc1(3, sp(c).wrapping_add(0x64));
    c.swc1(1, sp(c).wrapping_add(0x58));
    c.sw(sp(c).wrapping_add(0x50), 0u64);
    c.sw(sp(c).wrapping_add(0x54), 0u64);
    c.sw(sp(c).wrapping_add(0x68), 0u64);
    call_ptr(c, 0x0031_be50, 0x0034_f18c)?;
    c.lwc1(12, sp(c).wrapping_add(0x64));
    c.g[A0][0] = sx(sp(c).wrapping_add(0x208));
    c.g[A1][0] = sx(sp(c).wrapping_add(0x20c));
    call_ptr(c, 0x0031_be50, 0x0034_f19c)?;
    c.lwc1(12, sp(c).wrapping_add(0x68));
    c.g[A0][0] = sx(sp(c).wrapping_add(0x210));
    c.g[A1][0] = sx(sp(c).wrapping_add(0x214));
    call_ptr(c, 0x0031_be50, 0x0034_f1ac)?;

    // Segment weights and the first basis rows (scalar FPU).
    c.lwc1(0, sp(c).wrapping_add(0x200));
    c.lwc1(2, sp(c).wrapping_add(0x208));
    c.lwc1(7, sp(c).wrapping_add(0x204));
    c.neg_s(13, 0);
    c.lwc1(10, sp(c).wrapping_add(0x210));
    c.mul_s(8, 0, 2);
    c.lwc1(9, sp(c).wrapping_add(0x214));
    c.mul_s(3, 7, 2);
    c.mul_s(16, 7, 10);
    c.lwc1(1, sp(c).wrapping_add(0x50));
    c.lwc1(4, sp(c).wrapping_add(0x20c));
    c.mul_s(14, 0, 9);
    c.swc1(1, sp(c).wrapping_add(0x30));
    c.mul_s(15, 8, 10);
    c.neg_s(6, 4);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 17);
    c.mul_s(1, 7, 9);
    c.lwc1(11, sp(c).wrapping_add(0x54));
    c.mul_s(5, 16, 2);
    c.lwc1(12, sp(c).wrapping_add(0x58));
    c.mul_s(0, 0, 10);
    c.swc1(11, sp(c).wrapping_add(0x34));
    c.mul_s(3, 3, 9);
    c.swc1(12, sp(c).wrapping_add(0x38));
    c.mul_s(8, 8, 9);
    c.swc1(2, sp(c).wrapping_add(0x8));
    c.mul_s(7, 7, 4);
    c.sw(sp(c).wrapping_add(0xc), 0u64);
    c.mul_s(13, 13, 4);
    c.sw(sp(c).wrapping_add(0x1c), 0u64);
    c.sub_s(0, 0, 3);
    c.sw(sp(c).wrapping_add(0x2c), 0u64);
    c.add_s(5, 5, 14);
    c.swc1(7, sp(c).wrapping_add(0x28));
    c.sub_s(1, 1, 15);
    c.swc1(13, sp(c).wrapping_add(0x18));
    c.mul_s(6, 6, 10);
    c.swc1(0, sp(c).wrapping_add(0x20));
    c.add_s(8, 8, 16);
    c.swc1(5, sp(c).wrapping_add(0x24));
    c.mul_s(4, 4, 9);
    c.swc1(1, sp(c).wrapping_add(0x14));
    c.swc1(6, sp(c).wrapping_add(0x4));
    c.swc1(8, sp(c).wrapping_add(0x10));
    c.swc1(4, sp(c));
    c.swc1(17, sp(c).wrapping_add(0x3c));
    c.g[V0][0] = c.lw(oc604_ea(c.g[S0][0], 0x18));
    c.g[T6][0] = c.lw(oc604_ea(c.g[V0][0], 0x80));
    c.lwc1(0, oc604_ea(c.g[V0][0], 0x84));
    c.g[A2][0] = c.lw(oc604_ea(c.g[T6][0], 0x8));
    c.swc1(0, sp(c).wrapping_add(0x78));
    c.swc1(17, sp(c).wrapping_add(0x7c));
    c.swc1(0, sp(c).wrapping_add(0x70));
    c.swc1(0, sp(c).wrapping_add(0x74));
    c.g[V0][0] = c.lw(oc604_ea(c.g[T6][0], 0x4));
    let t = (c.g[V0][0] as i64) <= 0;
    c.g[T7][0] = 0;
    if !t {
        c.cycles += 11;
        c.g[T0][0] = c.g[S1][0];
        c.g[S3][0] = u64::MAX;
        c.g[A3][0] = sx(sp(c).wrapping_add(0x80));
        c.g[T5][0] = sx(sp(c).wrapping_add(0x120));
        c.g[T2][0] = sx(sp(c).wrapping_add(0xc0));
        c.g[T4][0] = sx(sp(c).wrapping_add(0x100));
        c.g[T9][0] = c.g[SP][0];
        c.g[T3][0] = sx(sp(c).wrapping_add(0x1c0));
        c.g[T8][0] = sx(sp(c).wrapping_add(0x160));
        c.g[T1][0] = c.g[T0][0];
        loop {
            c.cycles += 3;
            c.g[A1][0] = c.lw(c.g[A2][0] as u32);
            let skip = c.g[A1][0] == c.g[S3][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
            if !skip {
                // A sample with a real index: blend the weights (in f0..f3),
                // then the three basis products into the T5/A3/T0 buffers.
                c.cycles += 77;
                c.g[A0][0] = c.lw(oc604_ea(c.g[A2][0], 0xc));
                c.lwc1(7, sp(c).wrapping_add(0x70));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.lq(2, c.g[A0][0] as u32);
                c.lwc1(6, sp(c).wrapping_add(0x74));
                c.sq(2, sp(c).wrapping_add(0x100));
                c.sq(2, sp(c).wrapping_add(0xc0));
                c.lwc1(5, sp(c).wrapping_add(0x78));
                c.lq(2, oc604_ea(c.g[A0][0], 0x10));
                c.lwc1(4, sp(c).wrapping_add(0x7c));
                c.sq(2, sp(c).wrapping_add(0x100));
                c.sq(2, sp(c).wrapping_add(0xd0));
                c.lq(3, oc604_ea(c.g[A0][0], 0x20));
                c.sq(3, sp(c).wrapping_add(0x100));
                c.sq(3, sp(c).wrapping_add(0xe0));
                c.lq(2, oc604_ea(c.g[A0][0], 0x30));
                c.sq(2, sp(c).wrapping_add(0x110));
                c.lwc1(1, sp(c).wrapping_add(0x110));
                c.lwc1(2, sp(c).wrapping_add(0x114));
                c.lwc1(3, sp(c).wrapping_add(0x118));
                c.mul_s(1, 1, 7);
                c.lwc1(0, sp(c).wrapping_add(0x11c));
                c.mul_s(2, 2, 6);
                c.mul_s(3, 3, 5);
                c.mul_s(0, 0, 4);
                c.swc1(1, sp(c).wrapping_add(0x100));
                c.swc1(2, sp(c).wrapping_add(0x104));
                c.swc1(3, sp(c).wrapping_add(0x108));
                c.swc1(0, sp(c).wrapping_add(0x10c));
                c.lq(2, sp(c).wrapping_add(0x100));
                c.sq(2, sp(c).wrapping_add(0xf0));
                oc604_quad_load(c, 4, A1);
                oc604_quad_load(c, 8, T2);
                oc604_mat4_columns(c);
                oc604_quad_store(c, 12, T5);
                oc604_quad_load(c, 1, T5);
                oc604_quad_store(c, 1, A3);
                oc604_quad_load(c, 1, A3);
                oc604_quad_store(c, 1, T0);
                c.g[V0][0] = c.lw(oc604_ea(c.g[T6][0], 0x4));
            } else {
                // The index is -1: the same products from the other buffers.
                c.cycles += 111;
                oc604_quad_load(c, 4, S2);
                oc604_quad_load(c, 8, T9);
                oc604_mat4_columns(c);
                oc604_quad_store(c, 12, T4);
                oc604_quad_load(c, 1, T4);
                oc604_quad_store(c, 1, T2);
                c.g[A0][0] = c.lw(oc604_ea(c.g[A2][0], 0xc));
                c.lwc1(6, sp(c).wrapping_add(0x70));
                c.lq(2, c.g[A0][0] as u32);
                c.lwc1(7, sp(c).wrapping_add(0x74));
                c.sq(2, sp(c).wrapping_add(0x1a0));
                c.sq(2, sp(c).wrapping_add(0x160));
                c.lwc1(5, sp(c).wrapping_add(0x78));
                c.lq(2, oc604_ea(c.g[A0][0], 0x10));
                c.lwc1(4, sp(c).wrapping_add(0x7c));
                c.sq(2, sp(c).wrapping_add(0x1a0));
                c.sq(2, sp(c).wrapping_add(0x170));
                c.lq(3, oc604_ea(c.g[A0][0], 0x20));
                c.sq(3, sp(c).wrapping_add(0x1a0));
                c.sq(3, sp(c).wrapping_add(0x180));
                c.lq(2, oc604_ea(c.g[A0][0], 0x30));
                c.sq(2, sp(c).wrapping_add(0x1b0));
                c.lwc1(1, sp(c).wrapping_add(0x1b0));
                c.lwc1(2, sp(c).wrapping_add(0x1b4));
                c.lwc1(3, sp(c).wrapping_add(0x1b8));
                c.mul_s(1, 1, 6);
                c.lwc1(0, sp(c).wrapping_add(0x1bc));
                c.mul_s(2, 2, 7);
                c.mul_s(3, 3, 5);
                c.mul_s(0, 0, 4);
                c.swc1(1, sp(c).wrapping_add(0x1a0));
                c.swc1(2, sp(c).wrapping_add(0x1a4));
                c.swc1(3, sp(c).wrapping_add(0x1a8));
                c.swc1(0, sp(c).wrapping_add(0x1ac));
                c.lq(2, sp(c).wrapping_add(0x1a0));
                c.sq(2, sp(c).wrapping_add(0x190));
                oc604_quad_load(c, 4, T2);
                oc604_quad_load(c, 8, T8);
                oc604_mat4_columns(c);
                oc604_quad_store(c, 12, T3);
                oc604_quad_load(c, 1, T3);
                oc604_quad_store(c, 1, A3);
                oc604_quad_load(c, 1, A3);
                oc604_quad_store(c, 1, T1);
                c.g[V0][0] = c.lw(oc604_ea(c.g[T6][0], 0x4));
            }
            c.cycles += 6;
            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(1));
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = ((c.g[T7][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_f2c0);
            }
        }
    }
    c.cycles += 10;
    c.g[V0][0] = c.lhu(oc604_ea(c.g[S0][0], 0x12));
    c.lq(17, sp(c).wrapping_add(0x250));
    c.g[V0][0] &= 0xfffe;
    c.lq(18, sp(c).wrapping_add(0x240));
    c.sh(oc604_ea(c.g[S0][0], 0x12), c.g[V0][0]);
    c.lq(16, sp(c).wrapping_add(0x260));
    c.lq(19, sp(c).wrapping_add(0x230));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x220));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x270));
    c.pc = t;
    Ok(())
}

/// `Floating_BuildMatrices` (0x34f120), see [`floating_build_matrices_body`].
pub fn floating_node_build_matrices(c: &mut Cpu) {
    guest(c, floating_build_matrices_body)
}

// ---------------------------------------------------------------------------
// cFloatingNode::applyForce (0x34f600), `Floating_v43`
// ---------------------------------------------------------------------------

/// `void cFloatingNode::applyForce(const tIntersectInfo&, const t4Vector&)`
/// (0x34f600): the contact arm is the intersection point minus the node
/// position (`transform` through the vtable slot at +0xc0/+0xc4); the force
/// adds to `Force` (+0x44) and the torque (arm x force) projected on the two
/// rotated axes adds to `TX` (+0x60) and `TY` (+0x70).
fn floating_apply_force_body(c: &mut Cpu) -> Flow {
    c.cycles += 99;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x90));
    c.sq(16, sp(c).wrapping_add(0x80));
    c.sq(17, sp(c).wrapping_add(0x70));
    c.g[S0][0] = c.g[A0][0];
    c.sq(18, sp(c).wrapping_add(0x60));
    c.g[S1][0] = c.g[A1][0];
    c.sd(sp(c).wrapping_add(0x50), c.g[RA][0]);
    c.g[S2][0] = c.g[A2][0];
    c.g[V1][0] = c.lw(oc604_ea(c.g[S0][0], 0xc));
    c.g[A0][0] = c.lh(oc604_ea(c.g[V1][0], 0xc0));
    c.g[V0][0] = c.lw(oc604_ea(c.g[V1][0], 0xc4));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x0034_f634)?;
    c.lq(3, oc604_ea(c.g[V0][0], 0x30));
    c.sq(3, sp(c).wrapping_add(0x10));
    c.lqc2(3, c.g[S1][0] as u32);
    c.lqc2(4, sp(c).wrapping_add(0x10));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x20));
    c.lq(3, sp(c).wrapping_add(0x20));
    c.sq(3, sp(c));
    c.g[V1][0] = 0x500000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xec0));
    c.lqc2(8, c.g[V1][0] as u32);
    oc604_quad_load(c, 4, V0);
    oc604_mul4(c, 8, 12);
    c.sqc2(12, sp(c).wrapping_add(0x20));
    c.lq(3, sp(c).wrapping_add(0x20));
    c.sq(3, sp(c).wrapping_add(0x10));
    c.g[V1][0] = 0x500000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xeb0));
    c.lqc2(8, c.g[V1][0] as u32);
    oc604_quad_load(c, 4, V0);
    oc604_mul4(c, 8, 12);
    c.sqc2(12, sp(c).wrapping_add(0x30));
    c.lwc1(0, oc604_ea(c.g[S0][0], 0x44));
    c.lqc2(3, c.g[S2][0] as u32);
    c.lqc2(5, oc604_ea(c.g[S1][0], 0x10));
    oc604_axis_pass(c);
    c.qmfc2(3, 4);
    c.mtc1(3, 1);
    c.lq(2, sp(c).wrapping_add(0x30));
    c.add_s(0, 0, 1);
    c.sq(2, sp(c).wrapping_add(0x20));
    c.swc1(0, oc604_ea(c.g[S0][0], 0x44));
    c.lqc2(4, c.g[S2][0] as u32);
    c.lqc2(5, sp(c) as u32);
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, sp(c).wrapping_add(0x40));
    c.lq(2, sp(c).wrapping_add(0x40));
    c.lwc1(1, oc604_ea(c.g[S0][0], 0x60));
    c.sq(2, sp(c).wrapping_add(0x30));
    c.lqc2(3, sp(c).wrapping_add(0x30));
    c.lqc2(5, sp(c).wrapping_add(0x10));
    oc604_axis_pass(c);
    c.qmfc2(3, 4);
    c.mtc1(3, 0);
    c.add_s(1, 1, 0);
    c.swc1(1, oc604_ea(c.g[S0][0], 0x60));
    c.lwc1(0, oc604_ea(c.g[S0][0], 0x70));
    c.lqc2(3, sp(c).wrapping_add(0x30));
    c.lqc2(5, sp(c).wrapping_add(0x20));
    oc604_axis_pass(c);
    c.qmfc2(2, 4);
    c.mtc1(2, 1);
    c.lq(17, sp(c).wrapping_add(0x70));
    c.add_s(0, 0, 1);
    c.lq(18, sp(c).wrapping_add(0x60));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x50));
    c.swc1(0, oc604_ea(c.g[S0][0], 0x70));
    c.lq(16, sp(c).wrapping_add(0x80));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x90));
    c.pc = t;
    Ok(())
}

/// `cFloatingNode::applyForce` (0x34f600), see [`floating_apply_force_body`].
pub fn floating_node_apply_force(c: &mut Cpu) {
    guest(c, floating_apply_force_body)
}

// ---------------------------------------------------------------------------
// cFloatingNode::updateIntersectInfo (0x34f790), `Floating_v42`
// ---------------------------------------------------------------------------

/// `void cFloatingNode::updateIntersectInfo(tIntersectInfo&)` (0x34f790): the
/// base class fills `intInfo` first (`cMoveNode::updateIntersectInfo`,
/// 0x356ae0); then the node adds its own contribution: the arm from the
/// node position, the angular velocity from the two rotated axes scaled by
/// the spring rates (`WX`/`WY` at +0x40/+0x44 of the spring state), and the
/// linear velocity (`Vel`) with `arm x angVel`.
fn floating_update_intersect_info_body(c: &mut Cpu) -> Flow {
    c.cycles += 103;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0xb0));
    c.sq(16, sp(c).wrapping_add(0xa0));
    c.sq(17, sp(c).wrapping_add(0x90));
    c.g[S0][0] = c.g[A0][0];
    c.sd(sp(c).wrapping_add(0x80), c.g[RA][0]);
    c.g[S1][0] = c.g[A1][0];
    call_ptr(c, 0x0035_6ae0, 0x0034_f7ac)?;
    c.g[V1][0] = c.lw(oc604_ea(c.g[S0][0], 0xc));
    c.g[A0][0] = c.lh(oc604_ea(c.g[V1][0], 0xc0));
    c.g[V0][0] = c.lw(oc604_ea(c.g[V1][0], 0xc4));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x0034_f7c0)?;
    c.lq(3, oc604_ea(c.g[V0][0], 0x30));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c));
    c.sq(3, sp(c).wrapping_add(0x10));
    c.lqc2(3, c.g[S1][0] as u32);
    c.lqc2(4, sp(c).wrapping_add(0x10));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x20));
    c.lq(3, sp(c).wrapping_add(0x20));
    c.sq(3, sp(c));
    c.g[V1][0] = 0x500000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xec0));
    c.lqc2(8, c.g[V1][0] as u32);
    oc604_quad_load(c, 4, V0);
    oc604_mul4(c, 8, 12);
    c.sqc2(12, sp(c).wrapping_add(0x20));
    c.lq(3, sp(c).wrapping_add(0x20));
    c.sq(3, sp(c).wrapping_add(0x10));
    c.g[V1][0] = 0x500000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xeb0));
    c.lqc2(8, c.g[V1][0] as u32);
    oc604_quad_load(c, 4, V0);
    oc604_mul4(c, 8, 12);
    c.sqc2(12, sp(c).wrapping_add(0x30));
    c.lwc1(0, oc604_ea(c.g[S0][0], 0x30));
    c.mfc1(3, 0);
    c.lqc2(4, sp(c).wrapping_add(0x10));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp(c).wrapping_add(0x50));
    c.lq(2, sp(c).wrapping_add(0x50));
    c.lwc1(0, oc604_ea(c.g[S0][0], 0x40));
    c.sq(2, sp(c).wrapping_add(0x40));
    c.mfc1(3, 0);
    c.lqc2(4, sp(c).wrapping_add(0x20));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp(c).wrapping_add(0x60));
    c.lq(2, sp(c).wrapping_add(0x60));
    c.sq(2, sp(c).wrapping_add(0x50));
    c.lqc2(3, sp(c).wrapping_add(0x40));
    c.lqc2(4, sp(c).wrapping_add(0x50));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x60));
    c.lwc1(0, oc604_ea(c.g[S0][0], 0x1c));
    c.lq(2, sp(c).wrapping_add(0x60));
    c.swc1(0, sp(c).wrapping_add(0x58));
    c.sq(2, sp(c).wrapping_add(0x30));
    c.sw(sp(c).wrapping_add(0x50), 0u64);
    c.sw(sp(c).wrapping_add(0x54), 0u64);
    c.sw(sp(c).wrapping_add(0x5c), 0u64);
    c.lqc2(4, sp(c));
    c.lqc2(5, sp(c).wrapping_add(0x30));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, sp(c).wrapping_add(0x70));
    c.lq(16, sp(c).wrapping_add(0xa0));
    c.lq(2, sp(c).wrapping_add(0x70));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x80));
    c.sq(2, sp(c).wrapping_add(0x60));
    c.lqc2(3, sp(c).wrapping_add(0x50));
    c.lqc2(4, sp(c).wrapping_add(0x60));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x70));
    c.lq(2, sp(c).wrapping_add(0x70));
    c.sq(2, sp(c).wrapping_add(0x40));
    c.lqc2(3, oc604_ea(c.g[S1][0], 0x20));
    c.lqc2(4, sp(c).wrapping_add(0x40));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, oc604_ea(c.g[S1][0], 0x20));
    c.lqc2(3, oc604_ea(c.g[S1][0], 0x30));
    c.lqc2(4, sp(c).wrapping_add(0x30));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, oc604_ea(c.g[S1][0], 0x30));
    c.lq(17, sp(c).wrapping_add(0x90));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0xb0));
    c.pc = t;
    Ok(())
}

/// `cFloatingNode::updateIntersectInfo` (0x34f790), see
/// [`floating_update_intersect_info_body`].
pub fn floating_node_update_intersect_info(c: &mut Cpu) {
    guest(c, floating_update_intersect_info_body)
}

// ---------------------------------------------------------------------------
// DeadFade_Render (0x350cb8)
// ---------------------------------------------------------------------------

/// `DeadFade_Render` (0x350cb8): draws the fade of the dead-rider overlay.
/// The frame is checked first (byte at +0x7d of the state, the table entry
/// at `[GP+0x16c8]`); when the fade is active it reads the timing from the
/// game table at `[S4+0x10d8]` (`S4` = `[GP+0x2a90]`), computes the fade
/// colour, and calls the two draw hooks through the function table.
fn dead_fade_render_body(c: &mut Cpu) -> Flow {
    c.cycles += 17;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0xe0));
    c.g[V0][0] = c.lw(oc604_ea(c.g[GP][0], 0x16c8));
    c.sq(21, sp(c).wrapping_add(0x70));
    c.sq(16, sp(c).wrapping_add(0xc0));
    c.g[S5][0] = c.g[A0][0];
    c.sq(17, sp(c).wrapping_add(0xb0));
    c.g[A0][0] = 0xff;
    c.sq(18, sp(c).wrapping_add(0xa0));
    c.sq(19, sp(c).wrapping_add(0x90));
    c.sq(20, sp(c).wrapping_add(0x80));
    c.sd(sp(c).wrapping_add(0x60), c.g[RA][0]);
    c.swc1(20, sp(c).wrapping_add(0xd0));
    c.g[A1][0] = c.lw(oc604_ea(c.g[S5][0], 0x18));
    c.g[A2][0] = c.lw(c.g[V0][0] as u32);
    c.g[V1][0] = c.lbu(oc604_ea(c.g[A1][0], 0x7d));
    let t = c.g[V1][0] == c.g[A0][0];
    c.g[V0][0] = 0x1;
    if !t {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(oc604_ea(c.g[A1][0], 0x7d));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = c.lw(oc604_ea(c.g[V0][0], 0x24));
        c.g[V1][0] ^= 0x6;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V0][0] = c.lh(oc604_ea(c.g[A1][0], 0x7e));
            c.g[A0][0] = 0x18;
            c.mult(0, 3, 2, 4);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V1][0] = c.lw(oc604_ea(c.g[V0][0], 0x3f0));
            c.g[V1][0] ^= 0x3;
            c.g[V0][0] = (c.g[V1][0] < 0x1) as u64;
        }
    }

    // The fade itself, when the first stage returned non-zero.
    c.cycles += 2;
    let active = c.g[V0][0] != 0;
    c.g[S4][0] = c.lw(oc604_ea(c.g[GP][0], 0x2a90));
    if active {
        c.cycles += 37;
        c.g[S2][0] = sx(sp(c).wrapping_add(0x20));
        c.g[V0][0] = c.lw(oc604_ea(c.g[S5][0], 0x18));
        c.g[S3][0] = sx(sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.lw(oc604_ea(c.g[S4][0], 0x10d8));
        c.g[A0][0] = c.g[S2][0];
        c.lwc1(0, oc604_ea(c.g[V0][0], 0x60));
        c.g[S0][0] = c.lh(oc604_ea(c.g[S1][0], 0x2e8));
        c.swc1(0, sp(c));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20);
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.lwc1(0, oc604_ea(c.g[V0][0], 0x64));
        c.swc1(0, sp(c).wrapping_add(0x4));
        c.lwc1(1, oc604_ea(c.g[V0][0], 0x68));
        c.swc1(20, sp(c).wrapping_add(0xc));
        c.swc1(1, sp(c).wrapping_add(0x8));
        c.lwc1(0, oc604_ea(c.g[V0][0], 0x6c));
        c.swc1(0, sp(c).wrapping_add(0x10));
        c.lwc1(1, oc604_ea(c.g[V0][0], 0x70));
        c.swc1(1, sp(c).wrapping_add(0x14));
        c.lwc1(0, oc604_ea(c.g[V0][0], 0x74));
        c.swc1(20, sp(c).wrapping_add(0x1c));
        c.swc1(0, sp(c).wrapping_add(0x18));
        c.g[V0][0] = c.lw(oc604_ea(c.g[S4][0], 0x10d8));
        c.g[A1][0] = c.lh(oc604_ea(c.g[V0][0], 0x158));
        c.g[V1][0] = c.lw(oc604_ea(c.g[V0][0], 0x15c));
        let t = c.g[V1][0] as u32;
        c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, t, 0x0035_0dac)?;
        c.g[V0][0] = c.lw(oc604_ea(c.g[S1][0], 0x2ec));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, t, 0x0035_0dc4)?;
        c.g[V1][0] = 0x1;
        let not_one = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0x500000;
        if not_one {
            c.cycles += 29;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
            oc604_quad_load(c, 1, V0);
            oc604_quad_store(c, 1, SP);
            c.lwc1(0, oc604_ea(c.g[S5][0], 0x30));
            c.g[A3][0] = c.g[SP][0];
            c.swc1(20, sp(c).wrapping_add(0x4c));
            c.g[T1][0] = 0;
            c.sw(sp(c).wrapping_add(0x40), 0u64);
            c.neg_s(0, 0);
            c.sw(sp(c).wrapping_add(0x44), 0u64);
            c.g[T2][0] = 0;
            c.g[A1][0] = c.lw(oc604_ea(c.g[S5][0], 0x18));
            c.swc1(0, sp(c).wrapping_add(0x48));
            c.lq(2, sp(c).wrapping_add(0x40));
            c.g[T0][0] = c.lw(oc604_ea(c.g[S5][0], 0x38));
            c.lwc1(12, oc604_ea(c.g[S5][0], 0x28));
            c.sq(2, sp(c).wrapping_add(0x30));
            c.g[V1][0] = c.lw(oc604_ea(c.g[S4][0], 0x10d8));
            c.g[A2][0] = c.lw(oc604_ea(c.g[A1][0], 0x94));
            c.g[A0][0] = c.lh(oc604_ea(c.g[V1][0], 0x2f8));
            c.g[V0][0] = c.lw(oc604_ea(c.g[V1][0], 0x2fc));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0035_0e44)?;
        }
    }
    c.cycles += 10;
    c.lq(16, sp(c).wrapping_add(0xc0));
    c.lq(17, sp(c).wrapping_add(0xb0));
    c.lq(18, sp(c).wrapping_add(0xa0));
    c.lq(19, sp(c).wrapping_add(0x90));
    c.lq(20, sp(c).wrapping_add(0x80));
    c.lq(21, sp(c).wrapping_add(0x70));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x60));
    c.lwc1(20, sp(c).wrapping_add(0xd0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0xe0));
    c.pc = t;
    Ok(())
}

/// `DeadFade_Render` (0x350cb8), see [`dead_fade_render_body`].
pub fn dead_fade_render(c: &mut Cpu) {
    guest(c, dead_fade_render_body)
}

// ---------------------------------------------------------------------------
// ModifierBlock_Load (0x3536d8): not installed yet (see the note below).
// ---------------------------------------------------------------------------

/// `ModifierBlock_Load` (0x3536d8): reads the modifier type word of a block and
/// builds the typed modifier objects it lists (spline, multi-spline, parent,
/// position, roller, magnet, ava-spline, and the sub-lists: boost, particles,
/// halo, rails), storing each into the block and pushing list entries.
///
/// Not in `OVERRIDES`: its eight constructors and the pool allocator
/// (`0x317d70`) need a real heap in the fixture, and the constructors
/// depend on the modifier table, so the differential test is not set up yet.
/// The port is kept here, written from the generated body.
pub fn modifier_block_load(c: &mut Cpu) {
    guest(c, modifier_block_load_body)
}

/// The typed constructor of one modifier group: `size` bytes from the pool
/// (`0x490000 - off` names the type), the constructor `ctor` on the block in
/// `S1`, the result stored at `[S2 + store]`. `cyc` is the group's cycle
/// charge on the allocation path.
fn oc604_mb_make(
    c: &mut Cpu,
    size: u64,
    off: u32,
    alloc_ret: u32,
    ctor: u32,
    ctor_ret: u32,
    store: u32,
    cyc: u64,
) -> Flow {
    c.cycles += cyc;
    c.g[A1][0] = 0x490000;
    c.g[A0][0] = size;
    c.g[A1][0] = sx(0x490000u32.wrapping_sub(off));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, alloc_ret)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, ctor, ctor_ret)?;
    c.sw(oc604_ea(c.g[S2][0], store), c.g[V0][0]);
    Ok(())
}

/// One group of the type chain: `V0` holds the type it compares with (set by
/// the caller), `next` is what `V0` becomes. True when the group matched.
fn oc604_mb_group(
    c: &mut Cpu,
    entry_cycles: u64,
    next: u64,
    size: u64,
    off: u32,
    alloc_ret: u32,
    ctor: u32,
    ctor_ret: u32,
) -> Result<bool, Unwound> {
    c.cycles += entry_cycles;
    let other = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = next;
    if other {
        return Ok(false);
    }
    oc604_mb_make(c, size, off, alloc_ret, ctor, ctor_ret, 0, 11)?;
    Ok(true)
}

fn modifier_block_load_body(c: &mut Cpu) -> Flow {
    c.cycles += 28;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0xe0));
    c.g[A2][0] = 0x34;
    c.sq(18, sp(c).wrapping_add(0xb0));
    c.sq(17, sp(c).wrapping_add(0xc0));
    c.g[S2][0] = c.g[A0][0];
    c.sq(16, sp(c).wrapping_add(0xd0));
    c.g[S1][0] = c.g[A1][0];
    c.sq(19, sp(c).wrapping_add(0xa0));
    c.g[A1][0] = c.g[SP][0];
    c.sq(20, sp(c).wrapping_add(0x90));
    c.sq(21, sp(c).wrapping_add(0x80));
    c.sq(22, sp(c).wrapping_add(0x70));
    c.sq(23, sp(c).wrapping_add(0x60));
    c.sq(30, sp(c).wrapping_add(0x50));
    c.sd(sp(c).wrapping_add(0x40), c.g[RA][0]);
    c.sw(c.g[S2][0] as u32, 0u64);
    c.sw(oc604_ea(c.g[S2][0], 0x4), 0u64);
    c.sw(oc604_ea(c.g[S2][0], 0x8), 0u64);
    c.sw(oc604_ea(c.g[S2][0], 0xc), 0u64);
    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
    c.g[A0][0] = c.lh(oc604_ea(c.g[V0][0], 0x10));
    c.g[V1][0] = c.lw(oc604_ea(c.g[V0][0], 0x14));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x0035_3738)?;
    c.g[V0][0] = c.lw(sp(c) as u32);
    c.g[V0][0] &= 0x1;
    let none = c.g[V0][0] == 0;
    c.g[V1][0] = c.lw(sp(c).wrapping_add(0x4));
    let mut matched = false;
    if !none {
        // Group 1: the spline.
        c.g[V0][0] = 0x1;
        matched = oc604_mb_group(c, 3, 2, 0xf0, 0x1730, 0x0035_376c, 0x0035_95d8, 0x0035_3778)?;
        if !matched {
            // Group 2: the multi-spline.
            matched = oc604_mb_group(c, 2, 4, 0x58, 0x1730, 0x0035_37a0, 0x0035_a118, 0x0035_37ac)?;
        }
        if !matched {
            // Group 3: the parent (type 4 in the table).
            matched = oc604_mb_group(c, 2, 3, 0xb0, 0x1720, 0x0035_37d4, 0x0035_7090, 0x0035_37e0)?;
        }
        if !matched {
            // Group 4: the position modifier.
            matched = oc604_mb_group(c, 2, 5, 0x90, 0x1710, 0x0035_3808, 0x0035_6f68, 0x0035_3814)?;
        }
        if !matched {
            // Group 5: the roller.
            matched = oc604_mb_group(
                c,
                2,
                6,
                0x2d0,
                0x16f8,
                0x0035_383c,
                0x0035_df70,
                0x0035_3848,
            )?;
        }
        if !matched {
            // Group 6: the magnet.
            matched = oc604_mb_group(c, 2, 7, 0xb0, 0x16e8, 0x0035_3870, 0x0035_7358, 0x0035_387c)?;
        }
        if !matched {
            // Group 7: the ava-spline (compares with 7).
            c.cycles += 2;
            let other = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.lw(sp(c) as u32);
            if !other {
                oc604_mb_make(
                    c,
                    0x90,
                    0x16d8,
                    0x0035_38a4,
                    0x0035_7798,
                    0x0035_38b0,
                    0,
                    10,
                )?;
                matched = true;
            }
        }
    }
    if none || matched {
        c.cycles += 1;
        c.g[V0][0] = c.lw(sp(c) as u32);
    }

    // Stage 2: the sub-objects from the flags word (bit 1, 2, 3).
    c.cycles += 3;
    c.g[V0][0] &= 0x2;
    let t = c.g[V0][0] == 0;
    c.g[A0][0] = 0x50;
    if !t {
        oc604_mb_make(
            c,
            0x50,
            0x16c0,
            0x0035_38d8,
            0x0035_f788,
            0x0035_38e4,
            0x4,
            9,
        )?;
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw(sp(c) as u32);
    c.g[V0][0] &= 0x4;
    let t = c.g[V0][0] == 0;
    c.g[A0][0] = 0xe8;
    if !t {
        oc604_mb_make(
            c,
            0xe8,
            0x16a8,
            0x0035_390c,
            0x0035_f1a8,
            0x0035_3918,
            0x8,
            9,
        )?;
    }

    // Stage 3: boost (flag 8), with the second flag word.
    c.cycles += 4;
    c.g[V0][0] = c.lw(sp(c) as u32) & 0x8;
    let no_boost = c.g[V0][0] == 0;
    c.g[V1][0] = c.lw(sp(c).wrapping_add(0x8));
    let mut boost_done = no_boost;
    if !no_boost {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        let skip = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0xc));
        if !skip {
            oc604_mb_make(
                c,
                0x50,
                0x1698,
                0x0035_3950,
                0x0034_2808,
                0x0035_395c,
                0xc,
                10,
            )?;
            boost_done = true;
        }
    }
    if boost_done {
        c.cycles += 1;
    }
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0xc));

    // Stage 4: the object list (count at [sp+0xc]), particles, halo, then
    // the rail list (count at [sp+0x10]).
    c.cycles += 2;
    let count = c.g[V0][0] as i64;
    c.g[S0][0] = 0;
    if count > 0 {
        c.cycles += 7;
        c.g[S3][0] = sx(sp(c).wrapping_add(0x14));
        c.g[FP][0] = 0x490000;
        c.g[S7][0] = 0x2;
        c.g[S6][0] = 0x490000;
        c.g[S5][0] = 0x3;
        c.g[S4][0] = 0x490000;
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
        loop {
            let mut pushed = false;
            c.cycles += 4;
            c.g[V1][0] = 0x1;
            c.g[V0][0] = c.lb(c.g[V0][0] as u32);
            let other = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = 0x1f0;
            if !other {
                c.cycles += 9;
                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x1688));
                c.g[A2][0] = 0x2000_0000;
                c.g[RA][0] = sx(0x0035_39a8);
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0035_39a8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0034_59a8, 0x0035_39b4)?;
                c.g[A1][0] = c.g[V0][0];
                pushed = true;
            }
            if !pushed {
                c.cycles += 2;
                let other = c.g[V0][0] != c.g[S7][0];
                c.g[A0][0] = 0x270;
                if !other {
                    c.cycles += 9;
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1670));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[RA][0] = sx(0x0035_39d4);
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, 0x0035_39d4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0034_5d80, 0x0035_39e0)?;
                    c.g[A1][0] = c.g[V0][0];
                    pushed = true;
                }
            }
            if !pushed {
                c.cycles += 2;
                let other = c.g[V0][0] != c.g[S5][0];
                c.g[V0][0] = 0;
                if !other {
                    c.cycles += 10;
                    c.g[A0][0] = 0x40;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1658));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[RA][0] = sx(0x0035_3a04);
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, 0x0035_3a04)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0034_61c0, 0x0035_3a10)?;
                    c.g[A1][0] = c.g[V0][0];
                    pushed = true;
                }
                if !pushed {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[V0][0];
                }
            }
            c.cycles += 7;
            c.g[RA][0] = sx(0x0035_3a24);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x0034_5720, 0x0035_3a24)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = c.lw(sp(c).wrapping_add(0xc));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_3988);
            }
        }
    }
    c.cycles += 3;
    c.g[S0][0] = 0;
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 3;
        c.g[S3][0] = 0x490000;
        c.g[A2][0] = 0x2000_0000;
        loop {
            c.cycles += 15;
            c.g[A3][0] = 0;
            c.g[A0][0] = 0x90;
            c.g[RA][0] = sx(0x0035_3a60);
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1648));
            call_ptr(c, 0x0031_7d70, 0x0035_3a60)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0035_ba88, 0x0035_3a70)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1c));
            call_ptr(c, 0x0035_b690, 0x0035_3a7c)?;
            c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = 0x2000_0000;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_3a50);
            }
        }
    }
    c.cycles += 12;
    c.lq(16, sp(c).wrapping_add(0xd0));
    c.lq(17, sp(c).wrapping_add(0xc0));
    c.lq(18, sp(c).wrapping_add(0xb0));
    c.lq(19, sp(c).wrapping_add(0xa0));
    c.lq(20, sp(c).wrapping_add(0x90));
    c.lq(21, sp(c).wrapping_add(0x80));
    c.lq(22, sp(c).wrapping_add(0x70));
    c.lq(23, sp(c).wrapping_add(0x60));
    c.lq(30, sp(c).wrapping_add(0x50));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0xe0));
    c.pc = t;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// A leaf that returns 0 (`0x1406c0`): the vtable target of the fixtures.
    const LEAF: u32 = 0x0014_06c0;

    #[test]
    fn floating_v43_matches() {
        check(
            ee::s11::f_0034f600__generated,
            floating_node_apply_force,
            200,
            &|c, seed| {
                let mut r = Rng(0x43 ^ seed as u64);
                scramble(c, &mut r);
                let node = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(DATA + 0x1100);
                c.g[A2][0] = sx(DATA + 0x1200);
                c.bus.write32(node + 0xc, block);
                c.bus.write16(block + 0xc0, 0);
                c.bus.write32(block + 0xc4, LEAF);
                for k in 0..0x180 {
                    c.bus.write32(DATA + 0x1000 + 4 * k, r.next_u32());
                }
                c.bus.write32(node + 0x44, (seed as f32 * 0.5).to_bits());
                c.bus.write32(node + 0x60, r.next_u32());
                c.bus.write32(node + 0x70, r.next_u32());
            },
        );
    }

    #[test]
    fn floating_v42_matches() {
        check(
            ee::s11::f_0034f790__generated,
            floating_node_update_intersect_info,
            200,
            &|c, seed| {
                let mut r = Rng(0x42 ^ seed as u64);
                scramble(c, &mut r);
                let node = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(DATA + 0x1100);
                c.bus.write32(node + 0xc, block);
                c.bus.write16(block + 0xc0, 0);
                c.bus.write32(block + 0xc4, LEAF);
                for k in 0..0x180 {
                    c.bus.write32(DATA + 0x1000 + 4 * k, r.next_u32());
                }
                c.bus.write32(node + 0x30, (seed as f32).to_bits());
                c.bus.write32(node + 0x40, r.next_u32());
                c.bus.write32(node + 0x1c, r.next_u32());
                c.bus.write32(node + 0x18, (seed as f32).to_bits());
            },
        );
    }

    #[test]
    fn dead_fade_render_matches() {
        check(
            ee::s11::f_00350cb8__generated,
            dead_fade_render,
            200,
            &|c, seed| {
                let mut r = Rng(0xdead ^ seed as u64);
                scramble(c, &mut r);
                let gp = crate::difftest::GAME_GP;
                let state = DATA + 0x100;
                let table = DATA + 0x2000;
                let timing = DATA + 0x2400;
                c.g[GP][0] = sx(gp);
                c.g[S5][0] = sx(state);
                c.bus.write32(gp + 0x16c8, DATA + 0x2800);
                c.bus.write32(gp + 0x2a90, DATA + 0x2c00);
                c.bus.write32(DATA + 0x2800, table);
                c.bus.write32(state + 0x18, DATA + 0x200);
                c.bus.write8(
                    DATA + 0x200 + 0x7d,
                    if seed % 4 == 0 {
                        0xff
                    } else {
                        (seed % 3) as u8
                    },
                );
                c.bus.write16(DATA + 0x200 + 0x7e, (seed % 5) as u16);
                c.bus.write32(table, table + 0x100);
                c.bus.write32(table + 0x100 + 0x24, (seed % 2) as u32 * 6);
                c.bus.write32(DATA + 0x2c00 + 0x10d8, timing);
                c.bus.write16(timing + 0x2e8, 0);
                c.bus.write16(timing + 0x158, 0);
                c.bus.write32(timing + 0x15c, LEAF);
                c.bus.write16(timing + 0x2f8, 0);
                c.bus.write32(timing + 0x2fc, LEAF);
                c.bus.write32(timing + 0x2ec, LEAF);
                c.bus.write32(DATA + 0x200 + 0x94, DATA + 0x2400);
                for k in 0..0x40 {
                    c.bus.write32(DATA + 0x200 + 4 * k, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn floating_build_matrices_matches() {
        check(
            ee::s11::f_0034f120__generated,
            floating_node_build_matrices,
            200,
            &|c, seed| {
                let mut r = Rng(0x120 ^ seed as u64);
                scramble(c, &mut r);
                let node = DATA + 0x100;
                let inst = DATA + 0x300;
                let samples = DATA + 0x400;
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x1400);
                c.bus.write32(node + 0x18, inst);
                c.bus.write32(inst + 0x80, samples);
                c.bus.write32(inst + 0x84, (seed as f32).to_bits());
                c.bus
                    .write32(samples + 0x4, if seed % 3 == 0 { 0 } else { 2 });
                c.bus.write32(samples + 0x8, DATA + 0x500);
                c.bus
                    .write32(DATA + 0x500, if seed % 2 == 0 { u32::MAX } else { 0 });
                c.bus.write32(DATA + 0x500 + 0xc, DATA + 0x600);
                for k in 0..0x200 {
                    c.bus.write32(DATA + 0x600 + 4 * k, r.next_u32());
                }
                for k in 0..0x100 {
                    c.bus.write32(DATA + 0x1000 + 4 * k, r.next_u32());
                }
                c.bus.write16(node + 0x12, 0xffff);
            },
        );
    }
}
