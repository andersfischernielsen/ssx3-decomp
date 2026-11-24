//! Staging module oc483 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc483`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0024e8d8,
    visual_effects_menus_static_init_destroy,
    "VisualEffectsMenus_StaticInitDestroy",
)];

/// `VisualEffectsMenus_StaticInitDestroy` (0x0024e8d8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn visual_effects_menus_static_init_destroy(c: &mut Cpu) {
    if c.pc != 0x0024e8d8 {
        return ee::s06::f_0024e8d8__generated(c);
    }
    guest(c, |c| {
        'b_24f694: {
            'b_24f690: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.g[V0][0] = 0xffff;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[A1][0] != c.g[V0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_24f690;
                } else {
                    'b_24f270: {
                        c.cycles += 2;
                        let t = c.g[A0][0] == 0u64;
                        c.g[T5][0] = 0x4d0000;
                        if t {
                            break 'b_24f270;
                        } else {
                            c.cycles += 599;
                            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4fd4));
                            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4fd0));
                            c.g[T6][0] = 0x4d0000;
                            c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4fcc));
                            c.g[T7][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4fc8));
                            c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x6a70));
                            c.swc1(9, (c.g[T5][0] as u32).wrapping_sub(0x6a90));
                            c.g[S1][0] = 0x4d0000;
                            c.swc1(10, (c.g[T6][0] as u32).wrapping_sub(0x6a80));
                            c.g[S2][0] = 0x4d0000;
                            c.swc1(11, (c.g[T7][0] as u32).wrapping_sub(0x6a70));
                            c.g[S3][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4fc4));
                            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6a40));
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4fc0));
                            c.g[S5][0] = 0x4d0000;
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4fbc));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[T3][0] = 0x4d0000;
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4fb8));
                            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x6aa0));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x6a90));
                            c.swc1(8, (c.g[S1][0] as u32).wrapping_sub(0x6a60));
                            c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x6a80));
                            c.swc1(4, (c.g[S2][0] as u32).wrapping_sub(0x6a50));
                            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6a60));
                            c.swc1(5, (c.g[S3][0] as u32).wrapping_sub(0x6a40));
                            c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6a50));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4fb4));
                            c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6a30));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4fb0));
                            c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6a20));
                            c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[T9][0] = 0x4d0000;
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 6);
                            c.g[S1][0] = 0x4d0000;
                            c.swc1(0, (c.g[S5][0] as u32).wrapping_sub(0x6aa0));
                            c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_sub(0x6a10));
                            c.swc1(3, (c.g[S4][0] as u32).wrapping_sub(0x6a30));
                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x69d0));
                            c.swc1(1, (c.g[T3][0] as u32).wrapping_sub(0x6a20));
                            c.g[S6][0] = 0x4d0000;
                            c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0xc));
                            c.g[S2][0] = 0x4d0000;
                            c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x69c0));
                            c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x4fac));
                            c.g[T7][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x69e0));
                            c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x4fa8));
                            c.g[S3][0] = 0x4d0000;
                            c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x69b0));
                            c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x4fa4));
                            c.g[FP][0] = 0x4d0000;
                            c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x4fa0));
                            c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x69a0));
                            c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[T8][0] = 0x4d0000;
                            c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = 0x4d0000;
                            c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6990));
                            c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x4f9c));
                            c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_sub(0x6a00));
                            c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x4f98));
                            c.g[S7][0] = 0x4d0000;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T6][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x69f0));
                            c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[V1][0] = 0x4d0000;
                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4f94));
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6950));
                            c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6930));
                            c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = 0x4d0000;
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6940));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6920));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6900));
                            c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
                            c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x68d0));
                            c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = 0x4d0000;
                            c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x4f90));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4f8c));
                            c.g[A2][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x68f0));
                            c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
                            c.g[FP][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x68b0));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4f88));
                            c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x68a0));
                            c.swc1(2, (c.g[T9][0] as u32).wrapping_sub(0x6a10));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4f84));
                            c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4f80));
                            c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4f7c));
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4f78));
                            c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
                            c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[T8][0] as u32).wrapping_sub(0x6a00));
                            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[S7][0] as u32).wrapping_sub(0x69f0));
                            c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4f74));
                            c.swc1(0, (c.g[S6][0] as u32).wrapping_sub(0x69e0));
                            c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x69d0));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6840));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4f70));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4f6c));
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x69c0));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[T4][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6820));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4f68));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4f64));
                            c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6810));
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4f60));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4f5c));
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x69b0));
                            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x69a0));
                            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
                            c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
                            c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x67e0));
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[A0][0] = 0x4d0000;
                            c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x67c0));
                            c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[A0][0] = 0x4d0000;
                            c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x67b0));
                            c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A0][0] = 0x4d0000;
                            c.g[S3][0] = 0x4d0000;
                            c.g[T7][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x67a0));
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6960));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0xc));
                            c.g[T6][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6790));
                            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4f58));
                            c.g[T5][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6780));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4f54));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4f50));
                            c.swc1(1, (c.g[A0][0] as u32).wrapping_sub(0x6930));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4f4c));
                            c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6770));
                            c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4f48));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4f44));
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6990));
                            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6980));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6980));
                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4f40));
                            c.swc1(6, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6970));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6970));
                            c.swc1(6, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6960));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6950));
                            c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6720));
                            c.g[V0][0] = 0x4d0000;
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4f3c));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6940));
                            c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = 0x4d0000;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(7, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.swc1(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[V1][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4f38));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6920));
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6910));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6980));
                            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6970));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6960));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6910));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                            c.swc1(1, (c.g[T1][0] as u32).wrapping_add(0xc));
                            c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4f34));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6900));
                            c.swc1(4, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(5, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4f30));
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4f2c));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x68f0));
                            c.swc1(6, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.g[V1][0];
                            c.swc1(7, (c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x68e0));
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4f28));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x68e0));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x68d0));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(9, (c.g[T1][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = c.g[V0][0];
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x68c0));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x68c0));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x68b0));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x68a0));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x68e0));
                            c.swc1(11, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(10, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x68c0));
                            c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A3][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4f24));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6890));
                            c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6890));
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6830));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[FP][0] = 0x4d0000;
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0xc));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4f20));
                            c.g[A3][0] = c.g[V1][0];
                            c.swc1(4, (c.g[FP][0] as u32).wrapping_sub(0x6850));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6880));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4f1c));
                            c.g[FP][0] = 0x4d0000;
                            c.swc1(3, (c.g[T0][0] as u32).wrapping_add(0xc));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4f18));
                            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = c.g[FP][0];
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4f14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6870));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6880));
                            c.swc1(5, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x6870));
                            c.g[A3][0] = c.g[V1][0];
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6860));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6860));
                            c.g[FP][0] = 0x4d0000;
                            c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6850));
                            c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[V0][0] = 0x4d0000;
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4f10));
                            c.g[V1][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4f0c));
                            c.g[A3][0] = c.g[V1][0];
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6840));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6830));
                            c.swc1(6, (c.g[T0][0] as u32).wrapping_add(0x4));
                            c.g[FP][0] = 0x4d0000;
                            c.swc1(7, (c.g[T0][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4f08));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6830));
                            c.swc1(8, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4f04));
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x6820));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6880));
                            c.swc1(9, (c.g[T4][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = 0x4d0000;
                            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4f00));
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6870));
                            c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6810));
                            c.swc1(10, (c.g[T2][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6800));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6860));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6850));
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6830));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[FP][0] = 0x4d0000;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6800));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x67f0));
                            c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4efc));
                            c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4ef8));
                            c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0xc));
                            c.swc1(1, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x67f0));
                            c.g[FP][0] = 0x4d0000;
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x67e0));
                            c.g[V0][0] = c.g[FP][0];
                            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x67d0));
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x67d0));
                            c.g[V1][0] = 0x4d0000;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x67c0));
                            c.g[FP][0] = 0x4d0000;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[V0][0] = 0x4d0000;
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4ef4));
                            c.g[V1][0] = 0x4d0000;
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x67b0));
                            c.swc1(3, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4ef0));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6800));
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x67a0));
                            c.swc1(4, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = 0x4d0000;
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4eec));
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x67f0));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6790));
                            c.swc1(5, (c.g[T6][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = 0x4d0000;
                            c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x67d0));
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4ee8));
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4ee4));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6780));
                            c.swc1(6, (c.g[T5][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0x1;
                            c.swc1(7, (c.g[T5][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.swc1(12, (c.g[FP][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = 0x2;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[FP][0] = 0x4d0000;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6750));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = 0x3;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                            c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[S2][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6760));
                            c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x4));
                            c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f08), c.g[V1][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f00), 0u64);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f10), c.g[A3][0]);
                            c.g[V0][0] = c.g[V1][0] & 0xff;
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4ee0));
                            c.g[V0][0] = c.g[V0][0] | 0x100;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f18), c.g[S1][0]);
                            c.g[V1][0] = c.g[A3][0] & 0xff;
                            c.g[A2][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1f00));
                            c.g[A3][0] = 0x4d0000;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4edc));
                            c.g[A1][0] = c.g[S1][0] & 0xff;
                            c.swc1(2, (c.g[S2][0] as u32).wrapping_add(0xc));
                            c.g[S1][0] = 0x4d0000;
                            c.swc1(1, (c.g[FP][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = c.g[V1][0] | 0x200;
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4ed8));
                            c.g[A1][0] = c.g[A1][0] | 0x300;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4ed4));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6770));
                            c.swc1(3, (c.g[S3][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(4, (c.g[S3][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4ed0));
                            c.g[S3][0] = 0x4d0000;
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4ecc));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_sub(0x6760));
                            c.swc1(5, (c.g[S2][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = c.g[A3][0];
                            c.swc1(6, (c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6730));
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4ec8));
                            c.g[S2][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4ec4));
                            c.swc1(0, (c.g[S2][0] as u32).wrapping_sub(0x6750));
                            c.swc1(7, (c.g[FP][0] as u32).wrapping_add(0x4));
                            c.g[S2][0] = 0x4;
                            c.swc1(8, (c.g[FP][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = c.g[S3][0];
                            c.swc1(0, (c.g[S3][0] as u32).wrapping_sub(0x6740));
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6740));
                            c.g[S3][0] = 0x5;
                            c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6730));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = 0x9;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f00), c.g[A2][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f08), c.g[V0][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f10), c.g[V1][0]);
                            c.g[V0][0] = 0x7;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = 0x8;
                            c.sw((c.g[FP][0] as u32).wrapping_add(0x4), 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[FP][0] = 0x6;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1efc), 0u64);
                            c.g[S1][0] = 0xa;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1ef8), 0u64);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f18), c.g[A1][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f20), c.g[S2][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f28), c.g[S3][0]);
                            c.g[S2][0] = 0xb;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f30), c.g[FP][0]);
                            c.g[S3][0] = 0xc;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f38), c.g[V0][0]);
                            c.g[FP][0] = 0xd;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f40), c.g[V1][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f48), c.g[A3][0]);
                            c.g[T2][0] = c.g[V0][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f50), c.g[S1][0]);
                            c.g[T0][0] = c.g[V1][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f58), c.g[S2][0]);
                            c.g[A3][0] = c.g[A3][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f60), c.g[S3][0]);
                            c.g[A2][0] = c.g[S1][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x1f68), c.g[FP][0]);
                            c.g[A1][0] = c.g[S2][0] & 0xff;
                            c.g[T1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1f20));
                            c.g[V1][0] = c.g[S3][0] & 0xff;
                            c.g[T3][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1f28));
                            c.g[V0][0] = c.g[FP][0] & 0xff;
                            c.g[T4][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1f30));
                            c.g[T1][0] = c.g[T1][0] | 0x400;
                            c.g[T3][0] = c.g[T3][0] | 0x500;
                            c.g[T2][0] = c.g[T2][0] | 0x700;
                            c.g[T4][0] = c.g[T4][0] | 0x600;
                            c.g[T0][0] = c.g[T0][0] | 0x800;
                            c.g[A3][0] = c.g[A3][0] | 0x900;
                            c.g[A2][0] = c.g[A2][0] | 0xa00;
                            c.g[A1][0] = c.g[A1][0] | 0xb00;
                            c.g[V1][0] = c.g[V1][0] | 0xc00;
                            c.g[V0][0] = c.g[V0][0] | 0xd00;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f20), c.g[T1][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f28), c.g[T3][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f30), c.g[T4][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f38), c.g[T2][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f40), c.g[T0][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f48), c.g[A3][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f50), c.g[A2][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f58), c.g[A1][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f60), c.g[V1][0]);
                            c.g[RA][0] = sx(0x0024f1a8);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x1f68), c.g[V0][0]);
                            Callee {
                                addr: 0x00247e60,
                                f: ee::s06::RenderTogglesMenu_ctor,
                            }
                            .call(c, 0x0024f1a8)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1b4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5cd8));
                            Callee {
                                addr: 0x00248970,
                                f: ee::s06::VisualEffectsTestMenu_ctor,
                            }
                            .call(c, 0x0024f1b4)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1c0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5840));
                            Callee {
                                addr: 0x00249148,
                                f: ee::s06::LightGlowMenu_ctor,
                            }
                            .call(c, 0x0024f1c0)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1cc);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5108));
                            Callee {
                                addr: 0x00248dc8,
                                f: ee::s06::PlantsMenu_ctor,
                            }
                            .call(c, 0x0024f1cc)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1d8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4d38));
                            Callee {
                                addr: 0x002498a0,
                                f: ee::s06::DepthFogMenu_ctor,
                            }
                            .call(c, 0x0024f1d8)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1e4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4418));
                            Callee {
                                addr: 0x00249b28,
                                f: ee::s06::HeightFogMenu_ctor,
                            }
                            .call(c, 0x0024f1e4)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1f0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4218));
                            Callee {
                                addr: 0x00249c68,
                                f: ee::s06::DetailSystemMenu_ctor,
                            }
                            .call(c, 0x0024f1f0)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f1fc);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3eb0));
                            Callee {
                                addr: 0x0024a228,
                                f: ee::s06::ScreenTintMenu_ctor,
                            }
                            .call(c, 0x0024f1fc)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f208);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3550));
                            Callee {
                                addr: 0x0024abb8,
                                f: ee::s06::ShadowMenu_ctor,
                            }
                            .call(c, 0x0024f208)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f214);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3220));
                            Callee {
                                addr: 0x0024ae90,
                                f: ee::s06::SnowSurfaceMenu_ctor,
                            }
                            .call(c, 0x0024f214)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f220);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2f58));
                            Callee {
                                addr: 0x0024b0e8,
                                f: ee::s06::BoardTrailMenu_ctor,
                            }
                            .call(c, 0x0024f220)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f22c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2988));
                            Callee {
                                addr: 0x0024b6d0,
                                f: ee::s06::FlagsMenu_ctor,
                            }
                            .call(c, 0x0024f22c)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f238);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27c0));
                            Callee {
                                addr: 0x0024b7d0,
                                f: ee::s06::PowerUpFXTogglesMenu_ctor,
                            }
                            .call(c, 0x0024f238)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f244);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2378));
                            Callee {
                                addr: 0x0024bbe8,
                                f: ee::s06::SnowfallTogglesMenu_ctor,
                            }
                            .call(c, 0x0024f244)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f250);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1358));
                            Callee {
                                addr: 0x0024bf60,
                                f: ee::s06::CameraSplashMenu_ctor,
                            }
                            .call(c, 0x0024f250)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f25c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11f0));
                            Callee {
                                addr: 0x0024cca0,
                                f: ee::s06::TubeMenu_ctor,
                            }
                            .call(c, 0x0024f25c)?;
                            c.g[A0][0] = 0x4d0000;
                            c.g[RA][0] = sx(0x0024f268);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1480));
                            Callee {
                                addr: 0x0024ceb0,
                                f: ee::s06::PathArrowMenu_ctor,
                            }
                            .call(c, 0x0024f268)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                            if t {
                                break 'b_24f694;
                            } else {
                                break 'b_24f270;
                            }
                        }
                    }
                    c.cycles += 264;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f280);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1480));
                    Callee {
                        addr: 0x0024e7c8,
                        f: ee::s06::PathArrowMenu_dtor,
                    }
                    .call(c, 0x0024f280)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x11f0));
                    c.g[RA][0] = sx(0x0024f294);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x25c));
                    call_ptr(c, 0x002ca280, 0x0024f294)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x22c));
                    c.g[RA][0] = sx(0x0024f2a0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2a0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1fc));
                    c.g[RA][0] = sx(0x0024f2ac);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2ac)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
                    c.g[RA][0] = sx(0x0024f2b8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2b8)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    c.g[RA][0] = sx(0x0024f2c4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2c4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f2d0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2d0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f2dc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2dc)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f2e8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f2e8)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f2f4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f2f4)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f304);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1358));
                    Callee {
                        addr: 0x0024e388,
                        f: ee::s06::CameraSplashMenu_dtor,
                    }
                    .call(c, 0x0024f304)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f314);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2378));
                    Callee {
                        addr: 0x0024e200,
                        f: ee::s06::SnowfallTogglesMenu_dtor,
                    }
                    .call(c, 0x0024f314)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f324);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27c0));
                    Callee {
                        addr: 0x0024e0f8,
                        f: ee::s06::PowerUpFXTogglesMenu_dtor,
                    }
                    .call(c, 0x0024f324)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2988));
                    c.g[RA][0] = sx(0x0024f338);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    call_ptr(c, 0x002ca280, 0x0024f338)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f344);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f344)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f350);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f350)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f35c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f35c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f368);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f368)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f378);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2f58));
                    Callee {
                        addr: 0x0024dfa0,
                        f: ee::s06::BoardTrailMenu_dtor,
                    }
                    .call(c, 0x0024f378)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3220));
                    c.g[RA][0] = sx(0x0024f38c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x294));
                    call_ptr(c, 0x002ca280, 0x0024f38c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x260));
                    c.g[RA][0] = sx(0x0024f398);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f398)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x22c));
                    c.g[RA][0] = sx(0x0024f3a4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3a4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f8));
                    c.g[RA][0] = sx(0x0024f3b0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3b0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c4));
                    c.g[RA][0] = sx(0x0024f3bc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3bc)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    c.g[RA][0] = sx(0x0024f3c8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3c8)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f3d4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3d4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f3e0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3e0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f3ec);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f3ec)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f3f8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f3f8)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3550));
                    c.g[RA][0] = sx(0x0024f40c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fc));
                    call_ptr(c, 0x002ca280, 0x0024f40c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c8));
                    c.g[RA][0] = sx(0x0024f418);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f418)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x294));
                    c.g[RA][0] = sx(0x0024f424);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f424)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x260));
                    c.g[RA][0] = sx(0x0024f430);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f430)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x22c));
                    c.g[RA][0] = sx(0x0024f43c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f43c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f8));
                    c.g[RA][0] = sx(0x0024f448);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f448)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c4));
                    c.g[RA][0] = sx(0x0024f454);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f454)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    c.g[RA][0] = sx(0x0024f460);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f460)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f46c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f46c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f478);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f478)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f484);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f484)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f490);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f490)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f4a0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3eb0));
                    Callee {
                        addr: 0x0024dd70,
                        f: ee::s06::ScreenTintMenu_dtor,
                    }
                    .call(c, 0x0024f4a0)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4218));
                    c.g[RA][0] = sx(0x0024f4b4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x334));
                    call_ptr(c, 0x002ca280, 0x0024f4b4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x300));
                    c.g[RA][0] = sx(0x0024f4c0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f4c0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2cc));
                    c.g[RA][0] = sx(0x0024f4cc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f4cc)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x298));
                    c.g[RA][0] = sx(0x0024f4d8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f4d8)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x264));
                    c.g[RA][0] = sx(0x0024f4e4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f4e4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x230));
                    c.g[RA][0] = sx(0x0024f4f0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f4f0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1fc));
                    c.g[RA][0] = sx(0x0024f4fc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f4fc)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
                    c.g[RA][0] = sx(0x0024f508);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f508)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    c.g[RA][0] = sx(0x0024f514);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f514)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f520);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f520)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f52c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f52c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f538);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f538)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f544);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f544)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4418));
                    c.g[RA][0] = sx(0x0024f558);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
                    call_ptr(c, 0x002ca280, 0x0024f558)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    c.g[RA][0] = sx(0x0024f564);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f564)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f570);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f570)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f57c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f57c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f588);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f588)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f594);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f594)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f5a4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4d38));
                    Callee {
                        addr: 0x0024dc68,
                        f: ee::s06::DepthFogMenu_dtor,
                    }
                    .call(c, 0x0024f5a4)?;
                    c.g[S0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5108));
                    c.g[RA][0] = sx(0x0024f5b8);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x398));
                    call_ptr(c, 0x002ca280, 0x0024f5b8)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x364));
                    c.g[RA][0] = sx(0x0024f5c4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f5c4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x330));
                    c.g[RA][0] = sx(0x0024f5d0);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f5d0)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fc));
                    c.g[RA][0] = sx(0x0024f5dc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f5dc)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c8));
                    c.g[RA][0] = sx(0x0024f5e8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f5e8)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x298));
                    c.g[RA][0] = sx(0x0024f5f4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f5f4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x264));
                    c.g[RA][0] = sx(0x0024f600);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f600)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x230));
                    c.g[RA][0] = sx(0x0024f60c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f60c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1fc));
                    c.g[RA][0] = sx(0x0024f618);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f618)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
                    c.g[RA][0] = sx(0x0024f624);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f624)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
                    c.g[RA][0] = sx(0x0024f630);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f630)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
                    c.g[RA][0] = sx(0x0024f63c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f63c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
                    c.g[RA][0] = sx(0x0024f648);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f648)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
                    c.g[RA][0] = sx(0x0024f654);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002ca280, 0x0024f654)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0024f660);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002caa80, 0x0024f660)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f670);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5840));
                    Callee {
                        addr: 0x0024dab8,
                        f: ee::s06::LightGlowMenu_dtor,
                    }
                    .call(c, 0x0024f670)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f680);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5cd8));
                    Callee {
                        addr: 0x0024d998,
                        f: ee::s06::VisualEffectsTestMenu_dtor,
                    }
                    .call(c, 0x0024f680)?;
                    c.g[A0][0] = 0x4d0000;
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x0024f690);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6720));
                    Callee {
                        addr: 0x0024d710,
                        f: ee::s06::RenderTogglesMenu_dtor,
                    }
                    .call(c, 0x0024f690)?;
                    break 'b_24f690;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            break 'b_24f694;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::{fresh, run_generated, DATA};

    /// `init` (a0) zero: the dtor chain; non-zero: the ctor chain; `prio`
    /// (a1) 0xFFFF runs the body at all.
    fn setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0xe8d8, 0);
        c.g[A0][0] = (seed % 2) as u64;
        c.g[A1][0] = if seed % 5 == 0 { 0 } else { 0xFFFF };
        c.g[A2][0] = DATA as u64;
        c.pc = 0x0024_e8d8;
    }

    #[test]
    fn visual_effects_menus_static_init_destroy_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..200 {
            let mut c = fresh();
            setup(&mut c, seed);
            let b = c.cycles;
            run_generated(&mut c, ee::s06::f_0024e8d8__generated);
            seen.insert(c.cycles - b);
        }
        assert!(seen.len() >= 3, "paths: {seen:?}");
    }

    #[test]
    fn visual_effects_menus_static_init_destroy_matches() {
        check(
            ee::s06::f_0024e8d8__generated,
            visual_effects_menus_static_init_destroy,
            200,
            &setup,
        );
    }
}
