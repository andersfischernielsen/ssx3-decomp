//! Staging module oc479 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc479`.

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
    0x00364cd0,
    defer_man_emit_material_setup,
    "DeferMan_EmitMaterialSetup",
)];

/// `DeferMan_EmitMaterialSetup` (0x00364cd0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn defer_man_emit_material_setup(c: &mut Cpu) {
    if c.pc != 0x00364cd0 {
        return ee::s12::f_00364cd0__generated(c);
    }
    guest(c, |c| {
        'b_365e28: {
            'b_365e24: {
                'b_365e20: {
                    c.cycles += 20;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
                    c.g[V0][0] = 0x60000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[V0][0] = c.g[V0][0] | 0x8000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[T2][0] = 0x180;
                    c.g[T9][0] = c.g[T0][0];
                    c.g[T3][0] = c.g[A3][0];
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1cc0));
                    c.g[T0][0] = 0x60000;
                    c.g[T1][0] = c.lw(c.g[T9][0] as u32);
                    c.g[T0][0] = c.g[T0][0] | 0x9cd0;
                    c.mult(0, 3, 3, 10);
                    c.g[V0][0] = (c.g[A1][0] < 0x9u64) as u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[T1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    let t = c.g[V0][0] == 0u64;
                    c.g[T8][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if t {
                        break 'b_365e20;
                    } else {
                        'b_365e1c: {
                            'b_365e0c: {
                                'b_365df4: {
                                    'b_365db0: {
                                        'b_365da0: {
                                            'b_365d7c: {
                                                'b_3658d4: {
                                                    'b_3658cc: {
                                                        'b_365624: {
                                                            'b_365314: {
                                                                'b_365190: {
                                                                    'b_364e38: {
                                                                        c.cycles += 7;
                                                                        c.g[V0][0] = 0x490000;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[A1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x2040,
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[V1][0] as u32);
                                                                        let t = c.g[A0][0] as u32;
                                                                        match t {
                                                                            0x00364d3c => {
                                                                                'b_364db8: {
                                                                                    c.cycles += 3;
                                                                                    c.g[V0][0] =
                                                                                        0x1;
                                                                                    let t = c.g[A2]
                                                                                        [0]
                                                                                        == c.g[V0]
                                                                                            [0];
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    if t {
                                                                                        break 'b_364db8;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            3;
                                                                                        c.g[V0][0] = ((c.g[A2][0] as i64) < 2i64) as u64;
                                                                                        if c.g[V0]
                                                                                            [0]
                                                                                            == 0u64
                                                                                        {
                                                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                            break 'b_365e24;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            if c.g[A2][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
break 'b_365e24;
} else {
c.cycles += 23;
c.g[V0][0] = 0x430000;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1a70));
c.g[V1][0] = 0x50000000;
c.g[V0][0] = c.g[V0][0] << 32;
c.g[A1][0] = 0x450000;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4f80));
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A1][0] = c.g[A1][0] << 32;
c.sd(c.g[A0][0] as u32, c.g[V0][0]);
c.g[V1][0] = 0x30000000;
c.g[V1][0] = c.g[V1][0] | 0xc;
c.g[A1][0] = c.g[A1][0] | c.g[V1][0];
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
c.sd((c.g[A0][0] as u32).wrapping_add(0x18), 0u64);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
break 'b_364db8;
}
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 32;
                                                                                c.g[V1][0] =
                                                                                    0x10000000;
                                                                                c.g[V1][0] = c.g
                                                                                    [V1][0]
                                                                                    | 0x7;
                                                                                c.g[A1][0] = 0xd80c;
                                                                                c.g[A1][0] = c.g
                                                                                    [A1][0]
                                                                                    << 47;
                                                                                c.sd(
                                                                                    c.g[A0][0]
                                                                                        as u32,
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
                                                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0x80));
                                                                                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x20));
                                                                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0x90));
                                                                                c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xa0));
                                                                                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x40));
                                                                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xb0));
                                                                                c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x50));
                                                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xc0));
                                                                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x60));
                                                                                c.lq(3, (c.g[T8][0] as u32).wrapping_add(0xd0));
                                                                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x70));
                                                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                let t = true;
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x80));
                                                                                if t {
                                                                                    break 'b_365e1c;
                                                                                } else {
                                                                                    break 'b_364e38;
                                                                                }
                                                                            }
                                                                            0x00364e38 => {
                                                                                break 'b_364e38;
                                                                            }
                                                                            0x00365190 => {
                                                                                break 'b_365190;
                                                                            }
                                                                            0x00365314 => {
                                                                                break 'b_365314;
                                                                            }
                                                                            0x00365624 => {
                                                                                break 'b_365624;
                                                                            }
                                                                            0x003658d4 => {
                                                                                break 'b_3658d4;
                                                                            }
                                                                            0x00365d7c => {
                                                                                break 'b_365d7c;
                                                                            }
                                                                            0x00365e20 => {
                                                                                break 'b_365e20;
                                                                            }
                                                                            _ => {
                                                                                c.jump_out(
                                                                                    t, 0x00364cd0,
                                                                                    0x00365e3c,
                                                                                );
                                                                                return Ok(());
                                                                            }
                                                                        }
                                                                    }
                                                                    'b_365110: {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = 0x1;
                                                                        let t = c.g[A2][0]
                                                                            == c.g[V0][0];
                                                                        c.g[A0][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xf0));
                                                                        if t {
                                                                            break 'b_365110;
                                                                        } else {
                                                                            c.cycles += 3;
                                                                            c.g[V0][0] = ((c.g[A2]
                                                                                [0]
                                                                                as i64)
                                                                                < 2i64)
                                                                                as u64;
                                                                            if c.g[V0][0] == 0u64 {
                                                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                break 'b_365e24;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[A2][0]
                                                                                    != 0u64
                                                                                {
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    break 'b_365e24;
                                                                                } else {
                                                                                    c.cycles += 174;
                                                                                    c.g[V0][0] =
                                                                                        0x430000;
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xa80));
                                                                                    c.g[V1][0] =
                                                                                        0x50000000;
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        << 32;
                                                                                    c.g[A2][0] =
                                                                                        0x10000000;
                                                                                    c.g[A2][0] = c
                                                                                        .g[A2][0]
                                                                                        | 0xd;
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        | c.g[V1]
                                                                                            [0];
                                                                                    c.g[A3][0] =
                                                                                        0xd816;
                                                                                    c.g[A3][0] = c
                                                                                        .g[A3][0]
                                                                                        << 47;
                                                                                    c.sd(
                                                                                        c.g[A0][0]
                                                                                            as u32,
                                                                                        c.g[V0][0],
                                                                                    );
                                                                                    c.g[T0][0] =
                                                                                        0x80;
                                                                                    c.g[T0][0] = c
                                                                                        .g[T0][0]
                                                                                        << 32;
                                                                                    c.g[T0][0] = c
                                                                                        .g[T0][0]
                                                                                        | 0x80;
                                                                                    c.g[T2][0] =
                                                                                        0x302e0000;
                                                                                    c.g[T2][0] = c
                                                                                        .g[T2][0]
                                                                                        | 0x4000;
                                                                                    c.g[T2][0] = c
                                                                                        .g[T2][0]
                                                                                        << 32;
                                                                                    c.g[T2][0] = c
                                                                                        .g[T2][0]
                                                                                        | 0x8000;
                                                                                    c.g[A1][0] =
                                                                                        0x412;
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.g[T3][0] =
                                                                                        0x312e0000;
                                                                                    c.g[T3][0] = c
                                                                                        .g[T3][0]
                                                                                        | 0x4000;
                                                                                    c.g[T3][0] = c
                                                                                        .g[T3][0]
                                                                                        << 32;
                                                                                    c.g[T3][0] = c
                                                                                        .g[T3][0]
                                                                                        | 0x8000;
                                                                                    c.g[T4][0] =
                                                                                        0x302e0000;
                                                                                    c.g[T4][0] = c
                                                                                        .g[T4][0]
                                                                                        | 0xc000;
                                                                                    c.g[T4][0] = c
                                                                                        .g[T4][0]
                                                                                        << 32;
                                                                                    c.g[T4][0] = c
                                                                                        .g[T4][0]
                                                                                        | 0x8000;
                                                                                    c.g[T5][0] =
                                                                                        0x312e0000;
                                                                                    c.g[T5][0] = c
                                                                                        .g[T5][0]
                                                                                        | 0xc000;
                                                                                    c.g[T5][0] = c
                                                                                        .g[T5][0]
                                                                                        << 32;
                                                                                    c.g[T5][0] = c
                                                                                        .g[T5][0]
                                                                                        | 0x8000;
                                                                                    c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                                                                                    c.g[T6][0] =
                                                                                        0x6c2a0000;
                                                                                    c.g[T6][0] = c
                                                                                        .g[T6][0]
                                                                                        | 0x200;
                                                                                    c.g[T6][0] = c
                                                                                        .g[T6][0]
                                                                                        << 32;
                                                                                    c.g[T1][0] =
                                                                                        0x30000000;
                                                                                    c.g[T1][0] = c
                                                                                        .g[T1][0]
                                                                                        | 0x12;
                                                                                    c.g[T7][0] =
                                                                                        0x10000000;
                                                                                    c.g[T7][0] = c
                                                                                        .g[T7][0]
                                                                                        | 0x6;
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.g[S0][0] =
                                                                                        0x212;
                                                                                    c.g[S0][0] = c
                                                                                        .g[S0][0]
                                                                                        << 32;
                                                                                    c.g[S0][0] = c
                                                                                        .g[S0][0]
                                                                                        | 0x200;
                                                                                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x28a8));
                                                                                    c.g[S1][0] =
                                                                                        0x216;
                                                                                    c.g[S1][0] = c
                                                                                        .g[S1][0]
                                                                                        << 32;
                                                                                    c.g[S1][0] = c
                                                                                        .g[S1][0]
                                                                                        | 0x204;
                                                                                    c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                                                                                    c.g[S2][0] =
                                                                                        0x21c;
                                                                                    c.g[S2][0] = c
                                                                                        .g[S2][0]
                                                                                        << 32;
                                                                                    c.g[S2][0] = c
                                                                                        .g[S2][0]
                                                                                        | 0x20a;
                                                                                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x28a4));
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x28a0));
                                                                                    c.sd((c.g[V1][0] as u32).wrapping_add(0x18), 0u64);
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V0][0] as u32).wrapping_add(0x20), 0u64);
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V1][0] as u32).wrapping_add(0x28), c.g[A3][0]);
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.lq(2, (c.g[T8][0] as u32).wrapping_add(0x80));
                                                                                    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.lq(2, (c.g[T8][0] as u32).wrapping_add(0x90));
                                                                                    c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x40));
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xa0));
                                                                                    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x50));
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xb0));
                                                                                    c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x60));
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xc0));
                                                                                    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x70));
                                                                                    c.lq(3, (c.g[T8][0] as u32).wrapping_add(0xd0));
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sq(3, (c.g[V0][0] as u32).wrapping_add(0x80));
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[A0][0] as u32).wrapping_add(0x90), c.g[T0][0]);
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V0][0] as u32).wrapping_add(0x98), c.g[T0][0]);
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V1][0] as u32).wrapping_add(0xa0), c.g[T2][0]);
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V0][0] as u32).wrapping_add(0xa8), c.g[A1][0]);
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V1][0] as u32).wrapping_add(0xb0), c.g[T3][0]);
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V0][0] as u32).wrapping_add(0xb8), c.g[A1][0]);
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V1][0] as u32).wrapping_add(0xc0), c.g[T4][0]);
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V0][0] as u32).wrapping_add(0xc8), c.g[A1][0]);
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V1][0] as u32).wrapping_add(0xd0), c.g[T5][0]);
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V0][0] as u32).wrapping_add(0xd8), c.g[A1][0]);
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V1][0] as u32).wrapping_add(0xe0), 0u64);
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V0][0] as u32).wrapping_add(0xe8), c.g[T6][0]);
                                                                                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1158));
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1a0));
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        << 32;
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        | c.g[T1]
                                                                                            [0];
                                                                                    c.sd((c.g[A0][0] as u32).wrapping_add(0xf0), c.g[V0][0]);
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V1][0] as u32).wrapping_add(0xf8), 0u64);
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x320));
                                                                                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x100));
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        << 32;
                                                                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[V1][0]);
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        | c.g[T1]
                                                                                            [0];
                                                                                    c.sd((c.g[A0][0] as u32).wrapping_add(0x100), c.g[V0][0]);
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                                                                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[V0][0]);
                                                                                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T7][0]);
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                                                                                    c.swc1(
                                                                                        2,
                                                                                        c.g[SP][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                                                                                    c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V1][0] as u32).wrapping_add(0x18), 0u64);
                                                                                    c.g[V0][0] = c
                                                                                        .lw(
                                                                                        c.g[SP][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.g[V1][0] = c
                                                                                        .g[V0][0]
                                                                                        << 32;
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        | c.g[V1]
                                                                                            [0];
                                                                                    c.sd((c.g[A0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                                                                    c.g[V1][0] = c
                                                                                        .lw(
                                                                                        c.g[SP][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.g[V0][0] = c
                                                                                        .g[V1][0]
                                                                                        << 32;
                                                                                    c.g[V1][0] = c
                                                                                        .g[V1][0]
                                                                                        | c.g[V0]
                                                                                            [0];
                                                                                    c.sd((c.g[A0][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V0][0] as u32).wrapping_add(0x30), c.g[S1][0]);
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V1][0] as u32).wrapping_add(0x38), 0u64);
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.g[V1][0] = c
                                                                                        .g[V0][0]
                                                                                        << 32;
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        | c.g[V1]
                                                                                            [0];
                                                                                    c.sd((c.g[A0][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.g[V0][0] = c
                                                                                        .g[V1][0]
                                                                                        << 32;
                                                                                    c.g[V1][0] = c
                                                                                        .g[V1][0]
                                                                                        | c.g[V0]
                                                                                            [0];
                                                                                    c.sd((c.g[A0][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V0][0] as u32).wrapping_add(0x50), c.g[S2][0]);
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.sd((c.g[V1][0] as u32).wrapping_add(0x58), 0u64);
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.g[V1][0] = c
                                                                                        .g[V0][0]
                                                                                        << 32;
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        | c.g[V1]
                                                                                            [0];
                                                                                    c.sd((c.g[A0][0] as u32).wrapping_add(0x60), c.g[V0][0]);
                                                                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    c.g[V0][0] = c
                                                                                        .g[V1][0]
                                                                                        << 32;
                                                                                    c.g[V1][0] = c
                                                                                        .g[V1][0]
                                                                                        | c.g[V0]
                                                                                            [0];
                                                                                    c.sd((c.g[A0][0] as u32).wrapping_add(0x68), c.g[V1][0]);
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                    let t = true;
                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70));
                                                                                    if t {
                                                                                        break 'b_365e1c;
                                                                                    } else {
                                                                                        break 'b_365110;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 32;
                                                                    c.g[V1][0] = 0x10000000;
                                                                    c.g[V1][0] = c.g[V1][0] | 0x7;
                                                                    c.g[A1][0] = 0xd80c;
                                                                    c.g[A1][0] = c.g[A1][0] << 47;
                                                                    c.sd(
                                                                        c.g[A0][0] as u32,
                                                                        c.g[V1][0],
                                                                    );
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    c.sd(
                                                                        (c.g[V0][0] as u32)
                                                                            .wrapping_add(0x8),
                                                                        0u64,
                                                                    );
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    c.sd(
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x10),
                                                                        0u64,
                                                                    );
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    c.sd(
                                                                        (c.g[V0][0] as u32)
                                                                            .wrapping_add(0x18),
                                                                        c.g[A1][0],
                                                                    );
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[T8][0] as u32)
                                                                            .wrapping_add(0x80),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x20),
                                                                    );
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[T8][0] as u32)
                                                                            .wrapping_add(0x90),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x30),
                                                                    );
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[T8][0] as u32)
                                                                            .wrapping_add(0xa0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x40),
                                                                    );
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[T8][0] as u32)
                                                                            .wrapping_add(0xb0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x50),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[T8][0] as u32)
                                                                            .wrapping_add(0xc0),
                                                                    );
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[A1][0] as u32)
                                                                            .wrapping_add(0x60),
                                                                    );
                                                                    c.lq(
                                                                        3,
                                                                        (c.g[T8][0] as u32)
                                                                            .wrapping_add(0xd0),
                                                                    );
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    c.sq(
                                                                        3,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x70),
                                                                    );
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    let t = true;
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x80));
                                                                    if t {
                                                                        break 'b_365e1c;
                                                                    } else {
                                                                        break 'b_365190;
                                                                    }
                                                                }
                                                                'b_365294: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = 0x1;
                                                                    let t =
                                                                        c.g[A2][0] == c.g[V0][0];
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    if t {
                                                                        break 'b_365294;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = ((c.g[A2][0]
                                                                            as i64)
                                                                            < 2i64)
                                                                            as u64;
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[V0][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xf0));
                                                                        if t {
                                                                            break 'b_365e24;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t =
                                                                                c.g[A2][0] != 0u64;
                                                                            c.lq(
                                                                                16,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x120,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_365e28;
                                                                            } else {
                                                                                c.cycles += 57;
                                                                                c.g[V0][0] =
                                                                                    0x430000;
                                                                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bd0));
                                                                                c.g[V1][0] =
                                                                                    0x50000000;
                                                                                c.g[V0][0] = c.g
                                                                                    [V0][0]
                                                                                    << 32;
                                                                                c.g[A1][0] =
                                                                                    0x450000;
                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4ec0));
                                                                                c.g[V0][0] = c.g
                                                                                    [V0][0]
                                                                                    | c.g[V1][0];
                                                                                c.g[A1][0] = c.g
                                                                                    [A1][0]
                                                                                    << 32;
                                                                                c.sd(
                                                                                    c.g[A0][0]
                                                                                        as u32,
                                                                                    c.g[V0][0],
                                                                                );
                                                                                c.g[A2][0] =
                                                                                    0x10000000;
                                                                                c.g[A2][0] = c.g
                                                                                    [A2][0]
                                                                                    | 0x8;
                                                                                c.g[V0][0] =
                                                                                    0x30000000;
                                                                                c.g[V0][0] = c.g
                                                                                    [V0][0]
                                                                                    | 0x7;
                                                                                c.g[A3][0] = 0xd80e;
                                                                                c.g[A3][0] = c.g
                                                                                    [A3][0]
                                                                                    << 47;
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.g[A1][0] = c.g
                                                                                    [A1][0]
                                                                                    | c.g[V0][0];
                                                                                c.g[T0][0] = 0x2c2;
                                                                                c.g[T0][0] = c.g
                                                                                    [T0][0]
                                                                                    << 32;
                                                                                c.g[T0][0] = c.g
                                                                                    [T0][0]
                                                                                    | 0x10c;
                                                                                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                                                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), 0u64);
                                                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sd((c.g[V0][0] as u32).wrapping_add(0x20), 0u64);
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sd((c.g[V1][0] as u32).wrapping_add(0x28), c.g[A3][0]);
                                                                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0x80));
                                                                                c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0x90));
                                                                                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x40));
                                                                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xa0));
                                                                                c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x50));
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xb0));
                                                                                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x60));
                                                                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xc0));
                                                                                c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x70));
                                                                                c.lq(3, (c.g[T8][0] as u32).wrapping_add(0xd0));
                                                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sq(3, (c.g[V0][0] as u32).wrapping_add(0x80));
                                                                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sd((c.g[A0][0] as u32).wrapping_add(0x90), c.g[T0][0]);
                                                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sd((c.g[V0][0] as u32).wrapping_add(0x98), 0u64);
                                                                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa0));
                                                                                c.sw((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[V0][0]);
                                                                                let t = true;
                                                                                c.sd((c.g[A0][0] as u32).wrapping_add(0xa0), c.g[A1][0]);
                                                                                if t {
                                                                                    break 'b_365e0c;
                                                                                } else {
                                                                                    break 'b_365294;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 32;
                                                                c.g[V1][0] = 0x10000000;
                                                                c.g[V1][0] = c.g[V1][0] | 0x7;
                                                                c.g[A1][0] = 0xd80c;
                                                                c.g[A1][0] = c.g[A1][0] << 47;
                                                                c.sd(c.g[A0][0] as u32, c.g[V1][0]);
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                    0u64,
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    0u64,
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                    c.g[A1][0],
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.lq(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0x80),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.lq(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0x90),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.lq(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0xa0),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x40),
                                                                );
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.lq(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0xb0),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x50),
                                                                );
                                                                c.lq(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0xc0),
                                                                );
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sq(
                                                                    2,
                                                                    (c.g[A1][0] as u32)
                                                                        .wrapping_add(0x60),
                                                                );
                                                                c.lq(
                                                                    3,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0xd0),
                                                                );
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sq(
                                                                    3,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x70),
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                let t = true;
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x80));
                                                                if t {
                                                                    break 'b_365e1c;
                                                                } else {
                                                                    break 'b_365314;
                                                                }
                                                            }
                                                            'b_3654bc: {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = 0x1;
                                                                let t = c.g[A2][0] == c.g[V0][0];
                                                                c.g[V0][0] = ((c.g[A2][0] as i64)
                                                                    < 2i64)
                                                                    as u64;
                                                                if t {
                                                                    break 'b_3654bc;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    if t {
                                                                        break 'b_365e24;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = c.g[A2][0] != 0u64;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x120,
                                                                                ),
                                                                        );
                                                                        if t {
                                                                            break 'b_365e28;
                                                                        } else {
                                                                            'b_3653cc: {
                                                                                c.cycles += 32;
                                                                                c.g[V0][0] =
                                                                                    0x440000;
                                                                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x65c0));
                                                                                c.g[V1][0] =
                                                                                    0x50000000;
                                                                                c.g[V0][0] = c.g
                                                                                    [V0][0]
                                                                                    << 32;
                                                                                c.g[A1][0] =
                                                                                    0x10000000;
                                                                                c.g[A1][0] = c.g
                                                                                    [A1][0]
                                                                                    | 0x8;
                                                                                c.g[V0][0] = c.g
                                                                                    [V0][0]
                                                                                    | c.g[V1][0];
                                                                                c.g[A2][0] = 0xd80e;
                                                                                c.g[A2][0] = c.g
                                                                                    [A2][0]
                                                                                    << 47;
                                                                                c.sd(
                                                                                    c.g[A0][0]
                                                                                        as u32,
                                                                                    c.g[V0][0],
                                                                                );
                                                                                c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.g[A3][0] = sx((c.g[T8][0] as u32).wrapping_add(0x80));
                                                                                c.g[T3][0] =
                                                                                    c.g[T2][0];
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                                                                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                                                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), 0u64);
                                                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sd((c.g[V0][0] as u32).wrapping_add(0x20), 0u64);
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sd((c.g[V1][0] as u32).wrapping_add(0x28), c.g[A2][0]);
                                                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70));
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
                                                                                c.g[V1][0] =
                                                                                    c.g[V0][0];
                                                                                let t = c.g[V1][0]
                                                                                    == c.g[A0][0];
                                                                                c.sw((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[V0][0]);
                                                                                if t {
                                                                                    break 'b_3653cc;
                                                                                } else {
                                                                                    'l_3653b0: loop {
                                                                                        c.cycles +=
                                                                                            7;
                                                                                        c.lq(2, c.g[A3][0] as u32);
                                                                                        c.sq(2, c.g[V1][0] as u32);
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                                        let t = c.g[V1][0] != c.g[A0][0];
                                                                                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
                                                                                        if t {
                                                                                            if event_due(c) { deliver_event_here(c, 0x003653b0); }
                                                                                            continue 'l_3653b0;
                                                                                        } else {
                                                                                            break 'b_3653cc;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            'b_36541c: {
                                                                                c.cycles += 12;
                                                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xc0));
                                                                                c.g[A1][0] =
                                                                                    c.g[SP][0];
                                                                                c.sw(
                                                                                    c.g[T2][0]
                                                                                        as u32,
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.sq(
                                                                                    2,
                                                                                    c.g[SP][0]
                                                                                        as u32,
                                                                                );
                                                                                c.g[AT][0] =
                                                                                    0x3d800000;
                                                                                c.mtc1(1, 1);
                                                                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.mul_s(0, 0, 1);
                                                                                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                                let t = c.g[V1][0]
                                                                                    == c.g[A0][0];
                                                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                if t {
                                                                                    break 'b_36541c;
                                                                                } else {
                                                                                    c.cycles += 1;
                                                                                    'l_365400: loop {
                                                                                        c.cycles +=
                                                                                            7;
                                                                                        c.lq(2, c.g[A1][0] as u32);
                                                                                        c.sq(2, c.g[V1][0] as u32);
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                                        let t = c.g[V1][0] != c.g[A0][0];
                                                                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                                                                                        if t {
                                                                                            if event_due(c) { deliver_event_here(c, 0x00365400); }
                                                                                            continue 'l_365400;
                                                                                        } else {
                                                                                            break 'b_36541c;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            'b_36546c: {
                                                                                c.cycles += 12;
                                                                                c.sw(
                                                                                    c.g[T3][0]
                                                                                        as u32,
                                                                                    c.g[V1][0],
                                                                                );
                                                                                c.g[A0][0] =
                                                                                    c.g[T0][0];
                                                                                c.g[AT][0] =
                                                                                    0x3d800000;
                                                                                c.mtc1(1, 1);
                                                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xd0));
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                                                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                                                                                c.mul_s(0, 0, 1);
                                                                                let t = c.g[V1][0]
                                                                                    == c.g[A1][0];
                                                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                                                                                if t {
                                                                                    break 'b_36546c;
                                                                                } else {
                                                                                    c.cycles += 1;
                                                                                    'l_365450: loop {
                                                                                        c.cycles +=
                                                                                            7;
                                                                                        c.lq(2, c.g[A0][0] as u32);
                                                                                        c.sq(2, c.g[V1][0] as u32);
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                                        let t = c.g[V1][0] != c.g[A1][0];
                                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                                                                                        if t {
                                                                                            if event_due(c) { deliver_event_here(c, 0x00365450); }
                                                                                            continue 'l_365450;
                                                                                        } else {
                                                                                            break 'b_36546c;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 11;
                                                                            c.sw(
                                                                                c.g[T3][0] as u32,
                                                                                c.g[V1][0],
                                                                            );
                                                                            c.g[A0][0] = c.g[T1][0];
                                                                            c.sw(
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x28,
                                                                                    ),
                                                                                0u64,
                                                                            );
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xf0,
                                                                                ));
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[T8][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x54,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[T8][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x40,
                                                                                    ),
                                                                            );
                                                                            c.g[A1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x10,
                                                                                    ));
                                                                            c.swc1(
                                                                                1,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x24,
                                                                                    ),
                                                                            );
                                                                            c.swc1(
                                                                                0,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x20,
                                                                                    ),
                                                                            );
                                                                            let t = c.g[V1][0]
                                                                                == c.g[A1][0];
                                                                            c.sw(
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x2c,
                                                                                    ),
                                                                                0u64,
                                                                            );
                                                                            if t {
                                                                                break 'b_3658cc;
                                                                            } else {
                                                                                'l_365498: loop {
                                                                                    c.cycles += 7;
                                                                                    c.lq(
                                                                                        2,
                                                                                        c.g[A0][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.sq(
                                                                                        2,
                                                                                        c.g[V1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                                    let t = c.g[V1]
                                                                                        [0]
                                                                                        != c.g[A1]
                                                                                            [0];
                                                                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                                                                                    if t {
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x00365498);
                                                                                        }
                                                                                        continue 'l_365498;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t =
                                                                                            true;
                                                                                        c.sw(c.g[T3][0] as u32, c.g[V1][0]);
                                                                                        if t {
                                                                                            break 'b_365e20;
                                                                                        } else {
                                                                                            break 'b_3654bc;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_365534: {
                                                                c.cycles += 23;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.g[V0][0] = 0x10000000;
                                                                c.g[V0][0] = c.g[V0][0] | 0x8;
                                                                c.g[A0][0] = 0xd80e;
                                                                c.g[A0][0] = c.g[A0][0] << 47;
                                                                c.g[A2][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0xf0));
                                                                c.sd(c.g[V1][0] as u32, c.g[V0][0]);
                                                                c.g[A1][0] = sx((c.g[T8][0]
                                                                    as u32)
                                                                    .wrapping_add(0x120));
                                                                c.g[T3][0] = c.g[A2][0];
                                                                c.g[T0][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.g[T1][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                c.sd(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                    0u64,
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    0u64,
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                    c.g[A0][0],
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.g[A0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x60));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x20));
                                                                c.g[V1][0] = c.g[V0][0];
                                                                let t = c.g[V1][0] == c.g[A0][0];
                                                                c.sw(
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0),
                                                                    c.g[V0][0],
                                                                );
                                                                if t {
                                                                    break 'b_365534;
                                                                } else {
                                                                    'l_365518: loop {
                                                                        c.cycles += 7;
                                                                        c.lq(2, c.g[A1][0] as u32);
                                                                        c.sq(2, c.g[V1][0] as u32);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x10,
                                                                                ));
                                                                        let t = c.g[V1][0]
                                                                            != c.g[A0][0];
                                                                        c.g[A1][0] =
                                                                            sx((c.g[A1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x10,
                                                                                ));
                                                                        if t {
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x00365518,
                                                                                );
                                                                            }
                                                                            continue 'l_365518;
                                                                        } else {
                                                                            break 'b_365534;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_365584: {
                                                                c.cycles += 12;
                                                                c.lq(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0x160),
                                                                );
                                                                c.g[A1][0] = c.g[SP][0];
                                                                c.sw(c.g[A2][0] as u32, c.g[V1][0]);
                                                                c.sq(2, c.g[SP][0] as u32);
                                                                c.g[AT][0] = 0x3d800000;
                                                                c.mtc1(1, 1);
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.mul_s(0, 0, 1);
                                                                c.g[A0][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[V1][0] == c.g[A0][0];
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                );
                                                                if t {
                                                                    break 'b_365584;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    'l_365568: loop {
                                                                        c.cycles += 7;
                                                                        c.lq(2, c.g[A1][0] as u32);
                                                                        c.sq(2, c.g[V1][0] as u32);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x10,
                                                                                ));
                                                                        let t = c.g[V1][0]
                                                                            != c.g[A0][0];
                                                                        c.g[A1][0] =
                                                                            sx((c.g[A1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x10,
                                                                                ));
                                                                        if t {
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x00365568,
                                                                                );
                                                                            }
                                                                            continue 'l_365568;
                                                                        } else {
                                                                            break 'b_365584;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            'b_3655d4: {
                                                                c.cycles += 12;
                                                                c.sw(c.g[T3][0] as u32, c.g[V1][0]);
                                                                c.g[A0][0] = c.g[T0][0];
                                                                c.g[AT][0] = 0x3d800000;
                                                                c.mtc1(1, 1);
                                                                c.lq(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0x170),
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.g[A1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                );
                                                                c.mul_s(0, 0, 1);
                                                                let t = c.g[V1][0] == c.g[A1][0];
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                );
                                                                if t {
                                                                    break 'b_3655d4;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    'l_3655b8: loop {
                                                                        c.cycles += 7;
                                                                        c.lq(2, c.g[A0][0] as u32);
                                                                        c.sq(2, c.g[V1][0] as u32);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x10,
                                                                                ));
                                                                        let t = c.g[V1][0]
                                                                            != c.g[A1][0];
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x10,
                                                                                ));
                                                                        if t {
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x003655b8,
                                                                                );
                                                                            }
                                                                            continue 'l_3655b8;
                                                                        } else {
                                                                            break 'b_3655d4;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 11;
                                                            c.sw(c.g[T3][0] as u32, c.g[V1][0]);
                                                            c.g[A0][0] = c.g[T1][0];
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x28),
                                                                0u64,
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                            c.lwc1(
                                                                1,
                                                                (c.g[T8][0] as u32)
                                                                    .wrapping_add(0xf4),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[T8][0] as u32)
                                                                    .wrapping_add(0xe0),
                                                            );
                                                            c.g[A1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.swc1(
                                                                1,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x24),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            let t = c.g[V1][0] == c.g[A1][0];
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x2c),
                                                                0u64,
                                                            );
                                                            if t {
                                                                break 'b_3658cc;
                                                            } else {
                                                                'l_365600: loop {
                                                                    c.cycles += 7;
                                                                    c.lq(2, c.g[A0][0] as u32);
                                                                    c.sq(2, c.g[V1][0] as u32);
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x10));
                                                                    let t =
                                                                        c.g[V1][0] != c.g[A1][0];
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x10));
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x00365600,
                                                                            );
                                                                        }
                                                                        continue 'l_365600;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = true;
                                                                        c.sw(
                                                                            c.g[T3][0] as u32,
                                                                            c.g[V1][0],
                                                                        );
                                                                        if t {
                                                                            break 'b_365e20;
                                                                        } else {
                                                                            break 'b_365624;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        'b_36579c: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = 0x1;
                                                            let t = c.g[A2][0] == c.g[V0][0];
                                                            c.g[V0][0] =
                                                                ((c.g[A2][0] as i64) < 2i64) as u64;
                                                            if t {
                                                                break 'b_36579c;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                if t {
                                                                    break 'b_365e24;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = c.g[A2][0] != 0u64;
                                                                    c.lq(
                                                                        16,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x120),
                                                                    );
                                                                    if t {
                                                                        break 'b_365e28;
                                                                    } else {
                                                                        'b_3656fc: {
                                                                            c.cycles += 39;
                                                                            c.g[V0][0] = 0x440000;
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xf0,
                                                                                ));
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x4c10,
                                                                                    ));
                                                                            c.g[V1][0] = 0x50000000;
                                                                            c.g[V0][0] =
                                                                                c.g[V0][0] << 32;
                                                                            c.g[A1][0] = 0x10000000;
                                                                            c.g[A1][0] =
                                                                                c.g[A1][0] | 0x4;
                                                                            c.g[V0][0] = c.g[V0][0]
                                                                                | c.g[V1][0];
                                                                            c.g[A2][0] = 0xd806;
                                                                            c.g[A2][0] =
                                                                                c.g[A2][0] << 47;
                                                                            c.sd(
                                                                                c.g[A0][0] as u32,
                                                                                c.g[V0][0],
                                                                            );
                                                                            c.g[T2][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0xf0,
                                                                                    ));
                                                                            c.g[AT][0] = 0x3d800000;
                                                                            c.mtc1(1, 1);
                                                                            c.g[A3][0] = c.g[SP][0];
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xf0,
                                                                                ));
                                                                            c.g[T3][0] = c.g[T2][0];
                                                                            c.g[T0][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x10,
                                                                                    ));
                                                                            c.g[T1][0] =
                                                                                sx((c.g[SP][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x20,
                                                                                    ));
                                                                            c.sd(
                                                                                (c.g[V1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                                0u64,
                                                                            );
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xf0,
                                                                                ));
                                                                            c.sd(
                                                                                (c.g[V0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x10,
                                                                                    ),
                                                                                c.g[A1][0],
                                                                            );
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xf0,
                                                                                ));
                                                                            c.sd(
                                                                                (c.g[V1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x18,
                                                                                    ),
                                                                                0u64,
                                                                            );
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xf0,
                                                                                ));
                                                                            c.sd(
                                                                                (c.g[V0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x20,
                                                                                    ),
                                                                                0u64,
                                                                            );
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xf0,
                                                                                ));
                                                                            c.sd(
                                                                                (c.g[V1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x28,
                                                                                    ),
                                                                                c.g[A2][0],
                                                                            );
                                                                            c.lq(
                                                                                2,
                                                                                (c.g[T8][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x160,
                                                                                    ),
                                                                            );
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xf0,
                                                                                ));
                                                                            c.sq(
                                                                                2,
                                                                                c.g[SP][0] as u32,
                                                                            );
                                                                            c.g[A1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x40,
                                                                                    ));
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                            );
                                                                            c.g[V1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x30,
                                                                                    ));
                                                                            c.g[A0][0] = c.g[V1][0];
                                                                            c.sw(
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf0,
                                                                                    ),
                                                                                c.g[V1][0],
                                                                            );
                                                                            c.mul_s(0, 0, 1);
                                                                            let t = c.g[A0][0]
                                                                                == c.g[A1][0];
                                                                            c.swc1(
                                                                                0,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_3656fc;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                'l_3656e0: loop {
                                                                                    c.cycles += 7;
                                                                                    c.lq(
                                                                                        2,
                                                                                        c.g[A3][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.sq(
                                                                                        2,
                                                                                        c.g[A0][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                                                                                    let t = c.g[A0]
                                                                                        [0]
                                                                                        != c.g[A1]
                                                                                            [0];
                                                                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
                                                                                    if t {
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x003656e0);
                                                                                        }
                                                                                        continue 'l_3656e0;
                                                                                    } else {
                                                                                        break 'b_3656fc;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_36574c: {
                                                                            c.cycles += 12;
                                                                            c.lq(
                                                                                2,
                                                                                (c.g[T8][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x170,
                                                                                    ),
                                                                            );
                                                                            c.g[A1][0] = c.g[T0][0];
                                                                            c.sw(
                                                                                c.g[T2][0] as u32,
                                                                                c.g[A0][0],
                                                                            );
                                                                            c.sq(
                                                                                2,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x10,
                                                                                    ),
                                                                            );
                                                                            c.g[AT][0] = 0x3d800000;
                                                                            c.mtc1(1, 1);
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x18,
                                                                                    ),
                                                                            );
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xf0,
                                                                                ));
                                                                            c.mul_s(0, 0, 1);
                                                                            c.g[A0][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x10,
                                                                                    ));
                                                                            let t = c.g[V1][0]
                                                                                == c.g[A0][0];
                                                                            c.swc1(
                                                                                0,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x18,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_36574c;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                'l_365730: loop {
                                                                                    c.cycles += 7;
                                                                                    c.lq(
                                                                                        2,
                                                                                        c.g[A1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.sq(
                                                                                        2,
                                                                                        c.g[V1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                                    let t = c.g[V1]
                                                                                        [0]
                                                                                        != c.g[A0]
                                                                                            [0];
                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                                                                                    if t {
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x00365730);
                                                                                        }
                                                                                        continue 'l_365730;
                                                                                    } else {
                                                                                        break 'b_36574c;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 11;
                                                                        c.sw(
                                                                            c.g[T3][0] as u32,
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A0][0] = c.g[T1][0];
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x28),
                                                                            0u64,
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xf0));
                                                                        c.lwc1(
                                                                            1,
                                                                            (c.g[T8][0] as u32)
                                                                                .wrapping_add(0xf4),
                                                                        );
                                                                        c.lwc1(
                                                                            0,
                                                                            (c.g[T8][0] as u32)
                                                                                .wrapping_add(0xe0),
                                                                        );
                                                                        c.g[A1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x10,
                                                                                ));
                                                                        c.swc1(
                                                                            1,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x24),
                                                                        );
                                                                        c.swc1(
                                                                            0,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x20),
                                                                        );
                                                                        let t = c.g[V1][0]
                                                                            == c.g[A1][0];
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x2c),
                                                                            0u64,
                                                                        );
                                                                        if t {
                                                                            break 'b_3658cc;
                                                                        } else {
                                                                            'l_365778: loop {
                                                                                c.cycles += 7;
                                                                                c.lq(
                                                                                    2,
                                                                                    c.g[A0][0]
                                                                                        as u32,
                                                                                );
                                                                                c.sq(
                                                                                    2,
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                );
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                                let t = c.g[V1][0]
                                                                                    != c.g[A1][0];
                                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                                                                                if t {
                                                                                    if event_due(c)
                                                                                    {
                                                                                        deliver_event_here(c, 0x00365778);
                                                                                    }
                                                                                    continue 'l_365778;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    let t = true;
                                                                                    c.sw(
                                                                                        c.g[T3][0]
                                                                                            as u32,
                                                                                        c.g[V1][0],
                                                                                    );
                                                                                    if t {
                                                                                        break 'b_365e20;
                                                                                    } else {
                                                                                        break 'b_36579c;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        'b_365834: {
                                                            c.cycles += 30;
                                                            c.g[V1][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                            c.g[V0][0] = 0x10000000;
                                                            c.g[V0][0] = c.g[V0][0] | 0x4;
                                                            c.g[A0][0] = 0xd806;
                                                            c.g[A0][0] = c.g[A0][0] << 47;
                                                            c.g[AT][0] = 0x3d800000;
                                                            c.mtc1(1, 1);
                                                            c.sd(c.g[V1][0] as u32, c.g[V0][0]);
                                                            c.g[A3][0] = sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                            c.g[A2][0] = c.g[SP][0];
                                                            c.g[T3][0] = c.g[A3][0];
                                                            c.g[V1][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                            c.g[T0][0] = sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[T1][0] = sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x20));
                                                            c.sd(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                                0u64,
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                            c.sd(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                                0u64,
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                            c.sd(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x18),
                                                                c.g[A0][0],
                                                            );
                                                            c.lq(
                                                                2,
                                                                (c.g[T8][0] as u32)
                                                                    .wrapping_add(0x160),
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                            c.sq(2, c.g[SP][0] as u32);
                                                            c.g[A1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x30));
                                                            c.lwc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            c.g[A0][0] = c.g[V1][0];
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xf0),
                                                                c.g[V1][0],
                                                            );
                                                            c.mul_s(0, 0, 1);
                                                            let t = c.g[A0][0] == c.g[A1][0];
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            if t {
                                                                break 'b_365834;
                                                            } else {
                                                                c.cycles += 1;
                                                                'l_365818: loop {
                                                                    c.cycles += 7;
                                                                    c.lq(2, c.g[A2][0] as u32);
                                                                    c.sq(2, c.g[A0][0] as u32);
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x10));
                                                                    let t =
                                                                        c.g[A0][0] != c.g[A1][0];
                                                                    c.g[A2][0] = sx((c.g[A2][0]
                                                                        as u32)
                                                                        .wrapping_add(0x10));
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x00365818,
                                                                            );
                                                                        }
                                                                        continue 'l_365818;
                                                                    } else {
                                                                        break 'b_365834;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        'b_365884: {
                                                            c.cycles += 12;
                                                            c.lq(
                                                                2,
                                                                (c.g[T8][0] as u32)
                                                                    .wrapping_add(0x170),
                                                            );
                                                            c.g[A1][0] = c.g[T0][0];
                                                            c.sw(c.g[A3][0] as u32, c.g[A0][0]);
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.g[AT][0] = 0x3d800000;
                                                            c.mtc1(1, 1);
                                                            c.lwc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                            c.mul_s(0, 0, 1);
                                                            c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            let t = c.g[V1][0] == c.g[A0][0];
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            if t {
                                                                break 'b_365884;
                                                            } else {
                                                                c.cycles += 1;
                                                                'l_365868: loop {
                                                                    c.cycles += 7;
                                                                    c.lq(2, c.g[A1][0] as u32);
                                                                    c.sq(2, c.g[V1][0] as u32);
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x10));
                                                                    let t =
                                                                        c.g[V1][0] != c.g[A0][0];
                                                                    c.g[A1][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x10));
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x00365868,
                                                                            );
                                                                        }
                                                                        continue 'l_365868;
                                                                    } else {
                                                                        break 'b_365884;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 11;
                                                        c.sw(c.g[T3][0] as u32, c.g[V1][0]);
                                                        c.g[A0][0] = c.g[T1][0];
                                                        c.sw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x28),
                                                            0u64,
                                                        );
                                                        c.g[V1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                        c.lwc1(
                                                            1,
                                                            (c.g[T8][0] as u32).wrapping_add(0xf4),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[T8][0] as u32).wrapping_add(0xe0),
                                                        );
                                                        c.g[A1][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x24),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        let t = c.g[V1][0] == c.g[A1][0];
                                                        c.sw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x2c),
                                                            0u64,
                                                        );
                                                        if t {
                                                            break 'b_3658cc;
                                                        } else {
                                                            'l_3658b0: loop {
                                                                c.cycles += 7;
                                                                c.lq(2, c.g[A0][0] as u32);
                                                                c.sq(2, c.g[V1][0] as u32);
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                let t = c.g[V1][0] != c.g[A1][0];
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                if t {
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x003658b0,
                                                                        );
                                                                    }
                                                                    continue 'l_3658b0;
                                                                } else {
                                                                    break 'b_3658cc;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.sw(c.g[T3][0] as u32, c.g[V1][0]);
                                                    if t {
                                                        break 'b_365e20;
                                                    } else {
                                                        break 'b_3658d4;
                                                    }
                                                }
                                                'b_365950: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[GP][0] as u32)
                                                            .wrapping_add(0x28a0));
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_365950;
                                                    } else {
                                                        c.cycles += 28;
                                                        c.g[V1][0] = 0x500000;
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 3);
                                                        c.g[V0][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x480));
                                                        c.g[AT][0] = 0x3f000000;
                                                        c.mtc1(1, 2);
                                                        c.mtc1(0, 0);
                                                        c.g[A0][0] = 0x1;
                                                        c.swc1(
                                                            3,
                                                            (c.g[V0][0] as u32).wrapping_add(0x3c),
                                                        );
                                                        c.g[AT][0] = 0xffffffffbf000000;
                                                        c.mtc1(1, 1);
                                                        c.swc1(
                                                            2,
                                                            (c.g[V1][0] as u32).wrapping_sub(0x480),
                                                        );
                                                        c.swc1(
                                                            1,
                                                            (c.g[V0][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.swc1(
                                                            2,
                                                            (c.g[V0][0] as u32).wrapping_add(0x34),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[V0][0] as u32).wrapping_add(0x2c),
                                                        );
                                                        c.sw(
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_add(0x28a0),
                                                            c.g[A0][0],
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[V0][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[V0][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.swc1(
                                                            2,
                                                            (c.g[V0][0] as u32).wrapping_add(0x30),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[V0][0] as u32).wrapping_add(0x4),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[V0][0] as u32).wrapping_add(0x24),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[V0][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[V0][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[V0][0] as u32).wrapping_add(0x28),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[V0][0] as u32).wrapping_add(0x38),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[V0][0] as u32).wrapping_add(0xc),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[V0][0] as u32).wrapping_add(0x1c),
                                                        );
                                                        c.g[V0][0] = 0x1;
                                                        break 'b_365950;
                                                    }
                                                }
                                                'b_365b84: {
                                                    c.cycles += 2;
                                                    let t = c.g[A2][0] == c.g[V0][0];
                                                    c.g[V0][0] =
                                                        ((c.g[A2][0] as i64) < 2i64) as u64;
                                                    if t {
                                                        break 'b_365b84;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x2;
                                                        if t {
                                                            break 'b_365da0;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[A2][0] != 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xf0));
                                                            if t {
                                                                break 'b_365e24;
                                                            } else {
                                                                c.cycles += 131;
                                                                c.g[V0][0] = 0x430000;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x4990));
                                                                c.g[T4][0] = 0x50000000;
                                                                c.g[V0][0] = c.g[V0][0] << 32;
                                                                c.g[A0][0] = 0x10000000;
                                                                c.g[A0][0] = c.g[A0][0] | 0x7;
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] | c.g[T4][0];
                                                                c.g[A1][0] = 0xd80c;
                                                                c.g[A1][0] = c.g[A1][0] << 47;
                                                                c.sd(c.g[V1][0] as u32, c.g[V0][0]);
                                                                c.g[A3][0] = 0x10000000;
                                                                c.g[A3][0] = c.g[A3][0] | 0x5;
                                                                c.g[T0][0] = 0x6c040000;
                                                                c.g[T0][0] = c.g[T0][0] | 0x392;
                                                                c.g[T0][0] = c.g[T0][0] << 32;
                                                                c.g[T1][0] = 0x312e0000;
                                                                c.g[T1][0] = c.g[T1][0] | 0x4000;
                                                                c.g[T1][0] = c.g[T1][0] << 32;
                                                                c.g[T1][0] = c.g[T1][0] | 0x8000;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.g[T2][0] = 0x412;
                                                                c.g[A2][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x30));
                                                                c.sd(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                    0u64,
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    c.g[A0][0],
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                    0u64,
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                    0u64,
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x28),
                                                                    c.g[A1][0],
                                                                );
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.lq(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0x80),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.lq(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0x90),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x40),
                                                                );
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.lq(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0xa0),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x50),
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.lq(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0xb0),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x60),
                                                                );
                                                                c.lq(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0xc0),
                                                                );
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sq(
                                                                    2,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x70),
                                                                );
                                                                c.lq(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0xd0),
                                                                );
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sq(
                                                                    2,
                                                                    (c.g[A1][0] as u32)
                                                                        .wrapping_add(0x80),
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x90),
                                                                    c.g[A3][0],
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x98),
                                                                    0u64,
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0xa0),
                                                                    0u64,
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0xa8),
                                                                    c.g[T0][0],
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0xb0),
                                                                    c.g[T1][0],
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.sd(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0xb8),
                                                                    c.g[T2][0],
                                                                );
                                                                c.lqc2(1, c.g[T8][0] as u32);
                                                                c.lqc2(
                                                                    2,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.lqc2(
                                                                    3,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lqc2(
                                                                    4,
                                                                    (c.g[T8][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.sqc2(1, c.g[A2][0] as u32);
                                                                c.sqc2(
                                                                    2,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.sqc2(
                                                                    3,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.sqc2(
                                                                    4,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.g[V0][0] = 0x500000;
                                                                c.g[A0][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x70));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x480));
                                                                c.g[V1][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0xb0));
                                                                c.lqc2(4, c.g[V0][0] as u32);
                                                                c.lqc2(
                                                                    5,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.lqc2(
                                                                    6,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lqc2(
                                                                    7,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.lqc2(8, c.g[A2][0] as u32);
                                                                c.lqc2(
                                                                    9,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.lqc2(
                                                                    10,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lqc2(
                                                                    11,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.vu0().mula(
                                                                    0b1111,
                                                                    4,
                                                                    Src::Bc(8, X),
                                                                );
                                                                c.vu0().madda(
                                                                    0b1111,
                                                                    5,
                                                                    Src::Bc(8, Y),
                                                                );
                                                                c.vu0().madda(
                                                                    0b1111,
                                                                    6,
                                                                    Src::Bc(8, Z),
                                                                );
                                                                c.vu0().madd(
                                                                    0b1111,
                                                                    12,
                                                                    7,
                                                                    Src::Bc(8, W),
                                                                );
                                                                c.vu0().mula(
                                                                    0b1111,
                                                                    4,
                                                                    Src::Bc(9, X),
                                                                );
                                                                c.vu0().madda(
                                                                    0b1111,
                                                                    5,
                                                                    Src::Bc(9, Y),
                                                                );
                                                                c.vu0().madda(
                                                                    0b1111,
                                                                    6,
                                                                    Src::Bc(9, Z),
                                                                );
                                                                c.vu0().madd(
                                                                    0b1111,
                                                                    13,
                                                                    7,
                                                                    Src::Bc(9, W),
                                                                );
                                                                c.vu0().mula(
                                                                    0b1111,
                                                                    4,
                                                                    Src::Bc(10, X),
                                                                );
                                                                c.vu0().madda(
                                                                    0b1111,
                                                                    5,
                                                                    Src::Bc(10, Y),
                                                                );
                                                                c.vu0().madda(
                                                                    0b1111,
                                                                    6,
                                                                    Src::Bc(10, Z),
                                                                );
                                                                c.vu0().madd(
                                                                    0b1111,
                                                                    14,
                                                                    7,
                                                                    Src::Bc(10, W),
                                                                );
                                                                c.vu0().mula(
                                                                    0b1111,
                                                                    4,
                                                                    Src::Bc(11, X),
                                                                );
                                                                c.vu0().madda(
                                                                    0b1111,
                                                                    5,
                                                                    Src::Bc(11, Y),
                                                                );
                                                                c.vu0().madda(
                                                                    0b1111,
                                                                    6,
                                                                    Src::Bc(11, Z),
                                                                );
                                                                c.vu0().madd(
                                                                    0b1111,
                                                                    15,
                                                                    7,
                                                                    Src::Bc(11, W),
                                                                );
                                                                c.sqc2(12, c.g[V1][0] as u32);
                                                                c.sqc2(
                                                                    13,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.sqc2(
                                                                    14,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.sqc2(
                                                                    15,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.lqc2(1, c.g[V1][0] as u32);
                                                                c.lqc2(
                                                                    2,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.lqc2(
                                                                    3,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lqc2(
                                                                    4,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.sqc2(1, c.g[A0][0] as u32);
                                                                c.sqc2(
                                                                    2,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.sqc2(
                                                                    3,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.sqc2(
                                                                    4,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.lqc2(1, c.g[A0][0] as u32);
                                                                c.lqc2(
                                                                    2,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.lqc2(
                                                                    3,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lqc2(
                                                                    4,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.sqc2(1, c.g[A2][0] as u32);
                                                                c.sqc2(
                                                                    2,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.sqc2(
                                                                    3,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.sqc2(
                                                                    4,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.lq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0xc0),
                                                                );
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.lq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x40),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0xd0),
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.lq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x50),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0xe0),
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0));
                                                                c.g[V0][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0xf0));
                                                                let t = c.g[T3][0] == 0u64;
                                                                c.sw(
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xf0),
                                                                    c.g[V0][0],
                                                                );
                                                                if t {
                                                                    break 'b_365e20;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[V0][0] = c.g[T3][0] << 32;
                                                                    c.g[V0][0] =
                                                                        c.g[V0][0] | c.g[T4][0];
                                                                    let t = true;
                                                                    c.sd(
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0xf0),
                                                                        c.g[V0][0],
                                                                    );
                                                                    if t {
                                                                        break 'b_365e0c;
                                                                    } else {
                                                                        break 'b_365b84;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 121;
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.g[V0][0] = 0x10000000;
                                                c.g[V0][0] = c.g[V0][0] | 0x7;
                                                c.g[A0][0] = 0xd80c;
                                                c.g[A0][0] = c.g[A0][0] << 47;
                                                c.g[A1][0] = 0x10000000;
                                                c.g[A1][0] = c.g[A1][0] | 0x5;
                                                c.sd(c.g[V1][0] as u32, c.g[V0][0]);
                                                c.g[A2][0] = 0x6c040000;
                                                c.g[A2][0] = c.g[A2][0] | 0x392;
                                                c.g[A2][0] = c.g[A2][0] << 32;
                                                c.g[A3][0] = 0x312e0000;
                                                c.g[A3][0] = c.g[A3][0] | 0x4000;
                                                c.g[A3][0] = c.g[A3][0] << 32;
                                                c.g[A3][0] = c.g[A3][0] | 0x8000;
                                                c.g[T0][0] = 0x412;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.sd(
                                                    (c.g[V0][0] as u32).wrapping_add(0x18),
                                                    c.g[A0][0],
                                                );
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0x80));
                                                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x20));
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0x90));
                                                c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xa0));
                                                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x40));
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xb0));
                                                c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x50));
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xc0));
                                                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x60));
                                                c.lq(2, (c.g[T8][0] as u32).wrapping_add(0xd0));
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x70));
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.sd(
                                                    (c.g[V1][0] as u32).wrapping_add(0x80),
                                                    c.g[A1][0],
                                                );
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.sd((c.g[V0][0] as u32).wrapping_add(0x88), 0u64);
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.sd((c.g[V1][0] as u32).wrapping_add(0x90), 0u64);
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.sd(
                                                    (c.g[V0][0] as u32).wrapping_add(0x98),
                                                    c.g[A2][0],
                                                );
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.sd(
                                                    (c.g[V1][0] as u32).wrapping_add(0xa0),
                                                    c.g[A3][0],
                                                );
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.sd(
                                                    (c.g[V0][0] as u32).wrapping_add(0xa8),
                                                    c.g[T0][0],
                                                );
                                                c.lqc2(1, c.g[T8][0] as u32);
                                                c.lqc2(2, (c.g[T8][0] as u32).wrapping_add(0x10));
                                                c.lqc2(3, (c.g[T8][0] as u32).wrapping_add(0x20));
                                                c.lqc2(4, (c.g[T8][0] as u32).wrapping_add(0x30));
                                                c.sqc2(1, c.g[SP][0] as u32);
                                                c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                c.g[V0][0] = 0x500000;
                                                c.g[A0][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x40));
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x480));
                                                c.g[V1][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x80));
                                                c.lqc2(4, c.g[V0][0] as u32);
                                                c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                c.lqc2(6, (c.g[V0][0] as u32).wrapping_add(0x20));
                                                c.lqc2(7, (c.g[V0][0] as u32).wrapping_add(0x30));
                                                c.lqc2(8, c.g[SP][0] as u32);
                                                c.lqc2(9, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                c.lqc2(10, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.lqc2(11, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                                                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                                                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                                                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                                                c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                                                c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                                                c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                                                c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                                                c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                                                c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                                                c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                                                c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                                                c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                                                c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                                                c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                                                c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                                                c.sqc2(12, c.g[V1][0] as u32);
                                                c.sqc2(13, (c.g[V1][0] as u32).wrapping_add(0x10));
                                                c.sqc2(14, (c.g[V1][0] as u32).wrapping_add(0x20));
                                                c.sqc2(15, (c.g[V1][0] as u32).wrapping_add(0x30));
                                                c.lqc2(1, c.g[V1][0] as u32);
                                                c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                                                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                                                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                                                c.sqc2(1, c.g[A0][0] as u32);
                                                c.sqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                                                c.sqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                                                c.sqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
                                                c.lqc2(1, c.g[A0][0] as u32);
                                                c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                                                c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                                                c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
                                                c.sqc2(1, c.g[SP][0] as u32);
                                                c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.lq(2, c.g[SP][0] as u32);
                                                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0xb0));
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                c.sq(2, (c.g[A0][0] as u32).wrapping_add(0xc0));
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0xd0));
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.g[V0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0xe0));
                                                let t = c.g[T3][0] == 0u64;
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0xf0),
                                                    c.g[V0][0],
                                                );
                                                if t {
                                                    break 'b_365e20;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[V1][0] = 0x50000000;
                                                    c.g[V0][0] = c.g[T3][0] << 32;
                                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                                    let t = true;
                                                    c.sd(
                                                        (c.g[A0][0] as u32).wrapping_add(0xe0),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_365e0c;
                                                    } else {
                                                        break 'b_365d7c;
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x1;
                                            let t = c.g[A2][0] == c.g[V0][0];
                                            c.g[V0][0] = ((c.g[A2][0] as i64) < 2i64) as u64;
                                            if t {
                                                break 'b_365df4;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_365da0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[A2][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                                    if t {
                                                        break 'b_365db0;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        if t {
                                                            break 'b_365e28;
                                                        } else {
                                                            break 'b_365da0;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                        if t {
                                            break 'b_365df4;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                                            if t {
                                                break 'b_365e28;
                                            } else {
                                                break 'b_365db0;
                                            }
                                        }
                                    }
                                    c.cycles += 13;
                                    c.g[V0][0] = 0x430000;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4990));
                                    c.g[A3][0] = 0x50000000;
                                    c.g[V0][0] = c.g[V0][0] << 32;
                                    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                                    c.sd(c.g[A0][0] as u32, c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                    let t = c.g[T3][0] == 0u64;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[V0][0]);
                                    if t {
                                        break 'b_365e20;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.g[T3][0] << 32;
                                        c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                                        let t = true;
                                        c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                        if t {
                                            break 'b_365e0c;
                                        } else {
                                            break 'b_365df4;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[T3][0] == 0u64;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                if t {
                                    break 'b_365e20;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = 0x50000000;
                                    c.g[V0][0] = c.g[T3][0] << 32;
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.sd(c.g[A0][0] as u32, c.g[V0][0]);
                                    break 'b_365e0c;
                                }
                            }
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                            break 'b_365e1c;
                        }
                        c.cycles += 1;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[V0][0]);
                        break 'b_365e20;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                break 'b_365e24;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
            break 'b_365e28;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sw(c.g[T9][0] as u32, c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
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

    const SELF: u32 = DATA;
    const PP: u32 = DATA + 0x10_0000;
    const PKT: u32 = DATA + 0x10_0100;

    /// `(self, type, mode, view)` with the jump table (0x492040) the game
    /// binary holds, the view-state index and a packet cursor.
    fn setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x4cd0, 0);
        let table = [
            0x365e20u32,
            0x364d3c,
            0x364e38,
            0x365190,
            0x365314,
            0x365624,
            0x3658d4,
            0x365d7c,
            0x365e20,
        ];
        for (i, t) in table.iter().enumerate() {
            c.bus.write32(0x0049_2040 + 4 * i as u32, *t);
        }
        for i in 0..0x400u32 {
            c.bus.write32(SELF + 0x69_cc0 + 4 * i, r.next_u32());
        }
        c.bus.write32(SELF + 0x69_cc0, r.next_u32() % 4);
        c.bus.write32(PP, PKT);
        c.g[A0][0] = SELF as u64;
        c.g[A1][0] = (seed % 10) as u64;
        c.g[A2][0] = (seed / 10 % 3) as u64;
        c.g[A3][0] = (r.next_u32() % 3) as u64;
        c.g[T0][0] = PP as u64;
        c.pc = 0x0036_4cd0;
    }

    #[test]
    fn defer_man_emit_material_setup_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..200 {
            let mut c = fresh();
            setup(&mut c, seed);
            let b = c.cycles;
            run_generated(&mut c, ee::s12::f_00364cd0__generated);
            seen.insert(c.cycles - b);
        }
        assert!(seen.len() >= 12, "paths: {seen:?}");
    }

    #[test]
    fn defer_man_emit_material_setup_matches() {
        check(
            ee::s12::f_00364cd0__generated,
            defer_man_emit_material_setup,
            200,
            &setup,
        );
    }
}
