//! Staging module oc687 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc687`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0034a028,
        spring_rail_init_pos_from_instance,
        "SpringRail_InitPosFromInstance",
    ),
    (0x0034a0b0, spring_rail_v48, "SpringRail_v48"),
    (0x0034dac8, anim_object_base_ctor_2, "AnimObject_BaseCtor_2"),
    (0x0034ed88, floating_ctor_2, "Floating_ctor_2"),
    (0x0034ef40, floating_v50, "Floating_v50"),
    (0x003536d8, modifier_block_load, "ModifierBlock_Load"),
    (0x003552d8, mod_obj_ctor_2, "ModObj_ctor_2"),
    (0x003592d0, anim_delta_v15, "AnimDelta_v15"),
    (0x0035ba88, rail_modifier_ctor_2, "RailModifier_ctor_2"),
    (0x003669f0, vram_heap_evict_range, "VramHeap_EvictRange"),
];

/// `void SpringRail_InitPosFromInstance(SpringRail* node, Instance* inst)`
/// (0x34a028): looks the instance's handle (`inst + 0x50`: low byte, then the
/// rest) up in the world's two-level instance table (`World_GetInstance`),
/// asks `Instance_GetNodeMatrix` for the node matrix (`inst + 0x54`) and
/// copies its translation row to the first quad of `inst`.
pub fn spring_rail_init_pos_from_instance(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_34a084: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
            c.g[RA][0] = sx(0x0034a03c);
            c.g[S0][0] = c.g[A1][0];
            call_ptr(c, 0x002d1bd8, 0x0034a03c)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.g[A1][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            let t = c.g[V1][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_34a084;
            } else {
                c.cycles += 8;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = 0u64;
                if t {
                    break 'b_34a084;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                    break 'b_34a084;
                }
            }
        }
        c.cycles += 10;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        c.g[RA][0] = sx(0x0034a090);
        c.g[A2][0] = c.g[SP][0];
        Callee {
            addr: 0x0034fed8,
            f: ee::s11::Instance_GetNodeMatrix,
        }
        .call(c, 0x0034a090)?;
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(2, (c.g[S0][0] as u32));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `void SpringRail_v48(SpringRail* self)` (0x34a0b0, leaf): rebuilds the
/// per-segment matrices (`Mats`, 0x40 bytes each, at `self + 0x50`) from the
/// node array (`Nodes`, 0x60 bytes each, at `self + 0x70`): the first segment,
/// the middle ones while `i < NumNodes - 2`, then the last. Each segment is
/// three VU0 transforms by the matrix at 0x4ffac0 and a transposition, with
/// the intermediate quads spilled to the 0x50-byte frame (observable bytes).
/// Transcribed from the generated body.
pub fn spring_rail_v48(c: &mut Cpu) {
    'b_34a3dc: {
        c.cycles += 99;
        c.g[T5][0] = c.g[A0][0];
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x70));
        c.g[A1][0] = 0x500000;
        c.g[A0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x50));
        c.g[T7][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x540));
        c.lwc1(2, (c.g[V1][0] as u32));
        c.g[T6][0] = 0x1;
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x60));
        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0xc0));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lqc2(4, (c.g[T7][0] as u32));
        c.lqc2(5, (c.g[T7][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[T7][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[T7][0] as u32).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 3);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(2, (c.g[SP][0] as u32));
        c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x64));
        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0xc4));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x24));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x2c));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[T7][0] as u32));
        c.lqc2(5, (c.g[T7][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[T7][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[T7][0] as u32).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0xc8));
        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x68));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x3c));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lqc2(4, (c.g[T7][0] as u32));
        c.lqc2(5, (c.g[T7][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[T7][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[T7][0] as u32).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(3, (c.g[SP][0] as u32));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(3, (c.g[A0][0] as u32));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x20));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x3c));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
        c.lq(12, (c.g[A0][0] as u32));
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
        c.sq(12, (c.g[A0][0] as u32));
        c.sq(11, (c.g[A0][0] as u32).wrapping_add(0x10));
        c.sq(10, (c.g[A0][0] as u32).wrapping_add(0x20));
        c.sq(9, (c.g[A0][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
        c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[T8][0] = c.g[A1][0];
        if t {
            break 'b_34a3dc;
        } else {
            c.cycles += 3;
            c.mov_s(4, 3);
            c.g[T4][0] = 0x60;
            'l_34a248: loop {
                c.cycles += 101;
                c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x70));
                c.g[T3][0] = sx((c.g[T6][0] as u32) << 6);
                c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V1][0] = c.g[A0][0];
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x60));
                c.lwc1(3, (c.g[V1][0] as u32));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x60));
                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0xc0));
                c.lwc1(0, (c.g[A0][0] as u32));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(4, (c.g[T7][0] as u32));
                c.lqc2(5, (c.g[T7][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[T7][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[T7][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(2, (c.g[SP][0] as u32));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x64));
                c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0xc4));
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x2c));
                c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[T7][0] as u32));
                c.lqc2(5, (c.g[T7][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[T7][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[T7][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0xc8));
                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x68));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x38));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x3c));
                c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lqc2(4, (c.g[T7][0] as u32));
                c.lqc2(5, (c.g[T7][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[T7][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[T7][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(3, (c.g[SP][0] as u32));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(3, (c.g[T3][0] as u32));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(2, (c.g[T3][0] as u32).wrapping_add(0x10));
                c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(3, (c.g[T3][0] as u32).wrapping_add(0x20));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x3c));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(2, (c.g[T3][0] as u32).wrapping_add(0x30));
                c.lq(10, (c.g[T3][0] as u32));
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
                c.sq(10, (c.g[T3][0] as u32));
                c.sq(9, (c.g[T3][0] as u32).wrapping_add(0x10));
                c.sq(8, (c.g[T3][0] as u32).wrapping_add(0x20));
                c.sq(7, (c.g[T3][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x20));
                c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
                c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x60));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034a248);
                    }
                    continue 'l_34a248;
                } else {
                    break 'b_34a3dc;
                }
            }
        }
    }
    c.cycles += 99;
    c.g[V1][0] = 0x60;
    c.g[A2][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x70));
    c.mult(0, 3, 14, 3);
    c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x50));
    c.g[A3][0] = sx((c.g[T8][0] as u32).wrapping_sub(0x540));
    c.g[A0][0] = sx((c.g[T6][0] as u32) << 6);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 3);
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.g[V1][0] = c.g[A1][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x60));
    c.lwc1(2, (c.g[V1][0] as u32));
    c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x60));
    c.lwc1(0, (c.g[A1][0] as u32));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x1c));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(4, (c.g[A3][0] as u32));
    c.lqc2(5, (c.g[A3][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[A3][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[A3][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(2, (c.g[SP][0] as u32));
    c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x64));
    c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x24));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x28));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lqc2(4, (c.g[A3][0] as u32));
    c.lqc2(5, (c.g[A3][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[A3][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[A3][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x68));
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lqc2(4, (c.g[A3][0] as u32));
    c.lqc2(5, (c.g[A3][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[A3][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[A3][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(3, (c.g[SP][0] as u32));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(3, (c.g[A0][0] as u32));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.lq(11, (c.g[A0][0] as u32));
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
    c.sq(11, (c.g[A0][0] as u32));
    c.sq(10, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.sq(9, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.sq(8, (c.g[A0][0] as u32).wrapping_add(0x30));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t;
    return;
}

/// `cAnimNode* cAnimNode::cAnimNode(self, int, Stream* s)` (0x34dac8), the
/// stream-loaded animation node: builds the embedded `ModObj` (`ModObj_ctor_2`
/// on `self + 0x14`), sets the vtable (0x490cc8), allocates one slot per mesh
/// of the node's model (`operator_new_named`, size `count << 6`), builds the
/// mesh cache (`AnimObject_AllocNodeInfo`), reads the 20-byte header and one
/// word from the stream and evaluates the first frame (`AnimObject_v52`).
/// Returns `self`.
pub fn anim_object_base_ctor_2(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_34db38: {
            c.cycles += 26;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
            c.g[S2][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.g[S0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[RA][0] = sx(0x0034daf8);
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x44));
            Callee {
                addr: 0x003552d8,
                f: ee::s11::ModObj_ctor_2,
            }
            .call(c, 0x0034daf8)?;
            c.g[V1][0] = 0x490000;
            c.g[A1][0] = 0x490000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xcc8));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x18a8));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[V1][0]);
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x80));
            c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[RA][0] = sx(0x0034db28);
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 6);
            call_ptr(c, 0x00317e30, 0x0034db28)?;
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_34db38;
            } else {
                c.cycles += 2;
                c.g[A0][0] = 0xffffffffffffffff;
                break 'b_34db38;
            }
        }
        c.cycles += 27;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0034db44);
        c.sw((c.g[S3][0] as u32), c.g[V0][0]);
        Callee {
            addr: 0x0034e448,
            f: ee::s11::AnimObject_AllocNodeInfo,
        }
        .call(c, 0x0034db44)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x14;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0034db60);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0034db60)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0034db7c);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0034db7c)?;
        c.g[RA][0] = sx(0x0034db84);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0034e348,
            f: ee::s11::AnimObject_v52,
        }
        .call(c, 0x0034db84)?;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `Floating_ctor_2(self, int, Stream* s)` (0x34ed88): the second floating
/// node constructor, the same node without the spring setup. Builds the
/// `ModObj` (`ModObj_ctor_2`), sets the vtable (0x48f858) and clears the part
/// pointer (+0x78), reads one word from the stream and bounds the node when
/// it is set (`Floating_AllocBoundBox`), then reads the per-mesh frames and
/// ORs flag bit 0 into the halfword at +0x12. Returns `self`.
pub fn floating_ctor_2(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_34edec: {
            c.cycles += 22;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[RA][0] = sx(0x0034eda8);
            c.g[S1][0] = c.g[A0][0];
            Callee {
                addr: 0x003552d8,
                f: ee::s11::ModObj_ctor_2,
            }
            .call(c, 0x0034eda8)?;
            c.g[V1][0] = 0x490000;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x78), 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7a8));
            c.g[A1][0] = c.g[SP][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.g[A2][0] = 0x4;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0034edd4);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0034edd4)?;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                break 'b_34edec;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0034ede8);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0034eeb8,
                    f: ee::s11::Floating_AllocBoundBox,
                }
                .call(c, 0x0034ede8)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                break 'b_34edec;
            }
        }
        'b_34ee38: {
            c.cycles += 18;
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2c));
            c.g[A2][0] = 0x48;
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0034ee08);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0034ee08)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.g[A1][0] = 0x490000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1780));
            c.g[A2][0] = 0x20000000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x80));
            c.g[A3][0] = 0u64;
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[RA][0] = sx(0x0034ee2c);
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 6);
            call_ptr(c, 0x00317e30, 0x0034ee2c)?;
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_34ee38;
            } else {
                c.cycles += 1;
                c.g[A0][0] = 0xffffffffffffffff;
                break 'b_34ee38;
            }
        }
        c.cycles += 11;
        c.sw((c.g[S2][0] as u32), c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x12));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.g[V1][0] | 0x1;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sh((c.g[S1][0] as u32).wrapping_add(0x12), c.g[V1][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `Floating_v50(self)` (0x34ef40): the floating node's per-frame update. With
/// a second part (`self + 0x78`) it takes the sum and difference of two
/// virtual matrix rows (VU0 `sub` then `add`) into the part's first rows, then
/// `World_GetNash` and `SpatialHash_Update`. Without one it calls `ModObj_v50`.
pub fn floating_v50(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_34f038: {
            'b_34f02c: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                let t = c.g[A0][0] == 0u64;

                if t {
                    break 'b_34f02c;
                } else {
                    c.cycles += 52;
                    c.lq(3, (c.g[A0][0] as u32));
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), 0u64);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x48));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0034ef94);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0034ef94)?;
                    c.lq(3, (c.g[V0][0] as u32).wrapping_add(0x30));
                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.sq(2, (c.g[SP][0] as u32));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0034efcc);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0034efcc)?;
                    c.lq(3, (c.g[V0][0] as u32).wrapping_add(0x30));
                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                    c.lq(3, (c.g[SP][0] as u32));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(3, (c.g[A0][0] as u32));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0034f00c);
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x002d1be0, 0x0034f00c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A1][0] = 0u64;
                    c.g[A3][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0034f024);
                    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    call_ptr(c, 0x003291e0, 0x0034f024)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        break 'b_34f038;
                    } else {
                        break 'b_34f02c;
                    }
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x0034f034);
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x003568b0,
                f: ee::s11::ModObj_v50,
            }
            .call(c, 0x0034f034)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            break 'b_34f038;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `void ModifierBlock_Load(ModifierBlock* self, Stream* s)` (0x3536d8): reads
/// the 52-byte modifier header and builds the typed objects it lists: the root
/// transform (spline, multi-spline, parent, position, roller, magnet or
/// ava-spline by `type`), the UV scroll, tex flip and boost modifiers by
/// `flags`, `numA` effects (particle, dynamic particle, halo) pushed on the
/// effect list and `numB` rail modifiers pushed on the rail list. Each object
/// is a named pool allocation (`EA_MemAllocNamed`) plus its constructor.
pub fn modifier_block_load(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_3538b8: {
            'b_3538b4: {
                c.cycles += 28;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
                c.g[A2][0] = 0x34;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[S1][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[A1][0] = c.g[SP][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                c.sw((c.g[S2][0] as u32), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00353738);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00353738)?;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                if t {
                    break 'b_3538b4;
                } else {
                    'b_353780: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_353780;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = 0x490000;
                            c.g[A0][0] = 0xf0;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1730));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x0035376c);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x0035376c)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00353778);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x003595d8,
                                f: ee::s11::SplineModifier_ctor_2,
                            }
                            .call(c, 0x00353778)?;
                            let t = true;
                            c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                            if t {
                                break 'b_3538b4;
                            } else {
                                break 'b_353780;
                            }
                        }
                    }
                    'b_3537b4: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x4;
                        if t {
                            break 'b_3537b4;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = 0x490000;
                            c.g[A0][0] = 0x58;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1730));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x003537a0);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x003537a0)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x003537ac);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x0035a118,
                                f: ee::s11::MultiSplineModifier_ctor_2,
                            }
                            .call(c, 0x003537ac)?;
                            let t = true;
                            c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                            if t {
                                break 'b_3538b4;
                            } else {
                                break 'b_3537b4;
                            }
                        }
                    }
                    'b_3537e8: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x3;
                        if t {
                            break 'b_3537e8;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = 0x490000;
                            c.g[A0][0] = 0xb0;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1720));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x003537d4);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x003537d4)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x003537e0);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00357090,
                                f: ee::s11::ParentModifier_ctor_2,
                            }
                            .call(c, 0x003537e0)?;
                            let t = true;
                            c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                            if t {
                                break 'b_3538b4;
                            } else {
                                break 'b_3537e8;
                            }
                        }
                    }
                    'b_35381c: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x5;
                        if t {
                            break 'b_35381c;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = 0x490000;
                            c.g[A0][0] = 0x90;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1710));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x00353808);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x00353808)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00353814);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00356f68,
                                f: ee::s11::PositionModifier_ctor_3,
                            }
                            .call(c, 0x00353814)?;
                            let t = true;
                            c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                            if t {
                                break 'b_3538b4;
                            } else {
                                break 'b_35381c;
                            }
                        }
                    }
                    'b_353850: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x6;
                        if t {
                            break 'b_353850;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = 0x490000;
                            c.g[A0][0] = 0x2d0;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16f8));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x0035383c);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x0035383c)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00353848);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x0035df70,
                                f: ee::s11::RollerModifier_ctor_2,
                            }
                            .call(c, 0x00353848)?;
                            let t = true;
                            c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                            if t {
                                break 'b_3538b4;
                            } else {
                                break 'b_353850;
                            }
                        }
                    }
                    'b_353884: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x7;
                        if t {
                            break 'b_353884;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = 0x490000;
                            c.g[A0][0] = 0xb0;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16e8));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x00353870);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x00353870)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0035387c);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00357358,
                                f: ee::s11::MagnetModifier_ctor_2,
                            }
                            .call(c, 0x0035387c)?;
                            let t = true;
                            c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                            if t {
                                break 'b_3538b4;
                            } else {
                                break 'b_353884;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                    if t {
                        break 'b_3538b8;
                    } else {
                        c.cycles += 10;
                        c.g[A1][0] = 0x490000;
                        c.g[A0][0] = 0x90;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16d8));
                        c.g[A2][0] = 0x20000000;
                        c.g[RA][0] = sx(0x003538a4);
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x003538a4)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x003538b0);
                        c.g[A1][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00357798,
                            f: ee::s11::AvaSplineModifier_ctor_2,
                        }
                        .call(c, 0x003538b0)?;
                        c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                        break 'b_3538b4;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
            break 'b_3538b8;
        }
        'b_3538e8: {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] & 0x2;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0x50;
            if t {
                break 'b_3538e8;
            } else {
                c.cycles += 9;
                c.g[A1][0] = 0x490000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16c0));
                c.g[A2][0] = 0x20000000;
                c.g[RA][0] = sx(0x003538d8);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x003538d8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x003538e4);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0035f788, 0x003538e4)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                break 'b_3538e8;
            }
        }
        'b_35391c: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
            c.g[V0][0] = c.g[V0][0] & 0x4;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0xe8;
            if t {
                break 'b_35391c;
            } else {
                c.cycles += 9;
                c.g[A1][0] = 0x490000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16a8));
                c.g[A2][0] = 0x20000000;
                c.g[RA][0] = sx(0x0035390c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0035390c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00353918);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0035f1a8, 0x00353918)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                break 'b_35391c;
            }
        }
        'b_353964: {
            'b_353960: {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x8;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                if t {
                    break 'b_353960;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    if t {
                        break 'b_353964;
                    } else {
                        c.cycles += 10;
                        c.g[A1][0] = 0x490000;
                        c.g[A0][0] = 0x50;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1698));
                        c.g[A2][0] = 0x20000000;
                        c.g[RA][0] = sx(0x00353950);
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00353950)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x0035395c);
                        c.g[A1][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00342808,
                            f: ee::s11::BoostModifier_ctor_2,
                        }
                        .call(c, 0x0035395c)?;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                        break 'b_353960;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
            break 'b_353964;
        }
        'b_353a38: {
            c.cycles += 2;
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_353a38;
            } else {
                c.cycles += 7;
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
                c.g[FP][0] = 0x490000;
                c.g[S7][0] = 0x2;
                c.g[S6][0] = 0x490000;
                c.g[S5][0] = 0x3;
                c.g[S4][0] = 0x490000;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                'l_353988: loop {
                    'b_353a1c: {
                        'b_3539bc: {
                            c.cycles += 4;
                            c.g[V1][0] = 0x1;
                            c.g[V0][0] = c.lb((c.g[V0][0] as u32));
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[A0][0] = 0x1f0;
                            if t {
                                break 'b_3539bc;
                            } else {
                                c.cycles += 9;
                                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x1688));
                                c.g[A2][0] = 0x20000000;
                                c.g[RA][0] = sx(0x003539a8);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x003539a8)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x003539b4);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x003459a8,
                                    f: ee::s11::ParticleModifier_ctor_2,
                                }
                                .call(c, 0x003539b4)?;
                                let t = true;
                                c.g[A1][0] = c.g[V0][0];
                                if t {
                                    break 'b_353a1c;
                                } else {
                                    break 'b_3539bc;
                                }
                            }
                        }
                        'b_3539e8: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != c.g[S7][0];
                            c.g[A0][0] = 0x270;
                            if t {
                                break 'b_3539e8;
                            } else {
                                c.cycles += 9;
                                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1670));
                                c.g[A2][0] = 0x20000000;
                                c.g[RA][0] = sx(0x003539d4);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x003539d4)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x003539e0);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x00345d80,
                                    f: ee::s11::DynamicParticleModifier_ctor_2,
                                }
                                .call(c, 0x003539e0)?;
                                let t = true;
                                c.g[A1][0] = c.g[V0][0];
                                if t {
                                    break 'b_353a1c;
                                } else {
                                    break 'b_3539e8;
                                }
                            }
                        }
                        'b_353a18: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != c.g[S5][0];
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_353a18;
                            } else {
                                c.cycles += 10;
                                c.g[A0][0] = 0x40;
                                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1658));
                                c.g[A2][0] = 0x20000000;
                                c.g[RA][0] = sx(0x00353a04);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x00353a04)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x00353a10);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x003461c0,
                                    f: ee::s11::HaloModifier_ctor_2,
                                }
                                .call(c, 0x00353a10)?;
                                let t = true;
                                c.g[A1][0] = c.g[V0][0];
                                if t {
                                    break 'b_353a1c;
                                } else {
                                    break 'b_353a18;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A1][0] = c.g[V0][0];
                        break 'b_353a1c;
                    }
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00353a24);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                    Callee {
                        addr: 0x00345720,
                        f: ee::s11::ModifierList_PushBack,
                    }
                    .call(c, 0x00353a24)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00353988);
                        }
                        continue 'l_353988;
                    } else {
                        break 'b_353a38;
                    }
                }
            }
        }
        'b_353a8c: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_353a8c;
            } else {
                c.cycles += 3;
                c.g[S3][0] = 0x490000;
                c.g[A2][0] = 0x20000000;
                'l_353a50: loop {
                    c.cycles += 15;
                    c.g[A3][0] = 0u64;
                    c.g[A0][0] = 0x90;
                    c.g[RA][0] = sx(0x00353a60);
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1648));
                    call_ptr(c, 0x00317d70, 0x00353a60)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00353a70);
                    c.g[A1][0] = c.g[S1][0];
                    Callee {
                        addr: 0x0035ba88,
                        f: ee::s11::RailModifier_ctor_2,
                    }
                    .call(c, 0x00353a70)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00353a7c);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1c));
                    Callee {
                        addr: 0x0035b690,
                        f: ee::s11::RailModifier_ListPushBack,
                    }
                    .call(c, 0x00353a7c)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = 0x20000000;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00353a50);
                        }
                        continue 'l_353a50;
                    } else {
                        break 'b_353a8c;
                    }
                }
            }
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.pc = t;
        return Ok(());
    });
}

/// `ModObj* ModObj::ModObj(self, int, Stream* s)` (0x3552d8), the stream
/// constructor: builds the world-object base (`WorldObj_ctor_2`), sets the
/// `ModObj` vtable (0x491028), reads `self + 0x20/0x24/0x28` and a block flag
/// from the stream, creates and loads the modifier block when the flag is set,
/// runs `ModObj_v48` and sets flag bit 0 of the halfword at +0x12. Returns
/// `self`.
pub fn mod_obj_ctor_2(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_355394: {
            c.cycles += 42;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[RA][0] = sx(0x003552f4);
            c.g[S1][0] = c.g[A2][0];
            Callee {
                addr: 0x0034fb90,
                f: ee::s11::WorldObj_ctor_2,
            }
            .call(c, 0x003552f4)?;
            c.g[V1][0] = 0x490000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1028));
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.g[A2][0] = 0x4;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00355320);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00355320)?;
            c.g[A3][0] = c.lw((c.g[S1][0] as u32));
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
            c.g[A2][0] = 0x4;
            c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0035533c);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0035533c)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32));
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x28));
            c.g[A2][0] = 0x4;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00355358);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00355358)?;
            c.g[A3][0] = c.lw((c.g[S1][0] as u32));
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = 0x4;
            c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00355374);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00355374)?;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
            let t = c.g[V0][0] == 0u64;

            if t {
                break 'b_355394;
            } else {
                c.cycles += 5;
                c.g[RA][0] = sx(0x00355388);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x00355468,
                    f: ee::s11::ModObj_CreateModifierBlock,
                }
                .call(c, 0x00355388)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[RA][0] = sx(0x00355394);
                c.g[A1][0] = c.g[S1][0];
                Callee {
                    addr: 0x003536d8,
                    f: ee::s11::ModifierBlock_Load,
                }
                .call(c, 0x00355394)?;
                break 'b_355394;
            }
        }
        c.cycles += 11;
        c.g[RA][0] = sx(0x0035539c);
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x00356780,
            f: ee::s11::ModObj_v48,
        }
        .call(c, 0x0035539c)?;
        c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x12));
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.g[V1][0] | 0x1;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sh((c.g[S0][0] as u32).wrapping_add(0x12), c.g[V1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `bool AnimDelta_v15(self)` (0x3592d0): one step of the delta animation:
/// stores the float literal at gp-0x2a6c to `+0x24` and `+0x28`, runs the
/// loop-counter / remaining-time bookkeeping (`+0xc`, `+0x6c`, `+0x10`, flags at `+0x42`) and calls `AnimObject_v15` or the
/// owner's virtual at `+0x110`. Returns 1 or 0.
pub fn anim_delta_v15(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_3593bc: {
            'b_3593b8: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2a6c));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                let t = c.g[V0][0] != 0u64;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x24));
                if t {
                    break 'b_3593b8;
                } else {
                    'b_3593b0: {
                        'b_359374: {
                            c.cycles += 6;
                            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x6c));
                            c.mtc1(0, 2);
                            c.c_lt_s(2, 1);
                            let t = !c.fcc();
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            if t {
                                break 'b_359374;
                            } else {
                                c.cycles += 2;
                                if (c.g[V0][0] as i64) < 0 {
                                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x42));
                                    break 'b_3593b0;
                                } else {
                                    'b_359344: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;

                                        if t {
                                            break 'b_359344;
                                        } else {
                                            c.cycles += 2;
                                            let t = !c.fcc();

                                            if t {
                                                break 'b_359344;
                                            } else {
                                                c.cycles += 7;
                                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x10));
                                                c.abs_s(0, 0);
                                                c.sub_s(0, 1, 0);
                                                c.c_lt_s(0, 2);
                                                let t = !c.fcc();
                                                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x6c));
                                                if t {
                                                    break 'b_359344;
                                                } else {
                                                    c.cycles += 1;
                                                    c.swc1(
                                                        2,
                                                        (c.g[S0][0] as u32).wrapping_add(0x6c),
                                                    );
                                                    break 'b_359344;
                                                }
                                            }
                                        }
                                    }
                                    'b_35936c: {
                                        c.cycles += 8;
                                        c.g[RA][0] = sx(0x0035934c);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x00341d48,
                                            f: ee::s11::AnimObject_v15,
                                        }
                                        .call(c, 0x0035934c)?;
                                        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x6c));
                                        c.mtc1(0, 0);
                                        c.c_lt_s(0, 1);
                                        let t = !c.fcc();
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_35936c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                            if t {
                                                break 'b_3593bc;
                                            } else {
                                                break 'b_35936c;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
                                    if t {
                                        break 'b_3593b8;
                                    } else {
                                        break 'b_359374;
                                    }
                                }
                            }
                        }
                        'b_359380: {
                            c.cycles += 2;
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            if t {
                                break 'b_359380;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                                break 'b_359380;
                            }
                        }
                        'b_3593ac: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_3593ac;
                            } else {
                                c.cycles += 8;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x110));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x114));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x003593a4);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x003593a4)?;
                                let t = true;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_3593bc;
                                } else {
                                    break 'b_3593ac;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x42));
                        break 'b_3593b0;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] | 0x1;
                    c.sh((c.g[S0][0] as u32).wrapping_add(0x42), c.g[V0][0]);
                    break 'b_3593b8;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_3593bc;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `RailModifier* RailModifier::RailModifier(self, Stream* s)` (0x35ba88): the
/// stream constructor: vtable, two reads (the 0x30-byte bound box and the
/// instance), `BuildRailNodes`, the nash insert of the whole box, then the
/// removal of every segment box of the instance's track from the spatial hash,
/// with the track found through the instance handle (`World_GetInstance`).
/// The resumable pc-dispatcher body converted from the generated text: the
/// normal entry only, any other `pc` (a resume after an unwound callee or a
/// delivered event) runs the generated body. Returns `self`.
pub fn rail_modifier_ctor_2(c: &mut Cpu) {
    if c.pc != 0x0035ba88 {
        // INSTALL: switch to `ee::s11::f_0035ba88__generated` once the function
        // is in overrides.txt (the plain name goes through the override slot).
        return ee::s11::f_0035ba88__generated(c);
    }
    guest(c, |c| -> Flow {
        let mut pc: u32 = 0x0035ba88;
        loop {
            match pc {
                0x0035ba88 => {
                    c.cycles += 70;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.g[V1][0] = 0xffffffffffff0000;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[V0][0] = 0x490000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x11d0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[A2][0] = 0x30;
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x30), c.g[V1][0]);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sw((c.g[S3][0] as u32), 0u64);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[A3][0] as u32;
                    c.g[RA][0] = sx(0x0035baf4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0035baf4)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0035bb08);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0035bb08)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0035bb14);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    Callee {
                        addr: 0x0035c0e8,
                        f: ee::s11::RailModifier_BuildRailNodes,
                    }
                    .call(c, 0x0035bb14)?;
                    c.g[RA][0] = sx(0x0035bb1c);
                    call_ptr(c, 0x002d1be0, 0x0035bb1c)?;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0035bb30);
                    c.g[S1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x44));
                    call_ptr(c, 0x00328f28, 0x0035bb30)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = 0xffffffffffffffff;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[S0][0];
                    }
                    c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    if c.g[A1][0] != 0 {
                        c.g[A0][0] = c.g[V0][0];
                    }
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = 0x2;
                    if c.g[A3][0] == 0 {
                        c.g[A0][0] = c.g[V0][0];
                    }
                    c.g[RA][0] = sx(0x0035bb74);
                    c.g[A3][0] = c.g[SP][0];
                    call_ptr(c, 0x00328c20, 0x0035bb74)?;
                    c.g[RA][0] = sx(0x0035bb7c);
                    call_ptr(c, 0x002d1bd8, 0x0035bb7c)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[A1][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    if c.g[V1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                        pc = 0x0035bba8;
                        continue;
                    }
                    pc = 0x0035bba0;
                    continue;
                }
                0x0035bba0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S5][0] = 0u64;
                    if t {
                        pc = 0x0035bbcc;
                        continue;
                    }
                    pc = 0x0035bba8;
                    continue;
                }
                0x0035bba8 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x0035bbc8;
                        continue;
                    }
                    pc = 0x0035bbc4;
                    continue;
                }
                0x0035bbc4 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x0035bbc8;
                    continue;
                }
                0x0035bbc8 => {
                    c.cycles += 1;
                    c.g[S5][0] = c.g[V0][0];
                    pc = 0x0035bbcc;
                    continue;
                }
                0x0035bbcc => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x20));
                    c.g[S2][0] = 0u64;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
                    if t {
                        pc = 0x0035bd40;
                        continue;
                    }
                    pc = 0x0035bbdc;
                    continue;
                }
                0x0035bbdc => {
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 20);
                    c.g[S4][0] = 0xffffffffffffffff;
                    pc = 0x0035bbe8;
                    continue;
                }
                0x0035bbe8 => {
                    c.cycles += 44;
                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x6c));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x70));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                    c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x74));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1c));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x78));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x7c));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x80));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x2c));
                    c.g[RA][0] = sx(0x0035bc24);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x28));
                    call_ptr(c, 0x002d1be0, 0x0035bc24)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x0035bc40);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                    call_ptr(c, 0x00328f28, 0x0035bc40)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x2;
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A0][0] = ((c.g[S4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[T0][0] = ((c.g[S4][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[A2][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[A3][0] = c.g[SP][0];
                    if c.g[T0][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0035bc88);
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x003284b8, 0x0035bc88)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035bcec;
                        continue;
                    }
                    pc = 0x0035bc98;
                    continue;
                }
                0x0035bc98 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035bcec;
                        continue;
                    }
                    pc = 0x0035bca4;
                    continue;
                }
                0x0035bca4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035bcec;
                        continue;
                    }
                    pc = 0x0035bcb0;
                    continue;
                }
                0x0035bcb0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035bcec;
                        continue;
                    }
                    pc = 0x0035bcbc;
                    continue;
                }
                0x0035bcbc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035bcec;
                        continue;
                    }
                    pc = 0x0035bcc8;
                    continue;
                }
                0x0035bcc8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035bcec;
                        continue;
                    }
                    pc = 0x0035bcd4;
                    continue;
                }
                0x0035bcd4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0035bcec;
                        continue;
                    }
                    pc = 0x0035bce0;
                    continue;
                }
                0x0035bce0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    let t = c.g[V0][0] == 0u64;
                    if t {
                        pc = 0x0035bcf4;
                        continue;
                    }
                    pc = 0x0035bcec;
                    continue;
                }
                0x0035bcec => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0035bd1c;
                        continue;
                    }
                    pc = 0x0035bcf4;
                    continue;
                }
                0x0035bcf4 => {
                    c.cycles += 3;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = 0u64;
                    pc = 0x0035bd00;
                    continue;
                }
                0x0035bd00 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0035bcec) {
                            return Ok(());
                        }
                        pc = 0x0035bcec;
                        continue;
                    }
                    pc = 0x0035bd0c;
                    continue;
                }
                0x0035bd0c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0035bd00) {
                            return Ok(());
                        }
                        pc = 0x0035bd00;
                        continue;
                    }
                    pc = 0x0035bd18;
                    continue;
                }
                0x0035bd18 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x0035bd1c;
                    continue;
                }
                0x0035bd1c => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x20));
                        pc = 0x0035bd34;
                        continue;
                    }
                    pc = 0x0035bd24;
                    continue;
                }
                0x0035bd24 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0035bd2c);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317e50, 0x0035bd2c)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x20));
                    pc = 0x0035bd34;
                    continue;
                }
                0x0035bd34 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                    if t {
                        if back_edge_unwinds(c, 0x0035bbe8) {
                            return Ok(());
                        }
                        pc = 0x0035bbe8;
                        continue;
                    }
                    pc = 0x0035bd40;
                    continue;
                }
                0x0035bd40 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.g[S3][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0035ba88, 0x0035bd6c);
                    return Ok(());
                }
            }
        }
    });
}

/// `void VramHeap_EvictRange(VramHeap* h, int size, int skip_a, int skip_b, int
/// level)` (0x3669f0): slides a window over the heap's address-ordered block
/// chain looking for the cheapest run of blocks (>= `size` bytes, no locked
/// block, owner neither `skip_a` nor `skip_b`, cost from the owner table at
/// 0x53ab80), then frees every used block of that run (`VramHeap_FreeBlock`
/// for each, collected in a 292-entry frame array first).
pub fn vram_heap_evict_range(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_366a90: {
            'b_366a8c: {
                c.cycles += 15;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4d0));
                c.g[V0][0] = 0x1c;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x4a0));
                c.g[T7][0] = 0x7fff0000;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x4c0));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x4b0));
                c.g[T8][0] = 0xffffffffffffffff;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x490), c.g[RA][0]);
                c.g[T7][0] = c.g[T7][0] | 0xffff;
                c.g[T2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x23c0));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1ff0));
                c.mult(0, 2, 10, 2);
                let t = (c.g[T2][0] as i64) < 0;
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                if t {
                    break 'b_366a8c;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                    c.g[V0][0] = c.g[V1][0] & 0x1;
                    let t = c.g[V0][0] == 0u64;
                    c.g[T3][0] = c.g[T2][0];
                    if t {
                        break 'b_366a90;
                    } else {
                        c.cycles += 3;
                        c.g[T1][0] = 0x1c;
                        c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                        'l_366a48: loop {
                            'b_366a6c: {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                if c.g[V0][0] != 0u64 {
                                    c.g[T2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                    break 'b_366a6c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                    if c.g[V0][0] == c.g[A2][0] {
                                        c.g[T2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                        break 'b_366a6c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != c.g[A3][0];
                                        c.g[T3][0] = c.g[T2][0];
                                        if t {
                                            break 'b_366a90;
                                        } else {
                                            c.cycles += 1;
                                            c.g[T2][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                            break 'b_366a6c;
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1ff0));
                            c.mult(0, 2, 10, 9);
                            let t = (c.g[T2][0] as i64) < 0;
                            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            if t {
                                break 'b_366a8c;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                c.g[V0][0] = c.g[V1][0] & 0x1;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00366a48);
                                    }
                                    continue 'l_366a48;
                                } else {
                                    break 'b_366a8c;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[T3][0] = c.g[T2][0];
            break 'b_366a90;
        }
        'b_366c28: {
            'b_366c24: {
                c.cycles += 4;
                c.g[T4][0] = c.g[A0][0];
                c.g[T6][0] = 0u64;
                let t = (c.g[T2][0] as i64) < 0;
                c.g[T5][0] = 0u64;
                if t {
                    break 'b_366c24;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = 0x540000;
                    c.g[S0][0] = 0x1c;
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5480));
                    'l_366ab0: loop {
                        'b_366c14: {
                            'b_366b1c: {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[A1][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V0][0] = c.lw((c.g[T4][0] as u32));
                                    break 'b_366b1c;
                                } else {
                                    'b_366ad0: {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[T5][0] as i64) < (c.g[T7][0] as i64)) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_366ad0;
                                        } else {
                                            c.cycles += 2;
                                            c.g[T8][0] = c.g[T2][0];
                                            c.g[T7][0] = c.g[T5][0];
                                            break 'b_366ad0;
                                        }
                                    }
                                    'b_366b0c: {
                                        'b_366b08: {
                                            c.cycles += 7;
                                            c.g[T2][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                            c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                            c.g[T1][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[S1][0] as u32)));
                                            c.g[V1][0] = c.g[V1][0] & 0x1;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[T6][0] = sx((c.g[T6][0] as u32)
                                                .wrapping_sub((c.g[T2][0] as u32)));
                                            if t {
                                                break 'b_366b08;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V0][0] = c.lw((c.g[T1][0] as u32));
                                                c.g[V0][0] = ((c.g[T0][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[T2][0] = c
                                                        .lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                                    break 'b_366b0c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                                                    c.mult(0, 2, 10, 2);
                                                    c.g[T5][0] = sx((c.g[T5][0] as u32)
                                                        .wrapping_sub((c.g[V0][0] as u32)));
                                                    break 'b_366b08;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[T2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                        break 'b_366b0c;
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1ff0));
                                    c.mult(0, 2, 10, 16);
                                    let t = true;
                                    c.g[A0][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    if t {
                                        break 'b_366c14;
                                    } else {
                                        break 'b_366b1c;
                                    }
                                }
                            }
                            'b_366bd4: {
                                'b_366bcc: {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                                    if t {
                                        break 'b_366bcc;
                                    } else {
                                        'b_366b50: {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[T2][0] =
                                                    c.lw((c.g[T4][0] as u32).wrapping_add(0x10));
                                                break 'b_366b50;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[T4][0] as u32).wrapping_add(0x8));
                                                if c.g[V0][0] == c.g[A2][0] {
                                                    c.g[T2][0] = c
                                                        .lw((c.g[T4][0] as u32).wrapping_add(0x10));
                                                    break 'b_366b50;
                                                } else {
                                                    c.cycles += 2;
                                                    if c.g[V0][0] != c.g[A3][0] {
                                                        c.g[V1][0] = c.g[V1][0] & 0x1;
                                                        break 'b_366bd4;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[T2][0] = c
                                                            .lw((c.g[T4][0] as u32)
                                                                .wrapping_add(0x10));
                                                        break 'b_366b50;
                                                    }
                                                }
                                            }
                                        }
                                        'b_366bbc: {
                                            'b_366bb8: {
                                                c.cycles += 4;
                                                c.g[T1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x1ff0));
                                                c.mult(0, 2, 10, 16);
                                                let t = (c.g[T2][0] as i64) < 0;
                                                c.g[A0][0] = sx((c.g[T1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                if t {
                                                    break 'b_366bb8;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                    c.g[V0][0] = c.g[V1][0] & 0x1;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[T3][0] = c.g[T1][0];
                                                    if t {
                                                        break 'b_366bb8;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[T1][0] = 0x1c;
                                                        c.g[V0][0] =
                                                            sx(((c.g[V1][0] as u32) as i32 >> 1)
                                                                as u32);
                                                        'l_366b78: loop {
                                                            'b_366b9c: {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                                if c.g[V0][0] != 0u64 {
                                                                    c.g[T2][0] = c
                                                                        .lw((c.g[A0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_366b9c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    if c.g[V0][0] == c.g[A2][0] {
                                                                        c.g[T2][0] = c.lw((c.g[A0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_366b9c;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] != c.g[A3][0]
                                                                        {
                                                                            c.g[T3][0] = c.g[T2][0];
                                                                            break 'b_366bbc;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[T2][0] = c.lw((c.g
                                                                                [A0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x10,
                                                                                ));
                                                                            break 'b_366b9c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.mult(0, 2, 10, 9);
                                                            let t = (c.g[T2][0] as i64) < 0;
                                                            c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[T3][0] as u32)));
                                                            if t {
                                                                break 'b_366bb8;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[V1][0] =
                                                                    c.lw((c.g[A0][0] as u32));
                                                                c.g[V0][0] = c.g[V1][0] & 0x1;
                                                                let t = c.g[V0][0] != 0u64;
                                                                c.g[V0][0] =
                                                                    sx(((c.g[V1][0] as u32) as i32
                                                                        >> 1)
                                                                        as u32);
                                                                if t {
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x00366b78,
                                                                        );
                                                                    }
                                                                    continue 'l_366b78;
                                                                } else {
                                                                    break 'b_366bb8;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[T3][0] = c.g[T2][0];
                                            break 'b_366bbc;
                                        }
                                        c.cycles += 4;
                                        c.g[T4][0] = c.g[A0][0];
                                        c.g[T6][0] = 0u64;
                                        let t = true;
                                        c.g[T5][0] = 0u64;
                                        if t {
                                            break 'b_366c14;
                                        } else {
                                            break 'b_366bcc;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[V0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.g[V1][0] & 0x1;
                                break 'b_366bd4;
                            }
                            'b_366c08: {
                                'b_366c04: {
                                    c.cycles += 5;
                                    c.g[T3][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                    c.g[T1][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[T6][0] =
                                        sx((c.g[T6][0] as u32).wrapping_add((c.g[T3][0] as u32)));
                                    if t {
                                        break 'b_366c04;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[T1][0] as u32));
                                        c.g[V0][0] =
                                            ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[T3][0] =
                                                c.lw((c.g[T4][0] as u32).wrapping_add(0x10));
                                            break 'b_366c08;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                                            c.mult(0, 3, 11, 2);
                                            c.g[T5][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[T5][0] as u32)));
                                            break 'b_366c04;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[T3][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x10));
                                break 'b_366c08;
                            }
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1ff0));
                            c.mult(0, 2, 11, 16);
                            c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            break 'b_366c14;
                        }
                        c.cycles += 2;
                        let t = (c.g[T3][0] as i64) < 0;
                        c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        if t {
                            break 'b_366c28;
                        } else {
                            c.cycles += 2;
                            let t = (c.g[T7][0] as i64) > 0;

                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00366ab0);
                                }
                                continue 'l_366ab0;
                            } else {
                                break 'b_366c24;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[A1][0] as i64)) as u64;
            break 'b_366c28;
        }
        'b_366c40: {
            c.cycles += 2;
            let t = c.g[V0][0] != 0u64;
            c.g[A3][0] = c.g[T8][0];
            if t {
                break 'b_366c40;
            } else {
                'b_366c3c: {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[T5][0] as i64) < (c.g[T7][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[T8][0] = c.g[T2][0];
                        break 'b_366c3c;
                    } else {
                        break 'b_366c3c;
                    }
                }
                c.cycles += 1;
                c.g[A3][0] = c.g[T8][0];
                break 'b_366c40;
            }
        }
        'b_366ca0: {
            c.cycles += 3;
            c.g[T1][0] = 0u64;
            let t = (c.g[A1][0] as i64) <= 0;
            c.g[T0][0] = 0u64;
            if t {
                break 'b_366ca0;
            } else {
                c.cycles += 5;
                c.g[T2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1ff0));
                c.g[T3][0] = 0x1c;
                c.g[A2][0] = c.g[SP][0];
                c.mult(0, 4, 7, 11);
                'l_366c60: loop {
                    'b_366c84: {
                        c.cycles += 5;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        if c.g[V0][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1ff0));
                            break 'b_366c84;
                        } else {
                            c.cycles += 4;
                            c.sw((c.g[A2][0] as u32), c.g[A3][0]);
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1ff0));
                            break 'b_366c84;
                        }
                    }
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.mult(0, 4, 7, 11);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00366c60);
                        }
                        continue 'l_366c60;
                    } else {
                        break 'b_366ca0;
                    }
                }
            }
        }
        'b_366cc8: {
            c.cycles += 2;
            let t = (c.g[T0][0] as i64) <= 0;
            c.g[S1][0] = c.g[T0][0];
            if t {
                break 'b_366cc8;
            } else {
                c.cycles += 2;
                c.g[S0][0] = c.g[SP][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                'l_366cb0: loop {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x00366cc0);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    Callee {
                        addr: 0x003663d8,
                        f: ee::s12::VramHeap_FreeBlock,
                    }
                    .call(c, 0x00366cc0)?;
                    if c.g[S1][0] != 0u64 {
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                        if event_due(c) {
                            deliver_event_here(c, 0x00366cb0);
                        }
                        continue 'l_366cb0;
                    } else {
                        break 'b_366cc8;
                    }
                }
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x4c0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x4b0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x4a0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x490));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4d0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_lohi, fakes, fill, Rng, DATA, GAME_GP, SP as TEST_SP};

    /// Gate of the fake callees below: written into the test machine's own
    /// RAM, so other tests' machines reach the bodies the slots held.
    const OC687_GATE: u32 = DATA + 0xe_0000;
    /// The instance handle `RailModifier_BuildRailNodes` (fake) stores.
    const OC687_HANDLE: u32 = DATA + 0xe_0100;
    const OC687_MAGIC: u32 = 0x0687_c0de;
    /// Fixture data lives above the area the fakes' returned pointers point
    /// into (`DATA..DATA + 0x40000`).
    const FIX: u32 = DATA + 0x4_0000;

    /// Gated fakes for the callees that have override slots: with the gate
    /// set they log the call and return a pointer (or unwind); otherwise
    /// they run the body the slot held.
    macro_rules! oc687_fakes {
        ($($fake:ident, $shard:ident, $slot:ident, $gen:ident, $addr:expr;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC687_GATE) != OC687_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::$shard::$gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc687_install_plain_fakes() {
                $(
                    fakes::hook(
                        std::ptr::addr_of_mut!(ee::$shard::$slot),
                        $fake,
                        fakes::prev_cell($fake),
                    );
                )*
            }
        };
    }

    oc687_fakes! {
        oc687_fake_get_nash, s09, OVERRIDE_002D1BE0, f_002d1be0__generated, 0x002d_1be0;
        oc687_fake_node_matrix, s11, OVERRIDE_0034FED8, f_0034fed8__generated, 0x0034_fed8;
        oc687_fake_world_ctor_2, s11, OVERRIDE_0034FB90, f_0034fb90__generated, 0x0034_fb90;
        oc687_fake_alloc_named, s10, OVERRIDE_00317D70, f_00317d70__generated, 0x0031_7d70;
        oc687_fake_new_named, s10, OVERRIDE_00317E30, f_00317e30__generated, 0x0031_7e30;
        oc687_fake_delete, s10, OVERRIDE_00317E50, f_00317e50__generated, 0x0031_7e50;
        oc687_fake_spline_ctor, s11, OVERRIDE_003595D8, f_003595d8__generated, 0x0035_95d8;
        oc687_fake_multi_spline_ctor, s11, OVERRIDE_0035A118, f_0035a118__generated, 0x0035_a118;
        oc687_fake_parent_ctor, s11, OVERRIDE_00357090, f_00357090__generated, 0x0035_7090;
        oc687_fake_position_ctor, s11, OVERRIDE_00356F68, f_00356f68__generated, 0x0035_6f68;
        oc687_fake_roller_ctor, s11, OVERRIDE_0035DF70, f_0035df70__generated, 0x0035_df70;
        oc687_fake_magnet_ctor, s11, OVERRIDE_00357358, f_00357358__generated, 0x0035_7358;
        oc687_fake_ava_ctor, s11, OVERRIDE_00357798, f_00357798__generated, 0x0035_7798;
        oc687_fake_uv_ctor, s12, OVERRIDE_0035F788, f_0035f788__generated, 0x0035_f788;
        oc687_fake_texflip_ctor, s12, OVERRIDE_0035F1A8, f_0035f1a8__generated, 0x0035_f1a8;
        oc687_fake_boost_ctor, s11, OVERRIDE_00342808, f_00342808__generated, 0x0034_2808;
        oc687_fake_particle_ctor, s11, OVERRIDE_003459A8, f_003459a8__generated, 0x0034_59a8;
        oc687_fake_dyn_particle_ctor, s11, OVERRIDE_00345D80, f_00345d80__generated, 0x0034_5d80;
        oc687_fake_halo_ctor, s11, OVERRIDE_003461C0, f_003461c0__generated, 0x0034_61c0;
        oc687_fake_list_push_back, s11, OVERRIDE_00345720, f_00345720__generated, 0x0034_5720;
        oc687_fake_rail_list_push_back, s11, OVERRIDE_0035B690, f_0035b690__generated, 0x0035_b690;
        oc687_fake_key_from_aabb, s10, OVERRIDE_00328F28, f_00328f28__generated, 0x0032_8f28;
        oc687_fake_hash_insert, s10, OVERRIDE_00328C20, f_00328c20__generated, 0x0032_8c20;
        oc687_fake_hash_remove, s10, OVERRIDE_003284B8, f_003284b8__generated, 0x0032_84b8;
        oc687_fake_hash_update, s10, OVERRIDE_003291E0, f_003291e0__generated, 0x0032_91e0;
        oc687_fake_mod_obj_v50, s11, OVERRIDE_003568B0, f_003568b0__generated, 0x0035_68b0;
        oc687_fake_anim_v52, s11, OVERRIDE_0034E348, f_0034e348__generated, 0x0034_e348;
        oc687_fake_alloc_node_info, s11, OVERRIDE_0034E448, f_0034e448__generated, 0x0034_e448;
        oc687_fake_alloc_bound_box, s11, OVERRIDE_0034EEB8, f_0034eeb8__generated, 0x0034_eeb8;
        oc687_fake_create_block, s11, OVERRIDE_00355468, f_00355468__generated, 0x0035_5468;
        oc687_fake_mod_obj_v48, s11, OVERRIDE_00356780, f_00356780__generated, 0x0035_6780;
        oc687_fake_anim_v15, s11, OVERRIDE_00341D48, f_00341d48__generated, 0x0034_1d48;
        oc687_fake_free_block, s12, OVERRIDE_003663D8, f_003663d8__generated, 0x0036_63d8;
    }

    /// `RailModifier_BuildRailNodes` (fake): logs the call like the others,
    /// and stores the handle the fixture chose at `rail + 0x30` (the real one
    /// stores the instance handle there).
    fn oc687_fake_build_rail_nodes(c: &mut Cpu) {
        if c.bus.read32(OC687_GATE) != OC687_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc687_fake_build_rail_nodes),
                ee::s11::f_0035c0e8__generated,
            );
        }
        let rail = c.g[A0][0] as u32;
        let handle = c.bus.read32(OC687_HANDLE);
        fakes::fake_call(c, 0x0035_c0e8);
        c.bus.write32(rail.wrapping_add(0x30), handle);
    }

    fn oc687_install_fakes() {
        oc687_install_plain_fakes();
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0035C0E8),
            oc687_fake_build_rail_nodes,
            fakes::prev_cell(oc687_fake_build_rail_nodes),
        );
    }

    /// Turn the fakes on in this machine.
    fn oc687_gate(c: &mut Cpu) {
        c.bus.write32(OC687_GATE, OC687_MAGIC);
    }

    /// A vtable at `vt` whose slot at `off` calls fake method number `n`
    /// (`{delta, fn}`: the receiver is moved by the random delta).
    fn oc687_vslot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, n: u32) {
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * n);
    }

    /// A replay stream at `st` whose vtable has two read methods (+0x10
    /// `read(buf, n)`, +0x18 `readReplayInstance()`).
    fn oc687_stream(c: &mut Cpu, r: &mut Rng, st: u32) {
        let vt = st + 0x100;
        c.bus.write32(st, vt);
        oc687_vslot(c, r, vt, 0x10, 0);
        oc687_vslot(c, r, vt, 0x18, 1);
    }

    /// A zero or non-zero word at `addr` (`nonzero` of the seed's choice).
    fn oc687_flag(c: &mut Cpu, r: &mut Rng, addr: u32, nonzero: bool) {
        let v = if nonzero { r.next_u32() | 1 } else { 0 };
        c.bus.write32(addr, v);
    }

    /// `SpringRail_InitPosFromInstance`: the instance's handle (low byte and
    /// the rest) walks the world's two-level instance table (entry missing,
    /// or second level empty, or found); the node matrix call is a fake.
    #[test]
    fn spring_rail_init_pos_from_instance_matches() {
        oc687_install_fakes();
        fakes::covered(
            ee::s11::f_0034a028__generated,
            spring_rail_init_pos_from_instance,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x34_a028, 1);
                oc687_gate(c);
                c.pc = 0x0034_a028;
                let (inst, world, w1, t1, e, t2) = (
                    FIX,
                    FIX + 0x1000,
                    FIX + 0x1100,
                    FIX + 0x1200,
                    FIX + 0x1300,
                    FIX + 0x1400,
                );
                c.bus.write32(GAME_GP + 0x16c8, world);
                c.bus.write32(world, w1);
                c.bus.write32(w1 + 8, t1);
                let (lo, hi) = (r.next_u32() % 8, r.next_u32() % 8);
                c.bus.write32(inst + 0x50, lo | hi << 8);
                c.bus.write32(inst + 0x54, r.next_u32());
                c.bus
                    .write32(t1 + lo * 4, if seed % 4 == 0 { 0 } else { e });
                c.bus.write32(e + 0x1c, t2);
                let x = if seed % 3 == 0 {
                    r.next_u32() & 0xff
                } else {
                    r.next_u32() | 0x100
                };
                c.bus.write32(t2 + hi * 4, x);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = inst as u64;
            },
        );
    }

    /// `SpringRail_v48` (leaf): 1..6 nodes (no loop, one pass, several), random
    /// node floats and matrices; the matrix constant at 0x4ffac0.
    #[test]
    fn spring_rail_v48_matches() {
        check_lohi(
            ee::s11::f_0034a0b0__generated,
            spring_rail_v48,
            300,
            &|c, seed| {
                let mut r = Rng(0x34a0_b0aa ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0034_a0b0;
                c.g[GP][0] = GAME_GP as u64;
                let (rail, mats, nodes) = (FIX, FIX + 0x1000, FIX + 0x2000);
                c.g[A0][0] = rail as u64;
                c.bus.write32(rail + 0x20, seed % 6 + 1);
                c.bus.write32(rail + 0x50, mats);
                c.bus.write32(rail + 0x70, nodes);
                fill(c, &mut r, mats, 0x100);
                for i in 0..0x180 {
                    let v = if seed % 5 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(4.0)
                    };
                    c.bus.write32(nodes + 4 * i, v);
                }
                for i in 0..16 {
                    c.bus.write32(0x004f_fac0 + 4 * i, r.f32(2.0));
                }
            },
        );
    }

    /// `AnimObject_BaseCtor_2` with the real `ModObj_ctor_2` (and, through it,
    /// `ModifierBlock_Load` on an all-zero header): the mesh count of the
    /// node's model is 0, 1, 3 or 0x40 (the allocation size and the `-1`
    /// slot count), the block word read by `ModObj_ctor_2` zero or not.
    #[test]
    fn anim_object_base_ctor_2_matches() {
        oc687_install_fakes();
        fakes::covered(
            ee::s11::f_0034dac8__generated,
            anim_object_base_ctor_2,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x34_dac8, 10);
                oc687_gate(c);
                c.pc = 0x0034_dac8;
                let (me, st, model, info) = (FIX, FIX + 0x1000, FIX + 0x1400, FIX + 0x1500);
                oc687_stream(c, &mut r, st);
                c.bus.write32(me + 0x2c, model);
                c.bus.write32(model + 0x80, info);
                c.bus
                    .write32(info + 4, [0, 1, 3, 0x40][(seed % 4) as usize]);
                oc687_flag(c, &mut r, TEST_SP - 0xa0, seed % 3 != 0);
                c.g[A0][0] = me as u64;
                c.g[A2][0] = st as u64;
            },
        );
    }

    /// `Floating_ctor_2` with the real `ModObj_ctor_2`: the bound-box word the
    /// first read stores (zero: no `AllocBoundBox`), and the mesh count.
    #[test]
    fn floating_ctor_2_matches() {
        oc687_install_fakes();
        fakes::covered(
            ee::s11::f_0034ed88__generated,
            floating_ctor_2,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x34_ed88, 10);
                oc687_gate(c);
                c.pc = 0x0034_ed88;
                let (me, st, model, info) = (FIX, FIX + 0x1000, FIX + 0x1400, FIX + 0x1500);
                oc687_stream(c, &mut r, st);
                c.bus.write32(me + 0x18, model);
                c.bus.write32(model + 0x80, info);
                c.bus
                    .write32(info + 4, [0, 1, 3, 0x40][(seed % 4) as usize]);
                oc687_flag(c, &mut r, TEST_SP - 0x50, seed % 2 == 0);
                oc687_flag(c, &mut r, TEST_SP - 0x90, seed % 3 != 0);
                c.g[A0][0] = me as u64;
                c.g[A2][0] = st as u64;
            },
        );
    }

    /// `Floating_v50`: no second part (`ModObj_v50` instead), or a part: the
    /// two virtual matrix getters, the VU0 `sub` / `add` of their rows, the
    /// nash lookup and the spatial update.
    #[test]
    fn floating_v50_matches() {
        oc687_install_fakes();
        fakes::covered(
            ee::s11::f_0034ef40__generated,
            floating_v50,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x34_ef40, 4);
                oc687_gate(c);
                c.pc = 0x0034_ef40;
                let (me, owner, part) = (FIX, FIX + 0x1000, FIX + 0x2000);
                c.bus.write32(me + 0xc, owner);
                c.bus.write32(me + 0x18, r.next_u32());
                c.bus
                    .write32(me + 0x78, if seed % 3 == 0 { 0 } else { part });
                // The owner's two virtual matrix getters share one vtable word
                // pair at +0xc0 (`{delta, fn}`).
                let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                c.bus.write16(owner + 0xc0, delta as u16);
                c.bus.write32(owner + 0xc4, fakes::FAKE);
                fill(c, &mut r, part, 0x10);
                for i in 0..12 {
                    c.bus.write32(part + 0x20 + 4 * i, r.f32(4.0));
                }
                c.g[A0][0] = me as u64;
            },
        );
    }

    /// Sets the 52-byte modifier header the stream's read method would fill
    /// (the fake read leaves it alone) at the callee's frame, `SP - 0xe0 -
    /// extra`.
    fn oc687_block_header(c: &mut Cpu, r: &mut Rng, seed: u32, extra: u32) {
        let h = TEST_SP - 0xe0 - extra;
        let flags = [0, 1, 3, 5, 9, 0xf, 2, 4][(seed % 8) as usize] | (r.next_u32() & 1);
        c.bus.write32(h, flags);
        c.bus.write32(h + 4, (seed / 8) % 9);
        c.bus.write32(h + 8, (seed / 3) % 3);
        c.bus.write32(h + 0xc, (seed / 5) % 4);
        c.bus.write32(h + 0x10, (seed / 7) % 3);
        for i in 0..4 {
            c.bus.write8(h + 0x14 + i, (r.next_u32() % 5) as u8);
        }
    }

    /// `ModifierBlock_Load`: the root transform type 1..7 (and none), the UV
    /// scroll, tex flip and boost flags, 0..3 effects of type 1..3 (or none),
    /// and 0..2 rails (the real `RailModifier_ctor_2` runs on a zero world).
    #[test]
    fn modifier_block_load_matches() {
        oc687_install_fakes();
        fakes::covered(
            ee::s11::f_003536d8__generated,
            modifier_block_load,
            400,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x35_36d8, 14);
                oc687_gate(c);
                c.pc = 0x0035_36d8;
                let (me, st) = (FIX, FIX + 0x1000);
                oc687_stream(c, &mut r, st);
                oc687_block_header(c, &mut r, seed, 0);
                c.g[A0][0] = me as u64;
                c.g[A1][0] = st as u64;
            },
        );
    }

    /// `ModObj_ctor_2`: the block word read last (zero: no block, else the
    /// block is created and loaded: `ModifierBlock_Load` runs for real on a
    /// zero header).
    #[test]
    fn mod_obj_ctor_2_matches() {
        oc687_install_fakes();
        fakes::covered(
            ee::s11::f_003552d8__generated,
            mod_obj_ctor_2,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x35_52d8, 8);
                oc687_gate(c);
                c.pc = 0x0035_52d8;
                let (me, st) = (FIX, FIX + 0x1000);
                oc687_stream(c, &mut r, st);
                oc687_flag(c, &mut r, TEST_SP - 0x40, seed % 3 != 0);
                c.g[A0][0] = me as u64;
                c.g[A2][0] = st as u64;
            },
        );
    }

    /// `AnimDelta_v15`: the update-state word, the loop counter at +0xc
    /// (negative, zero, positive), the float at +0x6c against zero, the
    /// delta at +0x10 and the flags halfword.
    #[test]
    fn anim_delta_v15_matches() {
        oc687_install_fakes();
        fakes::covered(
            ee::s11::f_003592d0__generated,
            anim_delta_v15,
            400,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x35_92d0, 2);
                oc687_gate(c);
                c.pc = 0x0035_92d0;
                let (me, owner) = (FIX, FIX + 0x1000);
                fill(c, &mut r, me, 0x20);
                c.bus
                    .write32(me + 8, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(
                    me + 0xc,
                    [u32::MAX, 0, 1, 5, 0x8000_0000, r.next_u32()][((seed / 2) % 6) as usize],
                );
                let f = match (seed / 4) % 5 {
                    0 => 0,
                    1 => 1.0f32.to_bits(),
                    2 => (-1.0f32).to_bits(),
                    3 => 0.25f32.to_bits(),
                    _ => r.f32(2.0),
                };
                c.bus.write32(me + 0x6c, f);
                c.bus.write32(me + 0x10, r.f32(2.0));
                c.bus.write32(me + 0x3c, owner);
                let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                c.bus.write16(owner + 0x110, delta as u16);
                c.bus.write32(owner + 0x114, fakes::FAKE);
                c.bus.write32(GAME_GP - 0x2a6c, r.f32(2.0));
                c.g[A0][0] = me as u64;
            },
        );
    }

    /// `RailModifier_ctor_2`: the instance handle goes through the world's two
    /// level table to an instance record with 0..3 rail nodes (or to none:
    /// handle not found), the stream reads and the spatial calls are fakes.
    #[test]
    fn rail_modifier_ctor_2_matches() {
        oc687_install_fakes();
        fakes::covered(
            ee::s11::f_0035ba88__generated,
            rail_modifier_ctor_2,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x35_ba88, 12);
                oc687_gate(c);
                c.pc = 0x0035_ba88;
                let (me, st) = (FIX, FIX + 0x1000);
                let (world, w1, t1, e, t2, inst, nodes) = (
                    FIX + 0x2000,
                    FIX + 0x2100,
                    FIX + 0x2200,
                    FIX + 0x2300,
                    FIX + 0x2400,
                    FIX + 0x3000,
                    FIX + 0x4000,
                );
                oc687_stream(c, &mut r, st);
                c.bus.write32(GAME_GP + 0x16c8, world);
                c.bus.write32(world, w1);
                c.bus.write32(w1 + 8, t1);
                let (lo, hi) = (r.next_u32() % 8, r.next_u32() % 8);
                c.bus.write32(OC687_HANDLE, lo | hi << 8);
                c.bus
                    .write32(t1 + lo * 4, if seed % 5 == 0 { 0 } else { e });
                c.bus.write32(e + 0x44, t2);
                let x = if seed % 7 == 1 {
                    r.next_u32() & 0xff
                } else {
                    (inst >> 2) << 8 | (r.next_u32() & 0xff)
                };
                c.bus.write32(t2 + hi * 4, x);
                let n = (seed / 5) % 4;
                c.bus.write32(inst + 0x20, n);
                c.bus.write32(inst + 0x24, nodes);
                for k in 0..4 {
                    let nd = nodes + 0x100 * k;
                    for i in 0..0x40 {
                        c.bus.write32(nd + 4 * i, r.f32(4.0));
                    }
                    c.bus
                        .write32(nd + 0x64, if k < 3 { nodes + 0x100 * (k + 1) } else { 0 });
                }
                c.g[A0][0] = me as u64;
                c.g[A1][0] = st as u64;
            },
        );
    }

    const OC687_VRAM_OWNERS: u32 = 0x0053_ab80;

    /// `VramHeap_EvictRange`: 1..10 blocks of a chain (used, free, locked,
    /// owners equal to the skipped ones, owner costs), a size that fits
    /// somewhere or not; `VramHeap_FreeBlock` is a fake.
    #[test]
    fn vram_heap_evict_range_matches() {
        oc687_install_fakes();
        fakes::covered(
            ee::s12::f_003669f0__generated,
            vram_heap_evict_range,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x36_69f0, 3);
                oc687_gate(c);
                c.pc = 0x0036_69f0;
                let (heap, nodes) = (FIX, FIX + 0x4000);
                let n = 1 + r.next_u32() % 10;
                c.bus.write32(heap + 0x1ff0, nodes);
                c.bus.write32(heap + 0x23c0, 0);
                // The node before the first one (index -1): not used, huge.
                c.bus.write32(nodes - 0x1c, 0);
                c.bus.write32(nodes - 0x18, 0x7fff_ffff);
                c.bus.write32(nodes - 0x14, 0);
                c.bus.write32(nodes - 0x0c, u32::MAX);
                let skip_a = r.next_u32() % 8;
                let skip_b = r.next_u32() % 8;
                let mut total = 0;
                for i in 0..n {
                    let nd = nodes + 0x1c * i;
                    let used = r.next_u32() % 3 != 0;
                    let locked = r.next_u32() % 4 == 0;
                    c.bus.write32(nd, used as u32 | (locked as u32) << 1);
                    let size = 0x100 * (1 + r.next_u32() % 4);
                    total += size;
                    c.bus.write32(nd + 4, size);
                    let owner = match r.next_u32() % 4 {
                        0 => skip_a,
                        1 => skip_b,
                        _ => r.next_u32() % 8,
                    };
                    c.bus.write32(nd + 8, owner);
                    c.bus
                        .write32(nd + 0x10, if i + 1 < n { i + 1 } else { u32::MAX });
                }
                for i in 0..8 {
                    c.bus.write32(OC687_VRAM_OWNERS + 8 * i, r.next_u32() % 8);
                    c.bus
                        .write32(OC687_VRAM_OWNERS + 8 * i + 4, r.next_u32() % 50);
                }
                c.g[A0][0] = heap as u64;
                c.g[A1][0] = match seed % 6 {
                    0 => 0x100,
                    1 => total as u64,
                    2 => (total + 0x100) as u64,
                    _ => (0x100 * (1 + r.next_u32() % 8)) as u64,
                };
                c.g[A2][0] = skip_a as u64;
                c.g[A3][0] = if seed % 5 == 0 {
                    0x7fff_ffff
                } else {
                    skip_b as u64
                };
                c.g[T0][0] = (r.next_u32() % 8) as u64;
            },
        );
    }
}
